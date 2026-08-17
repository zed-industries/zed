/*
 * Configuration for math routines.
 *
 * Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
 * See https://llvm.org/LICENSE.txt for license information.
 * SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
 */

#ifndef _MATH_CONFIG_H
#define _MATH_CONFIG_H

#include <math.h>
#include <stdint.h>

#ifndef WANT_ROUNDING
/* If defined to 1, return correct results for special cases in non-nearest
   rounding modes (logf (1.0f) returns 0.0f with FE_DOWNWARD rather than -0.0f).
   This may be set to 0 if there is no fenv support or if math functions only
   get called in round to nearest mode.  */
# define WANT_ROUNDING 1
#endif
#ifndef WANT_ERRNO
/* If defined to 1, set errno in math functions according to ISO C.  Many math
   libraries do not set errno, so this is 0 by default.  It may need to be
   set to 1 if math.h has (math_errhandling & MATH_ERRNO) != 0.  */
# define WANT_ERRNO 0
#endif
#ifndef WANT_ERRNO_UFLOW
/* Set errno to ERANGE if result underflows to 0 (in all rounding modes).  */
# define WANT_ERRNO_UFLOW (WANT_ROUNDING && WANT_ERRNO)
#endif

/* Compiler can inline round as a single instruction.  */
#ifndef HAVE_FAST_ROUND
# if __aarch64__
#   define HAVE_FAST_ROUND 1
# else
#   define HAVE_FAST_ROUND 0
# endif
#endif

/* Compiler can inline lround, but not (long)round(x).  */
#ifndef HAVE_FAST_LROUND
# if __aarch64__ && (100*__GNUC__ + __GNUC_MINOR__) >= 408 && __NO_MATH_ERRNO__
#   define HAVE_FAST_LROUND 1
# else
#   define HAVE_FAST_LROUND 0
# endif
#endif

/* Compiler can inline fma as a single instruction.  */
#ifndef HAVE_FAST_FMA
# if defined FP_FAST_FMA || __aarch64__
#   define HAVE_FAST_FMA 1
# else
#   define HAVE_FAST_FMA 0
# endif
#endif

/* Provide *_finite symbols and some of the glibc hidden symbols
   so libmathlib can be used with binaries compiled against glibc
   to interpose math functions with both static and dynamic linking.  */
#ifndef USE_GLIBC_ABI
# if __GNUC__
#   define USE_GLIBC_ABI 1
# else
#   define USE_GLIBC_ABI 0
# endif
#endif

/* Optionally used extensions.  */
#ifdef __GNUC__
# define HIDDEN __attribute__ ((__visibility__ ("hidden")))
# define NOINLINE __attribute__ ((noinline))
# define UNUSED __attribute__ ((unused))
# define likely(x) __builtin_expect (!!(x), 1)
# define unlikely(x) __builtin_expect (x, 0)
# if __GNUC__ >= 9
#   define attribute_copy(f) __attribute__ ((copy (f)))
# else
#   define attribute_copy(f)
# endif
# define strong_alias(f, a) \
  extern __typeof (f) a __attribute__ ((alias (#f))) attribute_copy (f);
# define hidden_alias(f, a) \
  extern __typeof (f) a __attribute__ ((alias (#f), visibility ("hidden"))) \
  attribute_copy (f);
#else
# define HIDDEN
# define NOINLINE
# define UNUSED
# define likely(x) (x)
# define unlikely(x) (x)
#endif

#if HAVE_FAST_ROUND
/* When set, the roundtoint and converttoint functions are provided with
   the semantics documented below.  */
# define TOINT_INTRINSICS 1

/* Round x to nearest int in all rounding modes, ties have to be rounded
   consistently with converttoint so the results match.  If the result
   would be outside of [-2^31, 2^31-1] then the semantics is unspecified.  */
static inline double_t
roundtoint (double_t x)
{
  return round (x);
}

/* Convert x to nearest int in all rounding modes, ties have to be rounded
   consistently with roundtoint.  If the result is not representable in an
   int32_t then the semantics is unspecified.  */
static inline int32_t
converttoint (double_t x)
{
# if HAVE_FAST_LROUND
  return lround (x);
# else
  return (long) round (x);
# endif
}
#endif

static inline uint32_t
asuint (float f)
{
  union
  {
    float f;
    uint32_t i;
  } u = {f};
  return u.i;
}

static inline float
asfloat (uint32_t i)
{
  union
  {
    uint32_t i;
    float f;
  } u = {i};
  return u.f;
}

static inline uint64_t
asuint64 (double f)
{
  union
  {
    double f;
    uint64_t i;
  } u = {f};
  return u.i;
}

static inline double
asdouble (uint64_t i)
{
  union
  {
    uint64_t i;
    double f;
  } u = {i};
  return u.f;
}

#ifndef IEEE_754_2008_SNAN
# define IEEE_754_2008_SNAN 1
#endif
static inline int
issignalingf_inline (float x)
{
  uint32_t ix = asuint (x);
  if (!IEEE_754_2008_SNAN)
    return (ix & 0x7fc00000) == 0x7fc00000;
  return 2 * (ix ^ 0x00400000) > 2u * 0x7fc00000;
}

static inline int
issignaling_inline (double x)
{
  uint64_t ix = asuint64 (x);
  if (!IEEE_754_2008_SNAN)
    return (ix & 0x7ff8000000000000) == 0x7ff8000000000000;
  return 2 * (ix ^ 0x0008000000000000) > 2 * 0x7ff8000000000000ULL;
}

#if __aarch64__ && __GNUC__
/* Prevent the optimization of a floating-point expression.  */
static inline float
opt_barrier_float (float x)
{
  __asm__ __volatile__ ("" : "+w" (x));
  return x;
}
static inline double
opt_barrier_double (double x)
{
  __asm__ __volatile__ ("" : "+w" (x));
  return x;
}
/* Force the evaluation of a floating-point expression for its side-effect.  */
static inline void
force_eval_float (float x)
{
  __asm__ __volatile__ ("" : "+w" (x));
}
static inline void
force_eval_double (double x)
{
  __asm__ __volatile__ ("" : "+w" (x));
}
#else
static inline float
opt_barrier_float (float x)
{
  volatile float y = x;
  return y;
}
static inline double
opt_barrier_double (double x)
{
  volatile double y = x;
  return y;
}
static inline void
force_eval_float (float x)
{
  volatile float y UNUSED = x;
}
static inline void
force_eval_double (double x)
{
  volatile double y UNUSED = x;
}
#endif

/* Evaluate an expression as the specified type, normally a type
   cast should be enough, but compilers implement non-standard
   excess-precision handling, so when FLT_EVAL_METHOD != 0 then
   these functions may need to be customized.  */
static inline float
eval_as_float (float x)
{
  return x;
}
static inline double
eval_as_double (double x)
{
  return x;
}

/* Error handling tail calls for special cases, with a sign argument.
   The sign of the return value is set if the argument is non-zero.  */

/* The result overflows.  */
HIDDEN float __math_oflowf (uint32_t);
/* The result underflows to 0 in nearest rounding mode.  */
HIDDEN float __math_uflowf (uint32_t);
/* The result underflows to 0 in some directed rounding mode only.  */
HIDDEN float __math_may_uflowf (uint32_t);
/* Division by zero.  */
HIDDEN float __math_divzerof (uint32_t);
/* The result overflows.  */
HIDDEN double __math_oflow (uint32_t);
/* The result underflows to 0 in nearest rounding mode.  */
HIDDEN double __math_uflow (uint32_t);
/* The result underflows to 0 in some directed rounding mode only.  */
HIDDEN double __math_may_uflow (uint32_t);
/* Division by zero.  */
HIDDEN double __math_divzero (uint32_t);

/* Error handling using input checking.  */

/* Invalid input unless it is a quiet NaN.  */
HIDDEN float __math_invalidf (float);
/* Invalid input unless it is a quiet NaN.  */
HIDDEN double __math_invalid (double);

/* Error handling using output checking, only for errno setting.  */

/* Check if the result overflowed to infinity.  */
HIDDEN double __math_check_oflow (double);
/* Check if the result underflowed to 0.  */
HIDDEN double __math_check_uflow (double);

/* Check if the result overflowed to infinity.  */
static inline double
check_oflow (double x)
{
  return WANT_ERRNO ? __math_check_oflow (x) : x;
}

/* Check if the result underflowed to 0.  */
static inline double
check_uflow (double x)
{
  return WANT_ERRNO ? __math_check_uflow (x) : x;
}


/* Shared between expf, exp2f and powf.  */
#define EXP2F_TABLE_BITS 5
#define EXP2F_POLY_ORDER 3
extern const struct exp2f_data
{
  uint64_t tab[1 << EXP2F_TABLE_BITS];
  double shift_scaled;
  double poly[EXP2F_POLY_ORDER];
  double shift;
  double invln2_scaled;
  double poly_scaled[EXP2F_POLY_ORDER];
} __exp2f_data HIDDEN;

#define LOGF_TABLE_BITS 4
#define LOGF_POLY_ORDER 4
extern const struct logf_data
{
  struct
  {
    double invc, logc;
  } tab[1 << LOGF_TABLE_BITS];
  double ln2;
  double poly[LOGF_POLY_ORDER - 1]; /* First order coefficient is 1.  */
} __logf_data HIDDEN;

#define LOG2F_TABLE_BITS 4
#define LOG2F_POLY_ORDER 4
extern const struct log2f_data
{
  struct
  {
    double invc, logc;
  } tab[1 << LOG2F_TABLE_BITS];
  double poly[LOG2F_POLY_ORDER];
} __log2f_data HIDDEN;

#define POWF_LOG2_TABLE_BITS 4
#define POWF_LOG2_POLY_ORDER 5
#if TOINT_INTRINSICS
# define POWF_SCALE_BITS EXP2F_TABLE_BITS
#else
# define POWF_SCALE_BITS 0
#endif
#define POWF_SCALE ((double) (1 << POWF_SCALE_BITS))
extern const struct powf_log2_data
{
  struct
  {
    double invc, logc;
  } tab[1 << POWF_LOG2_TABLE_BITS];
  double poly[POWF_LOG2_POLY_ORDER];
} __powf_log2_data HIDDEN;


#define EXP_TABLE_BITS 7
#define EXP_POLY_ORDER 5
/* Use polynomial that is optimized for a wider input range.  This may be
   needed for good precision in non-nearest rounding and !TOINT_INTRINSICS.  */
#define EXP_POLY_WIDE 0
/* Use close to nearest rounding toint when !TOINT_INTRINSICS.  This may be
   needed for good precision in non-nearest rounding and !EXP_POLY_WIDE.  */
#define EXP_USE_TOINT_NARROW 0
#define EXP2_POLY_ORDER 5
#define EXP2_POLY_WIDE 0
extern const struct exp_data
{
  double invln2N;
  double shift;
  double negln2hiN;
  double negln2loN;
  double poly[4]; /* Last four coefficients.  */
  double exp2_shift;
  double exp2_poly[EXP2_POLY_ORDER];
  uint64_t tab[2*(1 << EXP_TABLE_BITS)];
} __exp_data HIDDEN;

#define LOG_TABLE_BITS 7
#define LOG_POLY_ORDER 6
#define LOG_POLY1_ORDER 12
extern const struct log_data
{
  double ln2hi;
  double ln2lo;
  double poly[LOG_POLY_ORDER - 1]; /* First coefficient is 1.  */
  double poly1[LOG_POLY1_ORDER - 1];
  struct {double invc, logc;} tab[1 << LOG_TABLE_BITS];
#if !HAVE_FAST_FMA
  struct {double chi, clo;} tab2[1 << LOG_TABLE_BITS];
#endif
} __log_data HIDDEN;

#define LOG2_TABLE_BITS 6
#define LOG2_POLY_ORDER 7
#define LOG2_POLY1_ORDER 11
extern const struct log2_data
{
  double invln2hi;
  double invln2lo;
  double poly[LOG2_POLY_ORDER - 1];
  double poly1[LOG2_POLY1_ORDER - 1];
  struct {double invc, logc;} tab[1 << LOG2_TABLE_BITS];
#if !HAVE_FAST_FMA
  struct {double chi, clo;} tab2[1 << LOG2_TABLE_BITS];
#endif
} __log2_data HIDDEN;

#define POW_LOG_TABLE_BITS 7
#define POW_LOG_POLY_ORDER 8
extern const struct pow_log_data
{
  double ln2hi;
  double ln2lo;
  double poly[POW_LOG_POLY_ORDER - 1]; /* First coefficient is 1.  */
  /* Note: the pad field is unused, but allows slightly faster indexing.  */
  struct {double invc, pad, logc, logctail;} tab[1 << POW_LOG_TABLE_BITS];
} __pow_log_data HIDDEN;

#endif
