use dioxus::prelude::*;
use dioxus_toast::{ToastInfo, ToastManager};

use crate::{
    infra::models::{AvaliableRoom, Server, User},
    providers::LoggedIn,
    services::Client,
};

#[component]
pub fn Login() -> Element {
    let mut user = use_context::<Signal<User>>();
    let mut server = use_context::<Signal<Server>>();
    let is_logged_in = use_context::<Signal<LoggedIn>>();
    let mut toast = use_context::<Signal<ToastManager>>();
    let client = use_context::<Signal<Client>>();
    let avaliable_rooms = use_context::<Signal<Vec<AvaliableRoom>>>();

    let mut u = use_signal(String::new);
    let mut s = use_signal(String::new);

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
                        class: "
                                bg-[#22262b] p-[10.5px] rounded w-full
                        ",
                        onclick: move |_| {
                            if u.read().is_empty() || s.read().is_empty() {
                                let _ = toast.write().popup(
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
                                user.set(User {
                                    name: u.read().to_string(),
                                    uuid: String::new(),
                                });
                                server.set(Server {
                                    addres: s.read().to_string(),
                                });

                                let server_addr = server.read().addres.clone();
                                let user_data = user.read().clone();
                                let mut client_signal = client.clone();
                                let mut is_logged_in_signal = is_logged_in.clone();
                                let avaliable_rooms = avaliable_rooms.clone();

                                spawn(async move {
                                    match Client::new(&server_addr, &user_data).await {
                                        Ok(mut c) => {
                                            let _ = c.start_recive_task(avaliable_rooms).await;
                                            let _ = c.login(&user_data).await;

                                            client_signal.set(c);
                                            is_logged_in_signal.set(LoggedIn(true));
                                        }
                                        Err(err) =>{
                                            let _ = toast.write().popup(
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
