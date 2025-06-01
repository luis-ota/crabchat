use dioxus::prelude::*;

#[component]
pub fn RoomCard(
    name: String,
    users_count: i32,
    is_locked: bool,
) -> Element {
    rsx! {
        div {
            class: "w-[90%] p-1 flex flex-col rounded-md border border-black-100 bg-gray-100",
            div {
                class: "w-full flex items-center justify-between",
                div {
                    class: "flex items-center px-2",
                    span { class: "mr-1", "👤" }
                    span { "{users_count}" }
                }
                div {
                    class: "text-gray-500",
                    if is_locked {
                        span { class: "text-red-500", "🔒" }
                    } else {
                        span { class: "text-green-500", "🔓" }
                    }
                }
            }
            div {
                class: "flex-grow flex items-center justify-center text-lg font-medium",
                "{name}"
            }
        }
    }
}