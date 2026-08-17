use core::convert::TryInto;

use crate::common::BytesPerPixel;

/// SIMD helpers for `fn unfilter`
///
/// TODO(https://github.com/rust-lang/rust/issues/86656): Stop gating this module behind the
/// "unstable" feature of the `png` crate.  This should be possible once the "portable_simd"
/// feature of Rust gets stabilized.
///
/// This is only known to help on x86, with no change measured on most benchmarks on ARM,
/// and even severely regressing some of them.
/// So despite the code being portable, we only enable this for x86.
/// We can add more platforms once this code is proven to be beneficial for them.
#[cfg(all(feature = "unstable", target_arch = "x86_64"))]
mod simd {
    use std::simd::num::{SimdInt, SimdUint};
    use std::simd::{u8x4, u8x8, LaneCount, Simd, SimdElement, SupportedLaneCount};

    /// Scalar Paeth function wrapped in SIMD scaffolding.
    ///
    /// This is needed because simply running the function on the inputs
    /// makes the compiler think our inputs are too short
    /// to benefit from vectorization.
    /// Putting it in SIMD scaffolding fixes that.
    /// https://github.com/image-rs/image-png/issues/511
    ///
    /// Funnily, the autovectorizer does a better job here
    /// than a handwritten algorithm using std::simd!
    /// We used to have a handwritten one but this is just faster.
    fn paeth_predictor<const N: usize>(
        a: Simd<i16, N>,
        b: Simd<i16, N>,
        c: Simd<i16, N>,
    ) -> Simd<i16, N>
    where
        LaneCount<N>: SupportedLaneCount,
    {
        let mut out = [0; N];
        for i in 0..N {
            out[i] = super::filter_paeth_stbi_i16(a[i].into(), b[i].into(), c[i].into());
        }
        out.into()
    }

    /// Functionally equivalent to `simd::paeth_predictor` but does not temporarily convert
    /// the SIMD elements to `i16`.
    fn paeth_predictor_u8<const N: usize>(
        a: Simd<u8, N>,
        b: Simd<u8, N>,
        c: Simd<u8, N>,
    ) -> Simd<u8, N>
    where
        LaneCount<N>: SupportedLaneCount,
    {
        let mut out = [0; N];
        for i in 0..N {
            out[i] = super::filter_paeth_stbi(a[i].into(), b[i].into(), c[i].into());
        }
        out.into()
    }

    /// Memory of previous pixels (as needed to unfilter `FilterType::Paeth`).
    /// See also https://www.w3.org/TR/png/#filter-byte-positions
    #[derive(Default)]
    struct PaethState<T, const N: usize>
    where
        T: SimdElement,
        LaneCount<N>: SupportedLaneCount,
    {
        /// Previous pixel in the previous row.
        c: Simd<T, N>,

        /// Previous pixel in the current row.
        a: Simd<T, N>,
    }

    /// Mutates `x` as needed to unfilter `FilterType::Paeth`.
    ///
    /// `b` is the current pixel in the previous row.  `x` is the current pixel in the current row.
    /// See also https://www.w3.org/TR/png/#filter-byte-positions
    fn paeth_step<const N: usize>(
        state: &mut PaethState<i16, N>,
        b: Simd<u8, N>,
        x: &mut Simd<u8, N>,
    ) where
        LaneCount<N>: SupportedLaneCount,
    {
        // Storing the inputs.
        let b = b.cast::<i16>();

        // Calculating the new value of the current pixel.
        let predictor = paeth_predictor(state.a, b, state.c);
        *x += predictor.cast::<u8>();

        // Preparing for the next step.
        state.c = b;
        state.a = x.cast::<i16>();
    }

    /// Computes the Paeth predictor without converting `u8` to `i16`.
    ///
    /// See `simd::paeth_step`.
    fn paeth_step_u8<const N: usize>(
        state: &mut PaethState<u8, N>,
        b: Simd<u8, N>,
        x: &mut Simd<u8, N>,
    ) where
        LaneCount<N>: SupportedLaneCount,
    {
        // Calculating the new value of the current pixel.
        *x += paeth_predictor_u8(state.a, b, state.c);

        // Preparing for the next step.
        state.c = b;
        state.a = *x;
    }

    fn load3(src: &[u8]) -> u8x4 {
        u8x4::from_array([src[0], src[1], src[2], 0])
    }

    fn store3(src: u8x4, dest: &mut [u8]) {
        dest[0..3].copy_from_slice(&src.to_array()[0..3])
    }

    /// Undoes `FilterType::Paeth` for `BytesPerPixel::Three`.
    pub fn unfilter_paeth3(mut prev_row: &[u8], mut curr_row: &mut [u8]) {
        debug_assert_eq!(prev_row.len(), curr_row.len());
        debug_assert_eq!(prev_row.len() % 3, 0);

        let mut state = PaethState::<i16, 4>::default();
        while prev_row.len() >= 4 {
            // `u8x4` requires working with `[u8;4]`, but we can just load and ignore the first
            // byte from the next triple.  This optimization technique mimics the algorithm found
            // in
            // https://github.com/glennrp/libpng/blob/f8e5fa92b0e37ab597616f554bee254157998227/intel/filter_sse2_intrinsics.c#L130-L131
            let b = u8x4::from_slice(prev_row);
            let mut x = u8x4::from_slice(curr_row);

            paeth_step(&mut state, b, &mut x);

            // We can speculate that writing 4 bytes might be more efficient (just as with using
            // `u8x4::from_slice` above), but we can't use that here, because we can't clobber the
            // first byte of the next pixel in the `curr_row`.
            store3(x, curr_row);

            prev_row = &prev_row[3..];
            curr_row = &mut curr_row[3..];
        }
        // Can't use `u8x4::from_slice` for the last `[u8;3]`.
        let b = load3(prev_row);
        let mut x = load3(curr_row);
        paeth_step(&mut state, b, &mut x);
        store3(x, curr_row);
    }

    /// Undoes `FilterType::Paeth` for `BytesPerPixel::Four` and `BytesPerPixel::Eight`.
    ///
    /// This function calculates the Paeth predictor entirely in `Simd<u8, N>`
    /// without converting to an intermediate `Simd<i16, N>`. Doing so avoids
    /// paying a small performance penalty converting between types.
    pub fn unfilter_paeth_u8<const N: usize>(prev_row: &[u8], curr_row: &mut [u8])
    where
        LaneCount<N>: SupportedLaneCount,
    {
        debug_assert_eq!(prev_row.len(), curr_row.len());
        debug_assert_eq!(prev_row.len() % N, 0);
        assert!(matches!(N, 4 | 8));

        let mut state = PaethState::<u8, N>::default();
        for (prev_row, curr_row) in prev_row.chunks_exact(N).zip(curr_row.chunks_exact_mut(N)) {
            let b = Simd::from_slice(prev_row);
            let mut x = Simd::from_slice(curr_row);

            paeth_step_u8(&mut state, b, &mut x);

            curr_row[..N].copy_from_slice(&x.to_array()[..N]);
        }
    }

    fn load6(src: &[u8]) -> u8x8 {
        u8x8::from_array([src[0], src[1], src[2], src[3], src[4], src[5], 0, 0])
    }

    fn store6(src: u8x8, dest: &mut [u8]) {
        dest[0..6].copy_from_slice(&src.to_array()[0..6])
    }

    /// Undoes `FilterType::Paeth` for `BytesPerPixel::Six`.
    pub fn unfilter_paeth6(mut prev_row: &[u8], mut curr_row: &mut [u8]) {
        debug_assert_eq!(prev_row.len(), curr_row.len());
        debug_assert_eq!(prev_row.len() % 6, 0);

        let mut state = PaethState::<i16, 8>::default();
        while prev_row.len() >= 8 {
            // `u8x8` requires working with `[u8;8]`, but we can just load and ignore the first two
            // bytes from the next pixel.  This optimization technique mimics the algorithm found
            // in
            // https://github.com/glennrp/libpng/blob/f8e5fa92b0e37ab597616f554bee254157998227/intel/filter_sse2_intrinsics.c#L130-L131
            let b = u8x8::from_slice(prev_row);
            let mut x = u8x8::from_slice(curr_row);

            paeth_step(&mut state, b, &mut x);

            // We can speculate that writing 8 bytes might be more efficient (just as with using
            // `u8x8::from_slice` above), but we can't use that here, because we can't clobber the
            // first bytes of the next pixel in the `curr_row`.
            store6(x, curr_row);

            prev_row = &prev_row[6..];
            curr_row = &mut curr_row[6..];
        }
        // Can't use `u8x8::from_slice` for the last `[u8;6]`.
        let b = load6(prev_row);
        let mut x = load6(curr_row);
        paeth_step(&mut state, b, &mut x);
        store6(x, curr_row);
    }
}

/// The byte level filter applied to scanlines to prepare them for compression.
///
/// Compression in general benefits from repetitive data. The filter is a content-aware method of
/// compressing the range of occurring byte values to help the compression algorithm. Note that
/// this does not operate on pixels but on raw bytes of a scanline.
///
/// Details on how each filter works can be found in the [PNG Book](http://www.libpng.org/pub/png/book/chapter09.html).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum FilterType {
    NoFilter = 0,
    Sub = 1,
    Up = 2,
    Avg = 3,
    Paeth = 4,
}

impl Default for FilterType {
    fn default() -> Self {
        FilterType::Sub
    }
}

impl FilterType {
    /// u8 -> Self. Temporary solution until Rust provides a canonical one.
    pub fn from_u8(n: u8) -> Option<FilterType> {
        match n {
            0 => Some(FilterType::NoFilter),
            1 => Some(FilterType::Sub),
            2 => Some(FilterType::Up),
            3 => Some(FilterType::Avg),
            4 => Some(FilterType::Paeth),
            _ => None,
        }
    }
}

/// Adaptive filtering tries every possible filter for each row and uses a heuristic to select the best one.
/// This improves compression ratio, but makes encoding slightly slower.
///
/// It is recommended to use `Adaptive` whenever you care about compression ratio.
/// Filtering is quite cheap compared to other parts of encoding, but can contribute
/// to the compression ratio significantly.
///
/// `NonAdaptive` filtering is the default.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum AdaptiveFilterType {
    Adaptive,
    NonAdaptive,
}

impl Default for AdaptiveFilterType {
    fn default() -> Self {
        AdaptiveFilterType::NonAdaptive
    }
}

fn filter_paeth(a: u8, b: u8, c: u8) -> u8 {
    // On ARM this algorithm performs much better than the one above adapted from stb,
    // and this is the better-studied algorithm we've always used here,
    // so we default to it on all non-x86 platforms.
    let pa = (i16::from(b) - i16::from(c)).abs();
    let pb = (i16::from(a) - i16::from(c)).abs();
    let pc = ((i16::from(a) - i16::from(c)) + (i16::from(b) - i16::from(c))).abs();

    let mut out = a;
    let mut min = pa;

    if pb < min {
        min = pb;
        out = b;
    }
    if pc < min {
        out = c;
    }

    out
}

fn filter_paeth_stbi(a: u8, b: u8, c: u8) -> u8 {
    // Decoding optimizes better with this algorithm than with `filter_paeth`
    //
    // This formulation looks very different from the reference in the PNG spec, but is
    // actually equivalent and has favorable data dependencies and admits straightforward
    // generation of branch-free code, which helps performance significantly.
    //
    // Adapted from public domain PNG implementation:
    // https://github.com/nothings/stb/blob/5c205738c191bcb0abc65c4febfa9bd25ff35234/stb_image.h#L4657-L4668
    let thresh = i16::from(c) * 3 - (i16::from(a) + i16::from(b));
    let lo = a.min(b);
    let hi = a.max(b);
    let t0 = if hi as i16 <= thresh { lo } else { c };
    let t1 = if thresh <= lo as i16 { hi } else { t0 };
    return t1;
}

#[cfg(any(test, all(feature = "unstable", target_arch = "x86_64")))]
fn filter_paeth_stbi_i16(a: i16, b: i16, c: i16) -> i16 {
    // Like `filter_paeth_stbi` but vectorizes better when wrapped in SIMD types.
    // Used for bpp=3 and bpp=6
    let thresh = c * 3 - (a + b);
    let lo = a.min(b);
    let hi = a.max(b);
    let t0 = if hi <= thresh { lo } else { c };
    let t1 = if thresh <= lo { hi } else { t0 };
    return t1;
}

fn filter_paeth_fpnge(a: u8, b: u8, c: u8) -> u8 {
    // This is an optimized version of the paeth filter from the PNG specification, proposed by
    // Luca Versari for [FPNGE](https://www.lucaversari.it/FJXL_and_FPNGE.pdf). It operates
    // entirely on unsigned 8-bit quantities, making it more conducive to vectorization.
    //
    //     p = a + b - c
    //     pa = |p - a| = |a + b - c - a| = |b - c| = max(b, c) - min(b, c)
    //     pb = |p - b| = |a + b - c - b| = |a - c| = max(a, c) - min(a, c)
    //     pc = |p - c| = |a + b - c - c| = |(b - c) + (a - c)| = ...
    //
    // Further optimizing the calculation of `pc` a bit tricker. However, notice that:
    //
    //        a > c && b > c
    //    ==> (a - c) > 0 && (b - c) > 0
    //    ==> pc > (a - c) && pc > (b - c)
    //    ==> pc > |a - c| && pc > |b - c|
    //    ==> pc > pb && pc > pa
    //
    // Meaning that if `c` is smaller than `a` and `b`, the value of `pc` is irrelevant. Similar
    // reasoning applies if `c` is larger than the other two inputs. Assuming that `c >= b` and
    // `c <= b` or vice versa:
    //
    //     pc = ||b - c| - |a - c|| =  |pa - pb| = max(pa, pb) - min(pa, pb)
    //
    let pa = b.max(c) - c.min(b);
    let pb = a.max(c) - c.min(a);
    let pc = if (a < c) == (c < b) {
        pa.max(pb) - pa.min(pb)
    } else {
        255
    };

    if pa <= pb && pa <= pc {
        a
    } else if pb <= pc {
        b
    } else {
        c
    }
}

pub(crate) fn unfilter(
    mut filter: FilterType,
    tbpp: BytesPerPixel,
    previous: &[u8],
    current: &mut [u8],
) {
    use self::FilterType::*;

    // If the previous row is empty, then treat it as if it were filled with zeros.
    if previous.is_empty() {
        if filter == Paeth {
            filter = Sub;
        } else if filter == Up {
            filter = NoFilter;
        }
    }

    // [2023/01 @okaneco] - Notes on optimizing decoding filters
    //
    // Links:
    // [PR]: https://github.com/image-rs/image-png/pull/382
    // [SWAR]: http://aggregate.org/SWAR/over.html
    // [AVG]: http://aggregate.org/MAGIC/#Average%20of%20Integers
    //
    // #382 heavily refactored and optimized the following filters making the
    // implementation nonobvious. These comments function as a summary of that
    // PR with an explanation of the choices made below.
    //
    // #382 originally started with trying to optimize using a technique called
    // SWAR, SIMD Within a Register. SWAR uses regular integer types like `u32`
    // and `u64` as SIMD registers to perform vertical operations in parallel,
    // usually involving bit-twiddling. This allowed each `BytesPerPixel` (bpp)
    // pixel to be decoded in parallel: 3bpp and 4bpp in a `u32`, 6bpp and 8pp
    // in a `u64`. The `Sub` filter looked like the following code block, `Avg`
    // was similar but used a bitwise average method from [AVG]:
    // ```
    // // See "Unpartitioned Operations With Correction Code" from [SWAR]
    // fn swar_add_u32(x: u32, y: u32) -> u32 {
    //     // 7-bit addition so there's no carry over the most significant bit
    //     let n = (x & 0x7f7f7f7f) + (y & 0x7f7f7f7f); // 0x7F = 0b_0111_1111
    //     // 1-bit parity/XOR addition to fill in the missing MSB
    //     n ^ (x ^ y) & 0x80808080                     // 0x80 = 0b_1000_0000
    // }
    //
    // let mut prev =
    //     u32::from_ne_bytes([current[0], current[1], current[2], current[3]]);
    // for chunk in current[4..].chunks_exact_mut(4) {
    //     let cur = u32::from_ne_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]);
    //     let new_chunk = swar_add_u32(cur, prev);
    //     chunk.copy_from_slice(&new_chunk.to_ne_bytes());
    //     prev = new_chunk;
    // }
    // ```
    // While this provided a measurable increase, @fintelia found that this idea
    // could be taken even further by unrolling the chunks component-wise and
    // avoiding unnecessary byte-shuffling by using byte arrays instead of
    // `u32::from|to_ne_bytes`. The bitwise operations were no longer necessary
    // so they were reverted to their obvious arithmetic equivalent. Lastly,
    // `TryInto` was used instead of `copy_from_slice`. The `Sub` code now
    // looked like this (with asserts to remove `0..bpp` bounds checks):
    // ```
    // assert!(len > 3);
    // let mut prev = [current[0], current[1], current[2], current[3]];
    // for chunk in current[4..].chunks_exact_mut(4) {
    //     let new_chunk = [
    //         chunk[0].wrapping_add(prev[0]),
    //         chunk[1].wrapping_add(prev[1]),
    //         chunk[2].wrapping_add(prev[2]),
    //         chunk[3].wrapping_add(prev[3]),
    //     ];
    //     *TryInto::<&mut [u8; 4]>::try_into(chunk).unwrap() = new_chunk;
    //     prev = new_chunk;
    // }
    // ```
    // The compiler was able to optimize the code to be even faster and this
    // method even sped up Paeth filtering! Assertions were experimentally
    // added within loop bodies which produced better instructions but no
    // difference in speed. Finally, the code was refactored to remove manual
    // slicing and start the previous pixel chunks with arrays of `[0; N]`.
    // ```
    // let mut prev = [0; 4];
    // for chunk in current.chunks_exact_mut(4) {
    //     let new_chunk = [
    //         chunk[0].wrapping_add(prev[0]),
    //         chunk[1].wrapping_add(prev[1]),
    //         chunk[2].wrapping_add(prev[2]),
    //         chunk[3].wrapping_add(prev[3]),
    //     ];
    //     *TryInto::<&mut [u8; 4]>::try_into(chunk).unwrap() = new_chunk;
    //     prev = new_chunk;
    // }
    // ```
    // While we're not manually bit-twiddling anymore, a possible takeaway from
    // this is to "think in SWAR" when dealing with small byte arrays. Unrolling
    // array operations and performing them component-wise may unlock previously
    // unavailable optimizations from the compiler, even when using the
    // `chunks_exact` methods for their potential auto-vectorization benefits.
    match filter {
        NoFilter => {}
        Sub => match tbpp {
            BytesPerPixel::One => {
                current.iter_mut().reduce(|&mut prev, curr| {
                    *curr = curr.wrapping_add(prev);
                    curr
                });
            }
            BytesPerPixel::Two => {
                let mut prev = [0; 2];
                for chunk in current.chunks_exact_mut(2) {
                    let new_chunk = [
                        chunk[0].wrapping_add(prev[0]),
                        chunk[1].wrapping_add(prev[1]),
                    ];
                    *TryInto::<&mut [u8; 2]>::try_into(chunk).unwrap() = new_chunk;
                    prev = new_chunk;
                }
            }
            BytesPerPixel::Three => {
                let mut prev = [0; 3];
                for chunk in current.chunks_exact_mut(3) {
                    let new_chunk = [
                        chunk[0].wrapping_add(prev[0]),
                        chunk[1].wrapping_add(prev[1]),
                        chunk[2].wrapping_add(prev[2]),
                    ];
                    *TryInto::<&mut [u8; 3]>::try_into(chunk).unwrap() = new_chunk;
                    prev = new_chunk;
                }
            }
            BytesPerPixel::Four => {
                let mut prev = [0; 4];
                for chunk in current.chunks_exact_mut(4) {
                    let new_chunk = [
                        chunk[0].wrapping_add(prev[0]),
                        chunk[1].wrapping_add(prev[1]),
                        chunk[2].wrapping_add(prev[2]),
                        chunk[3].wrapping_add(prev[3]),
                    ];
                    *TryInto::<&mut [u8; 4]>::try_into(chunk).unwrap() = new_chunk;
                    prev = new_chunk;
                }
            }
            BytesPerPixel::Six => {
                let mut prev = [0; 6];
                for chunk in current.chunks_exact_mut(6) {
                    let new_chunk = [
                        chunk[0].wrapping_add(prev[0]),
                        chunk[1].wrapping_add(prev[1]),
                        chunk[2].wrapping_add(prev[2]),
                        chunk[3].wrapping_add(prev[3]),
                        chunk[4].wrapping_add(prev[4]),
                        chunk[5].wrapping_add(prev[5]),
                    ];
                    *TryInto::<&mut [u8; 6]>::try_into(chunk).unwrap() = new_chunk;
                    prev = new_chunk;
                }
            }
            BytesPerPixel::Eight => {
                let mut prev = [0; 8];
                for chunk in current.chunks_exact_mut(8) {
                    let new_chunk = [
                        chunk[0].wrapping_add(prev[0]),
                        chunk[1].wrapping_add(prev[1]),
                        chunk[2].wrapping_add(prev[2]),
                        chunk[3].wrapping_add(prev[3]),
                        chunk[4].wrapping_add(prev[4]),
                        chunk[5].wrapping_add(prev[5]),
                        chunk[6].wrapping_add(prev[6]),
                        chunk[7].wrapping_add(prev[7]),
                    ];
                    *TryInto::<&mut [u8; 8]>::try_into(chunk).unwrap() = new_chunk;
                    prev = new_chunk;
                }
            }
        },
        Up => {
            for (curr, &above) in current.iter_mut().zip(previous) {
                *curr = curr.wrapping_add(above);
            }
        }
        Avg if previous.is_empty() => match tbpp {
            BytesPerPixel::One => {
                current.iter_mut().reduce(|&mut prev, curr| {
                    *curr = curr.wrapping_add(prev / 2);
                    curr
                });
            }
            BytesPerPixel::Two => {
                let mut prev = [0; 2];
                for chunk in current.chunks_exact_mut(2) {
                    let new_chunk = [
                        chunk[0].wrapping_add(prev[0] / 2),
                        chunk[1].wrapping_add(prev[1] / 2),
                    ];
                    *TryInto::<&mut [u8; 2]>::try_into(chunk).unwrap() = new_chunk;
                    prev = new_chunk;
                }
            }
            BytesPerPixel::Three => {
                let mut prev = [0; 3];
                for chunk in current.chunks_exact_mut(3) {
                    let new_chunk = [
                        chunk[0].wrapping_add(prev[0] / 2),
                        chunk[1].wrapping_add(prev[1] / 2),
                        chunk[2].wrapping_add(prev[2] / 2),
                    ];
                    *TryInto::<&mut [u8; 3]>::try_into(chunk).unwrap() = new_chunk;
                    prev = new_chunk;
                }
            }
            BytesPerPixel::Four => {
                let mut prev = [0; 4];
                for chunk in current.chunks_exact_mut(4) {
                    let new_chunk = [
                        chunk[0].wrapping_add(prev[0] / 2),
                        chunk[1].wrapping_add(prev[1] / 2),
                        chunk[2].wrapping_add(prev[2] / 2),
                        chunk[3].wrapping_add(prev[3] / 2),
                    ];
                    *TryInto::<&mut [u8; 4]>::try_into(chunk).unwrap() = new_chunk;
                    prev = new_chunk;
                }
            }
            BytesPerPixel::Six => {
                let mut prev = [0; 6];
                for chunk in current.chunks_exact_mut(6) {
                    let new_chunk = [
                        chunk[0].wrapping_add(prev[0] / 2),
                        chunk[1].wrapping_add(prev[1] / 2),
                        chunk[2].wrapping_add(prev[2] / 2),
                        chunk[3].wrapping_add(prev[3] / 2),
                        chunk[4].wrapping_add(prev[4] / 2),
                        chunk[5].wrapping_add(prev[5] / 2),
                    ];
                    *TryInto::<&mut [u8; 6]>::try_into(chunk).unwrap() = new_chunk;
                    prev = new_chunk;
                }
            }
            BytesPerPixel::Eight => {
                let mut prev = [0; 8];
                for chunk in current.chunks_exact_mut(8) {
                    let new_chunk = [
                        chunk[0].wrapping_add(prev[0] / 2),
                        chunk[1].wrapping_add(prev[1] / 2),
                        chunk[2].wrapping_add(prev[2] / 2),
                        chunk[3].wrapping_add(prev[3] / 2),
                        chunk[4].wrapping_add(prev[4] / 2),
                        chunk[5].wrapping_add(prev[5] / 2),
                        chunk[6].wrapping_add(prev[6] / 2),
                        chunk[7].wrapping_add(prev[7] / 2),
                    ];
                    *TryInto::<&mut [u8; 8]>::try_into(chunk).unwrap() = new_chunk;
                    prev = new_chunk;
                }
            }
        },
        Avg => match tbpp {
            BytesPerPixel::One => {
                let mut lprev = [0; 1];
                for (chunk, above) in current.chunks_exact_mut(1).zip(previous.chunks_exact(1)) {
                    let new_chunk =
                        [chunk[0].wrapping_add(((above[0] as u16 + lprev[0] as u16) / 2) as u8)];
                    *TryInto::<&mut [u8; 1]>::try_into(chunk).unwrap() = new_chunk;
                    lprev = new_chunk;
                }
            }
            BytesPerPixel::Two => {
                let mut lprev = [0; 2];
                for (chunk, above) in current.chunks_exact_mut(2).zip(previous.chunks_exact(2)) {
                    let new_chunk = [
                        chunk[0].wrapping_add(((above[0] as u16 + lprev[0] as u16) / 2) as u8),
                        chunk[1].wrapping_add(((above[1] as u16 + lprev[1] as u16) / 2) as u8),
                    ];
                    *TryInto::<&mut [u8; 2]>::try_into(chunk).unwrap() = new_chunk;
                    lprev = new_chunk;
                }
            }
            BytesPerPixel::Three => {
                let mut lprev = [0; 3];
                for (chunk, above) in current.chunks_exact_mut(3).zip(previous.chunks_exact(3)) {
                    let new_chunk = [
                        chunk[0].wrapping_add(((above[0] as u16 + lprev[0] as u16) / 2) as u8),
                        chunk[1].wrapping_add(((above[1] as u16 + lprev[1] as u16) / 2) as u8),
                        chunk[2].wrapping_add(((above[2] as u16 + lprev[2] as u16) / 2) as u8),
                    ];
                    *TryInto::<&mut [u8; 3]>::try_into(chunk).unwrap() = new_chunk;
                    lprev = new_chunk;
                }
            }
            BytesPerPixel::Four => {
                let mut lprev = [0; 4];
                for (chunk, above) in current.chunks_exact_mut(4).zip(previous.chunks_exact(4)) {
                    let new_chunk = [
                        chunk[0].wrapping_add(((above[0] as u16 + lprev[0] as u16) / 2) as u8),
                        chunk[1].wrapping_add(((above[1] as u16 + lprev[1] as u16) / 2) as u8),
                        chunk[2].wrapping_add(((above[2] as u16 + lprev[2] as u16) / 2) as u8),
                        chunk[3].wrapping_add(((above[3] as u16 + lprev[3] as u16) / 2) as u8),
                    ];
                    *TryInto::<&mut [u8; 4]>::try_into(chunk).unwrap() = new_chunk;
                    lprev = new_chunk;
                }
            }
            BytesPerPixel::Six => {
                let mut lprev = [0; 6];
                for (chunk, above) in current.chunks_exact_mut(6).zip(previous.chunks_exact(6)) {
                    let new_chunk = [
                        chunk[0].wrapping_add(((above[0] as u16 + lprev[0] as u16) / 2) as u8),
                        chunk[1].wrapping_add(((above[1] as u16 + lprev[1] as u16) / 2) as u8),
                        chunk[2].wrapping_add(((above[2] as u16 + lprev[2] as u16) / 2) as u8),
                        chunk[3].wrapping_add(((above[3] as u16 + lprev[3] as u16) / 2) as u8),
                        chunk[4].wrapping_add(((above[4] as u16 + lprev[4] as u16) / 2) as u8),
                        chunk[5].wrapping_add(((above[5] as u16 + lprev[5] as u16) / 2) as u8),
                    ];
                    *TryInto::<&mut [u8; 6]>::try_into(chunk).unwrap() = new_chunk;
                    lprev = new_chunk;
                }
            }
            BytesPerPixel::Eight => {
                let mut lprev = [0; 8];
                for (chunk, above) in current.chunks_exact_mut(8).zip(previous.chunks_exact(8)) {
                    let new_chunk = [
                        chunk[0].wrapping_add(((above[0] as u16 + lprev[0] as u16) / 2) as u8),
                        chunk[1].wrapping_add(((above[1] as u16 + lprev[1] as u16) / 2) as u8),
                        chunk[2].wrapping_add(((above[2] as u16 + lprev[2] as u16) / 2) as u8),
                        chunk[3].wrapping_add(((above[3] as u16 + lprev[3] as u16) / 2) as u8),
                        chunk[4].wrapping_add(((above[4] as u16 + lprev[4] as u16) / 2) as u8),
                        chunk[5].wrapping_add(((above[5] as u16 + lprev[5] as u16) / 2) as u8),
                        chunk[6].wrapping_add(((above[6] as u16 + lprev[6] as u16) / 2) as u8),
                        chunk[7].wrapping_add(((above[7] as u16 + lprev[7] as u16) / 2) as u8),
                    ];
                    *TryInto::<&mut [u8; 8]>::try_into(chunk).unwrap() = new_chunk;
                    lprev = new_chunk;
                }
            }
        },
        #[allow(unreachable_code)]
        Paeth => {
            // Select the fastest Paeth filter implementation based on the target architecture.
            let filter_paeth_decode = if cfg!(target_arch = "x86_64") {
                filter_paeth_stbi
            } else {
                filter_paeth
            };

            // Paeth filter pixels:
            // C B D
            // A X
            match tbpp {
                BytesPerPixel::One => {
                    let mut a_bpp = [0; 1];
                    let mut c_bpp = [0; 1];
                    for (chunk, b_bpp) in current.chunks_exact_mut(1).zip(previous.chunks_exact(1))
                    {
                        let new_chunk = [chunk[0]
                            .wrapping_add(filter_paeth_decode(a_bpp[0], b_bpp[0], c_bpp[0]))];
                        *TryInto::<&mut [u8; 1]>::try_into(chunk).unwrap() = new_chunk;
                        a_bpp = new_chunk;
                        c_bpp = b_bpp.try_into().unwrap();
                    }
                }
                BytesPerPixel::Two => {
                    let mut a_bpp = [0; 2];
                    let mut c_bpp = [0; 2];
                    for (chunk, b_bpp) in current.chunks_exact_mut(2).zip(previous.chunks_exact(2))
                    {
                        let new_chunk = [
                            chunk[0]
                                .wrapping_add(filter_paeth_decode(a_bpp[0], b_bpp[0], c_bpp[0])),
                            chunk[1]
                                .wrapping_add(filter_paeth_decode(a_bpp[1], b_bpp[1], c_bpp[1])),
                        ];
                        *TryInto::<&mut [u8; 2]>::try_into(chunk).unwrap() = new_chunk;
                        a_bpp = new_chunk;
                        c_bpp = b_bpp.try_into().unwrap();
                    }
                }
                BytesPerPixel::Three => {
                    // Do not enable this algorithm on ARM, that would be a big performance hit
                    #[cfg(all(feature = "unstable", target_arch = "x86_64"))]
                    {
                        simd::unfilter_paeth3(previous, current);
                        return;
                    }

                    let mut a_bpp = [0; 3];
                    let mut c_bpp = [0; 3];
                    for (chunk, b_bpp) in current.chunks_exact_mut(3).zip(previous.chunks_exact(3))
                    {
                        let new_chunk = [
                            chunk[0]
                                .wrapping_add(filter_paeth_decode(a_bpp[0], b_bpp[0], c_bpp[0])),
                            chunk[1]
                                .wrapping_add(filter_paeth_decode(a_bpp[1], b_bpp[1], c_bpp[1])),
                            chunk[2]
                                .wrapping_add(filter_paeth_decode(a_bpp[2], b_bpp[2], c_bpp[2])),
                        ];
                        *TryInto::<&mut [u8; 3]>::try_into(chunk).unwrap() = new_chunk;
                        a_bpp = new_chunk;
                        c_bpp = b_bpp.try_into().unwrap();
                    }
                }
                BytesPerPixel::Four => {
                    #[cfg(all(feature = "unstable", target_arch = "x86_64"))]
                    {
                        simd::unfilter_paeth_u8::<4>(previous, current);
                        return;
                    }

                    let mut a_bpp = [0; 4];
                    let mut c_bpp = [0; 4];
                    for (chunk, b_bpp) in current.chunks_exact_mut(4).zip(previous.chunks_exact(4))
                    {
                        let new_chunk = [
                            chunk[0]
                                .wrapping_add(filter_paeth_decode(a_bpp[0], b_bpp[0], c_bpp[0])),
                            chunk[1]
                                .wrapping_add(filter_paeth_decode(a_bpp[1], b_bpp[1], c_bpp[1])),
                            chunk[2]
                                .wrapping_add(filter_paeth_decode(a_bpp[2], b_bpp[2], c_bpp[2])),
                            chunk[3]
                                .wrapping_add(filter_paeth_decode(a_bpp[3], b_bpp[3], c_bpp[3])),
                        ];
                        *TryInto::<&mut [u8; 4]>::try_into(chunk).unwrap() = new_chunk;
                        a_bpp = new_chunk;
                        c_bpp = b_bpp.try_into().unwrap();
                    }
                }
                BytesPerPixel::Six => {
                    #[cfg(all(feature = "unstable", target_arch = "x86_64"))]
                    {
                        simd::unfilter_paeth6(previous, current);
                        return;
                    }

                    let mut a_bpp = [0; 6];
                    let mut c_bpp = [0; 6];
                    for (chunk, b_bpp) in current.chunks_exact_mut(6).zip(previous.chunks_exact(6))
                    {
                        let new_chunk = [
                            chunk[0]
                                .wrapping_add(filter_paeth_decode(a_bpp[0], b_bpp[0], c_bpp[0])),
                            chunk[1]
                                .wrapping_add(filter_paeth_decode(a_bpp[1], b_bpp[1], c_bpp[1])),
                            chunk[2]
                                .wrapping_add(filter_paeth_decode(a_bpp[2], b_bpp[2], c_bpp[2])),
                            chunk[3]
                                .wrapping_add(filter_paeth_decode(a_bpp[3], b_bpp[3], c_bpp[3])),
                            chunk[4]
                                .wrapping_add(filter_paeth_decode(a_bpp[4], b_bpp[4], c_bpp[4])),
                            chunk[5]
                                .wrapping_add(filter_paeth_decode(a_bpp[5], b_bpp[5], c_bpp[5])),
                        ];
                        *TryInto::<&mut [u8; 6]>::try_into(chunk).unwrap() = new_chunk;
                        a_bpp = new_chunk;
                        c_bpp = b_bpp.try_into().unwrap();
                    }
                }
                BytesPerPixel::Eight => {
                    #[cfg(all(feature = "unstable", target_arch = "x86_64"))]
                    {
                        simd::unfilter_paeth_u8::<8>(previous, current);
                        return;
                    }

                    let mut a_bpp = [0; 8];
                    let mut c_bpp = [0; 8];
                    for (chunk, b_bpp) in current.chunks_exact_mut(8).zip(previous.chunks_exact(8))
                    {
                        let new_chunk = [
                            chunk[0]
                                .wrapping_add(filter_paeth_decode(a_bpp[0], b_bpp[0], c_bpp[0])),
                            chunk[1]
                                .wrapping_add(filter_paeth_decode(a_bpp[1], b_bpp[1], c_bpp[1])),
                            chunk[2]
                                .wrapping_add(filter_paeth_decode(a_bpp[2], b_bpp[2], c_bpp[2])),
                            chunk[3]
                                .wrapping_add(filter_paeth_decode(a_bpp[3], b_bpp[3], c_bpp[3])),
                            chunk[4]
                                .wrapping_add(filter_paeth_decode(a_bpp[4], b_bpp[4], c_bpp[4])),
                            chunk[5]
                                .wrapping_add(filter_paeth_decode(a_bpp[5], b_bpp[5], c_bpp[5])),
                            chunk[6]
                                .wrapping_add(filter_paeth_decode(a_bpp[6], b_bpp[6], c_bpp[6])),
                            chunk[7]
                                .wrapping_add(filter_paeth_decode(a_bpp[7], b_bpp[7], c_bpp[7])),
                        ];
                        *TryInto::<&mut [u8; 8]>::try_into(chunk).unwrap() = new_chunk;
                        a_bpp = new_chunk;
                        c_bpp = b_bpp.try_into().unwrap();
                    }
                }
            }
        }
    }
}

fn filter_internal(
    method: FilterType,
    bpp: usize,
    len: usize,
    previous: &[u8],
    current: &[u8],
    output: &mut [u8],
) -> FilterType {
    use self::FilterType::*;

    // This value was chosen experimentally based on what achieved the best performance. The
    // Rust compiler does auto-vectorization, and 32-bytes per loop iteration seems to enable
    // the fastest code when doing so.
    const CHUNK_SIZE: usize = 32;

    match method {
        NoFilter => {
            output.copy_from_slice(current);
            NoFilter
        }
        Sub => {
            let mut out_chunks = output[bpp..].chunks_exact_mut(CHUNK_SIZE);
            let mut cur_chunks = current[bpp..].chunks_exact(CHUNK_SIZE);
            let mut prev_chunks = current[..len - bpp].chunks_exact(CHUNK_SIZE);

            for ((out, cur), prev) in (&mut out_chunks).zip(&mut cur_chunks).zip(&mut prev_chunks) {
                for i in 0..CHUNK_SIZE {
                    out[i] = cur[i].wrapping_sub(prev[i]);
                }
            }

            for ((out, cur), &prev) in out_chunks
                .into_remainder()
                .iter_mut()
                .zip(cur_chunks.remainder())
                .zip(prev_chunks.remainder())
            {
                *out = cur.wrapping_sub(prev);
            }

            output[..bpp].copy_from_slice(&current[..bpp]);
            Sub
        }
        Up => {
            let mut out_chunks = output.chunks_exact_mut(CHUNK_SIZE);
            let mut cur_chunks = current.chunks_exact(CHUNK_SIZE);
            let mut prev_chunks = previous.chunks_exact(CHUNK_SIZE);

            for ((out, cur), prev) in (&mut out_chunks).zip(&mut cur_chunks).zip(&mut prev_chunks) {
                for i in 0..CHUNK_SIZE {
                    out[i] = cur[i].wrapping_sub(prev[i]);
                }
            }

            for ((out, cur), &prev) in out_chunks
                .into_remainder()
                .iter_mut()
                .zip(cur_chunks.remainder())
                .zip(prev_chunks.remainder())
            {
                *out = cur.wrapping_sub(prev);
            }
            Up
        }
        Avg => {
            let mut out_chunks = output[bpp..].chunks_exact_mut(CHUNK_SIZE);
            let mut cur_chunks = current[bpp..].chunks_exact(CHUNK_SIZE);
            let mut cur_minus_bpp_chunks = current[..len - bpp].chunks_exact(CHUNK_SIZE);
            let mut prev_chunks = previous[bpp..].chunks_exact(CHUNK_SIZE);

            for (((out, cur), cur_minus_bpp), prev) in (&mut out_chunks)
                .zip(&mut cur_chunks)
                .zip(&mut cur_minus_bpp_chunks)
                .zip(&mut prev_chunks)
            {
                for i in 0..CHUNK_SIZE {
                    // Bitwise average of two integers without overflow and
                    // without converting to a wider bit-width. See:
                    // http://aggregate.org/MAGIC/#Average%20of%20Integers
                    // If this is unrolled by component, consider reverting to
                    // `((cur_minus_bpp[i] as u16 + prev[i] as u16) / 2) as u8`
                    out[i] = cur[i].wrapping_sub(
                        (cur_minus_bpp[i] & prev[i]) + ((cur_minus_bpp[i] ^ prev[i]) >> 1),
                    );
                }
            }

            for (((out, cur), &cur_minus_bpp), &prev) in out_chunks
                .into_remainder()
                .iter_mut()
                .zip(cur_chunks.remainder())
                .zip(cur_minus_bpp_chunks.remainder())
                .zip(prev_chunks.remainder())
            {
                *out = cur.wrapping_sub((cur_minus_bpp & prev) + ((cur_minus_bpp ^ prev) >> 1));
            }

            for i in 0..bpp {
                output[i] = current[i].wrapping_sub(previous[i] / 2);
            }
            Avg
        }
        Paeth => {
            let mut out_chunks = output[bpp..].chunks_exact_mut(CHUNK_SIZE);
            let mut cur_chunks = current[bpp..].chunks_exact(CHUNK_SIZE);
            let mut a_chunks = current[..len - bpp].chunks_exact(CHUNK_SIZE);
            let mut b_chunks = previous[bpp..].chunks_exact(CHUNK_SIZE);
            let mut c_chunks = previous[..len - bpp].chunks_exact(CHUNK_SIZE);

            for ((((out, cur), a), b), c) in (&mut out_chunks)
                .zip(&mut cur_chunks)
                .zip(&mut a_chunks)
                .zip(&mut b_chunks)
                .zip(&mut c_chunks)
            {
                for i in 0..CHUNK_SIZE {
                    out[i] = cur[i].wrapping_sub(filter_paeth_fpnge(a[i], b[i], c[i]));
                }
            }

            for ((((out, cur), &a), &b), &c) in out_chunks
                .into_remainder()
                .iter_mut()
                .zip(cur_chunks.remainder())
                .zip(a_chunks.remainder())
                .zip(b_chunks.remainder())
                .zip(c_chunks.remainder())
            {
                *out = cur.wrapping_sub(filter_paeth_fpnge(a, b, c));
            }

            for i in 0..bpp {
                output[i] = current[i].wrapping_sub(filter_paeth_fpnge(0, previous[i], 0));
            }
            Paeth
        }
    }
}

pub(crate) fn filter(
    method: FilterType,
    adaptive: AdaptiveFilterType,
    bpp: BytesPerPixel,
    previous: &[u8],
    current: &[u8],
    output: &mut [u8],
) -> FilterType {
    use FilterType::*;
    let bpp = bpp.into_usize();
    let len = current.len();

    match adaptive {
        AdaptiveFilterType::NonAdaptive => {
            filter_internal(method, bpp, len, previous, current, output)
        }
        AdaptiveFilterType::Adaptive => {
            let mut min_sum: u64 = u64::MAX;
            let mut filter_choice = FilterType::NoFilter;
            for &filter in [Sub, Up, Avg, Paeth].iter() {
                filter_internal(filter, bpp, len, previous, current, output);
                let sum = sum_buffer(output);
                if sum <= min_sum {
                    min_sum = sum;
                    filter_choice = filter;
                }
            }

            if filter_choice != Paeth {
                filter_internal(filter_choice, bpp, len, previous, current, output);
            }
            filter_choice
        }
    }
}

// Helper function for Adaptive filter buffer summation
fn sum_buffer(buf: &[u8]) -> u64 {
    const CHUNK_SIZE: usize = 32;

    let mut buf_chunks = buf.chunks_exact(CHUNK_SIZE);
    let mut sum = 0_u64;

    for chunk in &mut buf_chunks {
        // At most, `acc` can be `32 * (i8::MIN as u8) = 32 * 128 = 4096`.
        let mut acc = 0;
        for &b in chunk {
            acc += u64::from((b as i8).unsigned_abs());
        }
        sum = sum.saturating_add(acc);
    }

    let mut acc = 0;
    for &b in buf_chunks.remainder() {
        acc += u64::from((b as i8).unsigned_abs());
    }

    sum.saturating_add(acc)
}

#[cfg(test)]
mod test {
    use super::*;
    use core::iter;

    #[test]
    fn roundtrip() {
        // A multiple of 8, 6, 4, 3, 2, 1
        const LEN: u8 = 240;
        let previous: Vec<_> = iter::repeat(1).take(LEN.into()).collect();
        let current: Vec<_> = (0..LEN).collect();
        let expected = current.clone();
        let adaptive = AdaptiveFilterType::NonAdaptive;

        let roundtrip = |kind, bpp: BytesPerPixel| {
            let mut output = vec![0; LEN.into()];
            filter(kind, adaptive, bpp, &previous, &current, &mut output);
            unfilter(kind, bpp, &previous, &mut output);
            assert_eq!(
                output, expected,
                "Filtering {:?} with {:?} does not roundtrip",
                bpp, kind
            );
        };

        let filters = [
            FilterType::NoFilter,
            FilterType::Sub,
            FilterType::Up,
            FilterType::Avg,
            FilterType::Paeth,
        ];

        let bpps = [
            BytesPerPixel::One,
            BytesPerPixel::Two,
            BytesPerPixel::Three,
            BytesPerPixel::Four,
            BytesPerPixel::Six,
            BytesPerPixel::Eight,
        ];

        for &filter in filters.iter() {
            for &bpp in bpps.iter() {
                roundtrip(filter, bpp);
            }
        }
    }

    #[test]
    #[ignore] // takes ~20s without optimizations
    fn paeth_impls_are_equivalent() {
        for a in 0..=255 {
            for b in 0..=255 {
                for c in 0..=255 {
                    let baseline = filter_paeth(a, b, c);
                    let fpnge = filter_paeth_fpnge(a, b, c);
                    let stbi = filter_paeth_stbi(a, b, c);
                    let stbi_i16 = filter_paeth_stbi_i16(a as i16, b as i16, c as i16);

                    assert_eq!(baseline, fpnge);
                    assert_eq!(baseline, stbi);
                    assert_eq!(baseline as i16, stbi_i16);
                }
            }
        }
    }

    #[test]
    fn roundtrip_ascending_previous_line() {
        // A multiple of 8, 6, 4, 3, 2, 1
        const LEN: u8 = 240;
        let previous: Vec<_> = (0..LEN).collect();
        let current: Vec<_> = (0..LEN).collect();
        let expected = current.clone();
        let adaptive = AdaptiveFilterType::NonAdaptive;

        let roundtrip = |kind, bpp: BytesPerPixel| {
            let mut output = vec![0; LEN.into()];
            filter(kind, adaptive, bpp, &previous, &current, &mut output);
            unfilter(kind, bpp, &previous, &mut output);
            assert_eq!(
                output, expected,
                "Filtering {:?} with {:?} does not roundtrip",
                bpp, kind
            );
        };

        let filters = [
            FilterType::NoFilter,
            FilterType::Sub,
            FilterType::Up,
            FilterType::Avg,
            FilterType::Paeth,
        ];

        let bpps = [
            BytesPerPixel::One,
            BytesPerPixel::Two,
            BytesPerPixel::Three,
            BytesPerPixel::Four,
            BytesPerPixel::Six,
            BytesPerPixel::Eight,
        ];

        for &filter in filters.iter() {
            for &bpp in bpps.iter() {
                roundtrip(filter, bpp);
            }
        }
    }

    #[test]
    // This tests that converting u8 to i8 doesn't overflow when taking the
    // absolute value for adaptive filtering: -128_i8.abs() will panic in debug
    // or produce garbage in release mode. The sum of 0..=255u8 should equal the
    // sum of the absolute values of -128_i8..=127, or abs(-128..=0) + 1..=127.
    fn sum_buffer_test() {
        let sum = (0..=128).sum::<u64>() + (1..=127).sum::<u64>();
        let buf: Vec<u8> = (0_u8..=255).collect();

        assert_eq!(sum, crate::filter::sum_buffer(&buf));
    }
}
