use crash_handler::{CrashEventResult, CrashHandler};
use log::info;
use minidumper::{Client, LoopAction, MinidumpBinary};
use release_channel::{RELEASE_CHANNEL, ReleaseChannel};
use serde::{Deserialize, Serialize};
use smol::process::Command;

#[cfg(target_os = "macos")]
use std::sync::atomic::AtomicU32;
use std::{
    env,
    fs::{self, File},
    io,
    panic::{self, PanicHookInfo},
    path::{Path, PathBuf},
    process::{self},
    sync::{
        Arc, OnceLock,
        atomic::{AtomicBool, Ordering},
    },
    thread,
    time::Duration,
};

// set once the crash handler has initialized and the client has connected to it
pub static CRASH_HANDLER: OnceLock<Arc<Client>> = OnceLock::new();
// set when the first minidump request is made to avoid generating duplicate crash reports
pub static REQUESTED_MINIDUMP: AtomicBool = AtomicBool::new(false);
const CRASH_HANDLER_PING_TIMEOUT: Duration = Duration::from_secs(60);
const CRASH_HANDLER_CONNECT_TIMEOUT: Duration = Duration::from_secs(10);

#[cfg(target_os = "macos")]
static PANIC_THREAD_ID: AtomicU32 = AtomicU32::new(0);

pub async fn init(crash_init: InitCrashHandler) {
    if *RELEASE_CHANNEL == ReleaseChannel::Dev && env::var("ZED_GENERATE_MINIDUMPS").is_err() {
        let old_hook = panic::take_hook();
        panic::set_hook(Box::new(move |info| {
            unsafe { env::set_var("RUST_BACKTRACE", "1") };
            old_hook(info);
            // prevent the macOS crash dialog from popping up
            std::process::exit(1);
        }));
        return;
    } else {
        panic::set_hook(Box::new(panic_hook));
    }

    let exe = env::current_exe().expect("unable to find ourselves");
    let zed_pid = process::id();
    // TODO: we should be able to get away with using 1 crash-handler process per machine,
    // but for now we append the PID of the current process which makes it unique per remote
    // server or interactive zed instance. This solves an issue where occasionally the socket
    // used by the crash handler isn't destroyed correctly which causes it to stay on the file
    // system and block further attempts to initialize crash handlers with that socket path.
    let socket_name = paths::temp_dir().join(format!("zed-crash-handler-{zed_pid}"));
    let _crash_handler = Command::new(exe)
        .arg("--crash-handler")
        .arg(&socket_name)
        .spawn()
        .expect("unable to spawn server process");
    #[cfg(target_os = "linux")]
    let server_pid = _crash_handler.id();
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
        smol::Timer::after(retry_frequency).await;
    }
    let client = maybe_client.unwrap();
    client
        .send_message(1, serde_json::to_vec(&crash_init).unwrap())
        .unwrap();

    let client = Arc::new(client);
    let handler = CrashHandler::attach(unsafe {
        let client = client.clone();
        crash_handler::make_crash_event(move |crash_context: &crash_handler::CrashContext| {
            // only request a minidump once
            let res = if REQUESTED_MINIDUMP
                .compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed)
                .is_ok()
            {
                #[cfg(target_os = "macos")]
                suspend_all_other_threads();

                client.ping().unwrap();
                client.request_dump(crash_context).is_ok()
            } else {
                true
            };
            CrashEventResult::Handled(res)
        })
    })
    .expect("failed to attach signal handler");

    #[cfg(target_os = "linux")]
    {
        handler.set_ptracer(Some(server_pid));
    }
    CRASH_HANDLER.set(client.clone()).ok();
    std::mem::forget(handler);
    info!("crash handler registered");

    loop {
        client.ping().ok();
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
                self.active_gpu
                    .set(gpu_specs)
                    .expect("already set active gpu");
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
    let message = info
        .payload()
        .downcast_ref::<&str>()
        .map(|s| s.to_string())
        .or_else(|| info.payload().downcast_ref::<String>().cloned())
        .unwrap_or_else(|| "Box<Any>".to_string());

    let span = info
        .location()
        .map(|loc| format!("{}:{}", loc.file(), loc.line()))
        .unwrap_or_default();

    // wait 500ms for the crash handler process to start up
    // if it's still not there just write panic info and no minidump
    let retry_frequency = Duration::from_millis(100);
    for _ in 0..5 {
        if let Some(client) = CRASH_HANDLER.get() {
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
