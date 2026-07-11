pub use env_var::{EnvVar, bool_env_var, env_var};
use std::sync::LazyLock;

/// Whether Zed is running in stateless mode.
/// When true, Zed will use in-memory databases instead of persistent storage.
pub static ZED_STATELESS: LazyLock<bool> = bool_env_var!("ZED_STATELESS");

pub const ZED_CLI_ORIGIN_WINDOW_ID: &str = "ZED_CLI_ORIGIN_WINDOW_ID";
pub const ZED_CLI_ORIGIN_WORKSPACE_ID: &str = "ZED_CLI_ORIGIN_WORKSPACE_ID";
