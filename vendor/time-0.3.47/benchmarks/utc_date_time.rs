use std::hint::black_box;

use criterion::Bencher;
use time::macros::{offset, utc_datetime};

setup_benchmark! {
    "UtcDateTime",

    fn to_offset(ben: &mut Bencher<'_>) {
        ben.iter(|| black_box(utc_datetime!(2000-01-01 0:00)).to_offset(black_box(offset!(-5))));
    }
}
