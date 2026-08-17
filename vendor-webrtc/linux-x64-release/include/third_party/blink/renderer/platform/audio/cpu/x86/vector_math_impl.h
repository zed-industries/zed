// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifdef UNSAFE_BUFFERS_BUILD
// TODO(crbug.com/351564777): Remove this and convert code to safer constructs.
#pragma allow_unsafe_buffers
#endif

// This file intentionally does not have header guards, it's included from
// vector_math_avx.h and from vector_math_sse.h with different macro
// definitions. The following line silences a presubmit warning that would
// otherwise be triggered by this: no-include-guard-because-multiply-included

#include "build/build_config.h"

#if defined(ARCH_CPU_X86_FAMILY) && !BUILDFLAG(IS_MAC)

#include <algorithm>
#include <cmath>
#include <cstring>

#include "base/check_op.h"
#include "third_party/blink/renderer/platform/audio/audio_array.h"

namespace blink {
namespace vector_math {
namespace VECTOR_MATH_SIMD_NAMESPACE_NAME {

// This stride is chosen so that the same prepared filter created by
// AVX::PrepareFilterForConv can be used by both AVX::Conv and sse::Conv.
// A prepared filter created by sse::PrepareFilterForConv can only be used
// by sse::Conv.
constexpr size_t kReversedFilterStride = 8u / kPackedFloatsPerRegister;

bool IsAligned(const float* p) {
  constexpr size_t kBytesPerRegister = kBitsPerRegister / 8u;
  constexpr size_t kAlignmentOffsetMask = kBytesPerRegister - 1u;
  return (reinterpret_cast<size_t>(p) & kAlignmentOffsetMask) == 0u;
}

void PrepareFilterForConv(const float* filter_p,
                          int filter_stride,
                          size_t filter_size,
                          AudioFloatArray* prepared_filter) {
  // Only contiguous convolution is implemented. Correlation (positive
  // |filter_stride|) and support for non-contiguous vectors are not
  // implemented.
  DCHECK_EQ(-1, filter_stride);
  DCHECK(prepared_filter);

  // Reverse the filter and repeat each value across a vector
  prepared_filter->Allocate(kReversedFilterStride * kPackedFloatsPerRegister *
                            filter_size);
  MType* reversed_filter = reinterpret_cast<MType*>(prepared_filter->Data());
  for (size_t i = 0; i < filter_size; ++i) {
    reversed_filter[kReversedFilterStride * i] = MM_PS(set1)(*(filter_p - i));
  }
}

// Direct vector convolution:
// dest[k] = sum(source[k+m]*filter[m*filter_stride]) for all m
// provided that |prepared_filter_p| is |prepared_filter->Data()| and that
// |prepared_filter| is prepared with |PrepareFilterForConv|.
void Conv(const float* source_p,
          const float* prepared_filter_p,
          float* dest_p,
          uint32_t frames_to_process,
          size_t filter_size) {
  const float* const dest_end_p = dest_p + frames_to_process;

  DCHECK_EQ(0u, frames_to_process % kPackedFloatsPerRegister);
  DCHECK_EQ(0u, filter_size % kPackedFloatsPerRegister);

  const MType* reversed_filter =
      reinterpret_cast<const MType*>(prepared_filter_p);

  // Do convolution with kPackedFloatsPerRegister inputs at a time.
  while (dest_p < dest_end_p) {
    MType m_convolution_sum = MM_PS(setzero)();

    // |filter_size| is a multiple of kPackedFloatsPerRegister so we can unroll
    // the loop by kPackedFloatsPerRegister, manually.
    for (size_t i = 0; i < filter_size; i += kPackedFloatsPerRegister) {
      for (size_t j = 0; j < kPackedFloatsPerRegister; ++j) {
        size_t k = i + j;
        MType m_product;
        MType m_source;

        m_source = MM_PS(loadu)(source_p + k);
        m_product =
            MM_PS(mul)(reversed_filter[kReversedFilterStride * k], m_source);
        m_convolution_sum = MM_PS(add)(m_convolution_sum, m_product);
      }
    }
    MM_PS(storeu)(dest_p, m_convolution_sum);

    source_p += kPackedFloatsPerRegister;
    dest_p += kPackedFloatsPerRegister;
  }
}

// dest[k] = source1[k] + source2[k]
void Vadd(const float* source1p,
          const float* source2p,
          float* dest_p,
          uint32_t frames_to_process) {
  const float* const source1_end_p = source1p + frames_to_process;

  DCHECK(IsAligned(source1p));
  DCHECK_EQ(0u, frames_to_process % kPackedFloatsPerRegister);

#define ADD_ALL(loadSource2, storeDest)              \
  while (source1p < source1_end_p) {                 \
    MType m_source1 = MM_PS(load)(source1p);         \
    MType m_source2 = MM_PS(loadSource2)(source2p);  \
    MType m_dest = MM_PS(add)(m_source1, m_source2); \
    MM_PS(storeDest)(dest_p, m_dest);                \
    source1p += kPackedFloatsPerRegister;            \
    source2p += kPackedFloatsPerRegister;            \
    dest_p += kPackedFloatsPerRegister;              \
  }

  if (IsAligned(source2p)) {
    if (IsAligned(dest_p)) {
      ADD_ALL(load, store);
    } else {
      ADD_ALL(load, storeu);
    }
  } else {
    if (IsAligned(dest_p)) {
      ADD_ALL(loadu, store);
    } else {
      ADD_ALL(loadu, storeu);
    }
  }

#undef ADD_ALL
}

// dest[k] = source1[k] - source2[k]
void Vsub(const float* source1p,
          const float* source2p,
          float* dest_p,
          uint32_t frames_to_process) {
  const float* const source1_end_p = source1p + frames_to_process;

  DCHECK(IsAligned(source1p));
  DCHECK_EQ(0u, frames_to_process % kPackedFloatsPerRegister);

#define SUB_ALL(loadSource2, storeDest)              \
  while (source1p < source1_end_p) {                 \
    MType m_source1 = MM_PS(load)(source1p);         \
    MType m_source2 = MM_PS(loadSource2)(source2p);  \
    MType m_dest = MM_PS(sub)(m_source1, m_source2); \
    MM_PS(storeDest)(dest_p, m_dest);                \
    source1p += kPackedFloatsPerRegister;            \
    source2p += kPackedFloatsPerRegister;            \
    dest_p += kPackedFloatsPerRegister;              \
  }

  if (IsAligned(source2p)) {
    if (IsAligned(dest_p)) {
      SUB_ALL(load, store);
    } else {
      SUB_ALL(load, storeu);
    }
  } else {
    if (IsAligned(dest_p)) {
      SUB_ALL(loadu, store);
    } else {
      SUB_ALL(loadu, storeu);
    }
  }

#undef SUB_ALL
}

// dest[k] = clip(source[k], low_threshold, high_threshold)
//         = max(low_threshold, min(high_threshold, source[k]))
void Vclip(const float* source_p,
           const float* low_threshold_p,
           const float* high_threshold_p,
           float* dest_p,
           uint32_t frames_to_process) {
  const float* const source_end_p = source_p + frames_to_process;

  DCHECK(IsAligned(source_p));
  DCHECK_EQ(0u, frames_to_process % kPackedFloatsPerRegister);

  MType m_low_threshold = MM_PS(set1)(*low_threshold_p);
  MType m_high_threshold = MM_PS(set1)(*high_threshold_p);

#define CLIP_ALL(storeDest)                                                  \
  while (source_p < source_end_p) {                                          \
    MType m_source = MM_PS(load)(source_p);                                  \
    MType m_dest =                                                           \
        MM_PS(max)(m_low_threshold, MM_PS(min)(m_high_threshold, m_source)); \
    MM_PS(storeDest)(dest_p, m_dest);                                        \
    source_p += kPackedFloatsPerRegister;                                    \
    dest_p += kPackedFloatsPerRegister;                                      \
  }

  if (IsAligned(dest_p)) {
    CLIP_ALL(store);
  } else {
    CLIP_ALL(storeu);
  }

#undef CLIP_ALL
}

// *max_p = max(*max_p, source_max) where
// source_max = max(abs(source[k])) for all k
void Vmaxmgv(const float* source_p, float* max_p, uint32_t frames_to_process) {
  constexpr uint32_t kMask = 0x7FFFFFFFu;
  float kMask_float;
  std::memcpy(&kMask_float, &kMask, 4);
  const float* const source_end_p = source_p + frames_to_process;

  DCHECK(IsAligned(source_p));
  DCHECK_EQ(0u, frames_to_process % kPackedFloatsPerRegister);

  MType m_mask = MM_PS(set1)(kMask_float);
  MType m_max = MM_PS(setzero)();

  while (source_p < source_end_p) {
    MType m_source = MM_PS(load)(source_p);
    // Calculate the absolute value by ANDing the source with the mask,
    // which will set the sign bit to 0.
    m_source = MM_PS(and)(m_source, m_mask);
    m_max = MM_PS(max)(m_source, m_max);
    source_p += kPackedFloatsPerRegister;
  }

  // Combine the packed floats.
  const float* maxes = reinterpret_cast<const float*>(&m_max);
  for (unsigned i = 0u; i < kPackedFloatsPerRegister; ++i)
    *max_p = std::max(*max_p, maxes[i]);
}

// dest[k] = source1[k] * source2[k]
void Vmul(const float* source1p,
          const float* source2p,
          float* dest_p,
          uint32_t frames_to_process) {
  const float* const source1_end_p = source1p + frames_to_process;

  DCHECK(IsAligned(source1p));
  DCHECK_EQ(0u, frames_to_process % kPackedFloatsPerRegister);

#define MULTIPLY_ALL(loadSource2, storeDest)         \
  while (source1p < source1_end_p) {                 \
    MType m_source1 = MM_PS(load)(source1p);         \
    MType m_source2 = MM_PS(loadSource2)(source2p);  \
    MType m_dest = MM_PS(mul)(m_source1, m_source2); \
    MM_PS(storeDest)(dest_p, m_dest);                \
    source1p += kPackedFloatsPerRegister;            \
    source2p += kPackedFloatsPerRegister;            \
    dest_p += kPackedFloatsPerRegister;              \
  }

  if (IsAligned(source2p)) {
    if (IsAligned(dest_p)) {
      MULTIPLY_ALL(load, store);
    } else {
      MULTIPLY_ALL(load, storeu);
    }
  } else {
    if (IsAligned(dest_p)) {
      MULTIPLY_ALL(loadu, store);
    } else {
      MULTIPLY_ALL(loadu, storeu);
    }
  }

#undef MULTIPLY_ALL
}

// dest[k] += scale * source[k]
void Vsma(const float* source_p,
          const float* scale,
          float* dest_p,
          uint32_t frames_to_process) {
  const float* const source_end_p = source_p + frames_to_process;

  DCHECK(IsAligned(source_p));
  DCHECK_EQ(0u, frames_to_process % kPackedFloatsPerRegister);

  const MType m_scale = MM_PS(set1)(*scale);

#define SCALAR_MULTIPLY_AND_ADD_ALL(loadDest, storeDest)        \
  while (source_p < source_end_p) {                             \
    MType m_source = MM_PS(load)(source_p);                     \
    MType m_dest = MM_PS(loadDest)(dest_p);                     \
    m_dest = MM_PS(add)(m_dest, MM_PS(mul)(m_scale, m_source)); \
    MM_PS(storeDest)(dest_p, m_dest);                           \
    source_p += kPackedFloatsPerRegister;                       \
    dest_p += kPackedFloatsPerRegister;                         \
  }

  if (IsAligned(dest_p)) {
    SCALAR_MULTIPLY_AND_ADD_ALL(load, store);
  } else {
    SCALAR_MULTIPLY_AND_ADD_ALL(loadu, storeu);
  }

#undef SCALAR_MULTIPLY_AND_ADD_ALL
}

// dest[k] = scale * source[k]
void Vsmul(const float* source_p,
           const float* scale,
           float* dest_p,
           uint32_t frames_to_process) {
  const float* const source_end_p = source_p + frames_to_process;

  DCHECK(IsAligned(source_p));
  DCHECK_EQ(0u, frames_to_process % kPackedFloatsPerRegister);

  const MType m_scale = MM_PS(set1)(*scale);

#define SCALAR_MULTIPLY_ALL(storeDest)            \
  while (source_p < source_end_p) {               \
    MType m_source = MM_PS(load)(source_p);       \
    MType m_dest = MM_PS(mul)(m_scale, m_source); \
    MM_PS(storeDest)(dest_p, m_dest);             \
    source_p += kPackedFloatsPerRegister;         \
    dest_p += kPackedFloatsPerRegister;           \
  }

  if (IsAligned(dest_p)) {
    SCALAR_MULTIPLY_ALL(store);
  } else {
    SCALAR_MULTIPLY_ALL(storeu);
  }

#undef SCALAR_MULTIPLY_ALL
}

// dest[k] = addend + source[k]
void Vsadd(const float* source_p,
           const float* addend,
           float* dest_p,
           uint32_t frames_to_process) {
  const float* const source_end_p = source_p + frames_to_process;

  DCHECK(IsAligned(source_p));
  DCHECK_EQ(0u, frames_to_process % kPackedFloatsPerRegister);

  const MType m_addend = MM_PS(set1)(*addend);

#define SCALAR_ADD_ALL(storeDest)                  \
  while (source_p < source_end_p) {                \
    MType m_source = MM_PS(load)(source_p);        \
    MType m_dest = MM_PS(add)(m_addend, m_source); \
    MM_PS(storeDest)(dest_p, m_dest);              \
    source_p += kPackedFloatsPerRegister;          \
    dest_p += kPackedFloatsPerRegister;            \
  }

  if (IsAligned(dest_p)) {
    SCALAR_ADD_ALL(store);
  } else {
    SCALAR_ADD_ALL(storeu);
  }

#undef SCALAR_ADD_ALL
}

// sum += sum(source[k]^2) for all k
void Vsvesq(const float* source_p, float* sum_p, uint32_t frames_to_process) {
  const float* const source_end_p = source_p + frames_to_process;

  DCHECK(IsAligned(source_p));
  DCHECK_EQ(0u, frames_to_process % kPackedFloatsPerRegister);

  MType m_sum = MM_PS(setzero)();

  while (source_p < source_end_p) {
    MType m_source = MM_PS(load)(source_p);
    m_sum = MM_PS(add)(m_sum, MM_PS(mul)(m_source, m_source));
    source_p += kPackedFloatsPerRegister;
  }

  // Combine the packed floats.
  const float* sums = reinterpret_cast<const float*>(&m_sum);
  for (unsigned i = 0u; i < kPackedFloatsPerRegister; ++i)
    *sum_p += sums[i];
}

// real_dest[k] = real1[k] * real2[k] - imag1[k] * imag2[k]
// imag_dest[k] = real1[k] * imag2[k] + imag1[k] * real2[k]
void Zvmul(const float* real1p,
           const float* imag1p,
           const float* real2p,
           const float* imag2p,
           float* real_dest_p,
           float* imag_dest_p,
           uint32_t frames_to_process) {
  DCHECK(IsAligned(real1p));
  DCHECK_EQ(0u, frames_to_process % kPackedFloatsPerRegister);

#define MULTIPLY_ALL(loadOtherThanReal1, storeDest)                           \
  for (size_t i = 0u; i < frames_to_process; i += kPackedFloatsPerRegister) { \
    MType real1 = MM_PS(load)(real1p + i);                                    \
    MType real2 = MM_PS(loadOtherThanReal1)(real2p + i);                      \
    MType imag1 = MM_PS(loadOtherThanReal1)(imag1p + i);                      \
    MType imag2 = MM_PS(loadOtherThanReal1)(imag2p + i);                      \
    MType real =                                                              \
        MM_PS(sub)(MM_PS(mul)(real1, real2), MM_PS(mul)(imag1, imag2));       \
    MType imag =                                                              \
        MM_PS(add)(MM_PS(mul)(real1, imag2), MM_PS(mul)(imag1, real2));       \
    MM_PS(storeDest)(real_dest_p + i, real);                                  \
    MM_PS(storeDest)(imag_dest_p + i, imag);                                  \
  }

  if (IsAligned(imag1p) && IsAligned(real2p) && IsAligned(imag2p) &&
      IsAligned(real_dest_p) && IsAligned(imag_dest_p)) {
    MULTIPLY_ALL(load, store);
  } else {
    MULTIPLY_ALL(loadu, storeu);
  }

#undef MULTIPLY_ALL
}

}  // namespace VECTOR_MATH_SIMD_NAMESPACE_NAME
}  // namespace vector_math
}  // namespace blink

#endif  // defined(ARCH_CPU_X86_FAMILY) && !BUILDFLAG(IS_MAC)
