use serde::Serialize;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(inline_js = "
    export function getCookie(name) {
        try {
            let matches = document.cookie.match(new RegExp(
                '(?:^|; )' + name.replace(/([.$?*|{}()\\[\\]\\\\/+^])/g, '\\$1') + '=([^;]*)'
            ));
            return matches ? decodeURIComponent(matches[1]) : undefined;
        } catch (error) {
            return undefined;
        }
    }
")]
extern "C" {
    #[wasm_bindgen(catch)]
    fn getCookie(name: &str) -> Result<JsValue, JsValue>;
}

#[cfg(target_arch = "wasm32")]
fn get_cookie(name: &str) -> Option<String> {
    match getCookie(name) {
        Ok(cookie) if !cookie.is_undefined() => cookie.as_string(),
        _ => None,
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn get_cookie(name: &str) -> Option<String> {
    None
}

#[derive(Clone, Debug, Serialize)]
pub struct UserInfo {
    pub id: String,
    pub email: String,
    pub name: String,
    pub verified_email: Option<String>,
    pub given_name: Option<String>,
    pub family_name: Option<String>,
    pub picture: Option<String>,
    pub locale: Option<String>,
}

pub fn clear_query_parameters() {
    if let Some(window) = web_sys::window() {
        let location = window.location();
        let _ = location.set_search("");
        let _ = location.set_hash("");
    }
}

pub fn use_user() -> Option<UserInfo> {
    let id = get_cookie("user_id")?;
    let email = get_cookie("user_email")?;
    let name = get_cookie("user_name")?;

    Some(UserInfo {
        id,
        email,
        name,
        verified_email: get_cookie("verified_email"),
        given_name: get_cookie("given_name"),
        family_name: get_cookie("family_name"),
        picture: get_cookie("picture"),
        locale: get_cookie("locale"),
    })
}
