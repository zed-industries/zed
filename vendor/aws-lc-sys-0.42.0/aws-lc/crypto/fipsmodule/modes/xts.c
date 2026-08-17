// Copyright (c) 2011 The OpenSSL Project.  All rights reserved.
// SPDX-License-Identifier: Apache-2.0

#include <openssl/evp.h>

#include <string.h>

#include <openssl/aes.h>
#include <openssl/cipher.h>
#include <openssl/err.h>

#include "internal.h"
#include "../../internal.h"

size_t CRYPTO_xts128_encrypt(const XTS128_CONTEXT *ctx,
                             const uint8_t iv[16], const uint8_t *inp,
                             uint8_t *out, size_t len, int enc) {
  union {
    uint64_t u[2];
    uint8_t c[16];
  } tweak, scratch;
  unsigned int i;

  if (len < 16) return 0;

  OPENSSL_memcpy(tweak.c, iv, 16);

  (*ctx->block2)(tweak.c, tweak.c, ctx->key2);

  if (!enc && (len % 16)) len -= 16;

  while (len >= 16) {
    OPENSSL_memcpy(scratch.c, inp, 16);
    scratch.u[0] ^= tweak.u[0];
    scratch.u[1] ^= tweak.u[1];
    (*ctx->block1)(scratch.c, scratch.c, ctx->key1);
    scratch.u[0] ^= tweak.u[0];
    scratch.u[1] ^= tweak.u[1];
    OPENSSL_memcpy(out, scratch.c, 16);
    inp += 16;
    out += 16;
    len -= 16;

    if (len == 0) return 1;

    unsigned int carry, res;

#if defined(OPENSSL_BIG_ENDIAN)
    uint64_t tweak_u0, tweak_u1;
    tweak_u0 = CRYPTO_load_u64_le(&tweak.u[0]);
    tweak_u1 = CRYPTO_load_u64_le(&tweak.u[1]);
    res = 0x87 & (((int64_t)tweak_u1) >> 63);
    carry = (unsigned int)(tweak_u0 >> 63);
    tweak_u0 = (tweak_u0 << 1) ^ res;
    tweak_u1 = (tweak_u1 << 1) | carry;    
    CRYPTO_store_u64_le(&tweak.u[0], tweak_u0);
    CRYPTO_store_u64_le(&tweak.u[1], tweak_u1);
#else
    res = 0x87 & (((int64_t)tweak.u[1]) >> 63);
    carry = (unsigned int)(tweak.u[0] >> 63);
    tweak.u[0] = (tweak.u[0] << 1) ^ res;
    tweak.u[1] = (tweak.u[1] << 1) | carry;
#endif
  }
  if (enc) {
    for (i = 0; i < len; ++i) {
      uint8_t c = inp[i];
      out[i] = scratch.c[i];
      scratch.c[i] = c;
    }
    scratch.u[0] ^= tweak.u[0];
    scratch.u[1] ^= tweak.u[1];
    (*ctx->block1)(scratch.c, scratch.c, ctx->key1);
    scratch.u[0] ^= tweak.u[0];
    scratch.u[1] ^= tweak.u[1];
    OPENSSL_memcpy(out - 16, scratch.c, 16);
  } else {
    union {
      uint64_t u[2];
      uint8_t c[16];
    } tweak1;

    unsigned int carry, res;

#if defined(OPENSSL_BIG_ENDIAN)
    uint64_t tweak_u0, tweak_u1;
    tweak_u0 = CRYPTO_load_u64_le(&tweak.u[0]);
    tweak_u1 = CRYPTO_load_u64_le(&tweak.u[1]);
    res = 0x87 & (((int64_t)tweak_u1) >> 63);
    carry = (unsigned int)(tweak_u0 >> 63);
    tweak_u0 = (tweak_u0 << 1) ^ res;
    tweak_u1 = (tweak_u1 << 1) | carry;    
    CRYPTO_store_u64_le(&tweak1.u[0], tweak_u0);
    CRYPTO_store_u64_le(&tweak1.u[1], tweak_u1);
#else
    res = 0x87 & (((int64_t)tweak.u[1]) >> 63);
    carry = (unsigned int)(tweak.u[0] >> 63);
    tweak1.u[0] = (tweak.u[0] << 1) ^ res;
    tweak1.u[1] = (tweak.u[1] << 1) | carry;
#endif
    OPENSSL_memcpy(scratch.c, inp, 16);
    scratch.u[0] ^= tweak1.u[0];
    scratch.u[1] ^= tweak1.u[1];
    (*ctx->block1)(scratch.c, scratch.c, ctx->key1);
    scratch.u[0] ^= tweak1.u[0];
    scratch.u[1] ^= tweak1.u[1];

    for (i = 0; i < len; ++i) {
      uint8_t c = inp[16 + i];
      out[16 + i] = scratch.c[i];
      scratch.c[i] = c;
    }
    scratch.u[0] ^= tweak.u[0];
    scratch.u[1] ^= tweak.u[1];
    (*ctx->block1)(scratch.c, scratch.c, ctx->key1);
    scratch.u[0] ^= tweak.u[0];
    scratch.u[1] ^= tweak.u[1];
    OPENSSL_memcpy(out, scratch.c, 16);
  }

  return 1;
}
