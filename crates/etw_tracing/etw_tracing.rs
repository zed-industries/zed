#![cfg(target_os = "windows")]

use anyhow::{Context as _, Result, bail};
use gpui::{App, AppContext as _, DismissEvent, Global, actions};
use std::fmt::Write as _;
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};
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

enum EtwSessionState {
    Recording,
    ChoosingOutputPath,
    Stopping,
}

struct EtwSessionHandle {
    writer: net::OwnedWriteHalf,
    _listener: net::UnixListener,
    socket_path: PathBuf,
    state: EtwSessionState,
}

impl Drop for EtwSessionHandle {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.socket_path);
    }
}

struct GlobalEtwSession(Option<EtwSessionHandle>);

impl Global for GlobalEtwSession {}

fn show_etw_notification(cx: &mut App, message: impl Into<gpui::SharedString>) {
    let message = message.into();
    show_app_notification(NotificationId::unique::<EtwNotification>(), cx, move |cx| {
        cx.new(|cx| MessageNotification::new(message.clone(), cx))
    });
}

fn show_etw_status_notification(cx: &mut App, status: Result<StatusMessage>) {
    match status {
        Ok(StatusMessage::Stopped { output_path }) => {
            let message = format!("ETW trace saved to {}", output_path.display());
            show_app_notification(NotificationId::unique::<EtwNotification>(), cx, move |cx| {
                let message = message.clone();
                let output_path = output_path.clone();
                cx.new(|cx| {
                    MessageNotification::new(message, cx)
                        .primary_message("Show in File Manager")
                        .primary_on_click(move |_window, cx| {
                            cx.reveal_path(&output_path);
                            cx.emit(DismissEvent);
                        })
                })
            });
        }
        Ok(StatusMessage::Cancelled) => {
            show_etw_notification(cx, "ETW recording cancelled");
        }
        Ok(StatusMessage::Error { message }) => {
            show_etw_notification(cx, format!("ETW recording failed: {message}"));
        }
        Ok(StatusMessage::Started) => {
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
        prompt_for_etw_output_path(cx);
    });

    cx.on_action(|_: &CancelEtwTrace, cx: &mut App| {
        cancel_etw_recording(cx);
    });
}

fn prompt_for_etw_output_path(cx: &mut App) {
    let Some(session) = cx.global_mut::<GlobalEtwSession>().0.as_mut() else {
        show_etw_notification(cx, "No active ETW recording to stop");
        return;
    };
    match &session.state {
        EtwSessionState::Recording => {
            session.state = EtwSessionState::ChoosingOutputPath;
        }
        EtwSessionState::ChoosingOutputPath => {
            show_etw_notification(cx, "ETW recording is already waiting for a save location");
            return;
        }
        EtwSessionState::Stopping => {
            show_etw_notification(cx, "ETW recording is already stopping");
            return;
        }
    }

    let save_dialog = cx.prompt_for_new_path(&PathBuf::default(), Some("zed-trace.etl"));
    cx.spawn(async move |cx| {
        let picked = save_dialog.await.unwrap_or(Ok(None));
        cx.update(|cx| match picked {
            Ok(Some(output_path)) => save_etw_recording(output_path, cx),
            Ok(None) => resume_etw_recording(cx),
            Err(error) => {
                resume_etw_recording(cx);
                show_etw_notification(cx, format!("Failed to pick save location: {error:#}"));
            }
        });
    })
    .detach();
}

fn save_etw_recording(output_path: PathBuf, cx: &mut App) {
    let Some(session) = cx.global_mut::<GlobalEtwSession>().0.as_mut() else {
        return;
    };
    if !matches!(&session.state, EtwSessionState::ChoosingOutputPath) {
        return;
    }

    let command = Command::Save {
        output_path: output_path.clone(),
    };
    match send_json(&mut session.writer, &command) {
        Ok(()) => {
            session.state = EtwSessionState::Stopping;
            show_etw_notification(cx, "Stopping ETW recording...");
        }
        Err(error) => {
            session.state = EtwSessionState::Recording;
            show_etw_notification(cx, format!("Failed to stop ETW recording: {error:#}"));
        }
    }
}

fn resume_etw_recording(cx: &mut App) {
    let Some(session) = cx.global_mut::<GlobalEtwSession>().0.as_mut() else {
        return;
    };
    if matches!(&session.state, EtwSessionState::ChoosingOutputPath) {
        session.state = EtwSessionState::Recording;
    }
}

fn cancel_etw_recording(cx: &mut App) {
    let Some(session) = cx.global_mut::<GlobalEtwSession>().0.as_mut() else {
        show_etw_notification(cx, "No active ETW recording to cancel");
        return;
    };
    if matches!(&session.state, EtwSessionState::Stopping) {
        show_etw_notification(cx, "ETW recording is already stopping");
        return;
    }

    match send_json(&mut session.writer, &Command::Cancel) {
        Ok(()) => {
            session.state = EtwSessionState::Stopping;
            show_etw_notification(cx, "Cancelling ETW recording...");
        }
        Err(error) => {
            session.state = EtwSessionState::Recording;
            show_etw_notification(cx, format!("Failed to cancel ETW recording: {error:#}"));
        }
    }
}

fn start_etw_recording(cx: &mut App, heap_pid: Option<u32>) {
    if cx.global::<GlobalEtwSession>().0.is_some() {
        show_etw_notification(cx, "ETW recording is already in progress");
        return;
    }
    cx.spawn(async move |cx| {
        let result = cx
            .background_spawn(async move { launch_etw_recording(heap_pid) })
            .await;

        let EtwSession { mut reader, handle } = match result {
            Ok(session) => session,
            Err(error) => {
                cx.update(|cx| {
                    show_etw_notification(cx, format!("Failed to start ETW recording: {error:#}"));
                });
                return;
            }
        };

        cx.update(|cx| {
            cx.global_mut::<GlobalEtwSession>().0 = Some(handle);
            show_etw_notification(cx, "ETW recording started");
        });

        let status = cx
            .background_spawn(async move {
                recv_json(&mut reader).context("Receive status from subprocess")
            })
            .await;
        cx.update(|cx| {
            cx.global_mut::<GlobalEtwSession>().0 = None;
            show_etw_status_notification(cx, status);
        });
    })
    .detach();
}

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

pub fn record_etw_trace(heap_pid: Option<u32>, socket_path: &Path) -> Result<()> {
    unsafe {
        CoInitializeEx(None, COINIT_MULTITHREADED)
            .ok()
            .context("COM initialization failed")?;
    }

    let mut stream = net::UnixStream::connect(socket_path).context("Connect to parent socket")?;

    match record_etw_trace_inner(heap_pid, &mut stream) {
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

fn record_etw_trace_inner(heap_pid: Option<u32>, stream: &mut net::UnixStream) -> Result<()> {
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

    let command: Command =
        recv_json(&mut BufReader::new(&mut *stream)).context("Receive command from Zed")?;

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
        Command::Save { output_path } => {
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

            send_json(stream, &StatusMessage::Stopped { output_path }).log_err();
        }
    }

    Ok(())
}

struct EtwSession {
    reader: BufReader<net::OwnedReadHalf>,
    handle: EtwSessionHandle,
}

fn launch_etw_recording(heap_pid: Option<u32>) -> Result<EtwSession> {
    let sock_path = std::env::temp_dir().join(format!("zed-etw-{}.sock", std::process::id()));

    _ = std::fs::remove_file(&sock_path);
    let listener = net::UnixListener::bind(&sock_path).context("Bind Unix socket for ETW IPC")?;

    let exe_path = std::env::current_exe().context("Failed to get current exe path")?;
    let heap_arg = heap_pid.map_or(String::new(), |pid| format!(" --etw-zed-pid {pid}"));
    let args = format!(
        "--record-etw-trace{heap_arg} --etw-socket \"{}\"",
        sock_path.display(),
    );

    use windows::Win32::UI::Shell::ShellExecuteW;
    use windows_core::{HSTRING, PCWSTR};

    let operation = HSTRING::from("runas");
    let file = HSTRING::from(exe_path.to_string_lossy().as_ref());
    let parameters = HSTRING::from(args);

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
    let (read_half, write_half) = stream.into_split();
    let mut reader = BufReader::new(read_half);

    match recv_json(&mut reader).context("Wait for Started status")? {
        StatusMessage::Started => {}
        StatusMessage::Error { message } => {
            bail!("Subprocess reported error during start: {message}");
        }
        other => {
            bail!("Unexpected status from subprocess: {other:?}");
        }
    }

    Ok(EtwSession {
        reader,
        handle: EtwSessionHandle {
            writer: write_half,
            _listener: listener,
            socket_path: sock_path,
            state: EtwSessionState::Recording,
        },
    })
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(tag = "type")]
enum StatusMessage {
    Started,
    Stopped { output_path: PathBuf },
    Cancelled,
    Error { message: String },
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(tag = "type")]
enum Command {
    Save { output_path: PathBuf },
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
