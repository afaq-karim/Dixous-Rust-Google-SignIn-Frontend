use dioxus::prelude::*;
use crate::services::auth::{get_auth_code, fetch_user_info};
use tracing::{info, error, Level};
use web_sys::{window, console, UrlSearchParams};

#[component]
pub fn AuthCallback(segments: Vec<String>) -> Element {
    let segments = segments.clone();
    let location = window().unwrap().location();
    let href = location.href().unwrap();
    let search = location.search().unwrap();
    let search_params = UrlSearchParams::new_with_str(&search).unwrap();

    // Extract parameters immediately
    let code = search_params.get("code").unwrap_or_default();
    let state = search_params.get("state").unwrap_or_default();

    // Log captured parameters
    info!("Code: {:?}", segments);
    info!("State: {:?}", state);
    info!("URL: {:?}", location.href().unwrap());

    use_future({
        let href_clone = href.clone();
        move || {
            let href_clone = href_clone.clone();
            let location_clone = location.clone();
            async move {
                if let Some(code) = extract_code_from_href(&href_clone) {
                    if fetch_user_info(code).await.is_ok() {
                        let new_url = href_clone.split('?').next().unwrap_or("").to_string();
                        location_clone.set_href(&new_url).unwrap();
                    } else {
                        error!("Login failed!");
                    }
                }
            }
        }
    });

    rsx! {
        div {
            class: "text-center",
            "Processing login..."
        }
    }
}

fn extract_code_from_href(href: &str) -> Option<String> {
    href.split('?').nth(1)?.split('&')
        .find_map(|kv| {
            let mut parts = kv.split('=');
            match (parts.next(), parts.next()) {
                (Some("code"), Some(code)) => Some(code.to_owned()),
                _ => None,
            }
        })
}
