use crate::action::Action;
use crate::config::KeyMap;
use crossterm::event::{self, Event, KeyCode, KeyModifiers};
use std::collections::HashMap;

pub fn handle_input(keymap: &KeyMap) -> crossterm::Result<Option<Action>> {
    if event::poll(std::time::Duration::from_millis(200))? {
        if let Event::Key(key_event) = event::read()? {
            let key_str = key_event_to_string(key_event.code, key_event.modifiers);
            for (action, binding) in keymap {
                if *binding == key_str {
                    return Ok(Some(parse_action(action)));
                }
            }
        }
    }
    Ok(None)
}

fn key_event_to_string(code: KeyCode, mods: KeyModifiers) -> String {
    let mod_str = match mods {
        KeyModifiers::CONTROL => "ctrl+",
        KeyModifiers::ALT => "alt+",
        KeyModifiers::SHIFT => "shift+",
        _ => "",
    };
    match code {
        KeyCode::Char(c) => format!("{}{}", mod_str, c),
        KeyCode::Esc => "esc".into(),
        _ => "".into(),
    }
}

fn parse_action(key: &str) -> Action {
    match key {
        "quit" => Action::Quit,
        "toggle_help" => Action::ToggleHelp,
        "start_pomodoro" => Action::StartPomodoro,
        "stop_pomodoro" => Action::StopPomodoro,
        "toggle_timer" => Action::ToggleTimer,
        "dashboard" => Action::OpenDashboard,
        _ => Action::Redraw,
    }
}
