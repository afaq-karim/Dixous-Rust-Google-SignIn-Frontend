use dioxus::prelude::*;
use crate::services::auth::{sign_in, LOADING};

#[component]
pub fn SignIn() -> Element {
    rsx! {
        div {
            class: "flex flex-col items-center justify-center min-h-screen bg-gray-50 p-6",
            div {
                class: "space-y-4",
                button {
                    class: "px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-700",
                    onclick: move |_| sign_in(),
                    "Sign In With Google"
                },
                div {
                    class: "mt-2",
                    if *LOADING.read() {
                        div { "Loading..." }
                    }
                }
            }
        }
    }
}