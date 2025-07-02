use dioxus::prelude::*;

use crate::infra::models::AvailableRoom;

#[component]
pub fn RoomCard(info: AvailableRoom) -> Element {
    rsx! {
        div {
            class: "
                flex flex-row items-center justify-between gap-2
                p-1
                focus:border-transparent
                rounded-md border border-neutral-800
                bg-[#22262b]
                text-white font-mono
                transition duration-300 ease-in-out
                hover:shadow-wired
                hover:border-rustOrange
            ",

            div {
                class: "h-full flex gap-1 text-sm text-gray-300",
                span { "👤" }
                span { "{info.users_count}" }
            }

            div {
                class: "flex-grow py-2 text-center wired-text autowrap",
                "{info.info.base_info.name}"
            }

            div {
                class: "h-full flex",
                if info.has_password {
                    span { class: "text-red-500", "🔒" }
                } else {
                    span { class: "text-green-500", "🔓" }
                }
            }
        }
    }
}
