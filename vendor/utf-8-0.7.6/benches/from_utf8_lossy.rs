#![feature(test)]

extern crate test;
extern crate utf8;

#[path = "../tests/shared/data.rs"]
mod data;

#[path = "../tests/shared/string_from_utf8_lossy.rs"]
mod string_from_utf8_lossy;

#[bench]
fn bench_our_string_from_utf8_lossy(bencher: &mut test::Bencher) {
    bencher.bytes = data::DECODED_LOSSY.iter().map(|&(input, _expected)| input.len() as u64).sum();
    bencher.iter(|| {
        for &(input, _expected) in data::DECODED_LOSSY {
            test::black_box(string_from_utf8_lossy::string_from_utf8_lossy(input));
        }
    })
}

#[bench]
fn bench_std_string_from_utf8_lossy(bencher: &mut test::Bencher) {
    bencher.bytes = data::DECODED_LOSSY.iter().map(|&(input, _expected)| input.len() as u64).sum();
    bencher.iter(|| {
        for &(input, _expected) in data::DECODED_LOSSY {
            test::black_box(String::from_utf8_lossy(input));
        }
    })
}
