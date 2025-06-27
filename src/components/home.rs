use crate::components::Chat;
use crate::components::Header;
use crate::components::Login;
use crate::components::Nav;
use crate::components::cards::RoomCard;
use crate::infra::models::AvaliableRoom;
use crate::infra::models::Room;
use crate::providers::LoggedIn;
use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    let mut hide_left = use_signal(|| false);
    let mut hide_right = use_signal(|| true);
    let current_room = use_context::<Signal<Room>>();
    let avaliable_rooms = use_context::<Signal<Vec<AvaliableRoom>>>();
    let is_logged_in = use_context::<Signal<LoggedIn>>();

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
                            if hide_left() || !is_logged_in.read().0{
                                "collapsed translate-x-100"
                            } else {
                                "w-[20%]"
                            }
                        ),
                        for room in avaliable_rooms.read().iter() {
                                RoomCard { info: room.clone() }
                            }

                    }

                if is_logged_in.read().0{
                    Chat {
                        room_info: current_room,
                        on_button_click: move |_| hide_right.toggle(),
                        room_info_opened: hide_right()
                        }
                }else{
                    Login{}
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
