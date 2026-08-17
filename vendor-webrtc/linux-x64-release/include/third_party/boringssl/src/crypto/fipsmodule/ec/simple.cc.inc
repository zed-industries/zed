// Copyright 2001-2016 The OpenSSL Project Authors. All Rights Reserved.
// Copyright (c) 2002, Oracle and/or its affiliates. All rights reserved.
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

#include <string.h>

#include <openssl/bn.h>
#include <openssl/err.h>
#include <openssl/mem.h>

#include "internal.h"
#include "../../internal.h"


// Most method functions in this file are designed to work with non-trivial
// representations of field elements if necessary (see ecp_mont.c): while
// standard modular addition and subtraction are used, the field_mul and
// field_sqr methods will be used for multiplication, and field_encode and
// field_decode (if defined) will be used for converting between
// representations.
//
// Functions here specifically assume that if a non-trivial representation is
// used, it is a Montgomery representation (i.e. 'encoding' means multiplying
// by some factor R).

int ec_GFp_simple_group_set_curve(EC_GROUP *group, const BIGNUM *p,
                                  const BIGNUM *a, const BIGNUM *b,
                                  BN_CTX *ctx) {
  // p must be a prime > 3
  if (BN_num_bits(p) <= 2 || !BN_is_odd(p)) {
    OPENSSL_PUT_ERROR(EC, EC_R_INVALID_FIELD);
    return 0;
  }

  bssl::BN_CTXScope scope(ctx);
  BIGNUM *tmp = BN_CTX_get(ctx);
  if (tmp == nullptr) {
    return 0;
  }

  if (!BN_MONT_CTX_set(&group->field, p, ctx) ||
      !ec_bignum_to_felem(group, &group->a, a) ||
      !ec_bignum_to_felem(group, &group->b, b) ||
      // Reuse Z from the generator to cache the value one.
      !ec_bignum_to_felem(group, &group->generator.raw.Z, BN_value_one())) {
    return 0;
  }

  // group->a_is_minus3
  if (!BN_copy(tmp, a) ||
      !BN_add_word(tmp, 3)) {
    return 0;
  }
  group->a_is_minus3 = (0 == BN_cmp(tmp, &group->field.N));

  return 1;
}

int ec_GFp_simple_group_get_curve(const EC_GROUP *group, BIGNUM *p, BIGNUM *a,
                                  BIGNUM *b) {
  if ((p != NULL && !BN_copy(p, &group->field.N)) ||
      (a != NULL && !ec_felem_to_bignum(group, a, &group->a)) ||
      (b != NULL && !ec_felem_to_bignum(group, b, &group->b))) {
    return 0;
  }
  return 1;
}

void ec_GFp_simple_point_init(EC_JACOBIAN *point) {
  OPENSSL_memset(&point->X, 0, sizeof(EC_FELEM));
  OPENSSL_memset(&point->Y, 0, sizeof(EC_FELEM));
  OPENSSL_memset(&point->Z, 0, sizeof(EC_FELEM));
}

void ec_GFp_simple_point_copy(EC_JACOBIAN *dest, const EC_JACOBIAN *src) {
  OPENSSL_memcpy(&dest->X, &src->X, sizeof(EC_FELEM));
  OPENSSL_memcpy(&dest->Y, &src->Y, sizeof(EC_FELEM));
  OPENSSL_memcpy(&dest->Z, &src->Z, sizeof(EC_FELEM));
}

void ec_GFp_simple_point_set_to_infinity(const EC_GROUP *group,
                                         EC_JACOBIAN *point) {
  // Although it is strictly only necessary to zero Z, we zero the entire point
  // in case |point| was stack-allocated and yet to be initialized.
  ec_GFp_simple_point_init(point);
}

void ec_GFp_simple_invert(const EC_GROUP *group, EC_JACOBIAN *point) {
  ec_felem_neg(group, &point->Y, &point->Y);
}

int ec_GFp_simple_is_at_infinity(const EC_GROUP *group,
                                 const EC_JACOBIAN *point) {
  return ec_felem_non_zero_mask(group, &point->Z) == 0;
}

int ec_GFp_simple_is_on_curve(const EC_GROUP *group,
                              const EC_JACOBIAN *point) {
  // We have a curve defined by a Weierstrass equation
  //      y^2 = x^3 + a*x + b.
  // The point to consider is given in Jacobian projective coordinates
  // where  (X, Y, Z)  represents  (x, y) = (X/Z^2, Y/Z^3).
  // Substituting this and multiplying by  Z^6  transforms the above equation
  // into
  //      Y^2 = X^3 + a*X*Z^4 + b*Z^6.
  // To test this, we add up the right-hand side in 'rh'.
  //
  // This function may be used when double-checking the secret result of a point
  // multiplication, so we proceed in constant-time.

  void (*const felem_mul)(const EC_GROUP *, EC_FELEM *r, const EC_FELEM *a,
                          const EC_FELEM *b) = group->meth->felem_mul;
  void (*const felem_sqr)(const EC_GROUP *, EC_FELEM *r, const EC_FELEM *a) =
      group->meth->felem_sqr;

  // rh := X^2
  EC_FELEM rh;
  felem_sqr(group, &rh, &point->X);

  EC_FELEM tmp, Z4, Z6;
  felem_sqr(group, &tmp, &point->Z);
  felem_sqr(group, &Z4, &tmp);
  felem_mul(group, &Z6, &Z4, &tmp);

  // rh := rh + a*Z^4
  if (group->a_is_minus3) {
    ec_felem_add(group, &tmp, &Z4, &Z4);
    ec_felem_add(group, &tmp, &tmp, &Z4);
    ec_felem_sub(group, &rh, &rh, &tmp);
  } else {
    felem_mul(group, &tmp, &Z4, &group->a);
    ec_felem_add(group, &rh, &rh, &tmp);
  }

  // rh := (rh + a*Z^4)*X
  felem_mul(group, &rh, &rh, &point->X);

  // rh := rh + b*Z^6
  felem_mul(group, &tmp, &group->b, &Z6);
  ec_felem_add(group, &rh, &rh, &tmp);

  // 'lh' := Y^2
  felem_sqr(group, &tmp, &point->Y);

  ec_felem_sub(group, &tmp, &tmp, &rh);
  BN_ULONG not_equal = ec_felem_non_zero_mask(group, &tmp);

  // If Z = 0, the point is infinity, which is always on the curve.
  BN_ULONG not_infinity = ec_felem_non_zero_mask(group, &point->Z);

  return 1 & ~(not_infinity & not_equal);
}

int ec_GFp_simple_points_equal(const EC_GROUP *group, const EC_JACOBIAN *a,
                               const EC_JACOBIAN *b) {
  // This function is implemented in constant-time for two reasons. First,
  // although EC points are usually public, their Jacobian Z coordinates may be
  // secret, or at least are not obviously public. Second, more complex
  // protocols will sometimes manipulate secret points.
  //
  // This does mean that we pay a 6M+2S Jacobian comparison when comparing two
  // publicly affine points costs no field operations at all. If needed, we can
  // restore this optimization by keeping better track of affine vs. Jacobian
  // forms. See https://crbug.com/boringssl/326.

  // If neither |a| or |b| is infinity, we have to decide whether
  //     (X_a/Z_a^2, Y_a/Z_a^3) = (X_b/Z_b^2, Y_b/Z_b^3),
  // or equivalently, whether
  //     (X_a*Z_b^2, Y_a*Z_b^3) = (X_b*Z_a^2, Y_b*Z_a^3).

  void (*const felem_mul)(const EC_GROUP *, EC_FELEM *r, const EC_FELEM *a,
                          const EC_FELEM *b) = group->meth->felem_mul;
  void (*const felem_sqr)(const EC_GROUP *, EC_FELEM *r, const EC_FELEM *a) =
      group->meth->felem_sqr;

  EC_FELEM tmp1, tmp2, Za23, Zb23;
  felem_sqr(group, &Zb23, &b->Z);         // Zb23 = Z_b^2
  felem_mul(group, &tmp1, &a->X, &Zb23);  // tmp1 = X_a * Z_b^2
  felem_sqr(group, &Za23, &a->Z);         // Za23 = Z_a^2
  felem_mul(group, &tmp2, &b->X, &Za23);  // tmp2 = X_b * Z_a^2
  ec_felem_sub(group, &tmp1, &tmp1, &tmp2);
  const BN_ULONG x_not_equal = ec_felem_non_zero_mask(group, &tmp1);

  felem_mul(group, &Zb23, &Zb23, &b->Z);  // Zb23 = Z_b^3
  felem_mul(group, &tmp1, &a->Y, &Zb23);  // tmp1 = Y_a * Z_b^3
  felem_mul(group, &Za23, &Za23, &a->Z);  // Za23 = Z_a^3
  felem_mul(group, &tmp2, &b->Y, &Za23);  // tmp2 = Y_b * Z_a^3
  ec_felem_sub(group, &tmp1, &tmp1, &tmp2);
  const BN_ULONG y_not_equal = ec_felem_non_zero_mask(group, &tmp1);
  const BN_ULONG x_and_y_equal = ~(x_not_equal | y_not_equal);

  const BN_ULONG a_not_infinity = ec_felem_non_zero_mask(group, &a->Z);
  const BN_ULONG b_not_infinity = ec_felem_non_zero_mask(group, &b->Z);
  const BN_ULONG a_and_b_infinity = ~(a_not_infinity | b_not_infinity);

  const BN_ULONG equal =
      a_and_b_infinity | (a_not_infinity & b_not_infinity & x_and_y_equal);
  return equal & 1;
}

int ec_affine_jacobian_equal(const EC_GROUP *group, const EC_AFFINE *a,
                             const EC_JACOBIAN *b) {
  // If |b| is not infinity, we have to decide whether
  //     (X_a, Y_a) = (X_b/Z_b^2, Y_b/Z_b^3),
  // or equivalently, whether
  //     (X_a*Z_b^2, Y_a*Z_b^3) = (X_b, Y_b).

  void (*const felem_mul)(const EC_GROUP *, EC_FELEM *r, const EC_FELEM *a,
                          const EC_FELEM *b) = group->meth->felem_mul;
  void (*const felem_sqr)(const EC_GROUP *, EC_FELEM *r, const EC_FELEM *a) =
      group->meth->felem_sqr;

  EC_FELEM tmp, Zb2;
  felem_sqr(group, &Zb2, &b->Z);        // Zb2 = Z_b^2
  felem_mul(group, &tmp, &a->X, &Zb2);  // tmp = X_a * Z_b^2
  ec_felem_sub(group, &tmp, &tmp, &b->X);
  const BN_ULONG x_not_equal = ec_felem_non_zero_mask(group, &tmp);

  felem_mul(group, &tmp, &a->Y, &Zb2);  // tmp = Y_a * Z_b^2
  felem_mul(group, &tmp, &tmp, &b->Z);  // tmp = Y_a * Z_b^3
  ec_felem_sub(group, &tmp, &tmp, &b->Y);
  const BN_ULONG y_not_equal = ec_felem_non_zero_mask(group, &tmp);
  const BN_ULONG x_and_y_equal = ~(x_not_equal | y_not_equal);

  const BN_ULONG b_not_infinity = ec_felem_non_zero_mask(group, &b->Z);

  const BN_ULONG equal = b_not_infinity & x_and_y_equal;
  return equal & 1;
}

int ec_GFp_simple_cmp_x_coordinate(const EC_GROUP *group, const EC_JACOBIAN *p,
                                   const EC_SCALAR *r) {
  if (ec_GFp_simple_is_at_infinity(group, p)) {
    // |ec_get_x_coordinate_as_scalar| will check this internally, but this way
    // we do not push to the error queue.
    return 0;
  }

  EC_SCALAR x;
  return ec_get_x_coordinate_as_scalar(group, &x, p) &&
         ec_scalar_equal_vartime(group, &x, r);
}

void ec_GFp_simple_felem_to_bytes(const EC_GROUP *group, uint8_t *out,
                                  size_t *out_len, const EC_FELEM *in) {
  size_t len = BN_num_bytes(&group->field.N);
  bn_words_to_big_endian(out, len, in->words, group->field.N.width);
  *out_len = len;
}

int ec_GFp_simple_felem_from_bytes(const EC_GROUP *group, EC_FELEM *out,
                                   const uint8_t *in, size_t len) {
  if (len != BN_num_bytes(&group->field.N)) {
    OPENSSL_PUT_ERROR(EC, EC_R_DECODE_ERROR);
    return 0;
  }

  bn_big_endian_to_words(out->words, group->field.N.width, in, len);

  if (!bn_less_than_words(out->words, group->field.N.d, group->field.N.width)) {
    OPENSSL_PUT_ERROR(EC, EC_R_DECODE_ERROR);
    return 0;
  }

  return 1;
}
