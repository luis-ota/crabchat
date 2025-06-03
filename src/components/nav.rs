use dioxus::prelude::*;

#[component]
pub fn Nav(class: String, children: Element) -> Element {
    rsx! {
        div {
            class: format!("bg-[#2d3139] h-full {class}"),
            div {  class: "h-full px-3 py-4 gap-2 flex flex-col justify-between align-center",
            div {
                class: "h-full flex flex-col gap-4",
                input {
                    class: "
                    flex
                    p-2
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
                        bg-[#22262b] p-[10.5px] rounded
                ",
                "+"
            }
            }

        }
    }
}
