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

#include <assert.h>
#include <string.h>

#include <openssl/bn.h>
#include <openssl/err.h>
#include <openssl/mem.h>
#include <openssl/nid.h>

#include "../../internal.h"
#include "../bn/internal.h"
#include "../delocate.h"
#include "internal.h"

#include "builtin_curves.h"


static void ec_point_free(EC_POINT *point, int free_group);

static void ec_group_init_static_mont(BN_MONT_CTX *mont, size_t num_words,
                                      const BN_ULONG *modulus,
                                      const BN_ULONG *rr, uint64_t n0) {
  bn_set_static_words(&mont->N, modulus, num_words);
  bn_set_static_words(&mont->RR, rr, num_words);
#if defined(OPENSSL_64_BIT)
  mont->n0[0] = n0;
#elif defined(OPENSSL_32_BIT)
  mont->n0[0] = (uint32_t)n0;
  mont->n0[1] = (uint32_t)(n0 >> 32);
#else
#error "unknown word length"
#endif
}

static void ec_group_set_a_minus3(EC_GROUP *group) {
  const EC_FELEM *one = ec_felem_one(group);
  group->a_is_minus3 = 1;
  ec_felem_neg(group, &group->a, one);
  ec_felem_sub(group, &group->a, &group->a, one);
  ec_felem_sub(group, &group->a, &group->a, one);
}

DEFINE_METHOD_FUNCTION(EC_GROUP, EC_group_p224) {
  out->curve_name = NID_secp224r1;
  out->comment = "NIST P-224";
  // 1.3.132.0.33
  static const uint8_t kOIDP224[] = {0x2b, 0x81, 0x04, 0x00, 0x21};
  OPENSSL_memcpy(out->oid, kOIDP224, sizeof(kOIDP224));
  out->oid_len = sizeof(kOIDP224);

  ec_group_init_static_mont(&out->field, OPENSSL_ARRAY_SIZE(kP224Field),
                            kP224Field, kP224FieldRR, kP224FieldN0);
  ec_group_init_static_mont(&out->order, OPENSSL_ARRAY_SIZE(kP224Order),
                            kP224Order, kP224OrderRR, kP224OrderN0);

#if defined(BORINGSSL_HAS_UINT128) && !defined(OPENSSL_SMALL)
  out->meth = EC_GFp_nistp224_method();
  OPENSSL_memcpy(out->generator.raw.X.words, kP224GX, sizeof(kP224GX));
  OPENSSL_memcpy(out->generator.raw.Y.words, kP224GY, sizeof(kP224GY));
  out->generator.raw.Z.words[0] = 1;
  OPENSSL_memcpy(out->b.words, kP224B, sizeof(kP224B));
#else
  out->meth = EC_GFp_mont_method();
  OPENSSL_memcpy(out->generator.raw.X.words, kP224MontGX, sizeof(kP224MontGX));
  OPENSSL_memcpy(out->generator.raw.Y.words, kP224MontGY, sizeof(kP224MontGY));
  OPENSSL_memcpy(out->generator.raw.Z.words, kP224FieldR, sizeof(kP224FieldR));
  OPENSSL_memcpy(out->b.words, kP224MontB, sizeof(kP224MontB));
#endif
  out->generator.group = out;

  ec_group_set_a_minus3(out);
  out->has_order = 1;
  out->field_greater_than_order = 1;
}

DEFINE_METHOD_FUNCTION(EC_GROUP, EC_group_p256) {
  out->curve_name = NID_X9_62_prime256v1;
  out->comment = "NIST P-256";
  // 1.2.840.10045.3.1.7
  static const uint8_t kOIDP256[] = {0x2a, 0x86, 0x48, 0xce,
                                     0x3d, 0x03, 0x01, 0x07};
  OPENSSL_memcpy(out->oid, kOIDP256, sizeof(kOIDP256));
  out->oid_len = sizeof(kOIDP256);

  ec_group_init_static_mont(&out->field, OPENSSL_ARRAY_SIZE(kP256Field),
                            kP256Field, kP256FieldRR, kP256FieldN0);
  ec_group_init_static_mont(&out->order, OPENSSL_ARRAY_SIZE(kP256Order),
                            kP256Order, kP256OrderRR, kP256OrderN0);

#if !defined(OPENSSL_NO_ASM) &&                              \
    (defined(OPENSSL_X86_64) || defined(OPENSSL_AARCH64)) && \
    !defined(OPENSSL_SMALL)
  out->meth = EC_GFp_nistz256_method();
#else
  out->meth = EC_GFp_nistp256_method();
#endif
  out->generator.group = out;
  OPENSSL_memcpy(out->generator.raw.X.words, kP256MontGX, sizeof(kP256MontGX));
  OPENSSL_memcpy(out->generator.raw.Y.words, kP256MontGY, sizeof(kP256MontGY));
  OPENSSL_memcpy(out->generator.raw.Z.words, kP256FieldR, sizeof(kP256FieldR));
  OPENSSL_memcpy(out->b.words, kP256MontB, sizeof(kP256MontB));

  ec_group_set_a_minus3(out);
  out->has_order = 1;
  out->field_greater_than_order = 1;
}

DEFINE_METHOD_FUNCTION(EC_GROUP, EC_group_p384) {
  out->curve_name = NID_secp384r1;
  out->comment = "NIST P-384";
  // 1.3.132.0.34
  static const uint8_t kOIDP384[] = {0x2b, 0x81, 0x04, 0x00, 0x22};
  OPENSSL_memcpy(out->oid, kOIDP384, sizeof(kOIDP384));
  out->oid_len = sizeof(kOIDP384);

  ec_group_init_static_mont(&out->field, OPENSSL_ARRAY_SIZE(kP384Field),
                            kP384Field, kP384FieldRR, kP384FieldN0);
  ec_group_init_static_mont(&out->order, OPENSSL_ARRAY_SIZE(kP384Order),
                            kP384Order, kP384OrderRR, kP384OrderN0);

  out->meth = EC_GFp_mont_method();
  out->generator.group = out;
  OPENSSL_memcpy(out->generator.raw.X.words, kP384MontGX, sizeof(kP384MontGX));
  OPENSSL_memcpy(out->generator.raw.Y.words, kP384MontGY, sizeof(kP384MontGY));
  OPENSSL_memcpy(out->generator.raw.Z.words, kP384FieldR, sizeof(kP384FieldR));
  OPENSSL_memcpy(out->b.words, kP384MontB, sizeof(kP384MontB));

  ec_group_set_a_minus3(out);
  out->has_order = 1;
  out->field_greater_than_order = 1;
}

DEFINE_METHOD_FUNCTION(EC_GROUP, EC_group_p521) {
  out->curve_name = NID_secp521r1;
  out->comment = "NIST P-521";
  // 1.3.132.0.35
  static const uint8_t kOIDP521[] = {0x2b, 0x81, 0x04, 0x00, 0x23};
  OPENSSL_memcpy(out->oid, kOIDP521, sizeof(kOIDP521));
  out->oid_len = sizeof(kOIDP521);

  ec_group_init_static_mont(&out->field, OPENSSL_ARRAY_SIZE(kP521Field),
                            kP521Field, kP521FieldRR, kP521FieldN0);
  ec_group_init_static_mont(&out->order, OPENSSL_ARRAY_SIZE(kP521Order),
                            kP521Order, kP521OrderRR, kP521OrderN0);

  out->meth = EC_GFp_mont_method();
  out->generator.group = out;
  OPENSSL_memcpy(out->generator.raw.X.words, kP521MontGX, sizeof(kP521MontGX));
  OPENSSL_memcpy(out->generator.raw.Y.words, kP521MontGY, sizeof(kP521MontGY));
  OPENSSL_memcpy(out->generator.raw.Z.words, kP521FieldR, sizeof(kP521FieldR));
  OPENSSL_memcpy(out->b.words, kP521MontB, sizeof(kP521MontB));

  ec_group_set_a_minus3(out);
  out->has_order = 1;
  out->field_greater_than_order = 1;
}

EC_GROUP *EC_GROUP_new_curve_GFp(const BIGNUM *p, const BIGNUM *a,
                                 const BIGNUM *b, BN_CTX *ctx) {
  if (BN_num_bytes(p) > EC_MAX_BYTES) {
    OPENSSL_PUT_ERROR(EC, EC_R_INVALID_FIELD);
    return nullptr;
  }

  bssl::UniquePtr<BN_CTX> new_ctx;
  if (ctx == nullptr) {
    new_ctx.reset(BN_CTX_new());
    if (new_ctx == nullptr) {
      return nullptr;
    }
    ctx = new_ctx.get();
  }

  // Historically, |a| and |b| were not required to be fully reduced.
  // TODO(davidben): Can this be removed?
  bssl::BN_CTXScope scope(ctx);
  BIGNUM *a_reduced = BN_CTX_get(ctx);
  BIGNUM *b_reduced = BN_CTX_get(ctx);
  if (a_reduced == nullptr || b_reduced == nullptr ||
      !BN_nnmod(a_reduced, a, p, ctx) ||  //
      !BN_nnmod(b_reduced, b, p, ctx)) {
    return nullptr;
  }

  bssl::UniquePtr<EC_GROUP> ret(
      reinterpret_cast<EC_GROUP *>(OPENSSL_zalloc(sizeof(EC_GROUP))));
  if (ret == nullptr) {
    return nullptr;
  }
  ret->references = 1;
  ret->meth = EC_GFp_mont_method();
  bn_mont_ctx_init(&ret->field);
  bn_mont_ctx_init(&ret->order);
  ret->generator.group = ret.get();
  if (!ec_GFp_simple_group_set_curve(ret.get(), p, a_reduced, b_reduced, ctx)) {
    return nullptr;
  }

  return ret.release();
}

int EC_GROUP_set_generator(EC_GROUP *group, const EC_POINT *generator,
                           const BIGNUM *order, const BIGNUM *cofactor) {
  if (group->curve_name != NID_undef || group->has_order ||
      generator->group != group) {
    // |EC_GROUP_set_generator| may only be used with |EC_GROUP|s returned by
    // |EC_GROUP_new_curve_GFp| and may only used once on each group.
    // |generator| must have been created from |EC_GROUP_new_curve_GFp|, not a
    // copy, so that |generator->group->generator| is set correctly.
    OPENSSL_PUT_ERROR(EC, ERR_R_SHOULD_NOT_HAVE_BEEN_CALLED);
    return 0;
  }

  if (BN_num_bytes(order) > EC_MAX_BYTES) {
    OPENSSL_PUT_ERROR(EC, EC_R_INVALID_GROUP_ORDER);
    return 0;
  }

  // Require a cofactor of one for custom curves, which implies prime order.
  if (!BN_is_one(cofactor)) {
    OPENSSL_PUT_ERROR(EC, EC_R_INVALID_COFACTOR);
    return 0;
  }

  // Require that p < 2×order. This simplifies some ECDSA operations.
  //
  // Note any curve which did not satisfy this must have been invalid or use a
  // tiny prime (less than 17). See the proof in |field_element_to_scalar| in
  // the ECDSA implementation.
  bssl::UniquePtr<BIGNUM> tmp(BN_new());
  if (tmp == nullptr || !BN_lshift1(tmp.get(), order)) {
    return 0;
  }
  if (BN_cmp(tmp.get(), &group->field.N) <= 0) {
    OPENSSL_PUT_ERROR(EC, EC_R_INVALID_GROUP_ORDER);
    return 0;
  }

  EC_AFFINE affine;
  if (!ec_jacobian_to_affine(group, &affine, &generator->raw) ||
      !BN_MONT_CTX_set(&group->order, order, NULL)) {
    return 0;
  }

  group->field_greater_than_order = BN_cmp(&group->field.N, order) > 0;
  group->generator.raw.X = affine.X;
  group->generator.raw.Y = affine.Y;
  // |raw.Z| was set to 1 by |EC_GROUP_new_curve_GFp|.
  group->has_order = 1;
  return 1;
}

EC_GROUP *EC_GROUP_new_by_curve_name(int nid) {
  switch (nid) {
    case NID_secp224r1:
      return (EC_GROUP *)EC_group_p224();
    case NID_X9_62_prime256v1:
      return (EC_GROUP *)EC_group_p256();
    case NID_secp384r1:
      return (EC_GROUP *)EC_group_p384();
    case NID_secp521r1:
      return (EC_GROUP *)EC_group_p521();
    default:
      OPENSSL_PUT_ERROR(EC, EC_R_UNKNOWN_GROUP);
      return NULL;
  }
}

void EC_GROUP_free(EC_GROUP *group) {
  if (group == NULL ||
      // Built-in curves are static.
      group->curve_name != NID_undef ||
      !CRYPTO_refcount_dec_and_test_zero(&group->references)) {
    return;
  }

  bn_mont_ctx_cleanup(&group->order);
  bn_mont_ctx_cleanup(&group->field);
  OPENSSL_free(group);
}

EC_GROUP *EC_GROUP_dup(const EC_GROUP *a) {
  if (a == NULL ||
      // Built-in curves are static.
      a->curve_name != NID_undef) {
    return (EC_GROUP *)a;
  }

  // Groups are logically immutable (but for |EC_GROUP_set_generator| which must
  // be called early on), so we simply take a reference.
  EC_GROUP *group = (EC_GROUP *)a;
  CRYPTO_refcount_inc(&group->references);
  return group;
}

int EC_GROUP_cmp(const EC_GROUP *a, const EC_GROUP *b, BN_CTX *ignored) {
  // Note this function returns 0 if equal and non-zero otherwise.
  if (a == b) {
    return 0;
  }
  if (a->curve_name != b->curve_name) {
    return 1;
  }
  if (a->curve_name != NID_undef) {
    // Built-in curves may be compared by curve name alone.
    return 0;
  }

  // |a| and |b| are both custom curves. We compare the entire curve
  // structure. If |a| or |b| is incomplete (due to legacy OpenSSL mistakes,
  // custom curve construction is sadly done in two parts) but otherwise not the
  // same object, we consider them always unequal.
  return a->meth != b->meth ||  //
         !a->has_order || !b->has_order ||
         BN_cmp(&a->order.N, &b->order.N) != 0 ||
         BN_cmp(&a->field.N, &b->field.N) != 0 ||
         !ec_felem_equal(a, &a->a, &b->a) ||  //
         !ec_felem_equal(a, &a->b, &b->b) ||
         !ec_GFp_simple_points_equal(a, &a->generator.raw, &b->generator.raw);
}

const EC_POINT *EC_GROUP_get0_generator(const EC_GROUP *group) {
  return group->has_order ? &group->generator : NULL;
}

const BIGNUM *EC_GROUP_get0_order(const EC_GROUP *group) {
  assert(group->has_order);
  return &group->order.N;
}

int EC_GROUP_get_order(const EC_GROUP *group, BIGNUM *order, BN_CTX *ctx) {
  if (BN_copy(order, EC_GROUP_get0_order(group)) == NULL) {
    return 0;
  }
  return 1;
}

int EC_GROUP_order_bits(const EC_GROUP *group) {
  return BN_num_bits(&group->order.N);
}

int EC_GROUP_get_cofactor(const EC_GROUP *group, BIGNUM *cofactor,
                          BN_CTX *ctx) {
  // All |EC_GROUP|s have cofactor 1.
  return BN_set_word(cofactor, 1);
}

int EC_GROUP_get_curve_GFp(const EC_GROUP *group, BIGNUM *out_p, BIGNUM *out_a,
                           BIGNUM *out_b, BN_CTX *ctx) {
  return ec_GFp_simple_group_get_curve(group, out_p, out_a, out_b);
}

int EC_GROUP_get_curve_name(const EC_GROUP *group) { return group->curve_name; }

unsigned EC_GROUP_get_degree(const EC_GROUP *group) {
  return BN_num_bits(&group->field.N);
}

const char *EC_curve_nid2nist(int nid) {
  switch (nid) {
    case NID_secp224r1:
      return "P-224";
    case NID_X9_62_prime256v1:
      return "P-256";
    case NID_secp384r1:
      return "P-384";
    case NID_secp521r1:
      return "P-521";
  }
  return NULL;
}

int EC_curve_nist2nid(const char *name) {
  if (strcmp(name, "P-224") == 0) {
    return NID_secp224r1;
  }
  if (strcmp(name, "P-256") == 0) {
    return NID_X9_62_prime256v1;
  }
  if (strcmp(name, "P-384") == 0) {
    return NID_secp384r1;
  }
  if (strcmp(name, "P-521") == 0) {
    return NID_secp521r1;
  }
  return NID_undef;
}

EC_POINT *EC_POINT_new(const EC_GROUP *group) {
  if (group == NULL) {
    OPENSSL_PUT_ERROR(EC, ERR_R_PASSED_NULL_PARAMETER);
    return NULL;
  }

  EC_POINT *ret = reinterpret_cast<EC_POINT *>(OPENSSL_malloc(sizeof *ret));
  if (ret == NULL) {
    return NULL;
  }

  ret->group = EC_GROUP_dup(group);
  ec_GFp_simple_point_init(&ret->raw);
  return ret;
}

static void ec_point_free(EC_POINT *point, int free_group) {
  if (!point) {
    return;
  }
  if (free_group) {
    EC_GROUP_free(point->group);
  }
  OPENSSL_free(point);
}

void EC_POINT_free(EC_POINT *point) {
  ec_point_free(point, 1 /* free group */);
}

void EC_POINT_clear_free(EC_POINT *point) { EC_POINT_free(point); }

int EC_POINT_copy(EC_POINT *dest, const EC_POINT *src) {
  if (EC_GROUP_cmp(dest->group, src->group, NULL) != 0) {
    OPENSSL_PUT_ERROR(EC, EC_R_INCOMPATIBLE_OBJECTS);
    return 0;
  }
  if (dest == src) {
    return 1;
  }
  ec_GFp_simple_point_copy(&dest->raw, &src->raw);
  return 1;
}

EC_POINT *EC_POINT_dup(const EC_POINT *a, const EC_GROUP *group) {
  if (a == NULL) {
    return NULL;
  }

  EC_POINT *ret = EC_POINT_new(group);
  if (ret == NULL || !EC_POINT_copy(ret, a)) {
    EC_POINT_free(ret);
    return NULL;
  }

  return ret;
}

int EC_POINT_set_to_infinity(const EC_GROUP *group, EC_POINT *point) {
  if (EC_GROUP_cmp(group, point->group, NULL) != 0) {
    OPENSSL_PUT_ERROR(EC, EC_R_INCOMPATIBLE_OBJECTS);
    return 0;
  }
  ec_GFp_simple_point_set_to_infinity(group, &point->raw);
  return 1;
}

int EC_POINT_is_at_infinity(const EC_GROUP *group, const EC_POINT *point) {
  if (EC_GROUP_cmp(group, point->group, NULL) != 0) {
    OPENSSL_PUT_ERROR(EC, EC_R_INCOMPATIBLE_OBJECTS);
    return 0;
  }
  return ec_GFp_simple_is_at_infinity(group, &point->raw);
}

int EC_POINT_is_on_curve(const EC_GROUP *group, const EC_POINT *point,
                         BN_CTX *ctx) {
  if (EC_GROUP_cmp(group, point->group, NULL) != 0) {
    OPENSSL_PUT_ERROR(EC, EC_R_INCOMPATIBLE_OBJECTS);
    return 0;
  }
  return ec_GFp_simple_is_on_curve(group, &point->raw);
}

int EC_POINT_cmp(const EC_GROUP *group, const EC_POINT *a, const EC_POINT *b,
                 BN_CTX *ctx) {
  if (EC_GROUP_cmp(group, a->group, NULL) != 0 ||
      EC_GROUP_cmp(group, b->group, NULL) != 0) {
    OPENSSL_PUT_ERROR(EC, EC_R_INCOMPATIBLE_OBJECTS);
    return -1;
  }

  // Note |EC_POINT_cmp| returns zero for equality and non-zero for inequality.
  return ec_GFp_simple_points_equal(group, &a->raw, &b->raw) ? 0 : 1;
}

int EC_POINT_get_affine_coordinates_GFp(const EC_GROUP *group,
                                        const EC_POINT *point, BIGNUM *x,
                                        BIGNUM *y, BN_CTX *ctx) {
  if (group->meth->point_get_affine_coordinates == 0) {
    OPENSSL_PUT_ERROR(EC, ERR_R_SHOULD_NOT_HAVE_BEEN_CALLED);
    return 0;
  }
  if (EC_GROUP_cmp(group, point->group, NULL) != 0) {
    OPENSSL_PUT_ERROR(EC, EC_R_INCOMPATIBLE_OBJECTS);
    return 0;
  }
  EC_FELEM x_felem, y_felem;
  if (!group->meth->point_get_affine_coordinates(group, &point->raw,
                                                 x == NULL ? NULL : &x_felem,
                                                 y == NULL ? NULL : &y_felem) ||
      (x != NULL && !ec_felem_to_bignum(group, x, &x_felem)) ||
      (y != NULL && !ec_felem_to_bignum(group, y, &y_felem))) {
    return 0;
  }
  return 1;
}

int EC_POINT_get_affine_coordinates(const EC_GROUP *group,
                                    const EC_POINT *point, BIGNUM *x, BIGNUM *y,
                                    BN_CTX *ctx) {
  return EC_POINT_get_affine_coordinates_GFp(group, point, x, y, ctx);
}

void ec_affine_to_jacobian(const EC_GROUP *group, EC_JACOBIAN *out,
                           const EC_AFFINE *p) {
  out->X = p->X;
  out->Y = p->Y;
  out->Z = *ec_felem_one(group);
}

int ec_jacobian_to_affine(const EC_GROUP *group, EC_AFFINE *out,
                          const EC_JACOBIAN *p) {
  return group->meth->point_get_affine_coordinates(group, p, &out->X, &out->Y);
}

int ec_jacobian_to_affine_batch(const EC_GROUP *group, EC_AFFINE *out,
                                const EC_JACOBIAN *in, size_t num) {
  if (group->meth->jacobian_to_affine_batch == NULL) {
    OPENSSL_PUT_ERROR(EC, ERR_R_SHOULD_NOT_HAVE_BEEN_CALLED);
    return 0;
  }
  return group->meth->jacobian_to_affine_batch(group, out, in, num);
}

int ec_point_set_affine_coordinates(const EC_GROUP *group, EC_AFFINE *out,
                                    const EC_FELEM *x, const EC_FELEM *y) {
  void (*const felem_mul)(const EC_GROUP *, EC_FELEM *r, const EC_FELEM *a,
                          const EC_FELEM *b) = group->meth->felem_mul;
  void (*const felem_sqr)(const EC_GROUP *, EC_FELEM *r, const EC_FELEM *a) =
      group->meth->felem_sqr;

  // Check if the point is on the curve.
  EC_FELEM lhs, rhs;
  felem_sqr(group, &lhs, y);                   // lhs = y^2
  felem_sqr(group, &rhs, x);                   // rhs = x^2
  ec_felem_add(group, &rhs, &rhs, &group->a);  // rhs = x^2 + a
  felem_mul(group, &rhs, &rhs, x);             // rhs = x^3 + ax
  ec_felem_add(group, &rhs, &rhs, &group->b);  // rhs = x^3 + ax + b
  if (!ec_felem_equal(group, &lhs, &rhs)) {
    OPENSSL_PUT_ERROR(EC, EC_R_POINT_IS_NOT_ON_CURVE);
    // In the event of an error, defend against the caller not checking the
    // return value by setting a known safe value. Note this may not be possible
    // if the caller is in the process of constructing an arbitrary group and
    // the generator is missing.
    if (group->has_order) {
      out->X = group->generator.raw.X;
      out->Y = group->generator.raw.Y;
    }
    return 0;
  }

  out->X = *x;
  out->Y = *y;
  return 1;
}

int EC_POINT_set_affine_coordinates_GFp(const EC_GROUP *group, EC_POINT *point,
                                        const BIGNUM *x, const BIGNUM *y,
                                        BN_CTX *ctx) {
  if (EC_GROUP_cmp(group, point->group, NULL) != 0) {
    OPENSSL_PUT_ERROR(EC, EC_R_INCOMPATIBLE_OBJECTS);
    return 0;
  }

  if (x == NULL || y == NULL) {
    OPENSSL_PUT_ERROR(EC, ERR_R_PASSED_NULL_PARAMETER);
    return 0;
  }

  EC_FELEM x_felem, y_felem;
  EC_AFFINE affine;
  if (!ec_bignum_to_felem(group, &x_felem, x) ||
      !ec_bignum_to_felem(group, &y_felem, y) ||
      !ec_point_set_affine_coordinates(group, &affine, &x_felem, &y_felem)) {
    // In the event of an error, defend against the caller not checking the
    // return value by setting a known safe value.
    ec_set_to_safe_point(group, &point->raw);
    return 0;
  }

  ec_affine_to_jacobian(group, &point->raw, &affine);
  return 1;
}

int EC_POINT_set_affine_coordinates(const EC_GROUP *group, EC_POINT *point,
                                    const BIGNUM *x, const BIGNUM *y,
                                    BN_CTX *ctx) {
  return EC_POINT_set_affine_coordinates_GFp(group, point, x, y, ctx);
}

int EC_POINT_add(const EC_GROUP *group, EC_POINT *r, const EC_POINT *a,
                 const EC_POINT *b, BN_CTX *ctx) {
  if (EC_GROUP_cmp(group, r->group, NULL) != 0 ||
      EC_GROUP_cmp(group, a->group, NULL) != 0 ||
      EC_GROUP_cmp(group, b->group, NULL) != 0) {
    OPENSSL_PUT_ERROR(EC, EC_R_INCOMPATIBLE_OBJECTS);
    return 0;
  }
  group->meth->add(group, &r->raw, &a->raw, &b->raw);
  return 1;
}

int EC_POINT_dbl(const EC_GROUP *group, EC_POINT *r, const EC_POINT *a,
                 BN_CTX *ctx) {
  if (EC_GROUP_cmp(group, r->group, NULL) != 0 ||
      EC_GROUP_cmp(group, a->group, NULL) != 0) {
    OPENSSL_PUT_ERROR(EC, EC_R_INCOMPATIBLE_OBJECTS);
    return 0;
  }
  group->meth->dbl(group, &r->raw, &a->raw);
  return 1;
}


int EC_POINT_invert(const EC_GROUP *group, EC_POINT *a, BN_CTX *ctx) {
  if (EC_GROUP_cmp(group, a->group, NULL) != 0) {
    OPENSSL_PUT_ERROR(EC, EC_R_INCOMPATIBLE_OBJECTS);
    return 0;
  }
  ec_GFp_simple_invert(group, &a->raw);
  return 1;
}

static int arbitrary_bignum_to_scalar(const EC_GROUP *group, EC_SCALAR *out,
                                      const BIGNUM *in, BN_CTX *ctx) {
  if (ec_bignum_to_scalar(group, out, in)) {
    return 1;
  }

  ERR_clear_error();

  // This is an unusual input, so we do not guarantee constant-time processing.
  bssl::BN_CTXScope scope(ctx);
  BIGNUM *tmp = BN_CTX_get(ctx);
  return tmp != nullptr && BN_nnmod(tmp, in, EC_GROUP_get0_order(group), ctx) &&
         ec_bignum_to_scalar(group, out, tmp);
}

int ec_point_mul_no_self_test(const EC_GROUP *group, EC_POINT *r,
                              const BIGNUM *g_scalar, const EC_POINT *p,
                              const BIGNUM *p_scalar, BN_CTX *ctx) {
  // Previously, this function set |r| to the point at infinity if there was
  // nothing to multiply. But, nobody should be calling this function with
  // nothing to multiply in the first place.
  if ((g_scalar == NULL && p_scalar == NULL) ||
      (p == NULL) != (p_scalar == NULL)) {
    OPENSSL_PUT_ERROR(EC, ERR_R_PASSED_NULL_PARAMETER);
    return 0;
  }

  if (EC_GROUP_cmp(group, r->group, NULL) != 0 ||
      (p != NULL && EC_GROUP_cmp(group, p->group, NULL) != 0)) {
    OPENSSL_PUT_ERROR(EC, EC_R_INCOMPATIBLE_OBJECTS);
    return 0;
  }

  bssl::UniquePtr<BN_CTX> new_ctx;
  if (ctx == NULL) {
    new_ctx.reset(BN_CTX_new());
    if (new_ctx == NULL) {
      return 0;
    }
    ctx = new_ctx.get();
  }

  // If both |g_scalar| and |p_scalar| are non-NULL,
  // |ec_point_mul_scalar_public| would share the doublings between the two
  // products, which would be more efficient. However, we conservatively assume
  // the caller needs a constant-time operation. (ECDSA verification does not
  // use this function.)
  //
  // Previously, the low-level constant-time multiplication function aligned
  // with this function's calling convention, but this was misleading. Curves
  // which combined the two multiplications did not avoid the doubling case
  // in the incomplete addition formula and were not constant-time.

  if (g_scalar != NULL) {
    EC_SCALAR scalar;
    if (!arbitrary_bignum_to_scalar(group, &scalar, g_scalar, ctx) ||
        !ec_point_mul_scalar_base(group, &r->raw, &scalar)) {
      return 0;
    }
  }

  if (p_scalar != NULL) {
    EC_SCALAR scalar;
    EC_JACOBIAN tmp;
    if (!arbitrary_bignum_to_scalar(group, &scalar, p_scalar, ctx) ||
        !ec_point_mul_scalar(group, &tmp, &p->raw, &scalar)) {
      return 0;
    }
    if (g_scalar == NULL) {
      OPENSSL_memcpy(&r->raw, &tmp, sizeof(EC_JACOBIAN));
    } else {
      group->meth->add(group, &r->raw, &r->raw, &tmp);
    }
  }

  return 1;
}

int EC_POINT_mul(const EC_GROUP *group, EC_POINT *r, const BIGNUM *g_scalar,
                 const EC_POINT *p, const BIGNUM *p_scalar, BN_CTX *ctx) {
  boringssl_ensure_ecc_self_test();

  return ec_point_mul_no_self_test(group, r, g_scalar, p, p_scalar, ctx);
}

int ec_point_mul_scalar_public(const EC_GROUP *group, EC_JACOBIAN *r,
                               const EC_SCALAR *g_scalar, const EC_JACOBIAN *p,
                               const EC_SCALAR *p_scalar) {
  if (g_scalar == NULL || p_scalar == NULL || p == NULL) {
    OPENSSL_PUT_ERROR(EC, ERR_R_PASSED_NULL_PARAMETER);
    return 0;
  }

  if (group->meth->mul_public == NULL) {
    return group->meth->mul_public_batch(group, r, g_scalar, p, p_scalar, 1);
  }

  group->meth->mul_public(group, r, g_scalar, p, p_scalar);
  return 1;
}

int ec_point_mul_scalar_public_batch(const EC_GROUP *group, EC_JACOBIAN *r,
                                     const EC_SCALAR *g_scalar,
                                     const EC_JACOBIAN *points,
                                     const EC_SCALAR *scalars, size_t num) {
  if (group->meth->mul_public_batch == NULL) {
    OPENSSL_PUT_ERROR(EC, ERR_R_SHOULD_NOT_HAVE_BEEN_CALLED);
    return 0;
  }

  return group->meth->mul_public_batch(group, r, g_scalar, points, scalars,
                                       num);
}

int ec_point_mul_scalar(const EC_GROUP *group, EC_JACOBIAN *r,
                        const EC_JACOBIAN *p, const EC_SCALAR *scalar) {
  if (p == NULL || scalar == NULL) {
    OPENSSL_PUT_ERROR(EC, ERR_R_PASSED_NULL_PARAMETER);
    return 0;
  }

  group->meth->mul(group, r, p, scalar);

  // Check the result is on the curve to defend against fault attacks or bugs.
  // This has negligible cost compared to the multiplication.
  if (!ec_GFp_simple_is_on_curve(group, r)) {
    OPENSSL_PUT_ERROR(EC, ERR_R_INTERNAL_ERROR);
    return 0;
  }

  return 1;
}

int ec_point_mul_scalar_base(const EC_GROUP *group, EC_JACOBIAN *r,
                             const EC_SCALAR *scalar) {
  if (scalar == NULL) {
    OPENSSL_PUT_ERROR(EC, ERR_R_PASSED_NULL_PARAMETER);
    return 0;
  }

  group->meth->mul_base(group, r, scalar);

  // Check the result is on the curve to defend against fault attacks or bugs.
  // This has negligible cost compared to the multiplication. This can only
  // happen on bug or CPU fault, so it okay to leak this. The alternative would
  // be to proceed with bad data.
  if (!constant_time_declassify_int(ec_GFp_simple_is_on_curve(group, r))) {
    OPENSSL_PUT_ERROR(EC, ERR_R_INTERNAL_ERROR);
    return 0;
  }

  return 1;
}

int ec_point_mul_scalar_batch(const EC_GROUP *group, EC_JACOBIAN *r,
                              const EC_JACOBIAN *p0, const EC_SCALAR *scalar0,
                              const EC_JACOBIAN *p1, const EC_SCALAR *scalar1,
                              const EC_JACOBIAN *p2, const EC_SCALAR *scalar2) {
  if (group->meth->mul_batch == NULL) {
    OPENSSL_PUT_ERROR(EC, ERR_R_SHOULD_NOT_HAVE_BEEN_CALLED);
    return 0;
  }

  group->meth->mul_batch(group, r, p0, scalar0, p1, scalar1, p2, scalar2);

  // Check the result is on the curve to defend against fault attacks or bugs.
  // This has negligible cost compared to the multiplication.
  if (!ec_GFp_simple_is_on_curve(group, r)) {
    OPENSSL_PUT_ERROR(EC, ERR_R_INTERNAL_ERROR);
    return 0;
  }

  return 1;
}

int ec_init_precomp(const EC_GROUP *group, EC_PRECOMP *out,
                    const EC_JACOBIAN *p) {
  if (group->meth->init_precomp == NULL) {
    OPENSSL_PUT_ERROR(EC, ERR_R_SHOULD_NOT_HAVE_BEEN_CALLED);
    return 0;
  }

  return group->meth->init_precomp(group, out, p);
}

int ec_point_mul_scalar_precomp(const EC_GROUP *group, EC_JACOBIAN *r,
                                const EC_PRECOMP *p0, const EC_SCALAR *scalar0,
                                const EC_PRECOMP *p1, const EC_SCALAR *scalar1,
                                const EC_PRECOMP *p2,
                                const EC_SCALAR *scalar2) {
  if (group->meth->mul_precomp == NULL) {
    OPENSSL_PUT_ERROR(EC, ERR_R_SHOULD_NOT_HAVE_BEEN_CALLED);
    return 0;
  }

  group->meth->mul_precomp(group, r, p0, scalar0, p1, scalar1, p2, scalar2);

  // Check the result is on the curve to defend against fault attacks or bugs.
  // This has negligible cost compared to the multiplication.
  if (!ec_GFp_simple_is_on_curve(group, r)) {
    OPENSSL_PUT_ERROR(EC, ERR_R_INTERNAL_ERROR);
    return 0;
  }

  return 1;
}

void ec_point_select(const EC_GROUP *group, EC_JACOBIAN *out, BN_ULONG mask,
                     const EC_JACOBIAN *a, const EC_JACOBIAN *b) {
  ec_felem_select(group, &out->X, mask, &a->X, &b->X);
  ec_felem_select(group, &out->Y, mask, &a->Y, &b->Y);
  ec_felem_select(group, &out->Z, mask, &a->Z, &b->Z);
}

void ec_affine_select(const EC_GROUP *group, EC_AFFINE *out, BN_ULONG mask,
                      const EC_AFFINE *a, const EC_AFFINE *b) {
  ec_felem_select(group, &out->X, mask, &a->X, &b->X);
  ec_felem_select(group, &out->Y, mask, &a->Y, &b->Y);
}

void ec_precomp_select(const EC_GROUP *group, EC_PRECOMP *out, BN_ULONG mask,
                       const EC_PRECOMP *a, const EC_PRECOMP *b) {
  static_assert(sizeof(out->comb) == sizeof(*out),
                "out->comb does not span the entire structure");
  for (size_t i = 0; i < OPENSSL_ARRAY_SIZE(out->comb); i++) {
    ec_affine_select(group, &out->comb[i], mask, &a->comb[i], &b->comb[i]);
  }
}

int ec_cmp_x_coordinate(const EC_GROUP *group, const EC_JACOBIAN *p,
                        const EC_SCALAR *r) {
  return group->meth->cmp_x_coordinate(group, p, r);
}

int ec_get_x_coordinate_as_scalar(const EC_GROUP *group, EC_SCALAR *out,
                                  const EC_JACOBIAN *p) {
  uint8_t bytes[EC_MAX_BYTES];
  size_t len;
  if (!ec_get_x_coordinate_as_bytes(group, bytes, &len, sizeof(bytes), p)) {
    return 0;
  }

  // The x-coordinate is bounded by p, but we need a scalar, bounded by the
  // order. These may not have the same size. However, we must have p < 2×order,
  // assuming p is not tiny (p >= 17).
  //
  // Thus |bytes| will fit in |order.width + 1| words, and we can reduce by
  // performing at most one subtraction.
  //
  // Proof: We only work with prime order curves, so the number of points on
  // the curve is the order. Thus Hasse's theorem gives:
  //
  //     |order - (p + 1)| <= 2×sqrt(p)
  //         p + 1 - order <= 2×sqrt(p)
  //     p + 1 - 2×sqrt(p) <= order
  //       p + 1 - 2×(p/4)  < order       (p/4 > sqrt(p) for p >= 17)
  //         p/2 < p/2 + 1  < order
  //                     p  < 2×order
  //
  // Additionally, one can manually check this property for built-in curves. It
  // is enforced for legacy custom curves in |EC_GROUP_set_generator|.
  const BIGNUM *order = EC_GROUP_get0_order(group);
  BN_ULONG words[EC_MAX_WORDS + 1] = {0};
  bn_big_endian_to_words(words, order->width + 1, bytes, len);
  bn_reduce_once(out->words, words, /*carry=*/words[order->width], order->d,
                 order->width);
  return 1;
}

int ec_get_x_coordinate_as_bytes(const EC_GROUP *group, uint8_t *out,
                                 size_t *out_len, size_t max_out,
                                 const EC_JACOBIAN *p) {
  size_t len = BN_num_bytes(&group->field.N);
  assert(len <= EC_MAX_BYTES);
  if (max_out < len) {
    OPENSSL_PUT_ERROR(EC, EC_R_BUFFER_TOO_SMALL);
    return 0;
  }

  EC_FELEM x;
  if (!group->meth->point_get_affine_coordinates(group, p, &x, NULL)) {
    return 0;
  }

  ec_felem_to_bytes(group, out, out_len, &x);
  *out_len = len;
  return 1;
}

void ec_set_to_safe_point(const EC_GROUP *group, EC_JACOBIAN *out) {
  if (group->has_order) {
    ec_GFp_simple_point_copy(out, &group->generator.raw);
  } else {
    // The generator can be missing if the caller is in the process of
    // constructing an arbitrary group. In this case, we give up and use the
    // point at infinity.
    ec_GFp_simple_point_set_to_infinity(group, out);
  }
}

void EC_GROUP_set_asn1_flag(EC_GROUP *group, int flag) {}

int EC_GROUP_get_asn1_flag(const EC_GROUP *group) {
  return OPENSSL_EC_NAMED_CURVE;
}

const EC_METHOD *EC_GROUP_method_of(const EC_GROUP *group) {
  // This function exists purely to give callers a way to call
  // |EC_METHOD_get_field_type|. cryptography.io crashes if |EC_GROUP_method_of|
  // returns NULL, so return some other garbage pointer.
  return (const EC_METHOD *)0x12340000;
}

int EC_METHOD_get_field_type(const EC_METHOD *meth) {
  return NID_X9_62_prime_field;
}

void EC_GROUP_set_point_conversion_form(EC_GROUP *group,
                                        point_conversion_form_t form) {
  if (form != POINT_CONVERSION_UNCOMPRESSED) {
    abort();
  }
}
