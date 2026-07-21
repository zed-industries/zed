use crash_handler::{CrashEventResult, CrashHandler};
use log::info;
use minidumper::{LoopAction, MinidumpBinary, Server, SocketName};
use parking_lot::Mutex;
use serde::{Deserialize, Serialize};
use std::{panic::Location, pin::Pin};

use system_specs::GpuSpecs;

use std::{
    env,
    fs::{self, File},
    io, panic,
    path::{Path, PathBuf},
    process::{self},
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    thread,
    time::Duration,
};

pub use minidumper::Client;

const CRASH_HANDLER_PING_TIMEOUT: Duration = Duration::from_secs(60);
const CRASH_HANDLER_CONNECT_TIMEOUT: Duration = Duration::from_secs(10);

/// Force a backtrace to be printed on panic.
pub fn force_backtrace() {
    let old_hook = panic::take_hook();
    panic::set_hook(Box::new(move |info| {
        unsafe { env::set_var("RUST_BACKTRACE", "1") };
        old_hook(info);
        // prevent the macOS crash dialog from popping up
        if cfg!(target_os = "macos") {
            std::process::exit(1);
        }
    }));
}

/// Install crash signal handlers and spawn the crash-handler subprocess.
///
/// All work happens lazily in the returned future, so it runs on whichever
/// executor polls it. The keepalive task is passed to `spawn` so the caller
/// decides which executor to schedule it on.
pub fn init<F, S, C, P>(
    crash_init: InitCrashHandler,
    spawn: S,
    socket_path: P,
    wait_timer: C,
) -> impl Future<Output = Arc<Client>> + use<F, C, S, P>
where
    F: Future<Output = ()> + Send + Sync + 'static,
    C: (Fn(Duration) -> F) + Send + Sync + 'static,
    S: FnOnce(Pin<Box<dyn Future<Output = ()> + Send + 'static>>),
    P: FnOnce(u32) -> PathBuf,
{
    connect_and_keepalive(crash_init, socket_path, wait_timer, spawn)
}

/// Spawn the crash-handler subprocess, connect the IPC client, and run the
/// keepalive ping loop. This is the future returned by [`init`], so it runs on
/// whichever executor the caller polls it with.
async fn connect_and_keepalive<F, C, S, P>(
    crash_init: InitCrashHandler,
    socket_path: P,
    wait_timer: C,
    spawn: S,
) -> Arc<Client>
where
    F: Future<Output = ()> + Send + Sync + 'static,
    C: (Fn(Duration) -> F) + Send + Sync + 'static,
    S: FnOnce(Pin<Box<dyn Future<Output = ()> + Send + 'static>>),
    P: FnOnce(u32) -> PathBuf,
{
    let exe = env::current_exe().expect("unable to find ourselves");
    let socket_path = socket_path(process::id());
    let mut _crash_handler = spawn_crash_handler(&exe, &socket_path);
    info!("spawning crash handler process");
    let mut elapsed = Duration::ZERO;
    let retry_frequency = Duration::from_millis(100);
    let client = loop {
        if let Ok(client) = Client::with_name(SocketName::Path(&socket_path)) {
            info!("connected to crash handler process after {elapsed:?}");
            break client;
        }
        elapsed += retry_frequency;
        wait_timer(retry_frequency).await;
    };
    let client = Arc::new(client);

    panic::set_hook({
        let client = client.clone();
        Box::new(move |payload| {
            panic_hook(
                client.clone(),
                payload.payload_as_str().unwrap_or("Box<Any>"),
                payload.location(),
            )
        })
    });
    info!("panic handler registered");
    let handler = CrashHandler::attach(unsafe {
        let client = client.clone();
        let handler = move |crash_context: &crash_handler::CrashContext| {
            // set when the first minidump request is made to avoid generating duplicate crash reports
            static REQUESTED_MINIDUMP: AtomicBool = AtomicBool::new(false);

            // only request a minidump once
            let res = if REQUESTED_MINIDUMP
                .compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed)
                .is_ok()
            {
                #[cfg(target_os = "macos")]
                macos::suspend_all_other_threads();

                // on macos this "ping" is needed to ensure that all our
                // `client.send_message` calls have been processed before we trigger the
                // minidump request.
                client.ping().ok();
                let r = client.request_dump(crash_context);
                if let Err(e) = &r {
                    eprintln!("failed to request dump: {:?}", e);
                }
                #[cfg(target_os = "macos")]
                macos::resume_all_other_threads();
                r.is_ok()
            } else {
                true
            };
            CrashEventResult::Handled(res)
        };
        crash_handler::make_crash_event(handler)
    })
    .expect("failed to attach signal handler");

    info!("crash signal handlers installed");
    send_crash_server_message(&client, CrashServerMessage::Init(crash_init));

    #[cfg(all(target_os = "linux", target_env = "gnu"))]
    if let Some(address) = abort_message_address() {
        send_crash_server_message(
            &client,
            CrashServerMessage::AbortMessageLocation(AbortMessageLocation {
                pid: process::id(),
                address,
            }),
        );
    }

    #[cfg(target_os = "linux")]
    handler.set_ptracer(Some(_crash_handler.id()));

    info!("crash handler registered");
    spawn(Box::pin({
        let client = client.clone();
        async move {
            let _handler = { handler };
            loop {
                if let Err(e) = client.ping() {
                    #[cfg(not(target_os = "windows"))]
                    log::error!(
                        "ping failed: {:?}, process exit status: {:?}",
                        e,
                        _crash_handler.try_status()
                    );
                    #[cfg(target_os = "windows")]
                    log::error!("ping failed: {:?}", e,);
                    break;
                };
                wait_timer(Duration::from_secs(10)).await;
            }
        }
    }));
    client
}

pub struct CrashServer {
    initialization_params: Mutex<Option<InitCrashHandler>>,
    panic_info: Mutex<Option<CrashPanic>>,
    active_gpu: Mutex<Option<system_specs::GpuSpecs>>,
    user_info: Mutex<Option<UserInfo>>,
    abort_message_location: Mutex<Option<AbortMessageLocation>>,
    has_connection: Arc<AtomicBool>,
    logs_dir: PathBuf,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CrashInfo {
    pub init: InitCrashHandler,
    pub panic: Option<CrashPanic>,
    pub minidump_error: Option<String>,
    /// The diagnostic the C runtime recorded before aborting the process, e.g.
    /// glibc's "free(): invalid pointer". Only present when the crash was a
    /// runtime-initiated abort rather than a signal like SIGSEGV or a panic.
    #[serde(default)]
    pub abort_message: Option<String>,
    pub gpus: Vec<system_specs::GpuInfo>,
    pub active_gpu: Option<system_specs::GpuSpecs>,
    pub user_info: Option<UserInfo>,
}

/// Where to find the C runtime's abort diagnostic in the crashed process's
/// memory. Sent by the client at startup so that after a crash the server can
/// recover the message with `process_vm_readv`; the crashed process itself
/// can't safely do this work, since its heap may be corrupt and its allocator
/// locks may be held by the crashed thread.
#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
pub struct AbortMessageLocation {
    pub pid: u32,
    pub address: u64,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct InitCrashHandler {
    pub session_id: String,
    pub zed_version: String,
    pub binary: String,
    pub release_channel: String,
    pub commit_sha: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct CrashPanic {
    pub message: String,
    pub span: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct UserInfo {
    pub metrics_id: Option<String>,
    pub is_staff: Option<bool>,
}

fn send_crash_server_message(crash_client: &Arc<Client>, message: CrashServerMessage) {
    let data = match serde_json::to_vec(&message) {
        Ok(data) => data,
        Err(err) => {
            log::warn!("Failed to serialize crash server message: {:?}", err);
            return;
        }
    };

    if let Err(err) = crash_client.send_message(0, data) {
        log::warn!("Failed to send data to crash server {:?}", err);
    }
}

pub fn set_gpu_info(crash_client: &Arc<Client>, specs: GpuSpecs) {
    send_crash_server_message(crash_client, CrashServerMessage::GPUInfo(specs));
}

pub fn set_user_info(crash_client: &Arc<Client>, info: UserInfo) {
    send_crash_server_message(crash_client, CrashServerMessage::UserInfo(info));
}

#[derive(Serialize, Deserialize, Debug)]
enum CrashServerMessage {
    Init(InitCrashHandler),
    Panic(CrashPanic),
    GPUInfo(GpuSpecs),
    UserInfo(UserInfo),
    AbortMessageLocation(AbortMessageLocation),
}

/// glibc records the diagnostic it prints just before aborting (malloc integrity
/// failures like "free(): invalid pointer", assertion failures, stack-smashing
/// reports) in the private global `__abort_msg`, specifically so it can be
/// recovered post-mortem. Resolve its address here, in a safe context at startup.
/// The symbol is only exported at the GLIBC_PRIVATE version, which plain `dlsym`
/// won't resolve, and it has no stability guarantee, so a null result (e.g. musl,
/// or a future glibc removing it) just disables this diagnostic.
#[cfg(all(target_os = "linux", target_env = "gnu"))]
fn abort_message_address() -> Option<u64> {
    let ptr = unsafe {
        libc::dlvsym(
            libc::RTLD_DEFAULT,
            c"__abort_msg".as_ptr(),
            c"GLIBC_PRIVATE".as_ptr(),
        )
    };
    std::ptr::NonNull::new(ptr).map(|ptr| ptr.as_ptr() as u64)
}

/// Read the crashed process's abort diagnostic. `__abort_msg` points to a
/// `struct abort_msg_s { unsigned int size; char msg[]; }` that glibc allocates
/// with mmap so that it stays intact even when the heap is corrupt. `size` is
/// the total byte size of that mapping (header included, rounded up to whole
/// pages), not the message length; the message itself is NUL-terminated.
#[cfg(target_os = "linux")]
fn read_abort_message(location: AbortMessageLocation) -> Option<String> {
    let pointer_bytes = read_process_memory(location.pid, location.address, size_of::<usize>())?;
    let message_address = usize::from_ne_bytes(pointer_bytes.try_into().ok()?) as u64;
    if message_address == 0 {
        return None;
    }
    let size_bytes = read_process_memory(location.pid, message_address, size_of::<u32>())?;
    let size = u32::from_ne_bytes(size_bytes.try_into().ok()?);
    let message_bytes = read_process_memory(
        location.pid,
        message_address + size_of::<u32>() as u64,
        abort_message_read_len(size)?,
    )?;
    parse_abort_message(&message_bytes)
}

/// How many message bytes to read given the `size` field of glibc's
/// `abort_msg_s`. `size` holds the total size of the mmap'd allocation, so a
/// value that isn't a whole number of pages means the layout has changed and
/// we shouldn't trust it. Reading is capped at (one page minus the header),
/// which both bounds the work and ensures the read never extends past the end
/// of the mapping.
#[cfg(any(target_os = "linux", test))]
fn abort_message_read_len(size: u32) -> Option<usize> {
    // Every Linux page size (4 KiB, 16 KiB, 64 KiB, ...) is a multiple of 4 KiB.
    const PAGE_MULTIPLE: usize = 4096;
    const MAX_READ: usize = 4096;

    let size = size as usize;
    if size == 0 || !size.is_multiple_of(PAGE_MULTIPLE) {
        log::warn!("__abort_msg size field {size} is not page-rounded; layout may have changed");
        return None;
    }
    Some(size.min(MAX_READ) - size_of::<u32>())
}

/// The message is NUL-terminated inside a zero-filled mapping, so truncate at
/// the first NUL; `trim` alone would keep the padding, since NUL is not
/// whitespace.
#[cfg(any(target_os = "linux", test))]
fn parse_abort_message(bytes: &[u8]) -> Option<String> {
    let len = bytes
        .iter()
        .position(|&byte| byte == 0)
        .unwrap_or(bytes.len());
    let message = String::from_utf8_lossy(&bytes[..len]).trim().to_string();
    (!message.is_empty()).then_some(message)
}

#[cfg(target_os = "linux")]
fn read_process_memory(pid: u32, address: u64, len: usize) -> Option<Vec<u8>> {
    let mut buffer = vec![0u8; len];
    let local = libc::iovec {
        iov_base: buffer.as_mut_ptr().cast(),
        iov_len: len,
    };
    let remote = libc::iovec {
        iov_base: address as *mut libc::c_void,
        iov_len: len,
    };
    let bytes_read =
        unsafe { libc::process_vm_readv(pid as libc::pid_t, &local, 1, &remote, 1, 0) };
    if bytes_read < 0 {
        log::warn!(
            "process_vm_readv of {len} bytes at {address:#x} in pid {pid} failed: {}",
            io::Error::last_os_error()
        );
        return None;
    }
    if bytes_read as usize != len {
        log::warn!(
            "process_vm_readv short read at {address:#x} in pid {pid}: {bytes_read} of {len} bytes"
        );
        return None;
    }
    Some(buffer)
}

impl minidumper::ServerHandler for CrashServer {
    fn create_minidump_file(&self) -> Result<(File, PathBuf), io::Error> {
        let dump_path = self
            .logs_dir
            .join(
                &self
                    .initialization_params
                    .lock()
                    .as_ref()
                    .expect("Missing initialization data")
                    .session_id,
            )
            .with_extension("dmp");
        let file = File::create(&dump_path)?;
        Ok((file, dump_path))
    }

    fn on_minidump_created(&self, result: Result<MinidumpBinary, minidumper::Error>) -> LoopAction {
        let minidump_error = match result {
            Ok(MinidumpBinary { mut file, path, .. }) => {
                use io::Write;
                file.flush().ok();
                // TODO: clean this up once https://github.com/EmbarkStudios/crash-handling/issues/101 is addressed
                drop(file);
                let original_file = File::open(&path).unwrap();
                let compressed_path = path.with_extension("zstd");
                let compressed_file = File::create(&compressed_path).unwrap();
                zstd::stream::copy_encode(original_file, compressed_file, 0).ok();
                fs::rename(&compressed_path, path).unwrap();
                None
            }
            Err(e) => Some(format!("{e:?}")),
        };

        #[cfg(not(any(target_os = "linux", target_os = "freebsd")))]
        let gpus = vec![];

        #[cfg(any(target_os = "linux", target_os = "freebsd"))]
        let gpus = match system_specs::read_gpu_info_from_sys_class_drm() {
            Ok(gpus) => gpus,
            Err(err) => {
                log::warn!("Failed to collect GPU information for crash report: {err}");
                vec![]
            }
        };

        // The crashed process is still alive at this point: it stays parked in
        // its signal handler until the server acknowledges the dump request,
        // which happens after this callback returns.
        #[cfg(target_os = "linux")]
        let abort_message = (*self.abort_message_location.lock()).and_then(read_abort_message);
        #[cfg(not(target_os = "linux"))]
        let abort_message = None;

        let crash_info = CrashInfo {
            init: self
                .initialization_params
                .lock()
                .clone()
                .expect("not initialized"),
            panic: self.panic_info.lock().clone(),
            minidump_error,
            abort_message,
            active_gpu: self.active_gpu.lock().clone(),
            gpus,
            user_info: self.user_info.lock().clone(),
        };

        let crash_data_path = self
            .logs_dir
            .join(&crash_info.init.session_id)
            .with_extension("json");

        fs::write(crash_data_path, serde_json::to_vec(&crash_info).unwrap()).ok();

        LoopAction::Exit
    }

    fn on_message(&self, _: u32, buffer: Vec<u8>) {
        let message: CrashServerMessage =
            serde_json::from_slice(&buffer).expect("invalid init data");
        match message {
            CrashServerMessage::Init(init_data) => {
                self.initialization_params.lock().replace(init_data);
            }
            CrashServerMessage::Panic(crash_panic) => {
                self.panic_info.lock().replace(crash_panic);
            }
            CrashServerMessage::GPUInfo(gpu_specs) => {
                self.active_gpu.lock().replace(gpu_specs);
            }
            CrashServerMessage::UserInfo(user_info) => {
                self.user_info.lock().replace(user_info);
            }
            CrashServerMessage::AbortMessageLocation(location) => {
                self.abort_message_location.lock().replace(location);
            }
        }
    }

    fn on_client_disconnected(&self, _clients: usize) -> LoopAction {
        LoopAction::Exit
    }

    fn on_client_connected(&self, _clients: usize) -> LoopAction {
        self.has_connection.store(true, Ordering::SeqCst);
        LoopAction::Continue
    }
}

/// Rust's string-slicing panics embed the user's string content in the message,
/// e.g. "byte index 4 is out of bounds of `a`". Strip that suffix so we
/// don't upload arbitrary user text in crash reports.
fn strip_user_string_from_panic(message: &str) -> String {
    const STRING_PANIC_PREFIXES: &[&str] = &[
        // Older rustc (pre-1.95):
        "byte index ",
        "begin <= end (",
        // Newer rustc (1.95+):
        // https://github.com/rust-lang/rust/pull/145024
        "start byte index ",
        "end byte index ",
        "begin > end (",
    ];

    if (message.ends_with('`') || message.ends_with("`[...]"))
        && STRING_PANIC_PREFIXES
            .iter()
            .any(|prefix| message.starts_with(prefix))
        && let Some(open) = message.find('`')
    {
        return format!("{} `<redacted>`", &message[..open]);
    }
    message.to_owned()
}

pub fn panic_hook(crash_client: Arc<Client>, message: &str, location: Option<&Location>) {
    let message = strip_user_string_from_panic(message);

    let span = location
        .map(|loc| format!("{}:{}", loc.file(), loc.line()))
        .unwrap_or_default();

    let current_thread = std::thread::current();
    let thread_name = current_thread.name().unwrap_or("<unnamed>");

    let location = location.map_or_else(|| "<unknown>".to_owned(), |location| location.to_string());
    log::error!("thread '{thread_name}' panicked at {location}:\n{message}...");

    send_crash_server_message(
        &crash_client,
        CrashServerMessage::Panic(CrashPanic { message, span }),
    );
    log::error!("triggering a crash to generate a minidump...");

    #[cfg(target_os = "macos")]
    macos::set_panic_thread_id();
    #[cfg(target_os = "windows")]
    {
        // https://learn.microsoft.com/en-us/windows/win32/debug/system-error-codes--0-499-
        CrashHandler.simulate_exception(Some(234)); // (MORE_DATA_AVAILABLE)
    }
    #[cfg(not(target_os = "windows"))]
    {
        std::process::abort();
    }
}

#[cfg(target_os = "macos")]
mod macos {
    static PANIC_THREAD_ID: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);

    pub(super) fn set_panic_thread_id() {
        PANIC_THREAD_ID.store(
            unsafe { mach2::mach_init::mach_thread_self() },
            std::sync::atomic::Ordering::Release,
        );
    }

    pub(super) unsafe fn suspend_all_other_threads() {
        let task = unsafe { mach2::traps::current_task() };
        let mut threads: mach2::mach_types::thread_act_array_t = std::ptr::null_mut();
        let mut count = 0;
        unsafe {
            mach2::task::task_threads(task, &raw mut threads, &raw mut count);
        }
        let current = unsafe { mach2::mach_init::mach_thread_self() };
        for i in 0..count {
            let t = unsafe { *threads.add(i as usize) };
            if t != current {
                unsafe { mach2::thread_act::thread_suspend(t) };
            }
        }
    }

    pub(super) unsafe fn resume_all_other_threads() {
        let task = unsafe { mach2::traps::current_task() };
        let mut threads: mach2::mach_types::thread_act_array_t = std::ptr::null_mut();
        let mut count = 0;
        unsafe {
            mach2::task::task_threads(task, &raw mut threads, &raw mut count);
        }
        let current = unsafe { mach2::mach_init::mach_thread_self() };
        for i in 0..count {
            let t = unsafe { *threads.add(i as usize) };
            if t != current {
                unsafe { mach2::thread_act::thread_resume(t) };
            }
        }
    }
}
#[cfg(not(target_os = "windows"))]
fn spawn_crash_handler(exe: &Path, socket_name: &Path) -> async_process::Child {
    async_process::Command::new(exe)
        .arg("--crash-handler")
        .arg(&socket_name)
        .spawn()
        .expect("unable to spawn server process")
}

#[cfg(target_os = "windows")]
fn spawn_crash_handler(exe: &Path, socket_name: &Path) {
    use std::ffi::OsStr;
    use std::iter::once;
    use std::os::windows::ffi::OsStrExt;
    use windows::Win32::System::Threading::{
        CreateProcessW, PROCESS_CREATION_FLAGS, PROCESS_INFORMATION, STARTF_FORCEOFFFEEDBACK,
        STARTUPINFOW,
    };
    use windows::core::PWSTR;

    let mut command_line: Vec<u16> = OsStr::new(&format!(
        "\"{}\" --crash-handler \"{}\"",
        exe.display(),
        socket_name.display()
    ))
    .encode_wide()
    .chain(once(0))
    .collect();

    let mut startup_info = STARTUPINFOW::default();
    startup_info.cb = std::mem::size_of::<STARTUPINFOW>() as u32;

    // By default, Windows enables a "busy" cursor when a GUI application is launched.
    // This cursor is disabled once the application starts processing window messages.
    // Since the crash handler process doesn't process messages, this "busy" cursor stays enabled for a long time.
    // Disable the cursor feedback to prevent this from happening.
    startup_info.dwFlags = STARTF_FORCEOFFFEEDBACK;

    let mut process_info = PROCESS_INFORMATION::default();

    unsafe {
        CreateProcessW(
            None,
            Some(PWSTR(command_line.as_mut_ptr())),
            None,
            None,
            false,
            PROCESS_CREATION_FLAGS(0),
            None,
            None,
            &startup_info,
            &mut process_info,
        )
        .expect("unable to spawn server process");

        windows::Win32::Foundation::CloseHandle(process_info.hProcess).ok();
        windows::Win32::Foundation::CloseHandle(process_info.hThread).ok();
    }
}

pub fn crash_server(socket: &Path, logs_dir: PathBuf) {
    let Ok(mut server) = Server::with_name(SocketName::Path(socket)) else {
        log::info!("Couldn't create socket, there may already be a running crash server");
        return;
    };

    let shutdown = Arc::new(AtomicBool::new(false));
    let has_connection = Arc::new(AtomicBool::new(false));

    thread::Builder::new()
        .name("CrashServerTimeout".to_owned())
        .spawn({
            let shutdown = shutdown.clone();
            let has_connection = has_connection.clone();
            move || {
                std::thread::sleep(CRASH_HANDLER_CONNECT_TIMEOUT);
                if !has_connection.load(Ordering::SeqCst) {
                    shutdown.store(true, Ordering::SeqCst);
                }
            }
        })
        .unwrap();

    server
        .run(
            Box::new(CrashServer {
                initialization_params: Mutex::default(),
                panic_info: Mutex::default(),
                user_info: Mutex::default(),
                abort_message_location: Mutex::default(),
                has_connection,
                active_gpu: Mutex::default(),
                logs_dir,
            }),
            &shutdown,
            Some(CRASH_HANDLER_PING_TIMEOUT),
        )
        .expect("failed to run server");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn abort_message_read_len_requires_page_rounded_total() {
        assert_eq!(abort_message_read_len(0), None);
        // A message length rather than a mapping total means the glibc layout
        // has changed out from under us.
        assert_eq!(abort_message_read_len(23), None);
        assert_eq!(abort_message_read_len(4097), None);
        // The read must stay within the mapping: one page minus the header.
        assert_eq!(abort_message_read_len(4096), Some(4092));
        // Larger totals (long messages, larger page sizes) are clamped.
        assert_eq!(abort_message_read_len(8192), Some(4092));
        assert_eq!(abort_message_read_len(65536), Some(4092));
    }

    #[test]
    fn parse_abort_message_truncates_at_nul() {
        let mut buffer = b"free(): invalid pointer\n\0".to_vec();
        buffer.resize(4092, 0);
        assert_eq!(
            parse_abort_message(&buffer),
            Some("free(): invalid pointer".to_string())
        );
    }

    #[test]
    fn parse_abort_message_handles_missing_nul() {
        assert_eq!(
            parse_abort_message(b"double free or corruption (out)"),
            Some("double free or corruption (out)".to_string())
        );
    }

    #[test]
    fn parse_abort_message_rejects_empty() {
        assert_eq!(parse_abort_message(&[]), None);
        assert_eq!(parse_abort_message(&[0; 16]), None);
        assert_eq!(parse_abort_message(b"\n \0garbage after nul"), None);
    }

    /// End-to-end check of `read_abort_message` against a synthetic
    /// `abort_msg_s` in this very process (`process_vm_readv` may always read
    /// one's own memory). The message page is followed by a `PROT_NONE` guard
    /// page so the test fails if the read ever extends past the mapping glibc
    /// would have allocated.
    #[cfg(target_os = "linux")]
    #[test]
    fn read_abort_message_reads_glibc_layout_from_a_live_process() {
        let page_size = unsafe { libc::sysconf(libc::_SC_PAGESIZE) } as usize;
        unsafe {
            let mapping = libc::mmap(
                std::ptr::null_mut(),
                2 * page_size,
                libc::PROT_READ | libc::PROT_WRITE,
                libc::MAP_ANON | libc::MAP_PRIVATE,
                -1,
                0,
            );
            assert_ne!(mapping, libc::MAP_FAILED);
            assert_eq!(
                libc::mprotect(
                    mapping.cast::<u8>().add(page_size).cast(),
                    page_size,
                    libc::PROT_NONE
                ),
                0
            );

            mapping.cast::<u32>().write(page_size as u32);
            let message = b"free(): invalid pointer\n\0";
            std::ptr::copy_nonoverlapping(
                message.as_ptr(),
                mapping.cast::<u8>().add(size_of::<u32>()),
                message.len(),
            );

            // Stands in for the `__abort_msg` global: a pointer variable whose
            // address we hand to the reader.
            let abort_msg: *mut libc::c_void = mapping;
            let location = AbortMessageLocation {
                pid: process::id(),
                address: (&raw const abort_msg) as u64,
            };
            assert_eq!(
                read_abort_message(location),
                Some("free(): invalid pointer".to_string())
            );

            libc::munmap(mapping, 2 * page_size);
        }
    }
}
