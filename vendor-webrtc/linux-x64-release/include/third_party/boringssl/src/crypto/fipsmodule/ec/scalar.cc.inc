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
#include <openssl/err.h>
#include <openssl/mem.h>

#include "../../internal.h"
#include "../bn/internal.h"
#include "internal.h"


int ec_bignum_to_scalar(const EC_GROUP *group, EC_SCALAR *out,
                        const BIGNUM *in) {
  // Scalars, which are often secret, must be reduced modulo the order. Those
  // that are not will be discarded, so leaking the result of the comparison is
  // safe.
  if (!bn_copy_words(out->words, group->order.N.width, in) ||
      !constant_time_declassify_int(bn_less_than_words(
          out->words, group->order.N.d, group->order.N.width))) {
    OPENSSL_PUT_ERROR(EC, EC_R_INVALID_SCALAR);
    return 0;
  }
  return 1;
}

int ec_scalar_equal_vartime(const EC_GROUP *group, const EC_SCALAR *a,
                            const EC_SCALAR *b) {
  return OPENSSL_memcmp(a->words, b->words,
                        group->order.N.width * sizeof(BN_ULONG)) == 0;
}

int ec_scalar_is_zero(const EC_GROUP *group, const EC_SCALAR *a) {
  BN_ULONG mask = 0;
  for (int i = 0; i < group->order.N.width; i++) {
    mask |= a->words[i];
  }
  return mask == 0;
}

int ec_random_scalar(const EC_GROUP *group, EC_SCALAR *out,
                     const uint8_t additional_data[32]) {
  return bn_rand_range_words(out->words, 0, group->order.N.d,
                             group->order.N.width, additional_data);
}

int ec_random_nonzero_scalar(const EC_GROUP *group, EC_SCALAR *out,
                             const uint8_t additional_data[32]) {
  return bn_rand_range_words(out->words, 1, group->order.N.d,
                             group->order.N.width, additional_data);
}

void ec_scalar_to_bytes(const EC_GROUP *group, uint8_t *out, size_t *out_len,
                        const EC_SCALAR *in) {
  size_t len = BN_num_bytes(&group->order.N);
  bn_words_to_big_endian(out, len, in->words, group->order.N.width);
  *out_len = len;
}

int ec_scalar_from_bytes(const EC_GROUP *group, EC_SCALAR *out,
                         const uint8_t *in, size_t len) {
  if (len != BN_num_bytes(&group->order.N)) {
    OPENSSL_PUT_ERROR(EC, EC_R_INVALID_SCALAR);
    return 0;
  }

  bn_big_endian_to_words(out->words, group->order.N.width, in, len);

  if (!bn_less_than_words(out->words, group->order.N.d, group->order.N.width)) {
    OPENSSL_PUT_ERROR(EC, EC_R_INVALID_SCALAR);
    return 0;
  }

  return 1;
}

void ec_scalar_reduce(const EC_GROUP *group, EC_SCALAR *out,
                      const BN_ULONG *words, size_t num) {
  // Convert "from" Montgomery form so the value is reduced modulo the order.
  bn_from_montgomery_small(out->words, group->order.N.width, words, num,
                           &group->order);
  // Convert "to" Montgomery form to remove the R^-1 factor added.
  ec_scalar_to_montgomery(group, out, out);
}

void ec_scalar_add(const EC_GROUP *group, EC_SCALAR *r, const EC_SCALAR *a,
                   const EC_SCALAR *b) {
  const BIGNUM *order = &group->order.N;
  BN_ULONG tmp[EC_MAX_WORDS];
  bn_mod_add_words(r->words, a->words, b->words, order->d, tmp, order->width);
  OPENSSL_cleanse(tmp, sizeof(tmp));
}

void ec_scalar_sub(const EC_GROUP *group, EC_SCALAR *r, const EC_SCALAR *a,
                   const EC_SCALAR *b) {
  const BIGNUM *order = &group->order.N;
  BN_ULONG tmp[EC_MAX_WORDS];
  bn_mod_sub_words(r->words, a->words, b->words, order->d, tmp, order->width);
  OPENSSL_cleanse(tmp, sizeof(tmp));
}

void ec_scalar_neg(const EC_GROUP *group, EC_SCALAR *r, const EC_SCALAR *a) {
  EC_SCALAR zero;
  OPENSSL_memset(&zero, 0, sizeof(EC_SCALAR));
  ec_scalar_sub(group, r, &zero, a);
}

void ec_scalar_select(const EC_GROUP *group, EC_SCALAR *out, BN_ULONG mask,
                      const EC_SCALAR *a, const EC_SCALAR *b) {
  const BIGNUM *order = &group->order.N;
  bn_select_words(out->words, mask, a->words, b->words, order->width);
}

void ec_scalar_to_montgomery(const EC_GROUP *group, EC_SCALAR *r,
                             const EC_SCALAR *a) {
  const BIGNUM *order = &group->order.N;
  bn_to_montgomery_small(r->words, a->words, order->width, &group->order);
}

void ec_scalar_from_montgomery(const EC_GROUP *group, EC_SCALAR *r,
                               const EC_SCALAR *a) {
  const BIGNUM *order = &group->order.N;
  bn_from_montgomery_small(r->words, order->width, a->words, order->width,
                           &group->order);
}

void ec_scalar_mul_montgomery(const EC_GROUP *group, EC_SCALAR *r,
                              const EC_SCALAR *a, const EC_SCALAR *b) {
  const BIGNUM *order = &group->order.N;
  bn_mod_mul_montgomery_small(r->words, a->words, b->words, order->width,
                              &group->order);
}

void ec_simple_scalar_inv0_montgomery(const EC_GROUP *group, EC_SCALAR *r,
                                      const EC_SCALAR *a) {
  const BIGNUM *order = &group->order.N;
  bn_mod_inverse0_prime_mont_small(r->words, a->words, order->width,
                                   &group->order);
}

int ec_simple_scalar_to_montgomery_inv_vartime(const EC_GROUP *group,
                                               EC_SCALAR *r,
                                               const EC_SCALAR *a) {
  if (ec_scalar_is_zero(group, a)) {
    return 0;
  }

  // This implementation (in fact) runs in constant time,
  // even though for this interface it is not mandatory.

  // r = a^-1 in the Montgomery domain. This is
  // |ec_scalar_to_montgomery| followed by |ec_scalar_inv0_montgomery|, but
  // |ec_scalar_inv0_montgomery| followed by |ec_scalar_from_montgomery| is
  // equivalent and slightly more efficient.
  ec_scalar_inv0_montgomery(group, r, a);
  ec_scalar_from_montgomery(group, r, r);
  return 1;
}

void ec_scalar_inv0_montgomery(const EC_GROUP *group, EC_SCALAR *r,
                               const EC_SCALAR *a) {
  group->meth->scalar_inv0_montgomery(group, r, a);
}

int ec_scalar_to_montgomery_inv_vartime(const EC_GROUP *group, EC_SCALAR *r,
                                        const EC_SCALAR *a) {
  return group->meth->scalar_to_montgomery_inv_vartime(group, r, a);
}
