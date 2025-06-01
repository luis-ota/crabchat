use crate::components::Chat;
use crate::components::Header;
use crate::components::Nav;
use crate::components::RoomCard;
use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    let mut hide_left = use_signal(|| false);
    let mut hide_right = use_signal(|| true);

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
                                "w-[30%]"
                            }
                        ),

                        for i in 0..4 {
                            RoomCard { name: format!("rustloves {:?}", i), users_count: 3, is_locked: false }
                            }
                    }

                Chat {}

                Nav {
                        class: format_args!(
                            "nav-transition {}",
                            if hide_right() {
                                "collapsed -translate-x-100"
                            } else {
                                "w-[20%]"
                            }
                        )
                    }
            }
        }
    }
}
