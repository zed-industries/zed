#![cfg_attr(target_os = "windows", allow(unused, dead_code))]

use anyhow::Result;
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
    },
    Proxy {
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
fn main() -> Result<()> {
    use remote_server::unix::{execute_proxy, execute_run, init};

    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Run {
            log_file,
            pid_file,
            stdin_socket,
            stdout_socket,
        }) => {
            init(Some(log_file))?;
            execute_run(pid_file, stdin_socket, stdout_socket)
        }
        Some(Commands::Proxy { identifier }) => {
            init(None)?;
            execute_proxy(identifier)
        }
        Some(Commands::Version) => {
            eprintln!("{}", env!("ZED_PKG_VERSION"));
            Ok(())
        }
        None => {
            eprintln!("usage: remote <run|proxy|version>");
            std::process::exit(1);
        }
    }
}
