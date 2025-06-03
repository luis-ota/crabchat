use dioxus::prelude::*;

#[component]
pub fn Nav(class: String, children: Element) -> Element {
    rsx! {
        div {
            class: format!("bg-[#2d3139] h-full {class}"),
            div {  class: "h-full p-2 py-4 flex flex-col justify-between align-center",
            div {
                class: "flex flex-col gap-4",
                input {
                    class: "
                    flex
                    p-1
                    rounded-md border border-neutral-800
                    bg-[#22262b]
                    text-white font-mono wired-text outline-none
                    transition duration-300 ease-in-out
                    hover:shadow-wired
                    hover:border-rustOrange",
                    r#type: "text",
                    placeholder: "search...",
                }

                div{
                    class: "flex flex-col gap-1 overflow-auto",
                    {children}

                }
            }

            button {
                class: "
                    flex
                    justify-center
                    p-1
                    rounded-md border border-neutral-800
                    bg-[#22262b]
                    text-white font-mono wired-text
                    transition duration-300 ease-in-out
                    hover:shadow-wired
                    hover:border-rustOrange
                ",
                "+"
            }
            }

        }
    }
}
