// Copyright (c) 2008 The OpenSSL Project.  All rights reserved.
// SPDX-License-Identifier: Apache-2.0

#include <openssl/type_check.h>

#include <assert.h>
#include <string.h>

#include "internal.h"
#include "../../internal.h"


// NOTE: the IV/counter CTR mode is big-endian.  The code itself
// is endian-neutral.

// increment counter (128-bit int) by 1
static void ctr128_inc(uint8_t *counter) {
  uint32_t n = 16, c = 1;

  do {
    --n;
    c += counter[n];
    counter[n] = (uint8_t) c;
    c >>= 8;
  } while (n);
}

OPENSSL_STATIC_ASSERT(16 % sizeof(crypto_word_t) == 0,
                      ctr_block_cannot_be_divided_into_crypto_word_t)

// The input encrypted as though 128bit counter mode is being used.  The extra
// state information to record how much of the 128bit block we have used is
// contained in *num, and the encrypted counter is kept in ecount_buf.  Both
// *num and ecount_buf must be initialised with zeros before the first call to
// CRYPTO_ctr128_encrypt().
//
// This algorithm assumes that the counter is in the x lower bits of the IV
// (ivec), and that the application has full control over overflow and the rest
// of the IV.  This implementation takes NO responsibility for checking that
// the counter doesn't overflow into the rest of the IV when incremented.
void CRYPTO_ctr128_encrypt(const uint8_t *in, uint8_t *out, size_t len,
                           const AES_KEY *key, uint8_t ivec[16],
                           uint8_t ecount_buf[16], unsigned int *num,
                           block128_f block) {
  unsigned int n;

  assert(key && ecount_buf && num);
  assert(len == 0 || (in && out));
  assert(*num < 16);

  n = *num;

  while (n && len) {
    *(out++) = *(in++) ^ ecount_buf[n];
    --len;
    n = (n + 1) % 16;
  }
  while (len >= 16) {
    (*block)(ivec, ecount_buf, key);
    ctr128_inc(ivec);
    CRYPTO_xor16(out, in, ecount_buf);
    len -= 16;
    out += 16;
    in += 16;
    n = 0;
  }
  if (len) {
    (*block)(ivec, ecount_buf, key);
    ctr128_inc(ivec);
    while (len--) {
      out[n] = in[n] ^ ecount_buf[n];
      ++n;
    }
  }
  *num = n;
}

// increment upper 96 bits of 128-bit counter by 1
static void ctr96_inc(uint8_t *counter) {
  uint32_t n = 12, c = 1;

  do {
    --n;
    c += counter[n];
    counter[n] = (uint8_t) c;
    c >>= 8;
  } while (n);
}

void CRYPTO_ctr128_encrypt_ctr32(const uint8_t *in, uint8_t *out, size_t len,
                                 const AES_KEY *key, uint8_t ivec[16],
                                 uint8_t ecount_buf[16], unsigned int *num,
                                 ctr128_f func) {
  unsigned int n, ctr32;

  assert(key && ecount_buf && num);
  assert(len == 0 || (in && out));
  assert(*num < 16);

  n = *num;

  while (n && len) {
    *(out++) = *(in++) ^ ecount_buf[n];
    --len;
    n = (n + 1) % 16;
  }

  ctr32 = CRYPTO_load_u32_be(ivec + 12);
  while (len >= 16) {
    size_t blocks = len / 16;
    // 1<<28 is just a not-so-small yet not-so-large number...
    // Below condition is practically never met, but it has to
    // be checked for code correctness.
    if (sizeof(size_t) > sizeof(unsigned int) && blocks > (1U << 28)) {
      blocks = (1U << 28);
    }
    // As (*func) operates on 32-bit counter, caller
    // has to handle overflow. 'if' below detects the
    // overflow, which is then handled by limiting the
    // amount of blocks to the exact overflow point...
    ctr32 += (uint32_t)blocks;
    if (ctr32 < blocks) {
      blocks -= ctr32;
      ctr32 = 0;
    }
    (*func)(in, out, blocks, key, ivec);
    // (*func) does not update ivec, caller does:
    CRYPTO_store_u32_be(ivec + 12, ctr32);
    // ... overflow was detected, propogate carry.
    if (ctr32 == 0) {
      ctr96_inc(ivec);
    }
    blocks *= 16;
    len -= blocks;
    out += blocks;
    in += blocks;
  }
  if (len) {
    OPENSSL_memset(ecount_buf, 0, 16);
    (*func)(ecount_buf, ecount_buf, 1, key, ivec);
    ++ctr32;
    CRYPTO_store_u32_be(ivec + 12, ctr32);
    if (ctr32 == 0) {
      ctr96_inc(ivec);
    }
    while (len--) {
      out[n] = in[n] ^ ecount_buf[n];
      ++n;
    }
  }

  *num = n;
}
