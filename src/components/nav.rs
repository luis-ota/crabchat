use dioxus::prelude::*;

#[component]
pub fn Nav(id: String) -> Element {
    rsx! {
        div {
            id,
            div { id: "search" }
            div {  id: "public_rooms"}
            div {  id: "create" }
        }
    }
}
