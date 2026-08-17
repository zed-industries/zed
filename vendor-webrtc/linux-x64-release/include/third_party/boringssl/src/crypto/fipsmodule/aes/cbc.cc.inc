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


void CRYPTO_cbc128_encrypt(const uint8_t *in, uint8_t *out, size_t len,
                           const AES_KEY *key, uint8_t ivec[16],
                           block128_f block) {
  assert(key != NULL && ivec != NULL);
  if (len == 0) {
    // Avoid |ivec| == |iv| in the |memcpy| below, which is not legal in C.
    return;
  }

  assert(in != NULL && out != NULL);
  size_t n;
  const uint8_t *iv = ivec;
  while (len >= 16) {
    CRYPTO_xor16(out, in, iv);
    (*block)(out, out, key);
    iv = out;
    len -= 16;
    in += 16;
    out += 16;
  }

  while (len) {
    for (n = 0; n < 16 && n < len; ++n) {
      out[n] = in[n] ^ iv[n];
    }
    for (; n < 16; ++n) {
      out[n] = iv[n];
    }
    (*block)(out, out, key);
    iv = out;
    if (len <= 16) {
      break;
    }
    len -= 16;
    in += 16;
    out += 16;
  }

  OPENSSL_memcpy(ivec, iv, 16);
}

void CRYPTO_cbc128_decrypt(const uint8_t *in, uint8_t *out, size_t len,
                           const AES_KEY *key, uint8_t ivec[16],
                           block128_f block) {
  assert(key != NULL && ivec != NULL);
  if (len == 0) {
    // Avoid |ivec| == |iv| in the |memcpy| below, which is not legal in C.
    return;
  }

  assert(in != NULL && out != NULL);

  const uintptr_t inptr = (uintptr_t) in;
  const uintptr_t outptr = (uintptr_t) out;
  // If |in| and |out| alias, |in| must be ahead.
  assert(inptr >= outptr || inptr + len <= outptr);

  size_t n;
  alignas(16) uint8_t tmp[16];
  if ((inptr >= 32 && outptr <= inptr - 32) || inptr < outptr) {
    // If |out| is at least two blocks behind |in| or completely disjoint, there
    // is no need to decrypt to a temporary block.
    const uint8_t *iv = ivec;
    while (len >= 16) {
      (*block)(in, out, key);
      CRYPTO_xor16(out, out, iv);
      iv = in;
      len -= 16;
      in += 16;
      out += 16;
    }
    OPENSSL_memcpy(ivec, iv, 16);
  } else {
    static_assert(16 % sizeof(crypto_word_t) == 0,
                  "block cannot be evenly divided into words");

    while (len >= 16) {
      (*block)(in, tmp, key);
      for (n = 0; n < 16; n += sizeof(crypto_word_t)) {
        crypto_word_t c = CRYPTO_load_word_le(in + n);
        CRYPTO_store_word_le(out + n, CRYPTO_load_word_le(tmp + n) ^
                                          CRYPTO_load_word_le(ivec + n));
        CRYPTO_store_word_le(ivec + n, c);
      }
      len -= 16;
      in += 16;
      out += 16;
    }
  }

  while (len) {
    uint8_t c;
    (*block)(in, tmp, key);
    for (n = 0; n < 16 && n < len; ++n) {
      c = in[n];
      out[n] = tmp[n] ^ ivec[n];
      ivec[n] = c;
    }
    if (len <= 16) {
      for (; n < 16; ++n) {
        ivec[n] = in[n];
      }
      break;
    }
    len -= 16;
    in += 16;
    out += 16;
  }
}
