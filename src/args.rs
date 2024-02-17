use clap::{Parser, Subcommand};
use ratatui::backend::Backend;

use crate::{commands::{self, app::app_command::AppCommand, start_command::StartCommand, update_command::UpdateCommand}, terminal::tui::Tui};

pub trait Command {
    fn run(&self, terminal: &mut Tui) -> color_eyre::Result<()>;
}

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct RuntipiArgs {
    #[clap(subcommand)]
    pub command: Option<RuntipiMainCommand>,
}

#[derive(Debug, Subcommand)]
pub enum RuntipiMainCommand {
    /// Start your runtipi instance
    Start(StartCommand),
    /// Stop your runtipi instance
    Stop,
    /// Restart your runtipi instance
    Restart(StartCommand),
    /// Update your runtipi instance
    Update(UpdateCommand),
    /// Manage your apps
    App(AppCommand),
    /// Initiate a password reset for the admin user
    ResetPassword,
    /// Debug your runtipi instance
    Debug,
}

pub fn get_all_command_strings() -> Vec<String> {
    vec![
        "start".to_string(),
        "stop".to_string(),
        "restart".to_string(),
        "update".to_string(),
        "app".to_string(),
        "reset-password".to_string(),
        "debug".to_string(),
    ]
}

impl Command for RuntipiMainCommand {
    fn run(&self, terminal:  &mut Tui) -> color_eyre::Result<()> {
        match self {
            Self::Start(args) => args.run(terminal),
            Self::Stop => commands::stop_command::StopCommand.run(terminal),
            Self::Restart(args) => args.run(terminal),
            Self::Update(args) => args.run(terminal),
            Self::App(args) => args.run(terminal),
            Self::ResetPassword => commands::reset_password_command::ResetPasswordCommand.run(terminal),
            Self::Debug => commands::debug_command::DebugCommand.run(terminal),
        }
    }
}
