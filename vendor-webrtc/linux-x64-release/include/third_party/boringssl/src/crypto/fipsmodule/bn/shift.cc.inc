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
#include <string.h>

#include <openssl/err.h>

#include "internal.h"


int BN_lshift(BIGNUM *r, const BIGNUM *a, int n) {
  int i, nw, lb, rb;
  BN_ULONG *t, *f;
  BN_ULONG l;

  if (n < 0) {
    OPENSSL_PUT_ERROR(BN, BN_R_NEGATIVE_NUMBER);
    return 0;
  }

  r->neg = a->neg;
  nw = n / BN_BITS2;
  if (!bn_wexpand(r, a->width + nw + 1)) {
    return 0;
  }
  lb = n % BN_BITS2;
  rb = BN_BITS2 - lb;
  f = a->d;
  t = r->d;
  t[a->width + nw] = 0;
  if (lb == 0) {
    for (i = a->width - 1; i >= 0; i--) {
      t[nw + i] = f[i];
    }
  } else {
    for (i = a->width - 1; i >= 0; i--) {
      l = f[i];
      t[nw + i + 1] |= l >> rb;
      t[nw + i] = l << lb;
    }
  }
  OPENSSL_memset(t, 0, nw * sizeof(t[0]));
  r->width = a->width + nw + 1;
  bn_set_minimal_width(r);

  return 1;
}

int BN_lshift1(BIGNUM *r, const BIGNUM *a) {
  BN_ULONG *ap, *rp, t, c;
  int i;

  if (r != a) {
    r->neg = a->neg;
    if (!bn_wexpand(r, a->width + 1)) {
      return 0;
    }
    r->width = a->width;
  } else {
    if (!bn_wexpand(r, a->width + 1)) {
      return 0;
    }
  }
  ap = a->d;
  rp = r->d;
  c = 0;
  for (i = 0; i < a->width; i++) {
    t = *(ap++);
    *(rp++) = (t << 1) | c;
    c = t >> (BN_BITS2 - 1);
  }
  if (c) {
    *rp = 1;
    r->width++;
  }

  return 1;
}

void bn_rshift_words(BN_ULONG *r, const BN_ULONG *a, unsigned shift,
                     size_t num) {
  unsigned shift_bits = shift % BN_BITS2;
  size_t shift_words = shift / BN_BITS2;
  if (shift_words >= num) {
    OPENSSL_memset(r, 0, num * sizeof(BN_ULONG));
    return;
  }
  if (shift_bits == 0) {
    OPENSSL_memmove(r, a + shift_words, (num - shift_words) * sizeof(BN_ULONG));
  } else {
    for (size_t i = shift_words; i < num - 1; i++) {
      r[i - shift_words] =
          (a[i] >> shift_bits) | (a[i + 1] << (BN_BITS2 - shift_bits));
    }
    r[num - 1 - shift_words] = a[num - 1] >> shift_bits;
  }
  OPENSSL_memset(r + num - shift_words, 0, shift_words * sizeof(BN_ULONG));
}

int BN_rshift(BIGNUM *r, const BIGNUM *a, int n) {
  if (n < 0) {
    OPENSSL_PUT_ERROR(BN, BN_R_NEGATIVE_NUMBER);
    return 0;
  }

  if (!bn_wexpand(r, a->width)) {
    return 0;
  }
  bn_rshift_words(r->d, a->d, n, a->width);
  r->neg = a->neg;
  r->width = a->width;
  bn_set_minimal_width(r);
  return 1;
}

int bn_rshift_secret_shift(BIGNUM *r, const BIGNUM *a, unsigned n,
                           BN_CTX *ctx) {
  bssl::BN_CTXScope scope(ctx);
  BIGNUM *tmp = BN_CTX_get(ctx);
  unsigned max_bits;
  if (tmp == NULL || !BN_copy(r, a) || !bn_wexpand(tmp, r->width)) {
    return 0;
  }

  // Shift conditionally by powers of two.
  max_bits = BN_BITS2 * r->width;
  for (unsigned i = 0; (max_bits >> i) != 0; i++) {
    BN_ULONG mask = (n >> i) & 1;
    mask = 0 - mask;
    bn_rshift_words(tmp->d, r->d, 1u << i, r->width);
    bn_select_words(r->d, mask, tmp->d /* apply shift */,
                    r->d /* ignore shift */, r->width);
  }

  return 1;
}

void bn_rshift1_words(BN_ULONG *r, const BN_ULONG *a, size_t num) {
  if (num == 0) {
    return;
  }
  for (size_t i = 0; i < num - 1; i++) {
    r[i] = (a[i] >> 1) | (a[i + 1] << (BN_BITS2 - 1));
  }
  r[num - 1] = a[num - 1] >> 1;
}

int BN_rshift1(BIGNUM *r, const BIGNUM *a) {
  if (!bn_wexpand(r, a->width)) {
    return 0;
  }
  bn_rshift1_words(r->d, a->d, a->width);
  r->width = a->width;
  r->neg = a->neg;
  bn_set_minimal_width(r);
  return 1;
}

int BN_set_bit(BIGNUM *a, int n) {
  if (n < 0) {
    return 0;
  }

  int i = n / BN_BITS2;
  int j = n % BN_BITS2;
  if (a->width <= i) {
    if (!bn_wexpand(a, i + 1)) {
      return 0;
    }
    for (int k = a->width; k < i + 1; k++) {
      a->d[k] = 0;
    }
    a->width = i + 1;
  }

  a->d[i] |= (((BN_ULONG)1) << j);

  return 1;
}

int BN_clear_bit(BIGNUM *a, int n) {
  int i, j;

  if (n < 0) {
    return 0;
  }

  i = n / BN_BITS2;
  j = n % BN_BITS2;
  if (a->width <= i) {
    return 0;
  }

  a->d[i] &= (~(((BN_ULONG)1) << j));
  bn_set_minimal_width(a);
  return 1;
}

int bn_is_bit_set_words(const BN_ULONG *a, size_t num, size_t bit) {
  size_t i = bit / BN_BITS2;
  size_t j = bit % BN_BITS2;
  if (i >= num) {
    return 0;
  }
  return (a[i] >> j) & 1;
}

int BN_is_bit_set(const BIGNUM *a, int n) {
  if (n < 0) {
    return 0;
  }
  return bn_is_bit_set_words(a->d, a->width, n);
}

int BN_mask_bits(BIGNUM *a, int n) {
  if (n < 0) {
    return 0;
  }

  int w = n / BN_BITS2;
  int b = n % BN_BITS2;
  if (w >= a->width) {
    return 1;
  }
  if (b == 0) {
    a->width = w;
  } else {
    a->width = w + 1;
    a->d[w] &= ~(BN_MASK2 << b);
  }

  bn_set_minimal_width(a);
  return 1;
}

static int bn_count_low_zero_bits_word(BN_ULONG l) {
  static_assert(sizeof(BN_ULONG) <= sizeof(crypto_word_t),
                "crypto_word_t is too small");
  static_assert(sizeof(int) <= sizeof(crypto_word_t),
                "crypto_word_t is too small");
  static_assert(BN_BITS2 == sizeof(BN_ULONG) * 8, "BN_ULONG has padding bits");
  // C has very bizarre rules for types smaller than an int.
  static_assert(sizeof(BN_ULONG) >= sizeof(int),
                "BN_ULONG gets promoted to int");

  crypto_word_t mask;
  int bits = 0;

#if BN_BITS2 > 32
  // Check if the lower half of |x| are all zero.
  mask = constant_time_is_zero_w(l << (BN_BITS2 - 32));
  // If the lower half is all zeros, it is included in the bit count and we
  // count the upper half. Otherwise, we count the lower half.
  bits += 32 & mask;
  l = constant_time_select_w(mask, l >> 32, l);
#endif

  // The remaining blocks are analogous iterations at lower powers of two.
  mask = constant_time_is_zero_w(l << (BN_BITS2 - 16));
  bits += 16 & mask;
  l = constant_time_select_w(mask, l >> 16, l);

  mask = constant_time_is_zero_w(l << (BN_BITS2 - 8));
  bits += 8 & mask;
  l = constant_time_select_w(mask, l >> 8, l);

  mask = constant_time_is_zero_w(l << (BN_BITS2 - 4));
  bits += 4 & mask;
  l = constant_time_select_w(mask, l >> 4, l);

  mask = constant_time_is_zero_w(l << (BN_BITS2 - 2));
  bits += 2 & mask;
  l = constant_time_select_w(mask, l >> 2, l);

  mask = constant_time_is_zero_w(l << (BN_BITS2 - 1));
  bits += 1 & mask;

  return bits;
}

int BN_count_low_zero_bits(const BIGNUM *bn) {
  static_assert(sizeof(BN_ULONG) <= sizeof(crypto_word_t),
                "crypto_word_t is too small");
  static_assert(sizeof(int) <= sizeof(crypto_word_t),
                "crypto_word_t is too small");

  int ret = 0;
  crypto_word_t saw_nonzero = 0;
  for (int i = 0; i < bn->width; i++) {
    crypto_word_t nonzero = ~constant_time_is_zero_w(bn->d[i]);
    crypto_word_t first_nonzero = ~saw_nonzero & nonzero;
    saw_nonzero |= nonzero;

    int bits = bn_count_low_zero_bits_word(bn->d[i]);
    ret |= first_nonzero & (i * BN_BITS2 + bits);
  }

  // If got to the end of |bn| and saw no non-zero words, |bn| is zero. |ret|
  // will then remain zero.
  return ret;
}
