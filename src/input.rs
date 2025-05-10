use crate::action::{Action, MindmapAction::*};
use crate::config::KeyMap;
use crossterm::event::{self, Event, KeyCode, KeyModifiers};

pub fn handle_input(keymap: &KeyMap) -> crossterm::Result<Option<Action>> {
    if event::poll(std::time::Duration::from_millis(100))? {
        if let Event::Key(key_event) = event::read()? {
            let key_str = format_key(&key_event.code, key_event.modifiers);
            if let Some(map) = keymap.get("mindmap") {
                for (name, binding) in map {
                    if binding == &key_str {
                        return Ok(Some(match name.as_str() {
                            "quit" => Action::Quit,
                            "unselect" => Action::Mindmap(Unselect),
                            "scroll_up" => Action::Mindmap(ScrollUp),
                            "scroll_down" => Action::Mindmap(ScrollDown),
                            "delete" => Action::Mindmap(Delete),
                            "select_up" => Action::Mindmap(SelectUp),
                            "select_down" => Action::Mindmap(SelectDown),
                            "select_left" => Action::Mindmap(SelectLeft),
                            "select_right" => Action::Mindmap(SelectRight),
                            "erase" => Action::Mindmap(Erase),
                            "create_sibling" => Action::Mindmap(CreateSibling),
                            "create_child" => Action::Mindmap(CreateChild),
                            "create_free_node" => Action::Mindmap(CreateFreeNode),
                            "execute" => Action::Mindmap(Execute),
                            "drill_down" => Action::Mindmap(DrillDown),
                            "pop_up" => Action::Mindmap(PopUp),
                            "jump" => Action::Mindmap(Jump),
                            "toggle_completed" => Action::Mindmap(ToggleCompleted),
                            "toggle_hide_completed" => Action::Mindmap(ToggleHideCompleted),
                            "arrow" => Action::Mindmap(Arrow),
                            "auto_arrange" => Action::Mindmap(AutoArrange),
                            "toggle_collapsed" => Action::Mindmap(ToggleCollapsed),
                            "save" => Action::Mindmap(Save),
                            "toggle_show_logs" => Action::Mindmap(ToggleShowLogs),
                            "enter_command" => Action::Mindmap(EnterCommand),
                            "find_task" => Action::Mindmap(FindTask),
                            "yank_paste_node" => Action::Mindmap(YankPasteNode),
                            "raise_selected" => Action::Mindmap(RaiseSelected),
                            "lower_selected" => Action::Mindmap(LowerSelected),
                            "search" => Action::Mindmap(Search),
                            "undo_delete" => Action::Mindmap(UndoDelete),
                            "help" => Action::Mindmap(Help),
                            _ => Action::Custom(name.clone()),
                        }));
                    }
                }
            }
        }
    }
    Ok(None)
}

fn format_key(code: &KeyCode, mods: KeyModifiers) -> String {
    let prefix = if mods.contains(KeyModifiers::CONTROL) {
        "ctrl+"
    } else {
        ""
    };
    match code {
        KeyCode::Char(c) => format!("{}{}", prefix, c),
        KeyCode::Enter => "enter".to_string(),
        KeyCode::Tab => "tab".to_string(),
        KeyCode::Backspace => "backspace".to_string(),
        KeyCode::Delete => "del".to_string(),
        KeyCode::Esc => "esc".to_string(),
        KeyCode::Left => "left".to_string(),
        KeyCode::Right => "right".to_string(),
        KeyCode::Up => "up".to_string(),
        KeyCode::Down => "down".to_string(),
        KeyCode::PageUp => "pgup".to_string(),
        KeyCode::PageDown => "pgdn".to_string(),
        _ => "".to_string(),
    }
}
