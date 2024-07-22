use headless_chrome::browser::tab;
use headless_chrome::protocol::cdp::Page;
use headless_chrome::protocol::cdp::Target::CreateTarget;
use headless_chrome::{browser, Browser, LaunchOptions};
use regex::Regex;
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use tokio::sync::Mutex;
use totp_rs::{Algorithm, Secret, TOTP};

// Login with TOTP
// Using the chromiumoxide library to handle the login process for the website
pub async fn request_token(
    browser: Browser,
    api_key: String,
    username: String,
    password: String,
    mfa_encoded_secret: String,
) -> Result<(String, String), Box<dyn std::error::Error>> {
    const LOGIN_URL: &str =
        "http://ec2-3-110-151-1.ap-south-1.compute.amazonaws.com/tmp/simulate/login.html";
    const WAITING_TIME: u64 = 3; //s
    let new_tab = browser.new_tab()?;
    let navigated_tab = new_tab.navigate_to(LOGIN_URL)?.wait_until_navigated()?;

    // Wait for the page to load
    // Step 2: Enter username and password
    navigated_tab
        .find_element("input#userid")?
        .type_into(username.as_str())?;

    navigated_tab
        .find_element("input#password")?
        .type_into(password.as_str())?;

    navigated_tab.find_element("button")?.click()?;

    // Step 3: Generate TOTP token
    let totp = TOTP::new(
        Algorithm::SHA1,
        6,
        1,
        30,
        Secret::Raw("TestSecretSuperSecret".as_bytes().to_vec())
            .to_bytes()
            .unwrap(),
    )
    .unwrap();
    let token = totp.generate_current().unwrap();

    // Step 4: Enter TOTP token
    navigated_tab.wait_for_element("input#userid");
    navigated_tab
        .find_element("input#userid")?
        .type_into(mfa_encoded_secret.as_str())?;

    // Submit the form
    navigated_tab
        .find_element("button.button-orange.wide")?
        .click()?;
    navigated_tab.wait_for_element("strong#request_token");

    let requested_token = navigated_tab
        .find_element("strong#request_token")?
        .get_inner_text()?;

    // Step 5: Retrieve the token from localStorage
    // Define the JavaScript to retrieve the token from localStorage
    let script = r#"
     (function() {
         return localStorage.getItem('__storejs_dpmorgan_enctoken');
     })();
 "#; // Evaluate the JavaScript in the context of the page
    let encoded_token = navigated_tab
        .evaluate(script, true)?
        .value
        .unwrap()
        .to_string();

    // Check if the token exists
    if !requested_token.is_empty() {
        if !encoded_token.is_empty() {
            // Check if the token matches the expected format
            //let encoded_token = encoded_token.trim();
            let regex = Regex::new(r"^.{1,}$").unwrap(); // Assuming TOTP tokens are 21-character strings

            // Match the token against the regex
            if regex.is_match(encoded_token.clone().as_str()) {
                return Ok((requested_token, encoded_token));
            } else {
                // Returning the error
                return Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "Invalid encoded token found in the localStorage",
                )));
            }
        } else {
            // Returning the error
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                "No encoded token found in the localStorage",
            )));
        }
    } else {
        // Returning the error
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Could not find the requested token in the page",
        )));
    }
    return Ok((String::new(), String::new()));
}

pub async fn handle_margin(
    browser: Browser,
    margin_amount: i32,
) -> Result<i32, Box<dyn std::error::Error>> {
    const MARGIN_URL: &str =
        "http://ec2-3-110-151-1.ap-south-1.compute.amazonaws.com/tmp/simulate/margin.html";

    const LOGIN_URL: &str =
        "http://ec2-3-110-151-1.ap-south-1.compute.amazonaws.com/tmp/simulate/login.html";
    const WAITING_TIME: u64 = 3; //s

    // Step 6: Find the request token tab and open the popup
    for tab in browser.get_tabs().lock().unwrap().iter() {
        if tab.get_url().contains("simulate/session.html") {
            let navigated_tab = tab.navigate_to(MARGIN_URL)?.wait_until_navigated()?;
            navigated_tab
                .wait_for_element("button.button-green")?
                .click()?;
            break;
        }
    }
    thread::sleep(Duration::from_secs(WAITING_TIME));

    // Step 7: Find the popup tab and enter the margin amount
    for tabbinger in browser.get_tabs().lock().unwrap().iter() {
        if (tabbinger.get_url().contains("/simulate/popup")) {
            tabbinger.wait_for_element("input#addfunds_amount")?;
            tabbinger
                .find_element("input#addfunds_amount")?
                .type_into(margin_amount.to_string().as_str())?;

            tabbinger.wait_for_element("button#addfunds_submit")?;
            tabbinger.find_element("button#addfunds_submit")?.click()?;

            tabbinger.wait_for_element("button#submit")?;
            tabbinger.find_element("button#submit")?.click()?;

            tabbinger.wait_for_element("button#submit")?;
            tabbinger.find_element("button#submit")?.click()?;
        }
    }
    return Ok(margin_amount);
}
