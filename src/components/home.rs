use crate::components::Chat;
use crate::components::Header;
use crate::components::Nav;
use crate::infra::models::Room;
use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    let mut hide_left = use_signal(|| false);
    let mut hide_right = use_signal(|| true);
    let opened_room = Room::default();
    rsx! {
        div {
            class: "h-full flex flex-col items-center justify-center align-center",
            Header {
                on_button_click: move |_| hide_left.toggle(),
                left_sidebar_opened: hide_left()
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


                    }

                Chat {
                    room_info: opened_room,
                    on_button_click: move |_| hide_right.toggle(),
                    room_info_opened: hide_right()
                }

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
