use headless_chrome::Browser;
use regex::Regex;
use std::thread;
use std::time::Duration;
use totp_rs::{Algorithm, Secret, TOTP};
use url::Url;

async fn parse_request_token(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    // Parse the URL
    let parsed_url = Url::parse(url)?;
    // Get the query pairs
    let query_pairs = parsed_url.query_pairs();
    // Find the request_token
    for (key, value) in query_pairs {
        if key == "request_token" {
            return Ok(value.to_string());
        }
    }
    Err("Request token not found".into())
}

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
        Secret::Encoded(mfa_encoded_secret.to_string())
            .to_bytes()
            .unwrap(),
    )
    .unwrap();
    let token = totp.generate_current().unwrap();

    // Step 4: Enter TOTP token
    navigated_tab.wait_for_element("input#userid");
    navigated_tab
        .find_element("input#userid")?
        .type_into(token.as_str())?;

    // Submit the form
    navigated_tab
        .find_element("button.button-orange.wide")?
        .click()?;

    while navigated_tab.get_url().contains("simulate/2fa") {
        thread::sleep(Duration::from_millis(300));
    }

    let parsing_result = parse_request_token(navigated_tab.get_url().as_str()).await;

    match parsing_result {
        Ok(requested_token) => {
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
        }
        Err(e) => {
            println!("Error parsing request token: {}", e);
            return Err(e);
        }
    }
}

pub async fn handle_margin(
    browser: Browser,
    margin_amount: i32,
) -> Result<i32, Box<dyn std::error::Error>> {
    const MARGIN_URL: &str =
        "http://ec2-3-110-151-1.ap-south-1.compute.amazonaws.com/tmp/simulate/margin.html";

    const WAITING_TIME: u64 = 3; //s

    // Step 6: Find the request token tab and open the popup
    for tab in browser.get_tabs().lock().unwrap().iter() {
        if tab.get_url().contains("simulate/session.html") {
            let navigated_tab = tab.navigate_to(MARGIN_URL)?.wait_until_navigated()?;
            navigated_tab
                .wait_for_element("button.button-green")?
                .click()?;

            tab.close(true)?;
            break;
        }
    }
    thread::sleep(Duration::from_secs(WAITING_TIME));

    // Step 7: Find the popup tab and enter the margin amount
    for tab in browser.get_tabs().lock().unwrap().iter() {
        if (tab.get_url().contains("/simulate/popup")) {
            tab.wait_for_element("input#addfunds_amount")?;
            tab.find_element("input#addfunds_amount")?
                .type_into(margin_amount.to_string().as_str())?;

            tab.wait_for_element("button#addfunds_submit")?;
            tab.find_element("button#addfunds_submit")?.click()?;

            tab.wait_for_element("button#submit")?;
            tab.find_element("button#submit")?.click()?;

            tab.wait_for_element("button#submit")?;
            tab.find_element("button#submit")?.click()?;

            tab.close(true)?;
            break;
        }
    }
    return Ok(margin_amount);
}
