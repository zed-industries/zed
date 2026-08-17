/* Copyright 2018 Torbjørn Birch Moltu
 *
 * Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
 * http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
 * http://opensource.org/licenses/MIT>, at your option. This file may not be
 * copied, modified, or distributed except according to those terms.
 */

// Run with -- --nocapture to show error messages if setup fails.
// (or use ./do.sh)

// uses /usr/share/dict/ for text to convert to Vec<Utf*Char> and iterate over
#![cfg(all(unix, feature="std"))]
#![feature(test)]
extern crate test;
use test::{Bencher, black_box};
#[macro_use] extern crate lazy_static;
extern crate encode_unicode;
use encode_unicode::{CharExt, Utf8Char, Utf16Char, IterExt};

fn read_or_exit(file: &str) -> String {
    let mut fd = std::fs::File::open(file).unwrap_or_else(|err| {
        if err.kind() == std::io::ErrorKind::NotFound {
            eprintln!("{} not found, skipping benchmarks.", file);
            std::process::exit(0);
        } else {
            eprintln!("Failed to open {}: {}.", file, err);
            std::process::exit(1);
        }
    });
    let mut content = String::new();
    std::io::Read::read_to_string(&mut fd, &mut content).unwrap_or_else(|err| {
        eprintln!("Failed to read {}: {}.", file, err);
        std::process::exit(1);
    });
    content
}

lazy_static!{
    // TODO find a big chinese file; `aptitude search '?provides(wordlist)'` didn't have one
    static ref ENGLISH: String = read_or_exit("/usr/share/dict/american-english");
    static ref UTF8CHARS: Vec<Utf8Char> = ENGLISH.chars().map(|c| c.to_utf8() ).collect();
    static ref UTF16CHARS: Vec<Utf16Char> = ENGLISH.chars().map(|c| c.to_utf16() ).collect();
}


#[bench]
fn utf16_split_all_single_mulititerator(b: &mut Bencher) {
    b.iter(|| {
        black_box(&*UTF16CHARS).iter().to_units().for_each(|u| assert!(u != 0) );
    });
}
#[bench]
fn utf16_split_all_single_flatmap(b: &mut Bencher) {
    b.iter(|| {
        black_box(&*UTF16CHARS).iter().cloned().flatten().for_each(|u| assert!(u != 0) );
    });
}
#[bench]
fn utf16_split_all_single_cloned_flatten(b: &mut Bencher) {
    b.iter(|| {
        black_box(&*UTF16CHARS).iter().cloned().flatten().for_each(|u| assert!(u != 0) );
    });
}


#[bench]
fn utf8_split_mostly_ascii_multiiterator(b: &mut Bencher) {
    b.iter(|| {
        black_box(&*UTF8CHARS).iter().to_bytes().for_each(|b| assert!(b != 0) );
    });
}
#[bench]
fn utf8_split_mostly_ascii_flatmap(b: &mut Bencher) {
    b.iter(|| {
        black_box(&*UTF8CHARS).iter().cloned().flatten().for_each(|b| assert!(b != 0) );
    });
}
#[bench]
fn utf8_split_mostly_ascii_cloned_flatten(b: &mut Bencher) {
    b.iter(|| {
        black_box(&*UTF8CHARS).iter().cloned().flatten().for_each(|b| assert!(b != 0) );
    });
}


#[bench]
fn utf8_extend_mostly_ascii_multiiterator(b: &mut Bencher) {
    b.iter(|| {
        let vec: Vec<u8> = black_box(&*UTF8CHARS).iter().to_bytes().collect();
        assert_eq!(black_box(vec).len(), ENGLISH.len());
    });
}
#[bench]
fn utf8_extend_mostly_ascii_custom(b: &mut Bencher) {
    b.iter(|| {
        let vec: Vec<u8> = black_box(&*UTF8CHARS).iter().collect();
        assert_eq!(black_box(vec).len(), ENGLISH.len());
    });
}
#[bench]
fn utf8_extend_mostly_ascii_custom_str(b: &mut Bencher) {
    b.iter(|| {
        let vec: String = black_box(&*UTF8CHARS).iter().cloned().collect();
        assert_eq!(black_box(vec).len(), ENGLISH.len());
    });
}

#[bench]
fn utf16_extend_all_single_multiiterator(b: &mut Bencher) {
    b.iter(|| {
        let vec: Vec<u16> = black_box(&*UTF16CHARS).iter().to_units().collect();
        assert!(black_box(vec).len() < ENGLISH.len());
    });
}
#[bench]
fn utf16_extend_all_single_custom(b: &mut Bencher) {
    b.iter(|| {
        let vec: Vec<u16> = black_box(&*UTF16CHARS).iter().collect();
        assert!(black_box(vec).len() < ENGLISH.len());
    });
}
