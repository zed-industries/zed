// Copyright 2020 the Kurbo Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

#![cfg(nightly)]
#![feature(test)]
extern crate test;
use test::Bencher;

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
    test::black_box(rects[0].expand());
    test::black_box(rects[1].expand());
    test::black_box(rects[2].expand());

    test::black_box(rects[3].expand());
    test::black_box(rects[4].expand());
    test::black_box(rects[5].expand());

    test::black_box(rects[6].expand());
    test::black_box(rects[7].expand());
    test::black_box(rects[8].expand());
}

#[bench]
fn bench_expand(b: &mut Bencher) {
    // Creating the array here to prevent the compiler from optimizing all of it to NOP.
    let rects: [Rect; 9] = [
        RECT_POS, RECT_PAN, RECT_NEG, RECT_POR, RECT_PNR, RECT_NER, RECT_POM, RECT_PNM, RECT_NEM,
    ];
    b.iter(|| expand(&rects));
}
