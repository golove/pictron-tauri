use chrono::Local;
use image::io::Reader as ImageReader;
use image::GenericImageView;
use rand::{thread_rng, Rng};
use rayon::prelude::*;
use reqwest::blocking;
use rusqlite::Connection;
use std::error::Error;
use crate::ImgDetail;
use crate::Picture;
use scraper::{Html, Selector};
use serde::Serialize;
use crate::database;


#[derive(Debug, Serialize)]
struct PageInfo {
    title: String,
    href: String,
}
#[derive(Debug, Serialize)]
pub struct Spider {
    pictures: Vec<Picture>,
}

impl Spider {
    pub fn new(db: &Connection, url: &str) -> Result<Self, Box<dyn Error>> {
        match Self::get_web_content(url) {
            Ok(body) => {
                let pages_info = Self::parse_page(&body);

                let pictures: Vec<_> = pages_info
                    .par_iter()
                    .filter_map(|page_info| Self::fetch_picture_data(page_info))
                    .collect();

                let cloned_pictures = pictures.clone();

                // 批量插入图片数据
                for picture in pictures {
                    println!("Inserting picture: {}", &picture.id);
                    if let Err(err) = database::insert_picture(db, picture) {
                        eprintln!("Failed to insert picture: {}", err);
                        // Handle error here if needed
                    }
                }
                Ok(Spider {
                    pictures: cloned_pictures,
                })
            }

            Err(e) => {
                println!("Failed to fetch web content: {}", e);
                Err(e)
            }
        }
    }

    pub fn get_pictures(spider: Spider) -> Vec<Picture> {
        spider.pictures
    }

    fn parse_page(body: &str) -> Vec<PageInfo> {
        let document = Html::parse_document(body);
        let selector =
            Selector::parse("tr:nth-of-type(n+8) h3 a").expect("Failed to parse selector");
        let mut pages_info = Vec::new();
        for element in document.select(&selector) {
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

    fn parse_picture(body: &str) -> Vec<ImgDetail> {
        let document = Html::parse_document(body);
        let selector = Selector::parse("div.f14 img").expect("Failed to parse selector");
        let mut img_details = Vec::new();
        let client = blocking::Client::new();
        document.select(&selector).into_iter().for_each(|element| {
            if let Some(src) = element.value().attr("src") {
                match Self::get_image_dimensions(&client, src) {
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
            }
        });
        img_details
    }

    fn get_web_content(url: &str) -> Result<String, Box<dyn Error>> {
        // 尝试发送 HTTP 请求
        let response = blocking::get(url)?;

        // 尝试读取响应体作为字符串
        let body = response.text()?;

        Ok(body)
    }


    // 生成随机id
    pub fn new_id() -> u32 {
        let now = Local::now();
        let timestamp = now.timestamp() as u32;
        let random_number: u32 = thread_rng().gen();
        let unique_id = timestamp.wrapping_add(random_number);
        unique_id
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

    fn fetch_picture_data(page_info: &PageInfo) -> Option<Picture> {
        match Self::get_web_content(&page_info.href) {
            Ok(img_body) => {
                let srcs = Self::parse_picture(&img_body);
                let id = Self::new_id();
                Some(Picture {
                    id,
                    title: page_info.title.clone(),
                    url: page_info.href.clone(),
                    srcs,
                    star: 0,
                    collect: false,
                    download: false,
                    deleted: false,
                })
            }
            Err(e) => {
                eprintln!("Failed to fetch web content: {}", e);
                None
            }
        }
    }
}




