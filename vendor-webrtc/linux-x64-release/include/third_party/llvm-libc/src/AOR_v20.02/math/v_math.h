/*
 * Vector math abstractions.
 *
 * Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
 * See https://llvm.org/LICENSE.txt for license information.
 * SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
 */

#ifndef _V_MATH_H
#define _V_MATH_H

#ifndef WANT_VMATH
/* Enable the build of vector math code.  */
# define WANT_VMATH 1
#endif
#if WANT_VMATH

/* The goal of this header is to allow vector and scalar
   build of the same algorithm, the provided intrinsic
   wrappers are also vector length agnostic so they can
   be implemented for SVE too (or other simd architectures)
   and then the code should work on those targets too.  */

#if SCALAR
#define V_NAME(x) __s_##x
#elif VPCS && __aarch64__
#define V_NAME(x) __vn_##x
#define VPCS_ATTR __attribute__ ((aarch64_vector_pcs))
#else
#define V_NAME(x) __v_##x
#endif

#ifndef VPCS_ATTR
#define VPCS_ATTR
#endif
#ifndef VPCS_ALIAS
#define VPCS_ALIAS
#endif

#include <stdint.h>
#include "math_config.h"

typedef float f32_t;
typedef uint32_t u32_t;
typedef int32_t s32_t;
typedef double f64_t;
typedef uint64_t u64_t;
typedef int64_t s64_t;

/* reinterpret as type1 from type2.  */
static inline u32_t
as_u32_f32 (f32_t x)
{
  union { f32_t f; u32_t u; } r = {x};
  return r.u;
}
static inline f32_t
as_f32_u32 (u32_t x)
{
  union { u32_t u; f32_t f; } r = {x};
  return r.f;
}
static inline s32_t
as_s32_u32 (u32_t x)
{
  union { u32_t u; s32_t i; } r = {x};
  return r.i;
}
static inline u32_t
as_u32_s32 (s32_t x)
{
  union { s32_t i; u32_t u; } r = {x};
  return r.u;
}
static inline u64_t
as_u64_f64 (f64_t x)
{
  union { f64_t f; u64_t u; } r = {x};
  return r.u;
}
static inline f64_t
as_f64_u64 (u64_t x)
{
  union { u64_t u; f64_t f; } r = {x};
  return r.f;
}
static inline s64_t
as_s64_u64 (u64_t x)
{
  union { u64_t u; s64_t i; } r = {x};
  return r.i;
}
static inline u64_t
as_u64_s64 (s64_t x)
{
  union { s64_t i; u64_t u; } r = {x};
  return r.u;
}

#if SCALAR
#define V_SUPPORTED 1
typedef f32_t v_f32_t;
typedef u32_t v_u32_t;
typedef s32_t v_s32_t;
typedef f64_t v_f64_t;
typedef u64_t v_u64_t;
typedef s64_t v_s64_t;

static inline int
v_lanes32 (void)
{
  return 1;
}

static inline v_f32_t
v_f32 (f32_t x)
{
  return x;
}
static inline v_u32_t
v_u32 (u32_t x)
{
  return x;
}
static inline v_s32_t
v_s32 (s32_t x)
{
  return x;
}

static inline f32_t
v_get_f32 (v_f32_t x, int i)
{
  return x;
}
static inline u32_t
v_get_u32 (v_u32_t x, int i)
{
  return x;
}
static inline s32_t
v_get_s32 (v_s32_t x, int i)
{
  return x;
}

static inline void
v_set_f32 (v_f32_t *x, int i, f32_t v)
{
  *x = v;
}
static inline void
v_set_u32 (v_u32_t *x, int i, u32_t v)
{
  *x = v;
}
static inline void
v_set_s32 (v_s32_t *x, int i, s32_t v)
{
  *x = v;
}

/* true if any elements of a v_cond result is non-zero.  */
static inline int
v_any_u32 (v_u32_t x)
{
  return x != 0;
}
/* to wrap the result of relational operators.  */
static inline v_u32_t
v_cond_u32 (v_u32_t x)
{
  return x ? -1 : 0;
}
static inline v_f32_t
v_abs_f32 (v_f32_t x)
{
  return __builtin_fabsf (x);
}
static inline v_f32_t
v_fma_f32 (v_f32_t x, v_f32_t y, v_f32_t z)
{
  return __builtin_fmaf (x, y, z);
}
static inline v_f32_t
v_round_f32 (v_f32_t x)
{
  return __builtin_roundf (x);
}
static inline v_s32_t
v_round_s32 (v_f32_t x)
{
  return __builtin_lroundf (x); /* relies on -fno-math-errno.  */
}
/* convert to type1 from type2.  */
static inline v_f32_t
v_to_f32_s32 (v_s32_t x)
{
  return x;
}
static inline v_f32_t
v_to_f32_u32 (v_u32_t x)
{
  return x;
}
/* reinterpret as type1 from type2.  */
static inline v_u32_t
v_as_u32_f32 (v_f32_t x)
{
  union { v_f32_t f; v_u32_t u; } r = {x};
  return r.u;
}
static inline v_f32_t
v_as_f32_u32 (v_u32_t x)
{
  union { v_u32_t u; v_f32_t f; } r = {x};
  return r.f;
}
static inline v_s32_t
v_as_s32_u32 (v_u32_t x)
{
  union { v_u32_t u; v_s32_t i; } r = {x};
  return r.i;
}
static inline v_u32_t
v_as_u32_s32 (v_s32_t x)
{
  union { v_s32_t i; v_u32_t u; } r = {x};
  return r.u;
}
static inline v_f32_t
v_lookup_f32 (const f32_t *tab, v_u32_t idx)
{
  return tab[idx];
}
static inline v_u32_t
v_lookup_u32 (const u32_t *tab, v_u32_t idx)
{
  return tab[idx];
}
static inline v_f32_t
v_call_f32 (f32_t (*f) (f32_t), v_f32_t x, v_f32_t y, v_u32_t p)
{
  return f (x);
}
static inline v_f32_t
v_call2_f32 (f32_t (*f) (f32_t, f32_t), v_f32_t x1, v_f32_t x2, v_f32_t y,
	     v_u32_t p)
{
  return f (x1, x2);
}

static inline int
v_lanes64 (void)
{
  return 1;
}
static inline v_f64_t
v_f64 (f64_t x)
{
  return x;
}
static inline v_u64_t
v_u64 (u64_t x)
{
  return x;
}
static inline v_s64_t
v_s64 (s64_t x)
{
  return x;
}
static inline f64_t
v_get_f64 (v_f64_t x, int i)
{
  return x;
}
static inline void
v_set_f64 (v_f64_t *x, int i, f64_t v)
{
  *x = v;
}
/* true if any elements of a v_cond result is non-zero.  */
static inline int
v_any_u64 (v_u64_t x)
{
  return x != 0;
}
/* to wrap the result of relational operators.  */
static inline v_u64_t
v_cond_u64 (v_u64_t x)
{
  return x ? -1 : 0;
}
static inline v_f64_t
v_abs_f64 (v_f64_t x)
{
  return __builtin_fabs (x);
}
static inline v_f64_t
v_fma_f64 (v_f64_t x, v_f64_t y, v_f64_t z)
{
  return __builtin_fma (x, y, z);
}
static inline v_f64_t
v_round_f64 (v_f64_t x)
{
  return __builtin_round (x);
}
static inline v_s64_t
v_round_s64 (v_f64_t x)
{
  return __builtin_lround (x); /* relies on -fno-math-errno.  */
}
/* convert to type1 from type2.  */
static inline v_f64_t
v_to_f64_s64 (v_s64_t x)
{
  return x;
}
static inline v_f64_t
v_to_f64_u64 (v_u64_t x)
{
  return x;
}
/* reinterpret as type1 from type2.  */
static inline v_u64_t
v_as_u64_f64 (v_f64_t x)
{
  union { v_f64_t f; v_u64_t u; } r = {x};
  return r.u;
}
static inline v_f64_t
v_as_f64_u64 (v_u64_t x)
{
  union { v_u64_t u; v_f64_t f; } r = {x};
  return r.f;
}
static inline v_s64_t
v_as_s64_u64 (v_u64_t x)
{
  union { v_u64_t u; v_s64_t i; } r = {x};
  return r.i;
}
static inline v_u64_t
v_as_u64_s64 (v_s64_t x)
{
  union { v_s64_t i; v_u64_t u; } r = {x};
  return r.u;
}
static inline v_f64_t
v_lookup_f64 (const f64_t *tab, v_u64_t idx)
{
  return tab[idx];
}
static inline v_u64_t
v_lookup_u64 (const u64_t *tab, v_u64_t idx)
{
  return tab[idx];
}
static inline v_f64_t
v_call_f64 (f64_t (*f) (f64_t), v_f64_t x, v_f64_t y, v_u64_t p)
{
  return f (x);
}

#elif __aarch64__
#define V_SUPPORTED 1
#include <arm_neon.h>
typedef float32x4_t v_f32_t;
typedef uint32x4_t v_u32_t;
typedef int32x4_t v_s32_t;
typedef float64x2_t v_f64_t;
typedef uint64x2_t v_u64_t;
typedef int64x2_t v_s64_t;

static inline int
v_lanes32 (void)
{
  return 4;
}

static inline v_f32_t
v_f32 (f32_t x)
{
  return (v_f32_t){x, x, x, x};
}
static inline v_u32_t
v_u32 (u32_t x)
{
  return (v_u32_t){x, x, x, x};
}
static inline v_s32_t
v_s32 (s32_t x)
{
  return (v_s32_t){x, x, x, x};
}

static inline f32_t
v_get_f32 (v_f32_t x, int i)
{
  return x[i];
}
static inline u32_t
v_get_u32 (v_u32_t x, int i)
{
  return x[i];
}
static inline s32_t
v_get_s32 (v_s32_t x, int i)
{
  return x[i];
}

static inline void
v_set_f32 (v_f32_t *x, int i, f32_t v)
{
  (*x)[i] = v;
}
static inline void
v_set_u32 (v_u32_t *x, int i, u32_t v)
{
  (*x)[i] = v;
}
static inline void
v_set_s32 (v_s32_t *x, int i, s32_t v)
{
  (*x)[i] = v;
}

/* true if any elements of a v_cond result is non-zero.  */
static inline int
v_any_u32 (v_u32_t x)
{
  /* assume elements in x are either 0 or -1u.  */
  return vpaddd_u64 (vreinterpretq_u64_u32 (x)) != 0;
}
/* to wrap the result of relational operators.  */
static inline v_u32_t
v_cond_u32 (v_u32_t x)
{
  return x;
}
static inline v_f32_t
v_abs_f32 (v_f32_t x)
{
  return vabsq_f32 (x);
}
static inline v_f32_t
v_fma_f32 (v_f32_t x, v_f32_t y, v_f32_t z)
{
  return vfmaq_f32 (z, x, y);
}
static inline v_f32_t
v_round_f32 (v_f32_t x)
{
  return vrndaq_f32 (x);
}
static inline v_s32_t
v_round_s32 (v_f32_t x)
{
  return vcvtaq_s32_f32 (x);
}
/* convert to type1 from type2.  */
static inline v_f32_t
v_to_f32_s32 (v_s32_t x)
{
  return (v_f32_t){x[0], x[1], x[2], x[3]};
}
static inline v_f32_t
v_to_f32_u32 (v_u32_t x)
{
  return (v_f32_t){x[0], x[1], x[2], x[3]};
}
/* reinterpret as type1 from type2.  */
static inline v_u32_t
v_as_u32_f32 (v_f32_t x)
{
  union { v_f32_t f; v_u32_t u; } r = {x};
  return r.u;
}
static inline v_f32_t
v_as_f32_u32 (v_u32_t x)
{
  union { v_u32_t u; v_f32_t f; } r = {x};
  return r.f;
}
static inline v_s32_t
v_as_s32_u32 (v_u32_t x)
{
  union { v_u32_t u; v_s32_t i; } r = {x};
  return r.i;
}
static inline v_u32_t
v_as_u32_s32 (v_s32_t x)
{
  union { v_s32_t i; v_u32_t u; } r = {x};
  return r.u;
}
static inline v_f32_t
v_lookup_f32 (const f32_t *tab, v_u32_t idx)
{
  return (v_f32_t){tab[idx[0]], tab[idx[1]], tab[idx[2]], tab[idx[3]]};
}
static inline v_u32_t
v_lookup_u32 (const u32_t *tab, v_u32_t idx)
{
  return (v_u32_t){tab[idx[0]], tab[idx[1]], tab[idx[2]], tab[idx[3]]};
}
static inline v_f32_t
v_call_f32 (f32_t (*f) (f32_t), v_f32_t x, v_f32_t y, v_u32_t p)
{
  return (v_f32_t){p[0] ? f (x[0]) : y[0], p[1] ? f (x[1]) : y[1],
		   p[2] ? f (x[2]) : y[2], p[3] ? f (x[3]) : y[3]};
}
static inline v_f32_t
v_call2_f32 (f32_t (*f) (f32_t, f32_t), v_f32_t x1, v_f32_t x2, v_f32_t y,
	     v_u32_t p)
{
  return (
    v_f32_t){p[0] ? f (x1[0], x2[0]) : y[0], p[1] ? f (x1[1], x2[1]) : y[1],
	     p[2] ? f (x1[2], x2[2]) : y[2], p[3] ? f (x1[3], x2[3]) : y[3]};
}

static inline int
v_lanes64 (void)
{
  return 2;
}
static inline v_f64_t
v_f64 (f64_t x)
{
  return (v_f64_t){x, x};
}
static inline v_u64_t
v_u64 (u64_t x)
{
  return (v_u64_t){x, x};
}
static inline v_s64_t
v_s64 (s64_t x)
{
  return (v_s64_t){x, x};
}
static inline f64_t
v_get_f64 (v_f64_t x, int i)
{
  return x[i];
}
static inline void
v_set_f64 (v_f64_t *x, int i, f64_t v)
{
  (*x)[i] = v;
}
/* true if any elements of a v_cond result is non-zero.  */
static inline int
v_any_u64 (v_u64_t x)
{
  /* assume elements in x are either 0 or -1u.  */
  return vpaddd_u64 (x) != 0;
}
/* to wrap the result of relational operators.  */
static inline v_u64_t
v_cond_u64 (v_u64_t x)
{
  return x;
}
static inline v_f64_t
v_abs_f64 (v_f64_t x)
{
  return vabsq_f64 (x);
}
static inline v_f64_t
v_fma_f64 (v_f64_t x, v_f64_t y, v_f64_t z)
{
  return vfmaq_f64 (z, x, y);
}
static inline v_f64_t
v_round_f64 (v_f64_t x)
{
  return vrndaq_f64 (x);
}
static inline v_s64_t
v_round_s64 (v_f64_t x)
{
  return vcvtaq_s64_f64 (x);
}
/* convert to type1 from type2.  */
static inline v_f64_t
v_to_f64_s64 (v_s64_t x)
{
  return (v_f64_t){x[0], x[1]};
}
static inline v_f64_t
v_to_f64_u64 (v_u64_t x)
{
  return (v_f64_t){x[0], x[1]};
}
/* reinterpret as type1 from type2.  */
static inline v_u64_t
v_as_u64_f64 (v_f64_t x)
{
  union { v_f64_t f; v_u64_t u; } r = {x};
  return r.u;
}
static inline v_f64_t
v_as_f64_u64 (v_u64_t x)
{
  union { v_u64_t u; v_f64_t f; } r = {x};
  return r.f;
}
static inline v_s64_t
v_as_s64_u64 (v_u64_t x)
{
  union {  v_u64_t u; v_s64_t i; } r = {x};
  return r.i;
}
static inline v_u64_t
v_as_u64_s64 (v_s64_t x)
{
  union { v_s64_t i; v_u64_t u; } r = {x};
  return r.u;
}
static inline v_f64_t
v_lookup_f64 (const f64_t *tab, v_u64_t idx)
{
  return (v_f64_t){tab[idx[0]], tab[idx[1]]};
}
static inline v_u64_t
v_lookup_u64 (const u64_t *tab, v_u64_t idx)
{
  return (v_u64_t){tab[idx[0]], tab[idx[1]]};
}
static inline v_f64_t
v_call_f64 (f64_t (*f) (f64_t), v_f64_t x, v_f64_t y, v_u64_t p)
{
  return (v_f64_t){p[0] ? f (x[0]) : y[0], p[1] ? f (x[1]) : y[1]};
}
#endif

#endif
#endif
