/*
 * Copyright (c) The mlkem-native project authors
 * Copyright (c) The mldsa-native project authors
 * SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT
 */
#ifndef MLD_DEBUG_H
#define MLD_DEBUG_H
#include "common.h"

#if defined(MLDSA_DEBUG)

/**
 * Check debug assertion.
 *
 * Prints an error message to stderr and calls exit(1) if not.
 *
 * @param file Filename.
 * @param line Line number.
 * @param val  Value asserted to be non-zero.
 */
#define mld_debug_check_assert MLD_NAMESPACE(mldsa_debug_assert)
void mld_debug_check_assert(const char *file, int line, const int val);

/**
 * Check whether values in an array of int32_t are within specified bounds.
 *
 * Prints an error message to stderr and calls exit(1) if not.
 *
 * @param     file                  Filename.
 * @param     line                  Line number.
 * @param[in] ptr                   Base of array to be checked.
 * @param     len                   Number of int32_t in ptr.
 * @param     lower_bound_exclusive Exclusive lower bound.
 * @param     upper_bound_exclusive Exclusive upper bound.
 */
#define mld_debug_check_bounds MLD_NAMESPACE(mldsa_debug_check_bounds)
void mld_debug_check_bounds(const char *file, int line, const int32_t *ptr,
                            unsigned len, int64_t lower_bound_exclusive,
                            int64_t upper_bound_exclusive);

/* Check assertion, calling exit() upon failure
 *
 * val: Value that's asserted to be non-zero
 */
#define mld_assert(val) mld_debug_check_assert(__FILE__, __LINE__, (val))

/* Check bounds in array of int32_t's
 * ptr: Base of int32_t array; will be explicitly cast to int32_t*,
 *      so you may pass a byte-compatible type such as mld_poly or mld_polyvec.
 * len: Number of int32_t in array
 * value_lb: Inclusive lower value bound
 * value_ub: Exclusive upper value bound */
#define mld_assert_bound(ptr, len, value_lb, value_ub)                      \
  mld_debug_check_bounds(__FILE__, __LINE__, (const int32_t *)(ptr), (len), \
                         ((int64_t)(value_lb)) - 1, (value_ub))

/* Check absolute bounds in array of int32_t's
 * ptr: Base of array, expression of type int32_t*
 * len: Number of int32_t in array
 * value_abs_bd: Exclusive absolute upper bound */
#define mld_assert_abs_bound(ptr, len, value_abs_bd)               \
  mld_assert_bound((ptr), (len), (-((int64_t)(value_abs_bd)) + 1), \
                   (value_abs_bd))

/* Version of bounds assertions for 2-dimensional arrays */
#define mld_assert_bound_2d(ptr, len0, len1, value_lb, value_ub) \
  mld_assert_bound((ptr), ((len0) * (len1)), (value_lb), (value_ub))

#define mld_assert_abs_bound_2d(ptr, len0, len1, value_abs_bd) \
  mld_assert_abs_bound((ptr), ((len0) * (len1)), (value_abs_bd))

/* When running CBMC, convert debug assertions into proof obligations */
#elif defined(CBMC)
#include "cbmc.h"

#define mld_assert(val) cassert(val)

#define mld_assert_bound(ptr, len, value_lb, value_ub) \
  cassert(array_bound(((int32_t *)(ptr)), 0, (len), (value_lb), (value_ub)))

#define mld_assert_abs_bound(ptr, len, value_abs_bd) \
  cassert(array_abs_bound(((int32_t *)(ptr)), 0, (len), (value_abs_bd)))

/* Because of https://github.com/diffblue/cbmc/issues/8570, we can't
 * just use a single flattened array_bound(...) here. */
#define mld_assert_bound_2d(ptr, M, N, value_lb, value_ub)              \
  cassert(forall(kN, 0, (M),                                            \
                 array_bound(&((int32_t (*)[(N)])(ptr))[kN][0], 0, (N), \
                             (value_lb), (value_ub))))

#define mld_assert_abs_bound_2d(ptr, M, N, value_abs_bd)                    \
  cassert(forall(kN, 0, (M),                                                \
                 array_abs_bound(&((int32_t (*)[(N)])(ptr))[kN][0], 0, (N), \
                                 (value_abs_bd))))

#else /* !MLDSA_DEBUG && CBMC */

#define mld_assert(val) \
  do                    \
  {                     \
  } while (0)
#define mld_assert_bound(ptr, len, value_lb, value_ub) \
  do                                                   \
  {                                                    \
  } while (0)
#define mld_assert_abs_bound(ptr, len, value_abs_bd) \
  do                                                 \
  {                                                  \
  } while (0)

#define mld_assert_bound_2d(ptr, len0, len1, value_lb, value_ub) \
  do                                                             \
  {                                                              \
  } while (0)

#define mld_assert_abs_bound_2d(ptr, len0, len1, value_abs_bd) \
  do                                                           \
  {                                                            \
  } while (0)


#endif /* !MLDSA_DEBUG && !CBMC */
#endif /* !MLD_DEBUG_H */
