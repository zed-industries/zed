#![allow(deprecated)]
//! This is a collection of helper functions for performing common tasks on macOS.
//! These functions are only implemented for macOS, not iOS.
use crate::error::Error;
use std::collections::VecDeque;
use std::ptr::{null, NonNull};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{channel, Sender};
use std::sync::Mutex;
use std::time::Duration;
use std::{mem, thread};

use libc::pid_t;
use objc2_audio_toolbox::{
    kAudioOutputUnitProperty_CurrentDevice, kAudioOutputUnitProperty_EnableIO,
};
use objc2_core_audio::{
    kAudioDevicePropertyAvailableNominalSampleRates, kAudioDevicePropertyDeviceIsAlive,
    kAudioDevicePropertyDeviceNameCFString, kAudioDevicePropertyHogMode,
    kAudioDevicePropertyNominalSampleRate, kAudioDevicePropertyScopeOutput,
    kAudioDevicePropertyStreamConfiguration, kAudioHardwareNoError,
    kAudioHardwarePropertyDefaultInputDevice, kAudioHardwarePropertyDefaultOutputDevice,
    kAudioHardwarePropertyDevices, kAudioObjectPropertyElementMaster,
    kAudioObjectPropertyElementWildcard, kAudioObjectPropertyScopeGlobal,
    kAudioObjectPropertyScopeInput, kAudioObjectPropertyScopeOutput, kAudioObjectSystemObject,
    kAudioStreamPropertyAvailablePhysicalFormats, kAudioStreamPropertyPhysicalFormat,
    AudioDeviceID, AudioObjectAddPropertyListener, AudioObjectGetPropertyData,
    AudioObjectGetPropertyDataSize, AudioObjectID, AudioObjectPropertyAddress,
    AudioObjectPropertyListenerProc, AudioObjectPropertyScope, AudioObjectRemovePropertyListener,
    AudioObjectSetPropertyData, AudioStreamRangedDescription,
};
use objc2_core_audio_types::{AudioBufferList, AudioStreamBasicDescription, AudioValueRange};
use objc2_core_foundation::CFString;

use crate::audio_unit::audio_format::{AudioFormat, LinearPcmFlags};
use crate::audio_unit::sample_format::SampleFormat;
use crate::audio_unit::stream_format::StreamFormat;
use crate::audio_unit::{AudioUnit, Element, IOType, Scope};
use crate::OSStatus;

/// Helper function to get the device id of the default input or output device.
pub fn get_default_device_id(input: bool) -> Option<AudioDeviceID> {
    let selector = if input {
        kAudioHardwarePropertyDefaultInputDevice
    } else {
        kAudioHardwarePropertyDefaultOutputDevice
    };
    let property_address = AudioObjectPropertyAddress {
        mSelector: selector,
        mScope: kAudioObjectPropertyScopeGlobal,
        mElement: kAudioObjectPropertyElementMaster,
    };

    let mut audio_device_id: AudioDeviceID = 0;
    let data_size = mem::size_of::<AudioDeviceID>() as u32;
    let status = unsafe {
        AudioObjectGetPropertyData(
            kAudioObjectSystemObject as AudioObjectID,
            NonNull::from(&property_address),
            0,
            null(),
            NonNull::from(&data_size),
            NonNull::from(&mut audio_device_id).cast(),
        )
    };
    if status != kAudioHardwareNoError as i32 {
        return None;
    }

    Some(audio_device_id)
}

/// Find the device id for a device name.
/// Set `input` to `true` to find a playback device, or `false` for a capture device.
pub fn get_device_id_from_name(name: &str, input: bool) -> Option<AudioDeviceID> {
    let scope = match input {
        false => Scope::Output,
        true => Scope::Input,
    };
    if let Ok(all_ids) = get_audio_device_ids() {
        return all_ids
            .iter()
            .find(|id| {
                get_device_name(**id).unwrap_or_default() == name
                    && get_audio_device_supports_scope(**id, scope).unwrap_or_default()
            })
            .copied();
    }
    None
}

/// Create an AudioUnit instance from a device id.
/// Set `input` to `true` to create a playback device, or `false` for a capture device.
pub fn audio_unit_from_device_id(
    device_id: AudioDeviceID,
    input: bool,
) -> Result<AudioUnit, Error> {
    let mut audio_unit = AudioUnit::new(IOType::HalOutput)?;

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

    audio_unit.set_property(
        kAudioOutputUnitProperty_CurrentDevice,
        Scope::Global,
        Element::Output,
        Some(&device_id),
    )?;

    Ok(audio_unit)
}

/// List all audio device ids on the system.
pub fn get_audio_device_ids_for_scope(scope: Scope) -> Result<Vec<AudioDeviceID>, Error> {
    let dev_scope = match scope {
        Scope::Input => kAudioObjectPropertyScopeInput,
        Scope::Output => kAudioObjectPropertyScopeOutput,
        _ => kAudioObjectPropertyScopeGlobal,
    };
    let property_address = AudioObjectPropertyAddress {
        mSelector: kAudioHardwarePropertyDevices,
        mScope: dev_scope,
        mElement: kAudioObjectPropertyElementMaster,
    };

    macro_rules! try_status_or_return {
        ($status:expr) => {
            if $status != kAudioHardwareNoError as i32 {
                return Err(Error::Unknown($status));
            }
        };
    }

    let mut data_size = 0u32;
    let status = unsafe {
        AudioObjectGetPropertyDataSize(
            kAudioObjectSystemObject as AudioObjectID,
            NonNull::from(&property_address),
            0,
            null(),
            NonNull::from(&mut data_size),
        )
    };
    try_status_or_return!(status);

    let device_count = data_size / mem::size_of::<AudioDeviceID>() as u32;
    let mut audio_devices = vec![];
    audio_devices.reserve_exact(device_count as usize);
    unsafe { audio_devices.set_len(device_count as usize) };

    let status = unsafe {
        AudioObjectGetPropertyData(
            kAudioObjectSystemObject as AudioObjectID,
            NonNull::from(&property_address),
            0,
            null(),
            NonNull::from(&data_size),
            NonNull::new(audio_devices.as_mut_ptr()).unwrap().cast(),
        )
    };
    try_status_or_return!(status);
    Ok(audio_devices)
}

pub fn get_audio_device_ids() -> Result<Vec<AudioDeviceID>, Error> {
    get_audio_device_ids_for_scope(Scope::Global)
}

#[test]
fn test_get_audio_device_ids() {
    let _ = get_audio_device_ids().expect("Failed to get audio device ids");
}

#[test]
fn test_get_audio_device_ids_for_scope() {
    for scope in &[
        Scope::Global,
        Scope::Input,
        Scope::Output,
        Scope::Group,
        Scope::Part,
        Scope::Note,
        Scope::Layer,
        Scope::LayerItem,
    ] {
        let _ = get_audio_device_ids_for_scope(*scope).expect("Failed to get audio device ids");
    }
}

/// does this device support input / ouptut?
pub fn get_audio_device_supports_scope(devid: AudioDeviceID, scope: Scope) -> Result<bool, Error> {
    let dev_scope: AudioObjectPropertyScope = match scope {
        Scope::Input => kAudioObjectPropertyScopeInput,
        Scope::Output => kAudioObjectPropertyScopeOutput,
        _ => kAudioObjectPropertyScopeGlobal,
    };
    let property_address = AudioObjectPropertyAddress {
        mSelector: kAudioDevicePropertyStreamConfiguration,
        mScope: dev_scope,
        mElement: kAudioObjectPropertyElementWildcard,
    };

    macro_rules! try_status_or_return {
        ($status:expr) => {
            if $status != kAudioHardwareNoError as i32 {
                return Err(Error::Unknown($status));
            }
        };
    }

    let mut data_size = 0u32;
    let status = unsafe {
        AudioObjectGetPropertyDataSize(
            devid,
            NonNull::from(&property_address),
            0,
            null(),
            NonNull::from(&mut data_size),
        )
    };
    try_status_or_return!(status);

    let mut bfrs: Vec<u8> = Vec::with_capacity(data_size as usize);
    let buffers = bfrs.as_mut_ptr() as *mut AudioBufferList;
    unsafe {
        let status = AudioObjectGetPropertyData(
            devid,
            NonNull::from(&property_address),
            0,
            null(),
            NonNull::from(&data_size),
            NonNull::new(buffers).unwrap().cast(),
        );
        if status != kAudioHardwareNoError as i32 {
            return Err(Error::Unknown(status));
        }

        for i in 0..(*buffers).mNumberBuffers {
            let buf = (*buffers).mBuffers[i as usize];
            if buf.mNumberChannels > 0 {
                return Ok(true);
            }
        }
    }
    Ok(false)
}

/// Get the device name for a device id.
pub fn get_device_name(device_id: AudioDeviceID) -> Result<String, Error> {
    let property_address = AudioObjectPropertyAddress {
        mSelector: kAudioDevicePropertyDeviceNameCFString,
        mScope: kAudioDevicePropertyScopeOutput,
        mElement: kAudioObjectPropertyElementMaster,
    };

    macro_rules! try_status_or_return {
        ($status:expr) => {
            if $status != kAudioHardwareNoError as i32 {
                return Err(Error::Unknown($status));
            }
        };
    }

    let mut device_name: *const CFString = null();
    let data_size = mem::size_of::<*const CFString>() as u32;
    unsafe {
        let status = AudioObjectGetPropertyData(
            device_id,
            NonNull::from(&property_address),
            0,
            null(),
            NonNull::from(&data_size),
            NonNull::from(&mut device_name).cast(),
        );
        try_status_or_return!(status);

        Ok((&*device_name).to_string())
    }
}

/// Change the sample rate of a device.
/// Adapted from CPAL.
pub fn set_device_sample_rate(device_id: AudioDeviceID, new_rate: f64) -> Result<(), Error> {
    // Check whether or not we need to change the device sample rate to suit the one specified for the stream.
    unsafe {
        // Get the current sample rate.
        let mut property_address = AudioObjectPropertyAddress {
            mSelector: kAudioDevicePropertyNominalSampleRate,
            mScope: kAudioObjectPropertyScopeGlobal,
            mElement: kAudioObjectPropertyElementMaster,
        };
        let mut sample_rate: f64 = 0.0;
        let data_size = mem::size_of::<f64>() as u32;
        let status = AudioObjectGetPropertyData(
            device_id,
            NonNull::from(&property_address),
            0,
            null(),
            NonNull::from(&data_size),
            NonNull::from(&mut sample_rate).cast(),
        );
        Error::from_os_status(status)?;

        // If the requested sample rate is different to the device sample rate, update the device.
        if sample_rate as u32 != new_rate as u32 {
            // Get available sample rate ranges.
            property_address.mSelector = kAudioDevicePropertyAvailableNominalSampleRates;
            let mut data_size = 0u32;
            let status = AudioObjectGetPropertyDataSize(
                device_id,
                NonNull::from(&property_address),
                0,
                null(),
                NonNull::from(&mut data_size),
            );
            Error::from_os_status(status)?;
            let n_ranges = data_size as usize / mem::size_of::<AudioValueRange>();
            let mut ranges: Vec<AudioValueRange> = vec![];
            ranges.reserve_exact(n_ranges as usize);
            ranges.set_len(n_ranges);
            let status = AudioObjectGetPropertyData(
                device_id,
                NonNull::from(&property_address),
                0,
                null(),
                NonNull::from(&data_size),
                NonNull::new(ranges.as_mut_ptr()).unwrap().cast(),
            );
            Error::from_os_status(status)?;

            // Now that we have the available ranges, pick the one matching the desired rate.
            let new_rate_integer = new_rate as u32;
            let maybe_index = ranges.iter().position(|r| {
                r.mMinimum as u32 == new_rate_integer && r.mMaximum as u32 == new_rate_integer
            });
            let range_index = match maybe_index {
                None => return Err(Error::UnsupportedSampleRate),
                Some(i) => i,
            };

            // Update the property selector to specify the nominal sample rate.
            property_address.mSelector = kAudioDevicePropertyNominalSampleRate;

            // Add a listener to know when the sample rate changes.
            // Since the listener implements Drop, we don't need to manually unregister this later.
            let (sender, receiver) = channel();
            let mut listener = RateListener::new(device_id, Some(sender));
            listener.register()?;

            // Finally, set the sample rate.
            let status = AudioObjectSetPropertyData(
                device_id,
                NonNull::from(&property_address),
                0,
                null(),
                data_size,
                NonNull::from(&ranges[range_index]).cast(),
            );
            Error::from_os_status(status)?;

            // Wait for the reported_rate to change.
            //
            // This sometimes takes up to half a second, timeout after 2 sec to have a little margin.
            let timer = ::std::time::Instant::now();
            loop {
                if let Ok(reported_rate) = receiver.recv_timeout(Duration::from_millis(100)) {
                    if new_rate as usize == reported_rate as usize {
                        break;
                    }
                }
                if timer.elapsed() > Duration::from_secs(2) {
                    return Err(Error::UnsupportedSampleRate);
                }
            }
        };
        Ok(())
    }
}

/// Find the closest match of the physical formats to the provided `StreamFormat`.
/// This function will pick the first format it finds that supports the provided sample format, rate and number of channels.
/// The provided format flags in the `StreamFormat` are ignored.
pub fn find_matching_physical_format(
    device_id: AudioDeviceID,
    stream_format: StreamFormat,
) -> Option<AudioStreamBasicDescription> {
    if let Ok(all_formats) = get_supported_physical_stream_formats(device_id) {
        let requested_samplerate = stream_format.sample_rate as usize;
        let requested_bits = stream_format.sample_format.size_in_bits();
        let requested_float = stream_format.sample_format == SampleFormat::F32;
        let requested_channels = stream_format.channels;
        for fmt in all_formats {
            let min_rate = fmt.mSampleRateRange.mMinimum as usize;
            let max_rate = fmt.mSampleRateRange.mMaximum as usize;
            let rate = fmt.mFormat.mSampleRate as usize;
            let channels = fmt.mFormat.mChannelsPerFrame;
            if let Some(AudioFormat::LinearPCM(flags)) = AudioFormat::from_format_and_flag(
                fmt.mFormat.mFormatID,
                Some(fmt.mFormat.mFormatFlags),
            ) {
                let is_float = flags.contains(LinearPcmFlags::IS_FLOAT);
                let is_int = flags.contains(LinearPcmFlags::IS_SIGNED_INTEGER);
                if is_int && is_float {
                    // Probably never occurs, check just in case
                    continue;
                }
                if requested_float && !is_float {
                    // Wrong number type
                    continue;
                }
                if !requested_float && !is_int {
                    // Wrong number type
                    continue;
                }
                if requested_bits != fmt.mFormat.mBitsPerChannel {
                    // Wrong number of bits
                    continue;
                }
                if requested_channels > channels {
                    // Too few channels
                    continue;
                }
                if rate == requested_samplerate
                    || (requested_samplerate >= min_rate && requested_samplerate <= max_rate)
                {
                    return Some(fmt.mFormat);
                }
            }
        }
    }
    None
}

/// Change the physical stream format (sample rate and format) of a device.
pub fn set_device_physical_stream_format(
    device_id: AudioDeviceID,
    new_asbd: AudioStreamBasicDescription,
) -> Result<(), Error> {
    unsafe {
        // Get the current format.
        let property_address = AudioObjectPropertyAddress {
            mSelector: kAudioStreamPropertyPhysicalFormat,
            mScope: kAudioObjectPropertyScopeGlobal,
            mElement: kAudioObjectPropertyElementMaster,
        };
        let mut maybe_asbd: mem::MaybeUninit<AudioStreamBasicDescription> =
            mem::MaybeUninit::zeroed();
        let data_size = mem::size_of::<AudioStreamBasicDescription>() as u32;
        let status = AudioObjectGetPropertyData(
            device_id,
            NonNull::from(&property_address),
            0,
            null(),
            NonNull::from(&data_size),
            NonNull::from(&mut maybe_asbd).cast(),
        );
        Error::from_os_status(status)?;
        let asbd = maybe_asbd.assume_init();

        if !asbds_are_equal(&asbd, &new_asbd) {
            let property_address = AudioObjectPropertyAddress {
                mSelector: kAudioStreamPropertyPhysicalFormat,
                mScope: kAudioObjectPropertyScopeGlobal,
                mElement: kAudioObjectPropertyElementMaster,
            };

            let reported_asbd: mem::MaybeUninit<AudioStreamBasicDescription> =
                mem::MaybeUninit::zeroed();
            let mut reported_asbd = reported_asbd.assume_init();

            let status = AudioObjectSetPropertyData(
                device_id,
                NonNull::from(&property_address),
                0,
                null(),
                data_size,
                NonNull::from(&new_asbd).cast(),
            );
            Error::from_os_status(status)?;

            // Wait for the reported format to change.
            // This can take up to half a second, but we timeout after 2 sec just in case.
            let timer = ::std::time::Instant::now();
            loop {
                let status = AudioObjectGetPropertyData(
                    device_id,
                    NonNull::from(&property_address),
                    0,
                    null(),
                    NonNull::from(&data_size),
                    NonNull::from(&mut reported_asbd).cast(),
                );
                Error::from_os_status(status)?;
                if asbds_are_equal(&reported_asbd, &new_asbd) {
                    break;
                }
                thread::sleep(Duration::from_millis(5));
                if timer.elapsed() > Duration::from_secs(2) {
                    return Err(Error::UnsupportedStreamFormat);
                }
            }
        }
        Ok(())
    }
}

/// Helper to check if two ASBDs are equal.
fn asbds_are_equal(
    left: &AudioStreamBasicDescription,
    right: &AudioStreamBasicDescription,
) -> bool {
    left.mSampleRate as u32 == right.mSampleRate as u32
        && left.mFormatID == right.mFormatID
        && left.mFormatFlags == right.mFormatFlags
        && left.mBytesPerPacket == right.mBytesPerPacket
        && left.mFramesPerPacket == right.mFramesPerPacket
        && left.mBytesPerFrame == right.mBytesPerFrame
        && left.mChannelsPerFrame == right.mChannelsPerFrame
        && left.mBitsPerChannel == right.mBitsPerChannel
}

/// Get a vector with all supported physical formats as AudioBasicRangedDescriptions.
pub fn get_supported_physical_stream_formats(
    device_id: AudioDeviceID,
) -> Result<Vec<AudioStreamRangedDescription>, Error> {
    // Get available formats.
    let mut property_address = AudioObjectPropertyAddress {
        mSelector: kAudioStreamPropertyPhysicalFormat,
        mScope: kAudioObjectPropertyScopeGlobal,
        mElement: kAudioObjectPropertyElementMaster,
    };
    let allformats = unsafe {
        property_address.mSelector = kAudioStreamPropertyAvailablePhysicalFormats;
        let mut data_size = 0u32;
        let status = AudioObjectGetPropertyDataSize(
            device_id,
            NonNull::from(&property_address),
            0,
            null(),
            NonNull::from(&mut data_size),
        );
        Error::from_os_status(status)?;
        let n_formats = data_size as usize / mem::size_of::<AudioStreamRangedDescription>();
        let mut formats: Vec<AudioStreamRangedDescription> = vec![];
        formats.reserve_exact(n_formats as usize);
        formats.set_len(n_formats);

        let status = AudioObjectGetPropertyData(
            device_id,
            NonNull::from(&property_address),
            0,
            null(),
            NonNull::from(&data_size),
            NonNull::new(formats.as_mut_ptr()).unwrap().cast(),
        );
        Error::from_os_status(status)?;
        formats
    };
    Ok(allformats)
}

/// Changing the sample rate is an asynchronous process.
/// A RateListener can be used to get notified when the rate is changed.
pub struct RateListener {
    pub queue: Mutex<VecDeque<f64>>,
    sync_channel: Option<Sender<f64>>,
    device_id: AudioDeviceID,
    property_address: AudioObjectPropertyAddress,
    rate_listener: AudioObjectPropertyListenerProc,
}

impl Drop for RateListener {
    fn drop(&mut self) {
        let _ = self.unregister();
    }
}

impl RateListener {
    /// Create a new RateListener for the given AudioDeviceID.
    /// If an `std::sync::mpsc::Sender` is provided, then events will be pushed to that channel.
    /// If not, they will instead be stored in an internal queue that will need to be polled.
    /// The listener must be registered by calling `register()` in order to start receiving notifications.
    pub fn new(device_id: AudioDeviceID, sync_channel: Option<Sender<f64>>) -> RateListener {
        // Add our sample rate change listener callback.
        let property_address = AudioObjectPropertyAddress {
            mSelector: kAudioDevicePropertyNominalSampleRate,
            mScope: kAudioObjectPropertyScopeGlobal,
            mElement: kAudioObjectPropertyElementMaster,
        };
        let queue = Mutex::new(VecDeque::new());
        RateListener {
            queue,
            sync_channel,
            device_id,
            property_address,
            rate_listener: None,
        }
    }

    /// Register this listener to receive notifications.
    pub fn register(&mut self) -> Result<(), Error> {
        unsafe extern "C-unwind" fn rate_listener(
            device_id: AudioObjectID,
            _n_addresses: u32,
            _properties: NonNull<AudioObjectPropertyAddress>,
            self_ptr: *mut ::std::os::raw::c_void,
        ) -> OSStatus {
            let self_ptr: &mut RateListener = &mut *(self_ptr as *mut RateListener);
            let mut rate: f64 = 0.0;
            let data_size = mem::size_of::<f64>() as u32;
            let property_address = AudioObjectPropertyAddress {
                mSelector: kAudioDevicePropertyNominalSampleRate,
                mScope: kAudioObjectPropertyScopeGlobal,
                mElement: kAudioObjectPropertyElementMaster,
            };
            let result = AudioObjectGetPropertyData(
                device_id,
                NonNull::from(&property_address),
                0,
                null(),
                NonNull::from(&data_size),
                NonNull::from(&mut rate).cast(),
            );
            if let Some(sender) = &self_ptr.sync_channel {
                sender.send(rate).unwrap();
            } else {
                let mut queue = self_ptr.queue.lock().unwrap();
                queue.push_back(rate);
            }
            result
        }

        // Add our sample rate change listener callback.
        let status = unsafe {
            AudioObjectAddPropertyListener(
                self.device_id,
                NonNull::from(&self.property_address),
                Some(rate_listener),
                self as *const _ as *mut _,
            )
        };
        Error::from_os_status(status)?;
        self.rate_listener = Some(rate_listener);
        Ok(())
    }

    /// Unregister this listener to stop receiving notifications.
    pub fn unregister(&mut self) -> Result<(), Error> {
        if self.rate_listener.is_some() {
            let status = unsafe {
                AudioObjectRemovePropertyListener(
                    self.device_id,
                    NonNull::from(&self.property_address),
                    self.rate_listener,
                    self as *const _ as *mut _,
                )
            };
            Error::from_os_status(status)?;
            self.rate_listener = None;
        }
        Ok(())
    }

    /// Get the number of sample rate values received (equals the number of change events).
    /// Not used if the RateListener was created with a `std::sync::mpsc::Sender`.
    pub fn get_nbr_values(&self) -> usize {
        self.queue.lock().unwrap().len()
    }

    /// Copy all received values to a Vec. The latest value is the last element.
    /// The internal buffer is preserved.
    /// Not used if the RateListener was created with a `std::sync::mpsc::Sender`.
    pub fn copy_values(&self) -> Vec<f64> {
        self.queue
            .lock()
            .unwrap()
            .iter()
            .copied()
            .collect::<Vec<f64>>()
    }

    /// Get all received values as a Vec. The latest value is the last element.
    /// This clears the internal buffer.
    /// Not used if the RateListener was created with a `std::sync::mpsc::Sender`.
    pub fn drain_values(&mut self) -> Vec<f64> {
        self.queue.lock().unwrap().drain(..).collect::<Vec<f64>>()
    }
}

/// An AliveListener is used to get notified when a device is disconnected.
pub struct AliveListener {
    alive: Box<AtomicBool>,
    device_id: AudioDeviceID,
    property_address: AudioObjectPropertyAddress,
    alive_listener: AudioObjectPropertyListenerProc,
}

impl Drop for AliveListener {
    fn drop(&mut self) {
        let _ = self.unregister();
    }
}

impl AliveListener {
    /// Create a new AliveListener for the given AudioDeviceID.
    /// The listener must be registered by calling `register()` in order to start receiving notifications.
    pub fn new(device_id: AudioDeviceID) -> AliveListener {
        // Add our listener callback.
        let property_address = AudioObjectPropertyAddress {
            mSelector: kAudioDevicePropertyDeviceIsAlive,
            mScope: kAudioObjectPropertyScopeGlobal,
            mElement: kAudioObjectPropertyElementMaster,
        };
        AliveListener {
            alive: Box::new(AtomicBool::new(true)),
            device_id,
            property_address,
            alive_listener: None,
        }
    }

    /// Register this listener to receive notifications.
    pub fn register(&mut self) -> Result<(), Error> {
        unsafe extern "C-unwind" fn alive_listener(
            device_id: AudioObjectID,
            _n_addresses: u32,
            _properties: NonNull<AudioObjectPropertyAddress>,
            self_ptr: *mut ::std::os::raw::c_void,
        ) -> OSStatus {
            let self_ptr: &mut AliveListener = &mut *(self_ptr as *mut AliveListener);
            let mut alive: u32 = 0;
            let data_size = mem::size_of::<u32>() as u32;
            let property_address = AudioObjectPropertyAddress {
                mSelector: kAudioDevicePropertyDeviceIsAlive,
                mScope: kAudioObjectPropertyScopeGlobal,
                mElement: kAudioObjectPropertyElementMaster,
            };
            let result = AudioObjectGetPropertyData(
                device_id,
                NonNull::from(&property_address),
                0,
                null(),
                NonNull::from(&data_size),
                NonNull::from(&mut alive).cast(),
            );
            self_ptr.alive.store(alive > 0, Ordering::SeqCst);
            result
        }

        // Add our listener callback.
        let status = unsafe {
            AudioObjectAddPropertyListener(
                self.device_id,
                NonNull::from(&self.property_address),
                Some(alive_listener),
                self as *const _ as *mut _,
            )
        };
        Error::from_os_status(status)?;
        self.alive_listener = Some(alive_listener);
        Ok(())
    }

    /// Unregister this listener to stop receiving notifications
    pub fn unregister(&mut self) -> Result<(), Error> {
        if self.alive_listener.is_some() {
            let status = unsafe {
                AudioObjectRemovePropertyListener(
                    self.device_id,
                    NonNull::from(&self.property_address),
                    self.alive_listener,
                    self as *const _ as *mut _,
                )
            };
            Error::from_os_status(status)?;
            self.alive_listener = None;
        }
        Ok(())
    }

    /// Check if the device is still alive.
    pub fn is_alive(&self) -> bool {
        self.alive.load(Ordering::SeqCst)
    }
}

/// Helper for hog mode (exclusive access).
/// Get the pid of the process that currently owns exclusive access to a device.
/// A pid value of -1 means no process owns exclusive access.
pub fn get_hogging_pid(device_id: AudioDeviceID) -> Result<pid_t, Error> {
    let property_address = AudioObjectPropertyAddress {
        mSelector: kAudioDevicePropertyHogMode,
        mScope: kAudioObjectPropertyScopeGlobal,
        mElement: kAudioObjectPropertyElementMaster,
    };
    let pid = unsafe {
        let mut temp_pid: pid_t = 0;
        let data_size = mem::size_of::<pid_t>() as u32;
        let status = AudioObjectGetPropertyData(
            device_id,
            NonNull::from(&property_address),
            0,
            null(),
            NonNull::from(&data_size),
            NonNull::from(&mut temp_pid).cast(),
        );
        Error::from_os_status(status)?;
        temp_pid
    };
    Ok(pid)
}

/// Helper for hog mode (exclusive access).
/// Toggle hog mode for a device.
/// If no process owns exclusive access, then the calling process takes ownership.
/// If the calling process already has ownership, this is released.
/// If another process owns access, then nothing will happen.
/// Returns the pid of the new owning process.
/// A pid value of -1 means no process owns exclusive access.
pub fn toggle_hog_mode(device_id: AudioDeviceID) -> Result<pid_t, Error> {
    let property_address = AudioObjectPropertyAddress {
        mSelector: kAudioDevicePropertyHogMode,
        mScope: kAudioObjectPropertyScopeGlobal,
        mElement: kAudioObjectPropertyElementMaster,
    };
    let pid = unsafe {
        let mut temp_pid: pid_t = -1;
        let data_size = mem::size_of::<pid_t>() as u32;
        let status = AudioObjectSetPropertyData(
            device_id,
            NonNull::from(&property_address),
            0,
            null(),
            data_size as u32,
            NonNull::from(&temp_pid).cast(),
        );
        Error::from_os_status(status)?;
        let status = AudioObjectGetPropertyData(
            device_id,
            NonNull::from(&property_address),
            0,
            null(),
            NonNull::from(&data_size),
            NonNull::from(&mut temp_pid).cast(),
        );
        Error::from_os_status(status)?;
        temp_pid
    };
    Ok(pid)
}
