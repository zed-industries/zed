//! AAudio backend implementation.
//!
//! Default backend on Android.

use std::cmp;
use std::convert::TryInto;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use std::vec::IntoIter as VecIntoIter;

extern crate ndk;

use convert::{stream_instant, to_stream_instant};
use java_interface::{AudioDeviceInfo, AudioManager};

use crate::traits::{DeviceTrait, HostTrait, StreamTrait};
use crate::{
    BackendSpecificError, BufferSize, BuildStreamError, Data, DefaultStreamConfigError,
    DeviceDescription, DeviceDescriptionBuilder, DeviceDirection, DeviceId, DeviceIdError,
    DeviceNameError, DeviceType, DevicesError, InputCallbackInfo, InputStreamTimestamp,
    InterfaceType, OutputCallbackInfo, OutputStreamTimestamp, PauseStreamError, PlayStreamError,
    SampleFormat, StreamConfig, StreamError, SupportedBufferSize, SupportedStreamConfig,
    SupportedStreamConfigRange, SupportedStreamConfigsError,
};

mod convert;
mod java_interface;

use self::ndk::audio::AudioStream;
use java_interface::AudioDeviceType as AndroidDeviceType;

impl From<AndroidDeviceType> for DeviceType {
    fn from(device_type: AndroidDeviceType) -> Self {
        match device_type {
            AndroidDeviceType::BuiltinSpeaker
            | AndroidDeviceType::BuiltinSpeakerSafe
            | AndroidDeviceType::BleSpeaker => DeviceType::Speaker,

            AndroidDeviceType::BuiltinMic => DeviceType::Microphone,

            AndroidDeviceType::WiredHeadphones => DeviceType::Headphones,

            AndroidDeviceType::WiredHeadset
            | AndroidDeviceType::UsbHeadset
            | AndroidDeviceType::BleHeadset
            | AndroidDeviceType::BluetoothSCO => DeviceType::Headset,

            AndroidDeviceType::BuiltinEarpiece => DeviceType::Earpiece,

            AndroidDeviceType::HearingAid => DeviceType::HearingAid,

            AndroidDeviceType::Dock => DeviceType::Dock,

            AndroidDeviceType::Fm | AndroidDeviceType::FmTuner | AndroidDeviceType::TvTuner => {
                DeviceType::Tuner
            }

            AndroidDeviceType::RemoteSubmix => DeviceType::Virtual,

            _ => DeviceType::Unknown,
        }
    }
}

impl From<AndroidDeviceType> for InterfaceType {
    fn from(device_type: AndroidDeviceType) -> Self {
        match device_type {
            AndroidDeviceType::UsbDevice
            | AndroidDeviceType::UsbAccessory
            | AndroidDeviceType::UsbHeadset => InterfaceType::Usb,

            AndroidDeviceType::BluetoothA2DP
            | AndroidDeviceType::BluetoothSCO
            | AndroidDeviceType::BleHeadset
            | AndroidDeviceType::BleSpeaker
            | AndroidDeviceType::BleBroadcast => InterfaceType::Bluetooth,

            AndroidDeviceType::Hdmi | AndroidDeviceType::HdmiArc | AndroidDeviceType::HdmiEarc => {
                InterfaceType::Hdmi
            }

            AndroidDeviceType::LineAnalog
            | AndroidDeviceType::LineDigital
            | AndroidDeviceType::AuxLine => InterfaceType::Line,

            AndroidDeviceType::BuiltinEarpiece
            | AndroidDeviceType::BuiltinMic
            | AndroidDeviceType::BuiltinSpeaker
            | AndroidDeviceType::BuiltinSpeakerSafe => InterfaceType::BuiltIn,

            AndroidDeviceType::Ip => InterfaceType::Network,

            AndroidDeviceType::RemoteSubmix => InterfaceType::Virtual,

            _ => InterfaceType::Unknown,
        }
    }
}

// constants from android.media.AudioFormat
const CHANNEL_OUT_MONO: i32 = 4;
const CHANNEL_OUT_STEREO: i32 = 12;

// Android Java API supports up to 8 channels
// TODO: more channels available in native AAudio
// Maps channel masks to their corresponding channel counts
const CHANNEL_CONFIGS: [(i32, u16); 2] = [(CHANNEL_OUT_MONO, 1), (CHANNEL_OUT_STEREO, 2)];

const SAMPLE_RATES: [i32; 15] = [
    5512, 8000, 11025, 12000, 16000, 22050, 24000, 32000, 44100, 48000, 64000, 88200, 96000,
    176_400, 192_000,
];

pub struct Host;
#[derive(Clone)]
pub struct Device(Option<AudioDeviceInfo>);

/// Stream wraps AudioStream in Arc<Mutex<>> to provide Send + Sync semantics.
///
/// While the underlying ndk::audio::AudioStream is neither Send nor Sync in ndk 0.9.0
/// (see https://developer.android.com/ndk/guides/audio/aaudio/aaudio#thread-safety),
/// we wrap it in a mutex to enable safe concurrent access and manually implement Send + Sync.
///
/// # Safety
///
/// This is safe because:
/// - AAudio functions are designed to be called from any thread (the Android docs state
///   "AAudio is not thread-safe" meaning it lacks internal locking, not that it's unsafe)
/// - Audio callbacks are called on a dedicated AAudio thread and don't access Stream
/// - The Mutex ensures exclusive access for control operations (play, pause)
/// - The pointer in AudioStream (NonNull<AAudioStreamStruct>) is valid for the lifetime
///   of the stream and AAudio C API functions are thread-safe at the C level
#[derive(Clone)]
pub enum Stream {
    Input(Arc<Mutex<AudioStream>>),
    Output(Arc<Mutex<AudioStream>>),
}

// SAFETY: AudioStream can be safely sent between threads. The AAudio C API is thread-safe
// for moving stream ownership between threads. The NonNull pointer remains valid.
unsafe impl Send for Stream {}

// SAFETY: AudioStream can be safely shared between threads when protected by a Mutex.
// All operations on the stream go through the mutex, ensuring exclusive access.
unsafe impl Sync for Stream {}

// Compile-time assertion that Stream is Send and Sync
crate::assert_stream_send!(Stream);
crate::assert_stream_sync!(Stream);

pub use crate::iter::{SupportedInputConfigs, SupportedOutputConfigs};
pub type Devices = std::vec::IntoIter<Device>;

impl Host {
    pub fn new() -> Result<Self, crate::HostUnavailable> {
        Ok(Host)
    }
}

impl HostTrait for Host {
    type Devices = Devices;
    type Device = Device;

    fn is_available() -> bool {
        true
    }

    fn devices(&self) -> Result<Self::Devices, DevicesError> {
        if let Ok(devices) = AudioDeviceInfo::request(DeviceDirection::Duplex) {
            Ok(devices
                .into_iter()
                .map(|d| Device(Some(d)))
                .collect::<Vec<_>>()
                .into_iter())
        } else {
            Ok(vec![Device(None)].into_iter())
        }
    }

    fn default_input_device(&self) -> Option<Self::Device> {
        Some(Device(None))
    }

    fn default_output_device(&self) -> Option<Self::Device> {
        Some(Device(None))
    }
}

fn buffer_size_range() -> SupportedBufferSize {
    if let Ok(min_buffer_size) = AudioManager::get_frames_per_buffer() {
        SupportedBufferSize::Range {
            min: min_buffer_size as u32,
            max: i32::MAX as u32,
        }
    } else {
        SupportedBufferSize::Unknown
    }
}

fn default_supported_configs() -> VecIntoIter<SupportedStreamConfigRange> {
    const FORMATS: [SampleFormat; 2] = [SampleFormat::I16, SampleFormat::F32];

    let buffer_size = buffer_size_range();
    let mut output = Vec::with_capacity(SAMPLE_RATES.len() * CHANNEL_CONFIGS.len() * FORMATS.len());
    for sample_format in &FORMATS {
        for (_channel_mask, channel_count) in &CHANNEL_CONFIGS {
            for sample_rate in &SAMPLE_RATES {
                output.push(SupportedStreamConfigRange {
                    channels: *channel_count,
                    min_sample_rate: *sample_rate as u32,
                    max_sample_rate: *sample_rate as u32,
                    buffer_size,
                    sample_format: *sample_format,
                });
            }
        }
    }

    output.into_iter()
}

fn device_supported_configs(device: &AudioDeviceInfo) -> VecIntoIter<SupportedStreamConfigRange> {
    let sample_rates = if !device.sample_rates.is_empty() {
        device.sample_rates.as_slice()
    } else {
        &SAMPLE_RATES
    };

    const ALL_CHANNELS: [i32; 2] = [1, 2];
    let channel_counts = if !device.channel_counts.is_empty() {
        device.channel_counts.as_slice()
    } else {
        &ALL_CHANNELS
    };

    const ALL_FORMATS: [SampleFormat; 2] = [SampleFormat::I16, SampleFormat::F32];
    let formats = if !device.formats.is_empty() {
        device.formats.as_slice()
    } else {
        &ALL_FORMATS
    };

    let buffer_size = buffer_size_range();
    let mut output = Vec::with_capacity(sample_rates.len() * channel_counts.len() * formats.len());
    for sample_rate in sample_rates {
        for channel_count in channel_counts {
            assert!(*channel_count > 0);
            if *channel_count > 2 {
                // could be supported by the device
                // TODO: more channels available in native AAudio
                continue;
            }
            for format in formats {
                output.push(SupportedStreamConfigRange {
                    channels: cmp::min(*channel_count as u16, 2u16),
                    min_sample_rate: *sample_rate as u32,
                    max_sample_rate: *sample_rate as u32,
                    buffer_size,
                    sample_format: *format,
                });
            }
        }
    }

    output.into_iter()
}

fn configure_for_device(
    builder: ndk::audio::AudioStreamBuilder,
    device: &Device,
    config: &StreamConfig,
) -> ndk::audio::AudioStreamBuilder {
    let mut builder = if let Some(info) = &device.0 {
        builder.device_id(info.id)
    } else {
        builder
    };
    builder = builder.sample_rate(config.sample_rate.try_into().unwrap());

    // Note: Buffer size validation is not needed - the native AAudio API validates buffer sizes
    // when `open_stream()` is called.
    match &config.buffer_size {
        BufferSize::Default => builder,
        BufferSize::Fixed(size) => builder
            .frames_per_data_callback(*size as i32)
            .buffer_capacity_in_frames((*size * 2) as i32), // Double-buffering
    }
}

fn build_input_stream<D, E>(
    device: &Device,
    config: &StreamConfig,
    mut data_callback: D,
    mut error_callback: E,
    builder: ndk::audio::AudioStreamBuilder,
    sample_format: SampleFormat,
) -> Result<Stream, BuildStreamError>
where
    D: FnMut(&Data, &InputCallbackInfo) + Send + 'static,
    E: FnMut(StreamError) + Send + 'static,
{
    let builder = configure_for_device(builder, device, config);
    let created = Instant::now();
    let channel_count = config.channels as i32;
    let stream = builder
        .data_callback(Box::new(move |stream, data, num_frames| {
            let cb_info = InputCallbackInfo {
                timestamp: InputStreamTimestamp {
                    callback: to_stream_instant(created.elapsed()),
                    capture: stream_instant(stream),
                },
            };
            (data_callback)(
                &unsafe {
                    Data::from_parts(
                        data as *mut _,
                        (num_frames * channel_count).try_into().unwrap(),
                        sample_format,
                    )
                },
                &cb_info,
            );
            ndk::audio::AudioCallbackResult::Continue
        }))
        .error_callback(Box::new(move |_stream, error| {
            (error_callback)(StreamError::from(error))
        }))
        .open_stream()?;
    // SAFETY: Stream implements Send + Sync (see unsafe impl below). Arc<Mutex<AudioStream>>
    // is safe because the Mutex provides exclusive access and AudioStream's thread safety
    // is documented in the AAudio C API.
    #[allow(clippy::arc_with_non_send_sync)]
    Ok(Stream::Input(Arc::new(Mutex::new(stream))))
}

fn build_output_stream<D, E>(
    device: &Device,
    config: &StreamConfig,
    mut data_callback: D,
    mut error_callback: E,
    builder: ndk::audio::AudioStreamBuilder,
    sample_format: SampleFormat,
) -> Result<Stream, BuildStreamError>
where
    D: FnMut(&mut Data, &OutputCallbackInfo) + Send + 'static,
    E: FnMut(StreamError) + Send + 'static,
{
    let builder = configure_for_device(builder, device, config);
    let created = Instant::now();
    let channel_count = config.channels as i32;
    let stream = builder
        .data_callback(Box::new(move |stream, data, num_frames| {
            let cb_info = OutputCallbackInfo {
                timestamp: OutputStreamTimestamp {
                    callback: to_stream_instant(created.elapsed()),
                    playback: stream_instant(stream),
                },
            };
            (data_callback)(
                &mut unsafe {
                    Data::from_parts(
                        data as *mut _,
                        (num_frames * channel_count).try_into().unwrap(),
                        sample_format,
                    )
                },
                &cb_info,
            );
            ndk::audio::AudioCallbackResult::Continue
        }))
        .error_callback(Box::new(move |_stream, error| {
            (error_callback)(StreamError::from(error))
        }))
        .open_stream()?;
    // SAFETY: Stream implements Send + Sync (see unsafe impl below). Arc<Mutex<AudioStream>>
    // is safe because the Mutex provides exclusive access and AudioStream's thread safety
    // is documented in the AAudio C API.
    #[allow(clippy::arc_with_non_send_sync)]
    Ok(Stream::Output(Arc::new(Mutex::new(stream))))
}

impl DeviceTrait for Device {
    type SupportedInputConfigs = SupportedInputConfigs;
    type SupportedOutputConfigs = SupportedOutputConfigs;
    type Stream = Stream;

    fn name(&self) -> Result<String, DeviceNameError> {
        match &self.0 {
            None => Ok("default".to_string()),
            Some(info) => {
                let name = if info.address.is_empty() {
                    format!("{}:{:?}", info.product_name, info.device_type)
                } else {
                    format!(
                        "{}:{:?}:{}",
                        info.product_name, info.device_type, info.address
                    )
                };
                Ok(name)
            }
        }
    }

    fn description(&self) -> Result<DeviceDescription, DeviceNameError> {
        match &self.0 {
            None => Ok(DeviceDescriptionBuilder::new("Default Device".to_string()).build()),
            Some(info) => {
                let mut builder = DeviceDescriptionBuilder::new(info.product_name.clone())
                    .device_type(info.device_type.into())
                    .interface_type(info.device_type.into())
                    .direction(info.direction);

                // Add address if not empty
                if !info.address.is_empty() {
                    builder = builder.address(info.address.clone());
                }

                Ok(builder.build())
            }
        }
    }

    fn id(&self) -> Result<DeviceId, DeviceIdError> {
        let device_str = match &self.0 {
            None => "-1".to_string(), // Default device
            Some(info) => info.id.to_string(),
        };
        Ok(DeviceId(crate::platform::HostId::AAudio, device_str))
    }

    fn supported_input_configs(
        &self,
    ) -> Result<Self::SupportedInputConfigs, SupportedStreamConfigsError> {
        let configs = if let Some(info) = &self.0 {
            device_supported_configs(info)
        } else {
            default_supported_configs()
        };
        Ok(configs)
    }

    fn supported_output_configs(
        &self,
    ) -> Result<Self::SupportedOutputConfigs, SupportedStreamConfigsError> {
        let configs = if let Some(info) = &self.0 {
            device_supported_configs(info)
        } else {
            default_supported_configs()
        };
        Ok(configs)
    }

    fn default_input_config(&self) -> Result<SupportedStreamConfig, DefaultStreamConfigError> {
        let mut configs: Vec<_> = self.supported_input_configs().unwrap().collect();
        configs.sort_by(|a, b| b.cmp_default_heuristics(a));
        let config = configs
            .into_iter()
            .next()
            .ok_or(DefaultStreamConfigError::StreamTypeNotSupported)?
            .with_max_sample_rate();
        Ok(config)
    }

    fn default_output_config(&self) -> Result<SupportedStreamConfig, DefaultStreamConfigError> {
        let mut configs: Vec<_> = self.supported_output_configs().unwrap().collect();
        configs.sort_by(|a, b| b.cmp_default_heuristics(a));
        let config = configs
            .into_iter()
            .next()
            .ok_or(DefaultStreamConfigError::StreamTypeNotSupported)?
            .with_max_sample_rate();
        Ok(config)
    }

    fn build_input_stream_raw<D, E>(
        &self,
        config: &StreamConfig,
        sample_format: SampleFormat,
        data_callback: D,
        error_callback: E,
        _timeout: Option<Duration>,
    ) -> Result<Self::Stream, BuildStreamError>
    where
        D: FnMut(&Data, &InputCallbackInfo) + Send + 'static,
        E: FnMut(StreamError) + Send + 'static,
    {
        let format = match sample_format {
            SampleFormat::I16 => ndk::audio::AudioFormat::PCM_I16,
            SampleFormat::F32 => ndk::audio::AudioFormat::PCM_Float,
            sample_format => {
                return Err(BackendSpecificError {
                    description: format!("{} format is not supported on Android.", sample_format),
                }
                .into())
            }
        };
        let channel_count = match config.channels {
            1 => 1,
            2 => 2,
            channels => {
                // TODO: more channels available in native AAudio
                return Err(BackendSpecificError {
                    description: format!(
                        "{} channels are not supported yet (only 1 or 2).",
                        channels
                    ),
                }
                .into());
            }
        };

        let builder = ndk::audio::AudioStreamBuilder::new()?
            .direction(ndk::audio::AudioDirection::Input)
            .channel_count(channel_count)
            .format(format);

        build_input_stream(
            self,
            config,
            data_callback,
            error_callback,
            builder,
            sample_format,
        )
    }

    fn build_output_stream_raw<D, E>(
        &self,
        config: &StreamConfig,
        sample_format: SampleFormat,
        data_callback: D,
        error_callback: E,
        _timeout: Option<Duration>,
    ) -> Result<Self::Stream, BuildStreamError>
    where
        D: FnMut(&mut Data, &OutputCallbackInfo) + Send + 'static,
        E: FnMut(StreamError) + Send + 'static,
    {
        let format = match sample_format {
            SampleFormat::I16 => ndk::audio::AudioFormat::PCM_I16,
            SampleFormat::F32 => ndk::audio::AudioFormat::PCM_Float,
            sample_format => {
                return Err(BackendSpecificError {
                    description: format!("{} format is not supported on Android.", sample_format),
                }
                .into())
            }
        };
        let channel_count = match config.channels {
            1 => 1,
            2 => 2,
            channels => {
                // TODO: more channels available in native AAudio
                return Err(BackendSpecificError {
                    description: format!(
                        "{} channels are not supported yet (only 1 or 2).",
                        channels
                    ),
                }
                .into());
            }
        };

        let builder = ndk::audio::AudioStreamBuilder::new()?
            .direction(ndk::audio::AudioDirection::Output)
            .channel_count(channel_count)
            .format(format);

        build_output_stream(
            self,
            config,
            data_callback,
            error_callback,
            builder,
            sample_format,
        )
    }
}

impl StreamTrait for Stream {
    fn play(&self) -> Result<(), PlayStreamError> {
        match self {
            Self::Input(stream) => stream
                .lock()
                .unwrap()
                .request_start()
                .map_err(PlayStreamError::from),
            Self::Output(stream) => stream
                .lock()
                .unwrap()
                .request_start()
                .map_err(PlayStreamError::from),
        }
    }

    fn pause(&self) -> Result<(), PauseStreamError> {
        match self {
            Self::Input(_) => Err(BackendSpecificError {
                description: "Pause called on the input stream.".to_owned(),
            }
            .into()),
            Self::Output(stream) => stream
                .lock()
                .unwrap()
                .request_pause()
                .map_err(PauseStreamError::from),
        }
    }
}
