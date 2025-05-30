use dioxus::prelude::*;

#[component]
pub fn Chat() -> Element {
    rsx! {
        div {
            div { id: "chat" }
            div {  id: "input"}
        }
    }
}
