pub use crate::iter::{SupportedInputConfigs, SupportedOutputConfigs};

use super::sys;
use crate::BackendSpecificError;
use crate::ChannelCount;
use crate::DefaultStreamConfigError;
use crate::DeviceDescription;
use crate::DeviceDescriptionBuilder;
use crate::DeviceId;
use crate::DeviceIdError;
use crate::DeviceNameError;
use crate::DevicesError;
use crate::SampleFormat;
use crate::SupportedBufferSize;
use crate::SupportedStreamConfig;
use crate::SupportedStreamConfigRange;
use crate::SupportedStreamConfigsError;

use std::hash::{Hash, Hasher};
use std::sync::atomic::AtomicU32;
use std::sync::{Arc, Mutex};

/// A ASIO Device
#[derive(Clone)]
pub struct Device {
    /// The driver represented by this device.
    pub driver: Arc<sys::Driver>,

    // Input and/or Output stream.
    // A driver can only have one of each.
    // They need to be created at the same time.
    pub asio_streams: Arc<Mutex<sys::AsioStreams>>,
    pub current_callback_flag: Arc<AtomicU32>,
}

/// All available devices.
pub struct Devices {
    asio: Arc<sys::Asio>,
    drivers: std::vec::IntoIter<String>,
}

impl PartialEq for Device {
    fn eq(&self, other: &Self) -> bool {
        self.driver.name() == other.driver.name()
    }
}

impl Eq for Device {}

impl Hash for Device {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.driver.name().hash(state);
    }
}

impl Device {
    pub fn description(&self) -> Result<DeviceDescription, DeviceNameError> {
        let driver_name = self.driver.name().to_string();

        let direction = crate::device_description::direction_from_counts(
            self.driver.channels().ok().map(|c| c.ins as ChannelCount),
            self.driver.channels().ok().map(|c| c.outs as ChannelCount),
        );

        Ok(DeviceDescriptionBuilder::new(driver_name.clone())
            .driver(driver_name)
            .direction(direction)
            .build())
    }

    pub fn id(&self) -> Result<DeviceId, DeviceIdError> {
        Ok(DeviceId(
            crate::platform::HostId::Asio,
            self.driver.name().to_string(),
        ))
    }

    /// Gets the supported input configs.
    /// TODO currently only supports the default.
    /// Need to find all possible configs.
    pub fn supported_input_configs(
        &self,
    ) -> Result<SupportedInputConfigs, SupportedStreamConfigsError> {
        // Retrieve the default config for the total supported channels and supported sample
        // format.
        let f = match self.default_input_config() {
            Err(_) => return Err(SupportedStreamConfigsError::DeviceNotAvailable),
            Ok(f) => f,
        };

        // Collect a config for every combination of supported sample rate and number of channels.
        let mut supported_configs = vec![];
        for &rate in crate::COMMON_SAMPLE_RATES {
            if !self
                .driver
                .can_sample_rate(rate.into())
                .ok()
                .unwrap_or(false)
            {
                continue;
            }
            for channels in 1..f.channels + 1 {
                supported_configs.push(SupportedStreamConfigRange {
                    channels,
                    min_sample_rate: rate,
                    max_sample_rate: rate,
                    buffer_size: f.buffer_size,
                    sample_format: f.sample_format,
                })
            }
        }
        Ok(supported_configs.into_iter())
    }

    /// Gets the supported output configs.
    /// TODO currently only supports the default.
    /// Need to find all possible configs.
    pub fn supported_output_configs(
        &self,
    ) -> Result<SupportedOutputConfigs, SupportedStreamConfigsError> {
        // Retrieve the default config for the total supported channels and supported sample
        // format.
        let f = match self.default_output_config() {
            Err(_) => return Err(SupportedStreamConfigsError::DeviceNotAvailable),
            Ok(f) => f,
        };

        // Collect a config for every combination of supported sample rate and number of channels.
        let mut supported_configs = vec![];
        for &rate in crate::COMMON_SAMPLE_RATES {
            if !self
                .driver
                .can_sample_rate(rate.into())
                .ok()
                .unwrap_or(false)
            {
                continue;
            }
            for channels in 1..f.channels + 1 {
                supported_configs.push(SupportedStreamConfigRange {
                    channels,
                    min_sample_rate: rate,
                    max_sample_rate: rate,
                    buffer_size: f.buffer_size,
                    sample_format: f.sample_format,
                })
            }
        }
        Ok(supported_configs.into_iter())
    }

    /// Returns the default input config
    pub fn default_input_config(&self) -> Result<SupportedStreamConfig, DefaultStreamConfigError> {
        let channels = self.driver.channels().map_err(default_config_err)?.ins as u16;
        let sample_rate = self.driver.sample_rate().map_err(default_config_err)? as u32;
        let (min, max) = self.driver.buffersize_range().map_err(default_config_err)?;
        let buffer_size = SupportedBufferSize::Range {
            min: min as u32,
            max: max as u32,
        };
        // Map th ASIO sample type to a CPAL sample type
        let data_type = self.driver.input_data_type().map_err(default_config_err)?;
        let sample_format = convert_data_type(&data_type)
            .ok_or(DefaultStreamConfigError::StreamTypeNotSupported)?;
        Ok(SupportedStreamConfig {
            channels,
            sample_rate,
            buffer_size,
            sample_format,
        })
    }

    /// Returns the default output config
    pub fn default_output_config(&self) -> Result<SupportedStreamConfig, DefaultStreamConfigError> {
        let channels = self.driver.channels().map_err(default_config_err)?.outs as u16;
        let sample_rate = self.driver.sample_rate().map_err(default_config_err)? as u32;
        let (min, max) = self.driver.buffersize_range().map_err(default_config_err)?;
        let buffer_size = SupportedBufferSize::Range {
            min: min as u32,
            max: max as u32,
        };
        let data_type = self.driver.output_data_type().map_err(default_config_err)?;
        let sample_format = convert_data_type(&data_type)
            .ok_or(DefaultStreamConfigError::StreamTypeNotSupported)?;
        Ok(SupportedStreamConfig {
            channels,
            sample_rate,
            buffer_size,
            sample_format,
        })
    }
}

impl Devices {
    pub fn new(asio: Arc<sys::Asio>) -> Result<Self, DevicesError> {
        let drivers = asio.driver_names().into_iter();
        Ok(Devices { asio, drivers })
    }
}

impl Iterator for Devices {
    type Item = Device;

    /// Load drivers and return device
    fn next(&mut self) -> Option<Device> {
        loop {
            match self.drivers.next() {
                Some(name) => match self.asio.load_driver(&name) {
                    Ok(driver) => {
                        let driver = Arc::new(driver);
                        let asio_streams = Arc::new(Mutex::new(sys::AsioStreams {
                            input: None,
                            output: None,
                        }));
                        return Some(Device {
                            driver,
                            asio_streams,
                            // Initialize with sentinel value so it never matches global flag state (0 or 1).
                            current_callback_flag: Arc::new(AtomicU32::new(u32::MAX)),
                        });
                    }
                    Err(_) => continue,
                },
                None => return None,
            }
        }
    }
}

pub(crate) fn convert_data_type(ty: &sys::AsioSampleType) -> Option<SampleFormat> {
    let fmt = match *ty {
        sys::AsioSampleType::ASIOSTInt16MSB => SampleFormat::I16,
        sys::AsioSampleType::ASIOSTInt16LSB => SampleFormat::I16,
        sys::AsioSampleType::ASIOSTInt24MSB => SampleFormat::I24,
        sys::AsioSampleType::ASIOSTInt24LSB => SampleFormat::I24,
        sys::AsioSampleType::ASIOSTInt32MSB => SampleFormat::I32,
        sys::AsioSampleType::ASIOSTInt32LSB => SampleFormat::I32,
        sys::AsioSampleType::ASIOSTFloat32MSB => SampleFormat::F32,
        sys::AsioSampleType::ASIOSTFloat32LSB => SampleFormat::F32,
        sys::AsioSampleType::ASIOSTFloat64MSB => SampleFormat::F64,
        sys::AsioSampleType::ASIOSTFloat64LSB => SampleFormat::F64,
        _ => return None,
    };
    Some(fmt)
}

fn default_config_err(e: sys::AsioError) -> DefaultStreamConfigError {
    match e {
        sys::AsioError::NoDrivers | sys::AsioError::HardwareMalfunction => {
            DefaultStreamConfigError::DeviceNotAvailable
        }
        sys::AsioError::NoRate => DefaultStreamConfigError::StreamTypeNotSupported,
        err => {
            let description = format!("{}", err);
            BackendSpecificError { description }.into()
        }
    }
}
