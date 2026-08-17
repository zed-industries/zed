
extern crate arrayvec;
#[macro_use] extern crate bencher;

use std::io::Write;

use arrayvec::ArrayVec;

use bencher::Bencher;
use bencher::black_box;

fn extend_with_constant(b: &mut Bencher) {
    let mut v = ArrayVec::<u8, 512>::new();
    let cap = v.capacity();
    b.iter(|| {
        v.clear();
        let constant = black_box(1);
        v.extend((0..cap).map(move |_| constant));
        v[511]
    });
    b.bytes = v.capacity() as u64;
}

fn extend_with_range(b: &mut Bencher) {
    let mut v = ArrayVec::<u8, 512>::new();
    let cap = v.capacity();
    b.iter(|| {
        v.clear();
        let range = 0..cap;
        v.extend(range.map(|x| black_box(x as _)));
        v[511]
    });
    b.bytes = v.capacity() as u64;
}

fn extend_with_slice(b: &mut Bencher) {
    let mut v = ArrayVec::<u8, 512>::new();
    let data = [1; 512];
    b.iter(|| {
        v.clear();
        let iter = data.iter().map(|&x| x);
        v.extend(iter);
        v[511]
    });
    b.bytes = v.capacity() as u64;
}

fn extend_with_write(b: &mut Bencher) {
    let mut v = ArrayVec::<u8, 512>::new();
    let data = [1; 512];
    b.iter(|| {
        v.clear();
        v.write(&data[..]).ok();
        v[511]
    });
    b.bytes = v.capacity() as u64;
}

fn extend_from_slice(b: &mut Bencher) {
    let mut v = ArrayVec::<u8, 512>::new();
    let data = [1; 512];
    b.iter(|| {
        v.clear();
        v.try_extend_from_slice(&data).ok();
        v[511]
    });
    b.bytes = v.capacity() as u64;
}

benchmark_group!(benches,
                 extend_with_constant,
                 extend_with_range,
                 extend_with_slice,
                 extend_with_write,
                 extend_from_slice
);

benchmark_main!(benches);
