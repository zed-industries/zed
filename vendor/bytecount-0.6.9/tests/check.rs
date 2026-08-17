extern crate bytecount;
#[macro_use]
extern crate quickcheck;
extern crate rand;

use bytecount::{count, naive_count, naive_num_chars, num_chars};
use rand::RngCore;

fn random_bytes(len: usize) -> Vec<u8> {
    let mut result = vec![0; len];
    rand::thread_rng().fill_bytes(&mut result);
    result
}

quickcheck! {
    fn check_count_correct(x: (Vec<u8>, u8)) -> bool {
        let (haystack, needle) = x;
        count(&haystack, needle) == naive_count(&haystack, needle)
    }
}

#[test]
fn check_count_large() {
    let haystack = vec![0u8; if cfg!(miri) { 2_000 } else { 10_000_000 }];
    assert_eq!(naive_count(&haystack, 0), count(&haystack, 0));
    assert_eq!(naive_count(&haystack, 1), count(&haystack, 1));
}

#[test]
fn check_count_large_rand() {
    let haystack = random_bytes(if cfg!(miri) { 200 } else { 100_000 });
    for i in 0..=255 {
        assert_eq!(naive_count(&haystack, i), count(&haystack, i));
    }
}

#[test]
fn check_count_some() {
    let haystack = vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 68];
    let needle = 68;
    assert_eq!(count(&haystack, needle), naive_count(&haystack, needle));
}

#[test]
fn check_count_overflow() {
    let haystack = vec![0, 2, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let needle = 2;
    assert_eq!(count(&haystack, needle), naive_count(&haystack, needle));
}

#[test]
fn check_count_overflow_many() {
    let string = [b'x'; 20000];
    for i in 0..20000 {
        assert_eq!(count(&string[..i], b'x'), i);
    }
}

quickcheck! {
    fn check_num_chars_correct(haystack: Vec<u8>) -> bool {
        num_chars(&haystack) == naive_num_chars(&haystack)
    }
}

#[test]
fn check_num_chars_large() {
    let haystack = vec![0u8; if cfg!(miri) { 2_000 } else { 10_000_000 }];
    assert_eq!(naive_num_chars(&haystack), num_chars(&haystack));
    assert_eq!(naive_num_chars(&haystack), num_chars(&haystack));
}

#[test]
fn check_num_chars_some() {
    let haystack = vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 68];
    assert_eq!(num_chars(&haystack), naive_num_chars(&haystack));
}

#[test]
fn check_num_chars_overflow() {
    let haystack = vec![0, 2, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    assert_eq!(num_chars(&haystack), naive_num_chars(&haystack));
}

#[test]
fn check_num_chars_overflow_many() {
    let string = [b'x'; 20000];
    for i in 0..20000 {
        assert_eq!(num_chars(&string[..i]), i);
    }
}
