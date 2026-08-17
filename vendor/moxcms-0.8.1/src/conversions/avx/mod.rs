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
mod interpolator;
mod interpolator_q0_15;
mod lut4_to_3;
mod lut4_to_3_q0_15;
mod rgb_xyz;
mod rgb_xyz_opt;
mod rgb_xyz_q2_13_opt;
mod t_lut3_to_3;
mod t_lut3_to_3_q0_15;

#[cfg(feature = "lut")]
use crate::conversions::interpolator::BarycentricWeight;
#[cfg(feature = "avx_luts")]
pub(crate) use lut4_to_3::AvxLut4x3Factory;
#[cfg(feature = "avx_shaper_paths")]
pub(crate) use rgb_xyz::TransformShaperRgbAvx;
#[cfg(feature = "avx_shaper_optimized_paths")]
pub(crate) use rgb_xyz_opt::TransformShaperRgbOptAvx;
#[cfg(feature = "avx_shaper_fixed_point_paths")]
pub(crate) use rgb_xyz_q2_13_opt::TransformShaperRgbQ2_13OptAvx;
#[cfg(feature = "avx_luts")]
pub(crate) use t_lut3_to_3::AvxLut3x3Factory;

// this is required to ensure that interpolator never goes out of bounds
#[cfg(feature = "lut")]
fn assert_barycentric_lut_size_precondition<R, const GRID_SIZE: usize>(
    lut: &[BarycentricWeight<R>],
) {
    let k = lut
        .iter()
        .max_by(|a, &b| a.x.cmp(&b.x))
        .map(|x| x.x)
        .unwrap_or(0);
    let b = lut
        .iter()
        .max_by(|a, &b| a.x_n.cmp(&b.x_n))
        .map(|x| x.x)
        .unwrap_or(0);
    let max_possible_product = (k as u32 * (GRID_SIZE as u32 * GRID_SIZE as u32)
        + k as u32 * GRID_SIZE as u32
        + k as u32) as usize;
    let max_possible_product1 = (b as u32 * (GRID_SIZE as u32 * GRID_SIZE as u32)
        + b as u32 * GRID_SIZE as u32
        + b as u32) as usize;
    let cube_size = GRID_SIZE * GRID_SIZE * GRID_SIZE;
    assert!(max_possible_product < cube_size);
    assert!(max_possible_product1 < cube_size);
}

#[repr(align(32), C)]
#[derive(Debug)]
#[allow(unused)]
pub(crate) struct AvxAlignedU16(pub(crate) [u16; 16]);
