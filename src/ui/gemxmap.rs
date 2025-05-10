use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub fn render_gemxmap<B: Backend>(f: &mut Frame<B>) -> std::io::Result<()> {
    let size = f.size();

    let block = Block::default()
        .borders(Borders::ALL)
        .title(Span::styled(
            "🧠 gemx Mindmap",
            Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
        ));

    f.render_widget(block, size);
    Ok(())
}
