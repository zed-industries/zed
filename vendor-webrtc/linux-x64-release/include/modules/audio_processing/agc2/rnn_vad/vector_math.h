/*
 *  Copyright (c) 2020 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_AUDIO_PROCESSING_AGC2_RNN_VAD_VECTOR_MATH_H_
#define MODULES_AUDIO_PROCESSING_AGC2_RNN_VAD_VECTOR_MATH_H_

// Defines WEBRTC_ARCH_X86_FAMILY, used below.
#include "rtc_base/system/arch.h"

#if defined(WEBRTC_HAS_NEON)
#include <arm_neon.h>
#endif
#if defined(WEBRTC_ARCH_X86_FAMILY)
#include <emmintrin.h>
#endif

#include <numeric>

#include "api/array_view.h"
#include "modules/audio_processing/agc2/cpu_features.h"
#include "rtc_base/checks.h"
#include "rtc_base/numerics/safe_conversions.h"
#include "rtc_base/system/arch.h"

namespace webrtc {
namespace rnn_vad {

// Provides optimizations for mathematical operations having vectors as
// operand(s).
class VectorMath {
 public:
  explicit VectorMath(AvailableCpuFeatures cpu_features)
      : cpu_features_(cpu_features) {}

  // Computes the dot product between two equally sized vectors.
  float DotProduct(ArrayView<const float> x, ArrayView<const float> y) const {
    RTC_DCHECK_EQ(x.size(), y.size());
#if defined(WEBRTC_ARCH_X86_FAMILY)
    if (cpu_features_.avx2) {
      return DotProductAvx2(x, y);
    } else if (cpu_features_.sse2) {
      __m128 accumulator = _mm_setzero_ps();
      constexpr int kBlockSizeLog2 = 2;
      constexpr int kBlockSize = 1 << kBlockSizeLog2;
      const int incomplete_block_index = (x.size() >> kBlockSizeLog2)
                                         << kBlockSizeLog2;
      for (int i = 0; i < incomplete_block_index; i += kBlockSize) {
        RTC_DCHECK_LE(i + kBlockSize, x.size());
        const __m128 x_i = _mm_loadu_ps(&x[i]);
        const __m128 y_i = _mm_loadu_ps(&y[i]);
        // Multiply-add.
        const __m128 z_j = _mm_mul_ps(x_i, y_i);
        accumulator = _mm_add_ps(accumulator, z_j);
      }
      // Reduce `accumulator` by addition.
      __m128 high = _mm_movehl_ps(accumulator, accumulator);
      accumulator = _mm_add_ps(accumulator, high);
      high = _mm_shuffle_ps(accumulator, accumulator, 1);
      accumulator = _mm_add_ps(accumulator, high);
      float dot_product = _mm_cvtss_f32(accumulator);
      // Add the result for the last block if incomplete.
      for (int i = incomplete_block_index; i < dchecked_cast<int>(x.size());
           ++i) {
        dot_product += x[i] * y[i];
      }
      return dot_product;
    }
#elif defined(WEBRTC_HAS_NEON) && defined(WEBRTC_ARCH_ARM64)
    if (cpu_features_.neon) {
      float32x4_t accumulator = vdupq_n_f32(0.f);
      constexpr int kBlockSizeLog2 = 2;
      constexpr int kBlockSize = 1 << kBlockSizeLog2;
      const int incomplete_block_index = (x.size() >> kBlockSizeLog2)
                                         << kBlockSizeLog2;
      for (int i = 0; i < incomplete_block_index; i += kBlockSize) {
        RTC_DCHECK_LE(i + kBlockSize, x.size());
        const float32x4_t x_i = vld1q_f32(&x[i]);
        const float32x4_t y_i = vld1q_f32(&y[i]);
        accumulator = vfmaq_f32(accumulator, x_i, y_i);
      }
      // Reduce `accumulator` by addition.
      const float32x2_t tmp =
          vpadd_f32(vget_low_f32(accumulator), vget_high_f32(accumulator));
      float dot_product = vget_lane_f32(vpadd_f32(tmp, vrev64_f32(tmp)), 0);
      // Add the result for the last block if incomplete.
      for (int i = incomplete_block_index;
           i < webrtc::dchecked_cast<int>(x.size()); ++i) {
        dot_product += x[i] * y[i];
      }
      return dot_product;
    }
#endif
    return std::inner_product(x.begin(), x.end(), y.begin(), 0.f);
  }

 private:
  float DotProductAvx2(ArrayView<const float> x,
                       ArrayView<const float> y) const;

  const AvailableCpuFeatures cpu_features_;
};

}  // namespace rnn_vad
}  // namespace webrtc

#endif  // MODULES_AUDIO_PROCESSING_AGC2_RNN_VAD_VECTOR_MATH_H_
