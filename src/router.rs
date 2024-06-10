use dioxus::prelude::*;
use dioxus_router::prelude::*;

use crate::components::home::Home;
use crate::components::sign_in::SignIn;

#[derive(Clone, Debug, PartialEq, Routable)]
enum Route {
    #[route("/")]
    Home,

    #[route("/signin")]
    SignIn,
}

pub fn AppRouter() -> Element {
    rsx! {
        Router {
            Route {
                to: Route::Home,
                Home {}
            }
            Route {
                to: Route::SignIn,
                SignIn {}
            }
        }
    }
}