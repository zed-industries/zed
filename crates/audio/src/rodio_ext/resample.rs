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

        let mut this = Self {
            next_channel: 0,
            next_frame: 0,
            output_buffer: resampler.output_buffer_allocate(true),
            input_buffer: resampler.input_buffer_allocate(false),
            target_sample_rate,
            resampler,
            input,
        };
        this.bootstrap();
        this
    }

    pub fn into_inner(self) -> S {
        self.input
    }

    fn bootstrap(&mut self) -> Option<()> {
        for _ in 0..self.resampler.input_frames_next() {
            for input_channel in &mut self.input_buffer {
                input_channel.push(self.input.next()?);
            }
        }

        let (input_frames, output_frames) = self.resampler
            .process_into_buffer(&mut self.input_buffer, &mut self.output_buffer, None).expect("Input and output buffer channels are correct as they have been set by the resampler. The buffer for each channel is the same length. The buffer length is what is requested the resampler.");

        debug_assert_eq!(input_frames, self.input_buffer[0].len());
        debug_assert_eq!(output_frames, self.output_buffer[0].len());

        self.next_frame = self.resampler.output_delay();
        self.next_channel = 0;
        Some(())
    }

    #[cold]
    fn resample_buffer(&mut self) -> Option<()> {
        for input_channel in &mut self.input_buffer {
            input_channel.clear();
        }

        for _ in 0..self.resampler.input_frames_next() {
            for input_channel in &mut self.input_buffer {
                input_channel.push(self.input.next()?);
            }
        }

        let (input_frames, output_frames) = self.resampler
            .process_into_buffer(&mut self.input_buffer, &mut self.output_buffer, None).expect("Input and output buffer channels are correct as they have been set by the resampler. The buffer for each channel is the same length. The buffer length is what is requested the resampler.");

        debug_assert_eq!(input_frames, self.input_buffer[0].len());
        debug_assert_eq!(output_frames, self.output_buffer[0].len());

        self.next_frame = 0;
        self.next_channel = 0;

        Some(())
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

        if self.next_channel < (self.input.channels().get() - 1) as usize {
            self.next_channel += 1;
        } else {
            self.next_channel = 0;
            self.next_frame += 1;
        }

        sample
    }
}

impl<S: Source> Iterator for FixedResampler<S> {
    type Item = Sample;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(sample) = self.next_sample() {
            return Some(sample);
        }

        self.resample_buffer()?;
        self.next_sample()
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use crate::{
        test::{recording_of_voice, sine},
        RodioExt,
    };
    use itertools::Itertools;
    use rodio::{nz, Source};
    use spectrum_analyzer::{scaling::divide_by_N_sqrt, FrequencyLimit};

    #[derive(Debug)]
    struct PeakPitch {
        pub median: f32,
        pub error: f32,
    }

    fn assert_non_zero_volume_fuzzy(source: impl Source) {
        let sample_rate = source.sample_rate();
        let chunk_size = sample_rate.get() / 1000;
        let ms_volume = source.into_iter().chunks(chunk_size as usize);
        let ms_volume = ms_volume
            .into_iter()
            .map(|chunk| chunk.into_iter().map(|s| s.abs()).sum::<f32>() / chunk_size as f32);

        for (millis, volume) in ms_volume.enumerate() {
            assert!(
                volume > 0.01,
                "Volume about zero around {:?}",
                Duration::from_millis(millis as u64)
            )
        }
    }

    fn median_peak_pitch(source: impl Source) -> PeakPitch {
        use spectrum_analyzer::{samples_fft_to_spectrum, windows::hann_window};

        let channels = source.channels().get();
        let sample_rate = source.sample_rate().get();
        let nyquist_freq = (sample_rate / 2) as f32;
        let hundred_millis: usize = usize::try_from(sample_rate / 10)
            .unwrap()
            .next_power_of_two();

        // de-interleave (take channel 0)
        let samples: Vec<_> = source.step_by(channels as usize).collect();
        let mut resolution = 0f32;
        let mut peaks = samples
            .chunks_exact(hundred_millis)
            .map(|chunk| {
                let spectrum = samples_fft_to_spectrum(
                    &hann_window(chunk),
                    sample_rate,
                    // only care about the human audible range (sorry bats)
                    // (resamplers can include artifacts outside this range
                    // we do not care about since we wont hear them anyway)
                    FrequencyLimit::Range(20f32, 20_000f32.min(nyquist_freq)),
                    Some(&divide_by_N_sqrt),
                )
                .unwrap();

                resolution = resolution.max(spectrum.frequency_resolution());
                spectrum.max().0
            })
            .collect_vec();

        peaks.sort();
        let median = peaks[peaks.len() / 2].val();
        PeakPitch {
            median,
            error: resolution,
        }
    }

    #[test]
    fn constant_samplerate_preserves_length() {
        let test_signal = recording_of_voice(nz!(3), nz!(48_000));
        let resampled = test_signal.clone().constant_samplerate(nz!(16_000));

        let diff_in_length = test_signal
            .total_duration()
            .unwrap()
            .abs_diff(resampled.total_duration().unwrap());
        assert!(diff_in_length.as_secs_f32() < 0.1)
    }

    #[test]
    fn stereo_gets_preserved() {
        use rodio::{
            buffer::SamplesBuffer,
            source::{Function, SignalGenerator},
        };

        let sample_rate = nz!(48_000);
        let sample_rate_resampled = nz!(16_000);
        let frequency_0 = 550f32;
        let frequency_1 = 330f32;

        let channel0 = SignalGenerator::new(sample_rate, frequency_0, Function::Sine)
            .take_duration(Duration::from_secs(1));
        let channel1 = SignalGenerator::new(sample_rate, frequency_1, Function::Sine)
            .take_duration(Duration::from_secs(1));

        let source = channel0.interleave(channel1).collect_vec();
        let source = SamplesBuffer::new(nz!(2), sample_rate, source);
        let resampled = source
            .clone()
            .constant_samplerate(sample_rate_resampled)
            .collect_vec();

        let (channel0_resampled, channel1_resampled): (Vec<_>, Vec<_>) = resampled
            .chunks_exact(2)
            .map(|s| TryInto::<[_; 2]>::try_into(s).unwrap())
            .map(|[channel0, channel1]| (channel0, channel1))
            .unzip();

        for (resampled, frequency) in [
            (channel0_resampled, frequency_0),
            (channel1_resampled, frequency_1),
        ] {
            let resampled = SamplesBuffer::new(nz!(1), sample_rate_resampled, resampled);
            let peak_pitch = median_peak_pitch(resampled);
            assert!(
                (peak_pitch.median - frequency).abs() < peak_pitch.error,
                "pitch should be {frequency} but was {peak_pitch:?}"
            )
        }
    }

    #[test]
    fn resampler_does_not_add_any_latency() {
        let resampled = sine(nz!(1), nz!(48_000))
            .clone()
            .constant_samplerate(nz!(16_000))
            .into_samples_buffer();
        assert_non_zero_volume_fuzzy(resampled);
    }

    #[cfg(test)]
    mod constant_samplerate_preserves_pitch {
        use crate::test::sine;

        use super::*;

        #[test]
        fn one_channel() {
            let test_signal = sine(nz!(1), nz!(48_000));
            rodio::wav_to_file(test_signal.clone(), "test_signal2.wav").unwrap();
            let resampled = test_signal
                .clone()
                .constant_samplerate(nz!(16_000))
                .into_samples_buffer();
            rodio::wav_to_file(resampled.clone(), "resampled2.wav").unwrap();

            let peak_pitch_before = median_peak_pitch(test_signal);
            let peak_pitch_after = median_peak_pitch(resampled);

            assert!(
                (peak_pitch_before.median - peak_pitch_after.median).abs()
                    < peak_pitch_before.error.max(peak_pitch_after.error),
                "peak pitch_before: {peak_pitch_before:?}, peak pitch_after: {peak_pitch_after:?}"
            );
        }

        #[test]
        fn two_channel() {
            let test_signal = sine(nz!(2), nz!(48_000));
            let resampled = test_signal
                .clone()
                .constant_samplerate(nz!(16_000))
                .into_samples_buffer();

            let peak_pitch_before = median_peak_pitch(test_signal);
            let peak_pitch_after = median_peak_pitch(resampled);
            assert!(
                (peak_pitch_before.median - peak_pitch_after.median).abs()
                    < peak_pitch_before.error.max(peak_pitch_after.error),
                "peak pitch_before: {peak_pitch_before:?}, peak pitch_after: {peak_pitch_after:?}"
            );
        }
    }
}
