use super::OSStatus;
use super::Stream;
use super::{asbd_from_config, check_os_status, frames_to_duration, host_time_to_stream_instant};
use crate::host::coreaudio::macos::loopback::LoopbackDevice;
use crate::host::coreaudio::macos::StreamInner;
use crate::traits::DeviceTrait;
use crate::{
    BackendSpecificError, BufferSize, BuildStreamError, ChannelCount, Data,
    DefaultStreamConfigError, DeviceId, DeviceIdError, DeviceNameError, InputCallbackInfo,
    OutputCallbackInfo, SampleFormat, SampleRate, StreamConfig, StreamError, SupportedBufferSize,
    SupportedStreamConfig, SupportedStreamConfigRange, SupportedStreamConfigsError,
};
use coreaudio::audio_unit::render_callback::{self, data};
use coreaudio::audio_unit::{AudioUnit, Element, Scope};
use objc2_audio_toolbox::{
    kAudioOutputUnitProperty_CurrentDevice, kAudioOutputUnitProperty_EnableIO,
    kAudioUnitProperty_StreamFormat,
};
use objc2_core_audio::kAudioDevicePropertyDeviceUID;
use objc2_core_audio::kAudioObjectPropertyElementMain;
use objc2_core_audio::{
    kAudioAggregateDeviceClassID, kAudioDevicePropertyAvailableNominalSampleRates,
    kAudioDevicePropertyBufferFrameSize, kAudioDevicePropertyBufferFrameSizeRange,
    kAudioDevicePropertyNominalSampleRate, kAudioDevicePropertyStreamConfiguration,
    kAudioDevicePropertyStreamFormat, kAudioObjectPropertyClass, kAudioObjectPropertyElementMaster,
    kAudioObjectPropertyScopeGlobal, kAudioObjectPropertyScopeInput,
    kAudioObjectPropertyScopeOutput, AudioClassID, AudioDeviceID, AudioObjectGetPropertyData,
    AudioObjectGetPropertyDataSize, AudioObjectID, AudioObjectPropertyAddress,
    AudioObjectPropertyScope, AudioObjectSetPropertyData,
};
use objc2_core_audio_types::{
    AudioBuffer, AudioBufferList, AudioStreamBasicDescription, AudioValueRange,
};
use objc2_core_foundation::CFString;
use objc2_core_foundation::Type;

pub use super::enumerate::{
    default_input_device, default_output_device, SupportedInputConfigs, SupportedOutputConfigs,
};
use std::fmt;
use std::mem::{self, size_of};
use std::ptr::{null, NonNull};
use std::sync::mpsc::{channel, RecvTimeoutError};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use super::invoke_error_callback;
use super::property_listener::AudioObjectPropertyListener;
use coreaudio::audio_unit::macos_helpers::get_device_name;

/// Attempt to set the device sample rate to the provided rate.
/// Return an error if the requested sample rate is not supported by the device.
fn set_sample_rate(
    audio_device_id: AudioObjectID,
    target_sample_rate: SampleRate,
) -> Result<(), BuildStreamError> {
    // Get the current sample rate.
    let mut property_address = AudioObjectPropertyAddress {
        mSelector: kAudioDevicePropertyNominalSampleRate,
        mScope: kAudioObjectPropertyScopeGlobal,
        mElement: kAudioObjectPropertyElementMaster,
    };
    let mut sample_rate: f64 = 0.0;
    let mut data_size = mem::size_of::<f64>() as u32;
    let status = unsafe {
        AudioObjectGetPropertyData(
            audio_device_id,
            NonNull::from(&property_address),
            0,
            null(),
            NonNull::from(&mut data_size),
            NonNull::from(&mut sample_rate).cast(),
        )
    };
    coreaudio::Error::from_os_status(status)?;

    // If the requested sample rate is different to the device sample rate, update the device.
    if sample_rate as u32 != target_sample_rate {
        // Get available sample rate ranges.
        property_address.mSelector = kAudioDevicePropertyAvailableNominalSampleRates;
        let mut data_size = 0u32;
        let status = unsafe {
            AudioObjectGetPropertyDataSize(
                audio_device_id,
                NonNull::from(&property_address),
                0,
                null(),
                NonNull::from(&mut data_size),
            )
        };
        coreaudio::Error::from_os_status(status)?;
        let n_ranges = data_size as usize / mem::size_of::<AudioValueRange>();
        let mut ranges: Vec<AudioValueRange> = Vec::with_capacity(n_ranges);
        let status = unsafe {
            AudioObjectGetPropertyData(
                audio_device_id,
                NonNull::from(&property_address),
                0,
                null(),
                NonNull::from(&mut data_size),
                NonNull::new(ranges.as_mut_ptr()).unwrap().cast(),
            )
        };
        coreaudio::Error::from_os_status(status)?;
        unsafe {
            ranges.set_len(n_ranges);
        }

        // Now that we have the available ranges, pick the one matching the desired rate.
        let sample_rate = target_sample_rate;
        if !ranges
            .iter()
            .any(|r| sample_rate as f64 >= r.mMinimum && sample_rate as f64 <= r.mMaximum)
        {
            return Err(BuildStreamError::StreamConfigNotSupported);
        }

        let (send, recv) = channel::<Result<f64, coreaudio::Error>>();
        let sample_rate_address = AudioObjectPropertyAddress {
            mSelector: kAudioDevicePropertyNominalSampleRate,
            mScope: kAudioObjectPropertyScopeGlobal,
            mElement: kAudioObjectPropertyElementMaster,
        };
        // Send sample rate updates back on a channel.
        let sample_rate_handler = move || {
            let mut rate: f64 = 0.0;
            let mut data_size = mem::size_of::<f64>() as u32;

            let result = unsafe {
                AudioObjectGetPropertyData(
                    audio_device_id,
                    NonNull::from(&sample_rate_address),
                    0,
                    null(),
                    NonNull::from(&mut data_size),
                    NonNull::from(&mut rate).cast(),
                )
            };
            send.send(coreaudio::Error::from_os_status(result).map(|_| rate))
                .ok();
        };

        let listener = AudioObjectPropertyListener::new(
            audio_device_id,
            sample_rate_address,
            sample_rate_handler,
        )?;

        // Finally, set the sample rate.
        property_address.mSelector = kAudioDevicePropertyNominalSampleRate;
        // Set the nominal sample rate using a single f64 as required by CoreAudio.
        let rate = sample_rate as f64;
        let data_size = mem::size_of::<f64>() as u32;
        let status = unsafe {
            AudioObjectSetPropertyData(
                audio_device_id,
                NonNull::from(&property_address),
                0,
                null(),
                data_size,
                NonNull::from(&rate).cast(),
            )
        };
        coreaudio::Error::from_os_status(status)?;

        // Wait for the reported_rate to change.
        //
        // This should not take longer than a few ms, but we timeout after 1 sec just in case.
        // We loop over potentially several events from the channel to ensure
        // that we catch the expected change in sample rate.
        let mut timeout = Duration::from_secs(1);
        let start = Instant::now();

        loop {
            match recv.recv_timeout(timeout) {
                Err(err) => {
                    let description = match err {
                        RecvTimeoutError::Disconnected => {
                            "sample rate listener channel disconnected unexpectedly"
                        }
                        RecvTimeoutError::Timeout => {
                            "timeout waiting for sample rate update for device"
                        }
                    }
                    .to_string();
                    return Err(BackendSpecificError { description }.into());
                }
                Ok(Ok(reported_sample_rate)) => {
                    if reported_sample_rate == target_sample_rate as f64 {
                        break;
                    }
                }
                Ok(Err(_)) => {
                    // TODO: should we consider collecting this error?
                }
            };
            timeout = timeout
                .checked_sub(start.elapsed())
                .unwrap_or(Duration::ZERO);
        }
        listener.remove()?;
    }
    Ok(())
}

fn audio_unit_from_device(device: &Device, input: bool) -> Result<AudioUnit, coreaudio::Error> {
    let output_type = if !input && is_default_output_device(device) {
        coreaudio::audio_unit::IOType::DefaultOutput
    } else {
        coreaudio::audio_unit::IOType::HalOutput
    };
    let mut audio_unit = AudioUnit::new(output_type)?;

    if input {
        // Enable input processing.
        let enable_input = 1u32;
        audio_unit.set_property(
            kAudioOutputUnitProperty_EnableIO,
            Scope::Input,
            Element::Input,
            Some(&enable_input),
        )?;

        // Disable output processing.
        let disable_output = 0u32;
        audio_unit.set_property(
            kAudioOutputUnitProperty_EnableIO,
            Scope::Output,
            Element::Output,
            Some(&disable_output),
        )?;
    }

    // Device selection is a device-level property: always use Scope::Global + Element::Output
    audio_unit.set_property(
        kAudioOutputUnitProperty_CurrentDevice,
        Scope::Global,
        Element::Output,
        Some(&device.audio_device_id),
    )?;

    Ok(audio_unit)
}

fn get_io_buffer_frame_size_range(
    audio_unit: &AudioUnit,
) -> Result<SupportedBufferSize, coreaudio::Error> {
    // Device-level property: always use Scope::Global + Element::Output
    // regardless of whether this audio unit is configured for input or output
    let buffer_size_range: AudioValueRange = audio_unit.get_property(
        kAudioDevicePropertyBufferFrameSizeRange,
        Scope::Global,
        Element::Output,
    )?;

    Ok(SupportedBufferSize::Range {
        min: buffer_size_range.mMinimum as u32,
        max: buffer_size_range.mMaximum as u32,
    })
}

impl DeviceTrait for Device {
    type SupportedInputConfigs = SupportedInputConfigs;
    type SupportedOutputConfigs = SupportedOutputConfigs;
    type Stream = Stream;

    fn description(&self) -> Result<crate::DeviceDescription, DeviceNameError> {
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
        timeout: Option<Duration>,
    ) -> Result<Self::Stream, BuildStreamError>
    where
        D: FnMut(&Data, &InputCallbackInfo) + Send + 'static,
        E: FnMut(StreamError) + Send + 'static,
    {
        Device::build_input_stream_raw(
            self,
            config,
            sample_format,
            data_callback,
            error_callback,
            timeout,
        )
    }

    fn build_output_stream_raw<D, E>(
        &self,
        config: &StreamConfig,
        sample_format: SampleFormat,
        data_callback: D,
        error_callback: E,
        timeout: Option<Duration>,
    ) -> Result<Self::Stream, BuildStreamError>
    where
        D: FnMut(&mut Data, &OutputCallbackInfo) + Send + 'static,
        E: FnMut(StreamError) + Send + 'static,
    {
        Device::build_output_stream_raw(
            self,
            config,
            sample_format,
            data_callback,
            error_callback,
            timeout,
        )
    }
}

#[derive(Clone, Eq, Hash, PartialEq)]
pub struct Device {
    pub(crate) audio_device_id: AudioDeviceID,
}

fn is_default_input_device(device: &Device) -> bool {
    default_input_device().is_some_and(|d| d.audio_device_id == device.audio_device_id)
}

fn is_default_output_device(device: &Device) -> bool {
    default_output_device().is_some_and(|d| d.audio_device_id == device.audio_device_id)
}

impl Device {
    /// Construct a new device given its ID.
    /// Useful for constructing hidden devices.
    pub fn new(audio_device_id: AudioDeviceID) -> Self {
        Self { audio_device_id }
    }

    /// Checks if this device is an aggregate device.
    ///
    /// Aggregate devices combine multiple physical devices into a single logical device.
    fn is_aggregate_device(&self) -> bool {
        let property_address = AudioObjectPropertyAddress {
            mSelector: kAudioObjectPropertyClass,
            mScope: kAudioObjectPropertyScopeGlobal,
            mElement: kAudioObjectPropertyElementMain,
        };

        let mut class_id: AudioClassID = 0;
        let data_size = size_of::<AudioClassID>() as u32;

        // SAFETY: AudioObjectGetPropertyData is documented to write an AudioClassID
        // for kAudioObjectPropertyClass. We check the status before using the value.
        let status = unsafe {
            AudioObjectGetPropertyData(
                self.audio_device_id,
                NonNull::from(&property_address),
                0,
                null(),
                NonNull::from(&data_size),
                NonNull::from(&mut class_id).cast(),
            )
        };

        // If successful, check if it's an aggregate device
        status == 0 && class_id == kAudioAggregateDeviceClassID
    }

    fn description(&self) -> Result<crate::DeviceDescription, DeviceNameError> {
        let name = get_device_name(self.audio_device_id).map_err(|err| {
            DeviceNameError::BackendSpecific {
                err: BackendSpecificError {
                    description: err.to_string(),
                },
            }
        })?;

        let input_configs = self
            .supported_input_configs()
            .map(|configs| configs.count() as ChannelCount)
            .ok();
        let output_configs = self
            .supported_output_configs()
            .map(|configs| configs.count() as ChannelCount)
            .ok();

        let direction =
            crate::device_description::direction_from_counts(input_configs, output_configs);

        let mut builder = crate::DeviceDescriptionBuilder::new(name).direction(direction);

        // Check if this is an aggregate device
        if self.is_aggregate_device() {
            builder = builder.interface_type(crate::InterfaceType::Aggregate);
        }

        Ok(builder.build())
    }

    fn id(&self) -> Result<DeviceId, DeviceIdError> {
        let property_address = AudioObjectPropertyAddress {
            mSelector: kAudioDevicePropertyDeviceUID,
            mScope: kAudioObjectPropertyScopeGlobal,
            mElement: kAudioObjectPropertyElementMain,
        };

        // CFString is copied from the audio object, use wrap_under_create_rule
        let mut uid: *mut CFString = std::ptr::null_mut();
        let mut data_size = size_of::<*mut CFString>() as u32;

        // SAFETY: AudioObjectGetPropertyData is documented to write a CFString pointer
        // for kAudioDevicePropertyDeviceUID. We check the status code before use.
        let status = unsafe {
            AudioObjectGetPropertyData(
                self.audio_device_id,
                NonNull::from(&property_address),
                0,
                null(),
                NonNull::from(&mut data_size),
                NonNull::from(&mut uid).cast(),
            )
        };
        check_os_status(status)?;

        // SAFETY: Status was successful, meaning the API call succeeded.
        // We now check if the returned uid is non-null before use.
        if !uid.is_null() {
            let uid_string = unsafe { CFString::wrap_under_create_rule(uid).to_string() };
            Ok(DeviceId(crate::platform::HostId::CoreAudio, uid_string))
        } else {
            Err(DeviceIdError::BackendSpecific {
                err: BackendSpecificError {
                    description: "Device UID is null".to_string(),
                },
            })
        }
    }

    // Logic re-used between `supported_input_configs` and `supported_output_configs`.
    #[allow(clippy::cast_ptr_alignment)]
    fn supported_configs(
        &self,
        scope: AudioObjectPropertyScope,
    ) -> Result<SupportedOutputConfigs, SupportedStreamConfigsError> {
        let mut property_address = AudioObjectPropertyAddress {
            mSelector: kAudioDevicePropertyStreamConfiguration,
            mScope: scope,
            mElement: kAudioObjectPropertyElementMaster,
        };

        unsafe {
            // Retrieve the devices audio buffer list.
            let mut data_size = 0u32;
            let status = AudioObjectGetPropertyDataSize(
                self.audio_device_id,
                NonNull::from(&property_address),
                0,
                null(),
                NonNull::from(&mut data_size),
            );
            check_os_status(status)?;

            let mut audio_buffer_list: Vec<u8> = vec![];
            audio_buffer_list.reserve_exact(data_size as usize);
            let status = AudioObjectGetPropertyData(
                self.audio_device_id,
                NonNull::from(&property_address),
                0,
                null(),
                NonNull::from(&mut data_size),
                NonNull::new(audio_buffer_list.as_mut_ptr()).unwrap().cast(),
            );
            check_os_status(status)?;

            let audio_buffer_list = audio_buffer_list.as_mut_ptr() as *mut AudioBufferList;

            // Read the number of buffers without assuming alignment (avoid UB).
            let nb_ptr = core::ptr::addr_of!((*audio_buffer_list).mNumberBuffers);
            let n_buffers = core::ptr::read_unaligned(nb_ptr) as usize;
            // If there are no buffers, skip.
            if n_buffers == 0 {
                return Ok(vec![].into_iter());
            }

            // Count the number of channels as the sum of all channels in all output buffers.
            let first_buf_ptr =
                core::ptr::addr_of!((*audio_buffer_list).mBuffers) as *const AudioBuffer;
            let mut n_channels = 0usize;
            for i in 0..n_buffers {
                let buf_ptr = first_buf_ptr.add(i);
                // Read potentially unaligned
                let buf: AudioBuffer = core::ptr::read_unaligned(buf_ptr);
                n_channels += buf.mNumberChannels as usize;
            }

            // TODO: macOS should support U8, I16, I32, F32 and F64. This should allow for using
            // I16 but just use F32 for now as it's the default anyway.
            let sample_format = SampleFormat::F32;

            // Get available sample rate ranges.
            // The property "kAudioDevicePropertyAvailableNominalSampleRates" returns a list of pairs of
            // minimum and maximum sample rates but most of the devices returns pairs of same values though the underlying mechanism is unclear.
            // This may cause issues when, for example, sorting the configs by the sample rates.
            // We follows the implementation of RtAudio, which returns single element of config
            // when all the pairs have the same values and returns multiple elements otherwise.
            // See https://github.com/thestk/rtaudio/blob/master/RtAudio.cpp#L1369C1-L1375C39

            property_address.mSelector = kAudioDevicePropertyAvailableNominalSampleRates;
            let mut data_size = 0u32;
            let status = AudioObjectGetPropertyDataSize(
                self.audio_device_id,
                NonNull::from(&property_address),
                0,
                null(),
                NonNull::from(&mut data_size),
            );
            check_os_status(status)?;

            let n_ranges = data_size as usize / mem::size_of::<AudioValueRange>();
            let mut ranges: Vec<AudioValueRange> = Vec::with_capacity(n_ranges);
            let status = AudioObjectGetPropertyData(
                self.audio_device_id,
                NonNull::from(&property_address),
                0,
                null(),
                NonNull::from(&mut data_size),
                NonNull::new(ranges.as_mut_ptr()).unwrap().cast(),
            );
            check_os_status(status)?;

            ranges.set_len(n_ranges);

            #[allow(non_upper_case_globals)]
            let input = match scope {
                kAudioObjectPropertyScopeInput => Ok(true),
                kAudioObjectPropertyScopeOutput => Ok(false),
                _ => Err(BackendSpecificError {
                    description: format!("unexpected scope (neither input nor output): {scope:?}"),
                }),
            }?;
            let audio_unit = audio_unit_from_device(self, input)?;
            let buffer_size = get_io_buffer_frame_size_range(&audio_unit)?;

            // Collect the supported formats for the device.

            let contains_different_sample_rates = ranges.iter().any(|r| r.mMinimum != r.mMaximum);
            if ranges.is_empty() {
                Ok(vec![].into_iter())
            } else if contains_different_sample_rates {
                let res = ranges.iter().map(|range| SupportedStreamConfigRange {
                    channels: n_channels as ChannelCount,
                    min_sample_rate: range.mMinimum as u32,
                    max_sample_rate: range.mMaximum as u32,
                    buffer_size,
                    sample_format,
                });
                Ok(res.collect::<Vec<_>>().into_iter())
            } else {
                let fmt = SupportedStreamConfigRange {
                    channels: n_channels as ChannelCount,
                    min_sample_rate: ranges
                        .iter()
                        .map(|v| v.mMinimum as u32)
                        .min()
                        .expect("the list must not be empty"),
                    max_sample_rate: ranges
                        .iter()
                        .map(|v| v.mMaximum as u32)
                        .max()
                        .expect("the list must not be empty"),
                    buffer_size,
                    sample_format,
                };

                Ok(vec![fmt].into_iter())
            }
        }
    }

    fn supported_input_configs(
        &self,
    ) -> Result<SupportedOutputConfigs, SupportedStreamConfigsError> {
        self.supported_configs(kAudioObjectPropertyScopeInput)
    }

    fn supported_output_configs(
        &self,
    ) -> Result<SupportedOutputConfigs, SupportedStreamConfigsError> {
        self.supported_configs(kAudioObjectPropertyScopeOutput)
    }

    fn default_config(
        &self,
        scope: AudioObjectPropertyScope,
    ) -> Result<SupportedStreamConfig, DefaultStreamConfigError> {
        fn default_config_error_from_os_status(
            status: OSStatus,
        ) -> Result<(), DefaultStreamConfigError> {
            let err = match coreaudio::Error::from_os_status(status) {
                Err(err) => err,
                Ok(_) => return Ok(()),
            };
            match err {
                coreaudio::Error::AudioUnit(
                    coreaudio::error::AudioUnitError::FormatNotSupported,
                )
                | coreaudio::Error::AudioCodec(_)
                | coreaudio::Error::AudioFormat(_) => {
                    Err(DefaultStreamConfigError::StreamTypeNotSupported)
                }
                coreaudio::Error::AudioUnit(coreaudio::error::AudioUnitError::NoConnection) => {
                    Err(DefaultStreamConfigError::DeviceNotAvailable)
                }
                err => {
                    let description = format!("{err}");
                    let err = BackendSpecificError { description };
                    Err(err.into())
                }
            }
        }

        let property_address = AudioObjectPropertyAddress {
            mSelector: kAudioDevicePropertyStreamFormat,
            mScope: scope,
            mElement: kAudioObjectPropertyElementMaster,
        };

        unsafe {
            let mut asbd: AudioStreamBasicDescription = mem::zeroed();
            let mut data_size = mem::size_of::<AudioStreamBasicDescription>() as u32;
            let status = AudioObjectGetPropertyData(
                self.audio_device_id,
                NonNull::from(&property_address),
                0,
                null(),
                NonNull::from(&mut data_size),
                NonNull::from(&mut asbd).cast(),
            );
            default_config_error_from_os_status(status)?;

            let sample_format = {
                let audio_format = coreaudio::audio_unit::AudioFormat::from_format_and_flag(
                    asbd.mFormatID,
                    Some(asbd.mFormatFlags),
                );
                let flags = match audio_format {
                    Some(coreaudio::audio_unit::AudioFormat::LinearPCM(flags)) => flags,
                    _ => return Err(DefaultStreamConfigError::StreamTypeNotSupported),
                };
                let maybe_sample_format =
                    coreaudio::audio_unit::SampleFormat::from_flags_and_bits_per_sample(
                        flags,
                        asbd.mBitsPerChannel,
                    );
                match maybe_sample_format {
                    Some(coreaudio::audio_unit::SampleFormat::F32) => SampleFormat::F32,
                    Some(coreaudio::audio_unit::SampleFormat::I16) => SampleFormat::I16,
                    _ => return Err(DefaultStreamConfigError::StreamTypeNotSupported),
                }
            };

            #[allow(non_upper_case_globals)]
            let input = match scope {
                kAudioObjectPropertyScopeInput => Ok(true),
                kAudioObjectPropertyScopeOutput => Ok(false),
                _ => Err(BackendSpecificError {
                    description: format!("unexpected scope (neither input nor output): {scope:?}"),
                }),
            }?;
            let audio_unit = audio_unit_from_device(self, input)?;
            let buffer_size = get_io_buffer_frame_size_range(&audio_unit)?;

            let config = SupportedStreamConfig {
                sample_rate: asbd.mSampleRate as _,
                channels: asbd.mChannelsPerFrame as _,
                buffer_size,
                sample_format,
            };
            Ok(config)
        }
    }

    fn default_input_config(&self) -> Result<SupportedStreamConfig, DefaultStreamConfigError> {
        self.default_config(kAudioObjectPropertyScopeInput)
    }

    fn default_output_config(&self) -> Result<SupportedStreamConfig, DefaultStreamConfigError> {
        self.default_config(kAudioObjectPropertyScopeOutput)
    }

    /// Check if this device supports input (recording).
    fn supports_input(&self) -> bool {
        // Check if the device has input channels by trying to get its input configuration
        self.supported_input_configs()
            .map(|mut configs| configs.next().is_some())
            .unwrap_or(false)
    }
}

impl fmt::Debug for Device {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Device")
            .field("audio_device_id", &self.audio_device_id)
            .field("name", &self.name())
            .finish()
    }
}

impl Device {
    #[allow(clippy::cast_ptr_alignment)]
    #[allow(clippy::while_immutable_condition)]
    #[allow(clippy::float_cmp)]
    fn build_input_stream_raw<D, E>(
        &self,
        config: &StreamConfig,
        sample_format: SampleFormat,
        mut data_callback: D,
        error_callback: E,
        _timeout: Option<Duration>,
    ) -> Result<Stream, BuildStreamError>
    where
        D: FnMut(&Data, &InputCallbackInfo) + Send + 'static,
        E: FnMut(StreamError) + Send + 'static,
    {
        // The scope and element for working with a device's input stream.
        let scope = Scope::Output;
        let element = Element::Input;

        // Potentially change the device sample rate to match the config.
        set_sample_rate(self.audio_device_id, config.sample_rate)?;

        let mut loopback_aggregate: Option<LoopbackDevice> = None;
        let mut audio_unit = if self.supports_input() {
            audio_unit_from_device(self, true)?
        } else {
            loopback_aggregate.replace(LoopbackDevice::from_device(self)?);
            audio_unit_from_device(&loopback_aggregate.as_ref().unwrap().aggregate_device, true)?
        };

        // Configure stream format and buffer size for predictable callback behavior.
        configure_stream_format_and_buffer(&mut audio_unit, config, sample_format, scope, element)?;

        let error_callback = Arc::new(Mutex::new(error_callback));
        let error_callback_disconnect = error_callback.clone();

        // Register the callback that is being called by coreaudio whenever it needs data to be
        // fed to the audio buffer.
        let (bytes_per_channel, sample_rate, device_buffer_frames) =
            setup_callback_vars(&audio_unit, config, sample_format);

        type Args = render_callback::Args<data::Raw>;
        audio_unit.set_input_callback(move |args: Args| unsafe {
            // SAFETY: We configure the stream format as interleaved (via asbd_from_config which
            // does not set kAudioFormatFlagIsNonInterleaved). Interleaved format always has
            // exactly one buffer containing all channels, so mBuffers[0] is always valid.
            let AudioBuffer {
                mNumberChannels: channels,
                mDataByteSize: data_byte_size,
                mData: data,
            } = (*args.data.data).mBuffers[0];

            let data = data as *mut ();
            let len = data_byte_size as usize / bytes_per_channel;
            let data = Data::from_parts(data, len, sample_format);

            let callback = match host_time_to_stream_instant(args.time_stamp.mHostTime) {
                Err(err) => {
                    invoke_error_callback(&error_callback, err.into());
                    return Err(());
                }
                Ok(cb) => cb,
            };
            let buffer_frames = len / channels as usize;
            // Use device buffer size for latency calculation if available
            let latency_frames = device_buffer_frames.unwrap_or(
                // Fallback to callback buffer size if device buffer size is unknown
                // (may overestimate latency for BufferSize::Default)
                buffer_frames,
            );
            let delay = frames_to_duration(latency_frames, sample_rate);
            let capture = callback
                .sub(delay)
                .expect("`capture` occurs before origin of alsa `StreamInstant`");
            let timestamp = crate::InputStreamTimestamp { callback, capture };

            let info = InputCallbackInfo { timestamp };
            data_callback(&data, &info);
            Ok(())
        })?;

        // Create error callback for stream - either dummy or real based on device type
        let error_callback_for_stream: super::ErrorCallback = if is_default_input_device(self) {
            Box::new(|_: StreamError| {})
        } else {
            let error_callback_clone = error_callback_disconnect.clone();
            Box::new(move |err: StreamError| {
                invoke_error_callback(&error_callback_clone, err);
            })
        };

        let stream = Stream::new(
            StreamInner {
                playing: true,
                audio_unit,
                device_id: self.audio_device_id,
                _loopback_device: loopback_aggregate,
            },
            error_callback_for_stream,
        )?;

        stream
            .inner
            .lock()
            .map_err(|_| BuildStreamError::BackendSpecific {
                err: BackendSpecificError {
                    description: "A cpal stream operation panicked while holding the lock - this is a bug, please report it".to_string(),
                },
            })?
            .audio_unit
            .start()?;

        Ok(stream)
    }

    fn build_output_stream_raw<D, E>(
        &self,
        config: &StreamConfig,
        sample_format: SampleFormat,
        mut data_callback: D,
        error_callback: E,
        _timeout: Option<Duration>,
    ) -> Result<Stream, BuildStreamError>
    where
        D: FnMut(&mut Data, &OutputCallbackInfo) + Send + 'static,
        E: FnMut(StreamError) + Send + 'static,
    {
        let mut audio_unit = audio_unit_from_device(self, false)?;

        // The scope and element for working with a device's output stream.
        let scope = Scope::Input;
        let element = Element::Output;

        // Configure device buffer (see comprehensive documentation in input stream above)
        configure_stream_format_and_buffer(&mut audio_unit, config, sample_format, scope, element)?;

        let error_callback = Arc::new(Mutex::new(error_callback));
        let error_callback_disconnect = error_callback.clone();

        // Register the callback that is being called by coreaudio whenever it needs data to be
        // fed to the audio buffer.
        let (bytes_per_channel, sample_rate, device_buffer_frames) =
            setup_callback_vars(&audio_unit, config, sample_format);

        type Args = render_callback::Args<data::Raw>;
        audio_unit.set_render_callback(move |args: Args| unsafe {
            // SAFETY: We configure the stream format as interleaved (via asbd_from_config which
            // does not set kAudioFormatFlagIsNonInterleaved). Interleaved format always has
            // exactly one buffer containing all channels, so mBuffers[0] is always valid.
            let AudioBuffer {
                mNumberChannels: channels,
                mDataByteSize: data_byte_size,
                mData: data,
            } = (*args.data.data).mBuffers[0];

            let data = data as *mut ();
            let len = data_byte_size as usize / bytes_per_channel;
            let mut data = Data::from_parts(data, len, sample_format);

            let callback = match host_time_to_stream_instant(args.time_stamp.mHostTime) {
                Err(err) => {
                    invoke_error_callback(&error_callback, err.into());
                    return Err(());
                }
                Ok(cb) => cb,
            };
            let buffer_frames = len / channels as usize;
            // Use device buffer size for latency calculation if available
            let latency_frames = device_buffer_frames.unwrap_or(
                // Fallback to callback buffer size if device buffer size is unknown
                // (may overestimate latency for BufferSize::Default)
                buffer_frames,
            );
            let delay = frames_to_duration(latency_frames, sample_rate);
            let playback = callback
                .add(delay)
                .expect("`playback` occurs beyond representation supported by `StreamInstant`");
            let timestamp = crate::OutputStreamTimestamp { callback, playback };

            let info = OutputCallbackInfo { timestamp };
            data_callback(&mut data, &info);
            Ok(())
        })?;

        // Create error callback for stream - either dummy or real based on device type
        let error_callback_for_stream: super::ErrorCallback = if is_default_output_device(self) {
            Box::new(|_: StreamError| {})
        } else {
            let error_callback_clone = error_callback_disconnect.clone();
            Box::new(move |err: StreamError| {
                invoke_error_callback(&error_callback_clone, err);
            })
        };

        let stream = Stream::new(
            StreamInner {
                playing: true,
                audio_unit,
                device_id: self.audio_device_id,
                _loopback_device: None,
            },
            error_callback_for_stream,
        )?;

        stream
            .inner
            .lock()
            .map_err(|_| BuildStreamError::BackendSpecific {
                err: BackendSpecificError {
                    description: "A cpal stream operation panicked while holding the lock - this is a bug, please report it".to_string(),
                },
            })?
            .audio_unit
            .start()?;

        Ok(stream)
    }
}

/// Configure stream format and buffer size for CoreAudio stream.
///
/// This handles the common setup tasks for both input and output streams:
/// - Sets the stream format (ASBD)
/// - Configures buffer size for Fixed buffer size requests
fn configure_stream_format_and_buffer(
    audio_unit: &mut AudioUnit,
    config: &StreamConfig,
    sample_format: SampleFormat,
    scope: Scope,
    element: Element,
) -> Result<(), BuildStreamError> {
    // Set the stream format using stream-specific scope/element
    // - Input streams: scope=Output, element=Input (configuring output format of input element)
    // - Output streams: scope=Input, element=Output (configuring input format of output element)
    let asbd = asbd_from_config(config, sample_format);
    audio_unit.set_property(kAudioUnitProperty_StreamFormat, scope, element, Some(&asbd))?;

    // Configure device buffer size if requested
    if let BufferSize::Fixed(buffer_size) = config.buffer_size {
        // IMPORTANT: Buffer frame size is a DEVICE-LEVEL property, not stream-specific.
        // Unlike stream format above, we ALWAYS use Scope::Global + Element::Output
        // for device properties, regardless of whether this is an input or output stream.
        // This is consistent with other device properties like:
        // - kAudioOutputUnitProperty_CurrentDevice
        // - kAudioDevicePropertyBufferFrameSizeRange
        // The Element::Output here doesn't mean "output stream only" - it's the
        // canonical element used for device-wide properties in Core Audio.
        audio_unit.set_property(
            kAudioDevicePropertyBufferFrameSize,
            Scope::Global,
            Element::Output,
            Some(&buffer_size),
        )?;
    }

    Ok(())
}

/// Setup common callback variables and query device buffer size.
///
/// Returns (bytes_per_channel, sample_rate, device_buffer_frames)
fn setup_callback_vars(
    audio_unit: &AudioUnit,
    config: &StreamConfig,
    sample_format: SampleFormat,
) -> (usize, crate::SampleRate, Option<usize>) {
    let bytes_per_channel = sample_format.sample_size();
    let sample_rate = config.sample_rate;

    // Query device buffer size for latency calculation
    let device_buffer_frames = get_device_buffer_frame_size(audio_unit).ok();

    (bytes_per_channel, sample_rate, device_buffer_frames)
}

/// Query the current device buffer frame size from CoreAudio.
///
/// Buffer frame size is a device-level property that always uses Scope::Global + Element::Output,
/// regardless of whether the audio unit is configured for input or output streams.
fn get_device_buffer_frame_size(audio_unit: &AudioUnit) -> Result<usize, coreaudio::Error> {
    // Device-level property: always use Scope::Global + Element::Output
    // This is consistent with how we set the buffer size and query the buffer size range
    let frames: u32 = audio_unit.get_property(
        kAudioDevicePropertyBufferFrameSize,
        Scope::Global,
        Element::Output,
    )?;
    Ok(frames as usize)
}
