use std::{
    ffi::c_int,
    sync::{mpsc, OnceLock},
    time::Duration,
};

use gpui::{BackgroundExecutor, ForegroundExecutor};
use smol::stream::StreamExt;

// Must initialize backtrace_sender first

#[cfg(target_os = "macos")]
pub fn init(foreground_executor: ForegroundExecutor, background_executor: BackgroundExecutor) {
    static BACKTRACE_SENDER: OnceLock<mpsc::Sender<std::backtrace::Backtrace>> = OnceLock::new();

    let (backtrace_tx, backtrace_rx) = mpsc::channel();

    BACKTRACE_SENDER.get_or_init(|| backtrace_tx);

    fn get_backtrace_sender() -> mpsc::Sender<std::backtrace::Backtrace> {
        BACKTRACE_SENDER.get().unwrap().clone()
    }

    use std::thread;

    use nix::{
        sys::signal::{
            sigaction, SaFlags, SigAction, SigHandler, SigSet,
            Signal::{self, SIGUSR2},
        },
        unistd::Pid,
    };
    unsafe {
        extern "C" fn handle_sigusr2(_i: c_int) {
            BACKTRACE_SENDER
                .get()
                .unwrap()
                // NOTE: this allocates, it shouldn't (probably)
                .send(std::backtrace::Backtrace::force_capture())
                .ok();
        }

        let mut mask = SigSet::empty();
        mask.add(SIGUSR2);
        sigaction(
            Signal::SIGUSR2,
            &SigAction::new(
                SigHandler::Handler(handle_sigusr2),
                SaFlags::SA_RESTART,
                mask,
            ),
        )
        .unwrap();
    }

    let (mut tx, mut rx) = futures::channel::mpsc::channel(3);
    foreground_executor
        .spawn(async move { while let Some(_) = rx.next().await {} })
        .detach();

    foreground_executor
        .spawn(async move {
            fn really_expensive_blocking_thing() {
                loop {
                    thread::sleep(Duration::from_millis(500));
                    println!("Oh no");
                }
            }

            really_expensive_blocking_thing();
        })
        .detach();

    background_executor
        .spawn({
            let background_executor = background_executor.clone();
            async move {
                loop {
                    background_executor.timer(Duration::from_secs(1)).await;
                    match tx.try_send(()) {
                        Ok(_) => continue,
                        Err(e) => {
                            if e.into_send_error().is_full() {
                                nix::sys::signal::kill(Pid::this(), SIGUSR2).unwrap();
                            }
                            break;
                        }
                    }
                }
            }
        })
        .detach();

    background_executor
        .clone()
        .spawn(async move {
            loop {
                while let Some(backtrace) = backtrace_rx.recv().ok() {
                    if telemetry_settings.diagnostics {}
                    println!("Suspected hang on main thread: {:?}", backtrace);
                }
            }
        })
        .detach()
}

#[cfg(not(target_os = "macos"))]
pub fn init(_foreground_executor: ForegroundExecutor, _background_executor: BackgroundExecutor) {}
