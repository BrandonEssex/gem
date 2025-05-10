use crate::action::{Action, MindmapAction};
use crate::config::KeyMap;
use crossterm::event::{self, Event, KeyCode, KeyModifiers};

pub fn handle_input(keymap: &KeyMap) -> crossterm::Result<Option<Action>> {
    if event::poll(std::time::Duration::from_millis(100))? {
        if let Event::Key(key_event) = event::read()? {
            let key_str = format_key(&key_event.code, key_event.modifiers);
            if let Some(map) = keymap.get("mindmap") {
                for (name, binding) in map {
                    if binding == &key_str {
                        return Ok(Some(map_mindmap_action(name)));
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

fn map_mindmap_action(name: &str) -> Action {
    use MindmapAction::*;
    let act = match name {
        "quit" => Action::Quit,
        "unselect" => MindmapAction::Unselect,
        "scroll_up" => ScrollUp,
        "scroll_down" => ScrollDown,
        "delete" => Delete,
        "select_up" => SelectUp,
        "select_down" => SelectDown,
        "select_left" => SelectLeft,
        "select_right" => SelectRight,
        "erase" => Erase,
        "create_sibling" => CreateSibling,
        "create_child" => CreateChild,
        "create_free_node" => CreateFreeNode,
        "execute" => Execute,
        "drill_down" => DrillDown,
        "pop_up" => PopUp,
        "jump" => Jump,
        "toggle_completed" => ToggleCompleted,
        "toggle_hide_completed" => ToggleHideCompleted,
        "arrow" => Arrow,
        "auto_arrange" => AutoArrange,
        "toggle_collapsed" => ToggleCollapsed,
        "save" => Save,
        "toggle_show_logs" => ToggleShowLogs,
        "enter_command" => EnterCommand,
        "find_task" => FindTask,
        "yank_paste_node" => YankPasteNode,
        "raise_selected" => RaiseSelected,
        "lower_selected" => LowerSelected,
        "search" => Search,
        "undo_delete" => UndoDelete,
        "help" => Help,
        _ => return Action::Custom(name.to_string()),
    };
    Action::Mindmap(act)
}
