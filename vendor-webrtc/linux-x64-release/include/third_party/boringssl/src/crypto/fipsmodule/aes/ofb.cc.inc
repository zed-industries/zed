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


static_assert(16 % sizeof(size_t) == 0, "block cannot be divided into size_t");

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
