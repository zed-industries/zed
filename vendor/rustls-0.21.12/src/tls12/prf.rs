use ring::hmac;

fn concat_sign(key: &hmac::Key, a: &[u8], b: &[u8]) -> hmac::Tag {
    let mut ctx = hmac::Context::with_key(key);
    ctx.update(a);
    ctx.update(b);
    ctx.sign()
}

fn p(out: &mut [u8], alg: hmac::Algorithm, secret: &[u8], seed: &[u8]) {
    let hmac_key = hmac::Key::new(alg, secret);

    // A(1)
    let mut current_a = hmac::sign(&hmac_key, seed);
    let chunk_size = alg.digest_algorithm().output_len();
    for chunk in out.chunks_mut(chunk_size) {
        // P_hash[i] = HMAC_hash(secret, A(i) + seed)
        let p_term = concat_sign(&hmac_key, current_a.as_ref(), seed);
        chunk.copy_from_slice(&p_term.as_ref()[..chunk.len()]);

        // A(i+1) = HMAC_hash(secret, A(i))
        current_a = hmac::sign(&hmac_key, current_a.as_ref());
    }
}

fn concat(a: &[u8], b: &[u8]) -> Vec<u8> {
    let mut ret = Vec::new();
    ret.extend_from_slice(a);
    ret.extend_from_slice(b);
    ret
}

pub(crate) fn prf(out: &mut [u8], alg: hmac::Algorithm, secret: &[u8], label: &[u8], seed: &[u8]) {
    let joined_seed = concat(label, seed);
    p(out, alg, secret, &joined_seed);
}

#[cfg(test)]
mod tests {
    use ring::hmac::{HMAC_SHA256, HMAC_SHA512};

    #[test]
    fn check_sha256() {
        let secret = b"\x9b\xbe\x43\x6b\xa9\x40\xf0\x17\xb1\x76\x52\x84\x9a\x71\xdb\x35";
        let seed = b"\xa0\xba\x9f\x93\x6c\xda\x31\x18\x27\xa6\xf7\x96\xff\xd5\x19\x8c";
        let label = b"test label";
        let expect = include_bytes!("../testdata/prf-result.1.bin");
        let mut output = [0u8; 100];

        super::prf(&mut output, HMAC_SHA256, secret, label, seed);
        assert_eq!(expect.len(), output.len());
        assert_eq!(expect.to_vec(), output.to_vec());
    }

    #[test]
    fn check_sha512() {
        let secret = b"\xb0\x32\x35\x23\xc1\x85\x35\x99\x58\x4d\x88\x56\x8b\xbb\x05\xeb";
        let seed = b"\xd4\x64\x0e\x12\xe4\xbc\xdb\xfb\x43\x7f\x03\xe6\xae\x41\x8e\xe5";
        let label = b"test label";
        let expect = include_bytes!("../testdata/prf-result.2.bin");
        let mut output = [0u8; 196];

        super::prf(&mut output, HMAC_SHA512, secret, label, seed);
        assert_eq!(expect.len(), output.len());
        assert_eq!(expect.to_vec(), output.to_vec());
    }
}

#[cfg(bench)]
mod benchmarks {
    #[bench]
    fn bench_sha256(b: &mut test::Bencher) {
        let label = &b"extended master secret"[..];
        let seed = [0u8; 32];
        let key = &b"secret"[..];

        b.iter(|| {
            let mut out = [0u8; 48];
            super::prf(&mut out, ring::hmac::HMAC_SHA256, key, &label, &seed);
            test::black_box(out);
        });
    }
}
