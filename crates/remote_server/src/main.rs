#![cfg_attr(target_os = "windows", allow(unused, dead_code))]

use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(disable_version_flag = true)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
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

#[cfg(windows)]
fn main() {
    unimplemented!()
}

#[cfg(not(windows))]
fn main() {
    use remote::proxy::ProxyLaunchError;
    use remote_server::unix::{execute_proxy, execute_run};

    let cli = Cli::parse();

    let result = match cli.command {
        Some(Commands::Run {
            log_file,
            pid_file,
            stdin_socket,
            stdout_socket,
            stderr_socket,
        }) => execute_run(
            log_file,
            pid_file,
            stdin_socket,
            stdout_socket,
            stderr_socket,
        ),
        Some(Commands::Proxy {
            identifier,
            reconnect,
        }) => match execute_proxy(identifier, reconnect) {
            Ok(_) => Ok(()),
            Err(err) => {
                if let Some(err) = err.downcast_ref::<ProxyLaunchError>() {
                    std::process::exit(err.to_exit_code());
                }
                Err(err)
            }
        },
        Some(Commands::Version) => {
            println!("{}", env!("ZED_PKG_VERSION"));
            std::process::exit(0);
        }
        None => {
            eprintln!("usage: remote <run|proxy|version>");
            std::process::exit(1);
        }
    };
    if let Err(error) = result {
        log::error!("exiting due to error: {}", error);
        std::process::exit(1);
    }
}
