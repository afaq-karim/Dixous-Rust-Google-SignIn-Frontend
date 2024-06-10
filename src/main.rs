#![allow(non_snake_case)]

use dioxus::prelude::*;
use crate::components::home::Home;
use crate::components::sign_in::SignIn;
use crate::services::auth::{get_auth_code, fetch_user_info};
use tracing::{info, error, Level};
use web_sys;

mod components;
mod services;

#[derive(Clone, Debug, PartialEq, Routable)]
#[rustfmt::skip]
enum Route {
    #[route("/")]
    Home,

    #[route("/signin")]
    SignIn,
}

fn main() {
    dioxus_logger::init(Level::INFO).expect("Failed to init logger");
    launch(App);
}

fn App() -> Element {
    let code = use_signal(|| Some(get_auth_code()));
    
    let auth_resource = use_resource(move || {
        if let Some(auth_code) = code.cloned().flatten() {
            fetch_user_info(auth_code)
        } else {
            fetch_user_info("".to_string())
        }
    });

    match &*auth_resource.read_unchecked() {
        Some(Ok(result)) => {
            rsx! {
                div {
                     Router::<Route> { }
                }
            }
        }
        Some(Err(err)) => {
            rsx! { div { "An error occurred: {err}" } }
        }
        None => {
            rsx! { div { class: "loading", "Please wait while the items are loading..." } }
        }
    }
}
