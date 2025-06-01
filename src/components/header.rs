use dioxus::prelude::*;

#[component]
pub fn Header(on_left_click: EventHandler<()>, on_right_click: EventHandler<()>) -> Element {
    rsx! {
        div {
            class: "w-full p-2 flex items-center justify-between align-center",
            button {
                onclick: move |_| on_left_click.call(()),
                "..."
            }
            div { class: "mx-4", "crabchat" }
            button {
                onclick: move |_| on_right_click.call(()),
                "..."
            }
        }
    }
}
