#[cfg(not(target_os = "windows"))]
mod install_cli_binary;
mod register_zed_scheme;

#[cfg(not(target_os = "windows"))]
pub use install_cli_binary::{InstallCliBinary, install_cli_binary};
pub use register_zed_scheme::{RegisterZedScheme, register_zed_scheme};
