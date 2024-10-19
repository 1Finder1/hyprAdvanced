use crate::types::WorkspaceRule;
use hyprland::data::Clients;
use hyprland::dispatch;
use hyprland::dispatch::{Dispatch, DispatchType};
use hyprland::prelude::{HyprData, HyprDataVec};
use hyprland::shared::WorkspaceId;

pub fn rename_workspace() {
    let rules: Vec<WorkspaceRule> = vec![
        WorkspaceRule::new(".*rustrover".to_string(), " ".to_string()),
        WorkspaceRule::new(".*pycharm".to_string(), " ".to_string()),
        WorkspaceRule::new(".*webstorm".to_string(), " ".to_string()),
        WorkspaceRule::new(".*datagrip".to_string(), " ".to_string()),
        WorkspaceRule::new(".*telegram.*".to_string(), " ".to_string()),
        WorkspaceRule::new("firefox|.*browser.*".to_string(), "󰖟 ".to_string()),
    ];

    let clients = Clients::get()
        .expect("Unable to fetch workspace ids")
        .to_vec();

    let mut renamed_workspaces: Vec<WorkspaceId> = vec![];

    for client in clients {
        let matches = rules
            .iter()
            .filter(|i| i.compare(&client))
            .collect::<Vec<&WorkspaceRule>>();

        if renamed_workspaces.contains(&client.workspace.id) {
            continue;
        }

        if matches.is_empty() {
            dispatch!(
                RenameWorkspace,
                client.workspace.id,
                Some(&*client.workspace.id.to_string())
            )
            .expect("Could not dispatch rename workspace");
            continue;
        }

        let first_match = matches.first().unwrap();

        renamed_workspaces.push(client.workspace.id.clone());

        dispatch!(
            RenameWorkspace,
            client.workspace.id,
            Some(&*format!(
                "{} {}",
                client.workspace.id.to_string(),
                first_match.new_title.clone()
            ))
        )
        .expect("Unable to dispatch rename workspace");
    }
}
