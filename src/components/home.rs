use crate::components::Chat;
use crate::components::Header;
use crate::components::Login;
use crate::components::Nav;
use crate::components::cards::RoomCard;
use crate::components::nav::ActionProps;
use crate::components::nav::SearchProps;
use crate::infra::models::AvaliableRoom;
use crate::infra::models::Room;
use crate::providers::LoggedIn;
use crate::services::Client;
use dioxus::prelude::*;
use tracing::info;

#[component]
pub fn Home() -> Element {
    let mut hide_left = use_signal(|| false);
    let mut hide_right = use_signal(|| true);
    let current_room = use_context::<Signal<Room>>();
    let avaliable_rooms = use_context::<Signal<Vec<AvaliableRoom>>>();
    let is_logged_in = use_context::<Signal<LoggedIn>>();
    let client = use_context::<Signal<Client>>();

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
                    search: SearchProps{
                        on_search: EventHandler::new(move |query: String| {
                            let mut client = client.clone();
                            spawn(async move {
                                tracing::info!("{:#?}", query);

                                let parts: Vec<&str> = query.splitn(2, ':').collect();

                                match parts.as_slice() {
                                    [code] => {
                                        if let Err(e) = client.write().access_room(code.to_string(), None).await {
                                            tracing::error!("Failed to access room: {}", e);
                                        }
                                    },
                                    [code, pass] => {
                                        if let Err(e) = client.write().access_room(code.to_string(), Some(pass.to_string())).await {
                                            tracing::error!("Failed to access room: {}", e);
                                        }
                                    },
                                    _ => unreachable!(),
                                }
                                info!("asdfasdfsfd");
                            });
                        }),
                        class: "".into(),
                        placeholder: "room code".into(),
                    },

                    action: ActionProps{
                        onclick: EventHandler::new(move |_| {}),
                        class:"".into(),
                        children:rsx!{"+"}
                    },
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
                    search: SearchProps {
                            on_search: EventHandler::new(move |query| {
                                    // Handle the search query here
                                    println!("Search query: {}", query);
                                }),
                            class: "".into(),
                            placeholder: "room code".into(),
                        },
                        action: ActionProps {
                            onclick: EventHandler::new(move |_| {}),
                            class: "".into(),
                            children: rsx! { "+" },
                        },
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
