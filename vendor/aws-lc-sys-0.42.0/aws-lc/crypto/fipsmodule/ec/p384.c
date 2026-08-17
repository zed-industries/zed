// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC

#include <openssl/bn.h>
#include <openssl/ec.h>
#include <openssl/err.h>
#include <openssl/mem.h>

#include "../bn/internal.h"
#include "../cpucap/internal.h"
#include "../delocate.h"
#include "internal.h"
#include "ec_nistp.h"

#if !defined(OPENSSL_SMALL)

#if defined(EC_NISTP_USE_S2N_BIGNUM)
#  include "../../../third_party/s2n-bignum/s2n-bignum_aws-lc.h"
#else
#  if defined(EC_NISTP_USE_64BIT_LIMB)
#    include "../../../third_party/fiat/p384_64.h"
#  else
#    include "../../../third_party/fiat/p384_32.h"
#  endif
#endif

#if defined(EC_NISTP_USE_64BIT_LIMB)

#define P384_NLIMBS (6)
typedef uint64_t p384_limb_t;
typedef uint64_t p384_felem[P384_NLIMBS];
static const p384_felem p384_felem_one = {
    0xffffffff00000001, 0xffffffff, 0x1, 0x0, 0x0, 0x0};

#else  // 64BIT; else 32BIT

#define P384_NLIMBS (12)
typedef uint32_t p384_limb_t;
typedef uint32_t p384_felem[P384_NLIMBS];
static const p384_felem p384_felem_one = {
    0x1, 0xffffffff, 0xffffffff, 0x0, 0x1, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0};

#endif  // 64BIT

#if defined(EC_NISTP_USE_S2N_BIGNUM)

#define p384_felem_add(out, in0, in1)   bignum_add_p384(out, in0, in1)
#define p384_felem_sub(out, in0, in1)   bignum_sub_p384(out, in0, in1)
#define p384_felem_opp(out, in0)        bignum_neg_p384(out, in0)
#define p384_felem_to_bytes(out, in0)   bignum_tolebytes_6(out, in0)
#define p384_felem_from_bytes(out, in0) bignum_fromlebytes_6(out, in0)
#define p384_felem_to_mont(out, in0)    bignum_tomont_p384_selector(out, in0)
#define p384_felem_from_mont(out, in0)  bignum_deamont_p384_selector(out, in0)
#define p384_felem_mul(out, in0, in1)   bignum_montmul_p384_selector(out, in0, in1)
#define p384_felem_sqr(out, in0)        bignum_montsqr_p384_selector(out, in0)

static p384_limb_t p384_felem_nz(const p384_limb_t in1[P384_NLIMBS]) {
  return bignum_nonzero_6(in1);
}

#else // EC_NISTP_USE_S2N_BIGNUM

// Fiat-crypto implementation of field arithmetic
#define p384_felem_add(out, in0, in1)   fiat_p384_add(out, in0, in1)
#define p384_felem_sub(out, in0, in1)   fiat_p384_sub(out, in0, in1)
#define p384_felem_opp(out, in0)        fiat_p384_opp(out, in0)
#define p384_felem_mul(out, in0, in1)   fiat_p384_mul(out, in0, in1)
#define p384_felem_sqr(out, in0)        fiat_p384_square(out, in0)
#define p384_felem_to_mont(out, in0)    fiat_p384_to_montgomery(out, in0)
#define p384_felem_from_mont(out, in0)  fiat_p384_from_montgomery(out, in0)
#define p384_felem_to_bytes(out, in0)   fiat_p384_to_bytes(out, in0)
#define p384_felem_from_bytes(out, in0) fiat_p384_from_bytes(out, in0)

static p384_limb_t p384_felem_nz(const p384_limb_t in1[P384_NLIMBS]) {
  p384_limb_t ret;
  fiat_p384_nonzero(&ret, in1);
  return ret;
}

#endif // EC_NISTP_USE_S2N_BIGNUM

// The wrapper functions are needed for FIPS static build.
// Otherwise, initializing ec_nistp_meth with pointers to s2n-bignum
// functions directly generates :got: references that are also thought
// to be local_target by the delocator.
static inline void p384_felem_add_wrapper(ec_nistp_felem_limb *c,
                                          const ec_nistp_felem_limb *a,
                                          const ec_nistp_felem_limb *b) {
  p384_felem_add(c, a, b);
}

static inline void p384_felem_sub_wrapper(ec_nistp_felem_limb *c,
                                          const ec_nistp_felem_limb *a,
                                          const ec_nistp_felem_limb *b) {
  p384_felem_sub(c, a, b);
}

static inline void p384_felem_neg_wrapper(ec_nistp_felem_limb *c,
                                          const ec_nistp_felem_limb *a) {
  p384_felem_opp(c, a);
}

static void p384_from_generic(p384_felem out, const EC_FELEM *in) {
#ifdef OPENSSL_BIG_ENDIAN
  uint8_t tmp[P384_EC_FELEM_BYTES];
  bn_words_to_little_endian(tmp, P384_EC_FELEM_BYTES, in->words, P384_EC_FELEM_WORDS);
  p384_felem_from_bytes(out, tmp);
#else
  p384_felem_from_bytes(out, (const uint8_t *)in->words);
#endif
}

static void p384_to_generic(EC_FELEM *out, const p384_felem in) {
  // This works because 384 is a multiple of 64, so there are no excess bytes to
  // zero when rounding up to |BN_ULONG|s.
  OPENSSL_STATIC_ASSERT(
      384 / 8 == sizeof(BN_ULONG) * ((384 + BN_BITS2 - 1) / BN_BITS2),
      p384_felem_to_bytes_leaves_bytes_uninitialized);
#ifdef OPENSSL_BIG_ENDIAN
  uint8_t tmp[P384_EC_FELEM_BYTES];
  p384_felem_to_bytes(tmp, in);
  bn_little_endian_to_words(out->words, P384_EC_FELEM_WORDS, tmp, P384_EC_FELEM_BYTES);
#else
  p384_felem_to_bytes((uint8_t *)out->words, in);
#endif
}

static void p384_from_scalar(p384_felem out, const EC_SCALAR *in) {
#ifdef OPENSSL_BIG_ENDIAN
  uint8_t tmp[P384_EC_FELEM_BYTES];
  bn_words_to_little_endian(tmp, P384_EC_FELEM_BYTES, in->words, P384_EC_FELEM_WORDS);
  p384_felem_from_bytes(out, tmp);
#else
  p384_felem_from_bytes(out, (const uint8_t *)in->words);
#endif
}

// p384_inv_square calculates |out| = |in|^{-2}
//
// Based on Fermat's Little Theorem:
//   a^p = a (mod p)
//   a^{p-1} = 1 (mod p)
//   a^{p-3} = a^{-2} (mod p)
// p = 2^384 - 2^128 - 2^96 + 2^32 - 1
// Hexadecimal representation of p − 3:
// p-3 = ffffffff ffffffff ffffffff ffffffff ffffffff ffffffff ffffffff fffffffe
//       ffffffff 00000000 00000000 fffffffc
static void p384_inv_square(p384_felem out,
                            const p384_felem in) {
#if defined(EC_NISTP_USE_S2N_BIGNUM)
  ec_nistp_felem_limb in_sqr[P384_NLIMBS];
  p384_felem_sqr(in_sqr, in);
  bignum_montinv_p384(out, in_sqr);
#else
  // This implements the addition chain described in
  // https://briansmith.org/ecc-inversion-addition-chains-01#p384_field_inversion
  // The side comments show the value of the exponent:
  // squaring the element => doubling the exponent
  // multiplying by an element => adding to the exponent the power of that element
  p384_felem x2, x3, x6, x12, x15, x30, x60, x120;
  p384_felem_sqr(x2, in);   // 2^2 - 2^1
  p384_felem_mul(x2, x2, in);  // 2^2 - 2^0

  p384_felem_sqr(x3, x2);   // 2^3 - 2^1
  p384_felem_mul(x3, x3, in);  // 2^3 - 2^0

  p384_felem_sqr(x6, x3);
  for (int i = 1; i < 3; i++) {
    p384_felem_sqr(x6, x6);
  }                           // 2^6 - 2^3
  p384_felem_mul(x6, x6, x3);  // 2^6 - 2^0

  p384_felem_sqr(x12, x6);
  for (int i = 1; i < 6; i++) {
    p384_felem_sqr(x12, x12);
  }                             // 2^12 - 2^6
  p384_felem_mul(x12, x12, x6);  // 2^12 - 2^0

  p384_felem_sqr(x15, x12);
  for (int i = 1; i < 3; i++) {
    p384_felem_sqr(x15, x15);
  }                             // 2^15 - 2^3
  p384_felem_mul(x15, x15, x3);  // 2^15 - 2^0

  p384_felem_sqr(x30, x15);
  for (int i = 1; i < 15; i++) {
    p384_felem_sqr(x30, x30);
  }                              // 2^30 - 2^15
  p384_felem_mul(x30, x30, x15);  // 2^30 - 2^0

  p384_felem_sqr(x60, x30);
  for (int i = 1; i < 30; i++) {
    p384_felem_sqr(x60, x60);
  }                              // 2^60 - 2^30
  p384_felem_mul(x60, x60, x30);  // 2^60 - 2^0

  p384_felem_sqr(x120, x60);
  for (int i = 1; i < 60; i++) {
    p384_felem_sqr(x120, x120);
  }                                // 2^120 - 2^60
  p384_felem_mul(x120, x120, x60);  // 2^120 - 2^0

  p384_felem ret;
  p384_felem_sqr(ret, x120);
  for (int i = 1; i < 120; i++) {
    p384_felem_sqr(ret, ret);
  }                                // 2^240 - 2^120
  p384_felem_mul(ret, ret, x120);   // 2^240 - 2^0

  for (int i = 0; i < 15; i++) {
    p384_felem_sqr(ret, ret);
  }                                // 2^255 - 2^15
  p384_felem_mul(ret, ret, x15);    // 2^255 - 2^0

  // Why (1 + 30) in the loop?
  // This is as expressed in:
  //   https://briansmith.org/ecc-inversion-addition-chains-01#p384_field_inversion
  // My guess is to say that we're going to shift 31 bits, but this time we
  // won't add x31 to make all the new bits 1s, as was done in previous steps,
  // but we're going to add x30 so there will be 255 1s, then a 0, then 30 1s
  // to form this pattern:
  //   ffffffff ffffffff ffffffff ffffffff ffffffff ffffffff ffffffff fffffffe ffffffff
  // (the last 2 1s are appended in the following step).
  for (int i = 0; i < (1 + 30); i++) {
    p384_felem_sqr(ret, ret);
  }                                // 2^286 - 2^31
  p384_felem_mul(ret, ret, x30);    // 2^286 - 2^30 - 2^0

  p384_felem_sqr(ret, ret);
  p384_felem_sqr(ret, ret);      // 2^288 - 2^32 - 2^2
  p384_felem_mul(ret, ret, x2);     // 2^288 - 2^32 - 2^0

  // Why not 94 instead of (64 + 30) in the loop?
  // Similarly to the comment above, there is a shift of 94 bits
  // but what will be added is x30, which will cause 64 of those bits
  // to be 64 0s and 30 1s to complete the pattern above with:
  //   00000000 00000000 fffffffc
  // (the last 2 0s are appended by the last 2 shifts).
  for (int i = 0; i < (64 + 30); i++) {
    p384_felem_sqr(ret, ret);
  }                                // 2^382 - 2^126 - 2^94
  p384_felem_mul(ret, ret, x30);    // 2^382 - 2^126 - 2^94 + 2^30 - 2^0

  p384_felem_sqr(ret, ret);
  p384_felem_sqr(out, ret);      // 2^384 - 2^128 - 2^96 + 2^32 - 2^2 = p - 3
#endif
}

static void p384_point_double(p384_felem x_out,
                              p384_felem y_out,
                              p384_felem z_out,
                              const p384_felem x_in,
                              const p384_felem y_in,
                              const p384_felem z_in) {
#if defined(EC_NISTP_USE_S2N_BIGNUM)
  ec_nistp_felem_limb in[P384_NLIMBS * 3];
  ec_nistp_felem_limb out[P384_NLIMBS * 3];
  ec_nistp_coordinates_to_point(in, x_in, y_in, z_in, P384_NLIMBS);
  p384_montjdouble_selector(out, in);
  ec_nistp_point_to_coordinates(x_out, y_out, z_out, out, P384_NLIMBS);
#else
  ec_nistp_point_double(p384_methods(), x_out, y_out, z_out, x_in, y_in, z_in);
#endif
}

// p384_point_add calculates (x1, y1, z1) + (x2, y2, z2)
//
// The method is taken from:
//   http://hyperelliptic.org/EFD/g1p/auto-shortw-jacobian.html#addition-add-2007-bl
// adapted for mixed addition (z2 = 1, or z2 = 0 for the point at infinity).
//
// Coq transcription and correctness proof:
// <https://github.com/davidben/fiat-crypto/blob/c7b95f62b2a54b559522573310e9b487327d219a/src/Curves/Weierstrass/Jacobian.v#L467>
// <https://github.com/davidben/fiat-crypto/blob/c7b95f62b2a54b559522573310e9b487327d219a/src/Curves/Weierstrass/Jacobian.v#L544>
static void p384_point_add(p384_felem x3, p384_felem y3, p384_felem z3,
                           const p384_felem x1,
                           const p384_felem y1,
                           const p384_felem z1,
                           const int mixed,
                           const p384_felem x2,
                           const p384_felem y2,
                           const p384_felem z2) {
  ec_nistp_point_add(p384_methods(), x3, y3, z3, x1, y1, z1, mixed, x2, y2, z2);
}

#include "p384_table.h"

#if defined(EC_NISTP_USE_S2N_BIGNUM)
DEFINE_METHOD_FUNCTION(ec_nistp_meth, p384_methods) {
    out->felem_num_limbs = P384_NLIMBS;
    out->felem_num_bits = 384;
    out->felem_add = p384_felem_add_wrapper;
    out->felem_sub = p384_felem_sub_wrapper;
    out->felem_mul = bignum_montmul_p384_selector;
    out->felem_sqr = bignum_montsqr_p384_selector;
    out->felem_neg = p384_felem_neg_wrapper;
    out->felem_nz  = p384_felem_nz;
    out->felem_one = p384_felem_one;
    out->point_dbl = p384_point_double;
    out->point_add = p384_point_add;
    out->scalar_mul_base_table = (const ec_nistp_felem_limb*) p384_g_pre_comp;
}
#else
DEFINE_METHOD_FUNCTION(ec_nistp_meth, p384_methods) {
    out->felem_num_limbs = P384_NLIMBS;
    out->felem_num_bits = 384;
    out->felem_add = fiat_p384_add;
    out->felem_sub = fiat_p384_sub;
    out->felem_mul = fiat_p384_mul;
    out->felem_sqr = fiat_p384_square;
    out->felem_neg = fiat_p384_opp;
    out->felem_nz  = p384_felem_nz;
    out->felem_one = p384_felem_one;
    out->point_dbl = p384_point_double;
    out->point_add = p384_point_add;
    out->scalar_mul_base_table = (const ec_nistp_felem_limb*) p384_g_pre_comp;
}
#endif

// OPENSSL EC_METHOD FUNCTIONS

// Takes the Jacobian coordinates (X, Y, Z) of a point and returns:
//   (X', Y') = (X/Z^2, Y/Z^3).
static int ec_GFp_nistp384_point_get_affine_coordinates(
    const EC_GROUP *group, const EC_JACOBIAN *point,
    EC_FELEM *x_out, EC_FELEM *y_out) {

  if (constant_time_declassify_w(ec_GFp_simple_is_at_infinity(group, point))) {
    OPENSSL_PUT_ERROR(EC, EC_R_POINT_AT_INFINITY);
    return 0;
  }

  p384_felem z1, z2;
  p384_from_generic(z1, &point->Z);
  p384_inv_square(z2, z1);

  if (x_out != NULL) {
    p384_felem x;
    p384_from_generic(x, &point->X);
    p384_felem_mul(x, x, z2);
    p384_to_generic(x_out, x);
  }

  if (y_out != NULL) {
    p384_felem y;
    p384_from_generic(y, &point->Y);
    p384_felem_sqr(z2, z2);  // z^-4
    p384_felem_mul(y, y, z1);   // y * z
    p384_felem_mul(y, y, z2);   // y * z^-3
    p384_to_generic(y_out, y);
  }

  return 1;
}

static void ec_GFp_nistp384_add(const EC_GROUP *group, EC_JACOBIAN *r,
                                const EC_JACOBIAN *a, const EC_JACOBIAN *b) {
  p384_felem x1, y1, z1, x2, y2, z2;
  p384_from_generic(x1, &a->X);
  p384_from_generic(y1, &a->Y);
  p384_from_generic(z1, &a->Z);
  p384_from_generic(x2, &b->X);
  p384_from_generic(y2, &b->Y);
  p384_from_generic(z2, &b->Z);
  p384_point_add(x1, y1, z1, x1, y1, z1, 0 /* both Jacobian */, x2, y2, z2);
  p384_to_generic(&r->X, x1);
  p384_to_generic(&r->Y, y1);
  p384_to_generic(&r->Z, z1);
}

static void ec_GFp_nistp384_dbl(const EC_GROUP *group, EC_JACOBIAN *r,
                                const EC_JACOBIAN *a) {
  p384_felem x, y, z;
  p384_from_generic(x, &a->X);
  p384_from_generic(y, &a->Y);
  p384_from_generic(z, &a->Z);
  p384_point_double(x, y, z, x, y, z);
  p384_to_generic(&r->X, x);
  p384_to_generic(&r->Y, y);
  p384_to_generic(&r->Z, z);
}

// The calls to from/to_generic are needed for the case
// when BORINGSSL_HAS_UINT128 is undefined, i.e. p384_32.h fiat code is used;
// while OPENSSL_64_BIT is defined, i.e. BN_ULONG is uint64_t
static void ec_GFp_nistp384_mont_felem_to_bytes(
  const EC_GROUP *group, uint8_t *out, size_t *out_len, const EC_FELEM *in) {

  size_t len = BN_num_bytes(&group->field.N);
  EC_FELEM felem_tmp;
  p384_felem tmp;
  p384_from_generic(tmp, in);
  p384_felem_from_mont(tmp, tmp);
  p384_to_generic(&felem_tmp, tmp);

  bn_words_to_big_endian(out, len, felem_tmp.words, group->order.N.width);

  *out_len = len;
}

static int ec_GFp_nistp384_mont_felem_from_bytes(
  const EC_GROUP *group, EC_FELEM *out, const uint8_t *in, size_t len) {

  EC_FELEM felem_tmp;
  p384_felem tmp;
  // This function calls bn_cmp_words_consttime
  if (!ec_GFp_simple_felem_from_bytes(group, &felem_tmp, in, len)) {
    return 0;
  }
  p384_from_generic(tmp, &felem_tmp);
  p384_felem_to_mont(tmp, tmp);
  p384_to_generic(out, tmp);
  return 1;
}

static int ec_GFp_nistp384_cmp_x_coordinate(const EC_GROUP *group,
                                            const EC_JACOBIAN *p,
                                            const EC_SCALAR *r) {
  if (ec_GFp_simple_is_at_infinity(group, p)) {
    return 0;
  }

  // We wish to compare X/Z^2 with r. This is equivalent to comparing X with
  // r*Z^2. Note that X and Z are represented in Montgomery form, while r is
  // not.
  p384_felem Z2_mont;
  p384_from_generic(Z2_mont, &p->Z);
  p384_felem_mul(Z2_mont, Z2_mont, Z2_mont);

  p384_felem r_Z2;
  p384_from_scalar(r_Z2, r);  // r < order < p, so this is valid.
  p384_felem_mul(r_Z2, r_Z2, Z2_mont);

  p384_felem X;
  p384_from_generic(X, &p->X);
  p384_felem_from_mont(X, X);

  if (OPENSSL_memcmp(&r_Z2, &X, sizeof(r_Z2)) == 0) {
    return 1;
  }

  // During signing the x coefficient is reduced modulo the group order.
  // Therefore there is a small possibility, less than 2^189/2^384 = 1/2^195,
  // that group_order < p.x < p.
  // In that case, we need not only to compare against |r| but also to
  // compare against r+group_order.
  assert(group->field.N.width == group->order.N.width);
  EC_FELEM tmp;
  BN_ULONG carry =
      bn_add_words(tmp.words, r->words, group->order.N.d, group->field.N.width);
  if (carry == 0 &&
      bn_less_than_words(tmp.words, group->field.N.d, group->field.N.width)) {
    p384_from_generic(r_Z2, &tmp);
    p384_felem_mul(r_Z2, r_Z2, Z2_mont);
    if (OPENSSL_memcmp(&r_Z2, &X, sizeof(r_Z2)) == 0) {
      return 1;
    }
  }

  return 0;
}

// Multiplication of an arbitrary point by a scalar, r = [scalar]P.
static void ec_GFp_nistp384_point_mul(const EC_GROUP *group, EC_JACOBIAN *r,
                                      const EC_JACOBIAN *p,
                                      const EC_SCALAR *scalar) {

  p384_felem res[3] = {{0}, {0}, {0}}, tmp[3] = {{0}, {0}, {0}};

  p384_from_generic(tmp[0], &p->X);
  p384_from_generic(tmp[1], &p->Y);
  p384_from_generic(tmp[2], &p->Z);

#if defined(EC_NISTP_USE_S2N_BIGNUM)
  p384_montjscalarmul_selector((uint64_t*)res, scalar->words, (uint64_t*)tmp);
#else
  ec_nistp_scalar_mul(p384_methods(), res[0], res[1], res[2], tmp[0], tmp[1], tmp[2], scalar);
#endif

  p384_to_generic(&r->X, res[0]);
  p384_to_generic(&r->Y, res[1]);
  p384_to_generic(&r->Z, res[2]);
}

// Multiplication of the base point G of P-384 curve with the given scalar.
static void ec_GFp_nistp384_point_mul_base(const EC_GROUP *group,
                                           EC_JACOBIAN *r,
                                           const EC_SCALAR *scalar) {
  p384_felem res[3] = {{0}, {0}, {0}};

  ec_nistp_scalar_mul_base(p384_methods(), res[0], res[1], res[2], scalar);

  p384_to_generic(&r->X, res[0]);
  p384_to_generic(&r->Y, res[1]);
  p384_to_generic(&r->Z, res[2]);
}

// Computes [g_scalar]G + [p_scalar]P, where G is the base point of the P-384
// curve, and P is the given point |p|.
// Note: this function is NOT constant-time.
static void ec_GFp_nistp384_point_mul_public(const EC_GROUP *group,
                                             EC_JACOBIAN *r,
                                             const EC_SCALAR *g_scalar,
                                             const EC_JACOBIAN *p,
                                             const EC_SCALAR *p_scalar) {

  p384_felem res[3] = {{0}, {0}, {0}}, tmp[3] = {{0}, {0}, {0}};

  p384_from_generic(tmp[0], &p->X);
  p384_from_generic(tmp[1], &p->Y);
  p384_from_generic(tmp[2], &p->Z);

  ec_nistp_scalar_mul_public(p384_methods(), res[0], res[1], res[2], g_scalar, tmp[0], tmp[1], tmp[2], p_scalar);

  p384_to_generic(&r->X, res[0]);
  p384_to_generic(&r->Y, res[1]);
  p384_to_generic(&r->Z, res[2]);
}

DEFINE_METHOD_FUNCTION(EC_METHOD, EC_GFp_nistp384_method) {
  out->point_get_affine_coordinates =
      ec_GFp_nistp384_point_get_affine_coordinates;
  out->jacobian_to_affine_batch =
      ec_GFp_mont_jacobian_to_affine_batch;     // needed for TrustToken tests
  out->add = ec_GFp_nistp384_add;
  out->dbl = ec_GFp_nistp384_dbl;
  out->mul = ec_GFp_nistp384_point_mul;
  out->mul_base = ec_GFp_nistp384_point_mul_base;
  out->mul_public = ec_GFp_nistp384_point_mul_public;
  out->mul_batch = ec_GFp_mont_mul_batch;       // needed for TrustToken tests
  out->mul_public_batch = ec_GFp_mont_mul_public_batch;
  out->init_precomp = ec_GFp_mont_init_precomp; // needed for TrustToken tests
  out->mul_precomp = ec_GFp_mont_mul_precomp;   // needed for TrustToken tests
  out->felem_mul = ec_GFp_mont_felem_mul;
  out->felem_sqr = ec_GFp_mont_felem_sqr;
  out->felem_to_bytes = ec_GFp_nistp384_mont_felem_to_bytes;
  out->felem_from_bytes = ec_GFp_nistp384_mont_felem_from_bytes;
  out->felem_reduce = ec_GFp_mont_felem_reduce; // needed for ECTest.HashToCurve
  out->felem_exp = ec_GFp_mont_felem_exp;       // needed for ECTest.HashToCurve
  out->scalar_inv0_montgomery = ec_simple_scalar_inv0_montgomery;
  out->scalar_to_montgomery_inv_vartime =
      ec_simple_scalar_to_montgomery_inv_vartime;
  out->cmp_x_coordinate = ec_GFp_nistp384_cmp_x_coordinate;
}

// ----------------------------------------------------------------------------
//  Analysis of the doubling case occurrence in the Joye-Tunstall recoding:
//  p384_felem_mul_scalar_rwnaf()
// ----------------------------------------------------------------------------
//
// The JT scalar recoding is Algorithm 6: (Odd) Signed-Digit Recoding Algorithm in
// Joye, Tunstall, "Exponent Recoding and Regular Exponentiation Algorithms",
// AfricaCrypt 2009, available from
// https://citeseerx.ist.psu.edu/viewdoc/download?doi=10.1.1.477.1245&rep=rep1&type=pdf
//
// We write the algorithm using variables similar to those used in the code and
// in the proof detailed in util.c (t_i in the algorithm below is d in
// p384_felem_mul_scalar_rwnaf()):
//
// Input: k: odd scalar, where k = (b_{l-1}, ..., b_1, b_0) in binary form,
//        w: window width
// Output: k = (t_{h-1}, ..., t_1, t_0)
//         with t_i \in {\pm 1, \pm 3, ..., \pm (2^{w} - 1)},
//         h = ceil(t/w), i.e. t_i are positive and negative odd digits
//         which absolute value is less than (2^{w} - 1).
// i := 0
// j := 0
// while (k > 2^w):
//   window := (b_{j+w}, ..., b_j)   # (w+1)-bit window in k where
//                                   # the least significant bit is b_j
//   t_i := window - 2^w
//   k := k - t_i
//   k := k / 2^w          # k >> w
//   i += 1
//   j += w
// t_{h-1} := k
//
// Note that if b_{j+w} = 0, t_i will be negative;
// otherwise, if b_{j+w} = 1, t_i will be positive.
//
// The algorithm recodes the least (w+1) bits into a (odd) digit in the range
// [-(2^{w}-1), (2^{w}-1)] by subtracting 2^w from that digit and adding it back
// to the remaining bits of the scalar. This ensures that, after the w-bit right
// shift, the next least significant bit is 1, i.e. next digit is odd.
//
// In the following we will show that the non-trivial doubling case in
// single-point left-to-right windowed (or m-ary, m = 2^w) scalar multiplication
// may occur if and only if the (2^w)th bit of the group order is 1.  This only
// holds if the scalar is fully reduced and the group order is a prime that is
// much larger than 2^{w+1}.
//
// PROOF:
//
// Let n be the group order. Let l be the number of bits needed to represent n.
// Assume there exists some 0 <= k < n such that signed w-bit windowed
// multiplication hits the doubling case.
//
// Windowed multiplication consists of iterating over the digits t_i defined
// above by the algorithm from most to least significant. At iteration i
// (for i = ..., 3w, 2w, w, 0, starting from the most significant window), we:
//
//  1. Double the accumulator A, w times. Let A_i be the value of A at this
//     point, and it corresponds to a value [a_i]P.
//
//  2. Set A to T_i + A_i, where T_i is a precomputed multiple of P, [t_i]P

// From the algorithm steps we can see that the current digit
// t_i = (b_{w+i} ... b_i) - 2^w, b_i = 1     => -2^w < t_i < 2^w, t_i: odd
// which can also be written using C notation as
// t_i = [(k >> i) & ((1<<(w+1)) - 1)] - (1 << w)     -- (1)
//
// and the accumulator value
// a_i = b_{l-1} ... b_{i+w+1} 1
// when written as C notation
// a_i = (k >> (i+w+1)) << (w+1)) + (1 << w)          -- (2)
//
// Similarly to the recoding in util.c, a_i is bounded by
// 0 <= a_i < n + 2^w. Additionally, a_i can only be zero if b_(i+w-1) and up
// are all zero. (Note this implies a non-trivial P + (-P) is unreachable for
// all groups. That would imply the subsequent a_i is zero, which means all
// terms thus far were zero.)
//
// Let j be the index such that A_j = T_j ≠ ∞. We have a_j = t_j (mod n). We now
// determine the value of a_j - t_j, which must be divisible by n.
// Our bounds on a_j and t_j imply a_j - t_j is 0 or n.
// If it is 0, a_j = t_j. However, 2^w divides a_j and -2^w < t_i < 2^w, so this
// can only happen if a_j = t_j = 0, which is a trivial doubling.
// Therefore, a_j - t_j = n.
//
// Now we determine j. Suppose j > 0. w divides j, so j >= w. Then,
//
//   n = a_j - t_j = (k >> (j+w+1)) << (w+1)) + (1 << w) - t_j
//                 = k/2^j + 2^w - t_j
//                 < n/2^w + 2^w + 2^w-1
//
// n is much larger than 2^{w+1}, so this is impossible. Thus, j = 0: only the
// final addition may hit the doubling case.
//
// Finally, we consider bit patterns for n and k. Divide k into k_H + k_M + k_L
// such that k_H is the contribution from b_(l-1) .. b_{w+1},
// k_M is the contribution from b_w,
// and k_L is the contribution from b_(w-1) ... b_0.
// That is:
//
// - 2^{w+1} divides k_H
// - k_M is 0 or 2^w
// - 0 <= k_L < 2^w
//
// Divide n into n_H + n_M + n_L similarly.
// From (1) and (2), we have
//
// t_0 = (k_M + k_L) - 2^w
// a_0 = k_H + 2^w
//
// We try to find t_0 and a_0 such that
//
//               n = a_0 - t_0
// n_H + n_M + n_L = k_H + 2^w - (k_M + k_L - 2^w)
//                 = k_H + 2^{w+1} - (k_M + k_L)
//
// We know that k_H <= n_H.
//
// If k_H < n_H, then k_H <= n_H - 2^{w+1} (Note that 2^{w+1} divides both k_H
// and n_H). Then we would have
//
// n_H + n_M + n_L <= n_H - 2^{w+1} + 2^{w+1} - (k_M + k_L)
//       n_M + n_L <= - (k_M + k_L)
//
// Contradiction. Thus,
//
//          k_H = n_H                    -- (3)
// => n_M + n_L = 2^{w+1} - (k_M + k_L)  -- (4)
//
// We also have n > k; hence,
// n_M + n_L > k_M + k_L                 -- (5)
//
// For (3), (4) and (5) to hold,
// n_M = 2^w, k_M = 0.
//
// Otherwise, if n_M = 0 and k_M = 0
// n_L = 2^{w+1} - k_L
// n_L >= 2^{w+1} - (2^w - 1)
// n_L >= 2^w + 1
// Contradiction since n_L < 2^w.
//
// And if n_M = 0 and k_M = 2^w, (5) would not hold.
//
// Since n_M = 2^w, n_L >= 1, k_L >= 1, from (4) we have
//  k_M + k_L = 2^{w+1} - (n_M + n_L)
//           <= 2^{w+1} - 2^w - 1
//           <= 2^{w} - 1
// => k_M = 0
//
// Putting this together, from the group order of the curve, n, we can construct
// the scalar, k, that would incur a doubling in the last iteration as:
//
// if n_M = 2^w,
// k_H = n_H and
// k_M + k_L = 2^{w+1} - (n_M + n_L)
//
// COMMON CURVES:
//
// The group orders for common curves end in the following bit patterns:
//
//   P-521: ...00001001; w = 4, 5, 6, 7 are okay
//   P-384: ...01110011; w = 2, 3, 7    are okay
//   P-256: ...01010001; w = 2, 3, 5, 7 are okay
//
//
// CAN DOUBLING OCCUR IN RIGHT-TO-LEFT ALGORITHMS OR COMB ALGORITHMS?
//
// This question was answered empirically for P-384 group order n, w = 5,
// by asking:
// Is there a value d_76, such that
// - d_{76} * 2^{380} - a_{76} = n and
// - d_{76} * 2^{380} + a_{76} = k < n ?
//
// Setting
// d_76 = 0xf
// a_76 = (d_76 << 380) - n =
// -0xfffffffffffffffffffffffffffffffffffffffffffffffc7634d81f4372ddf581a0db248b0a77aecec196accc52973
// k = a_76 + (d_76 << 380) =
// 0xe00000000000000000000000000000000000000000000000389cb27e0bc8d220a7e5f24db74f58851313e695333ad68d
//
// n-k =
// 0x1fffffffffffffffffffffffffffffffffffffffffffffff8ec69b03e86e5bbeb0341b6491614ef5d9d832d5998a52e6
// => 0 < k < n
//
// -(1<<380)-a_76 = -0x389cb27e0bc8d220a7e5f24db74f58851313e695333ad68d
// => -2^380 < a_76 < 2^380
//
// This shows that such a k value exists.
//
// This resulted in modifying the comb algorithm used in
// ec_GFp_nistp384_point_mul_base() to proceed in a left-to-right fashion in
// order to add the least significant digit in the last iteration.
//
// We can probably construct values of k that would incur doubling for whenever
// any of the higher digits, t_{j-1}, (down to the middle digit, roughly) is
// added last. This is because the upper half of the group order of P-384 is all
// 1s, therefore we can find a value k < n, having a 0 at the (j*w)th bit which
// would become 1 in the recoding of t_j (being the least significant bit in
// t_j) and making t_{j-1} a negative digit. Hence, the difference between the
// accumulator value containing all digits and t_{j-1} * 2^{(j-1)*w} can be n.
//
// This was tested as follows in Python for j = 75, i.e. the second last digit:
// let that digit's value be the smallest possible value for w = 5, i.e. -31
//  d_75 = -31
// # assuming the accumulator contains the most significant digit d_76
//  a = n + (d_75 << 375); hex(a)
// '0xf07fffffffffffffffffffffffffffffffffffffffffffffc7634d81f4372ddf581a0db248b0a77aecec196accc52973L'
//  hex(n)
// '0xffffffffffffffffffffffffffffffffffffffffffffffffc7634d81f4372ddf581a0db248b0a77aecec196accc52973L'
//  k = a + (d_75 << 375); hex(k)
// '0xe0ffffffffffffffffffffffffffffffffffffffffffffffc7634d81f4372ddf581a0db248b0a77aecec196accc52973L'
//
// # Checks
//  hex(n-k)
// '0x1f0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000L'
//  hex(n - (a - (d_75 << 375)))
// '0x0L'
// => n > k
//    and a value k was found such that when adding d_75 last, the difference
//    between the accumulator a and (d_75 << 375) is n
//
//
// ----------------------------------------------------------------------------
//  Python code showing the doubling case occurrence in the Joye-Tunstall
//  recoding:
// ----------------------------------------------------------------------------
//
// from array import *
//
// # P-384 group order
// n = 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFC7634D81F4372DDF581A0DB248B0A77AECEC196ACCC52973
//
// # k value that causes a doubling case in left-to-right reconstruction
// k = 0xffffffffffffffffffffffffffffffffffffffffffffffffc7634d81f4372ddf581a0db248b0a77aecec196accc5294d
// # k value that causes a doubling case in right-to-left reconstruction
// k_r2l = 0xe00000000000000000000000000000000000000000000000389cb27e0bc8d220a7e5f24db74f58851313e695333ad68d
//
//
// def recode(k, w):
//     rec = array('i', [])
//     while k > (2 ** w):
//         window = k & ((2 ** (w + 1)) - 1)
//         d = window - (2 ** w)
//         k = k - d
//         k = (k >> w)
//         rec.append(d)
//     rec.append(k)
//     return rec
//
// # Rebuild k from the recoded scalar proceeding from left to right
// def rebuild_l2r(rec, w):
//     l = rec.buffer_info()[1]  # length of the recoded scalar array
//     # initialise accumulator
//     a = rec[l-1]
//     # for i from l-2 downto 0
//     for i in range(l-2,-1,-1):
//         a = (a << w)
//         if (a - rec[i]) == n:
//             print("L2R Doubling case: ")
//             print("    i =", i, " digit =", hex(rec[i]))
//             print("    a =", hex(a))
//         a += rec[i]
//     return a
//
// # Rebuild k from the recoded scalar proceeding from right to left
// def rebuild_r2l(rec, w):
//     l = rec.buffer_info()[1]  # length of the recoded scalar array
//     # initialise accumulator
//     a = rec[0]
//     # for i from 1 to l-1
//     for i in range(1,l):
//         shifted_d = rec[i] << (w*i)
//         if (shifted_d - a) == n:
//             print("R2L Doubling case: ")
//             print("    i =", i, " digit =", hex(rec[i]))
//             print("    a =", hex(a))
//         a += shifted_d
//     return a
//
// def test_recode():
//     w = 5
//
//     # Left-to-right recoding of k which causes a doubling case
//     assert k < n
//     print("k = ", hex(k))
//     # recode k
//     rec_k = recode(k,w)
//     # print(rec_k)
//     print("Digits of the recoded scalar:")
//     for a in rec_k:
//         print(hex(a), end=', ')
//     print()
//     # rebuild k
//     out_k = rebuild_l2r(rec_k, w)
//     if out_k != k:
//         print("ERROR: rebuilt value is different from recoded value")
//     print()
//
//     # Right-to-left recoding of k_r2l which causes a doubling case
//     assert k_r2l < n
//     print("k = ", hex(k_r2l))
//     # recode k_r2l
//     rec_k_r2l = recode(k_r2l,w)
//     # print(rec_k_r2l)
//     print("Digits of the recoded scalar:")
//     for a in rec_k_r2l:
//         print(hex(a), end=', ')
//     print()
//     # rebuild k_r2l
//     out_k_r2l = rebuild_r2l(rec_k_r2l, w)
//     if out_k_r2l != k_r2l:
//         print("ERROR: rebuilt R2L value is different from recoded value")
//     print()
//
// test_recode()
// '''
// Output:
// -------
// k =  0xffffffffffffffffffffffffffffffffffffffffffffffffc7634d81f4372ddf581a0db248b0a77aecec196accc5294d
// Digits of the recoded scalar:
// -0x13, -0x15, -0x15, -0x15, -0x13, 0x7, 0xb, 0xd, -0x7, 0x1, 0x1b, -0x7, 0xf, 0x1d, -0x3, -0xb, 0x11, -0x1b, -0xd, 0x5, -0x5, -0x19, 0x9, -0x1d, -0x7, 0x1b, 0x17, -0x5, 0x13, -0x5, -0xf, 0x1f, -0x1f, 0xd, -0xd, -0x19, 0x17, 0x3, 0x1f, 0x1f, 0x1f, 0x1f, 0x1f, 0x1f, 0x1f, 0x1f, 0x1f, 0x1f, 0x1f, 0x1f, 0x1f, 0x1f, 0x1f, 0x1f, 0x1f, 0x1f, 0x1f, 0x1f, 0x1f, 0x1f, 0x1f, 0x1f, 0x1f, 0x1f, 0x1f, 0x1f, 0x1f, 0x1f, 0x1f, 0x1f, 0x1f, 0x1f, 0x1f, 0x1f, 0x1f, 0x1f, 0xf,
// L2R Doubling case:
//     i = 0  digit = -0x13
//     a = 0xffffffffffffffffffffffffffffffffffffffffffffffffc7634d81f4372ddf581a0db248b0a77aecec196accc52960
//
// k =  0xe00000000000000000000000000000000000000000000000389cb27e0bc8d220a7e5f24db74f58851313e695333ad68d
// Digits of the recoded scalar:
// -0x13, 0x15, 0x15, 0x15, 0x13, -0x7, -0xb, -0xd, 0x7, -0x1, -0x1b, 0x7, -0xf, -0x1d, 0x3, 0xb, -0x11, 0x1b, 0xd, -0x5, 0x5, 0x19, -0x9, 0x1d, 0x7, -0x1b, -0x17, 0x5, -0x13, 0x5, 0xf, -0x1f, 0x1f, -0xd, 0xd, 0x19, -0x17, -0x3, -0x1f, -0x1f, -0x1f, -0x1f, -0x1f, -0x1f, -0x1f, -0x1f, -0x1f, -0x1f, -0x1f, -0x1f, -0x1f, -0x1f, -0x1f, -0x1f, -0x1f, -0x1f, -0x1f, -0x1f, -0x1f, -0x1f, -0x1f, -0x1f, -0x1f, -0x1f, -0x1f, -0x1f, -0x1f, -0x1f, -0x1f, -0x1f, -0x1f, -0x1f, -0x1f, -0x1f, -0x1f, -0x1f, 0xf,
// R2L Doubling case:
//     i = 76  digit = 0xf
//     a = -0xfffffffffffffffffffffffffffffffffffffffffffffffc7634d81f4372ddf581a0db248b0a77aecec196accc52973
// '''
//
#endif // !defined(OPENSSL_SMALL)
