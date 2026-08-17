// Copyright (C) 1995-1998 Eric Young (eay@cryptsoft.com) All rights reserved.
// SPDX-License-Identifier: Apache-2.0

#ifndef OPENSSL_HEADER_BLOWFISH_H
#define OPENSSL_HEADER_BLOWFISH_H

#include <openssl/base.h>

#ifdef __cplusplus
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

OPENSSL_DEPRECATED OPENSSL_EXPORT void BF_set_key(BF_KEY *key, size_t len,
                                                  const uint8_t *data);
OPENSSL_DEPRECATED OPENSSL_EXPORT void BF_encrypt(uint32_t *data,
                                                  const BF_KEY *key);
OPENSSL_DEPRECATED OPENSSL_EXPORT void BF_decrypt(uint32_t *data,
                                                  const BF_KEY *key);

OPENSSL_DEPRECATED OPENSSL_EXPORT void BF_ecb_encrypt(const uint8_t *in,
                                                      uint8_t *out,
                                                      const BF_KEY *key,
                                                      int enc);
OPENSSL_DEPRECATED OPENSSL_EXPORT void BF_cbc_encrypt(const uint8_t *in,
                                                      uint8_t *out,
                                                      size_t length,
                                                      const BF_KEY *schedule,
                                                      uint8_t *ivec, int enc);

OPENSSL_DEPRECATED OPENSSL_EXPORT void BF_cfb64_encrypt(
    const uint8_t *in, uint8_t *out, size_t length, const BF_KEY *schedule,
    uint8_t *ivec, int *num, int encrypt);

OPENSSL_DEPRECATED OPENSSL_EXPORT void BF_ofb64_encrypt(
    const uint8_t *in, uint8_t *out, size_t length, const BF_KEY *schedule,
    uint8_t *ivec, int *num);

#ifdef __cplusplus
}
#endif

#endif  // OPENSSL_HEADER_BLOWFISH_H
