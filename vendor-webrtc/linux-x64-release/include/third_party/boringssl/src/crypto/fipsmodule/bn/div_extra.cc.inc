// Copyright 2018 The BoringSSL Authors
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

#include <openssl/bn.h>

#include <assert.h>

#include "internal.h"


// The following functions use a Barrett reduction variant to avoid leaking the
// numerator. See http://ridiculousfish.com/blog/posts/labor-of-division-episode-i.html
//
// We use 32-bit numerator and 16-bit divisor for simplicity. This allows
// computing |m| and |q| without architecture-specific code.

// mod_u16 returns |n| mod |d|. |p| and |m| are the "magic numbers" for |d| (see
// reference). For proof of correctness in Coq, see
// https://github.com/davidben/fiat-crypto/blob/barrett/src/Arithmetic/BarrettReduction/RidiculousFish.v
// Note the Coq version of |mod_u16| additionally includes the computation of
// |p| and |m| from |bn_mod_u16_consttime| below.
static uint16_t mod_u16(uint32_t n, uint16_t d, uint32_t p, uint32_t m) {
  // Compute floor(n/d) per steps 3 through 5.
  uint32_t q = ((uint64_t)m * n) >> 32;
  // Note there is a typo in the reference. We right-shift by one, not two.
  uint32_t t = ((n - q) >> 1) + q;
  t = t >> (p - 1);

  // Multiply and subtract to get the remainder.
  n -= d * t;
  declassify_assert(n < d);
  return n;
}

// shift_and_add_mod_u16 returns |r| * 2^32 + |a| mod |d|. |p| and |m| are the
// "magic numbers" for |d| (see reference).
static uint16_t shift_and_add_mod_u16(uint16_t r, uint32_t a, uint16_t d,
                                      uint32_t p, uint32_t m) {
  // Incorporate |a| in two 16-bit chunks.
  uint32_t t = r;
  t <<= 16;
  t |= a >> 16;
  t = mod_u16(t, d, p, m);

  t <<= 16;
  t |= a & 0xffff;
  t = mod_u16(t, d, p, m);
  return t;
}

uint16_t bn_mod_u16_consttime(const BIGNUM *bn, uint16_t d) {
  if (d <= 1) {
    return 0;
  }

  // Compute the "magic numbers" for |d|. See steps 1 and 2.
  // This computes p = ceil(log_2(d)).
  uint32_t p = BN_num_bits_word(d - 1);
  // This operation is not constant-time, but |p| and |d| are public values.
  // Note that |p| is at most 16, so the computation fits in |uint64_t|.
  assert(p <= 16);
  uint32_t m = (uint32_t)(((UINT64_C(1) << (32 + p)) + d - 1) / d);

  uint16_t ret = 0;
  for (int i = bn->width - 1; i >= 0; i--) {
#if BN_BITS2 == 32
    ret = shift_and_add_mod_u16(ret, bn->d[i], d, p, m);
#elif BN_BITS2 == 64
    ret = shift_and_add_mod_u16(ret, bn->d[i] >> 32, d, p, m);
    ret = shift_and_add_mod_u16(ret, bn->d[i] & 0xffffffff, d, p, m);
#else
#error "Unknown BN_ULONG size"
#endif
  }
  return ret;
}
