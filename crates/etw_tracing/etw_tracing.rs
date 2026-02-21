#![cfg(target_os = "windows")]

use anyhow::{Context as _, Result, bail};
use gpui::{App, AppContext as _, DismissEvent, Global, actions};
use std::fmt::Write as _;
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};
use std::time::Duration;
use util::{ResultExt as _, defer};
use windows::Win32::Foundation::{VARIANT_BOOL, VARIANT_FALSE};
use windows::Win32::System::Com::{CLSCTX_INPROC_SERVER, COINIT_MULTITHREADED, CoInitializeEx};
use windows_core::{BSTR, Interface};
use workspace::notifications::simple_message_notification::MessageNotification;
use workspace::notifications::{NotificationId, show_app_notification};
use wprcontrol::*;

actions!(
    zed,
    [
        /// Starts recording an ETW (Event Tracing for Windows) trace.
        RecordEtwTrace,
        /// Starts recording an ETW (Event Tracing for Windows) trace with heap tracing.
        RecordEtwTraceWithHeapTracing,
        /// Saves an in-progress ETW trace to disk.
        SaveEtwTrace,
        /// Cancels an in-progress ETW trace without saving.
        CancelEtwTrace,
    ]
);

struct EtwNotification;

struct EtwSessionHandle {
    writer: net::OwnedWriteHalf,
    _listener: net::UnixListener,
    socket_path: PathBuf,
}

impl Drop for EtwSessionHandle {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.socket_path);
    }
}

struct GlobalEtwSession(Option<EtwSessionHandle>);

impl Global for GlobalEtwSession {}

fn has_active_etw_session(cx: &App) -> bool {
    cx.global::<GlobalEtwSession>().0.is_some()
}

fn show_etw_notification(cx: &mut App, message: impl Into<gpui::SharedString>) {
    let message = message.into();
    show_app_notification(NotificationId::unique::<EtwNotification>(), cx, move |cx| {
        cx.new(|cx| MessageNotification::new(message.clone(), cx))
    });
}

fn show_etw_notification_with_action(
    cx: &mut App,
    message: impl Into<gpui::SharedString>,
    button_label: impl Into<gpui::SharedString>,
    on_click: impl Fn(&mut gpui::Window, &mut gpui::Context<MessageNotification>)
    + Send
    + Sync
    + 'static,
) {
    let message = message.into();
    let button_label = button_label.into();
    let on_click = std::sync::Arc::new(on_click);
    show_app_notification(NotificationId::unique::<EtwNotification>(), cx, move |cx| {
        let message = message.clone();
        let button_label = button_label.clone();
        cx.new(|cx| {
            MessageNotification::new(message, cx)
                .primary_message(button_label)
                .primary_on_click_arc(on_click.clone())
        })
    });
}

fn show_etw_status_notification(cx: &mut App, status: Result<StatusMessage>, output_path: PathBuf) {
    match status {
        Ok(StatusMessage::Stopped) => {
            let display_path = output_path.display().to_string();
            show_etw_notification_with_action(
                cx,
                format!("ETW trace saved to {display_path}"),
                "Show in File Manager",
                move |_window, cx| {
                    cx.reveal_path(&output_path);
                    cx.emit(DismissEvent);
                },
            );
        }
        Ok(StatusMessage::TimedOut) => {
            let display_path = output_path.display().to_string();
            show_etw_notification_with_action(
                cx,
                format!("ETW recording timed out. Trace saved to {display_path}"),
                "Show in File Manager",
                move |_window, cx| {
                    cx.reveal_path(&output_path);
                    cx.emit(DismissEvent);
                },
            );
        }
        Ok(StatusMessage::Cancelled) => {
            show_etw_notification(cx, "ETW recording cancelled");
        }
        Ok(_) => {
            show_etw_notification(cx, "ETW recording ended unexpectedly");
        }
        Err(error) => {
            show_etw_notification(cx, format!("Failed to complete ETW recording: {error:#}"));
        }
    }
}

pub fn init(cx: &mut App) {
    cx.set_global(GlobalEtwSession(None));

    cx.on_action(|_: &RecordEtwTrace, cx: &mut App| {
        start_etw_recording(cx, None);
    });

    cx.on_action(|_: &RecordEtwTraceWithHeapTracing, cx: &mut App| {
        start_etw_recording(cx, Some(std::process::id()));
    });

    cx.on_action(|_: &SaveEtwTrace, cx: &mut App| {
        let session = cx.global_mut::<GlobalEtwSession>().0.as_mut();
        let Some(session) = session else {
            show_etw_notification(cx, "No active ETW recording to stop");
            return;
        };
        match send_json(&mut session.writer, &Command::Save) {
            Ok(()) => {
                show_etw_notification(cx, "Stopping ETW recording...");
            }
            Err(error) => {
                show_etw_notification(cx, format!("Failed to stop ETW recording: {error:#}"));
            }
        }
    });

    cx.on_action(|_: &CancelEtwTrace, cx: &mut App| {
        let session = cx.global_mut::<GlobalEtwSession>().0.as_mut();
        let Some(session) = session else {
            show_etw_notification(cx, "No active ETW recording to cancel");
            return;
        };
        match send_json(&mut session.writer, &Command::Cancel) {
            Ok(()) => {
                show_etw_notification(cx, "Cancelling ETW recording...");
            }
            Err(error) => {
                show_etw_notification(cx, format!("Failed to cancel ETW recording: {error:#}"));
            }
        }
    });
}

fn start_etw_recording(cx: &mut App, heap_pid: Option<u32>) {
    if has_active_etw_session(cx) {
        show_etw_notification(cx, "ETW recording is already in progress");
        return;
    }
    let save_dialog = cx.prompt_for_new_path(&PathBuf::default(), Some("zed-trace.etl"));
    cx.spawn(async move |cx| {
        let output_path = match save_dialog.await {
            Ok(Ok(Some(path))) => path,
            Ok(Ok(None)) => return,
            Ok(Err(error)) => {
                cx.update(|cx| {
                    show_etw_notification(cx, format!("Failed to pick save location: {error:#}"));
                });
                return;
            }
            Err(_) => return,
        };

        let result = cx
            .background_spawn(async move { launch_etw_recording(heap_pid, &output_path) })
            .await;

        let EtwSession {
            output_path,
            stream,
            listener,
            socket_path,
        } = match result {
            Ok(session) => session,
            Err(error) => {
                cx.update(|cx| {
                    show_etw_notification(cx, format!("Failed to start ETW recording: {error:#}"));
                });
                return;
            }
        };

        let (read_half, write_half) = stream.into_inner().into_split();

        cx.spawn(async |cx| {
            let status = cx
                .background_spawn(async move {
                    recv_json(&mut BufReader::new(read_half))
                        .context("Receive status from subprocess")
                })
                .await;
            cx.update(|cx| {
                cx.global_mut::<GlobalEtwSession>().0 = None;
                show_etw_status_notification(cx, status, output_path);
            });
        })
        .detach();

        cx.update(|cx| {
            cx.global_mut::<GlobalEtwSession>().0 = Some(EtwSessionHandle {
                writer: write_half,
                _listener: listener,
                socket_path,
            });
            show_etw_notification(cx, "ETW recording started");
        });
    })
    .detach();
}

const RECORDING_TIMEOUT: Duration = Duration::from_secs(60);

const INSTANCE_NAME: &str = "Zed";

const BUILTIN_PROFILES: &[&str] = &[
    "CPU.Verbose.Memory",
    "GPU.Light.Memory",
    "DiskIO.Light.Memory",
    "FileIO.Light.Memory",
];

fn heap_tracing_profile(heap_pid: Option<u32>) -> String {
    let (heap_provider, heap_collector) = match heap_pid {
        Some(pid) => (
            format!(
                r#"
    <HeapEventProvider Id="ZedHeapProvider">
      <HeapProcessIds Operation="Set">
        <HeapProcessId Value="{pid}"/>
      </HeapProcessIds>
    </HeapEventProvider>"#
            ),
            r#"
      <Collectors Operation="Add">
        <HeapEventCollectorId Value="HeapCollector_WPRHeapCollector">
          <HeapEventProviders Operation="Set">
            <HeapEventProviderId Value="ZedHeapProvider"/>
          </HeapEventProviders>
        </HeapEventCollectorId>
      </Collectors>"#
                .to_string(),
        ),
        None => (String::new(), String::new()),
    };

    format!(
        r#"<?xml version="1.0" encoding="utf-8"?>
<WindowsPerformanceRecorder Version="1.0" Author="Zed Industries">
  <Profiles>
    {heap_provider}

    <Profile Id="ZedHeap.Verbose.Memory" Base="Heap.Verbose.Memory" Name="ZedHeap" DetailLevel="Verbose" LoggingMode="Memory" Description="Heap tracing">
      {heap_collector}
    </Profile>
  </Profiles>

  <TraceMergeProperties>
    <TraceMergeProperty Id="TraceMerge_Default" Name="TraceMerge_Default">
      <FileCompression Value="true"/>
    </TraceMergeProperty>
  </TraceMergeProperties>
</WindowsPerformanceRecorder>"#
    )
}

fn wpr_error_context(hresult: windows_core::HRESULT, source: &windows_core::IUnknown) -> String {
    let mut out = format!("HRESULT: {hresult}");

    unsafe {
        let mut message = BSTR::new();
        let mut description = BSTR::new();
        let mut detail = BSTR::new();
        if WPRCFormatError(
            hresult,
            Some(source),
            &mut message,
            Some(&mut description),
            Some(&mut detail),
        )
        .is_ok()
        {
            for (label, value) in [
                ("Message", &message),
                ("Description", &description),
                ("Detail", &detail),
            ] {
                if !value.is_empty() {
                    let _ = write!(out, "\n  {label}: {value}");
                }
            }
        }
    }

    if let Ok(info) = source.cast::<IParsingErrorInfo>() {
        unsafe {
            if let Ok(line) = info.GetLineNumber() {
                let _ = write!(out, "\n  Parse error at line: {line}");
                if let Ok(col) = info.GetColumnNumber() {
                    let _ = write!(out, ", column: {col}");
                }
            }
            for (label, getter) in [
                ("Element type", info.GetElementType()),
                ("Element ID", info.GetElementId()),
                ("Description", info.GetDescription()),
            ] {
                if let Ok(value) = getter
                    && !value.is_empty()
                {
                    let _ = write!(out, "\n  {label}: {value}");
                }
            }
        }
    }

    fn append_control_chain(out: &mut String, source: &windows_core::IUnknown) {
        let Ok(info) = source.cast::<IControlErrorInfo>() else {
            return;
        };
        unsafe {
            if let Ok(object_type) = info.GetObjectType() {
                let name = match object_type {
                    wprcontrol::ObjectType_Profile => "Profile",
                    wprcontrol::ObjectType_Collector => "Collector",
                    wprcontrol::ObjectType_Provider => "Provider",
                    _ => "Unknown",
                };
                let _ = write!(out, "\n  Object type: {name}");
            }
            if let Ok(hr) = info.GetHResult() {
                let _ = write!(out, "\n  Inner HRESULT: {hr}");
            }
            if let Ok(desc) = info.GetDescription()
                && !desc.is_empty()
            {
                let _ = write!(out, "\n  Description: {desc}");
            }
            let mut inner = None;
            if info.GetInnerErrorInfo(&mut inner).is_ok()
                && let Some(inner) = inner
            {
                let _ = write!(out, "\n  Caused by:");
                append_control_chain(out, &inner);
            }
        }
    }
    append_control_chain(&mut out, source);

    if let Ok(info) = source.cast::<windows::Win32::System::Com::IErrorInfo>() {
        unsafe {
            if let Ok(desc) = info.GetDescription()
                && !desc.is_empty()
            {
                let _ = write!(out, "\n  IErrorInfo: {desc}");
            }
        }
    }

    out
}

trait WprContext<T> {
    fn wpr_context(self, source: &impl Interface) -> Result<T>;
}

impl<T> WprContext<T> for windows_core::Result<T> {
    fn wpr_context(self, source: &impl Interface) -> Result<T> {
        self.map_err(|e| {
            let unknown: windows_core::IUnknown = source.cast().expect("cast to IUnknown");
            let context = wpr_error_context(e.code(), &unknown);
            anyhow::anyhow!("{context}")
        })
    }
}

fn create_wpr<T: windows_core::Interface>(clsid: &windows_core::GUID) -> Result<T> {
    unsafe {
        WPRCCreateInstanceUnderInstanceName::<_, T>(
            &BSTR::from(INSTANCE_NAME),
            clsid,
            None,
            CLSCTX_INPROC_SERVER.0,
        )
        .context("WPRCCreateInstance failed")
    }
}

fn build_profile_collection(heap_pid: Option<u32>) -> Result<IProfileCollection> {
    let collection: IProfileCollection = create_wpr(&CProfileCollection)?;

    for profile_name in BUILTIN_PROFILES {
        let profile: IProfile = create_wpr(&CProfile)?;
        unsafe {
            profile
                .LoadFromFile(&BSTR::from(*profile_name), &BSTR::new())
                .wpr_context(&profile)
                .with_context(|| format!("Load built-in profile '{profile_name}'"))?;
            collection
                .Add(&profile, VARIANT_FALSE)
                .wpr_context(&collection)
                .with_context(|| format!("Add profile '{profile_name}' to collection"))?;
        }
    }

    let heap_xml = heap_tracing_profile(heap_pid);
    let heap_profile: IProfile = create_wpr(&CProfile)?;
    unsafe {
        heap_profile
            .LoadFromString(&BSTR::from(heap_xml))
            .wpr_context(&heap_profile)
            .context("Load profile from XML string")?;
        collection
            .Add(&heap_profile, VARIANT_BOOL(0))
            .wpr_context(&collection)
            .context("Add ZedHeap profile to collection")?;
    }

    Ok(collection)
}

pub fn record_etw_trace(
    heap_pid: Option<u32>,
    output_path: &Path,
    socket_path: &str,
) -> Result<()> {
    unsafe {
        CoInitializeEx(None, COINIT_MULTITHREADED)
            .ok()
            .context("COM initialization failed")?;
    }

    let socket_path = Path::new(socket_path);
    let mut stream = net::UnixStream::connect(socket_path).context("Connect to parent socket")?;

    match record_etw_trace_inner(heap_pid, output_path, &mut stream) {
        Ok(()) => Ok(()),
        Err(e) => {
            send_json(
                &mut stream,
                &StatusMessage::Error {
                    message: format!("{e:#}"),
                },
            )
            .log_err();
            Err(e)
        }
    }
}

fn record_etw_trace_inner(
    heap_pid: Option<u32>,
    output_path: &Path,
    stream: &mut net::UnixStream,
) -> Result<()> {
    let collection = build_profile_collection(heap_pid)?;
    let control_manager: IControlManager = create_wpr(&CControlManager)?;

    // Cancel any leftover sessions with the same name that might exist
    unsafe {
        _ = control_manager.Cancel(None);
    }

    unsafe {
        control_manager
            .Start(&collection)
            .wpr_context(&control_manager)
            .context("Start WPR recording")?;
    }

    // We must call Save or Cancel before returning or we'll leak the kernel buffers used to record the ETW session.
    let cancel_guard = defer({
        let control_manager = control_manager.clone();
        move || unsafe {
            let _ = control_manager.Cancel(None);
        }
    });

    send_json(stream, &StatusMessage::Started)?;

    let (command, timed_out) = receive_command(stream)?;

    match command {
        Command::Cancel => {
            unsafe {
                control_manager
                    .Cancel(None)
                    .wpr_context(&control_manager)
                    .context("Cancel WPR recording")?;
            }
            cancel_guard.abort();

            send_json(stream, &StatusMessage::Cancelled).log_err();
        }
        Command::Save => {
            unsafe {
                control_manager
                    .Save(
                        &BSTR::from(output_path.to_string_lossy().as_ref()),
                        &collection,
                        None,
                    )
                    .wpr_context(&control_manager)
                    .context("Stop WPR recording")?;
            }
            cancel_guard.abort();

            if timed_out {
                send_json(stream, &StatusMessage::TimedOut).log_err();
            } else {
                send_json(stream, &StatusMessage::Stopped).log_err();
            }
        }
    }

    Ok(())
}

fn receive_command(stream: &mut net::UnixStream) -> Result<(Command, bool)> {
    use std::os::windows::io::{AsRawSocket, AsSocket};
    use windows::Win32::Networking::WinSock::{SO_RCVTIMEO, SOL_SOCKET, setsockopt};

    // Set a receive timeout so read_line returns an error after `timeout`.
    let millis = RECORDING_TIMEOUT.as_millis() as u32;
    let socket = stream.as_socket();
    let ret = unsafe {
        setsockopt(
            windows::Win32::Networking::WinSock::SOCKET(socket.as_raw_socket() as _),
            SOL_SOCKET,
            SO_RCVTIMEO,
            Some(&millis.to_ne_bytes()),
        )
    };
    if ret != 0 {
        bail!("Failed to set socket receive timeout: setsockopt returned {ret}");
    }

    let mut reader = BufReader::new(&mut *stream);
    match recv_json::<Command>(&mut reader) {
        Ok(command) => Ok((command, false)),
        Err(error) => {
            log::warn!("Failed to receive ETW command, treating as timed-out Save: {error:#}");
            Ok((Command::Save, true))
        }
    }
}

pub struct EtwSession {
    output_path: PathBuf,
    stream: BufReader<net::UnixStream>,
    listener: net::UnixListener,
    socket_path: PathBuf,
}

pub fn launch_etw_recording(heap_pid: Option<u32>, output_path: &Path) -> Result<EtwSession> {
    let sock_path = std::env::temp_dir().join(format!("zed-etw-{}.sock", std::process::id()));

    _ = std::fs::remove_file(&sock_path);
    let listener = net::UnixListener::bind(&sock_path).context("Bind Unix socket for ETW IPC")?;

    let exe_path = std::env::current_exe().context("Failed to get current exe path")?;
    let pid_arg = heap_pid.map_or(-1i64, |pid| pid as i64);
    let args = format!(
        "--record-etw-trace --etw-zed-pid {} --etw-output \"{}\" --etw-socket \"{}\"",
        pid_arg,
        output_path.display(),
        sock_path.display(),
    );

    use windows::Win32::UI::Shell::ShellExecuteW;
    use windows_core::PCWSTR;

    let operation: Vec<u16> = "runas\0".encode_utf16().collect();
    let file: Vec<u16> = format!("{}\0", exe_path.to_string_lossy())
        .encode_utf16()
        .collect();
    let parameters: Vec<u16> = format!("{args}\0").encode_utf16().collect();

    let result = unsafe {
        ShellExecuteW(
            None,
            PCWSTR(operation.as_ptr()),
            PCWSTR(file.as_ptr()),
            PCWSTR(parameters.as_ptr()),
            PCWSTR::null(),
            windows::Win32::UI::WindowsAndMessaging::SW_HIDE,
        )
    };

    let result_code = result.0 as usize;
    if result_code <= 32 {
        bail!("ShellExecuteW failed to launch elevated process (code: {result_code})");
    }

    let (stream, _) = listener.accept().context("Accept subprocess connection")?;

    let mut session = EtwSession {
        output_path: output_path.to_path_buf(),
        stream: BufReader::new(stream),
        listener,
        socket_path: sock_path,
    };

    let status: StatusMessage =
        recv_json(&mut session.stream).context("Wait for Started status")?;

    match status {
        StatusMessage::Started => {}
        StatusMessage::Error { message } => {
            bail!("Subprocess reported error during start: {message}");
        }
        other => {
            bail!("Unexpected status from subprocess: {other:?}");
        }
    }

    Ok(session)
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(tag = "type")]
pub enum StatusMessage {
    Started,
    Stopped,
    TimedOut,
    Cancelled,
    Error { message: String },
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(tag = "type")]
pub enum Command {
    Save,
    Cancel,
}

fn send_json<T: serde::Serialize>(writer: &mut impl Write, value: &T) -> Result<()> {
    let json = serde_json::to_string(value).context("Serialize message")?;
    writeln!(writer, "{json}").context("Write to socket")?;
    writer.flush().context("Flush socket")?;
    Ok(())
}

fn recv_json<T: serde::de::DeserializeOwned>(reader: &mut impl BufRead) -> Result<T> {
    let mut line = String::new();
    reader.read_line(&mut line).context("Read from socket")?;
    if line.is_empty() {
        bail!("Socket closed before a message was received");
    }
    serde_json::from_str(line.trim()).context("Parse message")
}
