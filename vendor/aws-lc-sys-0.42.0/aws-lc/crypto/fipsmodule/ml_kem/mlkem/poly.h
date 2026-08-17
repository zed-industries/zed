/*
 * Copyright (c) The mlkem-native project authors
 * SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT
 */

/* References
 * ==========
 *
 * - [FIPS203]
 *   FIPS 203 Module-Lattice-Based Key-Encapsulation Mechanism Standard
 *   National Institute of Standards and Technology
 *   https://csrc.nist.gov/pubs/fips/203/final
 */

#ifndef MLK_POLY_H
#define MLK_POLY_H


#include "cbmc.h"
#include "common.h"
#include "debug.h"
#include "verify.h"

/* Absolute exclusive upper bound for the output of the inverse NTT */
#define MLK_INVNTT_BOUND (8 * MLKEM_Q)

/* Absolute exclusive upper bound for the output of the forward NTT */
#define MLK_NTT_BOUND (8 * MLKEM_Q)

/*
 * Elements of R_q = Z_q[X]/(X^n + 1). Represents polynomial
 * coeffs[0] + X*coeffs[1] + X^2*coeffs[2] + ... + X^{n-1}*coeffs[n-1]
 */
typedef struct
{
  int16_t coeffs[MLKEM_N];
} MLK_ALIGN mlk_poly;

/*
 * INTERNAL presentation of precomputed data speeding up
 * the base multiplication of two polynomials in NTT domain.
 */
typedef struct
{
  int16_t coeffs[MLKEM_N >> 1];
} MLK_ALIGN mlk_poly_mulcache;

/*************************************************
 * Name:        mlk_montgomery_reduce
 *
 * Description: Generic Montgomery reduction; given a 32-bit integer a, computes
 *              16-bit integer congruent to a * R^-1 mod q, where R=2^16
 *
 * Arguments:   - int32_t a: input integer to be reduced, of absolute value
 *                smaller or equal to INT32_MAX - 2^15 * MLKEM_Q.
 *
 * Returns:     integer congruent to a * R^-1 modulo q, with absolute value
 *                <= ceil(|a| / 2^16) + (MLKEM_Q + 1)/2
 *
 **************************************************/
static MLK_ALWAYS_INLINE int16_t mlk_montgomery_reduce(int32_t a)
__contract__(
    requires(a < +(INT32_MAX - (((int32_t)1 << 15) * MLKEM_Q)) &&
             a > -(INT32_MAX - (((int32_t)1 << 15) * MLKEM_Q)))
    /* We don't attempt to express an input-dependent output bound
     * as the post-condition here. There are two call-sites for this
     * function:
     * - The base multiplication: Here, we need no output bound.
     * - mlk_fqmul: Here, we inline this function and prove another spec
     *          for mlk_fqmul which does have a post-condition bound. */
)
{
  /* check-magic: 62209 == unsigned_mod(pow(MLKEM_Q, -1, 2^16), 2^16) */
  const uint32_t QINV = 62209;

  /* Compute a*q^{-1} mod 2^16 in unsigned representatives. */
  const uint16_t a_reduced = mlk_cast_int32_to_uint16(a);
  const uint16_t a_inverted = (a_reduced * QINV) & UINT16_MAX;

  /* Lift to signed canonical representative mod 2^16. */
  const int16_t t = mlk_cast_uint16_to_int16(a_inverted);

  int32_t r;

  mlk_assert(a < +(INT32_MAX - (((int32_t)1 << 15) * MLKEM_Q)) &&
             a > -(INT32_MAX - (((int32_t)1 << 15) * MLKEM_Q)));

  r = a - ((int32_t)t * MLKEM_Q);

  /*
   * PORTABILITY: Right-shift on a signed integer is, strictly-speaking,
   * implementation-defined for negative left argument. Here,
   * we assume it's sign-preserving "arithmetic" shift right. (C99 6.5.7 (5))
   */
  r = r >> 16;
  /* Bounds: |r >> 16| <= ceil(|r| / 2^16)
   *                   <= ceil(|a| / 2^16 + MLKEM_Q / 2)
   *                   <= ceil(|a| / 2^16) + (MLKEM_Q + 1) / 2
   *
   * (Note that |a >> n| = ceil(|a| / 2^16) for negative a)
   */
  return (int16_t)r;
}

#define mlk_poly_tomont MLK_NAMESPACE(poly_tomont)
/*************************************************
 * Name:        mlk_poly_tomont
 *
 * Description: Inplace conversion of all coefficients of a polynomial
 *              from normal domain to Montgomery domain
 *
 *              Bounds: Output < q in absolute value.
 *
 * Arguments:   - mlk_poly *r: pointer to input/output polynomial
 *
 * Specification: Internal normalization required in `mlk_indcpa_keypair_derand`
 *                as part of matrix-vector multiplication
 *                @[FIPS203, Algorithm 13, K-PKE.KeyGen, L18].
 *
 **************************************************/
MLK_INTERNAL_API
void mlk_poly_tomont(mlk_poly *r)
__contract__(
  requires(memory_no_alias(r, sizeof(mlk_poly)))
  assigns(memory_slice(r, sizeof(mlk_poly)))
  ensures(array_abs_bound(r->coeffs, 0, MLKEM_N, MLKEM_Q))
);

#define mlk_poly_mulcache_compute MLK_NAMESPACE(poly_mulcache_compute)
/************************************************************
 * Name: mlk_poly_mulcache_compute
 *
 * Description: Computes the mulcache for a polynomial in NTT domain
 *
 *              The mulcache of a degree-2 polynomial b := b0 + b1*X
 *              in Fq[X]/(X^2-zeta) is the value b1*zeta, needed when
 *              computing products of b in Fq[X]/(X^2-zeta).
 *
 *              The mulcache of a polynomial in NTT domain -- which is
 *              a 128-tuple of degree-2 polynomials in Fq[X]/(X^2-zeta),
 *              for varying zeta, is the 128-tuple of mulcaches of those
 *              polynomials.
 *
 * Arguments: - x: Pointer to mulcache to be populated
 *            - a: Pointer to input polynomial
 *
 * Specification:
 * - Caches `b_1 * \gamma` in @[FIPS203, Algorithm 12, BaseCaseMultiply, L1]
 *
 ************************************************************/
/*
 * NOTE: The default C implementation of this function populates
 * the mulcache with values in (-q,q), but this is not needed for the
 * higher level safety proofs, and thus not part of the spec.
 */
MLK_INTERNAL_API
void mlk_poly_mulcache_compute(mlk_poly_mulcache *x, const mlk_poly *a)
__contract__(
  requires(memory_no_alias(x, sizeof(mlk_poly_mulcache)))
  requires(memory_no_alias(a, sizeof(mlk_poly)))
  assigns(memory_slice(x, sizeof(mlk_poly_mulcache)))
);

#define mlk_poly_reduce MLK_NAMESPACE(poly_reduce)
/*************************************************
 * Name:        mlk_poly_reduce
 *
 * Description: Converts polynomial to _unsigned canonical_ representatives.
 *
 *              The input coefficients can be arbitrary integers in int16_t.
 *              The output coefficients are in [0,1,...,MLKEM_Q-1].
 *
 * Arguments:   - mlk_poly *r: pointer to input/output polynomial
 *
 * Specification: Normalizes on unsigned canoncial representatives
 *                ahead of calling @[FIPS203, Compress_d, Eq (4.7)].
 *                This is not made explicit in FIPS 203.
 *
 **************************************************/
/*
 * NOTE: The semantics of mlk_poly_reduce() is different in
 * the reference implementation, which requires
 * signed canonical output data. Unsigned canonical
 * outputs are better suited to the only remaining
 * use of mlk_poly_reduce() in the context of (de)serialization.
 */
MLK_INTERNAL_API
void mlk_poly_reduce(mlk_poly *r)
__contract__(
  requires(memory_no_alias(r, sizeof(mlk_poly)))
  assigns(memory_slice(r, sizeof(mlk_poly)))
  ensures(array_bound(r->coeffs, 0, MLKEM_N, 0, MLKEM_Q))
);

#define mlk_poly_add MLK_NAMESPACE(poly_add)
/************************************************************
 * Name: mlk_poly_add
 *
 * Description: Adds two polynomials in place
 *
 * Arguments: - r: Pointer to input-output polynomial to be added to.
 *            - b: Pointer to input polynomial that should be added
 *                 to r. Must be disjoint from r.
 *
 * The coefficients of r and b must be so that the addition does
 * not overflow. Otherwise, the behaviour of this function is undefined.
 *
 * Specification:
 * - @[FIPS203, 2.4.5, Arithmetic With Polynomials and NTT Representations]
 * - Used in @[FIPS203, Algorithm 14 (K-PKE.Encrypt), L21]
 *
 ************************************************************/
/*
 * NOTE: The reference implementation uses a 3-argument mlk_poly_add.
 * We specialize to the accumulator form to avoid reasoning about aliasing.
 */
MLK_INTERNAL_API
void mlk_poly_add(mlk_poly *r, const mlk_poly *b)
__contract__(
  requires(memory_no_alias(r, sizeof(mlk_poly)))
  requires(memory_no_alias(b, sizeof(mlk_poly)))
  requires(forall(k0, 0, MLKEM_N, (int32_t) r->coeffs[k0] + b->coeffs[k0] <= INT16_MAX))
  requires(forall(k1, 0, MLKEM_N, (int32_t) r->coeffs[k1] + b->coeffs[k1] >= INT16_MIN))
  ensures(forall(k, 0, MLKEM_N, r->coeffs[k] == old(*r).coeffs[k] + b->coeffs[k]))
  assigns(memory_slice(r, sizeof(mlk_poly)))
);

#define mlk_poly_sub MLK_NAMESPACE(poly_sub)
/*************************************************
 * Name:        mlk_poly_sub
 *
 * Description: Subtract two polynomials; no modular reduction is performed
 *
 * Arguments: - mlk_poly *r: Pointer to input-output polynomial to be added to.
 *            - const mlk_poly *b: Pointer to second input polynomial
 *
 * Specification:
 * - @[FIPS203, 2.4.5, Arithmetic With Polynomials and NTT Representations]
 * - Used in @[FIPS203, Algorithm 15, K-PKE.Decrypt, L6]
 *
 **************************************************/
/*
 * NOTE: The reference implementation uses a 3-argument mlk_poly_sub.
 * We specialize to the accumulator form to avoid reasoning about aliasing.
 */
MLK_INTERNAL_API
void mlk_poly_sub(mlk_poly *r, const mlk_poly *b)
__contract__(
  requires(memory_no_alias(r, sizeof(mlk_poly)))
  requires(memory_no_alias(b, sizeof(mlk_poly)))
  requires(forall(k0, 0, MLKEM_N, (int32_t) r->coeffs[k0] - b->coeffs[k0] <= INT16_MAX))
  requires(forall(k1, 0, MLKEM_N, (int32_t) r->coeffs[k1] - b->coeffs[k1] >= INT16_MIN))
  ensures(forall(k, 0, MLKEM_N, r->coeffs[k] == old(*r).coeffs[k] - b->coeffs[k]))
  assigns(memory_slice(r, sizeof(mlk_poly)))
);

#define mlk_poly_ntt MLK_NAMESPACE(poly_ntt)
/*************************************************
 * Name:        mlk_poly_ntt
 *
 * Description: Computes negacyclic number-theoretic transform (NTT) of
 *              a polynomial in place.
 *
 *              The input is assumed to be in normal order and
 *              coefficient-wise bound by MLKEM_Q in absolute value.
 *
 *              The output polynomial is in bitreversed order, and
 *              coefficient-wise bound by MLK_NTT_BOUND in absolute value.
 *
 *              (NOTE: Sometimes the input to the NTT is actually smaller,
 *               which gives better bounds.)
 *
 * Arguments:   - mlk_poly *p: pointer to in/output polynomial
 *
 * Specification: Implements @[FIPS203, Algorithm 9, NTT]
 *
 **************************************************/
MLK_INTERNAL_API
void mlk_poly_ntt(mlk_poly *r)
__contract__(
  requires(memory_no_alias(r, sizeof(mlk_poly)))
  requires(array_abs_bound(r->coeffs, 0, MLKEM_N, MLKEM_Q))
  assigns(memory_slice(r, sizeof(mlk_poly)))
  ensures(array_abs_bound(r->coeffs, 0, MLKEM_N, MLK_NTT_BOUND))
);

#define mlk_poly_invntt_tomont MLK_NAMESPACE(poly_invntt_tomont)
/*************************************************
 * Name:        mlk_poly_invntt_tomont
 *
 * Description: Computes inverse of negacyclic number-theoretic transform (NTT)
 *              of a polynomial in place;
 *              inputs assumed to be in bitreversed order, output in normal
 *              order
 *
 *              The input is assumed to be in bitreversed order, and can
 *              have arbitrary coefficients in int16_t.
 *
 *              The output polynomial is in normal order, and
 *              coefficient-wise bound by MLK_INVNTT_BOUND in absolute value.
 *
 * Arguments:   - uint16_t *a: pointer to in/output polynomial
 *
 * Specification: Implements composition of @[FIPS203, Algorithm 10, NTT^{-1}]
 *                and elementwise modular multiplication with a suitable
 *                Montgomery factor introduced during the base multiplication.
 *
 **************************************************/
MLK_INTERNAL_API
void mlk_poly_invntt_tomont(mlk_poly *r)
__contract__(
  requires(memory_no_alias(r, sizeof(mlk_poly)))
  assigns(memory_slice(r, sizeof(mlk_poly)))
  ensures(array_abs_bound(r->coeffs, 0, MLKEM_N, MLK_INVNTT_BOUND))
);

#endif /* !MLK_POLY_H */
