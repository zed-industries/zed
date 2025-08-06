#![cfg_attr(target_os = "windows", allow(unused, dead_code))]

use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(disable_version_flag = true)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
    /// Used for SSH/Git password authentication, to remove the need for netcat as a dependency,
    /// by having Zed act like netcat communicating over a Unix socket.
    #[arg(long, hide = true)]
    askpass: Option<String>,
    /// Used for recording minidumps on crashes by having the server run a separate
    /// process communicating over a socket.
    #[arg(long, hide = true)]
    crash_handler: Option<PathBuf>,
    /// Used for loading the environment from the project.
    #[arg(long, hide = true)]
    printenv: bool,
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
    use release_channel::{RELEASE_CHANNEL, ReleaseChannel};
    use remote::proxy::ProxyLaunchError;
    use remote_server::unix::{execute_proxy, execute_run};

    let cli = Cli::parse();

    if let Some(socket_path) = &cli.askpass {
        askpass::main(socket_path);
        return;
    }

    if let Some(socket) = &cli.crash_handler {
        crashes::crash_server(socket.as_path());
        return;
    }

    if cli.printenv {
        util::shell_env::print_env();
        return;
    }

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
