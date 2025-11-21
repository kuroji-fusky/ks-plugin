#![allow(non_snake_case)]

pub enum ToastType {
    Warn,
    Error,
    Info,
    BasicBitch,
}

pub mod Client {
    use super::ToastType;

    pub fn trigger_toast_notification(toast_type: &ToastType) {}

    // Idk why a plugin will change a language but it's here for a specfic use case lol
    pub fn change_locale(locale: &str) {}
}

pub mod Dashboard {
    pub fn spawn_panel(panel: &str) {}
    pub fn destroy_panel(panel: &str, bypass_warning: Option<&bool>) {}
    pub fn update_panel(panel: &str) {}
    pub fn update_panel_location() {}

    // Pauses updates from network fetches
    pub fn pause_updates(duration: Option<&u8>) {}
    pub fn resume_updates() {}
}
