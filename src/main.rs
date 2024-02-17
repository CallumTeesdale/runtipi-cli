mod args;
mod commands;
mod components;
mod terminal;
mod utils;

use args::{Command, RuntipiArgs};
use clap::Parser;
use crossterm::event;

fn main() -> color_eyre::Result<()> {
    terminal::tui::install_panic_hook();
   
    let args = RuntipiArgs::parse();
    let mut terminal = terminal::tui::init_terminal()?;
    let mut app = terminal::app::App::new();
    
    match args.command {
        Some(command) => command.run(&terminal)?,
        None => {
            let mut tui = terminal::tui::Tui::new()?;
            while !app.should_quit {
                tui.draw(&mut app)?;

                match tui.events.next()? {
                    terminal::event_handler::Event::Key(event) => {
                        if let event::KeyCode::Char('q') = event.code {
                            app.should_quit = true;
                        }
                    }
                    terminal::event_handler::Event::Tick => {}
                    terminal::event_handler::Event::Mouse(_) => todo!(),
                    terminal::event_handler::Event::Resize(_, _) => todo!(),
                }
            }
        },
    }
    terminal::tui::restore_terminal()?;
    Ok(())
}
