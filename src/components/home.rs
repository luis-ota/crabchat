use crate::components::Chat;
use crate::components::Header;
use crate::components::Nav;
use crate::components::RoomCard;
use dioxus::prelude::*;
use dioxus_free_icons::icons::ld_icons::LdPanelLeft;
use dioxus_free_icons::icons::ld_icons::LdPanelRight;

#[component]
pub fn Home() -> Element {
    let mut hide_left = use_signal(|| false);
    let mut hide_right = use_signal(|| true);

    rsx! {
        div {
            class: "h-full flex flex-col items-center justify-center align-center",
            Header {
                on_button_click: move |_| hide_left.toggle(),
                opened: hide_left()
            }
            div {
                class: "flex flex-row w-full h-full",
                Nav {
                        class: format_args!(
                            "nav-transition {}",
                            if hide_left() {
                                "collapsed translate-x-100"
                            } else {
                                "w-[20%]"
                            }
                        ),

                        for i in 0..4 {
                            RoomCard { name: format!("rustlovers {:?}", i), users_count: 5-1, is_locked: false }
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
