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

#include "common.h"
#if !defined(MLK_CONFIG_MULTILEVEL_NO_SHARED)


#include "cbmc.h"
#include "compress.h"
#include "debug.h"
#include "verify.h"

#if defined(MLK_CONFIG_MULTILEVEL_WITH_SHARED) || (MLKEM_K == 2 || MLKEM_K == 3)
/* Reference: `poly_compress()` in the reference implementation @[REF],
 *            for ML-KEM-{512,768}.
 *            - In contrast to the reference implementation, we assume
 *              unsigned canonical coefficients here.
 *              The reference implementation works with coefficients
 *              in the range (-MLKEM_Q+1,...,MLKEM_Q-1). */
MLK_STATIC_TESTABLE void mlk_poly_compress_d4_c(
    uint8_t r[MLKEM_POLYCOMPRESSEDBYTES_D4], const mlk_poly *a)
__contract__(
  requires(memory_no_alias(r, MLKEM_POLYCOMPRESSEDBYTES_D4))
  requires(memory_no_alias(a, sizeof(mlk_poly)))
  requires(array_bound(a->coeffs, 0, MLKEM_N, 0, MLKEM_Q))
  assigns(memory_slice(r, MLKEM_POLYCOMPRESSEDBYTES_D4))
)
{
  unsigned i;
  mlk_assert_bound(a, MLKEM_N, 0, MLKEM_Q);

  for (i = 0; i < MLKEM_N / 8; i++)
  __loop__(invariant(i <= MLKEM_N / 8))
  {
    unsigned j;
    uint8_t t[8] = {0};
    for (j = 0; j < 8; j++)
    __loop__(
      invariant(i <= MLKEM_N / 8 && j <= 8)
      invariant(array_bound(t, 0, j, 0, 16)))
    {
      t[j] = mlk_scalar_compress_d4(a->coeffs[8 * i + j]);
    }

    /* All t[i] are 4-bit wide, so the truncations don't alter the value. */
    r[i * 4] = (uint8_t)(t[0] | (t[1] << 4));
    r[i * 4 + 1] = (uint8_t)(t[2] | (t[3] << 4));
    r[i * 4 + 2] = (uint8_t)(t[4] | (t[5] << 4));
    r[i * 4 + 3] = (uint8_t)(t[6] | (t[7] << 4));
  }
}

MLK_INTERNAL_API
void mlk_poly_compress_d4(uint8_t r[MLKEM_POLYCOMPRESSEDBYTES_D4],
                          const mlk_poly *a)
__contract__(
  requires(memory_no_alias(r, MLKEM_POLYCOMPRESSEDBYTES_D4))
  requires(memory_no_alias(a, sizeof(mlk_poly)))
  requires(array_bound(a->coeffs, 0, MLKEM_N, 0, MLKEM_Q))
  assigns(memory_slice(r, MLKEM_POLYCOMPRESSEDBYTES_D4))
)
{
#if defined(MLK_USE_NATIVE_POLY_COMPRESS_D4)
  int ret;
  mlk_assert_bound(a, MLKEM_N, 0, MLKEM_Q);
  ret = mlk_poly_compress_d4_native(r, a->coeffs);
  if (ret == MLK_NATIVE_FUNC_SUCCESS)
  {
    return;
  }
#endif /* MLK_USE_NATIVE_POLY_COMPRESS_D4 */

  mlk_poly_compress_d4_c(r, a);
}

/* Reference: Embedded into `polyvec_compress()` in the
 *            reference implementation, for ML-KEM-{512,768}.
 *            - In contrast to the reference implementation, we assume
 *              unsigned canonical coefficients here.
 *              The reference implementation works with coefficients
 *              in the range (-MLKEM_Q+1,...,MLKEM_Q-1). */
MLK_STATIC_TESTABLE void mlk_poly_compress_d10_c(
    uint8_t r[MLKEM_POLYCOMPRESSEDBYTES_D10], const mlk_poly *a)
__contract__(
  requires(memory_no_alias(r, MLKEM_POLYCOMPRESSEDBYTES_D10))
  requires(memory_no_alias(a, sizeof(mlk_poly)))
  requires(array_bound(a->coeffs, 0, MLKEM_N, 0, MLKEM_Q))
  assigns(memory_slice(r, MLKEM_POLYCOMPRESSEDBYTES_D10))
)
{
  unsigned j;
  mlk_assert_bound(a, MLKEM_N, 0, MLKEM_Q);
  for (j = 0; j < MLKEM_N / 4; j++)
  __loop__(invariant(j <= MLKEM_N / 4))
  {
    unsigned k;
    uint16_t t[4];
    for (k = 0; k < 4; k++)
    __loop__(
      invariant(k <= 4)
      invariant(forall(r, 0, k, t[r] < (1u << 10))))
    {
      t[k] = mlk_scalar_compress_d10(a->coeffs[4 * j + k]);
    }

    /*
     * Make all implicit truncation explicit. No data is being
     * truncated for the LHS's since each t[i] is 10-bit in size.
     */
    r[5 * j + 0] = (uint8_t)((t[0] >> 0) & 0xFF);
    r[5 * j + 1] = (uint8_t)((t[0] >> 8) | ((t[1] << 2) & 0xFF));
    r[5 * j + 2] = (uint8_t)((t[1] >> 6) | ((t[2] << 4) & 0xFF));
    r[5 * j + 3] = (uint8_t)((t[2] >> 4) | ((t[3] << 6) & 0xFF));
    r[5 * j + 4] = (uint8_t)(t[3] >> 2);
  }
}

MLK_INTERNAL_API
void mlk_poly_compress_d10(uint8_t r[MLKEM_POLYCOMPRESSEDBYTES_D10],
                           const mlk_poly *a)
__contract__(
  requires(memory_no_alias(r, MLKEM_POLYCOMPRESSEDBYTES_D10))
  requires(memory_no_alias(a, sizeof(mlk_poly)))
  requires(array_bound(a->coeffs, 0, MLKEM_N, 0, MLKEM_Q))
  assigns(memory_slice(r, MLKEM_POLYCOMPRESSEDBYTES_D10))
)
{
#if defined(MLK_USE_NATIVE_POLY_COMPRESS_D10)
  int ret;
  mlk_assert_bound(a, MLKEM_N, 0, MLKEM_Q);
  ret = mlk_poly_compress_d10_native(r, a->coeffs);
  if (ret == MLK_NATIVE_FUNC_SUCCESS)
  {
    return;
  }
#endif /* MLK_USE_NATIVE_POLY_COMPRESS_D10 */

  mlk_poly_compress_d10_c(r, a);
}

/* Reference: `poly_decompress()` in the reference implementation @[REF],
 *            for ML-KEM-{512,768}. */
MLK_STATIC_TESTABLE void mlk_poly_decompress_d4_c(
    mlk_poly *r, const uint8_t a[MLKEM_POLYCOMPRESSEDBYTES_D4])
__contract__(
  requires(memory_no_alias(r, sizeof(mlk_poly)))
  requires(memory_no_alias(a, MLKEM_POLYCOMPRESSEDBYTES_D4))
  assigns(memory_slice(r, sizeof(mlk_poly)))
  ensures(array_bound(r->coeffs, 0, MLKEM_N, 0, MLKEM_Q))
)
{
  unsigned i;
  for (i = 0; i < MLKEM_N / 2; i++)
  __loop__(
    invariant(i <= MLKEM_N / 2)
    invariant(array_bound(r->coeffs, 0, 2 * i, 0, MLKEM_Q)))
  {
    r->coeffs[2 * i + 0] = mlk_scalar_decompress_d4((a[i] >> 0) & 0xF);
    r->coeffs[2 * i + 1] = mlk_scalar_decompress_d4((a[i] >> 4) & 0xF);
  }

  mlk_assert_bound(r, MLKEM_N, 0, MLKEM_Q);
}

MLK_INTERNAL_API
void mlk_poly_decompress_d4(mlk_poly *r,
                            const uint8_t a[MLKEM_POLYCOMPRESSEDBYTES_D4])
__contract__(
  requires(memory_no_alias(r, sizeof(mlk_poly)))
  requires(memory_no_alias(a, MLKEM_POLYCOMPRESSEDBYTES_D4))
  assigns(memory_slice(r, sizeof(mlk_poly)))
  ensures(array_bound(r->coeffs, 0, MLKEM_N, 0, MLKEM_Q))
)
{
#if defined(MLK_USE_NATIVE_POLY_DECOMPRESS_D4)
  int ret;
  ret = mlk_poly_decompress_d4_native(r->coeffs, a);
  if (ret == MLK_NATIVE_FUNC_SUCCESS)
  {
    mlk_assert_bound(r, MLKEM_N, 0, MLKEM_Q);
    return;
  }
#endif /* MLK_USE_NATIVE_POLY_DECOMPRESS_D4 */

  mlk_poly_decompress_d4_c(r, a);
}

/* Reference: Embedded into `polyvec_decompress()` in the
 *            reference implementation, for ML-KEM-{512,768}. */
MLK_STATIC_TESTABLE void mlk_poly_decompress_d10_c(
    mlk_poly *r, const uint8_t a[MLKEM_POLYCOMPRESSEDBYTES_D10])
__contract__(
  requires(memory_no_alias(r, sizeof(mlk_poly)))
  requires(memory_no_alias(a, MLKEM_POLYCOMPRESSEDBYTES_D10))
  assigns(memory_slice(r, sizeof(mlk_poly)))
  ensures(array_bound(r->coeffs, 0, MLKEM_N, 0, MLKEM_Q))
)
{
  unsigned j;
  for (j = 0; j < MLKEM_N / 4; j++)
  __loop__(
    invariant(j <= MLKEM_N / 4)
    invariant(array_bound(r->coeffs, 0, 4 * j, 0, MLKEM_Q)))
  {
    unsigned k;
    uint16_t t[4];
    uint8_t const *base = &a[5 * j];

    t[0] = 0x3FF & ((base[0] >> 0) | ((uint16_t)base[1] << 8));
    t[1] = 0x3FF & ((base[1] >> 2) | ((uint16_t)base[2] << 6));
    t[2] = 0x3FF & ((base[2] >> 4) | ((uint16_t)base[3] << 4));
    t[3] = 0x3FF & ((base[3] >> 6) | ((uint16_t)base[4] << 2));

    for (k = 0; k < 4; k++)
    __loop__(
      invariant(k <= 4)
      invariant(array_bound(r->coeffs, 0, 4 * j + k, 0, MLKEM_Q)))
    {
      r->coeffs[4 * j + k] = mlk_scalar_decompress_d10(t[k]);
    }
  }

  mlk_assert_bound(r, MLKEM_N, 0, MLKEM_Q);
}

MLK_INTERNAL_API
void mlk_poly_decompress_d10(mlk_poly *r,
                             const uint8_t a[MLKEM_POLYCOMPRESSEDBYTES_D10])
__contract__(
  requires(memory_no_alias(r, sizeof(mlk_poly)))
  requires(memory_no_alias(a, MLKEM_POLYCOMPRESSEDBYTES_D10))
  assigns(memory_slice(r, sizeof(mlk_poly)))
  ensures(array_bound(r->coeffs, 0, MLKEM_N, 0, MLKEM_Q))
)
{
#if defined(MLK_USE_NATIVE_POLY_DECOMPRESS_D10)
  int ret;
  ret = mlk_poly_decompress_d10_native(r->coeffs, a);
  if (ret == MLK_NATIVE_FUNC_SUCCESS)
  {
    mlk_assert_bound(r, MLKEM_N, 0, MLKEM_Q);
    return;
  }
#endif /* MLK_USE_NATIVE_POLY_DECOMPRESS_D10 */

  mlk_poly_decompress_d10_c(r, a);
}
#endif /* MLK_CONFIG_MULTILEVEL_WITH_SHARED || MLKEM_K == 2 || MLKEM_K == 3 */

#if defined(MLK_CONFIG_MULTILEVEL_WITH_SHARED) || MLKEM_K == 4
/* Reference: `poly_compress()` in the reference implementation @[REF],
 *            for ML-KEM-1024.
 *            - In contrast to the reference implementation, we assume
 *              unsigned canonical coefficients here.
 *              The reference implementation works with coefficients
 *              in the range (-MLKEM_Q+1,...,MLKEM_Q-1). */
MLK_STATIC_TESTABLE void mlk_poly_compress_d5_c(
    uint8_t r[MLKEM_POLYCOMPRESSEDBYTES_D5], const mlk_poly *a)
__contract__(
  requires(memory_no_alias(r, MLKEM_POLYCOMPRESSEDBYTES_D5))
  requires(memory_no_alias(a, sizeof(mlk_poly)))
  requires(array_bound(a->coeffs, 0, MLKEM_N, 0, MLKEM_Q))
  assigns(memory_slice(r, MLKEM_POLYCOMPRESSEDBYTES_D5))
)
{
  unsigned i;
  mlk_assert_bound(a, MLKEM_N, 0, MLKEM_Q);

  for (i = 0; i < MLKEM_N / 8; i++)
  __loop__(invariant(i <= MLKEM_N / 8))
  {
    unsigned j;
    uint8_t t[8] = {0};
    for (j = 0; j < 8; j++)
    __loop__(
      invariant(i <= MLKEM_N / 8 && j <= 8)
      invariant(array_bound(t, 0, j, 0, 32)))
    {
      t[j] = mlk_scalar_compress_d5(a->coeffs[8 * i + j]);
    }

    r[i * 5] = (uint8_t)(0xFF & ((t[0] >> 0) | (t[1] << 5)));
    r[i * 5 + 1] = (uint8_t)(0xFF & ((t[1] >> 3) | (t[2] << 2) | (t[3] << 7)));
    r[i * 5 + 2] = (uint8_t)(0xFF & ((t[3] >> 1) | (t[4] << 4)));
    r[i * 5 + 3] = (uint8_t)(0xFF & ((t[4] >> 4) | (t[5] << 1) | (t[6] << 6)));
    r[i * 5 + 4] = (uint8_t)(0xFF & ((t[6] >> 2) | (t[7] << 3)));
  }
}

MLK_INTERNAL_API
void mlk_poly_compress_d5(uint8_t r[MLKEM_POLYCOMPRESSEDBYTES_D5],
                          const mlk_poly *a)
__contract__(
  requires(memory_no_alias(r, MLKEM_POLYCOMPRESSEDBYTES_D5))
  requires(memory_no_alias(a, sizeof(mlk_poly)))
  requires(array_bound(a->coeffs, 0, MLKEM_N, 0, MLKEM_Q))
  assigns(memory_slice(r, MLKEM_POLYCOMPRESSEDBYTES_D5))
)
{
#if defined(MLK_USE_NATIVE_POLY_COMPRESS_D5)
  int ret;
  mlk_assert_bound(a, MLKEM_N, 0, MLKEM_Q);
  ret = mlk_poly_compress_d5_native(r, a->coeffs);
  if (ret == MLK_NATIVE_FUNC_SUCCESS)
  {
    return;
  }
#endif /* MLK_USE_NATIVE_POLY_COMPRESS_D5 */

  mlk_poly_compress_d5_c(r, a);
}

/* Reference: Embedded into `polyvec_compress()` in the
 *            reference implementation, for ML-KEM-1024.
 *            - In contrast to the reference implementation, we assume
 *              unsigned canonical coefficients here.
 *              The reference implementation works with coefficients
 *              in the range (-MLKEM_Q+1,...,MLKEM_Q-1). */
MLK_STATIC_TESTABLE void mlk_poly_compress_d11_c(
    uint8_t r[MLKEM_POLYCOMPRESSEDBYTES_D11], const mlk_poly *a)
__contract__(
  requires(memory_no_alias(r, MLKEM_POLYCOMPRESSEDBYTES_D11))
  requires(memory_no_alias(a, sizeof(mlk_poly)))
  requires(array_bound(a->coeffs, 0, MLKEM_N, 0, MLKEM_Q))
  assigns(memory_slice(r, MLKEM_POLYCOMPRESSEDBYTES_D11))
)
{
  unsigned j;
  mlk_assert_bound(a, MLKEM_N, 0, MLKEM_Q);

  for (j = 0; j < MLKEM_N / 8; j++)
  __loop__(invariant(j <= MLKEM_N / 8))
  {
    unsigned k;
    uint16_t t[8];
    for (k = 0; k < 8; k++)
    __loop__(
      invariant(k <= 8)
      invariant(forall(r, 0, k, t[r] < (1u << 11))))
    {
      t[k] = mlk_scalar_compress_d11(a->coeffs[8 * j + k]);
    }

    /*
     * Make all implicit truncation explicit. No data is being
     * truncated for the LHS's since each t[i] is 11-bit in size.
     */
    r[11 * j + 0] = (uint8_t)((t[0] >> 0) & 0xFF);
    r[11 * j + 1] = (uint8_t)((t[0] >> 8) | ((t[1] << 3) & 0xFF));
    r[11 * j + 2] = (uint8_t)((t[1] >> 5) | ((t[2] << 6) & 0xFF));
    r[11 * j + 3] = (uint8_t)((t[2] >> 2) & 0xFF);
    r[11 * j + 4] = (uint8_t)((t[2] >> 10) | ((t[3] << 1) & 0xFF));
    r[11 * j + 5] = (uint8_t)((t[3] >> 7) | ((t[4] << 4) & 0xFF));
    r[11 * j + 6] = (uint8_t)((t[4] >> 4) | ((t[5] << 7) & 0xFF));
    r[11 * j + 7] = (uint8_t)((t[5] >> 1) & 0xFF);
    r[11 * j + 8] = (uint8_t)((t[5] >> 9) | ((t[6] << 2) & 0xFF));
    r[11 * j + 9] = (uint8_t)((t[6] >> 6) | ((t[7] << 5) & 0xFF));
    r[11 * j + 10] = (uint8_t)(t[7] >> 3);
  }
}

MLK_INTERNAL_API
void mlk_poly_compress_d11(uint8_t r[MLKEM_POLYCOMPRESSEDBYTES_D11],
                           const mlk_poly *a)
__contract__(
  requires(memory_no_alias(r, MLKEM_POLYCOMPRESSEDBYTES_D11))
  requires(memory_no_alias(a, sizeof(mlk_poly)))
  requires(array_bound(a->coeffs, 0, MLKEM_N, 0, MLKEM_Q))
  assigns(memory_slice(r, MLKEM_POLYCOMPRESSEDBYTES_D11))
)
{
#if defined(MLK_USE_NATIVE_POLY_COMPRESS_D11)
  int ret;
  mlk_assert_bound(a, MLKEM_N, 0, MLKEM_Q);
  ret = mlk_poly_compress_d11_native(r, a->coeffs);
  if (ret == MLK_NATIVE_FUNC_SUCCESS)
  {
    return;
  }
#endif /* MLK_USE_NATIVE_POLY_COMPRESS_D11 */

  mlk_poly_compress_d11_c(r, a);
}

/* Reference: `poly_decompress()` in the reference implementation @[REF],
 *            for ML-KEM-1024. */
MLK_STATIC_TESTABLE void mlk_poly_decompress_d5_c(
    mlk_poly *r, const uint8_t a[MLKEM_POLYCOMPRESSEDBYTES_D5])
__contract__(
  requires(memory_no_alias(r, sizeof(mlk_poly)))
  requires(memory_no_alias(a, MLKEM_POLYCOMPRESSEDBYTES_D5))
  assigns(memory_slice(r, sizeof(mlk_poly)))
  ensures(array_bound(r->coeffs, 0, MLKEM_N, 0, MLKEM_Q))
)
{
  unsigned i;
  for (i = 0; i < MLKEM_N / 8; i++)
  __loop__(
    invariant(i <= MLKEM_N / 8)
    invariant(array_bound(r->coeffs, 0, 8 * i, 0, MLKEM_Q)))
  {
    unsigned j;
    uint8_t t[8];
    const unsigned offset = i * 5;
    /*
     * Explicitly truncate to avoid warning about
     * implicit truncation in CBMC and unwind loop for ease
     * of proof.
     */

    /*
     * Decompress 5 8-bit bytes (so 40 bits) into
     * 8 5-bit values stored in t[]
     */
    t[0] = 0x1F & (a[offset + 0] >> 0);
    t[1] = 0x1F & ((a[offset + 0] >> 5) | (a[offset + 1] << 3));
    t[2] = 0x1F & (a[offset + 1] >> 2);
    t[3] = 0x1F & ((a[offset + 1] >> 7) | (a[offset + 2] << 1));
    t[4] = 0x1F & ((a[offset + 2] >> 4) | (a[offset + 3] << 4));
    t[5] = 0x1F & (a[offset + 3] >> 1);
    t[6] = 0x1F & ((a[offset + 3] >> 6) | (a[offset + 4] << 2));
    t[7] = 0x1F & (a[offset + 4] >> 3);

    /* and copy to the correct slice in r[] */
    for (j = 0; j < 8; j++)
    __loop__(
      invariant(j <= 8 && i <= MLKEM_N / 8)
      invariant(array_bound(r->coeffs, 0, 8 * i + j, 0, MLKEM_Q)))
    {
      r->coeffs[8 * i + j] = mlk_scalar_decompress_d5(t[j]);
    }
  }

  mlk_assert_bound(r, MLKEM_N, 0, MLKEM_Q);
}

MLK_INTERNAL_API
void mlk_poly_decompress_d5(mlk_poly *r,
                            const uint8_t a[MLKEM_POLYCOMPRESSEDBYTES_D5])
__contract__(
  requires(memory_no_alias(r, sizeof(mlk_poly)))
  requires(memory_no_alias(a, MLKEM_POLYCOMPRESSEDBYTES_D5))
  assigns(memory_slice(r, sizeof(mlk_poly)))
  ensures(array_bound(r->coeffs, 0, MLKEM_N, 0, MLKEM_Q))
)
{
#if defined(MLK_USE_NATIVE_POLY_DECOMPRESS_D5)
  int ret;
  ret = mlk_poly_decompress_d5_native(r->coeffs, a);
  if (ret == MLK_NATIVE_FUNC_SUCCESS)
  {
    mlk_assert_bound(r, MLKEM_N, 0, MLKEM_Q);
    return;
  }
#endif /* MLK_USE_NATIVE_POLY_DECOMPRESS_D5 */

  mlk_poly_decompress_d5_c(r, a);
}

/* Reference: Embedded into `polyvec_decompress()` in the
 *            reference implementation, for ML-KEM-1024. */
MLK_STATIC_TESTABLE void mlk_poly_decompress_d11_c(
    mlk_poly *r, const uint8_t a[MLKEM_POLYCOMPRESSEDBYTES_D11])
__contract__(
  requires(memory_no_alias(r, sizeof(mlk_poly)))
  requires(memory_no_alias(a, MLKEM_POLYCOMPRESSEDBYTES_D11))
  assigns(memory_slice(r, sizeof(mlk_poly)))
  ensures(array_bound(r->coeffs, 0, MLKEM_N, 0, MLKEM_Q))
)
{
  unsigned j;
  for (j = 0; j < MLKEM_N / 8; j++)
  __loop__(
    invariant(j <= MLKEM_N / 8)
    invariant(array_bound(r->coeffs, 0, 8 * j, 0, MLKEM_Q)))
  {
    unsigned k;
    uint16_t t[8];
    uint8_t const *base = &a[11 * j];
    t[0] = 0x7FF & ((base[0] >> 0) | ((uint16_t)base[1] << 8));
    t[1] = 0x7FF & ((base[1] >> 3) | ((uint16_t)base[2] << 5));
    t[2] = 0x7FF & ((base[2] >> 6) | ((uint16_t)base[3] << 2) |
                    ((uint16_t)base[4] << 10));
    t[3] = 0x7FF & ((base[4] >> 1) | ((uint16_t)base[5] << 7));
    t[4] = 0x7FF & ((base[5] >> 4) | ((uint16_t)base[6] << 4));
    t[5] = 0x7FF & ((base[6] >> 7) | ((uint16_t)base[7] << 1) |
                    ((uint16_t)base[8] << 9));
    t[6] = 0x7FF & ((base[8] >> 2) | ((uint16_t)base[9] << 6));
    t[7] = 0x7FF & ((base[9] >> 5) | ((uint16_t)base[10] << 3));

    for (k = 0; k < 8; k++)
    __loop__(
      invariant(k <= 8)
      invariant(array_bound(r->coeffs, 0, 8 * j + k, 0, MLKEM_Q)))
    {
      r->coeffs[8 * j + k] = mlk_scalar_decompress_d11(t[k]);
    }
  }

  mlk_assert_bound(r, MLKEM_N, 0, MLKEM_Q);
}

MLK_INTERNAL_API
void mlk_poly_decompress_d11(mlk_poly *r,
                             const uint8_t a[MLKEM_POLYCOMPRESSEDBYTES_D11])
__contract__(
  requires(memory_no_alias(r, sizeof(mlk_poly)))
  requires(memory_no_alias(a, MLKEM_POLYCOMPRESSEDBYTES_D11))
  assigns(memory_slice(r, sizeof(mlk_poly)))
  ensures(array_bound(r->coeffs, 0, MLKEM_N, 0, MLKEM_Q))
)
{
#if defined(MLK_USE_NATIVE_POLY_DECOMPRESS_D11)
  int ret;
  ret = mlk_poly_decompress_d11_native(r->coeffs, a);
  if (ret == MLK_NATIVE_FUNC_SUCCESS)
  {
    mlk_assert_bound(r, MLKEM_N, 0, MLKEM_Q);
    return;
  }
#endif /* MLK_USE_NATIVE_POLY_DECOMPRESS_D11 */

  mlk_poly_decompress_d11_c(r, a);
}

#endif /* MLK_CONFIG_MULTILEVEL_WITH_SHARED || MLKEM_K == 4 */

/* Reference: `poly_tobytes()` in the reference implementation @[REF].
 *            - In contrast to the reference implementation, we assume
 *              unsigned canonical coefficients here.
 *              The reference implementation works with coefficients
 *              in the range (-MLKEM_Q+1,...,MLKEM_Q-1). */
MLK_STATIC_TESTABLE void mlk_poly_tobytes_c(uint8_t r[MLKEM_POLYBYTES],
                                            const mlk_poly *a)
__contract__(
  requires(memory_no_alias(r, MLKEM_POLYBYTES))
  requires(memory_no_alias(a, sizeof(mlk_poly)))
  requires(array_bound(a->coeffs, 0, MLKEM_N, 0, MLKEM_Q))
  assigns(memory_slice(r, MLKEM_POLYBYTES))
)
{
  unsigned i;
  mlk_assert_bound(a, MLKEM_N, 0, MLKEM_Q);

  for (i = 0; i < MLKEM_N / 2; i++)
  __loop__(invariant(i <= MLKEM_N / 2))
  {
    /* The conversion to uint16_t is safe since we assume that
     * the coefficients of `a` are non-negative. */
    const uint16_t t0 = (uint16_t)a->coeffs[2 * i];
    const uint16_t t1 = (uint16_t)a->coeffs[2 * i + 1];
    /*
     * t0 and t1 are both < MLKEM_Q, so contain at most 12 bits each of
     * significant data, so these can be packed into 24 bits or exactly
     * 3 bytes, as follows.
     */

    /* Least significant bits 0 - 7 of t0. */
    r[3 * i + 0] = (uint8_t)(t0 & 0xFF);

    /*
     * Most significant bits 8 - 11 of t0 become the least significant
     * nibble of the second byte. The least significant 4 bits
     * of t1 become the upper nibble of the second byte.
     *
     * The conversion to uint8_t does not alter the value.
     */
    r[3 * i + 1] = (uint8_t)((t0 >> 8) | ((t1 << 4) & 0xF0));

    /* Bits 4 - 11 of t1 become the third byte. The conversion to uint8_t
     * does not alter the value because t1 is 12-bit wide. */
    r[3 * i + 2] = (uint8_t)(t1 >> 4);
  }
}

MLK_INTERNAL_API
void mlk_poly_tobytes(uint8_t r[MLKEM_POLYBYTES], const mlk_poly *a)
{
#if defined(MLK_USE_NATIVE_POLY_TOBYTES)
  int ret;
  mlk_assert_bound(a, MLKEM_N, 0, MLKEM_Q);
  ret = mlk_poly_tobytes_native(r, a->coeffs);
  if (ret == MLK_NATIVE_FUNC_SUCCESS)
  {
    return;
  }
#endif /* MLK_USE_NATIVE_POLY_TOBYTES */

  mlk_poly_tobytes_c(r, a);
}

/* Reference: `poly_frombytes()` in the reference implementation @[REF]. */
MLK_STATIC_TESTABLE void mlk_poly_frombytes_c(mlk_poly *r,
                                              const uint8_t a[MLKEM_POLYBYTES])
__contract__(
  requires(memory_no_alias(a, MLKEM_POLYBYTES))
  requires(memory_no_alias(r, sizeof(mlk_poly)))
  assigns(memory_slice(r, sizeof(mlk_poly)))
  ensures(array_bound(r->coeffs, 0, MLKEM_N, 0, MLKEM_UINT12_LIMIT))
)
{
  unsigned i;
  for (i = 0; i < MLKEM_N / 2; i++)
  __loop__(
    invariant(i <= MLKEM_N / 2)
    invariant(array_bound(r->coeffs, 0, 2 * i, 0, MLKEM_UINT12_LIMIT)))
  {
    const uint8_t t0 = a[3 * i + 0];
    const uint8_t t1 = a[3 * i + 1];
    const uint8_t t2 = a[3 * i + 2];
    r->coeffs[2 * i + 0] = (int16_t)(t0 | ((t1 << 8) & 0xFFF));
    r->coeffs[2 * i + 1] = (int16_t)((t1 >> 4) | (t2 << 4));
  }

  /* Note that the coefficients are not canonical */
  mlk_assert_bound(r, MLKEM_N, 0, MLKEM_UINT12_LIMIT);
}

MLK_INTERNAL_API
void mlk_poly_frombytes(mlk_poly *r, const uint8_t a[MLKEM_POLYBYTES])
{
#if defined(MLK_USE_NATIVE_POLY_FROMBYTES)
  int ret;
  ret = mlk_poly_frombytes_native(r->coeffs, a);
  if (ret == MLK_NATIVE_FUNC_SUCCESS)
  {
    return;
  }
#endif /* MLK_USE_NATIVE_POLY_FROMBYTES */

  mlk_poly_frombytes_c(r, a);
}

/* Reference: `poly_frommsg()` in the reference implementation @[REF].
 *            - We use a value barrier around the bit-selection mask to
 *              reduce the risk of compiler-introduced branches.
 *              The reference implementation contains the expression
 *              `(msg[i] >> j) & 1` which the compiler can reason must
 *              be either 0 or 1. */
MLK_INTERNAL_API
void mlk_poly_frommsg(mlk_poly *r, const uint8_t msg[MLKEM_INDCPA_MSGBYTES])
{
  unsigned i;
#if (MLKEM_INDCPA_MSGBYTES != MLKEM_N / 8)
#error "MLKEM_INDCPA_MSGBYTES must be equal to MLKEM_N/8 bytes!"
#endif

  for (i = 0; i < MLKEM_N / 8; i++)
  __loop__(
    invariant(i <= MLKEM_N / 8)
    invariant(array_bound(r->coeffs, 0, 8 * i, 0, MLKEM_Q)))
  {
    unsigned j;
    for (j = 0; j < 8; j++)
    __loop__(
      invariant(i <  MLKEM_N / 8 && j <= 8)
      invariant(array_bound(r->coeffs, 0, 8 * i + j, 0, MLKEM_Q)))
    {
      /* mlk_ct_sel_int16(MLKEM_Q_HALF, 0, b) is `Decompress_1(b != 0)`
       * as per @[FIPS203, Eq (4.8)]. */

      /* Prevent the compiler from recognizing this as a bit selection */
      uint8_t mask = mlk_value_barrier_u8((uint8_t)(1u << j));
      r->coeffs[8 * i + j] = mlk_ct_sel_int16(MLKEM_Q_HALF, 0, msg[i] & mask);
    }
  }
  mlk_assert_abs_bound(r, MLKEM_N, MLKEM_Q);
}

/* Reference: `poly_tomsg()` in the reference implementation @[REF].
 *            - In contrast to the reference implementation, we assume
 *              unsigned canonical coefficients here.
 *              The reference implementation works with coefficients
 *              in the range (-MLKEM_Q+1,...,MLKEM_Q-1).
 */
MLK_INTERNAL_API
void mlk_poly_tomsg(uint8_t msg[MLKEM_INDCPA_MSGBYTES], const mlk_poly *a)
{
  unsigned i;
  mlk_assert_bound(a, MLKEM_N, 0, MLKEM_Q);

  for (i = 0; i < MLKEM_N / 8; i++)
  __loop__(invariant(i <= MLKEM_N / 8))
  {
    unsigned j;
    msg[i] = 0;
    for (j = 0; j < 8; j++)
    __loop__(
      invariant(i <= MLKEM_N / 8 && j <= 8))
    {
      uint32_t t = mlk_scalar_compress_d1(a->coeffs[8 * i + j]);
      msg[i] |= (uint8_t)(t << j);
    }
  }
}

#else /* !MLK_CONFIG_MULTILEVEL_NO_SHARED */

MLK_EMPTY_CU(compress)

#endif /* MLK_CONFIG_MULTILEVEL_NO_SHARED */
