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

#include <openssl/bn.h>
#include <openssl/err.h>
#include <openssl/mem.h>

#include "../bn/internal.h"
#include "../delocate.h"
#include "internal.h"


static void ec_GFp_mont_felem_to_montgomery(const EC_GROUP *group,
                                            EC_FELEM *out, const EC_FELEM *in) {
  bn_to_montgomery_small(out->words, in->words, group->field.N.width,
                         &group->field);
}

static void ec_GFp_mont_felem_from_montgomery(const EC_GROUP *group,
                                              EC_FELEM *out,
                                              const EC_FELEM *in) {
  bn_from_montgomery_small(out->words, group->field.N.width, in->words,
                           group->field.N.width, &group->field);
}

static void ec_GFp_mont_felem_inv0(const EC_GROUP *group, EC_FELEM *out,
                                   const EC_FELEM *a) {
  bn_mod_inverse0_prime_mont_small(out->words, a->words, group->field.N.width,
                                   &group->field);
}

void ec_GFp_mont_felem_mul(const EC_GROUP *group, EC_FELEM *r,
                           const EC_FELEM *a, const EC_FELEM *b) {
  bn_mod_mul_montgomery_small(r->words, a->words, b->words,
                              group->field.N.width, &group->field);
}

void ec_GFp_mont_felem_sqr(const EC_GROUP *group, EC_FELEM *r,
                           const EC_FELEM *a) {
  bn_mod_mul_montgomery_small(r->words, a->words, a->words,
                              group->field.N.width, &group->field);
}

void ec_GFp_mont_felem_to_bytes(const EC_GROUP *group, uint8_t *out,
                                size_t *out_len, const EC_FELEM *in) {
  EC_FELEM tmp;
  ec_GFp_mont_felem_from_montgomery(group, &tmp, in);
  ec_GFp_simple_felem_to_bytes(group, out, out_len, &tmp);
}

int ec_GFp_mont_felem_from_bytes(const EC_GROUP *group, EC_FELEM *out,
                                 const uint8_t *in, size_t len) {
  if (!ec_GFp_simple_felem_from_bytes(group, out, in, len)) {
    return 0;
  }

  ec_GFp_mont_felem_to_montgomery(group, out, out);
  return 1;
}

void ec_GFp_mont_felem_reduce(const EC_GROUP *group, EC_FELEM *out,
                              const BN_ULONG *words, size_t num) {
  // Convert "from" Montgomery form so the value is reduced mod p.
  bn_from_montgomery_small(out->words, group->field.N.width, words, num,
                           &group->field);
  // Convert "to" Montgomery form to remove the R^-1 factor added.
  ec_GFp_mont_felem_to_montgomery(group, out, out);
  // Convert to Montgomery form to match this implementation's representation.
  ec_GFp_mont_felem_to_montgomery(group, out, out);
}

void ec_GFp_mont_felem_exp(const EC_GROUP *group, EC_FELEM *out,
                           const EC_FELEM *a, const BN_ULONG *exp,
                           size_t num_exp) {
  bn_mod_exp_mont_small(out->words, a->words, group->field.N.width, exp,
                        num_exp, &group->field);
}

static int ec_GFp_mont_point_get_affine_coordinates(const EC_GROUP *group,
                                                    const EC_JACOBIAN *point,
                                                    EC_FELEM *x, EC_FELEM *y) {
  if (constant_time_declassify_int(
          ec_GFp_simple_is_at_infinity(group, point))) {
    OPENSSL_PUT_ERROR(EC, EC_R_POINT_AT_INFINITY);
    return 0;
  }

  // Transform (X, Y, Z) into (x, y) := (X/Z^2, Y/Z^3). Note the check above
  // ensures |point->Z| is non-zero, so the inverse always exists.
  EC_FELEM z1, z2;
  ec_GFp_mont_felem_inv0(group, &z2, &point->Z);
  ec_GFp_mont_felem_sqr(group, &z1, &z2);

  if (x != NULL) {
    ec_GFp_mont_felem_mul(group, x, &point->X, &z1);
  }

  if (y != NULL) {
    ec_GFp_mont_felem_mul(group, &z1, &z1, &z2);
    ec_GFp_mont_felem_mul(group, y, &point->Y, &z1);
  }

  return 1;
}

static int ec_GFp_mont_jacobian_to_affine_batch(const EC_GROUP *group,
                                                EC_AFFINE *out,
                                                const EC_JACOBIAN *in,
                                                size_t num) {
  if (num == 0) {
    return 1;
  }

  // Compute prefix products of all Zs. Use |out[i].X| as scratch space
  // to store these values.
  out[0].X = in[0].Z;
  for (size_t i = 1; i < num; i++) {
    ec_GFp_mont_felem_mul(group, &out[i].X, &out[i - 1].X, &in[i].Z);
  }

  // Some input was infinity iff the product of all Zs is zero.
  if (ec_felem_non_zero_mask(group, &out[num - 1].X) == 0) {
    OPENSSL_PUT_ERROR(EC, EC_R_POINT_AT_INFINITY);
    return 0;
  }

  // Invert the product of all Zs.
  EC_FELEM zinvprod;
  ec_GFp_mont_felem_inv0(group, &zinvprod, &out[num - 1].X);
  for (size_t i = num - 1; i < num; i--) {
    // Our loop invariant is that |zinvprod| is Z0^-1 * Z1^-1 * ... * Zi^-1.
    // Recover Zi^-1 by multiplying by the previous product.
    EC_FELEM zinv, zinv2;
    if (i == 0) {
      zinv = zinvprod;
    } else {
      ec_GFp_mont_felem_mul(group, &zinv, &zinvprod, &out[i - 1].X);
      // Maintain the loop invariant for the next iteration.
      ec_GFp_mont_felem_mul(group, &zinvprod, &zinvprod, &in[i].Z);
    }

    // Compute affine coordinates: x = X * Z^-2 and y = Y * Z^-3.
    ec_GFp_mont_felem_sqr(group, &zinv2, &zinv);
    ec_GFp_mont_felem_mul(group, &out[i].X, &in[i].X, &zinv2);
    ec_GFp_mont_felem_mul(group, &out[i].Y, &in[i].Y, &zinv2);
    ec_GFp_mont_felem_mul(group, &out[i].Y, &out[i].Y, &zinv);
  }

  return 1;
}

void ec_GFp_mont_add(const EC_GROUP *group, EC_JACOBIAN *out,
                     const EC_JACOBIAN *a, const EC_JACOBIAN *b) {
  if (a == b) {
    ec_GFp_mont_dbl(group, out, a);
    return;
  }

  // The method is taken from:
  //   http://hyperelliptic.org/EFD/g1p/auto-shortw-jacobian.html#addition-add-2007-bl
  //
  // Coq transcription and correctness proof:
  // <https://github.com/davidben/fiat-crypto/blob/c7b95f62b2a54b559522573310e9b487327d219a/src/Curves/Weierstrass/Jacobian.v#L467>
  // <https://github.com/davidben/fiat-crypto/blob/c7b95f62b2a54b559522573310e9b487327d219a/src/Curves/Weierstrass/Jacobian.v#L544>
  EC_FELEM x_out, y_out, z_out;
  BN_ULONG z1nz = ec_felem_non_zero_mask(group, &a->Z);
  BN_ULONG z2nz = ec_felem_non_zero_mask(group, &b->Z);

  // z1z1 = z1z1 = z1**2
  EC_FELEM z1z1;
  ec_GFp_mont_felem_sqr(group, &z1z1, &a->Z);

  // z2z2 = z2**2
  EC_FELEM z2z2;
  ec_GFp_mont_felem_sqr(group, &z2z2, &b->Z);

  // u1 = x1*z2z2
  EC_FELEM u1;
  ec_GFp_mont_felem_mul(group, &u1, &a->X, &z2z2);

  // two_z1z2 = (z1 + z2)**2 - (z1z1 + z2z2) = 2z1z2
  EC_FELEM two_z1z2;
  ec_felem_add(group, &two_z1z2, &a->Z, &b->Z);
  ec_GFp_mont_felem_sqr(group, &two_z1z2, &two_z1z2);
  ec_felem_sub(group, &two_z1z2, &two_z1z2, &z1z1);
  ec_felem_sub(group, &two_z1z2, &two_z1z2, &z2z2);

  // s1 = y1 * z2**3
  EC_FELEM s1;
  ec_GFp_mont_felem_mul(group, &s1, &b->Z, &z2z2);
  ec_GFp_mont_felem_mul(group, &s1, &s1, &a->Y);

  // u2 = x2*z1z1
  EC_FELEM u2;
  ec_GFp_mont_felem_mul(group, &u2, &b->X, &z1z1);

  // h = u2 - u1
  EC_FELEM h;
  ec_felem_sub(group, &h, &u2, &u1);

  BN_ULONG xneq = ec_felem_non_zero_mask(group, &h);

  // z_out = two_z1z2 * h
  ec_GFp_mont_felem_mul(group, &z_out, &h, &two_z1z2);

  // z1z1z1 = z1 * z1z1
  EC_FELEM z1z1z1;
  ec_GFp_mont_felem_mul(group, &z1z1z1, &a->Z, &z1z1);

  // s2 = y2 * z1**3
  EC_FELEM s2;
  ec_GFp_mont_felem_mul(group, &s2, &b->Y, &z1z1z1);

  // r = (s2 - s1)*2
  EC_FELEM r;
  ec_felem_sub(group, &r, &s2, &s1);
  ec_felem_add(group, &r, &r, &r);

  BN_ULONG yneq = ec_felem_non_zero_mask(group, &r);

  // This case will never occur in the constant-time |ec_GFp_mont_mul|.
  BN_ULONG is_nontrivial_double = ~xneq & ~yneq & z1nz & z2nz;
  if (constant_time_declassify_w(is_nontrivial_double)) {
    ec_GFp_mont_dbl(group, out, a);
    return;
  }

  // I = (2h)**2
  EC_FELEM i;
  ec_felem_add(group, &i, &h, &h);
  ec_GFp_mont_felem_sqr(group, &i, &i);

  // J = h * I
  EC_FELEM j;
  ec_GFp_mont_felem_mul(group, &j, &h, &i);

  // V = U1 * I
  EC_FELEM v;
  ec_GFp_mont_felem_mul(group, &v, &u1, &i);

  // x_out = r**2 - J - 2V
  ec_GFp_mont_felem_sqr(group, &x_out, &r);
  ec_felem_sub(group, &x_out, &x_out, &j);
  ec_felem_sub(group, &x_out, &x_out, &v);
  ec_felem_sub(group, &x_out, &x_out, &v);

  // y_out = r(V-x_out) - 2 * s1 * J
  ec_felem_sub(group, &y_out, &v, &x_out);
  ec_GFp_mont_felem_mul(group, &y_out, &y_out, &r);
  EC_FELEM s1j;
  ec_GFp_mont_felem_mul(group, &s1j, &s1, &j);
  ec_felem_sub(group, &y_out, &y_out, &s1j);
  ec_felem_sub(group, &y_out, &y_out, &s1j);

  ec_felem_select(group, &x_out, z1nz, &x_out, &b->X);
  ec_felem_select(group, &out->X, z2nz, &x_out, &a->X);
  ec_felem_select(group, &y_out, z1nz, &y_out, &b->Y);
  ec_felem_select(group, &out->Y, z2nz, &y_out, &a->Y);
  ec_felem_select(group, &z_out, z1nz, &z_out, &b->Z);
  ec_felem_select(group, &out->Z, z2nz, &z_out, &a->Z);
}

void ec_GFp_mont_dbl(const EC_GROUP *group, EC_JACOBIAN *r,
                     const EC_JACOBIAN *a) {
  if (group->a_is_minus3) {
    // The method is taken from:
    //   http://hyperelliptic.org/EFD/g1p/auto-shortw-jacobian-3.html#doubling-dbl-2001-b
    //
    // Coq transcription and correctness proof:
    // <https://github.com/mit-plv/fiat-crypto/blob/79f8b5f39ed609339f0233098dee1a3c4e6b3080/src/Curves/Weierstrass/Jacobian.v#L93>
    // <https://github.com/mit-plv/fiat-crypto/blob/79f8b5f39ed609339f0233098dee1a3c4e6b3080/src/Curves/Weierstrass/Jacobian.v#L201>
    EC_FELEM delta, gamma, beta, ftmp, ftmp2, tmptmp, alpha, fourbeta;
    // delta = z^2
    ec_GFp_mont_felem_sqr(group, &delta, &a->Z);
    // gamma = y^2
    ec_GFp_mont_felem_sqr(group, &gamma, &a->Y);
    // beta = x*gamma
    ec_GFp_mont_felem_mul(group, &beta, &a->X, &gamma);

    // alpha = 3*(x-delta)*(x+delta)
    ec_felem_sub(group, &ftmp, &a->X, &delta);
    ec_felem_add(group, &ftmp2, &a->X, &delta);

    ec_felem_add(group, &tmptmp, &ftmp2, &ftmp2);
    ec_felem_add(group, &ftmp2, &ftmp2, &tmptmp);
    ec_GFp_mont_felem_mul(group, &alpha, &ftmp, &ftmp2);

    // x' = alpha^2 - 8*beta
    ec_GFp_mont_felem_sqr(group, &r->X, &alpha);
    ec_felem_add(group, &fourbeta, &beta, &beta);
    ec_felem_add(group, &fourbeta, &fourbeta, &fourbeta);
    ec_felem_add(group, &tmptmp, &fourbeta, &fourbeta);
    ec_felem_sub(group, &r->X, &r->X, &tmptmp);

    // z' = (y + z)^2 - gamma - delta
    ec_felem_add(group, &delta, &gamma, &delta);
    ec_felem_add(group, &ftmp, &a->Y, &a->Z);
    ec_GFp_mont_felem_sqr(group, &r->Z, &ftmp);
    ec_felem_sub(group, &r->Z, &r->Z, &delta);

    // y' = alpha*(4*beta - x') - 8*gamma^2
    ec_felem_sub(group, &r->Y, &fourbeta, &r->X);
    ec_felem_add(group, &gamma, &gamma, &gamma);
    ec_GFp_mont_felem_sqr(group, &gamma, &gamma);
    ec_GFp_mont_felem_mul(group, &r->Y, &alpha, &r->Y);
    ec_felem_add(group, &gamma, &gamma, &gamma);
    ec_felem_sub(group, &r->Y, &r->Y, &gamma);
  } else {
    // The method is taken from:
    //   http://www.hyperelliptic.org/EFD/g1p/auto-shortw-jacobian.html#doubling-dbl-2007-bl
    //
    // Coq transcription and correctness proof:
    // <https://github.com/davidben/fiat-crypto/blob/c7b95f62b2a54b559522573310e9b487327d219a/src/Curves/Weierstrass/Jacobian.v#L102>
    // <https://github.com/davidben/fiat-crypto/blob/c7b95f62b2a54b559522573310e9b487327d219a/src/Curves/Weierstrass/Jacobian.v#L534>
    EC_FELEM xx, yy, yyyy, zz;
    ec_GFp_mont_felem_sqr(group, &xx, &a->X);
    ec_GFp_mont_felem_sqr(group, &yy, &a->Y);
    ec_GFp_mont_felem_sqr(group, &yyyy, &yy);
    ec_GFp_mont_felem_sqr(group, &zz, &a->Z);

    // s = 2*((x_in + yy)^2 - xx - yyyy)
    EC_FELEM s;
    ec_felem_add(group, &s, &a->X, &yy);
    ec_GFp_mont_felem_sqr(group, &s, &s);
    ec_felem_sub(group, &s, &s, &xx);
    ec_felem_sub(group, &s, &s, &yyyy);
    ec_felem_add(group, &s, &s, &s);

    // m = 3*xx + a*zz^2
    EC_FELEM m;
    ec_GFp_mont_felem_sqr(group, &m, &zz);
    ec_GFp_mont_felem_mul(group, &m, &group->a, &m);
    ec_felem_add(group, &m, &m, &xx);
    ec_felem_add(group, &m, &m, &xx);
    ec_felem_add(group, &m, &m, &xx);

    // x_out = m^2 - 2*s
    ec_GFp_mont_felem_sqr(group, &r->X, &m);
    ec_felem_sub(group, &r->X, &r->X, &s);
    ec_felem_sub(group, &r->X, &r->X, &s);

    // z_out = (y_in + z_in)^2 - yy - zz
    ec_felem_add(group, &r->Z, &a->Y, &a->Z);
    ec_GFp_mont_felem_sqr(group, &r->Z, &r->Z);
    ec_felem_sub(group, &r->Z, &r->Z, &yy);
    ec_felem_sub(group, &r->Z, &r->Z, &zz);

    // y_out = m*(s-x_out) - 8*yyyy
    ec_felem_add(group, &yyyy, &yyyy, &yyyy);
    ec_felem_add(group, &yyyy, &yyyy, &yyyy);
    ec_felem_add(group, &yyyy, &yyyy, &yyyy);
    ec_felem_sub(group, &r->Y, &s, &r->X);
    ec_GFp_mont_felem_mul(group, &r->Y, &r->Y, &m);
    ec_felem_sub(group, &r->Y, &r->Y, &yyyy);
  }
}

static int ec_GFp_mont_cmp_x_coordinate(const EC_GROUP *group,
                                        const EC_JACOBIAN *p,
                                        const EC_SCALAR *r) {
  if (!group->field_greater_than_order ||
      group->field.N.width != group->order.N.width) {
    // Do not bother optimizing this case. p > order in all commonly-used
    // curves.
    return ec_GFp_simple_cmp_x_coordinate(group, p, r);
  }

  if (ec_GFp_simple_is_at_infinity(group, p)) {
    return 0;
  }

  // We wish to compare X/Z^2 with r. This is equivalent to comparing X with
  // r*Z^2. Note that X and Z are represented in Montgomery form, while r is
  // not.
  EC_FELEM r_Z2, Z2_mont, X;
  ec_GFp_mont_felem_mul(group, &Z2_mont, &p->Z, &p->Z);
  // r < order < p, so this is valid.
  OPENSSL_memcpy(r_Z2.words, r->words, group->field.N.width * sizeof(BN_ULONG));
  ec_GFp_mont_felem_mul(group, &r_Z2, &r_Z2, &Z2_mont);
  ec_GFp_mont_felem_from_montgomery(group, &X, &p->X);

  if (ec_felem_equal(group, &r_Z2, &X)) {
    return 1;
  }

  // During signing the x coefficient is reduced modulo the group order.
  // Therefore there is a small possibility, less than 1/2^128, that group_order
  // < p.x < P. in that case we need not only to compare against |r| but also to
  // compare against r+group_order.
  BN_ULONG carry = bn_add_words(r_Z2.words, r->words, group->order.N.d,
                                group->field.N.width);
  if (carry == 0 &&
      bn_less_than_words(r_Z2.words, group->field.N.d, group->field.N.width)) {
    // r + group_order < p, so compare (r + group_order) * Z^2 against X.
    ec_GFp_mont_felem_mul(group, &r_Z2, &r_Z2, &Z2_mont);
    if (ec_felem_equal(group, &r_Z2, &X)) {
      return 1;
    }
  }

  return 0;
}

DEFINE_METHOD_FUNCTION(EC_METHOD, EC_GFp_mont_method) {
  out->point_get_affine_coordinates = ec_GFp_mont_point_get_affine_coordinates;
  out->jacobian_to_affine_batch = ec_GFp_mont_jacobian_to_affine_batch;
  out->add = ec_GFp_mont_add;
  out->dbl = ec_GFp_mont_dbl;
  out->mul = ec_GFp_mont_mul;
  out->mul_base = ec_GFp_mont_mul_base;
  out->mul_batch = ec_GFp_mont_mul_batch;
  out->mul_public_batch = ec_GFp_mont_mul_public_batch;
  out->init_precomp = ec_GFp_mont_init_precomp;
  out->mul_precomp = ec_GFp_mont_mul_precomp;
  out->felem_mul = ec_GFp_mont_felem_mul;
  out->felem_sqr = ec_GFp_mont_felem_sqr;
  out->felem_to_bytes = ec_GFp_mont_felem_to_bytes;
  out->felem_from_bytes = ec_GFp_mont_felem_from_bytes;
  out->felem_reduce = ec_GFp_mont_felem_reduce;
  out->felem_exp = ec_GFp_mont_felem_exp;
  out->scalar_inv0_montgomery = ec_simple_scalar_inv0_montgomery;
  out->scalar_to_montgomery_inv_vartime =
      ec_simple_scalar_to_montgomery_inv_vartime;
  out->cmp_x_coordinate = ec_GFp_mont_cmp_x_coordinate;
}
