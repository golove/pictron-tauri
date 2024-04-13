// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use regex::Regex;
use tauri::{CustomMenuItem, Manager, Menu, MenuItem, Submenu, Window};
use window_vibrancy::{apply_vibrancy, NSVisualEffectMaterial};

#[tauri::command]
//模拟网络请求
async fn fetch_data(url: String) -> Result<String, String> {
    let client = reqwest::Client::new();
    let response = client.get(&url).send().await;
    if let Ok(response) = response {
        if response.status().is_success() {
            let text = response.text().await;
            if let Ok(text) = text {
                return Ok(text);
            } else {
                return Err("Failed to parse response".into());
            }
        } else {
            return Err("Failed to fetch data".into());
        }
    } else {
        return Err("Failed to send request".into());
    }
}

// 使用正则表达式匹配网页内容并使用数组的形式返回匹配结果
#[tauri::command]
async fn match_data(url: String) -> Result<Vec<String>, String> {
    let response = fetch_data(url).await;
    if let Ok(response) = response {
        let pattern = r"html_data/14/([^<>']*).html";
        let re = Regex::new(pattern).unwrap();
        let matches = re.captures_iter(&response);
        let mut result = Vec::new();
        for match_ in matches {
            result.push(match_.get(0).unwrap().as_str().to_string());
        }
        // 去除result数组中重复的元素
        result.sort();
        result.dedup();
        // print!("result:{:?}", result);
        Ok(result)
    } else {
        Err("Failed to fetch data".into())
    }
}

// 使用match_data函数的返回值作为参数，并发调用fetche_data函数，获取网页内容
#[tauri::command]
async fn fetch_data_by_match(url: String) -> Result<Vec<Picture>, String> {
    let matches = match_data(url).await;
    if let Ok(matches) = matches {
        let mut tasks = Vec::new();
        for match_ in matches {
            let task = fetch_data(format!("http://dkleh8.xyz/pw/{}", match_)).await;
            tasks.push(task);
        }
        let mut pictures: Vec<Picture> = Vec::new();
        for index in 0..tasks.len() {
            if let Ok(response) = &tasks[index] {
                // responses.push(response);
                let picture = match_img_data(&response, index);
                print!("picture:{:#?}", picture);
                pictures.push(picture);
            }
        }
        Ok(pictures)
    } else {
        Err("Failed to match data".into())
    }
}

#[derive(Debug, serde::Serialize)]
struct Picture {
    id: usize,
    title: String,
    url: String,
    srcs: Vec<ImgDetail>,
    star: usize,
    collect: bool,
    download: bool,
    delete: bool,
}

#[derive(Debug, serde::Serialize)]
struct ImgDetail {
    src: String,
    aspect_ratio: f32,
}

use chrono::Local;
// 生成随机id
fn new_id() -> usize {
    let now = Local::now();
    let timestamp = now.timestamp();
    timestamp as usize
}

use image::GenericImageView;
// use image::DynamicImage;

fn match_img_data(html: &str, index: usize) -> Picture {
    let pattern = r"https://pic2303[a-z].[a-z]{2,}/i/([^<>']*)\.jpg";
    let pattern1 = r#"<span id="subject_tpc">(.*?)</span>"#;

    let re = Regex::new(pattern).unwrap();
    let re1 = Regex::new(pattern1).unwrap();
    let title = re1
        .captures_iter(html)
        .next()
        .unwrap()
        .get(1)
        .unwrap()
        .as_str()
        .to_string();

    print!("title:{}", title);
    let matches = re.captures_iter(html);
    let mut result = Vec::new();
    for match_ in matches {
        let src = match_.get(0).unwrap().as_str().to_string();
        // 根据src获取图片宽高

        let img = image::open(&src).unwrap();
        let (width, height) = img.dimensions();
        let aspect_ratio = width as f32 / height as f32;
        let img_detail = ImgDetail { src, aspect_ratio };

        result.push(img_detail);

        // print!("match:{:#?}",img_detail);
    }
    // // 去除result数组中重复的元素
    // result.sort();
    // result.dedup();

    let id = new_id() + index;
    let srcs = result;
    let star = 0;
    let collect = false;
    let download = false;
    let delete = false;

    Picture {
        id,
        title,
        url: "http://dkleh8.xyz/pw/thread.php?fid=14".to_string(),
        srcs,
        star,
        collect,
        download,
        delete,
    }
}

// the payload type must implement `Serialize` and `Clone`.
#[derive(Clone, serde::Serialize)]
struct Payload {
    message: String,
}

// init a background process on the command, and emit periodic events only to the window that used the command
#[tauri::command]
fn init_process(window: Window) {
    std::thread::spawn(move || loop {
        window
            .emit(
                "event-name",
                Payload {
                    message: "Tauri is awesome!".into(),
                },
            )
            .unwrap();
    });
}

fn main() {
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let close = CustomMenuItem::new("close".to_string(), "Close");
    let info = CustomMenuItem::new("info".to_string(), "Info");
    let submenu = Submenu::new("File", Menu::new().add_item(quit).add_item(close));
    let submenu1 = Submenu::new("About", Menu::new().add_item(info));
    let menu = Menu::new()
        .add_native_item(MenuItem::Copy)
        .add_item(CustomMenuItem::new("hide", "Hide"))
        .add_submenu(submenu)
        .add_submenu(submenu1);
    // 目标网页的URL
    // let url = "http://dkleh8.xyz/pw/thread.php?fid=14";
    tauri::Builder::default()
        .setup(|app| {
            // 根据label获取窗口实例
            let main_window = app.get_window("main").unwrap();
            // 设置窗口模糊特效
            #[cfg(target_os = "macos")]
            apply_vibrancy(&main_window, NSVisualEffectMaterial::Sidebar, None, None)
                .expect("Unsupported platform! 'apply_vibrancy' is only supported on macOS");

            let id = main_window.listen("event-name", |event| {
                println!("got window event-name with payload {:?}", event.payload());
            });
            // unlisten to the event using the `id` returned on the `listen` function
            // an `once` API is also exposed on the `Window` struct
            main_window.unlisten(id);

            // emit the `event-name` event to the `main` window
            main_window
                .emit(
                    "event-name",
                    Payload {
                        message: "Tauri is awesome!".into(),
                    },
                )
                .unwrap();

            Ok(())
        })
        .menu(menu)
        .invoke_handler(tauri::generate_handler![fetch_data_by_match,init_process])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
