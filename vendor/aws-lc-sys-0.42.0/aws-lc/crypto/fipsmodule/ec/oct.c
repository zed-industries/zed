// Originally written by Bodo Moeller for the OpenSSL project.
// Copyright (c) 1998-2005 The OpenSSL Project.  All rights reserved.
// Copyright 2002 Sun Microsystems, Inc. ALL RIGHTS RESERVED.
//
// The elliptic curve binary polynomial software is originally written by
// Sheueling Chang Shantz and Douglas Stebila of Sun Microsystems
// Laboratories.
//
// SPDX-License-Identifier: Apache-2.0

#include <openssl/ec.h>

#include <openssl/bn.h>
#include <openssl/err.h>

#include "internal.h"

static int is_point_conversion_form_hybrid(int form_bit) {
  return POINT_CONVERSION_HYBRID == (form_bit & ~1u);
}

static int is_hybrid_bytes_consistent(const uint8_t *in, size_t field_len) {
  // Check that the encoded solution in the first byte aligns with the computed
  // point's, i.e., that they have the same parity.
  return ((in[0] & 1) == (in[1 + field_len * 2 - 1] & 1));
}

size_t ec_point_byte_len(const EC_GROUP *group, point_conversion_form_t form) {
  if (form != POINT_CONVERSION_COMPRESSED &&
      form != POINT_CONVERSION_UNCOMPRESSED &&
      form != POINT_CONVERSION_HYBRID) {
    OPENSSL_PUT_ERROR(EC, EC_R_INVALID_FORM);
    return 0;
  }

  const size_t field_len = BN_num_bytes(&group->field.N);
  size_t output_len = 1 /* type byte */ + field_len;
  if (form == POINT_CONVERSION_UNCOMPRESSED ||
      form == POINT_CONVERSION_HYBRID) {
    // Uncompressed and Hybrid points have a second coordinate.
    output_len += field_len;
  }
  return output_len;
}

size_t ec_point_to_bytes(const EC_GROUP *group, const EC_AFFINE *point,
                         point_conversion_form_t form, uint8_t *buf,
                         size_t len) {
  size_t output_len = ec_point_byte_len(group, form);
  if (len < output_len) {
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
    if (form == POINT_CONVERSION_HYBRID) {
      // |POINT_CONVERSION_HYBRID| specifies y's solution of the quadratic
      // equation, but also encodes the y coordinate along with it.
      OPENSSL_memcpy(buf + 1 + field_len, y_buf, field_len);
    }
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

static int ec_point_from_hybrid(const EC_GROUP *group, EC_AFFINE *out,
                               const uint8_t *in, size_t len) {
  const size_t field_len = BN_num_bytes(&group->field.N);
  // |POINT_CONVERSION_HYBRID| has the solution of y encoded in the first byte
  // as well.
  if (len != 1 + 2 * field_len || !is_point_conversion_form_hybrid(in[0]) ||
      !is_hybrid_bytes_consistent(in, field_len)) {
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

  point_conversion_form_t form = buf[0];

  // OpenSSL supports decoding infinity.
  if (form == 0) {
    if (len != 1) {
      OPENSSL_PUT_ERROR(EC, EC_R_INVALID_ENCODING);
      return 0;
    }
    ec_GFp_simple_point_set_to_infinity(group, &point->raw);
    return 1;
  }

  const int y_bit = form & 1;
  form = form & ~1u;

  if (form == POINT_CONVERSION_UNCOMPRESSED) {
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

  if (form == POINT_CONVERSION_HYBRID) {
    EC_AFFINE affine;
    if (!ec_point_from_hybrid(group, &affine, buf, len)) {
      ec_set_to_safe_point(group, &point->raw);
      return 0;
    }
    ec_affine_to_jacobian(group, &point->raw, &affine);
    return 1;
  }

  const size_t field_len = BN_num_bytes(&group->field.N);
  if (form != POINT_CONVERSION_COMPRESSED ||
      len != 1 /* type byte */ + field_len) {
    OPENSSL_PUT_ERROR(EC, EC_R_INVALID_ENCODING);
    return 0;
  }

  // TODO(davidben): Integrate compressed coordinates with the lower-level EC
  // abstractions. This requires a way to compute square roots, which is tricky
  // for primes which are not 3 (mod 4), namely P-224 and custom curves. P-224's
  // prime is particularly inconvenient for compressed coordinates. See
  // https://cr.yp.to/papers/sqroot.pdf
  BN_CTX *new_ctx = NULL;
  if (ctx == NULL) {
    ctx = new_ctx = BN_CTX_new();
    if (ctx == NULL) {
      return 0;
    }
  }

  int ret = 0;
  BN_CTX_start(ctx);
  BIGNUM *x = BN_CTX_get(ctx);
  if (x == NULL || !BN_bin2bn(buf + 1, field_len, x)) {
    goto err;
  }
  if (BN_ucmp(x, &group->field.N) >= 0) {
    OPENSSL_PUT_ERROR(EC, EC_R_INVALID_ENCODING);
    goto err;
  }

  if (!EC_POINT_set_compressed_coordinates_GFp(group, point, x, y_bit, ctx)) {
    goto err;
  }

  ret = 1;

err:
  BN_CTX_end(ctx);
  BN_CTX_free(new_ctx);
  return ret;
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
                          size_t len, BN_CTX *ctx) {
  if (EC_GROUP_cmp(group, point->group, NULL) != 0) {
    OPENSSL_PUT_ERROR(EC, EC_R_INCOMPATIBLE_OBJECTS);
    return 0;
  }

  // OpenSSL encodes infinity to a single 0 octet.
  if (ec_GFp_simple_is_at_infinity(group, &point->raw)) {
    if(buf != NULL) {
      if (len < 1) {
        OPENSSL_PUT_ERROR(EC, EC_R_BUFFER_TOO_SMALL);
        return 0;
      }
      buf[0] = 0;
    }
    return 1;
  }

  if (buf == NULL) {
    // When |buf| is NULL, just return the number of bytes that would be
    // written, without doing an expensive Jacobian-to-affine conversion.
    return ec_point_byte_len(group, form);
  }
  EC_AFFINE affine;
  if (!ec_jacobian_to_affine(group, &affine, &point->raw)) {
    return 0;
  }
  return ec_point_to_bytes(group, &affine, form, buf, len);
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

  BN_CTX *new_ctx = NULL;
  int ret = 0;

  ERR_clear_error();

  if (ctx == NULL) {
    ctx = new_ctx = BN_CTX_new();
    if (ctx == NULL) {
      return 0;
    }
  }

  y_bit = (y_bit != 0);

  BN_CTX_start(ctx);
  BIGNUM *tmp1 = BN_CTX_get(ctx);
  BIGNUM *tmp2 = BN_CTX_get(ctx);
  BIGNUM *a = BN_CTX_get(ctx);
  BIGNUM *b = BN_CTX_get(ctx);
  BIGNUM *y = BN_CTX_get(ctx);
  if (y == NULL ||
      !EC_GROUP_get_curve_GFp(group, NULL, a, b, ctx)) {
    goto err;
  }

  // Recover y.  We have a Weierstrass equation
  //     y^2 = x^3 + a*x + b,
  // so  y  is one of the square roots of  x^3 + a*x + b.

  // tmp1 := x^3
  if (!BN_mod_sqr(tmp2, x, field, ctx) ||
      !BN_mod_mul(tmp1, tmp2, x, field, ctx)) {
    goto err;
  }

  // tmp1 := tmp1 + a*x
  if (group->a_is_minus3) {
    if (!bn_mod_lshift1_consttime(tmp2, x, field, ctx) ||
        !bn_mod_add_consttime(tmp2, tmp2, x, field, ctx) ||
        !bn_mod_sub_consttime(tmp1, tmp1, tmp2, field, ctx)) {
      goto err;
    }
  } else {
    if (!BN_mod_mul(tmp2, a, x, field, ctx) ||
        !bn_mod_add_consttime(tmp1, tmp1, tmp2, field, ctx)) {
      goto err;
    }
  }

  // tmp1 := tmp1 + b
  if (!bn_mod_add_consttime(tmp1, tmp1, b, field, ctx)) {
    goto err;
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
    goto err;
  }

  if (y_bit != BN_is_odd(y)) {
    if (BN_is_zero(y)) {
      OPENSSL_PUT_ERROR(EC, EC_R_INVALID_COMPRESSION_BIT);
      goto err;
    }
    if (!BN_usub(y, field, y)) {
      goto err;
    }
  }
  if (y_bit != BN_is_odd(y)) {
    OPENSSL_PUT_ERROR(EC, ERR_R_INTERNAL_ERROR);
    goto err;
  }

  if (!EC_POINT_set_affine_coordinates_GFp(group, point, x, y, ctx)) {
    goto err;
  }

  ret = 1;

err:
  BN_CTX_end(ctx);
  BN_CTX_free(new_ctx);
  return ret;
}
