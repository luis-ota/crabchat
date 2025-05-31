use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    rsx! {
            div { class:"h-full flex items-center justify-center align-center",
                "crabchat"
            }
    }
}
