#![feature(test)]

extern crate test;
extern crate unicode_normalization;

use std::fs;
use test::Bencher;
use unicode_normalization::UnicodeNormalization;

const ASCII: &str = "all types of normalized";
const NFC: &str = "Introducci\u{00f3}n a Unicode.pdf";
const NFD: &str = "Introduccio\u{0301}n a Unicode.pdf";

#[bench]
fn bench_is_nfc_ascii(b: &mut Bencher) {
    b.iter(|| unicode_normalization::is_nfc(ASCII));
}

#[bench]
fn bench_is_nfc_normalized(b: &mut Bencher) {
    b.iter(|| unicode_normalization::is_nfc(NFC));
}

#[bench]
fn bench_is_nfc_not_normalized(b: &mut Bencher) {
    b.iter(|| unicode_normalization::is_nfc(NFD));
}

#[bench]
fn bench_is_nfd_ascii(b: &mut Bencher) {
    b.iter(|| unicode_normalization::is_nfd(ASCII));
}

#[bench]
fn bench_is_nfd_normalized(b: &mut Bencher) {
    b.iter(|| unicode_normalization::is_nfd(NFD));
}

#[bench]
fn bench_is_nfd_not_normalized(b: &mut Bencher) {
    b.iter(|| unicode_normalization::is_nfd(NFC));
}

#[bench]
fn bench_is_nfc_stream_safe_ascii(b: &mut Bencher) {
    b.iter(|| unicode_normalization::is_nfc_stream_safe(ASCII));
}

#[bench]
fn bench_is_nfc_stream_safe_normalized(b: &mut Bencher) {
    b.iter(|| unicode_normalization::is_nfc_stream_safe(NFC));
}

#[bench]
fn bench_is_nfc_stream_safe_not_normalized(b: &mut Bencher) {
    b.iter(|| unicode_normalization::is_nfc_stream_safe(NFD));
}

#[bench]
fn bench_is_nfd_stream_safe_ascii(b: &mut Bencher) {
    b.iter(|| unicode_normalization::is_nfd_stream_safe(ASCII));
}

#[bench]
fn bench_is_nfd_stream_safe_normalized(b: &mut Bencher) {
    b.iter(|| unicode_normalization::is_nfd_stream_safe(NFD));
}

#[bench]
fn bench_is_nfd_stream_safe_not_normalized(b: &mut Bencher) {
    b.iter(|| unicode_normalization::is_nfd_stream_safe(NFC));
}

#[bench]
fn bench_nfc_ascii(b: &mut Bencher) {
    b.iter(|| ASCII.nfc().count());
}

#[bench]
fn bench_nfd_ascii(b: &mut Bencher) {
    b.iter(|| ASCII.nfd().count());
}

#[bench]
fn bench_nfc_long(b: &mut Bencher) {
    let long = fs::read_to_string("benches/long.txt").unwrap();
    b.iter(|| long.nfc().count());
}

#[bench]
fn bench_nfd_long(b: &mut Bencher) {
    let long = fs::read_to_string("benches/long.txt").unwrap();
    b.iter(|| long.nfd().count());
}

#[bench]
fn bench_nfkc_ascii(b: &mut Bencher) {
    b.iter(|| ASCII.nfkc().count());
}

#[bench]
fn bench_nfkd_ascii(b: &mut Bencher) {
    b.iter(|| ASCII.nfkd().count());
}

#[bench]
fn bench_nfkc_long(b: &mut Bencher) {
    let long = fs::read_to_string("benches/long.txt").unwrap();
    b.iter(|| long.nfkc().count());
}

#[bench]
fn bench_nfkd_long(b: &mut Bencher) {
    let long = fs::read_to_string("benches/long.txt").unwrap();
    b.iter(|| long.nfkd().count());
}

#[bench]
fn bench_streamsafe_ascii(b: &mut Bencher) {
    b.iter(|| ASCII.stream_safe().count());
}

#[bench]
fn bench_streamsafe_adversarial(b: &mut Bencher) {
    let s = "bo\u{0300}\u{0301}\u{0302}\u{0303}\u{0304}\u{0305}\u{0306}\u{0307}\u{0308}\u{0309}\u{030a}\u{030b}\u{030c}\u{030d}\u{030e}\u{030f}\u{0310}\u{0311}\u{0312}\u{0313}\u{0314}\u{0315}\u{0316}\u{0317}\u{0318}\u{0319}\u{031a}\u{031b}\u{031c}\u{031d}\u{032e}oom";
    b.iter(|| s.stream_safe().count());
}
