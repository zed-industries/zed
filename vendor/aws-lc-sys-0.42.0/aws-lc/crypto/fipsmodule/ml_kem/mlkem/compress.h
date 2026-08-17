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
 *
 * - [REF]
 *   CRYSTALS-Kyber C reference implementation
 *   Bos, Ducas, Kiltz, Lepoint, Lyubashevsky, Schanck, Schwabe, Seiler, Stehlé
 *   https://github.com/pq-crystals/kyber/tree/main/ref
 */

#ifndef MLK_COMPRESS_H
#define MLK_COMPRESS_H


#include "cbmc.h"
#include "common.h"
#include "debug.h"
#include "poly.h"
#include "verify.h"

/************************************************************
 * Name: mlk_scalar_compress_d1
 *
 * Description: Computes round(u * 2 / q)
 *
 * Arguments: - u: Unsigned canonical modulus modulo q
 *                 to be compressed.
 *
 * Specification: Compress_1 from @[FIPS203, Eq (4.7)].
 *
 ************************************************************/

/*
 * The multiplication in this routine will exceed UINT32_MAX
 * and wrap around for large values of u. This is expected and required.
 */
#ifdef CBMC
#pragma CPROVER check push
#pragma CPROVER check disable "unsigned-overflow"
#endif

/* Reference: Part of poly_tomsg() in the reference implementation @[REF]. */
static MLK_INLINE uint8_t mlk_scalar_compress_d1(int16_t u)
__contract__(
  requires(0 <= u && u <= MLKEM_Q - 1)
  ensures(return_value < 2)
  ensures(return_value == (((uint32_t)u * 2 + MLKEM_Q / 2) / MLKEM_Q) % 2)  )
{
  /* Compute as follows:
   * ```
   * round(u * 2 / MLKEM_Q)
   *   = round(u * 2 * (2^31 / MLKEM_Q) / 2^31)
   *  ~= round(u * 2 * round(2^31 / MLKEM_Q) / 2^31)
   * ```
   */
  /* check-magic: 1290168 == 2*round(2^31 / MLKEM_Q) */
  uint32_t d0 = (uint32_t)u * 1290168;
  /* Unsigned shifting by 31 positions leaves only the top bit. */
  return (uint8_t)((d0 + ((uint32_t)1u << 30)) >> 31);
}
#ifdef CBMC
#pragma CPROVER check pop
#endif

/************************************************************
 * Name: mlk_scalar_compress_d4
 *
 * Description: Computes round(u * 16 / q) % 16
 *
 * Arguments: - u: Unsigned canonical modulus modulo q
 *                 to be compressed.
 *
 * Specification: Compress_4 from @[FIPS203, Eq (4.7)].
 *
 ************************************************************/
/*
 * The multiplication in this routine will exceed UINT32_MAX
 * and wrap around for large values of u. This is expected and required.
 */
#ifdef CBMC
#pragma CPROVER check push
#pragma CPROVER check disable "unsigned-overflow"
#endif

/* Reference: Embedded into `poly_compress()` in the
 *            reference implementation @[REF]. */
static MLK_INLINE uint8_t mlk_scalar_compress_d4(int16_t u)
__contract__(
  requires(0 <= u && u <= MLKEM_Q - 1)
  ensures(return_value < 16)
  ensures(return_value == (((uint32_t)u * 16 + MLKEM_Q / 2) / MLKEM_Q) % 16))
{
  /* Compute as follows:
   * ```
   * round(u * 16 / MLKEM_Q)
   *   = round(u * 16 * (2^28 / MLKEM_Q) / 2^28)
   *  ~= round(u * 16 * round(2^28 / MLKEM_Q) / 2^28)
   * ```
   */
  /* check-magic: 1290160 == 16 * round(2^28 / MLKEM_Q) */
  uint32_t d0 = (uint32_t)u * 1290160;
  /* The return value is < 16, so not altered by the conversion to uint8_t. */
  return (uint8_t)((d0 + ((uint32_t)1u << 27)) >> 28); /* round(d0/2^28) */
}
#ifdef CBMC
#pragma CPROVER check pop
#endif

/************************************************************
 * Name: mlk_scalar_decompress_d4
 *
 * Description: Computes round(u * q / 16)
 *
 * Arguments: - u: Unsigned canonical modulus modulo 16
 *                 to be decompressed.
 *
 * Specification: Decompress_4 from @[FIPS203, Eq (4.8)].
 *
 ************************************************************/

/* Reference: Embedded into `poly_decompress()` in the
 *            reference implementation @[REF]. */
static MLK_INLINE int16_t mlk_scalar_decompress_d4(uint8_t u)
__contract__(
  requires(0 <= u && u < 16)
  ensures(return_value <= (MLKEM_Q - 1))
)
{
  /* The return value is in 0..MLKEM_Q-1, hence not altered by the
   * conversion to int16_t. */
  return (int16_t)((((uint32_t)u * MLKEM_Q) + 8) >> 4);
}

/************************************************************
 * Name: mlk_scalar_compress_d5
 *
 * Description: Computes round(u * 32 / q) % 32
 *
 * Arguments: - u: Unsigned canonical modulus modulo q
 *                 to be compressed.
 *
 * Specification: Compress_5 from @[FIPS203, Eq (4.7)].
 *
 ************************************************************/
/*
 * The multiplication in this routine will exceed UINT32_MAX
 * and wrap around for large values of u. This is expected and required.
 */
#ifdef CBMC
#pragma CPROVER check push
#pragma CPROVER check disable "unsigned-overflow"
#endif

/* Reference: Embedded into `poly_compress()` in the
 *            reference implementation @[REF]. */
static MLK_INLINE uint8_t mlk_scalar_compress_d5(int16_t u)
__contract__(
  requires(0 <= u && u <= MLKEM_Q - 1)
  ensures(return_value < 32)
  ensures(return_value == (((uint32_t)u * 32 + MLKEM_Q / 2) / MLKEM_Q) % 32)  )
{
  /* Compute as follows:
   * ```
   * round(u * 32 / MLKEM_Q)
   *   = round(u * 32 * (2^27 / MLKEM_Q) / 2^27)
   *  ~= round(u * 32 * round(2^27 / MLKEM_Q) / 2^27)
   * ```
   */
  /* check-magic: 1290176 == 2^5 * round(2^27 / MLKEM_Q) */
  uint32_t d0 = (uint32_t)u * 1290176;
  /* The return value is < 32, so not altered by the conversion to uint8_t. */
  return (uint8_t)((d0 + ((uint32_t)1u << 26)) >> 27); /* round(d0/2^27) */
}
#ifdef CBMC
#pragma CPROVER check pop
#endif

/************************************************************
 * Name: mlk_scalar_decompress_d5
 *
 * Description: Computes round(u * q / 32)
 *
 * Arguments: - u: Unsigned canonical modulus modulo 32
 *                 to be decompressed.
 *
 * Specification: Decompress_5 from @[FIPS203, Eq (4.8)].
 *
 ************************************************************/

/* Reference: Embedded into `poly_decompress()` in the
 *            reference implementation @[REF]. */
static MLK_INLINE int16_t mlk_scalar_decompress_d5(uint8_t u)
__contract__(
  requires(0 <= u && u < 32)
  ensures(0 <= return_value && return_value <= MLKEM_Q - 1)
)
{
  /* The return value is in 0..MLKEM_Q-1, hence not altered by the
   * conversion to int16_t. */
  return (int16_t)((((uint32_t)u * MLKEM_Q) + 16) >> 5);
}

/************************************************************
 * Name: mlk_scalar_compress_d10
 *
 * Description: Computes round(u * 2**10 / q) % 2**10
 *
 * Arguments: - u: Unsigned canonical modulus modulo q
 *                 to be compressed.
 *
 * Specification: Compress_10 from @[FIPS203, Eq (4.7)].
 *
 ************************************************************/
/*
 * The multiplication in this routine will exceed UINT32_MAX
 * and wrap around for large values of u. This is expected and required.
 */
#ifdef CBMC
#pragma CPROVER check push
#pragma CPROVER check disable "unsigned-overflow"
#endif

/* Reference: Embedded into `polyvec_compress()` in the
 *            reference implementation @[REF]. */
static MLK_INLINE uint16_t mlk_scalar_compress_d10(int16_t u)
__contract__(
  requires(0 <= u && u <= MLKEM_Q - 1)
  ensures(return_value < (1u << 10))
  ensures(return_value == (((uint32_t)u * (1u << 10) + MLKEM_Q / 2) / MLKEM_Q) % (1 << 10)))
{
  /* Compute as follows:
   * ```
   * round(u * 1024 / MLKEM_Q)
   *   = round(u * 1024 * (2^33 / MLKEM_Q) / 2^33)
   *  ~= round(u * 1024 * round(2^33 / MLKEM_Q) / 2^33)
   * ```
   */
  /* check-magic: 2642263040 == 2^10 * round(2^33 / MLKEM_Q) */
  uint64_t d0 = (uint64_t)u * 2642263040;
  d0 = (d0 + ((uint64_t)1u << 32)) >> 33; /* round(d0/2^33) */
  return (d0 & 0x3FF);
}
#ifdef CBMC
#pragma CPROVER check pop
#endif

/************************************************************
 * Name: mlk_scalar_decompress_d10
 *
 * Description: Computes round(u * q / 1024)
 *
 * Arguments: - u: Unsigned canonical modulus modulo 1024
 *                 to be decompressed.
 *
 * Specification: Decompress_10 from @[FIPS203, Eq (4.8)].
 *
 ************************************************************/

/* Reference: Embedded into `polyvec_decompress()` in the
 *            reference implementation @[REF]. */
static MLK_INLINE int16_t mlk_scalar_decompress_d10(uint16_t u)
__contract__(
  requires(0 <= u && u < 1024)
  ensures(0 <= return_value && return_value <= (MLKEM_Q - 1))
)
{
  /* The return value is in 0..MLKEM_Q-1, hence not altered by the
   * conversion to int16_t. */
  return (int16_t)((((uint32_t)u * MLKEM_Q) + 512) >> 10);
}

/************************************************************
 * Name: mlk_scalar_compress_d11
 *
 * Description: Computes round(u * 2**11 / q) % 2**11
 *
 * Arguments: - u: Unsigned canonical modulus modulo q
 *                 to be compressed.
 *
 * Specification: Compress_11 from @[FIPS203, Eq (4.7)].
 *
 ************************************************************/
/*
 * The multiplication in this routine will exceed UINT32_MAX
 * and wrap around for large values of u. This is expected and required.
 */
#ifdef CBMC
#pragma CPROVER check push
#pragma CPROVER check disable "unsigned-overflow"
#endif

/* Reference: Embedded into `polyvec_compress()` in the
 *            reference implementation @[REF]. */
static MLK_INLINE uint16_t mlk_scalar_compress_d11(int16_t u)
__contract__(
  requires(0 <= u && u <= MLKEM_Q - 1)
  ensures(return_value < (1u << 11))
  ensures(return_value == (((uint32_t)u * (1u << 11) + MLKEM_Q / 2) / MLKEM_Q) % (1 << 11)))
{
  /* Compute as follows:
   * ```
   * round(u * 2048 / MLKEM_Q)
   *   = round(u * 2048 * (2^33 / MLKEM_Q) / 2^33)
   *  ~= round(u * 2048 * round(2^33 / MLKEM_Q) / 2^33)
   * ```
   */
  /* check-magic: 5284526080 == 2^11 * round(2^33 / MLKEM_Q) */
  uint64_t d0 = (uint64_t)u * 5284526080;
  d0 = (d0 + ((uint64_t)1u << 32)) >> 33; /* round(d0/2^33) */
  return (d0 & 0x7FF);
}
#ifdef CBMC
#pragma CPROVER check pop
#endif

/************************************************************
 * Name: mlk_scalar_decompress_d11
 *
 * Description: Computes round(u * q / 2048)
 *
 * Arguments: - u: Unsigned canonical modulus modulo 2048
 *                 to be decompressed.
 *
 * Specification: Decompress_11 from @[FIPS203, Eq (4.8)].
 *
 ************************************************************/

/* Reference: Embedded into `polyvec_decompress()` in the
 *            reference implementation @[REF]. */
static MLK_INLINE int16_t mlk_scalar_decompress_d11(uint16_t u)
__contract__(
  requires(0 <= u && u < 2048)
  ensures(0 <= return_value && return_value <= (MLKEM_Q - 1))
)
{
  /* The return value is in 0..MLKEM_Q-1, hence not altered by the
   * conversion to int16_t. */
  return (int16_t)((((uint32_t)u * MLKEM_Q) + 1024) >> 11);
}

#if defined(MLK_CONFIG_MULTILEVEL_WITH_SHARED) || (MLKEM_K == 2 || MLKEM_K == 3)
#define mlk_poly_compress_d4 MLK_NAMESPACE(poly_compress_d4)
/*************************************************
 * Name:        mlk_poly_compress_d4
 *
 * Description: Compression (4 bits) and subsequent serialization of a
 *              polynomial
 *
 * Arguments:   - uint8_t *r: pointer to output byte array
 *                   (of length MLKEM_POLYCOMPRESSEDBYTES_D4 bytes)
 *              - const mlk_poly *a: pointer to input polynomial
 *                  Coefficients must be unsigned canonical,
 *                  i.e. in [0,1,..,MLKEM_Q-1].
 *
 * Specification: Implements `ByteEncode_4 (Compress_4 (a))`:
 *                - ByteEncode_d: @[FIPS203, Algorithm 5],
 *                - Compress_d: @[FIPS203, Eq (4.7)]
 *                  Extended to vectors as per
 *                  @[FIPS203, 2.4.8 Applying Algorithms to Arrays]
 *                - `ByteEncode_{d_v} (Compress_{d_v} (v))` appears in
 *                  @[FIPS203, Algorithm 14 (K-PKE.Encrypt), L23],
 *                  where `d_v=4` for ML-KEM-{512,768} @[FIPS203, Table 2].
 *
 **************************************************/
MLK_INTERNAL_API
void mlk_poly_compress_d4(uint8_t r[MLKEM_POLYCOMPRESSEDBYTES_D4],
                          const mlk_poly *a);

#define mlk_poly_compress_d10 MLK_NAMESPACE(poly_compress_d10)
/*************************************************
 * Name:        mlk_poly_compress_d10
 *
 * Description: Compression (10 bits) and subsequent serialization of a
 *              polynomial
 *
 * Arguments:   - uint8_t *r: pointer to output byte array
 *                   (of length MLKEM_POLYCOMPRESSEDBYTES_D10 bytes)
 *              - const mlk_poly *a: pointer to input polynomial
 *                  Coefficients must be unsigned canonical,
 *                  i.e. in [0,1,..,MLKEM_Q-1].
 *
 * Specification: Implements `ByteEncode_10 (Compress_10 (a))`:
 *                - ByteEncode_d: @[FIPS203, Algorithm 5],
 *                - Compress_d: @[FIPS203, Eq (4.7)]
 *                  Extended to vectors as per
 *                  @[FIPS203, 2.4.8 Applying Algorithms to Arrays]
 *                - `ByteEncode_{d_u} (Compress_{d_u} (u))` appears in
 *                  @[FIPS203, Algorithm 14 (K-PKE.Encrypt), L22],
 *                  where `d_u=10` for ML-KEM-{512,768} @[FIPS203, Table 2].
 *
 **************************************************/
MLK_INTERNAL_API
void mlk_poly_compress_d10(uint8_t r[MLKEM_POLYCOMPRESSEDBYTES_D10],
                           const mlk_poly *a);

#define mlk_poly_decompress_d4 MLK_NAMESPACE(poly_decompress_d4)
/*************************************************
 * Name:        mlk_poly_decompress_d4
 *
 * Description: De-serialization and subsequent decompression (dv bits) of a
 *              polynomial; approximate inverse of poly_compress
 *
 * Arguments:   - mlk_poly *r: pointer to output polynomial
 *              - const uint8_t *a: pointer to input byte array
 *                   (of length MLKEM_POLYCOMPRESSEDBYTES_D4 bytes)
 *
 * Upon return, the coefficients of the output polynomial are unsigned-canonical
 * (non-negative and smaller than MLKEM_Q).
 *
 * Specification: Implements `Decompress_4 (ByteDecode_4 (a))`:
 *                - ByteDecode_d: @[FIPS203, Algorithm 6],
 *                - Decompress_d: @[FIPS203, Eq (4.8)]
 *                  Extended to vectors as per
 *                  @[FIPS203, 2.4.8 Applying Algorithms to Arrays]
 *                - `Decompress_{d_v} (ByteDecode_{d_v} (v))` appears in
 *                  @[FIPS203, Algorithm 15 (K-PKE.Decrypt), L4],
 *                  where `d_v=4` for ML-KEM-{512,768} @[FIPS203, Table 2].
 *
 **************************************************/
MLK_INTERNAL_API
void mlk_poly_decompress_d4(mlk_poly *r,
                            const uint8_t a[MLKEM_POLYCOMPRESSEDBYTES_D4]);

#define mlk_poly_decompress_d10 MLK_NAMESPACE(poly_decompress_d10)
/*************************************************
 * Name:        mlk_poly_decompress_d10
 *
 * Description: De-serialization and subsequent decompression (10 bits) of a
 *              polynomial; approximate inverse of mlk_poly_compress_d10
 *
 * Arguments:   - mlk_poly *r: pointer to output polynomial
 *              - const uint8_t *a: pointer to input byte array
 *                   (of length MLKEM_POLYCOMPRESSEDBYTES_D10 bytes)
 *
 * Upon return, the coefficients of the output polynomial are unsigned-canonical
 * (non-negative and smaller than MLKEM_Q).
 *
 * Specification: Implements `Decompress_10 (ByteDecode_10 (a))`:
 *                - ByteDecode_d: @[FIPS203, Algorithm 6],
 *                - Decompress_d: @[FIPS203, Eq (4.8)]
 *                  Extended to vectors as per
 *                  @[FIPS203, 2.4.8 Applying Algorithms to Arrays]
 *                - `Decompress_{d_u} (ByteDecode_{d_u} (u))` appears in
 *                  @[FIPS203, Algorithm 15 (K-PKE.Decrypt), L3],
 *                  where `d_u=10` for ML-KEM-{512,768} @[FIPS203, Table 2].
 *
 **************************************************/
MLK_INTERNAL_API
void mlk_poly_decompress_d10(mlk_poly *r,
                             const uint8_t a[MLKEM_POLYCOMPRESSEDBYTES_D10]);
#endif /* MLK_CONFIG_MULTILEVEL_WITH_SHARED || MLKEM_K == 2 || MLKEM_K == 3 */

#if defined(MLK_CONFIG_MULTILEVEL_WITH_SHARED) || MLKEM_K == 4
#define mlk_poly_compress_d5 MLK_NAMESPACE(poly_compress_d5)
/*************************************************
 * Name:        mlk_poly_compress_d5
 *
 * Description: Compression (5 bits) and subsequent serialization of a
 *              polynomial
 *
 * Arguments:   - uint8_t *r: pointer to output byte array
 *                   (of length MLKEM_POLYCOMPRESSEDBYTES_D5 bytes)
 *              - const mlk_poly *a: pointer to input polynomial
 *                  Coefficients must be unsigned canonical,
 *                  i.e. in [0,1,..,MLKEM_Q-1].
 *
 * Specification: Implements `ByteEncode_5 (Compress_5 (a))`:
 *                - ByteEncode_d: @[FIPS203, Algorithm 5],
 *                - Compress_d: @[FIPS203, Eq (4.7)]
 *                  Extended to vectors as per
 *                  @[FIPS203, 2.4.8 Applying Algorithms to Arrays]
 *                - `ByteEncode_{d_v} (Compress_{d_v} (v))` appears in
 *                  @[FIPS203, Algorithm 14 (K-PKE.Encrypt), L23],
 *                  where `d_v=5` for ML-KEM-1024 @[FIPS203, Table 2].
 *
 **************************************************/
MLK_INTERNAL_API
void mlk_poly_compress_d5(uint8_t r[MLKEM_POLYCOMPRESSEDBYTES_D5],
                          const mlk_poly *a);

#define mlk_poly_compress_d11 MLK_NAMESPACE(poly_compress_d11)
/*************************************************
 * Name:        mlk_poly_compress_d11
 *
 * Description: Compression (11 bits) and subsequent serialization of a
 *              polynomial
 *
 * Arguments:   - uint8_t *r: pointer to output byte array
 *                   (of length MLKEM_POLYCOMPRESSEDBYTES_D11 bytes)
 *              - const mlk_poly *a: pointer to input polynomial
 *                  Coefficients must be unsigned canonical,
 *                  i.e. in [0,1,..,MLKEM_Q-1].
 *
 * Specification: `ByteEncode_11 (Compress_11 (a))`:
 *                - ByteEncode_d: @[FIPS203, Algorithm 5],
 *                - Compress_d: @[FIPS203, Eq (4.7)]
 *                  Extended to vectors as per
 *                  @[FIPS203, 2.4.8 Applying Algorithms to Arrays]
 *                - `ByteEncode_{d_u} (Compress_{d_u} (u))` appears in
 *                  @[FIPS203, Algorithm 14 (K-PKE.Encrypt), L22],
 *                  where `d_u=11` for ML-KEM-1024 @[FIPS203, Table 2].
 *
 **************************************************/
MLK_INTERNAL_API
void mlk_poly_compress_d11(uint8_t r[MLKEM_POLYCOMPRESSEDBYTES_D11],
                           const mlk_poly *a);

#define mlk_poly_decompress_d5 MLK_NAMESPACE(poly_decompress_d5)
/*************************************************
 * Name:        mlk_poly_decompress_d5
 *
 * Description: De-serialization and subsequent decompression (dv bits) of a
 *              polynomial; approximate inverse of poly_compress
 *
 * Arguments:   - mlk_poly *r: pointer to output polynomial
 *              - const uint8_t *a: pointer to input byte array
 *                   (of length MLKEM_POLYCOMPRESSEDBYTES_D5 bytes)
 *
 * Upon return, the coefficients of the output polynomial are unsigned-canonical
 * (non-negative and smaller than MLKEM_Q).
 *
 * Specification: Implements `Decompress_5 (ByteDecode_5 (a))`:
 *                - ByteDecode_d: @[FIPS203, Algorithm 6],
 *                - Decompress_d: @[FIPS203, Eq (4.8)]
 *                  Extended to vectors as per
 *                  @[FIPS203, 2.4.8 Applying Algorithms to Arrays]
 *                - `Decompress_{d_v} (ByteDecode_{d_v} (v))` appears in
 *                  @[FIPS203, Algorithm 15 (K-PKE.Decrypt), L4],
 *                  where `d_v=5` for ML-KEM-1024 @[FIPS203, Table 2].
 *
 **************************************************/
MLK_INTERNAL_API
void mlk_poly_decompress_d5(mlk_poly *r,
                            const uint8_t a[MLKEM_POLYCOMPRESSEDBYTES_D5]);

#define mlk_poly_decompress_d11 MLK_NAMESPACE(poly_decompress_d11)
/*************************************************
 * Name:        mlk_poly_decompress_d11
 *
 * Description: De-serialization and subsequent decompression (11 bits) of a
 *              polynomial; approximate inverse of mlk_poly_compress_d11
 *
 * Arguments:   - mlk_poly *r: pointer to output polynomial
 *              - const uint8_t *a: pointer to input byte array
 *                   (of length MLKEM_POLYCOMPRESSEDBYTES_D11 bytes)
 *
 * Upon return, the coefficients of the output polynomial are unsigned-canonical
 * (non-negative and smaller than MLKEM_Q).
 *
 * Specification: Implements `Decompress_11 (ByteDecode_11 (a))`:
 *                - ByteDecode_d: @[FIPS203, Algorithm 6],
 *                - Decompress_d: @[FIPS203, Eq (4.8)]
 *                  Extended to vectors as per
 *                  @[FIPS203, 2.4.8 Applying Algorithms to Arrays]
 *                - `Decompress_{d_u} (ByteDecode_{d_u} (u))` appears in
 *                  @[FIPS203, Algorithm 15 (K-PKE.Decrypt), L3],
 *                  where `d_u=11` for ML-KEM-1024 @[FIPS203, Table 2].
 *
 **************************************************/
MLK_INTERNAL_API
void mlk_poly_decompress_d11(mlk_poly *r,
                             const uint8_t a[MLKEM_POLYCOMPRESSEDBYTES_D11]);
#endif /* MLK_CONFIG_MULTILEVEL_WITH_SHARED || MLKEM_K == 4 */

#define mlk_poly_tobytes MLK_NAMESPACE(poly_tobytes)
/*************************************************
 * Name:        mlk_poly_tobytes
 *
 * Description: Serialization of a polynomial.
 *              Signed coefficients are converted to
 *              unsigned form before serialization.
 *
 * Arguments:   INPUT:
 *              - a: const pointer to input polynomial,
 *                with each coefficient in the range [0,1,..,Q-1]
 *              OUTPUT
 *              - r: pointer to output byte array
 *                   (of MLKEM_POLYBYTES bytes)
 *
 * Specification: Implements ByteEncode_12 @[FIPS203, Algorithm 5].
 *                Extended to vectors as per
 *                @[FIPS203, 2.4.8 Applying Algorithms to Arrays]
 *
 **************************************************/
MLK_INTERNAL_API
void mlk_poly_tobytes(uint8_t r[MLKEM_POLYBYTES], const mlk_poly *a)
__contract__(
  requires(memory_no_alias(r, MLKEM_POLYBYTES))
  requires(memory_no_alias(a, sizeof(mlk_poly)))
  requires(array_bound(a->coeffs, 0, MLKEM_N, 0, MLKEM_Q))
  assigns(memory_slice(r, MLKEM_POLYBYTES))
);


#define mlk_poly_frombytes MLK_NAMESPACE(poly_frombytes)
/*************************************************
 * Name:        mlk_poly_frombytes
 *
 * Description: De-serialization of a polynomial.
 *
 * Arguments:   INPUT
 *              - a: pointer to input byte array
 *                   (of MLKEM_POLYBYTES bytes)
 *              OUTPUT
 *              - r: pointer to output polynomial, with
 *                   each coefficient unsigned and in the range
 *                   0 .. 4095
 *
 * Specification: Implements ByteDecode_12 @[FIPS203, Algorithm 6].
 *                Extended to vectors as per
 *                @[FIPS203, 2.4.8 Applying Algorithms to Arrays]
 *
 **************************************************/
MLK_INTERNAL_API
void mlk_poly_frombytes(mlk_poly *r, const uint8_t a[MLKEM_POLYBYTES])
__contract__(
  requires(memory_no_alias(a, MLKEM_POLYBYTES))
  requires(memory_no_alias(r, sizeof(mlk_poly)))
  assigns(memory_slice(r, sizeof(mlk_poly)))
  ensures(array_bound(r->coeffs, 0, MLKEM_N, 0, MLKEM_UINT12_LIMIT))
);


#define mlk_poly_frommsg MLK_NAMESPACE(poly_frommsg)
/*************************************************
 * Name:        mlk_poly_frommsg
 *
 * Description: Convert 32-byte message to polynomial
 *
 * Arguments:   - mlk_poly *r: pointer to output polynomial
 *              - const uint8_t *msg: pointer to input message
 *
 * Specification: Implements `Decompress_1 (ByteDecode_1 (a))`:
 *                - ByteDecode_d: @[FIPS203, Algorithm 6],
 *                - Decompress_d: @[FIPS203, Eq (4.8)]
 *                  Extended to vectors as per
 *                  @[FIPS203, 2.4.8 Applying Algorithms to Arrays]
 *                - `Decompress_1 (ByteDecode_1 (w))` appears in
 *                  @[FIPS203, Algorithm 15 (K-PKE.Encrypt), L20].
 *
 **************************************************/
MLK_INTERNAL_API
void mlk_poly_frommsg(mlk_poly *r, const uint8_t msg[MLKEM_INDCPA_MSGBYTES])
__contract__(
  requires(memory_no_alias(msg, MLKEM_INDCPA_MSGBYTES))
  requires(memory_no_alias(r, sizeof(mlk_poly)))
  assigns(memory_slice(r, sizeof(mlk_poly)))
  ensures(array_bound(r->coeffs, 0, MLKEM_N, 0, MLKEM_Q))
);

#define mlk_poly_tomsg MLK_NAMESPACE(poly_tomsg)
/*************************************************
 * Name:        mlk_poly_tomsg
 *
 * Description: Convert polynomial to 32-byte message
 *
 * Arguments:   - uint8_t *msg: pointer to output message
 *              - const mlk_poly *r: pointer to input polynomial
 *                Coefficients must be unsigned canonical
 *
 * Specification: Implements `ByteEncode_1 (Compress_1 (a))`:
 *                - ByteEncode_d: @[FIPS203, Algorithm 5],
 *                - Compress_d: @[FIPS203, Eq (4.7)]
 *                  Extended to vectors as per
 *                  @[FIPS203, 2.4.8 Applying Algorithms to Arrays]
 *                - `ByteEncode_1 (Compress_1 (w))` appears in
 *                  @[FIPS203, Algorithm 14 (K-PKE.Decrypt), L7].
 *
 **************************************************/
MLK_INTERNAL_API
void mlk_poly_tomsg(uint8_t msg[MLKEM_INDCPA_MSGBYTES], const mlk_poly *r)
__contract__(
  requires(memory_no_alias(msg, MLKEM_INDCPA_MSGBYTES))
  requires(memory_no_alias(r, sizeof(mlk_poly)))
  requires(array_bound(r->coeffs, 0, MLKEM_N, 0, MLKEM_Q))
  assigns(memory_slice(msg, MLKEM_INDCPA_MSGBYTES))
);

#endif /* !MLK_COMPRESS_H */
