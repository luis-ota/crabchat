use dioxus::prelude::*;
use dioxus_toast::{ToastFrame, ToastManager};

use components::Home;

use crate::{
    infra::models::{AvaliableRoom, Room, Server, User},
    providers::LoggedIn,
    services::Client,
};

mod components;
mod infra;
mod providers;
mod services;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const FIRACODE_FONT: &str = "https://fonts.googleapis.com/css2?family=Fira+Code&display=swap";
const GLOBALS_CSS: Asset = asset!("/assets/styling/globals.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    use_context_provider(|| Signal::new(User::default()));
    use_context_provider(|| Signal::new(LoggedIn(false)));
    use_context_provider(|| Signal::new(Server::default()));
    use_context_provider(|| Signal::new(Room::default()));
    use_context_provider(|| Signal::new(Vec::<AvaliableRoom>::new()));
    use_context_provider(|| Signal::new(Client::default()));

    let toast = use_context_provider(|| Signal::new(ToastManager::default()));
    rsx! {

        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: GLOBALS_CSS }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        document::Link { rel: "stylesheet", href: FIRACODE_FONT }

        ToastFrame { manager: toast }
        Home {}

    }
}
