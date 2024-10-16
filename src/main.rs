use std::sync::{Arc, Mutex};
use hyprland::dispatch::{DispatchType, WindowIdentifier};
use hyprland::data::Client;
use hyprland::dispatch;
use hyprland::dispatch::{Dispatch, WorkspaceIdentifierWithSpecial};
use hyprland::event_listener::EventListener;
use hyprland::prelude::HyprDataActiveOptional;
use hyprland::shared::{Address, WorkspaceId, WorkspaceType};
use regex::Regex;

mod events;
mod event_listener;

struct PIPWindow {
    pub address: Option<Address>
}

impl PIPWindow {
    fn new() -> Self {
        Self {
            address: None
        }
    }

    fn set_address(&mut self, address: Option<Address>) {
        self.address = address
    }

    fn is_none(&self) -> bool {
        self.address.is_none()
    }
}


fn main() {
    let pip_manager = Arc::new(Mutex::new(PIPWindow::new()));
    let mut event_listener = EventListener::new();

    let (event_tx, event_rx) = crossbeam_channel::bounded::<Event>(32);

    let r:Regex = Regex::new("activewindowv2>>(?P<address>.*)").unwrap();

    println!("{:?}", r.captures("activewindowv2>>57d729ee6210").unwrap() );

    event_listener.add_active_window_change_handler({
        let pip_manager = pip_manager.clone();
        move |data| {
            let active_window = Client::get_active().unwrap();
            println!("active_window {data:?}");

            if let Some(active) = active_window {
                if active.class.is_empty() && active.floating {
                    pip_manager.lock().unwrap().set_address(Some(active.address))
                }
            }
        }
    });

    event_listener.add_window_close_handler({
        let pip_manager = pip_manager.clone();
        move |data| {
            println!("{data:?}");
            let mut pip = pip_manager.lock().unwrap();
            let address = pip.address.clone();

            if address.is_none() {return;}

            if data == address.unwrap() {
                pip.set_address(None);
            }
        }
    });

    event_listener.add_workspace_change_handler({
        let pip_manager = pip_manager.clone();
        move |id| {
            let pip = pip_manager.lock().unwrap();
            if pip.is_none() {
                return;
            }
            match id {
                WorkspaceType::Regular(id) => {
                    let _ = dispatch!(MoveToWorkspace,
                    WorkspaceIdentifierWithSpecial::Id(WorkspaceId::from(id.parse::<i32>().unwrap())),
                    Some(WindowIdentifier::Address(pip.address.clone().unwrap()))
                );
                }
                _ => {}
            }
        }
    });

    event_listener.start_listener().unwrap();
}
