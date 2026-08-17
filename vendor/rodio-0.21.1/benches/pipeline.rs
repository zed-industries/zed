use std::num::NonZero;
use std::time::Duration;

use divan::Bencher;
use rodio::ChannelCount;
use rodio::{source::UniformSourceIterator, Source};

mod shared;
use shared::music_wav;

fn main() {
    divan::main();
}

#[divan::bench]
fn long(bencher: Bencher) {
    bencher.with_inputs(music_wav).bench_values(|source| {
        let mut take_dur = source
            .high_pass(300)
            .amplify(1.2)
            .speed(0.9)
            .automatic_gain_control(Default::default())
            .delay(Duration::from_secs_f32(0.5))
            .fade_in(Duration::from_secs_f32(2.0))
            .take_duration(Duration::from_secs(10));
        take_dur.set_filter_fadeout();
        let effects_applied = take_dur
            .buffered()
            .reverb(Duration::from_secs_f32(0.05), 0.3)
            .skippable();
        let resampled = UniformSourceIterator::new(
            effects_applied,
            ChannelCount::new(2).unwrap(),
            NonZero::new(40_000).unwrap(),
        );
        resampled.for_each(divan::black_box_drop)
    })
}

#[divan::bench]
fn short(bencher: Bencher) {
    bencher.with_inputs(music_wav).bench_values(|source| {
        source
            .amplify(1.2)
            .low_pass(200)
            .for_each(divan::black_box_drop)
    })
}
