/*
 * Copyright (c) The mldsa-native project authors
 * SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT
 */
#ifndef MLD_POLYVEC_H
#define MLD_POLYVEC_H

#include "cbmc.h"
#include "common.h"
#include "poly.h"
#include "poly_kl.h"

/* Parameter set namespacing
 * This is to facilitate building multiple instances
 * of mldsa-native (e.g. with varying parameter sets)
 * within a single compilation unit. */
#define mld_polyvecl MLD_ADD_PARAM_SET(mld_polyvecl)
#define mld_polyveck MLD_ADD_PARAM_SET(mld_polyveck)
/* End of parameter set namespacing */

/** Vector of MLDSA_L polynomials. */
typedef struct
{
  mld_poly vec[MLDSA_L]; /**< Component polynomials. */
} mld_polyvecl;


#if !defined(MLD_CONFIG_NO_SIGN_API) && \
    (!defined(MLD_CONFIG_REDUCE_RAM) || defined(MLD_UNIT_TEST))
#define mld_polyvecl_uniform_gamma1 MLD_NAMESPACE_KL(polyvecl_uniform_gamma1)
/**
 * Sample vector of polynomials with uniformly random coefficients in
 * [-(MLDSA_GAMMA1 - 1), MLDSA_GAMMA1] by unpacking output stream of
 * SHAKE256(seed|nonce).
 *
 * @param[out] v     Pointer to output vector.
 * @param[in]  seed  Byte array with seed of length MLDSA_CRHBYTES.
 * @param      nonce 16-bit nonce.
 */
MLD_INTERNAL_API
void mld_polyvecl_uniform_gamma1(mld_polyvecl *v,
                                 const uint8_t seed[MLDSA_CRHBYTES],
                                 uint16_t nonce)
__contract__(
  requires(memory_no_alias(v, sizeof(mld_polyvecl)))
  requires(memory_no_alias(seed, MLDSA_CRHBYTES))
  requires(nonce <= (UINT16_MAX - MLDSA_L) / MLDSA_L)
  assigns(memory_slice(v, sizeof(mld_polyvecl)))
  ensures(forall(k0, 0, MLDSA_L,
    array_bound(v->vec[k0].coeffs, 0, MLDSA_N, -(MLDSA_GAMMA1 - 1), MLDSA_GAMMA1 + 1)))
);
#endif /* !MLD_CONFIG_NO_SIGN_API && (!MLD_CONFIG_REDUCE_RAM || MLD_UNIT_TEST) \
        */

#if !defined(MLD_CONFIG_NO_KEYPAIR_API) || \
    !defined(MLD_CONFIG_NO_VERIFY_API) ||  \
    (!defined(MLD_CONFIG_NO_SIGN_API) &&   \
     (!defined(MLD_CONFIG_REDUCE_RAM) || defined(MLD_UNIT_TEST)))
#define mld_polyvecl_ntt MLD_NAMESPACE_KL(polyvecl_ntt)
/**
 * Forward NTT of all polynomials in vector of length MLDSA_L. Output
 * coefficients are bounded by MLD_NTT_BOUND in absolute value.
 *
 * @param[in,out] v Pointer to input/output vector.
 */
MLD_INTERNAL_API
void mld_polyvecl_ntt(mld_polyvecl *v)
__contract__(
  requires(memory_no_alias(v, sizeof(mld_polyvecl)))
  requires(forall(k0, 0, MLDSA_L, array_abs_bound(v->vec[k0].coeffs, 0, MLDSA_N, MLDSA_Q)))
  assigns(memory_slice(v, sizeof(mld_polyvecl)))
  ensures(forall(k1, 0, MLDSA_L, array_abs_bound(v->vec[k1].coeffs, 0, MLDSA_N, MLD_NTT_BOUND)))
);
#endif /* !MLD_CONFIG_NO_KEYPAIR_API || !MLD_CONFIG_NO_VERIFY_API || \
          (!MLD_CONFIG_NO_SIGN_API && (!MLD_CONFIG_REDUCE_RAM ||     \
          MLD_UNIT_TEST)) */

#if !defined(MLD_CONFIG_REDUCE_RAM) || defined(MLD_UNIT_TEST)
#define mld_polyvecl_pointwise_acc_montgomery \
  MLD_NAMESPACE_KL(polyvecl_pointwise_acc_montgomery)
/**
 * Pointwise multiply vectors of polynomials of length MLDSA_L, multiply
 * resulting vector by 2^{-32} and add (accumulate) polynomials in it.
 * Input/output vectors are in NTT domain representation.
 *
 * The first input "u" must be the output of polyvec_matrix_expand() and so
 * have coefficients in [0, MLDSA_Q-1] inclusive.
 *
 * The second input "v" is assumed to be output of an NTT, and hence must have
 * coefficients bounded by [-(MLD_NTT_BOUND-1), MLD_NTT_BOUND-1] inclusive.
 *
 * @param[out] w Output polynomial.
 * @param[in]  u Pointer to first input vector.
 * @param[in]  v Pointer to second input vector.
 */
MLD_INTERNAL_API
void mld_polyvecl_pointwise_acc_montgomery(mld_poly *w, const mld_polyvecl *u,
                                           const mld_polyvecl *v)
__contract__(
  requires(memory_no_alias(w, sizeof(mld_poly)))
  requires(memory_no_alias(u, sizeof(mld_polyvecl)))
  requires(memory_no_alias(v, sizeof(mld_polyvecl)))
  requires(forall(l0, 0, MLDSA_L,
                  array_bound(u->vec[l0].coeffs, 0, MLDSA_N, 0, MLDSA_Q)))
  requires(forall(l1, 0, MLDSA_L,
    array_abs_bound(v->vec[l1].coeffs, 0, MLDSA_N, MLD_NTT_BOUND)))
  assigns(memory_slice(w, sizeof(mld_poly)))
  ensures(array_abs_bound(w->coeffs, 0, MLDSA_N, MLDSA_Q))
);
#endif /* !MLD_CONFIG_REDUCE_RAM || MLD_UNIT_TEST */

#if !defined(MLD_CONFIG_NO_KEYPAIR_API) || !defined(MLD_CONFIG_NO_VERIFY_API)
#define mld_polyvecl_chknorm MLD_NAMESPACE_KL(polyvecl_chknorm)
/**
 * Check infinity norm of polynomials in vector of length MLDSA_L. Assumes
 * input mld_polyvecl to be reduced by polyvecl_reduce().
 *
 * @param[in] v Pointer to vector.
 * @param     B Norm bound.
 *
 * @return 0 if norm of all polynomials is strictly smaller than
 *         B <= (MLDSA_Q-1)/8 and 0xFFFFFFFF otherwise.
 */
MLD_INTERNAL_API
MLD_MUST_CHECK_RETURN_VALUE
uint32_t mld_polyvecl_chknorm(const mld_polyvecl *v, int32_t B)
__contract__(
  requires(memory_no_alias(v, sizeof(mld_polyvecl)))
  requires(0 <= B && B <= (MLDSA_Q - 1) / 8)
  requires(forall(k0, 0, MLDSA_L,
    array_bound(v->vec[k0].coeffs, 0, MLDSA_N, -MLD_REDUCE32_RANGE_MAX, MLD_REDUCE32_RANGE_MAX)))
  ensures(return_value == 0 || return_value == 0xFFFFFFFF)
  ensures((return_value == 0) == forall(k1, 0, MLDSA_L, array_abs_bound(v->vec[k1].coeffs, 0, MLDSA_N, B)))
);
#endif /* !MLD_CONFIG_NO_KEYPAIR_API || !MLD_CONFIG_NO_VERIFY_API */

/** Vector of MLDSA_K polynomials. */
typedef struct
{
  mld_poly vec[MLDSA_K]; /**< Component polynomials. */
} mld_polyveck;

#if (!defined(MLD_CONFIG_NO_SIGN_API) && defined(MLD_CONFIG_REDUCE_RAM)) || \
    defined(MLD_UNIT_TEST)
#define mld_polyveck_reduce MLD_NAMESPACE_KL(polyveck_reduce)
/**
 * Reduce coefficients of polynomials in vector of length MLDSA_K to
 * representatives in [-MLD_REDUCE32_RANGE_MAX, MLD_REDUCE32_RANGE_MAX].
 *
 * @param[in,out] v Pointer to input/output vector.
 */
MLD_INTERNAL_API
void mld_polyveck_reduce(mld_polyveck *v)
__contract__(
  requires(memory_no_alias(v, sizeof(mld_polyveck)))
  requires(forall(k0, 0, MLDSA_K,
    array_bound(v->vec[k0].coeffs, 0, MLDSA_N, INT32_MIN, MLD_REDUCE32_DOMAIN_MAX)))
  assigns(memory_slice(v, sizeof(mld_polyveck)))
  ensures(forall(k1, 0, MLDSA_K,
    array_bound(v->vec[k1].coeffs, 0, MLDSA_N, -MLD_REDUCE32_RANGE_MAX, MLD_REDUCE32_RANGE_MAX)))
);
#endif /* (!MLD_CONFIG_NO_SIGN_API && MLD_CONFIG_REDUCE_RAM) || MLD_UNIT_TEST \
        */

#if !defined(MLD_CONFIG_NO_SIGN_API) || defined(MLD_UNIT_TEST)
#define mld_polyveck_caddq MLD_NAMESPACE_KL(polyveck_caddq)
/**
 * For all coefficients of polynomials in vector of length MLDSA_K add MLDSA_Q
 * if coefficient is negative.
 *
 * @param[in,out] v Pointer to input/output vector.
 */
MLD_INTERNAL_API
void mld_polyveck_caddq(mld_polyveck *v)
__contract__(
  requires(memory_no_alias(v, sizeof(mld_polyveck)))
  requires(forall(k0, 0, MLDSA_K,
    array_abs_bound(v->vec[k0].coeffs, 0, MLDSA_N, MLDSA_Q)))
  assigns(memory_slice(v, sizeof(mld_polyveck)))
  ensures(forall(k1, 0, MLDSA_K,
    array_bound(v->vec[k1].coeffs, 0, MLDSA_N, 0, MLDSA_Q)))
);
#endif /* !MLD_CONFIG_NO_SIGN_API || MLD_UNIT_TEST */

#if (!defined(MLD_CONFIG_NO_SIGN_API) || defined(MLD_UNIT_TEST)) && \
    (!defined(MLD_CONFIG_REDUCE_RAM) || defined(MLD_UNIT_TEST))
#define mld_polyveck_ntt MLD_NAMESPACE_KL(polyveck_ntt)
/**
 * Forward NTT of all polynomials in vector of length MLDSA_K. Output
 * coefficients are bounded by MLD_NTT_BOUND in absolute value.
 *
 * @param[in,out] v Pointer to input/output vector.
 */
MLD_INTERNAL_API
void mld_polyveck_ntt(mld_polyveck *v)
__contract__(
  requires(memory_no_alias(v, sizeof(mld_polyveck)))
  requires(forall(k0, 0, MLDSA_K, array_abs_bound(v->vec[k0].coeffs, 0, MLDSA_N, MLDSA_Q)))
  assigns(memory_slice(v, sizeof(mld_polyveck)))
  ensures(forall(k1, 0, MLDSA_K, array_abs_bound(v->vec[k1].coeffs, 0, MLDSA_N, MLD_NTT_BOUND)))
);
#endif /* (!MLD_CONFIG_NO_SIGN_API || MLD_UNIT_TEST) && \
          (!MLD_CONFIG_REDUCE_RAM || MLD_UNIT_TEST) */

#if !defined(MLD_CONFIG_NO_SIGN_API) || defined(MLD_UNIT_TEST)
#define mld_polyveck_invntt_tomont MLD_NAMESPACE_KL(polyveck_invntt_tomont)
/**
 * Inverse NTT and multiplication by 2^{32} of polynomials in vector of
 * length MLDSA_K.
 *
 * Input coefficients need to be less than MLDSA_Q, and output coefficients
 * are bounded by MLD_INTT_BOUND.
 *
 * @param[in,out] v Pointer to input/output vector.
 */
MLD_INTERNAL_API
void mld_polyveck_invntt_tomont(mld_polyveck *v)
__contract__(
  requires(memory_no_alias(v, sizeof(mld_polyveck)))
  requires(forall(k0, 0, MLDSA_K, array_abs_bound(v->vec[k0].coeffs, 0, MLDSA_N, MLDSA_Q)))
  assigns(memory_slice(v, sizeof(mld_polyveck)))
  ensures(forall(k1, 0, MLDSA_K, array_abs_bound(v->vec[k1].coeffs, 0, MLDSA_N, MLD_INTT_BOUND)))
);
#endif /* !MLD_CONFIG_NO_SIGN_API || MLD_UNIT_TEST */

#if !defined(MLD_CONFIG_NO_KEYPAIR_API)
#define mld_polyveck_chknorm MLD_NAMESPACE_KL(polyveck_chknorm)
/**
 * Check infinity norm of polynomials in vector of length MLDSA_K. Assumes
 * input mld_polyveck to be reduced by polyveck_reduce().
 *
 * @param[in] v Pointer to vector.
 * @param     B Norm bound.
 *
 * @return 0 if norm of all polynomials are strictly smaller than
 *         B <= (MLDSA_Q-1)/8 and 0xFFFFFFFF otherwise.
 */
MLD_INTERNAL_API
MLD_MUST_CHECK_RETURN_VALUE
uint32_t mld_polyveck_chknorm(const mld_polyveck *v, int32_t B)
__contract__(
  requires(memory_no_alias(v, sizeof(mld_polyveck)))
  requires(0 <= B && B <= (MLDSA_Q - 1) / 8)
  requires(forall(k0, 0, MLDSA_K,
                  array_bound(v->vec[k0].coeffs, 0, MLDSA_N,
                              -MLD_REDUCE32_RANGE_MAX, MLD_REDUCE32_RANGE_MAX)))
  ensures(return_value == 0 || return_value == 0xFFFFFFFF)
  ensures((return_value == 0) == forall(k1, 0, MLDSA_K, array_abs_bound(v->vec[k1].coeffs, 0, MLDSA_N, B)))
);

#endif /* !MLD_CONFIG_NO_KEYPAIR_API */

#if !defined(MLD_CONFIG_NO_SIGN_API)
#define mld_polyveck_decompose MLD_NAMESPACE_KL(polyveck_decompose)
/**
 * For all coefficients a of polynomials in vector of length MLDSA_K, compute
 * high and low bits a0, a1 such a mod^+ MLDSA_Q = a1*ALPHA + a0 with
 * -ALPHA/2 < a0 <= ALPHA/2 except a1 = (MLDSA_Q-1)/ALPHA where we set
 * a1 = 0 and -ALPHA/2 <= a0 = a mod MLDSA_Q - MLDSA_Q < 0. Assumes
 * coefficients to be standard representatives.
 *
 * @reference{The reference implementation has the input polynomial as a
 * separate argument that may be aliased with either of the outputs. Removing
 * the aliasing eases CBMC proofs.}
 *
 * @param[out]    v1 Pointer to output vector of polynomials with
 *                   coefficients a1.
 * @param[in,out] v0 Pointer to input/output vector of polynomials. Output
 *                   polynomial has coefficients a0.
 */
MLD_INTERNAL_API
void mld_polyveck_decompose(mld_polyveck *v1, mld_polyveck *v0)
__contract__(
  requires(memory_no_alias(v1,  sizeof(mld_polyveck)))
  requires(memory_no_alias(v0, sizeof(mld_polyveck)))
  requires(forall(k0, 0, MLDSA_K,
    array_bound(v0->vec[k0].coeffs, 0, MLDSA_N, 0, MLDSA_Q)))
  assigns(memory_slice(v1, sizeof(mld_polyveck)))
  assigns(memory_slice(v0, sizeof(mld_polyveck)))
  ensures(forall(k1, 0, MLDSA_K,
                 array_bound(v1->vec[k1].coeffs, 0, MLDSA_N, 0, (MLDSA_Q-1)/(2*MLDSA_GAMMA2))))
  ensures(forall(k2, 0, MLDSA_K,
                 array_abs_bound(v0->vec[k2].coeffs, 0, MLDSA_N, MLDSA_GAMMA2+1)))
);
#endif /* !MLD_CONFIG_NO_SIGN_API */

#if !defined(MLD_CONFIG_NO_SIGN_API)
#define mld_polyveck_pack_w1 MLD_NAMESPACE_KL(polyveck_pack_w1)
/**
 * Bit-pack polynomial vector w1 with coefficients in [0, 15] or [0, 43]. Input
 * coefficients are assumed to be standard representatives.
 *
 * @param[out] r  Pointer to output byte array with at least
 *                MLDSA_K * MLDSA_POLYW1_PACKEDBYTES bytes.
 * @param[in]  w1 Pointer to input polynomial vector.
 */
MLD_INTERNAL_API
void mld_polyveck_pack_w1(uint8_t r[MLDSA_K * MLDSA_POLYW1_PACKEDBYTES],
                          const mld_polyveck *w1)
__contract__(
  requires(memory_no_alias(r, MLDSA_K * MLDSA_POLYW1_PACKEDBYTES))
  requires(memory_no_alias(w1, sizeof(mld_polyveck)))
  requires(forall(k1, 0, MLDSA_K,
    array_bound(w1->vec[k1].coeffs, 0, MLDSA_N, 0, (MLDSA_Q-1)/(2*MLDSA_GAMMA2))))
  assigns(memory_slice(r, MLDSA_K * MLDSA_POLYW1_PACKEDBYTES))
);
#endif /* !MLD_CONFIG_NO_SIGN_API */

#if !defined(MLD_CONFIG_NO_KEYPAIR_API)
#define mld_polyveck_pack_eta MLD_NAMESPACE_KL(polyveck_pack_eta)
/**
 * Bit-pack polynomial vector with coefficients in [-MLDSA_ETA, MLDSA_ETA].
 *
 * @param[out] r Pointer to output byte array with
 *               MLDSA_K * MLDSA_POLYETA_PACKEDBYTES bytes.
 * @param[in]  p Pointer to input polynomial vector.
 */
MLD_INTERNAL_API
void mld_polyveck_pack_eta(uint8_t r[MLDSA_K * MLDSA_POLYETA_PACKEDBYTES],
                           const mld_polyveck *p)
__contract__(
  requires(memory_no_alias(r,  MLDSA_K * MLDSA_POLYETA_PACKEDBYTES))
  requires(memory_no_alias(p, sizeof(mld_polyveck)))
  requires(forall(k1, 0, MLDSA_K,
    array_abs_bound(p->vec[k1].coeffs, 0, MLDSA_N, MLDSA_ETA + 1)))
  assigns(memory_slice(r, MLDSA_K * MLDSA_POLYETA_PACKEDBYTES))
);

#define mld_polyvecl_pack_eta MLD_NAMESPACE_KL(polyvecl_pack_eta)
/**
 * Bit-pack polynomial vector with coefficients in [-MLDSA_ETA, MLDSA_ETA].
 *
 * @param[out] r Pointer to output byte array with
 *               MLDSA_L * MLDSA_POLYETA_PACKEDBYTES bytes.
 * @param[in]  p Pointer to input polynomial vector.
 */
MLD_INTERNAL_API
void mld_polyvecl_pack_eta(uint8_t r[MLDSA_L * MLDSA_POLYETA_PACKEDBYTES],
                           const mld_polyvecl *p)
__contract__(
  requires(memory_no_alias(r,  MLDSA_L * MLDSA_POLYETA_PACKEDBYTES))
  requires(memory_no_alias(p, sizeof(mld_polyvecl)))
  requires(forall(k1, 0, MLDSA_L,
    array_abs_bound(p->vec[k1].coeffs, 0, MLDSA_N, MLDSA_ETA + 1)))
  assigns(memory_slice(r, MLDSA_L * MLDSA_POLYETA_PACKEDBYTES))
);

#endif /* !MLD_CONFIG_NO_KEYPAIR_API */

#if !defined(MLD_CONFIG_NO_KEYPAIR_API) || \
    (!defined(MLD_CONFIG_NO_SIGN_API) &&   \
     (!defined(MLD_CONFIG_REDUCE_RAM) || defined(MLD_UNIT_TEST)))
#define mld_polyvecl_unpack_eta MLD_NAMESPACE_KL(polyvecl_unpack_eta)
/**
 * Unpack polynomial vector with coefficients in [-MLDSA_ETA, MLDSA_ETA].
 *
 * @param[out] p Pointer to output polynomial vector.
 * @param[in]  r Input byte array with bit-packed polynomial vector.
 */
MLD_INTERNAL_API
void mld_polyvecl_unpack_eta(
    mld_polyvecl *p, const uint8_t r[MLDSA_L * MLDSA_POLYETA_PACKEDBYTES])
__contract__(
  requires(memory_no_alias(r,  MLDSA_L * MLDSA_POLYETA_PACKEDBYTES))
  requires(memory_no_alias(p, sizeof(mld_polyvecl)))
  assigns(memory_slice(p, sizeof(mld_polyvecl)))
  ensures(forall(k1, 0, MLDSA_L,
    array_bound(p->vec[k1].coeffs, 0, MLDSA_N, MLD_POLYETA_UNPACK_LOWER_BOUND, MLDSA_ETA + 1)))
);
#endif /* !MLD_CONFIG_NO_KEYPAIR_API || (!MLD_CONFIG_NO_SIGN_API && \
          (!MLD_CONFIG_REDUCE_RAM || MLD_UNIT_TEST)) */

#if !defined(MLD_CONFIG_NO_VERIFY_API)
#define mld_polyvecl_unpack_z MLD_NAMESPACE_KL(polyvecl_unpack_z)
/**
 * Unpack polynomial vector with coefficients in
 * [-(MLDSA_GAMMA1 - 1), MLDSA_GAMMA1].
 *
 * @param[out] z Pointer to output polynomial vector.
 * @param[in]  r Input byte array with bit-packed polynomial vector.
 */
MLD_INTERNAL_API
void mld_polyvecl_unpack_z(mld_polyvecl *z,
                           const uint8_t r[MLDSA_L * MLDSA_POLYZ_PACKEDBYTES])
__contract__(
  requires(memory_no_alias(r,  MLDSA_L * MLDSA_POLYZ_PACKEDBYTES))
  requires(memory_no_alias(z, sizeof(mld_polyvecl)))
  assigns(memory_slice(z, sizeof(mld_polyvecl)))
  ensures(forall(k1, 0, MLDSA_L,
    array_bound(z->vec[k1].coeffs, 0, MLDSA_N, -(MLDSA_GAMMA1 - 1), MLDSA_GAMMA1 + 1)))
);
#endif /* !MLD_CONFIG_NO_VERIFY_API */

#if !defined(MLD_CONFIG_NO_KEYPAIR_API) || \
    (!defined(MLD_CONFIG_NO_SIGN_API) &&   \
     (!defined(MLD_CONFIG_REDUCE_RAM) || defined(MLD_UNIT_TEST)))
#define mld_polyveck_unpack_eta MLD_NAMESPACE_KL(polyveck_unpack_eta)
/**
 * Unpack polynomial vector with coefficients in [-MLDSA_ETA, MLDSA_ETA].
 *
 * @param[out] p Pointer to output polynomial vector.
 * @param[in]  r Input byte array with bit-packed polynomial vector.
 */
MLD_INTERNAL_API
void mld_polyveck_unpack_eta(
    mld_polyveck *p, const uint8_t r[MLDSA_K * MLDSA_POLYETA_PACKEDBYTES])
__contract__(
  requires(memory_no_alias(r,  MLDSA_K * MLDSA_POLYETA_PACKEDBYTES))
  requires(memory_no_alias(p, sizeof(mld_polyveck)))
  assigns(memory_slice(p, sizeof(mld_polyveck)))
  ensures(forall(k1, 0, MLDSA_K,
    array_bound(p->vec[k1].coeffs, 0, MLDSA_N, MLD_POLYETA_UNPACK_LOWER_BOUND, MLDSA_ETA + 1)))
);
#endif /* !MLD_CONFIG_NO_KEYPAIR_API || (!MLD_CONFIG_NO_SIGN_API && \
          (!MLD_CONFIG_REDUCE_RAM || MLD_UNIT_TEST)) */


#endif /* !MLD_POLYVEC_H */
