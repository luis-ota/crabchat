use crate::components::Chat;
use crate::components::Header;
use crate::components::Login;
use crate::components::Nav;
use crate::components::cards::RoomCard;
use crate::infra::models::AvaliableRoom;
use crate::infra::models::Room;
use crate::infra::models::User;
use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    let mut hide_left = use_signal(|| false);
    let mut hide_right = use_signal(|| true);
    let current_room = use_context::<Signal<Room>>();
    let avaliable_rooms = use_context::<Signal<Vec<AvaliableRoom>>>();
    let user = use_context::<Signal<User>>();

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
                            if hide_left() || user.read().name == ""{
                                "collapsed translate-x-100"
                            } else {
                                "w-[20%]"
                            }
                        ),
                        for room in avaliable_rooms.read().iter() {
                                RoomCard { info: room.clone() }
                            }

                    }

                if user.read().name != ""{
                    Chat {
                    room_info: current_room,
                    on_button_click: move |_| hide_right.toggle(),
                    room_info_opened: hide_right()
                }}else{
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
