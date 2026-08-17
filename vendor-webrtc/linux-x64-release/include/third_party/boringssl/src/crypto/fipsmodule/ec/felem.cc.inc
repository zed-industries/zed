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

#include <assert.h>

#include "internal.h"
#include "../bn/internal.h"
#include "../../internal.h"


const EC_FELEM *ec_felem_one(const EC_GROUP *group) {
  // We reuse generator.Z as a cache for 1 in the field.
  return &group->generator.raw.Z;
}

int ec_bignum_to_felem(const EC_GROUP *group, EC_FELEM *out, const BIGNUM *in) {
  uint8_t bytes[EC_MAX_BYTES];
  size_t len = BN_num_bytes(&group->field.N);
  assert(sizeof(bytes) >= len);
  if (BN_is_negative(in) || BN_cmp(in, &group->field.N) >= 0 ||
      !BN_bn2bin_padded(bytes, len, in)) {
    OPENSSL_PUT_ERROR(EC, EC_R_COORDINATES_OUT_OF_RANGE);
    return 0;
  }

  return ec_felem_from_bytes(group, out, bytes, len);
}

int ec_felem_to_bignum(const EC_GROUP *group, BIGNUM *out, const EC_FELEM *in) {
  uint8_t bytes[EC_MAX_BYTES];
  size_t len;
  ec_felem_to_bytes(group, bytes, &len, in);
  return BN_bin2bn(bytes, len, out) != NULL;
}

void ec_felem_to_bytes(const EC_GROUP *group, uint8_t *out, size_t *out_len,
                       const EC_FELEM *in) {
  group->meth->felem_to_bytes(group, out, out_len, in);
}

int ec_felem_from_bytes(const EC_GROUP *group, EC_FELEM *out, const uint8_t *in,
                        size_t len) {
  return group->meth->felem_from_bytes(group, out, in, len);
}

void ec_felem_neg(const EC_GROUP *group, EC_FELEM *out, const EC_FELEM *a) {
  // -a is zero if a is zero and p-a otherwise.
  BN_ULONG mask = ec_felem_non_zero_mask(group, a);
  BN_ULONG borrow = bn_sub_words(out->words, group->field.N.d, a->words,
                                 group->field.N.width);
  assert(borrow == 0);
  (void)borrow;
  for (int i = 0; i < group->field.N.width; i++) {
    out->words[i] &= mask;
  }
}

void ec_felem_add(const EC_GROUP *group, EC_FELEM *out, const EC_FELEM *a,
                  const EC_FELEM *b) {
  EC_FELEM tmp;
  bn_mod_add_words(out->words, a->words, b->words, group->field.N.d, tmp.words,
                   group->field.N.width);
}

void ec_felem_sub(const EC_GROUP *group, EC_FELEM *out, const EC_FELEM *a,
                  const EC_FELEM *b) {
  EC_FELEM tmp;
  bn_mod_sub_words(out->words, a->words, b->words, group->field.N.d, tmp.words,
                   group->field.N.width);
}

BN_ULONG ec_felem_non_zero_mask(const EC_GROUP *group, const EC_FELEM *a) {
  BN_ULONG mask = 0;
  for (int i = 0; i < group->field.N.width; i++) {
    mask |= a->words[i];
  }
  return ~constant_time_is_zero_w(mask);
}

void ec_felem_select(const EC_GROUP *group, EC_FELEM *out, BN_ULONG mask,
                     const EC_FELEM *a, const EC_FELEM *b) {
  bn_select_words(out->words, mask, a->words, b->words, group->field.N.width);
}

int ec_felem_equal(const EC_GROUP *group, const EC_FELEM *a,
                   const EC_FELEM *b) {
  return CRYPTO_memcmp(a->words, b->words,
                       group->field.N.width * sizeof(BN_ULONG)) == 0;
}
