use crate::RodioExt;
use crate::rodio_ext::ConstantChannelCount;
use crate::test::sine;
use crate::test::spectrum_duration;

use super::human_perceivable_energy;

use rodio::buffer::SamplesBuffer;
use rodio::nz;
use spectrum_analyzer::FrequencyLimit;
use spectrum_analyzer::FrequencySpectrum;
use spectrum_analyzer::scaling::divide_by_N_sqrt;
use spectrum_analyzer::windows::hann_window;

use super::maximum_energy;

use rodio::Source;

use std::time::Duration;

#[derive(Debug, Clone)]
pub struct VoiceSegment {
    pub start: Duration,
    pub end: Duration,
}

impl VoiceSegment {
    const ZERO: Self = Self {
        start: Duration::ZERO,
        end: Duration::ZERO,
    };

    fn length(&self) -> Duration {
        self.end - self.start
    }

    fn until(&self, other: &Self) -> Duration {
        debug_assert!(self.end < other.start);
        other.start - self.end
    }
}

pub(crate) struct BasicVoiceDetector {
    pub(crate) segments_with_voice: Vec<VoiceSegment>,
}

impl BasicVoiceDetector {
    pub(crate) fn new(source: impl Source + Clone) -> Self {
        // only works on mono
        let source = ConstantChannelCount::new(source, nz!(1)).into_samples_buffer();

        // this gives a good resolution
        let minimum_chunk_duration = Duration::from_millis(20);
        let actual_chunk_duration = spectrum_duration(&source, minimum_chunk_duration);

        let mut spectrum_start_pos = Duration::ZERO;
        let mut partial_segment = None;

        // empirically determined (by looking in audacity)
        // see the 'soup' test for how
        //
        // while this might seem low remember humans precieve sound
        // logarithmically. So 40% of energy sounds like 80% volume.
        let threshold = 0.4 * maximum_energy(source.clone());
        let segments_with_voice: Vec<_> = iter_spectra(source.clone(), actual_chunk_duration)
            .filter_map(|spectrum| {
                let voice_detected = human_perceivable_energy(&spectrum) > threshold;
                spectrum_start_pos += actual_chunk_duration;
                match (&mut partial_segment, voice_detected) {
                    (Some(VoiceSegment { end, .. }), true) => *end = spectrum_start_pos,
                    (Some(VoiceSegment { start, .. }), false) => {
                        let res = Some(VoiceSegment {
                            start: *start,
                            end: spectrum_start_pos,
                        });
                        partial_segment = None;
                        return res;
                    }
                    (None, true) => {
                        partial_segment = Some(VoiceSegment {
                            start: spectrum_start_pos,
                            end: spectrum_start_pos,
                        })
                    }
                    (None, false) => partial_segment = None,
                };
                None
            })
            .collect();

        Self {
            segments_with_voice,
        }
    }

    pub fn voice_less_duration(&self) -> Duration {
        self.segments_with_voice
            .iter()
            .map(|range| range.end - range.start)
            .sum()
    }

    fn beep_where_voice_detected(&self, source: &impl Source) -> SamplesBuffer {
        let sine = sine(source.channels(), source.sample_rate());

        let mut with_voice = [VoiceSegment::ZERO]
            .iter()
            .chain(self.segments_with_voice.iter())
            .peekable();
        let mut samples = Vec::new();

        loop {
            let Some(current_voice_segment) = with_voice.next() else {
                break;
            };

            let voice_range_duration = current_voice_segment.length();
            samples.extend(
                sine.clone()
                    .amplify(1.0)
                    .take_duration(voice_range_duration),
            );

            let Some(next_voice_segment) = with_voice.peek() else {
                break;
            };
            let until_next = current_voice_segment.until(next_voice_segment);
            samples.extend(sine.clone().amplify(0.0).take_duration(until_next));
        }

        SamplesBuffer::new(nz!(1), source.sample_rate(), samples)
    }

    pub fn add_voice_activity_as_channel(mut source: impl Source + Clone) -> impl Source {
        let detector = Self::new(source.clone());
        let mut voice_activity = detector.beep_where_voice_detected(&source).into_iter();

        let mut samples = Vec::new();
        loop {
            let Some(s1) = source.next() else {
                break;
            };
            let Some(s2) = source.next() else {
                break;
            };
            let Some(s3) = voice_activity.next() else {
                break;
            };

            samples.extend_from_slice(&[s1, s2, s3]);
        }
        SamplesBuffer::new(
            source.channels().checked_add(1).unwrap(),
            source.sample_rate(),
            samples,
        )
    }
}

fn iter_spectra(
    expected: impl Source + Clone,
    chunk_duration: Duration,
) -> impl Iterator<Item = FrequencySpectrum> {
    assert!(expected.total_duration().is_some());

    let chunk_size = super::spectra_chunk_size(&expected, chunk_duration);
    let expected_samples: Vec<_> = expected.clone().collect();
    expected_samples
        .chunks_exact(chunk_size)
        .map(|input| {
            super::samples_fft_to_spectrum(
                &hann_window(input),
                expected.sample_rate().get(),
                FrequencyLimit::Min(4.0),
                Some(&divide_by_N_sqrt),
            )
            .unwrap()
        })
        .collect::<Vec<_>>()
        .into_iter()
}

#[cfg(test)]
mod test {

    use crate::test::{detector::BasicVoiceDetector, recording_of_voice};
    use rodio::{nz, wav_to_file};

    #[test]
    fn soup() {
        let original = recording_of_voice(nz!(1), nz!(48000));
        let detector = BasicVoiceDetector::new(original.clone());
        let siny = detector.beep_where_voice_detected(&original);
        wav_to_file(siny, "voice_activity.wav").unwrap();
    }
}
