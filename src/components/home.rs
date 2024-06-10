use dioxus::prelude::*;
use crate::services::services::{use_user};
use crate::services::auth::{log_out, get_auth_code, fetch_user_info, LOADING};
use web_sys;

#[component]
pub fn Home() -> Element {
    use_future({
        move || async move {
            if let Some(code) = get_auth_code() {
                let _ = fetch_user_info(code).await;
            }
        }
    });

    let user = use_user();
    
    rsx! {
        div {
            class: "flex flex-col items-center justify-center min-h-screen bg-gray-100 text-lg text-gray-800",
            if let Some(user) = user {
                div {
                    {format!("Logged in as: {}, Email: {}", user.name, user.email)}
                }
                button {
                    class: "px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-700",
                    onclick: |_| log_out(),
                    "Logout"
                }
            } else {
                div {
                    class: "text-red-500",
                    "Not logged in"
                }
                button {
                    class: "px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-700",
                    onclick: move |_| {
                        // Change this URL to your sign-in page URL
                        let sign_in_url = "http://localhost:8080/signin";
                        web_sys::window().unwrap().location().set_href(sign_in_url).unwrap();
                    },
                    "Sign In"
                }
            }
        }
    }
}
