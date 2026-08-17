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

#ifndef MLK_POLY_K_H
#define MLK_POLY_K_H

#include "common.h"
#include "compress.h"
#include "poly.h"

/* Parameter set namespacing
 * This is to facilitate building multiple instances
 * of mlkem-native (e.g. with varying parameter sets)
 * within a single compilation unit. */
#define mlk_polyvec MLK_ADD_PARAM_SET(mlk_polyvec)
#define mlk_polymat MLK_ADD_PARAM_SET(mlk_polymat)
#define mlk_polyvec_mulcache MLK_ADD_PARAM_SET(mlk_polyvec_mulcache)
/* End of parameter set namespacing */

typedef struct
{
  mlk_poly vec[MLKEM_K];
} MLK_ALIGN mlk_polyvec;

typedef struct
{
  mlk_polyvec vec[MLKEM_K];
} MLK_ALIGN mlk_polymat;

typedef struct
{
  mlk_poly_mulcache vec[MLKEM_K];
} MLK_ALIGN mlk_polyvec_mulcache;

#define mlk_poly_compress_du MLK_NAMESPACE_K(poly_compress_du)
/*************************************************
 * Name:        mlk_poly_compress_du
 *
 * Description: Compression (du bits) and subsequent serialization of a
 *              polynomial
 *
 * Arguments:   - uint8_t *r: pointer to output byte array
 *                  (of length MLKEM_POLYCOMPRESSEDBYTES_DU bytes)
 *              - const mlk_poly *a: pointer to input polynomial
 *                  Coefficients must be unsigned canonical,
 *                  i.e. in [0,1,..,MLKEM_Q-1].
 *
 * Specification: Implements `ByteEncode_{d_u} (Compress_{d_u} (u))`
 *                in @[FIPS203, Algorithm 14 (K-PKE.Encrypt), L22],
 *                with level-specific d_u defined in @[FIPS203, Table 2],
 *                and given by MLKEM_DU here.
 *
 **************************************************/
static MLK_INLINE void mlk_poly_compress_du(
    uint8_t r[MLKEM_POLYCOMPRESSEDBYTES_DU], const mlk_poly *a)
__contract__(
  requires(memory_no_alias(r, MLKEM_POLYCOMPRESSEDBYTES_DU))
  requires(memory_no_alias(a, sizeof(mlk_poly)))
  requires(array_bound(a->coeffs, 0, MLKEM_N, 0, MLKEM_Q))
  assigns(memory_slice(r, MLKEM_POLYCOMPRESSEDBYTES_DU)))
{
#if MLKEM_DU == 10
  mlk_poly_compress_d10(r, a);
#elif MLKEM_DU == 11
  mlk_poly_compress_d11(r, a);
#else
#error "Invalid value of MLKEM_DU"
#endif
}

#define mlk_poly_decompress_du MLK_NAMESPACE_K(poly_decompress_du)
/*************************************************
 * Name:        mlk_poly_decompress_du
 *
 * Description: De-serialization and subsequent decompression (du bits) of a
 *              polynomial; approximate inverse of mlk_poly_compress_du
 *
 * Arguments:   - mlk_poly *r: pointer to output polynomial
 *              - const uint8_t *a: pointer to input byte array
 *                   (of length MLKEM_POLYCOMPRESSEDBYTES_DU bytes)
 *
 * Upon return, the coefficients of the output polynomial are unsigned-canonical
 * (non-negative and smaller than MLKEM_Q).
 *
 * Specification: Implements `Decompress_{d_u} (ByteDecode_{d_u} (u))`
 *                in @[FIPS203, Algorithm 15 (K-PKE.Decrypt), L3].
 *                with level-specific d_u defined in @[FIPS203, Table 2],
 *                and given by MLKEM_DU here.
 *
 **************************************************/
static MLK_INLINE void mlk_poly_decompress_du(
    mlk_poly *r, const uint8_t a[MLKEM_POLYCOMPRESSEDBYTES_DU])
__contract__(
  requires(memory_no_alias(a, MLKEM_POLYCOMPRESSEDBYTES_DU))
  requires(memory_no_alias(r, sizeof(mlk_poly)))
  assigns(memory_slice(r, sizeof(mlk_poly)))
  ensures(array_bound(r->coeffs, 0, MLKEM_N, 0, MLKEM_Q)))
{
#if MLKEM_DU == 10
  mlk_poly_decompress_d10(r, a);
#elif MLKEM_DU == 11
  mlk_poly_decompress_d11(r, a);
#else
#error "Invalid value of MLKEM_DU"
#endif
}

#define mlk_poly_compress_dv MLK_NAMESPACE_K(poly_compress_dv)
/*************************************************
 * Name:        mlk_poly_compress_dv
 *
 * Description: Compression (dv bits) and subsequent serialization of a
 *              polynomial
 *
 * Arguments:   - uint8_t *r: pointer to output byte array
 *                  (of length MLKEM_POLYCOMPRESSEDBYTES_DV bytes)
 *              - const mlk_poly *a: pointer to input polynomial
 *                  Coefficients must be unsigned canonical,
 *                  i.e. in [0,1,..,MLKEM_Q-1].
 *
 * Specification: Implements `ByteEncode_{d_v} (Compress_{d_v} (v))`
 *                in @[FIPS203, Algorithm 14 (K-PKE.Encrypt), L23].
 *                with level-specific d_v defined in @[FIPS203, Table 2],
 *                and given by MLKEM_DV here.
 *
 **************************************************/
static MLK_INLINE void mlk_poly_compress_dv(
    uint8_t r[MLKEM_POLYCOMPRESSEDBYTES_DV], const mlk_poly *a)
__contract__(
  requires(memory_no_alias(r, MLKEM_POLYCOMPRESSEDBYTES_DV))
  requires(memory_no_alias(a, sizeof(mlk_poly)))
  requires(array_bound(a->coeffs, 0, MLKEM_N, 0, MLKEM_Q))
  assigns(memory_slice(r, MLKEM_POLYCOMPRESSEDBYTES_DV)))
{
#if MLKEM_DV == 4
  mlk_poly_compress_d4(r, a);
#elif MLKEM_DV == 5
  mlk_poly_compress_d5(r, a);
#else
#error "Invalid value of MLKEM_DV"
#endif
}


#define mlk_poly_decompress_dv MLK_NAMESPACE_K(poly_decompress_dv)
/*************************************************
 * Name:        mlk_poly_decompress_dv
 *
 * Description: De-serialization and subsequent decompression (dv bits) of a
 *              polynomial; approximate inverse of poly_compress
 *
 * Arguments:   - mlk_poly *r: pointer to output polynomial
 *              - const uint8_t *a: pointer to input byte array
 *                  (of length MLKEM_POLYCOMPRESSEDBYTES_DV bytes)
 *
 * Upon return, the coefficients of the output polynomial are unsigned-canonical
 * (non-negative and smaller than MLKEM_Q).
 *
 * Specification: Implements `Decompress_{d_v} (ByteDecode_{d_v} (v))`
 *                in @[FIPS203, Algorithm 15 (K-PKE.Decrypt), L4].
 *                with level-specific d_v defined in @[FIPS203, Table 2],
 *                and given by MLKEM_DV here.
 *
 **************************************************/
static MLK_INLINE void mlk_poly_decompress_dv(
    mlk_poly *r, const uint8_t a[MLKEM_POLYCOMPRESSEDBYTES_DV])
__contract__(
  requires(memory_no_alias(a, MLKEM_POLYCOMPRESSEDBYTES_DV))
  requires(memory_no_alias(r, sizeof(mlk_poly)))
  assigns(memory_slice(r, sizeof(mlk_poly)))
  ensures(array_bound(r->coeffs, 0, MLKEM_N, 0, MLKEM_Q)))
{
#if MLKEM_DV == 4
  mlk_poly_decompress_d4(r, a);
#elif MLKEM_DV == 5
  mlk_poly_decompress_d5(r, a);
#else
#error "Invalid value of MLKEM_DV"
#endif
}

#define mlk_polyvec_compress_du MLK_NAMESPACE_K(polyvec_compress_du)
/*************************************************
 * Name:        mlk_polyvec_compress_du
 *
 * Description: Compress and serialize vector of polynomials
 *
 * Arguments:   - uint8_t *r: pointer to output byte array
 *                            (needs space for MLKEM_POLYVECCOMPRESSEDBYTES_DU)
 *              - const mlk_polyvec a: pointer to input vector of polynomials.
 *                                  Coefficients must be unsigned canonical,
 *                                  i.e. in [0,1,..,MLKEM_Q-1].
 *
 * Specification: Implements `ByteEncode_{d_u} (Compress_{d_u} (u))`
 *                in @[FIPS203, Algorithm 14 (K-PKE.Encrypt), L22].
 *                with level-specific d_u defined in @[FIPS203, Table 2],
 *                and given by MLKEM_DU here.
 *
 **************************************************/
MLK_INTERNAL_API
void mlk_polyvec_compress_du(uint8_t r[MLKEM_POLYVECCOMPRESSEDBYTES_DU],
                             const mlk_polyvec *a)
__contract__(
  requires(memory_no_alias(r, MLKEM_POLYVECCOMPRESSEDBYTES_DU))
  requires(memory_no_alias(a, sizeof(mlk_polyvec)))
  requires(forall(k0, 0, MLKEM_K,
         array_bound(a->vec[k0].coeffs, 0, MLKEM_N, 0, MLKEM_Q)))
  assigns(memory_slice(r, MLKEM_POLYVECCOMPRESSEDBYTES_DU))
);

#define mlk_polyvec_decompress_du MLK_NAMESPACE_K(polyvec_decompress_du)
/*************************************************
 * Name:        mlk_polyvec_decompress_du
 *
 * Description: De-serialize and decompress vector of polynomials;
 *              approximate inverse of mlk_polyvec_compress_du
 *
 * Arguments:   - mlk_polyvec r:       pointer to output vector of polynomials.
 *                Output will have coefficients normalized to [0,..,q-1].
 *              - const uint8_t *a: pointer to input byte array
 *                                  (of length MLKEM_POLYVECCOMPRESSEDBYTES_DU)
 *
 * Specification: Implements `Decompress_{d_u} (ByteDecode_{d_u} (u))`
 *                in @[FIPS203, Algorithm 15 (K-PKE.Decrypt), L3].
 *                with level-specific d_u defined in @[FIPS203, Table 2],
 *                and given by MLKEM_DU here.
 *
 **************************************************/
MLK_INTERNAL_API
void mlk_polyvec_decompress_du(mlk_polyvec *r,
                               const uint8_t a[MLKEM_POLYVECCOMPRESSEDBYTES_DU])
__contract__(
  requires(memory_no_alias(a, MLKEM_POLYVECCOMPRESSEDBYTES_DU))
  requires(memory_no_alias(r, sizeof(mlk_polyvec)))
  assigns(memory_slice(r, sizeof(mlk_polyvec)))
  ensures(forall(k0, 0, MLKEM_K,
         array_bound(r->vec[k0].coeffs, 0, MLKEM_N, 0, MLKEM_Q)))
);

#define mlk_polyvec_tobytes MLK_NAMESPACE_K(polyvec_tobytes)
/*************************************************
 * Name:        mlk_polyvec_tobytes
 *
 * Description: Serialize vector of polynomials
 *
 * Arguments:   - uint8_t *r: pointer to output byte array
 *                            (needs space for MLKEM_POLYVECBYTES)
 *              - const mlk_polyvec a: pointer to input vector of polynomials
 *                  Each polynomial must have coefficients in [0,..,q-1].
 *
 * Specification: Implements ByteEncode_12 @[FIPS203, Algorithm 5].
 *                Extended to vectors as per
 *                @[FIPS203, 2.4.8 Applying Algorithms to Arrays]
 *                and @[FIPS203, 2.4.6, Matrices and Vectors]
 *
 **************************************************/
MLK_INTERNAL_API
void mlk_polyvec_tobytes(uint8_t r[MLKEM_POLYVECBYTES], const mlk_polyvec *a)
__contract__(
  requires(memory_no_alias(a, sizeof(mlk_polyvec)))
  requires(memory_no_alias(r, MLKEM_POLYVECBYTES))
  requires(forall(k0, 0, MLKEM_K,
         array_bound(a->vec[k0].coeffs, 0, MLKEM_N, 0, MLKEM_Q)))
  assigns(memory_slice(r, MLKEM_POLYVECBYTES))
);

#define mlk_polyvec_frombytes MLK_NAMESPACE_K(polyvec_frombytes)
/*************************************************
 * Name:        mlk_polyvec_frombytes
 *
 * Description: De-serialize vector of polynomials;
 *              inverse of mlk_polyvec_tobytes
 *
 * Arguments:   - const mlk_polyvec a: pointer to output vector of polynomials
 *                 (of length MLKEM_POLYVECBYTES). Output will have coefficients
 *                 normalized in [0..4095].
 *              - uint8_t *r: pointer to input byte array
 *
 * Specification: Implements ByteDecode_12 @[FIPS203, Algorithm 6].
 *                Extended to vectors as per
 *                @[FIPS203, 2.4.8 Applying Algorithms to Arrays]
 *                and @[FIPS203, 2.4.6, Matrices and Vectors]
 *
 **************************************************/
MLK_INTERNAL_API
void mlk_polyvec_frombytes(mlk_polyvec *r, const uint8_t a[MLKEM_POLYVECBYTES])
__contract__(
  requires(memory_no_alias(r, sizeof(mlk_polyvec)))
  requires(memory_no_alias(a, MLKEM_POLYVECBYTES))
  assigns(memory_slice(r, sizeof(mlk_polyvec)))
  ensures(forall(k0, 0, MLKEM_K,
        array_bound(r->vec[k0].coeffs, 0, MLKEM_N, 0, MLKEM_UINT12_LIMIT)))
);

#define mlk_polyvec_ntt MLK_NAMESPACE_K(polyvec_ntt)
/*************************************************
 * Name:        mlk_polyvec_ntt
 *
 * Description: Apply forward NTT to all elements of a vector of polynomials.
 *
 *              The input is assumed to be in normal order and
 *              coefficient-wise bound by MLKEM_Q in absolute value.
 *
 *              The output polynomial is in bitreversed order, and
 *              coefficient-wise bound by MLK_NTT_BOUND in absolute value.
 *
 * Arguments:   - mlk_polyvec r: pointer to in/output vector of polynomials
 *
 * Specification:
 * - Implements @[FIPS203, Algorithm 9, NTT]
 * - Extended to vectors as per @[FIPS203, 2.4.6, Matrices and Vectors]
 *
 **************************************************/
MLK_INTERNAL_API
void mlk_polyvec_ntt(mlk_polyvec *r)
__contract__(
  requires(memory_no_alias(r, sizeof(mlk_polyvec)))
  requires(forall(j, 0, MLKEM_K,
  array_abs_bound(r->vec[j].coeffs, 0, MLKEM_N, MLKEM_Q)))
  assigns(memory_slice(r, sizeof(mlk_polyvec)))
  ensures(forall(j, 0, MLKEM_K,
  array_abs_bound(r->vec[j].coeffs, 0, MLKEM_N, MLK_NTT_BOUND)))
);

#define mlk_polyvec_invntt_tomont MLK_NAMESPACE_K(polyvec_invntt_tomont)
/*************************************************
 * Name:        mlk_polyvec_invntt_tomont
 *
 * Description: Apply inverse NTT to all elements of a vector of polynomials
 *              and multiply by Montgomery factor 2^16
 *
 *              The input is assumed to be in bitreversed order, and can
 *              have arbitrary coefficients in int16_t.
 *
 *              The output polynomial is in normal order, and
 *              coefficient-wise bound by MLK_INVNTT_BOUND in absolute value.
 *
 * Arguments:   - mlk_polyvec r: pointer to in/output vector of polynomials
 *
 * Specification:
 * - Implements @[FIPS203, Algorithm 10, NTT^{-1}]
 * - Extended to vectors as per @[FIPS203, 2.4.6, Matrices and Vectors]
 *
 **************************************************/
MLK_INTERNAL_API
void mlk_polyvec_invntt_tomont(mlk_polyvec *r)
__contract__(
  requires(memory_no_alias(r, sizeof(mlk_polyvec)))
  assigns(memory_slice(r, sizeof(mlk_polyvec)))
  ensures(forall(j, 0, MLKEM_K,
  array_abs_bound(r->vec[j].coeffs, 0, MLKEM_N, MLK_INVNTT_BOUND)))
);

#define mlk_polyvec_basemul_acc_montgomery_cached \
  MLK_NAMESPACE_K(polyvec_basemul_acc_montgomery_cached)
/*************************************************
 * Name:        mlk_polyvec_basemul_acc_montgomery_cached
 *
 * Description: Scalar product of two vectors of polynomials in NTT domain,
 *              using mulcache for second operand.
 *
 *              Bounds:
 *              - Every coefficient of a is assumed to be in [0..4095]
 *              - No bounds guarantees for the coefficients in the result.
 *
 * Arguments:   - mlk_poly *r: pointer to output polynomial
 *              - const mlk_polyvec a: pointer to first input polynomial vector
 *              - const mlk_polyvec b: pointer to second input polynomial
 *                vector
 *              - const mlk_polyvec_mulcache b_cache: pointer to mulcache
 *                  for second input polynomial vector. Can be computed
 *                  via mlk_polyvec_mulcache_compute().
 *
 * Specification: Implements
 *                - @[FIPS203, Section 2.4.7, Eq (2.14)]
 *                - @[FIPS203, Algorithm 11, MultiplyNTTs]
 *                - @[FIPS203, Algorithm 12, BaseCaseMultiply]
 *
 **************************************************/
MLK_INTERNAL_API
void mlk_polyvec_basemul_acc_montgomery_cached(
    mlk_poly *r, const mlk_polyvec *a, const mlk_polyvec *b,
    const mlk_polyvec_mulcache *b_cache)
__contract__(
  requires(memory_no_alias(r, sizeof(mlk_poly)))
  requires(memory_no_alias(a, sizeof(mlk_polyvec)))
  requires(memory_no_alias(b, sizeof(mlk_polyvec)))
  requires(memory_no_alias(b_cache, sizeof(mlk_polyvec_mulcache)))
  requires(forall(k1, 0, MLKEM_K,
     array_bound(a->vec[k1].coeffs, 0, MLKEM_N, 0, MLKEM_UINT12_LIMIT)))
  assigns(memory_slice(r, sizeof(mlk_poly)))
);

#define mlk_polyvec_mulcache_compute MLK_NAMESPACE_K(polyvec_mulcache_compute)
/************************************************************
 * Name: mlk_polyvec_mulcache_compute
 *
 * Description: Computes the mulcache for a vector of polynomials in NTT domain
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
 *              The mulcache of a vector of polynomials is the vector
 *              of mulcaches of its entries.
 *
 * Arguments: - x: Pointer to mulcache to be populated
 *            - a: Pointer to input polynomial vector
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
void mlk_polyvec_mulcache_compute(mlk_polyvec_mulcache *x, const mlk_polyvec *a)
__contract__(
  requires(memory_no_alias(x, sizeof(mlk_polyvec_mulcache)))
  requires(memory_no_alias(a, sizeof(mlk_polyvec)))
  assigns(memory_slice(x, sizeof(mlk_polyvec_mulcache)))
);

#define mlk_polyvec_reduce MLK_NAMESPACE_K(polyvec_reduce)
/*************************************************
 * Name:        mlk_polyvec_reduce
 *
 * Description: Applies Barrett reduction to each coefficient
 *              of each element of a vector of polynomials;
 *              for details of the Barrett reduction see comments in poly.c
 *
 * Arguments:   - mlk_polyvec r: pointer to input/output polynomial
 *
 * Specification: Normalizes on unsigned canoncial representatives
 *                ahead of calling @[FIPS203, Compress_d, Eq (4.7)].
 *                This is not made explicit in FIPS 203.
 *
 **************************************************/
/*
 * NOTE: The semantics of mlk_polyvec_reduce() is different in
 *       the reference implementation, which requires
 *       signed canonical output data. Unsigned canonical
 *       outputs are better suited to the only remaining
 *       use of mlk_poly_reduce() in the context of (de)serialization.
 */
MLK_INTERNAL_API
void mlk_polyvec_reduce(mlk_polyvec *r)
__contract__(
  requires(memory_no_alias(r, sizeof(mlk_polyvec)))
  assigns(memory_slice(r, sizeof(mlk_polyvec)))
  ensures(forall(k0, 0, MLKEM_K,
    array_bound(r->vec[k0].coeffs, 0, MLKEM_N, 0, MLKEM_Q)))
);

#define mlk_polyvec_add MLK_NAMESPACE_K(polyvec_add)
/*************************************************
 * Name:        mlk_polyvec_add
 *
 * Description: Add vectors of polynomials
 *
 * Arguments: - mlk_polyvec r: pointer to input-output vector of polynomials to
 *              be added to
 *            - const mlk_polyvec b: pointer to second input vector of
 *              polynomials
 *
 * The coefficients of r and b must be so that the addition does
 * not overflow. Otherwise, the behaviour of this function is undefined.
 *
 * The coefficients returned in *r are in int16_t which is sufficient
 * to prove type-safety of calling units. Therefore, no stronger
 * ensures clause is required on this function.
 *
 * Specification:
 * - @[FIPS203, 2.4.5, Arithmetic With Polynomials and NTT Representations]
 * - Used in @[FIPS203, Algorithm 14 (K-PKE.Encrypt), L19]
 *
 **************************************************/
MLK_INTERNAL_API
void mlk_polyvec_add(mlk_polyvec *r, const mlk_polyvec *b)
__contract__(
  requires(memory_no_alias(r, sizeof(mlk_polyvec)))
  requires(memory_no_alias(b, sizeof(mlk_polyvec)))
  requires(forall(j0, 0, MLKEM_K,
          forall(k0, 0, MLKEM_N,
            (int32_t)r->vec[j0].coeffs[k0] + b->vec[j0].coeffs[k0] <= INT16_MAX)))
  requires(forall(j1, 0, MLKEM_K,
          forall(k1, 0, MLKEM_N,
            (int32_t)r->vec[j1].coeffs[k1] + b->vec[j1].coeffs[k1] >= INT16_MIN)))
  assigns(memory_slice(r, sizeof(mlk_polyvec)))
);

#define mlk_polyvec_tomont MLK_NAMESPACE_K(polyvec_tomont)
/*************************************************
 * Name:        mlk_polyvec_tomont
 *
 * Description: Inplace conversion of all coefficients of a polynomial
 *              vector from normal domain to Montgomery domain
 *
 *              Bounds: Output < q in absolute value.
 *
 *
 * Specification: Internal normalization required in `mlk_indcpa_keypair_derand`
 *                as part of matrix-vector multiplication
 *                @[FIPS203, Algorithm 13, K-PKE.KeyGen, L18].
 *
 **************************************************/
MLK_INTERNAL_API
void mlk_polyvec_tomont(mlk_polyvec *r)
__contract__(
  requires(memory_no_alias(r, sizeof(mlk_polyvec)))
  assigns(memory_slice(r, sizeof(mlk_polyvec)))
  ensures(forall(j, 0, MLKEM_K,
    array_abs_bound(r->vec[j].coeffs, 0, MLKEM_N, MLKEM_Q)))
);

#define mlk_poly_getnoise_eta1_4x MLK_NAMESPACE_K(poly_getnoise_eta1_4x)
/*************************************************
 * Name:        mlk_poly_getnoise_eta1_4x
 *
 * Description: Batch sample four polynomials deterministically from a seed
 *              and nonces, with output polynomials close to centered binomial
 *              distribution with parameter MLKEM_ETA1.
 *
 * Arguments:   - mlk_poly *r{0,1,2,3}: pointer to output polynomial. The last
 *                polynomial pointer may be NULL.
 *              - const uint8_t *seed: pointer to input seed
 *                                     (of length MLKEM_SYMBYTES bytes)
 *              - uint8_t nonce{0,1,2,3}: one-byte input nonce
 *
 * Specification:
 * Implements 4x `SamplePolyCBD_{eta1} (PRF_{eta1} (sigma, N))`:
 * - @[FIPS203, Algorithm 8, SamplePolyCBD_eta]
 * - @[FIPS203, Eq (4.3), PRF_eta]
 * - `SamplePolyCBD_{eta1} (PRF_{eta1} (sigma, N))` appears in
 *   @[FIPS203, Algorithm 13, K-PKE.KeyGen, L{9, 13}]
 *   @[FIPS203, Algorithm 14, K-PKE.Encrypt, L10]
 *
 **************************************************/
MLK_INTERNAL_API
void mlk_poly_getnoise_eta1_4x(mlk_poly *r0, mlk_poly *r1, mlk_poly *r2,
                               mlk_poly *r3, const uint8_t seed[MLKEM_SYMBYTES],
                               uint8_t nonce0, uint8_t nonce1, uint8_t nonce2,
                               uint8_t nonce3)
__contract__(
  requires(memory_no_alias(seed, MLKEM_SYMBYTES))
  requires(memory_no_alias(r0, sizeof(mlk_poly)))
  requires(memory_no_alias(r1, sizeof(mlk_poly)))
  requires(memory_no_alias(r2, sizeof(mlk_poly)))
  requires(r3 == NULL || memory_no_alias(r3, sizeof(mlk_poly)))
  assigns(memory_slice(r0, sizeof(mlk_poly)))
  assigns(memory_slice(r1, sizeof(mlk_poly)))
  assigns(memory_slice(r2, sizeof(mlk_poly)))
  assigns(r3 != NULL: memory_slice(r3, sizeof(mlk_poly)))
  ensures(array_abs_bound(r0->coeffs,0, MLKEM_N, MLKEM_ETA1 + 1))
  ensures(array_abs_bound(r1->coeffs,0, MLKEM_N, MLKEM_ETA1 + 1))
  ensures(array_abs_bound(r2->coeffs,0, MLKEM_N, MLKEM_ETA1 + 1))
  ensures(r3 != NULL ==> array_abs_bound(r3->coeffs,0, MLKEM_N, MLKEM_ETA1 + 1))
);

#if MLKEM_ETA1 == MLKEM_ETA2
/*
 * We only require mlk_poly_getnoise_eta2_4x for ml-kem-768 and ml-kem-1024
 * where MLKEM_ETA2 = MLKEM_ETA1 = 2.
 * For ml-kem-512, mlk_poly_getnoise_eta1122_4x is used instead.
 */
#define mlk_poly_getnoise_eta2_4x mlk_poly_getnoise_eta1_4x
#endif /* MLKEM_ETA1 == MLKEM_ETA2 */

#if MLKEM_K == 2 || MLKEM_K == 4
#define mlk_poly_getnoise_eta2 MLK_NAMESPACE_K(poly_getnoise_eta2)
/*************************************************
 * Name:        mlk_poly_getnoise_eta2
 *
 * Description: Sample a polynomial deterministically from a seed and a nonce,
 *              with output polynomial close to centered binomial distribution
 *              with parameter MLKEM_ETA2
 *
 * Arguments:   - mlk_poly *r: pointer to output polynomial
 *              - const uint8_t *seed: pointer to input seed
 *                                     (of length MLKEM_SYMBYTES bytes)
 *              - uint8_t nonce: one-byte input nonce
 *
 * Specification:
 * Implements `SamplePolyCBD_{eta2} (PRF_{eta2} (sigma, N))`:
 * - @[FIPS203, Algorithm 8, SamplePolyCBD_eta]
 * - @[FIPS203, Eq (4.3), PRF_eta]
 * - `SamplePolyCBD_{eta2} (PRF_{eta2} (sigma, N))` appears in
 *   @[FIPS203, Algorithm 14, K-PKE.Encrypt, L14]
 *
 **************************************************/
MLK_INTERNAL_API
void mlk_poly_getnoise_eta2(mlk_poly *r, const uint8_t seed[MLKEM_SYMBYTES],
                            uint8_t nonce)
__contract__(
  requires(memory_no_alias(r, sizeof(mlk_poly)))
  requires(memory_no_alias(seed, MLKEM_SYMBYTES))
  assigns(memory_slice(r, sizeof(mlk_poly)))
  ensures(array_abs_bound(r->coeffs, 0, MLKEM_N, MLKEM_ETA2 + 1))
);
#endif /* MLKEM_K == 2 || MLKEM_K == 4 */

#if MLKEM_K == 2
#define mlk_poly_getnoise_eta1122_4x MLK_NAMESPACE_K(poly_getnoise_eta1122_4x)
/*************************************************
 * Name:        mlk_poly_getnoise_eta1122_4x
 *
 * Description: Batch sample four polynomials deterministically from a seed
 * and a nonces, with output polynomials close to centered binomial
 * distribution with parameter MLKEM_ETA1 and MLKEM_ETA2
 *
 * Arguments:   - mlk_poly *r{0,1,2,3}: pointer to output polynomial
 *              - const uint8_t *seed: pointer to input seed
 *                                     (of length MLKEM_SYMBYTES bytes)
 *              - uint8_t nonce{0,1,2,3}: one-byte input nonce
 *
 * Specification:
 * Implements two instances each of
 * `SamplePolyCBD_{eta1} (PRF_{eta1} (sigma, N))` and
 * `SamplePolyCBD_{eta2} (PRF_{eta2} (sigma, N))`:
 * - @[FIPS203, Algorithm 8, SamplePolyCBD_eta]
 * - @[FIPS203, Eq (4.3), PRF_eta]
 * - `SamplePolyCBD_{eta2} (PRF_{eta2} (sigma, N))` appears in
 *   @[FIPS203, Algorithm 14, K-PKE.Encrypt, L14]
 *
 **************************************************/
MLK_INTERNAL_API
void mlk_poly_getnoise_eta1122_4x(mlk_poly *r0, mlk_poly *r1, mlk_poly *r2,
                                  mlk_poly *r3,
                                  const uint8_t seed[MLKEM_SYMBYTES],
                                  uint8_t nonce0, uint8_t nonce1,
                                  uint8_t nonce2, uint8_t nonce3)
__contract__(
  requires(memory_no_alias(r0, sizeof(mlk_poly)))
  requires(memory_no_alias(r1, sizeof(mlk_poly)))
  requires(memory_no_alias(r2, sizeof(mlk_poly)))
  requires(memory_no_alias(r3, sizeof(mlk_poly)))
  requires(memory_no_alias(seed, MLKEM_SYMBYTES))
  assigns(memory_slice(r0, sizeof(mlk_poly)))
  assigns(memory_slice(r1, sizeof(mlk_poly)))
  assigns(memory_slice(r2, sizeof(mlk_poly)))
  assigns(memory_slice(r3, sizeof(mlk_poly)))
  ensures(array_abs_bound(r0->coeffs,0, MLKEM_N, MLKEM_ETA1 + 1)
       && array_abs_bound(r1->coeffs,0, MLKEM_N, MLKEM_ETA1 + 1)
       && array_abs_bound(r2->coeffs,0, MLKEM_N, MLKEM_ETA2 + 1)
       && array_abs_bound(r3->coeffs,0, MLKEM_N, MLKEM_ETA2 + 1))
);
#endif /* MLKEM_K == 2 */

#endif /* !MLK_POLY_K_H */
