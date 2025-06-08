use dioxus::prelude::*;
use dioxus_free_icons::{
    Icon,
    icons::ld_icons::{LdSendHorizontal, LdSettings},
};

use crate::{components::cards::MessageCard, infra::models::Room};

#[component]
pub fn Chat(
    room_info: Signal<Room>,
    on_button_click: EventHandler<()>,
    room_info_opened: bool,
) -> Element {
    rsx! {
            div {
                class:"w-full h-full flex flex-col items-center justify-center align-center",
                div {
                    class: "p-4 w-full flex items-center justify-between align-center",
                    span{}
                    span{"{room_info.read().info.base_info.name}"}
                    button {
                        onclick: move |_| on_button_click.call(()),
                            Icon {
                                width: 20,
                                height: 20,
                                fill: "white",
                                icon: LdSettings,
                            }
                    }

                }
                div {
                    class: "h-full w-full flex-row flex justify-center items-start",
                    for user_message in room_info.read().messages.to_owned() {
                        MessageCard { user_message }
                    }

                }

                div {
                    class: "w-full px-3 gap-3 flex flex-row justify-between items-center m-4",
                        input {
                            class: "
                            flex w-full h-full
                            p-3
                            rounded-md border-0
                            bg-[#22262b]
                            text-white font-mono wired-text
                            outline-none
                            ",
                            r#type: "text",
                            placeholder: "message...",

                    }
                    button {
                        class: "h-full bg-[#22262b] p-3 rounded",
                        Icon{
                            width: 20,
                            height: 20,
                            fill: "white",
                            icon: LdSendHorizontal,
                        }
                    }
                }
            }
    }
}
