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
    /// CLI mode used by remote terminals to control the local Zed instance
    /// through the remote server.
    #[command(subcommand)]
    Cli(CliCommand),
}

#[derive(Subcommand)]
pub enum CliCommand {
    /// Open paths/urls in the connected local Zed window
    Open {
        /// Unique connection identifier for this remote session
        #[arg(long)]
        identifier: String,
        /// Wait for all opened items to close before exiting
        #[arg(long)]
        wait: bool,
        /// Create a new workspace window
        #[arg(short = 'n', long)]
        new: bool,
        /// Add to the current workspace (overrides --new)
        #[arg(short = 'a', long)]
        add: bool,
        /// Pairs of file paths to diff. Use --diff old --diff new
        #[arg(long, action = clap::ArgAction::Append, num_args = 2, value_names = ["OLD", "NEW"])]
        diff: Vec<String>,
        /// Paths or urls to open
        #[arg(value_name = "PATHS_OR_URLS")] 
        paths_or_urls: Vec<String>,
    },
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
        Commands::Cli(CliCommand::Open {
            identifier,
            wait,
            new,
            add,
            diff,
            paths_or_urls,
        }) => {
            use smol::io::{AsyncReadExt as _, AsyncWriteExt as _};
            // Compute cli.sock based on server state dir
            let server_dir = paths::remote_server_state_dir().join(&identifier);
            let cli_socket = server_dir.join("cli.sock");

            // Build payload and send as JSON
            #[derive(serde::Serialize)]
            struct DiffPair<'a> { old_path: &'a str, new_path: &'a str }
            #[derive(serde::Serialize)]
            struct CliOpenArgs<'a> {
                paths: Vec<&'a str>,
                urls: Vec<&'a str>,
                diff_paths: Vec<DiffPair<'a>>,
                wait: bool,
                open_new_workspace: Option<bool>,
            }
            let mut paths = Vec::new();
            let mut urls = Vec::new();
            for arg in &paths_or_urls {
                if arg.starts_with("http://")
                    || arg.starts_with("https://")
                    || arg.starts_with("file://")
                    || arg.starts_with("zed://")
                    || arg.starts_with("ssh://")
                {
                    urls.push(arg.as_str());
                } else {
                    paths.push(arg.as_str());
                }
            }
            let mut diff_pairs = Vec::new();
            for chunk in diff.chunks(2) {
                if chunk.len() == 2 {
                    diff_pairs.push(DiffPair { old_path: &chunk[0], new_path: &chunk[1] });
                }
            }
            let open_new_workspace = if new { Some(true) } else if add { Some(false) } else { None };
            let payload = CliOpenArgs { paths, urls, diff_paths: diff_pairs, wait, open_new_workspace };
            let payload = serde_json::to_vec(&payload)?;

            let status: i32 = smol::block_on(async move {
                let mut stream = smol::net::unix::UnixStream::connect(&cli_socket).await?;
                stream.write_all(&payload).await?;
                stream.flush().await?;
                // Signal EOF so server's read_to_end completes
                stream.close().await?;
                let mut resp_buf = Vec::new();
                stream.read_to_end(&mut resp_buf).await?;
                let response: serde_json::Value = serde_json::from_slice(&resp_buf).unwrap_or(serde_json::json!({"status": 1}));
                let status = response.get("status").and_then(|s| s.as_i64()).unwrap_or(1) as i32;
                anyhow::Ok(status)
            })?;
            std::process::exit(status);
        }
    }
}
