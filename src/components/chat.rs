use dioxus::prelude::*;

#[component]
pub fn Chat() -> Element {
    rsx! {
        div {
            id:"chat",
            div { id: "main" }
            div {  id: "input"}
        }
    }
}
