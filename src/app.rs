use crate::{
    action::Action,
    config::load_keymap,
    input::handle_input,
    storage::{ingest::process_incoming_folder, Storage},
    ui::dashboard::draw_dashboard,
};
use std::io::{self, Stdout};
use tui::{
    backend::CrosstermBackend,
    Terminal,
};

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let mut storage = Storage::load_or_init()?;
    process_incoming_folder(&mut storage)?;

    let keymap = load_keymap();
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    terminal.clear()?;
    draw_dashboard(&mut terminal, &storage, &keymap)?;

    loop {
        if let Some(action) = handle_input(&keymap)? {
            match action {
                Action::Quit => break,
                Action::Redraw => draw_dashboard(&mut terminal, &storage, &keymap)?,
                _ => {}
            }
        }
    }

    terminal.clear()?;
    Ok(())
}
