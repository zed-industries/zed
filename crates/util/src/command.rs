use std::ffi::OsStr;

#[cfg(windows)]
pub fn new_std_command(exe: impl AsRef<OsStr>) -> std::process::Command {
    let mut p = std::process::Command::new(exe);
    std::os::windows::process::CommandExt::creation_flags(
        &mut p,
        windows::Win32::System::Threading::CREATE_NO_WINDOW.0,
    );
    p
}

#[cfg(not(windows))]
pub fn new_std_command(exe: impl AsRef<OsStr>) -> std::process::Command {
    std::process::Command::new(exe)
}

#[cfg(windows)]
pub fn new_smol_command(exe: impl AsRef<OsStr>) -> smol::process::Command {
    let mut p = smol::process::Command::new(exe);
    smol::process::windows::CommandExt::creation_flags(
        &mut p,
        windows::Win32::System::Threading::CREATE_NO_WINDOW.0,
    );
    p
}

#[cfg(not(windows))]
pub fn new_smol_command(exe: impl AsRef<OsStr>) -> smol::process::Command {
    smol::process::Command::new(exe)
}
