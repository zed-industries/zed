/*
 * // Copyright (c) Radzivon Bartoshyk 4/2025. All rights reserved.
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
#[cfg(feature = "in_place")]
use crate::InPlaceTransformExecutor;
use crate::conversions::TransformMatrixShaper;
use crate::conversions::rgbxyz::{
    TransformMatrixShaperOptimized, make_rgb_xyz_rgb_transform, make_rgb_xyz_rgb_transform_opt,
};
use crate::conversions::rgbxyz_fixed::make_rgb_xyz_q2_13_opt;
use crate::{CmsError, Layout, TransformExecutor, TransformOptions};
use num_traits::AsPrimitive;
use std::sync::Arc;

const FIXED_POINT_SCALE: i32 = 13; // Q2.13;

pub(crate) trait RgbXyzFactory<T: Clone + AsPrimitive<usize> + Default> {
    fn make_transform<const LINEAR_CAP: usize, const GAMMA_LUT: usize, const BIT_DEPTH: usize>(
        src_layout: Layout,
        dst_layout: Layout,
        profile: TransformMatrixShaper<T, LINEAR_CAP>,
        transform_options: TransformOptions,
    ) -> Result<Arc<dyn TransformExecutor<T> + Send + Sync>, CmsError>;

    #[cfg(feature = "in_place")]
    fn make_in_place_transform<
        const LINEAR_CAP: usize,
        const GAMMA_LUT: usize,
        const BIT_DEPTH: usize,
    >(
        layout: Layout,
        profile: TransformMatrixShaper<T, LINEAR_CAP>,
        transform_options: TransformOptions,
    ) -> Result<Arc<dyn InPlaceTransformExecutor<T> + Send + Sync>, CmsError>;
}

pub(crate) trait RgbXyzFactoryOpt<T: Clone + AsPrimitive<usize> + Default> {
    fn make_optimized_transform<
        const LINEAR_CAP: usize,
        const GAMMA_LUT: usize,
        const BIT_DEPTH: usize,
    >(
        src_layout: Layout,
        dst_layout: Layout,
        profile: TransformMatrixShaperOptimized<T, LINEAR_CAP>,
        transform_options: TransformOptions,
    ) -> Result<Arc<dyn TransformExecutor<T> + Send + Sync>, CmsError>;

    #[cfg(feature = "in_place")]
    fn make_in_place_optimized_transform<
        const LINEAR_CAP: usize,
        const GAMMA_LUT: usize,
        const BIT_DEPTH: usize,
    >(
        layout: Layout,
        profile: TransformMatrixShaperOptimized<T, LINEAR_CAP>,
        transform_options: TransformOptions,
    ) -> Result<Arc<dyn InPlaceTransformExecutor<T> + Send + Sync>, CmsError>;
}

impl RgbXyzFactory<u16> for u16 {
    fn make_transform<const LINEAR_CAP: usize, const GAMMA_LUT: usize, const BIT_DEPTH: usize>(
        src_layout: Layout,
        dst_layout: Layout,
        profile: TransformMatrixShaper<u16, LINEAR_CAP>,
        _: TransformOptions,
    ) -> Result<Arc<dyn TransformExecutor<u16> + Send + Sync>, CmsError> {
        make_rgb_xyz_rgb_transform::<u16, LINEAR_CAP>(
            src_layout, dst_layout, profile, GAMMA_LUT, BIT_DEPTH,
        )
    }

    #[cfg(feature = "in_place")]
    fn make_in_place_transform<
        const LINEAR_CAP: usize,
        const GAMMA_LUT: usize,
        const BIT_DEPTH: usize,
    >(
        layout: Layout,
        profile: TransformMatrixShaper<u16, LINEAR_CAP>,
        _: TransformOptions,
    ) -> Result<Arc<dyn InPlaceTransformExecutor<u16> + Send + Sync>, CmsError> {
        use crate::conversions::rgbxyz::make_in_place_rgb_xyz_transform;
        make_in_place_rgb_xyz_transform::<u16, LINEAR_CAP>(layout, profile, GAMMA_LUT, BIT_DEPTH)
    }
}

impl RgbXyzFactory<f32> for f32 {
    fn make_transform<const LINEAR_CAP: usize, const GAMMA_LUT: usize, const BIT_DEPTH: usize>(
        src_layout: Layout,
        dst_layout: Layout,
        profile: TransformMatrixShaper<f32, LINEAR_CAP>,
        _: TransformOptions,
    ) -> Result<Arc<dyn TransformExecutor<f32> + Send + Sync>, CmsError> {
        make_rgb_xyz_rgb_transform::<f32, LINEAR_CAP>(
            src_layout, dst_layout, profile, GAMMA_LUT, BIT_DEPTH,
        )
    }

    #[cfg(feature = "in_place")]
    fn make_in_place_transform<
        const LINEAR_CAP: usize,
        const GAMMA_LUT: usize,
        const BIT_DEPTH: usize,
    >(
        layout: Layout,
        profile: TransformMatrixShaper<f32, LINEAR_CAP>,
        _: TransformOptions,
    ) -> Result<Arc<dyn InPlaceTransformExecutor<f32> + Send + Sync>, CmsError> {
        use crate::conversions::rgbxyz::make_in_place_rgb_xyz_transform;
        make_in_place_rgb_xyz_transform::<f32, LINEAR_CAP>(layout, profile, GAMMA_LUT, BIT_DEPTH)
    }
}

impl RgbXyzFactory<f64> for f64 {
    fn make_transform<const LINEAR_CAP: usize, const GAMMA_LUT: usize, const BIT_DEPTH: usize>(
        src_layout: Layout,
        dst_layout: Layout,
        profile: TransformMatrixShaper<f64, LINEAR_CAP>,
        _: TransformOptions,
    ) -> Result<Arc<dyn TransformExecutor<f64> + Send + Sync>, CmsError> {
        make_rgb_xyz_rgb_transform::<f64, LINEAR_CAP>(
            src_layout, dst_layout, profile, GAMMA_LUT, BIT_DEPTH,
        )
    }

    #[cfg(feature = "in_place")]
    fn make_in_place_transform<
        const LINEAR_CAP: usize,
        const GAMMA_LUT: usize,
        const BIT_DEPTH: usize,
    >(
        layout: Layout,
        profile: TransformMatrixShaper<f64, LINEAR_CAP>,
        _: TransformOptions,
    ) -> Result<Arc<dyn InPlaceTransformExecutor<f64> + Send + Sync>, CmsError> {
        use crate::conversions::rgbxyz::make_in_place_rgb_xyz_transform;
        make_in_place_rgb_xyz_transform::<f64, LINEAR_CAP>(layout, profile, GAMMA_LUT, BIT_DEPTH)
    }
}

impl RgbXyzFactory<u8> for u8 {
    fn make_transform<const LINEAR_CAP: usize, const GAMMA_LUT: usize, const BIT_DEPTH: usize>(
        src_layout: Layout,
        dst_layout: Layout,
        profile: TransformMatrixShaper<u8, LINEAR_CAP>,
        _: TransformOptions,
    ) -> Result<Arc<dyn TransformExecutor<u8> + Send + Sync>, CmsError> {
        make_rgb_xyz_rgb_transform::<u8, LINEAR_CAP>(src_layout, dst_layout, profile, GAMMA_LUT, 8)
    }

    #[cfg(feature = "in_place")]
    fn make_in_place_transform<
        const LINEAR_CAP: usize,
        const GAMMA_LUT: usize,
        const BIT_DEPTH: usize,
    >(
        layout: Layout,
        profile: TransformMatrixShaper<u8, LINEAR_CAP>,
        _: TransformOptions,
    ) -> Result<Arc<dyn InPlaceTransformExecutor<u8> + Send + Sync>, CmsError> {
        use crate::conversions::rgbxyz::make_in_place_rgb_xyz_transform;
        make_in_place_rgb_xyz_transform::<u8, LINEAR_CAP>(layout, profile, GAMMA_LUT, BIT_DEPTH)
    }
}

// Optimized factories

impl RgbXyzFactoryOpt<u16> for u16 {
    fn make_optimized_transform<
        const LINEAR_CAP: usize,
        const GAMMA_LUT: usize,
        const BIT_DEPTH: usize,
    >(
        src_layout: Layout,
        dst_layout: Layout,
        profile: TransformMatrixShaperOptimized<u16, LINEAR_CAP>,
        transform_options: TransformOptions,
    ) -> Result<Arc<dyn TransformExecutor<u16> + Send + Sync>, CmsError> {
        if BIT_DEPTH >= 12 && transform_options.prefer_fixed_point {
            #[cfg(all(target_arch = "aarch64", feature = "neon_shaper_fixed_point_paths"))]
            {
                if std::arch::is_aarch64_feature_detected!("rdm") {
                    use crate::conversions::rgbxyz_fixed::make_rgb_xyz_q1_30_opt;
                    return make_rgb_xyz_q1_30_opt::<u16, LINEAR_CAP, 30>(
                        src_layout, dst_layout, profile, GAMMA_LUT, BIT_DEPTH,
                    );
                }
            }
        }
        if BIT_DEPTH < 16 && transform_options.prefer_fixed_point {
            #[cfg(all(target_arch = "x86_64", feature = "avx_shaper_fixed_point_paths"))]
            {
                use crate::conversions::rgbxyz_fixed::make_rgb_xyz_q2_13_transform_avx2_opt;
                if std::arch::is_x86_feature_detected!("avx2") {
                    return make_rgb_xyz_q2_13_transform_avx2_opt::<
                        u16,
                        LINEAR_CAP,
                        FIXED_POINT_SCALE,
                    >(
                        src_layout, dst_layout, profile, GAMMA_LUT, BIT_DEPTH
                    );
                }
            }
            #[cfg(all(
                any(target_arch = "x86", target_arch = "x86_64"),
                feature = "sse_shaper_fixed_point_paths"
            ))]
            {
                use crate::conversions::rgbxyz_fixed::make_rgb_xyz_q2_13_transform_sse_41_opt;
                if std::arch::is_x86_feature_detected!("sse4.1") {
                    return make_rgb_xyz_q2_13_transform_sse_41_opt::<
                        u16,
                        LINEAR_CAP,
                        FIXED_POINT_SCALE,
                    >(
                        src_layout, dst_layout, profile, GAMMA_LUT, BIT_DEPTH
                    );
                }
            }
            #[cfg(all(target_arch = "aarch64", feature = "neon"))]
            {
                return make_rgb_xyz_q2_13_opt::<u16, LINEAR_CAP, FIXED_POINT_SCALE>(
                    src_layout, dst_layout, profile, GAMMA_LUT, BIT_DEPTH,
                );
            }
        }
        make_rgb_xyz_rgb_transform_opt::<u16, LINEAR_CAP>(
            src_layout, dst_layout, profile, GAMMA_LUT, BIT_DEPTH,
        )
    }

    #[cfg(feature = "in_place")]
    fn make_in_place_optimized_transform<
        const LINEAR_CAP: usize,
        const GAMMA_LUT: usize,
        const BIT_DEPTH: usize,
    >(
        layout: Layout,
        profile: TransformMatrixShaperOptimized<u16, LINEAR_CAP>,
        transform_options: TransformOptions,
    ) -> Result<Arc<dyn InPlaceTransformExecutor<u16> + Send + Sync>, CmsError> {
        if transform_options.prefer_fixed_point && BIT_DEPTH < 16 {
            #[cfg(all(
                target_arch = "aarch64",
                feature = "in_place",
                feature = "neon_shaper_fixed_point_paths"
            ))]
            {
                use crate::conversions::rgbxyz::make_rgb_xyz_in_place_transform_q2_13_opt;
                return make_rgb_xyz_in_place_transform_q2_13_opt::<
                    u16,
                    LINEAR_CAP,
                    FIXED_POINT_SCALE,
                >(layout, profile, GAMMA_LUT, BIT_DEPTH);
            }

            #[cfg(all(
                target_arch = "x86_64",
                feature = "in_place",
                feature = "avx_shaper_fixed_point_paths"
            ))]
            {
                if std::arch::is_x86_feature_detected!("avx2") {
                    use crate::conversions::rgbxyz::make_avx_rgb_xyz_in_place_transform_q2_13_opt;
                    return make_avx_rgb_xyz_in_place_transform_q2_13_opt::<
                        u16,
                        LINEAR_CAP,
                        FIXED_POINT_SCALE,
                    >(layout, profile, GAMMA_LUT, BIT_DEPTH);
                }
            }

            #[cfg(all(
                any(target_arch = "x86_64", target_arch = "x86"),
                feature = "in_place",
                feature = "sse_shaper_fixed_point_paths"
            ))]
            {
                if std::arch::is_x86_feature_detected!("sse4.1") {
                    use crate::conversions::rgbxyz::make_sse_rgb_xyz_in_place_transform_q2_13_opt;
                    return make_sse_rgb_xyz_in_place_transform_q2_13_opt::<
                        u16,
                        LINEAR_CAP,
                        FIXED_POINT_SCALE,
                    >(layout, profile, GAMMA_LUT, BIT_DEPTH);
                }
            }
        }
        use crate::conversions::rgbxyz::make_rgb_xyz_in_place_transform_opt;
        make_rgb_xyz_in_place_transform_opt::<u16, LINEAR_CAP>(
            layout, profile, GAMMA_LUT, BIT_DEPTH,
        )
    }
}

impl RgbXyzFactoryOpt<f32> for f32 {
    fn make_optimized_transform<
        const LINEAR_CAP: usize,
        const GAMMA_LUT: usize,
        const BIT_DEPTH: usize,
    >(
        src_layout: Layout,
        dst_layout: Layout,
        profile: TransformMatrixShaperOptimized<f32, LINEAR_CAP>,
        transform_options: TransformOptions,
    ) -> Result<Arc<dyn TransformExecutor<f32> + Send + Sync>, CmsError> {
        if transform_options.prefer_fixed_point {
            #[cfg(all(target_arch = "x86_64", feature = "avx_shaper_fixed_point_paths"))]
            {
                use crate::conversions::rgbxyz_fixed::make_rgb_xyz_q2_13_transform_avx2_opt;
                if std::arch::is_x86_feature_detected!("avx2") {
                    return make_rgb_xyz_q2_13_transform_avx2_opt::<
                        f32,
                        LINEAR_CAP,
                        FIXED_POINT_SCALE,
                    >(
                        src_layout, dst_layout, profile, GAMMA_LUT, BIT_DEPTH
                    );
                }
            }
            #[cfg(all(
                any(target_arch = "x86", target_arch = "x86_64"),
                feature = "sse_shaper_fixed_point_paths"
            ))]
            {
                use crate::conversions::rgbxyz_fixed::make_rgb_xyz_q2_13_transform_sse_41_opt;
                if std::arch::is_x86_feature_detected!("sse4.1") {
                    return make_rgb_xyz_q2_13_transform_sse_41_opt::<
                        f32,
                        LINEAR_CAP,
                        FIXED_POINT_SCALE,
                    >(
                        src_layout, dst_layout, profile, GAMMA_LUT, BIT_DEPTH
                    );
                }
            }
            #[cfg(all(target_arch = "aarch64", feature = "neon_shaper_fixed_point_paths"))]
            {
                return if std::arch::is_aarch64_feature_detected!("rdm") {
                    use crate::conversions::rgbxyz_fixed::make_rgb_xyz_q1_30_opt;
                    make_rgb_xyz_q1_30_opt::<f32, LINEAR_CAP, 30>(
                        src_layout, dst_layout, profile, GAMMA_LUT, BIT_DEPTH,
                    )
                } else {
                    make_rgb_xyz_q2_13_opt::<f32, LINEAR_CAP, FIXED_POINT_SCALE>(
                        src_layout, dst_layout, profile, GAMMA_LUT, BIT_DEPTH,
                    )
                };
            }
        }
        make_rgb_xyz_rgb_transform_opt::<f32, LINEAR_CAP>(
            src_layout, dst_layout, profile, GAMMA_LUT, BIT_DEPTH,
        )
    }

    #[cfg(feature = "in_place")]
    fn make_in_place_optimized_transform<
        const LINEAR_CAP: usize,
        const GAMMA_LUT: usize,
        const BIT_DEPTH: usize,
    >(
        layout: Layout,
        profile: TransformMatrixShaperOptimized<f32, LINEAR_CAP>,
        _: TransformOptions,
    ) -> Result<Arc<dyn InPlaceTransformExecutor<f32> + Send + Sync>, CmsError> {
        use crate::conversions::rgbxyz::make_rgb_xyz_in_place_transform_opt;
        make_rgb_xyz_in_place_transform_opt::<f32, LINEAR_CAP>(
            layout, profile, GAMMA_LUT, BIT_DEPTH,
        )
    }
}

impl RgbXyzFactoryOpt<f64> for f64 {
    fn make_optimized_transform<
        const LINEAR_CAP: usize,
        const GAMMA_LUT: usize,
        const BIT_DEPTH: usize,
    >(
        src_layout: Layout,
        dst_layout: Layout,
        profile: TransformMatrixShaperOptimized<f64, LINEAR_CAP>,
        transform_options: TransformOptions,
    ) -> Result<Arc<dyn TransformExecutor<f64> + Send + Sync>, CmsError> {
        if transform_options.prefer_fixed_point {
            #[cfg(all(target_arch = "aarch64", feature = "neon_shaper_fixed_point_paths"))]
            {
                if std::arch::is_aarch64_feature_detected!("rdm") {
                    use crate::conversions::rgbxyz_fixed::make_rgb_xyz_q1_30_opt;
                    return make_rgb_xyz_q1_30_opt::<f64, LINEAR_CAP, 30>(
                        src_layout, dst_layout, profile, GAMMA_LUT, BIT_DEPTH,
                    );
                }
            }
        }
        make_rgb_xyz_rgb_transform_opt::<f64, LINEAR_CAP>(
            src_layout, dst_layout, profile, GAMMA_LUT, BIT_DEPTH,
        )
    }

    #[cfg(feature = "in_place")]
    fn make_in_place_optimized_transform<
        const LINEAR_CAP: usize,
        const GAMMA_LUT: usize,
        const BIT_DEPTH: usize,
    >(
        layout: Layout,
        profile: TransformMatrixShaperOptimized<f64, LINEAR_CAP>,
        _: TransformOptions,
    ) -> Result<Arc<dyn InPlaceTransformExecutor<f64> + Send + Sync>, CmsError> {
        use crate::conversions::rgbxyz::make_rgb_xyz_in_place_transform_opt;
        make_rgb_xyz_in_place_transform_opt::<f64, LINEAR_CAP>(
            layout, profile, GAMMA_LUT, BIT_DEPTH,
        )
    }
}

impl RgbXyzFactoryOpt<u8> for u8 {
    fn make_optimized_transform<
        const LINEAR_CAP: usize,
        const GAMMA_LUT: usize,
        const BIT_DEPTH: usize,
    >(
        src_layout: Layout,
        dst_layout: Layout,
        profile: TransformMatrixShaperOptimized<u8, LINEAR_CAP>,
        transform_options: TransformOptions,
    ) -> Result<Arc<dyn TransformExecutor<u8> + Send + Sync>, CmsError> {
        if transform_options.prefer_fixed_point {
            #[cfg(all(target_arch = "x86_64", feature = "avx512_shaper_fixed_point_paths"))]
            {
                use crate::conversions::rgbxyz_fixed::make_rgb_xyz_q2_13_transform_avx512_opt;
                if std::arch::is_x86_feature_detected!("avx512bw")
                    && std::arch::is_x86_feature_detected!("avx512vl")
                {
                    return make_rgb_xyz_q2_13_transform_avx512_opt::<
                        u8,
                        LINEAR_CAP,
                        FIXED_POINT_SCALE,
                    >(src_layout, dst_layout, profile, GAMMA_LUT, 8);
                }
            }
            #[cfg(all(target_arch = "x86_64", feature = "avx_shaper_fixed_point_paths"))]
            {
                use crate::conversions::rgbxyz_fixed::make_rgb_xyz_q2_13_transform_avx2_opt;
                if std::arch::is_x86_feature_detected!("avx2") {
                    return make_rgb_xyz_q2_13_transform_avx2_opt::<
                        u8,
                        LINEAR_CAP,
                        FIXED_POINT_SCALE,
                    >(src_layout, dst_layout, profile, GAMMA_LUT, 8);
                }
            }
            #[cfg(all(
                any(target_arch = "x86", target_arch = "x86_64"),
                feature = "sse_shaper_fixed_point_paths"
            ))]
            {
                use crate::conversions::rgbxyz_fixed::make_rgb_xyz_q2_13_transform_sse_41_opt;
                if std::arch::is_x86_feature_detected!("sse4.1") {
                    return make_rgb_xyz_q2_13_transform_sse_41_opt::<
                        u8,
                        LINEAR_CAP,
                        FIXED_POINT_SCALE,
                    >(src_layout, dst_layout, profile, GAMMA_LUT, 8);
                }
            }
            make_rgb_xyz_q2_13_opt::<u8, LINEAR_CAP, FIXED_POINT_SCALE>(
                src_layout, dst_layout, profile, GAMMA_LUT, 8,
            )
        } else {
            make_rgb_xyz_rgb_transform_opt::<u8, LINEAR_CAP>(
                src_layout, dst_layout, profile, GAMMA_LUT, 8,
            )
        }
    }

    #[cfg(feature = "in_place")]
    fn make_in_place_optimized_transform<
        const LINEAR_CAP: usize,
        const GAMMA_LUT: usize,
        const BIT_DEPTH: usize,
    >(
        layout: Layout,
        profile: TransformMatrixShaperOptimized<u8, LINEAR_CAP>,
        transform_options: TransformOptions,
    ) -> Result<Arc<dyn InPlaceTransformExecutor<u8> + Send + Sync>, CmsError> {
        if transform_options.prefer_fixed_point {
            #[cfg(all(
                target_arch = "aarch64",
                feature = "in_place",
                feature = "neon_shaper_fixed_point_paths"
            ))]
            {
                use crate::conversions::rgbxyz::make_rgb_xyz_in_place_transform_q2_13_opt;
                return make_rgb_xyz_in_place_transform_q2_13_opt::<
                    u8,
                    LINEAR_CAP,
                    FIXED_POINT_SCALE,
                >(layout, profile, GAMMA_LUT, 8);
            }

            #[cfg(all(
                target_arch = "x86_64",
                feature = "in_place",
                feature = "avx_shaper_fixed_point_paths"
            ))]
            {
                if std::arch::is_x86_feature_detected!("avx2") {
                    use crate::conversions::rgbxyz::make_avx_rgb_xyz_in_place_transform_q2_13_opt;
                    return make_avx_rgb_xyz_in_place_transform_q2_13_opt::<
                        u8,
                        LINEAR_CAP,
                        FIXED_POINT_SCALE,
                    >(layout, profile, GAMMA_LUT, 8);
                }
            }

            #[cfg(all(
                any(target_arch = "x86_64", target_arch = "x86"),
                feature = "in_place",
                feature = "sse_shaper_fixed_point_paths"
            ))]
            {
                if std::arch::is_x86_feature_detected!("sse4.1") {
                    use crate::conversions::rgbxyz::make_sse_rgb_xyz_in_place_transform_q2_13_opt;
                    return make_sse_rgb_xyz_in_place_transform_q2_13_opt::<
                        u8,
                        LINEAR_CAP,
                        FIXED_POINT_SCALE,
                    >(layout, profile, GAMMA_LUT, 8);
                }
            }
        }
        use crate::conversions::rgbxyz::make_rgb_xyz_in_place_transform_opt;
        make_rgb_xyz_in_place_transform_opt::<u8, LINEAR_CAP>(layout, profile, GAMMA_LUT, 8)
    }
}
