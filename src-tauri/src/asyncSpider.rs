use crate::{ImgDetail, Picture};
use chrono::Local;
use futures::future::try_join_all;
use image::io::Reader as ImageReader;
use image::GenericImageView;
use rand::{thread_rng, Rng};
use reqwest::Client;
use rusqlite::Connection;
use scraper::{Html, Selector};
use serde::Serialize;
use std::error::Error;
use tokio::task;

#[derive(Debug, Serialize)]
struct PageInfo {
    title: String,
    href: String,
}

// 假设 ImgDetail 和 Picture 已经被定义

#[derive(Debug, Serialize)]
pub struct Spider {
    pictures: Vec<Picture>,
}

impl Spider {
    // 注意 new 函数现在是异步的
    pub async fn new(db: &Connection, url: &str) -> Result<Self, Box<dyn Error>> {
        let body = Self::get_web_content(url).await?;

        let pages_info = Self::parse_page(&body);

        // 以并发方式获取图片数据
        let futures = pages_info
            .into_iter()
            .map(|page_info| Self::fetch_picture_data(&page_info));

        let pictures: Vec<_> = try_join_all(futures).await?.into_iter().collect();

        // 批量插入图片数据（此处略过具体实现）

        Ok(Spider { pictures })
    }

    async fn get_web_content(url: &str) -> Result<String, Box<dyn Error>> {
        let client = reqwest::Client::new();
        let response = client.get(url).send().await?;
        Ok(response.text().await?)
    }

    async fn fetch_picture_data(page_info: &PageInfo) -> Result<Picture, Box<dyn Error>> {
        // 异步获取 Web 内容
        let img_body = Self::get_web_content(&page_info.href).await?;
        let srcs = Self::parse_picture(&img_body).await?;
        // 生成图片信息
        let id = Self::new_id();

        Ok(Picture {
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

    // 解析图片详情，修改为异步函数
    async fn parse_picture(body: &str) -> Result<Vec<ImgDetail>, Box<dyn Error>> {
        // 解析 HTML，获取图片 URL，然后调用异步函数 fetch_image_dimensions 获取尺寸
        let document = Html::parse_document(body);
        let selector = Selector::parse("div.f14 img").expect("Failed to parse selector");
        let mut img_details = Vec::new();
        let client = blocking::Client::new();
        document.select(&selector).into_iter().for_each(|element| {
            if let Some(src) = element.value().attr("src") {
                match Self::get_image_dimensions(&client, src).await {
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
        Ok(img_details)
    }

    // 异步获取图片尺寸
    async fn fetch_image_dimensions(
        client: &Client,
        src: &str,
    ) -> Result<f32, Box<dyn Error>> {
        let res = client.get(src).send().await?;
       if let Ok(bytes) = res.bytes().await {
        let img = ImageReader::new(std::io::Cursor::new(bytes))
            .with_guessed_format()?
            .decode()?;
        // 返回宽高
        let (width, height) = img.dimensions();
        Ok((width as f32) / (height as f32))
       }esle{
        eprintln!("Failed to get image content for URL: {}", src);
        Ok(0.0)
       }
        
    }
    // 其他方法定义省略..

    // 解析页面，获取页面标题和链接
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

    // 生成随机id
    pub fn new_id() -> u32 {
        let now = Local::now();
        let timestamp = now.timestamp() as u32;
        let random_number: u32 = thread_rng().gen();
        let unique_id = timestamp.wrapping_add(random_number);
        unique_id
    }
}
