mod headless_project;

#[cfg(not(windows))]
pub mod unix;

#[cfg(test)]
mod remote_editing_tests;

use clap::Subcommand;
use std::path::PathBuf;

pub use headless_project::{HeadlessAppState, HeadlessProject};

#[derive(Subcommand)]
pub enum Commands {
    Run {
        #[arg(long)]
        log_file: PathBuf,
        #[arg(long)]
        pid_file: PathBuf,
        #[arg(long)]
        stdin_socket: PathBuf,
        #[arg(long)]
        stdout_socket: PathBuf,
        #[arg(long)]
        stderr_socket: PathBuf,
    },
    Proxy {
        #[arg(long)]
        reconnect: bool,
        #[arg(long)]
        identifier: String,
    },
    Version,
}

#[cfg(not(windows))]
pub fn run(command: Commands) -> anyhow::Result<()> {
    use anyhow::Context;
    use release_channel::{RELEASE_CHANNEL, ReleaseChannel};
    use unix::{ExecuteProxyError, execute_proxy, execute_run};

    match command {
        Commands::Run {
            log_file,
            pid_file,
            stdin_socket,
            stdout_socket,
            stderr_socket,
        } => execute_run(
            log_file,
            pid_file,
            stdin_socket,
            stdout_socket,
            stderr_socket,
        ),
        Commands::Proxy {
            identifier,
            reconnect,
        } => execute_proxy(identifier, reconnect)
            .inspect_err(|err| {
                if let ExecuteProxyError::ServerNotRunning(err) = err {
                    std::process::exit(err.to_exit_code());
                }
            })
            .context("running proxy on the remote server"),
        Commands::Version => {
            let release_channel = *RELEASE_CHANNEL;
            match release_channel {
                ReleaseChannel::Stable | ReleaseChannel::Preview => {
                    println!("{}", env!("ZED_PKG_VERSION"))
                }
                ReleaseChannel::Nightly | ReleaseChannel::Dev => {
                    println!(
                        "{}",
                        option_env!("ZED_COMMIT_SHA").unwrap_or(release_channel.dev_name())
                    )
                }
            };
            Ok(())
        }
    }
}
