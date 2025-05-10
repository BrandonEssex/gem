use crate::action::{Action, MindmapAction};
use crate::config::KeyMap;
use crossterm::event::{self, Event, KeyCode, KeyModifiers};

pub fn handle_input(keymap: &KeyMap) -> crossterm::Result<Option<Action>> {
    if event::poll(std::time::Duration::from_millis(100))? {
        if let Event::Key(key_event) = event::read()? {
            let key_str = format_key(&key_event.code, key_event.modifiers);
            if let Some(mmap) = keymap.get("mindmap") {
                for (name, binding) in mmap {
                    if binding == &key_str {
                        return Ok(Some(match name.as_str() {
                            "quit" => Action::Quit,
                            "save" => Action::Mindmap(MindmapAction::Save),
                            "scroll_up" => Action::Mindmap(MindmapAction::ScrollUp),
                            "scroll_down" => Action::Mindmap(MindmapAction::ScrollDown),
                            "select_up" => Action::Mindmap(MindmapAction::SelectUp),
                            "select_down" => Action::Mindmap(MindmapAction::SelectDown),
                            "select_left" => Action::Mindmap(MindmapAction::SelectLeft),
                            "select_right" => Action::Mindmap(MindmapAction::SelectRight),
                            "create_child" => Action::Mindmap(MindmapAction::CreateChild),
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
        KeyCode::Backspace => "backspace".to_string(),
        KeyCode::Tab => "tab".to_string(),
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
