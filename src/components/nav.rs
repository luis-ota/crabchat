use dioxus::prelude::*;

#[component]
pub fn Nav() -> Element {
    rsx! {
        div {
            div { id: "search" }
            div {  id: "public_rooms"}
            div {  id: "create" }
        }
    }
}
