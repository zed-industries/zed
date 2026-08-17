// Copyright (c) 2020, Google Inc.
// SPDX-License-Identifier: ISC

#include <openssl/ec.h>

#include <openssl/digest.h>
#include <openssl/err.h>
#include <openssl/nid.h>
#include <openssl/type_check.h>

#include <assert.h>

#include "internal.h"
#include "../fipsmodule/bn/internal.h"
#include "../fipsmodule/ec/internal.h"
#include "../internal.h"


// This file implements hash-to-curve, as described in RFC 9380.
//
// This hash-to-curve implementation is written generically with the
// expectation that we will eventually wish to support other curves. If it
// becomes a performance bottleneck, some possible optimizations by
// specializing it to the curve:
//
// - Rather than using a generic |felem_exp|, specialize the exponentation to
//   c2 with a faster addition chain.
//
// - |felem_mul| and |felem_sqr| are indirect calls to generic Montgomery
//   code. Given the few curves, we could specialize
//   |map_to_curve_simple_swu|. But doing this reasonably without duplicating
//   code in C is difficult. (C++ templates would be useful here.)
//
// - P-521's Z and c2 have small power-of-two absolute values. We could save
//   two multiplications in SSWU. (Other curves have reasonable values of Z
//   and inconvenient c2.) This is unlikely to be worthwhile without C++
//   templates to make specializing more convenient.

// expand_message_xmd implements the operation described in section 5.3.1 of
// RFC 9380. It returns one on success and zero on error.
static int expand_message_xmd(const EVP_MD *md, uint8_t *out, size_t out_len,
                              const uint8_t *msg, size_t msg_len,
                              const uint8_t *dst, size_t dst_len) {
  // See https://github.com/cfrg/draft-irtf-cfrg-hash-to-curve/issues/352
  if (dst_len == 0) {
    OPENSSL_PUT_ERROR(EC, ERR_R_SHOULD_NOT_HAVE_BEEN_CALLED);
    return 0;
  }

  int ret = 0;
  const size_t block_size = EVP_MD_block_size(md);
  const size_t md_size = EVP_MD_size(md);
  EVP_MD_CTX ctx;
  EVP_MD_CTX_init(&ctx);

  // Long DSTs are hashed down to size. See section 5.3.3.
  OPENSSL_STATIC_ASSERT(EVP_MAX_MD_SIZE < 256, hashed_DST_still_too_large)
  uint8_t dst_buf[EVP_MAX_MD_SIZE];
  if (dst_len >= 256) {
    static const char kPrefix[] = "H2C-OVERSIZE-DST-";
    if (!EVP_DigestInit_ex(&ctx, md, NULL) ||
        !EVP_DigestUpdate(&ctx, kPrefix, sizeof(kPrefix) - 1) ||
        !EVP_DigestUpdate(&ctx, dst, dst_len) ||
        !EVP_DigestFinal_ex(&ctx, dst_buf, NULL)) {
      goto err;
    }
    dst = dst_buf;
    dst_len = md_size;
  }
  uint8_t dst_len_u8 = (uint8_t)dst_len;

  // Compute b_0.
  static const uint8_t kZeros[EVP_MAX_MD_BLOCK_SIZE] = {0};
  // If |out_len| exceeds 16 bits then |i| will wrap below causing an error to
  // be returned. This depends on the static assert above.
  uint8_t l_i_b_str_zero[3] = {out_len >> 8, out_len, 0};
  uint8_t b_0[EVP_MAX_MD_SIZE];
  if (!EVP_DigestInit_ex(&ctx, md, NULL) ||
      !EVP_DigestUpdate(&ctx, kZeros, block_size) ||
      !EVP_DigestUpdate(&ctx, msg, msg_len) ||
      !EVP_DigestUpdate(&ctx, l_i_b_str_zero, sizeof(l_i_b_str_zero)) ||
      !EVP_DigestUpdate(&ctx, dst, dst_len) ||
      !EVP_DigestUpdate(&ctx, &dst_len_u8, 1) ||
      !EVP_DigestFinal_ex(&ctx, b_0, NULL)) {
    goto err;
  }

  uint8_t b_i[EVP_MAX_MD_SIZE];
  uint8_t i = 1;
  while (out_len > 0) {
    if (i == 0) {
      // Input was too large.
      OPENSSL_PUT_ERROR(EC, ERR_R_INTERNAL_ERROR);
      goto err;
    }
    if (i > 1) {
      for (size_t j = 0; j < md_size; j++) {
        b_i[j] ^= b_0[j];
      }
    } else {
      OPENSSL_memcpy(b_i, b_0, md_size);
    }

    if (!EVP_DigestInit_ex(&ctx, md, NULL) ||
        !EVP_DigestUpdate(&ctx, b_i, md_size) ||
        !EVP_DigestUpdate(&ctx, &i, 1) ||
        !EVP_DigestUpdate(&ctx, dst, dst_len) ||
        !EVP_DigestUpdate(&ctx, &dst_len_u8, 1) ||
        !EVP_DigestFinal_ex(&ctx, b_i, NULL)) {
      goto err;
    }

    size_t todo = out_len >= md_size ? md_size : out_len;
    OPENSSL_memcpy(out, b_i, todo);
    out += todo;
    out_len -= todo;
    i++;
  }

  ret = 1;

err:
  EVP_MD_CTX_cleanup(&ctx);
  return ret;
}

// num_bytes_to_derive determines the number of bytes to derive when hashing to
// a number modulo |modulus|. See the hash_to_field operation defined in
// section 5.2 of RFC 9380.
static int num_bytes_to_derive(size_t *out, const BIGNUM *modulus, unsigned k) {
  size_t bits = BN_num_bits(modulus);
  size_t L = (bits + k + 7) / 8;
  // We require 2^(8*L) < 2^(2*bits - 2) <= n^2 so to fit in bounds for
  // |felem_reduce| and |ec_scalar_reduce|. All defined hash-to-curve suites
  // define |k| to be well under this bound. (|k| is usually around half of
  // |p_bits|.)
  if (L * 8 >= 2 * bits - 2 ||
      L > 2 * EC_MAX_BYTES) {
    assert(0);
    OPENSSL_PUT_ERROR(EC, ERR_R_INTERNAL_ERROR);
    return 0;
  }

  *out = L;
  return 1;
}

// hash_to_field implements the operation described in section 5.2
// of RFC 9380, with count = 2. |k| is the security factor.
static int hash_to_field2(const EC_GROUP *group, const EVP_MD *md,
                          EC_FELEM *out1, EC_FELEM *out2, const uint8_t *dst,
                          size_t dst_len, unsigned k, const uint8_t *msg,
                          size_t msg_len) {
  size_t L;
  uint8_t buf[4 * EC_MAX_BYTES];
  if (!num_bytes_to_derive(&L, &group->field.N, k) ||
      !expand_message_xmd(md, buf, 2 * L, msg, msg_len, dst, dst_len)) {
    return 0;
  }
  BN_ULONG words[2 * EC_MAX_WORDS];
  size_t num_words = 2 * group->field.N.width;
  bn_big_endian_to_words(words, num_words, buf, L);
  group->meth->felem_reduce(group, out1, words, num_words);
  bn_big_endian_to_words(words, num_words, buf + L, L);
  group->meth->felem_reduce(group, out2, words, num_words);
  return 1;
}

// hash_to_scalar behaves like |hash_to_field2| but returns a value modulo the
// group order rather than a field element. |k| is the security factor.
static int hash_to_scalar(const EC_GROUP *group, const EVP_MD *md,
                          EC_SCALAR *out, const uint8_t *dst, size_t dst_len,
                          unsigned k, const uint8_t *msg, size_t msg_len) {
  const BIGNUM *order = EC_GROUP_get0_order(group);
  size_t L;
  uint8_t buf[EC_MAX_BYTES * 2];
  if (!num_bytes_to_derive(&L, order, k) ||
      !expand_message_xmd(md, buf, L, msg, msg_len, dst, dst_len)) {
    return 0;
  }

  BN_ULONG words[2 * EC_MAX_WORDS];
  size_t num_words = 2 * order->width;
  bn_big_endian_to_words(words, num_words, buf, L);
  ec_scalar_reduce(group, out, words, num_words);
  return 1;
}

static inline void mul_A(const EC_GROUP *group, EC_FELEM *out,
                         const EC_FELEM *in) {
  assert(group->a_is_minus3);
  EC_FELEM tmp;
  ec_felem_add(group, &tmp, in, in);      // tmp = 2*in
  ec_felem_add(group, &tmp, &tmp, &tmp);  // tmp = 4*in
  ec_felem_sub(group, out, in, &tmp);     // out = -3*in
}

// sgn0 implements the operation described in section 4.1.2 of RFC 9380.
static BN_ULONG sgn0(const EC_GROUP *group, const EC_FELEM *a) {
  uint8_t buf[EC_MAX_BYTES];
  size_t len;
  ec_felem_to_bytes(group, buf, &len, a);
  return buf[len - 1] & 1;
}

OPENSSL_UNUSED static int is_3mod4(const EC_GROUP *group) {
  return group->field.N.width > 0 && (group->field.N.d[0] & 3) == 3;
}

// sqrt_ratio_3mod4 implements the operation described in appendix F.2.1.2
// of RFC 9380.
static BN_ULONG sqrt_ratio_3mod4(const EC_GROUP *group, const EC_FELEM *Z,
                                 const BN_ULONG *c1, size_t num_c1,
                                 const EC_FELEM *c2, EC_FELEM *out_y,
                                 const EC_FELEM *u, const EC_FELEM *v) {
  assert(is_3mod4(group));

  void (*const felem_mul)(const EC_GROUP *, EC_FELEM *r, const EC_FELEM *a,
                          const EC_FELEM *b) = group->meth->felem_mul;
  void (*const felem_sqr)(const EC_GROUP *, EC_FELEM *r, const EC_FELEM *a) =
      group->meth->felem_sqr;

  EC_FELEM tv1, tv2, tv3, y1, y2;
  felem_sqr(group, &tv1, v);                             // 1. tv1 = v^2
  felem_mul(group, &tv2, u, v);                          // 2. tv2 = u * v
  felem_mul(group, &tv1, &tv1, &tv2);                    // 3. tv1 = tv1 * tv2
  group->meth->felem_exp(group, &y1, &tv1, c1, num_c1);  // 4. y1 = tv1^c1
  felem_mul(group, &y1, &y1, &tv2);                      // 5. y1 = y1 * tv2
  felem_mul(group, &y2, &y1, c2);                        // 6. y2 = y1 * c2
  felem_sqr(group, &tv3, &y1);                           // 7. tv3 = y1^2
  felem_mul(group, &tv3, &tv3, v);                       // 8. tv3 = tv3 * v

  // 9. isQR = tv3 == u
  // 10. y = CMOV(y2, y1, isQR)
  // 11. return (isQR, y)
  //
  // Note the specification's CMOV function and our |ec_felem_select| have the
  // opposite argument order.
  ec_felem_sub(group, &tv1, &tv3, u);
  const BN_ULONG isQR = ~ec_felem_non_zero_mask(group, &tv1);
  ec_felem_select(group, out_y, isQR, &y1, &y2);
  return isQR;
}

// map_to_curve_simple_swu implements the operation described in section 6.6.2
// of RFC 9380, using the straight-line implementation in appendix F.2.
static void map_to_curve_simple_swu(const EC_GROUP *group, const EC_FELEM *Z,
                                    const BN_ULONG *c1, size_t num_c1,
                                    const EC_FELEM *c2, EC_JACOBIAN *out,
                                    const EC_FELEM *u) {
  // This function requires the prime be 3 mod 4, and that A = -3.
  assert(is_3mod4(group));
  assert(group->a_is_minus3);

  void (*const felem_mul)(const EC_GROUP *, EC_FELEM *r, const EC_FELEM *a,
                          const EC_FELEM *b) = group->meth->felem_mul;
  void (*const felem_sqr)(const EC_GROUP *, EC_FELEM *r, const EC_FELEM *a) =
      group->meth->felem_sqr;

  EC_FELEM tv1, tv2, tv3, tv4, tv5, tv6, x, y, y1;
  felem_sqr(group, &tv1, u);                             // 1. tv1 = u^2
  felem_mul(group, &tv1, Z, &tv1);                       // 2. tv1 = Z * tv1
  felem_sqr(group, &tv2, &tv1);                          // 3. tv2 = tv1^2
  ec_felem_add(group, &tv2, &tv2, &tv1);                 // 4. tv2 = tv2 + tv1
  ec_felem_add(group, &tv3, &tv2, ec_felem_one(group));  // 5. tv3 = tv2 + 1
  felem_mul(group, &tv3, &group->b, &tv3);               // 6. tv3 = B * tv3

  // 7. tv4 = CMOV(Z, -tv2, tv2 != 0)
  const BN_ULONG tv2_non_zero = ec_felem_non_zero_mask(group, &tv2);
  ec_felem_neg(group, &tv4, &tv2);
  ec_felem_select(group, &tv4, tv2_non_zero, &tv4, Z);

  mul_A(group, &tv4, &tv4);                 // 8. tv4 = A * tv4
  felem_sqr(group, &tv2, &tv3);             // 9. tv2 = tv3^2
  felem_sqr(group, &tv6, &tv4);             // 10. tv6 = tv4^2
  mul_A(group, &tv5, &tv6);                 // 11. tv5 = A * tv6
  ec_felem_add(group, &tv2, &tv2, &tv5);    // 12. tv2 = tv2 + tv5
  felem_mul(group, &tv2, &tv2, &tv3);       // 13. tv2 = tv2 * tv3
  felem_mul(group, &tv6, &tv6, &tv4);       // 14. tv6 = tv6 * tv4
  felem_mul(group, &tv5, &group->b, &tv6);  // 15. tv5 = B * tv6
  ec_felem_add(group, &tv2, &tv2, &tv5);    // 16. tv2 = tv2 + tv5
  felem_mul(group, &x, &tv1, &tv3);         // 17. x = tv1 * tv3

  // 18. (is_gx1_square, y1) = sqrt_ratio(tv2, tv6)
  const BN_ULONG is_gx1_square =
      sqrt_ratio_3mod4(group, Z, c1, num_c1, c2, &y1, &tv2, &tv6);

  felem_mul(group, &y, &tv1, u);  // 19. y = tv1 * u
  felem_mul(group, &y, &y, &y1);  // 20. y = y * y1

  // 21. x = CMOV(x, tv3, is_gx1_square)
  ec_felem_select(group, &x, is_gx1_square, &tv3, &x);
  // 22. y = CMOV(y, y1, is_gx1_square)
  ec_felem_select(group, &y, is_gx1_square, &y1, &y);

  // 23. e1 = sgn0(u) == sgn0(y)
  BN_ULONG sgn0_u = sgn0(group, u);
  BN_ULONG sgn0_y = sgn0(group, &y);
  BN_ULONG not_e1 = sgn0_u ^ sgn0_y;
  not_e1 = ((BN_ULONG)0) - not_e1;

  // 24. y = CMOV(-y, y, e1)
  ec_felem_neg(group, &tv1, &y);
  ec_felem_select(group, &y, not_e1, &tv1, &y);

  // 25. x = x / tv4
  //
  // Our output is in projective coordinates, so rather than inverting |tv4|
  // now, represent (x / tv4, y) as (x * tv4, y * tv4^3, tv4). This is much more
  // efficient if the caller will do further computation on the output. (If the
  // caller will immediately convert to affine coordinates, it is slightly less
  // efficient, but only by a few field multiplications.)
  felem_mul(group, &out->X, &x, &tv4);
  felem_mul(group, &out->Y, &y, &tv6);
  out->Z = tv4;
}

static int hash_to_curve(const EC_GROUP *group, const EVP_MD *md,
                         const EC_FELEM *Z, const EC_FELEM *c2, unsigned k,
                         EC_JACOBIAN *out, const uint8_t *dst, size_t dst_len,
                         const uint8_t *msg, size_t msg_len) {
  EC_FELEM u0, u1;
  if (!hash_to_field2(group, md, &u0, &u1, dst, dst_len, k, msg, msg_len)) {
    return 0;
  }

  // Compute |c1| = (p - 3) / 4.
  BN_ULONG c1[EC_MAX_WORDS];
  size_t num_c1 = group->field.N.width;
  if (!bn_copy_words(c1, num_c1, &group->field.N)) {
    return 0;
  }
  bn_rshift_words(c1, c1, /*shift=*/2, /*num=*/num_c1);

  EC_JACOBIAN Q0, Q1;
  map_to_curve_simple_swu(group, Z, c1, num_c1, c2, &Q0, &u0);
  map_to_curve_simple_swu(group, Z, c1, num_c1, c2, &Q1, &u1);

  group->meth->add(group, out, &Q0, &Q1);  // R = Q0 + Q1
  // All our curves have cofactor one, so |clear_cofactor| is a no-op.
  return 1;
}

static int felem_from_u8(const EC_GROUP *group, EC_FELEM *out, uint8_t a) {
  uint8_t bytes[EC_MAX_BYTES] = {0};
  size_t len = BN_num_bytes(&group->field.N);
  bytes[len - 1] = a;
  return ec_felem_from_bytes(group, out, bytes, len);
}

// kP256Sqrt10 is sqrt(10) in P-256's field. It was computed as follows in
// python3:
//
// p =  2**256 - 2**224 + 2**192 + 2**96 - 1
// c2 = pow(10, (p+1)//4, p)
// assert pow(c2, 2, p) == 10
// ", ".join("0x%02x" % b for b in c2.to_bytes(256//8, 'big'))
static const uint8_t kP256Sqrt10[] = {
    0xda, 0x53, 0x8e, 0x3b, 0xe1, 0xd8, 0x9b, 0x99, 0xc9, 0x78, 0xfc,
    0x67, 0x51, 0x80, 0xaa, 0xb2, 0x7b, 0x8d, 0x1f, 0xf8, 0x4c, 0x55,
    0xd5, 0xb6, 0x2c, 0xcd, 0x34, 0x27, 0xe4, 0x33, 0xc4, 0x7f};

// kP384Sqrt12 is sqrt(12) in P-384's field. It was computed as follows in
// python3:
//
// p = 2**384 - 2**128 - 2**96 + 2**32 - 1
// c2 = pow(12, (p+1)//4, p)
// assert pow(c2, 2, p) == 12
// ", ".join("0x%02x" % b for b in c2.to_bytes(384//8, 'big'))
static const uint8_t kP384Sqrt12[] = {
    0x2a, 0xcc, 0xb4, 0xa6, 0x56, 0xb0, 0x24, 0x9c, 0x71, 0xf0, 0x50, 0x0e,
    0x83, 0xda, 0x2f, 0xdd, 0x7f, 0x98, 0xe3, 0x83, 0xd6, 0x8b, 0x53, 0x87,
    0x1f, 0x87, 0x2f, 0xcb, 0x9c, 0xcb, 0x80, 0xc5, 0x3c, 0x0d, 0xe1, 0xf8,
    0xa8, 0x0f, 0x7e, 0x19, 0x14, 0xe2, 0xec, 0x69, 0xf5, 0xa6, 0x26, 0xb3};

int ec_hash_to_curve_p256_xmd_sha256_sswu(const EC_GROUP *group,
                                          EC_JACOBIAN *out, const uint8_t *dst,
                                          size_t dst_len, const uint8_t *msg,
                                          size_t msg_len) {
  // See section 8.3 of RFC 9380.
  if (EC_GROUP_get_curve_name(group) != NID_X9_62_prime256v1) {
    OPENSSL_PUT_ERROR(EC, EC_R_GROUP_MISMATCH);
    return 0;
  }

  // Z = -10, c2 = sqrt(10)
  EC_FELEM Z, c2;
  if (!felem_from_u8(group, &Z, 10) ||
      !ec_felem_from_bytes(group, &c2, kP256Sqrt10, sizeof(kP256Sqrt10))) {
    return 0;
  }
  ec_felem_neg(group, &Z, &Z);

  return hash_to_curve(group, EVP_sha256(), &Z, &c2, /*k=*/128, out, dst,
                       dst_len, msg, msg_len);
}

int EC_hash_to_curve_p256_xmd_sha256_sswu(const EC_GROUP *group, EC_POINT *out,
                                          const uint8_t *dst, size_t dst_len,
                                          const uint8_t *msg, size_t msg_len) {
  if (EC_GROUP_cmp(group, out->group, NULL) != 0) {
    OPENSSL_PUT_ERROR(EC, EC_R_INCOMPATIBLE_OBJECTS);
    return 0;
  }
  return ec_hash_to_curve_p256_xmd_sha256_sswu(group, &out->raw, dst, dst_len,
                                               msg, msg_len);
}

int ec_hash_to_curve_p384_xmd_sha384_sswu(const EC_GROUP *group,
                                          EC_JACOBIAN *out, const uint8_t *dst,
                                          size_t dst_len, const uint8_t *msg,
                                          size_t msg_len) {
  // See section 8.3 of RFC 9380.
  if (EC_GROUP_get_curve_name(group) != NID_secp384r1) {
    OPENSSL_PUT_ERROR(EC, EC_R_GROUP_MISMATCH);
    return 0;
  }

  // Z = -12, c2 = sqrt(12)
  EC_FELEM Z, c2;
  if (!felem_from_u8(group, &Z, 12) ||
      !ec_felem_from_bytes(group, &c2, kP384Sqrt12, sizeof(kP384Sqrt12))) {
    return 0;
  }
  ec_felem_neg(group, &Z, &Z);

  return hash_to_curve(group, EVP_sha384(), &Z, &c2, /*k=*/192, out, dst,
                       dst_len, msg, msg_len);
}

int EC_hash_to_curve_p384_xmd_sha384_sswu(const EC_GROUP *group, EC_POINT *out,
                                          const uint8_t *dst, size_t dst_len,
                                          const uint8_t *msg, size_t msg_len) {
  if (EC_GROUP_cmp(group, out->group, NULL) != 0) {
    OPENSSL_PUT_ERROR(EC, EC_R_INCOMPATIBLE_OBJECTS);
    return 0;
  }
  return ec_hash_to_curve_p384_xmd_sha384_sswu(group, &out->raw, dst, dst_len,
                                               msg, msg_len);
}

int ec_hash_to_scalar_p384_xmd_sha384(
    const EC_GROUP *group, EC_SCALAR *out, const uint8_t *dst, size_t dst_len,
    const uint8_t *msg, size_t msg_len) {
  if (EC_GROUP_get_curve_name(group) != NID_secp384r1) {
    OPENSSL_PUT_ERROR(EC, EC_R_GROUP_MISMATCH);
    return 0;
  }

  return hash_to_scalar(group, EVP_sha384(), out, dst, dst_len, /*k=*/192, msg,
                        msg_len);
}

int ec_hash_to_curve_p384_xmd_sha512_sswu_draft07(
    const EC_GROUP *group, EC_JACOBIAN *out, const uint8_t *dst,
    size_t dst_len, const uint8_t *msg, size_t msg_len) {
  // See section 8.3 of draft-irtf-cfrg-hash-to-curve-07.
  if (EC_GROUP_get_curve_name(group) != NID_secp384r1) {
    OPENSSL_PUT_ERROR(EC, EC_R_GROUP_MISMATCH);
    return 0;
  }

  // Z = -12, c2 = sqrt(12)
  EC_FELEM Z, c2;
  if (!felem_from_u8(group, &Z, 12) ||
      !ec_felem_from_bytes(group, &c2, kP384Sqrt12, sizeof(kP384Sqrt12))) {
    return 0;
  }
  ec_felem_neg(group, &Z, &Z);

  return hash_to_curve(group, EVP_sha512(), &Z, &c2, /*k=*/192, out, dst,
                       dst_len, msg, msg_len);
}

int ec_hash_to_scalar_p384_xmd_sha512_draft07(
    const EC_GROUP *group, EC_SCALAR *out, const uint8_t *dst, size_t dst_len,
    const uint8_t *msg, size_t msg_len) {
  if (EC_GROUP_get_curve_name(group) != NID_secp384r1) {
    OPENSSL_PUT_ERROR(EC, EC_R_GROUP_MISMATCH);
    return 0;
  }

  return hash_to_scalar(group, EVP_sha512(), out, dst, dst_len, /*k=*/192, msg,
                        msg_len);
}
