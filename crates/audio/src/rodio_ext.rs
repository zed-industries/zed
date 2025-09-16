use std::{
    sync::{
        Arc, Mutex,
        atomic::{AtomicBool, Ordering},
    },
    time::Duration,
};

use crossbeam::queue::ArrayQueue;
use rodio::{ChannelCount, Sample, SampleRate, Source};

#[derive(Debug, thiserror::Error)]
#[error("Replay duration is too short must be >= 100ms")]
pub struct ReplayDurationTooShort;

pub trait RodioExt: Source + Sized {
    fn process_buffer<const N: usize, F>(self, callback: F) -> ProcessBuffer<N, Self, F>
    where
        F: FnMut(&mut [Sample; N]);
    fn inspect_buffer<const N: usize, F>(self, callback: F) -> InspectBuffer<N, Self, F>
    where
        F: FnMut(&[Sample; N]);
    fn replayable(
        self,
        duration: Duration,
    ) -> Result<(Replay, Replayable<Self>), ReplayDurationTooShort>;
    fn take_samples(self, n: usize) -> TakeSamples<Self>;
}

impl<S: Source> RodioExt for S {
    fn process_buffer<const N: usize, F>(self, callback: F) -> ProcessBuffer<N, Self, F>
    where
        F: FnMut(&mut [Sample; N]),
    {
        ProcessBuffer {
            inner: self,
            callback,
            buffer: [0.0; N],
            next: N,
        }
    }
    fn inspect_buffer<const N: usize, F>(self, callback: F) -> InspectBuffer<N, Self, F>
    where
        F: FnMut(&[Sample; N]),
    {
        InspectBuffer {
            inner: self,
            callback,
            buffer: [0.0; N],
            free: 0,
        }
    }
    /// Maintains a live replay with a history of at least `duration` seconds.
    ///
    /// Note:
    /// History can be 100ms longer if the source drops before or while the
    /// replay is being read
    ///
    /// # Errors
    /// If duration is smaller then 100ms
    fn replayable(
        self,
        duration: Duration,
    ) -> Result<(Replay, Replayable<Self>), ReplayDurationTooShort> {
        if duration < Duration::from_millis(100) {
            return Err(ReplayDurationTooShort);
        }

        let samples_per_second = self.sample_rate().get() as usize * self.channels().get() as usize;
        let samples_to_queue = duration.as_secs_f64() * samples_per_second as f64;
        let samples_to_queue =
            (samples_to_queue as usize).next_multiple_of(self.channels().get().into());

        let chunk_size =
            (samples_per_second.div_ceil(10)).next_multiple_of(self.channels().get() as usize);
        let chunks_to_queue = samples_to_queue.div_ceil(chunk_size);

        let is_active = Arc::new(AtomicBool::new(true));
        let queue = Arc::new(ReplayQueue::new(chunks_to_queue, chunk_size));
        Ok((
            Replay {
                rx: Arc::clone(&queue),
                buffer: Vec::new().into_iter(),
                sleep_duration: duration / 2,
                sample_rate: self.sample_rate(),
                channel_count: self.channels(),
                source_is_active: is_active.clone(),
            },
            Replayable {
                tx: queue,
                inner: self,
                buffer: Vec::with_capacity(chunk_size),
                chunk_size,
                is_active,
            },
        ))
    }
    fn take_samples(self, n: usize) -> TakeSamples<S> {
        TakeSamples {
            inner: self,
            left_to_take: n,
        }
    }
}

pub struct TakeSamples<S> {
    inner: S,
    left_to_take: usize,
}

impl<S: Source> Iterator for TakeSamples<S> {
    type Item = Sample;

    fn next(&mut self) -> Option<Self::Item> {
        if self.left_to_take == 0 {
            None
        } else {
            self.left_to_take -= 1;
            self.inner.next()
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, Some(self.left_to_take))
    }
}

impl<S: Source> Source for TakeSamples<S> {
    fn current_span_len(&self) -> Option<usize> {
        None // does not support spans
    }

    fn channels(&self) -> ChannelCount {
        self.inner.channels()
    }

    fn sample_rate(&self) -> SampleRate {
        self.inner.sample_rate()
    }

    fn total_duration(&self) -> Option<Duration> {
        Some(Duration::from_secs_f64(
            self.left_to_take as f64
                / self.sample_rate().get() as f64
                / self.channels().get() as f64,
        ))
    }
}

#[derive(Debug)]
struct ReplayQueue {
    inner: ArrayQueue<Vec<Sample>>,
    normal_chunk_len: usize,
    /// The last chunk in the queue may be smaller then
    /// the normal chunk size. This is always equal to the
    /// size of the last element in the queue.
    /// (so normally chunk_size)
    last_chunk: Mutex<Vec<Sample>>,
}

impl ReplayQueue {
    fn new(queue_len: usize, chunk_size: usize) -> Self {
        Self {
            inner: ArrayQueue::new(queue_len),
            normal_chunk_len: chunk_size,
            last_chunk: Mutex::new(Vec::new()),
        }
    }
    /// Returns the length in samples
    fn len(&self) -> usize {
        self.inner.len().saturating_sub(1) * self.normal_chunk_len
            + self
                .last_chunk
                .lock()
                .expect("Self::push_last can not poison this lock")
                .len()
    }

    fn pop(&self) -> Option<Vec<Sample>> {
        self.inner.pop() // removes element that was inserted first
    }

    fn push_last(&self, mut samples: Vec<Sample>) {
        let mut last_chunk = self
            .last_chunk
            .lock()
            .expect("Self::len can not poison this lock");
        std::mem::swap(&mut *last_chunk, &mut samples);
    }

    fn push_normal(&self, samples: Vec<Sample>) {
        let _pushed_out_of_ringbuf = self.inner.force_push(samples);
    }
}

pub struct ProcessBuffer<const N: usize, S, F>
where
    S: Source + Sized,
    F: FnMut(&mut [Sample; N]),
{
    inner: S,
    callback: F,
    /// Buffer used for both input and output.
    buffer: [Sample; N],
    /// Next already processed sample is at this index
    /// in buffer.
    ///
    /// If this is equal to the length of the buffer we have no more samples and
    /// we must get new ones and process them
    next: usize,
}

impl<const N: usize, S, F> Iterator for ProcessBuffer<N, S, F>
where
    S: Source + Sized,
    F: FnMut(&mut [Sample; N]),
{
    type Item = Sample;

    fn next(&mut self) -> Option<Self::Item> {
        self.next += 1;
        if self.next < self.buffer.len() {
            let sample = self.buffer[self.next];
            return Some(sample);
        }

        for sample in &mut self.buffer {
            *sample = self.inner.next()?
        }
        (self.callback)(&mut self.buffer);

        self.next = 0;
        Some(self.buffer[0])
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

impl<const N: usize, S, F> Source for ProcessBuffer<N, S, F>
where
    S: Source + Sized,
    F: FnMut(&mut [Sample; N]),
{
    fn current_span_len(&self) -> Option<usize> {
        None
    }

    fn channels(&self) -> rodio::ChannelCount {
        self.inner.channels()
    }

    fn sample_rate(&self) -> rodio::SampleRate {
        self.inner.sample_rate()
    }

    fn total_duration(&self) -> Option<std::time::Duration> {
        self.inner.total_duration()
    }
}

pub struct InspectBuffer<const N: usize, S, F>
where
    S: Source + Sized,
    F: FnMut(&[Sample; N]),
{
    inner: S,
    callback: F,
    /// Stores already emitted samples, once its full we call the callback.
    buffer: [Sample; N],
    /// Next free element in buffer. If this is equal to the buffer length
    /// we have no more free lements.
    free: usize,
}

impl<const N: usize, S, F> Iterator for InspectBuffer<N, S, F>
where
    S: Source + Sized,
    F: FnMut(&[Sample; N]),
{
    type Item = Sample;

    fn next(&mut self) -> Option<Self::Item> {
        let Some(sample) = self.inner.next() else {
            return None;
        };

        self.buffer[self.free] = sample;
        self.free += 1;

        if self.free == self.buffer.len() {
            (self.callback)(&self.buffer);
            self.free = 0
        }

        Some(sample)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

impl<const N: usize, S, F> Source for InspectBuffer<N, S, F>
where
    S: Source + Sized,
    F: FnMut(&[Sample; N]),
{
    fn current_span_len(&self) -> Option<usize> {
        None
    }

    fn channels(&self) -> rodio::ChannelCount {
        self.inner.channels()
    }

    fn sample_rate(&self) -> rodio::SampleRate {
        self.inner.sample_rate()
    }

    fn total_duration(&self) -> Option<std::time::Duration> {
        self.inner.total_duration()
    }
}

#[derive(Debug)]
pub struct Replayable<S: Source> {
    inner: S,
    buffer: Vec<Sample>,
    chunk_size: usize,
    tx: Arc<ReplayQueue>,
    is_active: Arc<AtomicBool>,
}

impl<S: Source> Iterator for Replayable<S> {
    type Item = Sample;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(sample) = self.inner.next() {
            self.buffer.push(sample);
            // If the buffer is full send it
            if self.buffer.len() == self.chunk_size {
                self.tx.push_normal(std::mem::take(&mut self.buffer));
            }
            Some(sample)
        } else {
            let last_chunk = std::mem::take(&mut self.buffer);
            self.tx.push_last(last_chunk);
            self.is_active.store(false, Ordering::Relaxed);
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

impl<S: Source> Source for Replayable<S> {
    fn current_span_len(&self) -> Option<usize> {
        self.inner.current_span_len()
    }

    fn channels(&self) -> ChannelCount {
        self.inner.channels()
    }

    fn sample_rate(&self) -> SampleRate {
        self.inner.sample_rate()
    }

    fn total_duration(&self) -> Option<Duration> {
        self.inner.total_duration()
    }
}

#[derive(Debug)]
pub struct Replay {
    rx: Arc<ReplayQueue>,
    buffer: std::vec::IntoIter<Sample>,
    sleep_duration: Duration,
    sample_rate: SampleRate,
    channel_count: ChannelCount,
    source_is_active: Arc<AtomicBool>,
}

impl Replay {
    pub fn source_is_active(&self) -> bool {
        // - source could return None and not drop
        // - source could be dropped before returning None
        self.source_is_active.load(Ordering::Relaxed) && Arc::strong_count(&self.rx) < 2
    }

    /// Duration of what is in the buffer and can be returned without blocking.
    pub fn duration_ready(&self) -> Duration {
        let samples_per_second = self.channels().get() as u32 * self.sample_rate().get();

        let seconds_queued = self.samples_ready() as f64 / samples_per_second as f64;
        Duration::from_secs_f64(seconds_queued)
    }

    /// Number of samples in the buffer and can be returned without blocking.
    pub fn samples_ready(&self) -> usize {
        self.rx.len() + self.buffer.len()
    }
}

impl Iterator for Replay {
    type Item = Sample;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(sample) = self.buffer.next() {
            return Some(sample);
        }

        loop {
            if let Some(new_buffer) = self.rx.pop() {
                self.buffer = new_buffer.into_iter();
                return self.buffer.next();
            }

            if !self.source_is_active() {
                return None;
            }

            // The queue does not support blocking on a next item. We want this queue as it
            // is quite fast and provides a fixed size. We know how many samples are in a
            // buffer so if we do not get one now we must be getting one after `sleep_duration`.
            std::thread::sleep(self.sleep_duration);
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        ((self.rx.len() + self.buffer.len()), None)
    }
}

impl Source for Replay {
    fn current_span_len(&self) -> Option<usize> {
        None // source is not compatible with spans
    }

    fn channels(&self) -> ChannelCount {
        self.channel_count
    }

    fn sample_rate(&self) -> SampleRate {
        self.sample_rate
    }

    fn total_duration(&self) -> Option<Duration> {
        None
    }
}

#[cfg(test)]
mod tests {
    use rodio::{nz, static_buffer::StaticSamplesBuffer};

    use super::*;

    const SAMPLES: [Sample; 5] = [0.0, 1.0, 2.0, 3.0, 4.0];

    fn test_source() -> StaticSamplesBuffer {
        StaticSamplesBuffer::new(nz!(1), nz!(1), &SAMPLES)
    }

    mod process_buffer {
        use super::*;

        #[test]
        fn callback_gets_all_samples() {
            let input = test_source();

            let _ = input
                .process_buffer::<{ SAMPLES.len() }, _>(|buffer| assert_eq!(*buffer, SAMPLES))
                .count();
        }
        #[test]
        fn callback_modifies_yielded() {
            let input = test_source();

            let yielded: Vec<_> = input
                .process_buffer::<{ SAMPLES.len() }, _>(|buffer| {
                    for sample in buffer {
                        *sample += 1.0;
                    }
                })
                .collect();
            assert_eq!(
                yielded,
                SAMPLES.into_iter().map(|s| s + 1.0).collect::<Vec<_>>()
            )
        }
        #[test]
        fn source_truncates_to_whole_buffers() {
            let input = test_source();

            let yielded = input
                .process_buffer::<3, _>(|buffer| assert_eq!(buffer, &SAMPLES[..3]))
                .count();
            assert_eq!(yielded, 3)
        }
    }

    mod inspect_buffer {
        use super::*;

        #[test]
        fn callback_gets_all_samples() {
            let input = test_source();

            let _ = input
                .inspect_buffer::<{ SAMPLES.len() }, _>(|buffer| assert_eq!(*buffer, SAMPLES))
                .count();
        }
        #[test]
        fn source_does_not_truncate() {
            let input = test_source();

            let yielded = input
                .inspect_buffer::<3, _>(|buffer| assert_eq!(buffer, &SAMPLES[..3]))
                .count();
            assert_eq!(yielded, SAMPLES.len())
        }
    }

    mod instant_replay {
        use super::*;

        #[test]
        fn continues_after_history() {
            let input = test_source();

            let (mut replay, mut source) = input
                .replayable(Duration::from_secs(3))
                .expect("longer then 100ms");

            source.by_ref().take(3).count();
            let yielded: Vec<Sample> = replay.by_ref().take(3).collect();
            assert_eq!(&yielded, &SAMPLES[0..3],);

            source.count();
            let yielded: Vec<Sample> = replay.collect();
            assert_eq!(&yielded, &SAMPLES[3..5],);
        }

        #[test]
        fn keeps_only_latest() {
            let input = test_source();

            let (mut replay, mut source) = input
                .replayable(Duration::from_secs(2))
                .expect("longer then 100ms");

            source.by_ref().take(5).count(); // get all items but do not end the source
            let yielded: Vec<Sample> = replay.by_ref().take(2).collect();
            assert_eq!(&yielded, &SAMPLES[3..5]);
            source.count(); // exhaust source
            assert_eq!(replay.next(), None);
        }

        #[test]
        fn keeps_correct_amount_of_seconds() {
            let input = StaticSamplesBuffer::new(nz!(1), nz!(16_000), &[0.0; 40_000]);

            let (replay, mut source) = input
                .replayable(Duration::from_secs(2))
                .expect("longer then 100ms");

            // exhaust but do not yet end source
            source.by_ref().take(40_000).count();

            // take all samples we can without blocking
            let ready = replay.samples_ready();
            let n_yielded = replay.take_samples(ready).count();

            let max = source.sample_rate().get() * source.channels().get() as u32 * 2;
            let margin = 16_000 / 10; // 100ms
            assert!(n_yielded as u32 >= max - margin);
        }

        #[test]
        fn samples_ready() {
            let input = StaticSamplesBuffer::new(nz!(1), nz!(16_000), &[0.0; 40_000]);
            let (mut replay, source) = input
                .replayable(Duration::from_secs(2))
                .expect("longer then 100ms");
            assert_eq!(replay.by_ref().samples_ready(), 0);

            source.take(8000).count(); // half a second
            let margin = 16_000 / 10; // 100ms
            let ready = replay.samples_ready();
            assert!(ready >= 8000 - margin);
        }
    }
}
