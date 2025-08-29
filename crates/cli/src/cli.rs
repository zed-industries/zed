use collections::HashMap;
pub use ipc_channel::ipc;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct IpcHandshake {
    pub requests: ipc::IpcSender<CliRequest>,
    pub responses: ipc::IpcReceiver<CliResponse>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum CliRequest {
    Open {
        paths: Vec<String>,
        urls: Vec<String>,
        diff_paths: Vec<[String; 2]>,
        wsl_args: Option<WslArgs>,
        wait: bool,
        open_new_workspace: Option<bool>,
        env: Option<HashMap<String, String>>,
        user_data_dir: Option<String>,
    },
}

#[derive(Debug, Serialize, Deserialize)]
pub enum CliResponse {
    Ping,
    Stdout { message: String },
    Stderr { message: String },
    Exit { status: i32 },
}

/// Command line arguments related to WSL remote support.
#[derive(clap::Args, Debug, Clone, Serialize, Deserialize)]
pub struct WslArgs {
    /// The name of a WSL distribution on which the given paths should be opened.
    /// If not specified, Zed will attempt to open the paths directly.
    ///
    /// Pass `-` to use the default WSL distribution.
    #[arg(long, value_name = "DISTRO", required = false)]
    pub wsl: String,

    /// The username to use when connecting to the WSL distribution, will use
    /// the default user if not specified.
    #[arg(long)]
    pub wsl_user: Option<String>,
}

/// When Zed started not as an *.app but as a binary (e.g. local development),
/// there's a possibility to tell it to behave "regularly".
pub const FORCE_CLI_MODE_ENV_VAR_NAME: &str = "ZED_FORCE_CLI_MODE";
