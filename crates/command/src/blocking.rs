
use std::ffi::OsStr;
#[cfg(windows)]
use std::os::windows::process::CommandExt;

#[cfg(windows)]
const CREATE_NO_WINDOW: u32 = 0x08000000;

/// Execute commands on the Windows platform,
/// without opening a window to maintain consistency with other system behaviors.
pub struct Command;

impl Command {
    pub fn new<S: AsRef<OsStr>>(program: S) -> std::process::Command {
        let mut inner = std::process::Command::new(program);
        #[cfg(windows)]
        inner.creation_flags(CREATE_NO_WINDOW);

        inner
    }
}

#[test]
fn should_work() {
    let output = Command::new("cmd")
        .args(["/C", "echo hello"])
        .output()
        .unwrap();
    assert_eq!("hello\r\n", String::from_utf8(output.stdout).unwrap());
}
