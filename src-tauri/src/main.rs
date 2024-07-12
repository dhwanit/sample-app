// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{Manager, Window};

#[derive(Clone, serde::Serialize)]
struct Payload {
    data: String,
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![login, add_margin])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
async fn login(api_key: String, username: String, password: String, mfa_encoded_secret: String, window: Window, app_handle: tauri::AppHandle) {
    // TODO login related work
    // Steps 1-4
    // ...

    // Emit event(s)
    window.emit("request-token", Payload { data: "<actual_request_token>".into() }).unwrap();
    window.emit("encoded-token", Payload { data: "<actual_encoded_token>".into() }).unwrap();
}

#[tauri::command]
async fn add_margin(margin_amount: i32, window: Window, app_handle: tauri::AppHandle) {
    // TODO add_margin related work
    // Steps 1-3
    // ...

    // Emit event(s)
    window.emit("add-margin-complete", {}).unwrap();
    // ...also for encoded-token if present
}
