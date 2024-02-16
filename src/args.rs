use clap::{Parser, Subcommand};

use crate::commands::{self, app::app_command::AppCommand, start_command::StartCommand, update_command::UpdateCommand};

pub trait Command {
    fn run(&self) -> color_eyre::Result<()>;
}

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct RuntipiArgs {
    #[clap(subcommand)]
    pub command: RuntipiMainCommand,
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

impl RuntipiMainCommand {
    pub fn run(&self) -> color_eyre::Result<()> {
        match self {
            Self::Start(args) => args.run(),
            Self::Stop => commands::stop_command::StopCommand.run(),
            Self::Restart(args) => args.run(),
            Self::Update(args) => args.run(),
            Self::App(args) => args.run(),
            Self::ResetPassword => commands::reset_password_command::ResetPasswordCommand.run(),
            Self::Debug => commands::debug_command::DebugCommand.run(),
        }
    }
}
