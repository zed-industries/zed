//!XXH3 implementation
//!
//!Provides `Hasher` only for 64bit as 128bit variant would not be much different due to trait
//!being limited to `u64` outputs.

use core::{ptr, mem, slice, hash};

use crate::xxh32_common as xxh32;
use crate::xxh64_common as xxh64;
use crate::xxh3_common::*;
use crate::utils::{Buffer, get_unaligned_chunk, get_aligned_chunk_ref};

// Code is as close to original C implementation as possible
// It does make it look ugly, but it is fast and easy to update once xxhash gets new version.

#[cfg(all(any(target_feature = "sse2", target_feature = "neon", all(target_family = "wasm", target_feature = "simd128")), not(target_feature = "avx2")))]
#[repr(align(16))]
#[derive(Clone)]
struct Acc([u64; ACC_NB]);
#[cfg(target_feature = "avx2")]
#[repr(align(32))]
#[derive(Clone)]
struct Acc([u64; ACC_NB]);
#[cfg(not(any(target_feature = "avx2", target_feature = "neon", all(target_family = "wasm", target_feature = "simd128"), target_feature = "sse2")))]
#[repr(align(8))]
#[derive(Clone)]
struct Acc([u64; ACC_NB]);

const INITIAL_ACC: Acc = Acc([
    xxh32::PRIME_3 as u64, xxh64::PRIME_1, xxh64::PRIME_2, xxh64::PRIME_3,
    xxh64::PRIME_4, xxh32::PRIME_2 as u64, xxh64::PRIME_5, xxh32::PRIME_1 as u64
]);

type LongHashFn = fn(&[u8], u64, &[u8]) -> u64;
type LongHashFn128 = fn(&[u8], u64, &[u8]) -> u128;

#[cfg(all(target_family = "wasm", target_feature = "simd128"))]
type StripeLanes = [[u8; mem::size_of::<core::arch::wasm32::v128>()]; STRIPE_LEN / mem::size_of::<core::arch::wasm32::v128>()];
#[cfg(all(target_arch = "x86", target_feature = "avx2"))]
type StripeLanes = [[u8; mem::size_of::<core::arch::x86::__m256i>()]; STRIPE_LEN / mem::size_of::<core::arch::x86::__m256i>()];
#[cfg(all(target_arch = "x86_64", target_feature = "avx2"))]
type StripeLanes = [[u8; mem::size_of::<core::arch::x86_64::__m256i>()]; STRIPE_LEN / mem::size_of::<core::arch::x86_64::__m256i>()];
#[cfg(all(target_arch = "x86", target_feature = "sse2", not(target_feature = "avx2")))]
type StripeLanes = [[u8; mem::size_of::<core::arch::x86::__m128i>()]; STRIPE_LEN / mem::size_of::<core::arch::x86::__m128i>()];
#[cfg(all(target_arch = "x86_64", target_feature = "sse2", not(target_feature = "avx2")))]
type StripeLanes = [[u8; mem::size_of::<core::arch::x86_64::__m128i>()]; STRIPE_LEN / mem::size_of::<core::arch::x86_64::__m128i>()];
#[cfg(target_feature = "neon")]
type StripeLanes = [[u8; mem::size_of::<core::arch::aarch64::uint8x16_t>()]; STRIPE_LEN / mem::size_of::<core::arch::aarch64::uint8x16_t>()];

#[cfg(any(target_feature = "sse2", target_feature = "avx2"))]
#[inline]
const fn _mm_shuffle(z: u32, y: u32, x: u32, w: u32) -> i32 {
    ((z << 6) | (y << 4) | (x << 2) | w) as i32
}

#[inline(always)]
const fn mult32_to64(left: u32, right: u32) -> u64 {
    (left as u64).wrapping_mul(right as u64)
}

//#[inline(always)]
//fn _mm_prefetch(_ptr: *const i8, _offset: isize) {
//    #[cfg(target_arch = "x86")]
//    unsafe {
//        core::arch::x86::_mm_prefetch(_ptr.offset(_offset), core::arch::x86::_MM_HINT_T0);
//    }
//
//    #[cfg(target_arch = "x86_64")]
//    unsafe {
//        core::arch::x86_64::_mm_prefetch(_ptr.offset(_offset), core::arch::x86_64::_MM_HINT_T0);
//    }
//}

macro_rules! to_u128 {
    ($lo:expr, $hi:expr) => {
        ($lo) as u128 | ((($hi) as u128) << 64)
    };
}

macro_rules! slice_offset_ptr {
    ($slice:expr, $offset:expr) => {{
        let slice = $slice;
        let offset = $offset;
        debug_assert!(slice.len() >= offset);

        #[allow(unused_unsafe)]
        unsafe {
            (slice.as_ptr() as *const u8).add(offset)
        }
    }}
}

#[inline(always)]
fn read_32le_unaligned(data: &[u8], offset: usize) -> u32 {
    u32::from_ne_bytes(*get_aligned_chunk_ref(data, offset)).to_le()
}

#[inline(always)]
fn read_64le_unaligned(data: &[u8], offset: usize) -> u64 {
    u64::from_ne_bytes(*get_aligned_chunk_ref(data, offset)).to_le()
}

#[inline(always)]
fn mix_two_accs(acc: &mut Acc, offset: usize, secret: &[[u8; 8]; 2]) -> u64 {
    mul128_fold64(acc.0[offset] ^ u64::from_ne_bytes(secret[0]).to_le(),
                  acc.0[offset + 1] ^ u64::from_ne_bytes(secret[1]).to_le())
}

#[inline]
fn merge_accs(acc: &mut Acc, secret: &[[[u8; 8]; 2]; 4], mut result: u64) -> u64 {
    macro_rules! mix_two_accs {
        ($idx:literal) => {
            result = result.wrapping_add(mix_two_accs(acc, $idx * 2, &secret[$idx]))
        }
    }

    mix_two_accs!(0);
    mix_two_accs!(1);
    mix_two_accs!(2);
    mix_two_accs!(3);

    avalanche(result)
}

#[inline(always)]
fn mix16_b(input: &[[u8; 8]; 2], secret: &[[u8; 8]; 2], seed: u64) -> u64 {
    let mut input_lo = u64::from_ne_bytes(input[0]).to_le();
    let mut input_hi = u64::from_ne_bytes(input[1]).to_le();

    input_lo ^= u64::from_ne_bytes(secret[0]).to_le().wrapping_add(seed);
    input_hi ^= u64::from_ne_bytes(secret[1]).to_le().wrapping_sub(seed);

    mul128_fold64(input_lo, input_hi)
}

#[inline(always)]
//Inputs are two chunks of unaligned u64
//Secret are two chunks of unaligned (u64, u64)
fn mix32_b(lo: &mut u64, hi: &mut u64, input_1: &[[u8; 8]; 2], input_2: &[[u8; 8]; 2], secret: &[[[u8; 8]; 2]; 2], seed: u64) {
    *lo = lo.wrapping_add(mix16_b(input_1, &secret[0], seed));
    *lo ^= u64::from_ne_bytes(input_2[0]).to_le().wrapping_add(u64::from_ne_bytes(input_2[1]).to_le());

    *hi = hi.wrapping_add(mix16_b(input_2, &secret[1], seed));
    *hi ^= u64::from_ne_bytes(input_1[0]).to_le().wrapping_add(u64::from_ne_bytes(input_1[1]).to_le());
}

#[inline(always)]
fn custom_default_secret(seed: u64) -> [u8; DEFAULT_SECRET_SIZE] {
    let mut result = mem::MaybeUninit::<[u8; DEFAULT_SECRET_SIZE]>::uninit();

    let nb_rounds = DEFAULT_SECRET_SIZE / 16;

    for idx in 0..nb_rounds {
        let low = get_unaligned_chunk::<u64>(&DEFAULT_SECRET, idx * 16).to_le().wrapping_add(seed);
        let hi = get_unaligned_chunk::<u64>(&DEFAULT_SECRET, idx * 16 + 8).to_le().wrapping_sub(seed);

        Buffer {
            ptr: result.as_mut_ptr() as *mut u8,
            len: DEFAULT_SECRET_SIZE,
            offset: idx * 16,
        }.copy_from_slice(&low.to_le_bytes());
        Buffer {
            ptr: result.as_mut_ptr() as *mut u8,
            len: DEFAULT_SECRET_SIZE,
            offset: idx * 16 + 8,
        }.copy_from_slice(&hi.to_le_bytes());
    }

    unsafe {
        result.assume_init()
    }
}

#[cfg(all(target_family = "wasm", target_feature = "simd128"))]
fn accumulate_512_wasm(acc: &mut Acc, input: &StripeLanes, secret: &StripeLanes) {
    const LANES: usize = ACC_NB;

    use core::arch::wasm32::*;

    let mut idx = 0usize;
    let xacc = acc.0.as_mut_ptr() as *mut v128;

    unsafe {
        while idx.wrapping_add(1) < LANES / 2 {
            let data_vec_1 = v128_load(input[idx].as_ptr() as _);
            let data_vec_2 = v128_load(input[idx.wrapping_add(1)].as_ptr() as _);

            let key_vec_1 = v128_load(secret[idx].as_ptr() as _);
            let key_vec_2 = v128_load(secret[idx.wrapping_add(1)].as_ptr() as _);

            let data_key_1 = v128_xor(data_vec_1, key_vec_1);
            let data_key_2 = v128_xor(data_vec_2, key_vec_2);

            let data_swap_1 = i64x2_shuffle::<1, 0>(data_vec_1, data_vec_1);
            let data_swap_2 = i64x2_shuffle::<1, 0>(data_vec_2, data_vec_2);

            let mixed_lo = i32x4_shuffle::<0, 2, 4, 6>(data_key_1, data_key_2);
            let mixed_hi = i32x4_shuffle::<1, 3, 5, 7>(data_key_1, data_key_2);

            let prod_1 = u64x2_extmul_low_u32x4(mixed_lo, mixed_hi);
            let prod_2 = u64x2_extmul_high_u32x4(mixed_lo, mixed_hi);

            let sum_1 = i64x2_add(prod_1, data_swap_1);
            let sum_2 = i64x2_add(prod_2, data_swap_2);

            xacc.add(idx).write(i64x2_add(sum_1, *xacc.add(idx)));
            xacc.add(idx.wrapping_add(1)).write(i64x2_add(sum_2, *xacc.add(idx.wrapping_add(1))));

            idx = idx.wrapping_add(2);
        }
    }
}

#[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
macro_rules! vld1q_u8 {
    ($ptr:expr) => {
        core::arch::aarch64::vld1q_u8($ptr)

    }
}

//For some dumb reasons vld1q_u8 is unstable for arm
#[cfg(all(target_arch = "arm", target_feature = "neon"))]
macro_rules! vld1q_u8 {
    ($ptr:expr) => {
        core::ptr::read_unaligned($ptr as *const core::arch::arm::uint8x16_t)
    }
}

#[cfg(target_feature = "neon")]
fn accumulate_512_neon(acc: &mut Acc, input: &StripeLanes, secret: &StripeLanes) {
    //Full Neon version from xxhash source
    const NEON_LANES: usize = ACC_NB;

    unsafe {
        #[cfg(target_arch = "arm")]
        use core::arch::arm::*;
        #[cfg(target_arch = "aarch64")]
        use core::arch::aarch64::*;

        let mut idx = 0usize;
        let xacc = acc.0.as_mut_ptr() as *mut uint64x2_t;

        while idx.wrapping_add(1) < NEON_LANES / 2 {
            /* data_vec = xinput[i]; */
            let data_vec_1 = vreinterpretq_u64_u8(vld1q_u8!(input[idx].as_ptr()));
            let data_vec_2 = vreinterpretq_u64_u8(vld1q_u8!(input[idx.wrapping_add(1)].as_ptr()));
            /* key_vec  = xsecret[i];  */
            let key_vec_1  = vreinterpretq_u64_u8(vld1q_u8!(secret[idx].as_ptr()));
            let key_vec_2  = vreinterpretq_u64_u8(vld1q_u8!(secret[idx.wrapping_add(1)].as_ptr()));
            /* data_swap = swap(data_vec) */
            let data_swap_1 = vextq_u64(data_vec_1, data_vec_1, 1);
            let data_swap_2 = vextq_u64(data_vec_2, data_vec_2, 1);
            /* data_key = data_vec ^ key_vec; */
            let data_key_1 = veorq_u64(data_vec_1, key_vec_1);
            let data_key_2 = veorq_u64(data_vec_2, key_vec_2);

            let unzipped = vuzpq_u32(
                vreinterpretq_u32_u64(data_key_1),
                vreinterpretq_u32_u64(data_key_2)
            );
            /* data_key_lo = data_key & 0xFFFFFFFF */
            let data_key_lo = unzipped.0;
            /* data_key_hi = data_key >> 32 */
            let data_key_hi = unzipped.1;

            //xxhash does it with inline assembly, but idk if I want to embed it here
            let sum_1 = vmlal_u32(data_swap_1, vget_low_u32(data_key_lo), vget_low_u32(data_key_hi));
            #[cfg(target_arch = "aarch64")]
            let sum_2 = vmlal_high_u32(data_swap_2, data_key_lo, data_key_hi);
            #[cfg(target_arch = "arm")]
            let sum_2 = vmlal_u32(data_swap_2, vget_high_u32(data_key_lo), vget_high_u32(data_key_hi));

            xacc.add(idx).write(vaddq_u64(*xacc.add(idx), sum_1));
            xacc.add(idx.wrapping_add(1)).write(vaddq_u64(*xacc.add(idx.wrapping_add(1)), sum_2));

            idx = idx.wrapping_add(2);
        }
    }
}

#[cfg(all(target_feature = "sse2", not(target_feature = "avx2")))]
fn accumulate_512_sse2(acc: &mut Acc, input: &StripeLanes, secret: &StripeLanes) {
    unsafe {
        #[cfg(target_arch = "x86")]
        use core::arch::x86::*;
        #[cfg(target_arch = "x86_64")]
        use core::arch::x86_64::*;

        let xacc = acc.0.as_mut_ptr() as *mut __m128i;

        for idx in 0..secret.len() {
            let data_vec = _mm_loadu_si128(input[idx].as_ptr() as _);
            let key_vec = _mm_loadu_si128(secret[idx].as_ptr() as _);
            let data_key = _mm_xor_si128(data_vec, key_vec);

            let data_key_lo = _mm_shuffle_epi32(data_key, _mm_shuffle(0, 3, 0, 1));
            let product = _mm_mul_epu32(data_key, data_key_lo);

            let data_swap = _mm_shuffle_epi32(data_vec, _mm_shuffle(1,0,3,2));
            let sum = _mm_add_epi64(*xacc.add(idx), data_swap);
            xacc.add(idx).write(_mm_add_epi64(product, sum));
        }
    }
}

#[cfg(target_feature = "avx2")]
fn accumulate_512_avx2(acc: &mut Acc, input: &StripeLanes, secret: &StripeLanes) {
    unsafe {
        #[cfg(target_arch = "x86")]
        use core::arch::x86::*;
        #[cfg(target_arch = "x86_64")]
        use core::arch::x86_64::*;

        let xacc = acc.0.as_mut_ptr() as *mut __m256i;

        for idx in 0..secret.len() {
            let data_vec = _mm256_loadu_si256(input[idx].as_ptr() as _);
            let key_vec = _mm256_loadu_si256(secret[idx].as_ptr() as _);
            let data_key = _mm256_xor_si256(data_vec, key_vec);

            let data_key_lo = _mm256_srli_epi64(data_key, 32);
            let product = _mm256_mul_epu32(data_key, data_key_lo);

            let data_swap = _mm256_shuffle_epi32(data_vec, _mm_shuffle(1,0,3,2));
            let sum = _mm256_add_epi64(*xacc.add(idx), data_swap);
            xacc.add(idx).write(_mm256_add_epi64(product, sum));
        }
    }
}

#[cfg(not(any(target_feature = "avx2", target_feature = "sse2", target_feature = "neon", all(target_family = "wasm", target_feature = "simd128"))))]
fn accumulate_512_scalar(acc: &mut Acc, input: &[[u8; 8]; ACC_NB], secret: &[[u8; 8]; ACC_NB]) {
    for idx in 0..ACC_NB {
        let data_val = u64::from_ne_bytes(input[idx]).to_le();
        let data_key = data_val ^ u64::from_ne_bytes(secret[idx]).to_le();

        acc.0[idx ^ 1] = acc.0[idx ^ 1].wrapping_add(data_val);
        acc.0[idx] = acc.0[idx].wrapping_add(mult32_to64((data_key & 0xFFFFFFFF) as u32, (data_key >> 32) as u32));
    }
}

#[cfg(all(target_family = "wasm", target_feature = "simd128"))]
use accumulate_512_wasm as accumulate_512;
#[cfg(target_feature = "neon")]
use accumulate_512_neon as accumulate_512;
#[cfg(all(target_feature = "sse2", not(target_feature = "avx2")))]
use accumulate_512_sse2 as accumulate_512;
#[cfg(target_feature = "avx2")]
use accumulate_512_avx2 as accumulate_512;
#[cfg(not(any(target_feature = "avx2", target_feature = "sse2", target_feature = "neon", all(target_family = "wasm", target_feature = "simd128"))))]
use accumulate_512_scalar as accumulate_512;

#[cfg(all(target_family = "wasm", target_feature = "simd128"))]
fn scramble_acc_wasm(acc: &mut Acc, secret: &StripeLanes) {
    use core::arch::wasm32::*;

    let xacc = acc.0.as_mut_ptr() as *mut v128;
    let prime = u64x2_splat(xxh32::PRIME_1 as _);

    unsafe {
        for idx in 0..secret.len() {
            let acc_vec = v128_load(xacc.add(idx) as _);
            let shifted = u64x2_shr(acc_vec, 47);
            let data_vec = v128_xor(acc_vec, shifted);
            let key_vec = v128_load(secret[idx].as_ptr() as _);
            let mixed = v128_xor(data_vec, key_vec);
            xacc.add(idx).write(i64x2_mul(mixed, prime));
        }
    }
}

#[cfg(target_feature = "neon")]
fn scramble_acc_neon(acc: &mut Acc, secret: &StripeLanes) {
    //Full Neon version from xxhash source
    unsafe {
        #[cfg(target_arch = "arm")]
        use core::arch::arm::*;
        #[cfg(target_arch = "aarch64")]
        use core::arch::aarch64::*;

        let xacc = acc.0.as_mut_ptr() as *mut uint64x2_t;

        let prime_low = vdup_n_u32(xxh32::PRIME_1);
        let prime_hi = vreinterpretq_u32_u64(vdupq_n_u64((xxh32::PRIME_1 as u64) << 32));

        for idx in 0..secret.len() {
           /* xacc[i] ^= (xacc[i] >> 47); */
            let acc_vec  = *xacc.add(idx);
            let shifted  = vshrq_n_u64(acc_vec, 47);
            let data_vec = veorq_u64(acc_vec, shifted);

            /* xacc[i] ^= xsecret[i]; */
            //According to xxhash sources you can do unaligned read here
            //but since Rust is kinda retarded about unaligned reads I'll avoid it for now
            let key_vec  = vreinterpretq_u64_u8(vld1q_u8!(secret[idx].as_ptr()));
            let data_key = veorq_u64(data_vec, key_vec);

            let prod_hi = vmulq_u32(vreinterpretq_u32_u64(data_key), prime_hi);
            let data_key_lo = vmovn_u64(data_key);
            xacc.add(idx).write(vmlal_u32(vreinterpretq_u64_u32(prod_hi), data_key_lo, prime_low));
        }
    }
}

#[cfg(all(target_feature = "sse2", not(target_feature = "avx2")))]
fn scramble_acc_sse2(acc: &mut Acc, secret: &StripeLanes) {
    unsafe {
        #[cfg(target_arch = "x86")]
        use core::arch::x86::*;
        #[cfg(target_arch = "x86_64")]
        use core::arch::x86_64::*;

        let xacc = acc.0.as_mut_ptr() as *mut __m128i;
        let prime32 = _mm_set1_epi32(xxh32::PRIME_1 as i32);

        for idx in 0..secret.len() {
            let acc_vec = *xacc.add(idx);
            let shifted = _mm_srli_epi64(acc_vec, 47);
            let data_vec = _mm_xor_si128(acc_vec, shifted);

            let key_vec = _mm_loadu_si128(secret[idx].as_ptr() as _);
            let data_key = _mm_xor_si128(data_vec, key_vec);

            let data_key_hi = _mm_shuffle_epi32(data_key, _mm_shuffle(0, 3, 0, 1));
            let prod_lo = _mm_mul_epu32(data_key, prime32);
            let prod_hi = _mm_mul_epu32(data_key_hi, prime32);
            xacc.add(idx).write(_mm_add_epi64(prod_lo, _mm_slli_epi64(prod_hi, 32)));
        }
    }
}

#[cfg(target_feature = "avx2")]
fn scramble_acc_avx2(acc: &mut Acc, secret: &StripeLanes) {
    unsafe {
        #[cfg(target_arch = "x86")]
        use core::arch::x86::*;
        #[cfg(target_arch = "x86_64")]
        use core::arch::x86_64::*;

        let xacc = acc.0.as_mut_ptr() as *mut __m256i;
        let prime32 = _mm256_set1_epi32(xxh32::PRIME_1 as i32);

        for idx in 0..secret.len() {
            let acc_vec = *xacc.add(idx);
            let shifted = _mm256_srli_epi64(acc_vec, 47);
            let data_vec = _mm256_xor_si256(acc_vec, shifted);

            let key_vec = _mm256_loadu_si256(secret[idx].as_ptr() as _);
            let data_key = _mm256_xor_si256(data_vec, key_vec);

            let data_key_hi = _mm256_srli_epi64(data_key, 32);
            let prod_lo = _mm256_mul_epu32(data_key, prime32);
            let prod_hi = _mm256_mul_epu32(data_key_hi, prime32);
            xacc.add(idx).write(_mm256_add_epi64(prod_lo, _mm256_slli_epi64(prod_hi, 32)));
        }
    }
}

#[cfg(not(any(target_feature = "avx2", target_feature = "sse2", target_feature = "neon", all(target_family = "wasm", target_feature = "simd128"))))]
fn scramble_acc_scalar(acc: &mut Acc, secret: &[[u8; 8]; ACC_NB]) {
    for idx in 0..secret.len() {
        let key = u64::from_ne_bytes(secret[idx]).to_le();
        let mut acc_val = xorshift64(acc.0[idx], 47);
        acc_val ^= key;
        acc.0[idx] = acc_val.wrapping_mul(xxh32::PRIME_1 as u64);
    }
}

#[cfg(all(target_family = "wasm", target_feature = "simd128"))]
use scramble_acc_wasm as scramble_acc;

#[cfg(target_feature = "neon")]
use scramble_acc_neon as scramble_acc;

#[cfg(all(target_feature = "sse2", not(target_feature = "avx2")))]
use scramble_acc_sse2 as scramble_acc;

#[cfg(target_feature = "avx2")]
use scramble_acc_avx2 as scramble_acc;

#[cfg(not(any(target_feature = "avx2", target_feature = "sse2", target_feature = "neon", all(target_family = "wasm", target_feature = "simd128"))))]
use scramble_acc_scalar as scramble_acc;

#[inline(always)]
fn accumulate_loop(acc: &mut Acc, input: *const u8, secret: *const u8, nb_stripes: usize) {
    for idx in 0..nb_stripes {
        unsafe {
            let input = input.add(idx * STRIPE_LEN);
            //Miri complains about it for dumb reason so for not turn off prefetch
            //_mm_prefetch(input as _, 320);

            accumulate_512(acc,
                &*(input as *const _),
                &*(secret.add(idx * SECRET_CONSUME_RATE) as *const _)
            );
        }
    }
}

#[inline]
fn hash_long_internal_loop(acc: &mut Acc, input: &[u8], secret: &[u8]) {
    let nb_stripes = (secret.len() - STRIPE_LEN) / SECRET_CONSUME_RATE;
    let block_len = STRIPE_LEN * nb_stripes;
    let nb_blocks = (input.len() - 1) / block_len;

    for idx in 0..nb_blocks {
        accumulate_loop(acc, slice_offset_ptr!(input, idx * block_len), secret.as_ptr(), nb_stripes);
        scramble_acc(acc, get_aligned_chunk_ref(secret, secret.len() - STRIPE_LEN));
    }

    //last partial block
    debug_assert!(input.len() > STRIPE_LEN);

    let nb_stripes = ((input.len() - 1) - (block_len * nb_blocks)) / STRIPE_LEN;
    debug_assert!(nb_stripes <= (secret.len() / SECRET_CONSUME_RATE));
    accumulate_loop(acc, slice_offset_ptr!(input, nb_blocks * block_len), secret.as_ptr(), nb_stripes);

    //last stripe
    accumulate_512(acc, get_aligned_chunk_ref(input, input.len() - STRIPE_LEN), get_aligned_chunk_ref(secret, secret.len() - STRIPE_LEN - SECRET_LASTACC_START));
}

#[inline(always)]
fn xxh3_64_1to3(input: &[u8], seed: u64, secret: &[u8]) -> u64 {
    let c1; let c2; let c3;
    unsafe {
        c1 = *input.get_unchecked(0);
        c2 = *input.get_unchecked(input.len() >> 1);
        c3 = *input.get_unchecked(input.len() - 1);
    };

    let combo = (c1 as u32) << 16 | (c2 as u32) << 24 | (c3 as u32) << 0 | (input.len() as u32) << 8;
    let flip = ((read_32le_unaligned(secret, 0) ^ read_32le_unaligned(secret, 4)) as u64).wrapping_add(seed);
    xxh64::avalanche((combo as u64) ^ flip)
}

#[inline(always)]
fn xxh3_64_4to8(input: &[u8], mut seed: u64, secret: &[u8]) -> u64 {
    debug_assert!(input.len() >= 4 && input.len() <= 8);

    seed ^= ((seed as u32).swap_bytes() as u64) << 32;

    let input1 = read_32le_unaligned(input, 0);
    let input2 = read_32le_unaligned(input, input.len() - 4);

    let flip = (read_64le_unaligned(secret, 8) ^ read_64le_unaligned(secret, 16)).wrapping_sub(seed);
    let input64 = (input2 as u64).wrapping_add((input1 as u64) << 32);
    let keyed = input64 ^ flip;

    strong_avalanche(keyed, input.len() as u64)
}

#[inline(always)]
fn xxh3_64_9to16(input: &[u8], seed: u64, secret: &[u8]) -> u64 {
    debug_assert!(input.len() >= 9 && input.len() <= 16);

    let flip1 = (read_64le_unaligned(secret, 24) ^ read_64le_unaligned(secret, 32)).wrapping_add(seed);
    let flip2 = (read_64le_unaligned(secret, 40) ^ read_64le_unaligned(secret, 48)).wrapping_sub(seed);

    let input_lo = read_64le_unaligned(input, 0) ^ flip1;
    let input_hi = read_64le_unaligned(input, input.len() - 8) ^ flip2;

    let acc = (input.len() as u64).wrapping_add(input_lo.swap_bytes())
                                  .wrapping_add(input_hi)
                                  .wrapping_add(mul128_fold64(input_lo, input_hi));

    avalanche(acc)
}

#[inline(always)]
fn xxh3_64_0to16(input: &[u8], seed: u64, secret: &[u8]) -> u64 {
    if input.len() > 8 {
        xxh3_64_9to16(input, seed, secret)
    } else if input.len() >= 4 {
        xxh3_64_4to8(input, seed, secret)
    } else if input.len() > 0 {
        xxh3_64_1to3(input, seed, secret)
    } else {
        xxh64::avalanche(seed ^ (read_64le_unaligned(secret, 56) ^ read_64le_unaligned(secret, 64)))
    }
}

#[inline(always)]
fn xxh3_64_7to128(input: &[u8], seed: u64, secret: &[u8]) -> u64 {
    let mut acc = (input.len() as u64).wrapping_mul(xxh64::PRIME_1);

    if input.len() > 32 {
        if input.len() > 64 {
            if input.len() > 96 {
                acc = acc.wrapping_add(mix16_b(
                    get_aligned_chunk_ref(input, 48),
                    get_aligned_chunk_ref(secret, 96),
                    seed
                ));
                acc = acc.wrapping_add(mix16_b(
                    get_aligned_chunk_ref(input, input.len() - 64),
                    get_aligned_chunk_ref(secret, 112),
                    seed
                ));
            }

            acc = acc.wrapping_add(mix16_b(
                get_aligned_chunk_ref(input, 32),
                get_aligned_chunk_ref(secret, 64),
                seed
            ));
            acc = acc.wrapping_add(mix16_b(
                get_aligned_chunk_ref(input, input.len() - 48),
                get_aligned_chunk_ref(secret, 80),
                seed
            ));
        }

        acc = acc.wrapping_add(mix16_b(
            get_aligned_chunk_ref(input, 16),
            get_aligned_chunk_ref(secret, 32),
            seed
        ));
        acc = acc.wrapping_add(mix16_b(
            get_aligned_chunk_ref(input, input.len() - 32),
            get_aligned_chunk_ref(secret, 48),
            seed
        ));
    }

    acc = acc.wrapping_add(mix16_b(
        get_aligned_chunk_ref(input, 0),
        get_aligned_chunk_ref(secret, 0),
        seed
    ));
    acc = acc.wrapping_add(mix16_b(
        get_aligned_chunk_ref(input, input.len() - 16),
        get_aligned_chunk_ref(secret, 16),
        seed
    ));

    avalanche(acc)
}

#[inline(never)]
fn xxh3_64_129to240(input: &[u8], seed: u64, secret: &[u8]) -> u64 {
    const START_OFFSET: usize = 3;
    const LAST_OFFSET: usize = 17;

    let mut acc = (input.len() as u64).wrapping_mul(xxh64::PRIME_1);
    let nb_rounds = input.len() / 16;
    debug_assert!(nb_rounds >= 8);

    let mut idx = 0;
    while idx < 8 {
        acc = acc.wrapping_add(
            mix16_b(
                get_aligned_chunk_ref(input, 16*idx),
                get_aligned_chunk_ref(secret, 16*idx),
                seed
            )
        );
        idx = idx.wrapping_add(1);
    }
    acc = avalanche(acc);

    while idx < nb_rounds {
        acc = acc.wrapping_add(
            mix16_b(
                get_aligned_chunk_ref(input, 16*idx),
                get_aligned_chunk_ref(secret, 16*(idx-8) + START_OFFSET),
                seed
            )
        );
        idx = idx.wrapping_add(1);
    }

    acc = acc.wrapping_add(
        mix16_b(
            get_aligned_chunk_ref(input, input.len()-16),
            get_aligned_chunk_ref(secret, SECRET_SIZE_MIN-LAST_OFFSET),
            seed
        )
    );

    avalanche(acc)
}

#[inline(always)]
fn xxh3_64_internal(input: &[u8], seed: u64, secret: &[u8], long_hash_fn: LongHashFn) -> u64 {
    debug_assert!(secret.len() >= SECRET_SIZE_MIN);

    if input.len() <= 16 {
        xxh3_64_0to16(input, seed, secret)
    } else if input.len() <= 128 {
        xxh3_64_7to128(input, seed, secret)
    } else if input.len() <= MID_SIZE_MAX {
        xxh3_64_129to240(input, seed, secret)
    } else {
        long_hash_fn(input, seed, secret)
    }
}

#[inline(always)]
fn xxh3_64_long_impl(input: &[u8], secret: &[u8]) -> u64 {
    let mut acc = INITIAL_ACC;

    hash_long_internal_loop(&mut acc, input, secret);

    merge_accs(&mut acc, get_aligned_chunk_ref(secret, SECRET_MERGEACCS_START), (input.len() as u64).wrapping_mul(xxh64::PRIME_1))
}

#[inline(never)]
fn xxh3_64_long_with_seed(input: &[u8], seed: u64, _secret: &[u8]) -> u64 {
    match seed {
        0 => xxh3_64_long_impl(input, &DEFAULT_SECRET),
        seed => xxh3_64_long_impl(input, &custom_default_secret(seed)),
    }
}

#[inline(never)]
fn xxh3_64_long_default(input: &[u8], _seed: u64, _secret: &[u8]) -> u64 {
    xxh3_64_long_impl(input, &DEFAULT_SECRET)
}

#[inline(never)]
fn xxh3_64_long_with_secret(input: &[u8], _seed: u64, secret: &[u8]) -> u64 {
    xxh3_64_long_impl(input, secret)
}

#[inline]
///Returns 64bit hash for provided input.
pub fn xxh3_64(input: &[u8]) -> u64 {
    xxh3_64_internal(input, 0, &DEFAULT_SECRET, xxh3_64_long_default)
}

#[inline]
///Returns 64bit hash for provided input using seed.
///
///Note: While overhead of deriving new secret from provided seed is low,
///it would more efficient to generate secret at compile time using special function
///`const_custom_default_secret` from `const_xxh3`
pub fn xxh3_64_with_seed(input: &[u8], seed: u64) -> u64 {
    xxh3_64_internal(input, seed, &DEFAULT_SECRET, xxh3_64_long_with_seed)
}

#[inline]
///Returns 64bit hash for provided input using custom secret.
pub fn xxh3_64_with_secret(input: &[u8], secret: &[u8]) -> u64 {
    xxh3_64_internal(input, 0, secret, xxh3_64_long_with_secret)
}

const INTERNAL_BUFFER_SIZE: usize = 256;
const STRIPES_PER_BLOCK: usize = (DEFAULT_SECRET_SIZE - STRIPE_LEN) / SECRET_CONSUME_RATE;

#[derive(Clone)]
#[repr(align(64))]
struct Aligned64<T>(T);

#[inline]
//Internal function shared between Xxh3 and Xxh3Default
fn xxh3_stateful_consume_stripes(acc: &mut Acc, nb_stripes: usize, nb_stripes_acc: usize, input: *const u8, secret: &[u8; DEFAULT_SECRET_SIZE]) -> usize {
    if (STRIPES_PER_BLOCK - nb_stripes_acc) <= nb_stripes {
        let stripes_to_end = STRIPES_PER_BLOCK - nb_stripes_acc;
        let stripes_after_end = nb_stripes - stripes_to_end;

        accumulate_loop(acc, input, slice_offset_ptr!(secret, nb_stripes_acc * SECRET_CONSUME_RATE), stripes_to_end);
        scramble_acc(acc, get_aligned_chunk_ref(secret, DEFAULT_SECRET_SIZE - STRIPE_LEN));
        accumulate_loop(acc, unsafe { input.add(stripes_to_end * STRIPE_LEN) }, secret.as_ptr(), stripes_after_end);
        stripes_after_end
    } else {
        accumulate_loop(acc, input, slice_offset_ptr!(secret, nb_stripes_acc * SECRET_CONSUME_RATE), nb_stripes);
        nb_stripes_acc.wrapping_add(nb_stripes)
    }
}

//Internal function shared between Xxh3 and Xxh3Default
fn xxh3_stateful_update(
    input: &[u8],
    total_len: &mut u64,
    acc: &mut Acc,
    buffer: &mut Aligned64<[mem::MaybeUninit<u8>; INTERNAL_BUFFER_SIZE]>, buffered_size: &mut u16,
    nb_stripes_acc: &mut usize,
    secret: &Aligned64<[u8; DEFAULT_SECRET_SIZE]>
) {
    const INTERNAL_BUFFER_STRIPES: usize = INTERNAL_BUFFER_SIZE / STRIPE_LEN;

    let mut input_ptr = input.as_ptr();
    let mut input_len = input.len();
    *total_len = total_len.wrapping_add(input_len as u64);

    if (input_len + *buffered_size as usize) <= INTERNAL_BUFFER_SIZE {
        unsafe {
            ptr::copy_nonoverlapping(input_ptr, (buffer.0.as_mut_ptr() as *mut u8).offset(*buffered_size as isize), input_len)
        }
        *buffered_size += input_len as u16;
        return;
    }

    if *buffered_size > 0 {
        let fill_len = INTERNAL_BUFFER_SIZE - *buffered_size as usize;

        unsafe {
            ptr::copy_nonoverlapping(input_ptr, (buffer.0.as_mut_ptr() as *mut u8).offset(*buffered_size as isize), fill_len);
            input_ptr = input_ptr.add(fill_len);
            input_len -= fill_len;
        }

        *nb_stripes_acc = xxh3_stateful_consume_stripes(acc, INTERNAL_BUFFER_STRIPES, *nb_stripes_acc, buffer.0.as_ptr() as *const u8, &secret.0);

        *buffered_size = 0;
    }

    debug_assert_ne!(input_len, 0);
    if input_len > INTERNAL_BUFFER_SIZE {
        loop {
            *nb_stripes_acc = xxh3_stateful_consume_stripes(acc, INTERNAL_BUFFER_STRIPES, *nb_stripes_acc, input_ptr, &secret.0);
            input_ptr = unsafe {
                input_ptr.add(INTERNAL_BUFFER_SIZE)
            };
            input_len = input_len - INTERNAL_BUFFER_SIZE;

            if input_len <= INTERNAL_BUFFER_SIZE {
                break;
            }
        }

        unsafe {
            ptr::copy_nonoverlapping(input_ptr.offset(-(STRIPE_LEN as isize)), (buffer.0.as_mut_ptr() as *mut u8).add(buffer.0.len() - STRIPE_LEN), STRIPE_LEN)
        }
    }

    debug_assert_ne!(input_len, 0);
    debug_assert_eq!(*buffered_size, 0);
    unsafe {
        ptr::copy_nonoverlapping(input_ptr, buffer.0.as_mut_ptr() as *mut u8, input_len)
    }
    *buffered_size = input_len as u16;
}

#[inline(always)]
//Internal function shared between Xxh3 and Xxh3Default
fn xxh3_stateful_digest_internal(acc: &mut Acc, nb_stripes_acc: usize, buffer: &[u8], old_buffer: &[mem::MaybeUninit<u8>], secret: &Aligned64<[u8; DEFAULT_SECRET_SIZE]>) {
    if buffer.len() >= STRIPE_LEN {
        let nb_stripes = (buffer.len() - 1) / STRIPE_LEN;
        xxh3_stateful_consume_stripes(acc, nb_stripes, nb_stripes_acc, buffer.as_ptr(), &secret.0);

        accumulate_512(acc,
            get_aligned_chunk_ref(buffer, buffer.len() - STRIPE_LEN),
            get_aligned_chunk_ref(&secret.0, DEFAULT_SECRET_SIZE - STRIPE_LEN - SECRET_LASTACC_START)
        );
    } else {
        let mut last_stripe = mem::MaybeUninit::<[u8; STRIPE_LEN]>::uninit();
        let catchup_size = STRIPE_LEN - buffer.len();
        debug_assert!(buffer.len() > 0);

        let last_stripe = unsafe {
            ptr::copy_nonoverlapping((old_buffer.as_ptr() as *const u8).add(INTERNAL_BUFFER_SIZE - buffer.len() - catchup_size), last_stripe.as_mut_ptr() as _, catchup_size);
            ptr::copy_nonoverlapping(buffer.as_ptr(), (last_stripe.as_mut_ptr() as *mut u8).add(catchup_size), buffer.len());
            slice::from_raw_parts(last_stripe.as_ptr() as *const u8, buffer.len() + catchup_size)
        };

        accumulate_512(acc, get_aligned_chunk_ref(&last_stripe, 0), get_aligned_chunk_ref(&secret.0, DEFAULT_SECRET_SIZE - STRIPE_LEN - SECRET_LASTACC_START));
    }
}

#[derive(Clone)]
///Default XXH3 Streaming algorithm
///
///This is optimized version of Xxh3 struct that uses default seed/secret
///
///Optimal for use in hash maps
pub struct Xxh3Default {
    acc: Acc,
    buffer: Aligned64<[mem::MaybeUninit<u8>; INTERNAL_BUFFER_SIZE]>,
    buffered_size: u16,
    nb_stripes_acc: usize,
    total_len: u64,
}

impl Xxh3Default {
    const DEFAULT_SECRET: Aligned64<[u8; DEFAULT_SECRET_SIZE]> = Aligned64(DEFAULT_SECRET);

    #[inline(always)]
    ///Creates new hasher with default settings
    pub const fn new() -> Self {
        Self {
            acc: INITIAL_ACC,
            buffer: Aligned64([mem::MaybeUninit::uninit(); INTERNAL_BUFFER_SIZE]),
            buffered_size: 0,
            nb_stripes_acc: 0,
            total_len: 0,
        }
    }

    #[inline(always)]
    ///Resets state
    pub fn reset(&mut self) {
        self.acc = INITIAL_ACC;
        self.total_len = 0;
        self.buffered_size = 0;
        self.nb_stripes_acc = 0;
    }

    #[inline(always)]
    fn buffered_input(&self) -> &[u8] {
        let ptr = self.buffer.0.as_ptr();
        unsafe {
            slice::from_raw_parts(ptr as *const u8, self.buffered_size as usize)
        }
    }

    #[inline(always)]
    fn processed_buffer(&self) -> &[mem::MaybeUninit<u8>] {
        let ptr = self.buffer.0.as_ptr();
        unsafe {
            slice::from_raw_parts(ptr.add(self.buffered_size as usize), self.buffer.0.len() - self.buffered_size as usize)
        }
    }

    #[inline(always)]
    ///Hashes provided chunk
    pub fn update(&mut self, input: &[u8]) {
        xxh3_stateful_update(input, &mut self.total_len, &mut self.acc, &mut self.buffer, &mut self.buffered_size, &mut self.nb_stripes_acc, &Self::DEFAULT_SECRET);
    }

    #[inline(never)]
    fn digest_mid_sized(&self) -> u64 {
        let mut acc = self.acc.clone();
        xxh3_stateful_digest_internal(&mut acc, self.nb_stripes_acc, self.buffered_input(), self.processed_buffer(), &Self::DEFAULT_SECRET);

        merge_accs(&mut acc, get_aligned_chunk_ref(&Self::DEFAULT_SECRET.0, SECRET_MERGEACCS_START),
                    self.total_len.wrapping_mul(xxh64::PRIME_1))
    }

    #[inline(never)]
    fn digest_mid_sized_128(&self) -> u128 {
        let mut acc = self.acc.clone();
        xxh3_stateful_digest_internal(&mut acc, self.nb_stripes_acc, self.buffered_input(), self.processed_buffer(), &Self::DEFAULT_SECRET);

        let low = merge_accs(&mut acc, get_aligned_chunk_ref(&Self::DEFAULT_SECRET.0, SECRET_MERGEACCS_START),
                                self.total_len.wrapping_mul(xxh64::PRIME_1));
        let high = merge_accs(&mut acc, get_aligned_chunk_ref(&Self::DEFAULT_SECRET.0,
                                  DEFAULT_SECRET_SIZE - mem::size_of_val(&self.acc) - SECRET_MERGEACCS_START),
                              !self.total_len.wrapping_mul(xxh64::PRIME_2));
        ((high as u128) << 64) | (low as u128)
    }

    ///Computes hash.
    pub fn digest(&self) -> u64 {
        //Separating digest mid sized allows us to inline this function, which benefits
        //code generation when hashing fixed size types and/or if the seed is known.
        if self.total_len > MID_SIZE_MAX as u64 {
            self.digest_mid_sized()
        } else {
            xxh3_64_internal(self.buffered_input(), 0, &Self::DEFAULT_SECRET.0, xxh3_64_long_default)
        }
    }

    ///Computes hash as 128bit integer.
    pub fn digest128(&self) -> u128 {
        //Separating digest mid sized allows us to inline this function, which benefits
        //code generation when hashing fixed size types and/or if the seed is known.
        if self.total_len > MID_SIZE_MAX as u64 {
            self.digest_mid_sized_128()
        } else {
            xxh3_128_internal(self.buffered_input(), 0, &Self::DEFAULT_SECRET.0, xxh3_128_long_default)
        }
    }
}

impl Default for Xxh3Default {
    #[inline(always)]
    fn default() -> Self {
        Self::new()
    }
}


impl hash::Hasher for Xxh3Default {
    #[inline(always)]
    fn finish(&self) -> u64 {
        self.digest()
    }

    #[inline(always)]
    fn write(&mut self, input: &[u8]) {
        self.update(input)
    }
}

#[cfg(feature = "std")]
impl std::io::Write for Xxh3Default {
    #[inline]
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.update(buf);
        Ok(buf.len())
    }

    #[inline]
    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

#[derive(Clone)]
///XXH3 Streaming algorithm
///
///Internal state uses rather large buffers, therefore it might be beneficial
///to store hasher on heap rather than stack.
///Implementation makes no attempts at that, leaving choice entirely to user.
///
///Note that it is better to use [Xxh3Default](struct.Xxh3Default.html) in hash maps
///due to Rust hash interface which requires to create new instance of hasher every time.
pub struct Xxh3 {
    acc: Acc,
    custom_secret: Aligned64<[u8; DEFAULT_SECRET_SIZE]>,
    buffer: Aligned64<[mem::MaybeUninit<u8>; INTERNAL_BUFFER_SIZE]>,
    buffered_size: u16,
    nb_stripes_acc: usize,
    total_len: u64,
    seed: u64,
}

impl Xxh3 {
    #[inline(always)]
    ///Creates new hasher with default settings
    pub const fn new() -> Self {
        Self::with_custom_ops(0, DEFAULT_SECRET)
    }

    #[inline]
    ///Creates new hasher with all options.
    const fn with_custom_ops(seed: u64, secret: [u8; DEFAULT_SECRET_SIZE]) -> Self {
        Self {
            acc: INITIAL_ACC,
            custom_secret: Aligned64(secret),
            buffer: Aligned64([mem::MaybeUninit::uninit(); INTERNAL_BUFFER_SIZE]),
            buffered_size: 0,
            nb_stripes_acc: 0,
            total_len: 0,
            seed,
        }
    }

    #[inline(always)]
    ///Creates new hasher with custom seed.
    pub const fn with_secret(secret: [u8; DEFAULT_SECRET_SIZE]) -> Self {
        Self::with_custom_ops(0, secret)
    }

    #[inline(always)]
    ///Creates new hasher with custom seed.
    pub fn with_seed(seed: u64) -> Self {
        Self::with_custom_ops(seed, custom_default_secret(seed))
    }

    #[inline(always)]
    ///Resets state
    pub fn reset(&mut self) {
        self.acc = INITIAL_ACC;
        self.total_len = 0;
        self.buffered_size = 0;
        self.nb_stripes_acc = 0;
    }

    #[inline(always)]
    fn buffered_input(&self) -> &[u8] {
        let ptr = self.buffer.0.as_ptr();
        unsafe {
            slice::from_raw_parts(ptr as *const u8, self.buffered_size as usize)
        }
    }

    #[inline(always)]
    fn processed_buffer(&self) -> &[mem::MaybeUninit<u8>] {
        let ptr = self.buffer.0.as_ptr();
        unsafe {
            slice::from_raw_parts(ptr.add(self.buffered_size as usize), self.buffer.0.len() - self.buffered_size as usize)
        }
    }

    #[inline]
    ///Hashes provided chunk
    pub fn update(&mut self, input: &[u8]) {
        xxh3_stateful_update(input, &mut self.total_len, &mut self.acc, &mut self.buffer, &mut self.buffered_size, &mut self.nb_stripes_acc, &self.custom_secret);
    }

    #[inline(never)]
    fn digest_mid_sized(&self) -> u64 {
        let mut acc = self.acc.clone();
        xxh3_stateful_digest_internal(&mut acc, self.nb_stripes_acc, self.buffered_input(), self.processed_buffer(), &self.custom_secret);

        merge_accs(&mut acc, get_aligned_chunk_ref(&self.custom_secret.0, SECRET_MERGEACCS_START),
                    self.total_len.wrapping_mul(xxh64::PRIME_1))
    }

    #[inline(never)]
    fn digest_mid_sized_128(&self) -> u128 {
        let mut acc = self.acc.clone();
        xxh3_stateful_digest_internal(&mut acc, self.nb_stripes_acc, self.buffered_input(), self.processed_buffer(), &self.custom_secret);

        let low = merge_accs(&mut acc, get_aligned_chunk_ref(&self.custom_secret.0, SECRET_MERGEACCS_START), self.total_len.wrapping_mul(xxh64::PRIME_1));
        let high = merge_accs(&mut acc, get_aligned_chunk_ref(&self.custom_secret.0, self.custom_secret.0.len() - mem::size_of_val(&self.acc) - SECRET_MERGEACCS_START), !self.total_len.wrapping_mul(xxh64::PRIME_2));
        ((high as u128) << 64) | (low as u128)
    }

    ///Computes hash.
    pub fn digest(&self) -> u64 {
        //Separating digest mid sized allows us to inline this function, which benefits
        //code generation when hashing fixed size types and/or if the seed is known.
        if self.total_len > MID_SIZE_MAX as u64 {
            self.digest_mid_sized()
        } else if self.seed > 0 {
            //Technically we should not need to use it.
            //But in all actuality original xxh3 implementation uses default secret for input with size less or equal to MID_SIZE_MAX
            xxh3_64_internal(self.buffered_input(), self.seed, &DEFAULT_SECRET, xxh3_64_long_with_seed)
        } else {
            xxh3_64_internal(self.buffered_input(), self.seed, &self.custom_secret.0, xxh3_64_long_with_secret)
        }
    }

    ///Computes hash as 128bit integer.
    pub fn digest128(&self) -> u128 {
        //Separating digest mid sized allows us to inline this function, which benefits
        //code generation when hashing fixed size types and/or if the seed is known.
        if self.total_len > MID_SIZE_MAX as u64 {
            self.digest_mid_sized_128()
        } else if self.seed > 0 {
            //Technically we should not need to use it.
            //But in all actuality original xxh3 implementation uses default secret for input with size less or equal to MID_SIZE_MAX
            xxh3_128_internal(self.buffered_input(), self.seed, &DEFAULT_SECRET, xxh3_128_long_with_seed)
        } else {
            xxh3_128_internal(self.buffered_input(), self.seed, &self.custom_secret.0, xxh3_128_long_with_secret)
        }
    }
}

impl Default for Xxh3 {
    #[inline(always)]
    fn default() -> Self {
        Self::new()
    }
}

impl core::hash::Hasher for Xxh3 {
    #[inline(always)]
    fn finish(&self) -> u64 {
        self.digest()
    }

    #[inline(always)]
    fn write(&mut self, input: &[u8]) {
        self.update(input)
    }
}

#[cfg(feature = "std")]
impl std::io::Write for Xxh3 {
    #[inline]
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.update(buf);
        Ok(buf.len())
    }

    #[inline]
    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

#[derive(Clone, Copy)]
///Hash builder for `Xxh3`
pub struct Xxh3Builder {
    seed: u64,
    secret: [u8; DEFAULT_SECRET_SIZE],
}

impl Xxh3Builder {
    #[inline(always)]
    ///Creates new instance with default params.
    pub const fn new() -> Self {
        Self {
            seed: 0,
            secret: DEFAULT_SECRET,
        }
    }

    #[inline(always)]
    ///Sets `seed` for `xxh3` algorithm
    pub const fn with_seed(mut self, seed: u64) -> Self {
        self.seed = seed;
        self
    }

    #[inline(always)]
    ///Sets custom `secret` for `xxh3` algorithm
    pub const fn with_secret(mut self, secret: [u8; DEFAULT_SECRET_SIZE]) -> Self {
        self.secret = secret;
        self
    }

    #[inline(always)]
    ///Creates `Xxh3` instance
    pub const fn build(self) -> Xxh3 {
        Xxh3::with_custom_ops(self.seed, self.secret)
    }
}

impl core::hash::BuildHasher for Xxh3Builder {
    type Hasher = Xxh3;

    #[inline(always)]
    fn build_hasher(&self) -> Self::Hasher {
        self.build()
    }
}

impl Default for Xxh3Builder {
    #[inline(always)]
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Copy)]
///Hash builder for `Xxh3Default`
pub struct Xxh3DefaultBuilder;

impl Xxh3DefaultBuilder {
    #[inline(always)]
    ///Creates new instance with default params.
    pub const fn new() -> Self {
        Self
    }

    #[inline(always)]
    ///Creates `Xxh3` instance
    pub const fn build(self) -> Xxh3Default {
        Xxh3Default::new()
    }
}

impl core::hash::BuildHasher for Xxh3DefaultBuilder {
    type Hasher = Xxh3Default;

    #[inline(always)]
    fn build_hasher(&self) -> Self::Hasher {
        self.build()
    }
}

impl Default for Xxh3DefaultBuilder {
    #[inline(always)]
    fn default() -> Self {
        Self::new()
    }
}

//
//128bit
//

#[inline]
fn xxh3_128_long_impl(input: &[u8], secret: &[u8]) -> u128 {
    let mut acc = INITIAL_ACC;

    hash_long_internal_loop(&mut acc, input, secret);

    debug_assert!(secret.len() >= mem::size_of::<Acc>() + SECRET_MERGEACCS_START);
    let lo = merge_accs(&mut acc, get_aligned_chunk_ref(secret, SECRET_MERGEACCS_START), (input.len() as u64).wrapping_mul(xxh64::PRIME_1));
    let hi = merge_accs(&mut acc,
                        get_aligned_chunk_ref(secret, secret.len() - mem::size_of::<Acc>() - SECRET_MERGEACCS_START),
                        !(input.len() as u64).wrapping_mul(xxh64::PRIME_2));

    lo as u128 | (hi as u128) << 64
}

#[inline(always)]
fn xxh3_128_9to16(input: &[u8], seed: u64, secret: &[u8]) -> u128 {
    let flip_lo = (read_64le_unaligned(secret, 32) ^ read_64le_unaligned(secret, 40)).wrapping_sub(seed);
    let flip_hi = (read_64le_unaligned(secret, 48) ^ read_64le_unaligned(secret, 56)).wrapping_add(seed);
    let input_lo = read_64le_unaligned(input, 0);
    let mut input_hi = read_64le_unaligned(input, input.len() - 8);

    let (mut mul_low, mut mul_high) = mul64_to128(input_lo ^ input_hi ^ flip_lo, xxh64::PRIME_1);

    mul_low = mul_low.wrapping_add((input.len() as u64 - 1) << 54);
    input_hi ^= flip_hi;
    mul_high = mul_high.wrapping_add(
        input_hi.wrapping_add(mult32_to64(input_hi as u32, xxh32::PRIME_2 - 1))
    );

    mul_low ^= mul_high.swap_bytes();

    let (result_low, mut result_hi) = mul64_to128(mul_low, xxh64::PRIME_2);
    result_hi = result_hi.wrapping_add(
        mul_high.wrapping_mul(xxh64::PRIME_2)
    );

    to_u128!(avalanche(result_low), avalanche(result_hi))
}

#[inline(always)]
fn xxh3_128_4to8(input: &[u8], mut seed: u64, secret: &[u8]) -> u128 {
    seed ^= ((seed as u32).swap_bytes() as u64) << 32;

    let lo = read_32le_unaligned(input, 0);
    let hi = read_32le_unaligned(input, input.len() - 4);
    let input_64 = (lo as u64).wrapping_add((hi as u64) << 32);

    let flip = (read_64le_unaligned(secret, 16) ^ read_64le_unaligned(secret, 24)).wrapping_add(seed);
    let keyed = input_64 ^ flip;

    let (mut lo, mut hi) = mul64_to128(keyed, xxh64::PRIME_1.wrapping_add((input.len() as u64) << 2));

    hi = hi.wrapping_add(lo << 1);
    lo ^= hi >> 3;

    lo = xorshift64(lo, 35).wrapping_mul(0x9FB21C651E98DF25);
    lo = xorshift64(lo, 28);
    hi = avalanche(hi);

    lo as u128 | (hi as u128) << 64
}

#[inline(always)]
fn xxh3_128_1to3(input: &[u8], seed: u64, secret: &[u8]) -> u128 {
    let c1; let c2; let c3;
    unsafe {
        c1 = *input.get_unchecked(0);
        c2 = *input.get_unchecked(input.len() >> 1);
        c3 = *input.get_unchecked(input.len() - 1);
    };
    let input_lo = (c1 as u32) << 16 | (c2 as u32) << 24 | (c3 as u32) << 0 | (input.len() as u32) << 8;
    let input_hi = input_lo.swap_bytes().rotate_left(13);

    let flip_lo = (read_32le_unaligned(secret, 0) as u64 ^ read_32le_unaligned(secret, 4) as u64).wrapping_add(seed);
    let flip_hi = (read_32le_unaligned(secret, 8) as u64 ^ read_32le_unaligned(secret, 12) as u64).wrapping_sub(seed);
    let keyed_lo = input_lo as u64 ^ flip_lo;
    let keyed_hi = input_hi as u64 ^ flip_hi;

    xxh64::avalanche(keyed_lo) as u128 | (xxh64::avalanche(keyed_hi) as u128) << 64
}

#[inline(always)]
fn xxh3_128_0to16(input: &[u8], seed: u64, secret: &[u8]) -> u128 {
    if input.len() > 8 {
        xxh3_128_9to16(input, seed, secret)
    } else if input.len() >= 4 {
        xxh3_128_4to8(input, seed, secret)
    } else if input.len() > 0 {
        xxh3_128_1to3(input, seed, secret)
    } else {
        let flip_lo = read_64le_unaligned(secret, 64) ^ read_64le_unaligned(secret, 72);
        let flip_hi = read_64le_unaligned(secret, 80) ^ read_64le_unaligned(secret, 88);
        xxh64::avalanche(seed ^ flip_lo) as u128 | (xxh64::avalanche(seed ^ flip_hi) as u128) << 64
    }
}

#[inline(always)]
fn xxh3_128_7to128(input: &[u8], seed: u64, secret: &[u8]) -> u128 {
    let mut lo = (input.len() as u64).wrapping_mul(xxh64::PRIME_1);
    let mut hi = 0;

    if input.len() > 32 {
        if input.len() > 64 {
            if input.len() > 96 {

                mix32_b(&mut lo, &mut hi,
                    get_aligned_chunk_ref(input, 48),
                    get_aligned_chunk_ref(input, input.len() - 64),
                    get_aligned_chunk_ref(secret, 96),
                    seed
                );
            }

            mix32_b(&mut lo, &mut hi,
                get_aligned_chunk_ref(input, 32),
                get_aligned_chunk_ref(input, input.len() - 48),
                get_aligned_chunk_ref(secret, 64),
                seed
            );
        }

        mix32_b(&mut lo, &mut hi,
            get_aligned_chunk_ref(input, 16),
            get_aligned_chunk_ref(input, input.len() - 32),
            get_aligned_chunk_ref(secret, 32),
            seed
        );
    }

    mix32_b(&mut lo, &mut hi,
        get_aligned_chunk_ref(input, 0),
        get_aligned_chunk_ref(input, input.len() - 16),
        get_aligned_chunk_ref(secret, 0),
        seed
    );

    to_u128!(
        avalanche(
            lo.wrapping_add(hi)
        ),
        0u64.wrapping_sub(
            avalanche(
                lo.wrapping_mul(xxh64::PRIME_1)
                  .wrapping_add(hi.wrapping_mul(xxh64::PRIME_4))
                  .wrapping_add((input.len() as u64).wrapping_sub(seed).wrapping_mul(xxh64::PRIME_2))
            )
        )
    )
}

#[inline(never)]
fn xxh3_128_129to240(input: &[u8], seed: u64, secret: &[u8]) -> u128 {
    const START_OFFSET: usize = 3;
    const LAST_OFFSET: usize = 17;
    let nb_rounds = input.len() / 32;
    debug_assert!(nb_rounds >= 4);

    let mut lo = (input.len() as u64).wrapping_mul(xxh64::PRIME_1);
    let mut hi = 0;

    let mut idx = 0;
    while idx < 4 {
        let offset_idx = 32 * idx;
        mix32_b(&mut lo, &mut hi,
            get_aligned_chunk_ref(input, offset_idx),
            get_aligned_chunk_ref(input, offset_idx + 16),
            get_aligned_chunk_ref(secret, offset_idx),
            seed
        );
        idx = idx.wrapping_add(1);
    }

    lo = avalanche(lo);
    hi = avalanche(hi);

    while idx < nb_rounds {
        mix32_b(&mut lo, &mut hi,
            get_aligned_chunk_ref(input, 32 * idx),
            get_aligned_chunk_ref(input, (32 * idx) + 16),
            get_aligned_chunk_ref(secret, START_OFFSET.wrapping_add(32 * (idx - 4))),
            seed
        );
        idx = idx.wrapping_add(1);
    }

    mix32_b(&mut lo, &mut hi,
        get_aligned_chunk_ref(input, input.len() - 16),
        get_aligned_chunk_ref(input, input.len() - 32),
        get_aligned_chunk_ref(secret, SECRET_SIZE_MIN - LAST_OFFSET - 16),
        0u64.wrapping_sub(seed)
    );

    to_u128!(
        avalanche(
            lo.wrapping_add(hi)
        ),
        0u64.wrapping_sub(
            avalanche(
                lo.wrapping_mul(xxh64::PRIME_1)
                  .wrapping_add(hi.wrapping_mul(xxh64::PRIME_4))
                  .wrapping_add((input.len() as u64).wrapping_sub(seed).wrapping_mul(xxh64::PRIME_2))
            )
        )
    )
}

#[inline(always)]
fn xxh3_128_internal(input: &[u8], seed: u64, secret: &[u8], long_hash_fn: LongHashFn128) -> u128 {
    debug_assert!(secret.len() >= SECRET_SIZE_MIN);

    if input.len() <= 16 {
        xxh3_128_0to16(input, seed, secret)
    } else if input.len() <= 128 {
        xxh3_128_7to128(input, seed, secret)
    } else if input.len() <= MID_SIZE_MAX {
        xxh3_128_129to240(input, seed, secret)
    } else {
        long_hash_fn(input, seed, secret)
    }
}

#[inline(never)]
fn xxh3_128_long_default(input: &[u8], _seed: u64, _secret: &[u8]) -> u128 {
    xxh3_128_long_impl(input, &DEFAULT_SECRET)
}

#[inline(never)]
fn xxh3_128_long_with_seed(input: &[u8], seed: u64, _secret: &[u8]) -> u128 {
    match seed {
        0 => xxh3_128_long_impl(input, &DEFAULT_SECRET),
        seed => xxh3_128_long_impl(input, &custom_default_secret(seed)),
    }
}

#[inline(never)]
fn xxh3_128_long_with_secret(input: &[u8], _seed: u64, secret: &[u8]) -> u128 {
    xxh3_128_long_impl(input, secret)
}

#[inline]
///Returns 128bit hash for provided input.
pub fn xxh3_128(input: &[u8]) -> u128 {
    xxh3_128_internal(input, 0, &DEFAULT_SECRET, xxh3_128_long_default)
}

#[inline]
///Returns 128 hash for provided input using seed.
///
///Note: While overhead of deriving new secret from provided seed is low,
///it would more efficient to generate secret at compile time using special function
///`const_custom_default_secret` from `const_xxh3`
pub fn xxh3_128_with_seed(input: &[u8], seed: u64) -> u128 {
    xxh3_128_internal(input, seed, &DEFAULT_SECRET, xxh3_128_long_with_seed)
}

#[inline]
///Returns 128 hash for provided input using custom secret.
pub fn xxh3_128_with_secret(input: &[u8], secret: &[u8]) -> u128 {
    xxh3_128_internal(input, 0, secret, xxh3_128_long_with_secret)
}
