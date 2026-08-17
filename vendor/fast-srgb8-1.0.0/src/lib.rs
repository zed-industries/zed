//! Small crate implementing fast conversion between linear float and 8-bit
//! sRGB.
//!
//! - [`f32_to_srgb8`]: Convert f32 to an sRGB u8. Meets all the requirements of
//!   [the most relevent public
//!   spec](https://microsoft.github.io/DirectX-Specs/d3d/archive/D3D11_3_FunctionalSpec.htm#FLOATtoSRGB)
//!   which includes:
//!     - Maximum error of 0.6 ULP (on integer side) — Note that in practice
//!       this is a higher max error than the naive implementation will give
//!       you, so for applications like scientific or medical imaging, perhaps
//!       this is less acceptable. That said, for normal graphics work, this
//!       should be fine.
//!     - Monotonic across the 0.0..=1.0 range. (If `f32_to_srgb8(a) >
//!       f32_to_srgb8(b)`, then `a > b`)
//!     - All possible outputs are achievable (round-trips with
//!       [`srgb8_to_f32`]).
//!
//! - [`f32x4_to_srgb8`]: Produces results identical to calling [`f32_to_srgb8`]
//!   4 times in a row. On targets where we have a SIMD implementation
//!   (currently SSE2-enabled x86 and x86_64), this will use that. Otherwise, it
//!   will just call `f32_to_srgb8` four times in a row, and return the results.
//!
//! - [`srgb8_to_f32`]: Inverse operation of [`f32_to_srgb8`]. Uses the standard
//!   technique of a 256-item lookup table.
//!
//! ## Benefits
//! - Large performance improvments over the naive implementation (see
//!   [README.md](github.com/thomcc/fast-srgb8) for benchmarks)
//! - Supports `no_std` (normally this is tricky, as these operations require
//!   `powf` naively, which is not available to libcore)
//! - No dependencies.
//! - SIMD support for conversion to sRGB (conversion from sRGB is already ~20x
//!   faster than naive impl, and would probably be slower in SIMD, so for now
//!   it's not implemented).
//! - Consistent and correct (according to at least one relevant spec) handling
//!   of edge cases, such as NaN/Inf/etc.
//! - Exhaustive checking of all inputs for correctness (in tests).

#![cfg_attr(not(test), no_std)]
#![cfg_attr(all(test, unstable_bench), feature(test))]
#[cfg(all(test, unstable_bench))]
extern crate test;

#[cfg(all(
    not(miri),
    any(target_arch = "x86_64", target_arch = "x86"),
    target_feature = "sse2"
))]
mod sse2;

/// Converts linear f32 RGB component to an 8-bit sRGB value.
///
/// If you have to do this for many values simultaneously, use
/// [`f32x4_to_srgb8`], which will compute 4 results at once (using SIMD
/// instructions if available).
///
/// Input less than 0.0, or greater than 1.0, is clamped to be inside that
/// range. NaN input is treated as identical to 0.0.
///
/// # Details
///
/// Conceptually, this is an optimized (and slightly approximated — see the
/// "Approximation" section below) version of the following "reference
/// implementation", which more or less looks like:
///
/// ```
/// // Conceptually equivalent (but see below)
/// fn to_srgb_reference(f: f32) -> u8 {
///     let v = if !(f > 0.0) {
///         0.0
///     } else if f <= 0.0031308 {
///         12.92 * f
///     } else if f < 1.0 {
///         1.055 * f.powf(1.0 / 2.4) - 0.055
///     } else {
///         1.0
///     };
///     (v * 255.0 + 0.5) as u8
/// }
/// ```
///
/// This crate's implementation uses a small lookup table (a `[u32; 104]` --
/// around 6.5 cache lines), and avoids needing to call `powf` (which, as an
/// added bonus, means it works great in `no_std`), and in practice is many
/// times faster than the alternative.
///
/// Additional, it's fairly amenable to implementing in SIMD (— everything is
/// easily parallelized aside from the table lookup), and so a 4-wide
/// implementation is also provided as [`f32x4_to_srgb8`]
///
/// ## Approximation
/// Note that this is *not* bitwise identical to the results of the
/// `to_srgb_reference` function above, it's just very close. The maximum error
/// is 0.544403 for an input of 0.31152344, where error is computed as the
/// absolute difference between the rounded integer and the "exact" value.
///
/// This almost certainly meets requirements for graphics: [The DirectX
/// spec](https://microsoft.github.io/DirectX-Specs/d3d/archive/D3D11_3_FunctionalSpec.htm#FLOATtoSRGB)
/// mandates that compliant implementations of this function have a maximum
/// error of less than "0.6 ULP on the integer side" — Ours is ~0.54, which is
/// within the requirement.
///
/// This means function is probably at least as accurate as whatever your GPU
/// driver and/or hardware does for sRGB framebuffers and such — very likely
/// even if it isn't using DirectX (it's spec tends to be descriptive of what's
/// available commonly, especially in cases like this (most cases) where it's
/// the only one that bothers to put a requirement).
///
/// Additionally, because this function converts the result `u8` — for the vast
/// majority of inputs it will return an identical result to the reference impl.
///
/// To be completely clear (since it was brought up as a concern): despite this
/// approximation, this function and [`srgb8_to_f32`] are inverses of eachother,
/// and round trip appropriately.
#[inline]
pub fn f32_to_srgb8(f: f32) -> u8 {
    const MAXV_BITS: u32 = 0x3f7fffff; // 1.0 - f32::EPSILON
    const MINV_BITS: u32 = 0x39000000; // 2^(-13)
    let minv = f32::from_bits(MINV_BITS);
    let maxv = f32::from_bits(MAXV_BITS);
    // written like this to handle nans.
    let mut input = f;
    if !(input > minv) {
        input = minv;
    }
    if input > maxv {
        input = maxv;
    }
    let fu = input.to_bits();
    #[cfg(all(not(unstable_bench), test))]
    {
        debug_assert!(MINV_BITS <= fu && fu <= MAXV_BITS);
    }
    // Safety: all input floats are clamped into the {minv, maxv} range, which
    // turns out in this case to guarantee that their bitwise reprs are clamped
    // to the {MINV_BITS, MAXV_BITS} range (guaranteed by the fact that
    // minv/maxv are the normal, finite, the same sign, and not zero).
    //
    // Because of that, the smallest result of `fu - MINV_BITS` is 0 (when `fu`
    // is `MINV_BITS`), and the largest is `0x067fffff`, (when `fu` is
    // `MAXV_BITS`). `0x067fffff >> 20` is 0x67, e.g. 103, and thus all possible
    // results are inbounds for the (104 item) table. This is all verified in
    // test code.
    //
    // Note that the compiler can't figure this out on it's own, so the
    // get_unchecked does help some.
    let entry = unsafe {
        let i = ((fu - MINV_BITS) >> 20) as usize;
        #[cfg(all(not(unstable_bench), test))]
        {
            debug_assert!(TO_SRGB8_TABLE.get(i).is_some());
        }
        *TO_SRGB8_TABLE.get_unchecked(i)
    };
    // bottom 16 bits are bias, top 9 are scale.
    let bias = (entry >> 16) << 9;
    let scale = entry & 0xffff;

    // lerp to the next highest mantissa bits.
    let t = (fu >> 12) & 0xff;
    let res = (bias + scale * t) >> 16;
    #[cfg(all(not(unstable_bench), test))]
    {
        debug_assert!(res < 256, "{}", res);
    }
    res as u8
}

/// Performs 4 simultaneous calls to [`f32_to_srgb8`], and returns 4 results.
///
/// If available, this uses SIMD to perform all 4 computations simultaneously —
/// currently this is just on x86_64 and x86 targets that suppost SSE2 (which in
/// practice will be all x86_64 (aside from weird things like OS kernels), and
/// all Rust targets beginning with `i686-`). On machines where it cannot use
/// the CPU's vector instructions, this function simply performs 4 calls to
/// [`f32_to_srgb8`].
///
/// The check for this support is performed at compile time, so it does no
/// runtime SIMD feature checks. This seems like the right call for SSE2.
///
/// Behavior is otherwise exactly (bitwise) identical to [`f32_to_srgb8`], so see
/// it's documentation for more information.
#[inline]
pub fn f32x4_to_srgb8(input: [f32; 4]) -> [u8; 4] {
    #[cfg(all(
        not(miri),
        any(target_arch = "x86_64", target_arch = "x86"),
        target_feature = "sse2"
    ))]
    unsafe {
        // Safety: we've checked that we're on x86/x86_64 and have SSE2
        crate::sse2::simd_to_srgb8(input)
    }
    #[cfg(not(all(
        not(miri),
        any(target_arch = "x86_64", target_arch = "x86"),
        target_feature = "sse2"
    )))]
    {
        [
            f32_to_srgb8(input[0]),
            f32_to_srgb8(input[1]),
            f32_to_srgb8(input[2]),
            f32_to_srgb8(input[3]),
        ]
    }
}

const TO_SRGB8_TABLE: [u32; 104] = [
    0x0073000d, 0x007a000d, 0x0080000d, 0x0087000d, 0x008d000d, 0x0094000d, 0x009a000d, 0x00a1000d,
    0x00a7001a, 0x00b4001a, 0x00c1001a, 0x00ce001a, 0x00da001a, 0x00e7001a, 0x00f4001a, 0x0101001a,
    0x010e0033, 0x01280033, 0x01410033, 0x015b0033, 0x01750033, 0x018f0033, 0x01a80033, 0x01c20033,
    0x01dc0067, 0x020f0067, 0x02430067, 0x02760067, 0x02aa0067, 0x02dd0067, 0x03110067, 0x03440067,
    0x037800ce, 0x03df00ce, 0x044600ce, 0x04ad00ce, 0x051400ce, 0x057b00c5, 0x05dd00bc, 0x063b00b5,
    0x06970158, 0x07420142, 0x07e30130, 0x087b0120, 0x090b0112, 0x09940106, 0x0a1700fc, 0x0a9500f2,
    0x0b0f01cb, 0x0bf401ae, 0x0ccb0195, 0x0d950180, 0x0e56016e, 0x0f0d015e, 0x0fbc0150, 0x10630143,
    0x11070264, 0x1238023e, 0x1357021d, 0x14660201, 0x156601e9, 0x165a01d3, 0x174401c0, 0x182401af,
    0x18fe0331, 0x1a9602fe, 0x1c1502d2, 0x1d7e02ad, 0x1ed4028d, 0x201a0270, 0x21520256, 0x227d0240,
    0x239f0443, 0x25c003fe, 0x27bf03c4, 0x29a10392, 0x2b6a0367, 0x2d1d0341, 0x2ebe031f, 0x304d0300,
    0x31d105b0, 0x34a80555, 0x37520507, 0x39d504c5, 0x3c37048b, 0x3e7c0458, 0x40a8042a, 0x42bd0401,
    0x44c20798, 0x488e071e, 0x4c1c06b6, 0x4f76065d, 0x52a50610, 0x55ac05cc, 0x5892058f, 0x5b590559,
    0x5e0c0a23, 0x631c0980, 0x67db08f6, 0x6c55087f, 0x70940818, 0x74a007bd, 0x787d076c, 0x7c330723,
];

/// Convert from a 8-bit sRGB component to a linear f32.
///
/// This is the inverse of [`srgb8_to_f32`] — and `c: u8` is roundtripped
/// through it, as shown below:
/// ```
/// use fast_srgb8::{f32_to_srgb8, srgb8_to_f32};
/// for c in 0..=255u8 {
///     // f32_to_srgb8(srgb8_to_f32(c)) is an identity operation
///     assert_eq!(f32_to_srgb8(srgb8_to_f32(c)), c);
/// }
/// ```
///
/// The implementation of this function isn't particularly clever — it just uses
/// a precomputed lookup table of all 256 results. That has a benefit in that it
/// allows this function to be a const fn, which is somewhat nice: generally
/// color constants hardcoded in source code are sRGB, and this means you can
/// use them to produce linear constants.
///
/// In practice this is way faster than the naive approach, and I'm unaware of
/// any faster ways of implementing it, but it's not really amenable to SIMD, so
/// no SIMD version is provided.
#[inline]
pub const fn srgb8_to_f32(c: u8) -> f32 {
    FROM_SRGB8_TABLE[c as usize]
}

#[rustfmt::skip]
const FROM_SRGB8_TABLE: [f32; 256] = [
    0.0, 0.000303527, 0.000607054, 0.00091058103, 0.001214108, 0.001517635, 0.0018211621, 0.002124689,
    0.002428216, 0.002731743, 0.00303527, 0.0033465356, 0.003676507, 0.004024717, 0.004391442,
    0.0047769533, 0.005181517, 0.0056053917, 0.0060488326, 0.006512091, 0.00699541, 0.0074990317,
    0.008023192, 0.008568125, 0.009134057, 0.009721218, 0.010329823, 0.010960094, 0.011612245,
    0.012286487, 0.012983031, 0.013702081, 0.014443844, 0.015208514, 0.015996292, 0.016807375,
    0.017641952, 0.018500218, 0.019382361, 0.020288562, 0.02121901, 0.022173883, 0.023153365,
    0.02415763, 0.025186857, 0.026241222, 0.027320892, 0.028426038, 0.029556843, 0.03071345, 0.03189604,
    0.033104774, 0.03433981, 0.035601325, 0.036889452, 0.038204376, 0.039546248, 0.04091521, 0.042311423,
    0.043735042, 0.045186214, 0.046665095, 0.048171833, 0.049706575, 0.051269468, 0.052860655, 0.05448028,
    0.056128494, 0.057805434, 0.05951124, 0.06124607, 0.06301003, 0.06480328, 0.06662595, 0.06847818,
    0.07036011, 0.07227186, 0.07421358, 0.07618539, 0.07818743, 0.08021983, 0.082282715, 0.084376216,
    0.086500466, 0.088655606, 0.09084173, 0.09305898, 0.095307484, 0.09758736, 0.09989874, 0.10224175,
    0.10461649, 0.10702311, 0.10946172, 0.111932434, 0.11443538, 0.116970696, 0.11953845, 0.12213881,
    0.12477186, 0.12743773, 0.13013652, 0.13286836, 0.13563336, 0.13843165, 0.14126332, 0.1441285,
    0.1470273, 0.14995982, 0.15292618, 0.1559265, 0.15896086, 0.16202943, 0.16513224, 0.16826946,
    0.17144115, 0.17464745, 0.17788847, 0.1811643, 0.18447503, 0.1878208, 0.19120172, 0.19461787,
    0.19806935, 0.2015563, 0.20507877, 0.2086369, 0.21223079, 0.21586053, 0.21952623, 0.22322798,
    0.22696589, 0.23074007, 0.23455065, 0.23839766, 0.2422812, 0.2462014, 0.25015837, 0.25415218,
    0.2581829, 0.26225072, 0.26635566, 0.27049786, 0.27467737, 0.27889434, 0.2831488, 0.2874409,
    0.2917707, 0.29613832, 0.30054384, 0.30498737, 0.30946895, 0.31398875, 0.31854683, 0.32314324,
    0.32777813, 0.33245158, 0.33716366, 0.34191445, 0.3467041, 0.3515327, 0.35640025, 0.36130688,
    0.3662527, 0.37123778, 0.37626222, 0.3813261, 0.38642952, 0.39157256, 0.3967553, 0.40197787,
    0.4072403, 0.4125427, 0.41788515, 0.42326775, 0.42869055, 0.4341537, 0.43965724, 0.44520125,
    0.45078585, 0.45641106, 0.46207705, 0.46778384, 0.47353154, 0.47932023, 0.48514998, 0.4910209,
    0.49693304, 0.5028866, 0.50888145, 0.5149178, 0.5209957, 0.52711535, 0.5332766, 0.5394797,
    0.5457247, 0.5520116, 0.5583406, 0.5647117, 0.57112503, 0.57758063, 0.5840786, 0.590619, 0.597202,
    0.60382754, 0.61049575, 0.61720675, 0.62396055, 0.63075733, 0.637597, 0.6444799, 0.6514058,
    0.65837497, 0.66538745, 0.67244333, 0.6795426, 0.68668544, 0.69387203, 0.70110214, 0.70837605,
    0.7156938, 0.72305536, 0.730461, 0.7379107, 0.7454045, 0.75294244, 0.76052475, 0.7681514, 0.77582246,
    0.78353804, 0.79129815, 0.79910296, 0.8069525, 0.8148468, 0.822786, 0.8307701, 0.83879924, 0.84687346,
    0.8549928, 0.8631574, 0.87136734, 0.8796226, 0.8879232, 0.89626956, 0.90466136, 0.913099, 0.92158204,
    0.93011117, 0.9386859, 0.9473069, 0.9559735, 0.9646866, 0.9734455, 0.98225087, 0.9911022, 1.0
];

#[cfg(test)]
mod tests {
    use super::*;
    fn srgb8_to_f32_ref(c: u8) -> f32 {
        let c = c as f32 * (1.0 / 255.0);
        if c <= 0.04045 {
            c / 12.92
        } else {
            ((c + 0.055) / 1.055).powf(2.4)
        }
    }
    #[test]
    fn test_from_srgb8() {
        let wanted = (0..=255).map(srgb8_to_f32_ref).collect::<Vec<_>>();
        assert_eq!(&FROM_SRGB8_TABLE[..], &wanted[..]);
        for i in 0..=255u8 {
            assert_eq!(srgb8_to_f32(i), srgb8_to_f32_ref(i));
            assert_eq!(f32_to_srgb8(srgb8_to_f32(i)), i, "{}", i);
        }
    }

    // run as `cargo test --release -- --nocapture --ignored`
    #[test]
    #[ignore]
    fn test_exhaustive_scalar() {
        // Simultaneously test that:
        // - monotonicity is respected
        // - error < 0.6f ULP on int side
        // - SIMD and Scalar return identical values
        let mut prev = 0;
        for i in 0..=!0u32 {
            // offset by the first NaN so that we iterate in a way that makes monotonicity easy to check.
            let f = f32::from_bits(i.wrapping_add((255 << 23) + 1));
            let c = f32_to_srgb8(f);
            let reference = unrounded_f32_to_srgb_ref(f);
            let err = (c as f32 - reference).abs();
            assert!(
                err < 0.6,
                "Error exceeds limit, {} >= 0.6 at {:?} (0x{:08x})",
                err,
                f,
                f.to_bits(),
            );
            assert!(
                c >= prev,
                "Monotonicity not respected {} < {} at  {:?} (0x{:08x})",
                c,
                prev,
                f,
                f.to_bits(),
            );
            prev = c;
            let v = f32x4_to_srgb8([f, f, f, f]);
            assert_eq!([c, c, c, c], v);
            if (i & 0xffffff) == 0 {
                println!("scalar: {}", i >> 24);
            }
        }
    }
    #[test]
    #[ignore]
    fn test_exhaustive_simd() {
        // verifies exactly identical results for all inputs.
        let mut i = 0;
        loop {
            let f0 = f32::from_bits(i);
            let f1 = f32::from_bits(i + 1);
            let f2 = f32::from_bits(i + 2);
            let f3 = f32::from_bits(i + 3);
            let v = f32x4_to_srgb8([f0, f1, f2, f3]);
            let c0 = f32_to_srgb8(f0);
            let c1 = f32_to_srgb8(f1);
            let c2 = f32_to_srgb8(f2);
            let c3 = f32_to_srgb8(f3);
            assert_eq!(
                v,
                [c0, c1, c2, c3],
                "simd/scalar mismatch at {:?} (starting at 0x{:08x})",
                [f0, f1, f2, f3],
                i,
            );
            if (i & 0xffffff) == 0 {
                println!("simd: {}", i >> 24);
            }
            i = i.wrapping_add(4);
            if i == 0 {
                break;
            }
        }
    }

    fn unrounded_f32_to_srgb_ref(f: f32) -> f32 {
        let v = if !(f > 0.0) {
            0.0
        } else if f <= 0.0031308 {
            12.92 * f
        } else if f < 1.0 {
            1.055 * f.powf(1.0 / 2.4) - 0.055
        } else {
            1.0
        };
        v * 255.0
    }

    #[cfg(unstable_bench)]
    mod bench {
        use super::*;
        fn f32_to_srgb_ref(f: f32) -> u8 {
            (unrounded_f32_to_srgb_ref(f) + 0.5) as u8
        }
        const BENCH_SUBDIV: usize = 50;
        #[bench]
        fn fast_scalar(b: &mut test::Bencher) {
            b.iter(|| {
                for i in 0..=BENCH_SUBDIV {
                    test::black_box(f32_to_srgb8(i as f32 / BENCH_SUBDIV as f32));
                }
            });
        }
        #[bench]
        fn naive_scalar(b: &mut test::Bencher) {
            b.iter(|| {
                for i in 0..=BENCH_SUBDIV {
                    test::black_box(f32_to_srgb_ref(i as f32 / BENCH_SUBDIV as f32));
                }
            });
        }
        #[bench]
        fn naive_f32x4(b: &mut test::Bencher) {
            b.iter(|| {
                for i in 0..=BENCH_SUBDIV {
                    let a = f32_to_srgb_ref(i as f32 / BENCH_SUBDIV as f32);
                    let b = f32_to_srgb_ref(i as f32 / BENCH_SUBDIV as f32 + 0.025);
                    let c = f32_to_srgb_ref(i as f32 / BENCH_SUBDIV as f32 + 0.05);
                    let d = f32_to_srgb_ref(i as f32 / BENCH_SUBDIV as f32 + 0.075);
                    test::black_box([a, b, c, d]);
                }
            });
        }
        #[bench]
        fn fast_f32x4(b: &mut test::Bencher) {
            b.iter(|| {
                for i in 0..=BENCH_SUBDIV {
                    let v = f32x4_to_srgb8([
                        i as f32 / BENCH_SUBDIV as f32,
                        i as f32 / BENCH_SUBDIV as f32 + 0.025,
                        i as f32 / BENCH_SUBDIV as f32 + 0.05,
                        i as f32 / BENCH_SUBDIV as f32 + 0.075,
                    ]);
                    test::black_box(v);
                }
            });
        }
        #[bench]
        fn fast_f32x4_nosimd(b: &mut test::Bencher) {
            b.iter(|| {
                for i in 0..=BENCH_SUBDIV {
                    let a = f32_to_srgb8(i as f32 / BENCH_SUBDIV as f32);
                    let b = f32_to_srgb8(i as f32 / BENCH_SUBDIV as f32 + 0.025);
                    let c = f32_to_srgb8(i as f32 / BENCH_SUBDIV as f32 + 0.05);
                    let d = f32_to_srgb8(i as f32 / BENCH_SUBDIV as f32 + 0.075);
                    test::black_box([a, b, c, d]);
                }
            });
        }

        #[bench]
        fn naive_from_srgb8(b: &mut test::Bencher) {
            b.iter(|| {
                for i in 0..=255 {
                    test::black_box(srgb8_to_f32_ref(i));
                }
            });
        }
        #[bench]
        fn fast_from_srgb8(b: &mut test::Bencher) {
            b.iter(|| {
                for i in 0..=255 {
                    test::black_box(srgb8_to_f32(i));
                }
            });
        }
    }
}
