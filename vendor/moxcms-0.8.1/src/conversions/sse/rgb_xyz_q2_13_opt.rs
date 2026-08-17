/*
 * // Copyright (c) Radzivon Bartoshyk 3/2025. All rights reserved.
 * //
 * // Redistribution and use in source and binary forms, with or without modification,
 * // are permitted provided that the following conditions are met:
 * //
 * // 1.  Redistributions of source code must retain the above copyright notice, this
 * // list of conditions and the following disclaimer.
 * //
 * // 2.  Redistributions in binary form must reproduce the above copyright notice,
 * // this list of conditions and the following disclaimer in the documentation
 * // and/or other materials provided with the distribution.
 * //
 * // 3.  Neither the name of the copyright holder nor the names of its
 * // contributors may be used to endorse or promote products derived from
 * // this software without specific prior written permission.
 * //
 * // THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
 * // AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * // IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
 * // DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE
 * // FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
 * // DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
 * // SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER
 * // CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY,
 * // OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * // OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */
#![cfg(feature = "sse_shaper_fixed_point_paths")]
use crate::conversions::rgbxyz_fixed::TransformMatrixShaperFpOptVec;
use crate::conversions::sse::SseAlignedU16;
use crate::transform::PointeeSizeExpressible;
use crate::{CmsError, Layout, TransformExecutor};
use num_traits::AsPrimitive;
#[cfg(target_arch = "x86")]
use std::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;

#[inline(always)]
#[allow(dead_code)]
pub(crate) unsafe fn _xmm_load_epi32(f: &i32) -> __m128i {
    let float_ref: &f32 = unsafe { &*(f as *const i32 as *const f32) };
    unsafe { _mm_castps_si128(_mm_load_ss(float_ref)) }
}

pub(crate) struct TransformShaperQ2_13OptSse<
    T: Copy,
    const SRC_LAYOUT: u8,
    const DST_LAYOUT: u8,
    const PRECISION: i32,
> {
    pub(crate) profile: TransformMatrixShaperFpOptVec<i32, i16, T>,
    pub(crate) bit_depth: usize,
    pub(crate) gamma_lut: usize,
}

impl<
    T: Copy + PointeeSizeExpressible + 'static,
    const SRC_LAYOUT: u8,
    const DST_LAYOUT: u8,
    const PRECISION: i32,
> TransformShaperQ2_13OptSse<T, SRC_LAYOUT, DST_LAYOUT, PRECISION>
where
    u32: AsPrimitive<T>,
{
    #[target_feature(enable = "sse4.1")]
    unsafe fn transform_impl(&self, src: &[T], dst: &mut [T]) -> Result<(), CmsError> {
        let src_cn = Layout::from(SRC_LAYOUT);
        let dst_cn = Layout::from(DST_LAYOUT);
        let src_channels = src_cn.channels();
        let dst_channels = dst_cn.channels();

        let mut temporary = SseAlignedU16([0; 8]);

        if src.len() / src_channels != dst.len() / dst_channels {
            return Err(CmsError::LaneSizeMismatch);
        }
        if src.len() % src_channels != 0 {
            return Err(CmsError::LaneMultipleOfChannels);
        }
        if dst.len() % dst_channels != 0 {
            return Err(CmsError::LaneMultipleOfChannels);
        }

        let t = self.profile.adaptation_matrix.transpose();

        let max_colors = ((1 << self.bit_depth) - 1).as_();

        unsafe {
            let m0 = _mm_setr_epi16(
                t.v[0][0], t.v[1][0], t.v[0][1], t.v[1][1], t.v[0][2], t.v[1][2], 0, 0,
            );
            let m2 = _mm_setr_epi16(t.v[2][0], 1, t.v[2][1], 1, t.v[2][2], 1, 0, 0);

            let rnd_val = ((1i32 << (PRECISION - 1)) as i16).to_ne_bytes();
            let rnd = _mm_set1_epi32(i32::from_ne_bytes([0, 0, rnd_val[0], rnd_val[1]]));

            let v_max_value = _mm_set1_epi32(self.gamma_lut as i32 - 1);

            // safety precondition for linearization table
            if T::FINITE {
                let cap = (1 << self.bit_depth) - 1;
                assert!(self.profile.linear.len() >= cap);
            } else {
                assert!(self.profile.linear.len() >= T::NOT_FINITE_LINEAR_TABLE_SIZE);
            }

            let lut_lin = &self.profile.linear;

            for (src, dst) in src
                .chunks_exact(src_channels)
                .zip(dst.chunks_exact_mut(dst_channels))
            {
                let rp = lut_lin.get_unchecked(src[src_cn.r_i()]._as_usize());
                let gp = lut_lin.get_unchecked(src[src_cn.g_i()]._as_usize());
                let bp = lut_lin.get_unchecked(src[src_cn.b_i()]._as_usize());

                let mut r = _xmm_load_epi32(rp);
                let mut g = _xmm_load_epi32(gp);
                let mut b = _xmm_load_epi32(bp);
                let a = if src_channels == 4 {
                    src[src_cn.a_i()]
                } else {
                    max_colors
                };

                r = _mm_shuffle_epi32::<0>(r);
                g = _mm_shuffle_epi32::<0>(g);
                b = _mm_shuffle_epi32::<0>(b);

                g = _mm_slli_epi32::<16>(g);

                let zrg0 = _mm_or_si128(r, g);
                let zbz0 = _mm_or_si128(b, rnd);

                let v0 = _mm_madd_epi16(zrg0, m0);
                let v1 = _mm_madd_epi16(zbz0, m2);

                let mut v = _mm_add_epi32(v0, v1);

                v = _mm_srai_epi32::<PRECISION>(v);
                v = _mm_max_epi32(v, _mm_setzero_si128());
                v = _mm_min_epi32(v, v_max_value);

                _mm_store_si128(temporary.0.as_mut_ptr() as *mut _, v);

                dst[dst_cn.r_i()] = self.profile.gamma[temporary.0[0] as usize];
                dst[dst_cn.g_i()] = self.profile.gamma[temporary.0[2] as usize];
                dst[dst_cn.b_i()] = self.profile.gamma[temporary.0[4] as usize];
                if dst_channels == 4 {
                    dst[dst_cn.a_i()] = a;
                }
            }
        }

        Ok(())
    }

    #[cfg(feature = "in_place")]
    #[target_feature(enable = "sse4.1")]
    unsafe fn transform_in_place_impl(&self, in_out: &mut [T]) -> Result<(), CmsError> {
        let src_cn = Layout::from(SRC_LAYOUT);
        let src_channels = src_cn.channels();

        assert_eq!(
            SRC_LAYOUT, DST_LAYOUT,
            "This is in-place transform, layout must not diverge"
        );

        let mut temporary = SseAlignedU16([0; 8]);

        if in_out.len() % src_channels != 0 {
            return Err(CmsError::LaneMultipleOfChannels);
        }

        let t = self.profile.adaptation_matrix.transpose();

        let max_colors = ((1 << self.bit_depth) - 1).as_();

        unsafe {
            let m0 = _mm_setr_epi16(
                t.v[0][0], t.v[1][0], t.v[0][1], t.v[1][1], t.v[0][2], t.v[1][2], 0, 0,
            );
            let m2 = _mm_setr_epi16(t.v[2][0], 1, t.v[2][1], 1, t.v[2][2], 1, 0, 0);

            let rnd_val = ((1i32 << (PRECISION - 1)) as i16).to_ne_bytes();
            let rnd = _mm_set1_epi32(i32::from_ne_bytes([0, 0, rnd_val[0], rnd_val[1]]));

            let v_max_value = _mm_set1_epi32(self.gamma_lut as i32 - 1);

            // safety precondition for linearization table
            if T::FINITE {
                let cap = (1 << self.bit_depth) - 1;
                assert!(self.profile.linear.len() >= cap);
            } else {
                assert!(self.profile.linear.len() >= T::NOT_FINITE_LINEAR_TABLE_SIZE);
            }

            let lut_lin = &self.profile.linear;

            for dst in in_out.chunks_exact_mut(src_channels) {
                let rp = lut_lin.get_unchecked(dst[src_cn.r_i()]._as_usize());
                let gp = lut_lin.get_unchecked(dst[src_cn.g_i()]._as_usize());
                let bp = lut_lin.get_unchecked(dst[src_cn.b_i()]._as_usize());

                let mut r = _xmm_load_epi32(rp);
                let mut g = _xmm_load_epi32(gp);
                let mut b = _xmm_load_epi32(bp);
                let a = if src_channels == 4 {
                    dst[src_cn.a_i()]
                } else {
                    max_colors
                };

                r = _mm_shuffle_epi32::<0>(r);
                g = _mm_shuffle_epi32::<0>(g);
                b = _mm_shuffle_epi32::<0>(b);

                g = _mm_slli_epi32::<16>(g);

                let zrg0 = _mm_or_si128(r, g);
                let zbz0 = _mm_or_si128(b, rnd);

                let v0 = _mm_madd_epi16(zrg0, m0);
                let v1 = _mm_madd_epi16(zbz0, m2);

                let mut v = _mm_add_epi32(v0, v1);

                v = _mm_srai_epi32::<PRECISION>(v);
                v = _mm_max_epi32(v, _mm_setzero_si128());
                v = _mm_min_epi32(v, v_max_value);

                _mm_store_si128(temporary.0.as_mut_ptr() as *mut _, v);

                dst[src_cn.r_i()] = self.profile.gamma[temporary.0[0] as usize];
                dst[src_cn.g_i()] = self.profile.gamma[temporary.0[2] as usize];
                dst[src_cn.b_i()] = self.profile.gamma[temporary.0[4] as usize];
                if src_channels == 4 {
                    dst[src_cn.a_i()] = a;
                }
            }
        }

        Ok(())
    }
}

impl<
    T: Copy + PointeeSizeExpressible + 'static + Default,
    const SRC_LAYOUT: u8,
    const DST_LAYOUT: u8,
    const PRECISION: i32,
> TransformExecutor<T> for TransformShaperQ2_13OptSse<T, SRC_LAYOUT, DST_LAYOUT, PRECISION>
where
    u32: AsPrimitive<T>,
{
    fn transform(&self, src: &[T], dst: &mut [T]) -> Result<(), CmsError> {
        unsafe { self.transform_impl(src, dst) }
    }
}

#[cfg(feature = "in_place")]
use crate::InPlaceTransformExecutor;

#[cfg(feature = "in_place")]
impl<
    T: Copy + PointeeSizeExpressible + 'static + Default,
    const SRC_LAYOUT: u8,
    const DST_LAYOUT: u8,
    const PRECISION: i32,
> InPlaceTransformExecutor<T> for TransformShaperQ2_13OptSse<T, SRC_LAYOUT, DST_LAYOUT, PRECISION>
where
    u32: AsPrimitive<T>,
{
    fn transform(&self, in_out: &mut [T]) -> Result<(), CmsError> {
        unsafe { self.transform_in_place_impl(in_out) }
    }
}
