use std::ffi::OsStr;

#[cfg(target_os = "windows")]
const CREATE_NO_WINDOW: u32 = 0x0800_0000_u32;

#[cfg(target_os = "windows")]
pub fn new_std_command(program: impl AsRef<OsStr>) -> std::process::Command {
    use std::os::windows::process::CommandExt;

    let mut command = std::process::Command::new(program);
    command.creation_flags(CREATE_NO_WINDOW);
    command
}

#[cfg(not(target_os = "windows"))]
pub fn new_std_command(program: impl AsRef<OsStr>) -> std::process::Command {
    std::process::Command::new(program)
}

#[cfg(target_os = "windows")]
pub fn new_smol_command(program: impl AsRef<OsStr>) -> smol::process::Command {
    use smol::process::windows::CommandExt;

    let mut command = smol::process::Command::new(program);
    command.creation_flags(CREATE_NO_WINDOW);
    command
}

#[cfg(target_os = "macos")]
pub fn new_smol_command(program: impl AsRef<OsStr>) -> smol::process::Command {
    use std::os::unix::process::CommandExt;

    // Create a std::process::Command first so we can use pre_exec
    let mut std_cmd = std::process::Command::new(program);

    // WORKAROUND: Reset exception ports before exec to prevent inheritance of
    // crash handler exception ports. Due to a timing issue, child processes can
    // inherit the parent's exception ports before they're fully stabilized,
    // which can block child process spawning.
    // See: https://github.com/zed-industries/zed/issues/36754
    unsafe {
        std_cmd.pre_exec(|| {
            // Reset all exception ports to system defaults for this task.
            // This prevents the child from inheriting the parent's crash handler
            // exception ports.
            reset_exception_ports();
            Ok(())
        });
    }

    // Convert to async_process::Command via From trait
    smol::process::Command::from(std_cmd)
}

#[cfg(all(not(target_os = "windows"), not(target_os = "macos")))]
pub fn new_smol_command(program: impl AsRef<OsStr>) -> smol::process::Command {
    smol::process::Command::new(program)
}

#[cfg(target_os = "macos")]
pub fn reset_exception_ports() {
    use mach2::exception_types::{
        EXC_MASK_ALL, EXCEPTION_DEFAULT, exception_behavior_t, exception_mask_t,
    };
    use mach2::kern_return::{KERN_SUCCESS, kern_return_t};
    use mach2::mach_types::task_t;
    use mach2::port::{MACH_PORT_NULL, mach_port_t};
    use mach2::thread_status::{THREAD_STATE_NONE, thread_state_flavor_t};
    use mach2::traps::mach_task_self;

    // FFI binding for task_set_exception_ports (not exposed by mach2 crate)
    unsafe extern "C" {
        fn task_set_exception_ports(
            task: task_t,
            exception_mask: exception_mask_t,
            new_port: mach_port_t,
            behavior: exception_behavior_t,
            new_flavor: thread_state_flavor_t,
        ) -> kern_return_t;
    }

    unsafe {
        let task = mach_task_self();
        // Reset all exception ports to MACH_PORT_NULL (system default)
        // This prevents the child process from inheriting the parent's crash handler
        let kr = task_set_exception_ports(
            task,
            EXC_MASK_ALL,
            MACH_PORT_NULL,
            EXCEPTION_DEFAULT as exception_behavior_t,
            THREAD_STATE_NONE,
        );

        if kr != KERN_SUCCESS {
            // Log but don't fail - the process can still work without this workaround
            eprintln!(
                "Warning: failed to reset exception ports in child process (kern_return: {})",
                kr
            );
        }
    }
}
