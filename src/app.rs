use crate::{
    action::{Action, MindmapAction},
    config::load_keymap,
    input::handle_input,
    storage::{ingest::process_incoming_folder, Storage},
    ui::{dashboard::draw_dashboard, gemxmap::render_gemxmap},
};
use std::io::stdout;
use tui::{backend::CrosstermBackend, Terminal};

enum ScreenMode {
    Dashboard,
    Mindmap,
}

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let mut storage = Storage::load_or_init()?;
    process_incoming_folder(&mut storage)?;

    let keymap = load_keymap();
    let stdout = stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let mut screen_mode = ScreenMode::Mindmap;

    terminal.clear()?;

    loop {
        terminal.draw(|f| match screen_mode {
            ScreenMode::Dashboard => {
                draw_dashboard(f, &storage, &keymap).ok();
            }
            ScreenMode::Mindmap => {
                render_gemxmap(f).ok();
            }
        })?;

        if let Some(action) = handle_input(&keymap)? {
            match action {
                Action::Quit => break,
                Action::Mindmap(MindmapAction::Unselect) => {
                    screen_mode = ScreenMode::Dashboard;
                }
                Action::Mindmap(_) => {
                    screen_mode = ScreenMode::Mindmap;
                }
                _ => {}
            }
        }
    }

    terminal.clear()?;
    Ok(())
}
