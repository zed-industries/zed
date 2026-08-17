/*
 * Copyright (C) 2010, Google Inc. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1.  Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2.  Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY APPLE INC. AND ITS CONTRIBUTORS ``AS IS'' AND
 * ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
 * ARE DISCLAIMED. IN NO EVENT SHALL APPLE INC. OR ITS CONTRIBUTORS BE LIABLE
 * FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
 * DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
 * SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER
 * CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT
 * LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY
 * OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH
 * DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_AUDIO_VECTOR_MATH_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_AUDIO_VECTOR_MATH_H_

#include <cstddef>
#include "third_party/blink/renderer/platform/audio/audio_array.h"
#include "third_party/blink/renderer/platform/platform_export.h"

// Defines the interface for several vector math functions whose implementation
// will ideally be optimized.

namespace blink::vector_math {

// Direct vector convolution:
//
// dest[k*dest_stride] =
//     sum(source[(k+m)*source_stride]*filter[m*filter_stride]) for all m
PLATFORM_EXPORT void Conv(const float* source_p,
                          int source_stride,
                          const float* filter_p,
                          int filter_stride,
                          float* dest_p,
                          int dest_stride,
                          uint32_t frames_to_process,
                          size_t filter_size,
                          const AudioFloatArray* prepared_filter);

// Prepare filter for Conv for faster processing.
PLATFORM_EXPORT void PrepareFilterForConv(const float* filter_p,
                                          int filter_stride,
                                          size_t filter_size,
                                          AudioFloatArray* prepared_filter);

// Vector scalar multiply and then add.
//
// dest[k*dest_stride] += scale * source[k*source_stride]
//
// Note: Mac has a different implementation, and it may produce slightly
// different results from what linux and windows would do.
PLATFORM_EXPORT void Vsma(const float* source_p,
                          int source_stride,
                          const float* scale,
                          float* dest_p,
                          int dest_stride,
                          uint32_t frames_to_process);

PLATFORM_EXPORT void Vsma(const float* source_p,
                          int source_stride,
                          float scale,
                          float* dest_p,
                          int dest_stride,
                          uint32_t frames_to_process);

// Vector scalar multiply:
//
// dest[k*dest_stride] = scale * source[k*source_stride]
PLATFORM_EXPORT void Vsmul(const float* source_p,
                           int source_stride,
                           const float* scale,
                           float* dest_p,
                           int dest_stride,
                           uint32_t frames_to_process);

PLATFORM_EXPORT void Vsmul(const float* source_p,
                           int source_stride,
                           float scale,
                           float* dest_p,
                           int dest_stride,
                           uint32_t frames_to_process);

PLATFORM_EXPORT void Vsadd(const float* source_p,
                           int source_stride,
                           const float* addend,
                           float* dest_p,
                           int dest_stride,
                           uint32_t frames_to_process);

PLATFORM_EXPORT void Vsadd(const float* source_p,
                           int source_stride,
                           float addend,
                           float* dest_p,
                           int dest_stride,
                           uint32_t frames_to_process);
// Vector add:
//
// dest[k*dest_stride] = source1[k*source_stride1] + source2[k*source_stride2]
PLATFORM_EXPORT void Vadd(const float* source1p,
                          int source_stride1,
                          const float* source2p,
                          int source_stride2,
                          float* dest_p,
                          int dest_stride,
                          uint32_t frames_to_process);

// Vector subtract:
//
// dest[k*dest_stride] = source1[k*source_stride1] - source2[k*source_stride2]
PLATFORM_EXPORT void Vsub(const float* source1p,
                          int source_stride1,
                          const float* source2p,
                          int source_stride2,
                          float* dest_p,
                          int dest_stride,
                          uint32_t frames_to_process);

// Finds the maximum magnitude of a float vector:
//
// max = max(abs(source[k*source_stride])) for all k.
PLATFORM_EXPORT void Vmaxmgv(const float* source_p,
                             int source_stride,
                             float* max_p,
                             uint32_t frames_to_process);

// Sums the squares of a float vector's elements:
//
// sum = sum(source[k*source_stride]^2, k = 0, frames_to_process);
PLATFORM_EXPORT void Vsvesq(const float* source_p,
                            int source_stride,
                            float* sum_p,
                            uint32_t frames_to_process);

// For an element-by-element multiply of two float vectors:
//
// dest[k*dest_stride] = source1[k*source_stride1] * source2[k*source_stride2]
PLATFORM_EXPORT void Vmul(const float* source1p,
                          int source_stride1,
                          const float* source2p,
                          int source_stride2,
                          float* dest_p,
                          int dest_stride,
                          uint32_t frames_to_process);

// Multiplies two complex vectors.  Complex version of Vmul where |rea1p| and
// |imag1p| forms the real and complex components of source1; |real2p| and
// |imag2p| the components of source2, and |real_dest_p| and |imag_dest_p|, the
// components of the destination.
PLATFORM_EXPORT void Zvmul(const float* real1p,
                           const float* imag1p,
                           const float* real2p,
                           const float* imag2p,
                           float* real_dest_p,
                           float* imag_dest_p,
                           uint32_t frames_to_process);

// Copies elements while clipping values to the threshold inputs.
//
// dest[k*dest_stride] = clip(source[k*source_stride], low, high)
//
// where y = clip(x, low, high) = max(low, min(x, high)), effectively making
// low <= y <= high.
PLATFORM_EXPORT void Vclip(const float* source_p,
                           int source_stride,
                           const float* low_threshold_p,
                           const float* high_threshold_p,
                           float* dest_p,
                           int dest_stride,
                           uint32_t frames_to_process);

PLATFORM_EXPORT void Vclip(const float* source_p,
                           int source_stride,
                           float low_threshold_p,
                           float high_threshold_p,
                           float* dest_p,
                           int dest_stride,
                           uint32_t frames_to_process);

}  // namespace blink::vector_math

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_AUDIO_VECTOR_MATH_H_
