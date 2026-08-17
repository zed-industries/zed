/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

use criterion::{criterion_group, criterion_main, Criterion};
use hmac::digest::FixedOutput;
use hmac::{Hmac, Mac};
#[cfg(not(any(target_arch = "powerpc", target_arch = "powerpc64")))]
use ring::hmac::{sign, Context, Key, HMAC_SHA256};
use sha2::Sha256;

pub fn hmac(c: &mut Criterion) {
    c.bench_function("hmac", |b| {
        b.iter(|| {
            let mut mac = Hmac::<Sha256>::new_from_slice(b"secret").unwrap();

            mac.update(b"hello, world");
            mac.finalize_fixed()
        })
    });
}

#[cfg(not(any(target_arch = "powerpc", target_arch = "powerpc64")))]
pub fn ring_multipart(c: &mut Criterion) {
    c.bench_function("ring_multipart", |b| {
        b.iter(|| {
            let k = Key::new(HMAC_SHA256, b"secret");
            let mut ctx = Context::with_key(&k);

            for slice in ["hello", ", ", "world"] {
                ctx.update(slice.as_ref());
            }

            ctx.sign()
        })
    });
}

#[cfg(not(any(target_arch = "powerpc", target_arch = "powerpc64")))]
pub fn ring_one_shot(c: &mut Criterion) {
    c.bench_function("ring_one_shot", |b| {
        b.iter(|| {
            let k = Key::new(HMAC_SHA256, b"secret");

            sign(&k, b"hello, world")
        })
    });
}

#[cfg(not(any(target_arch = "powerpc", target_arch = "powerpc64")))]
criterion_group! {
    name = benches;

    config = Criterion::default();

    targets = hmac, ring_multipart, ring_one_shot
}

#[cfg(any(target_arch = "powerpc", target_arch = "powerpc64"))]
criterion_group! {
    name = benches;

    config = Criterion::default();

    targets = hmac
}

criterion_main!(benches);
