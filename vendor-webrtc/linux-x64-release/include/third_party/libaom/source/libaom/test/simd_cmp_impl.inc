/*
 * Copyright (c) 2016, Alliance for Open Media. All rights reserved.
 *
 * This source code is subject to the terms of the BSD 2 Clause License and
 * the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
 * was not distributed with this source code in the LICENSE file, you can
 * obtain it at www.aomedia.org/license/software. If the Alliance for Open
 * Media Patent License 1.0 was not distributed with this source code in the
 * PATENTS file, you can obtain it at www.aomedia.org/license/patent.
 */

#include <assert.h>
#include <string>

#include "config/aom_dsp_rtcd.h"

#include "test/acm_random.h"
// Inlining not forced for the compiler due to some tests calling
// SIMD_INLINE functions via function pointers
#undef SIMD_INLINE
#define SIMD_INLINE static inline
#include "aom_dsp/aom_simd.h"
#include "aom_dsp/simd/v256_intrinsics_c.h"

// Machine tuned code goes into this file. This file is included from
// simd_cmp_sse2.cc, simd_cmp_ssse3.cc etc which define the macros
// ARCH (=neon, sse2, ssse3, etc), SIMD_NAMESPACE and ARCH_POSTFIX().

#ifdef _MSC_VER
// Disable "value of intrinsic immediate argument 'value' is out of range
// 'lowerbound - upperbound'" warning. Visual Studio emits this warning though
// the parameters are conditionally checked in e.g., v256_shr_n_byte. Adding a
// mask doesn't always appear to be sufficient.
#pragma warning(disable : 4556)
#endif

using libaom_test::ACMRandom;

namespace SIMD_NAMESPACE {

// Wrap templates around intrinsics using immediate values
template <int shift>
v64 imm_v64_shl_n_byte(v64 a) {
  return v64_shl_n_byte(a, shift);
}
template <int shift>
v64 imm_v64_shr_n_byte(v64 a) {
  return v64_shr_n_byte(a, shift);
}
template <int shift>
v64 imm_v64_shl_n_8(v64 a) {
  return v64_shl_n_8(a, shift);
}
template <int shift>
v64 imm_v64_shr_n_u8(v64 a) {
  return v64_shr_n_u8(a, shift);
}
template <int shift>
v64 imm_v64_shr_n_s8(v64 a) {
  return v64_shr_n_s8(a, shift);
}
template <int shift>
v64 imm_v64_shl_n_16(v64 a) {
  return v64_shl_n_16(a, shift);
}
template <int shift>
v64 imm_v64_shr_n_u16(v64 a) {
  return v64_shr_n_u16(a, shift);
}
template <int shift>
v64 imm_v64_shr_n_s16(v64 a) {
  return v64_shr_n_s16(a, shift);
}
template <int shift>
v64 imm_v64_shl_n_32(v64 a) {
  return v64_shl_n_32(a, shift);
}
template <int shift>
v64 imm_v64_shr_n_u32(v64 a) {
  return v64_shr_n_u32(a, shift);
}
template <int shift>
v64 imm_v64_shr_n_s32(v64 a) {
  return v64_shr_n_s32(a, shift);
}
template <int shift>
v64 imm_v64_align(v64 a, v64 b) {
  return v64_align(a, b, shift);
}

// Wrap templates around corresponding C implementations of the above
template <int shift>
c_v64 c_imm_v64_shl_n_byte(c_v64 a) {
  return c_v64_shl_n_byte(a, shift);
}
template <int shift>
c_v64 c_imm_v64_shr_n_byte(c_v64 a) {
  return c_v64_shr_n_byte(a, shift);
}
template <int shift>
c_v64 c_imm_v64_shl_n_8(c_v64 a) {
  return c_v64_shl_n_8(a, shift);
}
template <int shift>
c_v64 c_imm_v64_shr_n_u8(c_v64 a) {
  return c_v64_shr_n_u8(a, shift);
}
template <int shift>
c_v64 c_imm_v64_shr_n_s8(c_v64 a) {
  return c_v64_shr_n_s8(a, shift);
}
template <int shift>
c_v64 c_imm_v64_shl_n_16(c_v64 a) {
  return c_v64_shl_n_16(a, shift);
}
template <int shift>
c_v64 c_imm_v64_shr_n_u16(c_v64 a) {
  return c_v64_shr_n_u16(a, shift);
}
template <int shift>
c_v64 c_imm_v64_shr_n_s16(c_v64 a) {
  return c_v64_shr_n_s16(a, shift);
}
template <int shift>
c_v64 c_imm_v64_shl_n_32(c_v64 a) {
  return c_v64_shl_n_32(a, shift);
}
template <int shift>
c_v64 c_imm_v64_shr_n_u32(c_v64 a) {
  return c_v64_shr_n_u32(a, shift);
}
template <int shift>
c_v64 c_imm_v64_shr_n_s32(c_v64 a) {
  return c_v64_shr_n_s32(a, shift);
}
template <int shift>
c_v64 c_imm_v64_align(c_v64 a, c_v64 b) {
  return c_v64_align(a, b, shift);
}

template <int shift>
v128 imm_v128_shl_n_byte(v128 a) {
  return v128_shl_n_byte(a, shift);
}
template <int shift>
v128 imm_v128_shr_n_byte(v128 a) {
  return v128_shr_n_byte(a, shift);
}
template <int shift>
v128 imm_v128_shl_n_8(v128 a) {
  return v128_shl_n_8(a, shift);
}
template <int shift>
v128 imm_v128_shr_n_u8(v128 a) {
  return v128_shr_n_u8(a, shift);
}
template <int shift>
v128 imm_v128_shr_n_s8(v128 a) {
  return v128_shr_n_s8(a, shift);
}
template <int shift>
v128 imm_v128_shl_n_16(v128 a) {
  return v128_shl_n_16(a, shift);
}
template <int shift>
v128 imm_v128_shr_n_u16(v128 a) {
  return v128_shr_n_u16(a, shift);
}
template <int shift>
v128 imm_v128_shr_n_s16(v128 a) {
  return v128_shr_n_s16(a, shift);
}
template <int shift>
v128 imm_v128_shl_n_32(v128 a) {
  return v128_shl_n_32(a, shift);
}
template <int shift>
v128 imm_v128_shr_n_u32(v128 a) {
  return v128_shr_n_u32(a, shift);
}
template <int shift>
v128 imm_v128_shr_n_s32(v128 a) {
  return v128_shr_n_s32(a, shift);
}
template <int shift>
v128 imm_v128_shl_n_64(v128 a) {
  return v128_shl_n_64(a, shift);
}
template <int shift>
v128 imm_v128_shr_n_u64(v128 a) {
  return v128_shr_n_u64(a, shift);
}
template <int shift>
v128 imm_v128_shr_n_s64(v128 a) {
  return v128_shr_n_s64(a, shift);
}
template <int shift>
v128 imm_v128_align(v128 a, v128 b) {
  return v128_align(a, b, shift);
}

template <int shift>
c_v128 c_imm_v128_shl_n_byte(c_v128 a) {
  return c_v128_shl_n_byte(a, shift);
}
template <int shift>
c_v128 c_imm_v128_shr_n_byte(c_v128 a) {
  return c_v128_shr_n_byte(a, shift);
}
template <int shift>
c_v128 c_imm_v128_shl_n_8(c_v128 a) {
  return c_v128_shl_n_8(a, shift);
}
template <int shift>
c_v128 c_imm_v128_shr_n_u8(c_v128 a) {
  return c_v128_shr_n_u8(a, shift);
}
template <int shift>
c_v128 c_imm_v128_shr_n_s8(c_v128 a) {
  return c_v128_shr_n_s8(a, shift);
}
template <int shift>
c_v128 c_imm_v128_shl_n_16(c_v128 a) {
  return c_v128_shl_n_16(a, shift);
}
template <int shift>
c_v128 c_imm_v128_shr_n_u16(c_v128 a) {
  return c_v128_shr_n_u16(a, shift);
}
template <int shift>
c_v128 c_imm_v128_shr_n_s16(c_v128 a) {
  return c_v128_shr_n_s16(a, shift);
}
template <int shift>
c_v128 c_imm_v128_shl_n_32(c_v128 a) {
  return c_v128_shl_n_32(a, shift);
}
template <int shift>
c_v128 c_imm_v128_shr_n_u32(c_v128 a) {
  return c_v128_shr_n_u32(a, shift);
}
template <int shift>
c_v128 c_imm_v128_shr_n_s32(c_v128 a) {
  return c_v128_shr_n_s32(a, shift);
}
template <int shift>
c_v128 c_imm_v128_shl_n_64(c_v128 a) {
  return c_v128_shl_n_64(a, shift);
}
template <int shift>
c_v128 c_imm_v128_shr_n_u64(c_v128 a) {
  return c_v128_shr_n_u64(a, shift);
}
template <int shift>
c_v128 c_imm_v128_shr_n_s64(c_v128 a) {
  return c_v128_shr_n_s64(a, shift);
}
template <int shift>
c_v128 c_imm_v128_align(c_v128 a, c_v128 b) {
  return c_v128_align(a, b, shift);
}

template <int shift>
v256 imm_v256_shl_n_word(v256 a) {
  return v256_shl_n_word(a, shift);
}
template <int shift>
v256 imm_v256_shr_n_word(v256 a) {
  return v256_shr_n_word(a, shift);
}
template <int shift>
v256 imm_v256_shl_n_byte(v256 a) {
  return v256_shl_n_byte(a, shift);
}
template <int shift>
v256 imm_v256_shr_n_byte(v256 a) {
  return v256_shr_n_byte(a, shift);
}
template <int shift>
v256 imm_v256_shl_n_8(v256 a) {
  return v256_shl_n_8(a, shift);
}
template <int shift>
v256 imm_v256_shr_n_u8(v256 a) {
  return v256_shr_n_u8(a, shift);
}
template <int shift>
v256 imm_v256_shr_n_s8(v256 a) {
  return v256_shr_n_s8(a, shift);
}
template <int shift>
v256 imm_v256_shl_n_16(v256 a) {
  return v256_shl_n_16(a, shift);
}
template <int shift>
v256 imm_v256_shr_n_u16(v256 a) {
  return v256_shr_n_u16(a, shift);
}
template <int shift>
v256 imm_v256_shr_n_s16(v256 a) {
  return v256_shr_n_s16(a, shift);
}
template <int shift>
v256 imm_v256_shl_n_32(v256 a) {
  return v256_shl_n_32(a, shift);
}
template <int shift>
v256 imm_v256_shr_n_u32(v256 a) {
  return v256_shr_n_u32(a, shift);
}
template <int shift>
v256 imm_v256_shr_n_s32(v256 a) {
  return v256_shr_n_s32(a, shift);
}
template <int shift>
v256 imm_v256_shl_n_64(v256 a) {
  return v256_shl_n_64(a, shift);
}
template <int shift>
v256 imm_v256_shr_n_u64(v256 a) {
  return v256_shr_n_u64(a, shift);
}
template <int shift>
v256 imm_v256_shr_n_s64(v256 a) {
  return v256_shr_n_s64(a, shift);
}
template <int shift>
v256 imm_v256_align(v256 a, v256 b) {
  return v256_align(a, b, shift);
}

template <int shift>
c_v256 c_imm_v256_shl_n_word(c_v256 a) {
  return c_v256_shl_n_word(a, shift);
}
template <int shift>
c_v256 c_imm_v256_shr_n_word(c_v256 a) {
  return c_v256_shr_n_word(a, shift);
}
template <int shift>
c_v256 c_imm_v256_shl_n_byte(c_v256 a) {
  return c_v256_shl_n_byte(a, shift);
}
template <int shift>
c_v256 c_imm_v256_shr_n_byte(c_v256 a) {
  return c_v256_shr_n_byte(a, shift);
}
template <int shift>
c_v256 c_imm_v256_shl_n_8(c_v256 a) {
  return c_v256_shl_n_8(a, shift);
}
template <int shift>
c_v256 c_imm_v256_shr_n_u8(c_v256 a) {
  return c_v256_shr_n_u8(a, shift);
}
template <int shift>
c_v256 c_imm_v256_shr_n_s8(c_v256 a) {
  return c_v256_shr_n_s8(a, shift);
}
template <int shift>
c_v256 c_imm_v256_shl_n_16(c_v256 a) {
  return c_v256_shl_n_16(a, shift);
}
template <int shift>
c_v256 c_imm_v256_shr_n_u16(c_v256 a) {
  return c_v256_shr_n_u16(a, shift);
}
template <int shift>
c_v256 c_imm_v256_shr_n_s16(c_v256 a) {
  return c_v256_shr_n_s16(a, shift);
}
template <int shift>
c_v256 c_imm_v256_shl_n_32(c_v256 a) {
  return c_v256_shl_n_32(a, shift);
}
template <int shift>
c_v256 c_imm_v256_shr_n_u32(c_v256 a) {
  return c_v256_shr_n_u32(a, shift);
}
template <int shift>
c_v256 c_imm_v256_shr_n_s32(c_v256 a) {
  return c_v256_shr_n_s32(a, shift);
}
template <int shift>
c_v256 c_imm_v256_shl_n_64(c_v256 a) {
  return c_v256_shl_n_64(a, shift);
}
template <int shift>
c_v256 c_imm_v256_shr_n_u64(c_v256 a) {
  return c_v256_shr_n_u64(a, shift);
}
template <int shift>
c_v256 c_imm_v256_shr_n_s64(c_v256 a) {
  return c_v256_shr_n_s64(a, shift);
}
template <int shift>
c_v256 c_imm_v256_align(c_v256 a, c_v256 b) {
  return c_v256_align(a, b, shift);
}

namespace {

// Wrappers around the the SAD and SSD functions
uint32_t v64_sad_u8(v64 a, v64 b) {
  return v64_sad_u8_sum(::v64_sad_u8(v64_sad_u8_init(), a, b));
}
uint32_t v64_ssd_u8(v64 a, v64 b) {
  return v64_ssd_u8_sum(::v64_ssd_u8(v64_ssd_u8_init(), a, b));
}

uint32_t c_v64_sad_u8(c_v64 a, c_v64 b) {
  return c_v64_sad_u8_sum(::c_v64_sad_u8(c_v64_sad_u8_init(), a, b));
}
uint32_t c_v64_ssd_u8(c_v64 a, c_v64 b) {
  return c_v64_ssd_u8_sum(::c_v64_ssd_u8(c_v64_ssd_u8_init(), a, b));
}
uint32_t v128_sad_u8(v128 a, v128 b) {
  return v128_sad_u8_sum(::v128_sad_u8(v128_sad_u8_init(), a, b));
}
uint32_t v128_ssd_u8(v128 a, v128 b) {
  return v128_ssd_u8_sum(::v128_ssd_u8(v128_ssd_u8_init(), a, b));
}
uint32_t c_v128_sad_u8(c_v128 a, c_v128 b) {
  return c_v128_sad_u8_sum(::c_v128_sad_u8(c_v128_sad_u8_init(), a, b));
}
uint32_t c_v128_ssd_u8(c_v128 a, c_v128 b) {
  return c_v128_ssd_u8_sum(::c_v128_ssd_u8(c_v128_ssd_u8_init(), a, b));
}
uint32_t v128_sad_u16(v128 a, v128 b) {
  return v128_sad_u16_sum(::v128_sad_u16(v128_sad_u16_init(), a, b));
}
uint64_t v128_ssd_s16(v128 a, v128 b) {
  return v128_ssd_s16_sum(::v128_ssd_s16(v128_ssd_s16_init(), a, b));
}
uint32_t c_v128_sad_u16(c_v128 a, c_v128 b) {
  return c_v128_sad_u16_sum(::c_v128_sad_u16(c_v128_sad_u16_init(), a, b));
}
uint64_t c_v128_ssd_s16(c_v128 a, c_v128 b) {
  return c_v128_ssd_s16_sum(::c_v128_ssd_s16(c_v128_ssd_s16_init(), a, b));
}
uint32_t v256_sad_u8(v256 a, v256 b) {
  return v256_sad_u8_sum(::v256_sad_u8(v256_sad_u8_init(), a, b));
}
uint32_t v256_ssd_u8(v256 a, v256 b) {
  return v256_ssd_u8_sum(::v256_ssd_u8(v256_ssd_u8_init(), a, b));
}
uint32_t c_v256_sad_u8(c_v256 a, c_v256 b) {
  return c_v256_sad_u8_sum(::c_v256_sad_u8(c_v256_sad_u8_init(), a, b));
}
uint32_t c_v256_ssd_u8(c_v256 a, c_v256 b) {
  return c_v256_ssd_u8_sum(::c_v256_ssd_u8(c_v256_ssd_u8_init(), a, b));
}
uint32_t v256_sad_u16(v256 a, v256 b) {
  return v256_sad_u16_sum(::v256_sad_u16(v256_sad_u16_init(), a, b));
}
uint64_t v256_ssd_s16(v256 a, v256 b) {
  return v256_ssd_s16_sum(::v256_ssd_s16(v256_ssd_s16_init(), a, b));
}
uint32_t c_v256_sad_u16(c_v256 a, c_v256 b) {
  return c_v256_sad_u16_sum(::c_v256_sad_u16(c_v256_sad_u16_init(), a, b));
}
uint64_t c_v256_ssd_s16(c_v256 a, c_v256 b) {
  return c_v256_ssd_s16_sum(::c_v256_ssd_s16(c_v256_ssd_s16_init(), a, b));
}

typedef void (*fptr)();

typedef struct {
  const char *name;
  fptr ref;
  fptr simd;
} mapping;

#define MAP(name) \
  { #name, reinterpret_cast < fptr>(c_##name), reinterpret_cast < fptr>(name) }

const mapping m[] = { MAP(v64_sad_u8),
                      MAP(v64_ssd_u8),
                      MAP(v64_add_8),
                      MAP(v64_add_16),
                      MAP(v64_sadd_s8),
                      MAP(v64_sadd_u8),
                      MAP(v64_sadd_s16),
                      MAP(v64_add_32),
                      MAP(v64_sub_8),
                      MAP(v64_ssub_u8),
                      MAP(v64_ssub_s8),
                      MAP(v64_sub_16),
                      MAP(v64_ssub_s16),
                      MAP(v64_ssub_u16),
                      MAP(v64_sub_32),
                      MAP(v64_ziplo_8),
                      MAP(v64_ziphi_8),
                      MAP(v64_ziplo_16),
                      MAP(v64_ziphi_16),
                      MAP(v64_ziplo_32),
                      MAP(v64_ziphi_32),
                      MAP(v64_pack_s32_u16),
                      MAP(v64_pack_s32_s16),
                      MAP(v64_pack_s16_u8),
                      MAP(v64_pack_s16_s8),
                      MAP(v64_unziphi_8),
                      MAP(v64_unziplo_8),
                      MAP(v64_unziphi_16),
                      MAP(v64_unziplo_16),
                      MAP(v64_or),
                      MAP(v64_xor),
                      MAP(v64_and),
                      MAP(v64_andn),
                      MAP(v64_mullo_s16),
                      MAP(v64_mulhi_s16),
                      MAP(v64_mullo_s32),
                      MAP(v64_madd_s16),
                      MAP(v64_madd_us8),
                      MAP(v64_avg_u8),
                      MAP(v64_rdavg_u8),
                      MAP(v64_rdavg_u16),
                      MAP(v64_avg_u16),
                      MAP(v64_min_u8),
                      MAP(v64_max_u8),
                      MAP(v64_min_s8),
                      MAP(v64_max_s8),
                      MAP(v64_min_s16),
                      MAP(v64_max_s16),
                      MAP(v64_cmpgt_s8),
                      MAP(v64_cmplt_s8),
                      MAP(v64_cmpeq_8),
                      MAP(v64_cmpgt_s16),
                      MAP(v64_cmplt_s16),
                      MAP(v64_cmpeq_16),
                      MAP(v64_shuffle_8),
                      MAP(imm_v64_align<1>),
                      MAP(imm_v64_align<2>),
                      MAP(imm_v64_align<3>),
                      MAP(imm_v64_align<4>),
                      MAP(imm_v64_align<5>),
                      MAP(imm_v64_align<6>),
                      MAP(imm_v64_align<7>),
                      MAP(v64_abs_s8),
                      MAP(v64_abs_s16),
                      MAP(v64_unpacklo_u8_s16),
                      MAP(v64_unpackhi_u8_s16),
                      MAP(v64_unpacklo_s8_s16),
                      MAP(v64_unpackhi_s8_s16),
                      MAP(v64_unpacklo_u16_s32),
                      MAP(v64_unpacklo_s16_s32),
                      MAP(v64_unpackhi_u16_s32),
                      MAP(v64_unpackhi_s16_s32),
                      MAP(imm_v64_shr_n_byte<1>),
                      MAP(imm_v64_shr_n_byte<2>),
                      MAP(imm_v64_shr_n_byte<3>),
                      MAP(imm_v64_shr_n_byte<4>),
                      MAP(imm_v64_shr_n_byte<5>),
                      MAP(imm_v64_shr_n_byte<6>),
                      MAP(imm_v64_shr_n_byte<7>),
                      MAP(imm_v64_shl_n_byte<1>),
                      MAP(imm_v64_shl_n_byte<2>),
                      MAP(imm_v64_shl_n_byte<3>),
                      MAP(imm_v64_shl_n_byte<4>),
                      MAP(imm_v64_shl_n_byte<5>),
                      MAP(imm_v64_shl_n_byte<6>),
                      MAP(imm_v64_shl_n_byte<7>),
                      MAP(imm_v64_shl_n_8<1>),
                      MAP(imm_v64_shl_n_8<2>),
                      MAP(imm_v64_shl_n_8<3>),
                      MAP(imm_v64_shl_n_8<4>),
                      MAP(imm_v64_shl_n_8<5>),
                      MAP(imm_v64_shl_n_8<6>),
                      MAP(imm_v64_shl_n_8<7>),
                      MAP(imm_v64_shr_n_u8<1>),
                      MAP(imm_v64_shr_n_u8<2>),
                      MAP(imm_v64_shr_n_u8<3>),
                      MAP(imm_v64_shr_n_u8<4>),
                      MAP(imm_v64_shr_n_u8<5>),
                      MAP(imm_v64_shr_n_u8<6>),
                      MAP(imm_v64_shr_n_u8<7>),
                      MAP(imm_v64_shr_n_s8<1>),
                      MAP(imm_v64_shr_n_s8<2>),
                      MAP(imm_v64_shr_n_s8<3>),
                      MAP(imm_v64_shr_n_s8<4>),
                      MAP(imm_v64_shr_n_s8<5>),
                      MAP(imm_v64_shr_n_s8<6>),
                      MAP(imm_v64_shr_n_s8<7>),
                      MAP(imm_v64_shl_n_16<1>),
                      MAP(imm_v64_shl_n_16<2>),
                      MAP(imm_v64_shl_n_16<4>),
                      MAP(imm_v64_shl_n_16<6>),
                      MAP(imm_v64_shl_n_16<8>),
                      MAP(imm_v64_shl_n_16<10>),
                      MAP(imm_v64_shl_n_16<12>),
                      MAP(imm_v64_shl_n_16<14>),
                      MAP(imm_v64_shr_n_u16<1>),
                      MAP(imm_v64_shr_n_u16<2>),
                      MAP(imm_v64_shr_n_u16<4>),
                      MAP(imm_v64_shr_n_u16<6>),
                      MAP(imm_v64_shr_n_u16<8>),
                      MAP(imm_v64_shr_n_u16<10>),
                      MAP(imm_v64_shr_n_u16<12>),
                      MAP(imm_v64_shr_n_u16<14>),
                      MAP(imm_v64_shr_n_s16<1>),
                      MAP(imm_v64_shr_n_s16<2>),
                      MAP(imm_v64_shr_n_s16<4>),
                      MAP(imm_v64_shr_n_s16<6>),
                      MAP(imm_v64_shr_n_s16<8>),
                      MAP(imm_v64_shr_n_s16<10>),
                      MAP(imm_v64_shr_n_s16<12>),
                      MAP(imm_v64_shr_n_s16<14>),
                      MAP(imm_v64_shl_n_32<1>),
                      MAP(imm_v64_shl_n_32<4>),
                      MAP(imm_v64_shl_n_32<8>),
                      MAP(imm_v64_shl_n_32<12>),
                      MAP(imm_v64_shl_n_32<16>),
                      MAP(imm_v64_shl_n_32<20>),
                      MAP(imm_v64_shl_n_32<24>),
                      MAP(imm_v64_shl_n_32<28>),
                      MAP(imm_v64_shr_n_u32<1>),
                      MAP(imm_v64_shr_n_u32<4>),
                      MAP(imm_v64_shr_n_u32<8>),
                      MAP(imm_v64_shr_n_u32<12>),
                      MAP(imm_v64_shr_n_u32<16>),
                      MAP(imm_v64_shr_n_u32<20>),
                      MAP(imm_v64_shr_n_u32<24>),
                      MAP(imm_v64_shr_n_u32<28>),
                      MAP(imm_v64_shr_n_s32<1>),
                      MAP(imm_v64_shr_n_s32<4>),
                      MAP(imm_v64_shr_n_s32<8>),
                      MAP(imm_v64_shr_n_s32<12>),
                      MAP(imm_v64_shr_n_s32<16>),
                      MAP(imm_v64_shr_n_s32<20>),
                      MAP(imm_v64_shr_n_s32<24>),
                      MAP(imm_v64_shr_n_s32<28>),
                      MAP(v64_shl_8),
                      MAP(v64_shr_u8),
                      MAP(v64_shr_s8),
                      MAP(v64_shl_16),
                      MAP(v64_shr_u16),
                      MAP(v64_shr_s16),
                      MAP(v64_shl_32),
                      MAP(v64_shr_u32),
                      MAP(v64_shr_s32),
                      MAP(v64_hadd_u8),
                      MAP(v64_hadd_s16),
                      MAP(v64_dotp_s16),
                      MAP(v64_dotp_su8),
                      MAP(v64_u64),
                      MAP(v64_low_u32),
                      MAP(v64_high_u32),
                      MAP(v64_low_s32),
                      MAP(v64_high_s32),
                      MAP(v64_dup_8),
                      MAP(v64_dup_16),
                      MAP(v64_dup_32),
                      MAP(v64_from_32),
                      MAP(v64_zero),
                      MAP(v64_from_16),
                      MAP(v128_sad_u8),
                      MAP(v128_ssd_u8),
                      MAP(v128_sad_u16),
                      MAP(v128_ssd_s16),
                      MAP(v128_add_8),
                      MAP(v128_add_16),
                      MAP(v128_sadd_s8),
                      MAP(v128_sadd_u8),
                      MAP(v128_sadd_s16),
                      MAP(v128_add_32),
                      MAP(v128_add_64),
                      MAP(v128_sub_8),
                      MAP(v128_ssub_u8),
                      MAP(v128_ssub_s8),
                      MAP(v128_sub_16),
                      MAP(v128_ssub_s16),
                      MAP(v128_ssub_u16),
                      MAP(v128_sub_32),
                      MAP(v128_sub_64),
                      MAP(v128_ziplo_8),
                      MAP(v128_ziphi_8),
                      MAP(v128_ziplo_16),
                      MAP(v128_ziphi_16),
                      MAP(v128_ziplo_32),
                      MAP(v128_ziphi_32),
                      MAP(v128_ziplo_64),
                      MAP(v128_ziphi_64),
                      MAP(v128_unziphi_8),
                      MAP(v128_unziplo_8),
                      MAP(v128_unziphi_16),
                      MAP(v128_unziplo_16),
                      MAP(v128_unziphi_32),
                      MAP(v128_unziplo_32),
                      MAP(v128_pack_s32_u16),
                      MAP(v128_pack_s32_s16),
                      MAP(v128_pack_s16_u8),
                      MAP(v128_pack_s16_s8),
                      MAP(v128_or),
                      MAP(v128_xor),
                      MAP(v128_and),
                      MAP(v128_andn),
                      MAP(v128_mullo_s16),
                      MAP(v128_mulhi_s16),
                      MAP(v128_mullo_s32),
                      MAP(v128_madd_s16),
                      MAP(v128_madd_us8),
                      MAP(v128_avg_u8),
                      MAP(v128_rdavg_u8),
                      MAP(v128_rdavg_u16),
                      MAP(v128_avg_u16),
                      MAP(v128_min_u8),
                      MAP(v128_max_u8),
                      MAP(v128_min_s8),
                      MAP(v128_max_s8),
                      MAP(v128_min_s16),
                      MAP(v128_max_s16),
                      MAP(v128_min_s32),
                      MAP(v128_max_s32),
                      MAP(v128_cmpgt_s8),
                      MAP(v128_cmplt_s8),
                      MAP(v128_cmpeq_8),
                      MAP(v128_cmpgt_s16),
                      MAP(v128_cmpeq_16),
                      MAP(v128_cmplt_s16),
                      MAP(v128_cmpgt_s32),
                      MAP(v128_cmpeq_32),
                      MAP(v128_cmplt_s32),
                      MAP(v128_shuffle_8),
                      MAP(imm_v128_align<1>),
                      MAP(imm_v128_align<2>),
                      MAP(imm_v128_align<3>),
                      MAP(imm_v128_align<4>),
                      MAP(imm_v128_align<5>),
                      MAP(imm_v128_align<6>),
                      MAP(imm_v128_align<7>),
                      MAP(imm_v128_align<8>),
                      MAP(imm_v128_align<9>),
                      MAP(imm_v128_align<10>),
                      MAP(imm_v128_align<11>),
                      MAP(imm_v128_align<12>),
                      MAP(imm_v128_align<13>),
                      MAP(imm_v128_align<14>),
                      MAP(imm_v128_align<15>),
                      MAP(v128_abs_s8),
                      MAP(v128_abs_s16),
                      MAP(v128_padd_u8),
                      MAP(v128_padd_s16),
                      MAP(v128_unpacklo_u16_s32),
                      MAP(v128_unpacklo_s16_s32),
                      MAP(v128_unpackhi_u16_s32),
                      MAP(v128_unpackhi_s16_s32),
                      MAP(imm_v128_shr_n_byte<1>),
                      MAP(imm_v128_shr_n_byte<2>),
                      MAP(imm_v128_shr_n_byte<3>),
                      MAP(imm_v128_shr_n_byte<4>),
                      MAP(imm_v128_shr_n_byte<5>),
                      MAP(imm_v128_shr_n_byte<6>),
                      MAP(imm_v128_shr_n_byte<7>),
                      MAP(imm_v128_shr_n_byte<8>),
                      MAP(imm_v128_shr_n_byte<9>),
                      MAP(imm_v128_shr_n_byte<10>),
                      MAP(imm_v128_shr_n_byte<11>),
                      MAP(imm_v128_shr_n_byte<12>),
                      MAP(imm_v128_shr_n_byte<13>),
                      MAP(imm_v128_shr_n_byte<14>),
                      MAP(imm_v128_shr_n_byte<15>),
                      MAP(imm_v128_shl_n_byte<1>),
                      MAP(imm_v128_shl_n_byte<2>),
                      MAP(imm_v128_shl_n_byte<3>),
                      MAP(imm_v128_shl_n_byte<4>),
                      MAP(imm_v128_shl_n_byte<5>),
                      MAP(imm_v128_shl_n_byte<6>),
                      MAP(imm_v128_shl_n_byte<7>),
                      MAP(imm_v128_shl_n_byte<8>),
                      MAP(imm_v128_shl_n_byte<9>),
                      MAP(imm_v128_shl_n_byte<10>),
                      MAP(imm_v128_shl_n_byte<11>),
                      MAP(imm_v128_shl_n_byte<12>),
                      MAP(imm_v128_shl_n_byte<13>),
                      MAP(imm_v128_shl_n_byte<14>),
                      MAP(imm_v128_shl_n_byte<15>),
                      MAP(imm_v128_shl_n_8<1>),
                      MAP(imm_v128_shl_n_8<2>),
                      MAP(imm_v128_shl_n_8<3>),
                      MAP(imm_v128_shl_n_8<4>),
                      MAP(imm_v128_shl_n_8<5>),
                      MAP(imm_v128_shl_n_8<6>),
                      MAP(imm_v128_shl_n_8<7>),
                      MAP(imm_v128_shr_n_u8<1>),
                      MAP(imm_v128_shr_n_u8<2>),
                      MAP(imm_v128_shr_n_u8<3>),
                      MAP(imm_v128_shr_n_u8<4>),
                      MAP(imm_v128_shr_n_u8<5>),
                      MAP(imm_v128_shr_n_u8<6>),
                      MAP(imm_v128_shr_n_u8<7>),
                      MAP(imm_v128_shr_n_s8<1>),
                      MAP(imm_v128_shr_n_s8<2>),
                      MAP(imm_v128_shr_n_s8<3>),
                      MAP(imm_v128_shr_n_s8<4>),
                      MAP(imm_v128_shr_n_s8<5>),
                      MAP(imm_v128_shr_n_s8<6>),
                      MAP(imm_v128_shr_n_s8<7>),
                      MAP(imm_v128_shl_n_16<1>),
                      MAP(imm_v128_shl_n_16<2>),
                      MAP(imm_v128_shl_n_16<4>),
                      MAP(imm_v128_shl_n_16<6>),
                      MAP(imm_v128_shl_n_16<8>),
                      MAP(imm_v128_shl_n_16<10>),
                      MAP(imm_v128_shl_n_16<12>),
                      MAP(imm_v128_shl_n_16<14>),
                      MAP(imm_v128_shr_n_u16<1>),
                      MAP(imm_v128_shr_n_u16<2>),
                      MAP(imm_v128_shr_n_u16<4>),
                      MAP(imm_v128_shr_n_u16<6>),
                      MAP(imm_v128_shr_n_u16<8>),
                      MAP(imm_v128_shr_n_u16<10>),
                      MAP(imm_v128_shr_n_u16<12>),
                      MAP(imm_v128_shr_n_u16<14>),
                      MAP(imm_v128_shr_n_s16<1>),
                      MAP(imm_v128_shr_n_s16<2>),
                      MAP(imm_v128_shr_n_s16<4>),
                      MAP(imm_v128_shr_n_s16<6>),
                      MAP(imm_v128_shr_n_s16<8>),
                      MAP(imm_v128_shr_n_s16<10>),
                      MAP(imm_v128_shr_n_s16<12>),
                      MAP(imm_v128_shr_n_s16<14>),
                      MAP(imm_v128_shl_n_32<1>),
                      MAP(imm_v128_shl_n_32<4>),
                      MAP(imm_v128_shl_n_32<8>),
                      MAP(imm_v128_shl_n_32<12>),
                      MAP(imm_v128_shl_n_32<16>),
                      MAP(imm_v128_shl_n_32<20>),
                      MAP(imm_v128_shl_n_32<24>),
                      MAP(imm_v128_shl_n_32<28>),
                      MAP(imm_v128_shr_n_u32<1>),
                      MAP(imm_v128_shr_n_u32<4>),
                      MAP(imm_v128_shr_n_u32<8>),
                      MAP(imm_v128_shr_n_u32<12>),
                      MAP(imm_v128_shr_n_u32<16>),
                      MAP(imm_v128_shr_n_u32<20>),
                      MAP(imm_v128_shr_n_u32<24>),
                      MAP(imm_v128_shr_n_u32<28>),
                      MAP(imm_v128_shr_n_s32<1>),
                      MAP(imm_v128_shr_n_s32<4>),
                      MAP(imm_v128_shr_n_s32<8>),
                      MAP(imm_v128_shr_n_s32<12>),
                      MAP(imm_v128_shr_n_s32<16>),
                      MAP(imm_v128_shr_n_s32<20>),
                      MAP(imm_v128_shr_n_s32<24>),
                      MAP(imm_v128_shr_n_s32<28>),
                      MAP(imm_v128_shl_n_64<1>),
                      MAP(imm_v128_shl_n_64<4>),
                      MAP(imm_v128_shl_n_64<8>),
                      MAP(imm_v128_shl_n_64<12>),
                      MAP(imm_v128_shl_n_64<16>),
                      MAP(imm_v128_shl_n_64<20>),
                      MAP(imm_v128_shl_n_64<24>),
                      MAP(imm_v128_shl_n_64<28>),
                      MAP(imm_v128_shl_n_64<32>),
                      MAP(imm_v128_shl_n_64<36>),
                      MAP(imm_v128_shl_n_64<40>),
                      MAP(imm_v128_shl_n_64<44>),
                      MAP(imm_v128_shl_n_64<48>),
                      MAP(imm_v128_shl_n_64<52>),
                      MAP(imm_v128_shl_n_64<56>),
                      MAP(imm_v128_shl_n_64<60>),
                      MAP(imm_v128_shr_n_u64<1>),
                      MAP(imm_v128_shr_n_u64<4>),
                      MAP(imm_v128_shr_n_u64<8>),
                      MAP(imm_v128_shr_n_u64<12>),
                      MAP(imm_v128_shr_n_u64<16>),
                      MAP(imm_v128_shr_n_u64<20>),
                      MAP(imm_v128_shr_n_u64<24>),
                      MAP(imm_v128_shr_n_u64<28>),
                      MAP(imm_v128_shr_n_u64<32>),
                      MAP(imm_v128_shr_n_u64<36>),
                      MAP(imm_v128_shr_n_u64<40>),
                      MAP(imm_v128_shr_n_u64<44>),
                      MAP(imm_v128_shr_n_u64<48>),
                      MAP(imm_v128_shr_n_u64<52>),
                      MAP(imm_v128_shr_n_u64<56>),
                      MAP(imm_v128_shr_n_u64<60>),
                      MAP(imm_v128_shr_n_s64<1>),
                      MAP(imm_v128_shr_n_s64<4>),
                      MAP(imm_v128_shr_n_s64<8>),
                      MAP(imm_v128_shr_n_s64<12>),
                      MAP(imm_v128_shr_n_s64<16>),
                      MAP(imm_v128_shr_n_s64<20>),
                      MAP(imm_v128_shr_n_s64<24>),
                      MAP(imm_v128_shr_n_s64<28>),
                      MAP(imm_v128_shr_n_s64<32>),
                      MAP(imm_v128_shr_n_s64<36>),
                      MAP(imm_v128_shr_n_s64<40>),
                      MAP(imm_v128_shr_n_s64<44>),
                      MAP(imm_v128_shr_n_s64<48>),
                      MAP(imm_v128_shr_n_s64<52>),
                      MAP(imm_v128_shr_n_s64<56>),
                      MAP(imm_v128_shr_n_s64<60>),
                      MAP(v128_from_v64),
                      MAP(v128_zip_8),
                      MAP(v128_zip_16),
                      MAP(v128_zip_32),
                      MAP(v128_mul_s16),
                      MAP(v128_unpack_u8_s16),
                      MAP(v128_unpack_s8_s16),
                      MAP(v128_unpack_u16_s32),
                      MAP(v128_unpack_s16_s32),
                      MAP(v128_shl_8),
                      MAP(v128_shr_u8),
                      MAP(v128_shr_s8),
                      MAP(v128_shl_16),
                      MAP(v128_shr_u16),
                      MAP(v128_shr_s16),
                      MAP(v128_shl_32),
                      MAP(v128_shr_u32),
                      MAP(v128_shr_s32),
                      MAP(v128_shl_64),
                      MAP(v128_shr_u64),
                      MAP(v128_shr_s64),
                      MAP(v128_hadd_u8),
                      MAP(v128_dotp_su8),
                      MAP(v128_dotp_s16),
                      MAP(v128_dotp_s32),
                      MAP(v128_low_u32),
                      MAP(v128_low_v64),
                      MAP(v128_high_v64),
                      MAP(v128_from_64),
                      MAP(v128_from_32),
                      MAP(v128_movemask_8),
                      MAP(v128_zero),
                      MAP(v128_dup_8),
                      MAP(v128_dup_16),
                      MAP(v128_dup_32),
                      MAP(v128_dup_64),
                      MAP(v128_unpacklo_u8_s16),
                      MAP(v128_unpackhi_u8_s16),
                      MAP(v128_unpacklo_s8_s16),
                      MAP(v128_unpackhi_s8_s16),
                      MAP(v128_blend_8),
                      MAP(u32_load_unaligned),
                      MAP(u32_store_unaligned),
                      MAP(v64_load_unaligned),
                      MAP(v64_store_unaligned),
                      MAP(v128_load_unaligned),
                      MAP(v128_store_unaligned),
                      MAP(v256_sad_u8),
                      MAP(v256_ssd_u8),
                      MAP(v256_sad_u16),
                      MAP(v256_ssd_s16),
                      MAP(v256_hadd_u8),
                      MAP(v256_low_u64),
                      MAP(v256_dotp_su8),
                      MAP(v256_dotp_s16),
                      MAP(v256_dotp_s32),
                      MAP(v256_add_8),
                      MAP(v256_add_16),
                      MAP(v256_sadd_s8),
                      MAP(v256_sadd_u8),
                      MAP(v256_sadd_s16),
                      MAP(v256_add_32),
                      MAP(v256_add_64),
                      MAP(v256_sub_8),
                      MAP(v256_ssub_u8),
                      MAP(v256_ssub_s8),
                      MAP(v256_sub_16),
                      MAP(v256_ssub_u16),
                      MAP(v256_ssub_s16),
                      MAP(v256_sub_32),
                      MAP(v256_sub_64),
                      MAP(v256_ziplo_8),
                      MAP(v256_ziphi_8),
                      MAP(v256_ziplo_16),
                      MAP(v256_ziphi_16),
                      MAP(v256_ziplo_32),
                      MAP(v256_ziphi_32),
                      MAP(v256_ziplo_64),
                      MAP(v256_ziphi_64),
                      MAP(v256_unziphi_8),
                      MAP(v256_unziplo_8),
                      MAP(v256_unziphi_16),
                      MAP(v256_unziplo_16),
                      MAP(v256_unziphi_32),
                      MAP(v256_unziplo_32),
                      MAP(v256_unziphi_64),
                      MAP(v256_unziplo_64),
                      MAP(v256_pack_s32_u16),
                      MAP(v256_pack_s32_s16),
                      MAP(v256_pack_s16_u8),
                      MAP(v256_pack_s16_s8),
                      MAP(v256_or),
                      MAP(v256_xor),
                      MAP(v256_and),
                      MAP(v256_andn),
                      MAP(v256_mullo_s16),
                      MAP(v256_mulhi_s16),
                      MAP(v256_mullo_s32),
                      MAP(v256_madd_s16),
                      MAP(v256_madd_us8),
                      MAP(v256_avg_u8),
                      MAP(v256_rdavg_u8),
                      MAP(v256_rdavg_u16),
                      MAP(v256_avg_u16),
                      MAP(v256_min_u8),
                      MAP(v256_max_u8),
                      MAP(v256_min_s8),
                      MAP(v256_max_s8),
                      MAP(v256_min_s16),
                      MAP(v256_max_s16),
                      MAP(v256_min_s32),
                      MAP(v256_max_s32),
                      MAP(v256_cmpgt_s8),
                      MAP(v256_cmplt_s8),
                      MAP(v256_cmpeq_8),
                      MAP(v256_cmpgt_s16),
                      MAP(v256_cmplt_s16),
                      MAP(v256_cmpeq_16),
                      MAP(v256_cmpgt_s32),
                      MAP(v256_cmplt_s32),
                      MAP(v256_cmpeq_32),
                      MAP(v256_shuffle_8),
                      MAP(v256_pshuffle_8),
                      MAP(v256_wideshuffle_8),
                      MAP(imm_v256_align<1>),
                      MAP(imm_v256_align<2>),
                      MAP(imm_v256_align<3>),
                      MAP(imm_v256_align<4>),
                      MAP(imm_v256_align<5>),
                      MAP(imm_v256_align<6>),
                      MAP(imm_v256_align<7>),
                      MAP(imm_v256_align<8>),
                      MAP(imm_v256_align<9>),
                      MAP(imm_v256_align<10>),
                      MAP(imm_v256_align<11>),
                      MAP(imm_v256_align<12>),
                      MAP(imm_v256_align<13>),
                      MAP(imm_v256_align<14>),
                      MAP(imm_v256_align<15>),
                      MAP(imm_v256_align<16>),
                      MAP(imm_v256_align<17>),
                      MAP(imm_v256_align<18>),
                      MAP(imm_v256_align<19>),
                      MAP(imm_v256_align<20>),
                      MAP(imm_v256_align<21>),
                      MAP(imm_v256_align<22>),
                      MAP(imm_v256_align<23>),
                      MAP(imm_v256_align<24>),
                      MAP(imm_v256_align<25>),
                      MAP(imm_v256_align<26>),
                      MAP(imm_v256_align<27>),
                      MAP(imm_v256_align<28>),
                      MAP(imm_v256_align<29>),
                      MAP(imm_v256_align<30>),
                      MAP(imm_v256_align<31>),
                      MAP(v256_from_v128),
                      MAP(v256_zip_8),
                      MAP(v256_zip_16),
                      MAP(v256_zip_32),
                      MAP(v256_mul_s16),
                      MAP(v256_unpack_u8_s16),
                      MAP(v256_unpack_s8_s16),
                      MAP(v256_unpack_u16_s32),
                      MAP(v256_unpack_s16_s32),
                      MAP(v256_shl_8),
                      MAP(v256_shr_u8),
                      MAP(v256_shr_s8),
                      MAP(v256_shl_16),
                      MAP(v256_shr_u16),
                      MAP(v256_shr_s16),
                      MAP(v256_shl_32),
                      MAP(v256_shr_u32),
                      MAP(v256_shr_s32),
                      MAP(v256_shl_64),
                      MAP(v256_shr_u64),
                      MAP(v256_shr_s64),
                      MAP(v256_abs_s8),
                      MAP(v256_abs_s16),
                      MAP(v256_padd_u8),
                      MAP(v256_padd_s16),
                      MAP(v256_unpacklo_u16_s32),
                      MAP(v256_unpacklo_s16_s32),
                      MAP(v256_unpackhi_u16_s32),
                      MAP(v256_unpackhi_s16_s32),
                      MAP(imm_v256_shr_n_word<1>),
                      MAP(imm_v256_shr_n_word<2>),
                      MAP(imm_v256_shr_n_word<3>),
                      MAP(imm_v256_shr_n_word<4>),
                      MAP(imm_v256_shr_n_word<5>),
                      MAP(imm_v256_shr_n_word<6>),
                      MAP(imm_v256_shr_n_word<7>),
                      MAP(imm_v256_shr_n_word<8>),
                      MAP(imm_v256_shr_n_word<9>),
                      MAP(imm_v256_shr_n_word<10>),
                      MAP(imm_v256_shr_n_word<11>),
                      MAP(imm_v256_shr_n_word<12>),
                      MAP(imm_v256_shr_n_word<13>),
                      MAP(imm_v256_shr_n_word<14>),
                      MAP(imm_v256_shr_n_word<15>),
                      MAP(imm_v256_shl_n_word<1>),
                      MAP(imm_v256_shl_n_word<2>),
                      MAP(imm_v256_shl_n_word<3>),
                      MAP(imm_v256_shl_n_word<4>),
                      MAP(imm_v256_shl_n_word<5>),
                      MAP(imm_v256_shl_n_word<6>),
                      MAP(imm_v256_shl_n_word<7>),
                      MAP(imm_v256_shl_n_word<8>),
                      MAP(imm_v256_shl_n_word<9>),
                      MAP(imm_v256_shl_n_word<10>),
                      MAP(imm_v256_shl_n_word<11>),
                      MAP(imm_v256_shl_n_word<12>),
                      MAP(imm_v256_shl_n_word<13>),
                      MAP(imm_v256_shl_n_word<14>),
                      MAP(imm_v256_shl_n_word<15>),
                      MAP(imm_v256_shr_n_byte<1>),
                      MAP(imm_v256_shr_n_byte<2>),
                      MAP(imm_v256_shr_n_byte<3>),
                      MAP(imm_v256_shr_n_byte<4>),
                      MAP(imm_v256_shr_n_byte<5>),
                      MAP(imm_v256_shr_n_byte<6>),
                      MAP(imm_v256_shr_n_byte<7>),
                      MAP(imm_v256_shr_n_byte<8>),
                      MAP(imm_v256_shr_n_byte<9>),
                      MAP(imm_v256_shr_n_byte<10>),
                      MAP(imm_v256_shr_n_byte<11>),
                      MAP(imm_v256_shr_n_byte<12>),
                      MAP(imm_v256_shr_n_byte<13>),
                      MAP(imm_v256_shr_n_byte<14>),
                      MAP(imm_v256_shr_n_byte<15>),
                      MAP(imm_v256_shr_n_byte<16>),
                      MAP(imm_v256_shr_n_byte<17>),
                      MAP(imm_v256_shr_n_byte<18>),
                      MAP(imm_v256_shr_n_byte<19>),
                      MAP(imm_v256_shr_n_byte<20>),
                      MAP(imm_v256_shr_n_byte<21>),
                      MAP(imm_v256_shr_n_byte<22>),
                      MAP(imm_v256_shr_n_byte<23>),
                      MAP(imm_v256_shr_n_byte<24>),
                      MAP(imm_v256_shr_n_byte<25>),
                      MAP(imm_v256_shr_n_byte<26>),
                      MAP(imm_v256_shr_n_byte<27>),
                      MAP(imm_v256_shr_n_byte<28>),
                      MAP(imm_v256_shr_n_byte<29>),
                      MAP(imm_v256_shr_n_byte<30>),
                      MAP(imm_v256_shr_n_byte<31>),
                      MAP(imm_v256_shl_n_byte<1>),
                      MAP(imm_v256_shl_n_byte<2>),
                      MAP(imm_v256_shl_n_byte<3>),
                      MAP(imm_v256_shl_n_byte<4>),
                      MAP(imm_v256_shl_n_byte<5>),
                      MAP(imm_v256_shl_n_byte<6>),
                      MAP(imm_v256_shl_n_byte<7>),
                      MAP(imm_v256_shl_n_byte<8>),
                      MAP(imm_v256_shl_n_byte<9>),
                      MAP(imm_v256_shl_n_byte<10>),
                      MAP(imm_v256_shl_n_byte<11>),
                      MAP(imm_v256_shl_n_byte<12>),
                      MAP(imm_v256_shl_n_byte<13>),
                      MAP(imm_v256_shl_n_byte<14>),
                      MAP(imm_v256_shl_n_byte<15>),
                      MAP(imm_v256_shl_n_byte<16>),
                      MAP(imm_v256_shl_n_byte<17>),
                      MAP(imm_v256_shl_n_byte<18>),
                      MAP(imm_v256_shl_n_byte<19>),
                      MAP(imm_v256_shl_n_byte<20>),
                      MAP(imm_v256_shl_n_byte<21>),
                      MAP(imm_v256_shl_n_byte<22>),
                      MAP(imm_v256_shl_n_byte<23>),
                      MAP(imm_v256_shl_n_byte<24>),
                      MAP(imm_v256_shl_n_byte<25>),
                      MAP(imm_v256_shl_n_byte<26>),
                      MAP(imm_v256_shl_n_byte<27>),
                      MAP(imm_v256_shl_n_byte<28>),
                      MAP(imm_v256_shl_n_byte<29>),
                      MAP(imm_v256_shl_n_byte<30>),
                      MAP(imm_v256_shl_n_byte<31>),
                      MAP(imm_v256_shl_n_8<1>),
                      MAP(imm_v256_shl_n_8<2>),
                      MAP(imm_v256_shl_n_8<3>),
                      MAP(imm_v256_shl_n_8<4>),
                      MAP(imm_v256_shl_n_8<5>),
                      MAP(imm_v256_shl_n_8<6>),
                      MAP(imm_v256_shl_n_8<7>),
                      MAP(imm_v256_shr_n_u8<1>),
                      MAP(imm_v256_shr_n_u8<2>),
                      MAP(imm_v256_shr_n_u8<3>),
                      MAP(imm_v256_shr_n_u8<4>),
                      MAP(imm_v256_shr_n_u8<5>),
                      MAP(imm_v256_shr_n_u8<6>),
                      MAP(imm_v256_shr_n_u8<7>),
                      MAP(imm_v256_shr_n_s8<1>),
                      MAP(imm_v256_shr_n_s8<2>),
                      MAP(imm_v256_shr_n_s8<3>),
                      MAP(imm_v256_shr_n_s8<4>),
                      MAP(imm_v256_shr_n_s8<5>),
                      MAP(imm_v256_shr_n_s8<6>),
                      MAP(imm_v256_shr_n_s8<7>),
                      MAP(imm_v256_shl_n_16<1>),
                      MAP(imm_v256_shl_n_16<2>),
                      MAP(imm_v256_shl_n_16<4>),
                      MAP(imm_v256_shl_n_16<6>),
                      MAP(imm_v256_shl_n_16<8>),
                      MAP(imm_v256_shl_n_16<10>),
                      MAP(imm_v256_shl_n_16<12>),
                      MAP(imm_v256_shl_n_16<14>),
                      MAP(imm_v256_shr_n_u16<1>),
                      MAP(imm_v256_shr_n_u16<2>),
                      MAP(imm_v256_shr_n_u16<4>),
                      MAP(imm_v256_shr_n_u16<6>),
                      MAP(imm_v256_shr_n_u16<8>),
                      MAP(imm_v256_shr_n_u16<10>),
                      MAP(imm_v256_shr_n_u16<12>),
                      MAP(imm_v256_shr_n_u16<14>),
                      MAP(imm_v256_shr_n_s16<1>),
                      MAP(imm_v256_shr_n_s16<2>),
                      MAP(imm_v256_shr_n_s16<4>),
                      MAP(imm_v256_shr_n_s16<6>),
                      MAP(imm_v256_shr_n_s16<8>),
                      MAP(imm_v256_shr_n_s16<10>),
                      MAP(imm_v256_shr_n_s16<12>),
                      MAP(imm_v256_shr_n_s16<14>),
                      MAP(imm_v256_shl_n_32<1>),
                      MAP(imm_v256_shl_n_32<4>),
                      MAP(imm_v256_shl_n_32<8>),
                      MAP(imm_v256_shl_n_32<12>),
                      MAP(imm_v256_shl_n_32<16>),
                      MAP(imm_v256_shl_n_32<20>),
                      MAP(imm_v256_shl_n_32<24>),
                      MAP(imm_v256_shl_n_32<28>),
                      MAP(imm_v256_shr_n_u32<1>),
                      MAP(imm_v256_shr_n_u32<4>),
                      MAP(imm_v256_shr_n_u32<8>),
                      MAP(imm_v256_shr_n_u32<12>),
                      MAP(imm_v256_shr_n_u32<16>),
                      MAP(imm_v256_shr_n_u32<20>),
                      MAP(imm_v256_shr_n_u32<24>),
                      MAP(imm_v256_shr_n_u32<28>),
                      MAP(imm_v256_shr_n_s32<1>),
                      MAP(imm_v256_shr_n_s32<4>),
                      MAP(imm_v256_shr_n_s32<8>),
                      MAP(imm_v256_shr_n_s32<12>),
                      MAP(imm_v256_shr_n_s32<16>),
                      MAP(imm_v256_shr_n_s32<20>),
                      MAP(imm_v256_shr_n_s32<24>),
                      MAP(imm_v256_shr_n_s32<28>),
                      MAP(imm_v256_shl_n_64<1>),
                      MAP(imm_v256_shl_n_64<4>),
                      MAP(imm_v256_shl_n_64<8>),
                      MAP(imm_v256_shl_n_64<12>),
                      MAP(imm_v256_shl_n_64<16>),
                      MAP(imm_v256_shl_n_64<20>),
                      MAP(imm_v256_shl_n_64<24>),
                      MAP(imm_v256_shl_n_64<28>),
                      MAP(imm_v256_shl_n_64<32>),
                      MAP(imm_v256_shl_n_64<36>),
                      MAP(imm_v256_shl_n_64<40>),
                      MAP(imm_v256_shl_n_64<44>),
                      MAP(imm_v256_shl_n_64<48>),
                      MAP(imm_v256_shl_n_64<52>),
                      MAP(imm_v256_shl_n_64<56>),
                      MAP(imm_v256_shl_n_64<60>),
                      MAP(imm_v256_shr_n_u64<1>),
                      MAP(imm_v256_shr_n_u64<4>),
                      MAP(imm_v256_shr_n_u64<8>),
                      MAP(imm_v256_shr_n_u64<12>),
                      MAP(imm_v256_shr_n_u64<16>),
                      MAP(imm_v256_shr_n_u64<20>),
                      MAP(imm_v256_shr_n_u64<24>),
                      MAP(imm_v256_shr_n_u64<28>),
                      MAP(imm_v256_shr_n_u64<32>),
                      MAP(imm_v256_shr_n_u64<36>),
                      MAP(imm_v256_shr_n_u64<40>),
                      MAP(imm_v256_shr_n_u64<44>),
                      MAP(imm_v256_shr_n_u64<48>),
                      MAP(imm_v256_shr_n_u64<52>),
                      MAP(imm_v256_shr_n_u64<56>),
                      MAP(imm_v256_shr_n_u64<60>),
                      MAP(imm_v256_shr_n_s64<1>),
                      MAP(imm_v256_shr_n_s64<4>),
                      MAP(imm_v256_shr_n_s64<8>),
                      MAP(imm_v256_shr_n_s64<12>),
                      MAP(imm_v256_shr_n_s64<16>),
                      MAP(imm_v256_shr_n_s64<20>),
                      MAP(imm_v256_shr_n_s64<24>),
                      MAP(imm_v256_shr_n_s64<28>),
                      MAP(imm_v256_shr_n_s64<32>),
                      MAP(imm_v256_shr_n_s64<36>),
                      MAP(imm_v256_shr_n_s64<40>),
                      MAP(imm_v256_shr_n_s64<44>),
                      MAP(imm_v256_shr_n_s64<48>),
                      MAP(imm_v256_shr_n_s64<52>),
                      MAP(imm_v256_shr_n_s64<56>),
                      MAP(imm_v256_shr_n_s64<60>),
                      MAP(v256_movemask_8),
                      MAP(v256_zero),
                      MAP(v256_dup_8),
                      MAP(v256_dup_16),
                      MAP(v256_dup_32),
                      MAP(v256_dup_64),
                      MAP(v256_low_u32),
                      MAP(v256_low_v64),
                      MAP(v256_from_64),
                      MAP(v256_from_v64),
                      MAP(v256_ziplo_128),
                      MAP(v256_ziphi_128),
                      MAP(v256_unpacklo_u8_s16),
                      MAP(v256_unpackhi_u8_s16),
                      MAP(v256_unpacklo_s8_s16),
                      MAP(v256_unpackhi_s8_s16),
                      MAP(v256_blend_8),
                      { nullptr, nullptr, nullptr } };
#undef MAP

// Map reference functions to machine tuned functions. Since the
// functions depend on machine tuned types, the non-machine tuned
// instantiations of the test can't refer to these functions directly,
// so we refer to them by name and do the mapping here.
void Map(const char *name, fptr *ref, fptr *simd) {
  unsigned int i;
  for (i = 0; m[i].name && strcmp(name, m[i].name); i++) {
  }

  *ref = m[i].ref;
  *simd = m[i].simd;
}

// Used for printing errors in TestSimd1Arg, TestSimd2Args and TestSimd3Args
std::string Print(const uint8_t *a, int size) {
  std::string text = "0x";
  for (int i = 0; i < size; i++) {
    const uint8_t c = a[!CONFIG_BIG_ENDIAN ? size - 1 - i : i];
    // Same as snprintf(..., ..., "%02x", c)
    text += (c >> 4) + '0' + ((c >> 4) > 9) * ('a' - '0' - 10);
    text += (c & 15) + '0' + ((c & 15) > 9) * ('a' - '0' - 10);
  }

  return text;
}

// Used in TestSimd1Arg, TestSimd2Args and TestSimd3Args to restrict argument
// ranges
void SetMask(uint8_t *s, int size, uint32_t mask, uint32_t maskwidth) {
  switch (maskwidth) {
    case 0: {
      break;
    }
    case 8: {
      for (int i = 0; i < size; i++) s[i] &= mask;
      break;
    }
    case 16: {
      uint16_t *t = reinterpret_cast<uint16_t *>(s);
      assert(!(reinterpret_cast<uintptr_t>(s) & 1));
      for (int i = 0; i < size / 2; i++) t[i] &= mask;
      break;
    }
    case 32: {
      uint32_t *t = reinterpret_cast<uint32_t *>(s);
      assert(!(reinterpret_cast<uintptr_t>(s) & 3));
      for (int i = 0; i < size / 4; i++) t[i] &= mask;
      break;
    }
    case 64: {
      uint64_t *t = reinterpret_cast<uint64_t *>(s);
      assert(!(reinterpret_cast<uintptr_t>(s) & 7));
      for (int i = 0; i < size / 8; i++) t[i] &= mask;
      break;
    }
    default: {
      FAIL() << "Unsupported mask width";
      break;
    }
  }
}

// We need some extra load/store functions
void u64_store_aligned(void *p, uint64_t a) {
  v64_store_aligned(p, v64_from_64(a));
}
void s32_store_aligned(void *p, int32_t a) {
  u32_store_aligned(p, static_cast<uint32_t>(a));
}
void s64_store_aligned(void *p, int64_t a) {
  v64_store_aligned(p, v64_from_64(static_cast<uint64_t>(a)));
}

void c_u64_store_aligned(void *p, uint64_t a) {
  c_v64_store_aligned(p, c_v64_from_64(a));
}

void c_s32_store_aligned(void *p, int32_t a) {
  c_u32_store_aligned(p, static_cast<uint32_t>(a));
}

void c_s64_store_aligned(void *p, int64_t a) {
  c_v64_store_aligned(p, c_v64_from_64(static_cast<uint64_t>(a)));
}

uint64_t u64_load_aligned(const void *p) {
  return v64_u64(v64_load_aligned(p));
}
uint16_t u16_load_aligned(const void *p) {
  return *(reinterpret_cast<const uint16_t *>(p));
}
uint8_t u8_load_aligned(const void *p) {
  return *(reinterpret_cast<const uint8_t *>(p));
}

uint64_t c_u64_load_aligned(const void *p) {
  return c_v64_u64(c_v64_load_aligned(p));
}
uint16_t c_u16_load_aligned(const void *p) {
  return *(reinterpret_cast<const uint16_t *>(p));
}
uint8_t c_u8_load_aligned(const void *p) {
  return *(reinterpret_cast<const uint8_t *>(p));
}

// CompareSimd1Arg, CompareSimd2Args and CompareSimd3Args compare
// intrinsics taking 1, 2 or 3 arguments respectively with their
// corresponding C reference.  Ideally, the loads and stores should
// have gone into the template parameter list, but v64 and v128 could
// be typedef'ed to the same type (which is the case on x86) and then
// we can't instantiate both v64 and v128, so the function return and
// argument types, including the always differing types in the C
// equivalent are used instead.  The function arguments must be void
// pointers and then go through a cast to avoid matching errors in the
// branches eliminated by the typeid tests in the calling function.
template <typename Ret, typename Arg, typename CRet, typename CArg>
int CompareSimd1Arg(fptr store, fptr load, fptr simd, void *d, fptr c_store,
                    fptr c_load, fptr c_simd, void *ref_d, const void *a) {
  void (*const my_store)(void *, Ret) = (void (*const)(void *, Ret))store;
  Arg (*const my_load)(const void *) = (Arg(*const)(const void *))load;
  Ret (*const my_simd)(Arg) = (Ret(*const)(Arg))simd;
  void (*const my_c_store)(void *, CRet) = (void (*const)(void *, CRet))c_store;
  CArg (*const my_c_load)(const void *) = (CArg(*const)(const void *))c_load;
  CRet (*const my_c_simd)(CArg) = (CRet(*const)(CArg))c_simd;

  // Call reference and intrinsic
  my_c_store(ref_d, my_c_simd(my_c_load(a)));
  my_store(d, my_simd(my_load(a)));

  // Compare results
  return memcmp(ref_d, d, sizeof(CRet));
}

template <typename Ret, typename Arg1, typename Arg2, typename CRet,
          typename CArg1, typename CArg2>
int CompareSimd2Args(fptr store, fptr load1, fptr load2, fptr simd, void *d,
                     fptr c_store, fptr c_load1, fptr c_load2, fptr c_simd,
                     void *ref_d, const void *a, const void *b) {
  void (*const my_store)(void *, Ret) = (void (*const)(void *, Ret))store;
  Arg1 (*const my_load1)(const void *) = (Arg1(*const)(const void *))load1;
  Arg2 (*const my_load2)(const void *) = (Arg2(*const)(const void *))load2;
  Ret (*const my_simd)(Arg1, Arg2) = (Ret(*const)(Arg1, Arg2))simd;
  void (*const my_c_store)(void *, CRet) = (void (*const)(void *, CRet))c_store;
  CArg1 (*const my_c_load1)(const void *) =
      (CArg1(*const)(const void *))c_load1;
  CArg2 (*const my_c_load2)(const void *) =
      (CArg2(*const)(const void *))c_load2;
  CRet (*const my_c_simd)(CArg1, CArg2) = (CRet(*const)(CArg1, CArg2))c_simd;

  // Call reference and intrinsic
  my_c_store(ref_d, my_c_simd(my_c_load1(a), my_c_load2(b)));
  my_store(d, my_simd(my_load1(a), my_load2(b)));

  // Compare results
  return memcmp(ref_d, d, sizeof(CRet));
}

template <typename Ret, typename Arg1, typename Arg2, typename Arg3,
          typename CRet, typename CArg1, typename CArg2, typename CArg3>
int CompareSimd3Args(fptr store, fptr load1, fptr load2, fptr load3, fptr simd,
                     void *d, fptr c_store, fptr c_load1, fptr c_load2,
                     fptr c_load3, fptr c_simd, void *ref_d, const void *a,
                     const void *b, const void *c) {
  void (*const my_store)(void *, Ret) = (void (*const)(void *, Ret))store;
  Arg1 (*const my_load1)(const void *) = (Arg1(*const)(const void *))load1;
  Arg2 (*const my_load2)(const void *) = (Arg2(*const)(const void *))load2;
  Arg3 (*const my_load3)(const void *) = (Arg3(*const)(const void *))load3;
  Ret (*const my_simd)(Arg1, Arg2, Arg3) = (Ret(*const)(Arg1, Arg2, Arg3))simd;
  void (*const my_c_store)(void *, CRet) = (void (*const)(void *, CRet))c_store;
  CArg1 (*const my_c_load1)(const void *) =
      (CArg1(*const)(const void *))c_load1;
  CArg2 (*const my_c_load2)(const void *) =
      (CArg2(*const)(const void *))c_load2;
  CArg3 (*const my_c_load3)(const void *) =
      (CArg3(*const)(const void *))c_load3;
  CRet (*const my_c_simd)(CArg1, CArg2, CArg3) =
      (CRet(*const)(CArg1, CArg2, CArg3))c_simd;

  // Call reference and intrinsic
  my_c_store(ref_d, my_c_simd(my_c_load1(a), my_c_load2(b), my_c_load3(c)));
  my_store(d, my_simd(my_load1(a), my_load2(b), my_load3(c)));

  // Compare results
  return memcmp(ref_d, d, sizeof(CRet));
}

}  // namespace

template <typename CRet, typename CArg>
void TestSimd1Arg(uint32_t iterations, uint32_t mask, uint32_t maskwidth,
                  const char *name) {
  ACMRandom rnd(ACMRandom::DeterministicSeed());
  fptr ref_simd;
  fptr simd;
  int error = 0;
  DECLARE_ALIGNED(32, uint8_t, s[32]);
  DECLARE_ALIGNED(32, uint8_t, d[32]);
  DECLARE_ALIGNED(32, uint8_t, ref_d[32]);
  assert(sizeof(CArg) <= 32 && sizeof(CRet) <= 32);
  memset(ref_d, 0, sizeof(ref_d));
  memset(d, 0, sizeof(d));

  Map(name, &ref_simd, &simd);
  if (simd == nullptr || ref_simd == nullptr) {
    FAIL() << "Internal error: Unknown intrinsic function " << name;
  }
  for (unsigned int count = 0;
       count < iterations && !error && !testing::Test::HasFailure(); count++) {
    for (unsigned int c = 0; c < sizeof(CArg); c++) s[c] = rnd.Rand8();

    if (maskwidth) {
      SetMask(s, sizeof(CArg), mask, maskwidth);
    }

    if (typeid(CRet) == typeid(c_v64) && typeid(CArg) == typeid(c_v64)) {
      // V64_V64
      error = CompareSimd1Arg<v64, v64, c_v64, c_v64>(
          reinterpret_cast<fptr>(v64_store_aligned),
          reinterpret_cast<fptr>(v64_load_aligned), simd, d,
          reinterpret_cast<fptr>(c_v64_store_aligned),
          reinterpret_cast<fptr>(c_v64_load_aligned), ref_simd, ref_d, s);
    } else if (typeid(CRet) == typeid(c_v64) &&
               typeid(CArg) == typeid(uint8_t)) {
      // V64_U8
      error = CompareSimd1Arg<v64, uint8_t, c_v64, uint8_t>(
          reinterpret_cast<fptr>(v64_store_aligned),
          reinterpret_cast<fptr>(u8_load_aligned), simd, d,
          reinterpret_cast<fptr>(c_v64_store_aligned),
          reinterpret_cast<fptr>(c_u8_load_aligned), ref_simd, ref_d, s);
    } else if (typeid(CRet) == typeid(c_v64) &&
               typeid(CArg) == typeid(uint16_t)) {
      // V64_U16
      error = CompareSimd1Arg<v64, uint16_t, c_v64, uint16_t>(
          reinterpret_cast<fptr>(v64_store_aligned),
          reinterpret_cast<fptr>(u16_load_aligned), simd, d,
          reinterpret_cast<fptr>(c_v64_store_aligned),
          reinterpret_cast<fptr>(c_u16_load_aligned), ref_simd, ref_d, s);
    } else if (typeid(CRet) == typeid(c_v64) &&
               typeid(CArg) == typeid(uint32_t)) {
      // V64_U32
      error = CompareSimd1Arg<v64, uint32_t, c_v64, uint32_t>(
          reinterpret_cast<fptr>(v64_store_aligned),
          reinterpret_cast<fptr>(u32_load_aligned), simd, d,
          reinterpret_cast<fptr>(c_v64_store_aligned),
          reinterpret_cast<fptr>(c_u32_load_aligned), ref_simd, ref_d, s);
    } else if (typeid(CRet) == typeid(uint64_t) &&
               typeid(CArg) == typeid(c_v64)) {
      // U64_V64
      error = CompareSimd1Arg<uint64_t, v64, uint64_t, c_v64>(
          reinterpret_cast<fptr>(u64_store_aligned),
          reinterpret_cast<fptr>(v64_load_aligned), simd, d,
          reinterpret_cast<fptr>(c_u64_store_aligned),
          reinterpret_cast<fptr>(c_v64_load_aligned), ref_simd, ref_d, s);
    } else if (typeid(CRet) == typeid(int64_t) &&
               typeid(CArg) == typeid(c_v64)) {
      // S64_V64
      error = CompareSimd1Arg<int64_t, v64, int64_t, c_v64>(
          reinterpret_cast<fptr>(s64_store_aligned),
          reinterpret_cast<fptr>(v64_load_aligned), simd, d,
          reinterpret_cast<fptr>(c_s64_store_aligned),
          reinterpret_cast<fptr>(c_v64_load_aligned), ref_simd, ref_d, s);
    } else if (typeid(CRet) == typeid(uint32_t) &&
               typeid(CArg) == typeid(c_v64)) {
      // U32_V64
      error = CompareSimd1Arg<uint32_t, v64, uint32_t, c_v64>(
          reinterpret_cast<fptr>(u32_store_aligned),
          reinterpret_cast<fptr>(v64_load_aligned), simd, d,
          reinterpret_cast<fptr>(c_u32_store_aligned),
          reinterpret_cast<fptr>(c_v64_load_aligned), ref_simd, ref_d, s);
    } else if (typeid(CRet) == typeid(int32_t) &&
               typeid(CArg) == typeid(c_v64)) {
      // S32_V64
      error = CompareSimd1Arg<int32_t, v64, int32_t, c_v64>(
          reinterpret_cast<fptr>(s32_store_aligned),
          reinterpret_cast<fptr>(v64_load_aligned), simd, d,
          reinterpret_cast<fptr>(c_s32_store_aligned),
          reinterpret_cast<fptr>(c_v64_load_aligned), ref_simd, ref_d, s);
    } else if (typeid(CRet) == typeid(uint32_t) &&
               typeid(CArg) == typeid(c_v128)) {
      // U32_V128
      error = CompareSimd1Arg<uint32_t, v128, uint32_t, c_v128>(
          reinterpret_cast<fptr>(u32_store_aligned),
          reinterpret_cast<fptr>(v128_load_aligned), simd, d,
          reinterpret_cast<fptr>(c_u32_store_aligned),
          reinterpret_cast<fptr>(c_v128_load_aligned), ref_simd, ref_d, s);
    } else if (typeid(CRet) == typeid(uint64_t) &&
               typeid(CArg) == typeid(c_v128)) {
      // U64_V128
      error = CompareSimd1Arg<uint64_t, v128, uint64_t, c_v128>(
          reinterpret_cast<fptr>(u64_store_aligned),
          reinterpret_cast<fptr>(v128_load_aligned), simd, d,
          reinterpret_cast<fptr>(c_u64_store_aligned),
          reinterpret_cast<fptr>(c_v128_load_aligned), ref_simd, ref_d, s);
    } else if (typeid(CRet) == typeid(uint64_t) &&
               typeid(CArg) == typeid(c_v256)) {
      // U64_V256
      error = CompareSimd1Arg<uint64_t, v256, uint64_t, c_v256>(
          reinterpret_cast<fptr>(u64_store_aligned),
          reinterpret_cast<fptr>(v256_load_aligned), simd, d,
          reinterpret_cast<fptr>(c_u64_store_aligned),
          reinterpret_cast<fptr>(c_v256_load_aligned), ref_simd, ref_d, s);
    } else if (typeid(CRet) == typeid(c_v64) &&
               typeid(CArg) == typeid(c_v128)) {
      // V64_V128
      error = CompareSimd1Arg<v64, v128, c_v64, c_v128>(
          reinterpret_cast<fptr>(v64_store_aligned),
          reinterpret_cast<fptr>(v128_load_aligned), simd, d,
          reinterpret_cast<fptr>(c_v64_store_aligned),
          reinterpret_cast<fptr>(c_v128_load_aligned), ref_simd, ref_d, s);
    } else if (typeid(CRet) == typeid(c_v128) &&
               typeid(CArg) == typeid(c_v128)) {
      // V128_V128
      error = CompareSimd1Arg<v128, v128, c_v128, c_v128>(
          reinterpret_cast<fptr>(v128_store_aligned),
          reinterpret_cast<fptr>(v128_load_aligned), simd, d,
          reinterpret_cast<fptr>(c_v128_store_aligned),
          reinterpret_cast<fptr>(c_v128_load_aligned), ref_simd, ref_d, s);
    } else if (typeid(CRet) == typeid(c_v128) &&
               typeid(CArg) == typeid(c_v64)) {
      // V128_V64
      error = CompareSimd1Arg<v128, v64, c_v128, c_v64>(
          reinterpret_cast<fptr>(v128_store_aligned),
          reinterpret_cast<fptr>(v64_load_aligned), simd, d,
          reinterpret_cast<fptr>(c_v128_store_aligned),
          reinterpret_cast<fptr>(c_v64_load_aligned), ref_simd, ref_d, s);
    } else if (typeid(CRet) == typeid(c_v128) &&
               typeid(CArg) == typeid(uint8_t)) {
      // V128_U8
      error = CompareSimd1Arg<v128, uint8_t, c_v128, uint8_t>(
          reinterpret_cast<fptr>(v128_store_aligned),
          reinterpret_cast<fptr>(u8_load_aligned), simd, d,
          reinterpret_cast<fptr>(c_v128_store_aligned),
          reinterpret_cast<fptr>(c_u8_load_aligned), ref_simd, ref_d, s);
    } else if (typeid(CRet) == typeid(c_v128) &&
               typeid(CArg) == typeid(uint16_t)) {
      // V128_U16
      error = CompareSimd1Arg<v128, uint16_t, c_v128, uint16_t>(
          reinterpret_cast<fptr>(v128_store_aligned),
          reinterpret_cast<fptr>(u16_load_aligned), simd, d,
          reinterpret_cast<fptr>(c_v128_store_aligned),
          reinterpret_cast<fptr>(c_u16_load_aligned), ref_simd, ref_d, s);
    } else if (typeid(CRet) == typeid(c_v128) &&
               typeid(CArg) == typeid(uint32_t)) {
      // V128_U32
      error = CompareSimd1Arg<v128, uint32_t, c_v128, uint32_t>(
          reinterpret_cast<fptr>(v128_store_aligned),
          reinterpret_cast<fptr>(u32_load_aligned), simd, d,
          reinterpret_cast<fptr>(c_v128_store_aligned),
          reinterpret_cast<fptr>(c_u32_load_aligned), ref_simd, ref_d, s);
    } else if (typeid(CRet) == typeid(c_v128) &&
               typeid(CArg) == typeid(uint64_t)) {
      // V128_U64
      error = CompareSimd1Arg<v128, uint64_t, c_v128, uint64_t>(
          reinterpret_cast<fptr>(v128_store_aligned),
          reinterpret_cast<fptr>(u64_load_aligned), simd, d,
          reinterpret_cast<fptr>(c_v128_store_aligned),
          reinterpret_cast<fptr>(c_u64_load_aligned), ref_simd, ref_d, s);
    } else if (typeid(CRet) == typeid(c_v256) &&
               typeid(CArg) == typeid(c_v256)) {
      // V256_V256
      error = CompareSimd1Arg<v256, v256, c_v256, c_v256>(
          reinterpret_cast<fptr>(v256_store_aligned),
          reinterpret_cast<fptr>(v256_load_aligned), simd, d,
          reinterpret_cast<fptr>(c_v256_store_aligned),
          reinterpret_cast<fptr>(c_v256_load_aligned), ref_simd, ref_d, s);
    } else if (typeid(CRet) == typeid(c_v256) &&
               typeid(CArg) == typeid(c_v128)) {
      // V256_V128
      error = CompareSimd1Arg<v256, v128, c_v256, c_v128>(
          reinterpret_cast<fptr>(v256_store_aligned),
          reinterpret_cast<fptr>(v128_load_aligned), simd, d,
          reinterpret_cast<fptr>(c_v256_store_aligned),
          reinterpret_cast<fptr>(c_v128_load_aligned), ref_simd, ref_d, s);
    } else if (typeid(CRet) == typeid(c_v256) &&
               typeid(CArg) == typeid(uint8_t)) {
      // V256_U8
      error = CompareSimd1Arg<v256, uint8_t, c_v256, uint8_t>(
          reinterpret_cast<fptr>(v256_store_aligned),
          reinterpret_cast<fptr>(u8_load_aligned), simd, d,
          reinterpret_cast<fptr>(c_v256_store_aligned),
          reinterpret_cast<fptr>(c_u8_load_aligned), ref_simd, ref_d, s);
    } else if (typeid(CRet) == typeid(c_v256) &&
               typeid(CArg) == typeid(uint16_t)) {
      // V256_U16
      error = CompareSimd1Arg<v256, uint16_t, c_v256, uint16_t>(
          reinterpret_cast<fptr>(v256_store_aligned),
          reinterpret_cast<fptr>(u16_load_aligned), simd, d,
          reinterpret_cast<fptr>(c_v256_store_aligned),
          reinterpret_cast<fptr>(c_u16_load_aligned), ref_simd, ref_d, s);
    } else if (typeid(CRet) == typeid(c_v256) &&
               typeid(CArg) == typeid(uint32_t)) {
      // V256_U32
      error = CompareSimd1Arg<v256, uint32_t, c_v256, uint32_t>(
          reinterpret_cast<fptr>(v256_store_aligned),
          reinterpret_cast<fptr>(u32_load_aligned), simd, d,
          reinterpret_cast<fptr>(c_v256_store_aligned),
          reinterpret_cast<fptr>(c_u32_load_aligned), ref_simd, ref_d, s);
    } else if (typeid(CRet) == typeid(c_v256) &&
               typeid(CArg) == typeid(uint64_t)) {
      // V256_U64
      error = CompareSimd1Arg<v256, uint64_t, c_v256, uint64_t>(
          reinterpret_cast<fptr>(v256_store_aligned),
          reinterpret_cast<fptr>(u64_load_aligned), simd, d,
          reinterpret_cast<fptr>(c_v256_store_aligned),
          reinterpret_cast<fptr>(c_u64_load_aligned), ref_simd, ref_d, s);
    } else if (typeid(CRet) == typeid(uint32_t) &&
               typeid(CArg) == typeid(c_v256)) {
      // U32_V256
      error = CompareSimd1Arg<uint32_t, v256, uint32_t, c_v256>(
          reinterpret_cast<fptr>(u32_store_aligned),
          reinterpret_cast<fptr>(v256_load_aligned), simd, d,
          reinterpret_cast<fptr>(c_u32_store_aligned),
          reinterpret_cast<fptr>(c_v256_load_aligned), ref_simd, ref_d, s);
    } else if (typeid(CRet) == typeid(c_v64) &&
               typeid(CArg) == typeid(c_v256)) {
      // V64_V256
      error = CompareSimd1Arg<v64, v256, c_v64, c_v256>(
          reinterpret_cast<fptr>(v64_store_aligned),
          reinterpret_cast<fptr>(v256_load_aligned), simd, d,
          reinterpret_cast<fptr>(c_v64_store_aligned),
          reinterpret_cast<fptr>(c_v256_load_aligned), ref_simd, ref_d, s);
    } else {
      FAIL() << "Internal error: Unknown intrinsic function "
             << typeid(CRet).name() << " " << name << "(" << typeid(CArg).name()
             << ")";
    }
  }

  EXPECT_EQ(0, error) << "Error: mismatch for " << name << "("
                      << Print(s, sizeof(CArg)) << ") -> "
                      << Print(d, sizeof(CRet)) << " (simd), "
                      << Print(ref_d, sizeof(CRet)) << " (ref)";
}

template <typename CRet, typename CArg1, typename CArg2>
void TestSimd2Args(uint32_t iterations, uint32_t mask, uint32_t maskwidth,
                   const char *name) {
  ACMRandom rnd(ACMRandom::DeterministicSeed());
  fptr ref_simd;
  fptr simd;
  int error = 0;
  DECLARE_ALIGNED(32, uint8_t, s1[32]);
  DECLARE_ALIGNED(32, uint8_t, s2[32]);
  DECLARE_ALIGNED(32, uint8_t, d[32]);
  DECLARE_ALIGNED(32, uint8_t, ref_d[32]);
  assert(sizeof(CArg1) <= 32 && sizeof(CArg2) <= 32 && sizeof(CRet) <= 32);
  memset(ref_d, 0, sizeof(ref_d));
  memset(d, 0, sizeof(d));

  Map(name, &ref_simd, &simd);
  if (simd == nullptr || ref_simd == nullptr) {
    FAIL() << "Internal error: Unknown intrinsic function " << name;
  }

  for (unsigned int count = 0;
       count < iterations && !error && !testing::Test::HasFailure(); count++) {
    for (unsigned int c = 0; c < sizeof(CArg1); c++) s1[c] = rnd.Rand8();

    for (unsigned int c = 0; c < sizeof(CArg2); c++) s2[c] = rnd.Rand8();

    if (maskwidth) SetMask(s2, sizeof(CArg2), mask, maskwidth);

    if (typeid(CRet) == typeid(c_v64) && typeid(CArg1) == typeid(c_v64) &&
        typeid(CArg2) == typeid(c_v64)) {
      // V64_V64V64
      error = CompareSimd2Args<v64, v64, v64, c_v64, c_v64, c_v64>(
          reinterpret_cast<fptr>(v64_store_aligned),
          reinterpret_cast<fptr>(v64_load_aligned),
          reinterpret_cast<fptr>(v64_load_aligned), simd, d,
          reinterpret_cast<fptr>(c_v64_store_aligned),
          reinterpret_cast<fptr>(c_v64_load_aligned),
          reinterpret_cast<fptr>(c_v64_load_aligned),
          reinterpret_cast<fptr>(ref_simd), ref_d, s1, s2);
    } else if (typeid(CRet) == typeid(c_v64) &&
               typeid(CArg1) == typeid(uint32_t) &&
               typeid(CArg2) == typeid(uint32_t)) {
      // V64_U32U32
      error =
          CompareSimd2Args<v64, uint32_t, uint32_t, c_v64, uint32_t, uint32_t>(
              reinterpret_cast<fptr>(v64_store_aligned),
              reinterpret_cast<fptr>(u32_load_aligned),
              reinterpret_cast<fptr>(u32_load_aligned), simd, d,
              reinterpret_cast<fptr>(c_v64_store_aligned),
              reinterpret_cast<fptr>(c_u32_load_aligned),
              reinterpret_cast<fptr>(c_u32_load_aligned),
              reinterpret_cast<fptr>(ref_simd), ref_d, s1, s2);
    } else if (typeid(CRet) == typeid(uint32_t) &&
               typeid(CArg1) == typeid(c_v64) &&
               typeid(CArg2) == typeid(c_v64)) {
      // U32_V64V64
      error = CompareSimd2Args<uint32_t, v64, v64, uint32_t, c_v64, c_v64>(
          reinterpret_cast<fptr>(u32_store_aligned),
          reinterpret_cast<fptr>(v64_load_aligned),
          reinterpret_cast<fptr>(v64_load_aligned), simd, d,
          reinterpret_cast<fptr>(c_u32_store_aligned),
          reinterpret_cast<fptr>(c_v64_load_aligned),
          reinterpret_cast<fptr>(c_v64_load_aligned),
          reinterpret_cast<fptr>(ref_simd), ref_d, s1, s2);
    } else if (typeid(CRet) == typeid(int64_t) &&
               typeid(CArg1) == typeid(c_v64) &&
               typeid(CArg2) == typeid(c_v64)) {
      // S64_V64V64
      error = CompareSimd2Args<int64_t, v64, v64, int64_t, c_v64, c_v64>(
          reinterpret_cast<fptr>(s64_store_aligned),
          reinterpret_cast<fptr>(v64_load_aligned),
          reinterpret_cast<fptr>(v64_load_aligned), simd, d,
          reinterpret_cast<fptr>(c_s64_store_aligned),
          reinterpret_cast<fptr>(c_v64_load_aligned),
          reinterpret_cast<fptr>(c_v64_load_aligned),
          reinterpret_cast<fptr>(ref_simd), ref_d, s1, s2);
    } else if (typeid(CRet) == typeid(c_v64) &&
               typeid(CArg1) == typeid(c_v64) &&
               typeid(CArg2) == typeid(uint32_t)) {
      // V64_V64U32
      error = CompareSimd2Args<v64, v64, uint32_t, c_v64, c_v64, uint32_t>(
          reinterpret_cast<fptr>(v64_store_aligned),
          reinterpret_cast<fptr>(v64_load_aligned),
          reinterpret_cast<fptr>(u32_load_aligned), simd, d,
          reinterpret_cast<fptr>(c_v64_store_aligned),
          reinterpret_cast<fptr>(c_v64_load_aligned),
          reinterpret_cast<fptr>(c_u32_load_aligned),
          reinterpret_cast<fptr>(ref_simd), ref_d, s1, s2);
    } else if (typeid(CRet) == typeid(c_v128) &&
               typeid(CArg1) == typeid(c_v128) &&
               typeid(CArg2) == typeid(c_v128)) {
      // V128_V128V128
      error = CompareSimd2Args<v128, v128, v128, c_v128, c_v128, c_v128>(
          reinterpret_cast<fptr>(v128_store_aligned),
          reinterpret_cast<fptr>(v128_load_aligned),
          reinterpret_cast<fptr>(v128_load_aligned), simd, d,
          reinterpret_cast<fptr>(c_v128_store_aligned),
          reinterpret_cast<fptr>(c_v128_load_aligned),
          reinterpret_cast<fptr>(c_v128_load_aligned),
          reinterpret_cast<fptr>(ref_simd), ref_d, s1, s2);
    } else if (typeid(CRet) == typeid(uint32_t) &&
               typeid(CArg1) == typeid(c_v128) &&
               typeid(CArg2) == typeid(c_v128)) {
      // U32_V128V128
      error = CompareSimd2Args<uint32_t, v128, v128, uint32_t, c_v128, c_v128>(
          reinterpret_cast<fptr>(u32_store_aligned),
          reinterpret_cast<fptr>(v128_load_aligned),
          reinterpret_cast<fptr>(v128_load_aligned), simd, d,
          reinterpret_cast<fptr>(c_u32_store_aligned),
          reinterpret_cast<fptr>(c_v128_load_aligned),
          reinterpret_cast<fptr>(c_v128_load_aligned),
          reinterpret_cast<fptr>(ref_simd), ref_d, s1, s2);
    } else if (typeid(CRet) == typeid(uint64_t) &&
               typeid(CArg1) == typeid(c_v128) &&
               typeid(CArg2) == typeid(c_v128)) {
      // U64_V128V128
      error = CompareSimd2Args<uint64_t, v128, v128, uint64_t, c_v128, c_v128>(
          reinterpret_cast<fptr>(u64_store_aligned),
          reinterpret_cast<fptr>(v128_load_aligned),
          reinterpret_cast<fptr>(v128_load_aligned), simd, d,
          reinterpret_cast<fptr>(c_u64_store_aligned),
          reinterpret_cast<fptr>(c_v128_load_aligned),
          reinterpret_cast<fptr>(c_v128_load_aligned),
          reinterpret_cast<fptr>(ref_simd), ref_d, s1, s2);
    } else if (typeid(CRet) == typeid(int64_t) &&
               typeid(CArg1) == typeid(c_v128) &&
               typeid(CArg2) == typeid(c_v128)) {
      // S64_V128V128
      error = CompareSimd2Args<int64_t, v128, v128, int64_t, c_v128, c_v128>(
          reinterpret_cast<fptr>(s64_store_aligned),
          reinterpret_cast<fptr>(v128_load_aligned),
          reinterpret_cast<fptr>(v128_load_aligned), simd, d,
          reinterpret_cast<fptr>(c_s64_store_aligned),
          reinterpret_cast<fptr>(c_v128_load_aligned),
          reinterpret_cast<fptr>(c_v128_load_aligned),
          reinterpret_cast<fptr>(ref_simd), ref_d, s1, s2);
    } else if (typeid(CRet) == typeid(c_v128) &&
               typeid(CArg1) == typeid(uint64_t) &&
               typeid(CArg2) == typeid(uint64_t)) {
      // V128_U64U64
      error = CompareSimd2Args<v128, uint64_t, uint64_t, c_v128, uint64_t,
                               uint64_t>(
          reinterpret_cast<fptr>(v128_store_aligned),
          reinterpret_cast<fptr>(u64_load_aligned),
          reinterpret_cast<fptr>(u64_load_aligned), simd, d,
          reinterpret_cast<fptr>(c_v128_store_aligned),
          reinterpret_cast<fptr>(c_u64_load_aligned),
          reinterpret_cast<fptr>(c_u64_load_aligned),
          reinterpret_cast<fptr>(ref_simd), ref_d, s1, s2);
    } else if (typeid(CRet) == typeid(c_v128) &&
               typeid(CArg1) == typeid(c_v64) &&
               typeid(CArg2) == typeid(c_v64)) {
      // V128_V64V64
      error = CompareSimd2Args<v128, v64, v64, c_v128, c_v64, c_v64>(
          reinterpret_cast<fptr>(v128_store_aligned),
          reinterpret_cast<fptr>(v64_load_aligned),
          reinterpret_cast<fptr>(v64_load_aligned), simd, d,
          reinterpret_cast<fptr>(c_v128_store_aligned),
          reinterpret_cast<fptr>(c_v64_load_aligned),
          reinterpret_cast<fptr>(c_v64_load_aligned),
          reinterpret_cast<fptr>(ref_simd), ref_d, s1, s2);
    } else if (typeid(CRet) == typeid(c_v128) &&
               typeid(CArg1) == typeid(c_v128) &&
               typeid(CArg2) == typeid(uint32_t)) {
      // V128_V128U32
      error = CompareSimd2Args<v128, v128, uint32_t, c_v128, c_v128, uint32_t>(
          reinterpret_cast<fptr>(v128_store_aligned),
          reinterpret_cast<fptr>(v128_load_aligned),
          reinterpret_cast<fptr>(u32_load_aligned), simd, d,
          reinterpret_cast<fptr>(c_v128_store_aligned),
          reinterpret_cast<fptr>(c_v128_load_aligned),
          reinterpret_cast<fptr>(c_u32_load_aligned),
          reinterpret_cast<fptr>(ref_simd), ref_d, s1, s2);
    } else if (typeid(CRet) == typeid(c_v256) &&
               typeid(CArg1) == typeid(c_v256) &&
               typeid(CArg2) == typeid(c_v256)) {
      // V256_V256V256
      error = CompareSimd2Args<v256, v256, v256, c_v256, c_v256, c_v256>(
          reinterpret_cast<fptr>(v256_store_aligned),
          reinterpret_cast<fptr>(v256_load_aligned),
          reinterpret_cast<fptr>(v256_load_aligned), simd, d,
          reinterpret_cast<fptr>(c_v256_store_aligned),
          reinterpret_cast<fptr>(c_v256_load_aligned),
          reinterpret_cast<fptr>(c_v256_load_aligned),
          reinterpret_cast<fptr>(ref_simd), ref_d, s1, s2);
    } else if (typeid(CRet) == typeid(uint64_t) &&
               typeid(CArg1) == typeid(c_v256) &&
               typeid(CArg2) == typeid(c_v256)) {
      // U64_V256V256
      error = CompareSimd2Args<uint64_t, v256, v256, uint64_t, c_v256, c_v256>(
          reinterpret_cast<fptr>(u64_store_aligned),
          reinterpret_cast<fptr>(v256_load_aligned),
          reinterpret_cast<fptr>(v256_load_aligned), simd, d,
          reinterpret_cast<fptr>(c_u64_store_aligned),
          reinterpret_cast<fptr>(c_v256_load_aligned),
          reinterpret_cast<fptr>(c_v256_load_aligned),
          reinterpret_cast<fptr>(ref_simd), ref_d, s1, s2);
    } else if (typeid(CRet) == typeid(int64_t) &&
               typeid(CArg1) == typeid(c_v256) &&
               typeid(CArg2) == typeid(c_v256)) {
      // S64_V256V256
      error = CompareSimd2Args<int64_t, v256, v256, int64_t, c_v256, c_v256>(
          reinterpret_cast<fptr>(s64_store_aligned),
          reinterpret_cast<fptr>(v256_load_aligned),
          reinterpret_cast<fptr>(v256_load_aligned), simd, d,
          reinterpret_cast<fptr>(c_s64_store_aligned),
          reinterpret_cast<fptr>(c_v256_load_aligned),
          reinterpret_cast<fptr>(c_v256_load_aligned),
          reinterpret_cast<fptr>(ref_simd), ref_d, s1, s2);
    } else if (typeid(CRet) == typeid(uint32_t) &&
               typeid(CArg1) == typeid(c_v256) &&
               typeid(CArg2) == typeid(c_v256)) {
      // U32_V256V256
      error = CompareSimd2Args<uint32_t, v256, v256, uint32_t, c_v256, c_v256>(
          reinterpret_cast<fptr>(u32_store_aligned),
          reinterpret_cast<fptr>(v256_load_aligned),
          reinterpret_cast<fptr>(v256_load_aligned), simd, d,
          reinterpret_cast<fptr>(c_u32_store_aligned),
          reinterpret_cast<fptr>(c_v256_load_aligned),
          reinterpret_cast<fptr>(c_v256_load_aligned),
          reinterpret_cast<fptr>(ref_simd), ref_d, s1, s2);
    } else if (typeid(CRet) == typeid(c_v256) &&
               typeid(CArg1) == typeid(c_v128) &&
               typeid(CArg2) == typeid(c_v128)) {
      // V256_V128V128
      error = CompareSimd2Args<v256, v128, v128, c_v256, c_v128, c_v128>(
          reinterpret_cast<fptr>(v256_store_aligned),
          reinterpret_cast<fptr>(v128_load_aligned),
          reinterpret_cast<fptr>(v128_load_aligned), simd, d,
          reinterpret_cast<fptr>(c_v256_store_aligned),
          reinterpret_cast<fptr>(c_v128_load_aligned),
          reinterpret_cast<fptr>(c_v128_load_aligned),
          reinterpret_cast<fptr>(ref_simd), ref_d, s1, s2);
    } else if (typeid(CRet) == typeid(c_v256) &&
               typeid(CArg1) == typeid(c_v256) &&
               typeid(CArg2) == typeid(uint32_t)) {
      // V256_V256U32
      error = CompareSimd2Args<v256, v256, uint32_t, c_v256, c_v256, uint32_t>(
          reinterpret_cast<fptr>(v256_store_aligned),
          reinterpret_cast<fptr>(v256_load_aligned),
          reinterpret_cast<fptr>(u32_load_aligned), simd, d,
          reinterpret_cast<fptr>(c_v256_store_aligned),
          reinterpret_cast<fptr>(c_v256_load_aligned),
          reinterpret_cast<fptr>(c_u32_load_aligned),
          reinterpret_cast<fptr>(ref_simd), ref_d, s1, s2);

    } else {
      FAIL() << "Internal error: Unknown intrinsic function "
             << typeid(CRet).name() << " " << name << "("
             << typeid(CArg1).name() << ", " << typeid(CArg2).name() << ")";
    }
  }

  EXPECT_EQ(0, error) << "Error: mismatch for " << name << "("
                      << Print(s1, sizeof(CArg1)) << ", "
                      << Print(s2, sizeof(CArg2)) << ") -> "
                      << Print(d, sizeof(CRet)) << " (simd), "
                      << Print(ref_d, sizeof(CRet)) << " (ref)";
}

template <typename CRet, typename CArg1, typename CArg2, typename CArg3>
void TestSimd3Args(uint32_t iterations, uint32_t mask, uint32_t maskwidth,
                   const char *name) {
  ACMRandom rnd(ACMRandom::DeterministicSeed());
  fptr ref_simd;
  fptr simd;
  int error = 0;
  DECLARE_ALIGNED(32, uint8_t, s1[32]);
  DECLARE_ALIGNED(32, uint8_t, s2[32]);
  DECLARE_ALIGNED(32, uint8_t, s3[32]);
  DECLARE_ALIGNED(32, uint8_t, d[32]);
  DECLARE_ALIGNED(32, uint8_t, ref_d[32]);
  assert(sizeof(CArg1) <= 32 && sizeof(CArg2) <= 32 && sizeof(CArg3) <= 32 &&
         sizeof(CRet) <= 32);
  memset(ref_d, 0, sizeof(ref_d));
  memset(d, 0, sizeof(d));

  Map(name, &ref_simd, &simd);
  if (simd == nullptr || ref_simd == nullptr) {
    FAIL() << "Internal error: Unknown intrinsic function " << name;
  }

  for (unsigned int count = 0;
       count < iterations && !error && !testing::Test::HasFailure(); count++) {
    for (unsigned int c = 0; c < sizeof(CArg1); c++) s1[c] = rnd.Rand8();

    for (unsigned int c = 0; c < sizeof(CArg2); c++) s2[c] = rnd.Rand8();

    for (unsigned int c = 0; c < sizeof(CArg3); c++) s3[c] = rnd.Rand8();

    if (maskwidth) SetMask(s3, sizeof(CArg3), mask, maskwidth);

    if (typeid(CRet) == typeid(c_v128) && typeid(CArg1) == typeid(c_v128) &&
        typeid(CArg2) == typeid(c_v128) && typeid(CArg3) == typeid(c_v128)) {
      // V128_V128V128V128
      error = CompareSimd3Args<v128, v128, v128, v128, c_v128, c_v128, c_v128,
                               c_v128>(
          reinterpret_cast<fptr>(v128_store_aligned),
          reinterpret_cast<fptr>(v128_load_aligned),
          reinterpret_cast<fptr>(v128_load_aligned),
          reinterpret_cast<fptr>(v128_load_aligned), simd, d,
          reinterpret_cast<fptr>(c_v128_store_aligned),
          reinterpret_cast<fptr>(c_v128_load_aligned),
          reinterpret_cast<fptr>(c_v128_load_aligned),
          reinterpret_cast<fptr>(c_v128_load_aligned),
          reinterpret_cast<fptr>(ref_simd), ref_d, s1, s2, s3);
    } else if (typeid(CRet) == typeid(c_v256) &&
               typeid(CArg1) == typeid(c_v256) &&
               typeid(CArg2) == typeid(c_v256) &&
               typeid(CArg3) == typeid(c_v256)) {
      // V256_V256V256V256
      error = CompareSimd3Args<v256, v256, v256, v256, c_v256, c_v256, c_v256,
                               c_v256>(
          reinterpret_cast<fptr>(v256_store_aligned),
          reinterpret_cast<fptr>(v256_load_aligned),
          reinterpret_cast<fptr>(v256_load_aligned),
          reinterpret_cast<fptr>(v256_load_aligned), simd, d,
          reinterpret_cast<fptr>(c_v256_store_aligned),
          reinterpret_cast<fptr>(c_v256_load_aligned),
          reinterpret_cast<fptr>(c_v256_load_aligned),
          reinterpret_cast<fptr>(c_v256_load_aligned),
          reinterpret_cast<fptr>(ref_simd), ref_d, s1, s2, s3);
    } else {
      FAIL() << "Internal error: Unknown intrinsic function "
             << typeid(CRet).name() << " " << name << "("
             << typeid(CArg1).name() << ", " << typeid(CArg2).name() << ", "
             << typeid(CArg3).name() << ")";
    }
  }

  EXPECT_EQ(0, error) << "Error: mismatch for " << name << "("
                      << Print(s1, sizeof(CArg1)) << ", "
                      << Print(s2, sizeof(CArg2)) << ", "
                      << Print(s3, sizeof(CArg3)) << ") -> "
                      << Print(d, sizeof(CRet)) << " (simd), "
                      << Print(ref_d, sizeof(CRet)) << " (ref)";
}

// Instantiations to make the functions callable from another files
template void TestSimd1Arg<c_v64, uint8_t>(uint32_t, uint32_t, uint32_t,
                                           const char *);
template void TestSimd1Arg<c_v64, uint16_t>(uint32_t, uint32_t, uint32_t,
                                            const char *);
template void TestSimd1Arg<c_v64, uint32_t>(uint32_t, uint32_t, uint32_t,
                                            const char *);
template void TestSimd1Arg<c_v64, c_v64>(uint32_t, uint32_t, uint32_t,
                                         const char *);
template void TestSimd1Arg<uint32_t, c_v64>(uint32_t, uint32_t, uint32_t,
                                            const char *);
template void TestSimd1Arg<int32_t, c_v64>(uint32_t, uint32_t, uint32_t,
                                           const char *);
template void TestSimd1Arg<uint64_t, c_v64>(uint32_t, uint32_t, uint32_t,
                                            const char *);
template void TestSimd1Arg<int64_t, c_v64>(uint32_t, uint32_t, uint32_t,
                                           const char *);
template void TestSimd2Args<c_v64, uint32_t, uint32_t>(uint32_t, uint32_t,
                                                       uint32_t, const char *);
template void TestSimd2Args<c_v64, c_v64, c_v64>(uint32_t, uint32_t, uint32_t,
                                                 const char *);
template void TestSimd2Args<c_v64, c_v64, uint32_t>(uint32_t, uint32_t,
                                                    uint32_t, const char *);
template void TestSimd2Args<int64_t, c_v64, c_v64>(uint32_t, uint32_t, uint32_t,
                                                   const char *);
template void TestSimd2Args<uint32_t, c_v64, c_v64>(uint32_t, uint32_t,
                                                    uint32_t, const char *);
template void TestSimd1Arg<c_v128, c_v128>(uint32_t, uint32_t, uint32_t,
                                           const char *);
template void TestSimd1Arg<c_v128, uint8_t>(uint32_t, uint32_t, uint32_t,
                                            const char *);
template void TestSimd1Arg<c_v128, uint16_t>(uint32_t, uint32_t, uint32_t,
                                             const char *);
template void TestSimd1Arg<c_v128, uint32_t>(uint32_t, uint32_t, uint32_t,
                                             const char *);
template void TestSimd1Arg<c_v128, uint64_t>(uint32_t, uint32_t, uint32_t,
                                             const char *);
template void TestSimd1Arg<c_v128, c_v64>(uint32_t, uint32_t, uint32_t,
                                          const char *);
template void TestSimd1Arg<uint32_t, c_v128>(uint32_t, uint32_t, uint32_t,
                                             const char *);
template void TestSimd1Arg<uint64_t, c_v128>(uint32_t, uint32_t, uint32_t,
                                             const char *);
template void TestSimd1Arg<c_v64, c_v128>(uint32_t, uint32_t, uint32_t,
                                          const char *);
template void TestSimd2Args<c_v128, c_v128, c_v128>(uint32_t, uint32_t,
                                                    uint32_t, const char *);
template void TestSimd2Args<c_v128, c_v128, uint32_t>(uint32_t, uint32_t,
                                                      uint32_t, const char *);
template void TestSimd2Args<c_v128, uint64_t, uint64_t>(uint32_t, uint32_t,
                                                        uint32_t, const char *);
template void TestSimd2Args<c_v128, c_v64, c_v64>(uint32_t, uint32_t, uint32_t,
                                                  const char *);
template void TestSimd2Args<uint64_t, c_v128, c_v128>(uint32_t, uint32_t,
                                                      uint32_t, const char *);
template void TestSimd2Args<int64_t, c_v128, c_v128>(uint32_t, uint32_t,
                                                     uint32_t, const char *);
template void TestSimd2Args<uint32_t, c_v128, c_v128>(uint32_t, uint32_t,
                                                      uint32_t, const char *);
template void TestSimd3Args<c_v128, c_v128, c_v128, c_v128>(uint32_t, uint32_t,
                                                            uint32_t,
                                                            const char *);
template void TestSimd1Arg<c_v256, c_v128>(uint32_t, uint32_t, uint32_t,
                                           const char *);
template void TestSimd1Arg<c_v256, c_v256>(uint32_t, uint32_t, uint32_t,
                                           const char *);
template void TestSimd1Arg<uint64_t, c_v256>(uint32_t, uint32_t, uint32_t,
                                             const char *);
template void TestSimd1Arg<c_v256, uint8_t>(uint32_t, uint32_t, uint32_t,
                                            const char *);
template void TestSimd1Arg<c_v256, uint16_t>(uint32_t, uint32_t, uint32_t,
                                             const char *);
template void TestSimd1Arg<c_v256, uint32_t>(uint32_t, uint32_t, uint32_t,
                                             const char *);
template void TestSimd1Arg<c_v256, uint64_t>(uint32_t, uint32_t, uint32_t,
                                             const char *);
template void TestSimd1Arg<uint32_t, c_v256>(uint32_t, uint32_t, uint32_t,
                                             const char *);
template void TestSimd1Arg<c_v64, c_v256>(uint32_t, uint32_t, uint32_t,
                                          const char *);
template void TestSimd2Args<c_v256, c_v128, c_v128>(uint32_t, uint32_t,
                                                    uint32_t, const char *);
template void TestSimd2Args<c_v256, c_v256, c_v256>(uint32_t, uint32_t,
                                                    uint32_t, const char *);
template void TestSimd2Args<c_v256, c_v256, uint32_t>(uint32_t, uint32_t,
                                                      uint32_t, const char *);
template void TestSimd2Args<uint64_t, c_v256, c_v256>(uint32_t, uint32_t,
                                                      uint32_t, const char *);
template void TestSimd2Args<int64_t, c_v256, c_v256>(uint32_t, uint32_t,
                                                     uint32_t, const char *);
template void TestSimd2Args<uint32_t, c_v256, c_v256>(uint32_t, uint32_t,
                                                      uint32_t, const char *);
template void TestSimd3Args<c_v256, c_v256, c_v256, c_v256>(uint32_t, uint32_t,
                                                            uint32_t,
                                                            const char *);

}  // namespace SIMD_NAMESPACE
