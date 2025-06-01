use dioxus::prelude::*;
use crate::components::Header;
use crate::components::Nav;
use crate::components::Chat;


#[component]
pub fn Home() -> Element {
    rsx! {
            div { 
                class:"h-full flex flex-col items-center justify-center align-center",
                Header{}
                div {
                    class: "flex flex-row w-full h-full",
                    Nav { }
                    Chat { }
                    Nav { }


                }
            }
    }
}
