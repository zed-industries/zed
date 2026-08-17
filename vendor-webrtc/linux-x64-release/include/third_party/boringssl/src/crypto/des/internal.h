// Copyright 1995-2016 The OpenSSL Project Authors. All Rights Reserved.
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

#ifndef OPENSSL_HEADER_CRYPTO_DES_INTERNAL_H
#define OPENSSL_HEADER_CRYPTO_DES_INTERNAL_H

#include <openssl/base.h>
#include <openssl/des.h>

#include "../internal.h"

#if defined(__cplusplus)
extern "C" {
#endif


// TODO(davidben): Ideally these macros would be replaced with
// |CRYPTO_load_u32_le| and |CRYPTO_store_u32_le|.

#define c2l(c, l)                         \
  do {                                    \
    (l) = ((uint32_t)(*((c)++)));         \
    (l) |= ((uint32_t)(*((c)++))) << 8L;  \
    (l) |= ((uint32_t)(*((c)++))) << 16L; \
    (l) |= ((uint32_t)(*((c)++))) << 24L; \
  } while (0)

#define l2c(l, c)                                    \
  do {                                               \
    *((c)++) = (unsigned char)(((l)) & 0xff);        \
    *((c)++) = (unsigned char)(((l) >> 8L) & 0xff);  \
    *((c)++) = (unsigned char)(((l) >> 16L) & 0xff); \
    *((c)++) = (unsigned char)(((l) >> 24L) & 0xff); \
  } while (0)

// NOTE - c is not incremented as per c2l
#define c2ln(c, l1, l2, n)                     \
  do {                                         \
    (c) += (n);                                \
    (l1) = (l2) = 0;                           \
    switch (n) {                               \
      case 8:                                  \
        (l2) = ((uint32_t)(*(--(c)))) << 24L;  \
        [[fallthrough]];                       \
      case 7:                                  \
        (l2) |= ((uint32_t)(*(--(c)))) << 16L; \
        [[fallthrough]];                       \
      case 6:                                  \
        (l2) |= ((uint32_t)(*(--(c)))) << 8L;  \
        [[fallthrough]];                       \
      case 5:                                  \
        (l2) |= ((uint32_t)(*(--(c))));        \
        [[fallthrough]];                       \
      case 4:                                  \
        (l1) = ((uint32_t)(*(--(c)))) << 24L;  \
        [[fallthrough]];                       \
      case 3:                                  \
        (l1) |= ((uint32_t)(*(--(c)))) << 16L; \
        [[fallthrough]];                       \
      case 2:                                  \
        (l1) |= ((uint32_t)(*(--(c)))) << 8L;  \
        [[fallthrough]];                       \
      case 1:                                  \
        (l1) |= ((uint32_t)(*(--(c))));        \
    }                                          \
  } while (0)

// NOTE - c is not incremented as per l2c
#define l2cn(l1, l2, c, n)                                \
  do {                                                    \
    (c) += (n);                                           \
    switch (n) {                                          \
      case 8:                                             \
        *(--(c)) = (unsigned char)(((l2) >> 24L) & 0xff); \
        [[fallthrough]];                                  \
      case 7:                                             \
        *(--(c)) = (unsigned char)(((l2) >> 16L) & 0xff); \
        [[fallthrough]];                                  \
      case 6:                                             \
        *(--(c)) = (unsigned char)(((l2) >> 8L) & 0xff);  \
        [[fallthrough]];                                  \
      case 5:                                             \
        *(--(c)) = (unsigned char)(((l2)) & 0xff);        \
        [[fallthrough]];                                  \
      case 4:                                             \
        *(--(c)) = (unsigned char)(((l1) >> 24L) & 0xff); \
        [[fallthrough]];                                  \
      case 3:                                             \
        *(--(c)) = (unsigned char)(((l1) >> 16L) & 0xff); \
        [[fallthrough]];                                  \
      case 2:                                             \
        *(--(c)) = (unsigned char)(((l1) >> 8L) & 0xff);  \
        [[fallthrough]];                                  \
      case 1:                                             \
        *(--(c)) = (unsigned char)(((l1)) & 0xff);        \
    }                                                     \
  } while (0)


// Correctly-typed versions of DES functions.
//
// See https://crbug.com/boringssl/683.

void DES_set_key_ex(const uint8_t key[8], DES_key_schedule *schedule);
void DES_ecb_encrypt_ex(const uint8_t in[8], uint8_t out[8],
                        const DES_key_schedule *schedule, int is_encrypt);
void DES_ncbc_encrypt_ex(const uint8_t *in, uint8_t *out, size_t len,
                         const DES_key_schedule *schedule, uint8_t ivec[8],
                         int enc);
void DES_ecb3_encrypt_ex(const uint8_t input[8], uint8_t output[8],
                         const DES_key_schedule *ks1,
                         const DES_key_schedule *ks2,
                         const DES_key_schedule *ks3, int enc);
void DES_ede3_cbc_encrypt_ex(const uint8_t *in, uint8_t *out, size_t len,
                             const DES_key_schedule *ks1,
                             const DES_key_schedule *ks2,
                             const DES_key_schedule *ks3, uint8_t ivec[8],
                             int enc);


// Private functions.
//
// These functions are only exported for use in |decrepit|.

OPENSSL_EXPORT void DES_decrypt3(uint32_t data[2], const DES_key_schedule *ks1,
                                 const DES_key_schedule *ks2,
                                 const DES_key_schedule *ks3);

OPENSSL_EXPORT void DES_encrypt3(uint32_t data[2], const DES_key_schedule *ks1,
                                 const DES_key_schedule *ks2,
                                 const DES_key_schedule *ks3);


#if defined(__cplusplus)
}  // extern C
#endif

#endif  // OPENSSL_HEADER_CRYPTO_DES_INTERNAL_H
