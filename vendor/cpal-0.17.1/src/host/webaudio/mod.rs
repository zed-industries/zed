//! Web Audio backend implementation.
//!
//! Default backend on WebAssembly.

extern crate js_sys;
extern crate wasm_bindgen;
extern crate web_sys;

use self::wasm_bindgen::prelude::*;
use self::wasm_bindgen::JsCast;
use self::web_sys::{AudioContext, AudioContextOptions};
use crate::traits::{DeviceTrait, HostTrait, StreamTrait};
use crate::{
    BackendSpecificError, BufferSize, BuildStreamError, Data, DefaultStreamConfigError,
    DeviceDescription, DeviceDescriptionBuilder, DeviceId, DeviceIdError, DeviceNameError,
    DevicesError, InputCallbackInfo, OutputCallbackInfo, PauseStreamError, PlayStreamError,
    SampleFormat, SampleRate, StreamConfig, StreamError, SupportedBufferSize,
    SupportedStreamConfig, SupportedStreamConfigRange, SupportedStreamConfigsError,
};
use std::ops::DerefMut;
use std::sync::{Arc, Mutex, RwLock};
use std::time::Duration;

/// Type alias for shared closure handles used in audio callbacks
type ClosureHandle = Arc<RwLock<Option<Closure<dyn FnMut()>>>>;

/// Content is false if the iterator is empty.
pub struct Devices(bool);

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Device;

pub struct Host;

pub struct Stream {
    ctx: Arc<AudioContext>,
    on_ended_closures: Vec<ClosureHandle>,
    config: StreamConfig,
    buffer_size_frames: usize,
}

// WASM runs in a single-threaded environment, so Send and Sync are safe by design.
unsafe impl Send for Stream {}
unsafe impl Sync for Stream {}

// Compile-time assertion that Stream is Send and Sync
crate::assert_stream_send!(Stream);
crate::assert_stream_sync!(Stream);

pub use crate::iter::{SupportedInputConfigs, SupportedOutputConfigs};

const MIN_CHANNELS: u16 = 1;
const MAX_CHANNELS: u16 = 32;
const MIN_SAMPLE_RATE: SampleRate = 8_000;
const MAX_SAMPLE_RATE: SampleRate = 96_000;
const DEFAULT_SAMPLE_RATE: SampleRate = 44_100;
const MIN_BUFFER_SIZE: u32 = 1;
const MAX_BUFFER_SIZE: u32 = u32::MAX;
const DEFAULT_BUFFER_SIZE: usize = 2048;
const SUPPORTED_SAMPLE_FORMAT: SampleFormat = SampleFormat::F32;

impl Host {
    pub fn new() -> Result<Self, crate::HostUnavailable> {
        Ok(Host)
    }
}

impl HostTrait for Host {
    type Devices = Devices;
    type Device = Device;

    fn is_available() -> bool {
        // Assume this host is always available on webaudio.
        true
    }

    fn devices(&self) -> Result<Self::Devices, DevicesError> {
        Devices::new()
    }

    fn default_input_device(&self) -> Option<Self::Device> {
        default_input_device()
    }

    fn default_output_device(&self) -> Option<Self::Device> {
        default_output_device()
    }
}

impl Devices {
    fn new() -> Result<Self, DevicesError> {
        Ok(Self::default())
    }
}

impl Device {
    fn description(&self) -> Result<DeviceDescription, DeviceNameError> {
        Ok(DeviceDescriptionBuilder::new("Default Device".to_string())
            .direction(crate::DeviceDirection::Output)
            .build())
    }

    fn id(&self) -> Result<DeviceId, DeviceIdError> {
        Ok(DeviceId(
            crate::platform::HostId::WebAudio,
            "default".to_string(),
        ))
    }

    fn supported_input_configs(
        &self,
    ) -> Result<SupportedInputConfigs, SupportedStreamConfigsError> {
        // TODO
        Ok(Vec::new().into_iter())
    }

    fn supported_output_configs(
        &self,
    ) -> Result<SupportedOutputConfigs, SupportedStreamConfigsError> {
        let buffer_size = SupportedBufferSize::Range {
            min: MIN_BUFFER_SIZE,
            max: MAX_BUFFER_SIZE,
        };
        let configs: Vec<_> = (MIN_CHANNELS..=MAX_CHANNELS)
            .map(|channels| SupportedStreamConfigRange {
                channels,
                min_sample_rate: MIN_SAMPLE_RATE,
                max_sample_rate: MAX_SAMPLE_RATE,
                buffer_size,
                sample_format: SUPPORTED_SAMPLE_FORMAT,
            })
            .collect();
        Ok(configs.into_iter())
    }

    fn default_input_config(&self) -> Result<SupportedStreamConfig, DefaultStreamConfigError> {
        // TODO
        Err(DefaultStreamConfigError::StreamTypeNotSupported)
    }

    fn default_output_config(&self) -> Result<SupportedStreamConfig, DefaultStreamConfigError> {
        const EXPECT: &str = "expected at least one valid webaudio stream config";
        let config = self
            .supported_output_configs()
            .expect(EXPECT)
            .max_by(|a, b| a.cmp_default_heuristics(b))
            .unwrap()
            .with_sample_rate(DEFAULT_SAMPLE_RATE);

        Ok(config)
    }
}

impl DeviceTrait for Device {
    type SupportedInputConfigs = SupportedInputConfigs;
    type SupportedOutputConfigs = SupportedOutputConfigs;
    type Stream = Stream;

    fn description(&self) -> Result<DeviceDescription, DeviceNameError> {
        Device::description(self)
    }

    fn id(&self) -> Result<DeviceId, DeviceIdError> {
        Device::id(self)
    }

    fn supported_input_configs(
        &self,
    ) -> Result<Self::SupportedInputConfigs, SupportedStreamConfigsError> {
        Device::supported_input_configs(self)
    }

    fn supported_output_configs(
        &self,
    ) -> Result<Self::SupportedOutputConfigs, SupportedStreamConfigsError> {
        Device::supported_output_configs(self)
    }

    fn default_input_config(&self) -> Result<SupportedStreamConfig, DefaultStreamConfigError> {
        Device::default_input_config(self)
    }

    fn default_output_config(&self) -> Result<SupportedStreamConfig, DefaultStreamConfigError> {
        Device::default_output_config(self)
    }

    fn build_input_stream_raw<D, E>(
        &self,
        _config: &StreamConfig,
        _sample_format: SampleFormat,
        _data_callback: D,
        _error_callback: E,
        _timeout: Option<Duration>,
    ) -> Result<Self::Stream, BuildStreamError>
    where
        D: FnMut(&Data, &InputCallbackInfo) + Send + 'static,
        E: FnMut(StreamError) + Send + 'static,
    {
        // TODO
        Err(BuildStreamError::StreamConfigNotSupported)
    }

    /// Create an output stream.
    fn build_output_stream_raw<D, E>(
        &self,
        config: &StreamConfig,
        sample_format: SampleFormat,
        data_callback: D,
        _error_callback: E,
        _timeout: Option<Duration>,
    ) -> Result<Self::Stream, BuildStreamError>
    where
        D: FnMut(&mut Data, &OutputCallbackInfo) + Send + 'static,
        E: FnMut(StreamError) + Send + 'static,
    {
        if !valid_config(config, sample_format) {
            return Err(BuildStreamError::StreamConfigNotSupported);
        }

        let n_channels = config.channels as usize;

        let buffer_size_frames = match config.buffer_size {
            BufferSize::Fixed(v) => {
                if !(MIN_BUFFER_SIZE..=MAX_BUFFER_SIZE).contains(&v) {
                    return Err(BuildStreamError::StreamConfigNotSupported);
                }
                v as usize
            }
            BufferSize::Default => DEFAULT_BUFFER_SIZE,
        };
        let buffer_size_samples = buffer_size_frames * n_channels;
        let buffer_time_step_secs = buffer_time_step_secs(buffer_size_frames, config.sample_rate);

        let data_callback = Arc::new(Mutex::new(Box::new(data_callback)));

        // Create the WebAudio stream.
        let stream_opts = AudioContextOptions::new();
        stream_opts.set_sample_rate(config.sample_rate as f32);
        let ctx = AudioContext::new_with_context_options(&stream_opts).map_err(
            |err| -> BuildStreamError {
                let description = format!("{:?}", err);
                let err = BackendSpecificError { description };
                err.into()
            },
        )?;

        let destination = ctx.destination();

        // If possible, set the destination's channel_count to the given config.channel.
        // If not, fallback on the default destination channel_count to keep previous behavior
        // and do not return an error.
        if config.channels as u32 <= destination.max_channel_count() {
            destination.set_channel_count(config.channels as u32);
        }

        // SAFETY: WASM is single-threaded, so Arc is safe even though AudioContext is not Send/Sync
        #[allow(clippy::arc_with_non_send_sync)]
        let ctx = Arc::new(ctx);

        // A container for managing the lifecycle of the audio callbacks.
        let mut on_ended_closures: Vec<ClosureHandle> = Vec::new();

        // A cursor keeping track of the current time at which new frames should be scheduled.
        let time = Arc::new(RwLock::new(0f64));

        // Create a set of closures / callbacks which will continuously fetch and schedule sample
        // playback. Starting with two workers, e.g. a front and back buffer so that audio frames
        // can be fetched in the background.
        for _i in 0..2 {
            let data_callback_handle = data_callback.clone();
            let ctx_handle = ctx.clone();
            let time_handle = time.clone();

            // A set of temporary buffers to be used for intermediate sample transformation steps.
            let mut temporary_buffer = vec![0f32; buffer_size_samples];
            let mut temporary_channel_buffer = vec![0f32; buffer_size_frames];

            #[cfg(target_feature = "atomics")]
            let temporary_channel_array_view: js_sys::Float32Array;
            #[cfg(target_feature = "atomics")]
            {
                let temporary_channel_array = js_sys::ArrayBuffer::new(
                    (std::mem::size_of::<f32>() * buffer_size_frames) as u32,
                );
                temporary_channel_array_view = js_sys::Float32Array::new(&temporary_channel_array);
            }

            // Create a webaudio buffer which will be reused to avoid allocations.
            let ctx_buffer = ctx
                .create_buffer(
                    config.channels as u32,
                    buffer_size_frames as u32,
                    config.sample_rate as f32,
                )
                .map_err(|err| -> BuildStreamError {
                    let description = format!("{:?}", err);
                    let err = BackendSpecificError { description };
                    err.into()
                })?;

            // A self reference to this closure for passing to future audio event calls.
            // SAFETY: WASM is single-threaded, so Arc is safe even though Closure is not Send/Sync
            #[allow(clippy::arc_with_non_send_sync)]
            let on_ended_closure: ClosureHandle = Arc::new(RwLock::new(None));
            let on_ended_closure_handle = on_ended_closure.clone();

            on_ended_closure
                .write()
                .unwrap()
                .replace(Closure::wrap(Box::new(move || {
                    let now = ctx_handle.current_time();
                    let time_at_start_of_buffer = {
                        let time_at_start_of_buffer = time_handle
                            .read()
                            .expect("Unable to get a read lock on the time cursor");
                        // Synchronise first buffer as necessary (eg. keep the time value
                        // referenced to the context clock).
                        if *time_at_start_of_buffer > 0.001 {
                            *time_at_start_of_buffer
                        } else {
                            // 25ms of time to fetch the first sample data, increase to avoid
                            // initial underruns.
                            now + 0.025
                        }
                    };

                    // Populate the sample data into an interleaved temporary buffer.
                    {
                        let len = temporary_buffer.len();
                        let data = temporary_buffer.as_mut_ptr() as *mut ();
                        let mut data = unsafe { Data::from_parts(data, len, sample_format) };
                        let mut data_callback = data_callback_handle.lock().unwrap();
                        let callback = crate::StreamInstant::from_secs_f64(now);
                        let playback = crate::StreamInstant::from_secs_f64(time_at_start_of_buffer);
                        let timestamp = crate::OutputStreamTimestamp { callback, playback };
                        let info = OutputCallbackInfo { timestamp };
                        (data_callback.deref_mut())(&mut data, &info);
                    }

                    // Deinterleave the sample data and copy into the audio context buffer.
                    // We do not reference the audio context buffer directly e.g. getChannelData.
                    // As wasm-bindgen only gives us a copy, not a direct reference.
                    for channel in 0..n_channels {
                        for i in 0..buffer_size_frames {
                            temporary_channel_buffer[i] =
                                temporary_buffer[n_channels * i + channel];
                        }

                        #[cfg(not(target_feature = "atomics"))]
                        {
                            ctx_buffer
                                .copy_to_channel(&temporary_channel_buffer, channel as i32)
                                .expect(
                                    "Unable to write sample data into the audio context buffer",
                                );
                        }

                        // copyToChannel cannot be directly copied into from a SharedArrayBuffer,
                        // which WASM memory is backed by if the 'atomics' flag is enabled.
                        // This workaround copies the data into an intermediary buffer first.
                        // There's a chance browsers may eventually relax that requirement.
                        // See this issue: https://github.com/WebAudio/web-audio-api/issues/2565
                        #[cfg(target_feature = "atomics")]
                        {
                            temporary_channel_array_view.copy_from(&temporary_channel_buffer);
                            ctx_buffer
                                .unchecked_ref::<ExternalArrayAudioBuffer>()
                                .copy_to_channel(&temporary_channel_array_view, channel as i32)
                                .expect(
                                    "Unable to write sample data into the audio context buffer",
                                );
                        }
                    }

                    // Create an AudioBufferSourceNode, schedule it to playback the reused buffer
                    // in the future.
                    let source = ctx_handle
                        .create_buffer_source()
                        .expect("Unable to create a webaudio buffer source");
                    source.set_buffer(Some(&ctx_buffer));
                    source
                        .connect_with_audio_node(&ctx_handle.destination())
                        .expect(
                        "Unable to connect the web audio buffer source to the context destination",
                    );
                    source
                        .add_event_listener_with_callback(
                            "ended",
                            on_ended_closure_handle
                                .read()
                                .unwrap()
                                .as_ref()
                                .unwrap()
                                .as_ref()
                                .unchecked_ref(),
                        )
                        .expect("Failed to add ended event listener");

                    source
                        .start_with_when(time_at_start_of_buffer)
                        .expect("Unable to start the webaudio buffer source");

                    // Keep track of when the next buffer worth of samples should be played.
                    *time_handle.write().unwrap() = time_at_start_of_buffer + buffer_time_step_secs;
                }) as Box<dyn FnMut()>));

            on_ended_closures.push(on_ended_closure);
        }

        Ok(Stream {
            ctx,
            on_ended_closures,
            config: config.clone(),
            buffer_size_frames,
        })
    }
}

impl Stream {
    /// Return the [`AudioContext`](https://developer.mozilla.org/docs/Web/API/AudioContext) used
    /// by this stream.
    pub fn audio_context(&self) -> &AudioContext {
        &self.ctx
    }
}

impl StreamTrait for Stream {
    fn play(&self) -> Result<(), PlayStreamError> {
        let window = web_sys::window().unwrap();
        match self.ctx.resume() {
            Ok(_) => {
                // Begin webaudio playback, initially scheduling the closures to fire on a timeout
                // event.
                let mut offset_ms = 10;
                let time_step_secs =
                    buffer_time_step_secs(self.buffer_size_frames, self.config.sample_rate);
                let time_step_ms = (time_step_secs * 1_000.0) as i32;
                for on_ended_closure in self.on_ended_closures.iter() {
                    window
                        .set_timeout_with_callback_and_timeout_and_arguments_0(
                            on_ended_closure
                                .read()
                                .unwrap()
                                .as_ref()
                                .unwrap()
                                .as_ref()
                                .unchecked_ref(),
                            offset_ms,
                        )
                        .unwrap();
                    offset_ms += time_step_ms;
                }
                Ok(())
            }
            Err(err) => {
                let description = format!("{:?}", err);
                let err = BackendSpecificError { description };
                Err(err.into())
            }
        }
    }

    fn pause(&self) -> Result<(), PauseStreamError> {
        match self.ctx.suspend() {
            Ok(_) => Ok(()),
            Err(err) => {
                let description = format!("{:?}", err);
                let err = BackendSpecificError { description };
                Err(err.into())
            }
        }
    }
}

impl Drop for Stream {
    fn drop(&mut self) {
        let _ = self.ctx.close();
    }
}

impl Default for Devices {
    fn default() -> Devices {
        // We produce an empty iterator if the WebAudio API isn't available.
        Devices(is_webaudio_available())
    }
}

impl Iterator for Devices {
    type Item = Device;

    #[inline]
    fn next(&mut self) -> Option<Device> {
        if self.0 {
            self.0 = false;
            Some(Device)
        } else {
            None
        }
    }
}

fn default_input_device() -> Option<Device> {
    // TODO
    None
}

fn default_output_device() -> Option<Device> {
    if is_webaudio_available() {
        Some(Device)
    } else {
        None
    }
}

// Detects whether the `AudioContext` global variable is available.
fn is_webaudio_available() -> bool {
    js_sys::Reflect::get(&js_sys::global(), &JsValue::from("AudioContext"))
        .unwrap()
        .is_truthy()
}

// Whether or not the given stream configuration is valid for building a stream.
fn valid_config(conf: &StreamConfig, sample_format: SampleFormat) -> bool {
    conf.channels <= MAX_CHANNELS
        && conf.channels >= MIN_CHANNELS
        && conf.sample_rate <= MAX_SAMPLE_RATE
        && conf.sample_rate >= MIN_SAMPLE_RATE
        && sample_format == SUPPORTED_SAMPLE_FORMAT
}

fn buffer_time_step_secs(buffer_size_frames: usize, sample_rate: SampleRate) -> f64 {
    buffer_size_frames as f64 / sample_rate as f64
}

#[cfg(target_feature = "atomics")]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = AudioBuffer)]
    type ExternalArrayAudioBuffer;

    # [wasm_bindgen(catch, method, structural, js_class = "AudioBuffer", js_name = copyToChannel)]
    pub fn copy_to_channel(
        this: &ExternalArrayAudioBuffer,
        source: &js_sys::Float32Array,
        channel_number: i32,
    ) -> Result<(), JsValue>;
}
