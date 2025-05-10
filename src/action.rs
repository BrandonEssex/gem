#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Action {
    Quit,
    Redraw,
    ToggleHelp,
    Custom(String),
    Mindmap(MindmapAction),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum MindmapAction {
    Unselect,
    ScrollUp,
    ScrollDown,
    Delete,
    SelectUp,
    SelectDown,
    SelectLeft,
    SelectRight,
    Erase,
    CreateSibling,
    CreateChild,
    CreateFreeNode,
    Execute,
    DrillDown,
    PopUp,
    Jump,
    ToggleCompleted,
    ToggleHideCompleted,
    Arrow,
    AutoArrange,
    ToggleCollapsed,
    Save,
    ToggleShowLogs,
    EnterCommand,
    FindTask,
    YankPasteNode,
    RaiseSelected,
    LowerSelected,
    Search,
    UndoDelete,
    Help
}
