use std::ffi::OsStr;

pub mod blocking;

/// Execute commands on the Windows platform,
/// without opening a window to maintain consistency with other system behaviors.
pub struct Command;

impl Command {
    pub fn new<S: AsRef<OsStr>>(program: S) -> smol::process::Command {
        blocking::Command::new(program).into()
    }
}