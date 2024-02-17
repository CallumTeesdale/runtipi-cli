use ratatui::{
    layout::Alignment,
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};

use super::app::App;

pub fn render(app: &mut App, f: &mut Frame) {
    f.render_widget(
    Paragraph::new(format!(
      "
        Press `Esc`, `Ctrl-C` or `q` to stop running.\n\
        Welcome Find documentation and guides at: https://runtipi.io\n\nVisit: http://10.0.3.152:80 to access the dashboard\n\nTipi is entirely written in TypeScript and we are looking for contributors!\n\
      "
    ))
    .block(
      Block::default()
        .title("Runtipi")
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded),
    )
    .style(Style::default().fg(Color::Green))
    .alignment(Alignment::Center),
    f.size(),
  )
}
