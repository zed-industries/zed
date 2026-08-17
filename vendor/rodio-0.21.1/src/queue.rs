//! Queue that plays sounds one after the other.

use std::collections::VecDeque;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use crate::source::{Empty, SeekError, Source, Zero};
use crate::Sample;

use crate::common::{ChannelCount, SampleRate};
#[cfg(feature = "crossbeam-channel")]
use crossbeam_channel::{unbounded as channel, Receiver, Sender};
#[cfg(not(feature = "crossbeam-channel"))]
use std::sync::mpsc::{channel, Receiver, Sender};

/// Builds a new queue. It consists of an input and an output.
///
/// The input can be used to add sounds to the end of the queue, while the output implements
/// `Source` and plays the sounds.
///
/// The parameter indicates how the queue should behave if the queue becomes empty:
///
/// - If you pass `true`, then the queue is infinite and will play a silence instead until you add
///   a new sound.
/// - If you pass `false`, then the queue will report that it has finished playing.
///
pub fn queue(keep_alive_if_empty: bool) -> (Arc<SourcesQueueInput>, SourcesQueueOutput) {
    let input = Arc::new(SourcesQueueInput {
        next_sounds: Mutex::new(VecDeque::new()),
        keep_alive_if_empty: AtomicBool::new(keep_alive_if_empty),
    });

    let output = SourcesQueueOutput {
        current: Box::new(Empty::new()) as Box<_>,
        signal_after_end: None,
        input: input.clone(),
        samples_consumed_in_span: 0,
        padding_samples_remaining: 0,
    };

    (input, output)
}

// TODO: consider reimplementing this with `from_factory`

type Sound = Box<dyn Source + Send>;
type SignalDone = Option<Sender<()>>;

/// The input of the queue.
pub struct SourcesQueueInput {
    next_sounds: Mutex<VecDeque<(Sound, SignalDone)>>,

    // See constructor.
    keep_alive_if_empty: AtomicBool,
}

impl SourcesQueueInput {
    /// Adds a new source to the end of the queue.
    #[inline]
    pub fn append<T>(&self, source: T)
    where
        T: Source + Send + 'static,
    {
        self.next_sounds
            .lock()
            .unwrap()
            .push_back((Box::new(source) as Box<_>, None));
    }

    /// Adds a new source to the end of the queue.
    ///
    /// The `Receiver` will be signalled when the sound has finished playing.
    ///
    /// Enable the feature flag `crossbeam-channel` in rodio to use a `crossbeam_channel::Receiver` instead.
    #[inline]
    pub fn append_with_signal<T>(&self, source: T) -> Receiver<()>
    where
        T: Source + Send + 'static,
    {
        let (tx, rx) = channel();
        self.next_sounds
            .lock()
            .unwrap()
            .push_back((Box::new(source) as Box<_>, Some(tx)));
        rx
    }

    /// Sets whether the queue stays alive if there's no more sound to play.
    ///
    /// See also the constructor.
    pub fn set_keep_alive_if_empty(&self, keep_alive_if_empty: bool) {
        self.keep_alive_if_empty
            .store(keep_alive_if_empty, Ordering::Release);
    }

    /// Removes all the sounds from the queue. Returns the number of sounds cleared.
    pub fn clear(&self) -> usize {
        let mut sounds = self.next_sounds.lock().unwrap();
        let len = sounds.len();
        sounds.clear();
        len
    }
}
/// The output of the queue. Implements `Source`.
pub struct SourcesQueueOutput {
    // The current iterator that produces samples.
    current: Box<dyn Source + Send>,

    // Signal this sender before picking from `next`.
    signal_after_end: Option<Sender<()>>,

    // The next sounds.
    input: Arc<SourcesQueueInput>,

    // Track samples consumed in the current span to detect mid-span endings.
    samples_consumed_in_span: usize,

    // When a source ends mid-frame, this counts how many silence samples to inject
    // to complete the frame before transitioning to the next source.
    padding_samples_remaining: usize,
}

/// Returns a threshold span length that ensures frame alignment.
///
/// Spans must end on frame boundaries (multiples of channel count) to prevent
/// channel misalignment. Returns ~512 samples rounded to the nearest frame.
#[inline]
fn threshold(channels: ChannelCount) -> usize {
    const BASE_SAMPLES: usize = 512;
    let ch = channels.get() as usize;
    BASE_SAMPLES.div_ceil(ch) * ch
}

impl Source for SourcesQueueOutput {
    #[inline]
    fn current_span_len(&self) -> Option<usize> {
        // This function is non-trivial because the boundary between two sounds in the queue should
        // be a span boundary as well.
        //
        // The current sound is free to return `None` for `current_span_len()`, in which case
        // we *should* return the number of samples remaining the current sound.
        // This can be estimated with `size_hint()`.
        //
        // If the `size_hint` is `None` as well, we are in the worst case scenario. To handle this
        // situation we force a span to have a maximum number of samples indicate by this
        // constant.

        // Try the current `current_span_len`.
        if !self.current.is_exhausted() {
            return self.current.current_span_len();
        } else if self.input.keep_alive_if_empty.load(Ordering::Acquire)
            && self.input.next_sounds.lock().unwrap().is_empty()
        {
            // The next source will be a filler silence which will have a frame-aligned length
            return Some(threshold(self.current.channels()));
        }

        // Try the size hint.
        let (lower_bound, _) = self.current.size_hint();
        // The iterator default implementation just returns 0.
        // That's a problematic value, so skip it.
        if lower_bound > 0 {
            return Some(lower_bound);
        }

        // Otherwise we use a frame-aligned threshold value.
        Some(threshold(self.current.channels()))
    }

    #[inline]
    fn channels(&self) -> ChannelCount {
        if !self.current.is_exhausted() {
            // Current source is active (producing samples)
            // - Initially: never (Empty is exhausted immediately)
            // - After append: the appended source while playing
            // - With keep_alive: Zero (silence) while playing
            self.current.channels()
        } else if let Some((next, _)) = self.input.next_sounds.lock().unwrap().front() {
            // Current source exhausted, peek at next queued source
            // This is critical: UniformSourceIterator queries metadata during append,
            // before any samples are pulled. We must report the next source's metadata.
            next.channels()
        } else {
            // Queue is empty, no sources queued
            // - Initially: Empty
            // - With keep_alive: exhausted Zero between silence chunks (matches Empty)
            // - Without keep_alive: Empty (will end on next())
            self.current.channels()
        }
    }

    #[inline]
    fn sample_rate(&self) -> SampleRate {
        if !self.current.is_exhausted() {
            // Current source is active (producing samples)
            self.current.sample_rate()
        } else if let Some((next, _)) = self.input.next_sounds.lock().unwrap().front() {
            // Current source exhausted, peek at next queued source
            // This prevents wrong resampling setup in UniformSourceIterator
            next.sample_rate()
        } else {
            // Queue is empty, no sources queued
            self.current.sample_rate()
        }
    }

    #[inline]
    fn total_duration(&self) -> Option<Duration> {
        None
    }

    /// Only seeks within the current source.
    // We can not go back to previous sources. We could implement seek such
    // that it advances the queue if the position is beyond the current song.
    //
    // We would then however need to enable seeking backwards across sources too.
    // That no longer seems in line with the queue behaviour.
    //
    // A final pain point is that we would need the total duration for the
    // next few songs.
    #[inline]
    fn try_seek(&mut self, pos: Duration) -> Result<(), SeekError> {
        self.current.try_seek(pos)
    }
}

impl Iterator for SourcesQueueOutput {
    type Item = Sample;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            // If we're padding to complete a frame, return silence.
            if self.padding_samples_remaining > 0 {
                self.padding_samples_remaining -= 1;
                return Some(0.0);
            }

            // Basic situation that will happen most of the time.
            if let Some(sample) = self.current.next() {
                self.samples_consumed_in_span += 1;
                return Some(sample);
            }

            // Source ended - check if we ended mid-frame and need padding.
            let channels = self.current.channels().get() as usize;
            let incomplete_frame_samples = self.samples_consumed_in_span % channels;
            if incomplete_frame_samples > 0 {
                // We're mid-frame - need to pad with silence to complete it.
                self.padding_samples_remaining = channels - incomplete_frame_samples;
                // Reset counter now since we're transitioning to a new span.
                self.samples_consumed_in_span = 0;
                // Continue loop - next iteration will inject silence.
                continue;
            }

            // Reset counter and move to next sound.
            // In order to avoid inlining this expensive operation, the code is in another function.
            self.samples_consumed_in_span = 0;
            if self.go_next().is_err() {
                return None;
            }
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.current.size_hint().0, None)
    }
}

impl SourcesQueueOutput {
    // Called when `current` is empty, and we must jump to the next element.
    // Returns `Ok` if the sound should continue playing, or an error if it should stop.
    //
    // This method is separate so that it is not inlined.
    fn go_next(&mut self) -> Result<(), ()> {
        if let Some(signal_after_end) = self.signal_after_end.take() {
            let _ = signal_after_end.send(());
        }

        let (next, signal_after_end) = {
            let mut next = self.input.next_sounds.lock().unwrap();

            if let Some(next) = next.pop_front() {
                next
            } else {
                let channels = self.current.channels();
                let silence = Box::new(Zero::new_samples(
                    channels,
                    self.current.sample_rate(),
                    threshold(channels),
                )) as Box<_>;
                if self.input.keep_alive_if_empty.load(Ordering::Acquire) {
                    // Play a short silence in order to avoid spinlocking.
                    (silence, None)
                } else {
                    return Err(());
                }
            }
        };

        self.current = next;
        self.signal_after_end = signal_after_end;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::buffer::SamplesBuffer;
    use crate::math::nz;
    use crate::source::{SeekError, Source};
    use crate::{queue, ChannelCount, Sample, SampleRate};
    use std::time::Duration;

    #[test]
    fn basic() {
        let (tx, mut rx) = queue::queue(false);

        tx.append(SamplesBuffer::new(
            nz!(1),
            nz!(48000),
            vec![10.0, -10.0, 10.0, -10.0],
        ));
        tx.append(SamplesBuffer::new(
            nz!(2),
            nz!(96000),
            vec![5.0, 5.0, 5.0, 5.0],
        ));

        assert_eq!(rx.channels(), nz!(1));
        assert_eq!(rx.sample_rate().get(), 48000);
        assert_eq!(rx.next(), Some(10.0));
        assert_eq!(rx.next(), Some(-10.0));
        assert_eq!(rx.next(), Some(10.0));
        assert_eq!(rx.next(), Some(-10.0));
        assert_eq!(rx.channels(), nz!(2));
        assert_eq!(rx.sample_rate().get(), 96000);
        assert_eq!(rx.next(), Some(5.0));
        assert_eq!(rx.next(), Some(5.0));
        assert_eq!(rx.next(), Some(5.0));
        assert_eq!(rx.next(), Some(5.0));
        assert_eq!(rx.next(), None);
    }

    #[test]
    fn immediate_end() {
        let (_, mut rx) = queue::queue(false);
        assert_eq!(rx.next(), None);
    }

    #[test]
    fn keep_alive() {
        let (tx, mut rx) = queue::queue(true);
        tx.append(SamplesBuffer::new(
            nz!(1),
            nz!(48000),
            vec![10.0, -10.0, 10.0, -10.0],
        ));

        assert_eq!(rx.next(), Some(10.0));
        assert_eq!(rx.next(), Some(-10.0));
        assert_eq!(rx.next(), Some(10.0));
        assert_eq!(rx.next(), Some(-10.0));

        for _ in 0..100000 {
            assert_eq!(rx.next(), Some(0.0));
        }
    }

    #[test]
    #[ignore] // TODO: not yet implemented
    fn no_delay_when_added() {
        let (tx, mut rx) = queue::queue(true);

        for _ in 0..500 {
            assert_eq!(rx.next(), Some(0.0));
        }

        tx.append(SamplesBuffer::new(
            nz!(1),
            nz!(48000),
            vec![10.0, -10.0, 10.0, -10.0],
        ));
        assert_eq!(rx.next(), Some(10.0));
        assert_eq!(rx.next(), Some(-10.0));
        assert_eq!(rx.next(), Some(10.0));
        assert_eq!(rx.next(), Some(-10.0));
    }

    #[test]
    fn append_updates_metadata() {
        for keep_alive in [false, true] {
            let (tx, rx) = queue::queue(keep_alive);
            assert_eq!(
                rx.channels(),
                nz!(1),
                "Initial channels should be 1 (keep_alive={keep_alive})"
            );
            assert_eq!(
                rx.sample_rate(),
                nz!(48000),
                "Initial sample rate should be 48000 (keep_alive={keep_alive})"
            );

            tx.append(SamplesBuffer::new(
                nz!(2),
                nz!(44100),
                vec![0.1, 0.2, 0.3, 0.4],
            ));

            assert_eq!(
                rx.channels(),
                nz!(2),
                "Channels should update to 2 (keep_alive={keep_alive})"
            );
            assert_eq!(
                rx.sample_rate(),
                nz!(44100),
                "Sample rate should update to 44100 (keep_alive={keep_alive})"
            );
        }
    }

    #[test]
    fn span_ending_mid_frame() {
        let mut test_source1 = TestSource::new(&[0.1, 0.2, 0.1, 0.2, 0.1])
            .with_channels(nz!(2))
            .with_false_span_len(Some(6));
        let mut test_source2 = TestSource::new(&[0.3, 0.4, 0.3, 0.4]).with_channels(nz!(2));

        let (controls, mut source) = queue::queue(true);
        controls.append(test_source1.clone());
        controls.append(test_source2.clone());

        assert_eq!(source.next(), test_source1.next());
        assert_eq!(source.next(), test_source1.next());
        assert_eq!(source.next(), test_source1.next());
        assert_eq!(source.next(), test_source1.next());
        assert_eq!(source.next(), test_source1.next());
        assert_eq!(None, test_source1.next());

        // Source promised span of 6 but only delivered 5 samples.
        // With 2 channels, that's 2.5 frames. Queue should pad with silence.
        assert_eq!(
            source.next(),
            Some(0.0),
            "Expected silence to complete frame"
        );

        assert_eq!(source.next(), test_source2.next());
        assert_eq!(source.next(), test_source2.next());
        assert_eq!(source.next(), test_source2.next());
        assert_eq!(source.next(), test_source2.next());
    }

    /// Test helper source that allows setting false span length to simulate
    /// sources that end before their promised span length.
    #[derive(Debug, Clone)]
    struct TestSource {
        samples: Vec<Sample>,
        pos: usize,
        channels: ChannelCount,
        sample_rate: SampleRate,
        total_span_len: Option<usize>,
    }

    impl TestSource {
        fn new(samples: &[Sample]) -> Self {
            let samples = samples.to_vec();
            Self {
                total_span_len: Some(samples.len()),
                pos: 0,
                channels: nz!(1),
                sample_rate: nz!(44100),
                samples,
            }
        }

        fn with_channels(mut self, count: ChannelCount) -> Self {
            self.channels = count;
            self
        }

        fn with_false_span_len(mut self, total_len: Option<usize>) -> Self {
            self.total_span_len = total_len;
            self
        }
    }

    impl Iterator for TestSource {
        type Item = Sample;

        fn next(&mut self) -> Option<Self::Item> {
            let res = self.samples.get(self.pos).copied();
            self.pos += 1;
            res
        }

        fn size_hint(&self) -> (usize, Option<usize>) {
            let remaining = self.samples.len().saturating_sub(self.pos);
            (remaining, Some(remaining))
        }
    }

    impl Source for TestSource {
        fn current_span_len(&self) -> Option<usize> {
            self.total_span_len
        }

        fn channels(&self) -> ChannelCount {
            self.channels
        }

        fn sample_rate(&self) -> SampleRate {
            self.sample_rate
        }

        fn total_duration(&self) -> Option<Duration> {
            None
        }

        fn try_seek(&mut self, _: Duration) -> Result<(), SeekError> {
            Err(SeekError::NotSupported {
                underlying_source: std::any::type_name::<Self>(),
            })
        }
    }
}
