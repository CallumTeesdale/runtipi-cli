use crate::{args::Command, components::spinner, terminal::tui::Tui};
use color_eyre::eyre;
use ratatui::backend::Backend;
pub struct StopCommand;

impl Command for StopCommand {
    fn run(&self, terminal: &mut Tui) -> color_eyre::Result<()> {
        let spin = spinner::new("");

        spin.set_message("Stopping containers...");

        let args = vec!["down", "--remove-orphans", "--rmi", "local"];

        let output = std::process::Command::new("docker")
            .arg("compose")
            .args(&args)
            .output()
            .map_err(|e| e.to_string());

        if let Err(e) = output {
            spin.fail("Failed to stop containers. Please try to stop them manually");
            spin.finish();

            println!("\nDebug: {}", e);
            return Err(eyre::eyre!("Failed to stop containers"));
        }

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

        spin.succeed("Tipi successfully stopped");
        spin.finish();
        Ok(())
    }
}
