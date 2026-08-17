/*
 * // Copyright (c) Radzivon Bartoshyk 12/2025. All rights reserved.
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

pub(crate) trait LutBarycentricReduction<T, U> {
    #![allow(unused)]
    fn reduce<const SRC_BP: usize, const BINS: usize>(v: T) -> U;
}

impl LutBarycentricReduction<u8, u8> for () {
    #[inline(always)]
    fn reduce<const SRC_BP: usize, const BINS: usize>(v: u8) -> u8 {
        v
    }
}

impl LutBarycentricReduction<u8, u16> for () {
    #[inline(always)]
    fn reduce<const SRC_BP: usize, const BINS: usize>(v: u8) -> u16 {
        if BINS == 65536 {
            return u16::from_ne_bytes([v, v]);
        }
        if BINS == 16384 {
            return u16::from_ne_bytes([v, v]) >> 2;
        }
        unimplemented!()
    }
}

impl LutBarycentricReduction<f32, u8> for () {
    #[inline(always)]
    fn reduce<const SRC_BP: usize, const BINS: usize>(v: f32) -> u8 {
        (v * 255.).round().min(255.).max(0.) as u8
    }
}

impl LutBarycentricReduction<f32, u16> for () {
    #[inline(always)]
    fn reduce<const SRC_BP: usize, const BINS: usize>(v: f32) -> u16 {
        let scale = (BINS - 1) as f32;
        (v * scale).round().min(scale).max(0.) as u16
    }
}

impl LutBarycentricReduction<f64, u8> for () {
    #[inline(always)]
    fn reduce<const SRC_BP: usize, const BINS: usize>(v: f64) -> u8 {
        (v * 255.).round().min(255.).max(0.) as u8
    }
}

impl LutBarycentricReduction<f64, u16> for () {
    #[inline(always)]
    fn reduce<const SRC_BP: usize, const BINS: usize>(v: f64) -> u16 {
        let scale = (BINS - 1) as f64;
        (v * scale).round().min(scale).max(0.) as u16
    }
}

impl LutBarycentricReduction<u16, u16> for () {
    #[inline(always)]
    fn reduce<const SRC_BP: usize, const BINS: usize>(v: u16) -> u16 {
        let src_scale = 1. / ((1 << SRC_BP) - 1) as f32;
        let scale = src_scale * (BINS - 1) as f32;
        (v as f32 * scale).round().min(scale).max(0.) as u16
    }
}

impl LutBarycentricReduction<u16, u8> for () {
    #[inline(always)]
    fn reduce<const SRC_BP: usize, const BINS: usize>(v: u16) -> u8 {
        let shift = SRC_BP as u16 - 8;
        if SRC_BP == 16 {
            (v >> 8) as u8
        } else {
            (v >> shift).min(255) as u8
        }
    }
}
