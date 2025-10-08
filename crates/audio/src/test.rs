//! A complex end to end audio test comparing audio features like fft spectrum
//! of a signal before and after going through the audio pipeline

use std::io::Cursor;
use std::iter;
use std::time::Duration;

use gpui::UpdateGlobal;
use rand::SeedableRng;
use rand::rngs::SmallRng;
use rodio::{Decoder, SampleRate, mixer};
use rodio::{Source, buffer::SamplesBuffer};
use spectrum_analyzer::scaling::divide_by_N_sqrt;
use spectrum_analyzer::windows::hann_window;
use spectrum_analyzer::{FrequencyLimit, FrequencySpectrum, samples_fft_to_spectrum};

use crate::{Audio, RodioExt, VoipParts};

// TODO make a perf variant
#[gpui::test]
fn test_audio_chain(cx: &mut gpui::TestAppContext) {
    let test_signal = recording_of_davids_voice();
    let test_signal_duration = test_signal
        .total_duration()
        .expect("recordings have a length");
    let voip_parts = VoipParts::new(&cx.to_async()).unwrap();

    let audio_output = cx.update(|cx| Audio::update_global(cx, |audio, _| audio.setup_mixer()));

    let source = Audio::input_pipeline(voip_parts, test_signal.clone()).unwrap();
    cx.update(|cx| Audio::play_voip_stream(source, "test".to_string(), true, cx).unwrap());

    let channels = audio_output.channels();
    let sample_rate = audio_output.sample_rate();
    let samples: Vec<_> = audio_output.take_duration(test_signal_duration).collect();
    assert!(!samples.is_empty());

    let audio_output = SamplesBuffer::new(channels, sample_rate, samples);

    assert_similar_voice_spectra(test_signal, audio_output);
}

fn energy_of_spectrum(spectrum: &FrequencySpectrum) -> f32 {
    spectrum.max().1.val()
}

fn energy_of_chunk(chunk: &[rodio::Sample], sample_rate: SampleRate) -> f32 {
    let a_hann_window = hann_window(chunk);

    let a_spectrum = samples_fft_to_spectrum(
        &a_hann_window,
        sample_rate.get(),
        FrequencyLimit::All,
        Some(&divide_by_N_sqrt),
    )
    .unwrap();

    energy_of_spectrum(&a_spectrum)
}

fn maximum_energy(mut a: impl rodio::Source) -> f32 {
    let a_samples: Vec<_> = a.by_ref().collect();
    assert!(!a_samples.is_empty());
    let ten_millis: usize = usize::try_from(a.sample_rate().get() / 100)
        .unwrap()
        .next_power_of_two();
    a_samples
        .chunks_exact(ten_millis)
        .map(|chunk| energy_of_chunk(chunk, a.sample_rate()))
        .fold(0f32, |max, energy| max.max(energy))
}

struct BasicVoiceDetector {
    threshold: f32,
}

impl BasicVoiceDetector {
    fn new(source: impl rodio::Source) -> Self {
        Self {
            threshold: 0.5 * maximum_energy(source),
        }
    }
    fn may_contain_voice(&self, spectrum: &FrequencySpectrum) -> bool {
        energy_of_spectrum(&spectrum) > self.threshold
    }
}

// Test signals should be at least 50% voice
fn assert_similar_voice_spectra(a: impl rodio::Source + Clone, b: impl rodio::Source + Clone) {
    assert!(a.total_duration().is_some());
    assert!(b.total_duration().is_some());
    assert_eq!(a.sample_rate(), b.sample_rate());
    assert_eq!(a.channels(), b.channels());
    let a_samples: Vec<_> = a.clone().collect();
    let b_samples: Vec<_> = b.clone().collect();

    let voice_detector = BasicVoiceDetector::new(a.clone());
    let hundred_millis: usize = usize::try_from(a.sample_rate().get() / 10)
        .unwrap()
        .next_power_of_two();

    let scores = a_samples
        .chunks_exact(hundred_millis)
        .zip(b_samples.chunks_exact(hundred_millis))
        .filter_map(|(a_chunk, b_chunk)| {
            let a_hann_window = hann_window(a_chunk);
            let b_hann_window = hann_window(b_chunk);

            let a_spectrum = samples_fft_to_spectrum(
                &a_hann_window,
                a.sample_rate().get(),
                FrequencyLimit::Min(4.0),
                Some(&divide_by_N_sqrt),
            )
            .unwrap();
            let b_spectrum = samples_fft_to_spectrum(
                &b_hann_window,
                b.sample_rate().get(),
                FrequencyLimit::Min(4.0),
                Some(&divide_by_N_sqrt),
            )
            .unwrap();

            if voice_detector.may_contain_voice(&a_spectrum) {
                Some((a_spectrum, b_spectrum))
            } else {
                None
            }
        })
        .filter_map(|(a, b)| same_voice_signal(a, b))
        .collect::<Vec<_>>();

    let scored_duration = Duration::from_millis(100) * scores.len() as u32;
    assert!(
        scored_duration > a.total_duration().unwrap().mul_f32(0.5),
        "Less then 50% of signal 'a' contained voice. Not enough signal to
        check. scored_duration: {scored_duration:?}, input_duration: {:?}",
        a.total_duration().unwrap()
    );

    dbg!(&scores);
    let sameness =
        scores.iter().map(|same| *same as usize as f32).sum::<f32>() / scores.len() as f32;
    assert!(sameness > 0.95);
}

fn lowest_loud_frequency(spectrum: &FrequencySpectrum) -> f32 {
    let mut spectrum: Vec<_> = spectrum.data().iter().collect();
    spectrum.sort_by_key(|(_, amplitude)| amplitude);
    spectrum.reverse();
    spectrum.truncate(3);
    spectrum
        .iter()
        .map(|(freq, _)| freq)
        .min()
        .expect("Spectrum is never empty")
        .val()
}

/// A higher number means the spectra are more dissimilar around frequencies
/// indicating humans speak at.
///
/// Returns None if there is no human speech in spectrum a
fn same_voice_signal(a: FrequencySpectrum, b: FrequencySpectrum) -> Option<bool> {
    // The timbre of a voice (the difference in how voices sound) is determined
    // by all kinds of resonances in the throat/mouth. These happen at far
    // higher frequencies. We check some multiples (harmonics). Of the
    // fundamental voice frequency to be sure these harmonics are not distorted.

    let voice_freq_a = fundamental_voice_freq(&a)?;
    let Some(voice_freq_b) = fundamental_voice_freq(&b) else {
        return Some(false);
    };
    assert!(less_then_5percent_diff((voice_freq_a, voice_freq_b)));

    // guards against distortion of the voice
    // unfortunately affected by (de)noise as that (de)distorts voice.
    assert!(same_ratio_between_harmonics(&a, &b, voice_freq_a));

    Some(
        same_ratio_between_harmonics(&a, &b, voice_freq_a)
            && less_then_5percent_diff((voice_freq_a, fundamental_voice_freq(&b).unwrap_or(0.))),
    )
}

fn fundamental_voice_freq(a: &FrequencySpectrum) -> Option<f32> {
    let human_speech_range = 80.0..260.0;
    let lowest_loud_freq = lowest_loud_frequency(a);
    if !human_speech_range.contains(&lowest_loud_freq) {
        None
    } else {
        Some(lowest_loud_freq)
    }
}

fn same_ratio_between_harmonics(
    a: &FrequencySpectrum,
    b: &FrequencySpectrum,
    fundamental_voice_freq: f32,
) -> bool {
    fn ratios(
        spectrum: &FrequencySpectrum,
        fundamental_voice_freq: f32,
    ) -> impl Iterator<Item = f32> {
        let (_freq, fundamental) = spectrum.freq_val_closest(fundamental_voice_freq);

        let voice_harmonics = (2..=3)
            .into_iter()
            .map(move |i| dbg!(fundamental_voice_freq * i as f32));
        voice_harmonics.clone().map(move |freq| {
            let (_freq, harmonic) = spectrum.freq_val_closest(freq);
            harmonic.val() / fundamental.val()
        })
    }

    ratios(a, fundamental_voice_freq)
        .zip(ratios(b, fundamental_voice_freq))
        .inspect(|(a, b)| println!("a: {a}, b: {b}"))
        .all(less_then_5percent_diff)
}

fn less_then_5percent_diff((a, b): (f32, f32)) -> bool {
    (a - b).abs() < a * 0.1
}

fn recording_of_davids_voice() -> impl Source + Clone {
    let voice = include_bytes!("../input_test.wav");
    let voice = Cursor::new(voice);
    let voice = Decoder::new(voice).unwrap();
    SamplesBuffer::new(
        voice.channels(),
        voice.sample_rate(),
        voice.collect::<Vec<_>>(),
    )
}

#[test]
fn test_ignores_volume() {
    let original = recording_of_davids_voice();
    let amplified = original.clone().amplify(1.42);

    assert_similar_voice_spectra(original, amplified);
}

#[test]
fn test_ignore_low_volume_noise() {
    let original = recording_of_davids_voice();
    let original_volume = original.clone().max_by(|a, b| a.total_cmp(b)).unwrap();
    dbg!(original_volume);

    let noise = rodio::source::noise::WhiteUniform::new_with_rng(
        original.sample_rate(),
        SmallRng::seed_from_u64(1), // lets keep failure repeatable
    );
    let (mixer, with_noise) = mixer::mixer(original.channels(), original.sample_rate());

    mixer.add(original.clone());
    // adding 1% noise can be well heard as the noise is across
    // all frequencies so is perceived far more intense then a single wave
    mixer.add(noise.amplify(0.1 * original_volume));

    let with_noise = with_noise
        .take_duration(
            original
                .total_duration()
                .expect("should be a fixed length recording"),
        )
        .into_samples_buffer();
    assert_similar_voice_spectra(original, with_noise);
}

#[test]
fn test_ignores_small_shifts() {
    let original = recording_of_davids_voice();
    let shifted = iter::repeat(0f32).take(10).chain(original.clone());
    let shifted = SamplesBuffer::new(
        original.channels(),
        original.sample_rate(),
        shifted.collect::<Vec<_>>(),
    );

    assert_similar_voice_spectra(original, shifted);
}
