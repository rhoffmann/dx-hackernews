#![allow(non_snake_case)]
use dioxus::prelude::*;

pub(crate) mod api;
pub(crate) mod components;

use components::preview::*;
use components::story::*;

fn main() {
    launch(App)
}

pub fn App() -> Element {
    use_context_provider(|| Signal::new(PreviewState::Unset));
    rsx! {
        div {
            class: "flex flex-row w-full",
            div {
                class: "w-1/2",
                Stories {}
            }
            div {
                class: "w-1/2",
                Preview {}
            }
        }
    }
}
