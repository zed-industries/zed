// Copyright (c) 2016, Google Inc.
// SPDX-License-Identifier: ISC

#include <openssl/base.h>

#include <stdio.h>
#include <string.h>

#include <gtest/gtest.h>

#include <openssl/bn.h>
#include <openssl/ec.h>
#include <openssl/mem.h>
#include <openssl/nid.h>

#include "internal.h"
#include "../bn/internal.h"
#include "../cpucap/internal.h"
#include "../../internal.h"
#include "../../test/abi_test.h"
#include "../../test/file_test.h"
#include "../../test/test_util.h"
#include "p256-nistz.h"

// Disable tests if BORINGSSL_SHARED_LIBRARY is defined. These tests need access
// to internal functions.
#if !defined(OPENSSL_NO_ASM) && !defined(MY_ASSEMBLER_IS_TOO_OLD_FOR_AVX) && \
    (defined(OPENSSL_X86_64) || defined(OPENSSL_AARCH64))                 && \
    !defined(OPENSSL_SMALL) && !defined(BORINGSSL_SHARED_LIBRARY)

TEST(P256_NistzTest, SelectW5) {
  // Fill a table with some garbage input.
  stack_align_type buffer_table[64 + (sizeof(P256_POINT) * 16)];
  P256_POINT *aligned_table = (P256_POINT *) align_pointer(buffer_table, 64);

  for (size_t i = 0; i < 16; i++) {
    OPENSSL_memset(aligned_table[i].X, static_cast<uint8_t>(3 * i), sizeof(aligned_table[i].X));
    OPENSSL_memset(aligned_table[i].Y, static_cast<uint8_t>(3 * i + 1),
                   sizeof(aligned_table[i].Y));
    OPENSSL_memset(aligned_table[i].Z, static_cast<uint8_t>(3 * i + 2),
                   sizeof(aligned_table[i].Z));
  }

  for (int i = 0; i <= 16; i++) {
    P256_POINT val;
    ecp_nistz256_select_w5(&val, aligned_table, i);

    P256_POINT expected;
    if (i == 0) {
      OPENSSL_memset(&expected, 0, sizeof(expected));
    } else {
      expected = aligned_table[i-1];
    }

    EXPECT_EQ(Bytes(reinterpret_cast<const char *>(&expected), sizeof(expected)),
              Bytes(reinterpret_cast<const char *>(&val), sizeof(val)));
  }

  // This is a constant-time function, so it is only necessary to instrument one
  // index for ABI checking.
  P256_POINT val;
  CHECK_ABI(ecp_nistz256_select_w5, &val, aligned_table, 7);
}

TEST(P256_NistzTest, SelectW7) {
  // Fill a table with some garbage input.
  stack_align_type buffer_table[64 + (sizeof(P256_POINT_AFFINE) * 64)];
  P256_POINT_AFFINE *aligned_table = (P256_POINT_AFFINE *) align_pointer(buffer_table, 64);

  for (size_t i = 0; i < 64; i++) {
    OPENSSL_memset(aligned_table[i].X, static_cast<uint8_t>(2 * i), sizeof(aligned_table[i].X));
    OPENSSL_memset(aligned_table[i].Y, static_cast<uint8_t>(2 * i + 1),
                   sizeof(aligned_table[i].Y));
  }

  for (int i = 0; i <= 64; i++) {
    P256_POINT_AFFINE val;
    ecp_nistz256_select_w7(&val, aligned_table, i);

    P256_POINT_AFFINE expected;
    if (i == 0) {
      OPENSSL_memset(&expected, 0, sizeof(expected));
    } else {
      expected = aligned_table[i-1];
    }

    EXPECT_EQ(Bytes(reinterpret_cast<const char *>(&expected), sizeof(expected)),
              Bytes(reinterpret_cast<const char *>(&val), sizeof(val)));
  }

  // This is a constant-time function, so it is only necessary to instrument one
  // index for ABI checking.
  P256_POINT_AFFINE val;
  CHECK_ABI(ecp_nistz256_select_w7, &val, aligned_table, 42);
}

TEST(P256_NistzTest, BEEU) {
#if defined(OPENSSL_X86_64)
  if (!CRYPTO_is_AVX_capable()) {
    // No AVX support; cannot run the BEEU code.
    return;
  }
#endif

  const EC_GROUP *group = EC_group_p256();
  BN_ULONG order_words[P256_LIMBS];
  ASSERT_TRUE(
      bn_copy_words(order_words, P256_LIMBS, EC_GROUP_get0_order(group)));

  BN_ULONG in[P256_LIMBS], out[P256_LIMBS];
  EC_SCALAR in_scalar, out_scalar, result;
  OPENSSL_memset(in, 0, sizeof(in));

  // Trying to find the inverse of zero should fail.
  ASSERT_FALSE(beeu_mod_inverse_vartime(out, in, order_words));
  // This is not a constant-time function, so instrument both zero and a few
  // inputs below.
  ASSERT_FALSE(CHECK_ABI(beeu_mod_inverse_vartime, out, in, order_words));

  // kOneMont is 1, in Montgomery form.
  static const BN_ULONG kOneMont[P256_LIMBS] = {
      TOBN(0xc46353d, 0x039cdaaf),
      TOBN(0x43190552, 0x58e8617b),
      0,
      0xffffffff,
  };

  for (BN_ULONG i = 1; i < 2000; i++) {
    SCOPED_TRACE(i);

    in[0] = i;
    if (i >= 1000) {
      in[1] = i << 8;
      in[2] = i << 32;
      in[3] = i << 48;
    } else {
      in[1] = in[2] = in[3] = 0;
    }

    EXPECT_TRUE(bn_less_than_words(in, order_words, P256_LIMBS));
    ASSERT_TRUE(beeu_mod_inverse_vartime(out, in, order_words));
    EXPECT_TRUE(bn_less_than_words(out, order_words, P256_LIMBS));

    // Calculate out*in and confirm that it equals one, modulo the order.
    OPENSSL_memcpy(in_scalar.words, in, sizeof(in));
    OPENSSL_memcpy(out_scalar.words, out, sizeof(out));
    ec_scalar_to_montgomery(group, &in_scalar, &in_scalar);
    ec_scalar_to_montgomery(group, &out_scalar, &out_scalar);
    ec_scalar_mul_montgomery(group, &result, &in_scalar, &out_scalar);

    EXPECT_EQ(0, OPENSSL_memcmp(kOneMont, &result, sizeof(kOneMont)));

    // Invert the result and expect to get back to the original value.
    ASSERT_TRUE(beeu_mod_inverse_vartime(out, out, order_words));
    EXPECT_EQ(0, OPENSSL_memcmp(in, out, sizeof(in)));

    if (i < 5) {
      EXPECT_TRUE(CHECK_ABI(beeu_mod_inverse_vartime, out, in, order_words));
    }
  }
}

static bool GetFieldElement(FileTest *t, BN_ULONG out[P256_LIMBS],
                            const char *name) {
  std::vector<uint8_t> bytes;
  if (!t->GetBytes(&bytes, name)) {
    return false;
  }

  if (bytes.size() != BN_BYTES * P256_LIMBS) {
    ADD_FAILURE() << "Invalid length: " << name;
    return false;
  }

  // |byte| contains bytes in big-endian while |out| should contain |BN_ULONG|s
  // in little-endian.
  OPENSSL_memset(out, 0, P256_LIMBS * sizeof(BN_ULONG));
  for (size_t i = 0; i < bytes.size(); i++) {
    out[P256_LIMBS - 1 - (i / BN_BYTES)] <<= 8;
    out[P256_LIMBS - 1 - (i / BN_BYTES)] |= bytes[i];
  }

  return true;
}

static std::string FieldElementToString(const BN_ULONG a[P256_LIMBS]) {
  std::string ret;
  for (size_t i = P256_LIMBS-1; i < P256_LIMBS; i--) {
    char buf[2 * BN_BYTES + 1];
    snprintf(buf, sizeof(buf), BN_HEX_FMT2, a[i]);
    ret += buf;
  }
  return ret;
}

static testing::AssertionResult ExpectFieldElementsEqual(
    const char *expected_expr, const char *actual_expr,
    const BN_ULONG expected[P256_LIMBS], const BN_ULONG actual[P256_LIMBS]) {
  if (OPENSSL_memcmp(expected, actual, sizeof(BN_ULONG) * P256_LIMBS) == 0) {
    return testing::AssertionSuccess();
  }

  return testing::AssertionFailure()
         << "Expected: " << FieldElementToString(expected) << " ("
         << expected_expr << ")\n"
         << "Actual:   " << FieldElementToString(actual) << " (" << actual_expr
         << ")";
}

#define EXPECT_FIELD_ELEMENTS_EQUAL(a, b) \
  EXPECT_PRED_FORMAT2(ExpectFieldElementsEqual, a, b)

static bool PointToAffine(P256_POINT_AFFINE *out, const P256_POINT *in) {
  static const uint8_t kP[] = {
      0xff, 0xff, 0xff, 0xff, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00,
      0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xff, 0xff,
      0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
  };

  bssl::UniquePtr<BIGNUM> x(BN_new()), y(BN_new()), z(BN_new());
  bssl::UniquePtr<BIGNUM> p(BN_bin2bn(kP, sizeof(kP), nullptr));
  if (!x || !y || !z || !p ||
      !bn_set_words(x.get(), in->X, P256_LIMBS) ||
      !bn_set_words(y.get(), in->Y, P256_LIMBS) ||
      !bn_set_words(z.get(), in->Z, P256_LIMBS)) {
    return false;
  }

  // Coordinates must be fully-reduced.
  if (BN_cmp(x.get(), p.get()) >= 0 ||
      BN_cmp(y.get(), p.get()) >= 0 ||
      BN_cmp(z.get(), p.get()) >= 0) {
    return false;
  }

  if (BN_is_zero(z.get())) {
    // The point at infinity is represented as (0, 0).
    OPENSSL_memset(out, 0, sizeof(P256_POINT_AFFINE));
    return true;
  }

  bssl::UniquePtr<BN_CTX> ctx(BN_CTX_new());
  bssl::UniquePtr<BN_MONT_CTX> mont(
      BN_MONT_CTX_new_for_modulus(p.get(), ctx.get()));
  if (!ctx || !mont ||
      // Invert Z.
      !BN_from_montgomery(z.get(), z.get(), mont.get(), ctx.get()) ||
      !BN_mod_inverse(z.get(), z.get(), p.get(), ctx.get()) ||
      !BN_to_montgomery(z.get(), z.get(), mont.get(), ctx.get()) ||
      // Convert (X, Y, Z) to (X/Z^2, Y/Z^3).
      !BN_mod_mul_montgomery(x.get(), x.get(), z.get(), mont.get(),
                             ctx.get()) ||
      !BN_mod_mul_montgomery(x.get(), x.get(), z.get(), mont.get(),
                             ctx.get()) ||
      !BN_mod_mul_montgomery(y.get(), y.get(), z.get(), mont.get(),
                             ctx.get()) ||
      !BN_mod_mul_montgomery(y.get(), y.get(), z.get(), mont.get(),
                             ctx.get()) ||
      !BN_mod_mul_montgomery(y.get(), y.get(), z.get(), mont.get(),
                             ctx.get()) ||
      !bn_copy_words(out->X, P256_LIMBS, x.get()) ||
      !bn_copy_words(out->Y, P256_LIMBS, y.get())) {
    return false;
  }
  return true;
}

static testing::AssertionResult ExpectPointsEqual(
    const char *expected_expr, const char *actual_expr,
    const P256_POINT_AFFINE *expected, const P256_POINT *actual) {
  // There are multiple representations of the same |P256_POINT|, so convert to
  // |P256_POINT_AFFINE| and compare.
  P256_POINT_AFFINE affine;
  if (!PointToAffine(&affine, actual)) {
    return testing::AssertionFailure()
           << "Could not convert " << actual_expr << " to affine: ("
           << FieldElementToString(actual->X) << ", "
           << FieldElementToString(actual->Y) << ", "
           << FieldElementToString(actual->Z) << ")";
  }

  if (OPENSSL_memcmp(expected, &affine, sizeof(P256_POINT_AFFINE)) != 0) {
    return testing::AssertionFailure()
           << "Expected: (" << FieldElementToString(expected->X) << ", "
           << FieldElementToString(expected->Y) << ") (" << expected_expr
           << "; affine)\n"
           << "Actual:   (" << FieldElementToString(affine.X) << ", "
           << FieldElementToString(affine.Y) << ") (" << actual_expr << ")";
  }

  return testing::AssertionSuccess();
}

#define EXPECT_POINTS_EQUAL(a, b) EXPECT_PRED_FORMAT2(ExpectPointsEqual, a, b)

static void TestNegate(FileTest *t) {
  BN_ULONG a[P256_LIMBS], b[P256_LIMBS];
  ASSERT_TRUE(GetFieldElement(t, a, "A"));
  ASSERT_TRUE(GetFieldElement(t, b, "B"));

  // Test that -A = B.
  BN_ULONG ret[P256_LIMBS];
  ecp_nistz256_neg(ret, a);
  EXPECT_FIELD_ELEMENTS_EQUAL(b, ret);

  OPENSSL_memcpy(ret, a, sizeof(ret));
  ecp_nistz256_neg(ret, ret /* a */);
  EXPECT_FIELD_ELEMENTS_EQUAL(b, ret);

  // Test that -B = A.
  ecp_nistz256_neg(ret, b);
  EXPECT_FIELD_ELEMENTS_EQUAL(a, ret);

  OPENSSL_memcpy(ret, b, sizeof(ret));
  ecp_nistz256_neg(ret, ret /* b */);
  EXPECT_FIELD_ELEMENTS_EQUAL(a, ret);
}

static void TestMulMont(FileTest *t) {
  BN_ULONG a[P256_LIMBS], b[P256_LIMBS], result[P256_LIMBS];
  ASSERT_TRUE(GetFieldElement(t, a, "A"));
  ASSERT_TRUE(GetFieldElement(t, b, "B"));
  ASSERT_TRUE(GetFieldElement(t, result, "Result"));

  BN_ULONG ret[P256_LIMBS];
  ecp_nistz256_mul_mont(ret, a, b);
  EXPECT_FIELD_ELEMENTS_EQUAL(result, ret);

  ecp_nistz256_mul_mont(ret, b, a);
  EXPECT_FIELD_ELEMENTS_EQUAL(result, ret);

  OPENSSL_memcpy(ret, a, sizeof(ret));
  ecp_nistz256_mul_mont(ret, ret /* a */, b);
  EXPECT_FIELD_ELEMENTS_EQUAL(result, ret);

  OPENSSL_memcpy(ret, a, sizeof(ret));
  ecp_nistz256_mul_mont(ret, b, ret);
  EXPECT_FIELD_ELEMENTS_EQUAL(result, ret);

  OPENSSL_memcpy(ret, b, sizeof(ret));
  ecp_nistz256_mul_mont(ret, a, ret /* b */);
  EXPECT_FIELD_ELEMENTS_EQUAL(result, ret);

  OPENSSL_memcpy(ret, b, sizeof(ret));
  ecp_nistz256_mul_mont(ret, ret /* b */, a);
  EXPECT_FIELD_ELEMENTS_EQUAL(result, ret);

  if (OPENSSL_memcmp(a, b, sizeof(a)) == 0) {
    ecp_nistz256_sqr_mont(ret, a);
    EXPECT_FIELD_ELEMENTS_EQUAL(result, ret);

    OPENSSL_memcpy(ret, a, sizeof(ret));
    ecp_nistz256_sqr_mont(ret, ret /* a */);
    EXPECT_FIELD_ELEMENTS_EQUAL(result, ret);
  }
}

static void TestFromMont(FileTest *t) {
  BN_ULONG a[P256_LIMBS], result[P256_LIMBS];
  ASSERT_TRUE(GetFieldElement(t, a, "A"));
  ASSERT_TRUE(GetFieldElement(t, result, "Result"));

  BN_ULONG ret[P256_LIMBS];
  ecp_nistz256_from_mont(ret, a);
  EXPECT_FIELD_ELEMENTS_EQUAL(result, ret);

  OPENSSL_memcpy(ret, a, sizeof(ret));
  ecp_nistz256_from_mont(ret, ret /* a */);
  EXPECT_FIELD_ELEMENTS_EQUAL(result, ret);
}

static void TestPointAdd(FileTest *t) {
  P256_POINT a, b;
  P256_POINT_AFFINE result;
  ASSERT_TRUE(GetFieldElement(t, a.X, "A.X"));
  ASSERT_TRUE(GetFieldElement(t, a.Y, "A.Y"));
  ASSERT_TRUE(GetFieldElement(t, a.Z, "A.Z"));
  ASSERT_TRUE(GetFieldElement(t, b.X, "B.X"));
  ASSERT_TRUE(GetFieldElement(t, b.Y, "B.Y"));
  ASSERT_TRUE(GetFieldElement(t, b.Z, "B.Z"));
  ASSERT_TRUE(GetFieldElement(t, result.X, "Result.X"));
  ASSERT_TRUE(GetFieldElement(t, result.Y, "Result.Y"));

  P256_POINT ret;
  ecp_nistz256_point_add(&ret, &a, &b);
  EXPECT_POINTS_EQUAL(&result, &ret);

  ecp_nistz256_point_add(&ret, &b, &a);
  EXPECT_POINTS_EQUAL(&result, &ret);

  OPENSSL_memcpy(&ret, &a, sizeof(ret));
  ecp_nistz256_point_add(&ret, &ret /* a */, &b);
  EXPECT_POINTS_EQUAL(&result, &ret);

  OPENSSL_memcpy(&ret, &a, sizeof(ret));
  ecp_nistz256_point_add(&ret, &b, &ret /* a */);
  EXPECT_POINTS_EQUAL(&result, &ret);

  OPENSSL_memcpy(&ret, &b, sizeof(ret));
  ecp_nistz256_point_add(&ret, &a, &ret /* b */);
  EXPECT_POINTS_EQUAL(&result, &ret);

  OPENSSL_memcpy(&ret, &b, sizeof(ret));
  ecp_nistz256_point_add(&ret, &ret /* b */, &a);
  EXPECT_POINTS_EQUAL(&result, &ret);

  P256_POINT_AFFINE a_affine, b_affine, infinity;
  OPENSSL_memset(&infinity, 0, sizeof(infinity));
  ASSERT_TRUE(PointToAffine(&a_affine, &a));
  ASSERT_TRUE(PointToAffine(&b_affine, &b));

  // ecp_nistz256_point_add_affine does not work when a == b unless doubling the
  // point at infinity.
  if (OPENSSL_memcmp(&a_affine, &b_affine, sizeof(a_affine)) != 0 ||
      OPENSSL_memcmp(&a_affine, &infinity, sizeof(a_affine)) == 0) {
    ecp_nistz256_point_add_affine(&ret, &a, &b_affine);
    EXPECT_POINTS_EQUAL(&result, &ret);

    OPENSSL_memcpy(&ret, &a, sizeof(ret));
    ecp_nistz256_point_add_affine(&ret, &ret /* a */, &b_affine);
    EXPECT_POINTS_EQUAL(&result, &ret);

    ecp_nistz256_point_add_affine(&ret, &b, &a_affine);
    EXPECT_POINTS_EQUAL(&result, &ret);

    OPENSSL_memcpy(&ret, &b, sizeof(ret));
    ecp_nistz256_point_add_affine(&ret, &ret /* b */, &a_affine);
    EXPECT_POINTS_EQUAL(&result, &ret);
  }

  if (OPENSSL_memcmp(&a, &b, sizeof(a)) == 0) {
    ecp_nistz256_point_double(&ret, &a);
    EXPECT_POINTS_EQUAL(&result, &ret);

    ret = a;
    ecp_nistz256_point_double(&ret, &ret /* a */);
    EXPECT_POINTS_EQUAL(&result, &ret);
  }
}

static void TestOrdMulMont(FileTest *t) {
  // This test works on scalars rather than field elements, but the
  // representation is the same.
  BN_ULONG a[P256_LIMBS], b[P256_LIMBS], result[P256_LIMBS];
  ASSERT_TRUE(GetFieldElement(t, a, "A"));
  ASSERT_TRUE(GetFieldElement(t, b, "B"));
  ASSERT_TRUE(GetFieldElement(t, result, "Result"));

  BN_ULONG ret[P256_LIMBS];
  ecp_nistz256_ord_mul_mont(ret, a, b);
  EXPECT_FIELD_ELEMENTS_EQUAL(result, ret);

  ecp_nistz256_ord_mul_mont(ret, b, a);
  EXPECT_FIELD_ELEMENTS_EQUAL(result, ret);

  OPENSSL_memcpy(ret, a, sizeof(ret));
  ecp_nistz256_ord_mul_mont(ret, ret /* a */, b);
  EXPECT_FIELD_ELEMENTS_EQUAL(result, ret);

  OPENSSL_memcpy(ret, a, sizeof(ret));
  ecp_nistz256_ord_mul_mont(ret, b, ret);
  EXPECT_FIELD_ELEMENTS_EQUAL(result, ret);

  OPENSSL_memcpy(ret, b, sizeof(ret));
  ecp_nistz256_ord_mul_mont(ret, a, ret /* b */);
  EXPECT_FIELD_ELEMENTS_EQUAL(result, ret);

  OPENSSL_memcpy(ret, b, sizeof(ret));
  ecp_nistz256_ord_mul_mont(ret, ret /* b */, a);
  EXPECT_FIELD_ELEMENTS_EQUAL(result, ret);

  if (OPENSSL_memcmp(a, b, sizeof(a)) == 0) {
    ecp_nistz256_ord_sqr_mont(ret, a, 1);
    EXPECT_FIELD_ELEMENTS_EQUAL(result, ret);

    OPENSSL_memcpy(ret, a, sizeof(ret));
    ecp_nistz256_ord_sqr_mont(ret, ret /* a */, 1);
    EXPECT_FIELD_ELEMENTS_EQUAL(result, ret);
  }
}

TEST(P256_NistzTest, TestVectors) {
  return FileTestGTest("crypto/fipsmodule/ec/p256-nistz_tests.txt",
                       [](FileTest *t) {
    if (t->GetParameter() == "Negate") {
      TestNegate(t);
    } else if (t->GetParameter() == "MulMont") {
      TestMulMont(t);
    } else if (t->GetParameter() == "FromMont") {
      TestFromMont(t);
    } else if (t->GetParameter() == "PointAdd") {
      TestPointAdd(t);
    } else if (t->GetParameter() == "OrdMulMont") {
      TestOrdMulMont(t);
    } else {
      FAIL() << "Unknown test type:" << t->GetParameter();
    }
  });
}

// Instrument the functions covered in TestVectors for ABI checking.
TEST(P256_NistzTest, ABI) {
  BN_ULONG a[P256_LIMBS], b[P256_LIMBS], c[P256_LIMBS];
  OPENSSL_memset(a, 0x01, sizeof(a));
  // These functions are all constant-time, so it is only necessary to
  // instrument one call each for ABI checking.
  CHECK_ABI(ecp_nistz256_neg, b, a);
  CHECK_ABI(ecp_nistz256_mul_mont, c, a, b);
  CHECK_ABI(ecp_nistz256_sqr_mont, c, a);
  CHECK_ABI(ecp_nistz256_from_mont, c, a);
  CHECK_ABI(ecp_nistz256_ord_mul_mont, c, a, b);

  // Check a few different loop counts.
  CHECK_ABI(ecp_nistz256_ord_sqr_mont, b, a, 1);
  CHECK_ABI(ecp_nistz256_ord_sqr_mont, b, a, 3);

  // Point addition has some special cases around infinity and doubling. Test a
  // few different scenarios.
  static const P256_POINT kA = {
      {TOBN(0x60559ac7, 0xc8d0d89d), TOBN(0x6cda3400, 0x545f7e2c),
       TOBN(0x9b5159e0, 0x323e6048), TOBN(0xcb8dea33, 0x27057fe6)},
      {TOBN(0x81a2d3bc, 0xc93a2d53), TOBN(0x81f40762, 0xa4f33ccf),
       TOBN(0xc3c3300a, 0xa8ad50ea), TOBN(0x553de89b, 0x31719830)},
      {TOBN(0x3fd9470f, 0xb277d181), TOBN(0xc191b8d5, 0x6376f206),
       TOBN(0xb2572c1f, 0x45eda26f), TOBN(0x4589e40d, 0xf2efc546)},
  };
  static const P256_POINT kB = {
      {TOBN(0x3cf0b0aa, 0x92054341), TOBN(0xb949bb80, 0xdab57807),
       TOBN(0x99de6814, 0xefd21b3e), TOBN(0x32ad5649, 0x7c6c6e83)},
      {TOBN(0x06afaa02, 0x688399e0), TOBN(0x75f2d096, 0x2a3ce65c),
       TOBN(0xf6a31eb7, 0xca0244b3), TOBN(0x57b33b7a, 0xcfeee75e)},
      {TOBN(0x7617d2e0, 0xb4f1d35f), TOBN(0xa922cb10, 0x7f592b65),
       TOBN(0x12fd6c7a, 0x51a2f474), TOBN(0x337d5e1e, 0xc2fc711b)},
  };
  // This file represents Jacobian infinity as (*, *, 0).
  static const P256_POINT kInfinity = {
      {TOBN(0, 0), TOBN(0, 0), TOBN(0, 0), TOBN(0, 0)},
      {TOBN(0, 0), TOBN(0, 0), TOBN(0, 0), TOBN(0, 0)},
      {TOBN(0, 0), TOBN(0, 0), TOBN(0, 0), TOBN(0, 0)},
  };

  P256_POINT p;
  CHECK_ABI(ecp_nistz256_point_add, &p, &kA, &kB);
  CHECK_ABI(ecp_nistz256_point_add, &p, &kA, &kA);
  OPENSSL_memcpy(&p, &kA, sizeof(P256_POINT));
  ecp_nistz256_neg(p.Y, p.Y);
  CHECK_ABI(ecp_nistz256_point_add, &p, &kA, &p);  // A + -A
  CHECK_ABI(ecp_nistz256_point_add, &p, &kA, &kInfinity);
  CHECK_ABI(ecp_nistz256_point_add, &p, &kInfinity, &kA);
  CHECK_ABI(ecp_nistz256_point_add, &p, &kInfinity, &kInfinity);
  CHECK_ABI(ecp_nistz256_point_double, &p, &kA);
  CHECK_ABI(ecp_nistz256_point_double, &p, &kInfinity);

  static const P256_POINT_AFFINE kC = {
      {TOBN(0x7e3ad339, 0xfb3fa5f0), TOBN(0x559d669d, 0xe3a047b2),
       TOBN(0x8883b298, 0x7042e595), TOBN(0xfabada65, 0x7e477f08)},
      {TOBN(0xd9cfceb8, 0xda1c3e85), TOBN(0x80863761, 0x0ce6d6bc),
       TOBN(0xa8409d84, 0x66034f02), TOBN(0x05519925, 0x31a68d55)},
  };
  // This file represents affine infinity as (0, 0).
  static const P256_POINT_AFFINE kInfinityAffine = {
    {TOBN(0, 0), TOBN(0, 0), TOBN(0, 0), TOBN(0, 0)},
    {TOBN(0, 0), TOBN(0, 0), TOBN(0, 0), TOBN(0, 0)},
  };

  CHECK_ABI(ecp_nistz256_point_add_affine, &p, &kA, &kC);
  CHECK_ABI(ecp_nistz256_point_add_affine, &p, &kA, &kInfinityAffine);
  CHECK_ABI(ecp_nistz256_point_add_affine, &p, &kInfinity, &kInfinityAffine);
  CHECK_ABI(ecp_nistz256_point_add_affine, &p, &kInfinity, &kC);
}

#endif /* !defined(OPENSSL_NO_ASM) && !defined(MY_ASSEMBLER_IS_TOO_OLD_FOR_AVX) && \
          (defined(OPENSSL_X86_64) || defined(OPENSSL_AARCH64)) &&  \
          !defined(OPENSSL_SMALL) && !defined(BORINGSSL_SHARED_LIBRARY) */
