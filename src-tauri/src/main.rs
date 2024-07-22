// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::borrow::{Borrow, BorrowMut};
use std::thread;
use std::time::Duration;

use async_std::stream::StreamExt;
use headless_chrome::protocol::cdp::Page;
use headless_chrome::protocol::cdp::Target::CreateTarget;
use headless_chrome::{browser, Browser, LaunchOptions};

use regex::Regex;
use std::sync::Arc;
use tauri::{window, State, Window};
use tokio::sync::Mutex;

mod lib;
use lib::handle_margin;
use lib::request_token;

#[derive(Clone, serde::Serialize)]
struct Payload {
    data: String,
}

struct BrowserState(Arc<Mutex<Browser>>);

#[tauri::command]
async fn login(
    browser_sate: tauri::State<'_, BrowserState>,
    api_key: String,
    username: String,
    password: String,
    mfa_encoded_secret: String,
    window: Window,
    app_handle: tauri::AppHandle,
) -> Result<String, String> {
    // TODO login related work
    // Steps 1-4
    let browser = browser_sate.0.lock().await.clone();

    let token: Result<(String, String), Box<dyn std::error::Error>> = request_token(
        browser,
        api_key.clone(),
        username.clone(),
        password.clone(),
        mfa_encoded_secret.clone(),
    )
    .await;
    match token {
        Ok(token) => {
            // Emit event(s)
            window
                .emit(
                    "request-token",
                    Payload {
                        // get the first element of the tuple => request_token
                        data: token.0.clone().into(),
                    },
                )
                .unwrap();
            window
                .emit(
                    "encoded-token",
                    Payload {
                        // get the second element of the tuple => encoded_token
                        data: token.1.clone().into(),
                    },
                )
                .unwrap();
        }
        Err(err) => {
            // Emit event(s)
            println!("Error: {:?}", err);
        }
    }
    Ok("The token has been requested".to_string())
}

#[tauri::command]
async fn add_margin(
    browser_sate: tauri::State<'_, BrowserState>,
    margin_amount: i32,
    window: Window,
    app_handle: tauri::AppHandle,
) -> Result<String, String> {
    // TODO add_margin related work
    // Steps 1-3

    let browser = browser_sate.0.lock().await.clone();

    let margin = handle_margin(browser, margin_amount).await;
    match margin {
        Ok(margin) => {
            // Emit event(s)
            window
                .emit(
                    "add-margin-complete",
                    Payload {
                        data: margin.to_string(),
                    },
                )
                .unwrap();
        }
        Err(err) => {
            // Emit event(s)
            println!("Error: {:?}", err);
        }
    }

    return Ok(String::new());
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Launching the browser with head mode
    // Setting the options to run the headless browser
    let launch_options = LaunchOptions::default_builder()
        .ignore_certificate_errors(true)
        .headless(false)
        .idle_browser_timeout(Duration::from_secs(60)) // 1 minute time out for secure operation
        .build()?;
    let browser: Browser = Browser::new(launch_options).unwrap();

    tauri::Builder::default()
        .manage(BrowserState(Arc::new(Mutex::new(browser))))
        .invoke_handler(tauri::generate_handler![login, add_margin])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}
