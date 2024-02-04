// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]


#[tauri::command]
fn greet(name: &str) -> String {
   format!("Hello, {}!", name)
}

#[tauri::command]
fn spider()->String{
format!("spider successful")
}


fn main() {
  // 目标网页的URL
  // let url = "http://dkleh8.xyz/pw/thread.php?fid=14";

  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![greet,spider])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");

  
}