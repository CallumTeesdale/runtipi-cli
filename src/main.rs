mod args;
mod commands;
mod components;
mod terminal;
mod utils;
use args::{Command, RuntipiArgs};
use clap::Parser;
use terminal::{event_handler::Event, ui::update};

fn main() -> color_eyre::Result<()> {
    terminal::tui::install_panic_hook();

    let args = RuntipiArgs::parse();
    let mut tui = terminal::tui::Tui::new()?;
    let mut app = terminal::app::App::new();
    tui.enter()?;
    match args.command {
        Some(command) => command.run(&mut tui)?,
        None => {
            while !app.should_quit {
                tui.draw(&mut app)?;
                match tui.events.next()? {
                    Event::Key(key_event) => update(&mut app, key_event),
                    Event::Tick => {}
                    Event::Mouse(_) => todo!(),
                    Event::Resize(_, _) => todo!(),
                }
            }
        }
    }
    tui.exit()?;
    Ok(())
}
