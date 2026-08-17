#![allow(clippy::needless_range_loop)]

const P64_1: u64 = 0x9E3779B185EBCA87;
const P64_2: u64 = 0xC2B2AE3D27D4EB4F;
const P64_3: u64 = 0x165667B19E3779F9;
const P64_4: u64 = 0x85EBCA77C2B2AE63;
const P64_5: u64 = 0x27D4EB2F165667C5;
const P32_1: u64 = 0x9E3779B1;
const P32_2: u64 = 0x85EBCA77;
const P32_3: u64 = 0xC2B2AE3D;
const PRIME_MX1: u64 = 0x165667919E3779F9;
const PRIME_MX2: u64 = 0x9FB21C651E98DF25;

const STRIPE_LEN: usize = 64;
const SECRET_CONSUME_RATE: usize = 8;
const SECRET_MERGE_START: usize = 11;
const LASTACC_START: usize = 7;
const MIDSIZE_MAX: usize = 240;
const SECRET_MIN: usize = 136;
const MIDSIZE_START: usize = 3;
const MIDSIZE_LASTOFF: usize = 17;

const INIT_ACC: [u64; 8] = [P32_3, P64_1, P64_2, P64_3, P64_4, P32_2, P64_5, P32_1];

/// Pseudorandom secret (`XXH3_kSecret` from xxHash).
const K3SECRET: [u8; 192] = [
    0xb8, 0xfe, 0x6c, 0x39, 0x23, 0xa4, 0x4b, 0xbe, 0x7c, 0x01, 0x81, 0x2c, 0xf7, 0x21, 0xad, 0x1c,
    0xde, 0xd4, 0x6d, 0xe9, 0x83, 0x90, 0x97, 0xdb, 0x72, 0x40, 0xa4, 0xa4, 0xb7, 0xb3, 0x67, 0x1f,
    0xcb, 0x79, 0xe6, 0x4e, 0xcc, 0xc0, 0xe5, 0x78, 0x82, 0x5a, 0xd0, 0x7d, 0xcc, 0xff, 0x72, 0x21,
    0xb8, 0x08, 0x46, 0x74, 0xf7, 0x43, 0x24, 0x8e, 0xe0, 0x35, 0x90, 0xe6, 0x81, 0x3a, 0x26, 0x4c,
    0x3c, 0x28, 0x52, 0xbb, 0x91, 0xc3, 0x00, 0xcb, 0x88, 0xd0, 0x65, 0x8b, 0x1b, 0x53, 0x2e, 0xa3,
    0x71, 0x64, 0x48, 0x97, 0xa2, 0x0d, 0xf9, 0x4e, 0x38, 0x19, 0xef, 0x46, 0xa9, 0xde, 0xac, 0xd8,
    0xa8, 0xfa, 0x76, 0x3f, 0xe3, 0x9c, 0x34, 0x3f, 0xf9, 0xdc, 0xbb, 0xc7, 0xc7, 0x0b, 0x4f, 0x1d,
    0x8a, 0x51, 0xe0, 0x4b, 0xcd, 0xb4, 0x59, 0x31, 0xc8, 0x9f, 0x7e, 0xc9, 0xd9, 0x78, 0x73, 0x64,
    0xea, 0xc5, 0xac, 0x83, 0x34, 0xd3, 0xeb, 0xc3, 0xc5, 0x81, 0xa0, 0xff, 0xfa, 0x13, 0x63, 0xeb,
    0x17, 0x0d, 0xdd, 0x51, 0xb7, 0xf0, 0xda, 0x49, 0xd3, 0x16, 0x55, 0x26, 0x29, 0xd4, 0x68, 0x9e,
    0x2b, 0x16, 0xbe, 0x58, 0x7d, 0x47, 0xa1, 0xfc, 0x8f, 0xf8, 0xb8, 0xd1, 0x7a, 0xd0, 0x31, 0xce,
    0x45, 0xcb, 0x3a, 0x8f, 0x95, 0x16, 0x04, 0x28, 0xaf, 0xd7, 0xfb, 0xca, 0xbb, 0x4b, 0x40, 0x7e,
];

#[inline]
fn read_le32(s: &[u8], o: usize) -> u32 {
    u32::from_le_bytes(s[o..o + 4].try_into().unwrap())
}

#[inline]
fn read_le64(s: &[u8], o: usize) -> u64 {
    u64::from_le_bytes(s[o..o + 8].try_into().unwrap())
}

#[inline]
fn rotl64(x: u64, r: u32) -> u64 {
    x.rotate_left(r)
}

#[inline]
fn mul128_fold64(lhs: u64, rhs: u64) -> u64 {
    let p = u128::from(lhs) * u128::from(rhs);
    (p as u64) ^ ((p >> 64) as u64)
}

#[inline]
fn avalanche64(mut h: u64) -> u64 {
    h ^= h >> 33;
    h = h.wrapping_mul(P64_2);
    h ^= h >> 29;
    h = h.wrapping_mul(P64_3);
    h ^= h >> 32;
    h
}

#[inline]
fn avalanche3(mut h: u64) -> u64 {
    h ^= h >> 37;
    h = h.wrapping_mul(PRIME_MX1);
    h ^= h >> 32;
    h
}

#[inline]
fn rrmxmx(mut h: u64, len: u64) -> u64 {
    h ^= rotl64(h, 49) ^ rotl64(h, 24);
    h = h.wrapping_mul(PRIME_MX2);
    h ^= (h >> 35).wrapping_add(len);
    h = h.wrapping_mul(PRIME_MX2);
    h ^ (h >> 28)
}

fn len_1to3(input: &[u8], len: usize, secret: &[u8], seed: u64) -> u64 {
    let c1 = input[0] as u32;
    let c2 = input[len >> 1] as u32;
    let c3 = input[len - 1] as u32;
    let combined = (c1 << 16) | (c2 << 24) | c3 | ((len as u32) << 8);
    let bitflip = u64::from(read_le32(secret, 0) ^ read_le32(secret, 4)).wrapping_add(seed);
    avalanche64(u64::from(combined) ^ bitflip)
}

fn len_4to8(input: &[u8], len: usize, secret: &[u8], mut seed: u64) -> u64 {
    seed ^= u64::from((seed as u32).swap_bytes()) << 32;
    let input1 = u64::from(read_le32(input, 0));
    let input2 = u64::from(read_le32(input, len - 4));
    let bitflip = (read_le64(secret, 8) ^ read_le64(secret, 16)).wrapping_sub(seed);
    let input64 = input2 + (input1 << 32);
    rrmxmx(input64 ^ bitflip, len as u64)
}

fn len_9to16(input: &[u8], len: usize, secret: &[u8], seed: u64) -> u64 {
    let bitflip1 = (read_le64(secret, 24) ^ read_le64(secret, 32)).wrapping_add(seed);
    let bitflip2 = (read_le64(secret, 40) ^ read_le64(secret, 48)).wrapping_sub(seed);
    let input_lo = read_le64(input, 0) ^ bitflip1;
    let input_hi = read_le64(input, len - 8) ^ bitflip2;
    let acc = (len as u64)
        .wrapping_add(input_lo.swap_bytes())
        .wrapping_add(input_hi)
        .wrapping_add(mul128_fold64(input_lo, input_hi));
    avalanche3(acc)
}

fn len_0to16(input: &[u8], len: usize, secret: &[u8], seed: u64) -> u64 {
    if len > 8 {
        return len_9to16(input, len, secret, seed);
    }
    if len >= 4 {
        return len_4to8(input, len, secret, seed);
    }
    if len > 0 {
        return len_1to3(input, len, secret, seed);
    }
    avalanche64(seed ^ (read_le64(secret, 56) ^ read_le64(secret, 64)))
}

#[inline]
fn mix16b(input: &[u8], secret: &[u8], seed64: u64) -> u64 {
    let il = read_le64(input, 0);
    let ih = read_le64(input, 8);
    mul128_fold64(
        il ^ read_le64(secret, 0).wrapping_add(seed64),
        ih ^ read_le64(secret, 8).wrapping_sub(seed64),
    )
}

fn len_17to128(input: &[u8], len: usize, secret: &[u8], seed: u64) -> u64 {
    let mut acc = (len as u64).wrapping_mul(P64_1);
    let mut i = (len - 1) / 32;
    loop {
        acc = acc.wrapping_add(mix16b(&input[16 * i..], &secret[32 * i..], seed));
        acc = acc.wrapping_add(mix16b(
            &input[len - 16 * (i + 1)..],
            &secret[32 * i + 16..],
            seed,
        ));
        if i == 0 {
            break;
        }
        i -= 1;
    }
    avalanche3(acc)
}

fn len_129to240(input: &[u8], len: usize, secret: &[u8], seed: u64) -> u64 {
    let mut acc = (len as u64).wrapping_mul(P64_1);
    let nb_rounds = len / 16;
    for i in 0..8 {
        acc = acc.wrapping_add(mix16b(&input[16 * i..], &secret[16 * i..], seed));
    }
    let off_last = SECRET_MIN - MIDSIZE_LASTOFF;
    let mut acc_end = mix16b(&input[len - 16..], &secret[off_last..], seed);
    acc = avalanche3(acc);
    for i in 8..nb_rounds {
        acc_end = acc_end.wrapping_add(mix16b(
            &input[16 * i..],
            &secret[16 * (i - 8) + MIDSIZE_START..],
            seed,
        ));
    }
    avalanche3(acc.wrapping_add(acc_end))
}

#[inline]
fn mult32to64_add64(lhs: u64, rhs: u64, acc: u64) -> u64 {
    u64::from(lhs as u32)
        .wrapping_mul(u64::from(rhs as u32))
        .wrapping_add(acc)
}

fn accumulate_512(acc: &mut [u64; 8], input: &[u8], secret: &[u8]) {
    for lane in 0..8 {
        let data_val = read_le64(input, lane * 8);
        let data_key = data_val ^ read_le64(secret, lane * 8);
        acc[lane ^ 1] = acc[lane ^ 1].wrapping_add(data_val);
        acc[lane] = mult32to64_add64(data_key, data_key >> 32, acc[lane]);
    }
}

fn scramble_acc(acc: &mut [u64; 8], secret: &[u8]) {
    for lane in 0..8 {
        let key64 = read_le64(secret, lane * 8);
        let mut a = acc[lane];
        a ^= a >> 47;
        a ^= key64;
        a = a.wrapping_mul(P32_1);
        acc[lane] = a;
    }
}

fn accumulate(acc: &mut [u64; 8], input: &[u8], secret: &[u8], nb_stripes: usize) {
    for n in 0..nb_stripes {
        accumulate_512(
            acc,
            &input[n * STRIPE_LEN..],
            &secret[n * SECRET_CONSUME_RATE..],
        );
    }
}

fn hash_long_internal_loop(
    acc: &mut [u64; 8],
    input: &[u8],
    len: usize,
    secret: &[u8],
    secret_size: usize,
) {
    let nb_stripes_per_block = (secret_size - STRIPE_LEN) / SECRET_CONSUME_RATE;
    let block_len = STRIPE_LEN * nb_stripes_per_block;
    let nb_blocks = (len - 1) / block_len;
    for n in 0..nb_blocks {
        accumulate(acc, &input[n * block_len..], secret, nb_stripes_per_block);
        scramble_acc(acc, &secret[secret_size - STRIPE_LEN..]);
    }
    let nb_stripes = ((len - 1) - (block_len * nb_blocks)) / STRIPE_LEN;
    accumulate(acc, &input[nb_blocks * block_len..], secret, nb_stripes);
    let p = len - STRIPE_LEN;
    accumulate_512(
        acc,
        &input[p..],
        &secret[secret_size - STRIPE_LEN - LASTACC_START..],
    );
}

#[inline]
fn mix2_accs(a0: u64, a1: u64, secret: &[u8]) -> u64 {
    mul128_fold64(a0 ^ read_le64(secret, 0), a1 ^ read_le64(secret, 8))
}

fn merge_accs(acc: &[u64; 8], secret: &[u8], start: u64) -> u64 {
    let mut result64 = start;
    for i in 0..4 {
        result64 = result64.wrapping_add(mix2_accs(acc[2 * i], acc[2 * i + 1], &secret[16 * i..]));
    }
    avalanche3(result64)
}

fn finalize_long(acc: &[u64; 8], secret: &[u8], len: u64) -> u64 {
    merge_accs(acc, &secret[SECRET_MERGE_START..], len.wrapping_mul(P64_1))
}

fn xxh3_64(data: &[u8]) -> u64 {
    let len = data.len();
    let secret = &K3SECRET;
    let seed = 0u64;
    if len <= 16 {
        len_0to16(data, len, secret, seed)
    } else if len <= 128 {
        len_17to128(data, len, secret, seed)
    } else if len <= MIDSIZE_MAX {
        len_129to240(data, len, secret, seed)
    } else {
        let mut acc = INIT_ACC;
        hash_long_internal_loop(&mut acc, data, len, secret, secret.len());
        finalize_long(&acc, secret, len as u64)
    }
}

/// XXH3 64-bit digest of `s` (same as `XXH3_64bits` in the reference implementation).
#[inline]
pub(crate) fn xx3hash(s: &str) -> u64 {
    xxh3_64(s.as_bytes())
}

#[inline]
pub(crate) fn xx3hash_bytes(s: &[u8]) -> u64 {
    xxh3_64(s)
}
