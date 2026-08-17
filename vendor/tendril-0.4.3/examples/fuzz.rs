// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! A simple fuzz tester for the library.

#![deny(warnings)]

extern crate rand;
extern crate tendril;

use std::borrow::ToOwned;

use rand::distributions::{IndependentSample, Range};
use rand::Rng;
use tendril::StrTendril;

fn fuzz() {
    let mut rng = rand::thread_rng();
    let capacity = Range::new(0u32, 1 << 14).ind_sample(&mut rng);
    let mut buf_string = String::with_capacity(capacity as usize);
    let mut buf_tendril = StrTendril::with_capacity(capacity);
    let mut string_slices = vec![];
    let mut tendril_slices = vec![];

    for _ in 1..100_000 {
        if buf_string.len() > (1 << 30) {
            buf_string.truncate(0);
            buf_tendril.clear();
        }

        let dist_action = Range::new(0, 100);
        match dist_action.ind_sample(&mut rng) {
            0..=15 => {
                let (start, end) = random_slice(&mut rng, TEXT);
                let snip = &TEXT[start..end];
                buf_string.push_str(snip);
                buf_tendril.push_slice(snip);
                assert_eq!(&*buf_string, &*buf_tendril);
            }

            16..=31 => {
                let (start, end) = random_slice(&mut rng, &buf_string);
                let snip = &buf_string[start..end].to_owned();
                buf_string.push_str(&snip);
                buf_tendril.push_slice(&snip);
                assert_eq!(&*buf_string, &*buf_tendril);
            }

            32..=47 => {
                let lenstr = format!("[length = {}]", buf_tendril.len());
                buf_string.push_str(&lenstr);
                buf_tendril.push_slice(&lenstr);
                assert_eq!(&*buf_string, &*buf_tendril);
            }

            48..=63 => {
                let n = random_boundary(&mut rng, &buf_string);
                buf_tendril.pop_front(n as u32);
                buf_string = buf_string[n..].to_owned();
                assert_eq!(&*buf_string, &*buf_tendril);
            }

            64..=79 => {
                let new_len = random_boundary(&mut rng, &buf_string);
                let n = buf_string.len() - new_len;
                buf_string.truncate(new_len);
                buf_tendril.pop_back(n as u32);
                assert_eq!(&*buf_string, &*buf_tendril);
            }

            80..=90 => {
                let (start, end) = random_slice(&mut rng, &buf_string);
                buf_string = buf_string[start..end].to_owned();
                buf_tendril = buf_tendril.subtendril(start as u32, (end - start) as u32);
                assert_eq!(&*buf_string, &*buf_tendril);
            }

            91..=96 => {
                let c = rng.gen();
                buf_string.push(c);
                assert!(buf_tendril.try_push_char(c).is_ok());
                assert_eq!(&*buf_string, &*buf_tendril);
            }

            97 => {
                buf_string.truncate(0);
                buf_tendril.clear();
                assert_eq!(&*buf_string, &*buf_tendril);
            }

            _ => {
                let (start, end) = random_slice(&mut rng, &buf_string);
                string_slices.push(buf_string[start..end].to_owned());
                tendril_slices.push(buf_tendril.subtendril(start as u32, (end - start) as u32));
                assert_eq!(string_slices.len(), tendril_slices.len());
                assert!(string_slices
                    .iter()
                    .zip(tendril_slices.iter())
                    .all(|(s, t)| **s == **t));
            }
        }
    }
}

fn random_boundary<R: Rng>(rng: &mut R, text: &str) -> usize {
    loop {
        let i = Range::new(0, text.len() + 1).ind_sample(rng);
        if text.is_char_boundary(i) {
            return i;
        }
    }
}

fn random_slice<R: Rng>(rng: &mut R, text: &str) -> (usize, usize) {
    loop {
        let start = Range::new(0, text.len() + 1).ind_sample(rng);
        let end = Range::new(start, text.len() + 1).ind_sample(rng);
        if !text.is_char_boundary(start) {
            continue;
        }
        if end < text.len() && !text.is_char_boundary(end) {
            continue;
        }
        return (start, end);
    }
}

static TEXT: &'static str =
    "It was from the artists and poets that the pertinent answers came, and I \
     know that panic would have broken loose had they been able to compare notes. \
     As it was, lacking their original letters, I half suspected the compiler of \
     having asked leading questions, or of having edited the correspondence in \
     corroboration of what he had latently resolved to see.\
\
     ˙ǝǝs oʇ pǝʌʃosǝɹ ʎʃʇuǝʇɐʃ pɐɥ ǝɥ ʇɐɥʍ ɟo uoıʇɐɹoqoɹɹoɔ uı ǝɔuǝpuodsǝɹɹoɔ ǝɥʇ \
     pǝʇıpǝ ƃuıʌɐɥ ɟo ɹo 'suoıʇsǝnb ƃuıpɐǝʃ pǝʞsɐ ƃuıʌɐɥ ɟo ɹǝʃıdɯoɔ ǝɥʇ pǝʇɔǝdsns \
     ɟʃɐɥ I 'sɹǝʇʇǝʃ ʃɐuıƃıɹo ɹıǝɥʇ ƃuıʞɔɐʃ 'sɐʍ ʇı s∀ ˙sǝʇou ǝɹɐdɯoɔ oʇ ǝʃqɐ uǝǝq \
     ʎǝɥʇ pɐɥ ǝsooʃ uǝʞoɹq ǝʌɐɥ pʃnoʍ ɔıuɐd ʇɐɥʇ ʍouʞ I puɐ 'ǝɯɐɔ sɹǝʍsuɐ ʇuǝuıʇɹǝd \
     ǝɥʇ ʇɐɥʇ sʇǝod puɐ sʇsıʇɹɐ ǝɥʇ ɯoɹɟ sɐʍ ʇI";

fn main() {
    fuzz();
}
