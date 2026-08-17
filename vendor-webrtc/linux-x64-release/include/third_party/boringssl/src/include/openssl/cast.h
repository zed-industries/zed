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

#ifndef OPENSSL_HEADER_CAST_H
#define OPENSSL_HEADER_CAST_H

#include <openssl/base.h>   // IWYU pragma: export

#ifdef  __cplusplus
extern "C" {
#endif


#define CAST_ENCRYPT 1
#define CAST_DECRYPT 0

#define CAST_BLOCK 8
#define CAST_KEY_LENGTH 16

typedef struct cast_key_st {
  uint32_t data[32];
  int short_key;  // Use reduced rounds for short key
} CAST_KEY;

OPENSSL_EXPORT void CAST_set_key(CAST_KEY *key, size_t len,
                                 const uint8_t *data);
OPENSSL_EXPORT void CAST_ecb_encrypt(const uint8_t *in, uint8_t *out,
                                     const CAST_KEY *key, int enc);
OPENSSL_EXPORT void CAST_encrypt(uint32_t *data, const CAST_KEY *key);
OPENSSL_EXPORT void CAST_decrypt(uint32_t *data, const CAST_KEY *key);
OPENSSL_EXPORT void CAST_cbc_encrypt(const uint8_t *in, uint8_t *out,
                                     size_t length, const CAST_KEY *ks,
                                     uint8_t *iv, int enc);

OPENSSL_EXPORT void CAST_cfb64_encrypt(const uint8_t *in, uint8_t *out,
                                       size_t length, const CAST_KEY *schedule,
                                       uint8_t *ivec, int *num, int enc);

#ifdef  __cplusplus
}
#endif

#endif  // OPENSSL_HEADER_CAST_H
