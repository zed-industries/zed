//! # How to use cpal
//!
//! Here are some concepts cpal exposes:
//!
//! - A [`Host`] provides access to the available audio devices on the system.
//!   Some platforms have more than one host available, but every platform supported by CPAL has at
//!   least one [default_host] that is guaranteed to be available.
//! - A [`Device`] is an audio device that may have any number of input and
//!   output streams.
//! - A [`Stream`] is an open flow of audio data. Input streams allow you to
//!   receive audio data, output streams allow you to play audio data. You must choose which
//!   [Device] will run your stream before you can create one. Often, a default device can be
//!   retrieved via the [Host].
//!
//! The first step is to initialise the [`Host`]:
//!
//! ```
//! use cpal::traits::HostTrait;
//! let host = cpal::default_host();
//! ```
//!
//! Then choose an available [`Device`]. The easiest way is to use the default input or output
//! `Device` via the [`default_input_device()`] or [`default_output_device()`] methods on `host`.
//!
//! Alternatively, you can enumerate all the available devices with the [`devices()`] method.
//! Beware that the `default_*_device()` functions return an `Option<Device>` in case no device
//! is available for that stream type on the system.
//!
//! ```no_run
//! # use cpal::traits::HostTrait;
//! # let host = cpal::default_host();
//! let device = host.default_output_device().expect("no output device available");
//! ```
//!
//! Before we can create a stream, we must decide what the configuration of the audio stream is
//! going to be.
//! You can query all the supported configurations with the
//! [`supported_input_configs()`] and [`supported_output_configs()`] methods.
//! These produce a list of [`SupportedStreamConfigRange`] structs which can later be turned into
//! actual [`SupportedStreamConfig`] structs.
//!
//! If you don't want to query the list of configs,
//! you can also build your own [`StreamConfig`] manually, but doing so could lead to an error when
//! building the stream if the config is not supported by the device.
//!
//! > **Note**: the `supported_input/output_configs()` methods
//! > could return an error for example if the device has been disconnected.
//!
//! ```no_run
//! use cpal::traits::{DeviceTrait, HostTrait};
//! # let host = cpal::default_host();
//! # let device = host.default_output_device().unwrap();
//! let mut supported_configs_range = device.supported_output_configs()
//!     .expect("error while querying configs");
//! let supported_config = supported_configs_range.next()
//!     .expect("no supported config?!")
//!     .with_max_sample_rate();
//! ```
//!
//! Now that we have everything for the stream, we are ready to create it from our selected device:
//!
//! ```no_run
//! use cpal::Data;
//! use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
//! # let host = cpal::default_host();
//! # let device = host.default_output_device().unwrap();
//! # let config = device.default_output_config().unwrap().into();
//! let stream = device.build_output_stream(
//!     &config,
//!     move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
//!         // react to stream events and read or write stream data here.
//!     },
//!     move |err| {
//!         // react to errors here.
//!     },
//!     None // None=blocking, Some(Duration)=timeout
//! );
//! ```
//!
//! While the stream is running, the selected audio device will periodically call the data callback
//! that was passed to the function. For input streams, the callback receives `&`[`Data`] containing
//! captured audio samples. For output streams, the callback receives `&mut`[`Data`] to be filled
//! with audio samples for playback.
//!
//! > **Note**: Creating and running a stream will *not* block the thread. On modern platforms, the
//! > given callback is called by a dedicated, high-priority thread responsible for delivering
//! > audio data to the system's audio device in a timely manner. On older platforms that only
//! > provide a blocking API (e.g. ALSA), CPAL will create a thread in order to consistently
//! > provide non-blocking behaviour (currently this is a thread per stream, but this may change to
//! > use a single thread for all streams). *If this is an issue for your platform or design,
//! > please share your issue and use-case with the CPAL team on the GitHub issue tracker for
//! > consideration.*
//!
//! In this example, we simply fill the given output buffer with silence.
//!
//! ```no_run
//! use cpal::{Data, Sample, SampleFormat, FromSample};
//! use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
//! # let host = cpal::default_host();
//! # let device = host.default_output_device().unwrap();
//! # let supported_config = device.default_output_config().unwrap();
//! let err_fn = |err| eprintln!("an error occurred on the output audio stream: {}", err);
//! let sample_format = supported_config.sample_format();
//! let config = supported_config.into();
//! let stream = match sample_format {
//!     SampleFormat::F32 => device.build_output_stream(&config, write_silence::<f32>, err_fn, None),
//!     SampleFormat::I16 => device.build_output_stream(&config, write_silence::<i16>, err_fn, None),
//!     SampleFormat::U16 => device.build_output_stream(&config, write_silence::<u16>, err_fn, None),
//!     sample_format => panic!("Unsupported sample format '{sample_format}'")
//! }.unwrap();
//!
//! fn write_silence<T: Sample>(data: &mut [T], _: &cpal::OutputCallbackInfo) {
//!     for sample in data.iter_mut() {
//!         *sample = Sample::EQUILIBRIUM;
//!     }
//! }
//! ```
//!
//! Not all platforms automatically run the stream upon creation. To ensure the stream has started,
//! we can use [`Stream::play`](traits::StreamTrait::play).
//!
//! ```no_run
//! # use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
//! # let host = cpal::default_host();
//! # let device = host.default_output_device().unwrap();
//! # let supported_config = device.default_output_config().unwrap();
//! # let sample_format = supported_config.sample_format();
//! # let config = supported_config.into();
//! # let data_fn = move |_data: &mut cpal::Data, _: &cpal::OutputCallbackInfo| {};
//! # let err_fn = move |_err| {};
//! # let stream = device.build_output_stream_raw(&config, sample_format, data_fn, err_fn, None).unwrap();
//! stream.play().unwrap();
//! ```
//!
//! Some devices support pausing the audio stream. This can be useful for saving energy in moments
//! of silence.
//!
//! ```no_run
//! # use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
//! # let host = cpal::default_host();
//! # let device = host.default_output_device().unwrap();
//! # let supported_config = device.default_output_config().unwrap();
//! # let sample_format = supported_config.sample_format();
//! # let config = supported_config.into();
//! # let data_fn = move |_data: &mut cpal::Data, _: &cpal::OutputCallbackInfo| {};
//! # let err_fn = move |_err| {};
//! # let stream = device.build_output_stream_raw(&config, sample_format, data_fn, err_fn, None).unwrap();
//! stream.pause().unwrap();
//! ```
//!
//! [`default_input_device()`]: traits::HostTrait::default_input_device
//! [`default_output_device()`]: traits::HostTrait::default_output_device
//! [`devices()`]: traits::HostTrait::devices
//! [`supported_input_configs()`]: traits::DeviceTrait::supported_input_configs
//! [`supported_output_configs()`]: traits::DeviceTrait::supported_output_configs

#![cfg_attr(docsrs, feature(doc_cfg))]

// Extern crate declarations with `#[macro_use]` must unfortunately be at crate root.
#[cfg(all(
    target_arch = "wasm32",
    any(target_os = "emscripten", feature = "wasm-bindgen")
))]
extern crate js_sys;
#[cfg(all(
    target_arch = "wasm32",
    any(target_os = "emscripten", feature = "wasm-bindgen")
))]
extern crate wasm_bindgen;
#[cfg(all(
    target_arch = "wasm32",
    any(target_os = "emscripten", feature = "wasm-bindgen")
))]
extern crate web_sys;

#[cfg(all(
    target_arch = "wasm32",
    any(target_os = "emscripten", feature = "wasm-bindgen")
))]
use wasm_bindgen::prelude::*;

pub use device_description::{
    DeviceDescription, DeviceDescriptionBuilder, DeviceDirection, DeviceType, InterfaceType,
};
pub use error::*;
pub use platform::{
    available_hosts, default_host, host_from_id, Device, Devices, Host, HostId, Stream,
    SupportedInputConfigs, SupportedOutputConfigs, ALL_HOSTS,
};
pub use samples_formats::{FromSample, Sample, SampleFormat, SizedSample, I24, U24};
use std::convert::TryInto;
use std::time::Duration;

pub mod device_description;
mod error;
mod host;
pub mod platform;
mod samples_formats;
pub mod traits;

/// Iterator of devices wrapped in a filter to only include certain device types
pub type DevicesFiltered<I> = std::iter::Filter<I, fn(&<I as Iterator>::Item) -> bool>;

/// A host's device iterator yielding only *input* devices.
pub type InputDevices<I> = DevicesFiltered<I>;

/// A host's device iterator yielding only *output* devices.
pub type OutputDevices<I> = DevicesFiltered<I>;

/// Number of channels.
pub type ChannelCount = u16;

/// The number of samples processed per second for a single channel of audio.
pub type SampleRate = u32;

/// A frame represents one sample for each channel. For example, with stereo audio,
/// one frame contains two samples (left and right channels).
pub type FrameCount = u32;

/// A stable identifier for an audio device across all supported platforms.
///
/// Device IDs should remain stable across application restarts and can be serialized using `Display`/`FromStr`.
///
/// A device ID consists of a [`HostId`] identifying the audio backend and a device-specific identifier string.
///
/// # Example
///
/// ```no_run
/// use cpal::traits::{HostTrait, DeviceTrait};
/// use cpal::DeviceId;
/// use std::str::FromStr;
///
/// let host = cpal::default_host();
/// let device = host.default_output_device().unwrap();
/// let device_id = device.id().unwrap();
///
/// // Serialize to string (e.g., for storage in config file)
/// let id_string = device_id.to_string();
/// println!("Device ID: {}", id_string); // e.g., "wasapi:device_identifier"
///
/// // Deserialize from string
/// match DeviceId::from_str(&id_string) {
///     Ok(parsed_id) => {
///         // Retrieve the device by its ID
///         if let Some(device) = host.device_by_id(&parsed_id) {
///             println!("Found device: {:?}", device.id());
///         }
///     }
///     Err(e) => eprintln!("Failed to parse device ID: {}", e),
/// }
/// ```
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct DeviceId(pub crate::platform::HostId, pub String);

impl std::fmt::Display for DeviceId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.0, self.1)
    }
}

impl std::str::FromStr for DeviceId {
    type Err = DeviceIdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (host_str, device_str) = s.split_once(':').ok_or(DeviceIdError::BackendSpecific {
            err: BackendSpecificError {
                description: format!(
                    "Failed to parse device ID from: {s}\nCheck if format matches \"host:device_id\""
                ),
            },
        })?;

        let host_id = crate::platform::HostId::from_str(host_str)
            .map_err(|_| DeviceIdError::UnsupportedPlatform)?;

        Ok(DeviceId(host_id, device_str.to_string()))
    }
}

/// The buffer size requests the callback size for audio streams.
///
/// This controls the approximate size of the audio buffer passed to your callback.
/// The actual callback size depends on the host/platform implementation and hardware
/// constraints, and may differ from or vary around the requested size.
///
/// ## Callback Size Expectations
///
/// When you specify [`BufferSize::Fixed(x)`], you are **requesting** that callbacks
/// receive approximately `x` frames of audio data. However, **no guarantees can be
/// made** about the actual callback size:
///
/// - The host may round to hardware-supported values
/// - Different devices have different constraints
/// - The callback size may vary between calls (especially on mobile platforms)
/// - The actual size might be larger or smaller than requested
///
/// ## Latency Considerations
///
/// [`BufferSize::Default`] uses the host's default buffer size, which may be
/// surprisingly large, leading to higher latency. If low latency is desired,
/// [`BufferSize::Fixed`] should be used with a small value in accordance with
/// the [`SupportedBufferSize`] range from [`SupportedStreamConfig`].
///
/// Smaller buffer sizes reduce latency but may increase CPU usage and risk audio
/// dropouts if the callback cannot process audio quickly enough.
///
/// # Example
///
/// ```no_run
/// use cpal::traits::{DeviceTrait, HostTrait};
/// use cpal::{BufferSize, SupportedBufferSize};
///
/// let host = cpal::default_host();
/// let device = host.default_output_device().unwrap();
/// let config = device.default_output_config().unwrap();
///
/// // Check supported buffer size range
/// match config.buffer_size() {
///     SupportedBufferSize::Range { min, max } => {
///         println!("Buffer size range: {} - {}", min, max);
///         // Request a small buffer for low latency
///         let mut stream_config = config.config();
///         stream_config.buffer_size = BufferSize::Fixed(256);
///     }
///     SupportedBufferSize::Unknown => {
///         // Platform doesn't expose buffer size control
///         println!("Buffer size cannot be queried on this platform");
///     }
/// }
/// ```
///
/// [`BufferSize::Default`]: BufferSize::Default
/// [`BufferSize::Fixed`]: BufferSize::Fixed
/// [`BufferSize::Fixed(x)`]: BufferSize::Fixed
/// [`SupportedBufferSize`]: SupportedStreamConfig::buffer_size
/// [`SupportedStreamConfig`]: SupportedStreamConfig
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BufferSize {
    Default,
    Fixed(FrameCount),
}

#[cfg(all(
    target_arch = "wasm32",
    any(target_os = "emscripten", feature = "wasm-bindgen")
))]
impl wasm_bindgen::describe::WasmDescribe for BufferSize {
    fn describe() {
        <Option<FrameCount> as wasm_bindgen::describe::WasmDescribe>::describe();
    }
}

#[cfg(all(
    target_arch = "wasm32",
    any(target_os = "emscripten", feature = "wasm-bindgen")
))]
impl wasm_bindgen::convert::IntoWasmAbi for BufferSize {
    type Abi = <Option<FrameCount> as wasm_bindgen::convert::IntoWasmAbi>::Abi;

    fn into_abi(self) -> Self::Abi {
        match self {
            Self::Default => None,
            Self::Fixed(fc) => Some(fc),
        }
        .into_abi()
    }
}

#[cfg(all(
    target_arch = "wasm32",
    any(target_os = "emscripten", feature = "wasm-bindgen")
))]
impl wasm_bindgen::convert::FromWasmAbi for BufferSize {
    type Abi = <Option<FrameCount> as wasm_bindgen::convert::FromWasmAbi>::Abi;

    unsafe fn from_abi(js: Self::Abi) -> Self {
        match Option::<FrameCount>::from_abi(js) {
            None => Self::Default,
            Some(fc) => Self::Fixed(fc),
        }
    }
}

/// The set of parameters used to describe how to open a stream.
///
/// The sample format is omitted in favour of using a sample type.
///
/// See also [`BufferSize`] for details on buffer size behavior and latency considerations.
#[cfg_attr(
    all(
        target_arch = "wasm32",
        any(target_os = "emscripten", feature = "wasm-bindgen")
    ),
    wasm_bindgen
)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StreamConfig {
    pub channels: ChannelCount,
    pub sample_rate: SampleRate,
    pub buffer_size: BufferSize,
}

/// Describes the minimum and maximum supported buffer size for the device
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SupportedBufferSize {
    Range {
        min: FrameCount,
        max: FrameCount,
    },
    /// In the case that the platform provides no way of getting the default
    /// buffer size before starting a stream.
    Unknown,
}

/// Describes a range of supported stream configurations, retrieved via the
/// [`Device::supported_input/output_configs`](traits::DeviceTrait#required-methods) method.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SupportedStreamConfigRange {
    pub(crate) channels: ChannelCount,
    /// Minimum value for the sample rate of the supported formats.
    pub(crate) min_sample_rate: SampleRate,
    /// Maximum value for the sample rate of the supported formats.
    pub(crate) max_sample_rate: SampleRate,
    /// Buffer size ranges supported by the device
    pub(crate) buffer_size: SupportedBufferSize,
    /// Type of data expected by the device.
    pub(crate) sample_format: SampleFormat,
}

/// Common iterator types used by backend implementations.
///
/// All backends use these same concrete iterator types for supported stream configurations.
#[allow(dead_code)]
pub(crate) mod iter {
    use super::SupportedStreamConfigRange;

    /// Iterator type for supported input stream configurations.
    ///
    /// This is the iterator type returned by all backend implementations of
    /// [`DeviceTrait::supported_input_configs`](crate::traits::DeviceTrait::supported_input_configs).
    pub type SupportedInputConfigs = std::vec::IntoIter<SupportedStreamConfigRange>;

    /// Iterator type for supported output stream configurations.
    ///
    /// This is the iterator type returned by all backend implementations of
    /// [`DeviceTrait::supported_output_configs`](crate::traits::DeviceTrait::supported_output_configs).
    pub type SupportedOutputConfigs = std::vec::IntoIter<SupportedStreamConfigRange>;
}

/// Describes a single supported stream configuration, retrieved via either a
/// [`SupportedStreamConfigRange`] instance or one of the
/// [`Device::default_input/output_config`](traits::DeviceTrait#required-methods) methods.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SupportedStreamConfig {
    channels: ChannelCount,
    sample_rate: SampleRate,
    buffer_size: SupportedBufferSize,
    sample_format: SampleFormat,
}

/// A buffer of dynamically typed audio data, passed to raw stream callbacks.
///
/// Raw input stream callbacks receive `&Data`, while raw output stream callbacks expect `&mut Data`.
#[cfg_attr(target_os = "emscripten", wasm_bindgen)]
#[derive(Debug)]
pub struct Data {
    data: *mut (),
    len: usize,
    sample_format: SampleFormat,
}

/// A monotonic time instance associated with a stream, retrieved from either:
///
/// 1. A timestamp provided to the stream's underlying audio data callback or
/// 2. The same time source used to generate timestamps for a stream's underlying audio data
///    callback.
///
/// `StreamInstant` represents a duration since an unspecified origin point. The origin
/// is guaranteed to occur at or before the stream starts, and remains consistent for the
/// lifetime of that stream. Different streams may have different origins.
///
/// ## Host `StreamInstant` Sources
///
/// | Host | Source |
/// | ---- | ------ |
/// | alsa | `snd_pcm_status_get_htstamp` |
/// | coreaudio | `mach_absolute_time` |
/// | wasapi | `QueryPerformanceCounter` |
/// | asio | `timeGetTime` |
/// | emscripten | `AudioContext.getOutputTimestamp` |
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct StreamInstant {
    secs: i64,
    nanos: u32,
}

/// A timestamp associated with a call to an input stream's data callback.
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub struct InputStreamTimestamp {
    /// The instant the stream's data callback was invoked.
    pub callback: StreamInstant,
    /// The instant that data was captured from the device.
    ///
    /// E.g. The instant data was read from an ADC.
    pub capture: StreamInstant,
}

/// A timestamp associated with a call to an output stream's data callback.
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub struct OutputStreamTimestamp {
    /// The instant the stream's data callback was invoked.
    pub callback: StreamInstant,
    /// The predicted instant that data written will be delivered to the device for playback.
    ///
    /// E.g. The instant data will be played by a DAC.
    pub playback: StreamInstant,
}

/// Information relevant to a single call to the user's input stream data callback.
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub struct InputCallbackInfo {
    timestamp: InputStreamTimestamp,
}

/// Information relevant to a single call to the user's output stream data callback.
#[cfg_attr(target_os = "emscripten", wasm_bindgen)]
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub struct OutputCallbackInfo {
    timestamp: OutputStreamTimestamp,
}

impl SupportedStreamConfig {
    pub fn new(
        channels: ChannelCount,
        sample_rate: SampleRate,
        buffer_size: SupportedBufferSize,
        sample_format: SampleFormat,
    ) -> Self {
        Self {
            channels,
            sample_rate,
            buffer_size,
            sample_format,
        }
    }

    pub fn channels(&self) -> ChannelCount {
        self.channels
    }

    pub fn sample_rate(&self) -> SampleRate {
        self.sample_rate
    }

    pub fn buffer_size(&self) -> &SupportedBufferSize {
        &self.buffer_size
    }

    pub fn sample_format(&self) -> SampleFormat {
        self.sample_format
    }

    pub fn config(&self) -> StreamConfig {
        StreamConfig {
            channels: self.channels,
            sample_rate: self.sample_rate,
            buffer_size: BufferSize::Default,
        }
    }
}

impl StreamInstant {
    /// The amount of time elapsed from another instant to this one.
    ///
    /// Returns `None` if `earlier` is later than self.
    pub fn duration_since(&self, earlier: &Self) -> Option<Duration> {
        if self < earlier {
            None
        } else {
            (self.as_nanos() - earlier.as_nanos())
                .try_into()
                .ok()
                .map(Duration::from_nanos)
        }
    }

    /// Returns the instant in time after the given duration has passed.
    ///
    /// Returns `None` if the resulting instant would exceed the bounds of the underlying data
    /// structure.
    pub fn add(&self, duration: Duration) -> Option<Self> {
        self.as_nanos()
            .checked_add(duration.as_nanos() as i128)
            .and_then(Self::from_nanos_i128)
    }

    /// Returns the instant in time one `duration` ago.
    ///
    /// Returns `None` if the resulting instant would underflow. As a result, it is important to
    /// consider that on some platforms the [`StreamInstant`] may begin at `0` from the moment the
    /// source stream is created.
    pub fn sub(&self, duration: Duration) -> Option<Self> {
        self.as_nanos()
            .checked_sub(duration.as_nanos() as i128)
            .and_then(Self::from_nanos_i128)
    }

    fn as_nanos(&self) -> i128 {
        (self.secs as i128 * 1_000_000_000) + self.nanos as i128
    }

    #[allow(dead_code)]
    fn from_nanos(nanos: i64) -> Self {
        let secs = nanos / 1_000_000_000;
        let subsec_nanos = nanos - secs * 1_000_000_000;
        Self::new(secs, subsec_nanos as u32)
    }

    #[allow(dead_code)]
    fn from_nanos_i128(nanos: i128) -> Option<Self> {
        let secs = nanos / 1_000_000_000;
        if secs > i64::MAX as i128 || secs < i64::MIN as i128 {
            None
        } else {
            let subsec_nanos = nanos - secs * 1_000_000_000;
            debug_assert!(subsec_nanos < u32::MAX as i128);
            Some(Self::new(secs as i64, subsec_nanos as u32))
        }
    }

    #[allow(dead_code)]
    fn from_secs_f64(secs: f64) -> crate::StreamInstant {
        let s = secs.floor() as i64;
        let ns = ((secs - s as f64) * 1_000_000_000.0) as u32;
        Self::new(s, ns)
    }

    pub fn new(secs: i64, nanos: u32) -> Self {
        StreamInstant { secs, nanos }
    }
}

impl InputCallbackInfo {
    pub fn new(timestamp: InputStreamTimestamp) -> Self {
        Self { timestamp }
    }

    /// The timestamp associated with the call to an input stream's data callback.
    pub fn timestamp(&self) -> InputStreamTimestamp {
        self.timestamp
    }
}

impl OutputCallbackInfo {
    pub fn new(timestamp: OutputStreamTimestamp) -> Self {
        Self { timestamp }
    }

    /// The timestamp associated with the call to an output stream's data callback.
    pub fn timestamp(&self) -> OutputStreamTimestamp {
        self.timestamp
    }
}

// Note: Data does not implement `is_empty()` because it always contains a valid audio buffer
// by design. The buffer may contain silence, but it is never structurally empty.
#[allow(clippy::len_without_is_empty)]
impl Data {
    /// Constructor for host implementations to use.
    ///
    /// # Safety
    /// The following requirements must be met in order for the safety of `Data`'s API.
    /// - The `data` pointer must point to the first sample in the slice containing all samples.
    /// - The `len` must describe the length of the buffer as a number of samples in the expected
    ///   format specified via the `sample_format` argument.
    /// - The `sample_format` must correctly represent the underlying sample data delivered/expected
    ///   by the stream.
    pub unsafe fn from_parts(data: *mut (), len: usize, sample_format: SampleFormat) -> Self {
        Data {
            data,
            len,
            sample_format,
        }
    }

    /// The sample format of the internal audio data.
    pub fn sample_format(&self) -> SampleFormat {
        self.sample_format
    }

    /// The full length of the buffer in samples.
    ///
    /// The returned length is the same length as the slice of type `T` that would be returned via
    /// [`as_slice`](Self::as_slice) given a sample type that matches the inner sample format.
    pub fn len(&self) -> usize {
        self.len
    }

    /// The raw slice of memory representing the underlying audio data as a slice of bytes.
    ///
    /// It is up to the user to interpret the slice of memory based on [`Data::sample_format`].
    pub fn bytes(&self) -> &[u8] {
        let len = self.len * self.sample_format.sample_size();
        // The safety of this block relies on correct construction of the `Data` instance.
        // See the unsafe `from_parts` constructor for these requirements.
        unsafe { std::slice::from_raw_parts(self.data as *const u8, len) }
    }

    /// The raw slice of memory representing the underlying audio data as a slice of bytes.
    ///
    /// It is up to the user to interpret the slice of memory based on [`Data::sample_format`].
    pub fn bytes_mut(&mut self) -> &mut [u8] {
        let len = self.len * self.sample_format.sample_size();
        // The safety of this block relies on correct construction of the `Data` instance. See
        // the unsafe `from_parts` constructor for these requirements.
        unsafe { std::slice::from_raw_parts_mut(self.data as *mut u8, len) }
    }

    /// Access the data as a slice of sample type `T`.
    ///
    /// Returns `None` if the sample type does not match the expected sample format.
    pub fn as_slice<T>(&self) -> Option<&[T]>
    where
        T: SizedSample,
    {
        if T::FORMAT == self.sample_format {
            // The safety of this block relies on correct construction of the `Data` instance. See
            // the unsafe `from_parts` constructor for these requirements.
            unsafe { Some(std::slice::from_raw_parts(self.data as *const T, self.len)) }
        } else {
            None
        }
    }

    /// Access the data as a slice of sample type `T`.
    ///
    /// Returns `None` if the sample type does not match the expected sample format.
    pub fn as_slice_mut<T>(&mut self) -> Option<&mut [T]>
    where
        T: SizedSample,
    {
        if T::FORMAT == self.sample_format {
            // The safety of this block relies on correct construction of the `Data` instance. See
            // the unsafe `from_parts` constructor for these requirements.
            unsafe {
                Some(std::slice::from_raw_parts_mut(
                    self.data as *mut T,
                    self.len,
                ))
            }
        } else {
            None
        }
    }
}

impl SupportedStreamConfigRange {
    pub fn new(
        channels: ChannelCount,
        min_sample_rate: SampleRate,
        max_sample_rate: SampleRate,
        buffer_size: SupportedBufferSize,
        sample_format: SampleFormat,
    ) -> Self {
        Self {
            channels,
            min_sample_rate,
            max_sample_rate,
            buffer_size,
            sample_format,
        }
    }

    pub fn channels(&self) -> ChannelCount {
        self.channels
    }

    pub fn min_sample_rate(&self) -> SampleRate {
        self.min_sample_rate
    }

    pub fn max_sample_rate(&self) -> SampleRate {
        self.max_sample_rate
    }

    pub fn buffer_size(&self) -> &SupportedBufferSize {
        &self.buffer_size
    }

    pub fn sample_format(&self) -> SampleFormat {
        self.sample_format
    }

    /// Retrieve a [`SupportedStreamConfig`] with the given sample rate and buffer size.
    ///
    /// # Panics
    ///
    /// Panics if the given `sample_rate` is outside the range specified within
    /// this [`SupportedStreamConfigRange`] instance. For a non-panicking
    /// variant, use [`try_with_sample_rate`](#method.try_with_sample_rate).
    pub fn with_sample_rate(self, sample_rate: SampleRate) -> SupportedStreamConfig {
        self.try_with_sample_rate(sample_rate)
            .expect("sample rate out of range")
    }

    /// Retrieve a [`SupportedStreamConfig`] with the given sample rate and buffer size.
    ///
    /// Returns `None` if the given sample rate is outside the range specified
    /// within this [`SupportedStreamConfigRange`] instance.
    pub fn try_with_sample_rate(self, sample_rate: SampleRate) -> Option<SupportedStreamConfig> {
        if self.min_sample_rate <= sample_rate && sample_rate <= self.max_sample_rate {
            Some(SupportedStreamConfig {
                channels: self.channels,
                sample_rate,
                sample_format: self.sample_format,
                buffer_size: self.buffer_size,
            })
        } else {
            None
        }
    }

    /// Turns this [`SupportedStreamConfigRange`] into a [`SupportedStreamConfig`] corresponding to the maximum sample rate.
    #[inline]
    pub fn with_max_sample_rate(self) -> SupportedStreamConfig {
        SupportedStreamConfig {
            channels: self.channels,
            sample_rate: self.max_sample_rate,
            sample_format: self.sample_format,
            buffer_size: self.buffer_size,
        }
    }

    /// A comparison function which compares two [`SupportedStreamConfigRange`]s in terms of their priority of
    /// use as a default stream format.
    ///
    /// Some backends do not provide a default stream format for their audio devices. In these
    /// cases, CPAL attempts to decide on a reasonable default format for the user. To do this we
    /// use the "greatest" of all supported stream formats when compared with this method.
    ///
    /// SupportedStreamConfigs are prioritised by the following heuristics:
    ///
    /// **Channels**:
    ///
    /// - Stereo
    /// - Mono
    /// - Max available channels
    ///
    /// **Sample format**:
    /// - f32
    /// - i16
    /// - u16
    ///
    /// **Sample rate**:
    ///
    /// - 44100 (cd quality)
    /// - Max sample rate
    pub fn cmp_default_heuristics(&self, other: &Self) -> std::cmp::Ordering {
        use std::cmp::Ordering::Equal;
        use SampleFormat::{F32, I16, I24, I32, U16, U24, U32};

        let cmp_stereo = (self.channels == 2).cmp(&(other.channels == 2));
        if cmp_stereo != Equal {
            return cmp_stereo;
        }

        let cmp_mono = (self.channels == 1).cmp(&(other.channels == 1));
        if cmp_mono != Equal {
            return cmp_mono;
        }

        let cmp_channels = self.channels.cmp(&other.channels);
        if cmp_channels != Equal {
            return cmp_channels;
        }

        let cmp_f32 = (self.sample_format == F32).cmp(&(other.sample_format == F32));
        if cmp_f32 != Equal {
            return cmp_f32;
        }

        let cmp_i32 = (self.sample_format == I32).cmp(&(other.sample_format == I32));
        if cmp_i32 != Equal {
            return cmp_i32;
        }

        let cmp_u32 = (self.sample_format == U32).cmp(&(other.sample_format == U32));
        if cmp_u32 != Equal {
            return cmp_u32;
        }

        let cmp_i24 = (self.sample_format == I24).cmp(&(other.sample_format == I24));
        if cmp_i24 != Equal {
            return cmp_i24;
        }

        let cmp_u24 = (self.sample_format == U24).cmp(&(other.sample_format == U24));
        if cmp_u24 != Equal {
            return cmp_u24;
        }

        let cmp_i16 = (self.sample_format == I16).cmp(&(other.sample_format == I16));
        if cmp_i16 != Equal {
            return cmp_i16;
        }

        let cmp_u16 = (self.sample_format == U16).cmp(&(other.sample_format == U16));
        if cmp_u16 != Equal {
            return cmp_u16;
        }

        const HZ_44100: SampleRate = 44_100;
        let r44100_in_self = self.min_sample_rate <= HZ_44100 && HZ_44100 <= self.max_sample_rate;
        let r44100_in_other =
            other.min_sample_rate <= HZ_44100 && HZ_44100 <= other.max_sample_rate;
        let cmp_r44100 = r44100_in_self.cmp(&r44100_in_other);
        if cmp_r44100 != Equal {
            return cmp_r44100;
        }

        self.max_sample_rate.cmp(&other.max_sample_rate)
    }
}

#[test]
fn test_cmp_default_heuristics() {
    let mut formats = [
        SupportedStreamConfigRange {
            buffer_size: SupportedBufferSize::Range { min: 256, max: 512 },
            channels: 2,
            min_sample_rate: 1,
            max_sample_rate: 96000,
            sample_format: SampleFormat::F32,
        },
        SupportedStreamConfigRange {
            buffer_size: SupportedBufferSize::Range { min: 256, max: 512 },
            channels: 1,
            min_sample_rate: 1,
            max_sample_rate: 96000,
            sample_format: SampleFormat::F32,
        },
        SupportedStreamConfigRange {
            buffer_size: SupportedBufferSize::Range { min: 256, max: 512 },
            channels: 2,
            min_sample_rate: 1,
            max_sample_rate: 96000,
            sample_format: SampleFormat::I16,
        },
        SupportedStreamConfigRange {
            buffer_size: SupportedBufferSize::Range { min: 256, max: 512 },
            channels: 2,
            min_sample_rate: 1,
            max_sample_rate: 96000,
            sample_format: SampleFormat::U16,
        },
        SupportedStreamConfigRange {
            buffer_size: SupportedBufferSize::Range { min: 256, max: 512 },
            channels: 2,
            min_sample_rate: 1,
            max_sample_rate: 22050,
            sample_format: SampleFormat::F32,
        },
    ];

    formats.sort_by(|a, b| a.cmp_default_heuristics(b));

    // lowest-priority first:
    assert_eq!(formats[0].sample_format(), SampleFormat::F32);
    assert_eq!(formats[0].min_sample_rate(), 1);
    assert_eq!(formats[0].max_sample_rate(), 96000);
    assert_eq!(formats[0].channels(), 1);

    assert_eq!(formats[1].sample_format(), SampleFormat::U16);
    assert_eq!(formats[1].min_sample_rate(), 1);
    assert_eq!(formats[1].max_sample_rate(), 96000);
    assert_eq!(formats[1].channels(), 2);

    assert_eq!(formats[2].sample_format(), SampleFormat::I16);
    assert_eq!(formats[2].min_sample_rate(), 1);
    assert_eq!(formats[2].max_sample_rate(), 96000);
    assert_eq!(formats[2].channels(), 2);

    assert_eq!(formats[3].sample_format(), SampleFormat::F32);
    assert_eq!(formats[3].min_sample_rate(), 1);
    assert_eq!(formats[3].max_sample_rate(), 22050);
    assert_eq!(formats[3].channels(), 2);

    assert_eq!(formats[4].sample_format(), SampleFormat::F32);
    assert_eq!(formats[4].min_sample_rate(), 1);
    assert_eq!(formats[4].max_sample_rate(), 96000);
    assert_eq!(formats[4].channels(), 2);
}

impl From<SupportedStreamConfig> for StreamConfig {
    fn from(conf: SupportedStreamConfig) -> Self {
        conf.config()
    }
}

// If a backend does not provide an API for retrieving supported formats, we query it with a bunch
// of commonly used rates. This is always the case for WASAPI and is sometimes the case for ALSA.
#[allow(dead_code)]
pub(crate) const COMMON_SAMPLE_RATES: &[SampleRate] = &[
    5512, 8000, 11025, 12000, 16000, 22050, 24000, 32000, 44100, 48000, 64000, 88200, 96000,
    176400, 192000, 352800, 384000,
];

#[test]
fn test_stream_instant() {
    let a = StreamInstant::new(2, 0);
    let b = StreamInstant::new(-2, 0);
    let min = StreamInstant::new(i64::MIN, 0);
    let max = StreamInstant::new(i64::MAX, 0);
    assert_eq!(
        a.sub(Duration::from_secs(1)),
        Some(StreamInstant::new(1, 0))
    );
    assert_eq!(
        a.sub(Duration::from_secs(2)),
        Some(StreamInstant::new(0, 0))
    );
    assert_eq!(
        a.sub(Duration::from_secs(3)),
        Some(StreamInstant::new(-1, 0))
    );
    assert_eq!(min.sub(Duration::from_secs(1)), None);
    assert_eq!(
        b.add(Duration::from_secs(1)),
        Some(StreamInstant::new(-1, 0))
    );
    assert_eq!(
        b.add(Duration::from_secs(2)),
        Some(StreamInstant::new(0, 0))
    );
    assert_eq!(
        b.add(Duration::from_secs(3)),
        Some(StreamInstant::new(1, 0))
    );
    assert_eq!(max.add(Duration::from_secs(1)), None);
}
