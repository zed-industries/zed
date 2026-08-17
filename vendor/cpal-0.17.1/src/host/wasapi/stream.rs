use super::windows_err_to_cpal_err;
use crate::traits::StreamTrait;
use crate::{
    BackendSpecificError, BufferSize, Data, InputCallbackInfo, OutputCallbackInfo,
    PauseStreamError, PlayStreamError, SampleFormat, StreamError,
};
use std::mem;
use std::ptr;
use std::sync::mpsc::{channel, Receiver, SendError, Sender};
use std::thread::{self, JoinHandle};
use windows::Win32::Foundation;
use windows::Win32::Foundation::WAIT_OBJECT_0;
use windows::Win32::Media::Audio;
use windows::Win32::System::SystemServices;
use windows::Win32::System::Threading;

pub struct Stream {
    /// The high-priority audio processing thread calling callbacks.
    /// Option used for moving out in destructor.
    ///
    /// TODO: Actually set the thread priority.
    thread: Option<JoinHandle<()>>,

    // Commands processed by the `run()` method that is currently running.
    // `pending_scheduled_event` must be signalled whenever a command is added here, so that it
    // will get picked up.
    commands: Sender<Command>,

    // This event is signalled after a new entry is added to `commands`, so that the `run()`
    // method can be notified.
    pending_scheduled_event: Foundation::HANDLE,
}

// SAFETY: Windows Event HANDLEs are safe to send between threads - they are designed for
// synchronization. All fields of Stream are Send:
// - JoinHandle<()> is Send
// - Sender<Command> is Send
// - Foundation::HANDLE is Send (Windows synchronization primitive)
// See: https://learn.microsoft.com/en-us/windows/win32/api/synchapi/nf-synchapi-createeventa
unsafe impl Send for Stream {}

// SAFETY: Windows Event HANDLEs are safe to access from multiple threads simultaneously.
// All synchronization operations (SetEvent, WaitForSingleObject) are thread-safe.
// All fields of Stream are Sync:
// - JoinHandle<()> is Sync
// - Sender<Command> is Sync (uses internal synchronization)
// - Foundation::HANDLE for event objects supports concurrent access
// The audio thread owns all COM objects, so no cross-thread COM access occurs.
unsafe impl Sync for Stream {}

// Compile-time assertion that Stream is Send and Sync
crate::assert_stream_send!(Stream);
crate::assert_stream_sync!(Stream);

struct RunContext {
    // Streams that have been created in this event loop.
    stream: StreamInner,

    // Handles corresponding to the `event` field of each element of `voices`. Must always be in
    // sync with `voices`, except that the first element is always `pending_scheduled_event`.
    handles: Vec<Foundation::HANDLE>,

    commands: Receiver<Command>,
}

// Once we start running the eventloop, the RunContext will not be moved.
unsafe impl Send for RunContext {}

pub enum Command {
    PlayStream,
    PauseStream,
    Terminate,
}

pub enum AudioClientFlow {
    Render {
        render_client: Audio::IAudioRenderClient,
    },
    Capture {
        capture_client: Audio::IAudioCaptureClient,
    },
}

pub struct StreamInner {
    pub audio_client: Audio::IAudioClient,
    pub audio_clock: Audio::IAudioClock,
    pub client_flow: AudioClientFlow,
    // Event that is signalled by WASAPI whenever audio data must be written.
    pub event: Foundation::HANDLE,
    // True if the stream is currently playing. False if paused.
    pub playing: bool,
    // Number of frames of audio data in the underlying buffer allocated by WASAPI.
    pub max_frames_in_buffer: u32,
    // Number of bytes that each frame occupies.
    pub bytes_per_frame: u16,
    // The configuration with which the stream was created.
    pub config: crate::StreamConfig,
    // The sample format with which the stream was created.
    pub sample_format: SampleFormat,
}

impl Stream {
    pub(crate) fn new_input<D, E>(
        stream_inner: StreamInner,
        mut data_callback: D,
        mut error_callback: E,
    ) -> Stream
    where
        D: FnMut(&Data, &InputCallbackInfo) + Send + 'static,
        E: FnMut(StreamError) + Send + 'static,
    {
        let pending_scheduled_event = unsafe {
            Threading::CreateEventA(None, false, false, windows::core::PCSTR(ptr::null()))
        }
        .expect("cpal: could not create input stream event");
        let (tx, rx) = channel();

        let run_context = RunContext {
            handles: vec![pending_scheduled_event, stream_inner.event],
            stream: stream_inner,
            commands: rx,
        };

        let thread = thread::Builder::new()
            .name("cpal_wasapi_in".to_owned())
            .spawn(move || run_input(run_context, &mut data_callback, &mut error_callback))
            .unwrap();

        Stream {
            thread: Some(thread),
            commands: tx,
            pending_scheduled_event,
        }
    }

    pub(crate) fn new_output<D, E>(
        stream_inner: StreamInner,
        mut data_callback: D,
        mut error_callback: E,
    ) -> Stream
    where
        D: FnMut(&mut Data, &OutputCallbackInfo) + Send + 'static,
        E: FnMut(StreamError) + Send + 'static,
    {
        let pending_scheduled_event = unsafe {
            Threading::CreateEventA(None, false, false, windows::core::PCSTR(ptr::null()))
        }
        .expect("cpal: could not create output stream event");
        let (tx, rx) = channel();

        let run_context = RunContext {
            handles: vec![pending_scheduled_event, stream_inner.event],
            stream: stream_inner,
            commands: rx,
        };

        let thread = thread::Builder::new()
            .name("cpal_wasapi_out".to_owned())
            .spawn(move || run_output(run_context, &mut data_callback, &mut error_callback))
            .unwrap();

        Stream {
            thread: Some(thread),
            commands: tx,
            pending_scheduled_event,
        }
    }

    fn push_command(&self, command: Command) -> Result<(), SendError<Command>> {
        self.commands.send(command)?;
        unsafe {
            Threading::SetEvent(self.pending_scheduled_event).unwrap();
        }
        Ok(())
    }
}

impl Drop for Stream {
    fn drop(&mut self) {
        if self.push_command(Command::Terminate).is_ok() {
            self.thread.take().unwrap().join().unwrap();
            unsafe {
                let _ = Foundation::CloseHandle(self.pending_scheduled_event);
            }
        }
    }
}

impl StreamTrait for Stream {
    fn play(&self) -> Result<(), PlayStreamError> {
        self.push_command(Command::PlayStream)
            .map_err(|_| crate::error::PlayStreamError::DeviceNotAvailable)?;
        Ok(())
    }

    fn pause(&self) -> Result<(), PauseStreamError> {
        self.push_command(Command::PauseStream)
            .map_err(|_| crate::error::PauseStreamError::DeviceNotAvailable)?;
        Ok(())
    }
}

impl Drop for StreamInner {
    fn drop(&mut self) {
        unsafe {
            let _ = Foundation::CloseHandle(self.event);
        }
    }
}

// Process any pending commands that are queued within the `RunContext`.
// Returns `true` if the loop should continue running, `false` if it should terminate.
fn process_commands(run_context: &mut RunContext) -> Result<bool, StreamError> {
    // Process the pending commands.
    for command in run_context.commands.try_iter() {
        match command {
            Command::PlayStream => unsafe {
                if !run_context.stream.playing {
                    run_context
                        .stream
                        .audio_client
                        .Start()
                        .map_err(windows_err_to_cpal_err::<StreamError>)?;
                    run_context.stream.playing = true;
                }
            },
            Command::PauseStream => unsafe {
                if run_context.stream.playing {
                    run_context
                        .stream
                        .audio_client
                        .Stop()
                        .map_err(windows_err_to_cpal_err::<StreamError>)?;
                    run_context.stream.playing = false;
                }
            },
            Command::Terminate => {
                return Ok(false);
            }
        }
    }

    Ok(true)
}
// Wait for any of the given handles to be signalled.
//
// Returns the index of the `handle` that was signalled, or an `Err` if
// `WaitForMultipleObjectsEx` fails.
//
// This is called when the `run` thread is ready to wait for the next event. The
// next event might be some command submitted by the user (the first handle) or
// might indicate that one of the streams is ready to deliver or receive audio.
fn wait_for_handle_signal(handles: &[Foundation::HANDLE]) -> Result<usize, BackendSpecificError> {
    debug_assert!(handles.len() <= SystemServices::MAXIMUM_WAIT_OBJECTS as usize);
    let result = unsafe {
        Threading::WaitForMultipleObjectsEx(
            handles,
            false,               // Don't wait for all, just wait for the first
            Threading::INFINITE, // TODO: allow setting a timeout
            false,               // irrelevant parameter here
        )
    };
    if result == Foundation::WAIT_FAILED {
        let err = unsafe { Foundation::GetLastError() };
        let description = format!("`WaitForMultipleObjectsEx failed: {:?}", err);
        let err = BackendSpecificError { description };
        return Err(err);
    }
    // Notifying the corresponding task handler.
    let handle_idx = (result.0 - WAIT_OBJECT_0.0) as usize;
    Ok(handle_idx)
}

// Get the number of available frames that are available for writing/reading.
fn get_available_frames(stream: &StreamInner) -> Result<u32, StreamError> {
    unsafe {
        let padding = stream
            .audio_client
            .GetCurrentPadding()
            .map_err(windows_err_to_cpal_err::<StreamError>)?;
        Ok(stream.max_frames_in_buffer - padding)
    }
}

fn run_input(
    mut run_ctxt: RunContext,
    data_callback: &mut dyn FnMut(&Data, &InputCallbackInfo),
    error_callback: &mut dyn FnMut(StreamError),
) {
    boost_current_thread_priority(
        run_ctxt.stream.config.buffer_size,
        run_ctxt.stream.config.sample_rate,
    );

    loop {
        match process_commands_and_await_signal(&mut run_ctxt, error_callback) {
            Some(ControlFlow::Break) => break,
            Some(ControlFlow::Continue) => continue,
            None => (),
        }
        let capture_client = match run_ctxt.stream.client_flow {
            AudioClientFlow::Capture { ref capture_client } => capture_client.clone(),
            _ => unreachable!(),
        };
        match process_input(
            &run_ctxt.stream,
            capture_client,
            data_callback,
            error_callback,
        ) {
            ControlFlow::Break => break,
            ControlFlow::Continue => continue,
        }
    }
}

fn run_output(
    mut run_ctxt: RunContext,
    data_callback: &mut dyn FnMut(&mut Data, &OutputCallbackInfo),
    error_callback: &mut dyn FnMut(StreamError),
) {
    boost_current_thread_priority(
        run_ctxt.stream.config.buffer_size,
        run_ctxt.stream.config.sample_rate,
    );

    loop {
        match process_commands_and_await_signal(&mut run_ctxt, error_callback) {
            Some(ControlFlow::Break) => break,
            Some(ControlFlow::Continue) => continue,
            None => (),
        }
        let render_client = match run_ctxt.stream.client_flow {
            AudioClientFlow::Render { ref render_client } => render_client.clone(),
            _ => unreachable!(),
        };
        match process_output(
            &run_ctxt.stream,
            render_client,
            data_callback,
            error_callback,
        ) {
            ControlFlow::Break => break,
            ControlFlow::Continue => continue,
        }
    }
}

#[cfg(feature = "audio_thread_priority")]
fn boost_current_thread_priority(buffer_size: BufferSize, sample_rate: crate::SampleRate) {
    use audio_thread_priority::promote_current_thread_to_real_time;

    let buffer_size = if let BufferSize::Fixed(buffer_size) = buffer_size {
        buffer_size
    } else {
        // if the buffer size isn't fixed, let audio_thread_priority choose a sensible default value
        0
    };

    if let Err(err) = promote_current_thread_to_real_time(buffer_size, sample_rate) {
        eprintln!("Failed to promote audio thread to real-time priority: {err}");
    }
}

#[cfg(not(feature = "audio_thread_priority"))]
fn boost_current_thread_priority(_: BufferSize, _: crate::SampleRate) {
    unsafe {
        let thread_handle = Threading::GetCurrentThread();

        let _ =
            Threading::SetThreadPriority(thread_handle, Threading::THREAD_PRIORITY_TIME_CRITICAL);
    }
}

enum ControlFlow {
    Break,
    Continue,
}

fn process_commands_and_await_signal(
    run_context: &mut RunContext,
    error_callback: &mut dyn FnMut(StreamError),
) -> Option<ControlFlow> {
    // Process queued commands.
    match process_commands(run_context) {
        Ok(true) => (),
        Ok(false) => return Some(ControlFlow::Break),
        Err(err) => {
            error_callback(err);
            return Some(ControlFlow::Break);
        }
    };

    // Wait for any of the handles to be signalled.
    let handle_idx = match wait_for_handle_signal(&run_context.handles) {
        Ok(idx) => idx,
        Err(err) => {
            error_callback(err.into());
            return Some(ControlFlow::Break);
        }
    };

    // If `handle_idx` is 0, then it's `pending_scheduled_event` that was signalled in
    // order for us to pick up the pending commands. Otherwise, a stream needs data.
    if handle_idx == 0 {
        return Some(ControlFlow::Continue);
    }

    None
}

// The loop for processing pending input data.
fn process_input(
    stream: &StreamInner,
    capture_client: Audio::IAudioCaptureClient,
    data_callback: &mut dyn FnMut(&Data, &InputCallbackInfo),
    error_callback: &mut dyn FnMut(StreamError),
) -> ControlFlow {
    unsafe {
        // Get the available data in the shared buffer.
        let mut buffer: *mut u8 = ptr::null_mut();
        let mut flags = mem::MaybeUninit::uninit();
        loop {
            let mut frames_available = match capture_client.GetNextPacketSize() {
                Ok(0) => return ControlFlow::Continue,
                Ok(f) => f,
                Err(err) => {
                    error_callback(windows_err_to_cpal_err(err));
                    return ControlFlow::Break;
                }
            };
            let mut qpc_position: u64 = 0;
            let result = capture_client.GetBuffer(
                &mut buffer,
                &mut frames_available,
                flags.as_mut_ptr(),
                None,
                Some(&mut qpc_position),
            );

            match result {
                // TODO: Can this happen?
                Err(e) if e.code() == Audio::AUDCLNT_S_BUFFER_EMPTY => continue,
                Err(e) => {
                    error_callback(windows_err_to_cpal_err(e));
                    return ControlFlow::Break;
                }
                Ok(_) => (),
            }

            debug_assert!(!buffer.is_null());

            let data = buffer as *mut ();
            let len = frames_available as usize * stream.bytes_per_frame as usize
                / stream.sample_format.sample_size();
            let data = Data::from_parts(data, len, stream.sample_format);

            // The `qpc_position` is in 100 nanosecond units. Convert it to nanoseconds.
            let timestamp = match input_timestamp(stream, qpc_position) {
                Ok(ts) => ts,
                Err(err) => {
                    error_callback(err);
                    return ControlFlow::Break;
                }
            };
            let info = InputCallbackInfo { timestamp };
            data_callback(&data, &info);

            // Release the buffer.
            let result = capture_client
                .ReleaseBuffer(frames_available)
                .map_err(windows_err_to_cpal_err);
            if let Err(err) = result {
                error_callback(err);
                return ControlFlow::Break;
            }
        }
    }
}

// The loop for writing output data.
fn process_output(
    stream: &StreamInner,
    render_client: Audio::IAudioRenderClient,
    data_callback: &mut dyn FnMut(&mut Data, &OutputCallbackInfo),
    error_callback: &mut dyn FnMut(StreamError),
) -> ControlFlow {
    // The number of frames available for writing.
    let frames_available = match get_available_frames(stream) {
        Ok(0) => return ControlFlow::Continue, // TODO: Can this happen?
        Ok(n) => n,
        Err(err) => {
            error_callback(err);
            return ControlFlow::Break;
        }
    };

    unsafe {
        let buffer = match render_client.GetBuffer(frames_available) {
            Ok(b) => b,
            Err(e) => {
                error_callback(windows_err_to_cpal_err(e));
                return ControlFlow::Break;
            }
        };

        debug_assert!(!buffer.is_null());

        let data = buffer as *mut ();
        let len = frames_available as usize * stream.bytes_per_frame as usize
            / stream.sample_format.sample_size();
        let mut data = Data::from_parts(data, len, stream.sample_format);
        let sample_rate = stream.config.sample_rate;
        let timestamp = match output_timestamp(stream, frames_available, sample_rate) {
            Ok(ts) => ts,
            Err(err) => {
                error_callback(err);
                return ControlFlow::Break;
            }
        };
        let info = OutputCallbackInfo { timestamp };
        data_callback(&mut data, &info);

        if let Err(err) = render_client.ReleaseBuffer(frames_available, 0) {
            error_callback(windows_err_to_cpal_err(err));
            return ControlFlow::Break;
        }
    }

    ControlFlow::Continue
}

/// Convert the given duration in frames at the given sample rate to a `std::time::Duration`.
fn frames_to_duration(frames: u32, rate: crate::SampleRate) -> std::time::Duration {
    let secsf = frames as f64 / rate as f64;
    let secs = secsf as u64;
    let nanos = ((secsf - secs as f64) * 1_000_000_000.0) as u32;
    std::time::Duration::new(secs, nanos)
}

/// Use the stream's `IAudioClock` to produce the current stream instant.
///
/// Uses the QPC position produced via the `GetPosition` method.
fn stream_instant(stream: &StreamInner) -> Result<crate::StreamInstant, StreamError> {
    let mut position: u64 = 0;
    let mut qpc_position: u64 = 0;
    unsafe {
        stream
            .audio_clock
            .GetPosition(&mut position, Some(&mut qpc_position))
            .map_err(windows_err_to_cpal_err::<StreamError>)?;
    };
    // The `qpc_position` is in 100 nanosecond units. Convert it to nanoseconds.
    let qpc_nanos = qpc_position as i128 * 100;
    let instant = crate::StreamInstant::from_nanos_i128(qpc_nanos)
        .expect("performance counter out of range of `StreamInstant` representation");
    Ok(instant)
}

/// Produce the input stream timestamp.
///
/// `buffer_qpc_position` is the `qpc_position` returned via the `GetBuffer` call on the capture
/// client. It represents the instant at which the first sample of the retrieved buffer was
/// captured.
fn input_timestamp(
    stream: &StreamInner,
    buffer_qpc_position: u64,
) -> Result<crate::InputStreamTimestamp, StreamError> {
    // The `qpc_position` is in 100 nanosecond units. Convert it to nanoseconds.
    let qpc_nanos = buffer_qpc_position as i128 * 100;
    let capture = crate::StreamInstant::from_nanos_i128(qpc_nanos)
        .expect("performance counter out of range of `StreamInstant` representation");
    let callback = stream_instant(stream)?;
    Ok(crate::InputStreamTimestamp { capture, callback })
}

/// Produce the output stream timestamp.
///
/// `frames_available` is the number of frames available for writing as reported by subtracting the
/// result of `GetCurrentPadding` from the maximum buffer size.
///
/// `sample_rate` is the rate at which audio frames are processed by the device.
///
/// TODO: The returned `playback` is an estimate that assumes audio is delivered immediately after
/// `frames_available` are consumed. The reality is that there is likely a tiny amount of latency
/// after this, but not sure how to determine this.
fn output_timestamp(
    stream: &StreamInner,
    frames_available: u32,
    sample_rate: crate::SampleRate,
) -> Result<crate::OutputStreamTimestamp, StreamError> {
    let callback = stream_instant(stream)?;
    let buffer_duration = frames_to_duration(frames_available, sample_rate);
    let playback = callback
        .add(buffer_duration)
        .expect("`playback` occurs beyond representation supported by `StreamInstant`");
    Ok(crate::OutputStreamTimestamp { callback, playback })
}
