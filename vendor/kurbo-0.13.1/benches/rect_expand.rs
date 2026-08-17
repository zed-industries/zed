// Copyright 2020 the Kurbo Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Benchmarks of rect expansion.

#![allow(missing_docs, reason = "criterion emits undocumented functions")]

use criterion::{Criterion, criterion_group, criterion_main};
use std::hint::black_box;

use kurbo::Rect;

// In positive space
static RECT_POS: Rect = Rect::new(3.3, 3.6, 5.6, 4.1);
// In both positive and negative space
static RECT_PAN: Rect = Rect::new(-3.3, -3.6, 5.6, 4.1);
// In negative space
static RECT_NEG: Rect = Rect::new(-5.6, -4.1, -3.3, -3.6);
// In positive space reverse
static RECT_POR: Rect = Rect::new(5.6, 4.1, 3.3, 3.6);
// In both positive and negative space reverse
static RECT_PNR: Rect = Rect::new(5.6, 4.1, -3.3, -3.6);
// In negative space reverse
static RECT_NER: Rect = Rect::new(-3.3, -3.6, -5.6, -4.1);
// In positive space mixed
static RECT_POM: Rect = Rect::new(3.3, 4.1, 5.6, 3.6);
// In both positive and negative space mixed
static RECT_PNM: Rect = Rect::new(-3.3, 4.1, 5.6, -3.6);
// In negative space mixed
static RECT_NEM: Rect = Rect::new(-5.6, -3.6, -3.3, -4.1);

#[inline]
fn expand(rects: &[Rect; 9]) {
    black_box(rects[0].expand());
    black_box(rects[1].expand());
    black_box(rects[2].expand());

    black_box(rects[3].expand());
    black_box(rects[4].expand());
    black_box(rects[5].expand());

    black_box(rects[6].expand());
    black_box(rects[7].expand());
    black_box(rects[8].expand());
}

fn bench_expand(c: &mut Criterion) {
    // Creating the array here to prevent the compiler from optimizing all of it to NOP.
    let rects: [Rect; 9] = [
        RECT_POS, RECT_PAN, RECT_NEG, RECT_POR, RECT_PNR, RECT_NER, RECT_POM, RECT_PNM, RECT_NEM,
    ];
    c.bench_function("expand", |b| b.iter(|| expand(&rects)));
}

criterion_group!(benches, bench_expand);
criterion_main!(benches);
