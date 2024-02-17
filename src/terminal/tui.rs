use crossterm::{
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::prelude::*;
use std::{io::stdout, panic};

use super::{app::App, event_handler::EventHandler, ui};

pub type CrosstermTerminal =
    ratatui::Terminal<ratatui::backend::CrosstermBackend<std::io::Stdout>>;

pub struct Tui {
    terminal: CrosstermTerminal,
    pub events: EventHandler,
}


impl Tui {
    pub fn new() -> color_eyre::Result<Self> {
        let terminal = init_terminal()?;
        let events = EventHandler::new(250);
        Ok(Self { terminal, events })
    }


    pub fn draw(&mut self, app: &mut App) -> color_eyre::Result<()> {
        self.terminal.draw(|frame| ui::render(app, frame))?;
        Ok(())
    }

    pub fn enter(&mut self) -> color_eyre::Result<()> {
        self.terminal.backend_mut().clear()?;
        self.terminal.backend_mut().flush()?;
        Ok(())
    }


    pub fn reset(&mut self) -> color_eyre::Result<()> {
        self.terminal.backend_mut().clear()?;
        self.terminal.backend_mut().flush()?;
        Ok(())
    }


    pub fn exit(&mut self) -> color_eyre::Result<()> {
        restore_terminal()?;
        Ok(())
    }
}

pub fn init_terminal() -> color_eyre::Result<CrosstermTerminal> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    Ok(terminal)
}

pub fn restore_terminal() -> color_eyre::Result<()> {
    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}

pub fn install_panic_hook() {
    let original_hook = panic::take_hook();
    panic::set_hook(Box::new(move |panic_info| {
        stdout().execute(LeaveAlternateScreen).unwrap();
        disable_raw_mode().unwrap();
        original_hook(panic_info);
    }));
}
