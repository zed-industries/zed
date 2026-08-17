/*===---- arm_vector_types - ARM vector type ------===
 *
 *
 * Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
 * See https://llvm.org/LICENSE.txt for license information.
 * SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
 *
 *===-----------------------------------------------------------------------===
 */

#if !defined(__ARM_NEON_H) && !defined(__ARM_SVE_H)
#error "This file should not be used standalone. Please include arm_neon.h or arm_sve.h instead"

#endif
#ifndef __ARM_NEON_TYPES_H
#define __ARM_NEON_TYPES_H
typedef float float32_t;
typedef __fp16 float16_t;
#if defined(__aarch64__) || defined(__arm64ec__)
typedef __mfp8 mfloat8_t;
typedef double float64_t;
#endif


typedef uint64_t fpm_t;

enum __ARM_FPM_FORMAT { __ARM_FPM_E5M2, __ARM_FPM_E4M3 };

enum __ARM_FPM_OVERFLOW { __ARM_FPM_INFNAN, __ARM_FPM_SATURATE };

static __inline__ fpm_t __attribute__((__always_inline__, __nodebug__))
__arm_fpm_init(void) {
  return 0;
}

static __inline__ fpm_t __attribute__((__always_inline__, __nodebug__))
__arm_set_fpm_src1_format(fpm_t __fpm, enum __ARM_FPM_FORMAT __format) {
  return (__fpm & ~7ull) | (fpm_t)__format;
}

static __inline__ fpm_t __attribute__((__always_inline__, __nodebug__))
__arm_set_fpm_src2_format(fpm_t __fpm, enum __ARM_FPM_FORMAT __format) {
  return (__fpm & ~0x38ull) | ((fpm_t)__format << 3u);
}

static __inline__ fpm_t __attribute__((__always_inline__, __nodebug__))
__arm_set_fpm_dst_format(fpm_t __fpm, enum __ARM_FPM_FORMAT __format) {
  return (__fpm & ~0x1c0ull) | ((fpm_t)__format << 6u);
}

static __inline__ fpm_t __attribute__((__always_inline__, __nodebug__))
__arm_set_fpm_overflow_mul(fpm_t __fpm, enum __ARM_FPM_OVERFLOW __behaviour) {
  return (__fpm & ~0x4000ull) | ((fpm_t)__behaviour << 14u);
}

static __inline__ fpm_t __attribute__((__always_inline__, __nodebug__))
__arm_set_fpm_overflow_cvt(fpm_t __fpm, enum __ARM_FPM_OVERFLOW __behaviour) {
  return (__fpm & ~0x8000ull) | ((fpm_t)__behaviour << 15u);
}

static __inline__ fpm_t __attribute__((__always_inline__, __nodebug__))
__arm_set_fpm_lscale(fpm_t __fpm, uint64_t __scale) {
  return (__fpm & ~0x7f0000ull) | (__scale << 16u);
}

static __inline__ fpm_t __attribute__((__always_inline__, __nodebug__))
__arm_set_fpm_nscale(fpm_t __fpm, int64_t __scale) {
  return (__fpm & ~0xff000000ull) | (((fpm_t)__scale & 0xffu) << 24u);
}

static __inline__ fpm_t __attribute__((__always_inline__, __nodebug__))
__arm_set_fpm_lscale2(fpm_t __fpm, uint64_t __scale) {
  return (uint32_t)__fpm | (__scale << 32u);
}

typedef __attribute__((neon_vector_type(8))) int8_t int8x8_t;
typedef __attribute__((neon_vector_type(16))) int8_t int8x16_t;
typedef __attribute__((neon_vector_type(4))) int16_t int16x4_t;
typedef __attribute__((neon_vector_type(8))) int16_t int16x8_t;
typedef __attribute__((neon_vector_type(2))) int32_t int32x2_t;
typedef __attribute__((neon_vector_type(4))) int32_t int32x4_t;
typedef __attribute__((neon_vector_type(1))) int64_t int64x1_t;
typedef __attribute__((neon_vector_type(2))) int64_t int64x2_t;
typedef __attribute__((neon_vector_type(8))) uint8_t uint8x8_t;
typedef __attribute__((neon_vector_type(16))) uint8_t uint8x16_t;
typedef __attribute__((neon_vector_type(4))) uint16_t uint16x4_t;
typedef __attribute__((neon_vector_type(8))) uint16_t uint16x8_t;
typedef __attribute__((neon_vector_type(2))) uint32_t uint32x2_t;
typedef __attribute__((neon_vector_type(4))) uint32_t uint32x4_t;
typedef __attribute__((neon_vector_type(1))) uint64_t uint64x1_t;
typedef __attribute__((neon_vector_type(2))) uint64_t uint64x2_t;
#if defined(__aarch64__) || defined(__arm64ec__)
typedef __attribute__((neon_vector_type(8))) mfloat8_t mfloat8x8_t;
typedef __attribute__((neon_vector_type(16))) mfloat8_t mfloat8x16_t;
#endif
typedef __attribute__((neon_vector_type(4))) float16_t float16x4_t;
typedef __attribute__((neon_vector_type(8))) float16_t float16x8_t;
typedef __attribute__((neon_vector_type(2))) float32_t float32x2_t;
typedef __attribute__((neon_vector_type(4))) float32_t float32x4_t;
#if defined(__aarch64__) || defined(__arm64ec__)
typedef __attribute__((neon_vector_type(1))) float64_t float64x1_t;
typedef __attribute__((neon_vector_type(2))) float64_t float64x2_t;
#endif

typedef struct int8x8x2_t {
  int8x8_t val[2];
} int8x8x2_t;

typedef struct int8x16x2_t {
  int8x16_t val[2];
} int8x16x2_t;

typedef struct int16x4x2_t {
  int16x4_t val[2];
} int16x4x2_t;

typedef struct int16x8x2_t {
  int16x8_t val[2];
} int16x8x2_t;

typedef struct int32x2x2_t {
  int32x2_t val[2];
} int32x2x2_t;

typedef struct int32x4x2_t {
  int32x4_t val[2];
} int32x4x2_t;

typedef struct int64x1x2_t {
  int64x1_t val[2];
} int64x1x2_t;

typedef struct int64x2x2_t {
  int64x2_t val[2];
} int64x2x2_t;

typedef struct uint8x8x2_t {
  uint8x8_t val[2];
} uint8x8x2_t;

typedef struct uint8x16x2_t {
  uint8x16_t val[2];
} uint8x16x2_t;

typedef struct uint16x4x2_t {
  uint16x4_t val[2];
} uint16x4x2_t;

typedef struct uint16x8x2_t {
  uint16x8_t val[2];
} uint16x8x2_t;

typedef struct uint32x2x2_t {
  uint32x2_t val[2];
} uint32x2x2_t;

typedef struct uint32x4x2_t {
  uint32x4_t val[2];
} uint32x4x2_t;

typedef struct uint64x1x2_t {
  uint64x1_t val[2];
} uint64x1x2_t;

typedef struct uint64x2x2_t {
  uint64x2_t val[2];
} uint64x2x2_t;

#if defined(__aarch64__) || defined(__arm64ec__)
typedef struct mfloat8x8x2_t {
  mfloat8x8_t val[2];
} mfloat8x8x2_t;

typedef struct mfloat8x16x2_t {
  mfloat8x16_t val[2];
} mfloat8x16x2_t;

#endif
typedef struct float16x4x2_t {
  float16x4_t val[2];
} float16x4x2_t;

typedef struct float16x8x2_t {
  float16x8_t val[2];
} float16x8x2_t;

typedef struct float32x2x2_t {
  float32x2_t val[2];
} float32x2x2_t;

typedef struct float32x4x2_t {
  float32x4_t val[2];
} float32x4x2_t;

#if defined(__aarch64__) || defined(__arm64ec__)
typedef struct float64x1x2_t {
  float64x1_t val[2];
} float64x1x2_t;

typedef struct float64x2x2_t {
  float64x2_t val[2];
} float64x2x2_t;

#endif
typedef struct int8x8x3_t {
  int8x8_t val[3];
} int8x8x3_t;

typedef struct int8x16x3_t {
  int8x16_t val[3];
} int8x16x3_t;

typedef struct int16x4x3_t {
  int16x4_t val[3];
} int16x4x3_t;

typedef struct int16x8x3_t {
  int16x8_t val[3];
} int16x8x3_t;

typedef struct int32x2x3_t {
  int32x2_t val[3];
} int32x2x3_t;

typedef struct int32x4x3_t {
  int32x4_t val[3];
} int32x4x3_t;

typedef struct int64x1x3_t {
  int64x1_t val[3];
} int64x1x3_t;

typedef struct int64x2x3_t {
  int64x2_t val[3];
} int64x2x3_t;

typedef struct uint8x8x3_t {
  uint8x8_t val[3];
} uint8x8x3_t;

typedef struct uint8x16x3_t {
  uint8x16_t val[3];
} uint8x16x3_t;

typedef struct uint16x4x3_t {
  uint16x4_t val[3];
} uint16x4x3_t;

typedef struct uint16x8x3_t {
  uint16x8_t val[3];
} uint16x8x3_t;

typedef struct uint32x2x3_t {
  uint32x2_t val[3];
} uint32x2x3_t;

typedef struct uint32x4x3_t {
  uint32x4_t val[3];
} uint32x4x3_t;

typedef struct uint64x1x3_t {
  uint64x1_t val[3];
} uint64x1x3_t;

typedef struct uint64x2x3_t {
  uint64x2_t val[3];
} uint64x2x3_t;

#if defined(__aarch64__) || defined(__arm64ec__)
typedef struct mfloat8x8x3_t {
  mfloat8x8_t val[3];
} mfloat8x8x3_t;

typedef struct mfloat8x16x3_t {
  mfloat8x16_t val[3];
} mfloat8x16x3_t;

#endif
typedef struct float16x4x3_t {
  float16x4_t val[3];
} float16x4x3_t;

typedef struct float16x8x3_t {
  float16x8_t val[3];
} float16x8x3_t;

typedef struct float32x2x3_t {
  float32x2_t val[3];
} float32x2x3_t;

typedef struct float32x4x3_t {
  float32x4_t val[3];
} float32x4x3_t;

#if defined(__aarch64__) || defined(__arm64ec__)
typedef struct float64x1x3_t {
  float64x1_t val[3];
} float64x1x3_t;

typedef struct float64x2x3_t {
  float64x2_t val[3];
} float64x2x3_t;

#endif
typedef struct int8x8x4_t {
  int8x8_t val[4];
} int8x8x4_t;

typedef struct int8x16x4_t {
  int8x16_t val[4];
} int8x16x4_t;

typedef struct int16x4x4_t {
  int16x4_t val[4];
} int16x4x4_t;

typedef struct int16x8x4_t {
  int16x8_t val[4];
} int16x8x4_t;

typedef struct int32x2x4_t {
  int32x2_t val[4];
} int32x2x4_t;

typedef struct int32x4x4_t {
  int32x4_t val[4];
} int32x4x4_t;

typedef struct int64x1x4_t {
  int64x1_t val[4];
} int64x1x4_t;

typedef struct int64x2x4_t {
  int64x2_t val[4];
} int64x2x4_t;

typedef struct uint8x8x4_t {
  uint8x8_t val[4];
} uint8x8x4_t;

typedef struct uint8x16x4_t {
  uint8x16_t val[4];
} uint8x16x4_t;

typedef struct uint16x4x4_t {
  uint16x4_t val[4];
} uint16x4x4_t;

typedef struct uint16x8x4_t {
  uint16x8_t val[4];
} uint16x8x4_t;

typedef struct uint32x2x4_t {
  uint32x2_t val[4];
} uint32x2x4_t;

typedef struct uint32x4x4_t {
  uint32x4_t val[4];
} uint32x4x4_t;

typedef struct uint64x1x4_t {
  uint64x1_t val[4];
} uint64x1x4_t;

typedef struct uint64x2x4_t {
  uint64x2_t val[4];
} uint64x2x4_t;

#if defined(__aarch64__) || defined(__arm64ec__)
typedef struct mfloat8x8x4_t {
  mfloat8x8_t val[4];
} mfloat8x8x4_t;

typedef struct mfloat8x16x4_t {
  mfloat8x16_t val[4];
} mfloat8x16x4_t;

#endif
typedef struct float16x4x4_t {
  float16x4_t val[4];
} float16x4x4_t;

typedef struct float16x8x4_t {
  float16x8_t val[4];
} float16x8x4_t;

typedef struct float32x2x4_t {
  float32x2_t val[4];
} float32x2x4_t;

typedef struct float32x4x4_t {
  float32x4_t val[4];
} float32x4x4_t;

#if defined(__aarch64__) || defined(__arm64ec__)
typedef struct float64x1x4_t {
  float64x1_t val[4];
} float64x1x4_t;

typedef struct float64x2x4_t {
  float64x2_t val[4];
} float64x2x4_t;

#endif
typedef __attribute__((neon_vector_type(4))) bfloat16_t bfloat16x4_t;
typedef __attribute__((neon_vector_type(8))) bfloat16_t bfloat16x8_t;

typedef struct bfloat16x4x2_t {
  bfloat16x4_t val[2];
} bfloat16x4x2_t;

typedef struct bfloat16x8x2_t {
  bfloat16x8_t val[2];
} bfloat16x8x2_t;

typedef struct bfloat16x4x3_t {
  bfloat16x4_t val[3];
} bfloat16x4x3_t;

typedef struct bfloat16x8x3_t {
  bfloat16x8_t val[3];
} bfloat16x8x3_t;

typedef struct bfloat16x4x4_t {
  bfloat16x4_t val[4];
} bfloat16x4x4_t;

typedef struct bfloat16x8x4_t {
  bfloat16x8_t val[4];
} bfloat16x8x4_t;

#endif // __ARM_NEON_TYPES_H
