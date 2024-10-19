use crate::types::HyprlandEvent;
use crossbeam_channel::Sender;
use regex::Regex;
use std::env::var;
use std::io::Read;
use std::os::unix::net::UnixStream;
use std::thread;

fn get_sock_path() -> String {
    let run_dir = var("XDG_RUNTIME_DIR").expect("No XDG_RUNTIME_DIR env variable set");
    let instance_dir =
        var("HYPRLAND_INSTANCE_SIGNATURE").expect("No HYPRLAND_INSTANCE_SIGNATURE env variable");

    format!("{run_dir}/hypr/{instance_dir}/.socket2.sock")
}

pub struct EventListener {
    event_tx: Sender<HyprlandEvent>,
    listener: UnixStream,
}

impl EventListener {
    pub fn new(event_tx: Sender<HyprlandEvent>) -> Self {
        let path = get_sock_path();
        let listener = UnixStream::connect(path).expect("Failed to create listener");

        Self { event_tx, listener }
    }

    pub fn start_loop(&self) {
        let sender = self.event_tx.clone();
        let mut listener = self.listener.try_clone().expect("Failed to clone listener");

        thread::spawn(move || {
            let reg =
                Regex::new("(?P<event>[A-z0-9]+)>>(?P<data>.*)").expect("Failed to compile regex");
            loop {
                let mut buf = [0; 4096];

                let num_read = listener.read(&mut buf).expect("Failed to read event");
                if num_read == 0 {
                    break;
                }
                let buf = &buf[..num_read];
                let content = String::from_utf8(buf.to_vec()).expect("Failed to parse event");

                let events = content.trim().split("\n").collect::<Vec<&str>>();

                for event in events {
                    let event_reg = reg.captures(&event).expect("Failed to match regex");

                    match &event_reg["event"] {
                        "workspacev2" | "createworkspacev2" => {
                            let workspace_id =
                                event_reg["data"].split(",").collect::<Vec<&str>>()[0];

                            sender.send(HyprlandEvent::Workspace(workspace_id.to_string()))
                        }
                        "activewindowv2" => sender
                            .send(HyprlandEvent::ActiveWindowV2(event_reg["data"].to_string())),
                        "closewindow" => {
                            sender.send(HyprlandEvent::CloseWindow(event_reg["data"].to_string()))
                        }
                        _ => Ok(()),
                    }
                    .expect("Failed to send event")
                }
            }
        });
    }
}
