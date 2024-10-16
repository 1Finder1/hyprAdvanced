pub enum HyprlandEvent {
    ActiveWindowV2(str),
    CloseWindow(str),
    Workspace(str)
}