use anyhow::{Context, Result};
use backtrace::{self, Backtrace};
use chrono::Utc;
use db::kvp::KEY_VALUE_STORE;
use gpui::{App, AppContext, SemanticVersion};
use isahc::config::Configurable;

use paths::{CRASHES_DIR, CRASHES_RETIRED_DIR};
use release_channel::ReleaseChannel;
use release_channel::RELEASE_CHANNEL;
use serde::{Deserialize, Serialize};
use settings::Settings;
use smol::stream::StreamExt;
use std::{
    env,
    ffi::OsStr,
    sync::{atomic::Ordering, Arc},
};
use std::{io::Write, panic, sync::atomic::AtomicU32, thread};
use util::{
    http::{self, HttpClient, HttpClientWithUrl},
    paths, ResultExt,
};

use crate::stdout_is_a_pty;

#[derive(Serialize, Deserialize)]
struct LocationData {
    file: String,
    line: u32,
}

#[derive(Serialize, Deserialize)]
struct Panic {
    thread: String,
    payload: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    location_data: Option<LocationData>,
    backtrace: Vec<String>,
    app_version: String,
    release_channel: String,
    os_name: String,
    os_version: Option<String>,
    architecture: String,
    panicked_on: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    installation_id: Option<String>,
    session_id: String,
}

#[derive(Serialize)]
struct PanicRequest {
    panic: Panic,
}

static PANIC_COUNT: AtomicU32 = AtomicU32::new(0);

pub fn init_panic_hook(app: &App, installation_id: Option<String>, session_id: String) {
    let is_pty = stdout_is_a_pty();
    let app_metadata = app.metadata();

    panic::set_hook(Box::new(move |info| {
        let prior_panic_count = PANIC_COUNT.fetch_add(1, Ordering::SeqCst);
        if prior_panic_count > 0 {
            // Give the panic-ing thread time to write the panic file
            loop {
                std::thread::yield_now();
            }
        }

        let thread = thread::current();
        let thread_name = thread.name().unwrap_or("<unnamed>");

        let payload = info
            .payload()
            .downcast_ref::<&str>()
            .map(|s| s.to_string())
            .or_else(|| info.payload().downcast_ref::<String>().map(|s| s.clone()))
            .unwrap_or_else(|| "Box<Any>".to_string());

        if *release_channel::RELEASE_CHANNEL == ReleaseChannel::Dev {
            let location = info.location().unwrap();
            let backtrace = Backtrace::new();
            eprintln!(
                "Thread {:?} panicked with {:?} at {}:{}:{}\n{:?}",
                thread_name,
                payload,
                location.file(),
                location.line(),
                location.column(),
                backtrace,
            );
            std::process::exit(-1);
        }

        let app_version = if let Some(version) = app_metadata.app_version {
            version.to_string()
        } else {
            option_env!("CARGO_PKG_VERSION")
                .unwrap_or("dev")
                .to_string()
        };

        let backtrace = Backtrace::new();
        let mut backtrace = backtrace
            .frames()
            .iter()
            .flat_map(|frame| {
                frame
                    .symbols()
                    .iter()
                    .filter_map(|frame| Some(format!("{:#}", frame.name()?)))
            })
            .collect::<Vec<_>>();

        // Strip out leading stack frames for rust panic-handling.
        if let Some(ix) = backtrace
            .iter()
            .position(|name| name == "rust_begin_unwind")
        {
            backtrace.drain(0..=ix);
        }

        let panic_data = Panic {
            thread: thread_name.into(),
            payload,
            location_data: info.location().map(|location| LocationData {
                file: location.file().into(),
                line: location.line(),
            }),
            app_version: app_version.to_string(),
            release_channel: RELEASE_CHANNEL.display_name().into(),
            os_name: app_metadata.os_name.into(),
            os_version: app_metadata
                .os_version
                .as_ref()
                .map(SemanticVersion::to_string),
            architecture: env::consts::ARCH.into(),
            panicked_on: Utc::now().timestamp_millis(),
            backtrace,
            installation_id: installation_id.clone(),
            session_id: session_id.clone(),
        };

        if let Some(panic_data_json) = serde_json::to_string_pretty(&panic_data).log_err() {
            log::error!("{}", panic_data_json);
        }

        if !is_pty {
            if let Some(panic_data_json) = serde_json::to_string(&panic_data).log_err() {
                let timestamp = chrono::Utc::now().format("%Y_%m_%d %H_%M_%S").to_string();
                let panic_file_path = paths::LOGS_DIR.join(format!("zed-{}.panic", timestamp));
                let panic_file = std::fs::OpenOptions::new()
                    .append(true)
                    .create(true)
                    .open(&panic_file_path)
                    .log_err();
                if let Some(mut panic_file) = panic_file {
                    writeln!(&mut panic_file, "{}", panic_data_json).log_err();
                    panic_file.flush().log_err();
                }
            }
        }

        std::process::abort();
    }));
}

pub fn init(
    http_client: Arc<HttpClientWithUrl>,
    installation_id: Option<String>,
    cx: &mut AppContext,
) {
    #[cfg(target_os = "macos")]
    monitor_main_thread_hangs(http_client.clone(), installation_id.clone(), cx);

    upload_panics_and_crashes(http_client, installation_id, cx)
}

#[cfg(target_os = "macos")]
pub fn monitor_main_thread_hangs(
    http_client: Arc<HttpClientWithUrl>,
    installation_id: Option<String>,
    cx: &AppContext,
) {
    use nix::sys::signal::{
        sigaction, SaFlags, SigAction, SigHandler, SigSet,
        Signal::{self, SIGUSR2},
    };

    use parking_lot::Mutex;

    use std::{
        ffi::c_int,
        sync::{mpsc, OnceLock},
        time::Duration,
    };
    use telemetry_events::{BacktraceFrame, HangReport};
    use util::http::Method;

    use nix::sys::pthread;

    let foreground_executor = cx.foreground_executor();
    let background_executor = cx.background_executor();
    let telemetry_settings = *client::TelemetrySettings::get_global(cx);
    let metadata = cx.app_metadata();

    // Initialize SIGUSR2 handler to send a backrace to a channel.
    let (backtrace_tx, backtrace_rx) = mpsc::channel();
    static BACKTRACE: Mutex<Vec<backtrace::Frame>> = Mutex::new(Vec::new());
    static BACKTRACE_SENDER: OnceLock<mpsc::Sender<()>> = OnceLock::new();
    BACKTRACE_SENDER.get_or_init(|| backtrace_tx);
    BACKTRACE.lock().reserve(100);

    fn handle_backtrace_signal() {
        unsafe {
            extern "C" fn handle_sigusr2(_i: c_int) {
                unsafe {
                    // ASYNC SIGNAL SAFETY: This lock is only accessed one other time,
                    // which can only be triggered by This signal handler. In addition,
                    // this signal handler is immediately removed by SA_RESETHAND, and this
                    // signal handler cannot be re-entrant due to to the SIGUSR2 mask defined
                    // below
                    let mut bt = BACKTRACE.lock();
                    bt.clear();
                    backtrace::trace_unsynchronized(|frame| {
                        if bt.len() < bt.capacity() {
                            bt.push(frame.clone());
                            true
                        } else {
                            false
                        }
                    });
                }

                BACKTRACE_SENDER.get().unwrap().send(()).ok();
            }

            let mut mask = SigSet::empty();
            mask.add(SIGUSR2);
            sigaction(
                Signal::SIGUSR2,
                &SigAction::new(
                    SigHandler::Handler(handle_sigusr2),
                    SaFlags::SA_RESTART | SaFlags::SA_RESETHAND,
                    mask,
                ),
            )
            .log_err();
        }
    }

    handle_backtrace_signal();
    let main_thread = pthread::pthread_self();

    let (mut tx, mut rx) = futures::channel::mpsc::channel(3);
    foreground_executor
        .spawn(async move { while let Some(_) = rx.next().await {} })
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
                                pthread::pthread_kill(main_thread, SIGUSR2).log_err();
                            }
                            // Only detect the first hang
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
                while let Some(_) = backtrace_rx.recv().ok() {
                    if !telemetry_settings.diagnostics {
                        return;
                    }

                    // ASYNC SIGNAL SAFETY: This lock is only accessed _after_
                    // the backtrace transmitter has fired, which itself is only done
                    // by the signal handler. And due to SA_RESETHAND  the signal handler
                    // will not run again until `handle_backtrace_signal` is called.
                    let raw_backtrace = BACKTRACE.lock().drain(..).collect::<Vec<_>>();
                    let backtrace: Vec<_> = raw_backtrace
                        .into_iter()
                        .map(|frame| {
                            let mut btf = BacktraceFrame {
                                ip: frame.ip() as usize,
                                symbol_addr: frame.symbol_address() as usize,
                                base: frame.module_base_address().map(|addr| addr as usize),
                                symbols: vec![],
                            };

                            backtrace::resolve_frame(&frame, |symbol| {
                                if let Some(name) = symbol.name() {
                                    btf.symbols.push(name.to_string());
                                }
                            });

                            btf
                        })
                        .collect();

                    // IMPORTANT: Don't move this to before `BACKTRACE.lock()`
                    handle_backtrace_signal();

                    log::error!(
                        "Suspected hang on main thread:\n{}",
                        backtrace
                            .iter()
                            .flat_map(|bt| bt.symbols.first().as_ref().map(|s| s.as_str()))
                            .collect::<Vec<_>>()
                            .join("\n")
                    );

                    let report = HangReport {
                        backtrace,
                        app_version: metadata.app_version,
                        os_name: metadata.os_name.to_owned(),
                        os_version: metadata.os_version,
                        architecture: env::consts::ARCH.into(),
                        installation_id: installation_id.clone(),
                    };

                    let Some(json_bytes) = serde_json::to_vec(&report).log_err() else {
                        continue;
                    };

                    let Some(checksum) = client::telemetry::calculate_json_checksum(&json_bytes)
                    else {
                        continue;
                    };

                    let Ok(url) = http_client.build_zed_api_url("/telemetry/hangs", &[]) else {
                        continue;
                    };

                    let Ok(request) = http::Request::builder()
                        .method(Method::POST)
                        .uri(url.as_ref())
                        .header("x-zed-checksum", checksum)
                        .body(json_bytes.into())
                    else {
                        continue;
                    };

                    if let Some(response) = http_client.send(request).await.log_err() {
                        if response.status() != 200 {
                            log::error!("Failed to send hang report: HTTP {:?}", response.status());
                        }
                    }
                }
            }
        })
        .detach()
}

fn upload_panics_and_crashes(
    http: Arc<HttpClientWithUrl>,
    installation_id: Option<String>,
    cx: &mut AppContext,
) {
    let telemetry_settings = *client::TelemetrySettings::get_global(cx);
    cx.background_executor()
        .spawn(async move {
            let most_recent_panic = upload_previous_panics(http.clone(), telemetry_settings)
                .await
                .log_err()
                .flatten();
            upload_previous_crashes(http, most_recent_panic, installation_id, telemetry_settings)
                .await
                .log_err()
        })
        .detach()
}

/// Uploads panics via `zed.dev`.
async fn upload_previous_panics(
    http: Arc<HttpClientWithUrl>,
    telemetry_settings: client::TelemetrySettings,
) -> Result<Option<(i64, String)>> {
    let panic_report_url = http.build_url("/api/panic");
    let mut children = smol::fs::read_dir(&*paths::LOGS_DIR).await?;

    let mut most_recent_panic = None;

    while let Some(child) = children.next().await {
        let child = child?;
        let child_path = child.path();

        if child_path.extension() != Some(OsStr::new("panic")) {
            continue;
        }
        let filename = if let Some(filename) = child_path.file_name() {
            filename.to_string_lossy()
        } else {
            continue;
        };

        if !filename.starts_with("zed") {
            continue;
        }

        if telemetry_settings.diagnostics {
            let panic_file_content = smol::fs::read_to_string(&child_path)
                .await
                .context("error reading panic file")?;

            let panic: Option<Panic> = serde_json::from_str(&panic_file_content)
                .ok()
                .or_else(|| {
                    panic_file_content
                        .lines()
                        .next()
                        .and_then(|line| serde_json::from_str(line).ok())
                })
                .unwrap_or_else(|| {
                    log::error!("failed to deserialize panic file {:?}", panic_file_content);
                    None
                });

            if let Some(panic) = panic {
                most_recent_panic = Some((panic.panicked_on, panic.payload.clone()));

                let body = serde_json::to_string(&PanicRequest { panic }).unwrap();

                let request = http::Request::post(&panic_report_url)
                    .redirect_policy(isahc::config::RedirectPolicy::Follow)
                    .header("Content-Type", "application/json")
                    .body(body.into())?;
                let response = http.send(request).await.context("error sending panic")?;
                if !response.status().is_success() {
                    log::error!("Error uploading panic to server: {}", response.status());
                }
            }
        }

        // We've done what we can, delete the file
        std::fs::remove_file(child_path)
            .context("error removing panic")
            .log_err();
    }
    Ok::<_, anyhow::Error>(most_recent_panic)
}

static LAST_CRASH_UPLOADED: &'static str = "LAST_CRASH_UPLOADED";

/// upload crashes from apple's diagnostic reports to our server.
/// (only if telemetry is enabled)
async fn upload_previous_crashes(
    http: Arc<HttpClientWithUrl>,
    most_recent_panic: Option<(i64, String)>,
    installation_id: Option<String>,
    telemetry_settings: client::TelemetrySettings,
) -> Result<()> {
    if !telemetry_settings.diagnostics {
        return Ok(());
    }
    let last_uploaded = KEY_VALUE_STORE
        .read_kvp(LAST_CRASH_UPLOADED)?
        .unwrap_or("zed-2024-01-17-221900.ips".to_string()); // don't upload old crash reports from before we had this.
    let mut uploaded = last_uploaded.clone();

    let crash_report_url = http.build_zed_api_url("/telemetry/crashes", &[])?;

    // crash directories are only set on MacOS
    for dir in [&*CRASHES_DIR, &*CRASHES_RETIRED_DIR]
        .iter()
        .filter_map(|d| d.as_deref())
    {
        let mut children = smol::fs::read_dir(&dir).await?;
        while let Some(child) = children.next().await {
            let child = child?;
            let Some(filename) = child
                .path()
                .file_name()
                .map(|f| f.to_string_lossy().to_lowercase())
            else {
                continue;
            };

            if !filename.starts_with("zed-") || !filename.ends_with(".ips") {
                continue;
            }

            if filename <= last_uploaded {
                continue;
            }

            let body = smol::fs::read_to_string(&child.path())
                .await
                .context("error reading crash file")?;

            let mut request = http::Request::post(&crash_report_url.to_string())
                .redirect_policy(isahc::config::RedirectPolicy::Follow)
                .header("Content-Type", "text/plain");

            if let Some((panicked_on, payload)) = most_recent_panic.as_ref() {
                request = request
                    .header("x-zed-panicked-on", format!("{}", panicked_on))
                    .header("x-zed-panic", payload)
            }
            if let Some(installation_id) = installation_id.as_ref() {
                request = request.header("x-zed-installation-id", installation_id);
            }

            let request = request.body(body.into())?;

            let response = http.send(request).await.context("error sending crash")?;
            if !response.status().is_success() {
                log::error!("Error uploading crash to server: {}", response.status());
            }

            if uploaded < filename {
                uploaded.clone_from(&filename);
                KEY_VALUE_STORE
                    .write_kvp(LAST_CRASH_UPLOADED.to_string(), filename)
                    .await?;
            }
        }
    }

    Ok(())
}
