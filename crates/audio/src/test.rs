//! A complex end to end audio test comparing audio features like fft spectrum
//! of a signal before and after going through the audio pipeline

use std::env::current_dir;
use std::io::Cursor;
use std::iter;
use std::time::Duration;

use gpui::BorrowAppContext;
use rand::SeedableRng;
use rand::rngs::SmallRng;
use rodio::{ChannelCount, Decoder, SampleRate, mixer, nz, wav_to_file};
use rodio::{Source, buffer::SamplesBuffer};
use spectrum_analyzer::scaling::divide_by_N_sqrt;
use spectrum_analyzer::windows::hann_window;
use spectrum_analyzer::{FrequencyLimit, FrequencySpectrum, samples_fft_to_spectrum};

use crate::{Audio, LEGACY_CHANNEL_COUNT, LEGACY_SAMPLE_RATE, RodioExt, VoipParts};

#[gpui::test]
fn test_input_pipeline(cx: &mut gpui::TestAppContext) {
    // strange params to invite bugs to show themselves
    let test_signal = recording_of_davids_voice(nz!(3), nz!(32_000));
    let test_signal_duration = test_signal
        .total_duration()
        .expect("recordings have a length");

    let voip_parts = VoipParts::new(&cx.to_async()).unwrap();
    let input = Audio::input_pipeline(voip_parts, test_signal.clone()).unwrap();

    let pipeline = input
        .take_duration(test_signal_duration)
        .into_samples_buffer();
    rodio::wav_to_file(test_signal.clone(), "test_signal.wav").unwrap();
    let expected = test_signal
        .constant_params(LEGACY_CHANNEL_COUNT, LEGACY_SAMPLE_RATE)
        .into_samples_buffer();
    // expected says its 48khz but its 16khz
    rodio::wav_to_file(expected.clone(), "expected.wav").unwrap();
    rodio::wav_to_file(pipeline.clone(), "pipeline.wav").unwrap();
    assert_similar_voice_spectra(expected, pipeline);
}

#[gpui::test]
fn test_output_pipeline(cx: &mut gpui::TestAppContext) {
    let test_signal = recording_of_davids_voice(LEGACY_CHANNEL_COUNT, LEGACY_SAMPLE_RATE);
    let test_signal_duration = test_signal
        .total_duration()
        .expect("recordings have a length");

    let audio_output =
        cx.update(|cx| cx.update_default_global::<Audio, _>(|audio, _| audio.setup_mixer()));

    cx.update(|cx| {
        Audio::play_voip_stream(test_signal.clone(), "test".to_string(), true, cx).unwrap()
    });

    let audio_output = audio_output
        .take_duration(test_signal_duration)
        .into_samples_buffer();

    // dont care about the channel count and sample rate, as long as the voice
    // signal matches
    let expected_output = test_signal
        .constant_params(audio_output.channels(), audio_output.sample_rate())
        .into_samples_buffer();
    // rodio::wav_to_file(audio_output, "audio_pipeline_output.wav").unwrap();
    assert_similar_voice_spectra(expected_output, audio_output);
}

// TODO make a perf variant
#[gpui::test]
fn test_full_audio_pipeline(cx: &mut gpui::TestAppContext) {
    let test_signal = recording_of_davids_voice(nz!(3), nz!(32_000));
    let test_signal_duration = test_signal
        .total_duration()
        .expect("recordings have a length");

    let audio_output =
        cx.update(|cx| cx.update_default_global::<Audio, _>(|audio, _| audio.setup_mixer()));
    let voip_parts = VoipParts::new(&cx.to_async()).unwrap();

    let input = Audio::input_pipeline(voip_parts, test_signal).unwrap();
    cx.update(|cx| Audio::play_voip_stream(input, "test".to_string(), true, cx).unwrap());

    let audio_output = audio_output
        .take_duration(test_signal_duration)
        .into_samples_buffer();

    // dont care about the channel count and sample rate, as long as the voice
    // signal matches
    let expected_output =
        recording_of_davids_voice(audio_output.channels(), audio_output.sample_rate());
    rodio::wav_to_file(audio_output.clone(), "audio_output.wav").unwrap();
    rodio::wav_to_file(audio_output.clone(), "expected_output.wav").unwrap();
    assert_similar_voice_spectra(expected_output, audio_output);
}

fn energy_of_spectrum(spectrum: &FrequencySpectrum) -> f32 {
    spectrum.max().1.val()
}

fn energy_of_chunk(chunk: &[rodio::Sample], sample_rate: SampleRate) -> f32 {
    let spectrum = samples_fft_to_spectrum(
        &hann_window(chunk),
        sample_rate.get(),
        FrequencyLimit::All,
        Some(&divide_by_N_sqrt),
    )
    .unwrap();

    energy_of_spectrum(&spectrum)
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
fn assert_similar_voice_spectra(
    expected: impl rodio::Source + Clone,
    pipeline: impl rodio::Source + Clone,
) {
    assert!(expected.total_duration().is_some());
    assert!(pipeline.total_duration().is_some());

    assert_eq!(expected.sample_rate(), pipeline.sample_rate());
    assert_eq!(expected.channels(), pipeline.channels());

    let voice_detector = BasicVoiceDetector::new(expected.clone());
    let hundred_millis: usize = usize::try_from(expected.sample_rate().get() / 10)
        .unwrap()
        .next_power_of_two();

    let total_duration = expected.total_duration().expect("just asserted");
    let mut voice_less_duration = Duration::ZERO;

    let expected_samplse: Vec<_> = expected.clone().collect();
    let pipeline_samples: Vec<_> = pipeline.clone().collect();
    for (input, output) in expected_samplse
        .chunks_exact(hundred_millis)
        .zip(pipeline_samples.chunks_exact(hundred_millis))
    {
        let expected = samples_fft_to_spectrum(
            &hann_window(input),
            expected.sample_rate().get(),
            FrequencyLimit::Min(4.0),
            Some(&divide_by_N_sqrt),
        )
        .unwrap();
        let pipeline = samples_fft_to_spectrum(
            &hann_window(output),
            pipeline.sample_rate().get(),
            FrequencyLimit::Min(4.0),
            Some(&divide_by_N_sqrt),
        )
        .unwrap();

        if !voice_detector.may_contain_voice(&expected) {
            voice_less_duration += Duration::from_millis(100);
            continue;
        }
        assert_same_voice_signal(expected, pipeline);
    }

    assert!(
        voice_less_duration.div_duration_f32(total_duration) < 0.5,
        "Test samples should be at least 50% voice and those parts should be recognized as such"
    )
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

fn assert_same_voice_signal(expected: FrequencySpectrum, pipeline: FrequencySpectrum) {
    // The timbre of a voice (the difference in how voices sound) is determined
    // by all kinds of resonances in the throat/mouth. These happen roughly at
    // multiples of the lowest usually loudest voice frequency.

    let (voice_freq_expected, voice_freq_pipeline) = match (
        fundamental_voice_freq(&expected),
        fundamental_voice_freq(&pipeline),
    ) {
        (None, _) => return,
        (Some(voice_freq_expected), None) => {
            panic!(
                "Could not find fundamental voice freq in output while there is one in the input at {voice_freq_expected}Hz.\nLoudest 5 frequencies in output:\n{}\n\n{}",
                display_loudest_5_frequencies(&pipeline),
                plot_spectra(&expected, &pipeline),
            );
        }
        (Some(voice_freq_expected), Some(voice_freq_pipeline)) => {
            (voice_freq_expected, voice_freq_pipeline)
        }
    };

    assert!(less_then_5percent_diff((
        voice_freq_expected,
        voice_freq_pipeline
    )));

    // Guards against voice distortion
    // unfortunately affected by (de)noise as that (de)distorts voice.
    assert!(same_ratio_between_harmonics(
        &expected,
        &pipeline,
        voice_freq_expected
    ));
}

fn fundamental_voice_freq(spectrum: &FrequencySpectrum) -> Option<f32> {
    let human_speech_range = 80.0..260.0;
    let lowest_loud_freq = lowest_loud_frequency(spectrum);
    if !human_speech_range.contains(&lowest_loud_freq) {
        None
    } else {
        Some(lowest_loud_freq)
    }
}

fn same_ratio_between_harmonics(
    expected: &FrequencySpectrum,
    pipeline: &FrequencySpectrum,
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

    ratios(expected, fundamental_voice_freq)
        .zip(ratios(pipeline, fundamental_voice_freq))
        .all(less_then_5percent_diff)
}

fn less_then_5percent_diff((a, b): (f32, f32)) -> bool {
    (a - b).abs() < a * 0.1
}

fn display_loudest_5_frequencies(spectrum: &FrequencySpectrum) -> String {
    let mut spectrum: Vec<_> = spectrum.data().iter().collect();
    spectrum.sort_by_key(|(_, amplitude)| amplitude);
    spectrum.reverse();
    spectrum.truncate(5);
    spectrum
        .into_iter()
        .map(|(freq, amplitude)| format!("freq: {freq},\tamplitude: {amplitude}\n"))
        .collect()
}

// Returns ascii encoding a link to open the plot
fn plot_spectra(expected: &FrequencySpectrum, pipeline: &FrequencySpectrum) -> String {
    use plotly::{Bar, Plot};

    let mut plot = Plot::new();

    let (x, y): (Vec<_>, Vec<_>) = expected
        .data()
        .iter()
        .map(|(freq, amplitude)| (freq.val(), amplitude.val()))
        .unzip();
    let trace = Bar::new(x, y)
        .name("expected")
        .show_legend(true)
        .opacity(0.5);
    plot.add_trace(trace);

    let (x, y): (Vec<_>, Vec<_>) = pipeline
        .data()
        .iter()
        .map(|(freq, amplitude)| (freq.val(), amplitude.val()))
        .unzip();
    let trace = Bar::new(x, y)
        .name("pipeline")
        .show_legend(true)
        .opacity(0.5);
    plot.add_trace(trace);

    let path = current_dir().unwrap().join("plot.html");
    plot.write_html(&path);

    link(path.display(), "Open spectra plot")
}

fn link(target: impl std::fmt::Display, name: &str) -> String {
    const START_OF_LINK: &str = "\x1b]8;;file://";
    const START_OF_NAME: &str = "\x1b\\";
    const END_OF_LINK: &str = "\x1b]8;;\x1b\\";

    format!("{START_OF_LINK}{target}{START_OF_NAME}{name}{END_OF_LINK}")
}

pub(crate) fn sine(channels: ChannelCount, sample_rate: SampleRate) -> impl Source + Clone {
    // We can not resample this file ourselves as then we would not be testing
    // the resampler. These are test files resampled by audacity.
    let recording = match (channels.get(), sample_rate.get()) {
        (1, 48_000) => include_bytes!("../test/sine_1_48000.wav").as_slice(),
        (2, 48_000) => include_bytes!("../test/sine_2_48000.wav").as_slice(),
        _ => panic!("No test recording with {channels} channels and sampler rate: {sample_rate}"),
    };

    let recording = Cursor::new(recording);
    let recording = Decoder::new(recording).unwrap();
    SamplesBuffer::new(
        recording.channels(),
        recording.sample_rate(),
        recording.collect::<Vec<_>>(),
    )
}

pub(crate) fn recording_of_davids_voice(
    channels: ChannelCount,
    sample_rate: SampleRate,
) -> impl Source + Clone {
    // We can not resample this file ourselves as then we would not be testing
    // the resampler. These are test files resampled by audacity.
    let recording = match (channels.get(), sample_rate.get()) {
        (1, 16_000) => include_bytes!("../test/input_test_1_16000.wav").as_slice(),
        (1, 48_000) => include_bytes!("../test/input_test_1_48000.wav").as_slice(),
        (2, 48_000) => include_bytes!("../test/input_test_2_48000.wav").as_slice(),
        (3, 32_000) => include_bytes!("../test/input_test_3_32000.wav").as_slice(),
        _ => panic!("No test recording with {channels} channels and sampler rate: {sample_rate}"),
    };

    let recording = Cursor::new(recording);
    let recording = Decoder::new(recording).unwrap();
    SamplesBuffer::new(
        recording.channels(),
        recording.sample_rate(),
        recording.collect::<Vec<_>>(),
    )
}

#[test]
// #[should_panic]
fn test_rejects_pitch_shift() {
    // also known as 'robot/chipmunk voice'
    let original = recording_of_davids_voice(nz!(1), nz!(16000));
    let pitch_shifted = original
        .clone()
        .speed(1.2) // effectively increases the pitch by 20%
        .constant_samplerate(original.sample_rate())
        .into_samples_buffer();
    wav_to_file(original.clone(), "original.wav").unwrap();
    wav_to_file(pitch_shifted.clone(), "pitch_shifted.wav").unwrap();

    assert_similar_voice_spectra(original, pitch_shifted);
}

#[test]
#[should_panic(expected = "placeholder")]
fn test_rejects_large_amounts_of_noise() {
    let original = recording_of_davids_voice(nz!(1), nz!(16000));
    let with_noise = add_noise(&original, 0.5);

    assert_similar_voice_spectra(original, with_noise);
}

#[test]
fn test_ignores_volume() {
    let original = recording_of_davids_voice(nz!(1), nz!(16000));
    let amplified = original.clone().amplify(1.42);

    assert_similar_voice_spectra(original, amplified);
}

#[test]
fn test_ignore_low_volume_noise() {
    let original = recording_of_davids_voice(nz!(1), nz!(16000));
    // 10% noise is actually already pretty loud as the noise is across
    // all frequencies so is perceived far more intense then a voice
    let with_noise = add_noise(&original, 0.1);
    assert_similar_voice_spectra(original, with_noise);
}

fn add_noise(original: &(impl Source + Clone + Send + 'static), amount: f32) -> SamplesBuffer {
    let original_volume = original.clone().max_by(|a, b| a.total_cmp(b)).unwrap();

    let noise = rodio::source::noise::WhiteUniform::new_with_rng(
        original.sample_rate(),
        SmallRng::seed_from_u64(1), // lets keep failure repeatable
    );
    let (mixer, with_noise) = mixer::mixer(original.channels(), original.sample_rate());

    mixer.add(original.clone());
    mixer.add(noise.amplify(amount * original_volume));

    let with_noise = with_noise
        .take_duration(
            original
                .total_duration()
                .expect("should be a fixed length recording"),
        )
        .into_samples_buffer();
    with_noise
}

#[test]
fn test_ignores_small_shifts() {
    let original = recording_of_davids_voice(nz!(1), nz!(16000));
    let shifted = iter::repeat(0f32).take(10).chain(original.clone());
    let shifted = SamplesBuffer::new(
        original.channels(),
        original.sample_rate(),
        shifted.collect::<Vec<_>>(),
    );

    assert_similar_voice_spectra(original, shifted);
}
