// Copyright 2008-2016 The OpenSSL Project Authors. All Rights Reserved.
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

#include <assert.h>
#include <string.h>

#include "internal.h"
#include "../../internal.h"


static_assert(16 % sizeof(crypto_word_t) == 0,
              "block cannot be divided into crypto_word_t");

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
