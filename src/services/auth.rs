use serde_json::{self, Value};
use dioxus::prelude::*;
use crate::services::services::UserInfo;
use wasm_bindgen::prelude::*;
use web_sys::{window, Document, console};
use js_sys::Date;
use reqwest::{header, Client};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::{info, error, Level};
use serde_urlencoded;
use std::error::Error;
use futures::future::join_all;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    #[wasm_bindgen(js_namespace = document)]
    fn cookie(s: &str);
}

fn set_cookie(name: &str, value: &str, days: i64) {
    let window = window().expect("should have a Window");
    let expiration = Date::new_0().get_time() + (days as f64) * 24.0 * 60.0 * 60.0 * 1000.0;
    let expires = Date::new(&JsValue::from_f64(expiration)).to_iso_string();

    let cookie_value = format!("{}={}; expires={}; path=/", name, value, expires.as_string().unwrap());

    // Set the cookie via direct JavaScript interop
    unsafe {
        cookie(&cookie_value);
    }
}

fn remove_cookie(name: &str) {
    // Set the cookie with an expired date
    let cookie_value = format!("{}=; expires=Thu, 01 Jan 1970 00:00:00 GMT; path=/", name);
    unsafe {
        cookie(&cookie_value);
    }
}

#[cfg(target_arch = "wasm32")]
pub fn get_auth_code() -> Option<String> {
    let window = web_sys::window()?;
    let location = window.location();
    let query = location.search().ok()?;
    if query.len() > 1 {
        let params: HashMap<String, String> = serde_urlencoded::from_str(&query[1..]).ok()?;
        params.get("code").cloned()
    } else {
        None
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub fn get_auth_code() -> Option<String> {
    None
}

pub static USER_INFO: GlobalSignal<Option<UserInfo>> = Signal::global(|| None);
pub static LOADING: GlobalSignal<bool> = Signal::global(|| false);

pub async fn sign_in() {
    *LOADING.write() = true;
    let client = reqwest::Client::new();
    info!("Sending HTTP request to auth endpoint");
    let result = client.post("http://localhost:8000/auth/google").send().await;
    info!("HTTP Request sent");

    match result {
        Ok(response) => {
            if response.status().is_success() {
                match response.json::<HashMap<String, String>>().await {
                    Ok(data) => {
                        if let Some(url) = data.get("url") {
                            log(&format!("Redirecting to: {}", url)); // Log URL
                            web_sys::window()
                                .unwrap()
                                .location()
                                .set_href(url)
                                .expect("failed to redirect");
                        } else {
                            error!("URL not found in response");
                            *USER_INFO.write() = None;
                        }
                    },
                    Err(e) => {
                        error!("Failed to deserialize response: {}", e);
                        *USER_INFO.write() = None;
                    }
                }
            } else {
                error!("Failed to sign in. Status: {}", response.status());
                *USER_INFO.write() = None;
            }
        },
        Err(e) => {
            error!("HTTP request had a problem: {}", e);
            *USER_INFO.write() = None;
        }
    }
    *LOADING.write() = false;
}
// pub fn sign_in() {
//     use_future(|| {
//         async move {
//             *LOADING.write() = true;
//             let client = Client::new();
//             info!("Sending HTTP request to auth endpoint");
//             let result = client.post("http://localhost:8000/auth/google").send().await;
//             info!("HTTP Request sent");

//             info!("HTTP Request Result: {:?}", result);
//             match result {
//                 Ok(response) => {
//                     if response.status().is_success() {
//                         match response.json::<HashMap<String, String>>().await {
//                             Ok(data) => {
//                                 if let Some(url) = data.get("url") {
//                                     log(&format!("Redirecting to: {}", url)); // Log URL
//                                     web_sys::window()
//                                         .unwrap()
//                                         .location()
//                                         .set_href(url)
//                                         .expect("failed to redirect");
//                                 } else {
//                                     error!("URL not found in response");
//                                     *USER_INFO.write() = None;
//                                 }
//                             },
//                             Err(e) => {
//                                 error!("Failed to deserialize response: {}", e);
//                                 *USER_INFO.write() = None;
//                             }
//                         }
//                     } else {
//                         error!("Failed to sign in. Status: {}", response.status());
//                         *USER_INFO.write() = None;
//                     }
//                 },
//                 Err(e) => {
//                     error!("HTTP request had a problem: {}", e);
//                    *USER_INFO.write() = None;
//                 }
//             }
//             *LOADING.write() = false;
//         }
//     });
// }

pub fn log_out() {
    *USER_INFO.write() = None;
    remove_cookie("user_info");
    web_sys::window().unwrap().location().set_href("/signin").unwrap();
}

pub async fn fetch_user_info(code: String) -> Result<Option<UserInfo>, Box<dyn Error>> {
    if code.is_empty() {
        info!("Authorization code is empty, aborting fetch.");
        return Ok(None);
    }

    info!("Fetching user information with code: {}", code);
    let client = Client::new();
    let response = client
        .get(&format!("http://localhost:8000/auth/google/callback?code={}", code))
        .send()
        .await?;

    if response.status().is_success() {
        let json: Value = response.json().await?;
        if json["status"] == "success" {
            let user_info = UserInfo {
                id: json["data"]["id"].to_string().replace("\"", ""),
                email: json["data"]["email"].to_string().replace("\"", ""),
                name: json["data"]["name"].as_str().unwrap_or_default().to_string(),
                verified_email: Some(json["data"]["verified_email"].to_string().replace("\"", "")),
                given_name: Some(json["data"]["given_name"].as_str().unwrap_or_default().to_string()),
                family_name: Some(json["data"]["family_name"].as_str().unwrap_or_default().to_string()),
                picture: Some(json["data"]["picture"].as_str().unwrap_or_default().to_string()),
                locale: Some(json["data"]["locale"].as_str().unwrap_or_default().to_string())
            };
            *USER_INFO.write() = Some(user_info.clone());
            set_cookie("user_info", &serde_json::to_string(&user_info)?, 14);
            Ok(Some(user_info))
        } else {
            Ok(None)
        }
    } else {
        Err("Failed to fetch user details from the server".into())
    }
}
