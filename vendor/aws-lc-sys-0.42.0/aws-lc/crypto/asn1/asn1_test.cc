// Copyright (c) 2016, Google Inc.
// SPDX-License-Identifier: ISC

#include <limits.h>
#include <stdio.h>

#include <map>
#include <string>
#include <vector>

#include <gtest/gtest.h>

#include <openssl/asn1.h>
#include <openssl/asn1t.h>
#include <openssl/bio.h>
#include <openssl/bytestring.h>
#include <openssl/err.h>
#include <openssl/mem.h>
#include <openssl/obj.h>
#include <openssl/span.h>
#include <openssl/x509v3.h>

#include "../test/test_util.h"
#include "internal.h"

#if defined(OPENSSL_THREADS)
#include <thread>
#endif


// kTag128 is an ASN.1 structure with a universal tag with number 128.
static const uint8_t kTag128[] = {
    0x1f, 0x81, 0x00, 0x01, 0x00,
};

// kTag258 is an ASN.1 structure with a universal tag with number 258.
static const uint8_t kTag258[] = {
    0x1f, 0x82, 0x02, 0x01, 0x00,
};

static_assert(V_ASN1_NEG_INTEGER == 258,
              "V_ASN1_NEG_INTEGER changed. Update kTag258 to collide with it.");

// kTagOverflow is an ASN.1 structure with a universal tag with number 2^35-1,
// which will not fit in an int.
static const uint8_t kTagOverflow[] = {
    0x1f, 0xff, 0xff, 0xff, 0xff, 0x7f, 0x01, 0x00,
};

TEST(ASN1Test, LargeTags) {
  const uint8_t *p = kTag258;
  bssl::UniquePtr<ASN1_TYPE> obj(d2i_ASN1_TYPE(NULL, &p, sizeof(kTag258)));
  EXPECT_FALSE(obj) << "Parsed value with illegal tag" << obj->type;
  ERR_clear_error();

  p = kTagOverflow;
  obj.reset(d2i_ASN1_TYPE(NULL, &p, sizeof(kTagOverflow)));
  EXPECT_FALSE(obj) << "Parsed value with tag overflow" << obj->type;
  ERR_clear_error();

  p = kTag128;
  obj.reset(d2i_ASN1_TYPE(NULL, &p, sizeof(kTag128)));
  ASSERT_TRUE(obj);
  EXPECT_EQ(128, obj->type);
  const uint8_t kZero = 0;
  EXPECT_EQ(Bytes(&kZero, 1), Bytes(obj->value.asn1_string->data,
                                    obj->value.asn1_string->length));
}

// |obj| and |i2d_func| require different template parameters because C++ may
// deduce, say, |ASN1_STRING*| via |obj| and |const ASN1_STRING*| via
// |i2d_func|. Template argument deduction then fails. The language is not able
// to resolve this by observing that |const ASN1_STRING*| works for both.
template <typename T, typename U>
void TestSerialize(T obj, int (*i2d_func)(U a, uint8_t **pp),
                   bssl::Span<const uint8_t> expected) {
  static_assert(std::is_convertible<T, U>::value,
                "incompatible parameter to i2d_func");
  // Test the allocating version first. It is easiest to debug.
  uint8_t *ptr = nullptr;
  int len = i2d_func(obj, &ptr);
  ASSERT_GT(len, 0);
  EXPECT_EQ(Bytes(expected), Bytes(ptr, len));
  OPENSSL_free(ptr);

  len = i2d_func(obj, nullptr);
  ASSERT_GT(len, 0);
  EXPECT_EQ(len, static_cast<int>(expected.size()));

  std::vector<uint8_t> buf(len);
  ptr = buf.data();
  len = i2d_func(obj, &ptr);
  ASSERT_EQ(len, static_cast<int>(expected.size()));
  EXPECT_EQ(ptr, buf.data() + buf.size());
  EXPECT_EQ(Bytes(expected), Bytes(buf));
}

static bssl::UniquePtr<BIGNUM> BIGNUMPow2(unsigned bit) {
  bssl::UniquePtr<BIGNUM> bn(BN_new());
  if (!bn ||
      !BN_set_bit(bn.get(), bit)) {
    return nullptr;
  }
  return bn;
}

TEST(ASN1Test, Integer) {
  bssl::UniquePtr<BIGNUM> int64_min = BIGNUMPow2(63);
  ASSERT_TRUE(int64_min);
  BN_set_negative(int64_min.get(), 1);

  bssl::UniquePtr<BIGNUM> int64_max = BIGNUMPow2(63);
  ASSERT_TRUE(int64_max);
  ASSERT_TRUE(BN_sub_word(int64_max.get(), 1));

  bssl::UniquePtr<BIGNUM> int32_min = BIGNUMPow2(31);
  ASSERT_TRUE(int32_min);
  BN_set_negative(int32_min.get(), 1);

  bssl::UniquePtr<BIGNUM> int32_max = BIGNUMPow2(31);
  ASSERT_TRUE(int32_max);
  ASSERT_TRUE(BN_sub_word(int32_max.get(), 1));

  struct {
    // der is the DER encoding of the INTEGER, including the tag and length.
    std::vector<uint8_t> der;
    // type and data are the corresponding fields of the |ASN1_STRING|
    // representation.
    int type;
    std::vector<uint8_t> data;
    // bn_asc is the |BIGNUM| representation, as parsed by the |BN_asc2bn|
    // function.
    const char *bn_asc;
  } kTests[] = {
      // -2^64 - 1
      {{0x02, 0x09, 0xfe, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff},
       V_ASN1_NEG_INTEGER,
       {0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01},
       "-0x10000000000000001"},
      // -2^64
      {{0x02, 0x09, 0xff, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00},
       V_ASN1_NEG_INTEGER,
       {0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00},
       "-0x10000000000000000"},
      // -2^64 + 1
      {{0x02, 0x09, 0xff, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01},
       V_ASN1_NEG_INTEGER,
       {0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff},
       "-0xffffffffffffffff"},
      // -2^63 - 1
      {{0x02, 0x09, 0xff, 0x7f, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff},
       V_ASN1_NEG_INTEGER,
       {0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01},
       "-0x8000000000000001"},
      // -2^63 (INT64_MIN)
      {{0x02, 0x08, 0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00},
       V_ASN1_NEG_INTEGER,
       {0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00},
       "-0x8000000000000000"},
      // -2^63 + 1
      {{0x02, 0x08, 0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01},
       V_ASN1_NEG_INTEGER,
       {0x7f, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff},
       "-0x7fffffffffffffff"},
      // -2^32 - 1
      {{0x02, 0x05, 0xfe, 0xff, 0xff, 0xff, 0xff},
       V_ASN1_NEG_INTEGER,
       {0x01, 0x00, 0x00, 0x00, 0x01},
       "-0x100000001"},
      // -2^32
      {{0x02, 0x05, 0xff, 0x00, 0x00, 0x00, 0x00},
       V_ASN1_NEG_INTEGER,
       {0x01, 0x00, 0x00, 0x00, 0x00},
       "-0x100000000"},
      // -2^32 + 1
      {{0x02, 0x05, 0xff, 0x00, 0x00, 0x00, 0x01},
       V_ASN1_NEG_INTEGER,
       {0xff, 0xff, 0xff, 0xff},
       "-0xffffffff"},
      // -2^31 - 1
      {{0x02, 0x05, 0xff, 0x7f, 0xff, 0xff, 0xff},
       V_ASN1_NEG_INTEGER,
       {0x80, 0x00, 0x00, 0x01},
       "-0x80000001"},
      // -2^31 (INT32_MIN)
      {{0x02, 0x04, 0x80, 0x00, 0x00, 0x00},
       V_ASN1_NEG_INTEGER,
       {0x80, 0x00, 0x00, 0x00},
       "-0x80000000"},
      // -2^31 + 1
      {{0x02, 0x04, 0x80, 0x00, 0x00, 0x01},
       V_ASN1_NEG_INTEGER,
       {0x7f, 0xff, 0xff, 0xff},
       "-0x7fffffff"},
      // -257
      {{0x02, 0x02, 0xfe, 0xff}, V_ASN1_NEG_INTEGER, {0x01, 0x01}, "-257"},
      // -256
      {{0x02, 0x02, 0xff, 0x00}, V_ASN1_NEG_INTEGER, {0x01, 0x00}, "-256"},
      // -255
      {{0x02, 0x02, 0xff, 0x01}, V_ASN1_NEG_INTEGER, {0xff}, "-255"},
      // -129
      {{0x02, 0x02, 0xff, 0x7f}, V_ASN1_NEG_INTEGER, {0x81}, "-129"},
      // -128
      {{0x02, 0x01, 0x80}, V_ASN1_NEG_INTEGER, {0x80}, "-128"},
      // -127
      {{0x02, 0x01, 0x81}, V_ASN1_NEG_INTEGER, {0x7f}, "-127"},
      // -1
      {{0x02, 0x01, 0xff}, V_ASN1_NEG_INTEGER, {0x01}, "-1"},
      // 0
      {{0x02, 0x01, 0x00}, V_ASN1_INTEGER, {}, "0"},
      // 1
      {{0x02, 0x01, 0x01}, V_ASN1_INTEGER, {0x01}, "1"},
      // 127
      {{0x02, 0x01, 0x7f}, V_ASN1_INTEGER, {0x7f}, "127"},
      // 128
      {{0x02, 0x02, 0x00, 0x80}, V_ASN1_INTEGER, {0x80}, "128"},
      // 129
      {{0x02, 0x02, 0x00, 0x81}, V_ASN1_INTEGER, {0x81}, "129"},
      // 255
      {{0x02, 0x02, 0x00, 0xff}, V_ASN1_INTEGER, {0xff}, "255"},
      // 256
      {{0x02, 0x02, 0x01, 0x00}, V_ASN1_INTEGER, {0x01, 0x00}, "256"},
      // 257
      {{0x02, 0x02, 0x01, 0x01}, V_ASN1_INTEGER, {0x01, 0x01}, "257"},
      // 2^31 - 2
      {{0x02, 0x04, 0x7f, 0xff, 0xff, 0xfe},
       V_ASN1_INTEGER,
       {0x7f, 0xff, 0xff, 0xfe},
       "0x7ffffffe"},
      // 2^31 - 1 (INT32_MAX)
      {{0x02, 0x04, 0x7f, 0xff, 0xff, 0xff},
       V_ASN1_INTEGER,
       {0x7f, 0xff, 0xff, 0xff},
       "0x7fffffff"},
      // 2^31
      {{0x02, 0x05, 0x00, 0x80, 0x00, 0x00, 0x00},
       V_ASN1_INTEGER,
       {0x80, 0x00, 0x00, 0x00},
       "0x80000000"},
      // 2^32 - 2
      {{0x02, 0x05, 0x00, 0xff, 0xff, 0xff, 0xfe},
       V_ASN1_INTEGER,
       {0xff, 0xff, 0xff, 0xfe},
       "0xfffffffe"},
      // 2^32 - 1 (UINT32_MAX)
      {{0x02, 0x05, 0x00, 0xff, 0xff, 0xff, 0xff},
       V_ASN1_INTEGER,
       {0xff, 0xff, 0xff, 0xff},
       "0xffffffff"},
      // 2^32
      {{0x02, 0x05, 0x01, 0x00, 0x00, 0x00, 0x00},
       V_ASN1_INTEGER,
       {0x01, 0x00, 0x00, 0x00, 0x00},
       "0x100000000"},
      // 2^63 - 2
      {{0x02, 0x08, 0x7f, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xfe},
       V_ASN1_INTEGER,
       {0x7f, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xfe},
       "0x7ffffffffffffffe"},
      // 2^63 - 1 (INT64_MAX)
      {{0x02, 0x08, 0x7f, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff},
       V_ASN1_INTEGER,
       {0x7f, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff},
       "0x7fffffffffffffff"},
      // 2^63
      {{0x02, 0x09, 0x00, 0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00},
       V_ASN1_INTEGER,
       {0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00},
       "0x8000000000000000"},
      // 2^64 - 2
      {{0x02, 0x09, 0x00, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xfe},
       V_ASN1_INTEGER,
       {0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xfe},
       "0xfffffffffffffffe"},
      // 2^64 - 1 (UINT64_MAX)
      {{0x02, 0x09, 0x00, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff},
       V_ASN1_INTEGER,
       {0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff},
       "0xffffffffffffffff"},
      // 2^64
      {{0x02, 0x09, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00},
       V_ASN1_INTEGER,
       {0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00},
       "0x10000000000000000"},
      // 2^64 + 1
      {{0x02, 0x09, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01},
       V_ASN1_INTEGER,
       {0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01},
       "0x10000000000000001"},
  };

  for (const auto &t : kTests) {
    SCOPED_TRACE(t.bn_asc);
    // Collect a map of different ways to construct the integer. The key is the
    // method used and is only retained to aid debugging.
    std::map<std::string, bssl::UniquePtr<ASN1_INTEGER>> objs;

    // Construct |ASN1_INTEGER| by setting the type and data manually.
    bssl::UniquePtr<ASN1_INTEGER> by_data(ASN1_STRING_type_new(t.type));
    ASSERT_TRUE(by_data);
    ASSERT_TRUE(ASN1_STRING_set(by_data.get(), t.data.data(), t.data.size()));
    objs["data"] = std::move(by_data);

    // Construct |ASN1_INTEGER| from a |BIGNUM|.
    BIGNUM *bn_raw = nullptr;
    ASSERT_TRUE(BN_asc2bn(&bn_raw, t.bn_asc));
    bssl::UniquePtr<BIGNUM> bn(bn_raw);
    bssl::UniquePtr<ASN1_INTEGER> by_bn(BN_to_ASN1_INTEGER(bn.get(), nullptr));
    ASSERT_TRUE(by_bn);
    objs["bn"] = std::move(by_bn);

    // Construct |ASN1_INTEGER| from decoding.
    const uint8_t *ptr = t.der.data();
    bssl::UniquePtr<ASN1_INTEGER> by_der(
        d2i_ASN1_INTEGER(nullptr, &ptr, t.der.size()));
    ASSERT_TRUE(by_der);
    EXPECT_EQ(ptr, t.der.data() + t.der.size());
    objs["der"] = std::move(by_der);

    // Construct |ASN1_INTEGER| from various C types, if it fits.
    bool fits_in_long = false, fits_in_i64 = false, fits_in_u64 = false;
    uint64_t u64 = 0;
    int64_t i64 = 0;
    long l = 0;
    uint64_t abs_u64;
    if (BN_get_u64(bn.get(), &abs_u64)) {
      fits_in_u64 = !BN_is_negative(bn.get());
      if (fits_in_u64) {
        u64 = abs_u64;
        bssl::UniquePtr<ASN1_INTEGER> by_u64(ASN1_INTEGER_new());
        ASSERT_TRUE(by_u64);
        ASSERT_TRUE(ASN1_INTEGER_set_uint64(by_u64.get(), u64));
        objs["u64"] = std::move(by_u64);
      }

      fits_in_i64 = BN_cmp(int64_min.get(), bn.get()) <= 0 &&
                    BN_cmp(bn.get(), int64_max.get()) <= 0;
      if (fits_in_i64) {
        if (BN_is_negative(bn.get())) {
          i64 = static_cast<int64_t>(0u - abs_u64);
        } else {
          i64 = static_cast<int64_t>(abs_u64);
        }
        bssl::UniquePtr<ASN1_INTEGER> by_i64(ASN1_INTEGER_new());
        ASSERT_TRUE(by_i64);
        ASSERT_TRUE(ASN1_INTEGER_set_int64(by_i64.get(), i64));
        objs["i64"] = std::move(by_i64);
      }

      if (sizeof(long) == 8) {
        fits_in_long = fits_in_i64;
      } else {
        ASSERT_EQ(4u, sizeof(long));
        fits_in_long = BN_cmp(int32_min.get(), bn.get()) <= 0 &&
                       BN_cmp(bn.get(), int32_max.get()) <= 0;
      }
      if (fits_in_long) {
        l = static_cast<long>(i64);
        bssl::UniquePtr<ASN1_INTEGER> by_long(ASN1_INTEGER_new());
        ASSERT_TRUE(by_long);
        ASSERT_TRUE(ASN1_INTEGER_set(by_long.get(), l));
        objs["long"] = std::move(by_long);
      }
    }

    // Default construction should return the zero |ASN1_INTEGER|.
    if (BN_is_zero(bn.get())) {
      bssl::UniquePtr<ASN1_INTEGER> by_default(ASN1_INTEGER_new());
      ASSERT_TRUE(by_default);
      objs["default"] = std::move(by_default);
    }

    // Test that every |ASN1_INTEGER| constructed behaves as expected.
    for (const auto &pair : objs) {
      // The fields should be as expected.
      SCOPED_TRACE(pair.first);
      const ASN1_INTEGER *obj = pair.second.get();
      EXPECT_EQ(t.type, ASN1_STRING_type(obj));
      EXPECT_EQ(Bytes(t.data), Bytes(ASN1_STRING_get0_data(obj),
                                     ASN1_STRING_length(obj)));

      // The object should encode correctly.
      TestSerialize(obj, i2d_ASN1_INTEGER, t.der);

      bssl::UniquePtr<BIGNUM> bn2(ASN1_INTEGER_to_BN(obj, nullptr));
      ASSERT_TRUE(bn2);
      EXPECT_EQ(0, BN_cmp(bn.get(), bn2.get()));

      if (fits_in_u64) {
        uint64_t v;
        ASSERT_TRUE(ASN1_INTEGER_get_uint64(&v, obj));
        EXPECT_EQ(v, u64);
      } else {
        uint64_t v;
        EXPECT_FALSE(ASN1_INTEGER_get_uint64(&v, obj));
      }

      if (fits_in_i64) {
        int64_t v;
        ASSERT_TRUE(ASN1_INTEGER_get_int64(&v, obj));
        EXPECT_EQ(v, i64);
      } else {
        int64_t v;
        EXPECT_FALSE(ASN1_INTEGER_get_int64(&v, obj));
      }

      if (fits_in_long) {
        EXPECT_EQ(l, ASN1_INTEGER_get(obj));
      } else {
        EXPECT_EQ(-1, ASN1_INTEGER_get(obj));
      }

      // All variations of integers should compare as equal to each other, as
      // strings or integers. (Functions like |ASN1_TYPE_cmp| rely on
      // string-based comparison.)
      for (const auto &pair2 : objs) {
        SCOPED_TRACE(pair2.first);
        EXPECT_EQ(0, ASN1_INTEGER_cmp(obj, pair2.second.get()));
        EXPECT_EQ(0, ASN1_STRING_cmp(obj, pair2.second.get()));
      }
    }

    // Although our parsers will never output non-minimal |ASN1_INTEGER|s, it is
    // possible to construct them manually. They should encode correctly.
    std::vector<uint8_t> data = t.data;
    const int kMaxExtraBytes = 5;
    for (int i = 0; i < kMaxExtraBytes; i++) {
      data.insert(data.begin(), 0x00);
      SCOPED_TRACE(Bytes(data));

      bssl::UniquePtr<ASN1_INTEGER> non_minimal(ASN1_STRING_type_new(t.type));
      ASSERT_TRUE(non_minimal);
      ASSERT_TRUE(ASN1_STRING_set(non_minimal.get(), data.data(), data.size()));

      TestSerialize(non_minimal.get(), i2d_ASN1_INTEGER, t.der);
    }
  }

  for (size_t i = 0; i < OPENSSL_ARRAY_SIZE(kTests); i++) {
    SCOPED_TRACE(Bytes(kTests[i].der));
    const uint8_t *ptr = kTests[i].der.data();
    bssl::UniquePtr<ASN1_INTEGER> a(
        d2i_ASN1_INTEGER(nullptr, &ptr, kTests[i].der.size()));
    ASSERT_TRUE(a);
    for (size_t j = 0; j < OPENSSL_ARRAY_SIZE(kTests); j++) {
      SCOPED_TRACE(Bytes(kTests[j].der));
      ptr = kTests[j].der.data();
      bssl::UniquePtr<ASN1_INTEGER> b(
          d2i_ASN1_INTEGER(nullptr, &ptr, kTests[j].der.size()));
      ASSERT_TRUE(b);

      // |ASN1_INTEGER_cmp| should compare numerically. |ASN1_STRING_cmp| does
      // not but should preserve equality.
      if (i < j) {
        EXPECT_LT(ASN1_INTEGER_cmp(a.get(), b.get()), 0);
        EXPECT_NE(ASN1_STRING_cmp(a.get(), b.get()), 0);
      } else if (i > j) {
        EXPECT_GT(ASN1_INTEGER_cmp(a.get(), b.get()), 0);
        EXPECT_NE(ASN1_STRING_cmp(a.get(), b.get()), 0);
      } else {
        EXPECT_EQ(ASN1_INTEGER_cmp(a.get(), b.get()), 0);
        EXPECT_EQ(ASN1_STRING_cmp(a.get(), b.get()), 0);
      }
    }
  }

  std::vector<uint8_t> kInvalidTests[] = {
      // The empty string is not an integer.
      {0x02, 0x00},
      // Integers must be minimally-encoded.
      {0x02, 0x02, 0x00, 0x00},
      {0x02, 0x02, 0x00, 0x7f},
      {0x02, 0x02, 0xff, 0xff},
      {0x02, 0x02, 0xff, 0x80},
  };
  for (const auto &invalid : kInvalidTests) {
    SCOPED_TRACE(Bytes(invalid));

    const uint8_t *ptr = invalid.data();
    bssl::UniquePtr<ASN1_INTEGER> integer(
        d2i_ASN1_INTEGER(nullptr, &ptr, invalid.size()));
    EXPECT_FALSE(integer);
  }

  // Callers expect |ASN1_INTEGER_get| and |ASN1_ENUMERATED_get| to return zero
  // given NULL.
  EXPECT_EQ(0, ASN1_INTEGER_get(nullptr));
  EXPECT_EQ(0, ASN1_ENUMERATED_get(nullptr));
}

// Although invalid, a negative zero should encode correctly.
TEST(ASN1Test, NegativeZero) {
  bssl::UniquePtr<ASN1_INTEGER> neg_zero(
      ASN1_STRING_type_new(V_ASN1_NEG_INTEGER));
  ASSERT_TRUE(neg_zero);
  EXPECT_EQ(0, ASN1_INTEGER_get(neg_zero.get()));

  static const uint8_t kDER[] = {0x02, 0x01, 0x00};
  TestSerialize(neg_zero.get(), i2d_ASN1_INTEGER, kDER);
}

TEST(ASN1Test, SerializeObject) {
  static const uint8_t kDER[] = {0x06, 0x09, 0x2a, 0x86, 0x48, 0x86,
                                 0xf7, 0x0d, 0x01, 0x01, 0x01};
  const ASN1_OBJECT *obj = OBJ_nid2obj(NID_rsaEncryption);
  TestSerialize(obj, i2d_ASN1_OBJECT, kDER);
}

TEST(ASN1Test, Boolean) {
  static const uint8_t kTrue[] = {0x01, 0x01, 0xff};
  TestSerialize(0xff, i2d_ASN1_BOOLEAN, kTrue);
  // Other constants are also correctly encoded as TRUE.
  TestSerialize(1, i2d_ASN1_BOOLEAN, kTrue);
  TestSerialize(0x100, i2d_ASN1_BOOLEAN, kTrue);

  const uint8_t *ptr = kTrue;
  EXPECT_EQ(0xff, d2i_ASN1_BOOLEAN(nullptr, &ptr, sizeof(kTrue)));
  EXPECT_EQ(ptr, kTrue + sizeof(kTrue));

  static const uint8_t kFalse[] = {0x01, 0x01, 0x00};
  TestSerialize(0x00, i2d_ASN1_BOOLEAN, kFalse);

  ptr = kFalse;
  EXPECT_EQ(0, d2i_ASN1_BOOLEAN(nullptr, &ptr, sizeof(kFalse)));
  EXPECT_EQ(ptr, kFalse + sizeof(kFalse));

  const std::vector<uint8_t> kInvalidBooleans[] = {
      // No tag header.
      {},
      // No length.
      {0x01},
      // Truncated contents.
      {0x01, 0x01},
      // Contents too short or too long.
      {0x01, 0x00},
      {0x01, 0x02, 0x00, 0x00},
      // Wrong tag number.
      {0x02, 0x01, 0x00},
      // Wrong tag class.
      {0x81, 0x01, 0x00},
      // Element is constructed.
      {0x21, 0x01, 0x00},
      // Not a DER encoding of TRUE.
      {0x01, 0x01, 0x01},
      // Non-minimal tag length.
      {0x01, 0x81, 0x01, 0xff},
  };
  for (const auto &invalid : kInvalidBooleans) {
    SCOPED_TRACE(Bytes(invalid));
    ptr = invalid.data();
    EXPECT_EQ(-1, d2i_ASN1_BOOLEAN(nullptr, &ptr, invalid.size()));
    ERR_clear_error();
  }
}

// The templates go through a different codepath, so test them separately.
TEST(ASN1Test, SerializeEmbeddedBoolean) {
  bssl::UniquePtr<BASIC_CONSTRAINTS> val(BASIC_CONSTRAINTS_new());
  ASSERT_TRUE(val);

  // BasicConstraints defaults to FALSE, so the encoding should be empty.
  static const uint8_t kLeaf[] = {0x30, 0x00};
  val->ca = 0;
  TestSerialize(val.get(), i2d_BASIC_CONSTRAINTS, kLeaf);

  // TRUE should always be encoded as 0xff, independent of what value the caller
  // placed in the |ASN1_BOOLEAN|.
  static const uint8_t kCA[] = {0x30, 0x03, 0x01, 0x01, 0xff};
  val->ca = 0xff;
  TestSerialize(val.get(), i2d_BASIC_CONSTRAINTS, kCA);
  val->ca = 1;
  TestSerialize(val.get(), i2d_BASIC_CONSTRAINTS, kCA);
  val->ca = 0x100;
  TestSerialize(val.get(), i2d_BASIC_CONSTRAINTS, kCA);
}

TEST(ASN1Test, ASN1Type) {
  const struct {
    int type;
    std::vector<uint8_t> der;
  } kTests[] = {
      // BOOLEAN { TRUE }
      {V_ASN1_BOOLEAN, {0x01, 0x01, 0xff}},
      // BOOLEAN { FALSE }
      {V_ASN1_BOOLEAN, {0x01, 0x01, 0x00}},
      // OCTET_STRING { "a" }
      {V_ASN1_OCTET_STRING, {0x04, 0x01, 0x61}},
      // OCTET_STRING { }
      {V_ASN1_OCTET_STRING, {0x04, 0x00}},
      // BIT_STRING { `01` `00` }
      {V_ASN1_BIT_STRING, {0x03, 0x02, 0x01, 0x00}},
      // INTEGER { -1 }
      {V_ASN1_INTEGER, {0x02, 0x01, 0xff}},
      // OBJECT_IDENTIFIER { 1.2.840.113554.4.1.72585.2 }
      {V_ASN1_OBJECT,
       {0x06, 0x0c, 0x2a, 0x86, 0x48, 0x86, 0xf7, 0x12, 0x04, 0x01, 0x84, 0xb7,
        0x09, 0x02}},
      // NULL {}
      {V_ASN1_NULL, {0x05, 0x00}},
      // SEQUENCE {}
      {V_ASN1_SEQUENCE, {0x30, 0x00}},
      // SET {}
      {V_ASN1_SET, {0x31, 0x00}},
      // [0] { UTF8String { "a" } }
      {V_ASN1_OTHER, {0xa0, 0x03, 0x0c, 0x01, 0x61}},
  };
  for (const auto &t : kTests) {
    SCOPED_TRACE(Bytes(t.der));

    // The input should successfully parse.
    const uint8_t *ptr = t.der.data();
    bssl::UniquePtr<ASN1_TYPE> val(d2i_ASN1_TYPE(nullptr, &ptr, t.der.size()));
    ASSERT_TRUE(val);

    EXPECT_EQ(ASN1_TYPE_get(val.get()), t.type);
    EXPECT_EQ(val->type, t.type);
    TestSerialize(val.get(), i2d_ASN1_TYPE, t.der);
  }
}

// Test that reading |value.ptr| from a FALSE |ASN1_TYPE| behaves correctly. The
// type historically supported this, so maintain the invariant in case external
// code relies on it.
TEST(ASN1Test, UnusedBooleanBits) {
  // OCTET_STRING { "a" }
  static const uint8_t kDER[] = {0x04, 0x01, 0x61};
  const uint8_t *ptr = kDER;
  bssl::UniquePtr<ASN1_TYPE> val(d2i_ASN1_TYPE(nullptr, &ptr, sizeof(kDER)));
  ASSERT_TRUE(val);
  EXPECT_EQ(V_ASN1_OCTET_STRING, val->type);
  EXPECT_TRUE(val->value.ptr);

  // Set |val| to a BOOLEAN containing FALSE.
  ASN1_TYPE_set(val.get(), V_ASN1_BOOLEAN, NULL);
  EXPECT_EQ(V_ASN1_BOOLEAN, val->type);
  EXPECT_FALSE(val->value.ptr);
}

TEST(ASN1Test, ParseASN1Object) {
  // 1.2.840.113554.4.1.72585.2, an arbitrary unknown OID.
  static const uint8_t kOID[] = {0x2a, 0x86, 0x48, 0x86, 0xf7, 0x12,
                                 0x04, 0x01, 0x84, 0xb7, 0x09, 0x02};
  ASN1_OBJECT *obj = ASN1_OBJECT_create(NID_undef, kOID, sizeof(kOID),
                                        "short name", "long name");
  ASSERT_TRUE(obj);

  // OBJECT_IDENTIFIER { 1.3.101.112 }
  static const uint8_t kDER[] = {0x06, 0x03, 0x2b, 0x65, 0x70};
  const uint8_t *ptr = kDER;
  // Parse an |ASN1_OBJECT| with object reuse.
  EXPECT_TRUE(d2i_ASN1_OBJECT(&obj, &ptr, sizeof(kDER)));
  EXPECT_EQ(NID_ED25519, OBJ_obj2nid(obj));
  ASN1_OBJECT_free(obj);

  // Repeat the test, this time overriding a static |ASN1_OBJECT|. It should
  // detect this and construct a new one.
  obj = OBJ_nid2obj(NID_rsaEncryption);
  ptr = kDER;
  EXPECT_TRUE(d2i_ASN1_OBJECT(&obj, &ptr, sizeof(kDER)));
  EXPECT_EQ(NID_ED25519, OBJ_obj2nid(obj));
  ASN1_OBJECT_free(obj);

  const std::vector<uint8_t> kInvalidObjects[] = {
      // No tag header.
      {},
      // No length.
      {0x06},
      // Truncated contents.
      {0x06, 0x01},
      // An OID may not be empty.
      {0x06, 0x00},
      // The last byte may not be a continuation byte (high bit set).
      {0x06, 0x03, 0x2b, 0x65, 0xf0},
      // Each component must be minimally-encoded.
      {0x06, 0x03, 0x2b, 0x65, 0x80, 0x70},
      {0x06, 0x03, 0x80, 0x2b, 0x65, 0x70},
      // Wrong tag number.
      {0x01, 0x03, 0x2b, 0x65, 0x70},
      // Wrong tag class.
      {0x86, 0x03, 0x2b, 0x65, 0x70},
      // Element is constructed.
      {0x26, 0x03, 0x2b, 0x65, 0x70},
      // Non-minimal tag length.
      {0x06, 0x81, 0x03, 0x2b, 0x65, 0x70},
  };
  for (const auto &invalid : kInvalidObjects) {
    SCOPED_TRACE(Bytes(invalid));
    ptr = invalid.data();
    obj = d2i_ASN1_OBJECT(nullptr, &ptr, invalid.size());
    EXPECT_FALSE(obj);
    ASN1_OBJECT_free(obj);
    ERR_clear_error();
  }
}

TEST(ASN1Test, BitString) {
  const size_t kNotWholeBytes = static_cast<size_t>(-1);
  const struct {
    std::vector<uint8_t> in;
    size_t num_bytes;
  } kValidInputs[] = {
      // Empty bit string
      {{0x03, 0x01, 0x00}, 0},
      // 0b1
      {{0x03, 0x02, 0x07, 0x80}, kNotWholeBytes},
      // 0b1010
      {{0x03, 0x02, 0x04, 0xa0}, kNotWholeBytes},
      // 0b1010101
      {{0x03, 0x02, 0x01, 0xaa}, kNotWholeBytes},
      // 0b10101010
      {{0x03, 0x02, 0x00, 0xaa}, 1},
      // Bits 0 and 63 are set
      {{0x03, 0x09, 0x00, 0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01}, 8},
      // 64 zero bits
      {{0x03, 0x09, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00}, 8},
  };
  for (const auto &test : kValidInputs) {
    SCOPED_TRACE(Bytes(test.in));
    // The input should parse and round-trip correctly.
    const uint8_t *ptr = test.in.data();
    bssl::UniquePtr<ASN1_BIT_STRING> val(
        d2i_ASN1_BIT_STRING(nullptr, &ptr, test.in.size()));
    ASSERT_TRUE(val);
    TestSerialize(val.get(), i2d_ASN1_BIT_STRING, test.in);

    // Check the byte count.
    size_t num_bytes;
    if (test.num_bytes == kNotWholeBytes) {
      EXPECT_FALSE(ASN1_BIT_STRING_num_bytes(val.get(), &num_bytes));
    } else {
      ASSERT_TRUE(ASN1_BIT_STRING_num_bytes(val.get(), &num_bytes));
      EXPECT_EQ(num_bytes, test.num_bytes);
    }
  }

  const std::vector<uint8_t> kInvalidInputs[] = {
      // Wrong tag
      {0x04, 0x01, 0x00},
      // Missing leading byte
      {0x03, 0x00},
      // Leading byte too high
      {0x03, 0x02, 0x08, 0x00},
      {0x03, 0x02, 0xff, 0x00},
      // Empty bit strings must have a zero leading byte.
      {0x03, 0x01, 0x01},
      // Unused bits must all be zero.
      {0x03, 0x02, 0x06, 0xc1 /* 0b11000001 */},
  };
  for (const auto &test : kInvalidInputs) {
    SCOPED_TRACE(Bytes(test));
    const uint8_t *ptr = test.data();
    bssl::UniquePtr<ASN1_BIT_STRING> val(
        d2i_ASN1_BIT_STRING(nullptr, &ptr, test.size()));
    EXPECT_FALSE(val);
  }
}

TEST(ASN1Test, SetBit) {
  bssl::UniquePtr<ASN1_BIT_STRING> val(ASN1_BIT_STRING_new());
  ASSERT_TRUE(val);
  static const uint8_t kBitStringEmpty[] = {0x03, 0x01, 0x00};
  TestSerialize(val.get(), i2d_ASN1_BIT_STRING, kBitStringEmpty);
  EXPECT_EQ(0, ASN1_BIT_STRING_get_bit(val.get(), 0));
  EXPECT_EQ(0, ASN1_BIT_STRING_get_bit(val.get(), 100));

  // Set a few bits via |ASN1_BIT_STRING_set_bit|.
  ASSERT_TRUE(ASN1_BIT_STRING_set_bit(val.get(), 0, 1));
  ASSERT_TRUE(ASN1_BIT_STRING_set_bit(val.get(), 1, 1));
  ASSERT_TRUE(ASN1_BIT_STRING_set_bit(val.get(), 2, 0));
  ASSERT_TRUE(ASN1_BIT_STRING_set_bit(val.get(), 3, 1));
  static const uint8_t kBitString1101[] = {0x03, 0x02, 0x04, 0xd0};
  TestSerialize(val.get(), i2d_ASN1_BIT_STRING, kBitString1101);
  EXPECT_EQ(1, ASN1_BIT_STRING_get_bit(val.get(), 0));
  EXPECT_EQ(1, ASN1_BIT_STRING_get_bit(val.get(), 1));
  EXPECT_EQ(0, ASN1_BIT_STRING_get_bit(val.get(), 2));
  EXPECT_EQ(1, ASN1_BIT_STRING_get_bit(val.get(), 3));
  EXPECT_EQ(0, ASN1_BIT_STRING_get_bit(val.get(), 4));

  // Bits that were set may be cleared.
  ASSERT_TRUE(ASN1_BIT_STRING_set_bit(val.get(), 1, 0));
  static const uint8_t kBitString1001[] = {0x03, 0x02, 0x04, 0x90};
  TestSerialize(val.get(), i2d_ASN1_BIT_STRING, kBitString1001);
  EXPECT_EQ(1, ASN1_BIT_STRING_get_bit(val.get(), 0));
  EXPECT_EQ(0, ASN1_BIT_STRING_get_bit(val.get(), 1));
  EXPECT_EQ(0, ASN1_BIT_STRING_get_bit(val.get(), 2));
  EXPECT_EQ(1, ASN1_BIT_STRING_get_bit(val.get(), 3));
  EXPECT_EQ(0, ASN1_BIT_STRING_get_bit(val.get(), 4));

  // Clearing trailing bits truncates the string.
  ASSERT_TRUE(ASN1_BIT_STRING_set_bit(val.get(), 3, 0));
  static const uint8_t kBitString1[] = {0x03, 0x02, 0x07, 0x80};
  TestSerialize(val.get(), i2d_ASN1_BIT_STRING, kBitString1);
  EXPECT_EQ(1, ASN1_BIT_STRING_get_bit(val.get(), 0));
  EXPECT_EQ(0, ASN1_BIT_STRING_get_bit(val.get(), 1));
  EXPECT_EQ(0, ASN1_BIT_STRING_get_bit(val.get(), 2));
  EXPECT_EQ(0, ASN1_BIT_STRING_get_bit(val.get(), 3));
  EXPECT_EQ(0, ASN1_BIT_STRING_get_bit(val.get(), 4));

  // Bits may be set beyond the end of the string.
  ASSERT_TRUE(ASN1_BIT_STRING_set_bit(val.get(), 63, 1));
  static const uint8_t kBitStringLong[] = {0x03, 0x09, 0x00, 0x80, 0x00, 0x00,
                                           0x00, 0x00, 0x00, 0x00, 0x01};
  TestSerialize(val.get(), i2d_ASN1_BIT_STRING, kBitStringLong);
  EXPECT_EQ(1, ASN1_BIT_STRING_get_bit(val.get(), 0));
  EXPECT_EQ(0, ASN1_BIT_STRING_get_bit(val.get(), 62));
  EXPECT_EQ(1, ASN1_BIT_STRING_get_bit(val.get(), 63));
  EXPECT_EQ(0, ASN1_BIT_STRING_get_bit(val.get(), 64));

  // The string can be truncated back down again.
  ASSERT_TRUE(ASN1_BIT_STRING_set_bit(val.get(), 63, 0));
  TestSerialize(val.get(), i2d_ASN1_BIT_STRING, kBitString1);
  EXPECT_EQ(1, ASN1_BIT_STRING_get_bit(val.get(), 0));
  EXPECT_EQ(0, ASN1_BIT_STRING_get_bit(val.get(), 62));
  EXPECT_EQ(0, ASN1_BIT_STRING_get_bit(val.get(), 63));
  EXPECT_EQ(0, ASN1_BIT_STRING_get_bit(val.get(), 64));

  // |ASN1_BIT_STRING_set_bit| also truncates when starting from a parsed
  // string.
  const uint8_t *ptr = kBitStringLong;
  val.reset(d2i_ASN1_BIT_STRING(nullptr, &ptr, sizeof(kBitStringLong)));
  ASSERT_TRUE(val);
  TestSerialize(val.get(), i2d_ASN1_BIT_STRING, kBitStringLong);
  ASSERT_TRUE(ASN1_BIT_STRING_set_bit(val.get(), 63, 0));
  TestSerialize(val.get(), i2d_ASN1_BIT_STRING, kBitString1);
  EXPECT_EQ(1, ASN1_BIT_STRING_get_bit(val.get(), 0));
  EXPECT_EQ(0, ASN1_BIT_STRING_get_bit(val.get(), 62));
  EXPECT_EQ(0, ASN1_BIT_STRING_get_bit(val.get(), 63));
  EXPECT_EQ(0, ASN1_BIT_STRING_get_bit(val.get(), 64));

  // A parsed bit string preserves trailing zero bits.
  static const uint8_t kBitString10010[] = {0x03, 0x02, 0x03, 0x90};
  ptr = kBitString10010;
  val.reset(d2i_ASN1_BIT_STRING(nullptr, &ptr, sizeof(kBitString10010)));
  ASSERT_TRUE(val);
  TestSerialize(val.get(), i2d_ASN1_BIT_STRING, kBitString10010);
  // But |ASN1_BIT_STRING_set_bit| will truncate it even if otherwise a no-op.
  ASSERT_TRUE(ASN1_BIT_STRING_set_bit(val.get(), 0, 1));
  TestSerialize(val.get(), i2d_ASN1_BIT_STRING, kBitString1001);
  EXPECT_EQ(1, ASN1_BIT_STRING_get_bit(val.get(), 0));
  EXPECT_EQ(0, ASN1_BIT_STRING_get_bit(val.get(), 62));
  EXPECT_EQ(0, ASN1_BIT_STRING_get_bit(val.get(), 63));
  EXPECT_EQ(0, ASN1_BIT_STRING_get_bit(val.get(), 64));

  // By default, a BIT STRING implicitly truncates trailing zeros.
  val.reset(ASN1_BIT_STRING_new());
  ASSERT_TRUE(val);
  static const uint8_t kZeros[64] = {0};
  ASSERT_TRUE(ASN1_STRING_set(val.get(), kZeros, sizeof(kZeros)));
  TestSerialize(val.get(), i2d_ASN1_BIT_STRING, kBitStringEmpty);
}

TEST(ASN1Test, StringToUTF8) {
  static const struct {
    std::vector<uint8_t> in;
    int type;
    const char *expected;
  } kTests[] = {
      // Non-minimal, two-byte UTF-8.
      {{0xc0, 0x81}, V_ASN1_UTF8STRING, nullptr},
      // Non-minimal, three-byte UTF-8.
      {{0xe0, 0x80, 0x81}, V_ASN1_UTF8STRING, nullptr},
      // Non-minimal, four-byte UTF-8.
      {{0xf0, 0x80, 0x80, 0x81}, V_ASN1_UTF8STRING, nullptr},
      // Truncated, four-byte UTF-8.
      {{0xf0, 0x80, 0x80}, V_ASN1_UTF8STRING, nullptr},
      // Low-surrogate value.
      {{0xed, 0xa0, 0x80}, V_ASN1_UTF8STRING, nullptr},
      // High-surrogate value.
      {{0xed, 0xb0, 0x81}, V_ASN1_UTF8STRING, nullptr},
      // Initial BOMs should be rejected from UCS-2 and UCS-4.
      {{0xfe, 0xff, 0, 88}, V_ASN1_BMPSTRING, nullptr},
      {{0, 0, 0xfe, 0xff, 0, 0, 0, 88}, V_ASN1_UNIVERSALSTRING, nullptr},
      // Otherwise, BOMs should pass through.
      {{0, 88, 0xfe, 0xff}, V_ASN1_BMPSTRING, "X\xef\xbb\xbf"},
      {{0, 0, 0, 88, 0, 0, 0xfe, 0xff}, V_ASN1_UNIVERSALSTRING,
       "X\xef\xbb\xbf"},
      // The maximum code-point should pass though.
      {{0, 16, 0xff, 0xfd}, V_ASN1_UNIVERSALSTRING, "\xf4\x8f\xbf\xbd"},
      // Values outside the Unicode space should not.
      {{0, 17, 0, 0}, V_ASN1_UNIVERSALSTRING, nullptr},
      // Non-characters should be rejected.
      {{0, 1, 0xff, 0xff}, V_ASN1_UNIVERSALSTRING, nullptr},
      {{0, 1, 0xff, 0xfe}, V_ASN1_UNIVERSALSTRING, nullptr},
      {{0, 0, 0xfd, 0xd5}, V_ASN1_UNIVERSALSTRING, nullptr},
      // BMPString is UCS-2, not UTF-16, so surrogate pairs are invalid.
      {{0xd8, 0, 0xdc, 1}, V_ASN1_BMPSTRING, nullptr},
      // INTEGERs are stored as strings, but cannot be converted to UTF-8.
      {{0x01}, V_ASN1_INTEGER, nullptr},
  };

  for (const auto &test : kTests) {
    SCOPED_TRACE(Bytes(test.in));
    SCOPED_TRACE(test.type);
    bssl::UniquePtr<ASN1_STRING> s(ASN1_STRING_type_new(test.type));
    ASSERT_TRUE(s);
    ASSERT_TRUE(ASN1_STRING_set(s.get(), test.in.data(), test.in.size()));

    uint8_t *utf8;
    const int utf8_len = ASN1_STRING_to_UTF8(&utf8, s.get());
    EXPECT_EQ(utf8_len < 0, test.expected == nullptr);
    if (utf8_len >= 0) {
      if (test.expected != nullptr) {
        EXPECT_EQ(Bytes(test.expected), Bytes(utf8, utf8_len));
      }
      OPENSSL_free(utf8);
    } else {
      ERR_clear_error();
    }
  }
}

static std::string ASN1StringToStdString(const ASN1_STRING *str) {
  return std::string(ASN1_STRING_get0_data(str),
                     ASN1_STRING_get0_data(str) + ASN1_STRING_length(str));
}

static bool ASN1Time_check_posix(const ASN1_TIME *s, int64_t t) {
  struct tm stm, ttm;
  int day, sec;

  switch (ASN1_STRING_type(s)) {
    case V_ASN1_GENERALIZEDTIME:
      if (!asn1_generalizedtime_to_tm(&stm, s)) {
        return false;
      }
      break;
    case V_ASN1_UTCTIME:
      if (!asn1_utctime_to_tm(&stm, s, /*allow_timezone_offset=*/1)) {
        return false;
      }
      break;
    default:
      return false;
  }
  if (!OPENSSL_posix_to_tm(t, &ttm) ||
      !OPENSSL_gmtime_diff(&day, &sec, &ttm, &stm)) {
    return false;
  }
  return day == 0 && sec ==0;
}

static std::string PrintStringToBIO(const ASN1_STRING *str,
                                    int (*print_func)(BIO *,
                                                      const ASN1_STRING *)) {
  const uint8_t *data;
  size_t len;
  bssl::UniquePtr<BIO> bio(BIO_new(BIO_s_mem()));
  if (!bio ||  //
      !print_func(bio.get(), str) ||
      !BIO_mem_contents(bio.get(), &data, &len)) {
    ADD_FAILURE() << "Could not print to BIO";
    return "";
  }
  return std::string(data, data + len);
}

// MSVC 2015 does not support compound literals e.g. using (struct tm){0,0,...}}
// in the list of test vectors below. Note: this returns  a copy of the stack
// allocated value t.
static struct tm make_tm(int sec, int min, int hour, int mday, int mon, int year, int wday, int yday, int isdst, long gmtoff, char* zone) {
  struct tm t;
  t.tm_sec = sec;
  t.tm_min = min;
  t.tm_hour = hour;
  t.tm_mday = mday;
  t.tm_mon = mon;
  t.tm_year = year;
  t.tm_wday = wday;
  t.tm_yday = yday;
  t.tm_isdst = isdst;
#if defined(__GNU__)
  t.tm_gmtoff = gmtoff;
  t.tm_zone = zone;
#endif
  return t;
}

TEST(ASN1Test, SetTime) {
  static const struct {
    int64_t time;
    const char *generalized;
    const char *utc;
    const char *printed;
    const struct tm expected_tm;
    // struct tm years are deltas from 1900 and months start at 0
    // AWS-LC does not calculate or set year day, weekday, timezone, or daylight savings
  } kTests[] = {
      {-631152001, "19491231235959Z", nullptr, "Dec 31 23:59:59 1949 GMT",
       make_tm(59, 59, 23, 31, 11, 49, 0, 0, 0, 0, nullptr)},
      {-631152000, "19500101000000Z", "500101000000Z", "Jan  1 00:00:00 1950 GMT",
       make_tm(0, 0, 0, 1, 0, 50, 0, 0, 0, 0, nullptr)},
      {0, "19700101000000Z", "700101000000Z", "Jan  1 00:00:00 1970 GMT",
       make_tm(0, 0, 0, 1, 0, 70, 0, 0, 0, 0, nullptr)},
      {981173106, "20010203040506Z", "010203040506Z", "Feb  3 04:05:06 2001 GMT",
       make_tm(6, 5, 4, 3, 1, 101, 0, 0, 0, 0, nullptr)},
      {951804000, "20000229060000Z", "000229060000Z", "Feb 29 06:00:00 2000 GMT",
       make_tm(0, 0, 6, 29, 1, 100, 0, 0, 0, 0, nullptr)},
      // NASA says this is the correct time for posterity.
      {-16751025, "19690621025615Z", "690621025615Z", "Jun 21 02:56:15 1969 GMT",
       make_tm(15, 56, 2, 21, 5, 69, 0, 0, 0, 0, nullptr)},
      // -1 is sometimes used as an error value. Ensure we correctly handle it.
      {-1, "19691231235959Z", "691231235959Z", "Dec 31 23:59:59 1969 GMT",
       make_tm(59, 59, 23, 31, 11, 69, 0, 0, 0, 0, nullptr)},
      {2524607999, "20491231235959Z", "491231235959Z", "Dec 31 23:59:59 2049 GMT",
       make_tm(59, 59, 23, 31, 11, 149, 0, 0, 0, 0, nullptr)},
      {2524608000, "20500101000000Z", nullptr, "Jan  1 00:00:00 2050 GMT",
       make_tm(0, 0, 0, 1, 0, 150, 0, 0, 0, 0, nullptr)},
      // Test boundary conditions.
      {-62167219200, "00000101000000Z", nullptr, "Jan  1 00:00:00 0 GMT",
       make_tm(0, 0, 0, 1, 0, -1900, 0, 0, 0, 0, nullptr)},
      {-62167219201, nullptr, nullptr, nullptr,
       make_tm(0, 0, 0, 0, 0, 0, 0, 0, 0, 0, nullptr)},
      {253402300799, "99991231235959Z", nullptr, "Dec 31 23:59:59 9999 GMT",
       make_tm(59, 59, 23, 31, 11, 8099, 0, 0, 0, 0, nullptr)},
      {253402300800, nullptr, nullptr, nullptr,
       make_tm(0, 0, 0, 0, 0, 0, 0, 0, 0, 0, nullptr)},
  };
  for (const auto &t : kTests) {
    int64_t tt;
    struct tm actual_time_t;
    OPENSSL_memset(&actual_time_t, 0, sizeof(actual_time_t));
    SCOPED_TRACE(t.time);

    bssl::UniquePtr<ASN1_UTCTIME> utc(ASN1_UTCTIME_set(nullptr, t.time));

    if (t.utc) {
      ASSERT_TRUE(utc);
      EXPECT_EQ(V_ASN1_UTCTIME, ASN1_STRING_type(utc.get()));
      EXPECT_EQ(t.utc, ASN1StringToStdString(utc.get()));
      EXPECT_TRUE(ASN1Time_check_posix(utc.get(), t.time));
      EXPECT_EQ(ASN1_TIME_to_posix(utc.get(), &tt), 1);
      EXPECT_EQ(tt, t.time);
      EXPECT_EQ(PrintStringToBIO(utc.get(), &ASN1_UTCTIME_print), t.printed);
      EXPECT_EQ(PrintStringToBIO(utc.get(), &ASN1_TIME_print), t.printed);
      EXPECT_EQ(ASN1_TIME_to_tm(utc.get(), &actual_time_t), 1);
      EXPECT_EQ(OPENSSL_memcmp(&t.expected_tm, &actual_time_t, sizeof(actual_time_t)), 0);
    } else {
      EXPECT_FALSE(utc);
    }

    bssl::UniquePtr<ASN1_GENERALIZEDTIME> generalized(
        ASN1_GENERALIZEDTIME_set(nullptr, t.time));
    if (t.generalized) {
      ASSERT_TRUE(generalized);
      EXPECT_EQ(V_ASN1_GENERALIZEDTIME, ASN1_STRING_type(generalized.get()));
      EXPECT_EQ(t.generalized, ASN1StringToStdString(generalized.get()));
      EXPECT_TRUE(ASN1Time_check_posix(generalized.get(), t.time));
      EXPECT_EQ(ASN1_TIME_to_posix(generalized.get(), &tt), 1);
      EXPECT_EQ(tt, t.time);
      EXPECT_EQ(
          PrintStringToBIO(generalized.get(), &ASN1_GENERALIZEDTIME_print),
          t.printed);
      EXPECT_EQ(PrintStringToBIO(generalized.get(), &ASN1_TIME_print),
                t.printed);
      EXPECT_EQ(ASN1_TIME_to_tm(generalized.get(), &actual_time_t), 1);
      EXPECT_EQ(OPENSSL_memcmp(&t.expected_tm, &actual_time_t, sizeof(actual_time_t)), 0);
    } else {
      EXPECT_FALSE(generalized);
    }

    bssl::UniquePtr<ASN1_TIME> choice(ASN1_TIME_set_posix(nullptr, t.time));
    if (t.generalized) {
      ASSERT_TRUE(choice);
      if (t.utc) {
        EXPECT_EQ(V_ASN1_UTCTIME, ASN1_STRING_type(choice.get()));
        EXPECT_EQ(t.utc, ASN1StringToStdString(choice.get()));
      } else {
        EXPECT_EQ(V_ASN1_GENERALIZEDTIME, ASN1_STRING_type(choice.get()));
        EXPECT_EQ(t.generalized, ASN1StringToStdString(choice.get()));
      }
      EXPECT_TRUE(ASN1Time_check_posix(choice.get(), t.time));
      EXPECT_EQ(ASN1_TIME_to_posix(choice.get(), &tt), 1);
      EXPECT_EQ(tt, t.time);
      EXPECT_EQ(ASN1_TIME_to_tm(choice.get(), &actual_time_t), 1);
      EXPECT_EQ(OPENSSL_memcmp(&t.expected_tm, &actual_time_t, sizeof(actual_time_t)), 0);
    } else {
      EXPECT_FALSE(choice);
    }
  }
}

TEST(ASN1Test, ASN1_TIME_to_tm_default_behavior) {
  struct tm actual_time_t;
  OPENSSL_memset(&actual_time_t, 0, sizeof(actual_time_t));
  // null ASN1_TIME should use the current time
  EXPECT_EQ(ASN1_TIME_to_tm(nullptr, &actual_time_t), 1);
  // The current time should be some point before 1900 (year 0)
  EXPECT_GE(actual_time_t.tm_year, 0);

  // null struct tm should just check that ASN1_TIME is valid
  bssl::UniquePtr<ASN1_UTCTIME> date(ASN1_UTCTIME_set(nullptr, 1685142702));
  EXPECT_EQ(ASN1_TIME_to_tm(date.get(), nullptr), 1);

  // Make date an unknown type
  date.get()->type = -100000000;
  EXPECT_EQ(ASN1_TIME_to_tm(date.get(), nullptr), 0);

  EXPECT_EQ(ASN1_TIME_to_tm(nullptr, nullptr), 0);
}

TEST(ASN1Test, TimeSetString) {
  bssl::UniquePtr<ASN1_STRING> s(ASN1_STRING_new());
  ASSERT_TRUE(s);

  ASSERT_TRUE(ASN1_UTCTIME_set_string(s.get(), "700101000000Z"));
  EXPECT_EQ(V_ASN1_UTCTIME, ASN1_STRING_type(s.get()));
  EXPECT_EQ("700101000000Z", ASN1StringToStdString(s.get()));

  ASSERT_TRUE(ASN1_GENERALIZEDTIME_set_string(s.get(), "19700101000000Z"));
  EXPECT_EQ(V_ASN1_GENERALIZEDTIME, ASN1_STRING_type(s.get()));
  EXPECT_EQ("19700101000000Z", ASN1StringToStdString(s.get()));

  // |ASN1_TIME_set_string| accepts either format. It relies on there being no
  // overlap between the two.
  ASSERT_TRUE(ASN1_TIME_set_string(s.get(), "700101000000Z"));
  EXPECT_EQ(V_ASN1_UTCTIME, ASN1_STRING_type(s.get()));
  EXPECT_EQ("700101000000Z", ASN1StringToStdString(s.get()));

  ASSERT_TRUE(ASN1_TIME_set_string(s.get(), "19700101000000Z"));
  EXPECT_EQ(V_ASN1_GENERALIZEDTIME, ASN1_STRING_type(s.get()));
  EXPECT_EQ("19700101000000Z", ASN1StringToStdString(s.get()));

  // |ASN1_TIME_set_string_X509| behaves similarly except it additionally
  // converts GeneralizedTime to UTCTime if it fits.
  ASSERT_TRUE(ASN1_TIME_set_string_X509(s.get(), "700101000000Z"));
  EXPECT_EQ(V_ASN1_UTCTIME, ASN1_STRING_type(s.get()));
  EXPECT_EQ("700101000000Z", ASN1StringToStdString(s.get()));

  ASSERT_TRUE(ASN1_TIME_set_string_X509(s.get(), "19700101000000Z"));
  EXPECT_EQ(V_ASN1_UTCTIME, ASN1_STRING_type(s.get()));
  EXPECT_EQ("700101000000Z", ASN1StringToStdString(s.get()));

  ASSERT_TRUE(ASN1_TIME_set_string_X509(s.get(), "19500101000000Z"));
  EXPECT_EQ(V_ASN1_UTCTIME, ASN1_STRING_type(s.get()));
  EXPECT_EQ("500101000000Z", ASN1StringToStdString(s.get()));

  ASSERT_TRUE(ASN1_TIME_set_string_X509(s.get(), "19491231235959Z"));
  EXPECT_EQ(V_ASN1_GENERALIZEDTIME, ASN1_STRING_type(s.get()));
  EXPECT_EQ("19491231235959Z", ASN1StringToStdString(s.get()));

  ASSERT_TRUE(ASN1_TIME_set_string_X509(s.get(), "20491231235959Z"));
  EXPECT_EQ(V_ASN1_UTCTIME, ASN1_STRING_type(s.get()));
  EXPECT_EQ("491231235959Z", ASN1StringToStdString(s.get()));

  ASSERT_TRUE(ASN1_TIME_set_string_X509(s.get(), "20500101000000Z"));
  EXPECT_EQ(V_ASN1_GENERALIZEDTIME, ASN1_STRING_type(s.get()));
  EXPECT_EQ("20500101000000Z", ASN1StringToStdString(s.get()));

  // Invalid inputs are rejected.
  EXPECT_FALSE(ASN1_UTCTIME_set_string(s.get(), "nope"));
  EXPECT_FALSE(ASN1_UTCTIME_set_string(s.get(), "19700101000000Z"));
  EXPECT_FALSE(ASN1_GENERALIZEDTIME_set_string(s.get(), "nope"));
  EXPECT_FALSE(ASN1_GENERALIZEDTIME_set_string(s.get(), "700101000000Z"));
  EXPECT_FALSE(ASN1_TIME_set_string(s.get(), "nope"));

  // If passed a null object, the functions validate the input without writing
  // to anything.
  EXPECT_TRUE(ASN1_UTCTIME_set_string(nullptr, "700101000000Z"));
  EXPECT_TRUE(ASN1_TIME_set_string(nullptr, "700101000000Z"));
  EXPECT_TRUE(ASN1_TIME_set_string_X509(nullptr, "700101000000Z"));
  EXPECT_TRUE(ASN1_GENERALIZEDTIME_set_string(nullptr, "19700101000000Z"));
  EXPECT_TRUE(ASN1_TIME_set_string(nullptr, "19700101000000Z"));
  EXPECT_TRUE(ASN1_TIME_set_string_X509(nullptr, "19700101000000Z"));
  // Test an input |ASN1_TIME_set_string_X509| won't convert to UTCTime.
  EXPECT_TRUE(ASN1_GENERALIZEDTIME_set_string(nullptr, "20500101000000Z"));
  EXPECT_TRUE(ASN1_TIME_set_string(nullptr, "20500101000000Z"));
  EXPECT_TRUE(ASN1_TIME_set_string_X509(nullptr, "20500101000000Z"));
  EXPECT_FALSE(ASN1_UTCTIME_set_string(nullptr, "nope"));
  EXPECT_FALSE(ASN1_GENERALIZEDTIME_set_string(nullptr, "nope"));
  EXPECT_FALSE(ASN1_TIME_set_string(nullptr, "nope"));
  EXPECT_FALSE(ASN1_TIME_set_string_X509(nullptr, "nope"));

  // Timezone offsets are not allowed by DER.
  EXPECT_FALSE(ASN1_UTCTIME_set_string(nullptr, "700101000000-0400"));
  EXPECT_FALSE(ASN1_TIME_set_string(nullptr, "700101000000-0400"));
  EXPECT_FALSE(ASN1_TIME_set_string_X509(nullptr, "700101000000-0400"));
  EXPECT_FALSE(ASN1_GENERALIZEDTIME_set_string(nullptr, "19700101000000-0400"));
  EXPECT_FALSE(ASN1_TIME_set_string(nullptr, "19700101000000-0400"));
  EXPECT_FALSE(ASN1_TIME_set_string_X509(nullptr, "19700101000000-0400"));
}

TEST(ASN1Test, AdjTime) {
  struct tm tm1, tm2;
  int days, secs;

  EXPECT_TRUE(OPENSSL_posix_to_tm(0, &tm1));
  EXPECT_TRUE(OPENSSL_posix_to_tm(0, &tm2));
  // Test values that are too large and should be rejected.
  EXPECT_FALSE(OPENSSL_gmtime_adj(&tm1, INT_MIN, INT_MIN));
  EXPECT_FALSE(OPENSSL_gmtime_adj(&tm1, INT_MAX, INT_MAX));
  // Basic functionality.
  EXPECT_TRUE(OPENSSL_gmtime_adj(&tm2, 1, 1));
  EXPECT_TRUE(OPENSSL_gmtime_diff(&days, &secs, &tm1, &tm2));
  EXPECT_EQ(days, 1);
  EXPECT_EQ(secs, 1);
  EXPECT_TRUE(OPENSSL_gmtime_diff(&days, &secs, &tm2, &tm1));
  EXPECT_EQ(days, -1);
  EXPECT_EQ(secs, -1);
  // Test a value of days that is very large, but valid.
  EXPECT_TRUE(OPENSSL_gmtime_adj(&tm2, 2932800, 0));
  EXPECT_TRUE(OPENSSL_gmtime_diff(&days, &secs, &tm1, &tm2));
  EXPECT_EQ(days, 2932801);
  EXPECT_EQ(secs, 1);
  EXPECT_TRUE(OPENSSL_gmtime_diff(&days, &secs, &tm2, &tm1));
  EXPECT_EQ(days, -2932801);
  EXPECT_EQ(secs, -1);
}
static std::vector<uint8_t> StringToVector(const std::string &str) {
  return std::vector<uint8_t>(str.begin(), str.end());
}

TEST(ASN1Test, StringPrintEx) {
  const struct {
    int type;
    std::vector<uint8_t> data;
    int str_flags;
    unsigned long flags;
    std::string expected;
  } kTests[] = {
      // A string like "hello" is never escaped or quoted.
      // |ASN1_STRFLGS_ESC_QUOTE| only introduces quotes when needed. Note
      // OpenSSL interprets T61String as Latin-1.
      {V_ASN1_T61STRING, StringToVector("hello"), 0, 0, "hello"},
      {V_ASN1_T61STRING, StringToVector("hello"), 0,
       ASN1_STRFLGS_ESC_2253 | ASN1_STRFLGS_ESC_CTRL | ASN1_STRFLGS_ESC_MSB,
       "hello"},
      {V_ASN1_T61STRING, StringToVector("hello"), 0,
       ASN1_STRFLGS_ESC_2253 | ASN1_STRFLGS_ESC_CTRL | ASN1_STRFLGS_ESC_MSB |
           ASN1_STRFLGS_ESC_QUOTE,
       "hello"},

      // By default, 8-bit characters are printed without escaping.
      {V_ASN1_T61STRING,
       {0, '\n', 0x80, 0xff, ',', '+', '"', '\\', '<', '>', ';'},
       0,
       0,
       std::string(1, '\0') + "\n\x80\xff,+\"\\<>;"},

      // Flags control different escapes. Note that any escape flag will cause
      // blackslashes to be escaped.
      {V_ASN1_T61STRING,
       {0, '\n', 0x80, 0xff, ',', '+', '"', '\\', '<', '>', ';'},
       0,
       ASN1_STRFLGS_ESC_2253,
       std::string(1, '\0') + "\n\x80\xff\\,\\+\\\"\\\\\\<\\>\\;"},
      {V_ASN1_T61STRING,
       {0, '\n', 0x80, 0xff, ',', '+', '"', '\\', '<', '>', ';'},
       0,
       ASN1_STRFLGS_ESC_CTRL,
       "\\00\\0A\x80\xff,+\"\\\\<>;"},
      {V_ASN1_T61STRING,
       {0, '\n', 0x80, 0xff, ',', '+', '"', '\\', '<', '>', ';'},
       0,
       ASN1_STRFLGS_ESC_MSB,
       std::string(1, '\0') + "\n\\80\\FF,+\"\\\\<>;"},
      {V_ASN1_T61STRING,
       {0, '\n', 0x80, 0xff, ',', '+', '"', '\\', '<', '>', ';'},
       0,
       ASN1_STRFLGS_ESC_2253 | ASN1_STRFLGS_ESC_CTRL | ASN1_STRFLGS_ESC_MSB,
       "\\00\\0A\\80\\FF\\,\\+\\\"\\\\\\<\\>\\;"},

      // When quoted, fewer characters need to be escaped in RFC 2253.
      {V_ASN1_T61STRING,
       {0, '\n', 0x80, 0xff, ',', '+', '"', '\\', '<', '>', ';'},
       0,
       ASN1_STRFLGS_ESC_2253 | ASN1_STRFLGS_ESC_CTRL | ASN1_STRFLGS_ESC_MSB |
           ASN1_STRFLGS_ESC_QUOTE,
       "\"\\00\\0A\\80\\FF,+\\\"\\\\<>;\""},

      // If no characters benefit from quotes, no quotes are added.
      {V_ASN1_T61STRING,
       {0, '\n', 0x80, 0xff, '"', '\\'},
       0,
       ASN1_STRFLGS_ESC_2253 | ASN1_STRFLGS_ESC_CTRL | ASN1_STRFLGS_ESC_MSB |
           ASN1_STRFLGS_ESC_QUOTE,
       "\\00\\0A\\80\\FF\\\"\\\\"},

      // RFC 2253 only escapes spaces at the start and end of a string.
      {V_ASN1_T61STRING, StringToVector("   "), 0, ASN1_STRFLGS_ESC_2253,
       "\\  \\ "},
      {V_ASN1_T61STRING, StringToVector("   "), 0,
       ASN1_STRFLGS_ESC_2253 | ASN1_STRFLGS_UTF8_CONVERT, "\\  \\ "},
      {V_ASN1_T61STRING, StringToVector("   "), 0,
       ASN1_STRFLGS_ESC_2253 | ASN1_STRFLGS_ESC_QUOTE, "\"   \""},

      // RFC 2253 only escapes # at the start of a string.
      {V_ASN1_T61STRING, StringToVector("###"), 0, ASN1_STRFLGS_ESC_2253,
       "\\###"},
      {V_ASN1_T61STRING, StringToVector("###"), 0,
       ASN1_STRFLGS_ESC_2253 | ASN1_STRFLGS_ESC_QUOTE, "\"###\""},

      // By default, strings are decoded and Unicode code points are
      // individually escaped.
      {V_ASN1_UTF8STRING, StringToVector("a\xc2\x80\xc4\x80\xf0\x90\x80\x80"),
       0, ASN1_STRFLGS_ESC_MSB, "a\\80\\U0100\\W00010000"},
      {V_ASN1_BMPSTRING,
       {0x00, 'a', 0x00, 0x80, 0x01, 0x00},
       0,
       ASN1_STRFLGS_ESC_MSB,
       "a\\80\\U0100"},
      {V_ASN1_UNIVERSALSTRING,
       {0x00, 0x00, 0x00, 'a',   //
        0x00, 0x00, 0x00, 0x80,  //
        0x00, 0x00, 0x01, 0x00,  //
        0x00, 0x01, 0x00, 0x00},
       0,
       ASN1_STRFLGS_ESC_MSB,
       "a\\80\\U0100\\W00010000"},

      // |ASN1_STRFLGS_UTF8_CONVERT| normalizes everything to UTF-8 and then
      // escapes individual bytes.
      {V_ASN1_IA5STRING, StringToVector("a\x80"), 0,
       ASN1_STRFLGS_ESC_MSB | ASN1_STRFLGS_UTF8_CONVERT, "a\\C2\\80"},
      {V_ASN1_T61STRING, StringToVector("a\x80"), 0,
       ASN1_STRFLGS_ESC_MSB | ASN1_STRFLGS_UTF8_CONVERT, "a\\C2\\80"},
      {V_ASN1_UTF8STRING, StringToVector("a\xc2\x80\xc4\x80\xf0\x90\x80\x80"),
       0, ASN1_STRFLGS_ESC_MSB | ASN1_STRFLGS_UTF8_CONVERT,
       "a\\C2\\80\\C4\\80\\F0\\90\\80\\80"},
      {V_ASN1_BMPSTRING,
       {0x00, 'a', 0x00, 0x80, 0x01, 0x00},
       0,
       ASN1_STRFLGS_ESC_MSB | ASN1_STRFLGS_UTF8_CONVERT,
       "a\\C2\\80\\C4\\80"},
      {V_ASN1_UNIVERSALSTRING,
       {0x00, 0x00, 0x00, 'a',   //
        0x00, 0x00, 0x00, 0x80,  //
        0x00, 0x00, 0x01, 0x00,  //
        0x00, 0x01, 0x00, 0x00},
       0,
       ASN1_STRFLGS_ESC_MSB | ASN1_STRFLGS_UTF8_CONVERT,
       "a\\C2\\80\\C4\\80\\F0\\90\\80\\80"},

      // The same as above, but without escaping the UTF-8 encoding.
      {V_ASN1_IA5STRING, StringToVector("a\x80"), 0, ASN1_STRFLGS_UTF8_CONVERT,
       "a\xc2\x80"},
      {V_ASN1_T61STRING, StringToVector("a\x80"), 0, ASN1_STRFLGS_UTF8_CONVERT,
       "a\xc2\x80"},
      {V_ASN1_UTF8STRING, StringToVector("a\xc2\x80\xc4\x80\xf0\x90\x80\x80"),
       0, ASN1_STRFLGS_UTF8_CONVERT, "a\xc2\x80\xc4\x80\xf0\x90\x80\x80"},
      {V_ASN1_BMPSTRING,
       {0x00, 'a', 0x00, 0x80, 0x01, 0x00},
       0,
       ASN1_STRFLGS_UTF8_CONVERT,
       "a\xc2\x80\xc4\x80"},
      {V_ASN1_UNIVERSALSTRING,
       {0x00, 0x00, 0x00, 'a',   //
        0x00, 0x00, 0x00, 0x80,  //
        0x00, 0x00, 0x01, 0x00,  //
        0x00, 0x01, 0x00, 0x00},
       0,
       ASN1_STRFLGS_UTF8_CONVERT,
       "a\xc2\x80\xc4\x80\xf0\x90\x80\x80"},

      // Types that cannot be decoded are, by default, treated as a byte string.
      {V_ASN1_OCTET_STRING, {0xff}, 0, 0, "\xff"},
      {-1, {0xff}, 0, 0, "\xff"},
      {100, {0xff}, 0, 0, "\xff"},

      // |ASN1_STRFLGS_UTF8_CONVERT| still converts these bytes to UTF-8.
      //
      // TODO(davidben): This seems like a bug. Although it's unclear because
      // the non-RFC-2253 options aren't especially sound. Can we just remove
      // them?
      {V_ASN1_OCTET_STRING, {0xff}, 0, ASN1_STRFLGS_UTF8_CONVERT, "\xc3\xbf"},
      {-1, {0xff}, 0, ASN1_STRFLGS_UTF8_CONVERT, "\xc3\xbf"},
      {100, {0xff}, 0, ASN1_STRFLGS_UTF8_CONVERT, "\xc3\xbf"},

      // |ASN1_STRFLGS_IGNORE_TYPE| causes the string type to be ignored, so it
      // is always treated as a byte string, even if it is not a valid encoding.
      {V_ASN1_UTF8STRING, {0xff}, 0, ASN1_STRFLGS_IGNORE_TYPE, "\xff"},
      {V_ASN1_BMPSTRING, {0xff}, 0, ASN1_STRFLGS_IGNORE_TYPE, "\xff"},
      {V_ASN1_UNIVERSALSTRING, {0xff}, 0, ASN1_STRFLGS_IGNORE_TYPE, "\xff"},

      // |ASN1_STRFLGS_SHOW_TYPE| prepends the type name.
      {V_ASN1_UTF8STRING, {'a'}, 0, ASN1_STRFLGS_SHOW_TYPE, "UTF8STRING:a"},
      {-1, {'a'}, 0, ASN1_STRFLGS_SHOW_TYPE, "(unknown):a"},
      {100, {'a'}, 0, ASN1_STRFLGS_SHOW_TYPE, "(unknown):a"},

      // |ASN1_STRFLGS_DUMP_ALL| and |ASN1_STRFLGS_DUMP_UNKNOWN| cause
      // non-string types to be printed in hex, though without the DER wrapper
      // by default.
      {V_ASN1_UTF8STRING, StringToVector("\xe2\x98\x83"), 0,
       ASN1_STRFLGS_DUMP_UNKNOWN, "\\U2603"},
      {V_ASN1_UTF8STRING, StringToVector("\xe2\x98\x83"), 0,
       ASN1_STRFLGS_DUMP_ALL, "#E29883"},
      {V_ASN1_OCTET_STRING, StringToVector("\xe2\x98\x83"), 0,
       ASN1_STRFLGS_DUMP_UNKNOWN, "#E29883"},
      {V_ASN1_OCTET_STRING, StringToVector("\xe2\x98\x83"), 0,
       ASN1_STRFLGS_DUMP_ALL, "#E29883"},

      // |ASN1_STRFLGS_DUMP_DER| includes the entire element.
      {V_ASN1_UTF8STRING, StringToVector("\xe2\x98\x83"), 0,
       ASN1_STRFLGS_DUMP_ALL | ASN1_STRFLGS_DUMP_DER, "#0C03E29883"},
      {V_ASN1_OCTET_STRING, StringToVector("\xe2\x98\x83"), 0,
       ASN1_STRFLGS_DUMP_ALL | ASN1_STRFLGS_DUMP_DER, "#0403E29883"},
      {V_ASN1_BIT_STRING,
       {0x80},
       ASN1_STRING_FLAG_BITS_LEFT | 4,
       ASN1_STRFLGS_DUMP_ALL | ASN1_STRFLGS_DUMP_DER,
       "#03020480"},
      // INTEGER { 1 }
      {V_ASN1_INTEGER,
       {0x01},
       0,
       ASN1_STRFLGS_DUMP_ALL | ASN1_STRFLGS_DUMP_DER,
       "#020101"},
      // INTEGER { -1 }
      {V_ASN1_NEG_INTEGER,
       {0x01},
       0,
       ASN1_STRFLGS_DUMP_ALL | ASN1_STRFLGS_DUMP_DER,
       "#0201FF"},
      // ENUMERATED { 1 }
      {V_ASN1_ENUMERATED,
       {0x01},
       0,
       ASN1_STRFLGS_DUMP_ALL | ASN1_STRFLGS_DUMP_DER,
       "#0A0101"},
      // ENUMERATED { -1 }
      {V_ASN1_NEG_ENUMERATED,
       {0x01},
       0,
       ASN1_STRFLGS_DUMP_ALL | ASN1_STRFLGS_DUMP_DER,
       "#0A01FF"},
  };
  for (const auto &t : kTests) {
    SCOPED_TRACE(t.type);
    SCOPED_TRACE(Bytes(t.data));
    SCOPED_TRACE(t.str_flags);
    SCOPED_TRACE(t.flags);

    bssl::UniquePtr<ASN1_STRING> str(ASN1_STRING_type_new(t.type));
    ASSERT_TRUE(str);
    ASSERT_TRUE(ASN1_STRING_set(str.get(), t.data.data(), t.data.size()));
    str->flags = t.str_flags;

    // If the |BIO| is null, it should measure the size.
    int len = ASN1_STRING_print_ex(nullptr, str.get(), t.flags);
    EXPECT_EQ(len, static_cast<int>(t.expected.size()));

    // Measuring the size should also work for the |FILE| version
    len = ASN1_STRING_print_ex_fp(nullptr, str.get(), t.flags);
    EXPECT_EQ(len, static_cast<int>(t.expected.size()));

    // Actually print the string.
    bssl::UniquePtr<BIO> bio(BIO_new(BIO_s_mem()));
    ASSERT_TRUE(bio);
    len = ASN1_STRING_print_ex(bio.get(), str.get(), t.flags);
    EXPECT_EQ(len, static_cast<int>(t.expected.size()));

    const uint8_t *bio_contents;
    size_t bio_len;
    ASSERT_TRUE(BIO_mem_contents(bio.get(), &bio_contents, &bio_len));
    EXPECT_EQ(t.expected, std::string(bio_contents, bio_contents + bio_len));
  }

  const struct {
    int type;
    std::vector<uint8_t> data;
    int str_flags;
    unsigned long flags;
  } kUnprintableTests[] = {
      // When decoding strings, invalid codepoints are errors.
      {V_ASN1_UTF8STRING, {0xff}, 0, ASN1_STRFLGS_ESC_MSB},
      {V_ASN1_BMPSTRING, {0xff}, 0, ASN1_STRFLGS_ESC_MSB},
      {V_ASN1_BMPSTRING, {0xff}, 0, ASN1_STRFLGS_ESC_MSB},
      {V_ASN1_UNIVERSALSTRING, {0xff}, 0, ASN1_STRFLGS_ESC_MSB},
  };
  for (const auto &t : kUnprintableTests) {
    SCOPED_TRACE(t.type);
    SCOPED_TRACE(Bytes(t.data));
    SCOPED_TRACE(t.str_flags);
    SCOPED_TRACE(t.flags);

    bssl::UniquePtr<ASN1_STRING> str(ASN1_STRING_type_new(t.type));
    ASSERT_TRUE(str);
    ASSERT_TRUE(ASN1_STRING_set(str.get(), t.data.data(), t.data.size()));
    str->flags = t.str_flags;

    // If the |BIO| is null, it should measure the size.
    int len = ASN1_STRING_print_ex(nullptr, str.get(), t.flags);
    EXPECT_EQ(len, -1);
    ERR_clear_error();

    // Measuring the size should also work for the |FILE| version
    len = ASN1_STRING_print_ex_fp(nullptr, str.get(), t.flags);
    EXPECT_EQ(len, -1);
    ERR_clear_error();

    // Actually print the string.
    bssl::UniquePtr<BIO> bio(BIO_new(BIO_s_mem()));
    ASSERT_TRUE(bio);
    len = ASN1_STRING_print_ex(bio.get(), str.get(), t.flags);
    EXPECT_EQ(len, -1);
    ERR_clear_error();
  }
}

TEST(ASN1Test, MBString) {
  const unsigned long kAll = B_ASN1_PRINTABLESTRING | B_ASN1_IA5STRING |
                             B_ASN1_T61STRING | B_ASN1_BMPSTRING |
                             B_ASN1_UNIVERSALSTRING | B_ASN1_UTF8STRING;

  const struct {
    int format;
    std::vector<uint8_t> in;
    unsigned long mask;
    int expected_type;
    std::vector<uint8_t> expected_data;
    int num_codepoints;
  } kTests[] = {
      // Given a choice of formats, we pick the smallest that fits.
      {MBSTRING_UTF8, {}, kAll, V_ASN1_PRINTABLESTRING, {}, 0},
      {MBSTRING_UTF8, {'a'}, kAll, V_ASN1_PRINTABLESTRING, {'a'}, 1},
      {MBSTRING_UTF8,
       {'a', 'A', '0', '\'', '(', ')', '+', ',', '-', '.', '/', ':', '=', '?'},
       kAll,
       V_ASN1_PRINTABLESTRING,
       {'a', 'A', '0', '\'', '(', ')', '+', ',', '-', '.', '/', ':', '=', '?'},
       14},
      {MBSTRING_UTF8, {'*'}, kAll, V_ASN1_IA5STRING, {'*'}, 1},
      {MBSTRING_UTF8, {'\n'}, kAll, V_ASN1_IA5STRING, {'\n'}, 1},
      {MBSTRING_UTF8,
       {0xc2, 0x80 /* U+0080 */},
       kAll,
       V_ASN1_T61STRING,
       {0x80},
       1},
      {MBSTRING_UTF8,
       {0xc4, 0x80 /* U+0100 */},
       kAll,
       V_ASN1_BMPSTRING,
       {0x01, 0x00},
       1},
      {MBSTRING_UTF8,
       {0xf0, 0x90, 0x80, 0x80 /* U+10000 */},
       kAll,
       V_ASN1_UNIVERSALSTRING,
       {0x00, 0x01, 0x00, 0x00},
       1},
      {MBSTRING_UTF8,
       {0xf0, 0x90, 0x80, 0x80 /* U+10000 */},
       kAll & ~B_ASN1_UNIVERSALSTRING,
       V_ASN1_UTF8STRING,
       {0xf0, 0x90, 0x80, 0x80},
       1},

      // NUL is not printable. It should also not terminate iteration.
      {MBSTRING_UTF8, {0}, kAll, V_ASN1_IA5STRING, {0}, 1},
      {MBSTRING_UTF8, {0, 'a'}, kAll, V_ASN1_IA5STRING, {0, 'a'}, 2},

      // When a particular format is specified, we use it.
      {MBSTRING_UTF8,
       {'a'},
       B_ASN1_PRINTABLESTRING,
       V_ASN1_PRINTABLESTRING,
       {'a'},
       1},
      {MBSTRING_UTF8, {'a'}, B_ASN1_IA5STRING, V_ASN1_IA5STRING, {'a'}, 1},
      {MBSTRING_UTF8, {'a'}, B_ASN1_T61STRING, V_ASN1_T61STRING, {'a'}, 1},
      {MBSTRING_UTF8, {'a'}, B_ASN1_UTF8STRING, V_ASN1_UTF8STRING, {'a'}, 1},
      {MBSTRING_UTF8,
       {'a'},
       B_ASN1_BMPSTRING,
       V_ASN1_BMPSTRING,
       {0x00, 'a'},
       1},
      {MBSTRING_UTF8,
       {'a'},
       B_ASN1_UNIVERSALSTRING,
       V_ASN1_UNIVERSALSTRING,
       {0x00, 0x00, 0x00, 'a'},
       1},

      // A long string with characters of many widths, to test sizes are
      // measured in code points.
      {MBSTRING_UTF8,
       {
           'a',                     //
           0xc2, 0x80,              // U+0080
           0xc4, 0x80,              // U+0100
           0xf0, 0x90, 0x80, 0x80,  // U+10000
       },
       B_ASN1_UNIVERSALSTRING,
       V_ASN1_UNIVERSALSTRING,
       {
           0x00, 0x00, 0x00, 'a',   //
           0x00, 0x00, 0x00, 0x80,  //
           0x00, 0x00, 0x01, 0x00,  //
           0x00, 0x01, 0x00, 0x00,  //
       },
       4},
  };
  for (const auto &t : kTests) {
    SCOPED_TRACE(t.format);
    SCOPED_TRACE(Bytes(t.in));
    SCOPED_TRACE(t.mask);

    // Passing in nullptr should do a dry run.
    EXPECT_EQ(t.expected_type,
              ASN1_mbstring_copy(nullptr, t.in.data(), t.in.size(), t.format,
                                 t.mask));

    // Test allocating a new object.
    ASN1_STRING *str = nullptr;
    EXPECT_EQ(
        t.expected_type,
        ASN1_mbstring_copy(&str, t.in.data(), t.in.size(), t.format, t.mask));
    ASSERT_TRUE(str);
    EXPECT_EQ(t.expected_type, ASN1_STRING_type(str));
    EXPECT_EQ(Bytes(t.expected_data),
              Bytes(ASN1_STRING_get0_data(str), ASN1_STRING_length(str)));

    // Test writing into an existing object.
    ASN1_STRING_free(str);
    str = ASN1_STRING_new();
    ASSERT_TRUE(str);
    ASN1_STRING *old_str = str;
    EXPECT_EQ(
        t.expected_type,
        ASN1_mbstring_copy(&str, t.in.data(), t.in.size(), t.format, t.mask));
    ASSERT_EQ(old_str, str);
    EXPECT_EQ(t.expected_type, ASN1_STRING_type(str));
    EXPECT_EQ(Bytes(t.expected_data),
              Bytes(ASN1_STRING_get0_data(str), ASN1_STRING_length(str)));
    ASN1_STRING_free(str);
    str = nullptr;

    // minsize and maxsize should be enforced, even in a dry run.
    EXPECT_EQ(t.expected_type,
              ASN1_mbstring_ncopy(nullptr, t.in.data(), t.in.size(), t.format,
                                  t.mask, /*minsize=*/t.num_codepoints,
                                  /*maxsize=*/t.num_codepoints));

    EXPECT_EQ(t.expected_type,
              ASN1_mbstring_ncopy(&str, t.in.data(), t.in.size(), t.format,
                                  t.mask, /*minsize=*/t.num_codepoints,
                                  /*maxsize=*/t.num_codepoints));
    ASSERT_TRUE(str);
    EXPECT_EQ(t.expected_type, ASN1_STRING_type(str));
    EXPECT_EQ(Bytes(t.expected_data),
              Bytes(ASN1_STRING_get0_data(str), ASN1_STRING_length(str)));
    ASN1_STRING_free(str);
    str = nullptr;

    EXPECT_EQ(-1, ASN1_mbstring_ncopy(
                      nullptr, t.in.data(), t.in.size(), t.format, t.mask,
                      /*minsize=*/t.num_codepoints + 1, /*maxsize=*/0));
    ERR_clear_error();
    EXPECT_EQ(-1, ASN1_mbstring_ncopy(
                      &str, t.in.data(), t.in.size(), t.format, t.mask,
                      /*minsize=*/t.num_codepoints + 1, /*maxsize=*/0));
    EXPECT_FALSE(str);
    ERR_clear_error();
    if (t.num_codepoints > 1) {
      EXPECT_EQ(-1, ASN1_mbstring_ncopy(
                        nullptr, t.in.data(), t.in.size(), t.format, t.mask,
                        /*minsize=*/0, /*maxsize=*/t.num_codepoints - 1));
      ERR_clear_error();
      EXPECT_EQ(-1, ASN1_mbstring_ncopy(
                        &str, t.in.data(), t.in.size(), t.format, t.mask,
                        /*minsize=*/0, /*maxsize=*/t.num_codepoints - 1));
      EXPECT_FALSE(str);
      ERR_clear_error();
    }
  }

  const struct {
    int format;
    std::vector<uint8_t> in;
    unsigned long mask;
  } kInvalidTests[] = {
      // Invalid encodings are rejected.
      {MBSTRING_UTF8, {0xff}, B_ASN1_UTF8STRING},
      {MBSTRING_BMP, {0xff}, B_ASN1_UTF8STRING},
      {MBSTRING_UNIV, {0xff}, B_ASN1_UTF8STRING},

      // Lone surrogates are not code points.
      {MBSTRING_UTF8, {0xed, 0xa0, 0x80}, B_ASN1_UTF8STRING},
      {MBSTRING_BMP, {0xd8, 0x00}, B_ASN1_UTF8STRING},
      {MBSTRING_UNIV, {0x00, 0x00, 0xd8, 0x00}, B_ASN1_UTF8STRING},

      // The input does not fit in the allowed output types.
      {MBSTRING_UTF8, {'\n'}, B_ASN1_PRINTABLESTRING},
      {MBSTRING_UTF8,
       {0xc2, 0x80 /* U+0080 */},
       B_ASN1_PRINTABLESTRING | B_ASN1_IA5STRING},
      {MBSTRING_UTF8,
       {0xc4, 0x80 /* U+0100 */},
       B_ASN1_PRINTABLESTRING | B_ASN1_IA5STRING | B_ASN1_T61STRING},
      {MBSTRING_UTF8,
       {0xf0, 0x90, 0x80, 0x80 /* U+10000 */},
       B_ASN1_PRINTABLESTRING | B_ASN1_IA5STRING | B_ASN1_T61STRING |
           B_ASN1_BMPSTRING},

      // Unrecognized bits are ignored.
      {MBSTRING_UTF8, {'\n'}, B_ASN1_PRINTABLESTRING | B_ASN1_SEQUENCE},
  };
  for (const auto &t : kInvalidTests) {
    SCOPED_TRACE(t.format);
    SCOPED_TRACE(Bytes(t.in));
    SCOPED_TRACE(t.mask);

    EXPECT_EQ(-1, ASN1_mbstring_copy(nullptr, t.in.data(), t.in.size(),
                                     t.format, t.mask));
    ERR_clear_error();

    ASN1_STRING *str = nullptr;
    EXPECT_EQ(-1, ASN1_mbstring_copy(&str, t.in.data(), t.in.size(),
                                     t.format, t.mask));
    ERR_clear_error();
    EXPECT_EQ(nullptr, str);
  }

  // |len| values below -1 must be rejected; only -1 is special-cased to mean
  // "call strlen on |in|". Any other negative value would otherwise be cast
  // to a huge |size_t| by |CBS_init|.
  static const uint8_t kDummy[] = {'a'};
  ASN1_STRING *str = nullptr;
  EXPECT_EQ(-1, ASN1_mbstring_ncopy(&str, kDummy, -2, MBSTRING_UTF8,
                                    B_ASN1_UTF8STRING, /*minsize=*/0,
                                    /*maxsize=*/0));
  EXPECT_EQ(nullptr, str);
  ERR_clear_error();

  // |len == -1| treats |in| as NUL-terminated and should succeed.
  ASN1_STRING *str_from_strlen = nullptr;
  EXPECT_GE(ASN1_mbstring_ncopy(&str_from_strlen, (const uint8_t *)"a", -1,
                                MBSTRING_UTF8, B_ASN1_UTF8STRING,
                                /*minsize=*/0, /*maxsize=*/0),
            0);
  ASN1_STRING_free(str_from_strlen);
}

TEST(ASN1Test, StringByNID) {
  // |ASN1_mbstring_*| tests above test most of the interactions with |inform|,
  // so all tests below use UTF-8.
  const struct {
    int nid;
    std::string in;
    int expected_type;
    std::string expected;
  } kTests[] = {
      // Although DirectoryString and PKCS9String allow many types of strings,
      // we prefer UTF8String.
      {NID_commonName, "abc", V_ASN1_UTF8STRING, "abc"},
      {NID_commonName, "\xe2\x98\x83", V_ASN1_UTF8STRING, "\xe2\x98\x83"},
      {NID_localityName, "abc", V_ASN1_UTF8STRING, "abc"},
      {NID_stateOrProvinceName, "abc", V_ASN1_UTF8STRING, "abc"},
      {NID_organizationName, "abc", V_ASN1_UTF8STRING, "abc"},
      {NID_organizationalUnitName, "abc", V_ASN1_UTF8STRING, "abc"},
      {NID_pkcs9_unstructuredName, "abc", V_ASN1_UTF8STRING, "abc"},
      {NID_pkcs9_challengePassword, "abc", V_ASN1_UTF8STRING, "abc"},
      {NID_pkcs9_unstructuredAddress, "abc", V_ASN1_UTF8STRING, "abc"},
      {NID_givenName, "abc", V_ASN1_UTF8STRING, "abc"},
      {NID_givenName, "abc", V_ASN1_UTF8STRING, "abc"},
      {NID_givenName, "abc", V_ASN1_UTF8STRING, "abc"},
      {NID_surname, "abc", V_ASN1_UTF8STRING, "abc"},
      {NID_initials, "abc", V_ASN1_UTF8STRING, "abc"},
      {NID_name, "abc", V_ASN1_UTF8STRING, "abc"},

      // Some attribute types use a particular string type.
      {NID_countryName, "US", V_ASN1_PRINTABLESTRING, "US"},
      {NID_pkcs9_emailAddress, "example@example.com", V_ASN1_IA5STRING,
       "example@example.com"},
      {NID_serialNumber, "1234", V_ASN1_PRINTABLESTRING, "1234"},
      {NID_friendlyName, "abc", V_ASN1_BMPSTRING,
       std::string({'\0', 'a', '\0', 'b', '\0', 'c'})},
      {NID_dnQualifier, "US", V_ASN1_PRINTABLESTRING, "US"},
      {NID_domainComponent, "com", V_ASN1_IA5STRING, "com"},
      {NID_ms_csp_name, "abc", V_ASN1_BMPSTRING,
       std::string({'\0', 'a', '\0', 'b', '\0', 'c'})},

      // Unknown NIDs default to UTF8String.
      {NID_rsaEncryption, "abc", V_ASN1_UTF8STRING, "abc"},
  };
  for (const auto &t : kTests) {
    SCOPED_TRACE(t.nid);
    SCOPED_TRACE(t.in);

    // Test allocating a new object.
    bssl::UniquePtr<ASN1_STRING> str(ASN1_STRING_set_by_NID(
        nullptr, reinterpret_cast<const uint8_t *>(t.in.data()), t.in.size(),
        MBSTRING_UTF8, t.nid));
    ASSERT_TRUE(str);
    EXPECT_EQ(t.expected_type, ASN1_STRING_type(str.get()));
    EXPECT_EQ(Bytes(t.expected), Bytes(ASN1_STRING_get0_data(str.get()),
                                       ASN1_STRING_length(str.get())));

    // Test writing into an existing object.
    str.reset(ASN1_STRING_new());
    ASSERT_TRUE(str);
    ASN1_STRING *old_str = str.get();
    ASSERT_TRUE(ASN1_STRING_set_by_NID(
        &old_str, reinterpret_cast<const uint8_t *>(t.in.data()), t.in.size(),
        MBSTRING_UTF8, t.nid));
    ASSERT_EQ(old_str, str.get());
    EXPECT_EQ(t.expected_type, ASN1_STRING_type(str.get()));
    EXPECT_EQ(Bytes(t.expected), Bytes(ASN1_STRING_get0_data(str.get()),
                                       ASN1_STRING_length(str.get())));
  }

  const struct {
    int nid;
    std::string in;
  } kInvalidTests[] = {
      // DirectoryString forbids empty inputs.
      {NID_commonName, ""},
      {NID_localityName, ""},
      {NID_stateOrProvinceName, ""},
      {NID_organizationName, ""},
      {NID_organizationalUnitName, ""},
      {NID_pkcs9_unstructuredName, ""},
      {NID_pkcs9_challengePassword, ""},
      {NID_pkcs9_unstructuredAddress, ""},
      {NID_givenName, ""},
      {NID_givenName, ""},
      {NID_givenName, ""},
      {NID_surname, ""},
      {NID_initials, ""},
      {NID_name, ""},

      // Test upper bounds from RFC 5280.
      {NID_name, std::string(32769, 'a')},
      {NID_commonName, std::string(65, 'a')},
      {NID_localityName, std::string(129, 'a')},
      {NID_stateOrProvinceName, std::string(129, 'a')},
      {NID_organizationName, std::string(65, 'a')},
      {NID_organizationalUnitName, std::string(65, 'a')},
      {NID_pkcs9_emailAddress, std::string(256, 'a')},
      {NID_serialNumber, std::string(65, 'a')},

      // X520countryName must be exactly two characters.
      {NID_countryName, "A"},
      {NID_countryName, "AAA"},

      // Some string types cannot represent all codepoints.
      {NID_countryName, "\xe2\x98\x83"},
      {NID_pkcs9_emailAddress, "\xe2\x98\x83"},
      {NID_serialNumber, "\xe2\x98\x83"},
      {NID_dnQualifier, "\xe2\x98\x83"},
      {NID_domainComponent, "\xe2\x98\x83"},
  };
  for (const auto &t : kInvalidTests) {
    SCOPED_TRACE(t.nid);
    SCOPED_TRACE(t.in);
    bssl::UniquePtr<ASN1_STRING> str(ASN1_STRING_set_by_NID(
        nullptr, reinterpret_cast<const uint8_t *>(t.in.data()), t.in.size(),
        MBSTRING_UTF8, t.nid));
    EXPECT_FALSE(str);
    ERR_clear_error();
  }
}

TEST(ASN1Test, StringByCustomNID) {
  // This test affects library-global state. We rely on nothing else in the test
  // suite using these OIDs.
  int nid1 = OBJ_create("1.2.840.113554.4.1.72585.1000", "custom OID 1000",
                        "custom OID 1000");
  ASSERT_NE(NID_undef, nid1);
  int nid2 = OBJ_create("1.2.840.113554.4.1.72585.1001", "custom OID 1001",
                        "custom OID 1001");
  ASSERT_NE(NID_undef, nid2);

  // Values registered in the string table should be picked up.
  ASSERT_TRUE(ASN1_STRING_TABLE_add(nid1, 5, 10, V_ASN1_PRINTABLESTRING,
                                    STABLE_NO_MASK));
  bssl::UniquePtr<ASN1_STRING> str(ASN1_STRING_set_by_NID(
      nullptr, reinterpret_cast<const uint8_t *>("12345"), 5, MBSTRING_UTF8,
      nid1));
  ASSERT_TRUE(str);
  EXPECT_EQ(V_ASN1_PRINTABLESTRING, ASN1_STRING_type(str.get()));
  EXPECT_EQ(Bytes("12345"), Bytes(ASN1_STRING_get0_data(str.get()),
                                  ASN1_STRING_length(str.get())));

  // Minimum and maximum lengths are enforced.
  str.reset(ASN1_STRING_set_by_NID(
      nullptr, reinterpret_cast<const uint8_t *>("1234"), 4, MBSTRING_UTF8,
      nid1));
  EXPECT_FALSE(str);
  ERR_clear_error();
  str.reset(ASN1_STRING_set_by_NID(
      nullptr, reinterpret_cast<const uint8_t *>("12345678901"), 11,
      MBSTRING_UTF8, nid1));
  EXPECT_FALSE(str);
  ERR_clear_error();

  // Without |STABLE_NO_MASK|, we always pick UTF8String. -1 means there is no
  // length limit.
  ASSERT_TRUE(ASN1_STRING_TABLE_add(nid2, -1, -1, DIRSTRING_TYPE, 0));
  str.reset(ASN1_STRING_set_by_NID(nullptr,
                                   reinterpret_cast<const uint8_t *>("12345"),
                                   5, MBSTRING_UTF8, nid2));
  ASSERT_TRUE(str);
  EXPECT_EQ(V_ASN1_UTF8STRING, ASN1_STRING_type(str.get()));
  EXPECT_EQ(Bytes("12345"), Bytes(ASN1_STRING_get0_data(str.get()),
                                  ASN1_STRING_length(str.get())));

  // Overriding existing entries, built-in or custom, is an error.
  EXPECT_FALSE(
      ASN1_STRING_TABLE_add(NID_countryName, -1, -1, DIRSTRING_TYPE, 0));
  EXPECT_FALSE(ASN1_STRING_TABLE_add(nid1, -1, -1, DIRSTRING_TYPE, 0));
}

#if defined(OPENSSL_THREADS)
TEST(ASN1Test, StringByCustomNIDThreads) {
  // This test affects library-global state. We rely on nothing else in the test
  // suite using these OIDs.
  int nid1 = OBJ_create("1.2.840.113554.4.1.72585.1002", "custom OID 1002",
                        "custom OID 1002");
  ASSERT_NE(NID_undef, nid1);
  int nid2 = OBJ_create("1.2.840.113554.4.1.72585.1003", "custom OID 1003",
                        "custom OID 1003");
  ASSERT_NE(NID_undef, nid2);

  std::vector<std::thread> threads;
  threads.emplace_back([&] {
    ASSERT_TRUE(ASN1_STRING_TABLE_add(nid1, 5, 10, V_ASN1_PRINTABLESTRING,
                                      STABLE_NO_MASK));
    bssl::UniquePtr<ASN1_STRING> str(ASN1_STRING_set_by_NID(
        nullptr, reinterpret_cast<const uint8_t *>("12345"), 5, MBSTRING_UTF8,
        nid1));
    ASSERT_TRUE(str);
    EXPECT_EQ(V_ASN1_PRINTABLESTRING, ASN1_STRING_type(str.get()));
    EXPECT_EQ(Bytes("12345"), Bytes(ASN1_STRING_get0_data(str.get()),
                                    ASN1_STRING_length(str.get())));
  });
  threads.emplace_back([&] {
    ASSERT_TRUE(ASN1_STRING_TABLE_add(nid2, 5, 10, V_ASN1_PRINTABLESTRING,
                                      STABLE_NO_MASK));
    bssl::UniquePtr<ASN1_STRING> str(ASN1_STRING_set_by_NID(
        nullptr, reinterpret_cast<const uint8_t *>("12345"), 5, MBSTRING_UTF8,
        nid2));
    ASSERT_TRUE(str);
    EXPECT_EQ(V_ASN1_PRINTABLESTRING, ASN1_STRING_type(str.get()));
    EXPECT_EQ(Bytes("12345"), Bytes(ASN1_STRING_get0_data(str.get()),
                                    ASN1_STRING_length(str.get())));
  });
  for (auto &thread : threads) {
    thread.join();
  }
}
#endif  // OPENSSL_THREADS

// Test that multi-string types correctly encode negative ENUMERATED.
// Multi-string types cannot contain INTEGER, so we only test ENUMERATED.
TEST(ASN1Test, NegativeEnumeratedMultistring) {
  static const uint8_t kMinusOne[] = {0x0a, 0x01, 0xff};  // ENUMERATED { -1 }
  // |ASN1_PRINTABLE| is a multi-string type that allows ENUMERATED.
  const uint8_t *p = kMinusOne;
  bssl::UniquePtr<ASN1_STRING> str(
      d2i_ASN1_PRINTABLE(nullptr, &p, sizeof(kMinusOne)));
  ASSERT_TRUE(str);
  TestSerialize(str.get(), i2d_ASN1_PRINTABLE, kMinusOne);
}

// Encoding a CHOICE type with an invalid selector should fail.
TEST(ASN1Test, InvalidChoice) {
  bssl::UniquePtr<GENERAL_NAME> name(GENERAL_NAME_new());
  ASSERT_TRUE(name);
  // CHOICE types are initialized with an invalid selector.
  EXPECT_EQ(-1, name->type);
  // |name| should fail to encode.
  EXPECT_EQ(-1, i2d_GENERAL_NAME(name.get(), nullptr));

  // The error should be propagated through types containing |name|.
  bssl::UniquePtr<GENERAL_NAMES> names(GENERAL_NAMES_new());
  ASSERT_TRUE(names);
  EXPECT_TRUE(bssl::PushToStack(names.get(), std::move(name)));
  EXPECT_EQ(-1, i2d_GENERAL_NAMES(names.get(), nullptr));
}

// Encoding NID-only |ASN1_OBJECT|s should fail.
TEST(ASN1Test, InvalidObject) {
  EXPECT_EQ(-1, i2d_ASN1_OBJECT(OBJ_nid2obj(NID_kx_ecdhe), nullptr));

  bssl::UniquePtr<X509_ALGOR> alg(X509_ALGOR_new());
  ASSERT_TRUE(alg);
  ASSERT_TRUE(X509_ALGOR_set0(alg.get(), OBJ_nid2obj(NID_kx_ecdhe),
                              V_ASN1_UNDEF, nullptr));
  EXPECT_EQ(-1, i2d_X509_ALGOR(alg.get(), nullptr));
}

// Encoding invalid |ASN1_TYPE|s should fail. |ASN1_TYPE|s are
// default-initialized to an invalid type.
TEST(ASN1Test, InvalidASN1Type) {
  bssl::UniquePtr<ASN1_TYPE> obj(ASN1_TYPE_new());
  ASSERT_TRUE(obj);
  EXPECT_EQ(-1, obj->type);
  EXPECT_EQ(-1, i2d_ASN1_TYPE(obj.get(), nullptr));
}

// Encoding invalid MSTRING types should fail. An MSTRING is a CHOICE of
// string-like types. They are initialized to an invalid type.
TEST(ASN1Test, InvalidMSTRING) {
  bssl::UniquePtr<ASN1_STRING> obj(ASN1_TIME_new());
  ASSERT_TRUE(obj);
  EXPECT_EQ(-1, obj->type);
  EXPECT_EQ(-1, i2d_ASN1_TIME(obj.get(), nullptr));

  obj.reset(DIRECTORYSTRING_new());
  ASSERT_TRUE(obj);
  EXPECT_EQ(-1, obj->type);
  EXPECT_EQ(-1, i2d_DIRECTORYSTRING(obj.get(), nullptr));
}

TEST(ASN1Test, StringTableSorted) {
  const ASN1_STRING_TABLE *table;
  size_t table_len;
  asn1_get_string_table_for_testing(&table, &table_len);
  for (size_t i = 1; i < table_len; i++) {
    EXPECT_LT(table[i-1].nid, table[i].nid);
  }
}

TEST(ASN1Test, Null) {
  // An |ASN1_NULL| is an opaque, non-null pointer. It is an arbitrary signaling
  // value and does not need to be freed. (If the pointer is null, this is an
  // omitted OPTIONAL NULL.)
  EXPECT_NE(nullptr, ASN1_NULL_new());

  // It is safe to free either the non-null pointer or the null one.
  ASN1_NULL_free(ASN1_NULL_new());
  ASN1_NULL_free(nullptr);

  // A NULL may be decoded.
  static const uint8_t kNull[] = {0x05, 0x00};
  const uint8_t *ptr = kNull;
  EXPECT_NE(nullptr, d2i_ASN1_NULL(nullptr, &ptr, sizeof(kNull)));
  EXPECT_EQ(ptr, kNull + sizeof(kNull));

  // It may also be re-encoded.
  uint8_t *enc = nullptr;
  int enc_len = i2d_ASN1_NULL(ASN1_NULL_new(), &enc);
  ASSERT_GE(enc_len, 0);
  EXPECT_EQ(Bytes(kNull), Bytes(enc, enc_len));
  OPENSSL_free(enc);
  enc = nullptr;

  // Although the standalone representation of NULL is a non-null pointer, the
  // |ASN1_TYPE| representation is a null pointer.
  ptr = kNull;
  bssl::UniquePtr<ASN1_TYPE> null_type(
      d2i_ASN1_TYPE(nullptr, &ptr, sizeof(kNull)));
  ASSERT_TRUE(null_type);
  EXPECT_EQ(ptr, kNull + sizeof(kNull));
  EXPECT_EQ(V_ASN1_NULL, ASN1_TYPE_get(null_type.get()));
  EXPECT_EQ(nullptr, null_type->value.ptr);
}

TEST(ASN1Test, Pack) {
  bssl::UniquePtr<BASIC_CONSTRAINTS> val(BASIC_CONSTRAINTS_new());
  ASSERT_TRUE(val);
  val->ca = 0;

  // Test all three calling conventions.
  static const uint8_t kExpected[] = {0x30, 0x00};
  bssl::UniquePtr<ASN1_STRING> str(
      ASN1_item_pack(val.get(), ASN1_ITEM_rptr(BASIC_CONSTRAINTS), nullptr));
  ASSERT_TRUE(str);
  EXPECT_EQ(
      Bytes(ASN1_STRING_get0_data(str.get()), ASN1_STRING_length(str.get())),
      Bytes(kExpected));

  ASN1_STRING *raw = nullptr;
  str.reset(ASN1_item_pack(val.get(), ASN1_ITEM_rptr(BASIC_CONSTRAINTS), &raw));
  ASSERT_TRUE(str);
  EXPECT_EQ(raw, str.get());
  EXPECT_EQ(
      Bytes(ASN1_STRING_get0_data(str.get()), ASN1_STRING_length(str.get())),
      Bytes(kExpected));

  str.reset(ASN1_STRING_new());
  ASSERT_TRUE(str);
  raw = str.get();
  EXPECT_TRUE(
      ASN1_item_pack(val.get(), ASN1_ITEM_rptr(BASIC_CONSTRAINTS), &raw));
  EXPECT_EQ(raw, str.get());
  EXPECT_EQ(
      Bytes(ASN1_STRING_get0_data(str.get()), ASN1_STRING_length(str.get())),
      Bytes(kExpected));
}

TEST(ASN1Test, Unpack) {
  bssl::UniquePtr<ASN1_STRING> str(ASN1_STRING_new());
  ASSERT_TRUE(str);

  static const uint8_t kValid[] = {0x30, 0x00};
  ASSERT_TRUE(
      ASN1_STRING_set(str.get(), kValid, sizeof(kValid)));
  bssl::UniquePtr<BASIC_CONSTRAINTS> val(static_cast<BASIC_CONSTRAINTS *>(
      ASN1_item_unpack(str.get(), ASN1_ITEM_rptr(BASIC_CONSTRAINTS))));
  ASSERT_TRUE(val);
  EXPECT_EQ(val->ca, 0);
  EXPECT_EQ(val->pathlen, nullptr);

  static const uint8_t kInvalid[] = {0x31, 0x00};
  ASSERT_TRUE(ASN1_STRING_set(str.get(), kInvalid, sizeof(kInvalid)));
  val.reset(static_cast<BASIC_CONSTRAINTS *>(
      ASN1_item_unpack(str.get(), ASN1_ITEM_rptr(BASIC_CONSTRAINTS))));
  EXPECT_FALSE(val);

  static const uint8_t kTraiilingData[] = {0x30, 0x00, 0x00};
  ASSERT_TRUE(
      ASN1_STRING_set(str.get(), kTraiilingData, sizeof(kTraiilingData)));
  val.reset(static_cast<BASIC_CONSTRAINTS *>(
      ASN1_item_unpack(str.get(), ASN1_ITEM_rptr(BASIC_CONSTRAINTS))));
  EXPECT_FALSE(val);
}

TEST(ASN1Test, StringCmp) {
  struct Input {
    int type;
    std::vector<uint8_t> data;
    int flags;
    bool equals_previous;
  };
  // kInputs is a list of |ASN1_STRING| parameters, in sorted order. The input
  // should be sorted by bit length, then data, then type.
  const Input kInputs[] = {
      {V_ASN1_BIT_STRING, {}, ASN1_STRING_FLAG_BITS_LEFT | 0, false},
      {V_ASN1_BIT_STRING, {}, 0, true},
      // When |ASN1_STRING_FLAG_BITS_LEFT| is unset, BIT STRINGs implicitly
      // drop trailing zeros.
      {V_ASN1_BIT_STRING, {0x00, 0x00, 0x00, 0x00}, 0, true},

      {V_ASN1_OCTET_STRING, {}, 0, false},
      {V_ASN1_UTF8STRING, {}, 0, false},

      // BIT STRINGs with padding bits (i.e. not part of the actual value) are
      // shorter and thus sort earlier:
      // 1-bit inputs.
      {V_ASN1_BIT_STRING, {0x00}, ASN1_STRING_FLAG_BITS_LEFT | 7, false},
      {V_ASN1_BIT_STRING, {0x80}, ASN1_STRING_FLAG_BITS_LEFT | 7, false},
      // 2-bit inputs.
      {V_ASN1_BIT_STRING, {0x00}, ASN1_STRING_FLAG_BITS_LEFT | 6, false},
      {V_ASN1_BIT_STRING, {0xc0}, ASN1_STRING_FLAG_BITS_LEFT | 6, false},
      // 3-bit inputs.
      {V_ASN1_BIT_STRING, {0x00}, ASN1_STRING_FLAG_BITS_LEFT | 5, false},
      {V_ASN1_BIT_STRING, {0xe0}, ASN1_STRING_FLAG_BITS_LEFT | 5, false},
      // 4-bit inputs.
      {V_ASN1_BIT_STRING, {0xf0}, ASN1_STRING_FLAG_BITS_LEFT | 4, false},
      {V_ASN1_BIT_STRING, {0xf0}, 0, true},        // 4 trailing zeros dropped.
      {V_ASN1_BIT_STRING, {0xf0, 0x00}, 0, true},  // 12 trailing zeros dropped.
      // 5-bit inputs.
      {V_ASN1_BIT_STRING, {0x00}, ASN1_STRING_FLAG_BITS_LEFT | 3, false},
      {V_ASN1_BIT_STRING, {0xf0}, ASN1_STRING_FLAG_BITS_LEFT | 3, false},
      {V_ASN1_BIT_STRING, {0xf8}, ASN1_STRING_FLAG_BITS_LEFT | 3, false},
      // 6-bit inputs.
      {V_ASN1_BIT_STRING, {0x00}, ASN1_STRING_FLAG_BITS_LEFT | 2, false},
      {V_ASN1_BIT_STRING, {0xf0}, ASN1_STRING_FLAG_BITS_LEFT | 2, false},
      {V_ASN1_BIT_STRING, {0xfc}, ASN1_STRING_FLAG_BITS_LEFT | 2, false},
      // 7-bit inputs.
      {V_ASN1_BIT_STRING, {0x00}, ASN1_STRING_FLAG_BITS_LEFT | 1, false},
      {V_ASN1_BIT_STRING, {0xf0}, ASN1_STRING_FLAG_BITS_LEFT | 1, false},
      {V_ASN1_BIT_STRING, {0xfe}, ASN1_STRING_FLAG_BITS_LEFT | 1, false},

      // 8-bit inputs.
      {V_ASN1_BIT_STRING, {0x00}, ASN1_STRING_FLAG_BITS_LEFT | 0, false},
      {V_ASN1_OCTET_STRING, {0x00}, 0, false},
      {V_ASN1_UTF8STRING, {0x00}, 0, false},

      {V_ASN1_BIT_STRING, {0x80}, ASN1_STRING_FLAG_BITS_LEFT | 0, false},
      {V_ASN1_OCTET_STRING, {0x80}, 0, false},
      {V_ASN1_UTF8STRING, {0x80}, 0, false},

      {V_ASN1_BIT_STRING, {0xff}, ASN1_STRING_FLAG_BITS_LEFT | 0, false},
      {V_ASN1_BIT_STRING, {0xff}, 0, true},  // No trailing zeros to drop.
      {V_ASN1_OCTET_STRING, {0xff}, 0, false},
      {V_ASN1_UTF8STRING, {0xff}, 0, false},

      // Bytes are compared lexicographically.
      {V_ASN1_BIT_STRING, {0x00, 0x00}, ASN1_STRING_FLAG_BITS_LEFT | 0, false},
      {V_ASN1_OCTET_STRING, {0x00, 0x00}, 0, false},
      {V_ASN1_UTF8STRING, {0x00, 0x00}, 0, false},

      {V_ASN1_BIT_STRING, {0x00, 0xff}, ASN1_STRING_FLAG_BITS_LEFT | 0, false},
      {V_ASN1_OCTET_STRING, {0x00, 0xff}, 0, false},
      {V_ASN1_UTF8STRING, {0x00, 0xff}, 0, false},

      {V_ASN1_BIT_STRING, {0xff, 0x00}, ASN1_STRING_FLAG_BITS_LEFT | 0, false},
      {V_ASN1_OCTET_STRING, {0xff, 0x00}, 0, false},
      {V_ASN1_UTF8STRING, {0xff, 0x00}, 0, false},
  };
  std::vector<bssl::UniquePtr<ASN1_STRING>> strs;
  strs.reserve(OPENSSL_ARRAY_SIZE(kInputs));
  for (const auto &input : kInputs) {
    strs.emplace_back(ASN1_STRING_type_new(input.type));
    ASSERT_TRUE(strs.back());
    ASSERT_TRUE(ASN1_STRING_set(strs.back().get(), input.data.data(),
                                input.data.size()));
    strs.back()->flags = input.flags;
  }

  for (size_t i = 0; i < strs.size(); i++) {
    SCOPED_TRACE(i);
    bool expect_equal = true;
    for (size_t j = i; j < strs.size(); j++) {
      SCOPED_TRACE(j);
      if (j > i && !kInputs[j].equals_previous) {
        expect_equal = false;
      }

      const int cmp_i_j = ASN1_STRING_cmp(strs[i].get(), strs[j].get());
      const int cmp_j_i = ASN1_STRING_cmp(strs[j].get(), strs[i].get());
      if (expect_equal) {
        EXPECT_EQ(cmp_i_j, 0);
        EXPECT_EQ(cmp_j_i, 0);
      } else if (i < j) {
        EXPECT_LT(cmp_i_j, 0);
        EXPECT_GT(cmp_j_i, 0);
      } else {
        EXPECT_GT(cmp_i_j, 0);
        EXPECT_LT(cmp_j_i, 0);
      }
    }
  }
}

TEST(ASN1Test, PrintASN1Object) {
  const struct {
    std::vector<uint8_t> in;
    const char *expected;
  } kDataTests[] = {
      // Known OIDs print as the name.
      {{0x2a, 0x86, 0x48, 0x86, 0xf7, 0x0d, 0x01, 0x01, 0x01}, "rsaEncryption"},

      // Unknown OIDs print in decimal.
      {{0x2a, 0x86, 0x48, 0x86, 0xf7, 0x12, 0x04, 0x01, 0x84, 0xb7, 0x09, 0x00},
       "1.2.840.113554.4.1.72585.0"},

      // Inputs which cannot be parsed as OIDs print as "<INVALID>".
      {{0xff}, "<INVALID>"},

      // The function has an internal 80-byte buffer. Test inputs at that
      // boundary. First, 78 characters.
      {{0x2a, 0x86, 0x48, 0x86, 0xf7, 0x12, 0x04, 0x01, 0x84, 0xb7,
        0x09, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01},
       "1.2.840.113554.4.1.72585.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0."
       "0.0.0.1"},
      // 79 characters.
      {{0x2a, 0x86, 0x48, 0x86, 0xf7, 0x12, 0x04, 0x01, 0x84, 0xb7,
        0x09, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0a},
       "1.2.840.113554.4.1.72585.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0."
       "0.0.0.10"},
      // 80 characters.
      {{0x2a, 0x86, 0x48, 0x86, 0xf7, 0x12, 0x04, 0x01, 0x84, 0xb7,
        0x09, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x64},
       "1.2.840.113554.4.1.72585.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0."
       "0.0.0.100"},
      // 81 characters.
      {{0x2a, 0x86, 0x48, 0x86, 0xf7, 0x12, 0x04, 0x01, 0x84, 0xb7,
        0x09, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x87, 0x68},
       "1.2.840.113554.4.1.72585.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0."
       "0.0.0.1000"},
      // 82 characters.
      {{0x2a, 0x86, 0x48, 0x86, 0xf7, 0x12, 0x04, 0x01, 0x84, 0xb7,
        0x09, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xce, 0x10},
       "1.2.840.113554.4.1.72585.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0."
       "0.0.0.10000"},
  };
  for (const auto &t : kDataTests) {
    SCOPED_TRACE(Bytes(t.in));
    bssl::UniquePtr<ASN1_OBJECT> obj(ASN1_OBJECT_create(
        NID_undef, t.in.data(), t.in.size(), /*sn=*/nullptr, /*ln=*/nullptr));
    ASSERT_TRUE(obj);
    bssl::UniquePtr<BIO> bio(BIO_new(BIO_s_mem()));
    ASSERT_TRUE(bio);

    int len = i2a_ASN1_OBJECT(bio.get(), obj.get());
    EXPECT_EQ(len, static_cast<int>(strlen(t.expected)));

    const uint8_t *bio_data;
    size_t bio_len;
    BIO_mem_contents(bio.get(), &bio_data, &bio_len);
    EXPECT_EQ(t.expected,
              std::string(reinterpret_cast<const char *>(bio_data), bio_len));
  }

  // Test writing NULL.
  bssl::UniquePtr<BIO> bio(BIO_new(BIO_s_mem()));
  ASSERT_TRUE(bio);
  int len = i2a_ASN1_OBJECT(bio.get(), nullptr);
  EXPECT_EQ(len, 4);
  const uint8_t *bio_data;
  size_t bio_len;
  BIO_mem_contents(bio.get(), &bio_data, &bio_len);
  EXPECT_EQ("NULL",
            std::string(reinterpret_cast<const char *>(bio_data), bio_len));
}

const struct GetObjectTestData {
  std::vector<uint8_t> in;
  const int expected_return;
  const int expected_tag;
  const int expected_class;
  const int expected_length;
} kGetObjectTests[] = {
    // OpenSSL succeeds for all test cases below.
    // The header is valid, but there are not enough bytes for the length.
    {{0x30, 0x01}, 0x80, 0, 0, 0},
    {{0x30, 0x80, 0x00, 0x00}, 0x21, 0x10, 0, 0},
    {{0x00, 0x00}, 0x00, 0x00, 0x00, 0},
    {{0x01, 0x00}, 0x00, 0x01, 0x00, 0},
    {{0x41, 0x00}, 0x00, 0x01, 0x40, 0},
    {{0x81, 0x00}, 0x00, 0x01, 0x80, 0},
    {{0xC1, 0x00}, 0x00, 0x01, 0xC0, 0},
    {{0x1F, 0x20, 0x00}, 0x00, 0x20, 0x00, 0},
      // Rejected to avoid ambiguity with V_ASN1_NEG. Ruby has a test case expecting this to succeed.
    {{0x1F, 0xC0, 0x20, 0x00}, 0x80, 0x00, 0x00, 0},
    {{0x41, 0x02, 0xAB, 0xCD}, 0x00, 0x01, 0x40, 2},
    {{0x61, 0x00}, 0x20, 0x01, 0x40, 0},
    {{0x61, 0x80, 0xC2, 0x02, 0xAB, 0xCD, 0x00, 0x00}, 0x21, 0x01, 0x40, 0},
};

static void verifyGetObject(const GetObjectTestData& t) {
  long length;
  int tag;
  int tag_class;

  SCOPED_TRACE(Bytes(t.in));
  const uint8_t *ptr = t.in.data();
  EXPECT_EQ(t.expected_return,
            ASN1_get_object(&ptr, &length, &tag, &tag_class, t.in.size()));
  if (!(t.expected_return & 0x80)) {
    EXPECT_EQ(t.expected_length, length);
    EXPECT_EQ(t.expected_tag, tag);
    EXPECT_EQ(t.expected_class, tag_class);
  }
}

TEST(ASN1Test, GetObject) {
  for (const auto &t : kGetObjectTests) {
    verifyGetObject(t);
  }

  {
    GetObjectTestData test_case{ {0x41, 0x81, 0x80}, 0x00, 0x01, 0x40, 128 };
    for(int i = 0; i < 64; i++) {
      test_case.in.push_back(0xAB);
      test_case.in.push_back(0xCD);
    }
    verifyGetObject(test_case);
  }

  {
    GetObjectTestData test_case{ {0x41, 0x82, 0x01, 0x00}, 0x00, 0x01, 0x40, 256 };
    for(int i = 0; i < 128; i++) {
      test_case.in.push_back(0xAB);
      test_case.in.push_back(0xCD);
    }
    verifyGetObject(test_case);
  }

}

// The zero tag, constructed or primitive, is reserved and should rejected by
// the parser.
TEST(ASN1Test, ZeroTag) {
  ExpectParse(d2i_ASN1_TYPE, {0x00, 0x00}, true);
  ExpectParse(d2i_ASN1_TYPE, {0x00, 0x10, 0x00},
              false);  // OpenSSL also rejects this.
  // SEQUENCE {
  //   OBJECT_IDENTIFIER { 1.2.840.113554.4.1.72585.1 }
  //   [UNIVERSAL 0 PRIMITIVE] { "a" }
  // }
  ExpectParse(d2i_X509_ALGOR,
              {0x30, 0x11, 0x06, 0x0c, 0x2a, 0x86, 0x48, 0x86, 0xf7, 0x12, 0x04,
               0x01, 0x84, 0xb7, 0x09, 0x01, 0x00, 0x01, 0x61},
              true);


  // The following test cases are rejected by OpenSSL with their type specific
  // counterparts. They are parsable with |d2i_ASN1_TYPE| however, and we test
  // that later.
  const std::vector<uint8_t> zero_tag_sequence = {0x30, 0x02, 0x00, 0x00};
  const std::vector<uint8_t> zero_tag_set_any = {0x31, 0x02, 0x00, 0x00};
  // Taken from OpenSSL's test/asn1_decode_test.c.
  const std::vector<uint8_t> openssl_t_invalid_zero = {0x31, 0x02, 0x00, 0x00};
  // SEQUENCE {
  //   OBJECT_IDENTIFIER { 1.2.840.113554.4.1.72585.1 }
  //   [UNIVERSAL 0 PRIMITIVE] {}
  // }
  const std::vector<uint8_t> universal_0_primitive_empty = {
      0x30, 0x10, 0x06, 0x0c, 0x2a, 0x86, 0x48, 0x86, 0xf7,
      0x12, 0x04, 0x01, 0x84, 0xb7, 0x09, 0x01, 0x00, 0x00};
  ExpectParse(d2i_ASN1_SEQUENCE_ANY, zero_tag_sequence, false);
  ExpectParse(d2i_ASN1_SET_ANY, zero_tag_set_any, false);
  ExpectParse(d2i_X509_ALGOR, universal_0_primitive_empty, false);
  ExpectParse(d2i_ASN1_SEQUENCE_ANY, openssl_t_invalid_zero, false);
  // Test that the equivalent test cases are parsable with |ASN1_TYPE| (like
  // OpenSSL).
  ExpectParse(d2i_ASN1_TYPE, zero_tag_sequence, true);
  ExpectParse(d2i_ASN1_TYPE, zero_tag_set_any, true);
  ExpectParse(d2i_ASN1_TYPE, universal_0_primitive_empty, true);
  ExpectParse(d2i_ASN1_TYPE, openssl_t_invalid_zero, true);


  // TODO: Change expectation of below to true. Below use BER constructed
  //       strings and will still fail until we revert a70edd4.

  // SEQUENCE {
  //   OBJECT_IDENTIFIER { 1.2.840.113554.4.1.72585.1 }
  //   [UNIVERSAL 0 CONSTRUCTED] {}
  // }
  ExpectParse(d2i_X509_ALGOR,
              {0x30, 0x10, 0x06, 0x0c, 0x2a, 0x86, 0x48, 0x86, 0xf7, 0x12, 0x04,
               0x01, 0x84, 0xb7, 0x09, 0x01, 0x20, 0x00},
              true);

  ExpectParse(d2i_ASN1_TYPE, {0x20, 0x00}, true);
  ExpectParse(d2i_ASN1_TYPE, {0x20, 0x00}, true);
}

TEST(ASN1Test, IndefiniteLength) {
  // Indefinite lengths are more common across container types.
  ExpectParse(d2i_ASN1_SEQUENCE_ANY, {0x30, 0x80, 0x02, 0x01, 0x2a, 0x00, 0x00},
              true);
  ExpectParse(d2i_ASN1_SET_ANY,
              {0x31, 0x80, 0x02, 0x01, 0x01, 0x02, 0x01, 0x02, 0x00, 0x00},
              true);

  // constructed [APPLICATION 31] with an invalid indefinite length ending.
  ExpectParse(d2i_ASN1_TYPE, {0x7f, 0x1f, 0x80, 0x01, 0x01, 0x02, 0x00}, false);
  // Nested constructed [APPLICATION 31] within another constructed
  // [APPLICATION 31] with indefinite lengths.
  ExpectParse(d2i_ASN1_TYPE,
              {0x7f, 0x1f, 0x80, 0x7f, 0x1f, 0x80, 0x03, 0x01, 0x02, 0x00, 0x00,
               0x00, 0x00},
              true);

  // The ones below use constructed form. This is indicated with (0x20 | 0x??)
  // in the first byte.
  ExpectParse(
      d2i_ASN1_BIT_STRING,
      {0x23, 0x80, 0x03, 0x02, 0x00, 0xFF, 0x03, 0x02, 0x00, 0xAA, 0x00, 0x00},
      true);
  ExpectParse(
      d2i_ASN1_OCTET_STRING,
      {0x24, 0x80, 0x04, 0x02, 0x12, 0x34, 0x04, 0x02, 0x56, 0x78, 0x00, 0x00},
      true);
  // |ASN1_INTEGER|s only have primitive encoding. They do not have constructed
  // forms.
  ExpectParse(d2i_ASN1_INTEGER,
            {0x22, 0x80, 0x02, 0x01, 0x12, 0x02, 0x01, 0x34, 0x00, 0x00},
            false);
}

// Exhaustively test POSIX time conversions for every day across the millenium.
TEST(ASN1Test, POSIXTime) {
  const int kDaysInMonth[] = {31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31};

  // Test the epoch explicitly, to confirm our baseline is correct.
  struct tm civil_time;
  ASSERT_TRUE(OPENSSL_posix_to_tm(0, &civil_time));
  ASSERT_EQ(civil_time.tm_year + 1900, 1970);
  ASSERT_EQ(civil_time.tm_mon + 1, 1);
  ASSERT_EQ(civil_time.tm_mday, 1);
  ASSERT_EQ(civil_time.tm_hour, 0);
  ASSERT_EQ(civil_time.tm_min, 0);
  ASSERT_EQ(civil_time.tm_sec, 0);

  int64_t posix_time = -11676096000;  // Sat, 01 Jan 1600 00:00:00 +0000
  for (int year = 1600; year < 3000; year++) {
    SCOPED_TRACE(year);
    bool is_leap_year = (year % 4 == 0 && year % 100 != 0) || year % 400 == 0;
    for (int month = 1; month <= 12; month++) {
      SCOPED_TRACE(month);
      int days = kDaysInMonth[month - 1];
      if (month == 2 && is_leap_year) {
        days++;
      }
      for (int day = 1; day <= days; day++) {
        SCOPED_TRACE(day);
        SCOPED_TRACE(posix_time);

        ASSERT_TRUE(OPENSSL_posix_to_tm(posix_time, &civil_time));
        ASSERT_EQ(civil_time.tm_year + 1900, year);
        ASSERT_EQ(civil_time.tm_mon + 1, month);
        ASSERT_EQ(civil_time.tm_mday, day);
        ASSERT_EQ(civil_time.tm_hour, 0);
        ASSERT_EQ(civil_time.tm_min, 0);
        ASSERT_EQ(civil_time.tm_sec, 0);

        int64_t posix_time_computed;
        ASSERT_TRUE(OPENSSL_tm_to_posix(&civil_time, &posix_time_computed));
        ASSERT_EQ(posix_time_computed, posix_time);

        // Advance to the next day.
        posix_time += 24 * 60 * 60;
      }
    }
  }
}

TEST(ASN1Test, LargeString) {
  bssl::UniquePtr<ASN1_STRING> str(ASN1_STRING_type_new(V_ASN1_OCTET_STRING));
  ASSERT_TRUE(str);
  // Very large strings should be rejected by |ASN1_STRING_set|. Strictly
  // speaking, this is an invalid call because the buffer does not have that
  // much size available. |ASN1_STRING_set| should cleanly fail before it
  // crashes, and actually allocating 512 MiB in a test is likely to break.
  char b = 0;
  EXPECT_FALSE(ASN1_STRING_set(str.get(), &b, INT_MAX / 4));

#if defined(OPENSSL_64_BIT)
  // |ASN1_STRING_set| should tolerate lengths that exceed |int| without
  // overflow.
  EXPECT_FALSE(ASN1_STRING_set(str.get(), &b, 1 + (ossl_ssize_t{1} << 48)));
#endif
}


// Wrapper functions are needed to get around Control Flow Integrity Sanitizers.
static int i2d_ASN1_TYPE_void(const void *a, unsigned char **out) {
  return i2d_ASN1_TYPE((ASN1_TYPE *)a, out);
}
static void *d2i_ASN1_TYPE_void(void **a, const unsigned char **in, long len) {
  return d2i_ASN1_TYPE((ASN1_TYPE **)a, in, len);
}
static int i2d_ECPrivateKey_void(const void *a, unsigned char **out) {
  return i2d_ECPrivateKey((EC_KEY *)a, out);
}
static void *d2i_ECPrivateKey_void(void **a, const unsigned char **in, long len) {
  return d2i_ECPrivateKey((EC_KEY **)a, in, len);
}
static int i2d_X509_PUBKEY_void(const void *a, unsigned char **out) {
  return i2d_X509_PUBKEY((X509_PUBKEY *)a, out);
}
static void *d2i_X509_PUBKEY_void(void **a, const unsigned char **in, long len) {
  return d2i_X509_PUBKEY((X509_PUBKEY **)a, in, len);
}

TEST(ASN1Test, ASN1Dup) {
  const uint8_t *tag = kTag128;
  bssl::UniquePtr<ASN1_TYPE> asn1(
      d2i_ASN1_TYPE(nullptr, &tag, sizeof(kTag128)));
  ASSERT_TRUE(asn1);
  EXPECT_EQ(128, asn1->type);
  bssl::UniquePtr<ASN1_TYPE> asn1_copy((ASN1_TYPE *)ASN1_dup(
      i2d_ASN1_TYPE_void, d2i_ASN1_TYPE_void, asn1.get()));
  ASSERT_TRUE(asn1_copy);
  EXPECT_EQ(ASN1_TYPE_cmp(asn1.get(), asn1_copy.get()), 0);

  bssl::UniquePtr<EC_KEY> key(EC_KEY_new_by_curve_name(NID_X9_62_prime256v1));
  ASSERT_TRUE(key);
  ASSERT_TRUE(EC_KEY_generate_key(key.get()));
  bssl::UniquePtr<EC_KEY> key_copy((EC_KEY *)ASN1_dup(
      i2d_ECPrivateKey_void, d2i_ECPrivateKey_void, key.get()));
  ASSERT_TRUE(key_copy);
  EXPECT_EQ(BN_cmp(EC_KEY_get0_private_key(key.get()),
                   EC_KEY_get0_private_key(key_copy.get())),
            0);
  EXPECT_EQ(EC_GROUP_cmp(EC_KEY_get0_group(key.get()),
                         EC_KEY_get0_group(key_copy.get()), nullptr),
            0);
  EXPECT_EQ(EC_POINT_cmp(EC_KEY_get0_group(key_copy.get()),
                         EC_KEY_get0_public_key(key.get()),
                         EC_KEY_get0_public_key(key_copy.get()), nullptr),
            0);

  bssl::UniquePtr<EVP_PKEY> evp_pkey(EVP_PKEY_new());
  OPENSSL_BEGIN_ALLOW_DEPRECATED
  ASSERT_FALSE(EVP_PKEY_get0(evp_pkey.get()));
  X509_PUBKEY *tmp_key = nullptr;
  ASSERT_TRUE(evp_pkey);
  ASSERT_TRUE(EVP_PKEY_set1_EC_KEY(evp_pkey.get(), key.get()));
  ASSERT_EQ(key.get(), EVP_PKEY_get0(evp_pkey.get()));
  OPENSSL_END_ALLOW_DEPRECATED
  ASSERT_TRUE(X509_PUBKEY_set(&tmp_key, evp_pkey.get()));
  bssl::UniquePtr<X509_PUBKEY> x509_pubkey(tmp_key);
  bssl::UniquePtr<X509_PUBKEY> x509_pubkey_copy((X509_PUBKEY *)ASN1_dup(
      i2d_X509_PUBKEY_void, d2i_X509_PUBKEY_void, x509_pubkey.get()));
  ASSERT_TRUE(x509_pubkey_copy);
  EXPECT_EQ(
      ASN1_STRING_cmp(X509_PUBKEY_get0_public_key(x509_pubkey.get()),
                      X509_PUBKEY_get0_public_key(x509_pubkey_copy.get())),
      0);
}

static std::tuple<int, int, int, int, int, int> TimeToTuple(const tm &t) {
  return std::make_tuple(t.tm_year, t.tm_mon, t.tm_mday, t.tm_hour, t.tm_min,
                         t.tm_sec);
}

TEST(ASN1Test, TimeOverflow) {
  // Input time is out of range and may overflow internal calculations to shift
  // |tm_year| and |tm_mon| to a more normal value.
  tm overflow_year;
  OPENSSL_memset(&overflow_year, 0, sizeof(overflow_year));
  overflow_year.tm_year = INT_MAX - 1899;
  overflow_year.tm_mday = 1;
  tm overflow_month;
  OPENSSL_memset(&overflow_month, 0, sizeof(overflow_month));
  overflow_month.tm_mon = INT_MAX;
  overflow_month.tm_mday = 1;
  int64_t posix_u64;
  EXPECT_FALSE(OPENSSL_tm_to_posix(&overflow_year, &posix_u64));
  EXPECT_FALSE(OPENSSL_tm_to_posix(&overflow_month, &posix_u64));
  time_t posix;
  EXPECT_FALSE(OPENSSL_timegm(&overflow_year, &posix));
  EXPECT_FALSE(OPENSSL_timegm(&overflow_month, &posix));
  EXPECT_FALSE(
      OPENSSL_gmtime_adj(&overflow_year, /*offset_day=*/0, /*offset_sec=*/0));
  EXPECT_FALSE(
      OPENSSL_gmtime_adj(&overflow_month, /*offset_day=*/0, /*offset_sec=*/0));
  int days, secs;
  EXPECT_FALSE(
      OPENSSL_gmtime_diff(&days, &secs, &overflow_year, &overflow_year));
  EXPECT_FALSE(
      OPENSSL_gmtime_diff(&days, &secs, &overflow_month, &overflow_month));

  // Input time is in range, but even adding one second puts it out of range.
  tm max_time;
  OPENSSL_memset(&max_time, 0, sizeof(max_time));
  max_time.tm_year = 9999 - 1900;
  max_time.tm_mon = 12 - 1;
  max_time.tm_mday = 31;
  max_time.tm_hour = 23;
  max_time.tm_min = 59;
  max_time.tm_sec = 59;
  tm copy = max_time;
  EXPECT_TRUE(OPENSSL_gmtime_adj(&copy, /*offset_day=*/0, /*offset_sec=*/0));
  EXPECT_EQ(TimeToTuple(copy), TimeToTuple(max_time));
  EXPECT_FALSE(OPENSSL_gmtime_adj(&copy, /*offset_day=*/0, /*offset_sec=*/1));

  // Likewise for the earliest representable time.
  tm min_time;
  OPENSSL_memset(&min_time, 0, sizeof(min_time));
  min_time.tm_year = 0 - 1900;
  min_time.tm_mon = 1 - 1;
  min_time.tm_mday = 1;
  min_time.tm_hour = 0;
  min_time.tm_min = 0;
  min_time.tm_sec = 0;
  copy = min_time;
  EXPECT_TRUE(OPENSSL_gmtime_adj(&copy, /*offset_day=*/0, /*offset_sec=*/0));
  EXPECT_EQ(TimeToTuple(copy), TimeToTuple(min_time));
  EXPECT_FALSE(OPENSSL_gmtime_adj(&copy, /*offset_day=*/0, /*offset_sec=*/-1));

  // Test we can offset between the minimum and maximum times.
  const int64_t kValidTimeRange = 315569519999;
  copy = min_time;
  EXPECT_TRUE(OPENSSL_gmtime_adj(&copy, /*offset_day=*/0, kValidTimeRange));
  EXPECT_EQ(TimeToTuple(copy), TimeToTuple(max_time));
  EXPECT_TRUE(OPENSSL_gmtime_adj(&copy, /*offset_day=*/0, -kValidTimeRange));
  EXPECT_EQ(TimeToTuple(copy), TimeToTuple(min_time));

  // The second offset may even exceed kValidTimeRange if it is canceled out by
  // offset_day.
  EXPECT_TRUE(OPENSSL_gmtime_adj(&copy, /*offset_day=*/-1,
                                 kValidTimeRange + 24 * 3600));
  EXPECT_EQ(TimeToTuple(copy), TimeToTuple(max_time));
  EXPECT_TRUE(OPENSSL_gmtime_adj(&copy, /*offset_day=*/1,
                                 -kValidTimeRange - 24 * 3600));
  EXPECT_EQ(TimeToTuple(copy), TimeToTuple(min_time));

  // Make sure the internal calculations for |OPENSSL_gmtime_adj| stay in
  // bounds.
  copy = max_time;
  EXPECT_FALSE(OPENSSL_gmtime_adj(&copy, INT_MAX, LONG_MAX));
  copy = min_time;
  EXPECT_FALSE(OPENSSL_gmtime_adj(&copy, INT_MIN, LONG_MIN));
}

// The ASN.1 macros do not work on Windows shared library builds, where usage of
// |OPENSSL_EXPORT| is a bit stricter.
#if !defined(OPENSSL_WINDOWS) || !defined(BORINGSSL_SHARED_LIBRARY)

typedef struct asn1_linked_list_st {
  struct asn1_linked_list_st *next;
} ASN1_LINKED_LIST;

DECLARE_ASN1_ITEM(ASN1_LINKED_LIST)
DECLARE_ASN1_FUNCTIONS(ASN1_LINKED_LIST)

ASN1_SEQUENCE(ASN1_LINKED_LIST) = {
    ASN1_OPT(ASN1_LINKED_LIST, next, ASN1_LINKED_LIST),
} ASN1_SEQUENCE_END(ASN1_LINKED_LIST)

IMPLEMENT_ASN1_FUNCTIONS(ASN1_LINKED_LIST)

static bool MakeLinkedList(bssl::UniquePtr<uint8_t> *out, size_t *out_len,
                           size_t count) {
  bssl::ScopedCBB cbb;
  std::vector<CBB> cbbs(count);
  if (!CBB_init(cbb.get(), 2 * count) ||
      !CBB_add_asn1(cbb.get(), &cbbs[0], CBS_ASN1_SEQUENCE)) {
    return false;
  }
  for (size_t i = 1; i < count; i++) {
    if (!CBB_add_asn1(&cbbs[i - 1], &cbbs[i], CBS_ASN1_SEQUENCE)) {
      return false;
    }
  }
  uint8_t *ptr;
  if (!CBB_finish(cbb.get(), &ptr, out_len)) {
    return false;
  }
  out->reset(ptr);
  return true;
}

TEST(ASN1Test, Recursive) {
  bssl::UniquePtr<uint8_t> data;
  size_t len;

  // Sanity-check that MakeLinkedList can be parsed.
  ASSERT_TRUE(MakeLinkedList(&data, &len, 5));
  const uint8_t *ptr = data.get();
  ASN1_LINKED_LIST *list = d2i_ASN1_LINKED_LIST(nullptr, &ptr, len);
  EXPECT_TRUE(list);
  ASN1_LINKED_LIST_free(list);

  // Excessively deep structures are rejected.
  ASSERT_TRUE(MakeLinkedList(&data, &len, 100));
  ptr = data.get();
  list = d2i_ASN1_LINKED_LIST(nullptr, &ptr, len);
  EXPECT_FALSE(list);
  // Note checking the error queue here does not work. The error "stack trace"
  // is too deep, so the |ASN1_R_NESTED_TOO_DEEP| entry drops off the queue.
  ASN1_LINKED_LIST_free(list);
}

static int i2d_ASN1_LINKED_LIST_void(const void *a, unsigned char **out) {
  return i2d_ASN1_LINKED_LIST((ASN1_LINKED_LIST *)a, out);
}

TEST(ASN1Test, BIO) {
  bssl::UniquePtr<BIO> bio(BIO_new(BIO_s_mem()));
  bssl::UniquePtr<uint8_t> data;
  size_t len;
  ASSERT_TRUE(MakeLinkedList(&data, &len, 5));
  const uint8_t *ptr = data.get();
  ASN1_LINKED_LIST *list = d2i_ASN1_LINKED_LIST(nullptr, &ptr, len);
  EXPECT_TRUE(list);

  // Retrieve expected bytes.
  uint8_t *expected = nullptr;
  int expected_len = i2d_ASN1_LINKED_LIST(list, &expected);
  ASSERT_GT(expected_len, 0);

  const uint8_t *out;
  size_t out_len;
  EXPECT_TRUE(ASN1_i2d_bio(i2d_ASN1_LINKED_LIST_void, bio.get(), list));
  ASSERT_TRUE(BIO_mem_contents(bio.get(), &out, &out_len));

  EXPECT_EQ(Bytes(out, out_len), Bytes(expected, expected_len));
  OPENSSL_free(expected);
  ASN1_LINKED_LIST_free(list);
}

struct IMPLICIT_CHOICE {
  ASN1_STRING *string;
};

DECLARE_ASN1_FUNCTIONS(IMPLICIT_CHOICE)

ASN1_SEQUENCE(IMPLICIT_CHOICE) = {
    ASN1_IMP(IMPLICIT_CHOICE, string, DIRECTORYSTRING, 0),
} ASN1_SEQUENCE_END(IMPLICIT_CHOICE)

IMPLEMENT_ASN1_FUNCTIONS(IMPLICIT_CHOICE)

// Test that the ASN.1 templates reject types with implicitly-tagged CHOICE
// types.
TEST(ASN1Test, ImplicitChoice) {
  // Serializing a type with an implicitly tagged CHOICE should fail.
  std::unique_ptr<IMPLICIT_CHOICE, decltype(&IMPLICIT_CHOICE_free)> obj(
      IMPLICIT_CHOICE_new(), IMPLICIT_CHOICE_free);
  EXPECT_EQ(-1, i2d_IMPLICIT_CHOICE(obj.get(), nullptr));

  // An implicitly-tagged CHOICE is an error. Depending on the implementation,
  // it may be misinterpreted as without the tag, or as clobbering the CHOICE
  // tag. Test both inputs and ensure they fail.

  // SEQUENCE { UTF8String {} }
  static const uint8_t kInput1[] = {0x30, 0x02, 0x0c, 0x00};
  const uint8_t *ptr = kInput1;
  EXPECT_EQ(nullptr, d2i_IMPLICIT_CHOICE(nullptr, &ptr, sizeof(kInput1)));

  // SEQUENCE { [0 PRIMITIVE] {} }
  static const uint8_t kInput2[] = {0x30, 0x02, 0x80, 0x00};
  ptr = kInput2;
  EXPECT_EQ(nullptr, d2i_IMPLICIT_CHOICE(nullptr, &ptr, sizeof(kInput2)));
}

struct REQUIRED_FIELD {
  ASN1_INTEGER *value;
  ASN1_INTEGER *value_imp;
  ASN1_INTEGER *value_exp;
  STACK_OF(ASN1_INTEGER) *seq;
  STACK_OF(ASN1_INTEGER) *seq_imp;
  STACK_OF(ASN1_INTEGER) *seq_exp;
  ASN1_NULL *null;
  ASN1_NULL *null_imp;
  ASN1_NULL *null_exp;
};

DECLARE_ASN1_FUNCTIONS(REQUIRED_FIELD)
ASN1_SEQUENCE(REQUIRED_FIELD) = {
    ASN1_SIMPLE(REQUIRED_FIELD, value, ASN1_INTEGER),
    ASN1_IMP(REQUIRED_FIELD, value_imp, ASN1_INTEGER, 0),
    ASN1_EXP(REQUIRED_FIELD, value_exp, ASN1_INTEGER, 1),
    ASN1_SEQUENCE_OF(REQUIRED_FIELD, seq, ASN1_INTEGER),
    ASN1_IMP_SEQUENCE_OF(REQUIRED_FIELD, seq_imp, ASN1_INTEGER, 2),
    ASN1_EXP_SEQUENCE_OF(REQUIRED_FIELD, seq_exp, ASN1_INTEGER, 3),
    ASN1_SIMPLE(REQUIRED_FIELD, null, ASN1_NULL),
    ASN1_IMP(REQUIRED_FIELD, null_imp, ASN1_NULL, 4),
    ASN1_EXP(REQUIRED_FIELD, null_exp, ASN1_NULL, 5),
} ASN1_SEQUENCE_END(REQUIRED_FIELD)
IMPLEMENT_ASN1_FUNCTIONS(REQUIRED_FIELD)

// Test that structures with missing required fields cannot be serialized. Test
// the full combination of tagging and SEQUENCE OF.
TEST(ASN1Test, MissingRequiredField) {
  EXPECT_EQ(-1, i2d_REQUIRED_FIELD(nullptr, nullptr));

  std::unique_ptr<REQUIRED_FIELD, decltype(&REQUIRED_FIELD_free)> obj(
      nullptr, REQUIRED_FIELD_free);
  for (auto field : {&REQUIRED_FIELD::value, &REQUIRED_FIELD::value_imp,
                     &REQUIRED_FIELD::value_exp}) {
    obj.reset(REQUIRED_FIELD_new());
    ASSERT_TRUE(obj);
    ASN1_INTEGER_free((*obj).*field);
    (*obj).*field = nullptr;
    EXPECT_EQ(-1, i2d_REQUIRED_FIELD(obj.get(), nullptr));
  }

  for (auto field : {&REQUIRED_FIELD::seq, &REQUIRED_FIELD::seq_imp,
                     &REQUIRED_FIELD::seq_exp}) {
    obj.reset(REQUIRED_FIELD_new());
    ASSERT_TRUE(obj);
    sk_ASN1_INTEGER_pop_free((*obj).*field, ASN1_INTEGER_free);
    (*obj).*field = nullptr;
    EXPECT_EQ(-1, i2d_REQUIRED_FIELD(obj.get(), nullptr));
  }

  for (auto field : {&REQUIRED_FIELD::null, &REQUIRED_FIELD::null_imp,
                     &REQUIRED_FIELD::null_exp}) {
    obj.reset(REQUIRED_FIELD_new());
    ASSERT_TRUE(obj);
    (*obj).*field = nullptr;
    EXPECT_EQ(-1, i2d_REQUIRED_FIELD(obj.get(), nullptr));
  }
}

struct BOOLEANS {
  ASN1_BOOLEAN required;
  ASN1_BOOLEAN optional;
  ASN1_BOOLEAN default_true;
  ASN1_BOOLEAN default_false;
};

DECLARE_ASN1_FUNCTIONS(BOOLEANS)
ASN1_SEQUENCE(BOOLEANS) = {
    ASN1_SIMPLE(BOOLEANS, required, ASN1_BOOLEAN),
    ASN1_IMP_OPT(BOOLEANS, optional, ASN1_BOOLEAN, 1),
    // Although not actually optional, |ASN1_TBOOLEAN| and |ASN1_FBOOLEAN| need
    // to be marked optional in the template.
    ASN1_IMP_OPT(BOOLEANS, default_true, ASN1_TBOOLEAN, 2),
    ASN1_IMP_OPT(BOOLEANS, default_false, ASN1_FBOOLEAN, 3),
} ASN1_SEQUENCE_END(BOOLEANS)
IMPLEMENT_ASN1_FUNCTIONS(BOOLEANS)

TEST(ASN1Test, OptionalAndDefaultBooleans) {
  std::unique_ptr<BOOLEANS, decltype(&BOOLEANS_free)> obj(nullptr,
                                                          BOOLEANS_free);

  // A default-constructed object should use, respectively, omitted, omitted,
  // TRUE, FALSE.
  //
  // TODO(davidben): Is the first one a bug? It seems more consistent for a
  // required BOOLEAN default to FALSE. |FOO_new| typically default-initializes
  // fields valid states. (Though there are exceptions. CHOICE, ANY, and OBJECT
  // IDENTIFIER are default-initialized to something invalid.)
  obj.reset(BOOLEANS_new());
  ASSERT_TRUE(obj);
  EXPECT_EQ(obj->required, ASN1_BOOLEAN_NONE);
  EXPECT_EQ(obj->optional, ASN1_BOOLEAN_NONE);
  EXPECT_EQ(obj->default_true, ASN1_BOOLEAN_TRUE);
  EXPECT_EQ(obj->default_false, ASN1_BOOLEAN_FALSE);

  // Trying to serialize this should fail, because |obj->required| is omitted.
  EXPECT_EQ(-1, i2d_BOOLEANS(obj.get(), nullptr));

  // Otherwise, this object is serializable. Most fields are omitted, due to
  // them being optional or defaulted.
  static const uint8_t kFieldsOmitted[] = {0x30, 0x03, 0x01, 0x01, 0x00};
  obj->required = 0;
  TestSerialize(obj.get(), i2d_BOOLEANS, kFieldsOmitted);

  const uint8_t *der = kFieldsOmitted;
  obj.reset(d2i_BOOLEANS(nullptr, &der, sizeof(kFieldsOmitted)));
  ASSERT_TRUE(obj);
  EXPECT_EQ(obj->required, ASN1_BOOLEAN_FALSE);
  EXPECT_EQ(obj->optional, ASN1_BOOLEAN_NONE);
  EXPECT_EQ(obj->default_true, ASN1_BOOLEAN_TRUE);
  EXPECT_EQ(obj->default_false, ASN1_BOOLEAN_FALSE);

  // Include the optinonal fields instead.
  static const uint8_t kFieldsIncluded[] = {0x30, 0x0c, 0x01, 0x01, 0xff,
                                            0x81, 0x01, 0x00, 0x82, 0x01,
                                            0x00, 0x83, 0x01, 0xff};
  obj->required = ASN1_BOOLEAN_TRUE;
  obj->optional = ASN1_BOOLEAN_FALSE;
  obj->default_true = ASN1_BOOLEAN_FALSE;
  obj->default_false = ASN1_BOOLEAN_TRUE;
  TestSerialize(obj.get(), i2d_BOOLEANS, kFieldsIncluded);

  der = kFieldsIncluded;
  obj.reset(d2i_BOOLEANS(nullptr, &der, sizeof(kFieldsIncluded)));
  ASSERT_TRUE(obj);
  EXPECT_EQ(obj->required, ASN1_BOOLEAN_TRUE);
  EXPECT_EQ(obj->optional, ASN1_BOOLEAN_FALSE);
  EXPECT_EQ(obj->default_true, ASN1_BOOLEAN_FALSE);
  EXPECT_EQ(obj->default_false, ASN1_BOOLEAN_TRUE);

  // TODO(https://crbug.com/boringssl/354): Reject explicit DEFAULTs.
}

// EXPLICIT_BOOLEAN is a [1] EXPLICIT BOOLEAN.
ASN1_ITEM_TEMPLATE(EXPLICIT_BOOLEAN) = ASN1_EX_TEMPLATE_TYPE(ASN1_TFLG_EXPLICIT,
                                                             1,
                                                             EXPLICIT_BOOLEAN,
                                                             ASN1_BOOLEAN)
ASN1_ITEM_TEMPLATE_END(EXPLICIT_BOOLEAN)

// EXPLICIT_OCTET_STRING is a [2] EXPLICIT OCTET STRING.
ASN1_ITEM_TEMPLATE(EXPLICIT_OCTET_STRING) = ASN1_EX_TEMPLATE_TYPE(
    ASN1_TFLG_EXPLICIT, 2, EXPLICIT_OCTET_STRING, ASN1_OCTET_STRING)
ASN1_ITEM_TEMPLATE_END(EXPLICIT_OCTET_STRING)

// DOUBLY_TAGGED is
//   SEQUENCE {
//     b   [3] EXPLICIT [1] EXPLICIT BOOLEAN OPTIONAL,
//     oct [4] EXPLICIT [2] EXPLICIT OCTET STRING OPTIONAL }
// with explicit tagging.
struct DOUBLY_TAGGED {
  ASN1_BOOLEAN b;
  ASN1_OCTET_STRING *oct;
};

DECLARE_ASN1_FUNCTIONS(DOUBLY_TAGGED)
ASN1_SEQUENCE(DOUBLY_TAGGED) = {
    ASN1_EXP_OPT(DOUBLY_TAGGED, b, EXPLICIT_BOOLEAN, 3),
    ASN1_EXP_OPT(DOUBLY_TAGGED, oct, EXPLICIT_OCTET_STRING, 4),
} ASN1_SEQUENCE_END(DOUBLY_TAGGED)
IMPLEMENT_ASN1_FUNCTIONS(DOUBLY_TAGGED)

// Test that optional fields with two layers of explicit tagging are correctly
// handled.
TEST(ASN1Test, DoublyTagged) {
  std::unique_ptr<DOUBLY_TAGGED, decltype(&DOUBLY_TAGGED_free)> obj(
      nullptr, DOUBLY_TAGGED_free);

  // Both fields missing.
  static const uint8_t kOmitted[] = {0x30, 0x00};
  const uint8_t *inp = kOmitted;
  obj.reset(d2i_DOUBLY_TAGGED(nullptr, &inp, sizeof(kOmitted)));
  ASSERT_TRUE(obj);
  EXPECT_EQ(obj->b, -1);
  EXPECT_FALSE(obj->oct);
  TestSerialize(obj.get(), i2d_DOUBLY_TAGGED, kOmitted);

  // Both fields present, true and the empty string.
  static const uint8_t kTrueEmpty[] = {0x30, 0x0d, 0xa3, 0x05, 0xa1,
                                       0x03, 0x01, 0x01, 0xff, 0xa4,
                                       0x04, 0xa2, 0x02, 0x04, 0x00};
  inp = kTrueEmpty;
  obj.reset(d2i_DOUBLY_TAGGED(nullptr, &inp, sizeof(kTrueEmpty)));
  ASSERT_TRUE(obj);
  EXPECT_EQ(obj->b, 0xff);
  ASSERT_TRUE(obj->oct);
  EXPECT_EQ(ASN1_STRING_length(obj->oct), 0);
  TestSerialize(obj.get(), i2d_DOUBLY_TAGGED, kTrueEmpty);
}

#define CHOICE_TYPE_OCT 0
#define CHOICE_TYPE_BOOL 1

struct CHOICE_TYPE {
  int type;
  union {
    ASN1_OCTET_STRING *oct;
    ASN1_BOOLEAN b;
  } value;
};

DECLARE_ASN1_FUNCTIONS(CHOICE_TYPE)
ASN1_CHOICE(CHOICE_TYPE) = {
    ASN1_SIMPLE(CHOICE_TYPE, value.oct, ASN1_OCTET_STRING),
    ASN1_SIMPLE(CHOICE_TYPE, value.b, ASN1_BOOLEAN),
} ASN1_CHOICE_END(CHOICE_TYPE)
IMPLEMENT_ASN1_FUNCTIONS(CHOICE_TYPE)

struct OPTIONAL_CHOICE {
  CHOICE_TYPE *choice;
};

DECLARE_ASN1_FUNCTIONS(OPTIONAL_CHOICE)
ASN1_SEQUENCE(OPTIONAL_CHOICE) = {
    ASN1_OPT(OPTIONAL_CHOICE, choice, CHOICE_TYPE),
} ASN1_SEQUENCE_END(OPTIONAL_CHOICE)
IMPLEMENT_ASN1_FUNCTIONS(OPTIONAL_CHOICE)

TEST(ASN1Test, OptionalChoice) {
  std::unique_ptr<OPTIONAL_CHOICE, decltype(&OPTIONAL_CHOICE_free)> obj(
      nullptr, OPTIONAL_CHOICE_free);

  // Value omitted.
  static const uint8_t kOmitted[] = {0x30, 0x00};
  const uint8_t *inp = kOmitted;
  obj.reset(d2i_OPTIONAL_CHOICE(nullptr, &inp, sizeof(kOmitted)));
  ASSERT_TRUE(obj);
  EXPECT_FALSE(obj->choice);
  TestSerialize(obj.get(), i2d_OPTIONAL_CHOICE, kOmitted);

  // Value is present as an OCTET STRING.
  static const uint8_t kOct[] = {0x30, 0x02, 0x04, 0x00};
  inp = kOct;
  obj.reset(d2i_OPTIONAL_CHOICE(nullptr, &inp, sizeof(kOct)));
  ASSERT_TRUE(obj);
  ASSERT_TRUE(obj->choice);
  ASSERT_EQ(obj->choice->type, CHOICE_TYPE_OCT);
  ASSERT_TRUE(obj->choice->value.oct);
  EXPECT_EQ(ASN1_STRING_length(obj->choice->value.oct), 0);
  TestSerialize(obj.get(), i2d_OPTIONAL_CHOICE, kOct);

  // Value is present as TRUE.
  static const uint8_t kTrue[] = {0x30, 0x03, 0x01, 0x01, 0xff};
  inp = kTrue;
  obj.reset(d2i_OPTIONAL_CHOICE(nullptr, &inp, sizeof(kTrue)));
  ASSERT_TRUE(obj);
  ASSERT_TRUE(obj->choice);
  ASSERT_EQ(obj->choice->type, CHOICE_TYPE_BOOL);
  EXPECT_EQ(obj->choice->value.b, ASN1_BOOLEAN_TRUE);
  TestSerialize(obj.get(), i2d_OPTIONAL_CHOICE, kTrue);
}

struct EMBED_X509_ALGOR {
  X509_ALGOR *simple;
  X509_ALGOR *opt;
  STACK_OF(X509_ALGOR) *seq;
};

struct EMBED_X509_EXTENSION {
  X509_EXTENSION *simple;
  X509_EXTENSION *opt;
  STACK_OF(X509_EXTENSION) *seq;
};

struct EMBED_X509_NAME {
  X509_NAME *simple;
  X509_NAME *opt;
  STACK_OF(X509_NAME) *seq;
};

struct EMBED_X509 {
  X509 *simple;
  X509 *opt;
  STACK_OF(X509) *seq;
};

DECLARE_ASN1_FUNCTIONS(EMBED_X509_ALGOR)
ASN1_SEQUENCE(EMBED_X509_ALGOR) = {
    ASN1_SIMPLE(EMBED_X509_ALGOR, simple, X509_ALGOR),
    ASN1_EXP_OPT(EMBED_X509_ALGOR, opt, X509_ALGOR, 0),
    ASN1_IMP_SEQUENCE_OF_OPT(EMBED_X509_ALGOR, seq, X509_ALGOR, 1),
} ASN1_SEQUENCE_END(EMBED_X509_ALGOR)
IMPLEMENT_ASN1_FUNCTIONS(EMBED_X509_ALGOR)

DECLARE_ASN1_FUNCTIONS(EMBED_X509_NAME)
ASN1_SEQUENCE(EMBED_X509_NAME) = {
    ASN1_SIMPLE(EMBED_X509_NAME, simple, X509_NAME),
    ASN1_EXP_OPT(EMBED_X509_NAME, opt, X509_NAME, 0),
    ASN1_IMP_SEQUENCE_OF_OPT(EMBED_X509_NAME, seq, X509_NAME, 1),
} ASN1_SEQUENCE_END(EMBED_X509_NAME)
IMPLEMENT_ASN1_FUNCTIONS(EMBED_X509_NAME)

DECLARE_ASN1_FUNCTIONS(EMBED_X509_EXTENSION)
ASN1_SEQUENCE(EMBED_X509_EXTENSION) = {
    ASN1_SIMPLE(EMBED_X509_EXTENSION, simple, X509_EXTENSION),
    ASN1_EXP_OPT(EMBED_X509_EXTENSION, opt, X509_EXTENSION, 0),
    ASN1_IMP_SEQUENCE_OF_OPT(EMBED_X509_EXTENSION, seq, X509_EXTENSION, 1),
} ASN1_SEQUENCE_END(EMBED_X509_EXTENSION)
IMPLEMENT_ASN1_FUNCTIONS(EMBED_X509_EXTENSION)

DECLARE_ASN1_FUNCTIONS(EMBED_X509)
ASN1_SEQUENCE(EMBED_X509) = {
    ASN1_SIMPLE(EMBED_X509, simple, X509),
    ASN1_EXP_OPT(EMBED_X509, opt, X509, 0),
    ASN1_IMP_SEQUENCE_OF_OPT(EMBED_X509, seq, X509, 1),
} ASN1_SEQUENCE_END(EMBED_X509)
IMPLEMENT_ASN1_FUNCTIONS(EMBED_X509)

template <typename EmbedT, typename T, typename MaybeConstT, typename StackT>
void TestEmbedType(bssl::Span<const uint8_t> inp,
                   int (*i2d)(MaybeConstT *, uint8_t **),
                   EmbedT *(*embed_new)(), void (*embed_free)(EmbedT *),
                   EmbedT *(*d2i_embed)(EmbedT **, const uint8_t **, long),
                   int (*i2d_embed)(EmbedT *, uint8_t **),
                   size_t (*sk_num)(const StackT *),
                   T *(*sk_value)(const StackT *, size_t)) {
  std::unique_ptr<EmbedT, decltype(embed_free)> obj(nullptr, embed_free);

  // Test only the first field present.
  bssl::ScopedCBB cbb;
  ASSERT_TRUE(CBB_init(cbb.get(), 64));
  CBB seq;
  ASSERT_TRUE(CBB_add_asn1(cbb.get(), &seq, CBS_ASN1_SEQUENCE));
  ASSERT_TRUE(CBB_add_bytes(&seq, inp.data(), inp.size()));
  ASSERT_TRUE(CBB_flush(cbb.get()));
  const uint8_t *ptr = CBB_data(cbb.get());
  obj.reset(d2i_embed(nullptr, &ptr, CBB_len(cbb.get())));
  ASSERT_TRUE(obj);
  ASSERT_TRUE(obj->simple);
  // Test the field was parsed correctly by reserializing it.
  TestSerialize(obj->simple, i2d, inp);
  EXPECT_FALSE(obj->opt);
  EXPECT_FALSE(obj->seq);
  TestSerialize(obj.get(), i2d_embed,
                {CBB_data(cbb.get()), CBB_len(cbb.get())});

  // Test all fields present.
  cbb.Reset();
  ASSERT_TRUE(CBB_init(cbb.get(), 64));
  ASSERT_TRUE(CBB_add_asn1(cbb.get(), &seq, CBS_ASN1_SEQUENCE));
  ASSERT_TRUE(CBB_add_bytes(&seq, inp.data(), inp.size()));
  CBB child;
  ASSERT_TRUE(CBB_add_asn1(
      &seq, &child, CBS_ASN1_CONTEXT_SPECIFIC | CBS_ASN1_CONSTRUCTED | 0));
  ASSERT_TRUE(CBB_add_bytes(&child, inp.data(), inp.size()));
  ASSERT_TRUE(CBB_add_asn1(
      &seq, &child, CBS_ASN1_CONTEXT_SPECIFIC | CBS_ASN1_CONSTRUCTED | 1));
  ASSERT_TRUE(CBB_add_bytes(&child, inp.data(), inp.size()));
  ASSERT_TRUE(CBB_add_bytes(&child, inp.data(), inp.size()));
  ASSERT_TRUE(CBB_flush(cbb.get()));
  ptr = CBB_data(cbb.get());
  obj.reset(d2i_embed(nullptr, &ptr, CBB_len(cbb.get())));
  ASSERT_TRUE(obj);
  ASSERT_TRUE(obj->simple);
  TestSerialize(obj->simple, i2d, inp);
  ASSERT_TRUE(obj->opt);
  TestSerialize(obj->opt, i2d, inp);
  ASSERT_EQ(sk_num(obj->seq), 2u);
  TestSerialize(sk_value(obj->seq, 0), i2d, inp);
  TestSerialize(sk_value(obj->seq, 1), i2d, inp);
  TestSerialize(obj.get(), i2d_embed,
                {CBB_data(cbb.get()), CBB_len(cbb.get())});
}

// Test that X.509 types defined in this library can be embedded into other
// types, as we rewrite them away from the templating system.
TEST(ASN1Test, EmbedTypes) {
  static const uint8_t kTestAlg[] = {0x30, 0x09, 0x06, 0x07, 0x2a, 0x86,
                                     0x48, 0xce, 0x3d, 0x04, 0x01};
  TestEmbedType(kTestAlg, i2d_X509_ALGOR, EMBED_X509_ALGOR_new,
                EMBED_X509_ALGOR_free, d2i_EMBED_X509_ALGOR,
                i2d_EMBED_X509_ALGOR, sk_X509_ALGOR_num, sk_X509_ALGOR_value);

  static const uint8_t kTestName[] = {
      0x30, 0x45, 0x31, 0x0b, 0x30, 0x09, 0x06, 0x03, 0x55, 0x04, 0x06, 0x13,
      0x02, 0x41, 0x55, 0x31, 0x13, 0x30, 0x11, 0x06, 0x03, 0x55, 0x04, 0x08,
      0x0c, 0x0a, 0x53, 0x6f, 0x6d, 0x65, 0x2d, 0x53, 0x74, 0x61, 0x74, 0x65,
      0x31, 0x21, 0x30, 0x1f, 0x06, 0x03, 0x55, 0x04, 0x0a, 0x0c, 0x18, 0x49,
      0x6e, 0x74, 0x65, 0x72, 0x6e, 0x65, 0x74, 0x20, 0x57, 0x69, 0x64, 0x67,
      0x69, 0x74, 0x73, 0x20, 0x50, 0x74, 0x79, 0x20, 0x4c, 0x74, 0x64};
  TestEmbedType(kTestName, i2d_X509_NAME, EMBED_X509_NAME_new,
                EMBED_X509_NAME_free, d2i_EMBED_X509_NAME, i2d_EMBED_X509_NAME,
                sk_X509_NAME_num, sk_X509_NAME_value);

  static const uint8_t kTestExtension[] = {0x30, 0x0c, 0x06, 0x03, 0x55,
                                           0x1d, 0x13, 0x04, 0x05, 0x30,
                                           0x03, 0x01, 0x01, 0xf};
  TestEmbedType(kTestExtension, i2d_X509_EXTENSION, EMBED_X509_EXTENSION_new,
                EMBED_X509_EXTENSION_free, d2i_EMBED_X509_EXTENSION,
                i2d_EMBED_X509_EXTENSION, sk_X509_EXTENSION_num,
                sk_X509_EXTENSION_value);

  static const uint8_t kTestCert[] = {
      0x30, 0x82, 0x01, 0xcf, 0x30, 0x82, 0x01, 0x76, 0xa0, 0x03, 0x02, 0x01,
      0x02, 0x02, 0x09, 0x00, 0xd9, 0x4c, 0x04, 0xda, 0x49, 0x7d, 0xbf, 0xeb,
      0x30, 0x09, 0x06, 0x07, 0x2a, 0x86, 0x48, 0xce, 0x3d, 0x04, 0x01, 0x30,
      0x45, 0x31, 0x0b, 0x30, 0x09, 0x06, 0x03, 0x55, 0x04, 0x06, 0x13, 0x02,
      0x41, 0x55, 0x31, 0x13, 0x30, 0x11, 0x06, 0x03, 0x55, 0x04, 0x08, 0x0c,
      0x0a, 0x53, 0x6f, 0x6d, 0x65, 0x2d, 0x53, 0x74, 0x61, 0x74, 0x65, 0x31,
      0x21, 0x30, 0x1f, 0x06, 0x03, 0x55, 0x04, 0x0a, 0x0c, 0x18, 0x49, 0x6e,
      0x74, 0x65, 0x72, 0x6e, 0x65, 0x74, 0x20, 0x57, 0x69, 0x64, 0x67, 0x69,
      0x74, 0x73, 0x20, 0x50, 0x74, 0x79, 0x20, 0x4c, 0x74, 0x64, 0x30, 0x1e,
      0x17, 0x0d, 0x31, 0x34, 0x30, 0x34, 0x32, 0x33, 0x32, 0x33, 0x32, 0x31,
      0x35, 0x37, 0x5a, 0x17, 0x0d, 0x31, 0x34, 0x30, 0x35, 0x32, 0x33, 0x32,
      0x33, 0x32, 0x31, 0x35, 0x37, 0x5a, 0x30, 0x45, 0x31, 0x0b, 0x30, 0x09,
      0x06, 0x03, 0x55, 0x04, 0x06, 0x13, 0x02, 0x41, 0x55, 0x31, 0x13, 0x30,
      0x11, 0x06, 0x03, 0x55, 0x04, 0x08, 0x0c, 0x0a, 0x53, 0x6f, 0x6d, 0x65,
      0x2d, 0x53, 0x74, 0x61, 0x74, 0x65, 0x31, 0x21, 0x30, 0x1f, 0x06, 0x03,
      0x55, 0x04, 0x0a, 0x0c, 0x18, 0x49, 0x6e, 0x74, 0x65, 0x72, 0x6e, 0x65,
      0x74, 0x20, 0x57, 0x69, 0x64, 0x67, 0x69, 0x74, 0x73, 0x20, 0x50, 0x74,
      0x79, 0x20, 0x4c, 0x74, 0x64, 0x30, 0x59, 0x30, 0x13, 0x06, 0x07, 0x2a,
      0x86, 0x48, 0xce, 0x3d, 0x02, 0x01, 0x06, 0x08, 0x2a, 0x86, 0x48, 0xce,
      0x3d, 0x03, 0x01, 0x07, 0x03, 0x42, 0x00, 0x04, 0xe6, 0x2b, 0x69, 0xe2,
      0xbf, 0x65, 0x9f, 0x97, 0xbe, 0x2f, 0x1e, 0x0d, 0x94, 0x8a, 0x4c, 0xd5,
      0x97, 0x6b, 0xb7, 0xa9, 0x1e, 0x0d, 0x46, 0xfb, 0xdd, 0xa9, 0xa9, 0x1e,
      0x9d, 0xdc, 0xba, 0x5a, 0x01, 0xe7, 0xd6, 0x97, 0xa8, 0x0a, 0x18, 0xf9,
      0xc3, 0xc4, 0xa3, 0x1e, 0x56, 0xe2, 0x7c, 0x83, 0x48, 0xdb, 0x16, 0x1a,
      0x1c, 0xf5, 0x1d, 0x7e, 0xf1, 0x94, 0x2d, 0x4b, 0xcf, 0x72, 0x22, 0xc1,
      0xa3, 0x50, 0x30, 0x4e, 0x30, 0x1d, 0x06, 0x03, 0x55, 0x1d, 0x0e, 0x04,
      0x16, 0x04, 0x14, 0xab, 0x84, 0xd2, 0xac, 0xab, 0x95, 0xf0, 0x82, 0x4e,
      0x16, 0x78, 0x07, 0x55, 0x57, 0x5f, 0xe4, 0x26, 0x8d, 0x82, 0xd1, 0x30,
      0x1f, 0x06, 0x03, 0x55, 0x1d, 0x23, 0x04, 0x18, 0x30, 0x16, 0x80, 0x14,
      0xab, 0x84, 0xd2, 0xac, 0xab, 0x95, 0xf0, 0x82, 0x4e, 0x16, 0x78, 0x07,
      0x55, 0x57, 0x5f, 0xe4, 0x26, 0x8d, 0x82, 0xd1, 0x30, 0x0c, 0x06, 0x03,
      0x55, 0x1d, 0x13, 0x04, 0x05, 0x30, 0x03, 0x01, 0x01, 0xff, 0x30, 0x09,
      0x06, 0x07, 0x2a, 0x86, 0x48, 0xce, 0x3d, 0x04, 0x01, 0x03, 0x48, 0x00,
      0x30, 0x45, 0x02, 0x21, 0x00, 0xf2, 0xa0, 0x35, 0x5e, 0x51, 0x3a, 0x36,
      0xc3, 0x82, 0x79, 0x9b, 0xee, 0x27, 0x50, 0x85, 0x8e, 0x70, 0x06, 0x74,
      0x95, 0x57, 0xd2, 0x29, 0x74, 0x00, 0xf4, 0xbe, 0x15, 0x87, 0x5d, 0xc4,
      0x07, 0x02, 0x20, 0x7c, 0x1e, 0x79, 0x14, 0x6a, 0x21, 0x83, 0xf0, 0x7a,
      0x74, 0x68, 0x79, 0x5f, 0x14, 0x99, 0x9a, 0x68, 0xb4, 0xf1, 0xcb, 0x9e,
      0x15, 0x5e, 0xe6, 0x1f, 0x32, 0x52, 0x61, 0x5e, 0x75, 0xc9, 0x14};
  TestEmbedType(kTestCert, i2d_X509, EMBED_X509_new, EMBED_X509_free,
                d2i_EMBED_X509, i2d_EMBED_X509, sk_X509_num, sk_X509_value);
}

TEST(ASN1Test, A2iASN1Integer) {
  // Test basic a2i_ASN1_INTEGER functionality - parsing hex strings from BIO
  const struct {
    const char *input;
    std::vector<uint8_t> expected_data;
    int expected_type;
  } kBasicTests[] = {
      // Simple hex values - note: leading "00" is stripped from the first line
      {"01", {0x01}, V_ASN1_INTEGER},
      {"FF", {0xff}, V_ASN1_INTEGER},
      {"AABB", {0xaa, 0xbb}, V_ASN1_INTEGER},
      {"AABBCC", {0xaa, 0xbb, 0xcc}, V_ASN1_INTEGER},
      {"0123456789ABCDEF", {0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef},
       V_ASN1_INTEGER},
      // Leading "00" is stripped; "0000FF" becomes "00FF" = {0x00, 0xff}
      {"0000FF", {0x00, 0xff}, V_ASN1_INTEGER},
  };

  for (const auto &t : kBasicTests) {
    SCOPED_TRACE(t.input);
    bssl::UniquePtr<BIO> bio(BIO_new_mem_buf(t.input, strlen(t.input)));
    ASSERT_TRUE(bio);

    bssl::UniquePtr<ASN1_INTEGER> val(ASN1_INTEGER_new());
    ASSERT_TRUE(val);

    char buf[256];
    int ret = a2i_ASN1_INTEGER(bio.get(), val.get(), buf, sizeof(buf));
    ASSERT_EQ(ret, 1);
    EXPECT_EQ(val->type, t.expected_type);
    EXPECT_EQ(Bytes(val->data, val->length), Bytes(t.expected_data));
  }

  // "00" alone results in empty data after stripping leading "00"
  {
    const char *input = "00";
    bssl::UniquePtr<BIO> bio(BIO_new_mem_buf(input, strlen(input)));
    ASSERT_TRUE(bio);
    bssl::UniquePtr<ASN1_INTEGER> val(ASN1_INTEGER_new());
    ASSERT_TRUE(val);
    char buf[256];
    int ret = a2i_ASN1_INTEGER(bio.get(), val.get(), buf, sizeof(buf));
    ASSERT_EQ(ret, 1);
    // After stripping "00", we have nothing left, so length is 0
    EXPECT_EQ(val->length, 0);
  }

  // Test invalid inputs
  const char *kInvalidTests[] = {
      "",      // Empty
      "A",     // Odd number of hex chars
      "ABC",   // Odd number of hex chars
      "GG",    // Invalid hex chars
  };
  for (const char *input : kInvalidTests) {
    SCOPED_TRACE(input);
    bssl::UniquePtr<BIO> bio(BIO_new_mem_buf(input, strlen(input)));
    ASSERT_TRUE(bio);

    bssl::UniquePtr<ASN1_INTEGER> val(ASN1_INTEGER_new());
    ASSERT_TRUE(val);

    char buf[256];
    int ret = a2i_ASN1_INTEGER(bio.get(), val.get(), buf, sizeof(buf));
    EXPECT_EQ(ret, 0);
    ERR_clear_error();
  }
}

TEST(ASN1Test, A2iASN1IntegerLineContinuation) {
  // Test line continuation with backslash.
  // i2a_ASN1_INTEGER outputs "\\\n" every 35 bytes (70 hex chars).
  // a2i_ASN1_INTEGER should handle this continuation.

  // Simple continuation test: "AABB\\\nCCDD" should parse as 0xAABBCCDD
  {
    const char *input = "AABB\\\nCCDD";
    bssl::UniquePtr<BIO> bio(BIO_new_mem_buf(input, strlen(input)));
    ASSERT_TRUE(bio);

    bssl::UniquePtr<ASN1_INTEGER> val(ASN1_INTEGER_new());
    ASSERT_TRUE(val);

    char buf[256];
    int ret = a2i_ASN1_INTEGER(bio.get(), val.get(), buf, sizeof(buf));
    ASSERT_EQ(ret, 1);
    static const uint8_t kExpected[] = {0xaa, 0xbb, 0xcc, 0xdd};
    EXPECT_EQ(Bytes(val->data, val->length), Bytes(kExpected));
  }

  // Test with longer continuation
  {
    const char *input = "112233445566\\\n778899AABB";
    bssl::UniquePtr<BIO> bio(BIO_new_mem_buf(input, strlen(input)));
    ASSERT_TRUE(bio);

    bssl::UniquePtr<ASN1_INTEGER> val(ASN1_INTEGER_new());
    ASSERT_TRUE(val);

    char buf[256];
    int ret = a2i_ASN1_INTEGER(bio.get(), val.get(), buf, sizeof(buf));
    ASSERT_EQ(ret, 1);
    static const uint8_t kExpected[] = {0x11, 0x22, 0x33, 0x44, 0x55,
                                        0x66, 0x77, 0x88, 0x99, 0xaa, 0xbb};
    EXPECT_EQ(Bytes(val->data, val->length), Bytes(kExpected));
  }
}

TEST(ASN1Test, I2aA2iASN1IntegerRoundTrip) {
  // Test round-trip: i2a_ASN1_INTEGER -> a2i_ASN1_INTEGER
  // For values small enough to not trigger line continuation, this should work.
  //
  // Note: {0x00} is intentionally excluded because a2i_ASN1_INTEGER strips
  // leading "00" from the first line, so "00" -> empty data. This is expected
  // behavior, not a bug.
  const std::vector<uint8_t> kTestValues[] = {
      {0x01},
      {0x7f},
      {0x80},
      {0xff},
      {0x01, 0x00},
      {0xde, 0xad, 0xbe, 0xef},
  };

  for (const auto &test_data : kTestValues) {
    SCOPED_TRACE(Bytes(test_data));

    // Create an ASN1_INTEGER with the test data
    bssl::UniquePtr<ASN1_INTEGER> orig(ASN1_INTEGER_new());
    ASSERT_TRUE(orig);
    ASSERT_TRUE(ASN1_STRING_set(orig.get(), test_data.data(), test_data.size()));

    // Serialize to hex via i2a_ASN1_INTEGER
    bssl::UniquePtr<BIO> out_bio(BIO_new(BIO_s_mem()));
    ASSERT_TRUE(out_bio);
    int written = i2a_ASN1_INTEGER(out_bio.get(), orig.get());
    ASSERT_GT(written, 0);

    // Get the hex string
    const uint8_t *hex_data = nullptr;
    size_t hex_len = 0;
    ASSERT_TRUE(BIO_mem_contents(out_bio.get(), &hex_data, &hex_len));

    // Parse it back via a2i_ASN1_INTEGER
    bssl::UniquePtr<BIO> in_bio(
        BIO_new_mem_buf(hex_data, static_cast<int>(hex_len)));
    ASSERT_TRUE(in_bio);

    bssl::UniquePtr<ASN1_INTEGER> parsed(ASN1_INTEGER_new());
    ASSERT_TRUE(parsed);

    char buf[256];
    int ret = a2i_ASN1_INTEGER(in_bio.get(), parsed.get(), buf, sizeof(buf));
    ASSERT_EQ(ret, 1);

    // The parsed value should match the original
    EXPECT_EQ(Bytes(orig->data, orig->length),
              Bytes(parsed->data, parsed->length));
  }
}

TEST(ASN1Test, I2aA2iASN1IntegerRoundTripLong) {
  // Test round-trip with a value long enough to trigger line continuation.
  // i2a_ASN1_INTEGER adds "\\\n" every 35 bytes.
  // We need > 35 bytes to trigger continuation.

  // Create a 40-byte value
  std::vector<uint8_t> long_value(40);
  for (size_t i = 0; i < long_value.size(); i++) {
    long_value[i] = static_cast<uint8_t>(i + 1);
  }

  bssl::UniquePtr<ASN1_INTEGER> orig(ASN1_INTEGER_new());
  ASSERT_TRUE(orig);
  ASSERT_TRUE(
      ASN1_STRING_set(orig.get(), long_value.data(), long_value.size()));

  // Serialize to hex via i2a_ASN1_INTEGER
  bssl::UniquePtr<BIO> out_bio(BIO_new(BIO_s_mem()));
  ASSERT_TRUE(out_bio);
  int written = i2a_ASN1_INTEGER(out_bio.get(), orig.get());
  ASSERT_GT(written, 0);

  // Get the hex string (should contain "\\\n" continuation)
  const uint8_t *hex_data = nullptr;
  size_t hex_len = 0;
  ASSERT_TRUE(BIO_mem_contents(out_bio.get(), &hex_data, &hex_len));

  // Verify that continuation is present
  std::string hex_str(reinterpret_cast<const char *>(hex_data), hex_len);
  EXPECT_NE(hex_str.find("\\\n"), std::string::npos)
      << "Expected line continuation in output: " << hex_str;

  // Parse it back via a2i_ASN1_INTEGER
  bssl::UniquePtr<BIO> in_bio(
      BIO_new_mem_buf(hex_data, static_cast<int>(hex_len)));
  ASSERT_TRUE(in_bio);

  bssl::UniquePtr<ASN1_INTEGER> parsed(ASN1_INTEGER_new());
  ASSERT_TRUE(parsed);

  char buf[256];
  int ret = a2i_ASN1_INTEGER(in_bio.get(), parsed.get(), buf, sizeof(buf));
  ASSERT_EQ(ret, 1);

  // The parsed value should match the original
  EXPECT_EQ(Bytes(orig->data, orig->length),
            Bytes(parsed->data, parsed->length));
}

#endif  // !WINDOWS || !SHARED_LIBRARY
