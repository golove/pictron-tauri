// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use cocoa::{
    appkit::{NSWindow, NSWindowToolbarStyle},
    base::id,
};
use objc::{class, msg_send, sel, sel_impl};

use state::{AppState, ServiceAccess};
// use chrono::Duration;
use tauri::{
    api::dir, window, AppHandle, CustomMenuItem, Manager, Menu, MenuItem, Result, State, Submenu,
    Window,
};
// tauri::api::file;

use window_vibrancy::{apply_vibrancy, NSVisualEffectMaterial};
mod spider;
use spider::Spider;

mod asyncSpider;
use scraper::Selector;

use std::{
    fs::File,
    io::Write,
    path::{self, PathBuf},
    time::{Duration, Instant},
};

use serde::{Deserialize, Serialize};

mod database;
mod state;

// mod async_spider;

// #[tauri::command]
// async fn read_file(path: String) -> Result<String, String> {
//     // 读取文件内容
//     let contents = tauri::api::file::read_text_file(&path).map_err(|e| e.to_string())?;
//     Ok(contents)
// }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImgDetail {
    pub src: String,
    pub aspect_ratio: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Picture {
    pub id: u32,
    pub title: String,
    pub url: String,
    pub srcs: Vec<ImgDetail>,
    pub star: u8,
    pub collect: bool,
    pub download: bool,
    pub deleted: bool,
}

#[derive(Serialize)]
struct SpiderResult {
    pictures: Vec<Picture>,
    duration: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PictronConfig {
    folderpath: String,
    pictureurl: String,
}

// #[tauri::command]
//(window: Window,app_handle: AppHandle, url: String) -> u64 {
//     let start_time = Instant::now();

//         app_handle.db( |db|{
//             async_spider::spider(&window,db,&url);
//         });

//     let end_time = Instant::now();
//     // 计算执行时间
//     let duration = (end_time - start_time).as_secs();

//     duration
// }
#[tauri::command]
async fn async_spider(app_handle: AppHandle, window: Window, url: String) -> u64 {
    let start_time = Instant::now();
    let db = database::Database::new(&app_handle);

    match asyncSpider::get_pictures(db, window, url).await {
        Ok(_) => {
            let end_time = Instant::now();
            let duration = (end_time - start_time).as_secs();
            duration
        }
        Err(e) => {
            let end_time = Instant::now();
            println!("spider error: {}", e);
            let duration = (end_time - start_time).as_secs();
            duration
        }
    }
}
// match asyncSpider::get_pictures(window,url).await{
//     Ok(_)=>{
//         let end_time = Instant::now();
//         // 计算执行时间
//         let duration = (end_time - start_time).as_secs();
//         duration
//     }
//     Err(e)=>{
//         println!("spider error: {}", e);
//         let end_time = Instant::now();
//         // 计算执行时间
//         let duration = (end_time - start_time).as_secs();
//         duration
//     }
// }

// }

#[tauri::command]
fn spider_img(app_handle: AppHandle, url: String) -> SpiderResult {
    let start_time = Instant::now();

    let db = database::Database::new(&app_handle);

    

    match Spider::new(db, &url) {
        // match app_handle.db(|db| Spider::generate_test_data(db)) {
        Ok(s) => {
            let pictures = Spider::get_pictures(s);

            let end_time = Instant::now();
            // 计算执行时间
            let duration = (end_time - start_time).as_secs();

            SpiderResult { pictures, duration }
        }
        Err(e) => {
            println!("spider error: {}", e);
            SpiderResult {
                pictures: vec![],
                duration: 0,
            }
        }
    }
}

#[tauri::command]
fn update_db(app_handle: AppHandle, id: i64, sql: String) {
    // print!("{}", sql);
    let db = database::Database::new(&app_handle);
    match db.update_picture(&sql) {
        Ok(_) => println!("update db success id{}", id),
        Err(e) => println!("update db error: {}", e),
    }
}
#[tauri::command]
fn select_from_db(app_handle: AppHandle, sql: String) -> Vec<Picture> {
    let db = database::Database::new(&app_handle);
    match db.select_picture(&sql) {
        Ok(pictures) => pictures,
        Err(e) => {
            println!("select db error: {}", e);
            vec![]
        }
    }
}

#[derive(Clone, serde::Serialize)]
struct Payload {
    msg: u32,
}

#[tauri::command]
fn insert_config(app_handle: AppHandle, path: String) -> String {
    let url = "http://dkleh8.xyz/pw/thread.php?fid=14";
    let db = database::Database::new(&app_handle);

    let sql = format!(
        "INSERT INTO pictron_config (folderpath, pictureurl) VALUES ('{}','{}')",
        &path, url
    );
    match db.inset_config(&sql) {
        Ok(_) => "ok".to_string(),
        Err(e) => e.to_string(),
    }
}

#[tauri::command]
fn get_folder_path(app_handle: AppHandle) -> bool {
    let db = database::Database::new(&app_handle);
 
        let sql = "SELECT folderpath FROM pictron_config WHERE id = 1";
        match db.select_config( &sql) {
            Ok(_) => true,
            Err(e) => false,
        }

}

fn home_dir(app_handle: AppHandle) -> PathBuf {
    let db = database::Database::new(&app_handle);
    let sql = "SELECT folderpath FROM pictron_config WHERE id = 1";
    match db.select_config(&sql) {
        Ok(row) => {
            let path = path::Path::new(&row);
            path.to_path_buf()
        }
        Err(e) => {
            println!("select db error: {}", e);
            let path = tauri::api::path::home_dir().unwrap().join("Downloads");
            let dbpath = path.clone().to_str().unwrap().to_string();
            let url = "http://dkleh8.xyz/pw/thread.php?fid=14";
            let sql = format!(
                "INSERT INTO pictron_config (folderpath, pictureurl) VALUES ('{}','{}')",
                &dbpath, url
            );
            match db.inset_config(&sql) {
                Ok(_) => println!("insert db success"),
                Err(e) => println!("insert db error: {}", e),
            }
            path
        }
    }
}

use tokio::spawn;

#[tauri::command]
async fn download_method(window:Window,srcs:Vec<ImgDetail>,title: String,id:u32){
   let mut index = 1;
    for src in srcs{
        let index_clone = index;
        let window_clone = window.clone();
       // 模拟等待300ms
      std::thread::sleep(Duration::from_millis(300));
     
       let event_name = "download-success".to_string() + id.to_string().as_str();
       let _ = window_clone.emit(&event_name, Payload { msg: index_clone });
        index += 1;
    }
}

#[tauri::command]
async fn download_img(
    app_handle: AppHandle,
    window: Window,
    srcs: Vec<ImgDetail>,
    title: String,
    id: u32,
) {
    // 选择下载目录
    let home_dir = home_dir(app_handle);
    let download_dir = home_dir.join(&title);
    if !download_dir.exists() {
        match std::fs::create_dir(&download_dir) {
            Ok(_) => {
                let mut index = 1;
                // 并发下载图片
                // let mut handles = vec![];
                for src in srcs {
                    let window_clone = window.clone();
                    let index_clone = index;
                    let url = src.src.clone();
                    let file_name = &url.split('/').last().unwrap();
                    let file_path = download_dir.join(file_name);
                    let file = File::create(&file_path).unwrap();
                    let handle = spawn(async move {
                        download(&url, file);
                        let event_name = "download-success".to_string() + id.to_string().as_str();
                        let _ = window_clone.emit(&event_name, Payload { msg: index_clone });
                    });

                    handle.await.unwrap();
                    index += 1;
                }
              
            }

            Err(e) => {
                print!("create dir error: {}", e);
                // format!("create dir error: {}", e)
            }
        }
    } else {
        print!("{} already exists", download_dir.to_str().unwrap());
        // format!("{} already exists", download_dir.to_str().unwrap())
    }
}

fn download(url: &str, mut file: File) -> String {
    match reqwest::blocking::get(url) {
        Ok(mut response) => {
            // 保存图片到指定目录

            let mut buffer = vec![];
            response.copy_to(&mut buffer).unwrap();
            file.write_all(&buffer).unwrap();
            // println!("download success");

            format!("download success")
        }
        Err(e) => {
            println!("download error: {}", e);
            return "download error".to_string();
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
        .manage(AppState {
            db: Default::default(),
        })
        .menu(menu)
        .invoke_handler(tauri::generate_handler![
            spider_img,
            update_db,
            select_from_db,
            download_img,
            insert_config,
            get_folder_path,
            async_spider,
            download_method
        ])
        .setup(|app| {
            // let handle = app.handle();

            // let app_state: tauri::State<AppState> = handle.state();
            // let mut db =
            //     database::initialize_database(&handle).expect("Database initialize should succeed");
            // *app_state.db.lock().unwrap() = Some(db);

            // 根据label获取窗口实例
            let main_window = app.get_window("main").unwrap();

            // 设置窗口模糊特效
            #[cfg(target_os = "macos")]
            apply_vibrancy(&main_window, NSVisualEffectMaterial::Sidebar, None, None)
                .expect("Unsupported platform! 'apply_vibrancy' is only supported on macOS");

            // 添加toolbar让"红绿灯"看起来更自然
            let ns_window = main_window.ns_window().unwrap() as id;
            unsafe {
                ns_window.setToolbar_(msg_send![class!(NSToolbar), new]);
                ns_window
                    .setToolbarStyle_(NSWindowToolbarStyle::NSWindowToolbarStyleUnifiedCompact);
            }

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
