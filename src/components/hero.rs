use dioxus::prelude::*;

use crate::components::{chat::Chat, nav::Nav};

#[component]
pub fn Hero() -> Element {
    rsx! {
        div {
            id: "hero",
            Nav{ id: String::from("nav-l") }
            Chat{}
            Nav{ id: String::from("nav-r") }

        }
    }
}
