use dioxus::prelude::*;
use dioxus_free_icons::{
    Icon,
    icons::ld_icons::{LdLogOut, LdPanelLeft, LdPanelRight},
};

use crate::{
    infra::models::{Server, User},
    providers::LoggedIn,
};

#[component]
pub fn Header(on_button_click: EventHandler<()>, left_sidebar_opened: bool) -> Element {
    let mut user = use_context::<Signal<User>>();
    let mut server = use_context::<Signal<Server>>();
    let mut is_logged_in = use_context::<Signal<LoggedIn>>();

    rsx! {
        div {
            class: "w-full bg-[#22262b] p-4 flex items-center justify-between align-center",
            if is_logged_in.read().0{

                button {
                    onclick: move |_| on_button_click.call(()),
                    if left_sidebar_opened {
                        Icon {
                            width: 20,
                            height: 20,
                            fill: "white",
                            icon: LdPanelLeft,
                        }
                    } else {
                        Icon {
                            width: 20,
                            height: 20,
                            fill: "white",
                            icon: LdPanelRight,
                        }
                    }
                }
            }else{span{}}
            div { class: "mx-4 wired-shadow wired-text text-2xl", "crabchat" }
            if is_logged_in.read().0{
                button{
                    onclick: move |_| {
                        user.set(User::default());
                        server.set(Server::default());
                    },

                    Icon {
                        width: 20,
                        height: 20,
                        fill: "white",
                        icon: LdLogOut,
                    }
                }
            }else{span {  }
            }
        }
    }
}
