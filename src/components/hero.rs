use dioxus::prelude::*;

use crate::components::{chat::Chat, nav::Nav};

#[component]
pub fn Hero() -> Element {
    rsx! {
        div {
            Nav{ }
            Chat{}
        }
    }
}
