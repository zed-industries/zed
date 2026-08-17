use std::{fmt::Debug, marker::PhantomData};

use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait},
    SupportedStreamConfigRange,
};

use crate::{
    common::assert_error_traits, speakers::config::OutputConfig, ChannelCount, DeviceSinkError,
    FixedSource, MixerDeviceSink, SampleRate,
};

/// Error configuring or opening speakers output
#[allow(missing_docs)]
#[derive(Debug, thiserror::Error, Clone)]
pub enum Error {
    /// No output device is available on the system.
    #[error("There is no output device")]
    NoDevice,
    /// Failed to get the default output configuration for the device.
    #[error("Could not get default output configuration for output device: '{device_name}'")]
    DefaultOutputConfig {
        #[source]
        source: cpal::DefaultStreamConfigError,
        device_name: String,
    },
    /// Failed to get the supported output configurations for the device.
    #[error("Could not get supported output configurations for output device: '{device_name}'")]
    OutputConfigs {
        #[source]
        source: cpal::SupportedStreamConfigsError,
        device_name: String,
    },
    /// The requested output configuration is not supported by the device.
    #[error("The output configuration is not supported by output device: '{device_name}'")]
    UnsupportedByDevice { device_name: String },
}
assert_error_traits! {Error}

/// Generic on the `SpeakersBuilder` which is only present when a config has been set.
/// Methods needing a config are only available on SpeakersBuilder with this
/// Generic set.
pub struct DeviceIsSet;
/// Generic on the `SpeakersBuilder` which is only present when a device has been set.
/// Methods needing a device set are only available on SpeakersBuilder with this
/// Generic set.
pub struct ConfigIsSet;

/// Generic on the `SpeakersBuilder` which indicates no config has been set.
/// Some methods are only available when this types counterpart: `ConfigIsSet` is present.
pub struct ConfigNotSet;
/// Generic on the `SpeakersBuilder` which indicates no device has been set.
/// Some methods are only available when this types counterpart: `DeviceIsSet` is present.
pub struct DeviceNotSet;

/// Builder for configuring and opening an OS-Sink, usually a speaker or headphone.
#[must_use]
pub struct SpeakersBuilder<Device, Config, E = fn(cpal::StreamError)>
where
    E: FnMut(cpal::StreamError) + Send + Clone + 'static,
{
    device: Option<(cpal::Device, Vec<SupportedStreamConfigRange>)>,
    config: Option<super::config::OutputConfig>,
    error_callback: E,

    device_set: PhantomData<Device>,
    config_set: PhantomData<Config>,
}

impl<Device, Config, E> Debug for SpeakersBuilder<Device, Config, E>
where
    E: FnMut(cpal::StreamError) + Send + Clone + 'static,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SpeakersBuilder")
            .field(
                "device",
                &self.device.as_ref().map(|d| {
                    d.0.description()
                        .map_or("unknown".to_string(), |d| d.name().to_string())
                }),
            )
            .field("config", &self.config)
            .finish()
    }
}

impl Default for SpeakersBuilder<DeviceNotSet, ConfigNotSet> {
    fn default() -> Self {
        Self {
            device: None,
            config: None,
            error_callback: default_error_callback,

            device_set: PhantomData,
            config_set: PhantomData,
        }
    }
}

fn default_error_callback(err: cpal::StreamError) {
    #[cfg(feature = "tracing")]
    tracing::error!("audio stream error: {err}");
    #[cfg(not(feature = "tracing"))]
    eprintln!("audio stream error: {err}");
}

impl SpeakersBuilder<DeviceNotSet, ConfigNotSet, fn(cpal::StreamError)> {
    /// Creates a new speakers builder.
    ///
    /// # Example
    /// ```no_run
    /// let builder = rodio::speakers::SpeakersBuilder::new();
    /// ```
    pub fn new() -> SpeakersBuilder<DeviceNotSet, ConfigNotSet, fn(cpal::StreamError)> {
        Self::default()
    }
}

impl<Device, Config, E> SpeakersBuilder<Device, Config, E>
where
    E: FnMut(cpal::StreamError) + Send + Clone + 'static,
{
    /// Sets the output device to use.
    ///
    /// # Example
    /// ```no_run
    /// # use rodio::speakers::{SpeakersBuilder, available_outputs};
    /// let output = available_outputs()?.remove(2);
    /// let builder = SpeakersBuilder::new().device(output)?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn device(
        &self,
        device: super::Output,
    ) -> Result<SpeakersBuilder<DeviceIsSet, ConfigNotSet, E>, Error> {
        let device = device.into_inner();
        let supported_configs = device
            .supported_output_configs()
            .map_err(|source| Error::OutputConfigs {
                source,
                device_name: device
                    .description()
                    .map_or("unknown".to_string(), |d| d.name().to_string()),
            })?
            .collect();
        Ok(SpeakersBuilder {
            device: Some((device, supported_configs)),
            config: self.config,
            error_callback: self.error_callback.clone(),
            device_set: PhantomData,
            config_set: PhantomData,
        })
    }

    /// Uses the system's default output device.
    ///
    /// # Example
    /// ```no_run
    /// # use rodio::speakers::SpeakersBuilder;
    /// let builder = SpeakersBuilder::new().default_device()?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn default_device(&self) -> Result<SpeakersBuilder<DeviceIsSet, ConfigNotSet, E>, Error> {
        let default_device = cpal::default_host()
            .default_output_device()
            .ok_or(Error::NoDevice)?;
        let supported_configs = default_device
            .supported_output_configs()
            .map_err(|source| Error::OutputConfigs {
                source,
                device_name: default_device
                    .description()
                    .map_or("unknown".to_string(), |d| d.name().to_string()),
            })?
            .collect();
        Ok(SpeakersBuilder {
            device: Some((default_device, supported_configs)),
            config: self.config,
            error_callback: self.error_callback.clone(),
            device_set: PhantomData,
            config_set: PhantomData,
        })
    }
}

impl<Config, E> SpeakersBuilder<DeviceIsSet, Config, E>
where
    E: FnMut(cpal::StreamError) + Send + Clone + 'static,
{
    /// Uses the device's default output configuration.
    ///
    /// # Example
    /// ```no_run
    /// # use rodio::speakers::SpeakersBuilder;
    /// let builder = SpeakersBuilder::new()
    ///     .default_device()?
    ///     .default_config()?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn default_config(&self) -> Result<SpeakersBuilder<DeviceIsSet, ConfigIsSet, E>, Error> {
        let device = &self.device.as_ref().expect("DeviceIsSet").0;
        let default_config: OutputConfig = device
            .default_output_config()
            .map_err(|source| Error::DefaultOutputConfig {
                source,
                device_name: device
                    .description()
                    .map_or("unknown".to_string(), |d| d.name().to_string()),
            })?
            .into();

        // Lets try getting f32 output from the default config, as thats
        // what rodio uses internally
        let config = if self
            .check_config(&default_config.with_f32_samples())
            .is_ok()
        {
            default_config.with_f32_samples()
        } else {
            default_config
        };

        Ok(SpeakersBuilder {
            device: self.device.clone(),
            config: Some(config),
            error_callback: self.error_callback.clone(),
            device_set: PhantomData,
            config_set: PhantomData,
        })
    }

    /// Sets a custom output configuration.
    ///
    /// # Example
    /// ```no_run
    /// # use rodio::speakers::{SpeakersBuilder, OutputConfig};
    /// # use std::num::NonZero;
    /// let config = OutputConfig {
    ///     sample_rate: NonZero::new(44_100).expect("44100 is not zero"),
    ///     channel_count: NonZero::new(2).expect("2 is not zero"),
    ///     buffer_size: cpal::BufferSize::Fixed(42_000),
    ///     sample_format: cpal::SampleFormat::U16,
    /// };
    /// let builder = SpeakersBuilder::new()
    ///     .default_device()?
    ///     .config(config)?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn config(
        &self,
        config: OutputConfig,
    ) -> Result<SpeakersBuilder<DeviceIsSet, ConfigIsSet, E>, Error> {
        self.check_config(&config)?;

        Ok(SpeakersBuilder {
            device: self.device.clone(),
            config: Some(config),
            error_callback: self.error_callback.clone(),
            device_set: PhantomData,
            config_set: PhantomData,
        })
    }

    fn check_config(&self, config: &OutputConfig) -> Result<(), Error> {
        let (device, supported_configs) = self.device.as_ref().expect("DeviceIsSet");
        if !supported_configs
            .iter()
            .any(|range| config.supported_given(range))
        {
            Err(Error::UnsupportedByDevice {
                device_name: device
                    .description()
                    .map_or("unknown".to_string(), |d| d.name().to_string()),
            })
        } else {
            Ok(())
        }
    }
}

impl<E> SpeakersBuilder<DeviceIsSet, ConfigIsSet, E>
where
    E: FnMut(cpal::StreamError) + Send + Clone + 'static,
{
    /// Sets the sample rate for output.
    ///
    /// # Error
    /// Returns an error if the requested sample rate combined with the
    /// other parameters can not be supported.
    ///
    /// # Example
    /// ```no_run
    /// # use rodio::speakers::SpeakersBuilder;
    /// let builder = SpeakersBuilder::new()
    ///     .default_device()?
    ///     .default_config()?
    ///     .try_sample_rate(44_100.try_into()?)?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn try_sample_rate(
        &self,
        sample_rate: SampleRate,
    ) -> Result<SpeakersBuilder<DeviceIsSet, ConfigIsSet, E>, Error> {
        let mut new_config = self.config.expect("ConfigIsSet");
        new_config.sample_rate = sample_rate;
        self.check_config(&new_config)?;

        Ok(SpeakersBuilder {
            device: self.device.clone(),
            config: Some(new_config),
            error_callback: self.error_callback.clone(),
            device_set: PhantomData,
            config_set: PhantomData,
        })
    }

    /// Try multiple sample rates, fall back to the default it non match. The
    /// sample rates are in order of preference. If the first can be supported
    /// the second will never be tried.
    ///
    /// # Example
    /// ```no_run
    /// # use rodio::speakers::SpeakersBuilder;
    /// let builder = SpeakersBuilder::new()
    ///     .default_device()?
    ///     .default_config()?
    ///     // 16k or its double with can trivially be resampled to 16k
    ///     .prefer_sample_rates([
    ///         16_000.try_into().expect("not zero"),
    ///         32_000.try_into().expect("not_zero"),
    ///     ]);
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn prefer_sample_rates(
        &self,
        sample_rates: impl IntoIterator<Item = SampleRate>,
    ) -> SpeakersBuilder<DeviceIsSet, ConfigIsSet, E> {
        self.set_preferred_if_supported(sample_rates, |config, sample_rate| {
            config.sample_rate = sample_rate
        })
    }

    fn set_preferred_if_supported<T>(
        &self,
        options: impl IntoIterator<Item = T>,
        setter: impl Fn(&mut OutputConfig, T),
    ) -> SpeakersBuilder<DeviceIsSet, ConfigIsSet, E> {
        let mut config = self.config.expect("ConfigIsSet");
        let mut final_config = config;

        for option in options.into_iter() {
            setter(&mut config, option);
            if self.check_config(&config).is_ok() {
                final_config = config;
                break;
            }
        }

        SpeakersBuilder {
            device: self.device.clone(),
            config: Some(final_config),
            error_callback: self.error_callback.clone(),
            device_set: PhantomData,
            config_set: PhantomData,
        }
    }

    /// Sets the number of output channels.
    ///
    /// # Example
    /// ```no_run
    /// # use rodio::speakers::SpeakersBuilder;
    /// let builder = SpeakersBuilder::new()
    ///     .default_device()?
    ///     .default_config()?
    ///     .try_channels(2.try_into()?)?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn try_channels(
        &self,
        channel_count: ChannelCount,
    ) -> Result<SpeakersBuilder<DeviceIsSet, ConfigIsSet, E>, Error> {
        let mut new_config = self.config.expect("ConfigIsSet");
        new_config.channel_count = channel_count;
        self.check_config(&new_config)?;

        Ok(SpeakersBuilder {
            device: self.device.clone(),
            config: Some(new_config),
            error_callback: self.error_callback.clone(),
            device_set: PhantomData,
            config_set: PhantomData,
        })
    }

    /// Try multiple channel counts, fall back to the default it non match. The
    /// channel counts are in order of preference. If the first can be supported
    /// the second will never be tried.
    ///
    /// # Example
    /// ```no_run
    /// # use rodio::speakers::SpeakersBuilder;
    /// let builder = SpeakersBuilder::new()
    ///     .default_device()?
    ///     .default_config()?
    ///     // We want mono, if thats not possible give
    ///     // us the lowest channel count
    ///     .prefer_channel_counts([
    ///         1.try_into().expect("not zero"),
    ///         2.try_into().expect("not_zero"),
    ///         3.try_into().expect("not_zero"),
    ///     ]);
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn prefer_channel_counts(
        &self,
        channel_counts: impl IntoIterator<Item = ChannelCount>,
    ) -> SpeakersBuilder<DeviceIsSet, ConfigIsSet, E> {
        self.set_preferred_if_supported(channel_counts, |config, count| {
            config.channel_count = count
        })
    }

    /// Sets the buffer size for the output.
    ///
    /// This has no impact on latency, though a too small buffer can lead to audio
    /// artifacts if your program can not get samples out of the buffer before they
    /// get overridden again.
    ///
    /// Normally the default output config will have this set up correctly.
    ///
    /// # Example
    /// ```no_run
    /// # use rodio::speakers::SpeakersBuilder;
    /// let builder = SpeakersBuilder::new()
    ///     .default_device()?
    ///     .default_config()?
    ///     .try_buffer_size(4096)?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn try_buffer_size(
        &self,
        buffer_size: u32,
    ) -> Result<SpeakersBuilder<DeviceIsSet, ConfigIsSet, E>, Error> {
        let mut new_config = self.config.expect("ConfigIsSet");
        new_config.buffer_size = cpal::BufferSize::Fixed(buffer_size);
        self.check_config(&new_config)?;

        Ok(SpeakersBuilder {
            device: self.device.clone(),
            config: Some(new_config),
            error_callback: self.error_callback.clone(),
            device_set: PhantomData,
            config_set: PhantomData,
        })
    }

    /// See the docs of [`try_buffer_size`](SpeakersBuilder::try_buffer_size)
    /// for more.
    ///
    /// Try multiple buffer sizes, fall back to the default it non match. The
    /// buffer sizes are in order of preference. If the first can be supported
    /// the second will never be tried.
    ///
    /// # Note
    /// We will not try buffer sizes larger then 100_000 to prevent this
    /// from hanging too long on open ranges.
    ///
    /// # Example
    /// ```no_run
    /// # use rodio::speakers::SpeakersBuilder;
    /// let builder = SpeakersBuilder::new()
    ///     .default_device()?
    ///     .default_config()?
    ///     .prefer_buffer_sizes([
    ///         2048.try_into().expect("not zero"),
    ///         4096.try_into().expect("not_zero"),
    ///     ]);
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    ///
    /// Get the smallest buffer size larger then 512.
    /// ```no_run
    /// # use rodio::speakers::SpeakersBuilder;
    /// let builder = SpeakersBuilder::new()
    ///     .default_device()?
    ///     .default_config()?
    ///     .prefer_buffer_sizes(4096..);
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn prefer_buffer_sizes(
        &self,
        buffer_sizes: impl IntoIterator<Item = u32>,
    ) -> SpeakersBuilder<DeviceIsSet, ConfigIsSet, E> {
        let buffer_sizes = buffer_sizes.into_iter().take_while(|size| *size < 100_000);

        self.set_preferred_if_supported(buffer_sizes, |config, size| {
            config.buffer_size = cpal::BufferSize::Fixed(size)
        })
    }
}

impl<Device, E> SpeakersBuilder<Device, ConfigIsSet, E>
where
    E: FnMut(cpal::StreamError) + Send + Clone + 'static,
{
    /// Returns the current output configuration.
    ///
    /// # Example
    /// ```no_run
    /// # use rodio::speakers::SpeakersBuilder;
    /// let builder = SpeakersBuilder::new()
    ///     .default_device()?
    ///     .default_config()?;
    /// let config = builder.get_config();
    /// println!("Sample rate: {}", config.sample_rate.get());
    /// println!("Channel count: {}", config.channel_count.get());
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn get_config(&self) -> OutputConfig {
        self.config.expect("ConfigIsSet")
    }
}

impl<E> SpeakersBuilder<DeviceIsSet, ConfigIsSet, E>
where
    E: FnMut(cpal::StreamError) + Send + Clone + 'static,
{
    /// Opens the OS-Sink and provide a mixer for playing sources on it.
    ///
    /// # Example
    /// ```no_run
    /// # use rodio::speakers::SpeakersBuilder;
    /// # use rodio::{Source, source::SineWave};
    /// # use std::time::Duration;
    /// let speakers = SpeakersBuilder::new()
    ///     .default_device()?
    ///     .default_config()?
    ///     .open_mixer()?;
    /// let mixer = speakers.mixer();
    /// mixer.add(SineWave::new(440.).take_duration(Duration::from_secs(4)));
    /// std::thread::sleep(Duration::from_secs(4));
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn open_mixer(&self) -> Result<MixerDeviceSink, crate::DeviceSinkError> {
        let device = self.device.as_ref().expect("DeviceIsSet").0.clone();
        let config = *self.config.as_ref().expect("ConfigIsSet");
        let error_callback = self.error_callback.clone();
        crate::stream::MixerDeviceSink::open(&device, &config.into_cpal_config(), error_callback)
    }

    // TODO
    // pub fn open_queue() -> Result<QueueSink, DeviceSinkError> {
    //     todo!()
    // }

    /// Open the device with the current configuration and play a single
    /// `FixedSource` on it.
    pub fn play(
        self,
        mut source: impl FixedSource + Send + 'static,
    ) -> Result<SinkHandle, PlayError> {
        use cpal::Sample as _;

        let config = self.config.expect("ConfigIsSet");
        let device = self.device.expect("DeviceIsSet").0;

        if config.channel_count != source.channels() {
            return Err(PlayError::WrongChannelCount {
                sink: config.channel_count,
                fixed_source: source.channels(),
            });
        }
        if config.sample_rate != source.sample_rate() {
            return Err(PlayError::WrongSampleRate {
                sink: config.sample_rate,
                fixed_source: source.sample_rate(),
            });
        }

        let cpal_config1 = config.into_cpal_config();
        let cpal_config2 = (&cpal_config1).into();

        macro_rules! build_output_streams {
        ($($sample_format:tt, $generic:ty);+) => {
            match config.sample_format {
                $(
                    cpal::SampleFormat::$sample_format => device.build_output_stream::<$generic, _, _>(
                        &cpal_config2,
                        move |data, _| {
                            data.iter_mut().for_each(|d| {
                                *d = source
                                    .next()
                                    .map(cpal::Sample::from_sample)
                                    .unwrap_or(<$generic>::EQUILIBRIUM)
                            })
                        },
                        self.error_callback,
                        None,
                    ),
                )+
                _ => return Err(DeviceSinkError::UnsupportedSampleFormat.into()),
            }
        };
    }

        let result = build_output_streams!(
            F32, f32;
            F64, f64;
            I8, i8;
            I16, i16;
            I24, cpal::I24;
            I32, i32;
            I64, i64;
            U8, u8;
            U16, u16;
            U24, cpal::U24;
            U32, u32;
            U64, u64
        );

        let stream = result.map_err(DeviceSinkError::BuildError)?;
        stream.play().map_err(DeviceSinkError::PlayError)?;

        Ok(SinkHandle { _stream: stream })
    }
}

// TODO cant introduce till we have introduced the other fixed source parts
// pub struct QueueSink;

/// A sink handle. When this is dropped anything playing through this Sink will
/// stop playing.
pub struct SinkHandle {
    _stream: cpal::Stream,
}

#[derive(Debug, thiserror::Error)]
pub enum PlayError {
    #[error("DeviceSink channel count ({sink}) does not match the source channel count ({fixed_source})")]
    WrongChannelCount {
        sink: ChannelCount,
        fixed_source: ChannelCount,
    },
    #[error(
        "DeviceSink sample rate ({sink}) does not match the source sample rate ({fixed_source})"
    )]
    WrongSampleRate {
        sink: SampleRate,
        fixed_source: SampleRate,
    },
    #[error(transparent)]
    DeviceSink(#[from] crate::DeviceSinkError),
}
