use clap::{Parser, Subcommand};
use error::Error;
use std::path::Path;
use systemd_service::{ServiceConfig, SystemdService};
use utils::replace_file::self_replace;

#[derive(Parser)]
#[command(name = "mplat actix web server")]
#[command(about = "mplat actix web server", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Run server
    Run,

    /// Show tool version
    Version,

    /// Install systemd service
    Install {
        /// bin file path
        #[arg(short, long)]
        bin: String,

        /// log file path
        #[arg(short, long)]
        log_file: String,

        /// execute user name
        #[arg(short, long)]
        user: Option<String>,

        /// execute group name
        #[arg(short, long)]
        group: Option<String>,
    },

    /// Update bin
    Update {
        /// bin file path
        #[arg(short, long)]
        bin: String,
    },
}

pub fn systemd_service(
    bin: &str,
    log_file: &str,
    user: Option<&str>,
    group: Option<&str>,
) -> Result<SystemdService, Error> {
    let user = user.unwrap_or("www-data");
    let group = group.unwrap_or("www-data");
    let config = ServiceConfig::new("mplat-server", bin, "mplat rust server")
        .user(user)
        .group(group)
        .restart("always")
        .restart_sec(10)
        .after(vec![
            "network.target".to_string(),
            "nginx.service".to_string(),
        ])
        .environment(vec![
            ("RUST_LOG".to_string(), "info".to_string()),
            ("PORT".to_string(), "6090".to_string()),
        ])
        .log_file(log_file);

    let systemd = SystemdService::new(config);
    Ok(systemd)
}

pub fn install_systemd_service(
    bin: &str,
    log_file: &str,
    user: Option<&str>,
    group: Option<&str>,
) -> Result<(), Error> {
    let systemd = systemd_service(bin, log_file, user, group)?;

    let content = systemd.generate();
    println!("Generated service file:\n{}", content);

    systemd.install_and_enable()?;
    systemd.start()?;

    Ok(())
}

pub fn update(bin: &str) -> Result<(), Error> {
    let systemd = systemd_service(bin, ".", None, None)?;
    systemd.stop()?;
    self_replace(Path::new(bin))?;
    systemd.restart()?;
    Ok(())
}
