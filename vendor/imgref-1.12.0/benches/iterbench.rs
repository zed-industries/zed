#![feature(test)]

extern crate test;
use imgref::*;
use test::Bencher;

#[bench]
fn iter_count(bench: &mut Bencher) {
    let img = Img::new_stride(vec![0x11223344u32; 802*600], 800, 600, 802);

    bench.iter(|| {
        img.pixels().count()
    });
}

#[bench]
fn iter_sum(bench: &mut Bencher) {
    let img = Img::new_stride(vec![0x11223344u32; 802*600], 800, 600, 802);

    bench.iter(|| {
        img.pixels().map(|p| p as usize).sum::<usize>()
    });
}

#[bench]
fn stride_ignorant_sum(bench: &mut Bencher) {
    let img = Img::new_stride(vec![0x11223344u32; 802*600], 800, 600, 802);

    bench.iter(|| {
        img.buf().iter().copied().map(|p| p as usize).sum::<usize>()
    });
}

#[bench]
fn chunked_sum(bench: &mut Bencher) {
    let img = Img::new_stride(vec![0x11223344u32; 802*600], 800, 600, 802);

    bench.iter(|| {
        img.buf().chunks(img.stride()).flat_map(|row| row[0..img.width()].iter()).copied().map(|p| p as usize).sum::<usize>()
    });
}
