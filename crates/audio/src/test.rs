//! A complex end to end audio test comparing audio features like fft spectrum
//! of a signal before and after going through the audio pipeline

use std::io::Cursor;
use std::iter;

use gpui::UpdateGlobal;
use rodio::{Decoder, SampleRate, mixer};
use rodio::{Source, buffer::SamplesBuffer, source::SineWave};
use spectrum_analyzer::scaling::divide_by_N_sqrt;
use spectrum_analyzer::windows::hann_window;
use spectrum_analyzer::{FrequencyLimit, FrequencySpectrum, samples_fft_to_spectrum};

use crate::{Audio, VoipParts};

// TODO make a perf variant
#[gpui::test]
fn test(cx: &mut gpui::TestAppContext) {
    let test_signal = SineWave::new(250.0);
    let voip_parts = VoipParts::new(&cx.to_async()).unwrap();

    let audio_output = cx.update(|cx| Audio::update_global(cx, |audio, _| audio.setup_mixer()));

    let source = Audio::input_pipeline(voip_parts, test_signal.clone()).unwrap();
    cx.update(|cx| Audio::play_voip_stream(source, "test".to_string(), true, cx).unwrap());

    assert_similar_spectra(audio_output.buffered(), test_signal);
}

fn energy_of_chunk(a_chunk: &[rodio::Sample], sample_rate: SampleRate) -> f32 {
    let a_hann_window = hann_window(a_chunk);

    let a_spectrum = samples_fft_to_spectrum(
        &a_hann_window,
        sample_rate.get(),
        FrequencyLimit::All,
        Some(&divide_by_N_sqrt),
    )
    .unwrap();

    a_spectrum.max().1.val()
}

fn maximum_voice_energy(mut a: impl rodio::Source) -> f32 {
    let a_samples: Vec<_> = a.by_ref().collect();
    let ten_millis: usize = usize::try_from(a.sample_rate().get() / 100)
        .unwrap()
        .next_power_of_two();
    a_samples
        .chunks_exact(ten_millis)
        .map(|chunk| energy_of_chunk(chunk, a.sample_rate()))
        .fold(0f32, |max, energy| max.max(energy))
}

fn assert_similar_spectra(mut a: impl rodio::Source + Clone, mut b: impl rodio::Source + Clone) {
    assert_eq!(a.sample_rate(), b.sample_rate());
    assert_eq!(a.channels(), b.channels());
    let a_samples: Vec<_> = a.by_ref().collect();
    let b_samples: Vec<_> = b.by_ref().collect();

    let thresh_a = 0.9 * maximum_voice_energy(a.clone());
    let thresh_b = 0.9 * maximum_voice_energy(b.clone());

    let ten_millis: usize = usize::try_from(a.sample_rate().get() / 100)
        .unwrap()
        .next_power_of_two();
    let mut scores = a_samples
        .chunks_exact(ten_millis)
        .zip(b_samples.chunks_exact(ten_millis))
        .filter_map(|(a_chunk, b_chunk)| {
            let a_hann_window = hann_window(a_chunk);
            let b_hann_window = hann_window(b_chunk);

            let a_spectrum = samples_fft_to_spectrum(
                &a_hann_window,
                a.sample_rate().get(),
                FrequencyLimit::All,
                Some(&divide_by_N_sqrt),
            )
            .unwrap();
            let b_spectrum = samples_fft_to_spectrum(
                &b_hann_window,
                b.sample_rate().get(),
                FrequencyLimit::All,
                Some(&divide_by_N_sqrt),
            )
            .unwrap();
            let rate = a.sample_rate();
            (energy_of_chunk(a_chunk, rate) > thresh_a || energy_of_chunk(b_chunk, rate) > thresh_b)
                .then(|| voice_frequency_similarity(a_spectrum, b_spectrum))
        })
        .collect::<Vec<_>>();
    scores.sort_by(|a, b| a.total_cmp(b));
    let mean = scores
        .iter()
        .fold(0., |total, sim| total + (sim / scores.len() as f32));
    // Floor division
    let ninety_ninth_pct = scores[scores.len() * 99 / 100];

    const THRESH_MEAN: f32 = 0.01;
    const THRESH_99: f32 = 0.05;
    assert!(
        mean < THRESH_MEAN && ninety_ninth_pct < THRESH_99,
        "mean: {mean}\nninety_ninth_pct: {ninety_ninth_pct}"
    );
}

/// A higher number means the spectra are more dissimilar around frequencies
/// indicating humans speak at.
fn voice_frequency_similarity(a: FrequencySpectrum, b: FrequencySpectrum) -> f32 {
    // The fundamental frequency of human speech is between 95 and 225 Hz.
    // The timbre of a voice (the difference in how voices sound) is determined
    // by all kinds of resonances in the throat/mouth. These happen at far
    // higher frequencies. We check some multiples (harmonics). Of the
    // fundamental voice frequency to be sure these harmonics are not distorted.
    let fundamental_voice_freqs = (95..225).into_iter().step_by(15);
    let voice_harmonics = (1..=32)
        .into_iter()
        .flat_map(|i| fundamental_voice_freqs.clone().map(move |freq| freq * i));

    // We do not care about any volume difference between the two. Normalize it.
    let a_average = a.average();
    let b_average = b.average();

    voice_harmonics
        .map(|frequency| {
            let (a_freq, a_amplitude) = a.freq_val_closest(frequency as f32);
            let (b_freq, b_amplitude) = b.freq_val_closest(frequency as f32);

            assert_eq!(a_freq, b_freq);
            // normalize
            let a_amplitude = a_amplitude / a_average;
            let b_amplitude = b_amplitude / b_average;

            (a_amplitude.val() - b_amplitude.val()).abs()
        })
        .fold(0f32, |max, similarity| max.max(similarity))
}

fn test_source() -> impl Source + Clone {
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
    let original = test_source();
    let amplified = original.clone().amplify(1.2);

    assert_similar_spectra(original, amplified);
}

#[test]
fn test_ignore_low_volume_noise() {
    let original = test_source();
    let noise = rodio::source::noise::WhiteUniform::new(original.sample_rate());
    let (mixer, with_noise) = mixer::mixer(original.channels(), original.sample_rate());
    mixer.add(original.clone());
    mixer.add(noise.amplify(0.1));

    assert_similar_spectra(original, with_noise.buffered());
}

#[test]
fn test_ignores_small_shifts() {
    let original = test_source();
    let shifted = iter::repeat(0f32).take(10).chain(original.clone());
    let shifted = SamplesBuffer::new(
        original.channels(),
        original.sample_rate(),
        shifted.collect::<Vec<_>>(),
    );

    assert_similar_spectra(original, shifted);
}

// thread 'test::test_ignores_volume' panicked at crates/audio/src/test.rs:88:5:
// assertion `left == right` failed
//   left: OrderableF32(0.032859605)
//  right: OrderableF32(0.03943154)
