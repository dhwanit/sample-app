// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use headless_chrome::{Browser, LaunchOptions};
use std::thread;
use std::time::Duration;

use std::sync::Arc;
use tauri::Window;
use tokio::sync::Mutex;

extern crate app;
use app::handle_margin;
use app::request_token;

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
    // wait some time for the browser to start
    thread::sleep(Duration::from_millis(300));
    // Close the open tabs for the
    let custom_tabs = [
        "about:blank",
        "chrome://newtab/",
        "http://ec2-3-110-151-1.ap-south-1.compute.amazonaws.com/tmp/simulate/login.html",
        "http://ec2-3-110-151-1.ap-south-1.compute.amazonaws.com/tmp/simulate/margin.html",
        "http://ec2-3-110-151-1.ap-south-1.compute.amazonaws.com/tmp/simulate/session.html",
        "http://ec2-3-110-151-1.ap-south-1.compute.amazonaws.com/tmp/simulate/popup.html",
        "http://ec2-3-110-151-1.ap-south-1.compute.amazonaws.com/tmp/simulate/2fa.html",
    ];
    for tab in browser.clone().get_tabs().lock().unwrap().iter() {
        if custom_tabs.contains(&tab.get_url().as_str()) {
            // wait some time for the tab to close
            thread::sleep(Duration::from_secs(1));
            tab.close(true)?;
        }
    }

    tauri::Builder::default()
        .manage(BrowserState(Arc::new(Mutex::new(browser.clone()))))
        .invoke_handler(tauri::generate_handler![login, add_margin])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    Ok(())
}
