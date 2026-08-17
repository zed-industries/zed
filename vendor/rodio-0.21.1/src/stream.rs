//! Output audio via the OS via mixers or play directly
//!
//! This module provides a builder that's used to configure and open audio output. Once
//! opened sources can be mixed into the output via `DeviceSink::mixer`.
//!
//! There is also a convenience function `play` for using that output mixer to
//! play a single sound.
use crate::common::{assert_error_traits, ChannelCount, SampleRate};
use crate::math::nz;
use crate::mixer::{mixer, Mixer};
use crate::player::Player;
use crate::{decoder, Source};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{BufferSize, Sample, SampleFormat, StreamConfig, I24};
use std::fmt;
use std::io::{Read, Seek};
use std::marker::Sync;
use std::num::NonZero;

const HZ_44100: SampleRate = nz!(44_100);

/// `cpal::Stream` container. Use `mixer()` method to control output.
///
/// <div class="warning">When dropped playback will end, and the associated
/// OS-Sink will be disposed</div>
///
/// # Note
/// On drop this will print a message to stderr or emit a log msg when tracing is
/// enabled. Though we recommend you do not you can disable that print/log with:
/// [`DeviceSink::log_on_drop(false)`](DeviceSink::log_on_drop).
/// If the `DeviceSink` is dropped because the program is panicking we do not print
/// or log anything.
///
/// # Example
/// ```no_run
/// # use rodio::DeviceSinkBuilder;
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let mut handle = DeviceSinkBuilder::open_default_sink()?;
/// handle.log_on_drop(false); // Not recommended during development
/// println!("Output config: {:?}", handle.config());
/// let mixer = handle.mixer();
/// # Ok(())
/// # }
/// ```
pub struct MixerDeviceSink {
    config: DeviceSinkConfig,
    mixer: Mixer,
    log_on_drop: bool,
    _stream: cpal::Stream,
}

impl MixerDeviceSink {
    /// Access the sink's mixer.
    pub fn mixer(&self) -> &Mixer {
        &self.mixer
    }

    /// Access the sink's config.
    pub fn config(&self) -> &DeviceSinkConfig {
        &self.config
    }

    /// When [`OS-Sink`] is dropped a message is logged to stderr or
    /// emitted through tracing if the tracing feature is enabled.
    pub fn log_on_drop(&mut self, enabled: bool) {
        self.log_on_drop = enabled;
    }
}

impl Drop for MixerDeviceSink {
    fn drop(&mut self) {
        if self.log_on_drop && !std::thread::panicking() {
            #[cfg(feature = "tracing")]
            tracing::debug!("Dropping DeviceSink, audio playing through this sink will stop");
            #[cfg(not(feature = "tracing"))]
            eprintln!("Dropping DeviceSink, audio playing through this sink will stop, to prevent this message from appearing use tracing or call `.log_on_drop(false)` on this DeviceSink")
        }
    }
}

impl fmt::Debug for MixerDeviceSink {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("MixerDeviceSink")
            .field("config", &self.config)
            .finish_non_exhaustive()
    }
}

/// Describes the OS-Sink's configuration
#[derive(Copy, Clone, Debug)]
pub struct DeviceSinkConfig {
    pub(crate) channel_count: ChannelCount,
    pub(crate) sample_rate: SampleRate,
    pub(crate) buffer_size: BufferSize,
    pub(crate) sample_format: SampleFormat,
}

impl Default for DeviceSinkConfig {
    fn default() -> Self {
        Self {
            channel_count: nz!(2),
            sample_rate: HZ_44100,
            buffer_size: BufferSize::Default,
            sample_format: SampleFormat::F32,
        }
    }
}

impl DeviceSinkConfig {
    /// Access the OS-Sink config's channel count.
    pub fn channel_count(&self) -> ChannelCount {
        self.channel_count
    }

    /// Access the OS-Sink config's sample rate.
    pub fn sample_rate(&self) -> SampleRate {
        self.sample_rate
    }

    /// Access the OS-Sink config's buffer size.
    pub fn buffer_size(&self) -> &BufferSize {
        &self.buffer_size
    }

    /// Access the OS-Sink config's sample format.
    pub fn sample_format(&self) -> SampleFormat {
        self.sample_format
    }
}

impl core::fmt::Debug for DeviceSinkBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let device = if let Some(device) = &self.device {
            "Some(".to_owned()
                + &device
                    .description()
                    .ok()
                    .map_or("UnNamed".to_string(), |d| d.name().to_string())
                + ")"
        } else {
            "None".to_owned()
        };

        f.debug_struct("DeviceSinkBuilder")
            .field("device", &device)
            .field("config", &self.config)
            .finish()
    }
}

fn default_error_callback(err: cpal::StreamError) {
    #[cfg(feature = "tracing")]
    tracing::error!("audio stream error: {err}");
    #[cfg(not(feature = "tracing"))]
    eprintln!("audio stream error: {err}");
}

/// Convenience builder for audio OS-player.
/// It provides methods to configure several parameters of the audio output and opening default
/// device. See examples for use-cases.
///
/// <div class="warning">When the DeviceSink is dropped playback will end, and the associated
/// OS-Sink will be disposed</div>
pub struct DeviceSinkBuilder<E = fn(cpal::StreamError)>
where
    E: FnMut(cpal::StreamError) + Send + 'static,
{
    device: Option<cpal::Device>,
    config: DeviceSinkConfig,
    error_callback: E,
}

impl Default for DeviceSinkBuilder {
    fn default() -> Self {
        Self {
            device: None,
            config: DeviceSinkConfig::default(),
            error_callback: default_error_callback,
        }
    }
}

impl DeviceSinkBuilder {
    /// Sets output device and its default parameters.
    pub fn from_device(device: cpal::Device) -> Result<DeviceSinkBuilder, DeviceSinkError> {
        let default_config = device
            .default_output_config()
            .map_err(DeviceSinkError::DefaultSinkConfigError)?;

        Ok(Self::default()
            .with_device(device)
            .with_supported_config(&default_config))
    }

    /// Sets default OS-Sink parameters for default output audio device.
    pub fn from_default_device() -> Result<DeviceSinkBuilder, DeviceSinkError> {
        let default_device = cpal::default_host()
            .default_output_device()
            .ok_or(DeviceSinkError::NoDevice)?;
        Self::from_device(default_device)
    }

    /// Try to open a new OS-Sink for the default output device with its default configuration.
    /// Failing that attempt to open OS-Sink with alternative configuration and/or non default
    /// output devices. Returns stream for first of the tried configurations that succeeds.
    /// If all attempts fail return the initial error.
    pub fn open_default_sink() -> Result<MixerDeviceSink, DeviceSinkError> {
        Self::from_default_device()
            .and_then(|x| x.open_stream())
            .or_else(|original_err| {
                let mut devices = match cpal::default_host().output_devices() {
                    Ok(devices) => devices,
                    Err(err) => {
                        #[cfg(feature = "tracing")]
                        tracing::error!("error getting list of output devices: {err}");
                        #[cfg(not(feature = "tracing"))]
                        eprintln!("error getting list of output devices: {err}");
                        return Err(original_err);
                    }
                };
                devices
                    .find_map(|d| {
                        Self::from_device(d)
                            .and_then(|x| x.open_sink_or_fallback())
                            .ok()
                    })
                    .ok_or(original_err)
            })
    }
}

impl<E> DeviceSinkBuilder<E>
where
    E: FnMut(cpal::StreamError) + Send + 'static,
{
    /// Sets output audio device keeping all existing stream parameters intact.
    /// This method is useful if you want to set other parameters yourself.
    /// To also set parameters that are appropriate for the device use [Self::from_device()] instead.
    pub fn with_device(mut self, device: cpal::Device) -> DeviceSinkBuilder<E> {
        self.device = Some(device);
        self
    }

    /// Sets number of OS-Sink's channels.
    pub fn with_channels(mut self, channel_count: ChannelCount) -> DeviceSinkBuilder<E> {
        assert!(channel_count.get() > 0);
        self.config.channel_count = channel_count;
        self
    }

    /// Sets OS-Sink's sample rate.
    pub fn with_sample_rate(mut self, sample_rate: SampleRate) -> DeviceSinkBuilder<E> {
        self.config.sample_rate = sample_rate;
        self
    }

    /// Sets preferred output buffer size.
    ///
    /// To play sound without any glitches the audio card may never receive a
    /// sample to late. Some samples might take longer to generate then
    /// others. For example because:
    ///  - The OS preempts the thread creating the samples. This happens more
    ///    often if the computer is under high load.
    ///  - The decoder needs to read more data from disk.
    ///  - Rodio code takes longer to run for some samples then others
    ///  - The OS can only send audio samples in groups to the DAC.
    ///
    /// The OS solves this by buffering samples. The larger that buffer the
    /// smaller the impact of variable sample generation time. On the other
    /// hand Rodio controls audio by changing the value of samples. We can not
    /// change a sample already in the OS buffer. That means there is a
    /// minimum delay (latency) of `<buffer size>/<sample_rate*channel_count>`
    /// seconds before a change made through rodio takes effect.
    ///
    /// # Large vs Small buffer
    /// - A larger buffer size results in high latency. Changes made trough
    ///   Rodio (volume/skip/effects etc) takes longer before they can be heard.
    /// - A small buffer might cause:
    ///   - Higher CPU usage
    ///   - Playback interruptions such as buffer underruns.
    ///   - Rodio to log errors like: `alsa::poll() returned POLLERR`
    ///
    /// # Recommendation
    /// If low latency is important to you consider offering the user a method
    /// to find the minimum buffer size that works well on their system under
    /// expected conditions. A good example of this approach can be seen in
    /// [mumble](https://www.mumble.info/documentation/user/audio-settings/)
    /// (specifically the *Output Delay* & *Jitter buffer*.
    ///
    /// These are some typical values that are a good starting point. They may also
    /// break audio completely, it depends on the system.
    /// - Low-latency (audio production, live monitoring): 512-1024
    /// - General use (games, media playback): 1024-2048
    /// - Stability-focused (background music, non-interactive): 2048-4096
    pub fn with_buffer_size(mut self, buffer_size: cpal::BufferSize) -> DeviceSinkBuilder<E> {
        self.config.buffer_size = buffer_size;
        self
    }

    /// Select scalar type that will carry a sample.
    pub fn with_sample_format(mut self, sample_format: SampleFormat) -> DeviceSinkBuilder<E> {
        self.config.sample_format = sample_format;
        self
    }

    /// Set available parameters from a CPAL supported config. You can get a list of
    /// such configurations for an output device using [crate::stream::supported_output_configs()]
    pub fn with_supported_config(
        mut self,
        config: &cpal::SupportedStreamConfig,
    ) -> DeviceSinkBuilder<E> {
        self.config = DeviceSinkConfig {
            channel_count: NonZero::new(config.channels())
                .expect("no valid cpal config has zero channels"),
            sample_rate: NonZero::new(config.sample_rate())
                .expect("no valid cpal config has zero sample rate"),
            sample_format: config.sample_format(),
            ..Default::default()
        };
        self
    }

    /// Set all OS-Sink parameters at once from CPAL stream config.
    pub fn with_config(mut self, config: &cpal::StreamConfig) -> DeviceSinkBuilder<E> {
        self.config = DeviceSinkConfig {
            channel_count: NonZero::new(config.channels)
                .expect("no valid cpal config has zero channels"),
            sample_rate: NonZero::new(config.sample_rate)
                .expect("no valid cpal config has zero sample rate"),
            buffer_size: config.buffer_size,
            ..self.config
        };
        self
    }

    /// Set a callback that will be called when an error occurs with the stream
    pub fn with_error_callback<F>(self, callback: F) -> DeviceSinkBuilder<F>
    where
        F: FnMut(cpal::StreamError) + Send + 'static,
    {
        DeviceSinkBuilder {
            device: self.device,
            config: self.config,
            error_callback: callback,
        }
    }

    /// Open OS-Sink using parameters configured so far.
    pub fn open_stream(self) -> Result<MixerDeviceSink, DeviceSinkError> {
        let device = self.device.as_ref().expect("No output device specified");

        MixerDeviceSink::open(device, &self.config, self.error_callback)
    }

    /// Try opening a new OS-Sink with the builder's current stream configuration.
    /// Failing that attempt to open stream with other available configurations
    /// supported by the device.
    /// If all attempts fail returns initial error.
    pub fn open_sink_or_fallback(&self) -> Result<MixerDeviceSink, DeviceSinkError>
    where
        E: Clone,
    {
        let device = self.device.as_ref().expect("No output device specified");
        let error_callback = &self.error_callback;

        MixerDeviceSink::open(device, &self.config, error_callback.clone()).or_else(|err| {
            for supported_config in supported_output_configs(device)? {
                if let Ok(handle) = DeviceSinkBuilder::default()
                    .with_device(device.clone())
                    .with_supported_config(&supported_config)
                    .with_error_callback(error_callback.clone())
                    .open_stream()
                {
                    return Ok(handle);
                }
            }
            Err(err)
        })
    }
}

/// A convenience function. Plays a sound once.
/// Returns a `Player` that can be used to control the sound.
pub fn play<R>(mixer: &Mixer, input: R) -> Result<Player, PlayError>
where
    R: Read + Seek + Send + Sync + 'static,
{
    let input = decoder::Decoder::new(input)?;
    let player = Player::connect_new(mixer);
    player.append(input);
    Ok(player)
}

impl From<&DeviceSinkConfig> for StreamConfig {
    fn from(config: &DeviceSinkConfig) -> Self {
        cpal::StreamConfig {
            channels: config.channel_count.get() as cpal::ChannelCount,
            sample_rate: config.sample_rate.get(),
            buffer_size: config.buffer_size,
        }
    }
}

/// An error occurred while attempting to play a sound.
#[derive(Debug, thiserror::Error, Clone)]
pub enum PlayError
where
    Self: Send + Sync + 'static,
{
    /// Attempting to decode the audio failed.
    #[error("Failed to decode audio")]
    DecoderError(
        #[from]
        #[source]
        decoder::DecoderError,
    ),
    /// The output device was lost.
    #[error("No output device")]
    NoDevice,
}
assert_error_traits!(PlayError);

/// Errors that might occur when interfacing with audio output.
#[derive(Debug, thiserror::Error)]
pub enum DeviceSinkError {
    /// Could not start playing the sink, see [cpal::PlayStreamError] for
    /// details.
    #[error("Could not start playing the stream")]
    PlayError(#[source] cpal::PlayStreamError),
    /// Failed to get the stream config for the given device. See
    /// [cpal::DefaultStreamConfigError] for details.
    #[error("Failed to get the config for the given device")]
    DefaultSinkConfigError(#[source] cpal::DefaultStreamConfigError),
    /// Error opening sink with OS. See [cpal::BuildStreamError] for details.
    #[error("Error opening the stream with the OS")]
    BuildError(#[source] cpal::BuildStreamError),
    /// Could not list supported configs for the device. Maybe it
    /// disconnected. For details see: [cpal::SupportedStreamConfigsError].
    #[error("Could not list supported configs for the device. Maybe its disconnected?")]
    SupportedConfigsError(#[source] cpal::SupportedStreamConfigsError),
    /// Could not find any output device
    #[error("Could not find any output device")]
    NoDevice,
    /// New cpal sample format that rodio does not yet support please open
    /// an issue if you run into this.
    #[error("New cpal sample format that rodio does not yet support please open an issue if you run into this.")]
    UnsupportedSampleFormat,
}

impl MixerDeviceSink {
    fn validate_config(config: &DeviceSinkConfig) {
        if let BufferSize::Fixed(sz) = config.buffer_size {
            assert!(sz > 0, "fixed buffer size must be greater than zero");
        }
    }

    pub(crate) fn open<E>(
        device: &cpal::Device,
        config: &DeviceSinkConfig,
        error_callback: E,
    ) -> Result<MixerDeviceSink, DeviceSinkError>
    where
        E: FnMut(cpal::StreamError) + Send + 'static,
    {
        Self::validate_config(config);
        let (controller, source) = mixer(config.channel_count, config.sample_rate);
        Self::init_stream(device, config, source, error_callback).and_then(|stream| {
            stream.play().map_err(DeviceSinkError::PlayError)?;
            Ok(Self {
                _stream: stream,
                mixer: controller,
                config: *config,
                log_on_drop: true,
            })
        })
    }

    fn init_stream<S, E>(
        device: &cpal::Device,
        config: &DeviceSinkConfig,
        mut samples: S,
        error_callback: E,
    ) -> Result<cpal::Stream, DeviceSinkError>
    where
        S: Source + Send + 'static,
        E: FnMut(cpal::StreamError) + Send + 'static,
    {
        let cpal_config = config.into();

        macro_rules! build_output_streams {
            ($($sample_format:tt, $generic:ty);+) => {
                match config.sample_format {
                    $(
                        cpal::SampleFormat::$sample_format => device.build_output_stream::<$generic, _, _>(
                            &cpal_config,
                            move |data, _| {
                                data.iter_mut().for_each(|d| {
                                    *d = samples
                                        .next()
                                        .map(Sample::from_sample)
                                        .unwrap_or(<$generic>::EQUILIBRIUM)
                                })
                            },
                            error_callback,
                            None,
                        ),
                    )+
                    _ => return Err(DeviceSinkError::UnsupportedSampleFormat),
                }
            };
        }

        let result = build_output_streams!(
            F32, f32;
            F64, f64;
            I8, i8;
            I16, i16;
            I24, I24;
            I32, i32;
            I64, i64;
            U8, u8;
            U16, u16;
            U24, cpal::U24;
            U32, u32;
            U64, u64
        );

        result.map_err(DeviceSinkError::BuildError)
    }
}

/// Return all formats supported by the device.
pub fn supported_output_configs(
    device: &cpal::Device,
) -> Result<impl Iterator<Item = cpal::SupportedStreamConfig>, DeviceSinkError> {
    let mut supported: Vec<_> = device
        .supported_output_configs()
        .map_err(DeviceSinkError::SupportedConfigsError)?
        .collect();
    supported.sort_by(|a, b| b.cmp_default_heuristics(a));

    Ok(supported.into_iter().flat_map(|sf| {
        let max_rate = sf.max_sample_rate();
        let min_rate = sf.min_sample_rate();
        let mut formats = vec![sf.with_max_sample_rate()];
        let preferred_rate = HZ_44100.get();
        if preferred_rate < max_rate && preferred_rate > min_rate {
            formats.push(sf.with_sample_rate(preferred_rate))
        }
        formats.push(sf.with_sample_rate(min_rate));
        formats
    }))
}
