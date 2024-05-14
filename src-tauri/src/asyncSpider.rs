use reqwest::Error;
use scraper::{Html, Selector};

use crate::database::{self, Database};
use crate::{ImgDetail, Picture};
use chrono::Local;
use rand::{thread_rng, Rng};
use rusqlite::Connection;
use tauri::Window;

#[derive(Clone, serde::Serialize)]
struct Payload {
    percentage: String,
    picture: Picture,
}

pub async fn get_pictures(db: Database, window: Window, url: String) -> Result<(), Error> {
    let selector = Selector::parse("a.link-muted").expect("Failed to parse selector");
    let spider = Spider::new(url, selector);
    let hrefs = spider.get_page_info().await;
    // let mut pictures = Vec::new();
    let mut index = 0;
    match hrefs {
        Ok(hrefs) => {
            let len = hrefs.len();
            for href in hrefs {
                // if index == 5 {break}
                let src = href.get_img_src().await;
                match src {
                    Ok(vec) => {
                        let picture = Picture {
                            id: new_id(),
                            title: href.title,
                            url: href.href,
                            srcs: vec,
                            star: 0,
                            collect: false,
                            download: false,
                            deleted: false,
                        };
                        index += 1;
                        let pic_clone = picture.clone();
                        if let Err(err) = db.insert_picture(pic_clone) {
                            eprintln!("Failed to insert picture: {}", err);
                            // Handle error here if needed
                        }
                        // pictures.push(picture);
                        let percentage = format!("{:.1}%", (index as f64 / len as f64) * 100.0);
                        window
                            .emit(
                                "spider-event",
                                Payload {
                                    percentage,
                                    picture,
                                },
                            )
                            .unwrap();
                    }
                    Err(e) => {
                        println!("{}", e);
                    }
                }
            }
        }
        Err(e) => {
            println!("{}", e)
        }
    }
    Ok(())
}

#[derive(Debug)]
pub struct Spider {
    pub url: String,
    pub selector: Selector,
}
impl Spider {
    pub fn new(url: String, selector: Selector) -> Self {
        Spider { url, selector }
    }

    pub async fn get_page_content(&self) -> Result<Html, Error> {
        // print!(" get page content running {} ",self.url);
        let client = reqwest::Client::new();
        match client.get(&self.url).send().await {
            Ok(response) => {
                let body = response.text().await?;
                let html = Html::parse_fragment(&body);
                Ok(html)
            }
            Err(e) => Err(e),
        }
    }

    pub async fn get_page_info(&self) -> Result<Vec<SpiderPageInfo>, Error> {
        let html = self.get_page_content().await;
        match html {
            Ok(html) => {
                let select = html.select(&self.selector);
                println!("{:?}", &self.selector);
                let mut page_info: Vec<SpiderPageInfo> = Vec::new();
                for info in select {
                    let selector = Selector::parse("p img").expect("Failed to parse selector");
                    // let info_clone = info.clone();
                    // let title = info_clone.value().attr("alt").unwrap_or("don't find name").to_string();
                    if let Some(href) = info.value().attr("href") {
                        // 使用空格 ' ' 或逗号 ',' 或句号 '.' 作为分隔符拆分字符串
                        let parts: Vec<&str> = href.split(|c: char| c == '/' || c == '.').collect();

                        // 遍历拆分后的部分并打印出来
                        let title = parts.get(1).unwrap().to_string();;

                        let url = "https://nicegirl.in".to_string() + href;
                        let spider = Spider::new(url, selector);
                        page_info.push(SpiderPageInfo::new(title, self.url.clone(), spider));
                    }
                }
                Ok(page_info)
            }
            Err(e) => {
                println!("{:?}", e);
                Err(e)
            }
        }
    }
}

#[derive(Debug)]
pub struct SpiderPageInfo {
    title: String,
    href: String,
    spider: Spider,
}
impl SpiderPageInfo {
    fn new(title: String, href: String, spider: Spider) -> Self {
        Self {
            title,
            href,
            spider,
        }
    }

    pub async fn get_img_src(&self) -> Result<Vec<ImgDetail>, Error> {
        // print!("get img src running");
        let html = self.spider.get_page_content().await;
        match html {
            Ok(html) => {
                // println!("{:?}",body);
                let mut srcs = vec![];
                let select = html.select(&self.spider.selector);
                for el in select {
                    if let Some(src) = el.value().attr("src") {
                        let url = src.to_string();
                        let img_detail = from_url_split_number(url);
                        srcs.push(img_detail);
                    }
                }
                Ok(srcs)
            }
            Err(e) => Err(e),
        }
    }
}

fn from_url_split_number(url: String) -> ImgDetail {
    if let Some(start) = url.rfind('/') {
        if let Some(end) = url.rfind('.') {
            let dimensions_part: &str = &url[(start + 1)..end];
            let dimensions: Vec<&str> = dimensions_part
                .split('_')
                .last()
                .unwrap()
                .split('x')
                .collect();
            let width: f32 = dimensions[0].parse().unwrap_or(0.0);
            let height: f32 = dimensions[1].parse().unwrap_or(0.0);
            let aspect_ratio = width / height;

            ImgDetail {
                src: url,
                aspect_ratio,
            }
        } else {
            ImgDetail {
                src: url,
                aspect_ratio: 0.0,
            }
        }
    } else {
        ImgDetail {
            src: url,
            aspect_ratio: 0.0,
        }
    }
}

fn new_id() -> u32 {
    let now = Local::now();
    let timestamp = now.timestamp() as u32;
    let random_number: u32 = thread_rng().gen();
    let unique_id = timestamp.wrapping_add(random_number);
    unique_id
}
