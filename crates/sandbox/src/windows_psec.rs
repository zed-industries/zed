//! Native Windows sandbox built on the Process Security Environment (PSEC) API.

use std::ffi::{CStr, OsStr, OsString, c_void};
use std::os::windows::ffi::{OsStrExt as _, OsStringExt as _};
use std::os::windows::io::{AsRawHandle as _, FromRawHandle as _, OwnedHandle};
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

use crate::{FsSetup, HostFilesystemLocation};

use anyhow::{Context as _, Result, anyhow, ensure};
use base64::Engine as _;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use flatbuffers::{FlatBufferBuilder, ForwardsUOffset, VOffsetT, Vector};
use windows::Win32::Foundation::{
    ERROR_ACCESS_DENIED, HANDLE, HANDLE_FLAG_INHERIT, SetHandleInformation, TRUE,
};
use windows::Win32::Storage::FileSystem::{
    CreateFileW, FILE_FLAG_BACKUP_SEMANTICS, FILE_LIST_DIRECTORY, FILE_NAME_NORMALIZED,
    FILE_SHARE_READ, FILE_SHARE_WRITE, GetFinalPathNameByHandleW, OPEN_EXISTING,
};
use windows::Win32::Storage::FileSystem::{GetDriveTypeW, GetLogicalDrives};
use windows::Win32::System::Console::{
    GetConsoleWindow, GetStdHandle, STD_ERROR_HANDLE, STD_INPUT_HANDLE, STD_OUTPUT_HANDLE,
    SetConsoleCtrlHandler,
};
use windows::Win32::System::Diagnostics::Debug::{
    GetErrorMode, SEM_NOGPFAULTERRORBOX, SetErrorMode, THREAD_ERROR_MODE,
};
use windows::Win32::System::Environment::{
    FreeEnvironmentStringsW, GetCommandLineW, GetEnvironmentStringsW,
};
use windows::Win32::System::JobObjects::{
    JOB_OBJECT_LIMIT_BREAKAWAY_OK, JOB_OBJECT_UILIMIT_READCLIPBOARD,
    JOB_OBJECT_UILIMIT_WRITECLIPBOARD, JOBOBJECT_EXTENDED_LIMIT_INFORMATION,
    JobObjectExtendedLimitInformation, QueryInformationJobObject,
};
use windows::Win32::System::LibraryLoader::{
    GetProcAddress, LOAD_LIBRARY_SEARCH_SYSTEM32, LoadLibraryExW,
};
use windows::Win32::System::Threading::{
    CREATE_BREAKAWAY_FROM_JOB, CREATE_NO_WINDOW, CREATE_UNICODE_ENVIRONMENT, CreateProcessW,
    DeleteProcThreadAttributeList, EXTENDED_STARTUPINFO_PRESENT, GetExitCodeProcess, INFINITE,
    InitializeProcThreadAttributeList, LPPROC_THREAD_ATTRIBUTE_LIST,
    PROC_THREAD_ATTRIBUTE_HANDLE_LIST, PROCESS_INFORMATION, STARTF_USESTDHANDLES, STARTUPINFOEXW,
    STARTUPINFOW, UpdateProcThreadAttribute, WaitForSingleObject,
};
use windows::Win32::System::WindowsProgramming::{DRIVE_FIXED, PROC_THREAD_ATTRIBUTE_INPUT};
use windows_core::{BOOL, HRESULT, PCSTR, PCWSTR, PWSTR};

/// Argv flag that makes the Zed CLI act as the PSEC launcher.
pub(crate) const LAUNCHER_FLAG: &str = "--windows-sandbox-launcher";

/// Vtable offset of the spec's `fs_read_write` field, which the launcher also
/// reads back to pin the granted folders.
const FS_READ_WRITE: VOffsetT = 12;

/// The network access a PSEC-sandboxed command gets.
#[derive(Clone, Copy, Debug)]
pub(crate) enum NetworkAccess {
    /// All network access blocked.
    None,
    /// Only `127.0.0.1:<port>`, the in-process HTTP/HTTPS proxy.
    LocalhostPort(u16),
    /// All outbound network access allowed.
    All,
}

/// Turn the crate's filesystem/network intent into a PSEC 1.0 policy.
pub(crate) fn build_spec(fs: &FsSetup, network: NetworkAccess) -> Result<Vec<u8>> {
    let paths = policy_paths(fs);
    log::debug!("[sandbox] PSEC policy: {paths:?}, network {network:?}");
    encode_spec(&paths, network)
}

/// The paths a PSEC policy grants or denies.
#[derive(Debug, Default)]
struct PolicyPaths {
    read_write: Vec<PathBuf>,
    read_only: Vec<PathBuf>,
    denied: Vec<PathBuf>,
}

/// The paths that enforce `fs`. In PSEC the most specific path wins.
fn policy_paths(fs: &FsSetup) -> PolicyPaths {
    let paths = |locations: &[HostFilesystemLocation]| -> Vec<PathBuf> {
        locations
            .iter()
            .map(|location| location.windows_canonical_path().to_path_buf())
            .collect()
    };
    let writable = paths(&fs.writable_paths);
    let protected = paths(&fs.protected_paths);
    let denied = paths(&fs.paths_disallowing_reads);
    let allowing = paths(&fs.paths_allowing_reads);
    let is_within = |path: &Path, ancestors: &[PathBuf]| {
        ancestors.iter().any(|ancestor| path.starts_with(ancestor))
    };

    let roots = fixed_drive_roots();
    let mut read_write: Vec<PathBuf> = if fs.allow_fs_write {
        // A read-write grant on a volume root doesn't apply to anything inside it.
        roots
            .iter()
            .flat_map(|root| top_level_directories(root))
            .collect()
    } else {
        writable.clone()
    };
    // Reads are allowed everywhere. Like Bubblewrap, PSEC can only protect a path
    // that exists when the environment is created.
    let mut read_only: Vec<PathBuf> = roots
        .into_iter()
        .chain(
            protected
                .iter()
                .filter(|path| !matches!(path.try_exists(), Ok(false)))
                .cloned(),
        )
        .collect();

    // A path allowing reads gets the access the rest of the policy gives it.
    for path in &allowing {
        if (fs.allow_fs_write || is_within(path, &writable)) && !is_within(path, &protected) {
            read_write.push(path.clone());
        } else {
            read_only.push(path.clone());
        }
    }

    // A grant inside a denied path would override the deny, unless a path allowing
    // reads covers it.
    let is_hidden = |path: &PathBuf| is_within(path, &denied) && !is_within(path, &allowing);
    read_write.retain(|path| !is_hidden(path));
    read_only.retain(|path| !is_hidden(path));
    PolicyPaths {
        read_write,
        read_only,
        denied,
    }
}

/// Encode a PSEC 1.0 policy FlatBuffer.
///
/// The layout follows the PSEC FlatBuffers schema:
/// <https://github.com/microsoft/mxc/blob/main/external/windows-sdk/ProcessSecurityEnvironment.fbs>.
fn encode_spec(paths: &PolicyPaths, network: NetworkAccess) -> Result<Vec<u8>> {
    // Vtable offsets (`4 + 2 * field index`) and enum values from the schema.
    const VERSION: VOffsetT = 4;
    const CAPABILITIES: VOffsetT = 6;
    const UI_RESTRICTIONS: VOffsetT = 10;
    const FS_READ_ONLY: VOffsetT = 14;
    const FS_DENY: VOffsetT = 16;
    const NETWORK_POLICY: VOffsetT = 18;
    const NETWORK_POLICY_PROXY: VOffsetT = 4;
    const NETWORK_POLICY_EGRESS: VOffsetT = 6;
    const NETWORK_POLICY_ALLOWED_APPCONTAINER_PEER: VOffsetT = 8;
    const PROXY_INFO_URL: VOffsetT = 4;
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
    let read_write = path_vector(&paths.read_write);
    let read_only = path_vector(&paths.read_only);
    let deny = path_vector(&paths.denied);
    let capabilities = match network {
        NetworkAccess::None => None,
        // Reaching a loopback listener in another process needs both of these.
        NetworkAccess::LocalhostPort(_) => Some("privateNetworkClientServer,networkLoopback"),
        // Without `networkLoopback`, PSEC blocks every loopback connection,
        // whatever the egress policy says.
        NetworkAccess::All => Some("internetClient,privateNetworkClientServer,networkLoopback"),
    }
    .map(|capabilities| builder.create_string(capabilities));

    let network_policy = match network {
        NetworkAccess::LocalhostPort(port) => {
            // With a proxy set, PSEC only allows connections to the proxy's address and
            // port. A proxy without a package identity also needs a peer name, which can
            // be anything.
            let url = builder.create_string(&format!("http://127.0.0.1:{port}"));
            let peer = builder.create_string("Zed");
            let proxy = builder.start_table();
            builder.push_slot_always(PROXY_INFO_URL, url);
            let proxy = builder.end_table(proxy);

            let network_policy = builder.start_table();
            builder.push_slot_always(NETWORK_POLICY_PROXY, proxy);
            builder.push_slot_always(NETWORK_POLICY_ALLOWED_APPCONTAINER_PEER, peer);
            builder.end_table(network_policy)
        }
        NetworkAccess::None | NetworkAccess::All => {
            let egress = builder.start_table();
            let default_action = if matches!(network, NetworkAccess::All) {
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
            builder.end_table(network_policy)
        }
    };

    let root = builder.start_table();
    // The inline `SchemaVersion { major: 1, minor: 0 }` struct: two
    // little-endian `uint16`s, written as one `u32`.
    builder.push_slot_always::<u32>(VERSION, 1);
    if let Some(capabilities) = capabilities {
        builder.push_slot_always(CAPABILITIES, capabilities);
    }
    // Block access to the clipboard, which can hold secrets.
    builder.push_slot::<u64>(
        UI_RESTRICTIONS,
        u64::from(JOB_OBJECT_UILIMIT_READCLIPBOARD.0 | JOB_OBJECT_UILIMIT_WRITECLIPBOARD.0),
        0,
    );
    if let Some(read_write) = read_write {
        builder.push_slot_always(FS_READ_WRITE, read_write);
    }
    if let Some(read_only) = read_only {
        builder.push_slot_always(FS_READ_ONLY, read_only);
    }
    if let Some(deny) = deny {
        builder.push_slot_always(FS_DENY, deny);
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
    let launcher = launcher_path()?;
    let launcher = launcher
        .to_str()
        .ok_or_else(|| anyhow!("Zed CLI path is not valid UTF-8: {}", launcher.display()))?
        .to_string();
    let mut launcher_args = Vec::with_capacity(args.len() + 4);
    launcher_args.push(LAUNCHER_FLAG.to_string());
    launcher_args.push(URL_SAFE_NO_PAD.encode(spec));
    launcher_args.push(program);
    launcher_args.extend(args);
    Ok((launcher, launcher_args))
}

fn launcher_path() -> Result<PathBuf> {
    // The launcher has to itself be a console-subsystem program, so it's implemented
    // in the CLI executable.
    let launcher = util::get_zed_cli_path()
        .context("in a development build, run `cargo build -p cli`")
        .context("could not find the Zed CLI, which starts sandboxed commands")?;
    Ok(dunce::simplified(&launcher).to_path_buf())
}

pub(crate) fn check_available() -> Result<()> {
    check_version_supported()?;
    launcher_path()?;
    Ok(())
}

fn check_version_supported() -> Result<()> {
    ensure!(
        exports()?.supports_version(1, 0)?,
        "this Windows build does not support Process Security Environment 1.0"
    );
    Ok(())
}

/// If this process was started as the launcher, run the sandboxed command and
/// exit with its exit code. Returns normally otherwise.
pub fn run_launcher_if_invoked() {
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

    // Disable Windows Error Reporting inside the sandbox, since it doesn't complete properly there.
    unsafe { SetErrorMode(THREAD_ERROR_MODE(GetErrorMode()) | SEM_NOGPFAULTERRORBOX) };

    // PSEC resolves each read-write path when the environment is created.
    // To make sure a path approved by the user hasn't since become a junction to somewhere else,
    // we open each folder (without sharing DELETE access, so it can't be renamed) and check it still
    // resolves to the same path. The handles stay open until the environment exists.
    let pinned_folders = pin_read_write_folders(&spec)?;
    let environment = SecurityEnvironment::create(&spec)?;
    drop(pinned_folders);

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
    // A PSEC launch builds a fresh environment unless it's given one, which would
    // drop everything Zed set for the command, such as the proxy variables. PSEC
    // still replaces `TEMP`, `TMP` and `LOCALAPPDATA` with the sandbox's own.
    let environment_block = unsafe { GetEnvironmentStringsW() };
    let created = unsafe {
        CreateProcessW(
            PCWSTR::null(),
            Some(PWSTR(command_line.as_mut_ptr())),
            None,
            None,
            !inherited_handles.is_empty(),
            creation_flags | CREATE_UNICODE_ENVIRONMENT,
            Some(environment_block.0.cast_const().cast()),
            PCWSTR::null(),
            &startup_info.StartupInfo,
            &mut process_information,
        )
    };
    unsafe { DeleteProcThreadAttributeList(attribute_list) };
    unsafe { FreeEnvironmentStringsW(environment_block) }
        .context("failed to free the environment block")?;
    created.with_context(|| format!("failed to start `{command}` in the sandbox"))?;
    let process = unsafe { OwnedHandle::from_raw_handle(process_information.hProcess.0) };
    let _thread = unsafe { OwnedHandle::from_raw_handle(process_information.hThread.0) };

    unsafe { WaitForSingleObject(HANDLE(process.as_raw_handle()), INFINITE) };
    let mut exit_code = 0u32;
    unsafe { GetExitCodeProcess(HANDLE(process.as_raw_handle()), &mut exit_code) }
        .context("failed to read the sandboxed command's exit code")?;
    Ok(exit_code)
}

type PathVector<'a> = ForwardsUOffset<Vector<'a, ForwardsUOffset<&'a str>>>;

/// The spec's root table, which [`flatbuffers::root`] verifies before reading.
struct SpecTable<'a>(flatbuffers::Table<'a>);

impl<'a> flatbuffers::Follow<'a> for SpecTable<'a> {
    type Inner = Self;

    unsafe fn follow(buffer: &'a [u8], location: usize) -> Self {
        Self(unsafe { flatbuffers::Table::new(buffer, location) })
    }
}

impl flatbuffers::Verifiable for SpecTable<'_> {
    fn run_verifier(
        verifier: &mut flatbuffers::Verifier,
        position: usize,
    ) -> std::result::Result<(), flatbuffers::InvalidFlatbuffer> {
        verifier
            .visit_table(position)?
            .visit_field::<PathVector>("fs_read_write", FS_READ_WRITE, false)?
            .finish();
        Ok(())
    }
}

fn pin_read_write_folders(spec: &[u8]) -> Result<Vec<OwnedHandle>> {
    let root = flatbuffers::root::<SpecTable>(spec).context("malformed sandbox policy")?;
    let paths = unsafe { root.0.get::<PathVector>(FS_READ_WRITE, None) };
    paths
        .into_iter()
        .flatten()
        .filter_map(|path| pin_folder(path).transpose())
        .collect()
}

/// Open `path` so that neither it nor any folder above it can be renamed or
/// deleted while the handle is open, then check that it still names itself.
fn pin_folder(path: &str) -> Result<Option<OwnedHandle>> {
    let wide_path: Vec<u16> = OsStr::new(path).encode_wide().chain([0]).collect();
    let handle = match unsafe {
        CreateFileW(
            PCWSTR(wide_path.as_ptr()),
            FILE_LIST_DIRECTORY.0,
            FILE_SHARE_READ | FILE_SHARE_WRITE,
            None,
            OPEN_EXISTING,
            FILE_FLAG_BACKUP_SEMANTICS,
            None,
        )
    } {
        Ok(handle) => unsafe { OwnedHandle::from_raw_handle(handle.0) },
        Err(error) if error.code() == ERROR_ACCESS_DENIED.to_hresult() => return Ok(None),
        Err(error) => {
            return Err(error).with_context(|| format!("failed to open the write grant `{path}`"));
        }
    };

    let mut buffer = vec![0u16; 32_768];
    let length = unsafe {
        GetFinalPathNameByHandleW(
            HANDLE(handle.as_raw_handle()),
            &mut buffer,
            FILE_NAME_NORMALIZED,
        )
    } as usize;
    ensure!(
        length != 0 && length < buffer.len(),
        "failed to resolve the write grant `{path}`"
    );
    let final_path = PathBuf::from(OsString::from_wide(&buffer[..length]));
    let final_path = dunce::simplified(&final_path);
    ensure!(
        final_path == Path::new(path),
        "the write grant `{path}` was redirected to `{}`",
        final_path.display()
    );
    Ok(Some(handle))
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
        .filter_map(|entry| match dunce::canonicalize(entry.path()) {
            Ok(directory) => Some(directory),
            Err(error) => {
                log::warn!(
                    "[sandbox] failed to resolve {} for writable grants: {error}",
                    entry.path().display()
                );
                None
            }
        })
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
type QueryProcessSecurityEnvironmentSupportFn =
    unsafe extern "system" fn(support_flags: *mut u64) -> HRESULT;

/// The `processmodel.dll` exports, which only exist on recent Windows builds.
struct Exports {
    create: CreateProcessSecurityEnvironmentFn,
    close: CloseProcessSecurityEnvironmentFn,
    version_supported: Option<IsProcessSecurityEnvironmentVersionSupportedFn>,
    query_support: Option<QueryProcessSecurityEnvironmentSupportFn>,
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
                query_support: export(c"QueryProcessSecurityEnvironmentSupport").map(
                    |function| {
                        std::mem::transmute::<RawExport, QueryProcessSecurityEnvironmentSupportFn>(
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

    fn support_flags(&self) -> Result<u64> {
        let Some(query_support) = self.query_support else {
            return Ok(0);
        };
        let mut flags = 0u64;
        unsafe { query_support(&mut flags) }
            .ok()
            .context("QueryProcessSecurityEnvironmentSupport failed")?;
        Ok(flags)
    }
}

/// Whether this Windows build can deny access to paths (`fs_deny`).
pub(crate) fn supports_fs_deny() -> bool {
    const PSE_SUPPORT_FS_DENY: u64 = 0x1;
    exports()
        .and_then(Exports::support_flags)
        .is_ok_and(|flags| flags & PSE_SUPPORT_FS_DENY != 0)
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
        if let Err(error) = check_version_supported() {
            eprintln!("skipping PSEC test: {error:#}");
            return None;
        }
        let command_line = format!(
            "zed.exe {LAUNCHER_FLAG} {} cmd.exe /c {script}",
            URL_SAFE_NO_PAD.encode(spec)
        );
        Some(run_launcher(&command_line).expect("launcher failed"))
    }

    fn fs_setup(
        allow_fs_write: bool,
        writable_paths: &[&Path],
        protected_paths: &[&Path],
        paths_disallowing_reads: &[&Path],
        paths_allowing_reads: &[&Path],
    ) -> FsSetup {
        let capture = |paths: &[&Path]| {
            paths
                .iter()
                .map(|path| HostFilesystemLocation::capture(path).unwrap())
                .collect()
        };
        FsSetup {
            allow_fs_write,
            writable_paths: capture(writable_paths),
            protected_paths: capture(protected_paths),
            paths_disallowing_reads: capture(paths_disallowing_reads),
            paths_allowing_reads: capture(paths_allowing_reads),
        }
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
            let writable_paths: &[&Path] = if allow_fs_write {
                &[]
            } else {
                &[self.project.path()]
            };
            let fs = fs_setup(
                allow_fs_write,
                writable_paths,
                &[&self.protected()],
                &[],
                &[],
            );
            build_spec(&fs, NetworkAccess::None).unwrap()
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

    #[test]
    #[expect(
        clippy::disallowed_methods,
        reason = "blocking on `mklink` is fine in a test"
    )]
    fn launcher_rejects_a_write_grant_that_is_now_a_junction() {
        if let Err(error) = check_version_supported() {
            eprintln!("skipping PSEC test: {error:#}");
            return;
        }
        let layout = Layout::new();
        let project = dunce::canonicalize(layout.project.path()).unwrap();
        let grant = project.join("grant");
        let status = std::process::Command::new("cmd")
            .args(["/c", "mklink", "/J"])
            .arg(&grant)
            .arg(layout.outside.path())
            .status()
            .unwrap();
        assert!(status.success());

        // Encoded directly: building the policy would resolve the junction first.
        let paths = PolicyPaths {
            read_write: vec![grant],
            ..Default::default()
        };
        let spec = encode_spec(&paths, NetworkAccess::None).unwrap();
        let command_line = format!(
            "zed.exe {LAUNCHER_FLAG} {} cmd.exe /c exit 0",
            URL_SAFE_NO_PAD.encode(spec)
        );
        let error = run_launcher(&command_line).unwrap_err();
        assert!(format!("{error:#}").contains("was redirected"), "{error:#}");
    }

    #[test]
    fn proxy_policy_only_reaches_the_proxy_port() {
        let proxy = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
        let other = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
        let proxy_port = proxy.local_addr().unwrap().port();
        let other_port = other.local_addr().unwrap().port();
        let spec = build_spec(
            &fs_setup(false, &[], &[], &[], &[]),
            NetworkAccess::LocalhostPort(proxy_port),
        )
        .unwrap();
        let connect = |port: u16| {
            run_sandboxed(
                &spec,
                &format!(
                    "powershell -NoProfile -NonInteractive -Command \"(New-Object Net.Sockets.TcpClient).Connect('127.0.0.1', {port})\""
                ),
            )
        };

        let Some(exit_code) = connect(proxy_port) else {
            return;
        };
        assert_eq!(exit_code, 0, "the sandbox should reach the proxy port");
        assert_ne!(
            connect(other_port),
            Some(0),
            "the sandbox should not reach other loopback ports"
        );
    }

    #[test]
    fn unrestricted_network_policy_reaches_loopback() {
        let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
        let port = listener.local_addr().unwrap().port();
        let spec = build_spec(&fs_setup(false, &[], &[], &[], &[]), NetworkAccess::All).unwrap();
        let Some(exit_code) = run_sandboxed(
            &spec,
            &format!(
                "powershell -NoProfile -NonInteractive -Command \"(New-Object Net.Sockets.TcpClient).Connect('127.0.0.1', {port})\""
            ),
        ) else {
            return;
        };
        assert_eq!(exit_code, 0, "the sandbox should reach loopback listeners");
    }

    #[test]
    fn hidden_paths_are_unreadable_except_for_exceptions() {
        let root = tempfile::tempdir().unwrap();
        let project = dunce::canonicalize(root.path()).unwrap();
        let hidden = project.join("hidden");
        let exception = hidden.join("exception");
        std::fs::create_dir_all(&exception).unwrap();
        std::fs::write(hidden.join("file.txt"), "hidden").unwrap();
        std::fs::write(exception.join("file.txt"), "visible").unwrap();
        let fs = fs_setup(false, &[&project], &[], &[&hidden], &[&exception]);
        let spec = build_spec(&fs, NetworkAccess::None).unwrap();

        let Some(exit_code) = run_sandboxed(
            &spec,
            &format!("type {}", hidden.join("file.txt").display()),
        ) else {
            return;
        };
        assert_ne!(exit_code, 0, "a hidden file should not be readable");
        assert_eq!(
            run_sandboxed(
                &spec,
                &format!("type {}", exception.join("file.txt").display())
            ),
            Some(0),
            "an exception should be readable"
        );
        assert_eq!(
            run_sandboxed(&spec, &format!(r"echo x> {}\new.txt", exception.display())),
            Some(0),
            "an exception inside the project should be writable"
        );
    }

    #[test]
    fn grants_inside_hidden_paths_are_dropped_unless_an_exception_covers_them() {
        let root = tempfile::tempdir().unwrap();
        let root = dunce::canonicalize(root.path()).unwrap();
        let directory = |relative: &str| {
            let path = root.join(relative);
            std::fs::create_dir_all(&path).unwrap();
            path
        };
        let home = directory("home");
        let project = directory(r"home\project");
        let project_git = directory(r"home\project\.git");
        let other = directory(r"home\other");
        let other_git = directory(r"home\other\.git");
        let config = directory(r"home\.config");
        let fs = fs_setup(
            false,
            &[&project, &other],
            &[&project_git, &other_git],
            &[&home],
            &[&project, &config],
        );

        let paths = policy_paths(&fs);
        assert!(paths.read_write.contains(&project));
        assert!(!paths.read_write.contains(&other));
        assert!(paths.read_only.contains(&project_git));
        assert!(paths.read_only.contains(&config));
        assert!(!paths.read_only.contains(&other_git));
        assert_eq!(paths.denied, [home]);
    }
}
