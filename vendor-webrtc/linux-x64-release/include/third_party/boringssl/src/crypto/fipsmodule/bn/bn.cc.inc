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

#include <openssl/bn.h>

#include <assert.h>
#include <limits.h>
#include <string.h>

#include <openssl/err.h>
#include <openssl/mem.h>

#include "../delocate.h"
#include "internal.h"


// BN_MAX_WORDS is the maximum number of words allowed in a |BIGNUM|. It is
// sized so byte and bit counts of a |BIGNUM| always fit in |int|, with room to
// spare.
#define BN_MAX_WORDS (INT_MAX / (4 * BN_BITS2))

BIGNUM *BN_new(void) {
  BIGNUM *bn = reinterpret_cast<BIGNUM *>(OPENSSL_malloc(sizeof(BIGNUM)));

  if (bn == NULL) {
    return NULL;
  }

  OPENSSL_memset(bn, 0, sizeof(BIGNUM));
  bn->flags = BN_FLG_MALLOCED;

  return bn;
}

BIGNUM *BN_secure_new(void) { return BN_new(); }

void BN_init(BIGNUM *bn) { OPENSSL_memset(bn, 0, sizeof(BIGNUM)); }

void BN_free(BIGNUM *bn) {
  if (bn == NULL) {
    return;
  }

  if ((bn->flags & BN_FLG_STATIC_DATA) == 0) {
    OPENSSL_free(bn->d);
  }

  if (bn->flags & BN_FLG_MALLOCED) {
    OPENSSL_free(bn);
  } else {
    bn->d = NULL;
  }
}

void BN_clear_free(BIGNUM *bn) { BN_free(bn); }

BIGNUM *BN_dup(const BIGNUM *src) {
  BIGNUM *copy;

  if (src == NULL) {
    return NULL;
  }

  copy = BN_new();
  if (copy == NULL) {
    return NULL;
  }

  if (!BN_copy(copy, src)) {
    BN_free(copy);
    return NULL;
  }

  return copy;
}

BIGNUM *BN_copy(BIGNUM *dest, const BIGNUM *src) {
  if (src == dest) {
    return dest;
  }

  if (!bn_wexpand(dest, src->width)) {
    return NULL;
  }

  OPENSSL_memcpy(dest->d, src->d, sizeof(src->d[0]) * src->width);

  dest->width = src->width;
  dest->neg = src->neg;
  return dest;
}

void BN_clear(BIGNUM *bn) {
  if (bn->d != NULL) {
    OPENSSL_memset(bn->d, 0, bn->dmax * sizeof(bn->d[0]));
  }

  bn->width = 0;
  bn->neg = 0;
}

DEFINE_METHOD_FUNCTION(BIGNUM, BN_value_one) {
  static const BN_ULONG kOneLimbs[1] = {1};
  out->d = (BN_ULONG *)kOneLimbs;
  out->width = 1;
  out->dmax = 1;
  out->neg = 0;
  out->flags = BN_FLG_STATIC_DATA;
}

// BN_num_bits_word returns the minimum number of bits needed to represent the
// value in |l|.
unsigned BN_num_bits_word(BN_ULONG l) {
  // |BN_num_bits| is often called on RSA prime factors. These have public bit
  // lengths, but all bits beyond the high bit are secret, so count bits in
  // constant time.
  BN_ULONG x, mask;
  int bits = (l != 0);

#if BN_BITS2 > 32
  // Look at the upper half of |x|. |x| is at most 64 bits long.
  x = l >> 32;
  // Set |mask| to all ones if |x| (the top 32 bits of |l|) is non-zero and all
  // all zeros otherwise.
  mask = 0u - x;
  mask = (0u - (mask >> (BN_BITS2 - 1)));
  // If |x| is non-zero, the lower half is included in the bit count in full,
  // and we count the upper half. Otherwise, we count the lower half.
  bits += 32 & mask;
  l ^= (x ^ l) & mask;  // |l| is |x| if |mask| and remains |l| otherwise.
#endif

  // The remaining blocks are analogous iterations at lower powers of two.
  x = l >> 16;
  mask = 0u - x;
  mask = (0u - (mask >> (BN_BITS2 - 1)));
  bits += 16 & mask;
  l ^= (x ^ l) & mask;

  x = l >> 8;
  mask = 0u - x;
  mask = (0u - (mask >> (BN_BITS2 - 1)));
  bits += 8 & mask;
  l ^= (x ^ l) & mask;

  x = l >> 4;
  mask = 0u - x;
  mask = (0u - (mask >> (BN_BITS2 - 1)));
  bits += 4 & mask;
  l ^= (x ^ l) & mask;

  x = l >> 2;
  mask = 0u - x;
  mask = (0u - (mask >> (BN_BITS2 - 1)));
  bits += 2 & mask;
  l ^= (x ^ l) & mask;

  x = l >> 1;
  mask = 0u - x;
  mask = (0u - (mask >> (BN_BITS2 - 1)));
  bits += 1 & mask;

  return bits;
}

unsigned BN_num_bits(const BIGNUM *bn) {
  const int width = bn_minimal_width(bn);
  if (width == 0) {
    return 0;
  }

  return (width - 1) * BN_BITS2 + BN_num_bits_word(bn->d[width - 1]);
}

unsigned BN_num_bytes(const BIGNUM *bn) { return (BN_num_bits(bn) + 7) / 8; }

void BN_zero(BIGNUM *bn) { bn->width = bn->neg = 0; }

int BN_one(BIGNUM *bn) { return BN_set_word(bn, 1); }

int BN_set_word(BIGNUM *bn, BN_ULONG value) {
  if (value == 0) {
    BN_zero(bn);
    return 1;
  }

  if (!bn_wexpand(bn, 1)) {
    return 0;
  }

  bn->neg = 0;
  bn->d[0] = value;
  bn->width = 1;
  return 1;
}

int BN_set_u64(BIGNUM *bn, uint64_t value) {
#if BN_BITS2 == 64
  return BN_set_word(bn, value);
#elif BN_BITS2 == 32
  if (value <= BN_MASK2) {
    return BN_set_word(bn, (BN_ULONG)value);
  }

  if (!bn_wexpand(bn, 2)) {
    return 0;
  }

  bn->neg = 0;
  bn->d[0] = (BN_ULONG)value;
  bn->d[1] = (BN_ULONG)(value >> 32);
  bn->width = 2;
  return 1;
#else
#error "BN_BITS2 must be 32 or 64."
#endif
}

int bn_set_words(BIGNUM *bn, const BN_ULONG *words, size_t num) {
  if (!bn_wexpand(bn, num)) {
    return 0;
  }
  OPENSSL_memmove(bn->d, words, num * sizeof(BN_ULONG));
  // |bn_wexpand| verified that |num| isn't too large.
  bn->width = (int)num;
  bn->neg = 0;
  return 1;
}

void bn_set_static_words(BIGNUM *bn, const BN_ULONG *words, size_t num) {
  if ((bn->flags & BN_FLG_STATIC_DATA) == 0) {
    OPENSSL_free(bn->d);
  }
  bn->d = (BN_ULONG *)words;

  assert(num <= BN_MAX_WORDS);
  bn->width = (int)num;
  bn->dmax = (int)num;
  bn->neg = 0;
  bn->flags |= BN_FLG_STATIC_DATA;
}

int bn_fits_in_words(const BIGNUM *bn, size_t num) {
  // All words beyond |num| must be zero.
  BN_ULONG mask = 0;
  for (size_t i = num; i < (size_t)bn->width; i++) {
    mask |= bn->d[i];
  }
  return mask == 0;
}

int bn_copy_words(BN_ULONG *out, size_t num, const BIGNUM *bn) {
  if (bn->neg) {
    OPENSSL_PUT_ERROR(BN, BN_R_NEGATIVE_NUMBER);
    return 0;
  }

  size_t width = (size_t)bn->width;
  if (width > num) {
    if (!bn_fits_in_words(bn, num)) {
      OPENSSL_PUT_ERROR(BN, BN_R_BIGNUM_TOO_LONG);
      return 0;
    }
    width = num;
  }

  OPENSSL_memset(out, 0, sizeof(BN_ULONG) * num);
  OPENSSL_memcpy(out, bn->d, sizeof(BN_ULONG) * width);
  return 1;
}

int BN_is_negative(const BIGNUM *bn) { return bn->neg != 0; }

void BN_set_negative(BIGNUM *bn, int sign) {
  if (sign && !BN_is_zero(bn)) {
    bn->neg = 1;
  } else {
    bn->neg = 0;
  }
}

int bn_wexpand(BIGNUM *bn, size_t words) {
  BN_ULONG *a;

  if (words <= (size_t)bn->dmax) {
    return 1;
  }

  if (words > BN_MAX_WORDS) {
    OPENSSL_PUT_ERROR(BN, BN_R_BIGNUM_TOO_LONG);
    return 0;
  }

  if (bn->flags & BN_FLG_STATIC_DATA) {
    OPENSSL_PUT_ERROR(BN, BN_R_EXPAND_ON_STATIC_BIGNUM_DATA);
    return 0;
  }

  a = reinterpret_cast<BN_ULONG *>(OPENSSL_calloc(words, sizeof(BN_ULONG)));
  if (a == NULL) {
    return 0;
  }

  OPENSSL_memcpy(a, bn->d, sizeof(BN_ULONG) * bn->width);

  OPENSSL_free(bn->d);
  bn->d = a;
  bn->dmax = (int)words;

  return 1;
}

int bn_expand(BIGNUM *bn, size_t bits) {
  if (bits + BN_BITS2 - 1 < bits) {
    OPENSSL_PUT_ERROR(BN, BN_R_BIGNUM_TOO_LONG);
    return 0;
  }
  return bn_wexpand(bn, (bits + BN_BITS2 - 1) / BN_BITS2);
}

int bn_resize_words(BIGNUM *bn, size_t words) {
  if ((size_t)bn->width <= words) {
    if (!bn_wexpand(bn, words)) {
      return 0;
    }
    OPENSSL_memset(bn->d + bn->width, 0,
                   (words - bn->width) * sizeof(BN_ULONG));
    bn->width = (int)words;
    return 1;
  }

  // All words beyond the new width must be zero.
  if (!bn_fits_in_words(bn, words)) {
    OPENSSL_PUT_ERROR(BN, BN_R_BIGNUM_TOO_LONG);
    return 0;
  }
  bn->width = (int)words;
  return 1;
}

void bn_select_words(BN_ULONG *r, BN_ULONG mask, const BN_ULONG *a,
                     const BN_ULONG *b, size_t num) {
  for (size_t i = 0; i < num; i++) {
    static_assert(sizeof(BN_ULONG) <= sizeof(crypto_word_t),
                  "crypto_word_t is too small");
    r[i] = constant_time_select_w(mask, a[i], b[i]);
  }
}

int bn_minimal_width(const BIGNUM *bn) {
  int ret = bn->width;
  while (ret > 0 && bn->d[ret - 1] == 0) {
    ret--;
  }
  return ret;
}

void bn_set_minimal_width(BIGNUM *bn) {
  bn->width = bn_minimal_width(bn);
  if (bn->width == 0) {
    bn->neg = 0;
  }
}
