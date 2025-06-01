use dioxus::prelude::*;

#[component]
pub fn Nav(class: String) -> Element {
    rsx! {
        div {
            class: format!("bg-red-200 h-full flex items-center justify-center align-center {class}"),
            "nav"
        }
    }
}
