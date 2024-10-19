use crate::event_listener::EventListener;
use crate::rename_workspace::rename_workspace;
use crate::types::HyprlandEvent;
use hyprland::data::Client;
use hyprland::dispatch;
use hyprland::dispatch::{Dispatch, WorkspaceIdentifierWithSpecial};
use hyprland::dispatch::{DispatchType, WindowIdentifier};
use hyprland::prelude::HyprDataActiveOptional;
use hyprland::shared::{Address, WorkspaceId};
use std::sync::{Arc, Mutex};

mod event_listener;
mod rename_workspace;
mod types;

struct PIPWindow {
    pub address: Option<Address>,
}

impl PIPWindow {
    fn new() -> Self {
        Self { address: None }
    }

    fn set_address(&mut self, address: Option<Address>) {
        self.address = address
    }

    fn is_none(&self) -> bool {
        self.address.is_none()
    }
}

fn get_active_window() -> Option<Client> {
    Client::get_active().expect("Could not get active window")
}

fn main() {
    let pip_manager = Arc::new(Mutex::new(PIPWindow::new()));

    let (event_tx, event_rx) = crossbeam_channel::bounded::<HyprlandEvent>(32);

    let listener = EventListener::new(event_tx);

    listener.start_loop();

    while let Ok(event) = event_rx.recv() {
        rename_workspace();

        match event {
            HyprlandEvent::ActiveWindowV2(_) => {
                let active = get_active_window();

                if let Some(active) = active {
                    if active.floating && active.class.is_empty() {
                        pip_manager
                            .lock()
                            .unwrap()
                            .set_address(Some(active.address))
                    }
                }
            }
            HyprlandEvent::Workspace(id) => {
                let pip = pip_manager.lock().unwrap();
                if pip.is_none() {
                    continue;
                }
                dispatch!(
                    MoveToWorkspaceSilent,
                    WorkspaceIdentifierWithSpecial::Id(WorkspaceId::from(
                        id.parse::<i32>().unwrap()
                    )),
                    Some(WindowIdentifier::Address(pip.address.clone().unwrap()))
                )
                .expect("Failed to move window");
            }
            HyprlandEvent::CloseWindow(id) => {
                let mut pip = pip_manager.lock().unwrap();
                if let Some(address) = pip.address.clone() {
                    if address.to_string() == id {
                        pip.set_address(None)
                    }
                }
            }
        }
    }
}
