// Copyright 2014-2016 The OpenSSL Project Authors. All Rights Reserved.
// Copyright (c) 2014, Intel Corporation. All Rights Reserved.
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
//
// Originally written by Shay Gueron (1, 2), and Vlad Krasnov (1)
// (1) Intel Corporation, Israel Development Center, Haifa, Israel
// (2) University of Haifa, Israel
//
// Reference:
// S.Gueron and V.Krasnov, "Fast Prime Field Elliptic Curve Cryptography with
//                          256 Bit Primes"

#include <openssl/ec.h>

#include <assert.h>
#include <stdint.h>
#include <string.h>

#include <openssl/bn.h>
#include <openssl/crypto.h>
#include <openssl/err.h>

#include "../../internal.h"
#include "../bn/internal.h"
#include "../delocate.h"
#include "internal.h"
#include "p256-nistz.h"

#if !defined(OPENSSL_NO_ASM) &&                              \
    (defined(OPENSSL_X86_64) || defined(OPENSSL_AARCH64)) && \
    !defined(OPENSSL_SMALL)

typedef P256_POINT_AFFINE PRECOMP256_ROW[64];

// One converted into the Montgomery domain
static const BN_ULONG ONE_MONT[P256_LIMBS] = {
    TOBN(0x00000000, 0x00000001),
    TOBN(0xffffffff, 0x00000000),
    TOBN(0xffffffff, 0xffffffff),
    TOBN(0x00000000, 0xfffffffe),
};

// Precomputed tables for the default generator
#include "p256-nistz-table.h"

// Recode window to a signed digit, see |ec_GFp_nistp_recode_scalar_bits| in
// util.c for details
static crypto_word_t booth_recode_w5(crypto_word_t in) {
  crypto_word_t s, d;

  s = ~((in >> 5) - 1);
  d = (1 << 6) - in - 1;
  d = (d & s) | (in & ~s);
  d = (d >> 1) + (d & 1);

  return (d << 1) + (s & 1);
}

static crypto_word_t booth_recode_w7(crypto_word_t in) {
  crypto_word_t s, d;

  s = ~((in >> 7) - 1);
  d = (1 << 8) - in - 1;
  d = (d & s) | (in & ~s);
  d = (d >> 1) + (d & 1);

  return (d << 1) + (s & 1);
}

// copy_conditional copies |src| to |dst| if |move| is one and leaves it as-is
// if |move| is zero.
//
// WARNING: this breaks the usual convention of constant-time functions
// returning masks.
static void copy_conditional(BN_ULONG dst[P256_LIMBS],
                             const BN_ULONG src[P256_LIMBS], BN_ULONG move) {
  BN_ULONG mask1 = ((BN_ULONG)0) - move;
  BN_ULONG mask2 = ~mask1;

  dst[0] = (src[0] & mask1) ^ (dst[0] & mask2);
  dst[1] = (src[1] & mask1) ^ (dst[1] & mask2);
  dst[2] = (src[2] & mask1) ^ (dst[2] & mask2);
  dst[3] = (src[3] & mask1) ^ (dst[3] & mask2);
  if (P256_LIMBS == 8) {
    dst[4] = (src[4] & mask1) ^ (dst[4] & mask2);
    dst[5] = (src[5] & mask1) ^ (dst[5] & mask2);
    dst[6] = (src[6] & mask1) ^ (dst[6] & mask2);
    dst[7] = (src[7] & mask1) ^ (dst[7] & mask2);
  }
}

// is_not_zero returns one iff in != 0 and zero otherwise.
//
// WARNING: this breaks the usual convention of constant-time functions
// returning masks.
//
// (define-fun is_not_zero ((in (_ BitVec 64))) (_ BitVec 64)
//   (bvlshr (bvor in (bvsub #x0000000000000000 in)) #x000000000000003f)
// )
//
// (declare-fun x () (_ BitVec 64))
//
// (assert (and (= x #x0000000000000000) (= (is_not_zero x)
// #x0000000000000001))) (check-sat)
//
// (assert (and (not (= x #x0000000000000000)) (= (is_not_zero x)
// #x0000000000000000))) (check-sat)
//
static BN_ULONG is_not_zero(BN_ULONG in) {
  in |= (0 - in);
  in >>= BN_BITS2 - 1;
  return in;
}

#if defined(OPENSSL_X86_64)
// Dispatch between CPU variations. The "_adx" suffixed functions use MULX in
// addition to ADCX/ADOX. MULX is part of BMI2, not ADX, so we must check both
// capabilities.
static void ecp_nistz256_mul_mont(BN_ULONG res[P256_LIMBS],
                                  const BN_ULONG a[P256_LIMBS],
                                  const BN_ULONG b[P256_LIMBS]) {
  if (CRYPTO_is_BMI2_capable() && CRYPTO_is_ADX_capable()) {
    ecp_nistz256_mul_mont_adx(res, a, b);
  } else {
    ecp_nistz256_mul_mont_nohw(res, a, b);
  }
}

static void ecp_nistz256_sqr_mont(BN_ULONG res[P256_LIMBS],
                                  const BN_ULONG a[P256_LIMBS]) {
  if (CRYPTO_is_BMI2_capable() && CRYPTO_is_ADX_capable()) {
    ecp_nistz256_sqr_mont_adx(res, a);
  } else {
    ecp_nistz256_sqr_mont_nohw(res, a);
  }
}

static void ecp_nistz256_ord_mul_mont(BN_ULONG res[P256_LIMBS],
                                      const BN_ULONG a[P256_LIMBS],
                                      const BN_ULONG b[P256_LIMBS]) {
  if (CRYPTO_is_BMI2_capable() && CRYPTO_is_ADX_capable()) {
    ecp_nistz256_ord_mul_mont_adx(res, a, b);
  } else {
    ecp_nistz256_ord_mul_mont_nohw(res, a, b);
  }
}

static void ecp_nistz256_ord_sqr_mont(BN_ULONG res[P256_LIMBS],
                                      const BN_ULONG a[P256_LIMBS],
                                      BN_ULONG rep) {
  if (CRYPTO_is_BMI2_capable() && CRYPTO_is_ADX_capable()) {
    ecp_nistz256_ord_sqr_mont_adx(res, a, rep);
  } else {
    ecp_nistz256_ord_sqr_mont_nohw(res, a, rep);
  }
}

static void ecp_nistz256_select_w5(P256_POINT *val, const P256_POINT in_t[16],
                                   int index) {
  if (CRYPTO_is_AVX2_capable()) {
    ecp_nistz256_select_w5_avx2(val, in_t, index);
  } else {
    ecp_nistz256_select_w5_nohw(val, in_t, index);
  }
}

static void ecp_nistz256_select_w7(P256_POINT_AFFINE *val,
                                   const P256_POINT_AFFINE in_t[64],
                                   int index) {
  if (CRYPTO_is_AVX2_capable()) {
    ecp_nistz256_select_w7_avx2(val, in_t, index);
  } else {
    ecp_nistz256_select_w7_nohw(val, in_t, index);
  }
}

static void ecp_nistz256_point_double(P256_POINT *r, const P256_POINT *a) {
  if (CRYPTO_is_BMI2_capable() && CRYPTO_is_ADX_capable()) {
    ecp_nistz256_point_double_adx(r, a);
  } else {
    ecp_nistz256_point_double_nohw(r, a);
  }
}

static void ecp_nistz256_point_add(P256_POINT *r, const P256_POINT *a,
                                   const P256_POINT *b) {
  if (CRYPTO_is_BMI2_capable() && CRYPTO_is_ADX_capable()) {
    ecp_nistz256_point_add_adx(r, a, b);
  } else {
    ecp_nistz256_point_add_nohw(r, a, b);
  }
}

static void ecp_nistz256_point_add_affine(P256_POINT *r, const P256_POINT *a,
                                          const P256_POINT_AFFINE *b) {
  if (CRYPTO_is_BMI2_capable() && CRYPTO_is_ADX_capable()) {
    ecp_nistz256_point_add_affine_adx(r, a, b);
  } else {
    ecp_nistz256_point_add_affine_nohw(r, a, b);
  }
}
#endif  // OPENSSL_X86_64

// ecp_nistz256_from_mont sets |res| to |in|, converted from Montgomery domain
// by multiplying with 1.
static void ecp_nistz256_from_mont(BN_ULONG res[P256_LIMBS],
                                   const BN_ULONG in[P256_LIMBS]) {
  static const BN_ULONG ONE[P256_LIMBS] = {1};
  ecp_nistz256_mul_mont(res, in, ONE);
}

// ecp_nistz256_mod_inverse_sqr_mont sets |r| to (|in| * 2^-256)^-2 * 2^256 mod
// p. That is, |r| is the modular inverse square of |in| for input and output in
// the Montgomery domain.
static void ecp_nistz256_mod_inverse_sqr_mont(BN_ULONG r[P256_LIMBS],
                                              const BN_ULONG in[P256_LIMBS]) {
  // This implements the addition chain described in
  // https://briansmith.org/ecc-inversion-addition-chains-01#p256_field_inversion
  BN_ULONG x2[P256_LIMBS], x3[P256_LIMBS], x6[P256_LIMBS], x12[P256_LIMBS],
      x15[P256_LIMBS], x30[P256_LIMBS], x32[P256_LIMBS];
  ecp_nistz256_sqr_mont(x2, in);      // 2^2 - 2^1
  ecp_nistz256_mul_mont(x2, x2, in);  // 2^2 - 2^0

  ecp_nistz256_sqr_mont(x3, x2);      // 2^3 - 2^1
  ecp_nistz256_mul_mont(x3, x3, in);  // 2^3 - 2^0

  ecp_nistz256_sqr_mont(x6, x3);
  for (int i = 1; i < 3; i++) {
    ecp_nistz256_sqr_mont(x6, x6);
  }                                   // 2^6 - 2^3
  ecp_nistz256_mul_mont(x6, x6, x3);  // 2^6 - 2^0

  ecp_nistz256_sqr_mont(x12, x6);
  for (int i = 1; i < 6; i++) {
    ecp_nistz256_sqr_mont(x12, x12);
  }                                     // 2^12 - 2^6
  ecp_nistz256_mul_mont(x12, x12, x6);  // 2^12 - 2^0

  ecp_nistz256_sqr_mont(x15, x12);
  for (int i = 1; i < 3; i++) {
    ecp_nistz256_sqr_mont(x15, x15);
  }                                     // 2^15 - 2^3
  ecp_nistz256_mul_mont(x15, x15, x3);  // 2^15 - 2^0

  ecp_nistz256_sqr_mont(x30, x15);
  for (int i = 1; i < 15; i++) {
    ecp_nistz256_sqr_mont(x30, x30);
  }                                      // 2^30 - 2^15
  ecp_nistz256_mul_mont(x30, x30, x15);  // 2^30 - 2^0

  ecp_nistz256_sqr_mont(x32, x30);
  ecp_nistz256_sqr_mont(x32, x32);      // 2^32 - 2^2
  ecp_nistz256_mul_mont(x32, x32, x2);  // 2^32 - 2^0

  BN_ULONG ret[P256_LIMBS];
  ecp_nistz256_sqr_mont(ret, x32);
  for (int i = 1; i < 31 + 1; i++) {
    ecp_nistz256_sqr_mont(ret, ret);
  }                                     // 2^64 - 2^32
  ecp_nistz256_mul_mont(ret, ret, in);  // 2^64 - 2^32 + 2^0

  for (int i = 0; i < 96 + 32; i++) {
    ecp_nistz256_sqr_mont(ret, ret);
  }                                      // 2^192 - 2^160 + 2^128
  ecp_nistz256_mul_mont(ret, ret, x32);  // 2^192 - 2^160 + 2^128 + 2^32 - 2^0

  for (int i = 0; i < 32; i++) {
    ecp_nistz256_sqr_mont(ret, ret);
  }                                      // 2^224 - 2^192 + 2^160 + 2^64 - 2^32
  ecp_nistz256_mul_mont(ret, ret, x32);  // 2^224 - 2^192 + 2^160 + 2^64 - 2^0

  for (int i = 0; i < 30; i++) {
    ecp_nistz256_sqr_mont(ret, ret);
  }                                      // 2^254 - 2^222 + 2^190 + 2^94 - 2^30
  ecp_nistz256_mul_mont(ret, ret, x30);  // 2^254 - 2^222 + 2^190 + 2^94 - 2^0

  ecp_nistz256_sqr_mont(ret, ret);
  ecp_nistz256_sqr_mont(r, ret);  // 2^256 - 2^224 + 2^192 + 2^96 - 2^2
}

// r = p * p_scalar
static void ecp_nistz256_windowed_mul(const EC_GROUP *group, P256_POINT *r,
                                      const EC_JACOBIAN *p,
                                      const EC_SCALAR *p_scalar) {
  assert(p != NULL);
  assert(p_scalar != NULL);
  assert(group->field.N.width == P256_LIMBS);

  static const size_t kWindowSize = 5;
  static const crypto_word_t kMask = (1 << (5 /* kWindowSize */ + 1)) - 1;

  // A |P256_POINT| is (3 * 32) = 96 bytes, and the 64-byte alignment should
  // add no more than 63 bytes of overhead. Thus, |table| should require
  // ~1599 ((96 * 16) + 63) bytes of stack space.
  alignas(64) P256_POINT table[16];
  uint8_t p_str[33];
  OPENSSL_memcpy(p_str, p_scalar->words, 32);
  p_str[32] = 0;

  // table[0] is implicitly (0,0,0) (the point at infinity), therefore it is
  // not stored. All other values are actually stored with an offset of -1 in
  // table.
  P256_POINT *row = table;
  assert(group->field.N.width == P256_LIMBS);
  OPENSSL_memcpy(row[1 - 1].X, p->X.words, P256_LIMBS * sizeof(BN_ULONG));
  OPENSSL_memcpy(row[1 - 1].Y, p->Y.words, P256_LIMBS * sizeof(BN_ULONG));
  OPENSSL_memcpy(row[1 - 1].Z, p->Z.words, P256_LIMBS * sizeof(BN_ULONG));

  ecp_nistz256_point_double(&row[2 - 1], &row[1 - 1]);
  ecp_nistz256_point_add(&row[3 - 1], &row[2 - 1], &row[1 - 1]);
  ecp_nistz256_point_double(&row[4 - 1], &row[2 - 1]);
  ecp_nistz256_point_double(&row[6 - 1], &row[3 - 1]);
  ecp_nistz256_point_double(&row[8 - 1], &row[4 - 1]);
  ecp_nistz256_point_double(&row[12 - 1], &row[6 - 1]);
  ecp_nistz256_point_add(&row[5 - 1], &row[4 - 1], &row[1 - 1]);
  ecp_nistz256_point_add(&row[7 - 1], &row[6 - 1], &row[1 - 1]);
  ecp_nistz256_point_add(&row[9 - 1], &row[8 - 1], &row[1 - 1]);
  ecp_nistz256_point_add(&row[13 - 1], &row[12 - 1], &row[1 - 1]);
  ecp_nistz256_point_double(&row[14 - 1], &row[7 - 1]);
  ecp_nistz256_point_double(&row[10 - 1], &row[5 - 1]);
  ecp_nistz256_point_add(&row[15 - 1], &row[14 - 1], &row[1 - 1]);
  ecp_nistz256_point_add(&row[11 - 1], &row[10 - 1], &row[1 - 1]);
  ecp_nistz256_point_double(&row[16 - 1], &row[8 - 1]);

  BN_ULONG tmp[P256_LIMBS];
  alignas(32) P256_POINT h;
  size_t index = 255;
  crypto_word_t wvalue = p_str[(index - 1) / 8];
  wvalue = (wvalue >> ((index - 1) % 8)) & kMask;

  ecp_nistz256_select_w5(r, table, booth_recode_w5(wvalue) >> 1);

  while (index >= 5) {
    if (index != 255) {
      size_t off = (index - 1) / 8;

      wvalue = (crypto_word_t)p_str[off] | (crypto_word_t)p_str[off + 1] << 8;
      wvalue = (wvalue >> ((index - 1) % 8)) & kMask;

      wvalue = booth_recode_w5(wvalue);

      ecp_nistz256_select_w5(&h, table, wvalue >> 1);

      ecp_nistz256_neg(tmp, h.Y);
      copy_conditional(h.Y, tmp, (wvalue & 1));

      ecp_nistz256_point_add(r, r, &h);
    }

    index -= kWindowSize;

    ecp_nistz256_point_double(r, r);
    ecp_nistz256_point_double(r, r);
    ecp_nistz256_point_double(r, r);
    ecp_nistz256_point_double(r, r);
    ecp_nistz256_point_double(r, r);
  }

  // Final window
  wvalue = p_str[0];
  wvalue = (wvalue << 1) & kMask;

  wvalue = booth_recode_w5(wvalue);

  ecp_nistz256_select_w5(&h, table, wvalue >> 1);

  ecp_nistz256_neg(tmp, h.Y);
  copy_conditional(h.Y, tmp, wvalue & 1);

  ecp_nistz256_point_add(r, r, &h);
}

static crypto_word_t calc_first_wvalue(size_t *index, const uint8_t p_str[33]) {
  static const size_t kWindowSize = 7;
  static const crypto_word_t kMask = (1 << (7 /* kWindowSize */ + 1)) - 1;
  *index = kWindowSize;

  crypto_word_t wvalue = (p_str[0] << 1) & kMask;
  return booth_recode_w7(wvalue);
}

static crypto_word_t calc_wvalue(size_t *index, const uint8_t p_str[33]) {
  static const size_t kWindowSize = 7;
  static const crypto_word_t kMask = (1 << (7 /* kWindowSize */ + 1)) - 1;

  const size_t off = (*index - 1) / 8;
  crypto_word_t wvalue =
      (crypto_word_t)p_str[off] | (crypto_word_t)p_str[off + 1] << 8;
  wvalue = (wvalue >> ((*index - 1) % 8)) & kMask;
  *index += kWindowSize;

  return booth_recode_w7(wvalue);
}

static void ecp_nistz256_point_mul(const EC_GROUP *group, EC_JACOBIAN *r,
                                   const EC_JACOBIAN *p,
                                   const EC_SCALAR *scalar) {
  alignas(32) P256_POINT out;
  ecp_nistz256_windowed_mul(group, &out, p, scalar);

  assert(group->field.N.width == P256_LIMBS);
  OPENSSL_memcpy(r->X.words, out.X, P256_LIMBS * sizeof(BN_ULONG));
  OPENSSL_memcpy(r->Y.words, out.Y, P256_LIMBS * sizeof(BN_ULONG));
  OPENSSL_memcpy(r->Z.words, out.Z, P256_LIMBS * sizeof(BN_ULONG));
}

static void ecp_nistz256_point_mul_base(const EC_GROUP *group, EC_JACOBIAN *r,
                                        const EC_SCALAR *scalar) {
  uint8_t p_str[33];
  OPENSSL_memcpy(p_str, scalar->words, 32);
  p_str[32] = 0;

  // First window
  size_t index = 0;
  crypto_word_t wvalue = calc_first_wvalue(&index, p_str);

  alignas(32) P256_POINT_AFFINE t;
  alignas(32) P256_POINT p;
  ecp_nistz256_select_w7(&t, ecp_nistz256_precomputed[0], wvalue >> 1);
  ecp_nistz256_neg(p.Z, t.Y);
  copy_conditional(t.Y, p.Z, wvalue & 1);

  // Convert |t| from affine to Jacobian coordinates. We set Z to zero if |t|
  // is infinity and |ONE_MONT| otherwise. |t| was computed from the table, so
  // it is infinity iff |wvalue >> 1| is zero.
  OPENSSL_memcpy(p.X, t.X, sizeof(p.X));
  OPENSSL_memcpy(p.Y, t.Y, sizeof(p.Y));
  OPENSSL_memset(p.Z, 0, sizeof(p.Z));
  copy_conditional(p.Z, ONE_MONT, is_not_zero(wvalue >> 1));

  for (int i = 1; i < 37; i++) {
    wvalue = calc_wvalue(&index, p_str);

    ecp_nistz256_select_w7(&t, ecp_nistz256_precomputed[i], wvalue >> 1);

    alignas(32) BN_ULONG neg_Y[P256_LIMBS];
    ecp_nistz256_neg(neg_Y, t.Y);
    copy_conditional(t.Y, neg_Y, wvalue & 1);

    // Note |ecp_nistz256_point_add_affine| does not work if |p| and |t| are the
    // same non-infinity point.
    ecp_nistz256_point_add_affine(&p, &p, &t);
  }

  assert(group->field.N.width == P256_LIMBS);
  OPENSSL_memcpy(r->X.words, p.X, P256_LIMBS * sizeof(BN_ULONG));
  OPENSSL_memcpy(r->Y.words, p.Y, P256_LIMBS * sizeof(BN_ULONG));
  OPENSSL_memcpy(r->Z.words, p.Z, P256_LIMBS * sizeof(BN_ULONG));
}

static void ecp_nistz256_points_mul_public(const EC_GROUP *group,
                                           EC_JACOBIAN *r,
                                           const EC_SCALAR *g_scalar,
                                           const EC_JACOBIAN *p_,
                                           const EC_SCALAR *p_scalar) {
  assert(p_ != NULL && p_scalar != NULL && g_scalar != NULL);

  alignas(32) P256_POINT p;
  uint8_t p_str[33];
  OPENSSL_memcpy(p_str, g_scalar->words, 32);
  p_str[32] = 0;

  // First window
  size_t index = 0;
  size_t wvalue = calc_first_wvalue(&index, p_str);

  // Convert |p| from affine to Jacobian coordinates. We set Z to zero if |p|
  // is infinity and |ONE_MONT| otherwise. |p| was computed from the table, so
  // it is infinity iff |wvalue >> 1| is zero.
  if ((wvalue >> 1) != 0) {
    OPENSSL_memcpy(p.X, &ecp_nistz256_precomputed[0][(wvalue >> 1) - 1].X,
                   sizeof(p.X));
    OPENSSL_memcpy(p.Y, &ecp_nistz256_precomputed[0][(wvalue >> 1) - 1].Y,
                   sizeof(p.Y));
    OPENSSL_memcpy(p.Z, ONE_MONT, sizeof(p.Z));
  } else {
    OPENSSL_memset(p.X, 0, sizeof(p.X));
    OPENSSL_memset(p.Y, 0, sizeof(p.Y));
    OPENSSL_memset(p.Z, 0, sizeof(p.Z));
  }

  if ((wvalue & 1) == 1) {
    ecp_nistz256_neg(p.Y, p.Y);
  }

  for (int i = 1; i < 37; i++) {
    wvalue = calc_wvalue(&index, p_str);
    if ((wvalue >> 1) == 0) {
      continue;
    }

    alignas(32) P256_POINT_AFFINE t;
    OPENSSL_memcpy(&t, &ecp_nistz256_precomputed[i][(wvalue >> 1) - 1],
                   sizeof(t));
    if ((wvalue & 1) == 1) {
      ecp_nistz256_neg(t.Y, t.Y);
    }

    // Note |ecp_nistz256_point_add_affine| does not work if |p| and |t| are
    // the same non-infinity point, so it is important that we compute the
    // |g_scalar| term before the |p_scalar| term.
    ecp_nistz256_point_add_affine(&p, &p, &t);
  }

  alignas(32) P256_POINT tmp;
  ecp_nistz256_windowed_mul(group, &tmp, p_, p_scalar);
  ecp_nistz256_point_add(&p, &p, &tmp);

  assert(group->field.N.width == P256_LIMBS);
  OPENSSL_memcpy(r->X.words, p.X, P256_LIMBS * sizeof(BN_ULONG));
  OPENSSL_memcpy(r->Y.words, p.Y, P256_LIMBS * sizeof(BN_ULONG));
  OPENSSL_memcpy(r->Z.words, p.Z, P256_LIMBS * sizeof(BN_ULONG));
}

static int ecp_nistz256_get_affine(const EC_GROUP *group,
                                   const EC_JACOBIAN *point, EC_FELEM *x,
                                   EC_FELEM *y) {
  if (constant_time_declassify_int(
          ec_GFp_simple_is_at_infinity(group, point))) {
    OPENSSL_PUT_ERROR(EC, EC_R_POINT_AT_INFINITY);
    return 0;
  }

  BN_ULONG z_inv2[P256_LIMBS];
  assert(group->field.N.width == P256_LIMBS);
  ecp_nistz256_mod_inverse_sqr_mont(z_inv2, point->Z.words);

  if (x != NULL) {
    ecp_nistz256_mul_mont(x->words, z_inv2, point->X.words);
  }

  if (y != NULL) {
    ecp_nistz256_sqr_mont(z_inv2, z_inv2);                            // z^-4
    ecp_nistz256_mul_mont(y->words, point->Y.words, point->Z.words);  // y * z
    ecp_nistz256_mul_mont(y->words, y->words, z_inv2);  // y * z^-3
  }

  return 1;
}

static void ecp_nistz256_add(const EC_GROUP *group, EC_JACOBIAN *r,
                             const EC_JACOBIAN *a_, const EC_JACOBIAN *b_) {
  P256_POINT a, b;
  OPENSSL_memcpy(a.X, a_->X.words, P256_LIMBS * sizeof(BN_ULONG));
  OPENSSL_memcpy(a.Y, a_->Y.words, P256_LIMBS * sizeof(BN_ULONG));
  OPENSSL_memcpy(a.Z, a_->Z.words, P256_LIMBS * sizeof(BN_ULONG));
  OPENSSL_memcpy(b.X, b_->X.words, P256_LIMBS * sizeof(BN_ULONG));
  OPENSSL_memcpy(b.Y, b_->Y.words, P256_LIMBS * sizeof(BN_ULONG));
  OPENSSL_memcpy(b.Z, b_->Z.words, P256_LIMBS * sizeof(BN_ULONG));
  ecp_nistz256_point_add(&a, &a, &b);
  OPENSSL_memcpy(r->X.words, a.X, P256_LIMBS * sizeof(BN_ULONG));
  OPENSSL_memcpy(r->Y.words, a.Y, P256_LIMBS * sizeof(BN_ULONG));
  OPENSSL_memcpy(r->Z.words, a.Z, P256_LIMBS * sizeof(BN_ULONG));
}

static void ecp_nistz256_dbl(const EC_GROUP *group, EC_JACOBIAN *r,
                             const EC_JACOBIAN *a_) {
  P256_POINT a;
  OPENSSL_memcpy(a.X, a_->X.words, P256_LIMBS * sizeof(BN_ULONG));
  OPENSSL_memcpy(a.Y, a_->Y.words, P256_LIMBS * sizeof(BN_ULONG));
  OPENSSL_memcpy(a.Z, a_->Z.words, P256_LIMBS * sizeof(BN_ULONG));
  ecp_nistz256_point_double(&a, &a);
  OPENSSL_memcpy(r->X.words, a.X, P256_LIMBS * sizeof(BN_ULONG));
  OPENSSL_memcpy(r->Y.words, a.Y, P256_LIMBS * sizeof(BN_ULONG));
  OPENSSL_memcpy(r->Z.words, a.Z, P256_LIMBS * sizeof(BN_ULONG));
}

static void ecp_nistz256_inv0_mod_ord(const EC_GROUP *group, EC_SCALAR *out,
                                      const EC_SCALAR *in) {
  // table[i] stores a power of |in| corresponding to the matching enum value.
  enum {
    // The following indices specify the power in binary.
    i_1 = 0,
    i_10,
    i_11,
    i_101,
    i_111,
    i_1010,
    i_1111,
    i_10101,
    i_101010,
    i_101111,
    // The following indices specify 2^N-1, or N ones in a row.
    i_x6,
    i_x8,
    i_x16,
    i_x32
  };
  BN_ULONG table[15][P256_LIMBS];

  // https://briansmith.org/ecc-inversion-addition-chains-01#p256_scalar_inversion
  //
  // Even though this code path spares 12 squarings, 4.5%, and 13
  // multiplications, 25%, the overall sign operation is not that much faster,
  // not more that 2%. Most of the performance of this function comes from the
  // scalar operations.

  // Pre-calculate powers.
  OPENSSL_memcpy(table[i_1], in->words, P256_LIMBS * sizeof(BN_ULONG));

  ecp_nistz256_ord_sqr_mont(table[i_10], table[i_1], 1);

  ecp_nistz256_ord_mul_mont(table[i_11], table[i_1], table[i_10]);

  ecp_nistz256_ord_mul_mont(table[i_101], table[i_11], table[i_10]);

  ecp_nistz256_ord_mul_mont(table[i_111], table[i_101], table[i_10]);

  ecp_nistz256_ord_sqr_mont(table[i_1010], table[i_101], 1);

  ecp_nistz256_ord_mul_mont(table[i_1111], table[i_1010], table[i_101]);

  ecp_nistz256_ord_sqr_mont(table[i_10101], table[i_1010], 1);
  ecp_nistz256_ord_mul_mont(table[i_10101], table[i_10101], table[i_1]);

  ecp_nistz256_ord_sqr_mont(table[i_101010], table[i_10101], 1);

  ecp_nistz256_ord_mul_mont(table[i_101111], table[i_101010], table[i_101]);

  ecp_nistz256_ord_mul_mont(table[i_x6], table[i_101010], table[i_10101]);

  ecp_nistz256_ord_sqr_mont(table[i_x8], table[i_x6], 2);
  ecp_nistz256_ord_mul_mont(table[i_x8], table[i_x8], table[i_11]);

  ecp_nistz256_ord_sqr_mont(table[i_x16], table[i_x8], 8);
  ecp_nistz256_ord_mul_mont(table[i_x16], table[i_x16], table[i_x8]);

  ecp_nistz256_ord_sqr_mont(table[i_x32], table[i_x16], 16);
  ecp_nistz256_ord_mul_mont(table[i_x32], table[i_x32], table[i_x16]);

  // Compute |in| raised to the order-2.
  ecp_nistz256_ord_sqr_mont(out->words, table[i_x32], 64);
  ecp_nistz256_ord_mul_mont(out->words, out->words, table[i_x32]);
  static const struct {
    uint8_t p, i;
  } kChain[27] = {{32, i_x32},    {6, i_101111}, {5, i_111},    {4, i_11},
                  {5, i_1111},    {5, i_10101},  {4, i_101},    {3, i_101},
                  {3, i_101},     {5, i_111},    {9, i_101111}, {6, i_1111},
                  {2, i_1},       {5, i_1},      {6, i_1111},   {5, i_111},
                  {4, i_111},     {5, i_111},    {5, i_101},    {3, i_11},
                  {10, i_101111}, {2, i_11},     {5, i_11},     {5, i_11},
                  {3, i_1},       {7, i_10101},  {6, i_1111}};
  for (size_t i = 0; i < OPENSSL_ARRAY_SIZE(kChain); i++) {
    ecp_nistz256_ord_sqr_mont(out->words, out->words, kChain[i].p);
    ecp_nistz256_ord_mul_mont(out->words, out->words, table[kChain[i].i]);
  }
}

static int ecp_nistz256_scalar_to_montgomery_inv_vartime(const EC_GROUP *group,
                                                         EC_SCALAR *out,
                                                         const EC_SCALAR *in) {
#if defined(OPENSSL_X86_64)
  if (!CRYPTO_is_AVX_capable()) {
    // No AVX support; fallback to generic code.
    return ec_simple_scalar_to_montgomery_inv_vartime(group, out, in);
  }
#endif

  assert(group->order.N.width == P256_LIMBS);
  if (!beeu_mod_inverse_vartime(out->words, in->words, group->order.N.d)) {
    return 0;
  }

  // The result should be returned in the Montgomery domain.
  ec_scalar_to_montgomery(group, out, out);
  return 1;
}

static int ecp_nistz256_cmp_x_coordinate(const EC_GROUP *group,
                                         const EC_JACOBIAN *p,
                                         const EC_SCALAR *r) {
  if (ec_GFp_simple_is_at_infinity(group, p)) {
    return 0;
  }

  assert(group->order.N.width == P256_LIMBS);
  assert(group->field.N.width == P256_LIMBS);

  // We wish to compare X/Z^2 with r. This is equivalent to comparing X with
  // r*Z^2. Note that X and Z are represented in Montgomery form, while r is
  // not.
  BN_ULONG r_Z2[P256_LIMBS], Z2_mont[P256_LIMBS], X[P256_LIMBS];
  ecp_nistz256_mul_mont(Z2_mont, p->Z.words, p->Z.words);
  ecp_nistz256_mul_mont(r_Z2, r->words, Z2_mont);
  ecp_nistz256_from_mont(X, p->X.words);

  if (OPENSSL_memcmp(r_Z2, X, sizeof(r_Z2)) == 0) {
    return 1;
  }

  // During signing the x coefficient is reduced modulo the group order.
  // Therefore there is a small possibility, less than 1/2^128, that group_order
  // < p.x < P. in that case we need not only to compare against |r| but also to
  // compare against r+group_order.
  BN_ULONG carry = bn_add_words(r_Z2, r->words, group->order.N.d, P256_LIMBS);
  if (carry == 0 && bn_less_than_words(r_Z2, group->field.N.d, P256_LIMBS)) {
    // r + group_order < p, so compare (r + group_order) * Z^2 against X.
    ecp_nistz256_mul_mont(r_Z2, r_Z2, Z2_mont);
    if (OPENSSL_memcmp(r_Z2, X, sizeof(r_Z2)) == 0) {
      return 1;
    }
  }

  return 0;
}

DEFINE_METHOD_FUNCTION(EC_METHOD, EC_GFp_nistz256_method) {
  out->point_get_affine_coordinates = ecp_nistz256_get_affine;
  out->add = ecp_nistz256_add;
  out->dbl = ecp_nistz256_dbl;
  out->mul = ecp_nistz256_point_mul;
  out->mul_base = ecp_nistz256_point_mul_base;
  out->mul_public = ecp_nistz256_points_mul_public;
  out->felem_mul = ec_GFp_mont_felem_mul;
  out->felem_sqr = ec_GFp_mont_felem_sqr;
  out->felem_to_bytes = ec_GFp_mont_felem_to_bytes;
  out->felem_from_bytes = ec_GFp_mont_felem_from_bytes;
  out->felem_reduce = ec_GFp_mont_felem_reduce;
  // TODO(davidben): This should use the specialized field arithmetic
  // implementation, rather than the generic one.
  out->felem_exp = ec_GFp_mont_felem_exp;
  out->scalar_inv0_montgomery = ecp_nistz256_inv0_mod_ord;
  out->scalar_to_montgomery_inv_vartime =
      ecp_nistz256_scalar_to_montgomery_inv_vartime;
  out->cmp_x_coordinate = ecp_nistz256_cmp_x_coordinate;
}

#endif /* !defined(OPENSSL_NO_ASM) &&                              \
          (defined(OPENSSL_X86_64) || defined(OPENSSL_AARCH64)) && \
          !defined(OPENSSL_SMALL) */
