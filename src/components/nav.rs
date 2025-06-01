use dioxus::prelude::*;

#[component]
pub fn Nav(class: String, children: Element) -> Element {
    rsx! {
        div {
            class: format!("bg-red-200 h-full flex flex-col gap-1 items-center justify-center align-center {class}"),
            {children}
        }
    }
}
