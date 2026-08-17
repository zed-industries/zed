//! Mixer that plays multiple sounds at the same time.

use crate::common::{ChannelCount, SampleRate};
use crate::source::{SeekError, Source, UniformSourceIterator};
use crate::Sample;
use std::sync::Arc;
use std::time::Duration;

#[cfg(feature = "crossbeam-channel")]
use crossbeam_channel::{unbounded as channel, Receiver, Sender};
#[cfg(not(feature = "crossbeam-channel"))]
use std::sync::mpsc::{channel, Receiver, Sender};

/// Builds a new mixer.
///
/// You can choose the characteristics of the output thanks to this constructor. All the sounds
/// added to the mixer will be converted to these values.
///
/// After creating a mixer, you can add new sounds with the controller.
///
/// Note that mixer without any input source behaves like an `Empty` (not: `Zero`) source,
/// and thus, just after appending to a player, the mixer is removed from the player.
/// As a result, input sources added to the mixer later might not be forwarded to the player.
/// Add `Zero` source to prevent detaching the mixer from player.
pub fn mixer(channels: ChannelCount, sample_rate: SampleRate) -> (Mixer, MixerSource) {
    let (tx, rx) = channel();

    let input = Mixer(Arc::new(Inner {
        pending_tx: tx,
        channels,
        sample_rate,
    }));

    let output = MixerSource {
        current_sources: Vec::with_capacity(16),
        input: input.clone(),
        sample_count: 0,
        still_pending: vec![],
        pending_rx: rx,
    };

    (input, output)
}

/// The input of the mixer.
#[derive(Clone)]
pub struct Mixer(Arc<Inner>);

struct Inner {
    pending_tx: Sender<Box<dyn Source + Send>>,
    channels: ChannelCount,
    sample_rate: SampleRate,
}

impl Mixer {
    /// Adds a new source to mix to the existing ones.
    #[inline]
    pub fn add<T>(&self, source: T)
    where
        T: Source + Send + 'static,
    {
        let uniform_source =
            UniformSourceIterator::new(source, self.0.channels, self.0.sample_rate);
        // Ignore send errors (channel dropped means MixerSource was dropped)
        let _ = self.0.pending_tx.send(Box::new(uniform_source));
    }
}

/// The output of the mixer. Implements `Source`.
pub struct MixerSource {
    // The current iterator that produces samples.
    current_sources: Vec<Box<dyn Source + Send>>,

    // The pending sounds.
    input: Mixer,

    // The number of samples produced so far.
    sample_count: usize,

    // A temporary vec used in start_pending_sources.
    still_pending: Vec<Box<dyn Source + Send>>,

    // Receiver for pending sources from the channel.
    pending_rx: Receiver<Box<dyn Source + Send>>,
}

impl Source for MixerSource {
    #[inline]
    fn current_span_len(&self) -> Option<usize> {
        None
    }

    #[inline]
    fn channels(&self) -> ChannelCount {
        self.input.0.channels
    }

    #[inline]
    fn sample_rate(&self) -> SampleRate {
        self.input.0.sample_rate
    }

    #[inline]
    fn total_duration(&self) -> Option<Duration> {
        None
    }

    #[inline]
    fn try_seek(&mut self, _: Duration) -> Result<(), SeekError> {
        Err(SeekError::NotSupported {
            underlying_source: std::any::type_name::<Self>(),
        })
    }
}

impl Iterator for MixerSource {
    type Item = Sample;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.start_pending_sources();

        self.sample_count += 1;

        let sum = self.sum_current_sources();

        if self.current_sources.is_empty() {
            None
        } else {
            Some(sum)
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, None)
    }
}

impl MixerSource {
    // Samples from the #next() function are interlaced for each of the channels.
    // We need to ensure we start playing sources so that their samples are
    // in-step with the modulo of the samples produced so far. Otherwise, the
    // sound will play on the wrong channels, e.g. left / right will be reversed.
    fn start_pending_sources(&mut self) {
        while let Ok(source) = self.pending_rx.try_recv() {
            let in_step = self
                .sample_count
                .is_multiple_of(source.channels().get() as usize);

            if in_step {
                self.current_sources.push(source);
            } else {
                self.still_pending.push(source);
            }
        }
    }

    fn sum_current_sources(&mut self) -> Sample {
        let mut sum = 0.0;
        self.current_sources.retain_mut(|source| {
            match source.next() {
                Some(value) => {
                    sum += value;
                    true // Keep this source
                }
                None => false, // Remove exhausted source
            }
        });

        sum
    }
}

#[cfg(test)]
mod tests {
    use crate::buffer::SamplesBuffer;
    use crate::math::nz;
    use crate::mixer;
    use crate::source::Source;

    #[test]
    fn basic() {
        let (tx, mut rx) = mixer::mixer(nz!(1), nz!(48000));

        tx.add(SamplesBuffer::new(
            nz!(1),
            nz!(48000),
            vec![10.0, -10.0, 10.0, -10.0],
        ));
        tx.add(SamplesBuffer::new(
            nz!(1),
            nz!(48000),
            vec![5.0, 5.0, 5.0, 5.0],
        ));

        assert_eq!(rx.channels(), nz!(1));
        assert_eq!(rx.sample_rate().get(), 48000);
        assert_eq!(rx.next(), Some(15.0));
        assert_eq!(rx.next(), Some(-5.0));
        assert_eq!(rx.next(), Some(15.0));
        assert_eq!(rx.next(), Some(-5.0));
        assert_eq!(rx.next(), None);
    }

    #[test]
    fn channels_conv() {
        let (tx, mut rx) = mixer::mixer(nz!(2), nz!(48000));

        tx.add(SamplesBuffer::new(
            nz!(1),
            nz!(48000),
            vec![10.0, -10.0, 10.0, -10.0],
        ));
        tx.add(SamplesBuffer::new(
            nz!(1),
            nz!(48000),
            vec![5.0, 5.0, 5.0, 5.0],
        ));

        assert_eq!(rx.channels(), nz!(2));
        assert_eq!(rx.sample_rate().get(), 48000);
        assert_eq!(rx.next(), Some(15.0));
        assert_eq!(rx.next(), Some(15.0));
        assert_eq!(rx.next(), Some(-5.0));
        assert_eq!(rx.next(), Some(-5.0));
        assert_eq!(rx.next(), Some(15.0));
        assert_eq!(rx.next(), Some(15.0));
        assert_eq!(rx.next(), Some(-5.0));
        assert_eq!(rx.next(), Some(-5.0));
        assert_eq!(rx.next(), None);
    }

    #[test]
    fn rate_conv() {
        let (tx, mut rx) = mixer::mixer(nz!(1), nz!(96000));

        tx.add(SamplesBuffer::new(
            nz!(1),
            nz!(48000),
            vec![10.0, -10.0, 10.0, -10.0],
        ));
        tx.add(SamplesBuffer::new(
            nz!(1),
            nz!(48000),
            vec![5.0, 5.0, 5.0, 5.0],
        ));

        assert_eq!(rx.channels(), nz!(1));
        assert_eq!(rx.sample_rate().get(), 96000);
        assert_eq!(rx.next(), Some(15.0));
        assert_eq!(rx.next(), Some(5.0));
        assert_eq!(rx.next(), Some(-5.0));
        assert_eq!(rx.next(), Some(5.0));
        assert_eq!(rx.next(), Some(15.0));
        assert_eq!(rx.next(), Some(5.0));
        assert_eq!(rx.next(), Some(-5.0));
        assert_eq!(rx.next(), None);
    }

    #[test]
    fn start_afterwards() {
        let (tx, mut rx) = mixer::mixer(nz!(1), nz!(48000));

        tx.add(SamplesBuffer::new(
            nz!(1),
            nz!(48000),
            vec![10.0, -10.0, 10.0, -10.0],
        ));

        assert_eq!(rx.next(), Some(10.0));
        assert_eq!(rx.next(), Some(-10.0));

        tx.add(SamplesBuffer::new(
            nz!(1),
            nz!(48000),
            vec![5.0, 5.0, 6.0, 6.0, 7.0, 7.0, 7.0],
        ));

        assert_eq!(rx.next(), Some(15.0));
        assert_eq!(rx.next(), Some(-5.0));

        assert_eq!(rx.next(), Some(6.0));
        assert_eq!(rx.next(), Some(6.0));

        tx.add(SamplesBuffer::new(nz!(1), nz!(48000), vec![2.0]));

        assert_eq!(rx.next(), Some(9.0));
        assert_eq!(rx.next(), Some(7.0));
        assert_eq!(rx.next(), Some(7.0));

        assert_eq!(rx.next(), None);
    }
}
