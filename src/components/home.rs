use dioxus::prelude::*;
use crate::components::Header;
use crate::components::Nav;
use crate::components::Chat;

#[component]
pub fn Home() -> Element {
    let mut hide_left = use_signal(|| false);
    let mut hide_right = use_signal(|| false);

    rsx! {
        div {
            class: "h-full flex flex-col items-center justify-center align-center",
            Header {
                on_left_click: move |_| hide_left.toggle(),
                on_right_click: move |_| hide_right.toggle(),
            }
            div {
                class: "flex flex-row w-full h-full",
                Nav {
                        class: format_args!(
                            "nav-transition {}",
                            if hide_left() {
                                "collapsed translate-x-100"
                            } else {
                                "w-[1/5]"
                            }
                        )
                    }
                Chat {}
                Nav {
                        class: format_args!(
                            "nav-transition {}",
                            if hide_right() {
                                "collapsed -translate-x-100"
                            } else {
                                "w-[1/5]"
                            }
                        )
                    }
            }
        }
    }
}
