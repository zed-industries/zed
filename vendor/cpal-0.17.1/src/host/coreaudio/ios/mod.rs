//! CoreAudio implementation for iOS using AVAudioSession and RemoteIO Audio Units.

use std::sync::Mutex;

use coreaudio::audio_unit::render_callback::data;
use coreaudio::audio_unit::{render_callback, AudioUnit, Element, Scope};
use objc2_audio_toolbox::{kAudioOutputUnitProperty_EnableIO, kAudioUnitProperty_StreamFormat};
use objc2_core_audio_types::AudioBuffer;

use objc2_avf_audio::AVAudioSession;

use super::{asbd_from_config, frames_to_duration, host_time_to_stream_instant};
use crate::traits::{DeviceTrait, HostTrait, StreamTrait};

use crate::{
    BackendSpecificError, BufferSize, BuildStreamError, ChannelCount, Data,
    DefaultStreamConfigError, DeviceDescription, DeviceDescriptionBuilder, DeviceId, DeviceIdError,
    DeviceNameError, DevicesError, InputCallbackInfo, OutputCallbackInfo, PauseStreamError,
    PlayStreamError, SampleFormat, SampleRate, StreamConfig, StreamError, SupportedBufferSize,
    SupportedStreamConfig, SupportedStreamConfigRange, SupportedStreamConfigsError,
};

use self::enumerate::{
    default_input_device, default_output_device, Devices, SupportedInputConfigs,
    SupportedOutputConfigs,
};
use std::ptr::NonNull;
use std::time::Duration;

pub mod enumerate;

// These days the default of iOS is now F32 and no longer I16
const SUPPORTED_SAMPLE_FORMAT: SampleFormat = SampleFormat::F32;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Device;

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
        true
    }

    fn devices(&self) -> Result<Self::Devices, DevicesError> {
        Ok(Devices::new())
    }

    fn default_input_device(&self) -> Option<Self::Device> {
        default_input_device()
    }

    fn default_output_device(&self) -> Option<Self::Device> {
        default_output_device()
    }
}

impl Device {
    fn description(&self) -> Result<DeviceDescription, DeviceNameError> {
        // Query AVAudioSession to determine actual input/output availability
        // SAFETY: AVAudioSession::sharedInstance() returns the global audio session singleton
        let direction = unsafe {
            let audio_session = AVAudioSession::sharedInstance();
            let input_channels = Some(audio_session.inputNumberOfChannels() as ChannelCount);
            let output_channels = Some(audio_session.outputNumberOfChannels() as ChannelCount);

            crate::device_description::direction_from_counts(input_channels, output_channels)
        };

        Ok(DeviceDescriptionBuilder::new("Default Device".to_string())
            .direction(direction)
            .build())
    }

    fn id(&self) -> Result<DeviceId, DeviceIdError> {
        Ok(DeviceId(
            crate::platform::HostId::CoreAudio,
            "default".to_string(),
        ))
    }

    fn supported_input_configs(
        &self,
    ) -> Result<SupportedInputConfigs, SupportedStreamConfigsError> {
        Ok(get_supported_stream_configs(true))
    }

    fn supported_output_configs(
        &self,
    ) -> Result<SupportedOutputConfigs, SupportedStreamConfigsError> {
        Ok(get_supported_stream_configs(false))
    }

    fn default_input_config(&self) -> Result<SupportedStreamConfig, DefaultStreamConfigError> {
        // Get the primary (exact channel count) config from supported configs
        get_supported_stream_configs(true)
            .next()
            .map(|range| range.with_max_sample_rate())
            .ok_or(DefaultStreamConfigError::StreamTypeNotSupported)
    }

    fn default_output_config(&self) -> Result<SupportedStreamConfig, DefaultStreamConfigError> {
        // Get the maximum channel count config from supported configs
        get_supported_stream_configs(false)
            .last()
            .map(|range| range.with_max_sample_rate())
            .ok_or(DefaultStreamConfigError::StreamTypeNotSupported)
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
        // Configure buffer size and create audio unit
        let mut audio_unit = setup_stream_audio_unit(config, sample_format, true)?;

        // Query device buffer size for latency calculation
        let device_buffer_frames = Some(get_device_buffer_frames());

        // Set up input callback
        setup_input_callback(
            &mut audio_unit,
            sample_format,
            config.sample_rate,
            device_buffer_frames,
            data_callback,
            error_callback,
        )?;

        audio_unit.start()?;

        Ok(Stream::new(StreamInner {
            playing: true,
            audio_unit,
        }))
    }

    /// Create an output stream.
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
        // Configure buffer size and create audio unit
        let mut audio_unit = setup_stream_audio_unit(config, sample_format, false)?;

        // Query device buffer size for latency calculation
        let device_buffer_frames = Some(get_device_buffer_frames());

        // Set up output callback
        setup_output_callback(
            &mut audio_unit,
            sample_format,
            config.sample_rate,
            device_buffer_frames,
            data_callback,
            error_callback,
        )?;

        audio_unit.start()?;

        Ok(Stream::new(StreamInner {
            playing: true,
            audio_unit,
        }))
    }
}

pub struct Stream {
    inner: Mutex<StreamInner>,
}

impl Stream {
    fn new(inner: StreamInner) -> Self {
        Self {
            inner: Mutex::new(inner),
        }
    }
}

impl StreamTrait for Stream {
    fn play(&self) -> Result<(), PlayStreamError> {
        let mut stream = self
            .inner
            .lock()
            .map_err(|_| PlayStreamError::BackendSpecific {
                err: BackendSpecificError {
                    description: "A cpal stream operation panicked while holding the lock - this is a bug, please report it".to_string(),
                },
            })?;

        if !stream.playing {
            if let Err(e) = stream.audio_unit.start() {
                let description = format!("{}", e);
                let err = BackendSpecificError { description };
                return Err(err.into());
            }
            stream.playing = true;
        }
        Ok(())
    }

    fn pause(&self) -> Result<(), PauseStreamError> {
        let mut stream = self
            .inner
            .lock()
            .map_err(|_| PauseStreamError::BackendSpecific {
                err: BackendSpecificError {
                    description: "A cpal stream operation panicked while holding the lock - this is a bug, please report it".to_string(),
                },
            })?;

        if stream.playing {
            if let Err(e) = stream.audio_unit.stop() {
                let description = format!("{}", e);
                let err = BackendSpecificError { description };
                return Err(err.into());
            }

            stream.playing = false;
        }
        Ok(())
    }
}

struct StreamInner {
    playing: bool,
    audio_unit: AudioUnit,
}

fn create_audio_unit() -> Result<AudioUnit, coreaudio::Error> {
    AudioUnit::new(coreaudio::audio_unit::IOType::RemoteIO)
}

fn configure_for_recording(audio_unit: &mut AudioUnit) -> Result<(), coreaudio::Error> {
    // Enable mic recording
    let enable_input = 1u32;
    audio_unit.set_property(
        kAudioOutputUnitProperty_EnableIO,
        Scope::Input,
        Element::Input,
        Some(&enable_input),
    )?;

    // Disable output
    let disable_output = 0u32;
    audio_unit.set_property(
        kAudioOutputUnitProperty_EnableIO,
        Scope::Output,
        Element::Output,
        Some(&disable_output),
    )?;

    Ok(())
}

/// Configure AVAudioSession with the requested buffer size.
///
/// Note: iOS may not honor the exact request due to system constraints.
fn set_audio_session_buffer_size(
    buffer_size: u32,
    sample_rate: crate::SampleRate,
) -> Result<(), BuildStreamError> {
    // SAFETY: AVAudioSession::sharedInstance() returns the global audio session singleton
    let audio_session = unsafe { AVAudioSession::sharedInstance() };

    // Calculate preferred buffer duration in seconds
    let buffer_duration = buffer_size as f64 / sample_rate as f64;

    // Set the preferred IO buffer duration
    // SAFETY: setPreferredIOBufferDuration_error is safe to call with valid duration
    unsafe {
        audio_session
            .setPreferredIOBufferDuration_error(buffer_duration)
            .map_err(|_| BuildStreamError::StreamConfigNotSupported)?;
    }

    Ok(())
}

/// Get the actual buffer size from AVAudioSession.
///
/// This queries the current IO buffer duration from AVAudioSession and converts
/// it to frames based on the current sample rate.
fn get_device_buffer_frames() -> usize {
    // SAFETY: AVAudioSession methods are safe to call on the singleton instance
    unsafe {
        let audio_session = AVAudioSession::sharedInstance();
        let buffer_duration = audio_session.IOBufferDuration();
        let sample_rate = audio_session.sampleRate();
        (buffer_duration * sample_rate) as usize
    }
}

/// Get supported stream config ranges for input (is_input=true) or output (is_input=false).
fn get_supported_stream_configs(is_input: bool) -> std::vec::IntoIter<SupportedStreamConfigRange> {
    // SAFETY: AVAudioSession methods are safe to call on the singleton instance
    let (sample_rate, max_channels) = unsafe {
        let audio_session = AVAudioSession::sharedInstance();
        let sample_rate = audio_session.sampleRate() as u32;
        let max_channels = if is_input {
            audio_session.inputNumberOfChannels() as u16
        } else {
            audio_session.outputNumberOfChannels() as u16
        };
        (sample_rate, max_channels)
    };

    // Typical iOS hardware buffer frame limits according to Apple Technical Q&A QA1631.
    let buffer_size = SupportedBufferSize::Range {
        min: 256,
        max: 4096,
    };

    // For input, only return the exact channel count (no flexibility)
    // For output, support flexible channel counts up to the hardware maximum
    let min_channels = if is_input { max_channels } else { 1 };

    let configs: Vec<_> = (min_channels..=max_channels)
        .map(|channels| SupportedStreamConfigRange {
            channels,
            min_sample_rate: sample_rate,
            max_sample_rate: sample_rate,
            buffer_size,
            sample_format: SUPPORTED_SAMPLE_FORMAT,
        })
        .collect();

    configs.into_iter()
}

/// Setup audio unit with common configuration for input or output streams.
fn setup_stream_audio_unit(
    config: &StreamConfig,
    sample_format: SampleFormat,
    is_input: bool,
) -> Result<AudioUnit, BuildStreamError> {
    // Configure buffer size via AVAudioSession
    if let BufferSize::Fixed(buffer_size) = config.buffer_size {
        set_audio_session_buffer_size(buffer_size, config.sample_rate)?;
    }

    let mut audio_unit = create_audio_unit()?;

    if is_input {
        audio_unit.uninitialize()?;
        configure_for_recording(&mut audio_unit)?;
        audio_unit.initialize()?;
    }

    // Set the stream format in interleaved mode
    // For input: Output scope of Input element (data coming out of input)
    // For output: Input scope of Output element (data going into output)
    let (scope, element) = if is_input {
        (Scope::Output, Element::Input)
    } else {
        (Scope::Input, Element::Output)
    };

    let asbd = asbd_from_config(config, sample_format);
    audio_unit.set_property(kAudioUnitProperty_StreamFormat, scope, element, Some(&asbd))?;

    Ok(audio_unit)
}

/// Extract AudioBuffer and convert to Data, handling differences between input and output.
///
/// # Safety
///
/// Caller must ensure:
/// - `args.data.data` points to valid AudioBufferList
/// - For input: AudioBufferList has at least one buffer
/// - Buffer data remains valid for the callback duration
#[inline]
unsafe fn extract_audio_buffer(
    args: &render_callback::Args<data::Raw>,
    bytes_per_channel: usize,
    sample_format: SampleFormat,
    is_input: bool,
) -> (AudioBuffer, Data) {
    let buffer = if is_input {
        // Input: access through buffer array
        let first_buf_ptr = core::ptr::addr_of!((*args.data.data).mBuffers) as *const AudioBuffer;
        core::ptr::read_unaligned(first_buf_ptr)
    } else {
        // Output: direct access
        let buf_ptr = core::ptr::addr_of!((*args.data.data).mBuffers[0]);
        core::ptr::read_unaligned(buf_ptr)
    };

    let mut data_ptr = buffer.mData as *mut ();
    let mut len = buffer.mDataByteSize as usize / bytes_per_channel;

    // SAFETY: slice::from_raw_parts requires a non-null pointer.
    if data_ptr.is_null() {
        data_ptr = NonNull::dangling().as_ptr();
        len = 0;
    }

    let data = Data::from_parts(data_ptr, len, sample_format);

    (buffer, data)
}

/// Setup input callback with proper latency calculation.
fn setup_input_callback<D, E>(
    audio_unit: &mut AudioUnit,
    sample_format: SampleFormat,
    sample_rate: SampleRate,
    device_buffer_frames: Option<usize>,
    mut data_callback: D,
    mut error_callback: E,
) -> Result<(), BuildStreamError>
where
    D: FnMut(&Data, &InputCallbackInfo) + Send + 'static,
    E: FnMut(StreamError) + Send + 'static,
{
    let bytes_per_channel = sample_format.sample_size();
    type Args = render_callback::Args<data::Raw>;

    audio_unit.set_input_callback(move |args: Args| {
        // SAFETY: CoreAudio provides valid AudioBufferList for the callback duration
        let (buffer, data) =
            unsafe { extract_audio_buffer(&args, bytes_per_channel, sample_format, true) };

        let callback = match host_time_to_stream_instant(args.time_stamp.mHostTime) {
            Err(err) => {
                error_callback(err.into());
                return Err(());
            }
            Ok(cb) => cb,
        };

        let latency_frames = device_buffer_frames.unwrap_or_else(|| {
            let channels = buffer.mNumberChannels as usize;
            if channels > 0 {
                data.len() / channels
            } else {
                0
            }
        });
        let delay = frames_to_duration(latency_frames, sample_rate);
        let capture = callback
            .sub(delay)
            .expect("`capture` occurs before origin of alsa `StreamInstant`");
        let timestamp = crate::InputStreamTimestamp { callback, capture };

        let info = InputCallbackInfo { timestamp };
        data_callback(&data, &info);
        Ok(())
    })?;

    Ok(())
}

/// Setup output callback with proper latency calculation.
fn setup_output_callback<D, E>(
    audio_unit: &mut AudioUnit,
    sample_format: SampleFormat,
    sample_rate: SampleRate,
    device_buffer_frames: Option<usize>,
    mut data_callback: D,
    mut error_callback: E,
) -> Result<(), BuildStreamError>
where
    D: FnMut(&mut Data, &OutputCallbackInfo) + Send + 'static,
    E: FnMut(StreamError) + Send + 'static,
{
    let bytes_per_channel = sample_format.sample_size();
    type Args = render_callback::Args<data::Raw>;

    audio_unit.set_render_callback(move |args: Args| {
        // SAFETY: CoreAudio provides valid AudioBufferList for the callback duration
        let (buffer, mut data) =
            unsafe { extract_audio_buffer(&args, bytes_per_channel, sample_format, false) };

        let callback = match host_time_to_stream_instant(args.time_stamp.mHostTime) {
            Err(err) => {
                error_callback(err.into());
                return Err(());
            }
            Ok(cb) => cb,
        };

        let latency_frames = device_buffer_frames.unwrap_or_else(|| {
            let channels = buffer.mNumberChannels as usize;
            if channels > 0 {
                data.len() / channels
            } else {
                0
            }
        });
        let delay = frames_to_duration(latency_frames, sample_rate);
        let playback = callback
            .add(delay)
            .expect("`playback` occurs beyond representation supported by `StreamInstant`");
        let timestamp = crate::OutputStreamTimestamp { callback, playback };

        let info = OutputCallbackInfo { timestamp };
        data_callback(&mut data, &info);
        Ok(())
    })?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{BufferSize, SampleRate, StreamConfig};

    #[test]
    fn test_ios_fixed_buffer_size() {
        let host = crate::default_host();
        let device = host.default_output_device().unwrap();

        let config = StreamConfig {
            channels: 2,
            sample_rate: SampleRate(48000),
            buffer_size: BufferSize::Fixed(512),
        };

        let result = device.build_output_stream(
            &config,
            |_data: &mut [f32], _info: &crate::OutputCallbackInfo| {},
            |_err| {},
            None,
        );

        assert!(
            result.is_ok(),
            "BufferSize::Fixed should be supported on iOS via AVAudioSession"
        );
    }
}
