#![allow(non_snake_case)]
use dioxus::prelude::*;

pub(crate) mod components;

use components::preview::*;
use components::story::*;

fn main() {
    launch(App)
}

pub fn App() -> Element {
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
