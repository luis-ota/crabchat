use dioxus::prelude::*;

#[component]
pub fn Chat() -> Element {
    rsx! {
            div { 
                class:"bg-sky-500/50 w-[60%] h-full flex items-center justify-center align-center",
                "chat"
            }
    }
}
