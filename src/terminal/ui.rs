use std::env::args;

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, BorderType, Borders, List, ListItem, ListState, Paragraph},
    Frame,
};

use crate::{args::{self, Command}, commands};

use super::app::App;

pub fn render(app: &mut App, f: &mut Frame) {
    let size = f.size();

    // Define the layout
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(size.height), Constraint::Min(0)].as_ref())
        .split(size);

    // Create and render the Paragraph widget for the text
    let text = Paragraph::new(
        "Find documentation and guides at: https://runtipi.io\n
    \n
    Visit: http://10.0.3.152:80 to access the dashboard\n
    \n
    Tipi is entirely written in TypeScript and we are looking for contributors!\n
    \n
    commands:
    \n
    <Press 's' to start your tipi>\n
    <Press <C+s> to stop your tipi>\n
    <Press 'r' to restart your tipi>\n
    <Press 'u' to update your tipi>\n
    <Press 'a' to manage your apps>\n
    <Press 'p' to initiate a password reset for the admin user>\n
    <Press 'd' to debug your tipi>\n
    \n
    <Press 'q', 'C' <C+c> or 'Esc' to quit>
    ",
    )
    .block(
        Block::default()
            .title("Runtipi")
            .title_alignment(Alignment::Center)
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded),
    )
    .style(Style::default().fg(Color::Green))
    .alignment(Alignment::Center);
    f.render_widget(text, chunks[0]);

    // let list_items = args::get_all_command_strings();
    // // Create and render the List widget
    // let list = List::new(list_items)
    //     .block(Block::default().borders(Borders::ALL))
    //     .highlight_style(Style::default().add_modifier(Modifier::REVERSED))
    //     .highlight_symbol(">>");
    // f.render_stateful_widget(list, chunks[1], &mut app.list_state);
}

pub fn update(app: &mut App, key_event: KeyEvent) {
    match key_event.code {
        KeyCode::Esc | KeyCode::Char('q') => app.quit(),
        KeyCode::Char('c') | KeyCode::Char('C') => {
            if key_event.modifiers == KeyModifiers::CONTROL {
                app.quit()
            }
        }
        KeyCode::Char('s') => {
            if key_event.modifiers == KeyModifiers::NONE {
                let start_command = args::RuntipiMainCommand::Start(commands::start_command::StartCommand{
                    no_permissions: false,
                    env_file: None,
                });
            }
            if key_event.modifiers == KeyModifiers::CONTROL {
                let stop_command = args::RuntipiMainCommand::Stop;
            }
        }
        KeyCode::Char('r') => {}
        KeyCode::Char('u') => {}
        KeyCode::Char('a') => {}
        KeyCode::Char('p') => {}
        KeyCode::Char('d') => {}
        KeyCode::Down => {
            app.list_state.select(Some((app.list_state.selected().unwrap_or(0) + 1).min(2)));
        }
        KeyCode::Up => {
            app.list_state.select(Some((app.list_state.selected().unwrap_or(0)).saturating_sub(1)));
        }
        _ => {}
    };
}
