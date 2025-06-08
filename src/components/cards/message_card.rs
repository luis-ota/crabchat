use chrono::{DateTime, NaiveDateTime, Utc};
use dioxus::prelude::*;

use crate::infra::models::{User, UserMessage};

#[component]
pub fn MessageCard(user_message: ReadOnlySignal<UserMessage>) -> Element {
    let user_message: &UserMessage = &user_message.read();
    let user: &User = user_message.user.as_ref().unwrap();
    let from = "self";

    rsx! {
        div{
            class: "w-full flex justify-end p-[0.85rem]",
            div {
            class: "min-w-[50%]
                    max-w-[75%]
                    mb-2
                    flex flex-col
                    justify-end self-end
                    text-white font-mono
                    ",
            span { "{user.name}" }
            div {
                class: format!("
                            rounded
                            p-3
                            {}", if from == "self" { "bg-[#2d3139]" } else { "bg-[#22262b]" }),
                p {"{user_message.message}"}
                span {
                    class:"w-full flex justify-end",
                    "{display_datetime(&user_message.datetime).unwrap()}" }
            }

        }}
    }
}

fn display_datetime(dt: &str) -> Option<String> {
    let naive = NaiveDateTime::parse_from_str(dt, "%Y-%m-%d %H:%M:%S").ok()?;
    let datetime = DateTime::<Utc>::from_naive_utc_and_offset(naive, Utc);
    let now = Utc::now();

    if now.date_naive() == datetime.date_naive() {
        Some(datetime.format("%H:%M").to_string())
    } else {
        Some(datetime.format("%d/%m %H:%M").to_string())
    }
}
