use crash_handler::CrashHandler;
use log::info;
use minidumper::{Client, LoopAction, MinidumpBinary};

use std::{
    env,
    fs::File,
    io::{self, Write},
    path::{Path, PathBuf},
    process::{self, Command},
    sync::atomic::{AtomicBool, Ordering},
    thread,
    time::Duration,
};

pub static CRASH_HANDLER: AtomicBool = AtomicBool::new(false);
pub static REQUESTED_MINIDUMP: AtomicBool = AtomicBool::new(false);

// meant to be detached to lazily set up a crash handler, the CRASH_HANDLER atomic bool will
// be set to true once initialization is complete
pub async fn init() {
    let exe = env::current_exe().expect("unable to find ourselves");
    let zed_pid = process::id();
    let socket_name = paths::temp_dir().join(format!("zed-crash-handler-{zed_pid}"));
    #[allow(unused)]
    let server_pid = Command::new(exe)
        .arg("--crash-handler")
        .arg(&socket_name)
        .spawn()
        .expect("unable to spawn server process")
        .id();
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
    let handler = crash_handler::CrashHandler::attach(unsafe {
        crash_handler::make_crash_event(move |crash_context: &crash_handler::CrashContext| {
            // only request a minidump once
            let res = if REQUESTED_MINIDUMP
                .compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed)
                .is_ok()
            {
                client.send_message(2, "mistakes were made").unwrap();
                client.ping().unwrap();
                client.request_dump(crash_context).is_ok()
            } else {
                true
            };
            crash_handler::CrashEventResult::Handled(res)
        })
    })
    .expect("failed to attach signal handler");

    #[cfg(target_os = "linux")]
    {
        handler.set_ptracer(Some(server_pid));
    }
    CRASH_HANDLER.store(true, Ordering::Release);
    std::mem::forget(handler);
    info!("crash handler registered");
}

pub struct CrashServer;

impl minidumper::ServerHandler for CrashServer {
    fn create_minidump_file(&self) -> Result<(File, PathBuf), io::Error> {
        let dump_path = paths::logs_dir()
            .join(uuid::Uuid::new_v4().to_string())
            .with_extension("dmp");
        let file = File::create(&dump_path)?;
        Ok((file, dump_path))
    }

    fn on_minidump_created(&self, result: Result<MinidumpBinary, minidumper::Error>) -> LoopAction {
        match result {
            Ok(mut md_bin) => {
                use Write;
                let _ = md_bin.file.flush();
                info!("wrote minidump to disk {:?}", md_bin.path);
            }
            Err(e) => {
                info!("failed to write minidump: {:#}", e);
            }
        }

        LoopAction::Exit
    }

    fn on_message(&self, kind: u32, buffer: Vec<u8>) {
        info!(
            "kind: {kind}, message: {}",
            String::from_utf8(buffer).unwrap()
        );
    }

    fn on_client_disconnected(&self, clients: usize) -> LoopAction {
        info!("client disconnected, {clients} remaining");
        if clients == 0 {
            LoopAction::Exit
        } else {
            LoopAction::Continue
        }
    }
}

pub fn handle_panic() {
    // wait 500ms for the crash handler process to start up
    // if it's still not there just write panic info and no minidump
    let retry_frequency = Duration::from_millis(100);
    for _ in 0..5 {
        if CRASH_HANDLER.load(Ordering::Acquire) {
            log::error!("triggering a crash to generate a minidump...");
            #[cfg(linux)]
            CrashHandler.simulate_signal(crash_handler::SIGTRAP);
            #[cfg(not(linux))]
            CrashHandler.simulate_exception(None);
            break;
        }
        thread::sleep(retry_frequency);
    }
}

pub fn crash_server(socket: &Path) {
    let mut server = minidumper::Server::with_name(socket).expect("failed to create server");
    let ab = AtomicBool::new(false);
    server
        .run(Box::new(CrashServer), &ab, None)
        .expect("failed to run server");
}
