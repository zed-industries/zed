// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC

// Implementation of P-521 that uses Fiat-crypto for the field arithmetic
// found in third_party/fiat, similarly to p256.c and p384.c

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
#    include "../../../third_party/fiat/p521_64.h"
#  else
#    include "../../../third_party/fiat/p521_32.h"
#  endif
#endif

#if defined(EC_NISTP_USE_S2N_BIGNUM)

#define P521_NLIMBS (9)

typedef uint64_t p521_limb_t;
typedef uint64_t p521_felem[P521_NLIMBS]; // field element

static const p521_limb_t p521_felem_one[P521_NLIMBS] = {
    0x0000000000000001, 0x0000000000000000,
    0x0000000000000000, 0x0000000000000000,
    0x0000000000000000, 0x0000000000000000,
    0x0000000000000000, 0x0000000000000000,
    0x0000000000000000};

// The field characteristic p.
static const p521_limb_t p521_felem_p[P521_NLIMBS] = {
    0xffffffffffffffff, 0xffffffffffffffff,
    0xffffffffffffffff, 0xffffffffffffffff,
    0xffffffffffffffff, 0xffffffffffffffff,
    0xffffffffffffffff, 0xffffffffffffffff,
    0x1ff};

// s2n-bignum implementation of field arithmetic
#define p521_felem_add(out, in0, in1)   bignum_add_p521(out, in0, in1)
#define p521_felem_sub(out, in0, in1)   bignum_sub_p521(out, in0, in1)
#define p521_felem_opp(out, in0)        bignum_neg_p521(out, in0)
#define p521_felem_to_bytes(out, in0)   bignum_tolebytes_p521(out, in0)
#define p521_felem_from_bytes(out, in0) bignum_fromlebytes_p521(out, in0)
#define p521_felem_mul(out, in0, in1)   bignum_mul_p521_selector(out, in0, in1)
#define p521_felem_sqr(out, in0)        bignum_sqr_p521_selector(out, in0)

#else // EC_NISTP_USE_S2N_BIGNUM

#if defined(EC_NISTP_USE_64BIT_LIMB)

// In the 64-bit case Fiat-crypto represents a field element by 9 58-bit digits.
#define P521_NLIMBS (9)

typedef uint64_t p521_felem[P521_NLIMBS]; // field element
typedef uint64_t p521_limb_t;

// One in Fiat-crypto's representation (58-bit digits).
static const p521_limb_t p521_felem_one[P521_NLIMBS] = {
    0x0000000000000001, 0x0000000000000000,
    0x0000000000000000, 0x0000000000000000,
    0x0000000000000000, 0x0000000000000000,
    0x0000000000000000, 0x0000000000000000,
    0x0000000000000000};

// The field characteristic p in Fiat-crypto's representation (58-bit digits).
static const p521_limb_t p521_felem_p[P521_NLIMBS] = {
    0x03ffffffffffffff, 0x03ffffffffffffff,
    0x03ffffffffffffff, 0x03ffffffffffffff,
    0x03ffffffffffffff, 0x03ffffffffffffff,
    0x03ffffffffffffff, 0x03ffffffffffffff,
    0x01ffffffffffffff};

#else  // 64BIT; else 32BIT

// In the 32-bit case Fiat-crypto represents a field element by 19 digits
// with the following bit sizes:
// [28, 27, 28, 27, 28, 27, 27, 28, 27, 28, 27, 28, 27, 27, 28, 27, 28, 27, 27].
#define P521_NLIMBS (19)

typedef uint32_t p521_felem[P521_NLIMBS]; // field element
typedef uint32_t p521_limb_t;

// One in Fiat-crypto's representation.
static const p521_limb_t p521_felem_one[P521_NLIMBS] = {
    0x0000001, 0x0000000, 0x0000000, 0x0000000,
    0x0000000, 0x0000000, 0x0000000, 0x0000000,
    0x0000000, 0x0000000, 0x0000000, 0x0000000,
    0x0000000, 0x0000000, 0x0000000, 0x0000000,
    0x0000000, 0x0000000, 0x0000000};

// The field characteristic p in Fiat-crypto's representation.
static const p521_limb_t p521_felem_p[P521_NLIMBS] = {
    0xfffffff, 0x7ffffff, 0xfffffff, 0x7ffffff,
    0xfffffff, 0x7ffffff, 0x7ffffff, 0xfffffff,
    0x7ffffff, 0xfffffff, 0x7ffffff, 0xfffffff,
    0x7ffffff, 0x7ffffff, 0xfffffff, 0x7ffffff,
    0xfffffff, 0x7ffffff, 0x7ffffff};
#endif  // 64BIT

// Fiat-crypto implementation of field arithmetic
#define p521_felem_add(out, in0, in1)   fiat_secp521r1_carry_add(out, in0, in1)
#define p521_felem_sub(out, in0, in1)   fiat_secp521r1_carry_sub(out, in0, in1)
#define p521_felem_opp(out, in0)        fiat_secp521r1_carry_opp(out, in0)
#define p521_felem_mul(out, in0, in1)   fiat_secp521r1_carry_mul(out, in0, in1)
#define p521_felem_sqr(out, in0)        fiat_secp521r1_carry_square(out, in0)
#define p521_felem_to_bytes(out, in0)   fiat_secp521r1_to_bytes(out, in0)
#define p521_felem_from_bytes(out, in0) fiat_secp521r1_from_bytes(out, in0)

#endif // EC_NISTP_USE_S2N_BIGNUM

// The wrapper functions are needed for FIPS static build.
// Otherwise, initializing ec_nistp_meth with pointers to s2n-bignum
// functions directly generates :got: references that are also thought
// to be local_target by the delocator.
static inline void p521_felem_add_wrapper(ec_nistp_felem_limb *c,
                                          const ec_nistp_felem_limb *a,
                                          const ec_nistp_felem_limb *b) {
  p521_felem_add(c, a, b);
}

static inline void p521_felem_sub_wrapper(ec_nistp_felem_limb *c,
                                          const ec_nistp_felem_limb *a,
                                          const ec_nistp_felem_limb *b) {
  p521_felem_sub(c, a, b);
}

static inline void p521_felem_neg_wrapper(ec_nistp_felem_limb *c,
                                          const ec_nistp_felem_limb *a) {
  p521_felem_opp(c, a);
}

static p521_limb_t p521_felem_nz(const p521_limb_t in1[P521_NLIMBS]) {
  p521_limb_t is_not_zero = 0;
  for (int i = 0; i < P521_NLIMBS; i++) {
    is_not_zero |= in1[i];
  }

#if defined(EC_NISTP_USE_S2N_BIGNUM)
  return is_not_zero;
#else
  // Fiat-crypto functions may return p (the field characteristic)
  // instead of 0 in some cases, so we also check for that.
  p521_limb_t is_not_p = 0;
  for (int i = 0; i < P521_NLIMBS; i++) {
    is_not_p |= (in1[i] ^ p521_felem_p[i]);
  }

  return ~(constant_time_is_zero_w(is_not_p) |
           constant_time_is_zero_w(is_not_zero));
#endif
}

// NOTE: the input and output are in little-endian representation.
static void p521_from_generic(p521_felem out, const EC_FELEM *in) {
#ifdef OPENSSL_BIG_ENDIAN
  uint8_t tmp[P521_EC_FELEM_BYTES];
  bn_words_to_little_endian(tmp, P521_EC_FELEM_BYTES, in->words, P521_EC_FELEM_WORDS);
  p521_felem_from_bytes(out, tmp);
#else
  p521_felem_from_bytes(out, (const uint8_t *)in->words);
#endif
}

// NOTE: the input and output are in little-endian representation.
static void p521_to_generic(EC_FELEM *out, const p521_felem in) {
  // |p521_felem_to_bytes| function will write the result to the first 66 bytes
  // of |out| which is exactly how many bytes are needed to represent a 521-bit
  // element.
  // The number of BN_ULONGs to represent a 521-bit value is 9 and 17, when
  // BN_ULONG is 64-bit and 32-bit, respectively. Nine 64-bit BN_ULONGs
  // translate to 72 bytes, which means that we have to make sure that the
  // extra 6 bytes are zeroed out. To avoid confusion over 32 vs. 64 bit
  // systems and Fiat's vs. ours representation we zero out the whole element.
#ifdef OPENSSL_BIG_ENDIAN
  uint8_t tmp[P521_EC_FELEM_BYTES];
  p521_felem_to_bytes(tmp, in);
  bn_little_endian_to_words(out->words, P521_EC_FELEM_WORDS, tmp, P521_EC_FELEM_BYTES);
#else
  OPENSSL_memset((uint8_t*)out->words, 0, sizeof(out->words));
  // Convert the element to bytes.
  p521_felem_to_bytes((uint8_t *)out->words, in);
#endif
}

// Finite field inversion using Fermat Little Theorem.
// The code is autogenerated by the ECCKiila project:
//   https://arxiv.org/abs/2007.11481
static void p521_felem_inv(p521_felem output, const p521_felem t1) {
#if defined(EC_NISTP_USE_S2N_BIGNUM)
    bignum_inv_p521(output, t1);
#else
    /* temporary variables */
    p521_felem acc, t2, t4, t8, t16, t32, t64;
    p521_felem t128, t256, t512, t516, t518, t519;

    p521_felem_sqr(acc, t1);
    p521_felem_mul(t2, acc, t1);
    p521_felem_sqr(acc, t2);
    p521_felem_sqr(acc, acc);
    p521_felem_mul(t4, acc, t2);
    p521_felem_sqr(acc, t4);
    for (int i = 0; i < 3; i++) {
      p521_felem_sqr(acc, acc);
    }
    p521_felem_mul(t8, acc, t4);
    p521_felem_sqr(acc, t8);
    for (int i = 0; i < 7; i++) {
      p521_felem_sqr(acc, acc);
    }
    p521_felem_mul(t16, acc, t8);
    p521_felem_sqr(acc, t16);
    for (int i = 0; i < 15; i++) {
      p521_felem_sqr(acc, acc);
    }
    p521_felem_mul(t32, acc, t16);
    p521_felem_sqr(acc, t32);
    for (int i = 0; i < 31; i++) {
      p521_felem_sqr(acc, acc);
    }
    p521_felem_mul(t64, acc, t32);
    p521_felem_sqr(acc, t64);
    for (int i = 0; i < 63; i++) {
      p521_felem_sqr(acc, acc);
    }
    p521_felem_mul(t128, acc, t64);
    p521_felem_sqr(acc, t128);
    for (int i = 0; i < 127; i++) {
      p521_felem_sqr(acc, acc);
    }
    p521_felem_mul(t256, acc, t128);
    p521_felem_sqr(acc, t256);
    for (int i = 0; i < 255; i++) {
      p521_felem_sqr(acc, acc);
    }
    p521_felem_mul(t512, acc, t256);
    p521_felem_sqr(acc, t512);
    for (int i = 0; i < 3; i++) {
      p521_felem_sqr(acc, acc);
    }
    p521_felem_mul(t516, acc, t4);
    p521_felem_sqr(acc, t516);
    p521_felem_sqr(acc, acc);
    p521_felem_mul(t518, acc, t2);
    p521_felem_sqr(acc, t518);
    p521_felem_mul(t519, acc, t1);
    p521_felem_sqr(acc, t519);
    p521_felem_sqr(acc, acc);
    p521_felem_mul(output, acc, t1);
#endif
}

static void p521_point_double(p521_felem x_out,
                              p521_felem y_out,
                              p521_felem z_out,
                              const p521_felem x_in,
                              const p521_felem y_in,
                              const p521_felem z_in) {
#if defined(EC_NISTP_USE_S2N_BIGNUM)
  ec_nistp_felem_limb in[P521_NLIMBS * 3];
  ec_nistp_felem_limb out[P521_NLIMBS * 3];
  ec_nistp_coordinates_to_point(in, x_in, y_in, z_in, P521_NLIMBS);
  p521_jdouble_selector(out, in);
  ec_nistp_point_to_coordinates(x_out, y_out, z_out, out, P521_NLIMBS);
#else
  ec_nistp_point_double(p521_methods(), x_out, y_out, z_out, x_in, y_in, z_in);
#endif
}

// p521_point_add calculates (x1, y1, z1) + (x2, y2, z2)
//
// The method is taken from:
//   http://hyperelliptic.org/EFD/g1p/auto-shortw-jacobian.html#addition-add-2007-bl
// adapted for mixed addition (z2 = 1, or z2 = 0 for the point at infinity).
//
// Coq transcription and correctness proof:
// <https://github.com/davidben/fiat-crypto/blob/c7b95f62b2a54b559522573310e9b487327d219a/src/Curves/Weierstrass/Jacobian.v#L467>
// <https://github.com/davidben/fiat-crypto/blob/c7b95f62b2a54b559522573310e9b487327d219a/src/Curves/Weierstrass/Jacobian.v#L544>
static void p521_point_add(p521_felem x3, p521_felem y3, p521_felem z3,
                           const p521_felem x1,
                           const p521_felem y1,
                           const p521_felem z1,
                           const int mixed,
                           const p521_felem x2,
                           const p521_felem y2,
                           const p521_felem z2) {
  ec_nistp_point_add(p521_methods(), x3, y3, z3, x1, y1, z1, mixed, x2, y2, z2);
}

#include "p521_table.h"

#if defined(EC_NISTP_USE_S2N_BIGNUM)
DEFINE_METHOD_FUNCTION(ec_nistp_meth, p521_methods) {
    out->felem_num_limbs = P521_NLIMBS;
    out->felem_num_bits = 521;
    out->felem_add = p521_felem_add_wrapper;
    out->felem_sub = p521_felem_sub_wrapper;
    out->felem_mul = bignum_mul_p521_selector;
    out->felem_sqr = bignum_sqr_p521_selector;
    out->felem_neg = p521_felem_neg_wrapper;
    out->felem_nz  = p521_felem_nz;
    out->felem_one = p521_felem_one;
    out->point_dbl = p521_point_double;
    out->point_add = p521_point_add;
    out->scalar_mul_base_table = (const ec_nistp_felem_limb*) p521_g_pre_comp;
}
#else
DEFINE_METHOD_FUNCTION(ec_nistp_meth, p521_methods) {
    out->felem_num_limbs = P521_NLIMBS;
    out->felem_num_bits = 521;
    out->felem_add = fiat_secp521r1_carry_add;
    out->felem_sub = fiat_secp521r1_carry_sub;
    out->felem_mul = fiat_secp521r1_carry_mul;
    out->felem_sqr = fiat_secp521r1_carry_square;
    out->felem_neg = fiat_secp521r1_carry_opp;
    out->felem_nz  = p521_felem_nz;
    out->felem_one = p521_felem_one;
    out->point_dbl = p521_point_double;
    out->point_add = p521_point_add;
    out->scalar_mul_base_table = (const ec_nistp_felem_limb*) p521_g_pre_comp;
}
#endif

// OPENSSL EC_METHOD FUNCTIONS

// Takes the Jacobian coordinates (X, Y, Z) of a point and returns:
//   (X', Y') = (X/Z^2, Y/Z^3).
static int ec_GFp_nistp521_point_get_affine_coordinates(
    const EC_GROUP *group, const EC_JACOBIAN *point,
    EC_FELEM *x_out, EC_FELEM *y_out) {

  if (constant_time_declassify_w(ec_GFp_simple_is_at_infinity(group, point))) {
    OPENSSL_PUT_ERROR(EC, EC_R_POINT_AT_INFINITY);
    return 0;
  }

  p521_felem z1, z2;
  p521_from_generic(z1, &point->Z);
  p521_felem_inv(z2, z1);
  p521_felem_sqr(z2, z2);

  if (x_out != NULL) {
    p521_felem x;
    p521_from_generic(x, &point->X);
    p521_felem_mul(x, x, z2);
    p521_to_generic(x_out, x);
  }

  if (y_out != NULL) {
    p521_felem y;
    p521_from_generic(y, &point->Y);
    p521_felem_sqr(z2, z2);  // z^-4
    p521_felem_mul(y, y, z1);   // y * z
    p521_felem_mul(y, y, z2);   // y * z^-3
    p521_to_generic(y_out, y);
  }

  return 1;
}

static void ec_GFp_nistp521_add(const EC_GROUP *group, EC_JACOBIAN *r,
                                const EC_JACOBIAN *a, const EC_JACOBIAN *b) {
  p521_felem x1, y1, z1, x2, y2, z2;
  p521_from_generic(x1, &a->X);
  p521_from_generic(y1, &a->Y);
  p521_from_generic(z1, &a->Z);
  p521_from_generic(x2, &b->X);
  p521_from_generic(y2, &b->Y);
  p521_from_generic(z2, &b->Z);
  p521_point_add(x1, y1, z1, x1, y1, z1, 0 /* both Jacobian */, x2, y2, z2);
  p521_to_generic(&r->X, x1);
  p521_to_generic(&r->Y, y1);
  p521_to_generic(&r->Z, z1);
}

static void ec_GFp_nistp521_dbl(const EC_GROUP *group, EC_JACOBIAN *r,
                                const EC_JACOBIAN *a) {
  p521_felem x, y, z;
  p521_from_generic(x, &a->X);
  p521_from_generic(y, &a->Y);
  p521_from_generic(z, &a->Z);
  p521_point_double(x, y, z, x, y, z);
  p521_to_generic(&r->X, x);
  p521_to_generic(&r->Y, y);
  p521_to_generic(&r->Z, z);
}

// Multiplication of an arbitrary point by a scalar, r = [scalar]P.
static void ec_GFp_nistp521_point_mul(const EC_GROUP *group, EC_JACOBIAN *r,
                                      const EC_JACOBIAN *p,
                                      const EC_SCALAR *scalar) {

  p521_felem res[3] = {{0}, {0}, {0}}, tmp[3] = {{0}, {0}, {0}};

  p521_from_generic(tmp[0], &p->X);
  p521_from_generic(tmp[1], &p->Y);
  p521_from_generic(tmp[2], &p->Z);

#if defined(EC_NISTP_USE_S2N_BIGNUM)
  p521_jscalarmul_selector((uint64_t*)res, scalar->words, (uint64_t*)tmp);
#else
  ec_nistp_scalar_mul(p521_methods(), res[0], res[1], res[2], tmp[0], tmp[1], tmp[2], scalar);
#endif

  p521_to_generic(&r->X, res[0]);
  p521_to_generic(&r->Y, res[1]);
  p521_to_generic(&r->Z, res[2]);
}

// Multiplication of the base point G of P-521 curve with the given scalar.
static void ec_GFp_nistp521_point_mul_base(const EC_GROUP *group,
                                           EC_JACOBIAN *r,
                                           const EC_SCALAR *scalar) {
  p521_felem res[3] = {{0}, {0}, {0}};

  ec_nistp_scalar_mul_base(p521_methods(), res[0], res[1], res[2], scalar);

  p521_to_generic(&r->X, res[0]);
  p521_to_generic(&r->Y, res[1]);
  p521_to_generic(&r->Z, res[2]);
}

// Computes [g_scalar]G + [p_scalar]P, where G is the base point of the P-521
// curve, and P is the given point |p|.
// Note: this function is NOT constant-time.
static void ec_GFp_nistp521_point_mul_public(const EC_GROUP *group,
                                             EC_JACOBIAN *r,
                                             const EC_SCALAR *g_scalar,
                                             const EC_JACOBIAN *p,
                                             const EC_SCALAR *p_scalar) {

  p521_felem res[3] = {{0}, {0}, {0}}, tmp[3] = {{0}, {0}, {0}};

  p521_from_generic(tmp[0], &p->X);
  p521_from_generic(tmp[1], &p->Y);
  p521_from_generic(tmp[2], &p->Z);

  ec_nistp_scalar_mul_public(p521_methods(), res[0], res[1], res[2], g_scalar, tmp[0], tmp[1], tmp[2], p_scalar);

  // Copy the result to the output.
  p521_to_generic(&r->X, res[0]);
  p521_to_generic(&r->Y, res[1]);
  p521_to_generic(&r->Z, res[2]);
}

static void ec_GFp_nistp521_felem_mul(const EC_GROUP *group, EC_FELEM *r,
                                      const EC_FELEM *a, const EC_FELEM *b) {
  p521_felem felem1, felem2, felem3;
  p521_from_generic(felem1, a);
  p521_from_generic(felem2, b);
  p521_felem_mul(felem3, felem1, felem2);
  p521_to_generic(r, felem3);
}

static void ec_GFp_nistp521_felem_sqr(const EC_GROUP *group, EC_FELEM *r,
                                      const EC_FELEM *a) {
  p521_felem felem1, felem2;
  p521_from_generic(felem1, a);
  p521_felem_sqr(felem2, felem1);
  p521_to_generic(r, felem2);
}

DEFINE_METHOD_FUNCTION(EC_METHOD, EC_GFp_nistp521_method) {
  out->point_get_affine_coordinates =
      ec_GFp_nistp521_point_get_affine_coordinates;
  out->add = ec_GFp_nistp521_add;
  out->dbl = ec_GFp_nistp521_dbl;
  out->mul = ec_GFp_nistp521_point_mul;
  out->mul_base = ec_GFp_nistp521_point_mul_base;
  out->mul_public = ec_GFp_nistp521_point_mul_public;
  out->felem_mul = ec_GFp_nistp521_felem_mul;
  out->felem_sqr = ec_GFp_nistp521_felem_sqr;
  out->felem_to_bytes = ec_GFp_simple_felem_to_bytes;
  out->felem_from_bytes = ec_GFp_simple_felem_from_bytes;
  out->scalar_inv0_montgomery = ec_simple_scalar_inv0_montgomery;
  out->scalar_to_montgomery_inv_vartime =
      ec_simple_scalar_to_montgomery_inv_vartime;
  out->cmp_x_coordinate = ec_GFp_simple_cmp_x_coordinate;
}

// ----------------------------------------------------------------------------
//  Analysis of the doubling case occurrence in the Joye-Tunstall recoding:
//  see the analysis at the bottom of the |p384.c| file.
#endif // !defined(OPENSSL_SMALL)
