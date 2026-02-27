use crash_handler::{CrashEventResult, CrashHandler};
use futures::future::BoxFuture;
use log::info;
use minidumper::{Client, LoopAction, MinidumpBinary};
use release_channel::{RELEASE_CHANNEL, ReleaseChannel};
use serde::{Deserialize, Serialize};
use std::cell::Cell;
use std::mem;

#[cfg(not(target_os = "windows"))]
use smol::process::Command;

#[cfg(target_os = "macos")]
use std::sync::atomic::AtomicU32;
use std::{
    env,
    fs::{self, File},
    io,
    panic::{self, AssertUnwindSafe, PanicHookInfo},
    path::{Path, PathBuf},
    process::{self},
    sync::{
        Arc, OnceLock,
        atomic::{AtomicBool, Ordering},
    },
    thread,
    time::Duration,
};

thread_local! {
    static ALLOW_UNWIND: Cell<bool> = const { Cell::new(false) };
}

/// Catch a panic as an error instead of aborting the process. Unlike plain
/// `catch_unwind`, this bypasses the crash-reporting panic hook which would
/// normally abort before unwinding can occur.
///
/// **Use sparingly.** Prefer this only for isolating third-party code
/// that is known to panic, where you want to handle the failure gracefully
/// instead of crashing.
pub fn recoverable_panic<T>(closure: impl FnOnce() -> T) -> anyhow::Result<T> {
    ALLOW_UNWIND.with(|flag| flag.set(true));
    let result = panic::catch_unwind(AssertUnwindSafe(closure));
    ALLOW_UNWIND.with(|flag| flag.set(false));
    result.map_err(|payload| {
        let message = payload
            .downcast_ref::<&str>()
            .map(|s| s.to_string())
            .or_else(|| payload.downcast_ref::<String>().cloned())
            .unwrap_or_else(|| "unknown panic".to_string());
        anyhow::anyhow!("panic: {message}")
    })
}

// set once the crash handler has initialized and the client has connected to it
pub static CRASH_HANDLER: OnceLock<Arc<Client>> = OnceLock::new();
// set when the first minidump request is made to avoid generating duplicate crash reports
pub static REQUESTED_MINIDUMP: AtomicBool = AtomicBool::new(false);
const CRASH_HANDLER_PING_TIMEOUT: Duration = Duration::from_secs(60);
const CRASH_HANDLER_CONNECT_TIMEOUT: Duration = Duration::from_secs(10);

#[cfg(target_os = "macos")]
static PANIC_THREAD_ID: AtomicU32 = AtomicU32::new(0);

fn should_install_crash_handler() -> bool {
    if let Ok(value) = env::var("ZED_GENERATE_MINIDUMPS") {
        return value == "true" || value == "1";
    }

    if *RELEASE_CHANNEL == ReleaseChannel::Dev {
        return false;
    }

    true
}

/// Install crash signal handlers and spawn the crash-handler subprocess.
///
/// The synchronous portion (signal handlers, panic hook) runs inline.
/// The async keepalive task is passed to `spawn` so the caller decides
/// which executor to schedule it on.
pub fn init(crash_init: InitCrashHandler, spawn: impl FnOnce(BoxFuture<'static, ()>)) {
    if !should_install_crash_handler() {
        let old_hook = panic::take_hook();
        panic::set_hook(Box::new(move |info| {
            if ALLOW_UNWIND.with(|flag| flag.get()) {
                return;
            }
            unsafe { env::set_var("RUST_BACKTRACE", "1") };
            old_hook(info);
            // prevent the macOS crash dialog from popping up
            if cfg!(target_os = "macos") {
                std::process::exit(1);
            }
        }));
        return;
    }

    panic::set_hook(Box::new(panic_hook));

    let handler = CrashHandler::attach(unsafe {
        crash_handler::make_crash_event(move |crash_context: &crash_handler::CrashContext| {
            let Some(client) = CRASH_HANDLER.get() else {
                return CrashEventResult::Handled(false);
            };

            // only request a minidump once
            let res = if REQUESTED_MINIDUMP
                .compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed)
                .is_ok()
            {
                #[cfg(target_os = "macos")]
                suspend_all_other_threads();

                // on macos this "ping" is needed to ensure that all our
                // `client.send_message` calls have been processed before we trigger the
                // minidump request.
                client.ping().ok();
                client.request_dump(crash_context).is_ok()
            } else {
                true
            };
            CrashEventResult::Handled(res)
        })
    })
    .expect("failed to attach signal handler");

    info!("crash signal handlers installed");

    spawn(Box::pin(connect_and_keepalive(crash_init, handler)));
}

/// Spawn the crash-handler subprocess, connect the IPC client, and run the
/// keepalive ping loop. Called on a background executor by [`init`].
async fn connect_and_keepalive(crash_init: InitCrashHandler, handler: CrashHandler) {
    let exe = env::current_exe().expect("unable to find ourselves");
    let zed_pid = process::id();
    let socket_name = paths::temp_dir().join(format!("zed-crash-handler-{zed_pid}"));
    #[cfg(not(target_os = "windows"))]
    let _crash_handler = Command::new(exe)
        .arg("--crash-handler")
        .arg(&socket_name)
        .spawn()
        .expect("unable to spawn server process");

    #[cfg(target_os = "windows")]
    spawn_crash_handler_windows(&exe, &socket_name);

    info!("spawning crash handler process");

    let mut elapsed = Duration::ZERO;
    let retry_frequency = Duration::from_millis(100);
    let mut maybe_client = None;
    while maybe_client.is_none() {
        if let Ok(client) = Client::with_name(socket_name.as_path()) {
            maybe_client = Some(client);
            info!("connected to crash handler process after {elapsed:?}");
            break;
        }
        elapsed += retry_frequency;
        // Crash reporting is called outside of gpui in the remote server right now
        #[allow(clippy::disallowed_methods)]
        smol::Timer::after(retry_frequency).await;
    }
    let client = maybe_client.unwrap();
    client
        .send_message(1, serde_json::to_vec(&crash_init).unwrap())
        .unwrap();

    let client = Arc::new(client);

    #[cfg(target_os = "linux")]
    handler.set_ptracer(Some(_crash_handler.id()));

    // Publishing the client to the OnceLock makes it visible to the signal
    // handler callback installed earlier.
    CRASH_HANDLER.set(client.clone()).ok();
    // mem::forget so that the drop is not called
    mem::forget(handler);
    info!("crash handler registered");

    loop {
        client.ping().ok();
        // Crash reporting is called outside of gpui in the remote server right now
        #[allow(clippy::disallowed_methods)]
        smol::Timer::after(Duration::from_secs(10)).await;
    }
}

#[cfg(target_os = "macos")]
unsafe fn suspend_all_other_threads() {
    let task = unsafe { mach2::traps::current_task() };
    let mut threads: mach2::mach_types::thread_act_array_t = std::ptr::null_mut();
    let mut count = 0;
    unsafe {
        mach2::task::task_threads(task, &raw mut threads, &raw mut count);
    }
    let current = unsafe { mach2::mach_init::mach_thread_self() };
    let panic_thread = PANIC_THREAD_ID.load(Ordering::SeqCst);
    for i in 0..count {
        let t = unsafe { *threads.add(i as usize) };
        if t != current && t != panic_thread {
            unsafe { mach2::thread_act::thread_suspend(t) };
        }
    }
}

pub struct CrashServer {
    initialization_params: OnceLock<InitCrashHandler>,
    panic_info: OnceLock<CrashPanic>,
    active_gpu: OnceLock<system_specs::GpuSpecs>,
    has_connection: Arc<AtomicBool>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CrashInfo {
    pub init: InitCrashHandler,
    pub panic: Option<CrashPanic>,
    pub minidump_error: Option<String>,
    pub gpus: Vec<system_specs::GpuInfo>,
    pub active_gpu: Option<system_specs::GpuSpecs>,
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

impl minidumper::ServerHandler for CrashServer {
    fn create_minidump_file(&self) -> Result<(File, PathBuf), io::Error> {
        let err_message = "Missing initialization data";
        let dump_path = paths::logs_dir()
            .join(
                &self
                    .initialization_params
                    .get()
                    .expect(err_message)
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

        let crash_info = CrashInfo {
            init: self
                .initialization_params
                .get()
                .expect("not initialized")
                .clone(),
            panic: self.panic_info.get().cloned(),
            minidump_error,
            active_gpu: self.active_gpu.get().cloned(),
            gpus,
        };

        let crash_data_path = paths::logs_dir()
            .join(&crash_info.init.session_id)
            .with_extension("json");

        fs::write(crash_data_path, serde_json::to_vec(&crash_info).unwrap()).ok();

        LoopAction::Exit
    }

    fn on_message(&self, kind: u32, buffer: Vec<u8>) {
        match kind {
            1 => {
                let init_data =
                    serde_json::from_slice::<InitCrashHandler>(&buffer).expect("invalid init data");
                self.initialization_params
                    .set(init_data)
                    .expect("already initialized");
            }
            2 => {
                let panic_data =
                    serde_json::from_slice::<CrashPanic>(&buffer).expect("invalid panic data");
                self.panic_info.set(panic_data).expect("already panicked");
            }
            3 => {
                let gpu_specs: system_specs::GpuSpecs =
                    bincode::deserialize(&buffer).expect("gpu specs");
                // we ignore the case where it was already set because this message is sent
                // on each new window. in theory all zed windows should be using the same
                // GPU so this is fine.
                self.active_gpu.set(gpu_specs).ok();
            }
            _ => {
                panic!("invalid message kind");
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

pub fn panic_hook(info: &PanicHookInfo) {
    let message = info.payload_as_str().unwrap_or("Box<Any>").to_owned();

    let span = info
        .location()
        .map(|loc| format!("{}:{}", loc.file(), loc.line()))
        .unwrap_or_default();

    let current_thread = std::thread::current();
    let thread_name = current_thread.name().unwrap_or("<unnamed>");

    if ALLOW_UNWIND.with(|flag| flag.get()) {
        log::error!("thread '{thread_name}' panicked at {span} (allowing unwind):\n{message}");
        return;
    }

    // wait 500ms for the crash handler process to start up
    // if it's still not there just write panic info and no minidump
    let retry_frequency = Duration::from_millis(100);
    for _ in 0..5 {
        if let Some(client) = CRASH_HANDLER.get() {
            let location = info
                .location()
                .map_or_else(|| "<unknown>".to_owned(), |location| location.to_string());
            log::error!("thread '{thread_name}' panicked at {location}:\n{message}...");
            client
                .send_message(
                    2,
                    serde_json::to_vec(&CrashPanic { message, span }).unwrap(),
                )
                .ok();
            log::error!("triggering a crash to generate a minidump...");

            #[cfg(target_os = "macos")]
            PANIC_THREAD_ID.store(
                unsafe { mach2::mach_init::mach_thread_self() },
                Ordering::SeqCst,
            );

            cfg_if::cfg_if! {
                if #[cfg(target_os = "windows")] {
                    // https://learn.microsoft.com/en-us/windows/win32/debug/system-error-codes--0-499-
                    CrashHandler.simulate_exception(Some(234)); // (MORE_DATA_AVAILABLE)
                    break;
                } else {
                    std::process::abort();
                }
            }
        }
        thread::sleep(retry_frequency);
    }
}

#[cfg(target_os = "windows")]
fn spawn_crash_handler_windows(exe: &Path, socket_name: &Path) {
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

pub fn crash_server(socket: &Path) {
    let Ok(mut server) = minidumper::Server::with_name(socket) else {
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
                initialization_params: OnceLock::new(),
                panic_info: OnceLock::new(),
                has_connection,
                active_gpu: OnceLock::new(),
            }),
            &shutdown,
            Some(CRASH_HANDLER_PING_TIMEOUT),
        )
        .expect("failed to run server");
}
