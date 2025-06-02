use dioxus::prelude::*;
use dioxus_free_icons::{
    icons::ld_icons::{LdPanelLeft, LdPanelRight},
    Icon,
};

#[component]
pub fn Header(on_button_click: EventHandler<()>, opened: bool) -> Element {
    rsx! {
        div {
            class: "w-full bg-[#22262b] p-4 flex items-center justify-between align-center",
            button {
                onclick: move |_| on_button_click.call(()),
                if opened {
                    Icon {
                        width: 20,
                        height: 20,
                        fill: "white",
                        icon: LdPanelLeft,
                    }
                } else {
                    Icon {
                        width: 20,
                        height: 20,
                        fill: "white",
                        icon: LdPanelRight,
                    }
                }
            }
            div { class: "mx-4 wired-shadow wired-text", "crabchat" }
            span{}
        }
    }
}
