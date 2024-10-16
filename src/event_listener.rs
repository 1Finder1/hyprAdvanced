use std::os::unix::net::UnixListener;
use crossbeam_channel::Sender;
use crate::events::HyprlandEvent;

pub struct EventListened {
    event_tx: Sender<HyprlandEvent>
}

impl EventListened {
    fn new(event_tx: Sender<HyprlandEvent>) {
        let listener = UnixListener::bind("$XDG_RUNTIME_DIR/hypr/$HYPRLAND_INSTANCE_SIGNATURE/.socket2.sock");
           
    }
}