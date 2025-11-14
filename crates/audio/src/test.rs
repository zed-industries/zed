//! A complex end to end audio test comparing audio features like fft spectrum
//! of a signal before and after going through the audio pipeline

use std::env::current_dir;
use std::io::Cursor;
use std::iter;
use std::sync::atomic::Ordering;
use std::time::Duration;

use gpui::BorrowAppContext;
use plotly::layout::Axis;
use rand::SeedableRng;
use rand::rngs::SmallRng;
use rodio::{ChannelCount, Decoder, SampleRate, mixer, nz, wav_to_file};
use rodio::{Source, buffer::SamplesBuffer};
use spectrum_analyzer::scaling::divide_by_N_sqrt;
use spectrum_analyzer::windows::hann_window;
use spectrum_analyzer::{FrequencyLimit, FrequencySpectrum, samples_fft_to_spectrum};

use crate::audio_settings::LIVE_SETTINGS;
use crate::test::detector::BasicVoiceDetector;
use crate::{Audio, LEGACY_CHANNEL_COUNT, LEGACY_SAMPLE_RATE, RodioExt, VoipParts};

#[gpui::test]
fn test_input_pipeline(cx: &mut gpui::TestAppContext) {
    // strange params to invite bugs to show themselves
    let test_signal = recording_of_davids_voice(nz!(3), nz!(48_000));
    let test_signal_duration = test_signal
        .total_duration()
        .expect("recordings have a length");

    let voip_parts = VoipParts::new(&cx.to_async()).unwrap();
    LIVE_SETTINGS.denoise.store(false, Ordering::Relaxed);
    let input = Audio::input_pipeline(voip_parts, test_signal.clone()).unwrap();

    let input_pipeline = input
        .take_duration(test_signal_duration)
        .into_samples_buffer();

    let expected_output =
        recording_of_davids_voice(input_pipeline.channels(), input_pipeline.sample_rate());
    rodio::wav_to_file(input_pipeline.clone(), "input_pipeline_output.wav").unwrap();
    rodio::wav_to_file(expected_output.clone(), "input_pipeline_expect.wav").unwrap();
    assert_similar_voice_spectra(expected_output, input_pipeline);
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

    let output_pipeline = audio_output
        .take_duration(test_signal_duration)
        .into_samples_buffer();

    // dont care about the channel count and sample rate, as long as the voice
    // signal matches
    let expected_output =
        recording_of_davids_voice(output_pipeline.channels(), output_pipeline.sample_rate());
    rodio::wav_to_file(output_pipeline.clone(), "output_pipeline_output.wav").unwrap();
    rodio::wav_to_file(expected_output.clone(), "output_pipeline_expect.wav").unwrap();
    assert_similar_voice_spectra(expected_output, output_pipeline);
}

// TODO make a perf variant
#[gpui::test]
fn test_full_audio_pipeline(cx: &mut gpui::TestAppContext) {
    let test_signal = recording_of_davids_voice(nz!(3), nz!(44_100));
    let test_signal_duration = test_signal
        .total_duration()
        .expect("recordings have a length");

    let audio_output =
        cx.update(|cx| cx.update_default_global::<Audio, _>(|audio, _| audio.setup_mixer()));
    let voip_parts = VoipParts::new(&cx.to_async()).unwrap();

    let input = Audio::input_pipeline(voip_parts, test_signal).unwrap();
    cx.update(|cx| Audio::play_voip_stream(input, "test".to_string(), true, cx).unwrap());

    let full_pipeline = audio_output
        .take_duration(test_signal_duration)
        .into_samples_buffer();

    // dont care about the channel count and sample rate, as long as the voice
    // signal matches
    let expected_output =
        recording_of_davids_voice(full_pipeline.channels(), full_pipeline.sample_rate());
    rodio::wav_to_file(full_pipeline.clone(), "full_pipeline_output.wav").unwrap();
    rodio::wav_to_file(expected_output.clone(), "full_pipeline_expected.wav").unwrap();
    assert_similar_voice_spectra(expected_output, full_pipeline);
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

const CHUNK_DURATION: Duration = Duration::from_millis(100);

mod detector;

// Test signals should be at least 50% voice
fn assert_similar_voice_spectra(
    expected: impl rodio::Source + Clone,
    pipeline: impl rodio::Source + Clone,
) {
    assert!(expected.total_duration().is_some());
    assert!(pipeline.total_duration().is_some());

    assert_eq!(expected.sample_rate(), pipeline.sample_rate());
    assert_eq!(expected.channels(), pipeline.channels());

    let total_duration = expected.total_duration().expect("just asserted");
    let voice_detector = BasicVoiceDetector::new(expected.clone());
    assert!(
        voice_detector
            .voice_less_duration()
            .div_duration_f32(total_duration)
            < 0.75,
        "Test samples should be at least 25% voice and those parts should be recognized as such"
    );

    let expected_samples: Vec<_> = expected.clone().collect();
    let pipeline_samples: Vec<_> = pipeline.clone().collect();

    const CHUNK_DURATION: u64 = 50;
    let (passing, len) = voice_detector
        .segments_with_voice
        .into_iter()
        // beautiful functional code :3
        .flat_map(|to_judge| {
            let segment = to_judge.end - to_judge.start;
            let segments_per_chunk = segment.as_millis() as u64 / CHUNK_DURATION;
            (0..segments_per_chunk)
                .map(|idx| Duration::from_millis(idx * CHUNK_DURATION))
                .map(|offset| to_judge.start + offset)
                .collect::<Vec<_>>()
        })
        .map(|chunk_start| {
            let start =
                chunk_start.as_millis() as usize * expected.sample_rate().get() as usize / 1000;
            // This will slightly over-sample into the next chunk but it's Fine(tm)
            let length = 50 * expected.sample_rate().get() as usize / 1000;
            let end = start + length.next_power_of_two();
            (
                chunk_start,
                (&expected_samples[start..end], &pipeline_samples[start..end]),
            )
        })
        .map(|(chunk_start, (input, output))| {
            (
                chunk_start,
                (
                    samples_fft_to_spectrum(
                        &hann_window(input),
                        expected.sample_rate().get(),
                        FrequencyLimit::Min(4.0),
                        Some(&divide_by_N_sqrt),
                    )
                    .unwrap(),
                    samples_fft_to_spectrum(
                        &hann_window(output),
                        pipeline.sample_rate().get(),
                        FrequencyLimit::Min(4.0),
                        Some(&divide_by_N_sqrt),
                    )
                    .unwrap(),
                ),
            )
        })
        .filter_map(assert_same_voice_signal)
        .fold((0, 0), |(passing, len), passed| {
            (passing + u32::from(passed), len + 1)
        });

    assert!(
        passing > len * 9 / 10,
        ">10% of chunks mismatched: {passing} passing out of {len}"
    );
}

fn spectra_chunk_size(source: &impl Source) -> usize {
    ((CHUNK_DURATION.as_secs_f64() * source.sample_rate().get() as f64).ceil() as usize)
        .next_power_of_two()
}

fn assert_same_voice_signal(
    (chunk_start, (expected, pipeline)): (Duration, (FrequencySpectrum, FrequencySpectrum)),
) -> Option<bool> {
    // The timbre of a voice (the difference in how voices sound) is determined
    // by all kinds of resonances in the throat/mouth. These happen roughly at
    // multiples of the lowest usually loudest voice frequency.

    let (voice_freq_expected, voice_freq_pipeline) = match (
        fundamental_voice_freq(&expected),
        fundamental_voice_freq(&pipeline),
    ) {
        (None, _) => return dbg!(None),
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

    assert!(
        less_than_10percent_diff((voice_freq_expected, voice_freq_pipeline)),
        "expected: {voice_freq_expected}, pipeline: {voice_freq_pipeline}, at: {chunk_start:?}\n\n{}",
        plot_spectra(&expected, &pipeline)
    );

    // Guards against voice distortion
    // unfortunately affected by (de)noise as that (de)distorts voice.
    Some(same_ratio_between_harmonics(
        &expected,
        &pipeline,
        voice_freq_expected,
    ))
}

fn fundamental_voice_freq(spectrum: &FrequencySpectrum) -> Option<f32> {
    let human_speech_range = 90.0..260.0;
    let spectrum: Vec<_> = spectrum.data().iter().collect();
    spectrum
        .iter()
        .filter(|(freq, _)| human_speech_range.contains(&freq.val()))
        // .inspect(|(freq, ampl)| println!("{freq},{ampl}"))
        .max_by(|(_, a_ampl), (_, b_ampl)| a_ampl.val().total_cmp(&b_ampl.val()))
        .map(|(freq, _ampl)| freq.val())
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
            .map(move |i| dbg!(fundamental_voice_freq) * i as f32);
        voice_harmonics.clone().map(move |freq| {
            let (_freq, harmonic) = spectrum.freq_val_closest(freq);
            harmonic.val() / fundamental.val()
        })
    }

    ratios(expected, fundamental_voice_freq)
        .zip(ratios(pipeline, fundamental_voice_freq))
        .all(less_than_20percent_diff)
}

fn less_than_10percent_diff((a, b): (f32, f32)) -> bool {
    dbg!(a, b);
    (a - b).abs() < a.max(b) * 0.1
}

fn less_than_20percent_diff((a, b): (f32, f32)) -> bool {
    dbg!(a, b);
    (a - b).abs() < a.max(b) * 0.3
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

    let layout = plotly::Layout::new().x_axis(Axis::new().type_(plotly::layout::AxisType::Log));
    // .y_axis(Axis::new().type_(plotly::layout::AxisType::Log));
    plot.set_layout(layout);

    let (x, y): (Vec<_>, Vec<_>) = expected
        .data()
        .iter()
        .map(|(freq, amplitude)| (freq.val(), amplitude.val()))
        .filter(|(freq, _)| *freq > 85.0)
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
        .filter(|(freq, _)| *freq > 85.0)
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
        (1, 44_100) => include_bytes!("../test/input_test_1_44100.wav").as_slice(),
        (1, 48_000) => include_bytes!("../test/input_test_1_48000.wav").as_slice(),
        (2, 44_100) => include_bytes!("../test/input_test_2_44100.wav").as_slice(),
        (2, 48_000) => include_bytes!("../test/input_test_2_48000.wav").as_slice(),
        (3, 44_100) => include_bytes!("../test/input_test_3_44100.wav").as_slice(),
        (3, 48_000) => include_bytes!("../test/input_test_3_48000.wav").as_slice(),
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
#[should_panic]
fn test_rejects_pitch_shift() {
    // also known as 'robot/chipmunk voice'
    let original = recording_of_davids_voice(nz!(1), nz!(44100));
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
#[should_panic]
fn test_rejects_large_amounts_of_noise() {
    let original = recording_of_davids_voice(nz!(1), nz!(44100));
    let with_noise = add_noise(&original, 0.5);

    assert_similar_voice_spectra(original, with_noise);
}

#[test]
fn test_ignores_volume() {
    let original = recording_of_davids_voice(nz!(1), nz!(44100));
    let amplified = original.clone().amplify(1.42);

    assert_similar_voice_spectra(original, amplified);
}

#[test]
fn test_ignore_low_volume_noise() {
    let original = recording_of_davids_voice(nz!(1), nz!(44100));
    // 5% noise is quite hearable as the noise is across all frequencies so is
    // perceived far more intense then a voice
    let with_noise = add_noise(&original, 0.05);
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
    let original = recording_of_davids_voice(nz!(1), nz!(44100));
    let shifted = iter::repeat(0f32).take(10).chain(original.clone());
    let shifted = SamplesBuffer::new(
        original.channels(),
        original.sample_rate(),
        shifted.collect::<Vec<_>>(),
    );

    assert_similar_voice_spectra(original, shifted);
}
