use clap::Args;
use color_eyre::eyre;
use ratatui::backend::Backend;
use reqwest::blocking::Client;
use semver::{Error as SemverError, Version};
use std::path::PathBuf;
use std::str::FromStr;
use std::{env::current_dir, fs::File};

use crate::args::Command;
use crate::components::console_box::ConsoleBox;
use crate::terminal::tui::Tui;
use crate::utils::env;
use crate::{components::spinner, utils::system::get_architecture};
use self_update::self_replace::self_replace;
use serde::Deserialize;

#[derive(Debug, Clone)]
pub enum VersionEnum {
    Version(Version),
    Latest,
    Nightly,
}

impl FromStr for VersionEnum {
    type Err = SemverError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "latest" {
            Ok(VersionEnum::Latest)
        } else if s == "nightly" {
            Ok(VersionEnum::Nightly)
        } else {
            // Remove the 'v' prefix if present
            let version_str = if s.starts_with('v') || s.starts_with('V') { &s[1..] } else { s };

            let version = Version::parse(version_str)?;
            Ok(VersionEnum::Version(version))
        }
    }
}

impl ToString for VersionEnum {
    fn to_string(&self) -> String {
        match self {
            VersionEnum::Version(version) => version.to_string(),
            VersionEnum::Latest => "latest".to_string(),
            VersionEnum::Nightly => "nightly".to_string(),
        }
    }
}

#[derive(Deserialize, Debug)]
struct GithubRelease {
    tag_name: String,
}

#[derive(Debug, Args)]
pub struct UpdateCommand {
    /// The version to update to eg: v2.5.0 or latest
    pub version: VersionEnum,
    /// Path to a custom .env file. Can be relative to the current directory or absolute.
    #[clap(short, long)]
    pub env_file: Option<PathBuf>,
    /// Skip setting file permissions (not recommended)
    #[clap(long)]
    pub no_permissions: bool,
}

fn is_major_bump(current_version: &str, new_version: &str) -> bool {
    let current_version = current_version.split('.').collect::<Vec<&str>>();
    let new_version = new_version.split('.').collect::<Vec<&str>>();

    if current_version[0] < new_version[0] {
        return true;
    }

    false
}

impl Command for UpdateCommand {
    fn run(&self, terminal: &mut Tui) -> color_eyre::Result<()> {
        let spin = spinner::new("");

        spin.set_message("Grabbing releases from GitHub");

        let releases = self_update::backends::github::ReleaseList::configure()
            .repo_owner("runtipi")
            .repo_name("cli")
            .build()
            .unwrap();

        let fetch = releases.fetch().unwrap();

        // Find args.version in releases
        // If args.version is latest, use the latest non-prerelease version
        let wanted_version = if self.version.to_string() == "latest" {
            let url = "https://api.github.com/repos/runtipi/runtipi/releases/latest";

            let http_client = Client::builder().user_agent("reqwest").build().unwrap();
            let response = http_client.get(url).send().unwrap();

            if response.status().is_success() {
                let latest: GithubRelease = response.json().unwrap();

                latest.tag_name[1..].to_string()
            } else if self.version.to_string() == "nightly" {
                "nightly".to_string()
            } else {
                spin.fail("Failed to fetch latest release");
                spin.finish();
                return Err(eyre::eyre!("Failed to fetch latest release"));
            }
        } else {
            self.version.to_string()
        };

        let env_map = env::get_env_map();

        let current_version = env_map.get("TIPI_VERSION").unwrap().replace('v', "");
        if is_major_bump(&current_version, &wanted_version) {
            spin.fail("You are trying to update to a new major version. Please update manually using the update instructions on the website. https://runtipi.io/docs/reference/breaking-updates");
            spin.finish();
            return Err(eyre::eyre!("Major version bump"));
        }

        let release = fetch.iter().find(|r| r.version.as_str() == wanted_version);

        match release {
            Some(release) => {
                spin.succeed(format!("Found version {}", release.version).as_str());
            }
            None => {
                spin.fail(format!("Version {} not found", wanted_version).as_str());
                spin.finish();
                return Err(eyre::eyre!("Version not found"));
            }
        }

        let release = release.unwrap();

        let arch = get_architecture().unwrap_or("x86_64".to_string()).to_string();
        let arch = if arch == "arm64" { "aarch64".to_string() } else { "x86_64".to_string() };

        spin.set_message(format!("Downloading {} release", arch).as_str());

        let asset = release.asset_for(arch.as_str(), Some("linux"));

        if asset.is_none() {
            spin.fail(format!("No asset found for {} {} on release {}", arch, "linux", release.version).as_str());
            spin.finish();
            return Err(eyre::eyre!("No asset found"));
        }

        let asset = asset.unwrap();

        let current_dir = current_dir().expect("Unable to get current directory");

        let tmp_dir = tempfile::Builder::new().prefix("self_update").tempdir_in(&current_dir).unwrap();
        let tmp_tarball_path = tmp_dir.path().join(&asset.name);
        let tmp_tarball = File::create(&tmp_tarball_path).unwrap();

        let output = self_update::Download::from_url(&asset.download_url)
            .set_header(reqwest::header::ACCEPT, "application/octet-stream".parse().unwrap())
            .download_to(&tmp_tarball);

        match output {
            Ok(_) => {
                spin.succeed(format!("Downloaded {}", &asset.name).as_str());
            }
            Err(e) => {
                spin.fail("Failed to download release");
                spin.finish();
                println!("\nError: {}", e);
                return Err(eyre::eyre!("Failed to download release"));
            }
        }

        spin.set_message("Extracting tarball");

        let output = std::process::Command::new("tar")
            .arg("-xzf")
            .arg(&tmp_tarball_path)
            .arg("-C")
            .arg(&current_dir)
            .output();

        if let Err(e) = output {
            spin.fail("Failed to extract tarball");
            spin.finish();
            println!("\nError: {}", e);
            return Err(eyre::eyre!("Failed to extract tarball"));
        }
        spin.succeed("Extracted tarball");

        // asset.name with no extension
        let bin_name = asset.name.split('.').collect::<Vec<&str>>()[0];
        let new_executable_path = current_dir.join(bin_name);

        spin.set_message("Replacing old CLI");
        std::process::Command::new("chmod").arg("+x").arg(&new_executable_path);

        let result = self_replace(&new_executable_path);
        let _ = std::fs::remove_file(new_executable_path);

        if let Err(e) = result {
            spin.fail("Failed to replace old CLI");
            spin.finish();
            println!("\nError: {}", e);
            return Err(eyre::eyre!("Failed to replace old CLI"));
        }

        spin.succeed("Tipi updated successfully. Starting new CLI");

        spin.set_message("Starting Tipi... This may take a while.");

        // Start new CLI
        let mut run_args = vec!["start".to_string()];
        if self.no_permissions {
            run_args.push("--no-permissions".to_string());
        }

        let env_file = self.env_file.clone();

        if let Some(env_file) = env_file.clone() {
            if !env_file.exists() {
                spin.fail("Env file does not exist");
                spin.finish();
                return Err(eyre::eyre!("Env file does not exist"));
            }

            let env_file = env_file.canonicalize().unwrap();
            let env_file = env_file.to_str().unwrap();
            let env_file = env_file.to_string();
            run_args.push("--env-file".to_string());
            run_args.push(env_file);
        }

        // Run command start on new CLI
        let result = std::process::Command::new("./runtipi-cli").args(run_args).output();

        if let Err(e) = result {
            spin.fail("Failed to start new CLI");
            println!("\nDebug: {}", e);
            return Err(eyre::eyre!("Failed to start new CLI"));
        }

        if let Ok(output) = result {
            if !output.status.success() {
                spin.fail("Failed to start new CLI");
                println!("\nDebug: {}", String::from_utf8_lossy(&output.stderr));
            }
        }

        spin.finish();

        println!("\n");

        let ip_and_port = format!(
            "Visit http://{}:{} to access the dashboard\n\nYou are now running version {}\n\nTipi is entirely written in TypeScript and we are looking for contributors!",
            env_map.get("INTERNAL_IP").unwrap(),
            env_map.get("NGINX_PORT").unwrap(),
            release.version
        );

        let box_title = "Runtipi started successfully".to_string();

        let console_box = ConsoleBox::new(box_title, ip_and_port, 80, "green".to_string());
        console_box.print();
        Ok(())
    }
}
