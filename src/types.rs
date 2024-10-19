use hyprland::data::Client;
use regex::Regex;

pub enum HyprlandEvent {
    ActiveWindowV2(String),
    CloseWindow(String),
    Workspace(String),
}

pub struct WorkspaceRule {
    pub app_class: Regex,
    pub new_title: String,
}

impl WorkspaceRule {
    pub fn new(app_class: String, new_title: String) -> Self {
        Self {
            app_class: Regex::new(&app_class).expect("invalid app class"),
            new_title,
        }
    }

    pub fn compare(&self, client: &Client) -> bool {
        self.app_class.is_match(&client.class)
    }
}
