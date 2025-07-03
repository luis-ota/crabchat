use dioxus::prelude::*;
use dioxus_toast::ToastInfo;

use crate::{AppContext, infra::models::User, services::Client};

#[component]
pub fn Login() -> Element {
    let mut u = use_signal(String::new);
    let mut s = use_signal(String::new);
    let mut ctx = use_context::<AppContext>();

    rsx! {
        div {
            class: "w-full h-full flex flex-col justify-center items-center",
            span{class:"mx-4 my-8 wired-shadow wired-text text-3xl","login"}
            div {
                class:"w-40% flex flex-col gap-2",
                input {
                    class: "
                    flex w-full
                    p-2
                    rounded-md border border-neutral-800
                    bg-[#22262b]
                    text-white font-mono wired-text
                    outline-none
                    transition duration-300 ease-in-out
                    hover:shadow-wired
                    hover:border-rustOrange",
                    r#type: "text",
                    placeholder: "username",
                    value: "{u}",
                    oninput: move |e| u.set(e.value()),
                }
                input {
                    class: "
                    flex w-full
                    p-2
                    rounded-md border border-neutral-800
                    bg-[#22262b]
                    text-white font-mono wired-text
                    outline-none
                    transition duration-300 ease-in-out
                    hover:shadow-wired
                    hover:border-rustOrange",
                    r#type: "text",
                    placeholder: "server",
                    value: "{s}",
                    oninput: move |e| s.set(e.value()),
                }
                button {
                    class: "bg-[#22262b] p-[10.5px] rounded w-full",
                    onclick: move |_| {
                        if u.read().is_empty() || s.read().is_empty() {
                            let _ = ctx.toast.write().popup(
                                ToastInfo {
                                    heading: Some("Login Error".into()),
                                    context: "Please fill both username and server".into(),
                                    allow_toast_close: true,
                                    position: dioxus_toast::Position::TopRight,
                                    icon: None,
                                    hide_after: Some(7),
                                }
                            );
                        } else {
                            ctx.user.write().set(&User {
                                name: u.read().to_string(),
                                uuid: String::new(),
                            });

                            let server_addr = s.read().to_string();
                            let user_data = ctx.user.read().clone();
                            let mut available_rooms = ctx.available_rooms.clone();
                            let mut current_room = ctx.current_room.clone();
                            let mut toast = ctx.toast.clone();

                            spawn(async move {
                                match Client::new(&server_addr, &user_data).await {
                                    Ok(mut c) => {
                                        let _ = c.start_recive_task(available_rooms, current_room, toast);
                                        let _ = c.login(&user_data).await;

                                        ctx.client.write().set(c);
                                        ctx.is_logged_in.write().set(true);

                                    }
                                    Err(err) => {
                                        let _ = ctx.toast.write().popup(
                                            ToastInfo {
                                                heading: Some("Connection Error".into()),
                                                context: format!("Erro ao conectar: {:?}", err),
                                                allow_toast_close: true,
                                                position: dioxus_toast::Position::TopRight,
                                                icon: None,
                                                hide_after: Some(7),
                                            }
                                        );
                                    }
                                }
                            });
                        }
                    },
                    "access"
                }
            }
        }
    }
}
