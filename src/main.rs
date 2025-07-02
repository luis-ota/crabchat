use dioxus::prelude::*;
use dioxus_toast::{ToastFrame, ToastManager};

use crate::{
    infra::models::{AvailableRoom, Room, Server, User},
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

#[derive(Clone)]
pub struct AppContext {
    pub user: Signal<User>,
    pub server: Signal<Server>,
    pub is_logged_in: Signal<LoggedIn>,
    pub toast: Signal<ToastManager>,
    pub client: Signal<Client>,
    pub available_rooms: Signal<Vec<AvailableRoom>>,
    pub current_room: Signal<Room>,
}

impl AppContext {
    pub fn new() -> Self {
        Self {
            user: Signal::new(User::default()),
            server: Signal::new(Server::default()),
            is_logged_in: Signal::new(LoggedIn(false)),
            toast: Signal::new(ToastManager::default()),
            client: Signal::new(Client::default()),
            available_rooms: Signal::new(Vec::new()),
            current_room: Signal::new(Room::default()),
        }
    }
}

#[component]
fn App() -> Element {
    let app_context = AppContext::new();

    use_context_provider(|| app_context.user);
    use_context_provider(|| app_context.server);
    use_context_provider(|| app_context.is_logged_in);
    use_context_provider(|| app_context.toast);
    use_context_provider(|| app_context.client);
    use_context_provider(|| app_context.available_rooms);
    use_context_provider(|| app_context.current_room);

    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: GLOBALS_CSS }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        document::Link { rel: "stylesheet", href: FIRACODE_FONT }

        ToastFrame { manager: app_context.toast }

        components::Home {}
    }
}

fn main() {
    dioxus::launch(App);
}
