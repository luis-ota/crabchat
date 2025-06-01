use dioxus::prelude::*;

#[component]
pub fn Nav() -> Element {
    rsx! {
            div { 
                class:"bg-red-200 h-full w-[20%] flex items-center justify-center align-center",
                "nav"
            }
    }
}
