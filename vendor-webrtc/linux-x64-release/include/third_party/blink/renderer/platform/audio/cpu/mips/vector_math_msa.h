// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_AUDIO_CPU_MIPS_VECTOR_MATH_MSA_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_AUDIO_CPU_MIPS_VECTOR_MATH_MSA_H_

#include <algorithm>

#include "third_party/blink/renderer/platform/audio/vector_math_scalar.h"
#include "third_party/blink/renderer/platform/cpu/mips/common_macros_msa.h"

namespace blink {
namespace vector_math {
namespace msa {

// TODO: Consider optimizing these.
using scalar::Conv;
using scalar::Vsvesq;
using scalar::Zvmul;

ALWAYS_INLINE static void Vadd(const float* source1p,
                               int source_stride1,
                               const float* source2p,
                               int source_stride2,
                               float* dest_p,
                               int dest_stride,
                               uint32_t frames_to_process) {
  int n = frames_to_process;

  if (source_stride1 == 1 && source_stride2 == 1 && dest_stride == 1) {
    v4f32 vSrc1P0, vSrc1P1, vSrc1P2, vSrc1P3, vSrc1P4, vSrc1P5, vSrc1P6,
        vSrc1P7;
    v4f32 vSrc2P0, vSrc2P1, vSrc2P2, vSrc2P3, vSrc2P4, vSrc2P5, vSrc2P6,
        vSrc2P7;
    v4f32 vDst0, vDst1, vDst2, vDst3, vDst4, vDst5, vDst6, vDst7;

    for (; n >= 32; n -= 32) {
      LD_SP8(source1p, 4, vSrc1P0, vSrc1P1, vSrc1P2, vSrc1P3, vSrc1P4, vSrc1P5,
             vSrc1P6, vSrc1P7);
      LD_SP8(source2p, 4, vSrc2P0, vSrc2P1, vSrc2P2, vSrc2P3, vSrc2P4, vSrc2P5,
             vSrc2P6, vSrc2P7);
      ADD4(vSrc1P0, vSrc2P0, vSrc1P1, vSrc2P1, vSrc1P2, vSrc2P2, vSrc1P3,
           vSrc2P3, vDst0, vDst1, vDst2, vDst3);
      ADD4(vSrc1P4, vSrc2P4, vSrc1P5, vSrc2P5, vSrc1P6, vSrc2P6, vSrc1P7,
           vSrc2P7, vDst4, vDst5, vDst6, vDst7);
      ST_SP8(vDst0, vDst1, vDst2, vDst3, vDst4, vDst5, vDst6, vDst7, dest_p, 4);
    }
  }

  scalar::Vadd(source1p, source_stride1, source2p, source_stride2, dest_p,
               dest_stride, n);
}

ALWAYS_INLINE static void Vclip(const float* source_p,
                                int source_stride,
                                const float* low_threshold_p,
                                const float* high_threshold_p,
                                float* dest_p,
                                int dest_stride,
                                uint32_t frames_to_process) {
  int n = frames_to_process;

  if (source_stride == 1 && dest_stride == 1) {
    v4f32 vSrc0, vSrc1, vSrc2, vSrc3, vSrc4, vSrc5, vSrc6, vSrc7;
    v4f32 vDst0, vDst1, vDst2, vDst3, vDst4, vDst5, vDst6, vDst7;
    v4f32 vLowThr, vHighThr;
    FloatInt lowThr, highThr;

    lowThr.floatVal = *low_threshold_p;
    highThr.floatVal = *high_threshold_p;
    vLowThr = (v4f32)__msa_fill_w(lowThr.intVal);
    vHighThr = (v4f32)__msa_fill_w(highThr.intVal);

    for (; n >= 32; n -= 32) {
      LD_SP8(source_p, 4, vSrc0, vSrc1, vSrc2, vSrc3, vSrc4, vSrc5, vSrc6,
             vSrc7);
      VCLIP4(vSrc0, vSrc1, vSrc2, vSrc3, vLowThr, vHighThr, vDst0, vDst1, vDst2,
             vDst3);
      VCLIP4(vSrc4, vSrc5, vSrc6, vSrc7, vLowThr, vHighThr, vDst4, vDst5, vDst6,
             vDst7);
      ST_SP8(vDst0, vDst1, vDst2, vDst3, vDst4, vDst5, vDst6, vDst7, dest_p, 4);
    }
  }

  scalar::Vclip(source_p, source_stride, low_threshold_p, high_threshold_p,
                dest_p, dest_stride, n);
}

ALWAYS_INLINE static void Vmaxmgv(const float* source_p,
                                  int source_stride,
                                  float* max_p,
                                  uint32_t frames_to_process) {
  int n = frames_to_process;

  if (source_stride == 1) {
    v4f32 vMax = {
        0,
    };
    v4f32 vSrc0, vSrc1, vSrc2, vSrc3, vSrc4, vSrc5, vSrc6, vSrc7;
    const v16i8 vSignBitMask = (v16i8)__msa_fill_w(0x7FFFFFFF);

    for (; n >= 32; n -= 32) {
      LD_SP8(source_p, 4, vSrc0, vSrc1, vSrc2, vSrc3, vSrc4, vSrc5, vSrc6,
             vSrc7);
      AND_W4_SP(vSrc0, vSrc1, vSrc2, vSrc3, vSignBitMask);
      VMAX_W4_SP(vSrc0, vSrc1, vSrc2, vSrc3, vMax);
      AND_W4_SP(vSrc4, vSrc5, vSrc6, vSrc7, vSignBitMask);
      VMAX_W4_SP(vSrc4, vSrc5, vSrc6, vSrc7, vMax);
    }

    *max_p = std::max(*max_p, vMax[0]);
    *max_p = std::max(*max_p, vMax[1]);
    *max_p = std::max(*max_p, vMax[2]);
    *max_p = std::max(*max_p, vMax[3]);
  }

  scalar::Vmaxmgv(source_p, source_stride, max_p, n);
}

ALWAYS_INLINE static void Vmul(const float* source1p,
                               int source_stride1,
                               const float* source2p,
                               int source_stride2,
                               float* dest_p,
                               int dest_stride,
                               uint32_t frames_to_process) {
  int n = frames_to_process;

  if (source_stride1 == 1 && source_stride2 == 1 && dest_stride == 1) {
    v4f32 vSrc1P0, vSrc1P1, vSrc1P2, vSrc1P3, vSrc1P4, vSrc1P5, vSrc1P6,
        vSrc1P7;
    v4f32 vSrc2P0, vSrc2P1, vSrc2P2, vSrc2P3, vSrc2P4, vSrc2P5, vSrc2P6,
        vSrc2P7;
    v4f32 vDst0, vDst1, vDst2, vDst3, vDst4, vDst5, vDst6, vDst7;

    for (; n >= 32; n -= 32) {
      LD_SP8(source1p, 4, vSrc1P0, vSrc1P1, vSrc1P2, vSrc1P3, vSrc1P4, vSrc1P5,
             vSrc1P6, vSrc1P7);
      LD_SP8(source2p, 4, vSrc2P0, vSrc2P1, vSrc2P2, vSrc2P3, vSrc2P4, vSrc2P5,
             vSrc2P6, vSrc2P7);
      MUL4(vSrc1P0, vSrc2P0, vSrc1P1, vSrc2P1, vSrc1P2, vSrc2P2, vSrc1P3,
           vSrc2P3, vDst0, vDst1, vDst2, vDst3);
      MUL4(vSrc1P4, vSrc2P4, vSrc1P5, vSrc2P5, vSrc1P6, vSrc2P6, vSrc1P7,
           vSrc2P7, vDst4, vDst5, vDst6, vDst7);
      ST_SP8(vDst0, vDst1, vDst2, vDst3, vDst4, vDst5, vDst6, vDst7, dest_p, 4);
    }
  }

  scalar::Vmul(source1p, source_stride1, source2p, source_stride2, dest_p,
               dest_stride, n);
}

ALWAYS_INLINE static void Vsma(const float* source_p,
                               int source_stride,
                               const float* scale,
                               float* dest_p,
                               int dest_stride,
                               uint32_t frames_to_process) {
  int n = frames_to_process;

  if (source_stride == 1 && dest_stride == 1) {
    float* destPCopy = dest_p;
    v4f32 vScale;
    v4f32 vSrc0, vSrc1, vSrc2, vSrc3, vSrc4, vSrc5, vSrc6, vSrc7;
    v4f32 vDst0, vDst1, vDst2, vDst3, vDst4, vDst5, vDst6, vDst7;
    FloatInt scaleVal;

    scaleVal.floatVal = *scale;
    vScale = (v4f32)__msa_fill_w(scaleVal.intVal);

    for (; n >= 32; n -= 32) {
      LD_SP8(source_p, 4, vSrc0, vSrc1, vSrc2, vSrc3, vSrc4, vSrc5, vSrc6,
             vSrc7);
      LD_SP8(destPCopy, 4, vDst0, vDst1, vDst2, vDst3, vDst4, vDst5, vDst6,
             vDst7);
      VSMA4(vSrc0, vSrc1, vSrc2, vSrc3, vDst0, vDst1, vDst2, vDst3, vScale);
      VSMA4(vSrc4, vSrc5, vSrc6, vSrc7, vDst4, vDst5, vDst6, vDst7, vScale);
      ST_SP8(vDst0, vDst1, vDst2, vDst3, vDst4, vDst5, vDst6, vDst7, dest_p, 4);
    }
  }

  scalar::Vsma(source_p, source_stride, scale, dest_p, dest_stride, n);
}

ALWAYS_INLINE static void Vsmul(const float* source_p,
                                int source_stride,
                                const float* scale,
                                float* dest_p,
                                int dest_stride,
                                uint32_t frames_to_process) {
  int n = frames_to_process;

  if (source_stride == 1 && dest_stride == 1) {
    v4f32 vScale;
    v4f32 vSrc0, vSrc1, vSrc2, vSrc3, vSrc4, vSrc5, vSrc6, vSrc7;
    v4f32 vDst0, vDst1, vDst2, vDst3, vDst4, vDst5, vDst6, vDst7;
    FloatInt scaleVal;

    scaleVal.floatVal = *scale;
    vScale = (v4f32)__msa_fill_w(scaleVal.intVal);

    for (; n >= 32; n -= 32) {
      LD_SP8(source_p, 4, vSrc0, vSrc1, vSrc2, vSrc3, vSrc4, vSrc5, vSrc6,
             vSrc7);
      VSMUL4(vSrc0, vSrc1, vSrc2, vSrc3, vDst0, vDst1, vDst2, vDst3, vScale);
      VSMUL4(vSrc4, vSrc5, vSrc6, vSrc7, vDst4, vDst5, vDst6, vDst7, vScale);
      ST_SP8(vDst0, vDst1, vDst2, vDst3, vDst4, vDst5, vDst6, vDst7, dest_p, 4);
    }
  }

  scalar::Vsmul(source_p, source_stride, scale, dest_p, dest_stride, n);
}

}  // namespace msa
}  // namespace vector_math
}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_AUDIO_CPU_MIPS_VECTOR_MATH_MSA_H_
