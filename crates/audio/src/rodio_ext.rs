use rodio::Source;

pub trait RodioExt: Source + Sized {
    fn process_buffer<const N: usize, F>(self, callback: F) -> ProcessBuffer<N, Self, F>
    where
        F: FnMut(&mut [rodio::Sample; N]);
    fn inspect_buffer<const N: usize, F>(self, callback: F) -> InspectBuffer<N, Self, F>
    where
        F: FnMut(&[rodio::Sample; N]);
}

impl<S: Source> RodioExt for S {
    fn process_buffer<const N: usize, F>(self, callback: F) -> ProcessBuffer<N, Self, F>
    where
        F: FnMut(&mut [rodio::Sample; N]),
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
        F: FnMut(&[rodio::Sample; N]),
    {
        InspectBuffer {
            inner: self,
            callback,
            buffer: [0.0; N],
            free: 0,
        }
    }
}

pub struct ProcessBuffer<const N: usize, S, F>
where
    S: Source + Sized,
    F: FnMut(&mut [rodio::Sample; N]),
{
    inner: S,
    callback: F,
    /// Buffer used for both input and output.
    buffer: [rodio::Sample; N],
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
    F: FnMut(&mut [rodio::Sample; N]),
{
    type Item = rodio::Sample;

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
}

impl<const N: usize, S, F> Source for ProcessBuffer<N, S, F>
where
    S: Source + Sized,
    F: FnMut(&mut [rodio::Sample; N]),
{
    fn current_span_len(&self) -> Option<usize> {
        // TODO dvdsk this should be a spanless Source
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
    F: FnMut(&[rodio::Sample; N]),
{
    inner: S,
    callback: F,
    /// Stores already emitted samples, once its full we call the callback.
    buffer: [rodio::Sample; N],
    /// Next free element in buffer. If this is equal to the buffer length
    /// we have no more free lements.
    free: usize,
}

impl<const N: usize, S, F> Iterator for InspectBuffer<N, S, F>
where
    S: Source + Sized,
    F: FnMut(&[rodio::Sample; N]),
{
    type Item = rodio::Sample;

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
}

impl<const N: usize, S, F> Source for InspectBuffer<N, S, F>
where
    S: Source + Sized,
    F: FnMut(&[rodio::Sample; N]),
{
    fn current_span_len(&self) -> Option<usize> {
        // TODO dvdsk this should be a spanless Source
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

#[cfg(test)]
mod tests {
    use rodio::static_buffer::StaticSamplesBuffer;

    use super::*;

    #[cfg(test)]
    mod process_buffer {
        use super::*;

        #[test]
        fn callback_gets_all_samples() {
            const SAMPLES: [f32; 5] = [0.0, 1.0, 2.0, 3.0, 4.0];
            let input =
                StaticSamplesBuffer::new(1.try_into().unwrap(), 1.try_into().unwrap(), &SAMPLES);

            let _ = input
                .process_buffer::<{ SAMPLES.len() }, _>(|buffer| assert_eq!(*buffer, SAMPLES))
                .count();
        }
        #[test]
        fn callback_modifies_yielded() {
            const SAMPLES: [f32; 5] = [0.0, 1.0, 2.0, 3.0, 4.0];
            let input =
                StaticSamplesBuffer::new(1.try_into().unwrap(), 1.try_into().unwrap(), &SAMPLES);

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
            const SAMPLES: [f32; 5] = [0.0, 1.0, 2.0, 3.0, 4.0];
            let input =
                StaticSamplesBuffer::new(1.try_into().unwrap(), 1.try_into().unwrap(), &SAMPLES);

            let yielded = input
                .process_buffer::<3, _>(|buffer| assert_eq!(buffer, &SAMPLES[..3]))
                .count();
            assert_eq!(yielded, 3)
        }
    }

    #[cfg(test)]
    mod inspect_buffer {
        use super::*;

        #[test]
        fn callback_gets_all_samples() {
            const SAMPLES: [f32; 5] = [0.0, 1.0, 2.0, 3.0, 4.0];
            let input =
                StaticSamplesBuffer::new(1.try_into().unwrap(), 1.try_into().unwrap(), &SAMPLES);

            let _ = input
                .inspect_buffer::<{ SAMPLES.len() }, _>(|buffer| assert_eq!(*buffer, SAMPLES))
                .count();
        }
        #[test]
        fn source_does_not_truncate() {
            const SAMPLES: [f32; 5] = [0.0, 1.0, 2.0, 3.0, 4.0];
            let input =
                StaticSamplesBuffer::new(1.try_into().unwrap(), 1.try_into().unwrap(), &SAMPLES);

            let yielded = input
                .inspect_buffer::<3, _>(|buffer| assert_eq!(buffer, &SAMPLES[..3]))
                .count();
            assert_eq!(yielded, SAMPLES.len())
        }
    }
}
