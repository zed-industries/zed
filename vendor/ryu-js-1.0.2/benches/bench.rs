// cargo bench

#![allow(
    clippy::approx_constant,
    clippy::excessive_precision,
    clippy::unreadable_literal
)]

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::io::Write;

macro_rules! benches {
    ($($name:ident($value:expr),)*) => {
        mod bench_ryu_js {
            use super::*;
            $(
                pub fn $name(c: &mut Criterion) {
                    let mut buf = ryu_js::Buffer::new();

                    c.bench_function(concat!("ryu_js_", stringify!($name)), move |b| b.iter(move || {
                        let value = black_box($value);
                        let formatted = buf.format_finite(value);
                        black_box(formatted);
                    }));
                }
            )*
        }
        criterion_group!(bench_ryu_js, $( bench_ryu_js::$name, )*);

        mod bench_std_fmt {
            use super::*;
            $(
                pub fn $name(c: &mut Criterion) {
                    let mut buf = Vec::with_capacity(20);

                    c.bench_function(concat!("std_fmt_", stringify!($name)), move |b| b.iter(|| {
                        buf.clear();
                        let value = black_box($value);
                        write!(&mut buf, "{}", value).unwrap();
                        black_box(buf.as_slice());
                    }));
                }
            )*
        }
        criterion_group!(bench_std_fmt, $( bench_std_fmt::$name, )*);
        criterion_main!(bench_ryu_js, bench_std_fmt);
    };
}

benches! {
    bench_0_f64(0_f64),
    bench_short_f64(0.1234_f64),
    bench_e_f64(2.718281828459045_f64),
    bench_max_f64(f64::MAX),
    bench_0_f32(0_f32),
    bench_short_f32(0.1234_f32),
    bench_e_f32(2.718281828459045_f32),
    bench_max_f32(f32::MAX),
}
