// Copyright (c) 2008 The OpenSSL Project.  All rights reserved.
// SPDX-License-Identifier: Apache-2.0

#include <openssl/type_check.h>

#include <assert.h>
#include <string.h>

#include "internal.h"


OPENSSL_STATIC_ASSERT(16 % sizeof(size_t) == 0,
                      ofb_block_cannot_be_divided_into_size_t)

void CRYPTO_ofb128_encrypt(const uint8_t *in, uint8_t *out, size_t len,
                           const AES_KEY *key, uint8_t ivec[16], unsigned *num,
                           block128_f block) {
  assert(key != NULL && ivec != NULL && num != NULL);
  assert(len == 0 || (in != NULL && out != NULL));

  unsigned n = *num;

  while (n && len) {
    *(out++) = *(in++) ^ ivec[n];
    --len;
    n = (n + 1) % 16;
  }

  while (len >= 16) {
    (*block)(ivec, ivec, key);
    CRYPTO_xor16(out, in, ivec);
    len -= 16;
    out += 16;
    in += 16;
    n = 0;
  }
  if (len) {
    (*block)(ivec, ivec, key);
    while (len--) {
      out[n] = in[n] ^ ivec[n];
      ++n;
    }
  }
  *num = n;
}
