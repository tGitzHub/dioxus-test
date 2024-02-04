#![allow(non_snake_case)]
// import the prelude to get access to the `rsx!` macro and the `Scope` and `Element` types
use dioxus::prelude::*;

fn main() {
    #[cfg(not(target_arch = "wasm32"))]
    // launch the dioxus app in a window
    dioxus_desktop::launch(App);

    #[cfg(target_arch = "wasm32")]
    // launch the dioxus app in a webview
    dioxus_web::launch(App);
}

// define a component that renders a div with the text "Hello, world!"
fn App(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            "Hello, world!"
        }
    })
}