use divan::Bencher;
use rodio::source::UniformSourceIterator;

mod shared;
use shared::music_wav;

use rodio::{SampleRate, Source};

fn main() {
    divan::main();
}

#[divan::bench]
fn no_resampling(bencher: Bencher) {
    bencher
        .with_inputs(|| {
            let source = music_wav();
            (source.channels(), source.sample_rate(), source)
        })
        .bench_values(|(channels, sample_rate, source)| {
            UniformSourceIterator::<_>::new(source, channels, sample_rate)
                .for_each(divan::black_box_drop)
        })
}

// taken from: https://github.com/audiojs/sample-rate/readme.md commit: be31b67
const COMMON_SAMPLE_RATES: [u32; 12] = [
    8_000, 11_025, 16_000, 22_050, 44_100, 48_000, 88_200, 96_000, 176_400, 192_000, 352_800,
    384_000,
];

#[divan::bench(args = COMMON_SAMPLE_RATES)]
fn resample_to(bencher: Bencher, target_sample_rate: u32) {
    let target_sample_rate = SampleRate::new(target_sample_rate).expect("Is not zero");
    bencher
        .with_inputs(|| {
            let source = music_wav();
            (source.channels(), source)
        })
        .bench_values(|(channels, source)| {
            UniformSourceIterator::<_>::new(source, channels, target_sample_rate)
                .for_each(divan::black_box_drop)
        })
}
