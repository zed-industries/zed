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

#include <openssl/ec.h>

#include <assert.h>

#include "internal.h"
#include "../bn/internal.h"
#include "../../internal.h"


void ec_GFp_mont_mul(const EC_GROUP *group, EC_JACOBIAN *r,
                     const EC_JACOBIAN *p, const EC_SCALAR *scalar) {
  // This is a generic implementation for uncommon curves that not do not
  // warrant a tuned one. It uses unsigned digits so that the doubling case in
  // |ec_GFp_mont_add| is always unreachable, erring on safety and simplicity.

  // Compute a table of the first 32 multiples of |p| (including infinity).
  EC_JACOBIAN precomp[32];
  ec_GFp_simple_point_set_to_infinity(group, &precomp[0]);
  ec_GFp_simple_point_copy(&precomp[1], p);
  for (size_t j = 2; j < OPENSSL_ARRAY_SIZE(precomp); j++) {
    if (j & 1) {
      ec_GFp_mont_add(group, &precomp[j], &precomp[1], &precomp[j - 1]);
    } else {
      ec_GFp_mont_dbl(group, &precomp[j], &precomp[j / 2]);
    }
  }

  // Divide bits in |scalar| into windows.
  unsigned bits =  EC_GROUP_order_bits(group);
  int r_is_at_infinity = 1;
  for (unsigned i = bits - 1; i < bits; i--) {
    if (!r_is_at_infinity) {
      ec_GFp_mont_dbl(group, r, r);
    }
    if (i % 5 == 0) {
      // Compute the next window value.
      const size_t width = group->order.N.width;
      uint8_t window = bn_is_bit_set_words(scalar->words, width, i + 4) << 4;
      window |= bn_is_bit_set_words(scalar->words, width, i + 3) << 3;
      window |= bn_is_bit_set_words(scalar->words, width, i + 2) << 2;
      window |= bn_is_bit_set_words(scalar->words, width, i + 1) << 1;
      window |= bn_is_bit_set_words(scalar->words, width, i);

      // Select the entry in constant-time.
      EC_JACOBIAN tmp;
      OPENSSL_memset(&tmp, 0, sizeof(EC_JACOBIAN));
      for (size_t j = 0; j < OPENSSL_ARRAY_SIZE(precomp); j++) {
        BN_ULONG mask = constant_time_eq_w(j, window);
        ec_point_select(group, &tmp, mask, &precomp[j], &tmp);
      }

      if (r_is_at_infinity) {
        ec_GFp_simple_point_copy(r, &tmp);
        r_is_at_infinity = 0;
      } else {
        ec_GFp_mont_add(group, r, r, &tmp);
      }
    }
  }
  if (r_is_at_infinity) {
    ec_GFp_simple_point_set_to_infinity(group, r);
  }
}

void ec_GFp_mont_mul_base(const EC_GROUP *group, EC_JACOBIAN *r,
                          const EC_SCALAR *scalar) {
  ec_GFp_mont_mul(group, r, &group->generator.raw, scalar);
}

static void ec_GFp_mont_batch_precomp(const EC_GROUP *group, EC_JACOBIAN *out,
                                      size_t num, const EC_JACOBIAN *p) {
  assert(num > 1);
  ec_GFp_simple_point_set_to_infinity(group, &out[0]);
  ec_GFp_simple_point_copy(&out[1], p);
  for (size_t j = 2; j < num; j++) {
    if (j & 1) {
      ec_GFp_mont_add(group, &out[j], &out[1], &out[j - 1]);
    } else {
      ec_GFp_mont_dbl(group, &out[j], &out[j / 2]);
    }
  }
}

static void ec_GFp_mont_batch_get_window(const EC_GROUP *group,
                                         EC_JACOBIAN *out,
                                         const EC_JACOBIAN precomp[17],
                                         const EC_SCALAR *scalar, unsigned i) {
  const size_t width = group->order.N.width;
  uint8_t window = bn_is_bit_set_words(scalar->words, width, i + 4) << 5;
  window |= bn_is_bit_set_words(scalar->words, width, i + 3) << 4;
  window |= bn_is_bit_set_words(scalar->words, width, i + 2) << 3;
  window |= bn_is_bit_set_words(scalar->words, width, i + 1) << 2;
  window |= bn_is_bit_set_words(scalar->words, width, i) << 1;
  if (i > 0) {
    window |= bn_is_bit_set_words(scalar->words, width, i - 1);
  }
  crypto_word_t sign, digit;
  ec_GFp_nistp_recode_scalar_bits(&sign, &digit, window);

  // Select the entry in constant-time.
  OPENSSL_memset(out, 0, sizeof(EC_JACOBIAN));
  for (size_t j = 0; j < 17; j++) {
    BN_ULONG mask = constant_time_eq_w(j, digit);
    ec_point_select(group, out, mask, &precomp[j], out);
  }

  // Negate if necessary.
  EC_FELEM neg_Y;
  ec_felem_neg(group, &neg_Y, &out->Y);
  crypto_word_t sign_mask = sign;
  sign_mask = 0u - sign_mask;
  ec_felem_select(group, &out->Y, sign_mask, &neg_Y, &out->Y);
}

void ec_GFp_mont_mul_batch(const EC_GROUP *group, EC_JACOBIAN *r,
                           const EC_JACOBIAN *p0, const EC_SCALAR *scalar0,
                           const EC_JACOBIAN *p1, const EC_SCALAR *scalar1,
                           const EC_JACOBIAN *p2, const EC_SCALAR *scalar2) {
  EC_JACOBIAN precomp[3][17];
  ec_GFp_mont_batch_precomp(group, precomp[0], 17, p0);
  ec_GFp_mont_batch_precomp(group, precomp[1], 17, p1);
  if (p2 != NULL) {
    ec_GFp_mont_batch_precomp(group, precomp[2], 17, p2);
  }

  // Divide bits in |scalar| into windows.
  unsigned bits = EC_GROUP_order_bits(group);
  int r_is_at_infinity = 1;
  for (unsigned i = bits; i <= bits; i--) {
    if (!r_is_at_infinity) {
      ec_GFp_mont_dbl(group, r, r);
    }
    if (i % 5 == 0) {
      EC_JACOBIAN tmp;
      ec_GFp_mont_batch_get_window(group, &tmp, precomp[0], scalar0, i);
      if (r_is_at_infinity) {
        ec_GFp_simple_point_copy(r, &tmp);
        r_is_at_infinity = 0;
      } else {
        ec_GFp_mont_add(group, r, r, &tmp);
      }

      ec_GFp_mont_batch_get_window(group, &tmp, precomp[1], scalar1, i);
      ec_GFp_mont_add(group, r, r, &tmp);

      if (p2 != NULL) {
        ec_GFp_mont_batch_get_window(group, &tmp, precomp[2], scalar2, i);
        ec_GFp_mont_add(group, r, r, &tmp);
      }
    }
  }
  if (r_is_at_infinity) {
    ec_GFp_simple_point_set_to_infinity(group, r);
  }
}

static unsigned ec_GFp_mont_comb_stride(const EC_GROUP *group) {
  return (EC_GROUP_get_degree(group) + EC_MONT_PRECOMP_COMB_SIZE - 1) /
         EC_MONT_PRECOMP_COMB_SIZE;
}

int ec_GFp_mont_init_precomp(const EC_GROUP *group, EC_PRECOMP *out,
                             const EC_JACOBIAN *p) {
  // comb[i - 1] stores the ith element of the comb. That is, if i is
  // b4 * 2^4 + b3 * 2^3 + ... + b0 * 2^0, it stores k * |p|, where k is
  // b4 * 2^(4*stride) + b3 * 2^(3*stride) + ... + b0 * 2^(0*stride). stride
  // here is |ec_GFp_mont_comb_stride|. We store at index i - 1 because the 0th
  // comb entry is always infinity.
  EC_JACOBIAN comb[(1 << EC_MONT_PRECOMP_COMB_SIZE) - 1];
  unsigned stride = ec_GFp_mont_comb_stride(group);

  // We compute the comb sequentially by the highest set bit. Initially, all
  // entries up to 2^0 are filled.
  comb[(1 << 0) - 1] = *p;
  for (unsigned i = 1; i < EC_MONT_PRECOMP_COMB_SIZE; i++) {
    // Compute entry 2^i by doubling the entry for 2^(i-1) |stride| times.
    unsigned bit = 1 << i;
    ec_GFp_mont_dbl(group, &comb[bit - 1], &comb[bit / 2 - 1]);
    for (unsigned j = 1; j < stride; j++) {
      ec_GFp_mont_dbl(group, &comb[bit - 1], &comb[bit - 1]);
    }
    // Compute entries from 2^i + 1 to 2^i + (2^i - 1) by adding entry 2^i to
    // a previous entry.
    for (unsigned j = 1; j < bit; j++) {
      ec_GFp_mont_add(group, &comb[bit + j - 1], &comb[bit - 1], &comb[j - 1]);
    }
  }

  // Store the comb in affine coordinates to shrink the table. (This reduces
  // cache pressure and makes the constant-time selects faster.)
  static_assert(OPENSSL_ARRAY_SIZE(comb) == OPENSSL_ARRAY_SIZE(out->comb),
                "comb sizes did not match");
  return ec_jacobian_to_affine_batch(group, out->comb, comb,
                                     OPENSSL_ARRAY_SIZE(comb));
}

static void ec_GFp_mont_get_comb_window(const EC_GROUP *group,
                                        EC_JACOBIAN *out,
                                        const EC_PRECOMP *precomp,
                                        const EC_SCALAR *scalar, unsigned i) {
  const size_t width = group->order.N.width;
  unsigned stride = ec_GFp_mont_comb_stride(group);
  // Select the bits corresponding to the comb shifted up by |i|.
  unsigned window = 0;
  for (unsigned j = 0; j < EC_MONT_PRECOMP_COMB_SIZE; j++) {
    window |= bn_is_bit_set_words(scalar->words, width, j * stride + i)
              << j;
  }

  // Select precomp->comb[window - 1]. If |window| is zero, |match| will always
  // be zero, which will leave |out| at infinity.
  OPENSSL_memset(out, 0, sizeof(EC_JACOBIAN));
  for (unsigned j = 0; j < OPENSSL_ARRAY_SIZE(precomp->comb); j++) {
    BN_ULONG match = constant_time_eq_w(window, j + 1);
    ec_felem_select(group, &out->X, match, &precomp->comb[j].X, &out->X);
    ec_felem_select(group, &out->Y, match, &precomp->comb[j].Y, &out->Y);
  }
  BN_ULONG is_infinity = constant_time_is_zero_w(window);
  ec_felem_select(group, &out->Z, is_infinity, &out->Z, ec_felem_one(group));
}

void ec_GFp_mont_mul_precomp(const EC_GROUP *group, EC_JACOBIAN *r,
                             const EC_PRECOMP *p0, const EC_SCALAR *scalar0,
                             const EC_PRECOMP *p1, const EC_SCALAR *scalar1,
                             const EC_PRECOMP *p2, const EC_SCALAR *scalar2) {
  unsigned stride = ec_GFp_mont_comb_stride(group);
  int r_is_at_infinity = 1;
  for (unsigned i = stride - 1; i < stride; i--) {
    if (!r_is_at_infinity) {
      ec_GFp_mont_dbl(group, r, r);
    }

    EC_JACOBIAN tmp;
    ec_GFp_mont_get_comb_window(group, &tmp, p0, scalar0, i);
    if (r_is_at_infinity) {
      ec_GFp_simple_point_copy(r, &tmp);
      r_is_at_infinity = 0;
    } else {
      ec_GFp_mont_add(group, r, r, &tmp);
    }

    if (p1 != NULL) {
      ec_GFp_mont_get_comb_window(group, &tmp, p1, scalar1, i);
      ec_GFp_mont_add(group, r, r, &tmp);
    }

    if (p2 != NULL) {
      ec_GFp_mont_get_comb_window(group, &tmp, p2, scalar2, i);
      ec_GFp_mont_add(group, r, r, &tmp);
    }
  }
  if (r_is_at_infinity) {
    ec_GFp_simple_point_set_to_infinity(group, r);
  }
}
