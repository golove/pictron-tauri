// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]
use tokio;
use tauri::{
 CustomMenuItem, Manager, Menu, MenuItem, Submenu
};
use window_vibrancy::{apply_vibrancy, NSVisualEffectMaterial};
use mouse_position::mouse_position::Mouse;

#[derive(serde::Serialize, Default)]
struct MousePosition {
    x: i32,
    y: i32,
}
impl MousePosition {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}
// 全局获取当前的鼠标坐标
#[tauri::command]
fn mouse_position() -> Result<MousePosition, String> {
    let position = Mouse::get_mouse_position();
    match position {
        Mouse::Position { x, y } => return Ok(MousePosition::new(x, y)),
        Mouse::Error => Err("Cannot find mouse position".to_string()),
    }
}


async fn request() -> Result<(), reqwest::Error> {
    // 目标网页的URL
    let url = "https://www.baidu.com";
  
    // 发送HTTP GET请求
    let response = reqwest::get(url).await?;
  
    // 检查响应的状态码
    if response.status().is_success() {
        // 将响应体作为字符串读取
        let body = response.text().await?;
        println!("Response body:\n{}", body);
    } else {
        println!("Request failed with status: {}", response.status());
    }
  
    Ok(())
  }

// here `"quit".to_string()` defines the menu item id, and the second parameter is the menu item label.

#[tauri::command]
fn spider()->String{
format!("spider successful")
}



#[tokio::main] 
async fn main() {

  
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
    let window = app.get_window("main").unwrap();
    // 设置窗口模糊特效
    #[cfg(target_os = "macos")]
    apply_vibrancy(&window, NSVisualEffectMaterial::Sidebar, None, None)
        .expect("Unsupported platform! 'apply_vibrancy' is only supported on macOS");
   
    Ok(())
})
    .menu(menu)
    .invoke_handler(tauri::generate_handler![spider,mouse_position])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
  

  
}



