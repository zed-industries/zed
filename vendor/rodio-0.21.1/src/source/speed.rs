//! Playback Speed control Module.
//!
//! The main concept of this module is the [`Speed`] struct, which
//! encapsulates playback speed controls of the current player.
//!
//! In order to speed up a sink, the speed struct:
//! - Increases the current sample rate by the given factor.
//! - Updates the total duration function to cover for the new factor by dividing by the factor.
//! - Updates the try_seek function by multiplying the audio position by the factor.
//!
//! To speed up a source from sink all you need to do is call the   `set_speed(factor: f32)` function
//! For example, here is how you speed up your sound by using sink or playing raw:
//!
#![cfg_attr(not(feature = "playback"), doc = "```ignore")]
#![cfg_attr(feature = "playback", doc = "```no_run")]
//!# use std::fs::File;
//!# use rodio::{Decoder, Player, source::{Source, SineWave}};
//!
//! // Get an OS-Sink handle to the default physical sound device.
//! // Note that no sound will be played if the _handle_ is dropped.
//! let handle = rodio::DeviceSinkBuilder::open_default_sink()
//!         .expect("open default audio sink");
//! // Load a sound from a file, using a path relative to `Cargo.toml`
//! let file = File::open("examples/music.ogg").unwrap();
//! // Decode that sound file into a source
//! let source = Decoder::try_from(file).unwrap();
//! // Play the sound directly on the device 2x faster
//! handle.mixer().add(source.speed(2.0));
//! std::thread::sleep(std::time::Duration::from_secs(5));
//! ```
//! Here is how you would do it using the sink:
#![cfg_attr(not(feature = "playback"), doc = "```ignore")]
#![cfg_attr(feature = "playback", doc = "```no_run")]
//! use rodio::source::{Source, SineWave};
//! let source = SineWave::new(440.0)
//!    .take_duration(std::time::Duration::from_secs_f32(20.25))
//!    .amplify(0.20);
//! let handle = rodio::DeviceSinkBuilder::open_default_sink()
//!         .expect("open default audio sink");
//! let player = rodio::Player::connect_new(&handle.mixer());
//! player.set_speed(2.0);
//! player.append(source);
//! std::thread::sleep(std::time::Duration::from_secs(5));
//! ```
//! Notice the increase in pitch as the factor increases
//!
//! Since the samples are played faster the audio wave get shorter increasing their frequencies

use std::time::Duration;

use super::SeekError;
use crate::common::{ChannelCount, SampleRate};
use crate::Source;

/// Internal function that builds a `Speed` object.
pub fn speed<I>(input: I, factor: f32) -> Speed<I> {
    Speed { input, factor }
}

/// Filter that modifies each sample by a given value.
#[derive(Clone, Debug)]
pub struct Speed<I> {
    input: I,
    factor: f32,
}

impl<I> Speed<I>
where
    I: Source,
{
    /// Modifies the speed factor.
    #[inline]
    pub fn set_factor(&mut self, factor: f32) {
        self.factor = factor;
    }

    /// Returns a reference to the inner source.
    #[inline]
    pub fn inner(&self) -> &I {
        &self.input
    }

    /// Returns a mutable reference to the inner source.
    #[inline]
    pub fn inner_mut(&mut self) -> &mut I {
        &mut self.input
    }

    /// Returns the inner source.
    #[inline]
    pub fn into_inner(self) -> I {
        self.input
    }
}

impl<I> Iterator for Speed<I>
where
    I: Source,
{
    type Item = I::Item;

    #[inline]
    fn next(&mut self) -> Option<I::Item> {
        self.input.next()
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.input.size_hint()
    }
}

impl<I> ExactSizeIterator for Speed<I> where I: Source + ExactSizeIterator {}

impl<I> Source for Speed<I>
where
    I: Source,
{
    #[inline]
    fn current_span_len(&self) -> Option<usize> {
        self.input.current_span_len()
    }

    #[inline]
    fn channels(&self) -> ChannelCount {
        self.input.channels()
    }

    #[inline]
    fn sample_rate(&self) -> SampleRate {
        SampleRate::new((self.input.sample_rate().get() as f32 * self.factor).max(1.0) as u32)
            .expect("minimum is 1.0 > 0")
    }

    #[inline]
    fn total_duration(&self) -> Option<Duration> {
        self.input.total_duration().map(|d| d.div_f32(self.factor))
    }

    #[inline]
    fn try_seek(&mut self, pos: Duration) -> Result<(), SeekError> {
        let pos_accounting_for_speedup = pos.mul_f32(self.factor);
        self.input.try_seek(pos_accounting_for_speedup)
    }
}
