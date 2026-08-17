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

#ifndef OPENSSL_HEADER_BLOWFISH_H
#define OPENSSL_HEADER_BLOWFISH_H

#include <openssl/base.h>   // IWYU pragma: export

#ifdef  __cplusplus
extern "C" {
#endif


#define BF_ENCRYPT 1
#define BF_DECRYPT 0

#define BF_ROUNDS 16
#define BF_BLOCK 8

typedef struct bf_key_st {
  uint32_t P[BF_ROUNDS + 2];
  uint32_t S[4 * 256];
} BF_KEY;

OPENSSL_EXPORT void BF_set_key(BF_KEY *key, size_t len, const uint8_t *data);
OPENSSL_EXPORT void BF_encrypt(uint32_t *data, const BF_KEY *key);
OPENSSL_EXPORT void BF_decrypt(uint32_t *data, const BF_KEY *key);

OPENSSL_EXPORT void BF_ecb_encrypt(const uint8_t *in, uint8_t *out,
                                   const BF_KEY *key, int enc);
OPENSSL_EXPORT void BF_cbc_encrypt(const uint8_t *in, uint8_t *out,
                                   size_t length, const BF_KEY *schedule,
                                   uint8_t *ivec, int enc);


#ifdef  __cplusplus
}
#endif

#endif  // OPENSSL_HEADER_BLOWFISH_H
