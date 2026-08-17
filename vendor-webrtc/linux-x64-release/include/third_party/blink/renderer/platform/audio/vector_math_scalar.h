// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifdef UNSAFE_BUFFERS_BUILD
// TODO(crbug.com/351564777): Remove this and convert code to safer constructs.
#pragma allow_unsafe_buffers
#endif

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_AUDIO_VECTOR_MATH_SCALAR_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_AUDIO_VECTOR_MATH_SCALAR_H_

#include <algorithm>
#include <cmath>

#include "base/check_op.h"
#include "third_party/blink/renderer/platform/audio/audio_array.h"
#include "third_party/blink/renderer/platform/wtf/math_extras.h"

namespace blink {
namespace vector_math {
namespace scalar {

ALWAYS_INLINE static void Conv(const float* source_p,
                               int source_stride,
                               const float* filter_p,
                               int filter_stride,
                               float* dest_p,
                               int dest_stride,
                               uint32_t frames_to_process,
                               size_t filter_size,
                               const AudioFloatArray* /*prepared_filter*/) {
  // Only contiguous convolution is implemented. Correlation (positive
  // |filter_stride|) and support for non-contiguous vectors are not
  // implemented.
  DCHECK_EQ(1, source_stride);
  DCHECK_EQ(-1, filter_stride);
  DCHECK_EQ(1, dest_stride);

  size_t i = 0;

// FIXME: The macro can be further optimized to avoid pipeline stalls. One
// possibility is to maintain 4 separate sums and change the macro to
// CONVOLVE_FOUR_SAMPLES.
#define CONVOLVE_ONE_SAMPLE                   \
  do {                                        \
    sum += source_p[i + j] * *(filter_p - j); \
    j++;                                      \
  } while (0)

  while (i < frames_to_process) {
    size_t j = 0;
    float sum = 0;

    if (filter_size == 32) {
      CONVOLVE_ONE_SAMPLE;  // 1
      CONVOLVE_ONE_SAMPLE;  // 2
      CONVOLVE_ONE_SAMPLE;  // 3
      CONVOLVE_ONE_SAMPLE;  // 4
      CONVOLVE_ONE_SAMPLE;  // 5
      CONVOLVE_ONE_SAMPLE;  // 6
      CONVOLVE_ONE_SAMPLE;  // 7
      CONVOLVE_ONE_SAMPLE;  // 8
      CONVOLVE_ONE_SAMPLE;  // 9
      CONVOLVE_ONE_SAMPLE;  // 10

      CONVOLVE_ONE_SAMPLE;  // 11
      CONVOLVE_ONE_SAMPLE;  // 12
      CONVOLVE_ONE_SAMPLE;  // 13
      CONVOLVE_ONE_SAMPLE;  // 14
      CONVOLVE_ONE_SAMPLE;  // 15
      CONVOLVE_ONE_SAMPLE;  // 16
      CONVOLVE_ONE_SAMPLE;  // 17
      CONVOLVE_ONE_SAMPLE;  // 18
      CONVOLVE_ONE_SAMPLE;  // 19
      CONVOLVE_ONE_SAMPLE;  // 20

      CONVOLVE_ONE_SAMPLE;  // 21
      CONVOLVE_ONE_SAMPLE;  // 22
      CONVOLVE_ONE_SAMPLE;  // 23
      CONVOLVE_ONE_SAMPLE;  // 24
      CONVOLVE_ONE_SAMPLE;  // 25
      CONVOLVE_ONE_SAMPLE;  // 26
      CONVOLVE_ONE_SAMPLE;  // 27
      CONVOLVE_ONE_SAMPLE;  // 28
      CONVOLVE_ONE_SAMPLE;  // 29
      CONVOLVE_ONE_SAMPLE;  // 30

      CONVOLVE_ONE_SAMPLE;  // 31
      CONVOLVE_ONE_SAMPLE;  // 32

    } else if (filter_size == 64) {
      CONVOLVE_ONE_SAMPLE;  // 1
      CONVOLVE_ONE_SAMPLE;  // 2
      CONVOLVE_ONE_SAMPLE;  // 3
      CONVOLVE_ONE_SAMPLE;  // 4
      CONVOLVE_ONE_SAMPLE;  // 5
      CONVOLVE_ONE_SAMPLE;  // 6
      CONVOLVE_ONE_SAMPLE;  // 7
      CONVOLVE_ONE_SAMPLE;  // 8
      CONVOLVE_ONE_SAMPLE;  // 9
      CONVOLVE_ONE_SAMPLE;  // 10

      CONVOLVE_ONE_SAMPLE;  // 11
      CONVOLVE_ONE_SAMPLE;  // 12
      CONVOLVE_ONE_SAMPLE;  // 13
      CONVOLVE_ONE_SAMPLE;  // 14
      CONVOLVE_ONE_SAMPLE;  // 15
      CONVOLVE_ONE_SAMPLE;  // 16
      CONVOLVE_ONE_SAMPLE;  // 17
      CONVOLVE_ONE_SAMPLE;  // 18
      CONVOLVE_ONE_SAMPLE;  // 19
      CONVOLVE_ONE_SAMPLE;  // 20

      CONVOLVE_ONE_SAMPLE;  // 21
      CONVOLVE_ONE_SAMPLE;  // 22
      CONVOLVE_ONE_SAMPLE;  // 23
      CONVOLVE_ONE_SAMPLE;  // 24
      CONVOLVE_ONE_SAMPLE;  // 25
      CONVOLVE_ONE_SAMPLE;  // 26
      CONVOLVE_ONE_SAMPLE;  // 27
      CONVOLVE_ONE_SAMPLE;  // 28
      CONVOLVE_ONE_SAMPLE;  // 29
      CONVOLVE_ONE_SAMPLE;  // 30

      CONVOLVE_ONE_SAMPLE;  // 31
      CONVOLVE_ONE_SAMPLE;  // 32
      CONVOLVE_ONE_SAMPLE;  // 33
      CONVOLVE_ONE_SAMPLE;  // 34
      CONVOLVE_ONE_SAMPLE;  // 35
      CONVOLVE_ONE_SAMPLE;  // 36
      CONVOLVE_ONE_SAMPLE;  // 37
      CONVOLVE_ONE_SAMPLE;  // 38
      CONVOLVE_ONE_SAMPLE;  // 39
      CONVOLVE_ONE_SAMPLE;  // 40

      CONVOLVE_ONE_SAMPLE;  // 41
      CONVOLVE_ONE_SAMPLE;  // 42
      CONVOLVE_ONE_SAMPLE;  // 43
      CONVOLVE_ONE_SAMPLE;  // 44
      CONVOLVE_ONE_SAMPLE;  // 45
      CONVOLVE_ONE_SAMPLE;  // 46
      CONVOLVE_ONE_SAMPLE;  // 47
      CONVOLVE_ONE_SAMPLE;  // 48
      CONVOLVE_ONE_SAMPLE;  // 49
      CONVOLVE_ONE_SAMPLE;  // 50

      CONVOLVE_ONE_SAMPLE;  // 51
      CONVOLVE_ONE_SAMPLE;  // 52
      CONVOLVE_ONE_SAMPLE;  // 53
      CONVOLVE_ONE_SAMPLE;  // 54
      CONVOLVE_ONE_SAMPLE;  // 55
      CONVOLVE_ONE_SAMPLE;  // 56
      CONVOLVE_ONE_SAMPLE;  // 57
      CONVOLVE_ONE_SAMPLE;  // 58
      CONVOLVE_ONE_SAMPLE;  // 59
      CONVOLVE_ONE_SAMPLE;  // 60

      CONVOLVE_ONE_SAMPLE;  // 61
      CONVOLVE_ONE_SAMPLE;  // 62
      CONVOLVE_ONE_SAMPLE;  // 63
      CONVOLVE_ONE_SAMPLE;  // 64

    } else if (filter_size == 128) {
      CONVOLVE_ONE_SAMPLE;  // 1
      CONVOLVE_ONE_SAMPLE;  // 2
      CONVOLVE_ONE_SAMPLE;  // 3
      CONVOLVE_ONE_SAMPLE;  // 4
      CONVOLVE_ONE_SAMPLE;  // 5
      CONVOLVE_ONE_SAMPLE;  // 6
      CONVOLVE_ONE_SAMPLE;  // 7
      CONVOLVE_ONE_SAMPLE;  // 8
      CONVOLVE_ONE_SAMPLE;  // 9
      CONVOLVE_ONE_SAMPLE;  // 10

      CONVOLVE_ONE_SAMPLE;  // 11
      CONVOLVE_ONE_SAMPLE;  // 12
      CONVOLVE_ONE_SAMPLE;  // 13
      CONVOLVE_ONE_SAMPLE;  // 14
      CONVOLVE_ONE_SAMPLE;  // 15
      CONVOLVE_ONE_SAMPLE;  // 16
      CONVOLVE_ONE_SAMPLE;  // 17
      CONVOLVE_ONE_SAMPLE;  // 18
      CONVOLVE_ONE_SAMPLE;  // 19
      CONVOLVE_ONE_SAMPLE;  // 20

      CONVOLVE_ONE_SAMPLE;  // 21
      CONVOLVE_ONE_SAMPLE;  // 22
      CONVOLVE_ONE_SAMPLE;  // 23
      CONVOLVE_ONE_SAMPLE;  // 24
      CONVOLVE_ONE_SAMPLE;  // 25
      CONVOLVE_ONE_SAMPLE;  // 26
      CONVOLVE_ONE_SAMPLE;  // 27
      CONVOLVE_ONE_SAMPLE;  // 28
      CONVOLVE_ONE_SAMPLE;  // 29
      CONVOLVE_ONE_SAMPLE;  // 30

      CONVOLVE_ONE_SAMPLE;  // 31
      CONVOLVE_ONE_SAMPLE;  // 32
      CONVOLVE_ONE_SAMPLE;  // 33
      CONVOLVE_ONE_SAMPLE;  // 34
      CONVOLVE_ONE_SAMPLE;  // 35
      CONVOLVE_ONE_SAMPLE;  // 36
      CONVOLVE_ONE_SAMPLE;  // 37
      CONVOLVE_ONE_SAMPLE;  // 38
      CONVOLVE_ONE_SAMPLE;  // 39
      CONVOLVE_ONE_SAMPLE;  // 40

      CONVOLVE_ONE_SAMPLE;  // 41
      CONVOLVE_ONE_SAMPLE;  // 42
      CONVOLVE_ONE_SAMPLE;  // 43
      CONVOLVE_ONE_SAMPLE;  // 44
      CONVOLVE_ONE_SAMPLE;  // 45
      CONVOLVE_ONE_SAMPLE;  // 46
      CONVOLVE_ONE_SAMPLE;  // 47
      CONVOLVE_ONE_SAMPLE;  // 48
      CONVOLVE_ONE_SAMPLE;  // 49
      CONVOLVE_ONE_SAMPLE;  // 50

      CONVOLVE_ONE_SAMPLE;  // 51
      CONVOLVE_ONE_SAMPLE;  // 52
      CONVOLVE_ONE_SAMPLE;  // 53
      CONVOLVE_ONE_SAMPLE;  // 54
      CONVOLVE_ONE_SAMPLE;  // 55
      CONVOLVE_ONE_SAMPLE;  // 56
      CONVOLVE_ONE_SAMPLE;  // 57
      CONVOLVE_ONE_SAMPLE;  // 58
      CONVOLVE_ONE_SAMPLE;  // 59
      CONVOLVE_ONE_SAMPLE;  // 60

      CONVOLVE_ONE_SAMPLE;  // 61
      CONVOLVE_ONE_SAMPLE;  // 62
      CONVOLVE_ONE_SAMPLE;  // 63
      CONVOLVE_ONE_SAMPLE;  // 64
      CONVOLVE_ONE_SAMPLE;  // 65
      CONVOLVE_ONE_SAMPLE;  // 66
      CONVOLVE_ONE_SAMPLE;  // 67
      CONVOLVE_ONE_SAMPLE;  // 68
      CONVOLVE_ONE_SAMPLE;  // 69
      CONVOLVE_ONE_SAMPLE;  // 70

      CONVOLVE_ONE_SAMPLE;  // 71
      CONVOLVE_ONE_SAMPLE;  // 72
      CONVOLVE_ONE_SAMPLE;  // 73
      CONVOLVE_ONE_SAMPLE;  // 74
      CONVOLVE_ONE_SAMPLE;  // 75
      CONVOLVE_ONE_SAMPLE;  // 76
      CONVOLVE_ONE_SAMPLE;  // 77
      CONVOLVE_ONE_SAMPLE;  // 78
      CONVOLVE_ONE_SAMPLE;  // 79
      CONVOLVE_ONE_SAMPLE;  // 80

      CONVOLVE_ONE_SAMPLE;  // 81
      CONVOLVE_ONE_SAMPLE;  // 82
      CONVOLVE_ONE_SAMPLE;  // 83
      CONVOLVE_ONE_SAMPLE;  // 84
      CONVOLVE_ONE_SAMPLE;  // 85
      CONVOLVE_ONE_SAMPLE;  // 86
      CONVOLVE_ONE_SAMPLE;  // 87
      CONVOLVE_ONE_SAMPLE;  // 88
      CONVOLVE_ONE_SAMPLE;  // 89
      CONVOLVE_ONE_SAMPLE;  // 90

      CONVOLVE_ONE_SAMPLE;  // 91
      CONVOLVE_ONE_SAMPLE;  // 92
      CONVOLVE_ONE_SAMPLE;  // 93
      CONVOLVE_ONE_SAMPLE;  // 94
      CONVOLVE_ONE_SAMPLE;  // 95
      CONVOLVE_ONE_SAMPLE;  // 96
      CONVOLVE_ONE_SAMPLE;  // 97
      CONVOLVE_ONE_SAMPLE;  // 98
      CONVOLVE_ONE_SAMPLE;  // 99
      CONVOLVE_ONE_SAMPLE;  // 100

      CONVOLVE_ONE_SAMPLE;  // 101
      CONVOLVE_ONE_SAMPLE;  // 102
      CONVOLVE_ONE_SAMPLE;  // 103
      CONVOLVE_ONE_SAMPLE;  // 104
      CONVOLVE_ONE_SAMPLE;  // 105
      CONVOLVE_ONE_SAMPLE;  // 106
      CONVOLVE_ONE_SAMPLE;  // 107
      CONVOLVE_ONE_SAMPLE;  // 108
      CONVOLVE_ONE_SAMPLE;  // 109
      CONVOLVE_ONE_SAMPLE;  // 110

      CONVOLVE_ONE_SAMPLE;  // 111
      CONVOLVE_ONE_SAMPLE;  // 112
      CONVOLVE_ONE_SAMPLE;  // 113
      CONVOLVE_ONE_SAMPLE;  // 114
      CONVOLVE_ONE_SAMPLE;  // 115
      CONVOLVE_ONE_SAMPLE;  // 116
      CONVOLVE_ONE_SAMPLE;  // 117
      CONVOLVE_ONE_SAMPLE;  // 118
      CONVOLVE_ONE_SAMPLE;  // 119
      CONVOLVE_ONE_SAMPLE;  // 120

      CONVOLVE_ONE_SAMPLE;  // 121
      CONVOLVE_ONE_SAMPLE;  // 122
      CONVOLVE_ONE_SAMPLE;  // 123
      CONVOLVE_ONE_SAMPLE;  // 124
      CONVOLVE_ONE_SAMPLE;  // 125
      CONVOLVE_ONE_SAMPLE;  // 126
      CONVOLVE_ONE_SAMPLE;  // 127
      CONVOLVE_ONE_SAMPLE;  // 128
    } else {
      while (j < filter_size) {
        // Non-optimized using actual while loop.
        CONVOLVE_ONE_SAMPLE;
      }
    }
    dest_p[i++] = sum;
  }
#undef CONVOLVE_ONE_SAMPLE
}

ALWAYS_INLINE static void Vadd(const float* source1p,
                               int source_stride1,
                               const float* source2p,
                               int source_stride2,
                               float* dest_p,
                               int dest_stride,
                               uint32_t frames_to_process) {
  while (frames_to_process > 0u) {
    *dest_p = *source1p + *source2p;
    source1p += source_stride1;
    source2p += source_stride2;
    dest_p += dest_stride;
    --frames_to_process;
  }
}

ALWAYS_INLINE static void Vsub(const float* source1p,
                               int source_stride1,
                               const float* source2p,
                               int source_stride2,
                               float* dest_p,
                               int dest_stride,
                               uint32_t frames_to_process) {
  while (frames_to_process > 0u) {
    *dest_p = *source1p - *source2p;
    source1p += source_stride1;
    source2p += source_stride2;
    dest_p += dest_stride;
    --frames_to_process;
  }
}

ALWAYS_INLINE static void Vclip(const float* source_p,
                                int source_stride,
                                const float* low_threshold_p,
                                const float* high_threshold_p,
                                float* dest_p,
                                int dest_stride,
                                uint32_t frames_to_process) {
  while (frames_to_process > 0u) {
    *dest_p = ClampTo(*source_p, *low_threshold_p, *high_threshold_p);
    source_p += source_stride;
    dest_p += dest_stride;
    --frames_to_process;
  }
}

ALWAYS_INLINE static void Vmaxmgv(const float* source_p,
                                  int source_stride,
                                  float* max_p,
                                  uint32_t frames_to_process) {
  while (frames_to_process > 0u) {
    *max_p = std::max(*max_p, std::abs(*source_p));
    source_p += source_stride;
    --frames_to_process;
  }
}

ALWAYS_INLINE static void Vmul(const float* source1p,
                               int source_stride1,
                               const float* source2p,
                               int source_stride2,
                               float* dest_p,
                               int dest_stride,
                               uint32_t frames_to_process) {
  while (frames_to_process > 0u) {
    *dest_p = *source1p * *source2p;
    source1p += source_stride1;
    source2p += source_stride2;
    dest_p += dest_stride;
    --frames_to_process;
  }
}

ALWAYS_INLINE static void Vsma(const float* source_p,
                               int source_stride,
                               const float* scale,
                               float* dest_p,
                               int dest_stride,
                               uint32_t frames_to_process) {
  const float k = *scale;
  while (frames_to_process > 0u) {
    *dest_p += k * *source_p;
    source_p += source_stride;
    dest_p += dest_stride;
    --frames_to_process;
  }
}

ALWAYS_INLINE static void Vsmul(const float* source_p,
                                int source_stride,
                                const float* scale,
                                float* dest_p,
                                int dest_stride,
                                uint32_t frames_to_process) {
  const float k = *scale;
  while (frames_to_process > 0u) {
    *dest_p = k * *source_p;
    source_p += source_stride;
    dest_p += dest_stride;
    --frames_to_process;
  }
}

ALWAYS_INLINE static void Vsadd(const float* source_p,
                                int source_stride,
                                const float* addend,
                                float* dest_p,
                                int dest_stride,
                                uint32_t frames_to_process) {
  const float k = *addend;
  while (frames_to_process > 0u) {
    *dest_p = *source_p + k;
    source_p += source_stride;
    dest_p += dest_stride;
    --frames_to_process;
  }
}

ALWAYS_INLINE static void Vsvesq(const float* source_p,
                                 int source_stride,
                                 float* sum_p,
                                 uint32_t frames_to_process) {
  while (frames_to_process > 0u) {
    const float sample = *source_p;
    *sum_p += sample * sample;
    source_p += source_stride;
    --frames_to_process;
  }
}

ALWAYS_INLINE static void Zvmul(const float* real1p,
                                const float* imag1p,
                                const float* real2p,
                                const float* imag2p,
                                float* real_dest_p,
                                float* imag_dest_p,
                                uint32_t frames_to_process) {
  for (size_t i = 0u; i < frames_to_process; ++i) {
    // Read and compute result before storing them, in case the
    // destination is the same as one of the sources.
    float real_result = real1p[i] * real2p[i] - imag1p[i] * imag2p[i];
    float imag_result = real1p[i] * imag2p[i] + imag1p[i] * real2p[i];

    real_dest_p[i] = real_result;
    imag_dest_p[i] = imag_result;
  }
}

}  // namespace scalar
}  // namespace vector_math
}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_AUDIO_VECTOR_MATH_SCALAR_H_
