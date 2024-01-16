// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[tauri::command]
fn greet(name: &str) -> String {
   format!("Hello, {}!", name)
}

#[tauri::command]
fn showName(name: &str) -> String {
  format!("what's up, {}!", name)
}

#[tauri::command]
fn spider()->String{
format!("spider successful")
}




fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![greet,showName,spider])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}