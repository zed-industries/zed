// Copyright 2011-2016 The OpenSSL Project Authors. All Rights Reserved.
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

#include <openssl/bn.h>
#include <openssl/err.h>

#include "internal.h"


size_t ec_point_byte_len(const EC_GROUP *group, point_conversion_form_t form) {
  if (form != POINT_CONVERSION_COMPRESSED &&
      form != POINT_CONVERSION_UNCOMPRESSED) {
    OPENSSL_PUT_ERROR(EC, EC_R_INVALID_FORM);
    return 0;
  }

  const size_t field_len = BN_num_bytes(&group->field.N);
  size_t output_len = 1 /* type byte */ + field_len;
  if (form == POINT_CONVERSION_UNCOMPRESSED) {
    // Uncompressed points have a second coordinate.
    output_len += field_len;
  }
  return output_len;
}

size_t ec_point_to_bytes(const EC_GROUP *group, const EC_AFFINE *point,
                         point_conversion_form_t form, uint8_t *buf,
                         size_t max_out) {
  size_t output_len = ec_point_byte_len(group, form);
  if (max_out < output_len) {
    OPENSSL_PUT_ERROR(EC, EC_R_BUFFER_TOO_SMALL);
    return 0;
  }

  size_t field_len;
  ec_felem_to_bytes(group, buf + 1, &field_len, &point->X);
  assert(field_len == BN_num_bytes(&group->field.N));

  if (form == POINT_CONVERSION_UNCOMPRESSED) {
    ec_felem_to_bytes(group, buf + 1 + field_len, &field_len, &point->Y);
    assert(field_len == BN_num_bytes(&group->field.N));
    buf[0] = form;
  } else {
    uint8_t y_buf[EC_MAX_BYTES];
    ec_felem_to_bytes(group, y_buf, &field_len, &point->Y);
    buf[0] = form + (y_buf[field_len - 1] & 1);
  }

  return output_len;
}

int ec_point_from_uncompressed(const EC_GROUP *group, EC_AFFINE *out,
                               const uint8_t *in, size_t len) {
  const size_t field_len = BN_num_bytes(&group->field.N);
  if (len != 1 + 2 * field_len || in[0] != POINT_CONVERSION_UNCOMPRESSED) {
    OPENSSL_PUT_ERROR(EC, EC_R_INVALID_ENCODING);
    return 0;
  }

  EC_FELEM x, y;
  if (!ec_felem_from_bytes(group, &x, in + 1, field_len) ||
      !ec_felem_from_bytes(group, &y, in + 1 + field_len, field_len) ||
      !ec_point_set_affine_coordinates(group, out, &x, &y)) {
    return 0;
  }

  return 1;
}

static int ec_GFp_simple_oct2point(const EC_GROUP *group, EC_POINT *point,
                                   const uint8_t *buf, size_t len,
                                   BN_CTX *ctx) {
  if (len == 0) {
    OPENSSL_PUT_ERROR(EC, EC_R_BUFFER_TOO_SMALL);
    return 0;
  }

  uint8_t form = buf[0];
  if (form == static_cast<uint8_t>(POINT_CONVERSION_UNCOMPRESSED)) {
    EC_AFFINE affine;
    if (!ec_point_from_uncompressed(group, &affine, buf, len)) {
      // In the event of an error, defend against the caller not checking the
      // return value by setting a known safe value.
      ec_set_to_safe_point(group, &point->raw);
      return 0;
    }
    ec_affine_to_jacobian(group, &point->raw, &affine);
    return 1;
  }

  const int y_bit = form & 1;
  const size_t field_len = BN_num_bytes(&group->field.N);
  form = form & ~1u;
  if (form != static_cast<uint8_t>(POINT_CONVERSION_COMPRESSED) ||
      len != 1 /* type byte */ + field_len) {
    OPENSSL_PUT_ERROR(EC, EC_R_INVALID_ENCODING);
    return 0;
  }

  // TODO(davidben): Integrate compressed coordinates with the lower-level EC
  // abstractions. This requires a way to compute square roots, which is tricky
  // for primes which are not 3 (mod 4), namely P-224 and custom curves. P-224's
  // prime is particularly inconvenient for compressed coordinates. See
  // https://cr.yp.to/papers/sqroot.pdf
  bssl::UniquePtr<BN_CTX> new_ctx;
  if (ctx == nullptr) {
    new_ctx.reset(BN_CTX_new());
    if (new_ctx == nullptr) {
      return 0;
    }
    ctx = new_ctx.get();
  }

  bssl::BN_CTXScope scope(ctx);
  BIGNUM *x = BN_CTX_get(ctx);
  if (x == nullptr || !BN_bin2bn(buf + 1, field_len, x)) {
    return 0;
  }
  if (BN_ucmp(x, &group->field.N) >= 0) {
    OPENSSL_PUT_ERROR(EC, EC_R_INVALID_ENCODING);
    return 0;
  }

  if (!EC_POINT_set_compressed_coordinates_GFp(group, point, x, y_bit, ctx)) {
    return 0;
  }

  return 1;
}

int EC_POINT_oct2point(const EC_GROUP *group, EC_POINT *point,
                       const uint8_t *buf, size_t len, BN_CTX *ctx) {
  if (EC_GROUP_cmp(group, point->group, NULL) != 0) {
    OPENSSL_PUT_ERROR(EC, EC_R_INCOMPATIBLE_OBJECTS);
    return 0;
  }
  return ec_GFp_simple_oct2point(group, point, buf, len, ctx);
}

size_t EC_POINT_point2oct(const EC_GROUP *group, const EC_POINT *point,
                          point_conversion_form_t form, uint8_t *buf,
                          size_t max_out, BN_CTX *ctx) {
  if (EC_GROUP_cmp(group, point->group, NULL) != 0) {
    OPENSSL_PUT_ERROR(EC, EC_R_INCOMPATIBLE_OBJECTS);
    return 0;
  }
  if (buf == NULL) {
    // When |buf| is NULL, just return the number of bytes that would be
    // written, without doing an expensive Jacobian-to-affine conversion.
    if (ec_GFp_simple_is_at_infinity(group, &point->raw)) {
      OPENSSL_PUT_ERROR(EC, EC_R_POINT_AT_INFINITY);
      return 0;
    }
    return ec_point_byte_len(group, form);
  }
  EC_AFFINE affine;
  if (!ec_jacobian_to_affine(group, &affine, &point->raw)) {
    return 0;
  }
  return ec_point_to_bytes(group, &affine, form, buf, max_out);
}

size_t EC_POINT_point2buf(const EC_GROUP *group, const EC_POINT *point,
                          point_conversion_form_t form, uint8_t **out_buf,
                          BN_CTX *ctx) {
  *out_buf = NULL;
  size_t len = EC_POINT_point2oct(group, point, form, NULL, 0, ctx);
  if (len == 0) {
    return 0;
  }
  uint8_t *buf = reinterpret_cast<uint8_t *>(OPENSSL_malloc(len));
  if (buf == NULL) {
    return 0;
  }
  len = EC_POINT_point2oct(group, point, form, buf, len, ctx);
  if (len == 0) {
    OPENSSL_free(buf);
    return 0;
  }
  *out_buf = buf;
  return len;
}

int EC_POINT_set_compressed_coordinates_GFp(const EC_GROUP *group,
                                            EC_POINT *point, const BIGNUM *x,
                                            int y_bit, BN_CTX *ctx) {
  if (EC_GROUP_cmp(group, point->group, NULL) != 0) {
    OPENSSL_PUT_ERROR(EC, EC_R_INCOMPATIBLE_OBJECTS);
    return 0;
  }

  const BIGNUM *field = &group->field.N;
  if (BN_is_negative(x) || BN_cmp(x, field) >= 0) {
    OPENSSL_PUT_ERROR(EC, EC_R_INVALID_COMPRESSED_POINT);
    return 0;
  }

  ERR_clear_error();

  bssl::UniquePtr<BN_CTX> new_ctx;
  if (ctx == nullptr) {
    new_ctx.reset(BN_CTX_new());
    if (new_ctx == nullptr) {
      return 0;
    }
    ctx = new_ctx.get();
  }

  y_bit = (y_bit != 0);

  bssl::BN_CTXScope scope(ctx);
  BIGNUM *tmp1 = BN_CTX_get(ctx);
  BIGNUM *tmp2 = BN_CTX_get(ctx);
  BIGNUM *a = BN_CTX_get(ctx);
  BIGNUM *b = BN_CTX_get(ctx);
  BIGNUM *y = BN_CTX_get(ctx);
  if (y == NULL || !EC_GROUP_get_curve_GFp(group, NULL, a, b, ctx)) {
    return 0;
  }

  // Recover y.  We have a Weierstrass equation
  //     y^2 = x^3 + a*x + b,
  // so  y  is one of the square roots of  x^3 + a*x + b.

  // tmp1 := x^3
  if (!BN_mod_sqr(tmp2, x, field, ctx) ||
      !BN_mod_mul(tmp1, tmp2, x, field, ctx)) {
    return 0;
  }

  // tmp1 := tmp1 + a*x
  if (group->a_is_minus3) {
    if (!bn_mod_lshift1_consttime(tmp2, x, field, ctx) ||
        !bn_mod_add_consttime(tmp2, tmp2, x, field, ctx) ||
        !bn_mod_sub_consttime(tmp1, tmp1, tmp2, field, ctx)) {
      return 0;
    }
  } else {
    if (!BN_mod_mul(tmp2, a, x, field, ctx) ||
        !bn_mod_add_consttime(tmp1, tmp1, tmp2, field, ctx)) {
      return 0;
    }
  }

  // tmp1 := tmp1 + b
  if (!bn_mod_add_consttime(tmp1, tmp1, b, field, ctx)) {
    return 0;
  }

  if (!BN_mod_sqrt(y, tmp1, field, ctx)) {
    uint32_t err = ERR_peek_last_error();
    if (ERR_GET_LIB(err) == ERR_LIB_BN &&
        ERR_GET_REASON(err) == BN_R_NOT_A_SQUARE) {
      ERR_clear_error();
      OPENSSL_PUT_ERROR(EC, EC_R_INVALID_COMPRESSED_POINT);
    } else {
      OPENSSL_PUT_ERROR(EC, ERR_R_BN_LIB);
    }
    return 0;
  }

  if (y_bit != BN_is_odd(y)) {
    if (BN_is_zero(y)) {
      OPENSSL_PUT_ERROR(EC, EC_R_INVALID_COMPRESSION_BIT);
      return 0;
    }
    if (!BN_usub(y, field, y)) {
      return 0;
    }
  }
  if (y_bit != BN_is_odd(y)) {
    OPENSSL_PUT_ERROR(EC, ERR_R_INTERNAL_ERROR);
    return 0;
  }

  if (!EC_POINT_set_affine_coordinates_GFp(group, point, x, y, ctx)) {
    return 0;
  }

  return 1;
}
