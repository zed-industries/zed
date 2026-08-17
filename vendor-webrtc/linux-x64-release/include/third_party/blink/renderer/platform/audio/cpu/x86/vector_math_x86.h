// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifdef UNSAFE_BUFFERS_BUILD
// TODO(crbug.com/351564777): Remove this and convert code to safer constructs.
#pragma allow_unsafe_buffers
#endif

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_AUDIO_CPU_X86_VECTOR_MATH_X86_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_AUDIO_CPU_X86_VECTOR_MATH_X86_H_

#include "base/check_op.h"
#include "base/cpu.h"
#include "third_party/blink/renderer/platform/audio/cpu/x86/vector_math_avx.h"
#include "third_party/blink/renderer/platform/audio/cpu/x86/vector_math_sse.h"
#include "third_party/blink/renderer/platform/audio/vector_math_scalar.h"

namespace blink {
namespace vector_math {
namespace x86 {

struct FrameCounts {
  uint32_t scalar_for_alignment;
  uint32_t sse_for_alignment;
  uint32_t avx;
  uint32_t sse;
  uint32_t scalar;
};

static bool CPUSupportsAVX() {
  static const bool supports = ::base::CPU().has_avx();
  return supports;
}

static uint32_t GetAVXAlignmentOffsetInNumberOfFloats(const float* source_p) {
  constexpr uint32_t kBytesPerRegister = avx::kBitsPerRegister / 8u;
  constexpr uint32_t kAlignmentOffsetMask = kBytesPerRegister - 1u;
  uintptr_t offset =
      reinterpret_cast<uintptr_t>(source_p) & kAlignmentOffsetMask;
  DCHECK_EQ(0u, offset % sizeof(*source_p));
  return static_cast<uint32_t>(offset / sizeof(*source_p));
}

ALWAYS_INLINE static FrameCounts SplitFramesToProcess(
    const float* source_p,
    uint32_t frames_to_process) {
  FrameCounts counts = {0u, 0u, 0u, 0u, 0u};

  const uint32_t avx_alignment_offset =
      GetAVXAlignmentOffsetInNumberOfFloats(source_p);

  // If the first frame is not AVX aligned, the first several frames (at most
  // seven) must be processed separately for proper alignment.
  const uint32_t total_for_alignment =
      (avx::kPackedFloatsPerRegister - avx_alignment_offset) &
      ~avx::kFramesToProcessMask;
  const uint32_t scalar_for_alignment =
      total_for_alignment & ~sse::kFramesToProcessMask;
  const uint32_t sse_for_alignment =
      total_for_alignment & sse::kFramesToProcessMask;

  // Check which CPU features can be used based on the number of frames to
  // process and based on CPU support.
  const bool use_at_least_avx =
      CPUSupportsAVX() &&
      frames_to_process >= scalar_for_alignment + sse_for_alignment +
                               avx::kPackedFloatsPerRegister;
  const bool use_at_least_sse =
      use_at_least_avx ||
      frames_to_process >= scalar_for_alignment + sse::kPackedFloatsPerRegister;

  if (use_at_least_sse) {
    counts.scalar_for_alignment = scalar_for_alignment;
    frames_to_process -= counts.scalar_for_alignment;
    // The remaining frames are SSE aligned.
    DCHECK(sse::IsAligned(source_p + counts.scalar_for_alignment));

    if (use_at_least_avx) {
      counts.sse_for_alignment = sse_for_alignment;
      frames_to_process -= counts.sse_for_alignment;
      // The remaining frames are AVX aligned.
      DCHECK(avx::IsAligned(source_p + counts.scalar_for_alignment +
                            counts.sse_for_alignment));

      // Process as many as possible of the remaining frames using AVX.
      counts.avx = frames_to_process & avx::kFramesToProcessMask;
      frames_to_process -= counts.avx;
    }

    // Process as many as possible of the remaining frames using SSE.
    counts.sse = frames_to_process & sse::kFramesToProcessMask;
    frames_to_process -= counts.sse;
  }

  // Process the remaining frames separately.
  counts.scalar = frames_to_process;
  return counts;
}

ALWAYS_INLINE static void PrepareFilterForConv(
    const float* filter_p,
    int filter_stride,
    size_t filter_size,
    AudioFloatArray* prepared_filter) {
  if (CPUSupportsAVX()) {
    avx::PrepareFilterForConv(filter_p, filter_stride, filter_size,
                              prepared_filter);
  } else {
    sse::PrepareFilterForConv(filter_p, filter_stride, filter_size,
                              prepared_filter);
  }
}

ALWAYS_INLINE static void Conv(const float* source_p,
                               int source_stride,
                               const float* filter_p,
                               int filter_stride,
                               float* dest_p,
                               int dest_stride,
                               uint32_t frames_to_process,
                               size_t filter_size,
                               const AudioFloatArray* prepared_filter) {
  const float* prepared_filter_p =
      prepared_filter ? prepared_filter->Data() : nullptr;
  if (source_stride == 1 && dest_stride == 1 && prepared_filter_p) {
    if (CPUSupportsAVX() && (filter_size & ~avx::kFramesToProcessMask) == 0u) {
      // |frames_to_process| is always a multiply of render quantum and
      // therefore the frames can always be processed using AVX.
      CHECK_EQ(frames_to_process & ~avx::kFramesToProcessMask, 0u);
      avx::Conv(source_p, prepared_filter_p, dest_p, frames_to_process,
                filter_size);
      return;
    }
    if ((filter_size & ~sse::kFramesToProcessMask) == 0u) {
      // |frames_to_process| is always a multiply of render quantum and
      // therefore the frames can always be processed using SSE.
      CHECK_EQ(frames_to_process & ~sse::kFramesToProcessMask, 0u);
      sse::Conv(source_p, prepared_filter_p, dest_p, frames_to_process,
                filter_size);
      return;
    }
  }
  scalar::Conv(source_p, source_stride, filter_p, filter_stride, dest_p,
               dest_stride, frames_to_process, filter_size, nullptr);
}

ALWAYS_INLINE static void Vadd(const float* source1p,
                               int source_stride1,
                               const float* source2p,
                               int source_stride2,
                               float* dest_p,
                               int dest_stride,
                               uint32_t frames_to_process) {
  if (source_stride1 == 1 && source_stride2 == 1 && dest_stride == 1) {
    const FrameCounts frame_counts =
        SplitFramesToProcess(source1p, frames_to_process);

    scalar::Vadd(source1p, 1, source2p, 1, dest_p, 1,
                 frame_counts.scalar_for_alignment);
    size_t i = frame_counts.scalar_for_alignment;
    if (frame_counts.sse_for_alignment > 0u) {
      sse::Vadd(source1p + i, source2p + i, dest_p + i,
                frame_counts.sse_for_alignment);
      i += frame_counts.sse_for_alignment;
    }
    if (frame_counts.avx > 0u) {
      avx::Vadd(source1p + i, source2p + i, dest_p + i, frame_counts.avx);
      i += frame_counts.avx;
    }
    if (frame_counts.sse > 0u) {
      sse::Vadd(source1p + i, source2p + i, dest_p + i, frame_counts.sse);
      i += frame_counts.sse;
    }
    scalar::Vadd(source1p + i, 1, source2p + i, 1, dest_p + i, 1,
                 frame_counts.scalar);
    DCHECK_EQ(frames_to_process, i + frame_counts.scalar);
  } else {
    scalar::Vadd(source1p, source_stride1, source2p, source_stride2, dest_p,
                 dest_stride, frames_to_process);
  }
}

ALWAYS_INLINE static void Vsub(const float* source1p,
                               int source_stride1,
                               const float* source2p,
                               int source_stride2,
                               float* dest_p,
                               int dest_stride,
                               uint32_t frames_to_process) {
  if (source_stride1 == 1 && source_stride2 == 1 && dest_stride == 1) {
    const FrameCounts frame_counts =
        SplitFramesToProcess(source1p, frames_to_process);

    scalar::Vsub(source1p, 1, source2p, 1, dest_p, 1,
                 frame_counts.scalar_for_alignment);
    size_t i = frame_counts.scalar_for_alignment;
    if (frame_counts.sse_for_alignment > 0u) {
      sse::Vsub(source1p + i, source2p + i, dest_p + i,
                frame_counts.sse_for_alignment);
      i += frame_counts.sse_for_alignment;
    }
    if (frame_counts.avx > 0u) {
      avx::Vsub(source1p + i, source2p + i, dest_p + i, frame_counts.avx);
      i += frame_counts.avx;
    }
    if (frame_counts.sse > 0u) {
      sse::Vsub(source1p + i, source2p + i, dest_p + i, frame_counts.sse);
      i += frame_counts.sse;
    }
    scalar::Vsub(source1p + i, 1, source2p + i, 1, dest_p + i, 1,
                 frame_counts.scalar);
    DCHECK_EQ(frames_to_process, i + frame_counts.scalar);
  } else {
    scalar::Vsub(source1p, source_stride1, source2p, source_stride2, dest_p,
                 dest_stride, frames_to_process);
  }
}

ALWAYS_INLINE static void Vclip(const float* source_p,
                                int source_stride,
                                const float* low_threshold_p,
                                const float* high_threshold_p,
                                float* dest_p,
                                int dest_stride,
                                uint32_t frames_to_process) {
  if (source_stride == 1 && dest_stride == 1) {
    const FrameCounts frame_counts =
        SplitFramesToProcess(source_p, frames_to_process);

    scalar::Vclip(source_p, 1, low_threshold_p, high_threshold_p, dest_p, 1,
                  frame_counts.scalar_for_alignment);
    size_t i = frame_counts.scalar_for_alignment;
    if (frame_counts.sse_for_alignment > 0u) {
      sse::Vclip(source_p + i, low_threshold_p, high_threshold_p, dest_p + i,
                 frame_counts.sse_for_alignment);
      i += frame_counts.sse_for_alignment;
    }
    if (frame_counts.avx > 0u) {
      avx::Vclip(source_p + i, low_threshold_p, high_threshold_p, dest_p + i,
                 frame_counts.avx);
      i += frame_counts.avx;
    }
    if (frame_counts.sse > 0u) {
      sse::Vclip(source_p + i, low_threshold_p, high_threshold_p, dest_p + i,
                 frame_counts.sse);
      i += frame_counts.sse;
    }
    scalar::Vclip(source_p + i, 1, low_threshold_p, high_threshold_p,
                  dest_p + i, 1, frame_counts.scalar);
    DCHECK_EQ(frames_to_process, i + frame_counts.scalar);
  } else {
    scalar::Vclip(source_p, source_stride, low_threshold_p, high_threshold_p,
                  dest_p, dest_stride, frames_to_process);
  }
}

ALWAYS_INLINE static void Vmaxmgv(const float* source_p,
                                  int source_stride,
                                  float* max_p,
                                  uint32_t frames_to_process) {
  if (source_stride == 1) {
    const FrameCounts frame_counts =
        SplitFramesToProcess(source_p, frames_to_process);

    scalar::Vmaxmgv(source_p, 1, max_p, frame_counts.scalar_for_alignment);
    size_t i = frame_counts.scalar_for_alignment;
    if (frame_counts.sse_for_alignment > 0u) {
      sse::Vmaxmgv(source_p + i, max_p, frame_counts.sse_for_alignment);
      i += frame_counts.sse_for_alignment;
    }
    if (frame_counts.avx > 0u) {
      avx::Vmaxmgv(source_p + i, max_p, frame_counts.avx);
      i += frame_counts.avx;
    }
    if (frame_counts.sse > 0u) {
      sse::Vmaxmgv(source_p + i, max_p, frame_counts.sse);
      i += frame_counts.sse;
    }
    scalar::Vmaxmgv(source_p + i, 1, max_p, frame_counts.scalar);
    DCHECK_EQ(frames_to_process, i + frame_counts.scalar);
  } else {
    scalar::Vmaxmgv(source_p, source_stride, max_p, frames_to_process);
  }
}

ALWAYS_INLINE static void Vmul(const float* source1p,
                               int source_stride1,
                               const float* source2p,
                               int source_stride2,
                               float* dest_p,
                               int dest_stride,
                               uint32_t frames_to_process) {
  if (source_stride1 == 1 && source_stride2 == 1 && dest_stride == 1) {
    const FrameCounts frame_counts =
        SplitFramesToProcess(source1p, frames_to_process);

    scalar::Vmul(source1p, 1, source2p, 1, dest_p, 1,
                 frame_counts.scalar_for_alignment);
    size_t i = frame_counts.scalar_for_alignment;
    if (frame_counts.sse_for_alignment > 0u) {
      sse::Vmul(source1p + i, source2p + i, dest_p + i,
                frame_counts.sse_for_alignment);
      i += frame_counts.sse_for_alignment;
    }
    if (frame_counts.avx > 0u) {
      avx::Vmul(source1p + i, source2p + i, dest_p + i, frame_counts.avx);
      i += frame_counts.avx;
    }
    if (frame_counts.sse > 0u) {
      sse::Vmul(source1p + i, source2p + i, dest_p + i, frame_counts.sse);
      i += frame_counts.sse;
    }
    scalar::Vmul(source1p + i, 1, source2p + i, 1, dest_p + i, 1,
                 frame_counts.scalar);
    DCHECK_EQ(frames_to_process, i + frame_counts.scalar);
  } else {
    scalar::Vmul(source1p, source_stride1, source2p, source_stride2, dest_p,
                 dest_stride, frames_to_process);
  }
}

ALWAYS_INLINE static void Vsma(const float* source_p,
                               int source_stride,
                               const float* scale,
                               float* dest_p,
                               int dest_stride,
                               uint32_t frames_to_process) {
  if (source_stride == 1 && dest_stride == 1) {
    const FrameCounts frame_counts =
        SplitFramesToProcess(source_p, frames_to_process);

    scalar::Vsma(source_p, 1, scale, dest_p, 1,
                 frame_counts.scalar_for_alignment);
    size_t i = frame_counts.scalar_for_alignment;
    if (frame_counts.sse_for_alignment > 0u) {
      sse::Vsma(source_p + i, scale, dest_p + i,
                frame_counts.sse_for_alignment);
      i += frame_counts.sse_for_alignment;
    }
    if (frame_counts.avx > 0u) {
      avx::Vsma(source_p + i, scale, dest_p + i, frame_counts.avx);
      i += frame_counts.avx;
    }
    if (frame_counts.sse > 0u) {
      sse::Vsma(source_p + i, scale, dest_p + i, frame_counts.sse);
      i += frame_counts.sse;
    }
    scalar::Vsma(source_p + i, 1, scale, dest_p + i, 1, frame_counts.scalar);
    DCHECK_EQ(frames_to_process, i + frame_counts.scalar);
  } else {
    scalar::Vsma(source_p, source_stride, scale, dest_p, dest_stride,
                 frames_to_process);
  }
}

ALWAYS_INLINE static void Vsmul(const float* source_p,
                                int source_stride,
                                const float* scale,
                                float* dest_p,
                                int dest_stride,
                                uint32_t frames_to_process) {
  if (source_stride == 1 && dest_stride == 1) {
    const FrameCounts frame_counts =
        SplitFramesToProcess(source_p, frames_to_process);

    scalar::Vsmul(source_p, 1, scale, dest_p, 1,
                  frame_counts.scalar_for_alignment);
    size_t i = frame_counts.scalar_for_alignment;
    if (frame_counts.sse_for_alignment > 0u) {
      sse::Vsmul(source_p + i, scale, dest_p + i,
                 frame_counts.sse_for_alignment);
      i += frame_counts.sse_for_alignment;
    }
    if (frame_counts.avx > 0u) {
      avx::Vsmul(source_p + i, scale, dest_p + i, frame_counts.avx);
      i += frame_counts.avx;
    }
    if (frame_counts.sse > 0u) {
      sse::Vsmul(source_p + i, scale, dest_p + i, frame_counts.sse);
      i += frame_counts.sse;
    }
    scalar::Vsmul(source_p + i, 1, scale, dest_p + i, 1, frame_counts.scalar);
    DCHECK_EQ(frames_to_process, i + frame_counts.scalar);
  } else {
    scalar::Vsmul(source_p, source_stride, scale, dest_p, dest_stride,
                  frames_to_process);
  }
}

ALWAYS_INLINE static void Vsadd(const float* source_p,
                                int source_stride,
                                const float* addend,
                                float* dest_p,
                                int dest_stride,
                                uint32_t frames_to_process) {
  if (source_stride == 1 && dest_stride == 1) {
    const FrameCounts frame_counts =
        SplitFramesToProcess(source_p, frames_to_process);

    scalar::Vsadd(source_p, 1, addend, dest_p, 1,
                  frame_counts.scalar_for_alignment);
    size_t i = frame_counts.scalar_for_alignment;
    if (frame_counts.sse_for_alignment > 0u) {
      sse::Vsadd(source_p + i, addend, dest_p + i,
                 frame_counts.sse_for_alignment);
      i += frame_counts.sse_for_alignment;
    }
    if (frame_counts.avx > 0u) {
      avx::Vsadd(source_p + i, addend, dest_p + i, frame_counts.avx);
      i += frame_counts.avx;
    }
    if (frame_counts.sse > 0u) {
      sse::Vsadd(source_p + i, addend, dest_p + i, frame_counts.sse);
      i += frame_counts.sse;
    }
    scalar::Vsadd(source_p + i, 1, addend, dest_p + i, 1, frame_counts.scalar);
    DCHECK_EQ(frames_to_process, i + frame_counts.scalar);
  } else {
    scalar::Vsadd(source_p, source_stride, addend, dest_p, dest_stride,
                  frames_to_process);
  }
}

ALWAYS_INLINE static void Vsvesq(const float* source_p,
                                 int source_stride,
                                 float* sum_p,
                                 uint32_t frames_to_process) {
  if (source_stride == 1) {
    const FrameCounts frame_counts =
        SplitFramesToProcess(source_p, frames_to_process);

    scalar::Vsvesq(source_p, 1, sum_p, frame_counts.scalar_for_alignment);
    size_t i = frame_counts.scalar_for_alignment;
    if (frame_counts.sse_for_alignment > 0u) {
      sse::Vsvesq(source_p + i, sum_p, frame_counts.sse_for_alignment);
      i += frame_counts.sse_for_alignment;
    }
    if (frame_counts.avx > 0u) {
      avx::Vsvesq(source_p + i, sum_p, frame_counts.avx);
      i += frame_counts.avx;
    }
    if (frame_counts.sse > 0u) {
      sse::Vsvesq(source_p + i, sum_p, frame_counts.sse);
      i += frame_counts.sse;
    }
    scalar::Vsvesq(source_p + i, 1, sum_p, frame_counts.scalar);
    DCHECK_EQ(frames_to_process, i + frame_counts.scalar);
  } else {
    scalar::Vsvesq(source_p, source_stride, sum_p, frames_to_process);
  }
}

ALWAYS_INLINE static void Zvmul(const float* real1p,
                                const float* imag1p,
                                const float* real2p,
                                const float* imag2p,
                                float* real_dest_p,
                                float* imag_dest_p,
                                uint32_t frames_to_process) {
  FrameCounts frame_counts = SplitFramesToProcess(real1p, frames_to_process);

  scalar::Zvmul(real1p, imag1p, real2p, imag2p, real_dest_p, imag_dest_p,
                frame_counts.scalar_for_alignment);
  size_t i = frame_counts.scalar_for_alignment;
  if (frame_counts.sse_for_alignment > 0u) {
    sse::Zvmul(real1p + i, imag1p + i, real2p + i, imag2p + i, real_dest_p + i,
               imag_dest_p + i, frame_counts.sse_for_alignment);
    i += frame_counts.sse_for_alignment;
  }
  if (frame_counts.avx > 0u) {
    avx::Zvmul(real1p + i, imag1p + i, real2p + i, imag2p + i, real_dest_p + i,
               imag_dest_p + i, frame_counts.avx);
    i += frame_counts.avx;
  }
  if (frame_counts.sse > 0u) {
    sse::Zvmul(real1p + i, imag1p + i, real2p + i, imag2p + i, real_dest_p + i,
               imag_dest_p + i, frame_counts.sse);
    i += frame_counts.sse;
  }
  scalar::Zvmul(real1p + i, imag1p + i, real2p + i, imag2p + i, real_dest_p + i,
                imag_dest_p + i, frame_counts.scalar);
  DCHECK_EQ(frames_to_process, i + frame_counts.scalar);
}

}  // namespace x86
}  // namespace vector_math
}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_AUDIO_CPU_X86_VECTOR_MATH_X86_H_
