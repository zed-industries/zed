use std::ffi::OsStr;

#[cfg(target_os = "windows")]
const CREATE_NO_WINDOW: u32 = 0x0800_0000_u32;

pub fn new_std_command(program: impl AsRef<OsStr>) -> std::process::Command {
    #[allow(unused_mut)]
    let mut command = std::process::Command::new(program);

    #[cfg(target_os = "windows")]
    {
        use std::os::windows::process::CommandExt;
        command.creation_flags(CREATE_NO_WINDOW);
    }

    command
}

pub fn new_smol_command(program: impl AsRef<OsStr>) -> smol::process::Command {
    #[allow(unused_mut)]
    let mut command = smol::process::Command::new(program);

    #[cfg(target_os = "windows")]
    {
        use smol::process::windows::CommandExt;
        command.creation_flags(CREATE_NO_WINDOW);
    }

    command
}
