use crate::database;
use crate::ImgDetail;
use crate::Picture;
use chrono::Local;
use image::io::Reader as ImageReader;
use image::GenericImageView;
use rand::{thread_rng, Rng};
use rayon::prelude::*;
use rayon::str::Bytes;
use reqwest::blocking::Response;
use reqwest::blocking;
use rusqlite::Connection;
use scraper::{Html, Selector};
use serde::Serialize;
use tauri::Window;
use tokio::spawn;
use std::error::Error;

#[derive(Debug, Serialize)]
struct PageInfo {
    title: String,
    href: String,
}

async fn fetch_page(url: &str) -> Result<Html, reqwest::Error> {
    let client = reqwest::Client::new();
    let response = client.get(url).send().await?;
    let html = response.text().await?;
    Ok(Html::parse_document(&html))
}
fn fetch_img_page(src: &str) -> Result<Response, reqwest::Error> {
    let res = reqwest::blocking::Client::new().get(src).send()?;
    // let body = res.bytes()?;
    Ok(res)
}

fn parse_page(html: &Html) -> Vec<PageInfo> {
    // let document = Html::parse_document(body);
    let selector = Selector::parse("tr:nth-of-type(n+8) h3 a").expect("Failed to parse selector");
    let mut pages_info = Vec::new();
    for element in html.select(&selector) {
        let title = element.inner_html();
        if let Some(href) = element.value().attr("href") {
            let page_info: PageInfo = PageInfo {
                title: title.to_string(),
                href: "http://dkleh8.xyz/pw/".to_string() + href,
            };
            pages_info.push(page_info);
        }
    }
    pages_info
}

async fn parse_img_detail(html: &Html) -> Vec<ImgDetail> {
    // let document = Html::parse_document(body);
    let selector = Selector::parse("div.f14 img").expect("Failed to parse selector");
    let mut img_details = Vec::new();
    let client = blocking::Client::new();
    html.select(&selector).into_iter().for_each(|element| {
        if let Some(src) = element.value().attr("src") {
            let task = spawn(async move {
                match get_image_dimensions(&client,src) {
                    Ok(ratio) => {
                        let img_detail = ImgDetail {
                            src: src.to_string(),
                            aspect_ratio: ratio,
                        };
                        img_details.push(img_detail);
                    }
                    Err(e) => {
                        let img_detail = ImgDetail {
                            src: src.to_string(),
                            aspect_ratio: 1.0,
                        };
                        img_details.push(img_detail);
                        eprintln!("Failed to get image dimensions: {}", e);
                    }
                }
            });
        }
    });
    img_details
}

async fn parse_picture_data(page_info: &PageInfo) -> Result<Picture, reqwest::Error> {
    match fetch_page(&page_info.href).await {
        Ok(html) => {
            let srcs = parse_img_detail(&html).await;
            Picture {
                id: new_id(),
                title: page_info.title.to_string(),
                url: page_info.href.to_string(),
                srcs,
                star: 0,
                collect: false,
                download: false,
                deleted: false,
            }
        }
        Err(e) => {
            eprintln!("Failed to fetch page: {}", e);
        }
    }
}


fn get_image_dimensions(client: &blocking::Client, src: &str) -> Result<f32, Box<dyn Error>> {
    let response = client.get(src).send()?;

    if let Ok(image_bytes) = response.bytes() {
        let image = ImageReader::new(std::io::Cursor::new(image_bytes))
            .with_guessed_format()?
            .decode()?;
        let (width, height) = image.dimensions();
        Ok((width as f32) / (height as f32))
    } else {
        eprintln!("Failed to get image content for URL: {}", src);
        Ok(0.0)
    }
}


// 生成随机id
pub fn new_id() -> u32 {
    let now = Local::now();
    let timestamp = now.timestamp() as u32;
    let random_number: u32 = thread_rng().gen();
    let unique_id = timestamp.wrapping_add(random_number);
    unique_id
}

pub async fn spider(window: &Window, db: &Connection, url: &str) {
    match fetch_page(url).await {
        Ok(html) => {
            let page_infos = parse_page(&html);
            for page_info in page_infos {
                let task = spawn(async move {
                    match parse_picture_data(&page_info).await {
                        Ok(picture) => {
                            let _ = window.emit("spider-picture", picture.clone());
                            if let Err(e) = database::insert_picture(db, picture) {
                                eprintln!("Failed to insert picture: {}", e);
                            }
                        }
                        Err(e) => {
                            eprintln!("Failed to parse picture: {}", e);
                        }
                    }
                });

                task.await.unwrap();
            }
        }
        Err(e) => {
            eprintln!("Failed to fetch page: {}", e);
        }
    }
}
