use dioxus::prelude::*;

#[component]
pub fn Header() -> Element {
    rsx! {
            div { 
                class:"w-full p-2 flex items-center justify-center align-center",
                "crabchat"
            }
    }
}
