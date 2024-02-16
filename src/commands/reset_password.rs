use color_eyre::eyre;
use colored::Colorize;
use std::env;
use std::{fs::File, path::PathBuf};

use crate::args::Command;

pub struct ResetPasswordCommand;

impl Command for ResetPasswordCommand {
    fn run(&self) -> color_eyre::Result<()> {
        let root_folder: PathBuf = env::current_dir().expect("Unable to get current directory");
        let reset_password_request = File::create(root_folder.join("state").join("password-change-request"));

        match reset_password_request {
            Ok(_) => {
                println!(
                    "{} Password reset request created. Head back to the dashboard to set a new password.",
                    "✓".green()
                )
            }
            Err(_) => {
                println!(
                    "{} Unable to create password reset request. You can manually create an empty file at {} to initiate a password reset.",
                    "✗".red(),
                    root_folder.join("state").join("password-change-request").to_str().unwrap()
                );
                return Err(eyre::eyre!("Unable to create password reset request"));
            }
        }
        Ok(())
    }
}
