/*
 * Copyright (c) The mlkem-native project authors
 * SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT
 */

#ifndef MLK_NATIVE_API_H
#define MLK_NATIVE_API_H
/*
 * Native arithmetic interface
 *
 * This header is primarily for documentation purposes.
 * It should not be included by backend implementations.
 *
 * To ensure consistency with backends, the header will be
 * included automatically after inclusion of the active
 * backend, to ensure consistency of function signatures,
 * and run sanity checks.
 */

#include "../cbmc.h"
#include "../common.h"

/* Backends must return MLK_NATIVE_FUNC_SUCCESS upon success. */
#define MLK_NATIVE_FUNC_SUCCESS (0)
/* Backends may return MLK_NATIVE_FUNC_FALLBACK to signal to the frontend that
 * the target/parameters are unsupported; typically, this would be because of
 * dependencies on CPU features not detected on the host CPU. In this case,
 * the frontend falls back to the default C implementation. */
#define MLK_NATIVE_FUNC_FALLBACK (-1)


/* Absolute exclusive upper bound for the output of the inverse NTT
 *
 * NOTE: This is the same bound as in poly.h and has to be kept
 * in sync. */
#define MLK_INVNTT_BOUND (8 * MLKEM_Q)

/* Absolute exclusive upper bound for the output of the forward NTT
 *
 * NOTE: This is the same bound as in poly.h and has to be kept
 * in sync. */
#define MLK_NTT_BOUND (8 * MLKEM_Q)

/*
 * This is the C<->native interface allowing for the drop-in of
 * native code for performance critical arithmetic components of ML-KEM.
 *
 * A _backend_ is a specific implementation of (part of) this interface.
 *
 * To add a function to a backend, define MLK_USE_NATIVE_XXX and
 * implement `static inline xxx(...)` in the profile header.
 *
 * The only exception is MLK_USE_NATIVE_NTT_CUSTOM_ORDER. This option can
 * be set if there are native implementations for all of NTT, invNTT, and
 * base multiplication, and allows the native implementation to use a
 * custom order of polynomial coefficients in NTT domain -- the use of such
 * custom order is not an implementation-detail since the public matrix
 * is generated in NTT domain. In this case, a permutation function
 * mlk_poly_permute_bitrev_to_custom() needs to be provided that permutes
 * polynomials in NTT domain from bitreversed to the custom order.
 */

/*
 * Those functions are meant to be trivial wrappers around the chosen native
 * implementation. The are static inline to avoid unnecessary calls.
 * The macro before each declaration controls whether a native
 * implementation is present.
 */

#if defined(MLK_USE_NATIVE_NTT)
/*************************************************
 * Name:        mlk_ntt_native
 *
 * Description: Computes negacyclic number-theoretic transform (NTT) of
 *              a polynomial in place.
 *
 *              The input polynomial is assumed to be in normal order.
 *              The output polynomial is in bitreversed order, or of a
 *              custom order if MLK_USE_NATIVE_NTT_CUSTOM_ORDER is set.
 *              See the documentation of MLK_USE_NATIVE_NTT_CUSTOM_ORDER
 *              for more information.
 *
 * Arguments:   - int16_t p[MLKEM_N]: pointer to in/output polynomial
 **************************************************/
MLK_MUST_CHECK_RETURN_VALUE
static MLK_INLINE int mlk_ntt_native(int16_t p[MLKEM_N])
__contract__(
  requires(memory_no_alias(p, sizeof(int16_t) * MLKEM_N))
  requires(array_abs_bound(p, 0, MLKEM_N, MLKEM_Q))
  assigns(memory_slice(p, sizeof(int16_t) * MLKEM_N))
  ensures(return_value == MLK_NATIVE_FUNC_FALLBACK || return_value == MLK_NATIVE_FUNC_SUCCESS)
  ensures((return_value == MLK_NATIVE_FUNC_SUCCESS) ==> array_abs_bound(p, 0, MLKEM_N, MLK_NTT_BOUND))
  ensures((return_value == MLK_NATIVE_FUNC_FALLBACK) ==> array_abs_bound(p, 0, MLKEM_N, MLKEM_Q))
  ensures((return_value == MLK_NATIVE_FUNC_FALLBACK) ==> array_unchanged(p, MLKEM_N))
);
#endif /* MLK_USE_NATIVE_NTT */

#if defined(MLK_USE_NATIVE_NTT_CUSTOM_ORDER)
/*
 * This must only be set if NTT, invNTT, basemul, mulcache, and
 * to/from byte stream conversions all have native implementations
 * that are adapted to the custom order.
 */
#if !defined(MLK_USE_NATIVE_NTT) || !defined(MLK_USE_NATIVE_INTT) ||  \
    !defined(MLK_USE_NATIVE_POLY_MULCACHE_COMPUTE) ||                 \
    !defined(MLK_USE_NATIVE_POLYVEC_BASEMUL_ACC_MONTGOMERY_CACHED) || \
    !defined(MLK_USE_NATIVE_POLY_TOBYTES) ||                          \
    !defined(MLK_USE_NATIVE_POLY_FROMBYTES)
#error \
    "Invalid native profile: MLK_USE_NATIVE_NTT_CUSTOM_ORDER can only be \
set if there are native implementations for NTT, invNTT, mulcache, basemul, \
and to/from bytes conversions."
#endif /* !MLK_USE_NATIVE_NTT || !MLK_USE_NATIVE_INTT ||           \
          !MLK_USE_NATIVE_POLY_MULCACHE_COMPUTE ||                 \
          !MLK_USE_NATIVE_POLYVEC_BASEMUL_ACC_MONTGOMERY_CACHED || \
          !MLK_USE_NATIVE_POLY_TOBYTES || !MLK_USE_NATIVE_POLY_FROMBYTES */

/*************************************************
 * Name:        mlk_poly_permute_bitrev_to_custom
 *
 * Description: When MLK_USE_NATIVE_NTT_CUSTOM_ORDER is defined,
 *              convert a polynomial in NTT domain from bitreversed
 *              order to the custom order output by the native NTT.
 *
 *              This must only be defined if there is native code for
 *              all of (a) NTT, (b) invNTT, (c) basemul, (d) mulcache.
 * Arguments:   - int16_t p[MLKEM_N]: pointer to in/output polynomial
 *
 **************************************************/
static MLK_INLINE void mlk_poly_permute_bitrev_to_custom(int16_t p[MLKEM_N])
__contract__(
  /* We don't specify that this should be a permutation, but only
   * that it does not change the bound established at the end of mlk_gen_matrix. */
  requires(memory_no_alias(p, sizeof(int16_t) * MLKEM_N))
  requires(array_bound(p, 0, MLKEM_N, 0, MLKEM_Q))
  assigns(memory_slice(p, sizeof(int16_t) * MLKEM_N))
  ensures(array_bound(p, 0, MLKEM_N, 0, MLKEM_Q)));
#endif /* MLK_USE_NATIVE_NTT_CUSTOM_ORDER */

#if defined(MLK_USE_NATIVE_INTT)
/*************************************************
 * Name:        mlk_intt_native
 *
 * Description: Computes inverse of negacyclic number-theoretic transform (NTT)
 *              of a polynomial in place.
 *
 *              The input polynomial is in bitreversed order, or of a
 *              custom order if MLK_USE_NATIVE_NTT_CUSTOM_ORDER is set.
 *              See the documentation of MLK_USE_NATIVE_NTT_CUSTOM_ORDER
 *              for more information.
 *              The output polynomial is assumed to be in normal order.
 *
 * Arguments:   - uint16_t *a: pointer to in/output polynomial
 **************************************************/
MLK_MUST_CHECK_RETURN_VALUE
static MLK_INLINE int mlk_intt_native(int16_t p[MLKEM_N])
__contract__(
  requires(memory_no_alias(p, sizeof(int16_t) * MLKEM_N))
  assigns(memory_slice(p, sizeof(int16_t) * MLKEM_N))
  ensures(return_value == MLK_NATIVE_FUNC_FALLBACK || return_value == MLK_NATIVE_FUNC_SUCCESS)
  ensures((return_value == MLK_NATIVE_FUNC_SUCCESS) ==> array_abs_bound(p, 0, MLKEM_N, MLK_INVNTT_BOUND))
  ensures((return_value == MLK_NATIVE_FUNC_FALLBACK) ==> array_unchanged(p, MLKEM_N))
);
#endif /* MLK_USE_NATIVE_INTT */

#if defined(MLK_USE_NATIVE_POLY_REDUCE)
/*************************************************
 * Name:        mlk_poly_reduce_native
 *
 * Description: Applies modular reduction to all coefficients of a polynomial.
 *
 * Arguments:   - int16_t r[MLKEM_N]: pointer to input/output polynomial
 **************************************************/
MLK_MUST_CHECK_RETURN_VALUE
static MLK_INLINE int mlk_poly_reduce_native(int16_t p[MLKEM_N])
__contract__(
  requires(memory_no_alias(p, sizeof(int16_t) * MLKEM_N))
  assigns(memory_slice(p, sizeof(int16_t) * MLKEM_N))
  ensures(return_value == MLK_NATIVE_FUNC_FALLBACK || return_value == MLK_NATIVE_FUNC_SUCCESS)
  ensures((return_value == MLK_NATIVE_FUNC_SUCCESS) ==> array_bound(p, 0, MLKEM_N, 0, MLKEM_Q))
  ensures((return_value == MLK_NATIVE_FUNC_FALLBACK) ==> array_unchanged(p, MLKEM_N))
);
#endif /* MLK_USE_NATIVE_POLY_REDUCE */

#if defined(MLK_USE_NATIVE_POLY_TOMONT)
/*************************************************
 * Name:        mlk_poly_tomont_native
 *
 * Description: Inplace conversion of all coefficients of a polynomial
 *              from normal domain to Montgomery domain
 *
 * Arguments:   - int16_t r[MLKEM_N]: pointer to input/output polynomial
 **************************************************/
MLK_MUST_CHECK_RETURN_VALUE
static MLK_INLINE int mlk_poly_tomont_native(int16_t p[MLKEM_N])
__contract__(
  requires(memory_no_alias(p, sizeof(int16_t) * MLKEM_N))
  assigns(memory_slice(p, sizeof(int16_t) * MLKEM_N))
  ensures(return_value == MLK_NATIVE_FUNC_FALLBACK || return_value == MLK_NATIVE_FUNC_SUCCESS)
  ensures((return_value == MLK_NATIVE_FUNC_SUCCESS) ==> array_abs_bound(p, 0, MLKEM_N, MLKEM_Q))
  ensures((return_value == MLK_NATIVE_FUNC_FALLBACK) ==> array_unchanged(p, MLKEM_N))
);
#endif /* MLK_USE_NATIVE_POLY_TOMONT */

#if defined(MLK_USE_NATIVE_POLY_MULCACHE_COMPUTE)
/*************************************************
 * Name:        mlk_poly_mulcache_compute_native
 *
 * Description: Compute multiplication cache for a polynomial
 *              in NTT domain.
 *
 *              The purpose of the multiplication cache is to
 *              cache repeated computations required during a
 *              base multiplication of polynomials in NTT domain.
 *              The structure of the multiplication-cache is
 *              implementation defined.
 *
 * Arguments:   INPUT:
 *              - mlk_poly: const pointer to input polynomial.
 *                  This must be in NTT domain and inin bitreversed order, or of
 *                  a custom order if MLK_USE_NATIVE_NTT_CUSTOM_ORDER is set.
 *                  See the documentation of MLK_USE_NATIVE_NTT_CUSTOM_ORDER
 *                  for more information.
 *              OUTPUT
 *              - cache: pointer to multiplication cache
 **************************************************/
MLK_MUST_CHECK_RETURN_VALUE
static MLK_INLINE int mlk_poly_mulcache_compute_native(
    int16_t cache[MLKEM_N / 2], const int16_t mlk_poly[MLKEM_N])
__contract__(
  requires(memory_no_alias(cache, sizeof(int16_t) * (MLKEM_N / 2)))
  requires(memory_no_alias(mlk_poly, sizeof(int16_t) * MLKEM_N))
  assigns(memory_slice(cache, sizeof(int16_t) * (MLKEM_N / 2)))
  ensures(return_value == MLK_NATIVE_FUNC_FALLBACK || return_value == MLK_NATIVE_FUNC_SUCCESS)
  ensures((return_value == MLK_NATIVE_FUNC_SUCCESS) ==> array_abs_bound(cache, 0, MLKEM_N/2, MLKEM_Q))
);
#endif /* MLK_USE_NATIVE_POLY_MULCACHE_COMPUTE */

#if defined(MLK_USE_NATIVE_POLYVEC_BASEMUL_ACC_MONTGOMERY_CACHED)
#if defined(MLK_CONFIG_MULTILEVEL_WITH_SHARED) || MLKEM_K == 2
/*************************************************
 * Name:        poly_mulcache_compute_k2_native
 *
 * Description: Compute scalar product of length-2 polynomial vectors in NTT
 *              domain.
 *
 * Arguments:   INPUT:
 *              - a: First polynomial vector operand.
 *                 This must be in NTT domain and in bitreversed order, or of
 *                 a custom order if MLK_USE_NATIVE_NTT_CUSTOM_ORDER is set.
 *                 See the documentation of MLK_USE_NATIVE_NTT_CUSTOM_ORDER
 *                 for more information.
 *              - b: Second polynomial vector operand.
 *                 As for a.
 *              - b_cache: Multiplication-cache for b.
 *              OUTPUT
 *              - r: The result of the scalar product. This is again
 *                   in NTT domain, and of the same ordering as a and b.
 **************************************************/
MLK_MUST_CHECK_RETURN_VALUE
static MLK_INLINE int mlk_polyvec_basemul_acc_montgomery_cached_k2_native(
    int16_t r[MLKEM_N], const int16_t a[2 * MLKEM_N],
    const int16_t b[2 * MLKEM_N], const int16_t b_cache[2 * (MLKEM_N / 2)])
__contract__(
  requires(memory_no_alias(r, sizeof(int16_t) * MLKEM_N))
  requires(memory_no_alias(a, sizeof(int16_t) * 2 * MLKEM_N))
  requires(memory_no_alias(b, sizeof(int16_t) * 2 * MLKEM_N))
  requires(memory_no_alias(b_cache, sizeof(int16_t) * 2 * (MLKEM_N / 2)))
  requires(array_bound(a, 0, 2 * MLKEM_N, 0, MLKEM_UINT12_LIMIT))
  assigns(memory_slice(r, sizeof(int16_t) * MLKEM_N))
  ensures(return_value == MLK_NATIVE_FUNC_FALLBACK || return_value == MLK_NATIVE_FUNC_SUCCESS)
);
#endif /* MLK_CONFIG_MULTILEVEL_WITH_SHARED || MLKEM_K == 2 */

#if defined(MLK_CONFIG_MULTILEVEL_WITH_SHARED) || MLKEM_K == 3
/*************************************************
 * Name:        poly_mulcache_compute_k3_native
 *
 * Description: Compute scalar product of length-3 polynomial vectors in NTT
 *              domain.
 *
 * Arguments:   INPUT:
 *              - a: First polynomial vector operand.
 *                 This must be in NTT domain and in bitreversed order, or of
 *                 a custom order if MLK_USE_NATIVE_NTT_CUSTOM_ORDER is set.
 *                 See the documentation of MLK_USE_NATIVE_NTT_CUSTOM_ORDER
 *                 for more information.
 *              - b: Second polynomial vector operand.
 *                 As for a.
 *              - b_cache: Multiplication-cache for b.
 *              OUTPUT
 *              - r: The result of the scalar product. This is again
 *                   in NTT domain, and of the same ordering as a and b.
 **************************************************/
MLK_MUST_CHECK_RETURN_VALUE
static MLK_INLINE int mlk_polyvec_basemul_acc_montgomery_cached_k3_native(
    int16_t r[MLKEM_N], const int16_t a[3 * MLKEM_N],
    const int16_t b[3 * MLKEM_N], const int16_t b_cache[3 * (MLKEM_N / 2)])
__contract__(
  requires(memory_no_alias(r, sizeof(int16_t) * MLKEM_N))
  requires(memory_no_alias(a, sizeof(int16_t) * 3 * MLKEM_N))
  requires(memory_no_alias(b, sizeof(int16_t) * 3 * MLKEM_N))
  requires(memory_no_alias(b_cache, sizeof(int16_t) * 3 * (MLKEM_N / 2)))
  requires(array_bound(a, 0, 3 * MLKEM_N, 0, MLKEM_UINT12_LIMIT))
  assigns(memory_slice(r, sizeof(int16_t) * MLKEM_N))
  ensures(return_value == MLK_NATIVE_FUNC_FALLBACK || return_value == MLK_NATIVE_FUNC_SUCCESS)
);
#endif /* MLK_CONFIG_MULTILEVEL_WITH_SHARED || MLKEM_K == 3 */

#if defined(MLK_CONFIG_MULTILEVEL_WITH_SHARED) || MLKEM_K == 4
/*************************************************
 * Name:        poly_mulcache_compute_k4_native
 *
 * Description: Compute scalar product of length-4 polynomial vectors in NTT
 *              domain.
 *
 * Arguments:   INPUT:
 *              - a: First polynomial vector operand.
 *                 This must be in NTT domain and in bitreversed order, or of
 *                 a custom order if MLK_USE_NATIVE_NTT_CUSTOM_ORDER is set.
 *                 See the documentation of MLK_USE_NATIVE_NTT_CUSTOM_ORDER
 *                 for more information.
 *              - b: Second polynomial vector operand.
 *                 As for a.
 *              - b_cache: Multiplication-cache for b.
 *              OUTPUT
 *              - r: The result of the scalar product. This is again
 *                   in NTT domain, and of the same ordering as a and b.
 **************************************************/
MLK_MUST_CHECK_RETURN_VALUE
static MLK_INLINE int mlk_polyvec_basemul_acc_montgomery_cached_k4_native(
    int16_t r[MLKEM_N], const int16_t a[4 * MLKEM_N],
    const int16_t b[4 * MLKEM_N], const int16_t b_cache[4 * (MLKEM_N / 2)])
__contract__(
  requires(memory_no_alias(r, sizeof(int16_t) * MLKEM_N))
  requires(memory_no_alias(a, sizeof(int16_t) * 4 * MLKEM_N))
  requires(memory_no_alias(b, sizeof(int16_t) * 4 * MLKEM_N))
  requires(memory_no_alias(b_cache, sizeof(int16_t) * 4 * (MLKEM_N / 2)))
  requires(array_bound(a, 0, 4 * MLKEM_N, 0, MLKEM_UINT12_LIMIT))
  assigns(memory_slice(r, sizeof(int16_t) * MLKEM_N))
  ensures(return_value == MLK_NATIVE_FUNC_FALLBACK || return_value == MLK_NATIVE_FUNC_SUCCESS)
);
#endif /* MLK_CONFIG_MULTILEVEL_WITH_SHARED || MLKEM_K == 4 */
#endif /* MLK_USE_NATIVE_POLYVEC_BASEMUL_ACC_MONTGOMERY_CACHED */

#if defined(MLK_USE_NATIVE_POLY_TOBYTES)
/*************************************************
 * Name:        mlk_poly_tobytes_native
 *
 * Description: Serialization of a polynomial.
 *              Signed coefficients are converted to
 *              unsigned form before serialization.
 *
 * Arguments:   INPUT:
 *              - a: const pointer to input polynomial,
 *                with each coefficient in the range 0 .. Q-1
 *              OUTPUT
 *              - r: pointer to output byte array
 *                   (of MLKEM_POLYBYTES bytes)
 **************************************************/
MLK_MUST_CHECK_RETURN_VALUE
static MLK_INLINE int mlk_poly_tobytes_native(uint8_t r[MLKEM_POLYBYTES],
                                              const int16_t a[MLKEM_N])
__contract__(
  requires(memory_no_alias(r, MLKEM_POLYBYTES))
  requires(memory_no_alias(a, sizeof(int16_t) * MLKEM_N))
  requires(array_bound(a, 0, MLKEM_N, 0, MLKEM_Q))
  assigns(memory_slice(r, MLKEM_POLYBYTES))
  ensures(return_value == MLK_NATIVE_FUNC_SUCCESS || return_value == MLK_NATIVE_FUNC_FALLBACK)
);
#endif /* MLK_USE_NATIVE_POLY_TOBYTES */

#if defined(MLK_USE_NATIVE_POLY_FROMBYTES)
/*************************************************
 * Name:        mlk_poly_frombytes_native
 *
 * Description: Serialization of a polynomial.
 *              Signed coefficients are converted to
 *              unsigned form before serialization.
 *
 * Arguments:   INPUT:
 *              - r: pointer to output polynomial in NTT domain
 *              OUTPUT
 *              - a: const pointer to input byte array
 *                   (of MLKEM_POLYBYTES bytes)
 **************************************************/
MLK_MUST_CHECK_RETURN_VALUE
static MLK_INLINE int mlk_poly_frombytes_native(
    int16_t a[MLKEM_N], const uint8_t r[MLKEM_POLYBYTES])
__contract__(
  requires(memory_no_alias(r, MLKEM_POLYBYTES))
  requires(memory_no_alias(a, sizeof(int16_t) * MLKEM_N))
  assigns(memory_slice(a, sizeof(int16_t) * MLKEM_N))
  ensures(return_value == MLK_NATIVE_FUNC_SUCCESS || return_value == MLK_NATIVE_FUNC_FALLBACK)
  ensures((return_value == MLK_NATIVE_FUNC_SUCCESS) ==> array_bound(a, 0, MLKEM_N, 0, MLKEM_UINT12_LIMIT))
);
#endif /* MLK_USE_NATIVE_POLY_FROMBYTES */

#if defined(MLK_USE_NATIVE_REJ_UNIFORM)
/*************************************************
 * Name:        mlk_rej_uniform_native
 *
 * Description: Run rejection sampling on uniform random bytes to generate
 *              uniform random integers mod q
 *
 * Arguments:   - int16_t *r:          pointer to output buffer
 *              - unsigned len:        requested number of 16-bit integers
 *                                     (uniform mod q).
 *              - const uint8_t *buf:  pointer to input buffer
 *                                     (assumed to be uniform random bytes)
 *              - unsigned buflen:     length of input buffer in bytes.
 *
 * Return -1 if the native implementation does not support the input lengths.
 * Otherwise, returns non-negative number of sampled 16-bit integers (at most
 * len).
 **************************************************/
MLK_MUST_CHECK_RETURN_VALUE
static MLK_INLINE int mlk_rej_uniform_native(int16_t *r, unsigned len,
                                             const uint8_t *buf,
                                             unsigned buflen)
__contract__(
  requires(len <= 4096 && buflen <= 4096 && buflen % 3 == 0)
  requires(memory_no_alias(r, sizeof(int16_t) * len))
  requires(memory_no_alias(buf, buflen))
  assigns(memory_slice(r, sizeof(int16_t) * len))
  ensures(return_value != MLK_NATIVE_FUNC_FALLBACK
              ==> (0 <= return_value && return_value <= len))
  ensures(return_value != MLK_NATIVE_FUNC_FALLBACK
              ==> array_bound(r, 0, (unsigned) return_value, 0, MLKEM_Q))
);
#endif /* MLK_USE_NATIVE_REJ_UNIFORM */

#if defined(MLK_CONFIG_MULTILEVEL_WITH_SHARED) || (MLKEM_K == 2 || MLKEM_K == 3)
#if defined(MLK_USE_NATIVE_POLY_COMPRESS_D4)
/*************************************************
 * Name:        mlk_poly_compress_d4_native
 *
 * Description: Compression (4 bits) and subsequent serialization of a
 *              polynomial
 *
 * Arguments:   - uint8_t *r: pointer to output byte array
 *                   (of length MLKEM_POLYCOMPRESSEDBYTES_D4 bytes)
 *              - const int16_t a[MLKEM_N]: pointer to input polynomial
 *                  Coefficients must be unsigned canonical,
 *                  i.e. in [0,1,..,MLKEM_Q-1].
 **************************************************/
MLK_MUST_CHECK_RETURN_VALUE
static MLK_INLINE int mlk_poly_compress_d4_native(
    uint8_t r[MLKEM_POLYCOMPRESSEDBYTES_D4], const int16_t a[MLKEM_N])
__contract__(
  requires(memory_no_alias(r, MLKEM_POLYCOMPRESSEDBYTES_D4))
  requires(memory_no_alias(a, sizeof(int16_t) * MLKEM_N))
  requires(array_bound(a, 0, MLKEM_N, 0, MLKEM_Q))
  assigns(memory_slice(r, MLKEM_POLYCOMPRESSEDBYTES_D4))
  ensures(return_value == MLK_NATIVE_FUNC_SUCCESS || return_value == MLK_NATIVE_FUNC_FALLBACK));
#endif /* MLK_USE_NATIVE_POLY_COMPRESS_D4 */

#if defined(MLK_USE_NATIVE_POLY_COMPRESS_D10)
/*************************************************
 * Name:        mlk_poly_compress_d10_native
 *
 * Description: Compression (10 bits) and subsequent serialization of a
 *              polynomial
 *
 * Arguments:   - uint8_t *r: pointer to output byte array
 *                   (of length MLKEM_POLYCOMPRESSEDBYTES_D10 bytes)
 *              - const int16_t a[MLKEM_N]: pointer to input polynomial
 *                  Coefficients must be unsigned canonical,
 *                  i.e. in [0,1,..,MLKEM_Q-1].
 **************************************************/
MLK_MUST_CHECK_RETURN_VALUE
static MLK_INLINE int mlk_poly_compress_d10_native(
    uint8_t r[MLKEM_POLYCOMPRESSEDBYTES_D10], const int16_t a[MLKEM_N])
__contract__(
  requires(memory_no_alias(r, MLKEM_POLYCOMPRESSEDBYTES_D10))
  requires(memory_no_alias(a, sizeof(int16_t) * MLKEM_N))
  requires(array_bound(a, 0, MLKEM_N, 0, MLKEM_Q))
  assigns(memory_slice(r, MLKEM_POLYCOMPRESSEDBYTES_D10))
  ensures(return_value == MLK_NATIVE_FUNC_SUCCESS || return_value == MLK_NATIVE_FUNC_FALLBACK));
#endif /* MLK_USE_NATIVE_POLY_COMPRESS_D10 */

#if defined(MLK_USE_NATIVE_POLY_DECOMPRESS_D4)
/*************************************************
 * Name:        mlk_poly_decompress_d4
 *
 * Description: De-serialization and subsequent decompression (dv bits) of a
 *              polynomial; approximate inverse of poly_compress
 *
 * Arguments:   - int16_t r[MLKEM_N]: pointer to output polynomial
 *              - const uint8_t *a: pointer to input byte array
 *                   (of length MLKEM_POLYCOMPRESSEDBYTES_D4 bytes)
 *
 * Upon return, the coefficients of the output polynomial are unsigned-canonical
 * (non-negative and smaller than MLKEM_Q).
 *
 **************************************************/
MLK_MUST_CHECK_RETURN_VALUE
static MLK_INLINE int mlk_poly_decompress_d4_native(
    int16_t r[MLKEM_N], const uint8_t a[MLKEM_POLYCOMPRESSEDBYTES_D4])
__contract__(
  requires(memory_no_alias(r, sizeof(int16_t) * MLKEM_N))
  requires(memory_no_alias(a, MLKEM_POLYCOMPRESSEDBYTES_D4))
  assigns(memory_slice(r, sizeof(int16_t) * MLKEM_N))
  ensures(return_value == MLK_NATIVE_FUNC_SUCCESS || return_value == MLK_NATIVE_FUNC_FALLBACK)
  ensures((return_value == MLK_NATIVE_FUNC_SUCCESS) ==> array_bound(r, 0, MLKEM_N, 0, MLKEM_Q)));
#endif /* MLK_USE_NATIVE_POLY_DECOMPRESS_D4 */

#if defined(MLK_USE_NATIVE_POLY_DECOMPRESS_D10)
/*************************************************
 * Name:        mlk_poly_decompress_d10_native
 *
 * Description: De-serialization and subsequent decompression (10 bits) of a
 *              polynomial; approximate inverse of mlk_poly_compress_d10
 *
 * Arguments:   - int16_t r[MLKEM_N]: pointer to output polynomial
 *              - const uint8_t *a: pointer to input byte array
 *                   (of length MLKEM_POLYCOMPRESSEDBYTES_D10 bytes)
 *
 * Upon return, the coefficients of the output polynomial are unsigned-canonical
 * (non-negative and smaller than MLKEM_Q).
 *
 **************************************************/
MLK_MUST_CHECK_RETURN_VALUE
static MLK_INLINE int mlk_poly_decompress_d10_native(
    int16_t r[MLKEM_N], const uint8_t a[MLKEM_POLYCOMPRESSEDBYTES_D10])
__contract__(
  requires(memory_no_alias(r, sizeof(int16_t) * MLKEM_N))
  requires(memory_no_alias(a, MLKEM_POLYCOMPRESSEDBYTES_D10))
  assigns(memory_slice(r, sizeof(int16_t) * MLKEM_N))
  ensures(return_value == MLK_NATIVE_FUNC_SUCCESS || return_value == MLK_NATIVE_FUNC_FALLBACK)
  ensures((return_value == MLK_NATIVE_FUNC_SUCCESS) ==> array_bound(r, 0, MLKEM_N, 0, MLKEM_Q)));
#endif /* MLK_USE_NATIVE_POLY_DECOMPRESS_D10 */
#endif /* MLK_CONFIG_MULTILEVEL_WITH_SHARED || MLKEM_K == 2 || MLKEM_K == 3 */

#if defined(MLK_CONFIG_MULTILEVEL_WITH_SHARED) || MLKEM_K == 4
#if defined(MLK_USE_NATIVE_POLY_COMPRESS_D5)
/*************************************************
 * Name:        mlk_poly_compress_d5_native
 *
 * Description: Compression (5 bits) and subsequent serialization of a
 *              polynomial
 *
 * Arguments:   - uint8_t *r: pointer to output byte array
 *                   (of length MLKEM_POLYCOMPRESSEDBYTES_D5 bytes)
 *              - const int16_t a[MLKEM_N]: pointer to input polynomial
 *                  Coefficients must be unsigned canonical,
 *                  i.e. in [0,1,..,MLKEM_Q-1].
 **************************************************/
MLK_MUST_CHECK_RETURN_VALUE
static MLK_INLINE int mlk_poly_compress_d5_native(
    uint8_t r[MLKEM_POLYCOMPRESSEDBYTES_D5], const int16_t a[MLKEM_N])
__contract__(
  requires(memory_no_alias(r, MLKEM_POLYCOMPRESSEDBYTES_D5))
  requires(memory_no_alias(a, sizeof(int16_t) * MLKEM_N))
  requires(array_bound(a, 0, MLKEM_N, 0, MLKEM_Q))
  assigns(memory_slice(r, MLKEM_POLYCOMPRESSEDBYTES_D5))
  ensures(return_value == MLK_NATIVE_FUNC_SUCCESS || return_value == MLK_NATIVE_FUNC_FALLBACK));
#endif /* MLK_USE_NATIVE_POLY_COMPRESS_D5 */

#if defined(MLK_USE_NATIVE_POLY_COMPRESS_D11)
/*************************************************
 * Name:        mlk_poly_compress_d11_native
 *
 * Description: Compression (11 bits) and subsequent serialization of a
 *              polynomial
 *
 * Arguments:   - uint8_t *r: pointer to output byte array
 *                   (of length MLKEM_POLYCOMPRESSEDBYTES_D11 bytes)
 *              - const int16_t a[MLKEM_N]: pointer to input polynomial
 *                  Coefficients must be unsigned canonical,
 *                  i.e. in [0,1,..,MLKEM_Q-1].
 **************************************************/
MLK_MUST_CHECK_RETURN_VALUE
static MLK_INLINE int mlk_poly_compress_d11_native(
    uint8_t r[MLKEM_POLYCOMPRESSEDBYTES_D11], const int16_t a[MLKEM_N])
__contract__(
  requires(memory_no_alias(r, MLKEM_POLYCOMPRESSEDBYTES_D11))
  requires(memory_no_alias(a, sizeof(int16_t) * MLKEM_N))
  requires(array_bound(a, 0, MLKEM_N, 0, MLKEM_Q))
  assigns(memory_slice(r, MLKEM_POLYCOMPRESSEDBYTES_D11))
  ensures(return_value == MLK_NATIVE_FUNC_SUCCESS || return_value == MLK_NATIVE_FUNC_FALLBACK));
#endif /* MLK_USE_NATIVE_POLY_COMPRESS_D11 */

#if defined(MLK_USE_NATIVE_POLY_DECOMPRESS_D5)
/*************************************************
 * Name:        mlk_poly_decompress_d5_native
 *
 * Description: De-serialization and subsequent decompression (dv bits) of a
 *              polynomial; approximate inverse of poly_compress
 *
 * Arguments:   - int16_t r[MLKEM_N]: pointer to output polynomial
 *              - const uint8_t *a: pointer to input byte array
 *                   (of length MLKEM_POLYCOMPRESSEDBYTES_D5 bytes)
 *
 * Upon return, the coefficients of the output polynomial are unsigned-canonical
 * (non-negative and smaller than MLKEM_Q).
 *
 **************************************************/
MLK_MUST_CHECK_RETURN_VALUE
static MLK_INLINE int mlk_poly_decompress_d5_native(
    int16_t r[MLKEM_N], const uint8_t a[MLKEM_POLYCOMPRESSEDBYTES_D5])
__contract__(
  requires(memory_no_alias(r, sizeof(int16_t) * MLKEM_N))
  requires(memory_no_alias(a, MLKEM_POLYCOMPRESSEDBYTES_D5))
  assigns(memory_slice(r, sizeof(int16_t) * MLKEM_N))
  ensures(return_value == MLK_NATIVE_FUNC_SUCCESS || return_value == MLK_NATIVE_FUNC_FALLBACK)
  ensures((return_value == MLK_NATIVE_FUNC_SUCCESS) ==> array_bound(r, 0, MLKEM_N, 0, MLKEM_Q)));
#endif /* MLK_USE_NATIVE_POLY_DECOMPRESS_D5 */

#if defined(MLK_USE_NATIVE_POLY_DECOMPRESS_D11)
/*************************************************
 * Name:        mlk_poly_decompress_d11_native
 *
 * Description: De-serialization and subsequent decompression (11 bits) of a
 *              polynomial; approximate inverse of mlk_poly_compress_d11
 *
 * Arguments:   - int16_t r[MLKEM_N]: pointer to output polynomial
 *              - const uint8_t *a: pointer to input byte array
 *                   (of length MLKEM_POLYCOMPRESSEDBYTES_D11 bytes)
 *
 * Upon return, the coefficients of the output polynomial are unsigned-canonical
 * (non-negative and smaller than MLKEM_Q).
 *
 **************************************************/
MLK_MUST_CHECK_RETURN_VALUE
static MLK_INLINE int mlk_poly_decompress_d11_native(
    int16_t r[MLKEM_N], const uint8_t a[MLKEM_POLYCOMPRESSEDBYTES_D11])
__contract__(
  requires(memory_no_alias(r, sizeof(int16_t) * MLKEM_N))
  requires(memory_no_alias(a, MLKEM_POLYCOMPRESSEDBYTES_D11))
  assigns(memory_slice(r, sizeof(int16_t) * MLKEM_N))
  ensures(return_value == MLK_NATIVE_FUNC_SUCCESS || return_value == MLK_NATIVE_FUNC_FALLBACK)
  ensures((return_value == MLK_NATIVE_FUNC_SUCCESS) ==> array_bound(r, 0, MLKEM_N, 0, MLKEM_Q)));
#endif /* MLK_USE_NATIVE_POLY_DECOMPRESS_D11 */
#endif /* MLK_CONFIG_MULTILEVEL_WITH_SHARED || MLKEM_K == 4 */

#endif /* !MLK_NATIVE_API_H */
