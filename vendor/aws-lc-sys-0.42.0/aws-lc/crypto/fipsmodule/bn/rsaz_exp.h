// Copyright 2013-2016 The OpenSSL Project Authors. All Rights Reserved.
// Copyright (c) 2012, Intel Corporation. All Rights Reserved.
//
// Originally written by Shay Gueron (1, 2), and Vlad Krasnov (1)
// (1) Intel Corporation, Israel Development Center, Haifa, Israel
// (2) University of Haifa, Israel
//
// SPDX-License-Identifier: Apache-2.0

#ifndef OPENSSL_HEADER_BN_RSAZ_EXP_H
#define OPENSSL_HEADER_BN_RSAZ_EXP_H

#include <openssl/bn.h>

#include "internal.h"
#include "../../internal.h"
#include "../cpucap/internal.h"

#if defined(__cplusplus)
extern "C" {
#endif

#if !defined(OPENSSL_NO_ASM) && defined(OPENSSL_X86_64) && \
    !defined(MY_ASSEMBLER_IS_TOO_OLD_FOR_AVX)
#define RSAZ_ENABLED


// RSAZ_1024_mod_exp_avx2 sets |result| to |base_norm| raised to |exponent|
// modulo |m_norm|. |base_norm| must be fully-reduced and |exponent| must have
// the high bit set (it is 1024 bits wide). |RR| and |k0| must be |RR| and |n0|,
// respectively, extracted from |m_norm|'s |BN_MONT_CTX|. |storage_words| is a
// temporary buffer that must be aligned to |MOD_EXP_CTIME_ALIGN| bytes.
void RSAZ_1024_mod_exp_avx2(BN_ULONG result[16], const BN_ULONG base_norm[16],
                            const BN_ULONG exponent[16],
                            const BN_ULONG m_norm[16], const BN_ULONG RR[16],
                            BN_ULONG k0,
                            BN_ULONG storage_words[MOD_EXP_CTIME_STORAGE_LEN]);

OPENSSL_INLINE int rsaz_avx2_capable(void) {
  return CRYPTO_is_AVX2_capable();
}

OPENSSL_INLINE int rsaz_avx2_preferred(void) {
  if (CRYPTO_is_BMI1_capable() && CRYPTO_is_BMI2_capable() &&
      CRYPTO_is_ADX_capable()) {
    // If BMI1, BMI2, and ADX are available, x86_64-mont5.pl is faster. See the
    // .Lmulx4x_enter and .Lpowerx5_enter branches.
    return 0;
  }
  return CRYPTO_is_AVX2_capable();
}


// Assembly functions.

// RSAZ represents 1024-bit integers using unsaturated 29-bit limbs stored in
// 64-bit integers. This requires 36 limbs but padded up to 40.
//
// See crypto/bn/asm/rsaz-avx2.pl for further details.

// rsaz_1024_norm2red_avx2 converts |norm| from |BIGNUM| to RSAZ representation
// and writes the result to |red|.
void rsaz_1024_norm2red_avx2(BN_ULONG red[40], const BN_ULONG norm[16]);

// rsaz_1024_mul_avx2 computes |a| * |b| mod |n| and writes the result to |ret|.
// Inputs and outputs are in Montgomery form, using RSAZ's representation. |k|
// is -|n|^-1 mod 2^64 or |n0| from |BN_MONT_CTX|.
void rsaz_1024_mul_avx2(BN_ULONG ret[40], const BN_ULONG a[40],
                        const BN_ULONG b[40], const BN_ULONG n[40], BN_ULONG k);

// rsaz_1024_mul_avx2 computes |a|^(2*|count|) mod |n| and writes the result to
// |ret|. Inputs and outputs are in Montgomery form, using RSAZ's
// representation. |k| is -|n|^-1 mod 2^64 or |n0| from |BN_MONT_CTX|.
void rsaz_1024_sqr_avx2(BN_ULONG ret[40], const BN_ULONG a[40],
                        const BN_ULONG n[40], BN_ULONG k, int count);

// rsaz_1024_scatter5_avx2 stores |val| at index |i| of |tbl|. |i| must be
// positive and at most 31. It is treated as public. Note the table only uses 18
// |BN_ULONG|s per entry instead of 40. It packs two 29-bit limbs into each
// |BN_ULONG| and only stores 36 limbs rather than the padded 40.
void rsaz_1024_scatter5_avx2(BN_ULONG tbl[32 * 18], const BN_ULONG val[40],
                             int i);

// rsaz_1024_gather5_avx2 loads index |i| of |tbl| and writes it to |val|. |i|
// must be positive and at most 31. It is treated as secret. |tbl| must be
// aligned to 32 bytes.
void rsaz_1024_gather5_avx2(BN_ULONG val[40], const BN_ULONG tbl[32 * 18],
                            int i);

// rsaz_1024_red2norm_avx2 converts |red| from RSAZ to |BIGNUM| representation
// and writes the result to |norm|. The result will be <= the modulus.
//
// WARNING: The result of this operation may not be fully reduced. |norm| may be
// the modulus instead of zero. This function should be followed by a call to
// |bn_reduce_once|.
void rsaz_1024_red2norm_avx2(BN_ULONG norm[16], const BN_ULONG red[40]);

#if !defined(MY_ASSEMBLER_IS_TOO_OLD_FOR_512AVX)
#define RSAZ_512_ENABLED

// Dual Montgomery modular exponentiation using prime moduli of the
// same bit size, optimized with AVX512 ISA.
//
// Computes res|i| = base|i| ^ exp|i| mod m|i|.
//
// Input and output parameters for each exponentiation are independent and
// denoted here by index |i|, i = 1..2.
//
// Input and output are all in regular 2^64 radix.
//
// Each moduli shall be |modlen| bit size.
//
// Supported cases:
//   - 2x1024
//   - 2x1536
//   - 2x2048
//
//  [out] res|i|      - result of modular exponentiation: array of qword values
//                      in regular (2^64) radix. Size of array shall be enough
//                      to hold |modlen| bits.
//  [in]  base|i|     - base
//  [in]  exp|i|      - exponent
//  [in]  m|i|        - moduli
//  [in]  rr|i|       - Montgomery parameter RR = R^2 mod m|i|
//  [in]  k0_|i|      - Montgomery parameter k0 = -1/m|i| mod 2^64
//  [in]  modlen - moduli bit size
//
// \return 0 in case of failure,
//         1 in case of success.
//
// NB: This function does not do any checks on its arguments, its
// caller, `BN_mod_exp_mont_consttime_x2`, checks args. It should be
// the function used directly.
int RSAZ_mod_exp_avx512_x2(uint64_t *res1,
                           const uint64_t *base1,
                           const uint64_t *exponent1,
                           const uint64_t *m1,
                           const uint64_t *RR1,
                           uint64_t k0_1,
                           uint64_t *res2,
                           const uint64_t *base2,
                           const uint64_t *exponent2,
                           const uint64_t *m2,
                           const uint64_t *RR2,
                           uint64_t k0_2,
                           int modlen);

// Naming convention for the following functions:
//
//   * amm: Almost Montgomery Multiplication
//   * ams: Almost Montgomery Squaring
//   * 52xZZ: data represented as array of ZZ digits in 52-bit radix
//   * _x1_/_x2_:  1 or 2 independent inputs/outputs
//   * ifma256: uses 256-bit wide IFMA ISA (AVX512_IFMA256)
//
//
// Almost Montgomery Multiplication (AMM) for 20-digit number in radix
// 2^52.
//
// AMM is defined as presented in the paper [1].
//
// The input and output are presented in 2^52 radix domain, i.e.
// |res|, |a|, |b|, |m| are arrays of 20 64-bit qwords with 12 high
// bits zeroed.  |k0| is a Montgomery coefficient, which is here k0 =
// -1/m mod 2^64
//
// NB: the AMM implementation does not perform "conditional"
// subtraction step specified in the original algorithm as according
// to the Lemma 1 from the paper [2], the result will be always < 2*m
// and can be used as a direct input to the next AMM iteration.  This
// post-condition is true, provided the correct parameter |s| (notion
// of the Lemma 1 from [2]) is chosen, i.e.  s >= n + 2 * k, which
// matches our case: 1040 > 1024 + 2 * 1.
//
// [1] Gueron, S. Efficient software implementations of modular
//     exponentiation.  DOI: 10.1007/s13389-012-0031-5
// [2] Gueron, S. Enhanced Montgomery Multiplication.  DOI:
//     10.1007/3-540-36400-5_5
void rsaz_amm52x20_x1_ifma256(uint64_t *res, const uint64_t *a,
                              const uint64_t *b, const uint64_t *m,
                              uint64_t k0);

// Dual Almost Montgomery Multiplication for 20-digit number in radix
// 2^52
//
// See description of rsaz_amm52x20_x1_ifma256() above for
// details about Almost Montgomery Multiplication algorithm and
// function input parameters description.
//
// This function does two AMMs for two independent inputs, hence dual.
void rsaz_amm52x20_x2_ifma256(uint64_t *out, const uint64_t *a,
                              const uint64_t *b, const uint64_t *m,
                              const uint64_t k0[2]);

// Constant time extraction from the precomputed table of powers
// base^i, where i = 0..2^EXP_WIN_SIZE-1
//
// The input |red_table| contains precomputations for two independent
// base values and two independent moduli. The precomputed powers of
// the base values are stored contiguously in the table.
//
// Extracted value (output) is 2 20 digit numbers in 2^52 radix.
//
// EXP_WIN_SIZE = 5
void extract_multiplier_2x20_win5(uint64_t *red_Y,
                                  const uint64_t *red_table,
                                  int red_table_idx1, int red_table_idx2);

// Almost Montgomery Multiplication (AMM) for 30-digit number in radix
// 2^52.
//
// AMM is defined as presented in the paper [1].
//
// The input and output are presented in 2^52 radix domain, i.e.
// |res|, |a|, |b|, |m| are arrays of 32 64-bit qwords with 12 high
// bits zeroed
//
// NOTE: the function uses zero-padded data - 2 high QWs is a padding.
//
// |k0| is a Montgomery coefficient, which is here k0 = -1/m mod 2^64
//
// NB: the AMM implementation does not perform "conditional"
// subtraction step specified in the original algorithm as according
// to the Lemma 1 from the paper [2], the result will be always < 2*m
// and can be used as a direct input to the next AMM iteration.  This
// post-condition is true, provided the correct parameter |s| (notion
// of the Lemma 1 from [2]) is chosen, i.e.  s >= n + 2 * k, which
// matches our case: 1560 > 1536 + 2 * 1.
//
// [1] Gueron, S. Efficient software implementations of modular
//     exponentiation.  DOI: 10.1007/s13389-012-0031-5
// [2] Gueron, S. Enhanced Montgomery Multiplication.  DOI:
//     10.1007/3-540-36400-5_5
void rsaz_amm52x30_x1_ifma256(uint64_t *res, const uint64_t *a,
                              const uint64_t *b, const uint64_t *m,
                              uint64_t k0);
// Dual Almost Montgomery Multiplication for 30-digit number in radix
// 2^52
//
// See description of rsaz_amm52x30_x1_ifma256() above for
// details about Almost Montgomery Multiplication algorithm and
// function input parameters description.
//
// This function does two AMMs for two independent inputs, hence dual.
//
// NOTE: the function uses zero-padded data - 2 high QWs is a padding.
void rsaz_amm52x30_x2_ifma256(uint64_t *out, const uint64_t *a,
                              const uint64_t *b, const uint64_t *m,
                              const uint64_t k0[2]);

// Constant time extraction from the precomputed table of powers
// base^i, where i = 0..2^EXP_WIN_SIZE-1
//
// The input |red_table| contains precomputations for two independent
// base values.  |red_table_idx1| and |red_table_idx2| are
// corresponding power indexes.
//
// Extracted value (output) is 2 (30 + 2) digits numbers in 2^52
// radix.  (2 high QW is zero padding)
//
// EXP_WIN_SIZE = 5
void extract_multiplier_2x30_win5(uint64_t *red_Y,
                                  const uint64_t *red_table,
                                  int red_table_idx1, int red_table_idx2);

// Almost Montgomery Multiplication (AMM) for 40-digit number in radix
// 2^52.
//
// AMM is defined as presented in the paper [1].
//
// The input and output are presented in 2^52 radix domain, i.e.
// |res|, |a|, |b|, |m| are arrays of 40 64-bit qwords with 12 high
// bits zeroed.  |k0| is a Montgomery coefficient, which is here k0 =
// -1/m mod 2^64
//
// NB: the AMM implementation does not perform "conditional"
// subtraction step specified in the original algorithm as according
// to the Lemma 1 from the paper [2], the result will be always < 2*m
// and can be used as a direct input to the next AMM iteration.  This
// post-condition is true, provided the correct parameter |s| (notion
// of the Lemma 1 from [2]) is chosen, i.e.  s >= n + 2 * k, which
// matches our case: 2080 > 2048 + 2 * 1.
//
// [1] Gueron, S. Efficient software implementations of modular
//     exponentiation.  DOI: 10.1007/s13389-012-0031-5
// [2] Gueron, S. Enhanced Montgomery Multiplication.  DOI:
//     10.1007/3-540-36400-5_5
void rsaz_amm52x40_x1_ifma256(uint64_t *res, const uint64_t *a,
                              const uint64_t *b, const uint64_t *m,
                              uint64_t k0);

// Dual Almost Montgomery Multiplication for 40-digit number in radix
// 2^52
//
// See description of rsaz_amm52x40_x1_ifma256() above for
// details about Almost Montgomery Multiplication algorithm and
// function input parameters description.
//
// This function does two AMMs for two independent inputs, hence dual.
void rsaz_amm52x40_x2_ifma256(uint64_t *out, const uint64_t *a,
                              const uint64_t *b, const uint64_t *m,
                              const uint64_t k0[2]);

// Constant time extraction from the precomputed table of powers base^i, where
//    i = 0..2^EXP_WIN_SIZE-1
//
// The input |red_table| contains precomputations for two independent base values.
// |red_table_idx1| and |red_table_idx2| are corresponding power indexes.
//
// Extracted value (output) is 2 40 digits numbers in 2^52 radix.
//
// EXP_WIN_SIZE = 5
void extract_multiplier_2x40_win5(uint64_t *red_Y,
                                  const uint64_t *red_table,
                                  int red_table_idx1, int red_table_idx2);
#endif // !MY_ASSEMBLER_IS_TOO_OLD_FOR_512AVX

#endif  // !OPENSSL_NO_ASM && OPENSSL_X86_64

#if defined(__cplusplus)
}  // extern "C"
#endif

#endif  // OPENSSL_HEADER_BN_RSAZ_EXP_H
