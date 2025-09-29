use std::time::Duration;

use rodio::{Sample, SampleRate, Source};
use rubato::{FftFixedInOut, Resampler};

pub struct FixedResampler<S> {
    input: S,
    next_channel: usize,
    next_frame: usize,
    output_buffer: Vec<Vec<Sample>>,
    input_buffer: Vec<Vec<Sample>>,
    target_sample_rate: SampleRate,
    resampler: FftFixedInOut<Sample>,
}

impl<S: Source> FixedResampler<S> {
    pub fn new(input: S, target_sample_rate: SampleRate) -> Self {
        let chunk_size_in =
            Duration::from_millis(50).as_secs_f32() * input.sample_rate().get() as f32;
        let chunk_size_in = chunk_size_in.ceil() as usize;

        let resampler = FftFixedInOut::new(
            input.sample_rate().get() as usize,
            target_sample_rate.get() as usize,
            chunk_size_in,
            input.channels().get() as usize,
        )
        .expect(
            "sample rates are non zero, and we are not changing it so there is no resample ratio",
        );

        Self {
            next_channel: 0,
            next_frame: 0,
            output_buffer: resampler.output_buffer_allocate(true),
            input_buffer: resampler.input_buffer_allocate(false),
            target_sample_rate,
            resampler,
            input,
        }
    }
}

impl<S: Source> Source for FixedResampler<S> {
    fn current_span_len(&self) -> Option<usize> {
        None
    }

    fn channels(&self) -> rodio::ChannelCount {
        self.input.channels()
    }

    fn sample_rate(&self) -> rodio::SampleRate {
        self.target_sample_rate
    }

    fn total_duration(&self) -> Option<std::time::Duration> {
        self.input.total_duration()
    }
}

impl<S: Source> FixedResampler<S> {
    fn next_sample(&mut self) -> Option<Sample> {
        let sample = self.output_buffer[self.next_channel]
            .get(self.next_frame)
            .copied();
        self.next_channel = (self.next_channel + 1) % self.input.channels().get() as usize;
        self.next_frame += 1;

        sample
    }
}

impl<S: Source> Iterator for FixedResampler<S> {
    type Item = Sample;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(sample) = self.next_sample() {
            return Some(sample);
        }

        for input_channel in &mut self.input_buffer {
            input_channel.clear();
        }

        for _ in 0..self.resampler.input_frames_next() {
            for input_channel in &mut self.input_buffer {
                input_channel.push(self.input.next()?);
            }
        }

        self.resampler
            .process_into_buffer(&mut self.input_buffer, &mut self.output_buffer, None).expect("Input and output buffer channels are correct as they have been set by the resampler. The buffer for each channel is the same length. The buffer length is what is requested the resampler.");

        self.next_frame = 0;
        self.next_sample()
    }
}
