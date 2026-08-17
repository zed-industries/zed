// Copyright 2013-2016 The OpenSSL Project Authors. All Rights Reserved.
// Copyright (c) 2012, Intel Corporation. All Rights Reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//
// Originally written by Shay Gueron (1, 2), and Vlad Krasnov (1)
// (1) Intel Corporation, Israel Development Center, Haifa, Israel
// (2) University of Haifa, Israel

#ifndef OPENSSL_HEADER_CRYPTO_FIPSMODULE_BN_RSAZ_EXP_H
#define OPENSSL_HEADER_CRYPTO_FIPSMODULE_BN_RSAZ_EXP_H

#include <openssl/bn.h>

#include "internal.h"
#include "../../internal.h"

#if defined(__cplusplus)
extern "C" {
#endif

#if !defined(OPENSSL_NO_ASM) && defined(OPENSSL_X86_64)
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

inline int rsaz_avx2_capable(void) { return CRYPTO_is_AVX2_capable(); }

inline int rsaz_avx2_preferred(void) {
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


#endif  // !OPENSSL_NO_ASM && OPENSSL_X86_64

#if defined(__cplusplus)
}  // extern "C"
#endif

#endif  // OPENSSL_HEADER_CRYPTO_FIPSMODULE_BN_RSAZ_EXP_H
