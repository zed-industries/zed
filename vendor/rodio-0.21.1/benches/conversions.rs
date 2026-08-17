use dasp_sample::{Duplex, Sample as DaspSample};
use divan::Bencher;
use rodio::{conversions::SampleTypeConverter, Sample};

mod shared;

fn main() {
    divan::main();
}

#[divan::bench(types = [i16, u16, f32])]
fn from_sample<S: Duplex<Sample>>(bencher: Bencher) {
    bencher
        .with_inputs(|| {
            shared::music_wav()
                .map(|s| s.to_sample::<S>())
                .collect::<Vec<_>>()
                .into_iter()
        })
        .bench_values(|source| {
            SampleTypeConverter::<_, Sample>::new(source).for_each(divan::black_box_drop)
        })
}
