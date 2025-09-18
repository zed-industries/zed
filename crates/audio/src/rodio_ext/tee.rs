use rodio::{Sample, Source};

struct Tee<S> {
    inner: S,
}

impl<S> Tee<S> {
    fn clone(&self) -> Self {
        Self { inner: todo!() }
    }
}

impl<S: Source> Source for Tee<S> {
    fn current_span_len(&self) -> Option<usize> {
        todo!()
    }

    fn channels(&self) -> rodio::ChannelCount {
        todo!()
    }

    fn sample_rate(&self) -> rodio::SampleRate {
        todo!()
    }

    fn total_duration(&self) -> Option<std::time::Duration> {
        todo!()
    }
}

impl<S: Source> Iterator for Tee<S> {
    type Item = Sample;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
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
        let tee: Arc<Mutex<Option<Tee<_>>>> = Arc::new(Mutex::new(None));

        let tee1 = tee.clone();
        let t1 = thread::spawn(move || {
            let tee = match &mut *tee1.lock() {
                Some(tee) => Tee::clone(tee),
                none @ None => {
                    let tee = Tee {
                        inner: open_fake_microphone(),
                    };
                    let local_thee = Tee::clone(&tee);
                    *none = Some(tee);
                    none.as_mut().expect("just set to some");
                    local_thee
                }
            };

            tee.collect::<Vec<_>>()
        });

        let t2 = thread::spawn(move || {
            let tee = match &mut *tee.lock() {
                Some(tee) => Tee::clone(tee),
                none @ None => {
                    let tee = Tee {
                        inner: open_fake_microphone(),
                    };
                    let local_thee = Tee::clone(&tee);
                    *none = Some(tee);
                    none.as_mut().expect("just set to some");
                    local_thee
                }
            };

            tee.collect::<Vec<_>>()
        });
    }
}
