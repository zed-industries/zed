/*
 *  Copyright (c) 2013 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

// This header file includes the inline functions in
// the fix point signal processing library.

#ifndef COMMON_AUDIO_SIGNAL_PROCESSING_INCLUDE_SPL_INL_MIPS_H_
#define COMMON_AUDIO_SIGNAL_PROCESSING_INCLUDE_SPL_INL_MIPS_H_

static __inline int32_t WEBRTC_SPL_MUL_16_16(int32_t a, int32_t b) {
  int32_t value32 = 0;
  int32_t a1 = 0, b1 = 0;

  __asm __volatile(
#if defined(MIPS32_R2_LE)
      "seh    %[a1],          %[a]                \n\t"
      "seh    %[b1],          %[b]                \n\t"
#else
      "sll    %[a1],          %[a],         16    \n\t"
      "sll    %[b1],          %[b],         16    \n\t"
      "sra    %[a1],          %[a1],        16    \n\t"
      "sra    %[b1],          %[b1],        16    \n\t"
#endif
      "mul    %[value32],     %[a1],  %[b1]       \n\t"
      : [value32] "=r"(value32), [a1] "=&r"(a1), [b1] "=&r"(b1)
      : [a] "r"(a), [b] "r"(b)
      : "hi", "lo");
  return value32;
}

static __inline int32_t WEBRTC_SPL_MUL_16_32_RSFT16(int16_t a, int32_t b) {
  int32_t value32 = 0, b1 = 0, b2 = 0;
  int32_t a1 = 0;

  __asm __volatile(
#if defined(MIPS32_R2_LE)
      "seh    %[a1],          %[a]                        \n\t"
#else
      "sll    %[a1],          %[a],           16          \n\t"
      "sra    %[a1],          %[a1],          16          \n\t"
#endif
      "andi   %[b2],          %[b],           0xFFFF      \n\t"
      "sra    %[b1],          %[b],           16          \n\t"
      "sra    %[b2],          %[b2],          1           \n\t"
      "mul    %[value32],     %[a1],          %[b1]       \n\t"
      "mul    %[b2],          %[a1],          %[b2]       \n\t"
      "addiu  %[b2],          %[b2],          0x4000      \n\t"
      "sra    %[b2],          %[b2],          15          \n\t"
      "addu   %[value32],     %[value32],     %[b2]       \n\t"
      : [value32] "=&r"(value32), [b1] "=&r"(b1), [b2] "=&r"(b2), [a1] "=&r"(a1)
      : [a] "r"(a), [b] "r"(b)
      : "hi", "lo");
  return value32;
}

#if defined(MIPS_DSP_R1_LE)
static __inline int16_t WebRtcSpl_SatW32ToW16(int32_t value32) {
  __asm __volatile(
      "shll_s.w   %[value32], %[value32], 16      \n\t"
      "sra        %[value32], %[value32], 16      \n\t"
      : [value32] "+r"(value32)
      :);
  int16_t out16 = (int16_t)value32;
  return out16;
}

static __inline int16_t WebRtcSpl_AddSatW16(int16_t a, int16_t b) {
  int32_t value32 = 0;

  __asm __volatile("addq_s.ph      %[value32],     %[a],   %[b]    \n\t"
                   : [value32] "=r"(value32)
                   : [a] "r"(a), [b] "r"(b));
  return (int16_t)value32;
}

static __inline int32_t WebRtcSpl_AddSatW32(int32_t l_var1, int32_t l_var2) {
  int32_t l_sum;

  __asm __volatile(
      "addq_s.w   %[l_sum],       %[l_var1],      %[l_var2]    \n\t"
      : [l_sum] "=r"(l_sum)
      : [l_var1] "r"(l_var1), [l_var2] "r"(l_var2));

  return l_sum;
}

static __inline int16_t WebRtcSpl_SubSatW16(int16_t var1, int16_t var2) {
  int32_t value32;

  __asm __volatile("subq_s.ph  %[value32], %[var1],    %[var2]     \n\t"
                   : [value32] "=r"(value32)
                   : [var1] "r"(var1), [var2] "r"(var2));

  return (int16_t)value32;
}

static __inline int32_t WebRtcSpl_SubSatW32(int32_t l_var1, int32_t l_var2) {
  int32_t l_diff;

  __asm __volatile(
      "subq_s.w   %[l_diff],      %[l_var1],      %[l_var2]    \n\t"
      : [l_diff] "=r"(l_diff)
      : [l_var1] "r"(l_var1), [l_var2] "r"(l_var2));

  return l_diff;
}
#endif

static __inline int16_t WebRtcSpl_GetSizeInBits(uint32_t n) {
  int bits = 0;
  int i32 = 32;

  __asm __volatile(
      "clz    %[bits],    %[n]                    \n\t"
      "subu   %[bits],    %[i32],     %[bits]     \n\t"
      : [bits] "=&r"(bits)
      : [n] "r"(n), [i32] "r"(i32));

  return (int16_t)bits;
}

static __inline int16_t WebRtcSpl_NormW32(int32_t a) {
  int zeros = 0;

  __asm __volatile(
      ".set       push                                \n\t"
      ".set       noreorder                           \n\t"
      "bnez       %[a],       1f                      \n\t"
      " sra       %[zeros],   %[a],       31          \n\t"
      "b          2f                                  \n\t"
      " move      %[zeros],   $zero                   \n\t"
      "1:                                              \n\t"
      "xor        %[zeros],   %[a],       %[zeros]    \n\t"
      "clz        %[zeros],   %[zeros]                \n\t"
      "addiu      %[zeros],   %[zeros],   -1          \n\t"
      "2:                                              \n\t"
      ".set       pop                                 \n\t"
      : [zeros] "=&r"(zeros)
      : [a] "r"(a));

  return (int16_t)zeros;
}

static __inline int16_t WebRtcSpl_NormU32(uint32_t a) {
  int zeros = 0;

  __asm __volatile("clz    %[zeros],   %[a]    \n\t"
                   : [zeros] "=r"(zeros)
                   : [a] "r"(a));

  return (int16_t)(zeros & 0x1f);
}

static __inline int16_t WebRtcSpl_NormW16(int16_t a) {
  int zeros = 0;
  int a0 = a << 16;

  __asm __volatile(
      ".set       push                                \n\t"
      ".set       noreorder                           \n\t"
      "bnez       %[a0],      1f                      \n\t"
      " sra       %[zeros],   %[a0],      31          \n\t"
      "b          2f                                  \n\t"
      " move      %[zeros],   $zero                   \n\t"
      "1:                                              \n\t"
      "xor        %[zeros],   %[a0],      %[zeros]    \n\t"
      "clz        %[zeros],   %[zeros]                \n\t"
      "addiu      %[zeros],   %[zeros],   -1          \n\t"
      "2:                                              \n\t"
      ".set       pop                                 \n\t"
      : [zeros] "=&r"(zeros)
      : [a0] "r"(a0));

  return (int16_t)zeros;
}

static __inline int32_t WebRtc_MulAccumW16(int16_t a, int16_t b, int32_t c) {
  int32_t res = 0, c1 = 0;
  __asm __volatile(
#if defined(MIPS32_R2_LE)
      "seh    %[a],       %[a]            \n\t"
      "seh    %[b],       %[b]            \n\t"
#else
      "sll    %[a],       %[a],   16      \n\t"
      "sll    %[b],       %[b],   16      \n\t"
      "sra    %[a],       %[a],   16      \n\t"
      "sra    %[b],       %[b],   16      \n\t"
#endif
      "mul    %[res],     %[a],   %[b]    \n\t"
      "addu   %[c1],      %[c],   %[res]  \n\t"
      : [c1] "=r"(c1), [res] "=&r"(res)
      : [a] "r"(a), [b] "r"(b), [c] "r"(c)
      : "hi", "lo");
  return (c1);
}

#endif  // COMMON_AUDIO_SIGNAL_PROCESSING_INCLUDE_SPL_INL_MIPS_H_
