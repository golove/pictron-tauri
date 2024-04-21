// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

// use chrono::Duration;
use tauri::{CustomMenuItem, Manager, Menu, MenuItem, Submenu, Window};
use window_vibrancy::{apply_vibrancy, NSVisualEffectMaterial};
mod spider;
use spider::{Database, Picture, Spider};
use std::time::{Duration, Instant};

use serde::Serialize;

#[derive(Serialize)]
struct SpiderResult {
    pictures: Vec<Picture>,
    duration: u64,
}

#[tauri::command]
fn spider_img(url: String) -> SpiderResult {
    let start_time = Instant::now();
    let spider = Spider::new(&url);
    // let spider = Spider::generate_test_data();
    
    

    // 等待一秒，模拟网络请求
    // std::thread::sleep(Duration::from_secs(1));
    

    let end_time = Instant::now();
    // 计算执行时间
    let duration = (end_time - start_time).as_secs();
    
    match spider {
        Ok(s) => {
            let pictures = Spider::get_pictures(s);
            SpiderResult {
                pictures,
                duration,
            }
        }
        Err(e) => {
            println!("spider error: {}", e);
            SpiderResult {
                pictures: vec![],
                duration,
            }
        }
    }
    
}

#[tauri::command]
fn get_data_from_db() -> Vec<Picture> {
    let db = Database::new("picture.db").expect("Failed to create database");
   match Database::get_all_pictures(&db){
       Ok(pictures) => pictures,
       Err(e) => {
           println!("get data from db error: {}", e);
           vec![]
       }
    }
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
           

            Ok(())
        })
        .menu(menu)
        .invoke_handler(tauri::generate_handler![spider_img,get_data_from_db])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
