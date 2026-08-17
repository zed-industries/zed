/*
 * Copyright (c) The mldsa-native project authors
 * SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT
 */
#ifndef MLD_POLY_KL_H
#define MLD_POLY_KL_H

#include "cbmc.h"
#include "common.h"
#include "poly.h"

#if !defined(MLD_CONFIG_NO_SIGN_API)
#define mld_poly_decompose MLD_NAMESPACE_KL(poly_decompose)
/**
 * For all coefficients c of the input polynomial, compute high and low bits
 * c0, c1 such c mod MLDSA_Q = c1*ALPHA + c0 with -ALPHA/2 < c0 <= ALPHA/2
 * except c1 = (MLDSA_Q-1)/ALPHA where we set c1 = 0 and
 * -ALPHA/2 <= c0 = c mod MLDSA_Q - MLDSA_Q < 0. Assumes coefficients to be
 * standard representatives.
 *
 * @reference{The reference implementation has the input polynomial as a
 * separate argument that may be aliased with either of the outputs. Removing
 * the aliasing eases CBMC proofs.}
 *
 * @param[out]    a1 Pointer to output polynomial with coefficients c1.
 * @param[in,out] a0 Pointer to input/output polynomial. Output polynomial has
 *                   coefficients c0.
 */
MLD_INTERNAL_API
void mld_poly_decompose(mld_poly *a1, mld_poly *a0)
__contract__(
  requires(memory_no_alias(a1,  sizeof(mld_poly)))
  requires(memory_no_alias(a0, sizeof(mld_poly)))
  requires(array_bound(a0->coeffs, 0, MLDSA_N, 0, MLDSA_Q))
  assigns(memory_slice(a1, sizeof(mld_poly)))
  assigns(memory_slice(a0, sizeof(mld_poly)))
  ensures(array_bound(a1->coeffs, 0, MLDSA_N, 0, (MLDSA_Q-1)/(2*MLDSA_GAMMA2)))
  ensures(array_abs_bound(a0->coeffs, 0, MLDSA_N, MLDSA_GAMMA2+1))
);

#endif /* !MLD_CONFIG_NO_SIGN_API */

#if !defined(MLD_CONFIG_NO_VERIFY_API)
#define mld_poly_use_hint MLD_NAMESPACE_KL(poly_use_hint)
/**
 * Use hint polynomial h to correct the high bits of a in-place.
 *
 * @param[in,out] a Input/output polynomial.
 * @param[in]     h Hint polynomial.
 */
MLD_INTERNAL_API
void mld_poly_use_hint(mld_poly *a, const mld_poly *h)
__contract__(
  requires(memory_no_alias(a, sizeof(mld_poly)))
  requires(memory_no_alias(h, sizeof(mld_poly)))
  requires(array_bound(a->coeffs, 0, MLDSA_N, 0, MLDSA_Q))
  requires(array_bound(h->coeffs, 0, MLDSA_N, 0, 2))
  assigns(memory_slice(a, sizeof(mld_poly)))
  ensures(array_bound(a->coeffs, 0, MLDSA_N, 0, (MLDSA_Q-1)/(2*MLDSA_GAMMA2)))
);
#endif /* !MLD_CONFIG_NO_VERIFY_API */

#if !defined(MLD_CONFIG_NO_KEYPAIR_API)
#if !defined(MLD_CONFIG_SERIAL_FIPS202_ONLY)
#define mld_poly_uniform_eta_4x MLD_NAMESPACE_KL(poly_uniform_eta_4x)
/**
 * Sample four polynomials with uniformly random coefficients in
 * [-MLDSA_ETA, MLDSA_ETA] by performing rejection sampling on the output
 * stream from SHAKE256(seed|nonce_i).
 *
 * @param[out] r0     Pointer to first output polynomial.
 * @param[out] r1     Pointer to second output polynomial.
 * @param[out] r2     Pointer to third output polynomial.
 * @param[out] r3     Pointer to fourth output polynomial.
 * @param[in]  seed   Byte array with seed of length MLDSA_CRHBYTES.
 * @param      nonce0 First nonce.
 * @param      nonce1 Second nonce.
 * @param      nonce2 Third nonce.
 * @param      nonce3 Fourth nonce.
 */
MLD_INTERNAL_API
void mld_poly_uniform_eta_4x(mld_poly *r0, mld_poly *r1, mld_poly *r2,
                             mld_poly *r3, const uint8_t seed[MLDSA_CRHBYTES],
                             uint8_t nonce0, uint8_t nonce1, uint8_t nonce2,
                             uint8_t nonce3)
__contract__(
  requires(memory_no_alias(r0, sizeof(mld_poly)))
  requires(memory_no_alias(r1, sizeof(mld_poly)))
  requires(memory_no_alias(r2, sizeof(mld_poly)))
  requires(memory_no_alias(r3, sizeof(mld_poly)))
  requires(memory_no_alias(seed, MLDSA_CRHBYTES))
  assigns(memory_slice(r0, sizeof(mld_poly)))
  assigns(memory_slice(r1, sizeof(mld_poly)))
  assigns(memory_slice(r2, sizeof(mld_poly)))
  assigns(memory_slice(r3, sizeof(mld_poly)))
  ensures(array_abs_bound(r0->coeffs, 0, MLDSA_N, MLDSA_ETA + 1))
  ensures(array_abs_bound(r1->coeffs, 0, MLDSA_N, MLDSA_ETA + 1))
  ensures(array_abs_bound(r2->coeffs, 0, MLDSA_N, MLDSA_ETA + 1))
  ensures(array_abs_bound(r3->coeffs, 0, MLDSA_N, MLDSA_ETA + 1))
);
#endif /* !MLD_CONFIG_SERIAL_FIPS202_ONLY */

#if defined(MLD_CONFIG_SERIAL_FIPS202_ONLY)
#define mld_poly_uniform_eta MLD_NAMESPACE_KL(poly_uniform_eta)
/**
 * Sample polynomial with uniformly random coefficients in
 * [-MLDSA_ETA, MLDSA_ETA] by performing rejection sampling on the output
 * stream from SHAKE256(seed|nonce).
 *
 * @param[out] r     Pointer to output polynomial.
 * @param[in]  seed  Byte array with seed of length MLDSA_CRHBYTES.
 * @param      nonce Nonce.
 */
MLD_INTERNAL_API
void mld_poly_uniform_eta(mld_poly *r, const uint8_t seed[MLDSA_CRHBYTES],
                          uint8_t nonce)
__contract__(
  requires(memory_no_alias(r, sizeof(mld_poly)))
  requires(memory_no_alias(seed, MLDSA_CRHBYTES))
  assigns(memory_slice(r, sizeof(mld_poly)))
  ensures(array_abs_bound(r->coeffs, 0, MLDSA_N, MLDSA_ETA + 1))
);
#endif /* MLD_CONFIG_SERIAL_FIPS202_ONLY */
#endif /* !MLD_CONFIG_NO_KEYPAIR_API */

#if !defined(MLD_CONFIG_NO_SIGN_API)
#if MLD_CONFIG_PARAMETER_SET == 65 ||          \
    defined(MLD_CONFIG_SERIAL_FIPS202_ONLY) || \
    defined(MLD_CONFIG_REDUCE_RAM) || defined(MLD_UNIT_TEST)
#define mld_poly_uniform_gamma1 MLD_NAMESPACE_KL(poly_uniform_gamma1)
/**
 * Sample polynomial with uniformly random coefficients in
 * [-(MLDSA_GAMMA1 - 1), MLDSA_GAMMA1] by unpacking output stream of
 * SHAKE256(seed|nonce).
 *
 * @param[out] a     Pointer to output polynomial.
 * @param[in]  seed  Byte array with seed of length MLDSA_CRHBYTES.
 * @param      nonce 16-bit nonce.
 */
MLD_INTERNAL_API
void mld_poly_uniform_gamma1(mld_poly *a, const uint8_t seed[MLDSA_CRHBYTES],
                             uint16_t nonce)
__contract__(
  requires(memory_no_alias(a, sizeof(mld_poly)))
  requires(memory_no_alias(seed, MLDSA_CRHBYTES))
  assigns(memory_slice(a, sizeof(mld_poly)))
  ensures(array_bound(a->coeffs, 0, MLDSA_N, -(MLDSA_GAMMA1 - 1), MLDSA_GAMMA1 + 1))
);
#endif /* MLD_CONFIG_PARAMETER_SET == 65 || MLD_CONFIG_SERIAL_FIPS202_ONLY || \
          MLD_CONFIG_REDUCE_RAM || MLD_UNIT_TEST */

#if !defined(MLD_CONFIG_SERIAL_FIPS202_ONLY) && \
    (!defined(MLD_CONFIG_REDUCE_RAM) || defined(MLD_UNIT_TEST))
#define mld_poly_uniform_gamma1_4x MLD_NAMESPACE_KL(poly_uniform_gamma1_4x)
/**
 * Sample four polynomials with uniformly random coefficients in
 * [-(MLDSA_GAMMA1 - 1), MLDSA_GAMMA1] by unpacking output streams of
 * SHAKE256(seed|nonce_i).
 *
 * @param[out] r0     Pointer to first output polynomial.
 * @param[out] r1     Pointer to second output polynomial.
 * @param[out] r2     Pointer to third output polynomial.
 * @param[out] r3     Pointer to fourth output polynomial.
 * @param[in]  seed   Byte array with seed of length MLDSA_CRHBYTES.
 * @param      nonce0 First 16-bit nonce.
 * @param      nonce1 Second 16-bit nonce.
 * @param      nonce2 Third 16-bit nonce.
 * @param      nonce3 Fourth 16-bit nonce.
 */
MLD_INTERNAL_API
void mld_poly_uniform_gamma1_4x(mld_poly *r0, mld_poly *r1, mld_poly *r2,
                                mld_poly *r3,
                                const uint8_t seed[MLDSA_CRHBYTES],
                                uint16_t nonce0, uint16_t nonce1,
                                uint16_t nonce2, uint16_t nonce3)
__contract__(
  requires(memory_no_alias(r0, sizeof(mld_poly)))
  requires(memory_no_alias(r1, sizeof(mld_poly)))
  requires(memory_no_alias(r2, sizeof(mld_poly)))
  requires(memory_no_alias(r3, sizeof(mld_poly)))
  requires(memory_no_alias(seed, MLDSA_CRHBYTES))
  assigns(memory_slice(r0, sizeof(mld_poly)))
  assigns(memory_slice(r1, sizeof(mld_poly)))
  assigns(memory_slice(r2, sizeof(mld_poly)))
  assigns(memory_slice(r3, sizeof(mld_poly)))
  ensures(array_bound(r0->coeffs, 0, MLDSA_N, -(MLDSA_GAMMA1 - 1), MLDSA_GAMMA1 + 1))
  ensures(array_bound(r1->coeffs, 0, MLDSA_N, -(MLDSA_GAMMA1 - 1), MLDSA_GAMMA1 + 1))
  ensures(array_bound(r2->coeffs, 0, MLDSA_N, -(MLDSA_GAMMA1 - 1), MLDSA_GAMMA1 + 1))
  ensures(array_bound(r3->coeffs, 0, MLDSA_N, -(MLDSA_GAMMA1 - 1), MLDSA_GAMMA1 + 1))
);
#endif /* !MLD_CONFIG_SERIAL_FIPS202_ONLY && (!MLD_CONFIG_REDUCE_RAM || \
          MLD_UNIT_TEST) */
#endif /* !MLD_CONFIG_NO_SIGN_API */

#if !defined(MLD_CONFIG_NO_SIGN_API) || !defined(MLD_CONFIG_NO_VERIFY_API)
#define mld_poly_challenge MLD_NAMESPACE_KL(poly_challenge)
/**
 * Implementation of H. Samples polynomial with MLDSA_TAU nonzero coefficients
 * in {-1, 1} using the output stream of SHAKE256(seed).
 *
 * @param[out] c    Pointer to output polynomial.
 * @param[in]  seed Byte array containing seed of length MLDSA_CTILDEBYTES.
 */
MLD_INTERNAL_API
void mld_poly_challenge(mld_poly *c, const uint8_t seed[MLDSA_CTILDEBYTES])
__contract__(
  requires(memory_no_alias(c, sizeof(mld_poly)))
  requires(memory_no_alias(seed, MLDSA_CTILDEBYTES))
  assigns(memory_slice(c, sizeof(mld_poly)))
  /* All coefficients of c are -1, 0 or +1 */
  ensures(array_bound(c->coeffs, 0, MLDSA_N, -1, 2))
);
#endif /* !MLD_CONFIG_NO_SIGN_API || !MLD_CONFIG_NO_VERIFY_API */

#if !defined(MLD_CONFIG_NO_KEYPAIR_API)
#define mld_polyeta_pack MLD_NAMESPACE_KL(polyeta_pack)
/**
 * Bit-pack polynomial with coefficients in [-MLDSA_ETA, MLDSA_ETA].
 *
 * @param[out] r Pointer to output byte array with at least
 *               MLDSA_POLYETA_PACKEDBYTES bytes.
 * @param[in]  a Pointer to input polynomial.
 */
MLD_INTERNAL_API
void mld_polyeta_pack(uint8_t r[MLDSA_POLYETA_PACKEDBYTES], const mld_poly *a)
__contract__(
  requires(memory_no_alias(r, MLDSA_POLYETA_PACKEDBYTES))
  requires(memory_no_alias(a, sizeof(mld_poly)))
  requires(array_abs_bound(a->coeffs, 0, MLDSA_N, MLDSA_ETA + 1))
  assigns(memory_slice(r, MLDSA_POLYETA_PACKEDBYTES))
);
#endif /* !MLD_CONFIG_NO_KEYPAIR_API */

#if !defined(MLD_CONFIG_NO_KEYPAIR_API) || !defined(MLD_CONFIG_NO_SIGN_API)
/*
 * polyeta_unpack produces coefficients in [-MLDSA_ETA, MLDSA_ETA] for
 * well-formed inputs (i.e., those produced by polyeta_pack).
 * However, when passed an arbitrary byte array, it may produce smaller values,
 * i.e., values in [MLD_POLYETA_UNPACK_LOWER_BOUND, MLDSA_ETA].
 * Even though this should never happen, we use use the bound for arbitrary
 * inputs in the CBMC proofs.
 */
#if MLDSA_ETA == 2
#define MLD_POLYETA_UNPACK_LOWER_BOUND (-5)
#elif MLDSA_ETA == 4
#define MLD_POLYETA_UNPACK_LOWER_BOUND (-11)
#else
#error "Invalid value of MLDSA_ETA"
#endif

#define mld_polyeta_unpack MLD_NAMESPACE_KL(polyeta_unpack)
/**
 * Unpack polynomial with coefficients in [-MLDSA_ETA, MLDSA_ETA].
 *
 * @param[out] r Pointer to output polynomial.
 * @param[in]  a Byte array with bit-packed polynomial.
 */
MLD_INTERNAL_API
void mld_polyeta_unpack(mld_poly *r, const uint8_t a[MLDSA_POLYETA_PACKEDBYTES])
__contract__(
  requires(memory_no_alias(r, sizeof(mld_poly)))
  requires(memory_no_alias(a, MLDSA_POLYETA_PACKEDBYTES))
  assigns(memory_slice(r, sizeof(mld_poly)))
  ensures(array_bound(r->coeffs, 0, MLDSA_N, MLD_POLYETA_UNPACK_LOWER_BOUND, MLDSA_ETA + 1))
);
#endif /* !MLD_CONFIG_NO_KEYPAIR_API || !MLD_CONFIG_NO_SIGN_API */

#if !defined(MLD_CONFIG_NO_SIGN_API)
#define mld_polyz_pack MLD_NAMESPACE_KL(polyz_pack)
/**
 * Bit-pack polynomial with coefficients in
 * [-(MLDSA_GAMMA1 - 1), MLDSA_GAMMA1].
 *
 * @param[out] r Pointer to output byte array with at least
 *               MLDSA_POLYZ_PACKEDBYTES bytes.
 * @param[in]  a Pointer to input polynomial.
 */
MLD_INTERNAL_API
void mld_polyz_pack(uint8_t r[MLDSA_POLYZ_PACKEDBYTES], const mld_poly *a)
__contract__(
  requires(memory_no_alias(r, MLDSA_POLYZ_PACKEDBYTES))
  requires(memory_no_alias(a, sizeof(mld_poly)))
  requires(array_bound(a->coeffs, 0, MLDSA_N, -(MLDSA_GAMMA1 - 1), MLDSA_GAMMA1 + 1))
  assigns(memory_slice(r, MLDSA_POLYZ_PACKEDBYTES))
);
#endif /* !MLD_CONFIG_NO_SIGN_API */

#if !defined(MLD_CONFIG_NO_SIGN_API) || !defined(MLD_CONFIG_NO_VERIFY_API)
#define mld_polyz_unpack MLD_NAMESPACE_KL(polyz_unpack)
/**
 * Unpack polynomial z with coefficients in
 * [-(MLDSA_GAMMA1 - 1), MLDSA_GAMMA1].
 *
 * @param[out] r Pointer to output polynomial.
 * @param[in]  a Byte array with bit-packed polynomial.
 */
MLD_INTERNAL_API
void mld_polyz_unpack(mld_poly *r, const uint8_t a[MLDSA_POLYZ_PACKEDBYTES])
__contract__(
  requires(memory_no_alias(r, sizeof(mld_poly)))
  requires(memory_no_alias(a, MLDSA_POLYZ_PACKEDBYTES))
  assigns(memory_slice(r, sizeof(mld_poly)))
  ensures(array_bound(r->coeffs, 0, MLDSA_N, -(MLDSA_GAMMA1 - 1), MLDSA_GAMMA1 + 1))
);

#define mld_polyw1_pack MLD_NAMESPACE_KL(polyw1_pack)
/**
 * Bit-pack polynomial w1. Input coefficients must be in
 * [0, (MLDSA_Q-1)/(2*MLDSA_GAMMA2)), i.e. [0, 43] for ML-DSA-44 and [0, 15]
 * for ML-DSA-65/87. Dispatches to the value-specialized variant for the
 * selected parameter set.
 *
 * @param[out] r Pointer to output byte array with at least
 *               MLDSA_POLYW1_PACKEDBYTES bytes.
 * @param[in]  a Pointer to input polynomial.
 */
static MLD_INLINE void mld_polyw1_pack(uint8_t r[MLDSA_POLYW1_PACKEDBYTES],
                                       const mld_poly *a)
__contract__(
  requires(memory_no_alias(r, MLDSA_POLYW1_PACKEDBYTES))
  requires(memory_no_alias(a, sizeof(mld_poly)))
  requires(array_bound(a->coeffs, 0, MLDSA_N, 0, (MLDSA_Q-1)/(2*MLDSA_GAMMA2)))
  assigns(memory_slice(r, MLDSA_POLYW1_PACKEDBYTES))
)
{
#if MLD_CONFIG_PARAMETER_SET == 44
  mld_polyw1_pack_88(r, a);
#else
  mld_polyw1_pack_32(r, a);
#endif
}
#endif /* !MLD_CONFIG_NO_SIGN_API || !MLD_CONFIG_NO_VERIFY_API */

#endif /* !MLD_POLY_KL_H */
