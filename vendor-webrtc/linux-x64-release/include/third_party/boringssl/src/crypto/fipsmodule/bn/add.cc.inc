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

#include <string.h>

#include <openssl/err.h>
#include <openssl/mem.h>

#include "../../internal.h"
#include "internal.h"


int BN_add(BIGNUM *r, const BIGNUM *a, const BIGNUM *b) {
  const BIGNUM *tmp;
  int a_neg = a->neg, ret;

  //  a +  b	a+b
  //  a + -b	a-b
  // -a +  b	b-a
  // -a + -b	-(a+b)
  if (a_neg ^ b->neg) {
    // only one is negative
    if (a_neg) {
      tmp = a;
      a = b;
      b = tmp;
    }

    // we are now a - b
    if (BN_ucmp(a, b) < 0) {
      if (!BN_usub(r, b, a)) {
        return 0;
      }
      r->neg = 1;
    } else {
      if (!BN_usub(r, a, b)) {
        return 0;
      }
      r->neg = 0;
    }
    return 1;
  }

  ret = BN_uadd(r, a, b);
  r->neg = a_neg;
  return ret;
}

int bn_uadd_consttime(BIGNUM *r, const BIGNUM *a, const BIGNUM *b) {
  // Widths are public, so we normalize to make |a| the larger one.
  if (a->width < b->width) {
    const BIGNUM *tmp = a;
    a = b;
    b = tmp;
  }

  int max = a->width;
  int min = b->width;
  if (!bn_wexpand(r, max + 1)) {
    return 0;
  }
  r->width = max + 1;

  BN_ULONG carry = bn_add_words(r->d, a->d, b->d, min);
  for (int i = min; i < max; i++) {
    r->d[i] = CRYPTO_addc_w(a->d[i], 0, carry, &carry);
  }

  r->d[max] = carry;
  return 1;
}

int BN_uadd(BIGNUM *r, const BIGNUM *a, const BIGNUM *b) {
  if (!bn_uadd_consttime(r, a, b)) {
    return 0;
  }
  bn_set_minimal_width(r);
  return 1;
}

int BN_add_word(BIGNUM *a, BN_ULONG w) {
  BN_ULONG l;
  int i;

  // degenerate case: w is zero
  if (!w) {
    return 1;
  }

  // degenerate case: a is zero
  if (BN_is_zero(a)) {
    return BN_set_word(a, w);
  }

  // handle 'a' when negative
  if (a->neg) {
    a->neg = 0;
    i = BN_sub_word(a, w);
    if (!BN_is_zero(a)) {
      a->neg = !(a->neg);
    }
    return i;
  }

  for (i = 0; w != 0 && i < a->width; i++) {
    a->d[i] = l = a->d[i] + w;
    w = (w > l) ? 1 : 0;
  }

  if (w && i == a->width) {
    if (!bn_wexpand(a, a->width + 1)) {
      return 0;
    }
    a->width++;
    a->d[i] = w;
  }

  return 1;
}

int BN_sub(BIGNUM *r, const BIGNUM *a, const BIGNUM *b) {
  int add = 0, neg = 0;
  const BIGNUM *tmp;

  //  a -  b	a-b
  //  a - -b	a+b
  // -a -  b	-(a+b)
  // -a - -b	b-a
  if (a->neg) {
    if (b->neg) {
      tmp = a;
      a = b;
      b = tmp;
    } else {
      add = 1;
      neg = 1;
    }
  } else {
    if (b->neg) {
      add = 1;
      neg = 0;
    }
  }

  if (add) {
    if (!BN_uadd(r, a, b)) {
      return 0;
    }

    r->neg = neg;
    return 1;
  }

  if (BN_ucmp(a, b) < 0) {
    if (!BN_usub(r, b, a)) {
      return 0;
    }
    r->neg = 1;
  } else {
    if (!BN_usub(r, a, b)) {
      return 0;
    }
    r->neg = 0;
  }

  return 1;
}

int bn_usub_consttime(BIGNUM *r, const BIGNUM *a, const BIGNUM *b) {
  // |b| may have more words than |a| given non-minimal inputs, but all words
  // beyond |a->width| must then be zero.
  int b_width = b->width;
  if (b_width > a->width) {
    if (!bn_fits_in_words(b, a->width)) {
      OPENSSL_PUT_ERROR(BN, BN_R_ARG2_LT_ARG3);
      return 0;
    }
    b_width = a->width;
  }

  if (!bn_wexpand(r, a->width)) {
    return 0;
  }

  BN_ULONG borrow = bn_sub_words(r->d, a->d, b->d, b_width);
  for (int i = b_width; i < a->width; i++) {
    r->d[i] = CRYPTO_subc_w(a->d[i], 0, borrow, &borrow);
  }

  if (borrow) {
    OPENSSL_PUT_ERROR(BN, BN_R_ARG2_LT_ARG3);
    return 0;
  }

  r->width = a->width;
  r->neg = 0;
  return 1;
}

int BN_usub(BIGNUM *r, const BIGNUM *a, const BIGNUM *b) {
  if (!bn_usub_consttime(r, a, b)) {
    return 0;
  }
  bn_set_minimal_width(r);
  return 1;
}

int BN_sub_word(BIGNUM *a, BN_ULONG w) {
  int i;

  // degenerate case: w is zero
  if (!w) {
    return 1;
  }

  // degenerate case: a is zero
  if (BN_is_zero(a)) {
    i = BN_set_word(a, w);
    if (i != 0) {
      BN_set_negative(a, 1);
    }
    return i;
  }

  // handle 'a' when negative
  if (a->neg) {
    a->neg = 0;
    i = BN_add_word(a, w);
    a->neg = 1;
    return i;
  }

  if ((bn_minimal_width(a) == 1) && (a->d[0] < w)) {
    a->d[0] = w - a->d[0];
    a->neg = 1;
    return 1;
  }

  i = 0;
  for (;;) {
    if (a->d[i] >= w) {
      a->d[i] -= w;
      break;
    } else {
      a->d[i] -= w;
      i++;
      w = 1;
    }
  }

  if ((a->d[i] == 0) && (i == (a->width - 1))) {
    a->width--;
  }

  return 1;
}
