// Copyright 2013-2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Utilities for random number generation
//!
//! This release is a compatibility wrapper around `rand` version 0.4. Please
//! upgrade.

#![doc(html_logo_url = "https://www.rust-lang.org/logos/rust-logo-128x128-blk.png",
       html_favicon_url = "https://www.rust-lang.org/favicon.ico",
       html_root_url = "https://docs.rs/rand/0.3")]

#![deny(missing_debug_implementations)]

#![cfg_attr(feature = "i128_support", feature(i128_type))]

extern crate rand as rand4;

pub use rand4::OsRng;

pub use rand4::{IsaacRng, Isaac64Rng};
pub use rand4::ChaChaRng;

pub mod distributions;
pub use rand4::{isaac, chacha, reseeding, os, read};
mod rand_impls;

pub use rand4::Rng;
pub use rand4::Rand;
pub use rand4::SeedableRng;

pub use rand4::{Generator, AsciiGenerator};
pub use rand4::XorShiftRng;
pub use rand4::{Open01, Closed01};
pub use rand4::StdRng;
pub use rand4::{weak_rng, ThreadRng, thread_rng, random};

#[allow(deprecated)]
pub use rand4::sample;

#[allow(deprecated)]
#[cfg(test)]
mod test {
    use super::{Rng, thread_rng, random, SeedableRng, StdRng, sample,
                weak_rng};
    use std::iter::repeat;

    pub struct MyRng<R> { inner: R }

    impl<R: Rng> Rng for MyRng<R> {
        fn next_u32(&mut self) -> u32 {
            fn next<T: Rng>(t: &mut T) -> u32 {
                t.next_u32()
            }
            next(&mut self.inner)
        }
    }

    pub fn rng() -> MyRng<::ThreadRng> {
        MyRng { inner: ::thread_rng() }
    }

    struct ConstRng { i: u64 }
    impl Rng for ConstRng {
        fn next_u32(&mut self) -> u32 { self.i as u32 }
        fn next_u64(&mut self) -> u64 { self.i }

        // no fill_bytes on purpose
    }

    pub fn iter_eq<I, J>(i: I, j: J) -> bool
        where I: IntoIterator,
              J: IntoIterator<Item=I::Item>,
              I::Item: Eq
    {
        // make sure the iterators have equal length
        let mut i = i.into_iter();
        let mut j = j.into_iter();
        loop {
            match (i.next(), j.next()) {
                (Some(ref ei), Some(ref ej)) if ei == ej => { }
                (None, None) => return true,
                _ => return false,
            }
        }
    }

    #[test]
    fn test_fill_bytes_default() {
        let mut r = ConstRng { i: 0x11_22_33_44_55_66_77_88 };

        // check every remainder mod 8, both in small and big vectors.
        let lengths = [0, 1, 2, 3, 4, 5, 6, 7,
                       80, 81, 82, 83, 84, 85, 86, 87];
        for &n in lengths.iter() {
            let mut v = repeat(0u8).take(n).collect::<Vec<_>>();
            r.fill_bytes(&mut v);

            // use this to get nicer error messages.
            for (i, &byte) in v.iter().enumerate() {
                if byte == 0 {
                    panic!("byte {} of {} is zero", i, n)
                }
            }
        }
    }

    #[test]
    fn test_gen_range() {
        let mut r = thread_rng();
        for _ in 0..1000 {
            let a = r.gen_range(-3, 42);
            assert!(a >= -3 && a < 42);
            assert_eq!(r.gen_range(0, 1), 0);
            assert_eq!(r.gen_range(-12, -11), -12);
        }

        for _ in 0..1000 {
            let a = r.gen_range(10, 42);
            assert!(a >= 10 && a < 42);
            assert_eq!(r.gen_range(0, 1), 0);
            assert_eq!(r.gen_range(3_000_000, 3_000_001), 3_000_000);
        }

    }

    #[test]
    #[should_panic]
    fn test_gen_range_panic_int() {
        let mut r = thread_rng();
        r.gen_range(5, -2);
    }

    #[test]
    #[should_panic]
    fn test_gen_range_panic_usize() {
        let mut r = thread_rng();
        r.gen_range(5, 2);
    }

    #[test]
    fn test_gen_weighted_bool() {
        let mut r = thread_rng();
        assert_eq!(r.gen_weighted_bool(0), true);
        assert_eq!(r.gen_weighted_bool(1), true);
    }

    #[test]
    fn test_gen_ascii_str() {
        let mut r = thread_rng();
        assert_eq!(r.gen_ascii_chars().take(0).count(), 0);
        assert_eq!(r.gen_ascii_chars().take(10).count(), 10);
        assert_eq!(r.gen_ascii_chars().take(16).count(), 16);
    }

    #[test]
    fn test_gen_vec() {
        let mut r = thread_rng();
        assert_eq!(r.gen_iter::<u8>().take(0).count(), 0);
        assert_eq!(r.gen_iter::<u8>().take(10).count(), 10);
        assert_eq!(r.gen_iter::<f64>().take(16).count(), 16);
    }

    #[test]
    fn test_choose() {
        let mut r = thread_rng();
        assert_eq!(r.choose(&[1, 1, 1]).map(|&x|x), Some(1));

        let v: &[isize] = &[];
        assert_eq!(r.choose(v), None);
    }

    #[test]
    fn test_shuffle() {
        let mut r = thread_rng();
        let empty: &mut [isize] = &mut [];
        r.shuffle(empty);
        let mut one = [1];
        r.shuffle(&mut one);
        let b: &[_] = &[1];
        assert_eq!(one, b);

        let mut two = [1, 2];
        r.shuffle(&mut two);
        assert!(two == [1, 2] || two == [2, 1]);

        let mut x = [1, 1, 1];
        r.shuffle(&mut x);
        let b: &[_] = &[1, 1, 1];
        assert_eq!(x, b);
    }

    #[test]
    fn test_thread_rng() {
        let mut r = thread_rng();
        r.gen::<i32>();
        let mut v = [1, 1, 1];
        r.shuffle(&mut v);
        let b: &[_] = &[1, 1, 1];
        assert_eq!(v, b);
        assert_eq!(r.gen_range(0, 1), 0);
    }

    #[test]
    fn test_rng_trait_object() {
        let mut rng = thread_rng();
        {
            let mut r = &mut rng as &mut Rng;
            r.next_u32();
            (&mut r).gen::<i32>();
            let mut v = [1, 1, 1];
            (&mut r).shuffle(&mut v);
            let b: &[_] = &[1, 1, 1];
            assert_eq!(v, b);
            assert_eq!((&mut r).gen_range(0, 1), 0);
        }
        {
            let mut r = Box::new(rng) as Box<Rng>;
            r.next_u32();
            r.gen::<i32>();
            let mut v = [1, 1, 1];
            r.shuffle(&mut v);
            let b: &[_] = &[1, 1, 1];
            assert_eq!(v, b);
            assert_eq!(r.gen_range(0, 1), 0);
        }
    }

    #[test]
    fn test_random() {
        // not sure how to test this aside from just getting some values
        let _n : usize = random();
        let _f : f32 = random();
        let _o : Option<Option<i8>> = random();
        let _many : ((),
                     (usize,
                      isize,
                      Option<(u32, (bool,))>),
                     (u8, i8, u16, i16, u32, i32, u64, i64),
                     (f32, (f64, (f64,)))) = random();
    }

    #[test]
    fn test_sample() {
        let min_val = 1;
        let max_val = 100;

        let mut r = thread_rng();
        let vals = (min_val..max_val).collect::<Vec<i32>>();
        let small_sample = sample(&mut r, vals.iter(), 5);
        let large_sample = sample(&mut r, vals.iter(), vals.len() + 5);

        assert_eq!(small_sample.len(), 5);
        assert_eq!(large_sample.len(), vals.len());

        assert!(small_sample.iter().all(|e| {
            **e >= min_val && **e <= max_val
        }));
    }

    #[test]
    fn test_std_rng_seeded() {
        let s = thread_rng().gen_iter::<usize>().take(256).collect::<Vec<usize>>();
        let mut ra: StdRng = SeedableRng::from_seed(&s[..]);
        let mut rb: StdRng = SeedableRng::from_seed(&s[..]);
        assert!(iter_eq(ra.gen_ascii_chars().take(100),
                        rb.gen_ascii_chars().take(100)));
    }

    #[test]
    fn test_std_rng_reseed() {
        let s = thread_rng().gen_iter::<usize>().take(256).collect::<Vec<usize>>();
        let mut r: StdRng = SeedableRng::from_seed(&s[..]);
        let string1 = r.gen_ascii_chars().take(100).collect::<String>();

        r.reseed(&s);

        let string2 = r.gen_ascii_chars().take(100).collect::<String>();
        assert_eq!(string1, string2);
    }

    #[test]
    fn test_weak_rng() {
        let s = weak_rng().gen_iter::<usize>().take(256).collect::<Vec<usize>>();
        let mut ra: StdRng = SeedableRng::from_seed(&s[..]);
        let mut rb: StdRng = SeedableRng::from_seed(&s[..]);
        assert!(iter_eq(ra.gen_ascii_chars().take(100),
                        rb.gen_ascii_chars().take(100)));
    }
}
