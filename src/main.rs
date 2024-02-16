mod args;
mod commands;
mod components;
mod terminal;
mod utils;
use args::RuntipiArgs;
use clap::Parser;

fn main() -> color_eyre::Result<()> {
    terminal::tui::install_panic_hook();
    let args = RuntipiArgs::parse();
    args.command.run()?;
    terminal::tui::restore_terminal()?;
    Ok(())
}
