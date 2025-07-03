use dioxus::prelude::*;
use dioxus_free_icons::{
    Icon,
    icons::ld_icons::{LdLogOut, LdPanelLeft, LdPanelRight},
};

use crate::{
    AppContext,
    infra::models::{Server, User},
};

#[component]
pub fn Header(on_button_click: EventHandler<()>, left_sidebar_opened: bool) -> Element {
    let mut ctx = use_context::<AppContext>();

    rsx! {
        div {
            class: "w-full bg-[#22262b] p-4 flex items-center justify-between align-center",
            if ctx.is_logged_in.read().0{

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
            if ctx.is_logged_in.read().0{
                button{
                    onclick: move |_| {
                        ctx.user.set(User::default());
                        ctx.server.set(Server::default());
                        ctx.is_logged_in.write().set(false);
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
