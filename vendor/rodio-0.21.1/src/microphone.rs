//! A microphone Source
//!
//! # Basic Usage
//!
//! ```no_run
//! use rodio::microphone::MicrophoneBuilder;
//! use rodio::Source;
//! use std::time::Duration;
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let mic = MicrophoneBuilder::new()
//!     .default_device()?
//!     .default_config()?
//!     .open_stream()?;
//!
//! // Record audio for 3 seconds
//! let recording = mic.take_duration(Duration::from_secs(3));
//!
//! // You can now use this like any other Source
//! // For example, play it back or process it further
//! # Ok(())
//! # }
//! ```
//!
//! # Use preferred parameters if supported
//! Attempt to set a specific channel count, sample rate and buffer size but
//! fall back to the default if the device does not support these
//!
//! ```no_run
//! use rodio::microphone::MicrophoneBuilder;
//! use rodio::Source;
//! use std::time::Duration;
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let mut builder = MicrophoneBuilder::new()
//!     .default_device()?
//!     .default_config()?
//!     .prefer_channel_counts([
//!         1.try_into().expect("not zero"),
//!         2.try_into().expect("not zero"),
//!     ])
//!     .prefer_sample_rates([
//!         16_000.try_into().expect("not zero"),
//!         32_000.try_into().expect("not zero"),
//!     ])
//!     .prefer_buffer_sizes(512..);
//!
//! let mic = builder.open_stream()?;
//! # Ok(())
//! # }
//! ```
//!
//! # Configuration with Error Handling
//! Attempt to set a specific channel count but fall back to the default if
//! the device doesn't support it:
//!
//! ```no_run
//! use rodio::microphone::MicrophoneBuilder;
//! use rodio::Source;
//! use std::time::Duration;
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let mut builder = MicrophoneBuilder::new()
//!     .default_device()?
//!     .default_config()?;
//!
//! // Try to set stereo recording (2 channels), but continue with default if unsupported
//! if let Ok(configured_builder) = builder.try_channels(2.try_into()?) {
//!     builder = configured_builder;
//! } else {
//!     println!("Stereo recording not supported, using default channel configuration");
//!     // builder remains unchanged with default configuration
//! }
//!
//! let mic = builder.open_stream()?;
//! # Ok(())
//! # }
//! ```
//!
//! # Device Selection
//!
//! ```no_run
//! use rodio::microphone::{MicrophoneBuilder, available_inputs};
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! // List all available input devices
//! let inputs = available_inputs()?;
//! for (i, input) in inputs.iter().enumerate() {
//!     println!("Input {}: {}", i, input);
//! }
//!
//! // Use a specific device (e.g., the second one)
//! let mic = MicrophoneBuilder::new()
//!     .device(inputs[1].clone())?
//!     .default_config()?
//!     .open_stream()?;
//! # Ok(())
//! # }
//! ```

use core::fmt;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::{thread, time::Duration};

use crate::common::assert_error_traits;
use crate::conversions::SampleTypeConverter;
use crate::{Sample, Source};

mod builder;
mod config;
pub use builder::MicrophoneBuilder;
pub use config::InputConfig;
use cpal::I24;
use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait},
    Device,
};
use rtrb::RingBuffer;

/// Error that can occur when we can not list the input devices
#[derive(Debug, thiserror::Error, Clone)]
#[error("Could not list input devices")]
pub struct ListError(#[source] cpal::DevicesError);
assert_error_traits! {ListError}

/// An input device
#[derive(Clone)]
pub struct Input {
    inner: cpal::Device,
}

impl Input {
    /// Consumes the input and returns the inner device.
    pub fn into_inner(self) -> cpal::Device {
        self.inner
    }
}

impl fmt::Debug for Input {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Device")
            .field(
                "inner",
                &self
                    .inner
                    .description()
                    .ok()
                    .map_or("unknown".to_string(), |d| d.name().to_string()),
            )
            .finish()
    }
}

impl fmt::Display for Input {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            self.inner
                .description()
                .ok()
                .map_or("unknown".to_string(), |d| d.name().to_string())
        )
    }
}

/// Returns a list of available input devices on the system.
pub fn available_inputs() -> Result<Vec<Input>, ListError> {
    let host = cpal::default_host();
    let devices = host
        .input_devices()
        .map_err(ListError)?
        .map(|dev| Input { inner: dev });
    Ok(devices.collect())
}

/// A microphone input stream that can be used as `Source`.
pub struct Microphone {
    _stream_handle: cpal::Stream,
    buffer: rtrb::Consumer<Sample>,
    config: InputConfig,
    poll_interval: Duration,
    error_occurred: Arc<AtomicBool>,
}

impl Source for Microphone {
    fn current_span_len(&self) -> Option<usize> {
        None
    }

    fn channels(&self) -> crate::ChannelCount {
        self.config.channel_count
    }

    fn sample_rate(&self) -> crate::SampleRate {
        self.config.sample_rate
    }

    fn total_duration(&self) -> Option<std::time::Duration> {
        None
    }
}

impl Iterator for Microphone {
    type Item = Sample;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Ok(sample) = self.buffer.pop() {
                return Some(sample);
            } else if self.error_occurred.load(Ordering::Relaxed) {
                return None;
            } else {
                thread::sleep(self.poll_interval)
            }
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.buffer.slots(), None)
    }
}

/// Errors that can occur when opening a microphone source.
#[derive(Debug, thiserror::Error, Clone)]
pub enum OpenError {
    /// Failed to build the input stream.
    #[error("Could not open microphone")]
    BuildStream(#[source] cpal::BuildStreamError),
    /// This is a bug please report it
    #[error("This is a bug, please report it")]
    UnsupportedSampleFormat,
    /// Failed to start the input stream.
    #[error("Could not start the input stream")]
    Play(#[source] cpal::PlayStreamError),
}
assert_error_traits! {OpenError}

impl Microphone {
    fn open(
        device: Device,
        config: InputConfig,
        mut error_callback: impl FnMut(cpal::StreamError) + Send + 'static,
    ) -> Result<Self, OpenError> {
        let timeout = Some(Duration::from_millis(100));
        let hundred_ms_of_samples =
            config.channel_count.get() as u32 * config.sample_rate.get() / 10;
        let (mut tx, rx) = RingBuffer::<Sample>::new(hundred_ms_of_samples as usize);
        let error_occurred = Arc::new(AtomicBool::new(false));
        let error_callback = {
            let error_occurred = error_occurred.clone();
            move |source| {
                error_occurred.store(true, Ordering::Relaxed);
                error_callback(source);
            }
        };

        macro_rules! build_input_streams {
        ($($sample_format:tt, $generic:ty);+) => {
            match config.sample_format {
                $(
                    cpal::SampleFormat::$sample_format => device.build_input_stream::<$generic, _, _>(
                        &config.stream_config(),
                        move |data, _info| {
                            for sample in SampleTypeConverter::<_, Sample>::new(data.into_iter().copied()) {
                                let _skip_if_player_is_behind = tx.push(sample);
                            }
                        },
                        error_callback,
                        timeout,
                    ),
                )+
                _ => return Err(OpenError::UnsupportedSampleFormat),
            }
        };
    }

        let stream = build_input_streams!(
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
        )
        .map_err(OpenError::BuildStream)?;
        stream.play().map_err(OpenError::Play)?;

        Ok(Microphone {
            _stream_handle: stream,
            buffer: rx,
            config,
            poll_interval: Duration::from_millis(5),
            error_occurred,
        })
    }

    /// Get the configuration.
    ///
    /// # Example
    /// Print the sample rate and channel count.
    /// ```no_run
    /// # use rodio::microphone::MicrophoneBuilder;
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let mic = MicrophoneBuilder::new()
    ///     .default_device()?
    ///     .default_config()?
    ///     .open_stream()?;
    /// let config = mic.config();
    /// println!("Sample rate: {}", config.sample_rate.get());
    /// println!("Channel count: {}", config.channel_count.get());
    /// # Ok(())
    /// # }
    /// ```
    pub fn config(&self) -> &InputConfig {
        &self.config
    }
}
