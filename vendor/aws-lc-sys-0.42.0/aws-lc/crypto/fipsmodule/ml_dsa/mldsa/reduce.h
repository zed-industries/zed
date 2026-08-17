/*
 * Copyright (c) The mldsa-native project authors
 * SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT
 */
#ifndef MLD_REDUCE_H
#define MLD_REDUCE_H

#include "cbmc.h"
#include "common.h"
#include "ct.h"
#include "debug.h"

/* check-magic: -4186625 == pow(2,32,MLDSA_Q) */
#define MLD_MONT -4186625

/* Upper bound for domain of mld_reduce32() */
#define MLD_REDUCE32_DOMAIN_MAX (INT32_MAX - (1 << 22))

/* Absolute bound for range of mld_reduce32() */
/* check-magic: 6283009 == (MLD_REDUCE32_DOMAIN_MAX - 255 * MLDSA_Q + 1) */
#define MLD_REDUCE32_RANGE_MAX 6283009

/**
 * Generic Montgomery reduction; given a 64-bit integer a, computes a 32-bit
 * integer congruent to a * R^-1 mod MLDSA_Q, where R=2^32.
 *
 * @param a Input integer to be reduced, of absolute value smaller or equal
 *          to INT64_MAX - 2^31 * MLDSA_Q.
 *
 * @return Integer congruent to a * R^-1 modulo MLDSA_Q, with absolute value
 *         <= |a| / 2^32 + MLDSA_Q / 2.
 *         In particular, if |a| < 2^31 * MLDSA_Q, the absolute value of the
 *         return value is < MLDSA_Q.
 */
MLD_MUST_CHECK_RETURN_VALUE
static MLD_INLINE int32_t mld_montgomery_reduce(int64_t a)
__contract__(
  /* We don't attempt to express an input-dependent output bound
   * as the post-condition here, as all call-sites satisfy the
   * absolute input bound 2^31 * MLDSA_Q and higher-level
   * reasoning can be conducted using |return_value| < MLDSA_Q. */
  requires(a > -(((int64_t)1 << 31) * MLDSA_Q) &&
           a <  (((int64_t)1 << 31) * MLDSA_Q))
  ensures(return_value > -MLDSA_Q && return_value < MLDSA_Q)
)
{
  /* check-magic: 58728449 == unsigned_mod(pow(MLDSA_Q, -1, 2^32), 2^32) */
  const uint64_t QINV = 58728449;

  /*  Compute a*q^{-1} mod 2^32 in unsigned representatives */
  const uint32_t a_reduced = mld_cast_int64_to_uint32(a);
  const uint32_t a_inverted = (a_reduced * QINV) & UINT32_MAX;

  /* Lift to signed canonical representative mod 2^32. */
  const int32_t t = mld_cast_uint32_to_int32(a_inverted);

  int64_t r;

  mld_assert(a < +(INT64_MAX - (((int64_t)1 << 31) * MLDSA_Q)) &&
             a > -(INT64_MAX - (((int64_t)1 << 31) * MLDSA_Q)));

  r = a - (int64_t)t * MLDSA_Q;

  /*
   * PORTABILITY: Right-shift on a signed integer is, strictly-speaking,
   * implementation-defined for negative left argument. Here,
   * we assume it's sign-preserving "arithmetic" shift right. (C99 6.5.7 (5))
   */
  r = r >> 32;

  /* Bounds:
   *
   * By construction of the Montgomery multiplication, by the time we
   * compute r >> 32, r is divisible by 2^32, and hence
   *
   *   |r >> 32|  = |r| / 2^32
   *             <= |a| / 2^32 + MLDSA_Q / 2
   *
   * (In general, we would only have |x >> n| <= ceil(|x| / 2^n)).
   *
   * In particular, if |a| < 2^31 * MLDSA_Q, then |return_value| < MLDSA_Q.
   */
  return (int32_t)r;
}

/**
 * For finite field element a with a <= 2^{31} - 2^{22} - 1, compute
 * r congruent to a (mod MLDSA_Q) such that
 * -MLD_REDUCE32_RANGE_MAX <= r < MLD_REDUCE32_RANGE_MAX.
 *
 * @param a Finite field element.
 *
 * @return r.
 */
MLD_MUST_CHECK_RETURN_VALUE
static MLD_INLINE int32_t mld_reduce32(int32_t a)
__contract__(
  requires(a <= MLD_REDUCE32_DOMAIN_MAX)
  ensures(return_value >= -MLD_REDUCE32_RANGE_MAX)
  ensures(return_value <   MLD_REDUCE32_RANGE_MAX)
)
{
  int32_t t;

  t = (a + (1 << 22)) >> 23;
  t = a - t * MLDSA_Q;
  mld_assert((t - a) % MLDSA_Q == 0);
  return t;
}

/**
 * Add MLDSA_Q if input coefficient is negative.
 *
 * @param a Finite field element.
 *
 * @return r.
 */
MLD_MUST_CHECK_RETURN_VALUE
static MLD_INLINE int32_t mld_caddq(int32_t a)
__contract__(
  requires(a > -MLDSA_Q)
  requires(a < MLDSA_Q)
  ensures(return_value >= 0)
  ensures(return_value < MLDSA_Q)
  ensures(return_value == ((a >= 0) ? a : (a + MLDSA_Q)))
)
{
  return mld_ct_sel_int32(a + MLDSA_Q, a, mld_ct_cmask_neg_i32(a));
}


#endif /* !MLD_REDUCE_H */
