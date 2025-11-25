use std::{
    sync::{Arc, Mutex},
    thread::sleep,
    time::Duration,
};

use log::warn;
use rodio::{ChannelCount, Sample, SampleRate, Source};

struct Subscriber {
    tx: rtrb::Producer<Sample>,
    name: Box<str>,
}

impl Subscriber {
    fn fill(&self, buffer: &VecIter) {
        match self.tx.write_chunk_uninit(buffer.len()) {
            Ok(slots) => {
                slots.fill_from_iter(buffer.iter());
            }
            Err(_not_enough_free_slots) => {
                log::warn!("Audio consumer {} is lagging behind", self.name)
            }
        }
    }
}

struct Owner<S> {
    source: S,
    subscribers: Vec<Subscriber>,
}

struct Tee<S> {
    source: Arc<Mutex<Option<Owner<S>>>>,
    state: TeeState<S>,
    buffer: VecIter,
    sample_rate: SampleRate,
    channel_count: ChannelCount,
}

impl<S> Drop for Tee<S> {
    fn drop(&mut self) {}
}

impl<S> Tee<S> {
    fn clone(&self) -> Self {
        Self { inner: todo!() }
    }
}

impl<S: Source> Source for Tee<S> {
    fn current_span_len(&self) -> Option<usize> {
        None
    }

    fn channels(&self) -> rodio::ChannelCount {
        self.channel_count
    }

    fn sample_rate(&self) -> rodio::SampleRate {
        self.sample_rate
    }

    fn total_duration(&self) -> Option<std::time::Duration> {
        todo!()
    }
}

impl<S: Source> Iterator for Tee<S> {
    type Item = Sample;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(sample) = self.buffer.next() {
            return Some(sample);
        }
        self.buffer.reset();
        self.state.fill(self.buffer);
    }
}

// Basically vec::IntoIter but you can fill it up again
struct VecIter {
    inner: Vec<Sample>,
    next: usize,
}

impl Iterator for VecIter {
    type Item = Sample;

    fn next(&mut self) -> Option<Self::Item> {
        if self.next >= self.inner.len() {
            None
        } else {
            let sample = self.inner[self.next];
            self.next += 1;
            Some(sample)
        }
    }
}

impl VecIter {
    fn reset(&mut self) {
        self.next = 0;
        self.inner.clear();
    }
    fn is_full(&self) -> bool {
        self.inner.len() == self.inner.capacity()
    }
    fn capacity(&self) -> usize {
        self.inner.capacity()
    }
    fn len(&self) -> usize {
        self.inner.len()
    }
    fn push(&mut self, sample: Sample) {
        debug_assert_eq!(self.next, 0);
        self.inner.push(sample)
    }
    fn extend(&mut self, samples: impl Iterator<Item = Sample>) {
        debug_assert_eq!(self.next, 0);
        self.inner.extend(samples)
    }
    fn iter<'a>(&'a self) -> core::slice::Iter<'a, Sample> {
        self.inner.iter()
    }
}

enum TeeState<S> {
    Reading(rtrb::Consumer<Sample>),
    Producing(Owner<S>),
}
impl<S: Source> TeeState<S> {
    fn fill(&mut self, buffer: &mut VecIter) -> Option<()> {
        match self {
            Self::Reading(rx) => {
                while !rx.is_abandoned() {
                    let Ok(chunk) = rx.read_chunk(buffer.capacity()) else {
                        // todo something smarter here? Use Nia's cool new perf
                        // to figure out if it makes sense.
                        sleep(Duration::from_millis(2));
                        continue;
                    };
                    buffer.extend(chunk.into_iter());
                }
            }
            Self::Producing(Owner {
                source,
                subscribers,
            }) => {
                buffer.push(source.next()?);
                buffer.extend(source.take(buffer.capacity() - 1));
                for sub in subscribers {
                    sub.fill(buffer);
                }
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use std::{
        sync::{
            Arc,
            atomic::{AtomicBool, Ordering},
        },
        thread,
    };

    use parking_lot::Mutex;

    use super::*;

    fn open_fake_microphone() -> impl rodio::Source {
        static ALREADY_OPENED: AtomicBool = AtomicBool::new(false);
        if ALREADY_OPENED.swap(true, Ordering::Relaxed) {
            panic!()
        } else {
            rodio::source::SineWave::new(440.0)
        }
    }

    // Todo make it so that if the final Tee clone drops the microphone drops too
    #[test]
    fn tomato() {
        let microphone: Arc<Mutex<Option<Tee<_>>>> = Arc::new(Mutex::new(None));

        let microphone1 = microphone.clone();
        let t1 = thread::spawn(move || {
            let mic = match &mut *microphone1.lock() {
                Some(mic) => Tee::clone(mic),
                none @ None => {
                    let mic = Tee {
                        inner: open_fake_microphone(),
                    };
                    let local_mic = Tee::clone(&mic);
                    *none = Some(mic);
                    none.as_mut().expect("just set to some");
                    local_mic
                }
            };

            mic.collect::<Vec<_>>()
        });

        let t2 = thread::spawn(move || {
            let mic = match &mut *microphone.lock() {
                Some(mic) => Tee::clone(mic),
                none @ None => {
                    let mic = Tee {
                        inner: open_fake_microphone(),
                    };
                    let local_mic = Tee::clone(&mic);
                    *none = Some(mic);
                    none.as_mut().expect("just set to some");
                    local_mic
                }
            };

            mic.collect::<Vec<_>>()
        });

        let samples1 = t1.join().unwrap();
        let samples2 = t2.join().unwrap();
        assert_eq!(samples1, samples2)
    }
}
