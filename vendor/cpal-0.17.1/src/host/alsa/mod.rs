//! ALSA backend implementation.
//!
//! Default backend on Linux and BSD systems.

extern crate alsa;
extern crate libc;

use std::{
    cmp,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
    },
    thread::{self, JoinHandle},
    time::Duration,
    vec::IntoIter as VecIntoIter,
};

use self::alsa::poll::Descriptors;
pub use self::enumerate::Devices;

use crate::{
    iter::{SupportedInputConfigs, SupportedOutputConfigs},
    traits::{DeviceTrait, HostTrait, StreamTrait},
    BackendSpecificError, BufferSize, BuildStreamError, ChannelCount, Data,
    DefaultStreamConfigError, DeviceDescription, DeviceDescriptionBuilder, DeviceDirection,
    DeviceId, DeviceIdError, DeviceNameError, DevicesError, FrameCount, InputCallbackInfo,
    OutputCallbackInfo, PauseStreamError, PlayStreamError, Sample, SampleFormat, SampleRate,
    StreamConfig, StreamError, SupportedBufferSize, SupportedStreamConfig,
    SupportedStreamConfigRange, SupportedStreamConfigsError, I24, U24,
};

mod enumerate;

// ALSA Buffer Size Behavior
// =========================
//
// ## ALSA Latency Model
//
// **Hardware vs Software Buffer**: ALSA maintains a software buffer in memory that feeds
// a hardware buffer in the audio device. Audio latency is determined by how much data
// sits in the software buffer before being transferred to hardware.
//
// **Period-Based Transfer**: ALSA transfers data in chunks called "periods". When one
// period worth of data has been consumed by hardware, ALSA triggers a callback to refill
// that period in the software buffer.
//
// ## BufferSize::Fixed Behavior
//
// When `BufferSize::Fixed(x)` is specified, cpal attempts to configure the period size
// to approximately `x` frames to achieve the requested callback size. However, the
// actual callback size may differ from the request:
//
// - ALSA may round the period size to hardware-supported values
// - Different devices have different period size constraints
// - The callback size is not guaranteed to exactly match the request
// - If the requested size cannot be accommodated, ALSA will choose the nearest
//   supported configuration
//
// This mirrors the behavior documented in the cpal API where `BufferSize::Fixed(x)`
// requests but does not guarantee a specific callback size.
//
// ## BufferSize::Default Behavior
//
// When `BufferSize::Default` is specified, cpal does NOT set explicit period size or
// period count constraints, allowing the device/driver to choose sensible defaults.
//
// **Why not set defaults?** Different audio systems have different behaviors:
//
// - **Native ALSA hardware**: Typically chooses reasonable defaults (e.g., 512-2048
//   frame periods with 2-4 periods)
//
// - **PipeWire-ALSA plugin**: Allocates a large ring buffer (~1M frames at 48kHz) but
//   uses small periods (512-1024 frames). Critically, if you request `set_periods(2)`
//   without specifying period size, PipeWire calculates period = buffer/2, resulting
//   in pathologically large periods (~524K frames = 10 seconds). See issues #1029 and
//   #1036.
//
// By not constraining period configuration, PipeWire-ALSA can use its optimized defaults
// (small periods with many-period buffer), while native ALSA hardware uses its own defaults.
//
// **Startup latency**: Regardless of buffer size, cpal uses double-buffering for startup
// (start_threshold = 2 periods), ensuring low latency even with large multi-period ring
// buffers.

const DEFAULT_DEVICE: &str = "default";

/// The default Linux and BSD host type.
#[derive(Debug)]
pub struct Host;

impl Host {
    pub fn new() -> Result<Self, crate::HostUnavailable> {
        Ok(Host)
    }
}

impl HostTrait for Host {
    type Devices = Devices;
    type Device = Device;

    fn is_available() -> bool {
        // Assume ALSA is always available on Linux and BSD.
        true
    }

    fn devices(&self) -> Result<Self::Devices, DevicesError> {
        enumerate::devices()
    }

    fn default_input_device(&self) -> Option<Self::Device> {
        Some(Device::default())
    }

    fn default_output_device(&self) -> Option<Self::Device> {
        Some(Device::default())
    }
}

impl DeviceTrait for Device {
    type SupportedInputConfigs = SupportedInputConfigs;
    type SupportedOutputConfigs = SupportedOutputConfigs;
    type Stream = Stream;

    // ALSA overrides name() to return pcm_id directly instead of from description
    fn name(&self) -> Result<String, DeviceNameError> {
        Device::name(self)
    }

    fn description(&self) -> Result<DeviceDescription, DeviceNameError> {
        Device::description(self)
    }

    fn id(&self) -> Result<DeviceId, DeviceIdError> {
        Device::id(self)
    }

    // Override trait defaults to avoid opening devices during enumeration.
    //
    // ALSA does not guarantee transactional cleanup on failed snd_pcm_open(). Opening plugins like
    // alsaequal that fail with EPERM can leak FDs, poisoning the ALSA backend for the process
    // lifetime (subsequent device opens fail with EBUSY until process exit).
    fn supports_input(&self) -> bool {
        matches!(
            self.direction,
            DeviceDirection::Input | DeviceDirection::Duplex
        )
    }

    fn supports_output(&self) -> bool {
        matches!(
            self.direction,
            DeviceDirection::Output | DeviceDirection::Duplex
        )
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
        conf: &StreamConfig,
        sample_format: SampleFormat,
        data_callback: D,
        error_callback: E,
        timeout: Option<Duration>,
    ) -> Result<Self::Stream, BuildStreamError>
    where
        D: FnMut(&Data, &InputCallbackInfo) + Send + 'static,
        E: FnMut(StreamError) + Send + 'static,
    {
        let stream_inner =
            self.build_stream_inner(conf, sample_format, alsa::Direction::Capture)?;
        let stream = Self::Stream::new_input(
            Arc::new(stream_inner),
            data_callback,
            error_callback,
            timeout,
        );
        Ok(stream)
    }

    fn build_output_stream_raw<D, E>(
        &self,
        conf: &StreamConfig,
        sample_format: SampleFormat,
        data_callback: D,
        error_callback: E,
        timeout: Option<Duration>,
    ) -> Result<Self::Stream, BuildStreamError>
    where
        D: FnMut(&mut Data, &OutputCallbackInfo) + Send + 'static,
        E: FnMut(StreamError) + Send + 'static,
    {
        let stream_inner =
            self.build_stream_inner(conf, sample_format, alsa::Direction::Playback)?;
        let stream = Self::Stream::new_output(
            Arc::new(stream_inner),
            data_callback,
            error_callback,
            timeout,
        );
        Ok(stream)
    }
}

struct TriggerSender(libc::c_int);

struct TriggerReceiver(libc::c_int);

impl TriggerSender {
    fn wakeup(&self) {
        let buf = 1u64;
        let ret = unsafe { libc::write(self.0, &buf as *const u64 as *const _, 8) };
        assert_eq!(ret, 8);
    }
}

impl TriggerReceiver {
    fn clear_pipe(&self) {
        let mut out = 0u64;
        let ret = unsafe { libc::read(self.0, &mut out as *mut u64 as *mut _, 8) };
        assert_eq!(ret, 8);
    }
}

fn trigger() -> (TriggerSender, TriggerReceiver) {
    let mut fds = [0, 0];
    match unsafe { libc::pipe(fds.as_mut_ptr()) } {
        0 => (TriggerSender(fds[1]), TriggerReceiver(fds[0])),
        _ => panic!("Could not create pipe"),
    }
}

impl Drop for TriggerSender {
    fn drop(&mut self) {
        unsafe {
            libc::close(self.0);
        }
    }
}

impl Drop for TriggerReceiver {
    fn drop(&mut self) {
        unsafe {
            libc::close(self.0);
        }
    }
}

#[derive(Default)]
struct DeviceHandles {
    playback: Option<alsa::PCM>,
    capture: Option<alsa::PCM>,
}

impl DeviceHandles {
    /// Get a mutable reference to the `Option` for a specific `stream_type`.
    /// If the `Option` is `None`, the `alsa::PCM` will be opened and placed in
    /// the `Option` before returning. If `try_open()` returns `Ok` the contained
    /// `Option` is guaranteed to be `Some(..)`.
    fn try_open(
        &mut self,
        pcm_id: &str,
        stream_type: alsa::Direction,
    ) -> Result<&mut Option<alsa::PCM>, alsa::Error> {
        let handle = match stream_type {
            alsa::Direction::Playback => &mut self.playback,
            alsa::Direction::Capture => &mut self.capture,
        };

        if handle.is_none() {
            *handle = Some(alsa::pcm::PCM::new(pcm_id, stream_type, true)?);
        }

        Ok(handle)
    }

    /// Get a mutable reference to the `alsa::PCM` handle for a specific `stream_type`.
    /// If the handle is not yet opened, it will be opened and stored in `self`.
    fn get_mut(
        &mut self,
        pcm_id: &str,
        stream_type: alsa::Direction,
    ) -> Result<&mut alsa::PCM, alsa::Error> {
        Ok(self.try_open(pcm_id, stream_type)?.as_mut().unwrap())
    }

    /// Take ownership of the `alsa::PCM` handle for a specific `stream_type`.
    /// If the handle is not yet opened, it will be opened and returned.
    fn take(&mut self, name: &str, stream_type: alsa::Direction) -> Result<alsa::PCM, alsa::Error> {
        Ok(self.try_open(name, stream_type)?.take().unwrap())
    }
}

#[derive(Clone)]
pub struct Device {
    pcm_id: String,
    desc: Option<String>,
    direction: DeviceDirection,
    handles: Arc<Mutex<DeviceHandles>>,
}

impl PartialEq for Device {
    fn eq(&self, other: &Self) -> bool {
        self.pcm_id == other.pcm_id
    }
}

impl Eq for Device {}

impl std::hash::Hash for Device {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.pcm_id.hash(state);
    }
}

impl Device {
    fn build_stream_inner(
        &self,
        conf: &StreamConfig,
        sample_format: SampleFormat,
        stream_type: alsa::Direction,
    ) -> Result<StreamInner, BuildStreamError> {
        // Validate buffer size if Fixed is specified. This is necessary because
        // `set_period_size_near()` with `ValueOr::Nearest` will accept ANY value and return the
        // "nearest" supported value, which could be wildly different (e.g., requesting 4096 frames
        // might return 512 frames if that's "nearest").
        if let BufferSize::Fixed(requested_size) = conf.buffer_size {
            // Note: We use `default_input_config`/`default_output_config` to get the buffer size
            // range. This queries the CURRENT device (`self.pcm_id`), not the default device. The
            // buffer size range is the same across all format configurations for a given device
            // (see `supported_configs()`).
            let supported_config = match stream_type {
                alsa::Direction::Capture => self.default_input_config(),
                alsa::Direction::Playback => self.default_output_config(),
            };
            if let Ok(config) = supported_config {
                if let SupportedBufferSize::Range { min, max } = config.buffer_size {
                    if !(min..=max).contains(&requested_size) {
                        return Err(BuildStreamError::StreamConfigNotSupported);
                    }
                }
            }
        }

        let handle = match self
            .handles
            .lock()
            .unwrap()
            .take(&self.pcm_id, stream_type)
            .map_err(|e| (e, e.errno()))
        {
            Err((_, libc::ENOENT))
            | Err((_, libc::EBUSY))
            | Err((_, libc::EPERM))
            | Err((_, libc::EAGAIN)) => return Err(BuildStreamError::DeviceNotAvailable),
            Err((_, libc::EINVAL)) => return Err(BuildStreamError::InvalidArgument),
            Err((e, _)) => return Err(e.into()),
            Ok(handle) => handle,
        };

        let can_pause = set_hw_params_from_format(&handle, conf, sample_format)?;
        let period_samples = set_sw_params_from_format(&handle, conf, stream_type)?;

        handle.prepare()?;

        let num_descriptors = handle.count();
        if num_descriptors == 0 {
            let description = "poll descriptor count for stream was 0".to_string();
            let err = BackendSpecificError { description };
            return Err(err.into());
        }

        // Check to see if we can retrieve valid timestamps from the device.
        // Related: https://bugs.freedesktop.org/show_bug.cgi?id=88503
        let ts = handle.status()?.get_htstamp();
        let creation_instant = match (ts.tv_sec, ts.tv_nsec) {
            (0, 0) => Some(std::time::Instant::now()),
            _ => None,
        };

        if let alsa::Direction::Capture = stream_type {
            handle.start()?;
        }

        // Pre-compute a period-sized buffer filled with silence values.
        let period_frames = period_samples / conf.channels as usize;
        let period_bytes = period_samples * sample_format.sample_size();
        let mut silence_template = vec![0u8; period_bytes].into_boxed_slice();

        // Only fill buffer for unsigned formats that don't have a zero value for silence.
        if sample_format.is_uint() {
            fill_with_equilibrium(&mut silence_template, sample_format);
        }

        let stream_inner = StreamInner {
            dropping: AtomicBool::new(false),
            channel: handle,
            sample_format,
            num_descriptors,
            conf: conf.clone(),
            period_samples,
            period_frames,
            silence_template,
            can_pause,
            creation_instant,
        };

        Ok(stream_inner)
    }

    fn name(&self) -> Result<String, DeviceNameError> {
        Ok(self.pcm_id.clone())
    }

    fn description(&self) -> Result<DeviceDescription, DeviceNameError> {
        let name = self
            .desc
            .as_ref()
            .and_then(|desc| desc.lines().next())
            .unwrap_or(&self.pcm_id)
            .to_string();

        let mut builder = DeviceDescriptionBuilder::new(name)
            .driver(self.pcm_id.clone())
            .direction(self.direction);

        if let Some(ref desc) = self.desc {
            let lines = desc
                .lines()
                .map(|line| line.trim().to_string())
                .filter(|line| !line.is_empty())
                .collect();
            builder = builder.extended(lines);
        }

        Ok(builder.build())
    }

    fn id(&self) -> Result<DeviceId, DeviceIdError> {
        Ok(DeviceId(crate::platform::HostId::Alsa, self.pcm_id.clone()))
    }

    fn supported_configs(
        &self,
        stream_t: alsa::Direction,
    ) -> Result<VecIntoIter<SupportedStreamConfigRange>, SupportedStreamConfigsError> {
        // Open device handle and cache it for reuse in build_stream_inner().
        // This avoids opening the device twice in the common workflow:
        // 1. Query supported configs (opens and caches handle)
        // 2. Build stream (takes cached handle, or opens if not cached)
        let mut guard = self.handles.lock().unwrap();
        let pcm = match guard
            .get_mut(&self.pcm_id, stream_t)
            .map_err(|e| (e, e.errno()))
        {
            Err((_, libc::ENOENT))
            | Err((_, libc::EBUSY))
            | Err((_, libc::EPERM))
            | Err((_, libc::EAGAIN)) => {
                return Err(SupportedStreamConfigsError::DeviceNotAvailable)
            }
            Err((_, libc::EINVAL)) => return Err(SupportedStreamConfigsError::InvalidArgument),
            Err((e, _)) => return Err(e.into()),
            Ok(pcm) => pcm,
        };

        let hw_params = alsa::pcm::HwParams::any(pcm)?;

        // Test both LE and BE formats to detect what the hardware actually supports.
        // LE is listed first as it's the common case for most audio hardware.
        // Hardware reports its supported formats regardless of CPU endianness.
        const FORMATS: [(SampleFormat, alsa::pcm::Format); 18] = [
            (SampleFormat::I8, alsa::pcm::Format::S8),
            (SampleFormat::U8, alsa::pcm::Format::U8),
            (SampleFormat::I16, alsa::pcm::Format::S16LE),
            (SampleFormat::I16, alsa::pcm::Format::S16BE),
            (SampleFormat::U16, alsa::pcm::Format::U16LE),
            (SampleFormat::U16, alsa::pcm::Format::U16BE),
            (SampleFormat::I24, alsa::pcm::Format::S24LE),
            (SampleFormat::I24, alsa::pcm::Format::S24BE),
            (SampleFormat::U24, alsa::pcm::Format::U24LE),
            (SampleFormat::U24, alsa::pcm::Format::U24BE),
            (SampleFormat::I32, alsa::pcm::Format::S32LE),
            (SampleFormat::I32, alsa::pcm::Format::S32BE),
            (SampleFormat::U32, alsa::pcm::Format::U32LE),
            (SampleFormat::U32, alsa::pcm::Format::U32BE),
            (SampleFormat::F32, alsa::pcm::Format::FloatLE),
            (SampleFormat::F32, alsa::pcm::Format::FloatBE),
            (SampleFormat::F64, alsa::pcm::Format::Float64LE),
            (SampleFormat::F64, alsa::pcm::Format::Float64BE),
            //SND_PCM_FORMAT_IEC958_SUBFRAME_LE,
            //SND_PCM_FORMAT_IEC958_SUBFRAME_BE,
            //SND_PCM_FORMAT_MU_LAW,
            //SND_PCM_FORMAT_A_LAW,
            //SND_PCM_FORMAT_IMA_ADPCM,
            //SND_PCM_FORMAT_MPEG,
            //SND_PCM_FORMAT_GSM,
            //SND_PCM_FORMAT_SPECIAL,
            //SND_PCM_FORMAT_S24_3LE,
            //SND_PCM_FORMAT_S24_3BE,
            //SND_PCM_FORMAT_U24_3LE,
            //SND_PCM_FORMAT_U24_3BE,
            //SND_PCM_FORMAT_S20_3LE,
            //SND_PCM_FORMAT_S20_3BE,
            //SND_PCM_FORMAT_U20_3LE,
            //SND_PCM_FORMAT_U20_3BE,
            //SND_PCM_FORMAT_S18_3LE,
            //SND_PCM_FORMAT_S18_3BE,
            //SND_PCM_FORMAT_U18_3LE,
            //SND_PCM_FORMAT_U18_3BE,
        ];

        // Collect supported formats, deduplicating since we test both LE and BE variants.
        // If hardware supports both endiannesses (rare), we only report the format once.
        let mut supported_formats = Vec::new();
        for &(sample_format, alsa_format) in FORMATS.iter() {
            if hw_params.test_format(alsa_format).is_ok()
                && !supported_formats.contains(&sample_format)
            {
                supported_formats.push(sample_format);
            }
        }

        let min_rate = hw_params.get_rate_min()?;
        let max_rate = hw_params.get_rate_max()?;

        let sample_rates = if min_rate == max_rate || hw_params.test_rate(min_rate + 1).is_ok() {
            vec![(min_rate, max_rate)]
        } else {
            let mut rates = Vec::new();
            for &sample_rate in crate::COMMON_SAMPLE_RATES.iter() {
                if hw_params.test_rate(sample_rate).is_ok() {
                    rates.push((sample_rate, sample_rate));
                }
            }

            if rates.is_empty() {
                vec![(min_rate, max_rate)]
            } else {
                rates
            }
        };

        let min_channels = hw_params.get_channels_min()?;
        let max_channels = hw_params.get_channels_max()?;

        let max_channels = cmp::min(max_channels, 32); // TODO: limiting to 32 channels or too much stuff is returned
        let supported_channels = (min_channels..max_channels + 1)
            .filter_map(|num| {
                if hw_params.test_channels(num).is_ok() {
                    Some(num as ChannelCount)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        let (min_buffer_size, max_buffer_size) = hw_params_buffer_size_min_max(&hw_params);
        let buffer_size_range = SupportedBufferSize::Range {
            min: min_buffer_size,
            max: max_buffer_size,
        };

        let mut output = Vec::with_capacity(
            supported_formats.len() * supported_channels.len() * sample_rates.len(),
        );
        for &sample_format in supported_formats.iter() {
            for &channels in supported_channels.iter() {
                for &(min_rate, max_rate) in sample_rates.iter() {
                    output.push(SupportedStreamConfigRange {
                        channels,
                        min_sample_rate: min_rate,
                        max_sample_rate: max_rate,
                        buffer_size: buffer_size_range,
                        sample_format,
                    });
                }
            }
        }

        Ok(output.into_iter())
    }

    fn supported_input_configs(
        &self,
    ) -> Result<SupportedInputConfigs, SupportedStreamConfigsError> {
        self.supported_configs(alsa::Direction::Capture)
    }

    fn supported_output_configs(
        &self,
    ) -> Result<SupportedOutputConfigs, SupportedStreamConfigsError> {
        self.supported_configs(alsa::Direction::Playback)
    }

    // ALSA does not offer default stream formats, so instead we compare all supported formats by
    // the `SupportedStreamConfigRange::cmp_default_heuristics` order and select the greatest.
    fn default_config(
        &self,
        stream_t: alsa::Direction,
    ) -> Result<SupportedStreamConfig, DefaultStreamConfigError> {
        let mut formats: Vec<_> = {
            match self.supported_configs(stream_t) {
                Err(SupportedStreamConfigsError::DeviceNotAvailable) => {
                    return Err(DefaultStreamConfigError::DeviceNotAvailable);
                }
                Err(SupportedStreamConfigsError::InvalidArgument) => {
                    // this happens sometimes when querying for input and output capabilities, but
                    // the device supports only one
                    return Err(DefaultStreamConfigError::StreamTypeNotSupported);
                }
                Err(SupportedStreamConfigsError::BackendSpecific { err }) => {
                    return Err(err.into());
                }
                Ok(fmts) => fmts.collect(),
            }
        };

        formats.sort_by(|a, b| a.cmp_default_heuristics(b));

        match formats.into_iter().next_back() {
            Some(f) => {
                let min_r = f.min_sample_rate;
                let max_r = f.max_sample_rate;
                let mut format = f.with_max_sample_rate();
                const HZ_44100: SampleRate = 44_100;
                if min_r <= HZ_44100 && HZ_44100 <= max_r {
                    format.sample_rate = HZ_44100;
                }
                Ok(format)
            }
            None => Err(DefaultStreamConfigError::StreamTypeNotSupported),
        }
    }

    fn default_input_config(&self) -> Result<SupportedStreamConfig, DefaultStreamConfigError> {
        self.default_config(alsa::Direction::Capture)
    }

    fn default_output_config(&self) -> Result<SupportedStreamConfig, DefaultStreamConfigError> {
        self.default_config(alsa::Direction::Playback)
    }
}

impl Default for Device {
    fn default() -> Self {
        // "default" is a virtual ALSA device that redirects to the configured default. We cannot
        // determine its actual capabilities without opening it, so we return Unknown direction.
        Self {
            pcm_id: DEFAULT_DEVICE.to_owned(),
            desc: Some("Default Audio Device".to_string()),
            direction: DeviceDirection::Unknown,
            handles: Arc::new(Mutex::new(Default::default())),
        }
    }
}

struct StreamInner {
    // Flag used to check when to stop polling, regardless of the state of the stream
    // (e.g. broken due to a disconnected device).
    dropping: AtomicBool,

    // The ALSA channel.
    channel: alsa::pcm::PCM,

    // When converting between file descriptors and `snd_pcm_t`, this is the number of
    // file descriptors that this `snd_pcm_t` uses.
    num_descriptors: usize,

    // Format of the samples.
    sample_format: SampleFormat,

    // The configuration used to open this stream.
    conf: StreamConfig,

    // Cached values for performance in audio callback hot path
    period_samples: usize,
    period_frames: usize,
    silence_template: Box<[u8]>,

    #[allow(dead_code)]
    // Whether or not the hardware supports pausing the stream.
    // TODO: We need an API to expose this. See #197, #284.
    can_pause: bool,

    // In the case that the device does not return valid timestamps via `get_htstamp`, this field
    // will be `Some` and will contain an `Instant` representing the moment the stream was created.
    //
    // If this field is `Some`, then the stream will use the duration since this instant as a
    // source for timestamps.
    //
    // If this field is `None` then the elapsed duration between `get_trigger_htstamp` and
    // `get_htstamp` is used.
    creation_instant: Option<std::time::Instant>,
}

// Assume that the ALSA library is built with thread safe option.
unsafe impl Sync for StreamInner {}

pub struct Stream {
    /// The high-priority audio processing thread calling callbacks.
    /// Option used for moving out in destructor.
    thread: Option<JoinHandle<()>>,

    /// Handle to the underlying stream for playback controls.
    inner: Arc<StreamInner>,

    /// Used to signal to stop processing.
    trigger: TriggerSender,
}

// Compile-time assertion that Stream is Send and Sync
crate::assert_stream_send!(Stream);
crate::assert_stream_sync!(Stream);

struct StreamWorkerContext {
    descriptors: Box<[libc::pollfd]>,
    transfer_buffer: Box<[u8]>,
    poll_timeout: i32,
}

impl StreamWorkerContext {
    fn new(poll_timeout: &Option<Duration>, stream: &StreamInner, rx: &TriggerReceiver) -> Self {
        let poll_timeout: i32 = if let Some(d) = poll_timeout {
            d.as_millis().try_into().unwrap()
        } else {
            -1 // Don't timeout, wait forever.
        };

        // Pre-allocate buffer to exactly one period size with proper equilibrium values.
        let transfer_buffer = stream.silence_template.clone();

        // Pre-allocate and initialize descriptors vector: 1 for self-pipe + stream.num_descriptors
        // for ALSA. The descriptor count is constant for the lifetime of stream parameters, and
        // poll() overwrites revents on each call, so we only need to set up fd and events once.
        let total_descriptors = 1 + stream.num_descriptors;
        let mut descriptors = vec![
            libc::pollfd {
                fd: 0,
                events: 0,
                revents: 0
            };
            total_descriptors
        ]
        .into_boxed_slice();

        // Set up self-pipe descriptor at index 0
        descriptors[0] = libc::pollfd {
            fd: rx.0,
            events: libc::POLLIN,
            revents: 0,
        };

        // Set up ALSA descriptors starting at index 1
        let filled = stream
            .channel
            .fill(&mut descriptors[1..])
            .expect("Failed to fill ALSA descriptors");
        debug_assert_eq!(filled, stream.num_descriptors);

        Self {
            descriptors,
            transfer_buffer,
            poll_timeout,
        }
    }
}

fn input_stream_worker(
    rx: TriggerReceiver,
    stream: &StreamInner,
    data_callback: &mut (dyn FnMut(&Data, &InputCallbackInfo) + Send + 'static),
    error_callback: &mut (dyn FnMut(StreamError) + Send + 'static),
    timeout: Option<Duration>,
) {
    boost_current_thread_priority(stream.conf.buffer_size, stream.conf.sample_rate);

    let mut ctxt = StreamWorkerContext::new(&timeout, stream, &rx);
    loop {
        let flow =
            poll_descriptors_and_prepare_buffer(&rx, stream, &mut ctxt).unwrap_or_else(|err| {
                error_callback(err.into());
                PollDescriptorsFlow::Continue
            });

        match flow {
            PollDescriptorsFlow::Continue => {
                continue;
            }
            PollDescriptorsFlow::XRun => {
                error_callback(StreamError::BufferUnderrun);
                if let Err(err) = stream.channel.prepare() {
                    error_callback(err.into());
                }
                continue;
            }
            PollDescriptorsFlow::Return => return,
            PollDescriptorsFlow::Ready {
                status,
                delay_frames,
            } => {
                if let Err(err) = process_input(
                    stream,
                    &mut ctxt.transfer_buffer,
                    status,
                    delay_frames,
                    data_callback,
                ) {
                    error_callback(err.into());
                }
            }
        }
    }
}

fn output_stream_worker(
    rx: TriggerReceiver,
    stream: &StreamInner,
    data_callback: &mut (dyn FnMut(&mut Data, &OutputCallbackInfo) + Send + 'static),
    error_callback: &mut (dyn FnMut(StreamError) + Send + 'static),
    timeout: Option<Duration>,
) {
    boost_current_thread_priority(stream.conf.buffer_size, stream.conf.sample_rate);

    let mut ctxt = StreamWorkerContext::new(&timeout, stream, &rx);

    loop {
        let flow =
            poll_descriptors_and_prepare_buffer(&rx, stream, &mut ctxt).unwrap_or_else(|err| {
                error_callback(err.into());
                PollDescriptorsFlow::Continue
            });

        match flow {
            PollDescriptorsFlow::Continue => continue,
            PollDescriptorsFlow::XRun => {
                error_callback(StreamError::BufferUnderrun);
                if let Err(err) = stream.channel.prepare() {
                    error_callback(err.into());
                }
                continue;
            }
            PollDescriptorsFlow::Return => return,
            PollDescriptorsFlow::Ready {
                status,
                delay_frames,
            } => {
                if let Err(err) = process_output(
                    stream,
                    &mut ctxt.transfer_buffer,
                    status,
                    delay_frames,
                    data_callback,
                    error_callback,
                ) {
                    error_callback(err.into());
                }
            }
        }
    }
}

#[cfg(feature = "audio_thread_priority")]
fn boost_current_thread_priority(buffer_size: BufferSize, sample_rate: SampleRate) {
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
fn boost_current_thread_priority(_: BufferSize, _: SampleRate) {}

enum PollDescriptorsFlow {
    Continue,
    Return,
    Ready {
        status: alsa::pcm::Status,
        delay_frames: usize,
    },
    XRun,
}

// This block is shared between both input and output stream worker functions.
fn poll_descriptors_and_prepare_buffer(
    rx: &TriggerReceiver,
    stream: &StreamInner,
    ctxt: &mut StreamWorkerContext,
) -> Result<PollDescriptorsFlow, BackendSpecificError> {
    if stream.dropping.load(Ordering::Acquire) {
        // The stream has been requested to be destroyed.
        rx.clear_pipe();
        return Ok(PollDescriptorsFlow::Return);
    }

    let StreamWorkerContext {
        ref mut descriptors,
        ref poll_timeout,
        ..
    } = *ctxt;

    let res = alsa::poll::poll(descriptors, *poll_timeout)?;
    if res == 0 {
        let description = String::from("`alsa::poll()` spuriously returned");
        return Err(BackendSpecificError { description });
    }

    if descriptors[0].revents != 0 {
        // The stream has been requested to be destroyed.
        rx.clear_pipe();
        return Ok(PollDescriptorsFlow::Return);
    }

    let revents = stream.channel.revents(&descriptors[1..])?;
    if revents.contains(alsa::poll::Flags::ERR) {
        let description = String::from("`alsa::poll()` returned POLLERR");
        return Err(BackendSpecificError { description });
    }

    // Check if data is ready for processing (either input or output)
    if !revents.contains(alsa::poll::Flags::IN) && !revents.contains(alsa::poll::Flags::OUT) {
        // Nothing to process, poll again
        return Ok(PollDescriptorsFlow::Continue);
    }

    let status = stream.channel.status()?;
    let avail_frames = match stream.channel.avail() {
        Err(err) if err.errno() == libc::EPIPE => return Ok(PollDescriptorsFlow::XRun),
        res => res,
    }? as usize;
    let delay_frames = match status.get_delay() {
        // Buffer underrun detected, but notification happens in XRun handler
        d if d < 0 => 0,
        d => d as usize,
    };
    let available_samples = avail_frames * stream.conf.channels as usize;

    // ALSA can have spurious wakeups where poll returns but avail < avail_min.
    // This is documented to occur with dmix (timer-driven) and other plugins.
    // Verify we have room for at least one full period before processing.
    // See: https://bugzilla.kernel.org/show_bug.cgi?id=202499
    if available_samples < stream.period_samples {
        return Ok(PollDescriptorsFlow::Continue);
    }

    Ok(PollDescriptorsFlow::Ready {
        status,
        delay_frames,
    })
}

// Read input data from ALSA and deliver it to the user.
fn process_input(
    stream: &StreamInner,
    buffer: &mut [u8],
    status: alsa::pcm::Status,
    delay_frames: usize,
    data_callback: &mut (dyn FnMut(&Data, &InputCallbackInfo) + Send + 'static),
) -> Result<(), BackendSpecificError> {
    stream.channel.io_bytes().readi(buffer)?;
    let data = buffer.as_mut_ptr() as *mut ();
    let data = unsafe { Data::from_parts(data, stream.period_samples, stream.sample_format) };
    let callback = match stream.creation_instant {
        None => stream_timestamp_hardware(&status)?,
        Some(creation) => stream_timestamp_fallback(creation)?,
    };
    let delay_duration = frames_to_duration(delay_frames, stream.conf.sample_rate);
    let capture = callback
        .sub(delay_duration)
        .ok_or_else(|| BackendSpecificError {
            description: "`capture` is earlier than representation supported by `StreamInstant`"
                .to_string(),
        })?;
    let timestamp = crate::InputStreamTimestamp { callback, capture };
    let info = crate::InputCallbackInfo { timestamp };
    data_callback(&data, &info);

    Ok(())
}

// Request data from the user's function and write it via ALSA.
//
// Returns `true`
fn process_output(
    stream: &StreamInner,
    buffer: &mut [u8],
    status: alsa::pcm::Status,
    delay_frames: usize,
    data_callback: &mut (dyn FnMut(&mut Data, &OutputCallbackInfo) + Send + 'static),
    error_callback: &mut dyn FnMut(StreamError),
) -> Result<(), BackendSpecificError> {
    // Buffer is always pre-filled with equilibrium, user overwrites what they want
    buffer.copy_from_slice(&stream.silence_template);
    {
        let data = buffer.as_mut_ptr() as *mut ();
        let mut data =
            unsafe { Data::from_parts(data, stream.period_samples, stream.sample_format) };
        let callback = match stream.creation_instant {
            None => stream_timestamp_hardware(&status)?,
            Some(creation) => stream_timestamp_fallback(creation)?,
        };
        let delay_duration = frames_to_duration(delay_frames, stream.conf.sample_rate);
        let playback = callback
            .add(delay_duration)
            .ok_or_else(|| BackendSpecificError {
                description: "`playback` occurs beyond representation supported by `StreamInstant`"
                    .to_string(),
            })?;
        let timestamp = crate::OutputStreamTimestamp { callback, playback };
        let info = crate::OutputCallbackInfo { timestamp };
        data_callback(&mut data, &info);
    }

    loop {
        match stream.channel.io_bytes().writei(buffer) {
            Err(err) if err.errno() == libc::EPIPE => {
                // ALSA underrun or overrun.
                // See https://github.com/alsa-project/alsa-lib/blob/b154d9145f0e17b9650e4584ddfdf14580b4e0d7/src/pcm/pcm.c#L8767-L8770
                // Even if these recover successfully, they still may cause audible glitches.

                error_callback(StreamError::BufferUnderrun);
                if let Err(recover_err) = stream.channel.try_recover(err, true) {
                    error_callback(recover_err.into());
                }
            }
            Err(err) => {
                error_callback(err.into());
                continue;
            }
            Ok(result) if result != stream.period_frames => {
                let description = format!(
                    "unexpected number of frames written: expected {}, \
                        result {result} (this should never happen)",
                    stream.period_frames
                );
                error_callback(BackendSpecificError { description }.into());
                continue;
            }
            _ => {
                break;
            }
        }
    }
    Ok(())
}

// Use hardware timestamps from ALSA.
//
// This ensures accurate timestamps based on actual hardware timing.
#[inline]
fn stream_timestamp_hardware(
    status: &alsa::pcm::Status,
) -> Result<crate::StreamInstant, BackendSpecificError> {
    let trigger_ts = status.get_trigger_htstamp();
    let ts = status.get_htstamp();
    let nanos = timespec_diff_nanos(ts, trigger_ts);
    if nanos < 0 {
        let description = format!(
            "get_htstamp `{}.{}` was earlier than get_trigger_htstamp `{}.{}`",
            ts.tv_sec, ts.tv_nsec, trigger_ts.tv_sec, trigger_ts.tv_nsec
        );
        return Err(BackendSpecificError { description });
    }
    Ok(crate::StreamInstant::from_nanos(nanos))
}

// Use elapsed duration since stream creation as fallback when hardware timestamps are unavailable.
//
// This ensures positive values that are compatible with our `StreamInstant` representation.
#[inline]
fn stream_timestamp_fallback(
    creation: std::time::Instant,
) -> Result<crate::StreamInstant, BackendSpecificError> {
    let now = std::time::Instant::now();
    let duration = now.duration_since(creation);
    crate::StreamInstant::from_nanos_i128(duration.as_nanos() as i128).ok_or(BackendSpecificError {
        description: "stream duration has exceeded `StreamInstant` representation".to_string(),
    })
}

// Adapted from `timestamp2ns` here:
// https://fossies.org/linux/alsa-lib/test/audio_time.c
#[inline]
fn timespec_to_nanos(ts: libc::timespec) -> i64 {
    let nanos = ts.tv_sec * 1_000_000_000 + ts.tv_nsec;
    #[cfg(target_pointer_width = "64")]
    return nanos;
    #[cfg(not(target_pointer_width = "64"))]
    return nanos.into();
}

// Adapted from `timediff` here:
// https://fossies.org/linux/alsa-lib/test/audio_time.c
#[inline]
fn timespec_diff_nanos(a: libc::timespec, b: libc::timespec) -> i64 {
    timespec_to_nanos(a) - timespec_to_nanos(b)
}

// Convert the given duration in frames at the given sample rate to a `std::time::Duration`.
#[inline]
fn frames_to_duration(frames: usize, rate: crate::SampleRate) -> std::time::Duration {
    let secsf = frames as f64 / rate as f64;
    let secs = secsf as u64;
    let nanos = ((secsf - secs as f64) * 1_000_000_000.0) as u32;
    std::time::Duration::new(secs, nanos)
}

impl Stream {
    fn new_input<D, E>(
        inner: Arc<StreamInner>,
        mut data_callback: D,
        mut error_callback: E,
        timeout: Option<Duration>,
    ) -> Stream
    where
        D: FnMut(&Data, &InputCallbackInfo) + Send + 'static,
        E: FnMut(StreamError) + Send + 'static,
    {
        let (tx, rx) = trigger();
        // Clone the handle for passing into worker thread.
        let stream = inner.clone();
        let thread = thread::Builder::new()
            .name("cpal_alsa_in".to_owned())
            .spawn(move || {
                input_stream_worker(
                    rx,
                    &stream,
                    &mut data_callback,
                    &mut error_callback,
                    timeout,
                );
            })
            .unwrap();
        Self {
            thread: Some(thread),
            inner,
            trigger: tx,
        }
    }

    fn new_output<D, E>(
        inner: Arc<StreamInner>,
        mut data_callback: D,
        mut error_callback: E,
        timeout: Option<Duration>,
    ) -> Stream
    where
        D: FnMut(&mut Data, &OutputCallbackInfo) + Send + 'static,
        E: FnMut(StreamError) + Send + 'static,
    {
        let (tx, rx) = trigger();
        // Clone the handle for passing into worker thread.
        let stream = inner.clone();
        let thread = thread::Builder::new()
            .name("cpal_alsa_out".to_owned())
            .spawn(move || {
                output_stream_worker(
                    rx,
                    &stream,
                    &mut data_callback,
                    &mut error_callback,
                    timeout,
                );
            })
            .unwrap();
        Self {
            thread: Some(thread),
            inner,
            trigger: tx,
        }
    }
}

impl Drop for Stream {
    fn drop(&mut self) {
        self.inner.dropping.store(true, Ordering::Release);
        self.trigger.wakeup();
        self.thread.take().unwrap().join().unwrap();
    }
}

impl StreamTrait for Stream {
    fn play(&self) -> Result<(), PlayStreamError> {
        self.inner.channel.pause(false).ok();
        Ok(())
    }
    fn pause(&self) -> Result<(), PauseStreamError> {
        self.inner.channel.pause(true).ok();
        Ok(())
    }
}

// Convert ALSA frames to FrameCount, clamping to valid range.
// ALSA Frames are i64 (64-bit) or i32 (32-bit).
fn clamp_frame_count(buffer_size: alsa::pcm::Frames) -> FrameCount {
    buffer_size.max(1).try_into().unwrap_or(FrameCount::MAX)
}

fn hw_params_buffer_size_min_max(hw_params: &alsa::pcm::HwParams) -> (FrameCount, FrameCount) {
    let min_buf = hw_params
        .get_buffer_size_min()
        .map(clamp_frame_count)
        .unwrap_or(1);
    let max_buf = hw_params
        .get_buffer_size_max()
        .map(clamp_frame_count)
        .unwrap_or(FrameCount::MAX);
    (min_buf, max_buf)
}

// Fill a buffer with equilibrium values for any sample format.
// Works with any buffer size, even if not perfectly aligned to sample boundaries.
fn fill_with_equilibrium(buffer: &mut [u8], sample_format: SampleFormat) {
    macro_rules! fill_typed {
        ($sample_type:ty) => {{
            let sample_size = std::mem::size_of::<$sample_type>();

            assert_eq!(
                buffer.len() % sample_size,
                0,
                "Buffer size must be aligned to sample size for format {:?}",
                sample_format
            );

            let num_samples = buffer.len() / sample_size;
            let equilibrium = <$sample_type as Sample>::EQUILIBRIUM;

            // Safety: We verified the buffer size is correctly aligned for the sample type
            let samples = unsafe {
                std::slice::from_raw_parts_mut(
                    buffer.as_mut_ptr() as *mut $sample_type,
                    num_samples,
                )
            };

            for sample in samples {
                *sample = equilibrium;
            }
        }};
    }

    match sample_format {
        SampleFormat::I8 => fill_typed!(i8),
        SampleFormat::I16 => fill_typed!(i16),
        SampleFormat::I24 => fill_typed!(I24),
        SampleFormat::I32 => fill_typed!(i32),
        // SampleFormat::I48 => fill_typed!(I48),
        SampleFormat::I64 => fill_typed!(i64),
        SampleFormat::U8 => fill_typed!(u8),
        SampleFormat::U16 => fill_typed!(u16),
        SampleFormat::U24 => fill_typed!(U24),
        SampleFormat::U32 => fill_typed!(u32),
        // SampleFormat::U48 => fill_typed!(U48),
        SampleFormat::U64 => fill_typed!(u64),
        SampleFormat::F32 => fill_typed!(f32),
        SampleFormat::F64 => fill_typed!(f64),
    }
}

fn init_hw_params<'a>(
    pcm_handle: &'a alsa::pcm::PCM,
    config: &StreamConfig,
    sample_format: SampleFormat,
) -> Result<alsa::pcm::HwParams<'a>, BackendSpecificError> {
    let hw_params = alsa::pcm::HwParams::any(pcm_handle)?;
    hw_params.set_access(alsa::pcm::Access::RWInterleaved)?;

    // Determine which endianness the hardware actually supports for this format.
    // We prefer native endian (no conversion needed) but fall back to the opposite
    // endian if that's all the hardware supports (e.g., LE USB DAC on BE system).
    let alsa_format = sample_format_to_alsa_format(&hw_params, sample_format)?;
    hw_params.set_format(alsa_format)?;

    hw_params.set_rate(config.sample_rate, alsa::ValueOr::Nearest)?;
    hw_params.set_channels(config.channels as u32)?;
    Ok(hw_params)
}

/// Convert SampleFormat to the appropriate alsa::pcm::Format based on what the hardware supports.
/// Prefers native endian, falls back to non-native if that's all the hardware supports.
fn sample_format_to_alsa_format(
    hw_params: &alsa::pcm::HwParams,
    sample_format: SampleFormat,
) -> Result<alsa::pcm::Format, BackendSpecificError> {
    use alsa::pcm::Format;

    // For each sample format, define (native_endian_format, opposite_endian_format) pairs
    let (native, opposite) = match sample_format {
        SampleFormat::I8 => return Ok(Format::S8), // No endianness
        SampleFormat::U8 => return Ok(Format::U8), // No endianness
        #[cfg(target_endian = "little")]
        SampleFormat::I16 => (Format::S16LE, Format::S16BE),
        #[cfg(target_endian = "big")]
        SampleFormat::I16 => (Format::S16BE, Format::S16LE),
        #[cfg(target_endian = "little")]
        SampleFormat::U16 => (Format::U16LE, Format::U16BE),
        #[cfg(target_endian = "big")]
        SampleFormat::U16 => (Format::U16BE, Format::U16LE),
        #[cfg(target_endian = "little")]
        SampleFormat::I24 => (Format::S24LE, Format::S24BE),
        #[cfg(target_endian = "big")]
        SampleFormat::I24 => (Format::S24BE, Format::S24LE),
        #[cfg(target_endian = "little")]
        SampleFormat::U24 => (Format::U24LE, Format::U24BE),
        #[cfg(target_endian = "big")]
        SampleFormat::U24 => (Format::U24BE, Format::U24LE),
        #[cfg(target_endian = "little")]
        SampleFormat::I32 => (Format::S32LE, Format::S32BE),
        #[cfg(target_endian = "big")]
        SampleFormat::I32 => (Format::S32BE, Format::S32LE),
        #[cfg(target_endian = "little")]
        SampleFormat::U32 => (Format::U32LE, Format::U32BE),
        #[cfg(target_endian = "big")]
        SampleFormat::U32 => (Format::U32BE, Format::U32LE),
        #[cfg(target_endian = "little")]
        SampleFormat::F32 => (Format::FloatLE, Format::FloatBE),
        #[cfg(target_endian = "big")]
        SampleFormat::F32 => (Format::FloatBE, Format::FloatLE),
        #[cfg(target_endian = "little")]
        SampleFormat::F64 => (Format::Float64LE, Format::Float64BE),
        #[cfg(target_endian = "big")]
        SampleFormat::F64 => (Format::Float64BE, Format::Float64LE),
        _ => {
            return Err(BackendSpecificError {
                description: format!("Sample format '{sample_format}' is not supported"),
            })
        }
    };

    // Try native endian first (optimal - no conversion needed)
    if hw_params.test_format(native).is_ok() {
        return Ok(native);
    }

    // Fall back to opposite endian if hardware only supports that
    if hw_params.test_format(opposite).is_ok() {
        return Ok(opposite);
    }

    Err(BackendSpecificError {
        description: format!(
            "Sample format '{sample_format}' is not supported by hardware in any endianness"
        ),
    })
}

fn set_hw_params_from_format(
    pcm_handle: &alsa::pcm::PCM,
    config: &StreamConfig,
    sample_format: SampleFormat,
) -> Result<bool, BackendSpecificError> {
    let hw_params = init_hw_params(pcm_handle, config, sample_format)?;

    // When BufferSize::Fixed(x) is specified, we configure double-buffering with
    // buffer_size = 2x and period_size = x. This provides consistent low-latency
    // behavior across different ALSA implementations and hardware.
    if let BufferSize::Fixed(buffer_frames) = config.buffer_size {
        hw_params.set_buffer_size_near((2 * buffer_frames) as alsa::pcm::Frames)?;
        hw_params
            .set_period_size_near(buffer_frames as alsa::pcm::Frames, alsa::ValueOr::Nearest)?;
    }

    // Apply hardware parameters
    pcm_handle.hw_params(&hw_params)?;

    // For BufferSize::Default, constrain to device's configured period with 2-period buffering.
    // PipeWire-ALSA picks a good period size but pairs it with many periods (huge buffer).
    // We need to re-initialize hw_params and set BOTH period and buffer to constrain properly.
    if config.buffer_size == BufferSize::Default {
        if let Ok(period) = hw_params.get_period_size() {
            // Re-initialize hw_params to clear previous constraints
            let hw_params = init_hw_params(pcm_handle, config, sample_format)?;

            // Set both period (to device's chosen value) and buffer (to 2 periods)
            hw_params.set_period_size_near(period, alsa::ValueOr::Nearest)?;
            hw_params.set_buffer_size_near(2 * period)?;

            // Re-apply with new constraints
            pcm_handle.hw_params(&hw_params)?;
        }
    }

    Ok(hw_params.can_pause())
}

fn set_sw_params_from_format(
    pcm_handle: &alsa::pcm::PCM,
    config: &StreamConfig,
    stream_type: alsa::Direction,
) -> Result<usize, BackendSpecificError> {
    let sw_params = pcm_handle.sw_params_current()?;

    let period_samples = {
        let (buffer, period) = pcm_handle.get_params()?;
        if buffer == 0 {
            return Err(BackendSpecificError {
                description: "initialization resulted in a null buffer".to_string(),
            });
        }
        let start_threshold = match stream_type {
            alsa::Direction::Playback => {
                // Always use 2-period double-buffering: one period playing from hardware, one
                // period queued in the software buffer. This ensures consistent low latency
                // regardless of the total buffer size.
                2 * period
            }
            alsa::Direction::Capture => 1,
        };
        sw_params.set_start_threshold(start_threshold.try_into().unwrap())?;

        // Set avail_min based on stream direction. For playback, "avail" means space available
        // for writing (buffer_size - frames_queued). For capture, "avail" means data available
        // for reading (frames_captured). These opposite semantics require different values.
        let target_avail = match stream_type {
            alsa::Direction::Playback => {
                // Wake when buffer level drops to one period remaining (avail >= buffer - period).
                // This maintains double-buffering by refilling when we're down to one period.
                buffer - period
            }
            alsa::Direction::Capture => {
                // Wake when one period of data is available to read (avail >= period).
                // Using buffer - period here would cause excessive latency as capture would
                // wait for nearly the entire buffer to fill before reading.
                period
            }
        };
        sw_params.set_avail_min(target_avail as alsa::pcm::Frames)?;

        period as usize * config.channels as usize
    };

    sw_params.set_tstamp_mode(true)?;
    sw_params.set_tstamp_type(alsa::pcm::TstampType::MonotonicRaw)?;

    // tstamp_type param cannot be changed after the device is opened.
    // The default tstamp_type value on most Linux systems is "monotonic",
    // let's try to use it if setting the tstamp_type fails.
    if pcm_handle.sw_params(&sw_params).is_err() {
        sw_params.set_tstamp_type(alsa::pcm::TstampType::Monotonic)?;
        pcm_handle.sw_params(&sw_params)?;
    }

    Ok(period_samples)
}

impl From<alsa::Error> for BackendSpecificError {
    fn from(err: alsa::Error) -> Self {
        Self {
            description: err.to_string(),
        }
    }
}

impl From<alsa::Error> for BuildStreamError {
    fn from(err: alsa::Error) -> Self {
        let err: BackendSpecificError = err.into();
        err.into()
    }
}

impl From<alsa::Error> for SupportedStreamConfigsError {
    fn from(err: alsa::Error) -> Self {
        let err: BackendSpecificError = err.into();
        err.into()
    }
}

impl From<alsa::Error> for PlayStreamError {
    fn from(err: alsa::Error) -> Self {
        let err: BackendSpecificError = err.into();
        err.into()
    }
}

impl From<alsa::Error> for PauseStreamError {
    fn from(err: alsa::Error) -> Self {
        let err: BackendSpecificError = err.into();
        err.into()
    }
}

impl From<alsa::Error> for StreamError {
    fn from(err: alsa::Error) -> Self {
        let err: BackendSpecificError = err.into();
        err.into()
    }
}
