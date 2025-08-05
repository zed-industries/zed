use crash_handler::CrashHandler;
use log::info;
use minidumper::{Client, LoopAction, MinidumpBinary};

use std::{
    env,
    fs::File,
    io,
    path::{Path, PathBuf},
    process::{self, Command},
    sync::{
        OnceLock,
        atomic::{AtomicBool, Ordering},
    },
    thread,
    time::Duration,
};

// set once the crash handler has initialized and the client has connected to it
pub static CRASH_HANDLER: AtomicBool = AtomicBool::new(false);
// set when the first minidump request is made to avoid generating duplicate crash reports
pub static REQUESTED_MINIDUMP: AtomicBool = AtomicBool::new(false);
const CRASH_HANDLER_TIMEOUT: Duration = Duration::from_secs(60);

pub async fn init(id: String) {
    let exe = env::current_exe().expect("unable to find ourselves");
    let zed_pid = process::id();
    // TODO: we should be able to get away with using 1 crash-handler process per machine,
    // but for now we append the PID of the current process which makes it unique per remote
    // server or interactive zed instance. This solves an issue where occasionally the socket
    // used by the crash handler isn't destroyed correctly which causes it to stay on the file
    // system and block further attempts to initialize crash handlers with that socket path.
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
    client.send_message(1, id).unwrap(); // set session id on the server

    let client = std::sync::Arc::new(client);
    let handler = crash_handler::CrashHandler::attach(unsafe {
        let client = client.clone();
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

    loop {
        client.ping().ok();
        smol::Timer::after(Duration::from_secs(10)).await;
    }
}

pub struct CrashServer {
    session_id: OnceLock<String>,
}

impl minidumper::ServerHandler for CrashServer {
    fn create_minidump_file(&self) -> Result<(File, PathBuf), io::Error> {
        let err_message = "Need to send a message with the ID upon starting the crash handler";
        let dump_path = paths::logs_dir()
            .join(self.session_id.get().expect(err_message))
            .with_extension("dmp");
        let file = File::create(&dump_path)?;
        Ok((file, dump_path))
    }

    fn on_minidump_created(&self, result: Result<MinidumpBinary, minidumper::Error>) -> LoopAction {
        match result {
            Ok(mut md_bin) => {
                use io::Write;
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
        let message = String::from_utf8(buffer).expect("invalid utf-8");
        info!("kind: {kind}, message: {message}",);
        if kind == 1 {
            self.session_id
                .set(message)
                .expect("session id already initialized");
        }
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
            #[cfg(target_os = "linux")]
            CrashHandler.simulate_signal(crash_handler::Signal::Trap as u32);
            #[cfg(not(target_os = "linux"))]
            CrashHandler.simulate_exception(None);
            break;
        }
        thread::sleep(retry_frequency);
    }
}

pub fn crash_server(socket: &Path) {
    let Ok(mut server) = minidumper::Server::with_name(socket) else {
        log::info!("Couldn't create socket, there may already be a running crash server");
        return;
    };
    let ab = AtomicBool::new(false);
    server
        .run(
            Box::new(CrashServer {
                session_id: OnceLock::new(),
            }),
            &ab,
            Some(CRASH_HANDLER_TIMEOUT),
        )
        .expect("failed to run server");
}
