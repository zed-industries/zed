// Copyright (C) 1995-1998 Eric Young (eay@cryptsoft.com) All rights reserved.
// SPDX-License-Identifier: Apache-2.0

#ifndef OPENSSL_HEADER_CAST_INTERNAL_H
#define OPENSSL_HEADER_CAST_INTERNAL_H

#include <openssl/base.h>

#if defined(__cplusplus)
extern "C" {
#endif

struct cast_key_st {
  uint32_t data[32];
  int short_key;  // Use reduced rounds for short key
} /* CAST_KEY */;

extern const uint32_t CAST_S_table0[256];
extern const uint32_t CAST_S_table1[256];
extern const uint32_t CAST_S_table2[256];
extern const uint32_t CAST_S_table3[256];
extern const uint32_t CAST_S_table4[256];
extern const uint32_t CAST_S_table5[256];
extern const uint32_t CAST_S_table6[256];
extern const uint32_t CAST_S_table7[256];

#define CAST_BLOCK 8
#define CAST_KEY_LENGTH 16

void CAST_set_key(CAST_KEY *key, size_t len, const uint8_t *data);
void CAST_ecb_encrypt(const uint8_t *in, uint8_t *out, const CAST_KEY *key,
                      int enc);
void CAST_encrypt(uint32_t *data, const CAST_KEY *key);
void CAST_decrypt(uint32_t *data, const CAST_KEY *key);
void CAST_cbc_encrypt(const uint8_t *in, uint8_t *out, size_t length,
                      const CAST_KEY *ks, uint8_t *iv, int enc);

#if defined(__cplusplus)
} // extern C
#endif

#endif  // OPENSSL_HEADER_CAST_INTERNAL_H
