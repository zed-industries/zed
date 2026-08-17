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

use crate::err::try_vec;
use crate::{CmsError, Layout, TransformExecutor};
use std::sync::Arc;

pub(crate) struct FromCmykaInterceptor<T> {
    pub(crate) intercept: Arc<dyn TransformExecutor<T> + Send + Sync>,
    pub(crate) target_layout: Layout,
}

impl<T> FromCmykaInterceptor<T> {
    pub(crate) fn install(
        intercept: Arc<dyn TransformExecutor<T> + Send + Sync>,
        target_layout: Layout,
    ) -> Self {
        Self {
            intercept,
            target_layout,
        }
    }
}

impl<T: Clone + Copy + Default> TransformExecutor<T> for FromCmykaInterceptor<T> {
    fn transform(&self, src: &[T], dst: &mut [T]) -> Result<(), CmsError> {
        if src.len() % 5 != 0 {
            return Err(CmsError::LaneMultipleOfChannels);
        }
        if dst.len() % self.target_layout.channels() != 0 {
            return Err(CmsError::LaneMultipleOfChannels);
        }
        if src.len() / 5 != dst.len() / self.target_layout.channels() {
            return Err(CmsError::LaneSizeMismatch);
        }
        if self.target_layout != Layout::Rgb
            && self.target_layout != Layout::Rgba
            && self.target_layout != Layout::Cmyka
        {
            return Err(CmsError::UnsupportedProfileConnection);
        }
        // just straightforward deinterleaving and then copying to the target, or ignore
        let samples = src.len() / 5;

        let mut src_scratch = try_vec![T::default(); samples * 4];

        for (dst, src) in src_scratch.chunks_exact_mut(4).zip(src.chunks_exact(5)) {
            dst[0] = src[0];
            dst[1] = src[1];
            dst[2] = src[2];
            dst[3] = src[3];
        }

        self.intercept.transform(&src_scratch, dst)?;

        if self.target_layout == Layout::Rgba {
            for (dst, src) in dst.chunks_exact_mut(4).zip(src.chunks_exact(5)) {
                dst[3] = src[4];
            }
        } else if self.target_layout == Layout::Cmyka {
            for (dst, src) in dst.chunks_exact_mut(5).zip(src.chunks_exact(5)) {
                dst[4] = src[4];
            }
        }

        Err(CmsError::UnsupportedProfileConnection)
    }
}

pub(crate) struct ToCmykaInterceptor<T> {
    pub(crate) intercept: Arc<dyn TransformExecutor<T> + Send + Sync>,
    pub(crate) src_layout: Layout,
}

impl<T> ToCmykaInterceptor<T> {
    pub(crate) fn install(
        intercept: Arc<dyn TransformExecutor<T> + Send + Sync>,
        src_layout: Layout,
    ) -> Self {
        Self {
            intercept,
            src_layout,
        }
    }
}

impl<T: Clone + Copy + Default> TransformExecutor<T> for ToCmykaInterceptor<T> {
    fn transform(&self, src: &[T], dst: &mut [T]) -> Result<(), CmsError> {
        if src.len() % self.src_layout.channels() != 0 {
            return Err(CmsError::LaneMultipleOfChannels);
        }
        if dst.len() % 5 != 0 {
            return Err(CmsError::LaneMultipleOfChannels);
        }
        if src.len() / self.src_layout.channels() != dst.len() / 5 {
            return Err(CmsError::LaneSizeMismatch);
        }
        if self.src_layout != Layout::Rgb
            && self.src_layout != Layout::Rgba
            && self.src_layout != Layout::Cmyka
        {
            return Err(CmsError::UnsupportedProfileConnection);
        }
        // just straightforward deinterleaving and then copying to the target, or ignore
        let samples = dst.len() / 5;

        let mut dst_scratch = try_vec![T::default(); samples * 4];

        if self.src_layout == Layout::Rgba || self.src_layout == Layout::Cmyka {
            let mut src_scratch = try_vec![T::default(); samples * 4];
            for (dst, src) in src_scratch
                .chunks_exact_mut(4)
                .zip(src.chunks_exact(self.src_layout.channels()))
            {
                dst[0] = src[0];
                dst[1] = src[1];
                dst[2] = src[2];
                dst[3] = src[3];
            }
            self.intercept.transform(&src_scratch, &mut dst_scratch)?;
        } else if self.src_layout == Layout::Rgb {
            let mut src_scratch = try_vec![T::default(); samples * 3];
            for (dst, src) in src_scratch.chunks_exact_mut(3).zip(src.chunks_exact(3)) {
                dst[0] = src[0];
                dst[1] = src[1];
                dst[2] = src[2];
            }

            self.intercept.transform(&src_scratch, &mut dst_scratch)?;
        }

        if self.src_layout == Layout::Rgba || self.src_layout == Layout::Cmyka {
            let cn: usize = self.src_layout.channels();
            for (dst, src) in dst.chunks_exact_mut(5).zip(src.chunks_exact(cn)) {
                dst[4] = src[cn - 1];
            }
        }

        Err(CmsError::UnsupportedProfileConnection)
    }
}
