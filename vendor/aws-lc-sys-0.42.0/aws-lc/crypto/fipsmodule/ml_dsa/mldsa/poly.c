/*
 * Copyright (c) The mldsa-native project authors
 * Copyright (c) The mlkem-native project authors
 * SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT
 */

/* References
 * ==========
 *
 * - [FIPS204]
 *   FIPS 204 Module-Lattice-Based Digital Signature Standard
 *   National Institute of Standards and Technology
 *   https://csrc.nist.gov/pubs/fips/204/final
 *
 * - [REF]
 *   CRYSTALS-Dilithium reference implementation
 *   Bai, Ducas, Kiltz, Lepoint, Lyubashevsky, Schwabe, Seiler, Stehlé
 *   https://github.com/pq-crystals/dilithium/tree/master/ref
 */

#include "poly.h"

#include "common.h"
#include "ct.h"
#include "debug.h"
#include "reduce.h"
#include "rounding.h"
#include "symmetric.h"

#if !defined(MLD_CONFIG_MULTILEVEL_NO_SHARED)
#include "zetas.inc"

MLD_INTERNAL_API
void mld_poly_reduce(mld_poly *a)
{
  unsigned int i;
  mld_assert_bound(a->coeffs, MLDSA_N, INT32_MIN, MLD_REDUCE32_DOMAIN_MAX);

  for (i = 0; i < MLDSA_N; ++i)
  __loop__(
    invariant(i <= MLDSA_N)
    invariant(forall(k0, i, MLDSA_N, a->coeffs[k0] == loop_entry(*a).coeffs[k0]))
    invariant(array_bound(a->coeffs, 0, i, -MLD_REDUCE32_RANGE_MAX, MLD_REDUCE32_RANGE_MAX))
    decreases(MLDSA_N - i))
  {
    a->coeffs[i] = mld_reduce32(a->coeffs[i]);
  }

  mld_assert_bound(a->coeffs, MLDSA_N, -MLD_REDUCE32_RANGE_MAX,
                   MLD_REDUCE32_RANGE_MAX);
}

MLD_STATIC_TESTABLE void mld_poly_caddq_c(mld_poly *a)
__contract__(
  requires(memory_no_alias(a, sizeof(mld_poly)))
  requires(array_abs_bound(a->coeffs, 0, MLDSA_N, MLDSA_Q))
  assigns(memory_slice(a, sizeof(mld_poly)))
  ensures(array_bound(a->coeffs, 0, MLDSA_N, 0, MLDSA_Q))
)
{
  unsigned int i;
  mld_assert_abs_bound(a->coeffs, MLDSA_N, MLDSA_Q);

  for (i = 0; i < MLDSA_N; ++i)
  __loop__(
    invariant(i <= MLDSA_N)
    invariant(forall(k0, i, MLDSA_N, a->coeffs[k0] == loop_entry(*a).coeffs[k0]))
    invariant(array_bound(a->coeffs, 0, i, 0, MLDSA_Q))
    decreases(MLDSA_N - i)
    )
  {
    a->coeffs[i] = mld_caddq(a->coeffs[i]);
  }

  mld_assert_bound(a->coeffs, MLDSA_N, 0, MLDSA_Q);
}

MLD_INTERNAL_API
void mld_poly_caddq(mld_poly *a)
{
#if defined(MLD_USE_NATIVE_POLY_CADDQ)
  int ret;
  mld_assert_abs_bound(a->coeffs, MLDSA_N, MLDSA_Q);
  ret = mld_poly_caddq_native(a->coeffs);
  if (ret == MLD_NATIVE_FUNC_SUCCESS)
  {
    mld_assert_bound(a->coeffs, MLDSA_N, 0, MLDSA_Q);
    return;
  }
#endif /* MLD_USE_NATIVE_POLY_CADDQ */
  mld_poly_caddq_c(a);
}

#if !defined(MLD_CONFIG_NO_KEYPAIR_API) || !defined(MLD_CONFIG_NO_SIGN_API) || \
    defined(MLD_CONFIG_REDUCE_RAM) || defined(MLD_UNIT_TEST)
/* Reference: We use destructive version (output=first input) to avoid
 *            reasoning about aliasing in the CBMC specification */
MLD_INTERNAL_API
void mld_poly_add(mld_poly *r, const mld_poly *b)
{
  unsigned int i;
  for (i = 0; i < MLDSA_N; ++i)
  __loop__(
    assigns(i, memory_slice(r, sizeof(mld_poly)))
    invariant(i <= MLDSA_N)
    invariant(forall(k0, i, MLDSA_N, r->coeffs[k0] == loop_entry(*r).coeffs[k0]))
    invariant(forall(k1, 0, i, r->coeffs[k1] == loop_entry(*r).coeffs[k1] + b->coeffs[k1]))
    invariant(forall(k2, 0, i, r->coeffs[k2] < MLD_REDUCE32_DOMAIN_MAX))
    invariant(forall(k2, 0, i, r->coeffs[k2] >= INT32_MIN))
    decreases(MLDSA_N - i)
  )
  {
    r->coeffs[i] = r->coeffs[i] + b->coeffs[i];
  }
}
#endif /* !MLD_CONFIG_NO_KEYPAIR_API || !MLD_CONFIG_NO_SIGN_API || \
          MLD_CONFIG_REDUCE_RAM || MLD_UNIT_TEST */

#if !defined(MLD_CONFIG_NO_SIGN_API) || !defined(MLD_CONFIG_NO_VERIFY_API)
/* Reference: We use destructive version (output=first input) to avoid
 *            reasoning about aliasing in the CBMC specification */
MLD_INTERNAL_API
void mld_poly_sub(mld_poly *r, const mld_poly *b)
{
  unsigned int i;
  mld_assert_abs_bound(b->coeffs, MLDSA_N, MLDSA_Q);
  mld_assert_abs_bound(r->coeffs, MLDSA_N, MLDSA_Q);

  for (i = 0; i < MLDSA_N; ++i)
  __loop__(
    invariant(i <= MLDSA_N)
    invariant(array_bound(r->coeffs, 0, i, INT32_MIN, MLD_REDUCE32_DOMAIN_MAX))
    invariant(forall(k0, i, MLDSA_N, r->coeffs[k0] == loop_entry(*r).coeffs[k0]))
    decreases(MLDSA_N - i)
  )
  {
    r->coeffs[i] = r->coeffs[i] - b->coeffs[i];
  }

  mld_assert_bound(r->coeffs, MLDSA_N, INT32_MIN, MLD_REDUCE32_DOMAIN_MAX);
}
#endif /* !MLD_CONFIG_NO_SIGN_API || !MLD_CONFIG_NO_VERIFY_API */

#if !defined(MLD_CONFIG_NO_VERIFY_API)
MLD_INTERNAL_API
void mld_poly_shiftl(mld_poly *a)
{
  unsigned int i;
  mld_assert_bound(a->coeffs, MLDSA_N, 0, 1 << 10);

  for (i = 0; i < MLDSA_N; i++)
  __loop__(
    invariant(i <= MLDSA_N)
    invariant(array_bound(a->coeffs, 0, i, 0, MLDSA_Q))
    invariant(forall(k0, i, MLDSA_N, a->coeffs[k0] == loop_entry(*a).coeffs[k0]))
    decreases(MLDSA_N - i))
  {
    /* Reference: uses a left shift by MLDSA_D which is undefined behaviour in
     * C90/C99
     */
    a->coeffs[i] *= (1 << MLDSA_D);
  }
  mld_assert_bound(a->coeffs, MLDSA_N, 0, MLDSA_Q);
}
#endif /* !MLD_CONFIG_NO_VERIFY_API */

static MLD_INLINE int32_t mld_fqmul(int32_t a, int32_t b)
__contract__(
  requires(b > -MLDSA_Q_HALF && b < MLDSA_Q_HALF)
  ensures(return_value > -MLD_FQMUL_BOUND && return_value < MLD_FQMUL_BOUND)
)
{
  /* Bounds: We argue in mld_montgomery_reduce() that the result
   * of Montgomery reduction is < MLDSA_Q if the input is smaller
   * than 2^31 * MLDSA_Q in absolute value. Indeed, we have:
   *
   *    |a * b|   = |a| * |b|
   *              < 2^31 * MLDSA_Q_HALF
   *              < 2^31 * MLDSA_Q
   *
   * So the output is < MLDSA_Q < MLD_FQMUL_BOUND.
   */
  return mld_montgomery_reduce((int64_t)a * (int64_t)b);
}

/* mld_ntt_butterfly_block()
 *
 * Computes a block CT butterflies with a fixed twiddle factor,
 * using Montgomery multiplication.
 *
 * Parameters:
 * - r: Pointer to base of polynomial (_not_ the base of butterfly block)
 * - zeta: Twiddle factor to use for the butterfly. This must be in
 *         Montgomery form and signed canonical.
 * - start: Offset to the beginning of the butterfly block
 * - len: Index difference between coefficients subject to a butterfly
 * - bound: Ghost variable describing coefficient bound: Prior to `start`,
 *          coefficients must be bound by `bound + MLDSA_Q`. Post `start`,
 *          they must be bound by `bound`.
 * When this function returns, output coefficients in the index range
 * [start, start+2*len) have bound bumped to `bound + MLDSA_Q`.
 * Example:
 * - start=8, len=4
 *   This would compute the following four butterflies
 *          8     --    12
 *             9    --     13
 *                10   --     14
 *                   11   --     15
 * - start=4, len=2
 *   This would compute the following two butterflies
 *          4 -- 6
 *             5 -- 7
 */

/* Reference: Embedded in `ntt()` in the reference implementation @[REF]. */
static MLD_INLINE void mld_ntt_butterfly_block(int32_t r[MLDSA_N],
                                               const int32_t zeta,
                                               const unsigned start,
                                               const unsigned len,
                                               const unsigned bound)
__contract__(
  requires(start < MLDSA_N)
  requires(1 <= len && len <= MLDSA_N / 2 && start + 2 * len <= MLDSA_N)
  requires(0 <= bound && bound < INT32_MAX - MLD_FQMUL_BOUND)
  requires(-MLDSA_Q_HALF < zeta && zeta < MLDSA_Q_HALF)
  requires(memory_no_alias(r, sizeof(int32_t) * MLDSA_N))
  requires(array_abs_bound(r, 0, start, bound + MLD_FQMUL_BOUND))
  requires(array_abs_bound(r, start, MLDSA_N, bound))
  assigns(memory_slice(r, sizeof(int32_t) * MLDSA_N))
  ensures(array_abs_bound(r, 0, start + 2*len, bound + MLD_FQMUL_BOUND))
  ensures(array_abs_bound(r, start + 2 * len, MLDSA_N, bound)))
{
  /* `bound` is a ghost variable only needed in the CBMC specification */
  unsigned j;
  ((void)bound);
  for (j = start; j < start + len; j++)
  __loop__(
    invariant(start <= j && j <= start + len)
    /*
     * Coefficients are updated in strided pairs, so the bounds for the
     * intermediate states alternate twice between the old and new bound
     */
    invariant(array_abs_bound(r, 0,           j,           bound + MLD_FQMUL_BOUND))
    invariant(array_abs_bound(r, j,           start + len, bound))
    invariant(array_abs_bound(r, start + len, j + len,     bound + MLD_FQMUL_BOUND))
    invariant(array_abs_bound(r, j + len,     MLDSA_N,     bound))
    decreases(start + len - j))
  {
    int32_t t;
    t = mld_fqmul(r[j + len], zeta);
    r[j + len] = r[j] - t;
    r[j] = r[j] + t;
  }
}

/* mld_ntt_layer()
 *
 * Compute one layer of forward NTT
 *
 * Parameters:
 * - r:     Pointer to base of polynomial
 * - layer: Indicates which layer is being applied.
 */

/* Reference: Embedded in `ntt()` in the reference implementation @[REF]. */
static MLD_INLINE void mld_ntt_layer(int32_t r[MLDSA_N], const unsigned layer)
__contract__(
  requires(memory_no_alias(r, sizeof(int32_t) * MLDSA_N))
  requires(1 <= layer && layer <= 8)
  requires(array_abs_bound(r, 0, MLDSA_N, layer * MLD_FQMUL_BOUND))
  assigns(memory_slice(r, sizeof(int32_t) * MLDSA_N))
  ensures(array_abs_bound(r, 0, MLDSA_N, (layer + 1) * MLD_FQMUL_BOUND)))
{
  unsigned start, k, len;
  /* Twiddle factors for layer n are at indices 2^(n-1)..2^n-1. */
  k = 1u << (layer - 1);
  len = (unsigned)MLDSA_N >> layer;
  for (start = 0; start < MLDSA_N; start += 2 * len)
  __loop__(
    invariant(start < MLDSA_N + 2 * len)
    invariant(k <= MLDSA_N)
    invariant(2 * len * k == start + MLDSA_N)
    invariant(array_abs_bound(r, 0, start, layer * MLD_FQMUL_BOUND + MLD_FQMUL_BOUND))
    invariant(array_abs_bound(r, start, MLDSA_N, layer * MLD_FQMUL_BOUND))
    decreases(MLDSA_N - start))
  {
    int32_t zeta = mld_zetas[k++];
    mld_ntt_butterfly_block(r, zeta, start, len, layer * MLD_FQMUL_BOUND);
  }
}

MLD_STATIC_TESTABLE void mld_poly_ntt_c(mld_poly *a)
__contract__(
  requires(memory_no_alias(a, sizeof(mld_poly)))
  requires(array_abs_bound(a->coeffs, 0, MLDSA_N, MLDSA_Q))
  assigns(memory_slice(a, sizeof(mld_poly)))
  ensures(array_abs_bound(a->coeffs, 0, MLDSA_N, MLD_NTT_BOUND))
)
{
  unsigned int layer;
  int32_t *r;


  mld_assert_abs_bound(a->coeffs, MLDSA_N, MLDSA_Q);
  r = a->coeffs;

  for (layer = 1; layer < 9; layer++)
  __loop__(
    invariant(1 <= layer && layer <= 9)
    invariant(array_abs_bound(r, 0, MLDSA_N, layer * MLD_FQMUL_BOUND))
    decreases(9 - layer)
  )
  {
    mld_ntt_layer(r, layer);
  }

  mld_assert_abs_bound(a->coeffs, MLDSA_N, MLD_NTT_BOUND);
}

MLD_INTERNAL_API
void mld_poly_ntt(mld_poly *a)
{
#if defined(MLD_USE_NATIVE_NTT)
  int ret;
  mld_assert_abs_bound(a->coeffs, MLDSA_N, MLDSA_Q);
  ret = mld_ntt_native(a->coeffs);
  if (ret == MLD_NATIVE_FUNC_SUCCESS)
  {
    mld_assert_abs_bound(a->coeffs, MLDSA_N, MLD_NTT_BOUND);
    return;
  }
#endif /* MLD_USE_NATIVE_NTT */
  mld_poly_ntt_c(a);
}

/**
 * Scale a field element by mont/256, i.e., perform Montgomery multiplication
 * by mont^2/256.
 *
 * Input is expected to have absolute value smaller than 256 * MLDSA_Q. Output
 * has absolute value smaller than MLD_INTT_BOUND.
 *
 * @param a Field element to be scaled.
 */
static MLD_INLINE int32_t mld_fqscale(int32_t a)
__contract__(
  requires(a > -256*MLDSA_Q && a < 256*MLDSA_Q)
  ensures(return_value > -MLD_INTT_BOUND && return_value < MLD_INTT_BOUND)
)
{
  /* check-magic: 41978 == pow(2,64-8,MLDSA_Q) */
  const int32_t f = 41978;
  /* Bounds: MLD_INTT_BOUND is MLDSA_Q, so the bounds reasoning is just
   * a special case of that in mld_fqmul(). */
  return mld_montgomery_reduce((int64_t)a * f);
}

/* Reference: Embedded into `invntt_tomont()` in the reference implementation
 * @[REF] */
static MLD_INLINE void mld_invntt_layer(int32_t r[MLDSA_N], unsigned layer)
__contract__(
  requires(memory_no_alias(r, sizeof(int32_t) * MLDSA_N))
  requires(1 <= layer && layer <= 8)
  requires(array_abs_bound(r, 0, MLDSA_N, (MLDSA_N >> layer) * MLDSA_Q))
  assigns(memory_slice(r, sizeof(int32_t) * MLDSA_N))
  ensures(array_abs_bound(r, 0, MLDSA_N, (MLDSA_N >> (layer - 1)) * MLDSA_Q)))
{
  unsigned start, k, len;
  len = (unsigned)MLDSA_N >> layer;
  k = (1u << layer) - 1;
  for (start = 0; start < MLDSA_N; start += 2 * len)
  __loop__(
    invariant(start <= MLDSA_N && k <= 255)
    invariant(2 * len * k + start == 2 * MLDSA_N - 2 * len)
    invariant(array_abs_bound(r, 0, start, (MLDSA_N >> (layer - 1)) * MLDSA_Q))
    invariant(array_abs_bound(r, start, MLDSA_N, (MLDSA_N >> layer) * MLDSA_Q))
    decreases(MLDSA_N - start))
  {
    unsigned j;
    int32_t zeta = -mld_zetas[k--];

    /* The bound `(MLDSA_N >> (layer - 1)) * MLDSA_Q` is loose enough to
     * cover both the input bound `(MLDSA_N >> layer) * MLDSA_Q`
     * (for layers >= 1) and the fqmul output bound `MLD_FQMUL_BOUND`
     * (which is < 2 * MLDSA_Q <= (MLDSA_N >> (layer - 1)) * MLDSA_Q). */
    for (j = start; j < start + len; j++)
    __loop__(
      invariant(start <= j && j <= start + len)
      invariant(array_abs_bound(r, 0, start, (MLDSA_N >> (layer - 1)) * MLDSA_Q))
      invariant(array_abs_bound(r, start, j, (MLDSA_N >> (layer - 1)) * MLDSA_Q))
      invariant(array_abs_bound(r, j, start + len, (MLDSA_N >> layer) * MLDSA_Q))
      invariant(array_abs_bound(r, start + len, j + len, (MLDSA_N >> (layer - 1)) * MLDSA_Q))
      invariant(array_abs_bound(r, j + len, MLDSA_N, (MLDSA_N >> layer) * MLDSA_Q))
      decreases(start + len - j))
    {
      int32_t t = r[j];
      r[j] = t + r[j + len];
      r[j + len] = t - r[j + len];
      r[j + len] = mld_fqmul(r[j + len], zeta);
    }
  }
}

MLD_STATIC_TESTABLE void mld_poly_invntt_tomont_c(mld_poly *a)
__contract__(
  requires(memory_no_alias(a, sizeof(mld_poly)))
  requires(array_abs_bound(a->coeffs, 0, MLDSA_N, MLDSA_Q))
  assigns(memory_slice(a, sizeof(mld_poly)))
  ensures(array_abs_bound(a->coeffs, 0, MLDSA_N, MLD_INTT_BOUND))
)
{
  unsigned int layer, j;
  int32_t *r;

  mld_assert_abs_bound(a->coeffs, MLDSA_N, MLDSA_Q);

  r = a->coeffs;
  for (layer = 8; layer >= 1; layer--)
  __loop__(
    invariant(layer <= 8)
    /* Absolute bounds increase from 1Q before layer 8 */
    /* up to 256Q after layer 1                        */
    invariant(array_abs_bound(r, 0, MLDSA_N, (MLDSA_N >> layer) * MLDSA_Q))
    decreases(layer))
  {
    mld_invntt_layer(r, layer);
  }

  /* Coefficient bounds are now at 256Q. We now scale by mont / 256,
   * i.e., compute the Montgomery multiplication by mont^2 / 256.
   * mont corrects the mont^-1  factor introduced in the basemul.
   * 1/256 performs that scaling of the inverse NTT.
   * The reduced value is bounded by  MLD_INTT_BOUND in absolute
   * value.*/
  for (j = 0; j < MLDSA_N; ++j)
  __loop__(
    invariant(j <= MLDSA_N)
    invariant(array_abs_bound(r, 0, j, MLD_INTT_BOUND))
    invariant(array_abs_bound(r, j, MLDSA_N, MLDSA_N * MLDSA_Q))
    decreases(MLDSA_N - j)
  )
  {
    r[j] = mld_fqscale(r[j]);
  }

  mld_assert_abs_bound(a->coeffs, MLDSA_N, MLD_INTT_BOUND);
}


MLD_INTERNAL_API
void mld_poly_invntt_tomont(mld_poly *a)
{
#if defined(MLD_USE_NATIVE_INTT)
  int ret;
  mld_assert_abs_bound(a->coeffs, MLDSA_N, MLDSA_Q);
  ret = mld_intt_native(a->coeffs);
  if (ret == MLD_NATIVE_FUNC_SUCCESS)
  {
    mld_assert_abs_bound(a->coeffs, MLDSA_N, MLD_INTT_BOUND);
    return;
  }
#endif /* MLD_USE_NATIVE_INTT */
  mld_poly_invntt_tomont_c(a);
}

#if !defined(MLD_CONFIG_NO_SIGN_API) || !defined(MLD_CONFIG_NO_VERIFY_API) || \
    defined(MLD_CONFIG_REDUCE_RAM) || defined(MLD_UNIT_TEST)
MLD_STATIC_TESTABLE void mld_poly_pointwise_montgomery_c(mld_poly *a,
                                                         const mld_poly *b)
__contract__(
  requires(memory_no_alias(a, sizeof(mld_poly)))
  requires(memory_no_alias(b, sizeof(mld_poly)))
  requires(array_abs_bound(a->coeffs, 0, MLDSA_N, MLD_NTT_BOUND))
  requires(array_abs_bound(b->coeffs, 0, MLDSA_N, MLD_NTT_BOUND))
  assigns(memory_slice(a, sizeof(mld_poly)))
  ensures(array_abs_bound(a->coeffs, 0, MLDSA_N, MLDSA_Q))
)
{
  unsigned int i;
  mld_assert_abs_bound(a->coeffs, MLDSA_N, MLD_NTT_BOUND);
  mld_assert_abs_bound(b->coeffs, MLDSA_N, MLD_NTT_BOUND);

  for (i = 0; i < MLDSA_N; ++i)
  __loop__(
    invariant(i <= MLDSA_N)
    invariant(array_abs_bound(a->coeffs, 0, i, MLDSA_Q))
    invariant(array_abs_bound(a->coeffs, i, MLDSA_N, MLD_NTT_BOUND))
    decreases(MLDSA_N - i)
  )
  {
    a->coeffs[i] = mld_montgomery_reduce((int64_t)a->coeffs[i] * b->coeffs[i]);
  }
  mld_assert_abs_bound(a->coeffs, MLDSA_N, MLDSA_Q);
}

MLD_INTERNAL_API
void mld_poly_pointwise_montgomery(mld_poly *a, const mld_poly *b)
{
#if defined(MLD_USE_NATIVE_POINTWISE_MONTGOMERY)
  int ret;
  mld_assert_abs_bound(a->coeffs, MLDSA_N, MLD_NTT_BOUND);
  mld_assert_abs_bound(b->coeffs, MLDSA_N, MLD_NTT_BOUND);
  ret = mld_poly_pointwise_montgomery_native(a->coeffs, b->coeffs);
  if (ret == MLD_NATIVE_FUNC_SUCCESS)
  {
    mld_assert_abs_bound(a->coeffs, MLDSA_N, MLDSA_Q);
    return;
  }
#endif /* MLD_USE_NATIVE_POINTWISE_MONTGOMERY */
  mld_poly_pointwise_montgomery_c(a, b);
}
#endif /* !MLD_CONFIG_NO_SIGN_API || !MLD_CONFIG_NO_VERIFY_API || \
          MLD_CONFIG_REDUCE_RAM || MLD_UNIT_TEST */

#if !defined(MLD_CONFIG_NO_KEYPAIR_API)
MLD_INTERNAL_API
void mld_poly_power2round(mld_poly *a1, mld_poly *a0, const mld_poly *a)
{
  unsigned int i;
  mld_assert_bound(a->coeffs, MLDSA_N, 0, MLDSA_Q);

  for (i = 0; i < MLDSA_N; ++i)
  __loop__(
    assigns(i, memory_slice(a0, sizeof(mld_poly)), memory_slice(a1, sizeof(mld_poly)))
    invariant(i <= MLDSA_N)
    invariant(forall(k0, i, MLDSA_N, a->coeffs[k0] == loop_entry(*a).coeffs[k0]))
    invariant(array_bound(a0->coeffs, 0, i, -(MLD_2_POW_D/2)+1, (MLD_2_POW_D/2)+1))
    invariant(array_bound(a1->coeffs, 0, i, 0, ((MLDSA_Q - 1) / MLD_2_POW_D) + 1))
    decreases(MLDSA_N - i)
  )
  {
    mld_power2round(&a0->coeffs[i], &a1->coeffs[i], a->coeffs[i]);
  }

  mld_assert_bound(a0->coeffs, MLDSA_N, -(MLD_2_POW_D / 2) + 1,
                   (MLD_2_POW_D / 2) + 1);
  mld_assert_bound(a1->coeffs, MLDSA_N, 0, ((MLDSA_Q - 1) / MLD_2_POW_D) + 1);
}
#endif /* !MLD_CONFIG_NO_KEYPAIR_API */

#ifndef MLD_POLY_UNIFORM_NBLOCKS
#define MLD_POLY_UNIFORM_NBLOCKS \
  ((768 + MLD_STREAM128_BLOCKBYTES - 1) / MLD_STREAM128_BLOCKBYTES)
#endif
/* Reference: `mld_rej_uniform()` in the reference implementation @[REF].
 *            - Our signature differs from the reference implementation
 *              in that it adds the offset and always expects the base of the
 *              target buffer. This avoids shifting the buffer base in the
 *              caller, which appears tricky to reason about. */
MLD_STATIC_TESTABLE unsigned int mld_rej_uniform_c(int32_t *a,
                                                   unsigned int target,
                                                   unsigned int offset,
                                                   const uint8_t *buf,
                                                   unsigned int buflen)
__contract__(
  requires(offset <= target && target <= MLDSA_N)
  requires(buflen <= (MLD_POLY_UNIFORM_NBLOCKS * MLD_STREAM128_BLOCKBYTES) && buflen % 3 == 0)
  requires(memory_no_alias(a, sizeof(int32_t) * target))
  requires(memory_no_alias(buf, buflen))
  requires(array_bound(a, 0, offset, 0, MLDSA_Q))
  assigns(memory_slice(a, sizeof(int32_t) * target))
  ensures(offset <= return_value && return_value <= target)
  ensures(array_bound(a, 0, return_value, 0, MLDSA_Q))
)
{
  unsigned int ctr, pos;
  uint32_t t;
  mld_assert_bound(a, offset, 0, MLDSA_Q);

  ctr = offset;
  pos = 0;
  /* pos + 3 cannot overflow due to the assumption
  buflen <= (MLD_POLY_UNIFORM_NBLOCKS * MLD_STREAM128_BLOCKBYTES) */
  while (ctr < target && pos + 3 <= buflen)
  __loop__(
    invariant(offset <= ctr && ctr <= target && pos <= buflen)
    invariant(array_bound(a, 0, ctr, 0, MLDSA_Q))
    decreases(buflen - pos))
  {
    t = buf[pos++];
    t |= (uint32_t)buf[pos++] << 8;
    t |= (uint32_t)buf[pos++] << 16;
    t &= 0x7FFFFF;

    if (t < MLDSA_Q)
    {
      a[ctr++] = (int32_t)t;
    }
  }

  mld_assert_bound(a, ctr, 0, MLDSA_Q);

  return ctr;
}
/**
 * Sample uniformly random coefficients in [0, MLDSA_Q-1] by performing
 * rejection sampling on an array of random bytes.
 *
 * @param[out] a      Pointer to output array (allocated).
 * @param      target Requested number of coefficients to sample.
 * @param      offset Number of coefficients already sampled.
 * @param[in]  buf    Array of random bytes to sample from.
 * @param      buflen Length of array of random bytes (must be multiple of 3).
 *
 * @return Number of sampled coefficients. Can be smaller than len if not
 *         enough random bytes were given.
 */

/* Reference: `mld_rej_uniform()` in the reference implementation @[REF].
 *            - Our signature differs from the reference implementation
 *              in that it adds the offset and always expects the base of the
 *              target buffer. This avoids shifting the buffer base in the
 *              caller, which appears tricky to reason about. */
static unsigned int mld_rej_uniform(int32_t *a, unsigned int target,
                                    unsigned int offset, const uint8_t *buf,
                                    unsigned int buflen)
__contract__(
  requires(offset <= target && target <= MLDSA_N)
  requires(buflen <= (MLD_POLY_UNIFORM_NBLOCKS * MLD_STREAM128_BLOCKBYTES) && buflen % 3 == 0)
  requires(memory_no_alias(a, sizeof(int32_t) * target))
  requires(memory_no_alias(buf, buflen))
  requires(array_bound(a, 0, offset, 0, MLDSA_Q))
  assigns(memory_slice(a, sizeof(int32_t) * target))
  ensures(offset <= return_value && return_value <= target)
  ensures(array_bound(a, 0, return_value, 0, MLDSA_Q))
)
{
#if defined(MLD_USE_NATIVE_REJ_UNIFORM)
  int ret;
  mld_assert_bound(a, offset, 0, MLDSA_Q);
  if (offset == 0)
  {
    ret = mld_rej_uniform_native(a, target, buf, buflen);
    if (ret != MLD_NATIVE_FUNC_FALLBACK)
    {
      unsigned res = (unsigned)ret;
      mld_assert_bound(a, res, 0, MLDSA_Q);
      return res;
    }
  }
#endif /* MLD_USE_NATIVE_REJ_UNIFORM */

  return mld_rej_uniform_c(a, target, offset, buf, buflen);
}

/* Reference: poly_uniform() in the reference implementation @[REF].
 *           - Simplified from reference by removing buffer tail handling
 *             since buflen % 3 = 0 always holds true (MLD_STREAM128_BLOCKBYTES
 * = 168).
 *           - Modified rej_uniform interface to track offset directly.
 *           - Pass nonce packed in the extended seed array instead of a third
 *             argument.
 * */
MLD_INTERNAL_API
void mld_poly_uniform(mld_poly *a, const uint8_t seed[MLDSA_SEEDBYTES + 2])
{
  unsigned int ctr;
  unsigned int buflen = MLD_POLY_UNIFORM_NBLOCKS * MLD_STREAM128_BLOCKBYTES;
  MLD_ALIGN uint8_t buf[MLD_POLY_UNIFORM_NBLOCKS * MLD_STREAM128_BLOCKBYTES];
  mld_xof128_ctx state;

  mld_xof128_init(&state);
  mld_xof128_absorb_once(&state, seed, MLDSA_SEEDBYTES + 2);
  mld_xof128_squeezeblocks(buf, MLD_POLY_UNIFORM_NBLOCKS, &state);

  ctr = mld_rej_uniform(a->coeffs, MLDSA_N, 0, buf, buflen);
  buflen = MLD_STREAM128_BLOCKBYTES;
  while (ctr < MLDSA_N)
  __loop__(
    assigns(ctr, state, memory_slice(a, sizeof(mld_poly)), object_whole(buf))
    invariant(ctr <= MLDSA_N)
    invariant(array_bound(a->coeffs, 0, ctr, 0, MLDSA_Q))
    invariant(state.pos <= SHAKE128_RATE)
  )
  {
    mld_xof128_squeezeblocks(buf, 1, &state);
    ctr = mld_rej_uniform(a->coeffs, MLDSA_N, ctr, buf, buflen);
  }
  mld_xof128_release(&state);
  mld_assert_bound(a->coeffs, MLDSA_N, 0, MLDSA_Q);

  /* @[FIPS204, Section 3.6.3] Destruction of intermediate values. */
  mld_zeroize(buf, sizeof(buf));
}

#if !defined(MLD_CONFIG_SERIAL_FIPS202_ONLY) && \
    (!defined(MLD_CONFIG_REDUCE_RAM) || defined(MLD_UNIT_TEST))
MLD_INTERNAL_API
void mld_poly_uniform_4x(mld_poly *vec0, mld_poly *vec1, mld_poly *vec2,
                         mld_poly *vec3,
                         uint8_t seed[4][MLD_ALIGN_UP(MLDSA_SEEDBYTES + 2)])
{
  /* Temporary buffers for XOF output before rejection sampling */
  MLD_ALIGN uint8_t
      buf[4][MLD_ALIGN_UP(MLD_POLY_UNIFORM_NBLOCKS * MLD_STREAM128_BLOCKBYTES)];

  /* Tracks the number of coefficients we have already sampled */
  unsigned ctr[4];
  mld_xof128_x4_ctx state;
  unsigned buflen;

  mld_xof128_x4_init(&state);
  mld_xof128_x4_absorb(&state, seed, MLDSA_SEEDBYTES + 2);

  /*
   * Initially, squeeze heuristic number of MLD_POLY_UNIFORM_NBLOCKS.
   * This should generate the matrix entries with high probability.
   */

  mld_xof128_x4_squeezeblocks(buf, MLD_POLY_UNIFORM_NBLOCKS, &state);
  buflen = MLD_POLY_UNIFORM_NBLOCKS * MLD_STREAM128_BLOCKBYTES;
  ctr[0] = mld_rej_uniform(vec0->coeffs, MLDSA_N, 0, buf[0], buflen);
  ctr[1] = mld_rej_uniform(vec1->coeffs, MLDSA_N, 0, buf[1], buflen);
  ctr[2] = mld_rej_uniform(vec2->coeffs, MLDSA_N, 0, buf[2], buflen);
  ctr[3] = mld_rej_uniform(vec3->coeffs, MLDSA_N, 0, buf[3], buflen);

  /*
   * So long as not all matrix entries have been generated, squeeze
   * one more block a time until we're done.
   */
  buflen = MLD_STREAM128_BLOCKBYTES;
  while (ctr[0] < MLDSA_N || ctr[1] < MLDSA_N || ctr[2] < MLDSA_N ||
         ctr[3] < MLDSA_N)
  __loop__(
    assigns(ctr, state, object_whole(buf),
            memory_slice(vec0, sizeof(mld_poly)), memory_slice(vec1, sizeof(mld_poly)),
            memory_slice(vec2, sizeof(mld_poly)), memory_slice(vec3, sizeof(mld_poly)))
    invariant(ctr[0] <= MLDSA_N && ctr[1] <= MLDSA_N)
    invariant(ctr[2] <= MLDSA_N && ctr[3] <= MLDSA_N)
    invariant(array_bound(vec0->coeffs, 0, ctr[0], 0, MLDSA_Q))
    invariant(array_bound(vec1->coeffs, 0, ctr[1], 0, MLDSA_Q))
    invariant(array_bound(vec2->coeffs, 0, ctr[2], 0, MLDSA_Q))
    invariant(array_bound(vec3->coeffs, 0, ctr[3], 0, MLDSA_Q)))
  {
    mld_xof128_x4_squeezeblocks(buf, 1, &state);
    ctr[0] = mld_rej_uniform(vec0->coeffs, MLDSA_N, ctr[0], buf[0], buflen);
    ctr[1] = mld_rej_uniform(vec1->coeffs, MLDSA_N, ctr[1], buf[1], buflen);
    ctr[2] = mld_rej_uniform(vec2->coeffs, MLDSA_N, ctr[2], buf[2], buflen);
    ctr[3] = mld_rej_uniform(vec3->coeffs, MLDSA_N, ctr[3], buf[3], buflen);
  }
  mld_xof128_x4_release(&state);

  mld_assert_bound(vec0->coeffs, MLDSA_N, 0, MLDSA_Q);
  mld_assert_bound(vec1->coeffs, MLDSA_N, 0, MLDSA_Q);
  mld_assert_bound(vec2->coeffs, MLDSA_N, 0, MLDSA_Q);
  mld_assert_bound(vec3->coeffs, MLDSA_N, 0, MLDSA_Q);

  /* @[FIPS204, Section 3.6.3] Destruction of intermediate values. */
  mld_zeroize(buf, sizeof(buf));
}

#endif /* !MLD_CONFIG_SERIAL_FIPS202_ONLY && (!MLD_CONFIG_REDUCE_RAM || \
          MLD_UNIT_TEST) */

#if !defined(MLD_CONFIG_NO_KEYPAIR_API)
MLD_INTERNAL_API
void mld_polyt1_pack(uint8_t r[MLDSA_POLYT1_PACKEDBYTES], const mld_poly *a)
{
  unsigned int i;
  mld_assert_bound(a->coeffs, MLDSA_N, 0, 1 << 10);

  for (i = 0; i < MLDSA_N / 4; ++i)
  __loop__(
    invariant(i <= MLDSA_N/4)
    decreases(MLDSA_N / 4 - i))
  {
    r[5 * i + 0] = (uint8_t)((a->coeffs[4 * i + 0] >> 0) & 0xFF);
    r[5 * i + 1] =
        (uint8_t)(((a->coeffs[4 * i + 0] >> 8) | (a->coeffs[4 * i + 1] << 2)) &
                  0xFF);
    r[5 * i + 2] =
        (uint8_t)(((a->coeffs[4 * i + 1] >> 6) | (a->coeffs[4 * i + 2] << 4)) &
                  0xFF);
    r[5 * i + 3] =
        (uint8_t)(((a->coeffs[4 * i + 2] >> 4) | (a->coeffs[4 * i + 3] << 6)) &
                  0xFF);
    r[5 * i + 4] = (uint8_t)((a->coeffs[4 * i + 3] >> 2) & 0xFF);
  }
}
#endif /* !MLD_CONFIG_NO_KEYPAIR_API */

#if !defined(MLD_CONFIG_NO_VERIFY_API)
MLD_INTERNAL_API
void mld_polyt1_unpack(mld_poly *r, const uint8_t a[MLDSA_POLYT1_PACKEDBYTES])
{
  unsigned int i;

  for (i = 0; i < MLDSA_N / 4; ++i)
  __loop__(
    invariant(i <= MLDSA_N/4)
    invariant(array_bound(r->coeffs, 0, i*4, 0, 1 << 10))
    decreases(MLDSA_N / 4 - i))
  {
    r->coeffs[4 * i + 0] =
        ((a[5 * i + 0] >> 0) | ((int32_t)a[5 * i + 1] << 8)) & 0x3FF;
    r->coeffs[4 * i + 1] =
        ((a[5 * i + 1] >> 2) | ((int32_t)a[5 * i + 2] << 6)) & 0x3FF;
    r->coeffs[4 * i + 2] =
        ((a[5 * i + 2] >> 4) | ((int32_t)a[5 * i + 3] << 4)) & 0x3FF;
    r->coeffs[4 * i + 3] =
        ((a[5 * i + 3] >> 6) | ((int32_t)a[5 * i + 4] << 2)) & 0x3FF;
  }

  mld_assert_bound(r->coeffs, MLDSA_N, 0, 1 << 10);
}
#endif /* !MLD_CONFIG_NO_VERIFY_API */

#if !defined(MLD_CONFIG_NO_KEYPAIR_API)
MLD_INTERNAL_API
void mld_polyt0_pack(uint8_t r[MLDSA_POLYT0_PACKEDBYTES], const mld_poly *a)
{
  unsigned int i;
  uint32_t t[8];

  mld_assert_bound(a->coeffs, MLDSA_N, -(1 << (MLDSA_D - 1)) + 1,
                   (1 << (MLDSA_D - 1)) + 1);

  for (i = 0; i < MLDSA_N / 8; ++i)
  __loop__(
    invariant(i <= MLDSA_N/8)
    decreases(MLDSA_N / 8 - i))
  {
    /* Safety: a->coeffs[i] <= (1 << (MLDSA_D - 1) as they are output of
     * power2round, hence, these casts are safe. */
    t[0] = (uint32_t)((1 << (MLDSA_D - 1)) - a->coeffs[8 * i + 0]);
    t[1] = (uint32_t)((1 << (MLDSA_D - 1)) - a->coeffs[8 * i + 1]);
    t[2] = (uint32_t)((1 << (MLDSA_D - 1)) - a->coeffs[8 * i + 2]);
    t[3] = (uint32_t)((1 << (MLDSA_D - 1)) - a->coeffs[8 * i + 3]);
    t[4] = (uint32_t)((1 << (MLDSA_D - 1)) - a->coeffs[8 * i + 4]);
    t[5] = (uint32_t)((1 << (MLDSA_D - 1)) - a->coeffs[8 * i + 5]);
    t[6] = (uint32_t)((1 << (MLDSA_D - 1)) - a->coeffs[8 * i + 6]);
    t[7] = (uint32_t)((1 << (MLDSA_D - 1)) - a->coeffs[8 * i + 7]);

    r[13 * i + 0] = (uint8_t)((t[0]) & 0xFF);
    r[13 * i + 1] = (uint8_t)((t[0] >> 8) & 0xFF);
    r[13 * i + 1] |= (uint8_t)((t[1] << 5) & 0xFF);
    r[13 * i + 2] = (uint8_t)((t[1] >> 3) & 0xFF);
    r[13 * i + 3] = (uint8_t)((t[1] >> 11) & 0xFF);
    r[13 * i + 3] |= (uint8_t)((t[2] << 2) & 0xFF);
    r[13 * i + 4] = (uint8_t)((t[2] >> 6) & 0xFF);
    r[13 * i + 4] |= (uint8_t)((t[3] << 7) & 0xFF);
    r[13 * i + 5] = (uint8_t)((t[3] >> 1) & 0xFF);
    r[13 * i + 6] = (uint8_t)((t[3] >> 9) & 0xFF);
    r[13 * i + 6] |= (uint8_t)((t[4] << 4) & 0xFF);
    r[13 * i + 7] = (uint8_t)((t[4] >> 4) & 0xFF);
    r[13 * i + 8] = (uint8_t)((t[4] >> 12) & 0xFF);
    r[13 * i + 8] |= (uint8_t)((t[5] << 1) & 0xFF);
    r[13 * i + 9] = (uint8_t)((t[5] >> 7) & 0xFF);
    r[13 * i + 9] |= (uint8_t)((t[6] << 6) & 0xFF);
    r[13 * i + 10] = (uint8_t)((t[6] >> 2) & 0xFF);
    r[13 * i + 11] = (uint8_t)((t[6] >> 10) & 0xFF);
    r[13 * i + 11] |= (uint8_t)((t[7] << 3) & 0xFF);
    r[13 * i + 12] = (uint8_t)((t[7] >> 5) & 0xFF);
  }
}
#endif /* !MLD_CONFIG_NO_KEYPAIR_API */

#if !defined(MLD_CONFIG_NO_SIGN_API) || defined(MLD_UNIT_TEST)
MLD_INTERNAL_API
void mld_polyt0_unpack(mld_poly *r, const uint8_t a[MLDSA_POLYT0_PACKEDBYTES])
{
  unsigned int i;

  for (i = 0; i < MLDSA_N / 8; ++i)
  __loop__(
    invariant(i <= MLDSA_N/8)
    invariant(array_bound(r->coeffs, 0, i*8, -(1<<(MLDSA_D-1)) + 1, (1<<(MLDSA_D-1)) + 1))
    decreases(MLDSA_N / 8 - i))
  {
    r->coeffs[8 * i + 0] = a[13 * i + 0];
    r->coeffs[8 * i + 0] |= (int32_t)a[13 * i + 1] << 8;
    r->coeffs[8 * i + 0] &= 0x1FFF;

    r->coeffs[8 * i + 1] = a[13 * i + 1] >> 5;
    r->coeffs[8 * i + 1] |= (int32_t)a[13 * i + 2] << 3;
    r->coeffs[8 * i + 1] |= (int32_t)a[13 * i + 3] << 11;
    r->coeffs[8 * i + 1] &= 0x1FFF;

    r->coeffs[8 * i + 2] = a[13 * i + 3] >> 2;
    r->coeffs[8 * i + 2] |= (int32_t)a[13 * i + 4] << 6;
    r->coeffs[8 * i + 2] &= 0x1FFF;

    r->coeffs[8 * i + 3] = a[13 * i + 4] >> 7;
    r->coeffs[8 * i + 3] |= (int32_t)a[13 * i + 5] << 1;
    r->coeffs[8 * i + 3] |= (int32_t)a[13 * i + 6] << 9;
    r->coeffs[8 * i + 3] &= 0x1FFF;

    r->coeffs[8 * i + 4] = a[13 * i + 6] >> 4;
    r->coeffs[8 * i + 4] |= (int32_t)a[13 * i + 7] << 4;
    r->coeffs[8 * i + 4] |= (int32_t)a[13 * i + 8] << 12;
    r->coeffs[8 * i + 4] &= 0x1FFF;

    r->coeffs[8 * i + 5] = a[13 * i + 8] >> 1;
    r->coeffs[8 * i + 5] |= (int32_t)a[13 * i + 9] << 7;
    r->coeffs[8 * i + 5] &= 0x1FFF;

    r->coeffs[8 * i + 6] = a[13 * i + 9] >> 6;
    r->coeffs[8 * i + 6] |= (int32_t)a[13 * i + 10] << 2;
    r->coeffs[8 * i + 6] |= (int32_t)a[13 * i + 11] << 10;
    r->coeffs[8 * i + 6] &= 0x1FFF;

    r->coeffs[8 * i + 7] = a[13 * i + 11] >> 3;
    r->coeffs[8 * i + 7] |= (int32_t)a[13 * i + 12] << 5;
    r->coeffs[8 * i + 7] &= 0x1FFF;

    r->coeffs[8 * i + 0] = (1 << (MLDSA_D - 1)) - r->coeffs[8 * i + 0];
    r->coeffs[8 * i + 1] = (1 << (MLDSA_D - 1)) - r->coeffs[8 * i + 1];
    r->coeffs[8 * i + 2] = (1 << (MLDSA_D - 1)) - r->coeffs[8 * i + 2];
    r->coeffs[8 * i + 3] = (1 << (MLDSA_D - 1)) - r->coeffs[8 * i + 3];
    r->coeffs[8 * i + 4] = (1 << (MLDSA_D - 1)) - r->coeffs[8 * i + 4];
    r->coeffs[8 * i + 5] = (1 << (MLDSA_D - 1)) - r->coeffs[8 * i + 5];
    r->coeffs[8 * i + 6] = (1 << (MLDSA_D - 1)) - r->coeffs[8 * i + 6];
    r->coeffs[8 * i + 7] = (1 << (MLDSA_D - 1)) - r->coeffs[8 * i + 7];
  }

  mld_assert_bound(r->coeffs, MLDSA_N, -(1 << (MLDSA_D - 1)) + 1,
                   (1 << (MLDSA_D - 1)) + 1);
}
#endif /* !MLD_CONFIG_NO_SIGN_API || MLD_UNIT_TEST */

MLD_STATIC_TESTABLE uint32_t mld_poly_chknorm_c(const mld_poly *a, int32_t B)
__contract__(
  requires(memory_no_alias(a, sizeof(mld_poly)))
  requires(0 <= B && B <= MLDSA_Q - MLD_REDUCE32_RANGE_MAX)
  requires(array_bound(a->coeffs, 0, MLDSA_N, -MLD_REDUCE32_RANGE_MAX, MLD_REDUCE32_RANGE_MAX))
  ensures(return_value == 0 || return_value == 0xFFFFFFFF)
  ensures((return_value == 0) == array_abs_bound(a->coeffs, 0, MLDSA_N, B))
)
{
  unsigned int i;
  uint32_t t = 0;
  mld_assert_bound(a->coeffs, MLDSA_N, -MLD_REDUCE32_RANGE_MAX,
                   MLD_REDUCE32_RANGE_MAX);
  for (i = 0; i < MLDSA_N; ++i)
  __loop__(
    invariant(i <= MLDSA_N)
    invariant(t == 0 || t == 0xFFFFFFFF)
    invariant((t == 0) == array_abs_bound(a->coeffs, 0, i, B))
    decreases(MLDSA_N - i)
  )
  {
    /*
     * Since we know that -MLD_REDUCE32_RANGE_MAX <= a < MLD_REDUCE32_RANGE_MAX,
     * and B <= MLDSA_Q - MLD_REDUCE32_RANGE_MAX, to check if
     * -B < (a mod± MLDSA_Q) < B, it suffices to check if -B < a < B.
     *
     * We prove this to be true using the following CBMC assertions.
     * a ==> b expressed as !a || b to also allow run-time assertion.
     */
    mld_assert(a->coeffs[i] < B || a->coeffs[i] - MLDSA_Q <= -B);
    mld_assert(a->coeffs[i] > -B || a->coeffs[i] + MLDSA_Q >= B);

    /* Reference: Leaks which coefficient violates the bound via a conditional.
     * We are more conservative to reduce the number of declassifications in
     * constant-time testing.
     */

    /* if (abs(a[i]) >= B) */
    t |= mld_ct_cmask_neg_i32(B - 1 - mld_ct_abs_i32(a->coeffs[i]));
  }

  return t;
}

/* Reference: explicitly checks the bound B to be <= (MLDSA_Q - 1) / 8).
 * This is unnecessary as it's always a compile-time constant.
 * We instead model it as a precondition.
 * Checking the bound is performed using a conditional arguing
 * that it is okay to leak which coefficient violates the bound (while the
 * coefficient itself must remain secret).
 * We instead perform everything in constant-time.
 * Also it is sufficient to check that it is smaller than
 * MLDSA_Q - MLD_REDUCE32_RANGE_MAX > (MLDSA_Q - 1) / 8).
 */
MLD_INTERNAL_API
uint32_t mld_poly_chknorm(const mld_poly *a, int32_t B)
{
#if defined(MLD_USE_NATIVE_POLY_CHKNORM)
  int ret;
  int success;
  mld_assert_bound(a->coeffs, MLDSA_N, -MLD_REDUCE32_RANGE_MAX,
                   MLD_REDUCE32_RANGE_MAX);
  /* The native backend returns 0 if all coefficients are within the bound,
   * 1 if at least one coefficient exceeds the bound, and
   * -1 (MLD_NATIVE_FUNC_FALLBACK) if the platform does not have the
   * required capabilities to run the native function.
   */
  ret = mld_poly_chknorm_native(a->coeffs, B);

  success = (ret != MLD_NATIVE_FUNC_FALLBACK);
  /* Constant-time: It would be fine to leak the return value of chknorm
   * entirely (as it is fine to leak if any coefficient exceeded the bound or
   * not). However, it is cleaner to perform declassification in sign.c.
   * Hence, here we only declassify if the native function returned
   * MLD_NATIVE_FUNC_FALLBACK or not (which solely depends on system
   * capabilities).
   */
  MLD_CT_TESTING_DECLASSIFY(&success, sizeof(int));
  if (success)
  {
    /* Convert 0 / 1 to 0 / 0xFFFFFFFF here */
    return mld_ct_cmask_nonzero_u32((uint32_t)ret);
  }
#endif /* MLD_USE_NATIVE_POLY_CHKNORM */
  return mld_poly_chknorm_c(a, B);
}

#if !defined(MLD_CONFIG_NO_SIGN_API) || !defined(MLD_CONFIG_NO_VERIFY_API)
#if defined(MLD_CONFIG_MULTILEVEL_WITH_SHARED) || \
    MLD_CONFIG_PARAMETER_SET == 44
MLD_INTERNAL_API
void mld_polyw1_pack_88(uint8_t r[MLDSA_POLYW1_PACKEDBYTES_88],
                        const mld_poly *a)
{
  unsigned int i;

  mld_assert_bound(a->coeffs, MLDSA_N, 0,
                   (MLDSA_Q - 1) / (2 * MLDSA_GAMMA2_88));

  for (i = 0; i < MLDSA_N / 4; ++i)
  __loop__(
    invariant(i <= MLDSA_N/4)
    decreases(MLDSA_N / 4 - i))
  {
    r[3 * i + 0] = (uint8_t)((a->coeffs[4 * i + 0]) & 0xFF);
    r[3 * i + 0] |= (uint8_t)((a->coeffs[4 * i + 1] << 6) & 0xFF);
    r[3 * i + 1] = (uint8_t)((a->coeffs[4 * i + 1] >> 2) & 0xFF);
    r[3 * i + 1] |= (uint8_t)((a->coeffs[4 * i + 2] << 4) & 0xFF);
    r[3 * i + 2] = (uint8_t)((a->coeffs[4 * i + 2] >> 4) & 0xFF);
    r[3 * i + 2] |= (uint8_t)((a->coeffs[4 * i + 3] << 2) & 0xFF);
  }
}
#endif /* MLD_CONFIG_MULTILEVEL_WITH_SHARED || MLD_CONFIG_PARAMETER_SET == 44 \
        */

#if defined(MLD_CONFIG_MULTILEVEL_WITH_SHARED) || \
    (MLD_CONFIG_PARAMETER_SET == 65 || MLD_CONFIG_PARAMETER_SET == 87)
MLD_INTERNAL_API
void mld_polyw1_pack_32(uint8_t r[MLDSA_POLYW1_PACKEDBYTES_32],
                        const mld_poly *a)
{
  unsigned int i;

  mld_assert_bound(a->coeffs, MLDSA_N, 0,
                   (MLDSA_Q - 1) / (2 * MLDSA_GAMMA2_32));

  for (i = 0; i < MLDSA_N / 2; ++i)
  __loop__(
    invariant(i <= MLDSA_N/2)
    decreases(MLDSA_N / 2 - i))
  {
    r[i] =
        (uint8_t)((a->coeffs[2 * i + 0] | (a->coeffs[2 * i + 1] << 4)) & 0xFF);
  }
}
#endif /* MLD_CONFIG_MULTILEVEL_WITH_SHARED || MLD_CONFIG_PARAMETER_SET == 65 \
          || MLD_CONFIG_PARAMETER_SET == 87 */
#endif /* !MLD_CONFIG_NO_SIGN_API || !MLD_CONFIG_NO_VERIFY_API */

#else  /* !MLD_CONFIG_MULTILEVEL_NO_SHARED */
MLD_EMPTY_CU(mld_poly)
#endif /* MLD_CONFIG_MULTILEVEL_NO_SHARED */

/* To facilitate single-compilation-unit (SCU) builds, undefine all macros.
 * Don't modify by hand -- this is auto-generated by scripts/autogen. */
#undef MLD_POLY_UNIFORM_NBLOCKS
