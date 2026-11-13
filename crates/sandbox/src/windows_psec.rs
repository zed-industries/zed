//! Native Windows sandbox built on the Process Security Environment (PSEC) API.

use std::ffi::{CStr, OsStr, c_void};
use std::os::windows::io::{AsRawHandle as _, FromRawHandle as _, OwnedHandle};
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

use anyhow::{Context as _, Result, anyhow, ensure};
use base64::Engine as _;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use flatbuffers::{FlatBufferBuilder, VOffsetT};
use windows::Win32::Foundation::{HANDLE, HANDLE_FLAG_INHERIT, SetHandleInformation, TRUE};
use windows::Win32::Storage::FileSystem::{GetDriveTypeW, GetLogicalDrives};
use windows::Win32::System::Console::{
    GetConsoleWindow, GetStdHandle, STD_ERROR_HANDLE, STD_INPUT_HANDLE, STD_OUTPUT_HANDLE,
    SetConsoleCtrlHandler,
};
use windows::Win32::System::Environment::GetCommandLineW;
use windows::Win32::System::JobObjects::{
    JOB_OBJECT_LIMIT_BREAKAWAY_OK, JOB_OBJECT_LIMIT_SILENT_BREAKAWAY_OK,
    JOBOBJECT_EXTENDED_LIMIT_INFORMATION, JobObjectExtendedLimitInformation,
    QueryInformationJobObject,
};
use windows::Win32::System::LibraryLoader::{
    GetProcAddress, LOAD_LIBRARY_SEARCH_SYSTEM32, LoadLibraryExW,
};
use windows::Win32::System::Threading::{
    CREATE_BREAKAWAY_FROM_JOB, CREATE_NO_WINDOW, CreateProcessW, DeleteProcThreadAttributeList,
    EXTENDED_STARTUPINFO_PRESENT, GetExitCodeProcess, INFINITE, InitializeProcThreadAttributeList,
    LPPROC_THREAD_ATTRIBUTE_LIST, PROC_THREAD_ATTRIBUTE_HANDLE_LIST, PROCESS_INFORMATION,
    STARTF_USESTDHANDLES, STARTUPINFOEXW, STARTUPINFOW, UpdateProcThreadAttribute,
    WaitForSingleObject,
};
use windows::Win32::System::WindowsProgramming::{DRIVE_FIXED, PROC_THREAD_ATTRIBUTE_INPUT};
use windows_core::{BOOL, HRESULT, PCSTR, PCWSTR, PWSTR};

/// Argv flag that marks a re-exec of Zed as the PSEC launcher.
pub(crate) const LAUNCHER_FLAG: &str = "--windows-sandbox-launcher";

/// Encode the crate's filesystem/network intent as a PSEC 1.0 FlatBuffer.
///
/// The layout follows the PSEC FlatBuffers schema:
/// <https://github.com/microsoft/mxc/blob/main/external/windows-sdk/ProcessSecurityEnvironment.fbs>.
pub(crate) fn build_spec(
    allow_fs_write: bool,
    writable_paths: &[&Path],
    protected_paths: &[&Path],
    allow_network: bool,
) -> Result<Vec<u8>> {
    let roots = fixed_drive_roots();
    let mut read_write = Vec::new();
    if allow_fs_write {
        // A read-write grant on a volume root does not apply to anything
        // inside it, so grant each top-level directory instead.
        for root in &roots {
            read_write.extend(top_level_directories(root));
        }
    } else {
        for path in writable_paths {
            read_write.push(
                dunce::canonicalize(path)
                    .with_context(|| format!("invalid writable path `{}`", path.display()))?,
            );
        }
    }

    let mut read_only = roots;
    for path in protected_paths {
        match dunce::canonicalize(path) {
            Ok(path) => read_only.push(path),
            // Like Bubblewrap, PSEC can only protect a path that exists when
            // the environment is created.
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
            Err(error) => {
                return Err(error)
                    .with_context(|| format!("invalid protected path `{}`", path.display()));
            }
        }
    }
    log::debug!(
        "[sandbox] PSEC policy: read-write {read_write:?}, read-only {read_only:?}, network {allow_network}"
    );

    // Vtable offsets (`4 + 2 * field index`) and enum values from the schema.
    const VERSION: VOffsetT = 4;
    const CAPABILITIES: VOffsetT = 6;
    const FS_READ_WRITE: VOffsetT = 12;
    const FS_READ_ONLY: VOffsetT = 14;
    const NETWORK_POLICY: VOffsetT = 18;
    const NETWORK_POLICY_EGRESS: VOffsetT = 6;
    const ENDPOINT_POLICY_DEFAULT_ACTION: VOffsetT = 4;
    const FILTER_ACTION_DENY: i8 = 0;
    const FILTER_ACTION_ALLOW: i8 = 1;

    let mut builder = FlatBufferBuilder::with_capacity(1024);
    // Child objects must be finished before the table that refers to them is started.
    let mut path_vector = |paths: &[PathBuf]| {
        (!paths.is_empty()).then(|| {
            let paths: Vec<_> = paths
                .iter()
                .map(|path| builder.create_string(&path.to_string_lossy()))
                .collect();
            builder.create_vector(&paths)
        })
    };
    let read_write = path_vector(&read_write);
    let read_only = path_vector(&read_only);
    let capabilities =
        allow_network.then(|| builder.create_string("internetClient,privateNetworkClientServer"));

    let egress = builder.start_table();
    let default_action = if allow_network {
        FILTER_ACTION_ALLOW
    } else {
        FILTER_ACTION_DENY
    };
    builder.push_slot(
        ENDPOINT_POLICY_DEFAULT_ACTION,
        default_action,
        FILTER_ACTION_DENY,
    );
    let egress = builder.end_table(egress);

    let network_policy = builder.start_table();
    builder.push_slot_always(NETWORK_POLICY_EGRESS, egress);
    let network_policy = builder.end_table(network_policy);

    let root = builder.start_table();
    // The inline `SchemaVersion { major: 1, minor: 0 }` struct: two
    // little-endian `uint16`s, written as one `u32`.
    builder.push_slot_always::<u32>(VERSION, 1);
    if let Some(capabilities) = capabilities {
        builder.push_slot_always(CAPABILITIES, capabilities);
    }
    if let Some(read_write) = read_write {
        builder.push_slot_always(FS_READ_WRITE, read_write);
    }
    if let Some(read_only) = read_only {
        builder.push_slot_always(FS_READ_ONLY, read_only);
    }
    builder.push_slot_always(NETWORK_POLICY, network_policy);
    let root = builder.end_table(root);
    builder.required(root, VERSION, "version");
    builder.finish(root, Some("PSEC"));
    Ok(builder.finished_data().to_vec())
}

/// Wrap `program args` so it runs through the launcher under `spec`.
pub(crate) fn wrap_invocation(
    spec: &[u8],
    program: String,
    args: Vec<String>,
) -> Result<(String, Vec<String>)> {
    let launcher = std::env::current_exe().context("failed to resolve the Zed executable")?;
    let launcher = launcher
        .to_str()
        .ok_or_else(|| {
            anyhow!(
                "Zed executable path is not valid UTF-8: {}",
                launcher.display()
            )
        })?
        .to_string();
    let mut launcher_args = Vec::with_capacity(args.len() + 4);
    launcher_args.push(LAUNCHER_FLAG.to_string());
    launcher_args.push(URL_SAFE_NO_PAD.encode(spec));
    launcher_args.push(program);
    launcher_args.extend(args);
    Ok((launcher, launcher_args))
}

pub(crate) fn check_available() -> Result<()> {
    ensure!(
        exports()?.supports_version(1, 0)?,
        "this Windows build does not support Process Security Environment 1.0"
    );
    Ok(())
}

/// If this process was started as the launcher, run the sandboxed command and
/// exit with its exit code. Returns normally otherwise.
pub(crate) fn run_launcher_if_invoked() {
    if std::env::args_os().nth(1).as_deref() != Some(OsStr::new(LAUNCHER_FLAG)) {
        return;
    }
    let exit_code = match unsafe { GetCommandLineW().to_string() }
        .context("the command line is not valid UTF-16")
        .and_then(|command_line| run_launcher(&command_line))
    {
        Ok(exit_code) => exit_code as i32,
        Err(error) => {
            eprintln!("zed: failed to start the sandboxed command: {error:#}");
            126
        }
    };
    std::process::exit(exit_code);
}

/// Run the command after `<launcher> <flag> <spec> ` under the encoded spec.
fn run_launcher(command_line: &str) -> Result<u32> {
    let (_, rest) = command_line
        .split_once(LAUNCHER_FLAG)
        .context("missing launcher flag")?;
    let (spec, command) = rest
        .trim_start()
        .split_once(' ')
        .context("missing sandboxed command")?;
    let spec = URL_SAFE_NO_PAD
        .decode(spec)
        .context("malformed sandbox policy")?;

    // Ctrl+C reaches every process attached to the console.
    // The launcher shouldn't exit on Ctrl+C so that it can report its exit code to Zed.
    unsafe extern "system" fn ignore_console_control(_: u32) -> BOOL {
        TRUE
    }
    unsafe { SetConsoleCtrlHandler(Some(ignore_console_control), true) }
        .context("failed to install the console control handler")?;

    let environment = SecurityEnvironment::create(&spec)?;

    // PSEC launch doesn't copy standard handles into the child by default, so pass them explicitly.
    let std_handles = [STD_INPUT_HANDLE, STD_OUTPUT_HANDLE, STD_ERROR_HANDLE]
        .map(|kind| unsafe { GetStdHandle(kind) }.unwrap_or_default());
    let mut inherited_handles = Vec::new();
    for handle in std_handles {
        if handle.is_invalid() || handle.0.is_null() || inherited_handles.contains(&handle) {
            continue;
        }
        unsafe { SetHandleInformation(handle, HANDLE_FLAG_INHERIT.0, HANDLE_FLAG_INHERIT) }
            .context("failed to make a std handle inheritable")?;
        inherited_handles.push(handle);
    }

    let attribute_count = 1 + u32::from(!inherited_handles.is_empty());
    let mut size = 0;
    // The sizing call always fails with `ERROR_INSUFFICIENT_BUFFER`; it only reports `size`.
    unsafe { InitializeProcThreadAttributeList(None, attribute_count, None, &mut size) }.ok();
    let mut attribute_storage = vec![0usize; size.div_ceil(std::mem::size_of::<usize>())];
    let attribute_list = LPPROC_THREAD_ATTRIBUTE_LIST(attribute_storage.as_mut_ptr().cast());
    unsafe {
        InitializeProcThreadAttributeList(Some(attribute_list), attribute_count, None, &mut size)
    }
    .context("failed to initialize the process attribute list")?;
    unsafe {
        UpdateProcThreadAttribute(
            attribute_list,
            0,
            // `ProcThreadAttributeSecurityEnvironment` (35).
            35 | PROC_THREAD_ATTRIBUTE_INPUT as usize,
            Some((&raw const environment.0).cast()),
            std::mem::size_of::<HANDLE>(),
            None,
            None,
        )
    }
    .context("failed to attach the process security environment")?;
    if !inherited_handles.is_empty() {
        unsafe {
            UpdateProcThreadAttribute(
                attribute_list,
                0,
                PROC_THREAD_ATTRIBUTE_HANDLE_LIST as usize,
                Some(inherited_handles.as_ptr().cast()),
                std::mem::size_of_val(inherited_handles.as_slice()),
                None,
                None,
            )
        }
        .context("failed to set the inherited handle list")?;
    }

    let startup_info = STARTUPINFOEXW {
        StartupInfo: STARTUPINFOW {
            cb: std::mem::size_of::<STARTUPINFOEXW>() as u32,
            dwFlags: STARTF_USESTDHANDLES,
            hStdInput: std_handles[0],
            hStdOutput: std_handles[1],
            hStdError: std_handles[2],
            ..Default::default()
        },
        lpAttributeList: attribute_list,
    };

    let mut command_line = wide(command);
    let mut creation_flags = EXTENDED_STARTUPINFO_PRESENT;
    if unsafe { GetConsoleWindow() }.is_invalid() {
        creation_flags |= CREATE_NO_WINDOW;
    }

    // To launch a packaged (MSIX) app with PSEC, the child process needs to break away from any job that's present.
    // The job might not allow breakaway; we'll only try if it's allowed.
    // https://github.com/microsoft/mxc/issues/1754
    if current_job_limits().is_some_and(|flags| flags & JOB_OBJECT_LIMIT_BREAKAWAY_OK.0 != 0) {
        creation_flags |= CREATE_BREAKAWAY_FROM_JOB;
    }

    let mut process_information = PROCESS_INFORMATION::default();
    let created = unsafe {
        CreateProcessW(
            PCWSTR::null(),
            Some(PWSTR(command_line.as_mut_ptr())),
            None,
            None,
            !inherited_handles.is_empty(),
            creation_flags,
            None,
            PCWSTR::null(),
            &startup_info.StartupInfo,
            &mut process_information,
        )
    };
    unsafe { DeleteProcThreadAttributeList(attribute_list) };
    created.with_context(|| format!("failed to start `{command}` in the sandbox"))?;
    let process = unsafe { OwnedHandle::from_raw_handle(process_information.hProcess.0) };
    let _thread = unsafe { OwnedHandle::from_raw_handle(process_information.hThread.0) };

    unsafe { WaitForSingleObject(HANDLE(process.as_raw_handle()), INFINITE) };
    let mut exit_code = 0u32;
    unsafe { GetExitCodeProcess(HANDLE(process.as_raw_handle()), &mut exit_code) }
        .context("failed to read the sandboxed command's exit code")?;
    Ok(exit_code)
}

/// The limit flags of the job containing this process, or `None` outside a job.
fn current_job_limits() -> Option<u32> {
    let mut limits = JOBOBJECT_EXTENDED_LIMIT_INFORMATION::default();
    unsafe {
        QueryInformationJobObject(
            None,
            JobObjectExtendedLimitInformation,
            (&raw mut limits).cast(),
            std::mem::size_of::<JOBOBJECT_EXTENDED_LIMIT_INFORMATION>() as u32,
            None,
        )
    }
    .ok()?;
    Some(limits.BasicLimitInformation.LimitFlags.0)
}

/// Whether the launcher can start a packaged (MSIX) app as the sandboxed command.
pub fn can_launch_packaged_apps() -> bool {
    current_job_limits().is_none_or(|flags| {
        flags & (JOB_OBJECT_LIMIT_BREAKAWAY_OK.0 | JOB_OBJECT_LIMIT_SILENT_BREAKAWAY_OK.0) != 0
    })
}

fn fixed_drive_roots() -> Vec<PathBuf> {
    let drives = unsafe { GetLogicalDrives() };
    (0..26u8)
        .filter(|index| drives & (1 << index) != 0)
        .map(|index| format!("{}:\\", char::from(b'A' + index)))
        .filter(|root| {
            let root = wide(root);
            unsafe { GetDriveTypeW(PCWSTR(root.as_ptr())) == DRIVE_FIXED }
        })
        .map(PathBuf::from)
        .collect()
}

fn top_level_directories(root: &Path) -> Vec<PathBuf> {
    let entries = match std::fs::read_dir(root) {
        Ok(entries) => entries,
        Err(error) => {
            log::warn!(
                "[sandbox] failed to list {} for writable grants: {error}",
                root.display()
            );
            return Vec::new();
        }
    };
    entries
        .filter_map(|entry| entry.ok())
        // Junctions such as `C:\Documents and Settings` report as symlinks and
        // are skipped; their targets are granted directly.
        .filter(|entry| entry.file_type().is_ok_and(|file_type| file_type.is_dir()))
        .map(|entry| entry.path())
        .collect()
}

fn wide(text: &str) -> Vec<u16> {
    text.encode_utf16().chain(Some(0)).collect()
}

type RawExport = unsafe extern "system" fn() -> isize;
type CreateProcessSecurityEnvironmentFn = unsafe extern "system" fn(
    sandbox_specification: *const c_void,
    sandbox_specification_size: u32,
    flags: u32,
    process_security_environment: *mut HANDLE,
) -> HRESULT;
type CloseProcessSecurityEnvironmentFn =
    unsafe extern "system" fn(process_security_environment: HANDLE);
type IsProcessSecurityEnvironmentVersionSupportedFn =
    unsafe extern "system" fn(major: u32, available: *mut u8, minor: *mut u32) -> HRESULT;

/// The `processmodel.dll` exports, which only exist on recent Windows builds.
struct Exports {
    create: CreateProcessSecurityEnvironmentFn,
    close: CloseProcessSecurityEnvironmentFn,
    version_supported: Option<IsProcessSecurityEnvironmentVersionSupportedFn>,
}

fn exports() -> Result<&'static Exports> {
    static EXPORTS: OnceLock<Option<Exports>> = OnceLock::new();
    EXPORTS
        .get_or_init(|| unsafe {
            let dll = wide("processmodel.dll");
            let module =
                LoadLibraryExW(PCWSTR(dll.as_ptr()), None, LOAD_LIBRARY_SEARCH_SYSTEM32).ok()?;
            let export = |name: &CStr| GetProcAddress(module, PCSTR(name.as_ptr().cast()));
            Some(Exports {
                create: std::mem::transmute::<RawExport, CreateProcessSecurityEnvironmentFn>(
                    export(c"CreateProcessSecurityEnvironment")?,
                ),
                close: std::mem::transmute::<RawExport, CloseProcessSecurityEnvironmentFn>(
                    export(c"CloseProcessSecurityEnvironment")?,
                ),
                version_supported: export(c"IsProcessSecurityEnvironmentVersionSupported").map(
                    |function| {
                        std::mem::transmute::<RawExport, IsProcessSecurityEnvironmentVersionSupportedFn>(
                            function,
                        )
                    },
                ),
            })
        })
        .as_ref()
        .context("Process Security Environment is unavailable: processmodel.dll is missing or incomplete")
}

impl Exports {
    fn supports_version(&self, major: u32, minor: u32) -> Result<bool> {
        // Builds that predate the version query only implement 1.0.
        let Some(version_supported) = self.version_supported else {
            return Ok(major == 1 && minor == 0);
        };
        let mut available = 0u8;
        let mut supported_minor = 0u32;
        unsafe { version_supported(major, &mut available, &mut supported_minor) }
            .ok()
            .context("IsProcessSecurityEnvironmentVersionSupported failed")?;
        Ok(available != 0 && supported_minor >= minor)
    }
}

/// An open `HPROCESS_SECURITY_ENVIRONMENT`, closed on drop.
struct SecurityEnvironment(HANDLE);

impl SecurityEnvironment {
    fn create(spec: &[u8]) -> Result<Self> {
        const TERMINATE_ON_CLOSE: u32 = 1;
        let size = u32::try_from(spec.len()).context("sandbox policy is too large")?;
        let mut handle = HANDLE::default();
        unsafe { (exports()?.create)(spec.as_ptr().cast(), size, TERMINATE_ON_CLOSE, &mut handle) }
            .ok()
            .context("CreateProcessSecurityEnvironment failed")?;
        Ok(Self(handle))
    }
}

impl Drop for SecurityEnvironment {
    fn drop(&mut self) {
        if let Ok(exports) = exports() {
            unsafe { (exports.close)(self.0) };
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Run `cmd /c <script>` through the launcher in-process. Returns `None`
    /// (skipping the test) when this host has no PSEC support.
    fn run_sandboxed(spec: &[u8], script: &str) -> Option<u32> {
        if let Err(error) = check_available() {
            eprintln!("skipping PSEC test: {error:#}");
            return None;
        }
        let command_line = format!(
            "zed.exe {LAUNCHER_FLAG} {} cmd.exe /c {script}",
            URL_SAFE_NO_PAD.encode(spec)
        );
        Some(run_launcher(&command_line).expect("launcher failed"))
    }

    struct Layout {
        project: tempfile::TempDir,
        outside: tempfile::TempDir,
    }

    impl Layout {
        fn new() -> Self {
            let layout = Self {
                project: tempfile::tempdir().unwrap(),
                outside: tempfile::tempdir().unwrap(),
            };
            std::fs::create_dir(layout.protected()).unwrap();
            std::fs::write(layout.protected().join("config"), "protected").unwrap();
            std::fs::write(layout.outside.path().join("secret.txt"), "outside").unwrap();
            layout
        }

        fn protected(&self) -> PathBuf {
            self.project.path().join(".git")
        }

        fn spec(&self, allow_fs_write: bool) -> Vec<u8> {
            build_spec(
                allow_fs_write,
                &[self.project.path()],
                &[&self.protected()],
                false,
            )
            .unwrap()
        }
    }

    #[test]
    fn restricted_policy_confines_writes_and_protects_paths() {
        let layout = Layout::new();
        let spec = layout.spec(false);
        let project = layout.project.path();
        let protected = layout.protected();
        let outside = layout.outside.path();

        let Some(exit_code) = run_sandboxed(
            &spec,
            &format!("echo x> {}", project.join("a.txt").display()),
        ) else {
            return;
        };
        assert_eq!(exit_code, 0, "project should be writable");
        assert!(project.join("a.txt").exists());
        assert_eq!(
            run_sandboxed(&spec, "exit 7"),
            Some(7),
            "exit codes should pass through"
        );

        // `%TEMP%` is expanded inside the sandbox, where PSEC points it at
        // the environment's own AppContainer folder.
        assert_eq!(
            run_sandboxed(&spec, r"echo x> %TEMP%\t.txt"),
            Some(0),
            "PSEC's temp dir should be writable"
        );
        assert_eq!(
            run_sandboxed(
                &spec,
                &format!("type {}", outside.join("secret.txt").display())
            ),
            Some(0),
            "reads should be allowed everywhere"
        );
        assert_eq!(
            run_sandboxed(
                &spec,
                &format!("type {}", protected.join("config").display())
            ),
            Some(0),
            "protected paths should stay readable"
        );

        assert_ne!(
            run_sandboxed(
                &spec,
                &format!("echo x> {}", outside.join("b.txt").display())
            ),
            Some(0)
        );
        assert!(
            !outside.join("b.txt").exists(),
            "outside should be read-only"
        );
        assert_ne!(
            run_sandboxed(
                &spec,
                &format!("echo x>> {}", protected.join("config").display())
            ),
            Some(0)
        );
        assert_ne!(
            run_sandboxed(&spec, &format!("ren {} moved", protected.display())),
            Some(0)
        );
        assert_eq!(
            std::fs::read_to_string(protected.join("config")).unwrap(),
            "protected",
            "protected paths should not be writable or movable"
        );
    }

    #[test]
    fn unrestricted_policy_allows_writes_but_protects_paths() {
        let layout = Layout::new();
        let spec = layout.spec(true);
        let protected = layout.protected();
        let outside = layout.outside.path();

        let Some(exit_code) = run_sandboxed(
            &spec,
            &format!("echo x> {}", outside.join("b.txt").display()),
        ) else {
            return;
        };
        assert_eq!(exit_code, 0, "writes outside the project should be allowed");
        assert!(outside.join("b.txt").exists());

        assert_ne!(
            run_sandboxed(
                &spec,
                &format!("echo x>> {}", protected.join("config").display())
            ),
            Some(0)
        );
        assert_eq!(
            std::fs::read_to_string(protected.join("config")).unwrap(),
            "protected"
        );
    }
}
