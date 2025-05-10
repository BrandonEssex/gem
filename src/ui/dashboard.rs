use std::io;
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    style::{Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Paragraph},
    Frame,
};
use crate::storage::Storage;
use crate::config::KeyMap;

pub fn draw_dashboard<B: Backend>(
    f: &mut Frame<B>,
    storage: &Storage,
    keymap: &KeyMap,
) -> io::Result<()> {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([Constraint::Min(3), Constraint::Length(3)].as_ref())
        .split(f.size());

    let top = Paragraph::new(vec![
        Spans::from(Span::styled(
            "📊 gemx Dashboard",
            Style::default().add_modifier(Modifier::BOLD),
        )),
        Spans::from(format!("Notes: {}", storage.notes.len())),
        Spans::from(format!("Todos: {}", storage.todos.len())),
        Spans::from(format!("Projects: {}", storage.projects.len())),
    ])
    .block(Block::default().borders(Borders::ALL).title("Overview"));

    f.render_widget(top, chunks[0]);

    let keys = keymap
        .iter()
        .map(|(context, bindings)| {
            Spans::from(format!("{:?} → {} bindings", context, bindings.len()))
        })
        .collect::<Vec<_>>();

    let bottom = Paragraph::new(keys)
        .block(Block::default().borders(Borders::ALL).title("Shortcuts"));

    f.render_widget(bottom, chunks[1]);
    Ok(())
}
