//! Custom host backend.
//!
//! Allows user-defined host implementations with the `custom` feature.
//! See `examples/custom.rs` for usage.

use crate::traits::{DeviceTrait, HostTrait, StreamTrait};
use crate::{
    BuildStreamError, Data, DefaultStreamConfigError, DeviceDescription, DeviceId, DeviceIdError,
    DeviceNameError, DevicesError, InputCallbackInfo, OutputCallbackInfo, PauseStreamError,
    PlayStreamError, SampleFormat, StreamConfig, StreamError, SupportedStreamConfig,
    SupportedStreamConfigRange, SupportedStreamConfigsError,
};
use core::time::Duration;

/// A host that can be used to write custom [`HostTrait`] implementations.
///
/// # Usage
///
/// A [`CustomHost`](Host) can be used on its own, but most crates that depend on `cpal` use a [`cpal::Host`](crate::Host) instead.
/// You can turn a `CustomHost` into a `Host` fairly easily:
///
/// ```ignore
/// let custom = cpal::platform::CustomHost::from_host(/* ... */);
/// let host = cpal::Host::from(custom);
/// ```
///
/// Custom hosts are marked as unavailable and will not appear in [`cpal::available_hosts`](crate::available_hosts).
pub struct Host(Box<dyn HostErased>);

impl Host {
    // this only exists for impl_platform_host, which requires it
    pub(crate) fn new() -> Result<Self, crate::HostUnavailable> {
        Err(crate::HostUnavailable)
    }

    /// Construct a custom host from an arbitrary [`HostTrait`] implementation.
    pub fn from_host<T>(host: T) -> Self
    where
        T: HostTrait + Send + Sync + 'static,
        T::Device: Send + Sync + Clone,
        <T::Device as DeviceTrait>::SupportedInputConfigs: Clone,
        <T::Device as DeviceTrait>::SupportedOutputConfigs: Clone,
        <T::Device as DeviceTrait>::Stream: Send + Sync,
    {
        Self(Box::new(host))
    }
}

/// A device that can be used to write custom [`DeviceTrait`] implementations.
///
/// # Usage
///
/// A [`CustomDevice`](Device) can be used on its own, but most crates that depend on `cpal` use a [`cpal::Device`](crate::Device) instead.
/// You can turn a `Device` into a `Device` fairly easily:
///
/// ```ignore
/// let custom = cpal::platform::CustomDevice::from_device(/* ... */);
/// let device = cpal::Device::from(custom);
/// ```
///
/// `rodio`, for example, lets you build an `OutputStream` with a [`cpal::Device`](crate::Device):
/// ```ignore
/// let custom = cpal::platform::CustomDevice::from_device(/* ... */);
/// let device = cpal::Device::from(custom);
///
/// let stream_builder = rodio::OutputStreamBuilder::from_device(device).expect("failed to build stream");
/// ```
pub struct Device(Box<dyn DeviceErased>);

impl Device {
    /// Construct a custom device from an arbitrary [`DeviceTrait`] implementation.
    pub fn from_device<T>(device: T) -> Self
    where
        T: DeviceTrait + Send + Sync + Clone + 'static,
        T::SupportedInputConfigs: Clone,
        T::SupportedOutputConfigs: Clone,
        T::Stream: Send + Sync,
    {
        Self(Box::new(device))
    }
}

impl Clone for Device {
    fn clone(&self) -> Self {
        self.0.clone()
    }
}

/// A stream that can be used with custom [`StreamTrait`] implementations.
pub struct Stream(Box<dyn StreamErased>);

impl Stream {
    /// Construct a custom stream from an arbitrary [`StreamTrait`] implementation.
    pub fn from_stream<T>(stream: T) -> Self
    where
        T: StreamTrait + Send + Sync + 'static,
    {
        Self(Box::new(stream))
    }
}

// dyn-compatible versions of DeviceTrait, HostTrait, and StreamTrait
// these only accept/return things via trait objects

type Devices = Box<dyn Iterator<Item = Device>>;
trait HostErased: Send + Sync {
    fn devices(&self) -> Result<Devices, DevicesError>;
    fn default_input_device(&self) -> Option<Device>;
    fn default_output_device(&self) -> Option<Device>;
}

pub struct SupportedConfigs(Box<dyn SupportedConfigsErased>);

// A trait for supported configs. This only adds a dyn compatible clone function
// This is required because `SupportedInputConfigsInner` & `SupportedOutputConfigsInner` are `Clone`
trait SupportedConfigsErased: Iterator<Item = SupportedStreamConfigRange> {
    fn clone(&self) -> SupportedConfigs;
}

impl<T> SupportedConfigsErased for T
where
    T: Iterator<Item = SupportedStreamConfigRange> + Clone + 'static,
{
    fn clone(&self) -> SupportedConfigs {
        SupportedConfigs(Box::new(Clone::clone(self)))
    }
}

impl Iterator for SupportedConfigs {
    type Item = SupportedStreamConfigRange;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

impl Clone for SupportedConfigs {
    fn clone(&self) -> Self {
        self.0.clone()
    }
}

type ErrorCallback = Box<dyn FnMut(StreamError) + Send + 'static>;
type InputCallback = Box<dyn FnMut(&Data, &InputCallbackInfo) + Send + 'static>;
type OutputCallback = Box<dyn FnMut(&mut Data, &OutputCallbackInfo) + Send + 'static>;

trait DeviceErased: Send + Sync {
    fn name(&self) -> Result<String, DeviceNameError>;
    fn description(&self) -> Result<DeviceDescription, DeviceNameError>;
    fn id(&self) -> Result<DeviceId, DeviceIdError>;
    fn supports_input(&self) -> bool;
    fn supports_output(&self) -> bool;
    fn supported_input_configs(&self) -> Result<SupportedConfigs, SupportedStreamConfigsError>;
    fn supported_output_configs(&self) -> Result<SupportedConfigs, SupportedStreamConfigsError>;
    fn default_input_config(&self) -> Result<SupportedStreamConfig, DefaultStreamConfigError>;
    fn default_output_config(&self) -> Result<SupportedStreamConfig, DefaultStreamConfigError>;
    fn build_input_stream_raw(
        &self,
        config: &StreamConfig,
        sample_format: SampleFormat,
        data_callback: InputCallback,
        error_callback: ErrorCallback,
        timeout: Option<Duration>,
    ) -> Result<Stream, BuildStreamError>;
    fn build_output_stream_raw(
        &self,
        config: &StreamConfig,
        sample_format: SampleFormat,
        data_callback: OutputCallback,
        error_callback: ErrorCallback,
        timeout: Option<Duration>,
    ) -> Result<Stream, BuildStreamError>;
    // Required because `DeviceInner` is clone
    fn clone(&self) -> Device;
}

trait StreamErased: Send + Sync {
    fn play(&self) -> Result<(), PlayStreamError>;
    fn pause(&self) -> Result<(), PauseStreamError>;
}

fn device_to_erased(d: impl DeviceErased + 'static) -> Device {
    Device(Box::new(d))
}

impl<T> HostErased for T
where
    T: HostTrait + Send + Sync,
    T::Devices: 'static,
    T::Device: DeviceErased + 'static,
{
    fn devices(&self) -> Result<Devices, DevicesError> {
        let iter = <T as HostTrait>::devices(self)?;
        let erased = Box::new(iter.map(device_to_erased));
        Ok(erased)
    }

    fn default_input_device(&self) -> Option<Device> {
        <T as HostTrait>::default_input_device(self).map(device_to_erased)
    }

    fn default_output_device(&self) -> Option<Device> {
        <T as HostTrait>::default_output_device(self).map(device_to_erased)
    }
}

fn supported_configs_to_erased(
    i: impl Iterator<Item = SupportedStreamConfigRange> + Clone + 'static,
) -> SupportedConfigs {
    SupportedConfigs(Box::new(i))
}

fn stream_to_erased(s: impl StreamTrait + Send + Sync + 'static) -> Stream {
    Stream(Box::new(s))
}

impl<T> DeviceErased for T
where
    T: DeviceTrait + Send + Sync + Clone + 'static,
    T::SupportedInputConfigs: Clone + 'static,
    T::SupportedOutputConfigs: Clone + 'static,
    T::Stream: Send + Sync + 'static,
{
    #[allow(deprecated)]
    fn name(&self) -> Result<String, DeviceNameError> {
        <T as DeviceTrait>::name(self)
    }

    fn description(&self) -> Result<DeviceDescription, DeviceNameError> {
        <T as DeviceTrait>::description(self)
    }

    fn id(&self) -> Result<DeviceId, DeviceIdError> {
        <T as DeviceTrait>::id(self)
    }

    fn supports_input(&self) -> bool {
        <T as DeviceTrait>::supports_input(self)
    }

    fn supports_output(&self) -> bool {
        <T as DeviceTrait>::supports_output(self)
    }

    fn supported_input_configs(&self) -> Result<SupportedConfigs, SupportedStreamConfigsError> {
        <T as DeviceTrait>::supported_input_configs(self).map(supported_configs_to_erased)
    }

    fn supported_output_configs(&self) -> Result<SupportedConfigs, SupportedStreamConfigsError> {
        <T as DeviceTrait>::supported_output_configs(self).map(supported_configs_to_erased)
    }

    fn default_input_config(&self) -> Result<SupportedStreamConfig, DefaultStreamConfigError> {
        <T as DeviceTrait>::default_input_config(self)
    }

    fn default_output_config(&self) -> Result<SupportedStreamConfig, DefaultStreamConfigError> {
        <T as DeviceTrait>::default_output_config(self)
    }

    fn build_input_stream_raw(
        &self,
        config: &StreamConfig,
        sample_format: SampleFormat,
        data_callback: InputCallback,
        error_callback: ErrorCallback,
        timeout: Option<Duration>,
    ) -> Result<Stream, BuildStreamError> {
        <T as DeviceTrait>::build_input_stream_raw(
            self,
            config,
            sample_format,
            data_callback,
            error_callback,
            timeout,
        )
        .map(stream_to_erased)
    }

    fn build_output_stream_raw(
        &self,
        config: &StreamConfig,
        sample_format: SampleFormat,
        data_callback: OutputCallback,
        error_callback: ErrorCallback,
        timeout: Option<Duration>,
    ) -> Result<Stream, BuildStreamError> {
        <T as DeviceTrait>::build_output_stream_raw(
            self,
            config,
            sample_format,
            data_callback,
            error_callback,
            timeout,
        )
        .map(stream_to_erased)
    }

    fn clone(&self) -> Device {
        device_to_erased(Clone::clone(self))
    }
}

impl<T> StreamErased for T
where
    T: StreamTrait + Send + Sync,
{
    fn play(&self) -> Result<(), PlayStreamError> {
        <T as StreamTrait>::play(self)
    }

    fn pause(&self) -> Result<(), PauseStreamError> {
        <T as StreamTrait>::pause(self)
    }
}

// implementations of HostTrait, DeviceTrait, and StreamTrait for custom versions

impl HostTrait for Host {
    type Devices = Devices;
    type Device = Device;

    fn is_available() -> bool {
        false
    }

    fn devices(&self) -> Result<Self::Devices, DevicesError> {
        self.0.devices()
    }

    fn default_input_device(&self) -> Option<Self::Device> {
        self.0.default_input_device()
    }

    fn default_output_device(&self) -> Option<Self::Device> {
        self.0.default_output_device()
    }
}

impl DeviceTrait for Device {
    type SupportedInputConfigs = SupportedConfigs;

    type SupportedOutputConfigs = SupportedConfigs;

    type Stream = Stream;

    fn name(&self) -> Result<String, DeviceNameError> {
        self.0.name()
    }

    fn description(&self) -> Result<DeviceDescription, DeviceNameError> {
        self.0.description()
    }

    fn id(&self) -> Result<DeviceId, DeviceIdError> {
        self.0.id()
    }

    fn supports_input(&self) -> bool {
        self.0.supports_input()
    }

    fn supports_output(&self) -> bool {
        self.0.supports_output()
    }

    fn supported_input_configs(
        &self,
    ) -> Result<Self::SupportedInputConfigs, SupportedStreamConfigsError> {
        self.0.supported_input_configs()
    }

    fn supported_output_configs(
        &self,
    ) -> Result<Self::SupportedOutputConfigs, SupportedStreamConfigsError> {
        self.0.supported_output_configs()
    }

    fn default_input_config(&self) -> Result<SupportedStreamConfig, DefaultStreamConfigError> {
        self.0.default_input_config()
    }

    fn default_output_config(&self) -> Result<SupportedStreamConfig, DefaultStreamConfigError> {
        self.0.default_output_config()
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
        self.0.build_input_stream_raw(
            config,
            sample_format,
            Box::new(data_callback),
            Box::new(error_callback),
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
        self.0.build_output_stream_raw(
            config,
            sample_format,
            Box::new(data_callback),
            Box::new(error_callback),
            timeout,
        )
    }
}

impl StreamTrait for Stream {
    fn play(&self) -> Result<(), PlayStreamError> {
        self.0.play()
    }

    fn pause(&self) -> Result<(), PauseStreamError> {
        self.0.pause()
    }
}
