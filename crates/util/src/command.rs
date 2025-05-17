use collections::HashMap;
use std::ffi::OsStr;

#[cfg(target_os = "windows")]
use smol::process::windows::CommandExt;
#[cfg(target_os = "windows")]
use std::os::process::windows::CommandExt;

#[cfg(target_os = "windows")]
const CREATE_NO_WINDOW: u32 = 0x0800_0000_u32;

// Create a new command to run in a given environment
// (typically from the environment crate)
pub fn new_std_command(
    program: impl AsRef<OsStr>,
    env: &HashMap<String, String>,
) -> std::process::Command {
    let mut command = std::process::Command::new(program);
    command.env_clear().envs(env);
    #[cfg(target_os = "windows")]
    command.creation_flags(CREATE_NO_WINDOW);
    command
}

// Create a new command to run in a given environment
// (typically from the environment crate)
pub fn new_smol_command(
    program: impl AsRef<OsStr>,
    env: &HashMap<String, String>,
) -> smol::process::Command {
    let mut command = smol::process::Command::new(program);
    command.env_clear().envs(env);
    #[cfg(target_os = "windows")]
    command.creation_flags(CREATE_NO_WINDOW);
    command
}
