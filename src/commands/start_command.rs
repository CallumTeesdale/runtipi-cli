use std::env::current_dir;
use std::f64::consts::E;
use std::path::PathBuf;

use clap::Parser;
use color_eyre::{eyre, Section};
use ratatui::backend::Backend;
use ratatui::layout::{Constraint, Direction, Layout};

use crate::args::Command;
use crate::components::console_box::ConsoleBox;
use crate::components::spinner;
use crate::terminal::tui::Tui;
use crate::utils::{env, system};

#[derive(Parser, Debug)]
pub struct StartCommand {
    /// Path to a custom .env file. Can be relative to the current directory or absolute.
    #[clap(short, long)]
    pub env_file: Option<PathBuf>,
    /// Skip setting file permissions (not recommended)
    #[clap(long)]
    pub no_permissions: bool,
}

impl Command for StartCommand {
    fn run(&self, terminal: &mut Tui) -> color_eyre::Result<()> {
        let spin = spinner::new("TESTING...");
        // User permissions
        terminal.terminal.clear()?;
        terminal.terminal.draw(|frame| {
            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .margin(1)
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
                .split(frame.size());

            // Simple random step
            let simple = throbber_widgets_tui::Throbber::default();
            frame.render_widget(simple, chunks[0]);
        })?;


        if let Err(e) = system::ensure_docker() {
            spin.fail(e.to_string().as_str());
            spin.finish();
            return Err(eyre::eyre!("Failed to check user permissions")
            .suggestion("Please ensure that you have Docker installed and running"));
        }

        spin.succeed("User permissions are ok");

        // System files
        spin.set_message("Copying system files...");

        if let Err(e) = system::copy_system_files() {
            spin.fail("Failed to copy system files");
            spin.finish();
            println!("\nError: {}", e);
            return Err(
                eyre::eyre!("Failed to copy system files").suggestion("Please ensure that you have the required permissions to copy system files")
            );
        }
        spin.succeed("Copied system files");

        // Env file generation
        spin.set_message("Generating .env file...");

        if let Err(e) = env::generate_env_file(&self.env_file) {
            spin.fail("Failed to generate .env file");
            spin.finish();
            println!("\nError: {}", e);
            return Err(eyre::eyre!("Failed to generate .env file")
                .with_section(|| format!("Error: {}", e))
                .suggestion("Please ensure that you have the required permissions to generate the .env file"));
        }
        let env_map = env::get_env_map();

        spin.succeed("Generated .env file");

        spin.set_message("Ensuring file permissions... This may take a while depending on how many files there are to fix");

        if !self.no_permissions {
            if let Err(e) = system::ensure_file_permissions() {
                spin.fail(e.to_string().as_str());
                spin.finish();
                return Err(eyre::eyre!("Failed to ensure file permissions"));
            }
        }

        spin.succeed("File permissions ok");

        spin.set_message("Pulling images...");

        let root_folder: PathBuf = current_dir().expect("Unable to get current directory");

        let env_file_path = format!("{}/.env", root_folder.display());
        let output = std::process::Command::new("docker")
            .arg("compose")
            .arg("--env-file")
            .arg(&env_file_path)
            .arg("pull")
            .output();

        if let Err(e) = output {
            spin.fail("Failed to pull images");
            spin.finish();
            println!("\nError: {}", e);
            return Err(eyre::eyre!("Failed to pull images"));
        }

        if let Ok(output) = output {
            if !output.status.success() {
                spin.fail("Failed to pull images");

                let stderr = String::from_utf8_lossy(&output.stderr);
                println!("\nDebug: {}", stderr);
                return Err(eyre::eyre!("Failed to pull images"));
            }
        }
        spin.succeed("Images pulled");

        // Stop and remove containers
        spin.set_message("Stopping existing containers...");
        let container_names = vec![
            "tipi-reverse-proxy",
            "tipi-docker-proxy",
            "tipi-db",
            "tipi-redis",
            "tipi-worker",
            "tipi-dashboard",
        ];

        for container_name in container_names {
            let _ = std::process::Command::new("docker").arg("stop").arg(container_name).output();
            let _ = std::process::Command::new("docker").arg("rm").arg(container_name).output();
        }

        spin.succeed("Existing containers stopped");

        spin.set_message("Starting containers...");
        let user_compose_file = root_folder.join("user-config").join("tipi-compose.yml");

        let mut args = vec!["-f".to_string(), root_folder.join("docker-compose.yml").display().to_string()];

        if user_compose_file.exists() {
            args.push("-f".to_string());
            args.push(user_compose_file.display().to_string());
        }

        args.push("--env-file".to_string());
        args.push(env_file_path);
        args.push("up".to_string());
        args.push("--detach".to_string());
        args.push("--remove-orphans".to_string());
        args.push("--build".to_string());

        let output = std::process::Command::new("docker")
            .arg("compose")
            .args(&args)
            .output()
            .map_err(|e| e.to_string());

        if let Err(e) = output {
            spin.fail("Failed to start containers");
            spin.finish();
            println!("\nError: {}", e);
            return Err(eyre::eyre!("Failed to start containers"));
        }

        if let Ok(output) = output {
            if !output.status.success() {
                spin.fail("Failed to start containers");

                let stderr = String::from_utf8_lossy(&output.stderr);
                println!("\nDebug: {}", stderr);
                return Err(eyre::eyre!("Failed to start containers"));
            }
        }

        spin.succeed("Containers started");
        spin.finish();
        println!("\n");

        let ip_and_port = format!(
            "Visit http://{}:{} to access the dashboard",
            env_map.get("INTERNAL_IP").unwrap(),
            env_map.get("NGINX_PORT").unwrap()
        );

        let box_title = "Runtipi started successfully".to_string();
        let box_body = format!(
            "{}\n\n{}\n\n{}",
            ip_and_port,
            "Find documentation and guides at: https://runtipi.io",
            "Tipi is entirely written in TypeScript and we are looking for contributors!"
        );

        let console_box = ConsoleBox::new(box_title, box_body, 80, "green".to_string());
        console_box.print();
        Ok(())
    }
}
