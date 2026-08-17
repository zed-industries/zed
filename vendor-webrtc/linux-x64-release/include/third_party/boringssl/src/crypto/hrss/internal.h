// Copyright 2018 The BoringSSL Authors
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

#ifndef OPENSSL_HEADER_CRYPTO_HRSS_INTERNAL_H
#define OPENSSL_HEADER_CRYPTO_HRSS_INTERNAL_H

#include <openssl/base.h>
#include "../internal.h"

#if defined(__cplusplus)
extern "C" {
#endif


#define N 701
#define BITS_PER_WORD (sizeof(crypto_word_t) * 8)
#define WORDS_PER_POLY ((N + BITS_PER_WORD - 1) / BITS_PER_WORD)
#define BITS_IN_LAST_WORD (N % BITS_PER_WORD)

struct poly2 {
  crypto_word_t v[WORDS_PER_POLY];
};

struct poly3 {
  struct poly2 s, a;
};

OPENSSL_EXPORT void HRSS_poly3_mul(struct poly3 *out, const struct poly3 *x,
                                   const struct poly3 *y);
OPENSSL_EXPORT void HRSS_poly3_invert(struct poly3 *out,
                                      const struct poly3 *in);

// On x86-64, we can use the AVX2 code from [HRSS]. (The authors have given
// explicit permission for this and signed a CLA.) However it's 57KB of object
// code, so it's not used if |OPENSSL_SMALL| is defined.
#if !defined(OPENSSL_NO_ASM) && !defined(OPENSSL_SMALL) && \
    defined(OPENSSL_X86_64) && defined(OPENSSL_LINUX)
#define POLY_RQ_MUL_ASM
// POLY_MUL_RQ_SCRATCH_SPACE is the number of bytes of scratch space needed
// by the assembly function poly_Rq_mul.
#define POLY_MUL_RQ_SCRATCH_SPACE (6144 + 6144 + 12288 + 512 + 9408 + 32)

// poly_Rq_mul is defined in assembly. Inputs and outputs must be 16-byte-
// aligned.
extern void poly_Rq_mul(
    uint16_t r[N + 3], const uint16_t a[N + 3], const uint16_t b[N + 3],
    // The following should be `scratch[POLY_MUL_RQ_SCRATCH_SPACE]` but
    // GCC 11.1 has a bug with unions that breaks that.
    uint8_t scratch[]);
#endif


#if defined(__cplusplus)
}  // extern "C"
#endif

#endif  // !OPENSSL_HEADER_CRYPTO_HRSS_INTERNAL_H
