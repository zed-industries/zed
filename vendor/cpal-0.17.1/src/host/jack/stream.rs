use crate::traits::StreamTrait;
use crate::ChannelCount;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};

use crate::{
    BackendSpecificError, Data, InputCallbackInfo, OutputCallbackInfo, PauseStreamError,
    PlayStreamError, SampleRate, StreamError,
};

use super::JACK_SAMPLE_FORMAT;

type ErrorCallbackPtr = Arc<Mutex<dyn FnMut(StreamError) + Send + 'static>>;

pub struct Stream {
    // TODO: It might be faster to send a message when playing/pausing than to check this every iteration
    playing: Arc<AtomicBool>,
    async_client: jack::AsyncClient<JackNotificationHandler, LocalProcessHandler>,
    // Port names are stored in order to connect them to other ports in jack automatically
    input_port_names: Vec<String>,
    output_port_names: Vec<String>,
}

// Compile-time assertion that Stream is Send and Sync
crate::assert_stream_send!(Stream);
crate::assert_stream_sync!(Stream);

impl Stream {
    // TODO: Return error messages
    pub fn new_input<D, E>(
        client: jack::Client,
        channels: ChannelCount,
        data_callback: D,
        mut error_callback: E,
    ) -> Stream
    where
        D: FnMut(&Data, &InputCallbackInfo) + Send + 'static,
        E: FnMut(StreamError) + Send + 'static,
    {
        let mut ports = vec![];
        let mut port_names: Vec<String> = vec![];
        // Create ports
        for i in 0..channels {
            let port_try = client.register_port(&format!("in_{}", i), jack::AudioIn::default());
            match port_try {
                Ok(port) => {
                    // Get the port name in order to later connect it automatically
                    if let Ok(port_name) = port.name() {
                        port_names.push(port_name);
                    }
                    // Store the port into a Vec to move to the ProcessHandler
                    ports.push(port);
                }
                Err(e) => {
                    // If port creation failed, send the error back via the error_callback
                    error_callback(
                        BackendSpecificError {
                            description: e.to_string(),
                        }
                        .into(),
                    );
                }
            }
        }

        let playing = Arc::new(AtomicBool::new(true));

        let error_callback_ptr = Arc::new(Mutex::new(error_callback)) as ErrorCallbackPtr;

        let input_process_handler = LocalProcessHandler::new(
            vec![],
            ports,
            client.sample_rate() as u32,
            client.buffer_size() as usize,
            Some(Box::new(data_callback)),
            None,
            playing.clone(),
            Arc::clone(&error_callback_ptr),
        );

        let notification_handler = JackNotificationHandler::new(error_callback_ptr);

        let async_client = client
            .activate_async(notification_handler, input_process_handler)
            .unwrap();

        Stream {
            playing,
            async_client,
            input_port_names: port_names,
            output_port_names: vec![],
        }
    }

    pub fn new_output<D, E>(
        client: jack::Client,
        channels: ChannelCount,
        data_callback: D,
        mut error_callback: E,
    ) -> Stream
    where
        D: FnMut(&mut Data, &OutputCallbackInfo) + Send + 'static,
        E: FnMut(StreamError) + Send + 'static,
    {
        let mut ports = vec![];
        let mut port_names: Vec<String> = vec![];
        // Create ports
        for i in 0..channels {
            let port_try = client.register_port(&format!("out_{}", i), jack::AudioOut::default());
            match port_try {
                Ok(port) => {
                    // Get the port name in order to later connect it automatically
                    if let Ok(port_name) = port.name() {
                        port_names.push(port_name);
                    }
                    // Store the port into a Vec to move to the ProcessHandler
                    ports.push(port);
                }
                Err(e) => {
                    // If port creation failed, send the error back via the error_callback
                    error_callback(
                        BackendSpecificError {
                            description: e.to_string(),
                        }
                        .into(),
                    );
                }
            }
        }

        let playing = Arc::new(AtomicBool::new(true));

        let error_callback_ptr = Arc::new(Mutex::new(error_callback)) as ErrorCallbackPtr;

        let output_process_handler = LocalProcessHandler::new(
            ports,
            vec![],
            client.sample_rate() as u32,
            client.buffer_size() as usize,
            None,
            Some(Box::new(data_callback)),
            playing.clone(),
            Arc::clone(&error_callback_ptr),
        );

        let notification_handler = JackNotificationHandler::new(error_callback_ptr);

        let async_client = client
            .activate_async(notification_handler, output_process_handler)
            .unwrap();

        Stream {
            playing,
            async_client,
            input_port_names: vec![],
            output_port_names: port_names,
        }
    }

    /// Connect to the standard system outputs in jack, system:playback_1 and system:playback_2
    /// This has to be done after the client is activated, doing it just after creating the ports doesn't work.
    pub fn connect_to_system_outputs(&mut self) {
        // Get the system ports
        let system_ports = self.async_client.as_client().ports(
            Some("system:playback_.*"),
            None,
            jack::PortFlags::empty(),
        );

        // Connect outputs from this client to the system playback inputs
        for i in 0..self.output_port_names.len() {
            if i >= system_ports.len() {
                break;
            }
            match self
                .async_client
                .as_client()
                .connect_ports_by_name(&self.output_port_names[i], &system_ports[i])
            {
                Ok(_) => (),
                Err(e) => println!("Unable to connect to port with error {}", e),
            }
        }
    }

    /// Connect to the standard system outputs in jack, system:capture_1 and system:capture_2
    /// This has to be done after the client is activated, doing it just after creating the ports doesn't work.
    pub fn connect_to_system_inputs(&mut self) {
        // Get the system ports
        let system_ports = self.async_client.as_client().ports(
            Some("system:capture_.*"),
            None,
            jack::PortFlags::empty(),
        );

        // Connect outputs from this client to the system playback inputs
        for i in 0..self.input_port_names.len() {
            if i >= system_ports.len() {
                break;
            }
            match self
                .async_client
                .as_client()
                .connect_ports_by_name(&system_ports[i], &self.input_port_names[i])
            {
                Ok(_) => (),
                Err(e) => println!("Unable to connect to port with error {}", e),
            }
        }
    }
}

impl StreamTrait for Stream {
    fn play(&self) -> Result<(), PlayStreamError> {
        self.playing.store(true, Ordering::SeqCst);
        Ok(())
    }

    fn pause(&self) -> Result<(), PauseStreamError> {
        self.playing.store(false, Ordering::SeqCst);
        Ok(())
    }
}

type InputDataCallback = Box<dyn FnMut(&Data, &InputCallbackInfo) + Send + 'static>;
type OutputDataCallback = Box<dyn FnMut(&mut Data, &OutputCallbackInfo) + Send + 'static>;

struct LocalProcessHandler {
    /// No new ports are allowed to be created after the creation of the LocalProcessHandler as that would invalidate the buffer sizes
    out_ports: Vec<jack::Port<jack::AudioOut>>,
    in_ports: Vec<jack::Port<jack::AudioIn>>,

    sample_rate: SampleRate,
    buffer_size: usize,
    input_data_callback: Option<InputDataCallback>,
    output_data_callback: Option<OutputDataCallback>,

    // JACK audio samples are 32-bit float (unless you do some custom dark magic)
    temp_input_buffer: Vec<f32>,
    temp_output_buffer: Vec<f32>,
    playing: Arc<AtomicBool>,
    creation_timestamp: std::time::Instant,
    /// This should not be called on `process`, only on `buffer_size` because it can block.
    error_callback_ptr: ErrorCallbackPtr,
}

impl LocalProcessHandler {
    #[allow(clippy::too_many_arguments)]
    fn new(
        out_ports: Vec<jack::Port<jack::AudioOut>>,
        in_ports: Vec<jack::Port<jack::AudioIn>>,
        sample_rate: SampleRate,
        buffer_size: usize,
        input_data_callback: Option<InputDataCallback>,
        output_data_callback: Option<OutputDataCallback>,
        playing: Arc<AtomicBool>,
        error_callback_ptr: ErrorCallbackPtr,
    ) -> Self {
        // These may be reallocated in the `buffer_size` callback.
        let temp_input_buffer = vec![0.0; in_ports.len() * buffer_size];
        let temp_output_buffer = vec![0.0; out_ports.len() * buffer_size];

        LocalProcessHandler {
            out_ports,
            in_ports,
            sample_rate,
            buffer_size,
            input_data_callback,
            output_data_callback,
            temp_input_buffer,
            temp_output_buffer,
            playing,
            creation_timestamp: std::time::Instant::now(),
            error_callback_ptr,
        }
    }
}

fn temp_buffer_to_data(temp_input_buffer: &mut [f32], total_buffer_size: usize) -> Data {
    let slice = &mut temp_input_buffer[0..total_buffer_size];
    let data: *mut () = slice.as_mut_ptr().cast();
    let len = total_buffer_size;
    unsafe { Data::from_parts(data, len, JACK_SAMPLE_FORMAT) }
}

impl jack::ProcessHandler for LocalProcessHandler {
    fn process(&mut self, _: &jack::Client, process_scope: &jack::ProcessScope) -> jack::Control {
        if !self.playing.load(Ordering::SeqCst) {
            return jack::Control::Continue;
        }

        // This should be equal to self.buffer_size, but the implementation will
        // work even if it is less. Will panic in `temp_buffer_to_data` if greater.
        let current_frame_count = process_scope.n_frames() as usize;

        // Get timestamp data
        let cycle_times = process_scope.cycle_times();
        let current_start_usecs = match cycle_times {
            Ok(times) => times.current_usecs,
            Err(_) => {
                // jack was unable to get the current time information
                // Fall back to using Instants
                let now = std::time::Instant::now();
                let duration = now.duration_since(self.creation_timestamp);
                duration.as_micros() as u64
            }
        };
        let start_cycle_instant = micros_to_stream_instant(current_start_usecs);
        let start_callback_instant = start_cycle_instant
            .add(frames_to_duration(
                process_scope.frames_since_cycle_start() as usize,
                self.sample_rate,
            ))
            .expect("`playback` occurs beyond representation supported by `StreamInstant`");

        if let Some(input_callback) = &mut self.input_data_callback {
            // Let's get the data from the input ports and run the callback

            let num_in_channels = self.in_ports.len();

            // Read the data from the input ports into the temporary buffer
            // Go through every channel and store its data in the temporary input buffer
            for ch_ix in 0..num_in_channels {
                let input_channel = &self.in_ports[ch_ix].as_slice(process_scope);
                for i in 0..current_frame_count {
                    self.temp_input_buffer[ch_ix + i * num_in_channels] = input_channel[i];
                }
            }
            // Create a slice of exactly current_frame_count frames
            let data = temp_buffer_to_data(
                &mut self.temp_input_buffer,
                current_frame_count * num_in_channels,
            );
            // Create timestamp
            let frames_since_cycle_start = process_scope.frames_since_cycle_start() as usize;
            let duration_since_cycle_start =
                frames_to_duration(frames_since_cycle_start, self.sample_rate);
            let callback = start_callback_instant
                .add(duration_since_cycle_start)
                .expect("`playback` occurs beyond representation supported by `StreamInstant`");
            let capture = start_callback_instant;
            let timestamp = crate::InputStreamTimestamp { callback, capture };
            let info = crate::InputCallbackInfo { timestamp };
            input_callback(&data, &info);
        }

        if let Some(output_callback) = &mut self.output_data_callback {
            let num_out_channels = self.out_ports.len();

            // Create a slice of exactly current_frame_count frames
            let mut data = temp_buffer_to_data(
                &mut self.temp_output_buffer,
                current_frame_count * num_out_channels,
            );
            // Create timestamp
            let frames_since_cycle_start = process_scope.frames_since_cycle_start() as usize;
            let duration_since_cycle_start =
                frames_to_duration(frames_since_cycle_start, self.sample_rate);
            let callback = start_callback_instant
                .add(duration_since_cycle_start)
                .expect("`playback` occurs beyond representation supported by `StreamInstant`");
            let buffer_duration = frames_to_duration(current_frame_count, self.sample_rate);
            let playback = start_cycle_instant
                .add(buffer_duration)
                .expect("`playback` occurs beyond representation supported by `StreamInstant`");
            let timestamp = crate::OutputStreamTimestamp { callback, playback };
            let info = crate::OutputCallbackInfo { timestamp };
            output_callback(&mut data, &info);

            // Deinterlace
            for ch_ix in 0..num_out_channels {
                let output_channel = &mut self.out_ports[ch_ix].as_mut_slice(process_scope);
                for i in 0..current_frame_count {
                    output_channel[i] = self.temp_output_buffer[ch_ix + i * num_out_channels];
                }
            }
        }

        // Continue as normal
        jack::Control::Continue
    }

    fn buffer_size(&mut self, _: &jack::Client, size: jack::Frames) -> jack::Control {
        // The `buffer_size` callback is actually called on the process thread, but
        // it does not need to be suitable for real-time use. Thus we can simply allocate
        // new buffers here. It is also fine to call the error callback.
        // Details: https://github.com/RustAudio/rust-jack/issues/137
        let new_size = size as usize;
        if new_size != self.buffer_size {
            self.buffer_size = new_size;
            self.temp_input_buffer = vec![0.0; self.in_ports.len() * new_size];
            self.temp_output_buffer = vec![0.0; self.out_ports.len() * new_size];
            let description = format!("buffer size changed to: {}", new_size);
            if let Ok(mut mutex_guard) = self.error_callback_ptr.lock() {
                let err = &mut *mutex_guard;
                err(BackendSpecificError { description }.into());
            }
        }

        jack::Control::Continue
    }
}

fn micros_to_stream_instant(micros: u64) -> crate::StreamInstant {
    let nanos = micros * 1000;
    let secs = micros / 1_000_000;
    let subsec_nanos = nanos - secs * 1_000_000_000;
    crate::StreamInstant::new(secs as i64, subsec_nanos as u32)
}

// Convert the given duration in frames at the given sample rate to a `std::time::Duration`.
fn frames_to_duration(frames: usize, rate: crate::SampleRate) -> std::time::Duration {
    let secsf = frames as f64 / rate as f64;
    let secs = secsf as u64;
    let nanos = ((secsf - secs as f64) * 1_000_000_000.0) as u32;
    std::time::Duration::new(secs, nanos)
}

/// Receives notifications from the JACK server. It is unclear if this may be run concurrent with itself under JACK2 specs
/// so it needs to be Sync.
struct JackNotificationHandler {
    error_callback_ptr: ErrorCallbackPtr,
    init_sample_rate_flag: Arc<AtomicBool>,
}

impl JackNotificationHandler {
    pub fn new(error_callback_ptr: ErrorCallbackPtr) -> Self {
        JackNotificationHandler {
            error_callback_ptr,
            init_sample_rate_flag: Arc::new(AtomicBool::new(false)),
        }
    }

    fn send_error(&mut self, description: String) {
        // This thread isn't the audio thread, it's fine to block
        if let Ok(mut mutex_guard) = self.error_callback_ptr.lock() {
            let err = &mut *mutex_guard;
            err(BackendSpecificError { description }.into());
        }
    }
}

impl jack::NotificationHandler for JackNotificationHandler {
    unsafe fn shutdown(&mut self, _status: jack::ClientStatus, reason: &str) {
        self.send_error(format!("JACK was shut down for reason: {}", reason));
    }

    fn sample_rate(&mut self, _: &jack::Client, _srate: jack::Frames) -> jack::Control {
        match self.init_sample_rate_flag.load(Ordering::SeqCst) {
            false => {
                // One of these notifications is sent every time a client is started.
                self.init_sample_rate_flag.store(true, Ordering::SeqCst);
                jack::Control::Continue
            }
            true => {
                // The JACK server has changed the sample rate, invalidating this stream.
                // The stream configuration must be rebuilt with the new sample rate.
                if let Ok(mut cb) = self.error_callback_ptr.lock() {
                    cb(StreamError::StreamInvalidated);
                }
                jack::Control::Quit
            }
        }
    }

    fn xrun(&mut self, _: &jack::Client) -> jack::Control {
        if let Ok(mut cb) = self.error_callback_ptr.lock() {
            cb(StreamError::BufferUnderrun);
        }
        jack::Control::Continue
    }
}
