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

#ifndef AOM_AOM_DSP_AOM_DSP_COMMON_H_
#define AOM_AOM_DSP_AOM_DSP_COMMON_H_

#include <limits.h>

#include "config/aom_config.h"

#include "aom/aom_integer.h"
#include "aom_ports/mem.h"

#ifdef __cplusplus
extern "C" {
#endif

#if defined(_MSC_VER)
#define AOM_FORCE_INLINE __forceinline
#else
#define AOM_FORCE_INLINE __inline__ __attribute__((always_inline))
#endif

#define PI 3.141592653589793238462643383279502884

#define AOMMIN(x, y) (((x) < (y)) ? (x) : (y))
#define AOMMAX(x, y) (((x) > (y)) ? (x) : (y))
#define AOMSIGN(x) ((x) < 0 ? -1 : 0)

#define NELEMENTS(x) (int)(sizeof(x) / sizeof(x[0]))

#define IMPLIES(a, b) (!(a) || (b))  //  Logical 'a implies b' (or 'a -> b')

#define IS_POWER_OF_TWO(x) (((x) & ((x)-1)) == 0)

/* Left shifting a negative value became undefined behavior in C99 (downgraded
   from merely implementation-defined in C89). This should still compile to the
   correct thing on any two's-complement machine, but avoid ubsan warnings.*/
#define AOM_SIGNED_SHL(x, shift) ((x) * (((x)*0 + 1) << (shift)))

// These can be used to give a hint about branch outcomes.
// This can have an effect, even if your target processor has a
// good branch predictor, as these hints can affect basic block
// ordering by the compiler.
#ifdef __GNUC__
#define LIKELY(v) __builtin_expect(v, 1)
#define UNLIKELY(v) __builtin_expect(v, 0)
#else
#define LIKELY(v) (v)
#define UNLIKELY(v) (v)
#endif

typedef uint8_t qm_val_t;
#define AOM_QM_BITS 5

// Note:
// tran_low_t  is the datatype used for final transform coefficients.
// tran_high_t is the datatype used for intermediate transform stages.
typedef int64_t tran_high_t;
typedef int32_t tran_low_t;

static inline uint8_t clip_pixel(int val) {
  return (val > 255) ? 255 : (val < 0) ? 0 : val;
}

static inline int clamp(int value, int low, int high) {
  return value < low ? low : (value > high ? high : value);
}

static inline int64_t clamp64(int64_t value, int64_t low, int64_t high) {
  return value < low ? low : (value > high ? high : value);
}

static inline double fclamp(double value, double low, double high) {
  return value < low ? low : (value > high ? high : value);
}

static inline uint16_t clip_pixel_highbd(int val, int bd) {
  switch (bd) {
    case 8:
    default: return (uint16_t)clamp(val, 0, 255);
    case 10: return (uint16_t)clamp(val, 0, 1023);
    case 12: return (uint16_t)clamp(val, 0, 4095);
  }
}

// The result of this branchless code is equivalent to (value < 0 ? 0 : value)
// or max(0, value) and might be faster in some cases.
// Care should be taken since the behavior of right shifting signed type
// negative value is undefined by C standards and implementation defined,
static inline unsigned int negative_to_zero(int value) {
  return value & ~(value >> (sizeof(value) * 8 - 1));
}

// Returns the saturating cast of a double value to int.
static inline int saturate_cast_double_to_int(double d) {
  if (d > INT_MAX) return INT_MAX;
  return (int)d;
}

#ifdef __cplusplus
}  // extern "C"
#endif

#endif  // AOM_AOM_DSP_AOM_DSP_COMMON_H_
