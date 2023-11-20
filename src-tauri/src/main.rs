// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod aimedb;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
async fn get_aimeid(host: &str, access_code: &str) -> Result<i32, String> {
    Ok(aimedb::get_aimeid(host, access_code)
        .await
        .map_err(|e| e.to_string())?)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_aimeid])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
