#![no_std]
#![feature(test)]

extern crate test;

use hmac::Hmac;
use pbkdf2::pbkdf2;
use test::Bencher;

#[bench]
pub fn pbkdf2_hmac_sha1_16384_20(bh: &mut Bencher) {
    let password = b"my secure password";
    let salt = b"salty salt";
    let mut buf = [0u8; 20];
    bh.iter(|| {
        pbkdf2::<Hmac<sha1::Sha1>>(password, salt, 16_384, &mut buf);
        test::black_box(&buf);
    });
}

#[bench]
pub fn pbkdf2_hmac_sha256_16384_20(bh: &mut Bencher) {
    let password = b"my secure password";
    let salt = b"salty salt";
    let mut buf = [0u8; 20];
    bh.iter(|| {
        pbkdf2::<Hmac<sha2::Sha256>>(password, salt, 16_384, &mut buf);
        test::black_box(&buf);
    });
}

#[bench]
pub fn pbkdf2_hmac_sha512_16384_20(bh: &mut Bencher) {
    let password = b"my secure password";
    let salt = b"salty salt";
    let mut buf = [0u8; 20];
    bh.iter(|| {
        pbkdf2::<Hmac<sha2::Sha512>>(password, salt, 16_384, &mut buf);
        test::black_box(&buf);
    });
}
