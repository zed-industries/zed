/*
 *  Copyright (c) 2017 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_AUDIO_PROCESSING_AEC3_VECTOR_MATH_H_
#define MODULES_AUDIO_PROCESSING_AEC3_VECTOR_MATH_H_

// Defines WEBRTC_ARCH_X86_FAMILY, used below.
#include "rtc_base/system/arch.h"

#if defined(WEBRTC_HAS_NEON)
#include <arm_neon.h>
#endif
#if defined(WEBRTC_ARCH_X86_FAMILY)
#include <emmintrin.h>
#endif
#include <math.h>

#include <algorithm>
#include <array>
#include <functional>

#include "api/array_view.h"
#include "modules/audio_processing/aec3/aec3_common.h"
#include "rtc_base/checks.h"

namespace webrtc {
namespace aec3 {

// Provides optimizations for mathematical operations based on vectors.
class VectorMath {
 public:
  explicit VectorMath(Aec3Optimization optimization)
      : optimization_(optimization) {}

  // Elementwise square root.
  void SqrtAVX2(ArrayView<float> x);
  void Sqrt(ArrayView<float> x) {
    switch (optimization_) {
#if defined(WEBRTC_ARCH_X86_FAMILY)
      case Aec3Optimization::kSse2: {
        const int x_size = static_cast<int>(x.size());
        const int vector_limit = x_size >> 2;

        int j = 0;
        for (; j < vector_limit * 4; j += 4) {
          __m128 g = _mm_loadu_ps(&x[j]);
          g = _mm_sqrt_ps(g);
          _mm_storeu_ps(&x[j], g);
        }

        for (; j < x_size; ++j) {
          x[j] = sqrtf(x[j]);
        }
      } break;
      case Aec3Optimization::kAvx2:
        SqrtAVX2(x);
        break;
#endif
#if defined(WEBRTC_HAS_NEON)
      case Aec3Optimization::kNeon: {
        const int x_size = static_cast<int>(x.size());
        const int vector_limit = x_size >> 2;

        int j = 0;
        for (; j < vector_limit * 4; j += 4) {
          float32x4_t g = vld1q_f32(&x[j]);
#if !defined(WEBRTC_ARCH_ARM64)
          float32x4_t y = vrsqrteq_f32(g);

          // Code to handle sqrt(0).
          // If the input to sqrtf() is zero, a zero will be returned.
          // If the input to vrsqrteq_f32() is zero, positive infinity is
          // returned.
          const uint32x4_t vec_p_inf = vdupq_n_u32(0x7F800000);
          // check for divide by zero
          const uint32x4_t div_by_zero =
              vceqq_u32(vec_p_inf, vreinterpretq_u32_f32(y));
          // zero out the positive infinity results
          y = vreinterpretq_f32_u32(
              vandq_u32(vmvnq_u32(div_by_zero), vreinterpretq_u32_f32(y)));
          // from arm documentation
          // The Newton-Raphson iteration:
          //     y[n+1] = y[n] * (3 - d * (y[n] * y[n])) / 2)
          // converges to (1/√d) if y0 is the result of VRSQRTE applied to d.
          //
          // Note: The precision did not improve after 2 iterations.
          for (int i = 0; i < 2; i++) {
            y = vmulq_f32(vrsqrtsq_f32(vmulq_f32(y, y), g), y);
          }
          // sqrt(g) = g * 1/sqrt(g)
          g = vmulq_f32(g, y);
#else
          g = vsqrtq_f32(g);
#endif
          vst1q_f32(&x[j], g);
        }

        for (; j < x_size; ++j) {
          x[j] = sqrtf(x[j]);
        }
      }
#endif
      break;
      default:
        std::for_each(x.begin(), x.end(), [](float& a) { a = sqrtf(a); });
    }
  }

  // Elementwise vector multiplication z = x * y.
  void MultiplyAVX2(ArrayView<const float> x,
                    ArrayView<const float> y,
                    ArrayView<float> z);
  void Multiply(ArrayView<const float> x,
                ArrayView<const float> y,
                ArrayView<float> z) {
    RTC_DCHECK_EQ(z.size(), x.size());
    RTC_DCHECK_EQ(z.size(), y.size());
    switch (optimization_) {
#if defined(WEBRTC_ARCH_X86_FAMILY)
      case Aec3Optimization::kSse2: {
        const int x_size = static_cast<int>(x.size());
        const int vector_limit = x_size >> 2;

        int j = 0;
        for (; j < vector_limit * 4; j += 4) {
          const __m128 x_j = _mm_loadu_ps(&x[j]);
          const __m128 y_j = _mm_loadu_ps(&y[j]);
          const __m128 z_j = _mm_mul_ps(x_j, y_j);
          _mm_storeu_ps(&z[j], z_j);
        }

        for (; j < x_size; ++j) {
          z[j] = x[j] * y[j];
        }
      } break;
      case Aec3Optimization::kAvx2:
        MultiplyAVX2(x, y, z);
        break;
#endif
#if defined(WEBRTC_HAS_NEON)
      case Aec3Optimization::kNeon: {
        const int x_size = static_cast<int>(x.size());
        const int vector_limit = x_size >> 2;

        int j = 0;
        for (; j < vector_limit * 4; j += 4) {
          const float32x4_t x_j = vld1q_f32(&x[j]);
          const float32x4_t y_j = vld1q_f32(&y[j]);
          const float32x4_t z_j = vmulq_f32(x_j, y_j);
          vst1q_f32(&z[j], z_j);
        }

        for (; j < x_size; ++j) {
          z[j] = x[j] * y[j];
        }
      } break;
#endif
      default:
        std::transform(x.begin(), x.end(), y.begin(), z.begin(),
                       std::multiplies<float>());
    }
  }

  // Elementwise vector accumulation z += x.
  void AccumulateAVX2(ArrayView<const float> x, ArrayView<float> z);
  void Accumulate(ArrayView<const float> x, ArrayView<float> z) {
    RTC_DCHECK_EQ(z.size(), x.size());
    switch (optimization_) {
#if defined(WEBRTC_ARCH_X86_FAMILY)
      case Aec3Optimization::kSse2: {
        const int x_size = static_cast<int>(x.size());
        const int vector_limit = x_size >> 2;

        int j = 0;
        for (; j < vector_limit * 4; j += 4) {
          const __m128 x_j = _mm_loadu_ps(&x[j]);
          __m128 z_j = _mm_loadu_ps(&z[j]);
          z_j = _mm_add_ps(x_j, z_j);
          _mm_storeu_ps(&z[j], z_j);
        }

        for (; j < x_size; ++j) {
          z[j] += x[j];
        }
      } break;
      case Aec3Optimization::kAvx2:
        AccumulateAVX2(x, z);
        break;
#endif
#if defined(WEBRTC_HAS_NEON)
      case Aec3Optimization::kNeon: {
        const int x_size = static_cast<int>(x.size());
        const int vector_limit = x_size >> 2;

        int j = 0;
        for (; j < vector_limit * 4; j += 4) {
          const float32x4_t x_j = vld1q_f32(&x[j]);
          float32x4_t z_j = vld1q_f32(&z[j]);
          z_j = vaddq_f32(z_j, x_j);
          vst1q_f32(&z[j], z_j);
        }

        for (; j < x_size; ++j) {
          z[j] += x[j];
        }
      } break;
#endif
      default:
        std::transform(x.begin(), x.end(), z.begin(), z.begin(),
                       std::plus<float>());
    }
  }

 private:
  Aec3Optimization optimization_;
};

}  // namespace aec3

}  // namespace webrtc

#endif  // MODULES_AUDIO_PROCESSING_AEC3_VECTOR_MATH_H_
