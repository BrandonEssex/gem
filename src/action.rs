#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Action {
    Quit,
    Redraw,
    ToggleHelp,
    StartPomodoro,
    StopPomodoro,
    ToggleTimer,
    OpenDashboard,
    OpenEditor,
    IngestIncoming,
}
