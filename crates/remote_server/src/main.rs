use clap::Parser;
use remote_server::Commands;
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

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    if let Some(socket_path) = &cli.askpass {
        askpass::main(socket_path);
        return Ok(());
    }

    if let Some(socket) = &cli.crash_handler {
        crashes::crash_server(socket.as_path());
        return Ok(());
    }

    if cli.printenv {
        util::shell_env::print_env();
        return Ok(());
    }

    #[cfg(not(windows))]
    if let Some(command) = cli.command {
        remote_server::run(command)
    } else {
        eprintln!("usage: remote <run|proxy|version>");
        std::process::exit(1);
    }

    #[cfg(windows)]
    if let Some(_) = cli.command {
        eprintln!("run is not supported on Windows");
        std::process::exit(2);
    } else {
        eprintln!("usage: remote <run|proxy|version>");
        std::process::exit(1);
    }
}
