use criterion::{criterion_group, criterion_main, Criterion};
use glib::IntoGStr;
use std::hint::black_box;

pub fn str_into_gstr(c: &mut Criterion) {
    c.bench_function("str as IntoGStr", |b| {
        b.iter(|| {
            "abcdefg".run_with_gstr(|g| {
                black_box(g);
            })
        })
    });
}

pub fn string_into_gstr(c: &mut Criterion) {
    c.bench_function("String as IntoGStr", |b| {
        b.iter(|| {
            let mut s = String::from("abcdefg");
            s.shrink_to_fit();
            assert_eq!(s.capacity(), s.len());
            s.run_with_gstr(|g| {
                black_box(g);
            })
        })
    });
}

pub fn string_with_capacity_into_gstr(c: &mut Criterion) {
    c.bench_function("String::with_capacity as IntoGStr", |b| {
        b.iter(|| {
            let mut s = String::with_capacity(100);
            s.push_str("abcdefg");
            assert!(s.capacity() > s.len());
            s.run_with_gstr(|g| {
                black_box(g);
            })
        })
    });
}

criterion_group!(
    benches,
    str_into_gstr,
    string_into_gstr,
    string_with_capacity_into_gstr
);
criterion_main!(benches);
