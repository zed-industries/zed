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

pub fn replayable<S: Source>(
    source: S,
    duration: Duration,
) -> Result<(Replay, Replayable<S>), ReplayDurationTooShort> {
    if duration < Duration::from_millis(100) {
        return Err(ReplayDurationTooShort);
    }

    let samples_per_second = source.sample_rate().get() as usize * source.channels().get() as usize;
    let samples_to_queue = duration.as_secs_f64() * samples_per_second as f64;
    let samples_to_queue =
        (samples_to_queue as usize).next_multiple_of(source.channels().get().into());

    let chunk_size =
        (samples_per_second.div_ceil(10)).next_multiple_of(source.channels().get() as usize);
    let chunks_to_queue = samples_to_queue.div_ceil(chunk_size);

    let is_active = Arc::new(AtomicBool::new(true));
    let queue = Arc::new(ReplayQueue::new(chunks_to_queue, chunk_size));
    Ok((
        Replay {
            rx: Arc::clone(&queue),
            buffer: Vec::new().into_iter(),
            sleep_duration: duration / 2,
            sample_rate: source.sample_rate(),
            channel_count: source.channels(),
            source_is_active: is_active.clone(),
        },
        Replayable {
            tx: queue,
            inner: source,
            buffer: Vec::with_capacity(chunk_size),
            chunk_size,
            is_active,
        },
    ))
}

/// constant source, only works on a single span
#[derive(Debug)]
struct ReplayQueue {
    inner: ArrayQueue<Vec<Sample>>,
    normal_chunk_len: usize,
    /// The last chunk in the queue may be smaller than
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

/// constant source, only works on a single span
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

/// constant source, only works on a single span
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
    use super::*;
    use crate::{
        RodioExt,
        rodio_ext::tests::{SAMPLES, test_source},
    };

    #[test]
    fn continues_after_history() {
        let input = test_source();

        let (mut replay, mut source) = input
            .replayable(Duration::from_secs(3))
            .expect("longer than 100ms");

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
            .expect("longer than 100ms");

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
            .expect("longer than 100ms");

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
            .expect("longer than 100ms");
        assert_eq!(replay.by_ref().samples_ready(), 0);

        source.take(8000).count(); // half a second
        let margin = 16_000 / 10; // 100ms
        let ready = replay.samples_ready();
        assert!(ready >= 8000 - margin);
    }
}
