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
use crate::Layout;
use crate::matrix::Matrix3;
use crate::{CmsError, TransformExecutor};
use num_traits::AsPrimitive;
use std::sync::Arc;

/// Fixed point conversion Q2.13
#[allow(dead_code)]
pub(crate) struct TransformMatrixShaperFp<R, T> {
    pub(crate) r_linear: Vec<R>,
    pub(crate) g_linear: Vec<R>,
    pub(crate) b_linear: Vec<R>,
    pub(crate) r_gamma: Box<[T; 65536]>,
    pub(crate) g_gamma: Box<[T; 65536]>,
    pub(crate) b_gamma: Box<[T; 65536]>,
    pub(crate) adaptation_matrix: Matrix3<i16>,
}

/// Fixed point conversion Q2.13
///
/// Optimized routine for *all same curves* matrix shaper.
pub(crate) struct TransformMatrixShaperFixedPointOpt<R, W, T, const LINEAR_CAP: usize> {
    pub(crate) linear: Box<[R; LINEAR_CAP]>,
    pub(crate) gamma: Box<[T; 65536]>,
    pub(crate) adaptation_matrix: Matrix3<W>,
}

/// Fixed point conversion Q2.13
///
/// Optimized routine for *all same curves* matrix shaper.
#[allow(dead_code)]
pub(crate) struct TransformMatrixShaperFpOptVec<R, W, T> {
    pub(crate) linear: Vec<R>,
    pub(crate) gamma: Box<[T; 65536]>,
    pub(crate) adaptation_matrix: Matrix3<W>,
}

#[allow(unused)]
struct TransformMatrixShaperQ2_13Optimized<
    T: Copy,
    const SRC_LAYOUT: u8,
    const DST_LAYOUT: u8,
    const LINEAR_CAP: usize,
    const PRECISION: i32,
> {
    pub(crate) profile: TransformMatrixShaperFixedPointOpt<i16, i16, T, LINEAR_CAP>,
    pub(crate) bit_depth: usize,
    pub(crate) gamma_lut: usize,
}

#[allow(unused)]
impl<
    T: Clone + PointeeSizeExpressible + Copy + Default + 'static,
    const SRC_LAYOUT: u8,
    const DST_LAYOUT: u8,
    const LINEAR_CAP: usize,
    const PRECISION: i32,
> TransformExecutor<T>
    for TransformMatrixShaperQ2_13Optimized<T, SRC_LAYOUT, DST_LAYOUT, LINEAR_CAP, PRECISION>
where
    u32: AsPrimitive<T>,
{
    fn transform(&self, src: &[T], dst: &mut [T]) -> Result<(), CmsError> {
        let src_cn = Layout::from(SRC_LAYOUT);
        let dst_cn = Layout::from(DST_LAYOUT);
        let src_channels = src_cn.channels();
        let dst_channels = dst_cn.channels();

        if src.len() / src_channels != dst.len() / dst_channels {
            return Err(CmsError::LaneSizeMismatch);
        }
        if src.len() % src_channels != 0 {
            return Err(CmsError::LaneMultipleOfChannels);
        }
        if dst.len() % dst_channels != 0 {
            return Err(CmsError::LaneMultipleOfChannels);
        }

        let transform = self.profile.adaptation_matrix;
        let max_colors: T = ((1 << self.bit_depth as u32) - 1u32).as_();
        let rnd: i32 = (1i32 << (PRECISION - 1));

        let v_gamma_max = self.gamma_lut as i32 - 1;

        for (src, dst) in src
            .chunks_exact(src_channels)
            .zip(dst.chunks_exact_mut(dst_channels))
        {
            let r = self.profile.linear[src[src_cn.r_i()]._as_usize()];
            let g = self.profile.linear[src[src_cn.g_i()]._as_usize()];
            let b = self.profile.linear[src[src_cn.b_i()]._as_usize()];
            let a = if src_channels == 4 {
                src[src_cn.a_i()]
            } else {
                max_colors
            };

            let new_r = r as i32 * transform.v[0][0] as i32
                + g as i32 * transform.v[0][1] as i32
                + b as i32 * transform.v[0][2] as i32
                + rnd;

            let r_q2_13 = (new_r >> PRECISION).min(v_gamma_max).max(0) as u16;

            let new_g = r as i32 * transform.v[1][0] as i32
                + g as i32 * transform.v[1][1] as i32
                + b as i32 * transform.v[1][2] as i32
                + rnd;

            let g_q2_13 = (new_g >> PRECISION).min(v_gamma_max).max(0) as u16;

            let new_b = r as i32 * transform.v[2][0] as i32
                + g as i32 * transform.v[2][1] as i32
                + b as i32 * transform.v[2][2] as i32
                + rnd;

            let b_q2_13 = (new_b >> PRECISION).min(v_gamma_max).max(0) as u16;

            dst[dst_cn.r_i()] = self.profile.gamma[r_q2_13 as usize];
            dst[dst_cn.g_i()] = self.profile.gamma[g_q2_13 as usize];
            dst[dst_cn.b_i()] = self.profile.gamma[b_q2_13 as usize];
            if dst_channels == 4 {
                dst[dst_cn.a_i()] = a;
            }
        }
        Ok(())
    }
}

#[allow(unused_macros)]
macro_rules! create_rgb_xyz_dependant_q2_13_executor {
    ($dep_name: ident, $dependant: ident, $resolution: ident, $shaper: ident) => {
        pub(crate) fn $dep_name<
            T: Clone + Send + Sync + AsPrimitive<usize> + Default + PointeeSizeExpressible,
            const LINEAR_CAP: usize,
            const PRECISION: i32,
        >(
            src_layout: Layout,
            dst_layout: Layout,
            profile: $shaper<T, LINEAR_CAP>,
            gamma_lut: usize,
            bit_depth: usize,
        ) -> Result<Arc<dyn TransformExecutor<T> + Send + Sync>, CmsError>
        where
            u32: AsPrimitive<T>,
        {
            let q2_13_profile =
                profile.to_q2_13_n::<$resolution, PRECISION, LINEAR_CAP>(gamma_lut, bit_depth);
            if (src_layout == Layout::Rgba) && (dst_layout == Layout::Rgba) {
                return Ok(Arc::new($dependant::<
                    T,
                    { Layout::Rgba as u8 },
                    { Layout::Rgba as u8 },
                    LINEAR_CAP,
                    PRECISION,
                > {
                    profile: q2_13_profile,
                    bit_depth,
                    gamma_lut,
                }));
            } else if (src_layout == Layout::Rgb) && (dst_layout == Layout::Rgba) {
                return Ok(Arc::new($dependant::<
                    T,
                    { Layout::Rgb as u8 },
                    { Layout::Rgba as u8 },
                    LINEAR_CAP,
                    PRECISION,
                > {
                    profile: q2_13_profile,
                    bit_depth,
                    gamma_lut,
                }));
            } else if (src_layout == Layout::Rgba) && (dst_layout == Layout::Rgb) {
                return Ok(Arc::new($dependant::<
                    T,
                    { Layout::Rgba as u8 },
                    { Layout::Rgb as u8 },
                    LINEAR_CAP,
                    PRECISION,
                > {
                    profile: q2_13_profile,
                    bit_depth,
                    gamma_lut,
                }));
            } else if (src_layout == Layout::Rgb) && (dst_layout == Layout::Rgb) {
                return Ok(Arc::new($dependant::<
                    T,
                    { Layout::Rgb as u8 },
                    { Layout::Rgb as u8 },
                    LINEAR_CAP,
                    PRECISION,
                > {
                    profile: q2_13_profile,
                    bit_depth,
                    gamma_lut,
                }));
            }
            Err(CmsError::UnsupportedProfileConnection)
        }
    };
}

#[allow(unused_macros)]
macro_rules! create_rgb_xyz_dependant_q2_13_executor_fp {
    ($dep_name: ident, $dependant: ident, $resolution: ident, $shaper: ident) => {
        pub(crate) fn $dep_name<
            T: Clone + Send + Sync + AsPrimitive<usize> + Default + PointeeSizeExpressible,
            const LINEAR_CAP: usize,
            const PRECISION: i32,
        >(
            src_layout: Layout,
            dst_layout: Layout,
            profile: $shaper<T, LINEAR_CAP>,
            gamma_lut: usize,
            bit_depth: usize,
        ) -> Result<Arc<dyn TransformExecutor<T> + Send + Sync>, CmsError>
        where
            u32: AsPrimitive<T>,
        {
            let q2_13_profile = profile.to_q2_13_i::<$resolution, PRECISION>(gamma_lut, bit_depth);
            if (src_layout == Layout::Rgba) && (dst_layout == Layout::Rgba) {
                return Ok(Arc::new($dependant::<
                    T,
                    { Layout::Rgba as u8 },
                    { Layout::Rgba as u8 },
                    PRECISION,
                > {
                    profile: q2_13_profile,
                    bit_depth,
                    gamma_lut,
                }));
            } else if (src_layout == Layout::Rgb) && (dst_layout == Layout::Rgba) {
                return Ok(Arc::new($dependant::<
                    T,
                    { Layout::Rgb as u8 },
                    { Layout::Rgba as u8 },
                    PRECISION,
                > {
                    profile: q2_13_profile,
                    bit_depth,
                    gamma_lut,
                }));
            } else if (src_layout == Layout::Rgba) && (dst_layout == Layout::Rgb) {
                return Ok(Arc::new($dependant::<
                    T,
                    { Layout::Rgba as u8 },
                    { Layout::Rgb as u8 },
                    PRECISION,
                > {
                    profile: q2_13_profile,
                    bit_depth,
                    gamma_lut,
                }));
            } else if (src_layout == Layout::Rgb) && (dst_layout == Layout::Rgb) {
                return Ok(Arc::new($dependant::<
                    T,
                    { Layout::Rgb as u8 },
                    { Layout::Rgb as u8 },
                    PRECISION,
                > {
                    profile: q2_13_profile,
                    bit_depth,
                    gamma_lut,
                }));
            }
            Err(CmsError::UnsupportedProfileConnection)
        }
    };
}

#[cfg(all(target_arch = "aarch64", feature = "neon_shaper_fixed_point_paths"))]
macro_rules! create_rgb_xyz_dependant_q1_30_executor {
    ($dep_name: ident, $dependant: ident, $resolution: ident, $shaper: ident) => {
        pub(crate) fn $dep_name<
            T: Clone + Send + Sync + AsPrimitive<usize> + Default + PointeeSizeExpressible,
            const LINEAR_CAP: usize,
            const PRECISION: i32,
        >(
            src_layout: Layout,
            dst_layout: Layout,
            profile: $shaper<T, LINEAR_CAP>,
            gamma_lut: usize,
            bit_depth: usize,
        ) -> Result<Arc<dyn TransformExecutor<T> + Send + Sync>, CmsError>
        where
            u32: AsPrimitive<T>,
        {
            let q1_30_profile = profile.to_q1_30_n::<$resolution, PRECISION>(gamma_lut, bit_depth);
            if (src_layout == Layout::Rgba) && (dst_layout == Layout::Rgba) {
                return Ok(Arc::new($dependant::<
                    T,
                    { Layout::Rgba as u8 },
                    { Layout::Rgba as u8 },
                > {
                    profile: q1_30_profile,
                    gamma_lut,
                    bit_depth,
                }));
            } else if (src_layout == Layout::Rgb) && (dst_layout == Layout::Rgba) {
                return Ok(Arc::new($dependant::<
                    T,
                    { Layout::Rgb as u8 },
                    { Layout::Rgba as u8 },
                > {
                    profile: q1_30_profile,
                    gamma_lut,
                    bit_depth,
                }));
            } else if (src_layout == Layout::Rgba) && (dst_layout == Layout::Rgb) {
                return Ok(Arc::new($dependant::<
                    T,
                    { Layout::Rgba as u8 },
                    { Layout::Rgb as u8 },
                > {
                    profile: q1_30_profile,
                    gamma_lut,
                    bit_depth,
                }));
            } else if (src_layout == Layout::Rgb) && (dst_layout == Layout::Rgb) {
                return Ok(Arc::new($dependant::<
                    T,
                    { Layout::Rgb as u8 },
                    { Layout::Rgb as u8 },
                > {
                    profile: q1_30_profile,
                    gamma_lut,
                    bit_depth,
                }));
            }
            Err(CmsError::UnsupportedProfileConnection)
        }
    };
}

#[cfg(all(target_arch = "aarch64", feature = "neon_shaper_fixed_point_paths"))]
use crate::conversions::neon::{TransformShaperQ1_30NeonOpt, TransformShaperQ2_13NeonOpt};

#[cfg(all(target_arch = "aarch64", feature = "neon_shaper_fixed_point_paths"))]
create_rgb_xyz_dependant_q2_13_executor_fp!(
    make_rgb_xyz_q2_13_opt,
    TransformShaperQ2_13NeonOpt,
    i16,
    TransformMatrixShaperOptimized
);

#[cfg(all(target_arch = "aarch64", feature = "neon_shaper_fixed_point_paths"))]
create_rgb_xyz_dependant_q1_30_executor!(
    make_rgb_xyz_q1_30_opt,
    TransformShaperQ1_30NeonOpt,
    i32,
    TransformMatrixShaperOptimized
);

#[cfg(not(all(target_arch = "aarch64", feature = "neon_shaper_fixed_point_paths")))]
create_rgb_xyz_dependant_q2_13_executor!(
    make_rgb_xyz_q2_13_opt,
    TransformMatrixShaperQ2_13Optimized,
    i16,
    TransformMatrixShaperOptimized
);

#[cfg(all(
    any(target_arch = "x86", target_arch = "x86_64"),
    feature = "sse_shaper_fixed_point_paths"
))]
use crate::conversions::sse::TransformShaperQ2_13OptSse;

#[cfg(all(
    any(target_arch = "x86", target_arch = "x86_64"),
    feature = "sse_shaper_fixed_point_paths"
))]
create_rgb_xyz_dependant_q2_13_executor_fp!(
    make_rgb_xyz_q2_13_transform_sse_41_opt,
    TransformShaperQ2_13OptSse,
    i32,
    TransformMatrixShaperOptimized
);

#[cfg(all(target_arch = "x86_64", feature = "avx_shaper_fixed_point_paths"))]
use crate::conversions::avx::TransformShaperRgbQ2_13OptAvx;
use crate::conversions::rgbxyz::TransformMatrixShaperOptimized;
use crate::transform::PointeeSizeExpressible;

#[cfg(all(target_arch = "x86_64", feature = "avx_shaper_fixed_point_paths"))]
create_rgb_xyz_dependant_q2_13_executor_fp!(
    make_rgb_xyz_q2_13_transform_avx2_opt,
    TransformShaperRgbQ2_13OptAvx,
    i32,
    TransformMatrixShaperOptimized
);

#[cfg(all(target_arch = "x86_64", feature = "avx512_shaper_fixed_point_paths"))]
use crate::conversions::avx512::TransformShaperRgbQ2_13OptAvx512;

#[cfg(all(target_arch = "x86_64", feature = "avx512_shaper_fixed_point_paths"))]
create_rgb_xyz_dependant_q2_13_executor!(
    make_rgb_xyz_q2_13_transform_avx512_opt,
    TransformShaperRgbQ2_13OptAvx512,
    i32,
    TransformMatrixShaperOptimized
);
