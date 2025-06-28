use dioxus::prelude::*;
use tracing::info;

#[derive(Props, PartialEq, Clone)]
pub struct ActionProps {
    pub onclick: EventHandler<()>,
    pub class: String,
    pub children: Element,
}

#[derive(Props, PartialEq, Clone)]
pub struct SearchProps {
    pub on_search: EventHandler<String>,
    pub class: String,
    pub placeholder: String,
}

#[derive(Props, PartialEq, Clone)]
pub struct NavProps {
    pub class: String,
    pub children: Element,
    pub action: ActionProps,
    pub search: SearchProps,
}

#[component]
pub fn Nav(props: NavProps) -> Element {
    let mut search = use_signal(String::new);

    rsx! {
        div {
            class: format!("bg-[#2d3139] h-full {}", props.class),
            div {  class: "h-full px-3 py-4 gap-2 flex flex-col justify-between align-center",
            div {
                class: "h-full flex flex-col gap-4",
                input {
                    onkeydown: move |event| {
                            if event.key() == Key::Enter {
                                props.search.on_search.call(search.read().to_string());
                            }
                        },
                    class: format!("
                    flex
                    p-2
                    rounded-md border border-neutral-800
                    bg-[#22262b]
                    text-white font-mono wired-text
                    outline-none
                    transition duration-300 ease-in-out
                    hover:shadow-wired
                    hover:border-rustOrange
                    {}
                    ", props.search.class),
                    r#type: "text",
                    placeholder: format!("{}...", props.search.placeholder),
                    value: "{search}",
                    oninput: move |e| search.set(e.value()),
                }

                div{
                    class: "flex flex-col gap-1 overflow-auto",
                    {props.children}

                }
            }

            button {
                onclick: move |_| props.action.onclick.call(()),
                class: format!("bg-[#22262b] p-[10.5px] rounded {}", props.action.class),
                {props.action.children}
            }
            }

        }
    }
}
