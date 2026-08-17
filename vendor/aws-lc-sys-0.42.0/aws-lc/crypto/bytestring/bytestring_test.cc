// Copyright (c) 2014, Google Inc.
// SPDX-License-Identifier: ISC

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include <vector>

#include <gtest/gtest.h>
#include <openssl/asn1.h>

#include <openssl/bytestring.h>
#include <openssl/crypto.h>
#include <openssl/span.h>

#include "../internal.h"
#include "../test/test_util.h"
#include "internal.h"


TEST(CBSTest, Skip) {
  static const uint8_t kData[] = {1, 2, 3};
  CBS data;

  CBS_init(&data, kData, sizeof(kData));
  EXPECT_EQ(3u, CBS_len(&data));
  EXPECT_TRUE(CBS_skip(&data, 1));
  EXPECT_EQ(2u, CBS_len(&data));
  EXPECT_TRUE(CBS_skip(&data, 2));
  EXPECT_EQ(0u, CBS_len(&data));
  EXPECT_FALSE(CBS_skip(&data, 1));
}

TEST(CBSTest, GetUint) {
  static const uint8_t kData[] = {1,  2,  3,  4,  5,  6,  7,  8,  9,  10,
                                  11, 12, 13, 14, 15, 16, 17, 18, 19, 20};
  uint8_t u8;
  uint16_t u16;
  uint32_t u32;
  uint64_t u64;
  CBS data;

  CBS_init(&data, kData, sizeof(kData));
  ASSERT_TRUE(CBS_get_u8(&data, &u8));
  EXPECT_EQ(1u, u8);
  ASSERT_TRUE(CBS_get_u16(&data, &u16));
  EXPECT_EQ(0x203u, u16);
  ASSERT_TRUE(CBS_get_u24(&data, &u32));
  EXPECT_EQ(0x40506u, u32);
  ASSERT_TRUE(CBS_get_u32(&data, &u32));
  EXPECT_EQ(0x708090au, u32);
  ASSERT_TRUE(CBS_get_u64(&data, &u64));
  EXPECT_EQ(0xb0c0d0e0f101112u, u64);
  ASSERT_TRUE(CBS_get_last_u8(&data, &u8));
  EXPECT_EQ(0x14u, u8);
  ASSERT_TRUE(CBS_get_last_u8(&data, &u8));
  EXPECT_EQ(0x13u, u8);
  EXPECT_FALSE(CBS_get_u8(&data, &u8));
  EXPECT_FALSE(CBS_get_last_u8(&data, &u8));

  CBS_init(&data, kData, sizeof(kData));
  ASSERT_TRUE(CBS_get_u16le(&data, &u16));
  EXPECT_EQ(0x0201u, u16);
  ASSERT_TRUE(CBS_get_u32le(&data, &u32));
  EXPECT_EQ(0x06050403u, u32);
  ASSERT_TRUE(CBS_get_u64le(&data, &u64));
  EXPECT_EQ(0x0e0d0c0b0a090807u, u64);
}

TEST(CBSTest, GetPrefixed) {
  static const uint8_t kData[] = {1, 2, 0, 2, 3, 4, 0, 0, 3, 3, 2, 1};
  uint8_t u8;
  uint16_t u16;
  uint32_t u32;
  CBS data, prefixed;

  CBS_init(&data, kData, sizeof(kData));
  ASSERT_TRUE(CBS_get_u8_length_prefixed(&data, &prefixed));
  EXPECT_EQ(1u, CBS_len(&prefixed));
  ASSERT_TRUE(CBS_get_u8(&prefixed, &u8));
  EXPECT_EQ(2u, u8);
  ASSERT_TRUE(CBS_get_u16_length_prefixed(&data, &prefixed));
  EXPECT_EQ(2u, CBS_len(&prefixed));
  ASSERT_TRUE(CBS_get_u16(&prefixed, &u16));
  EXPECT_EQ(0x304u, u16);
  ASSERT_TRUE(CBS_get_u24_length_prefixed(&data, &prefixed));
  EXPECT_EQ(3u, CBS_len(&prefixed));
  ASSERT_TRUE(CBS_get_u24(&prefixed, &u32));
  EXPECT_EQ(0x30201u, u32);
}

TEST(CBSTest, GetPrefixedBad) {
  static const uint8_t kData1[] = {2, 1};
  static const uint8_t kData2[] = {0, 2, 1};
  static const uint8_t kData3[] = {0, 0, 2, 1};
  CBS data, prefixed;

  CBS_init(&data, kData1, sizeof(kData1));
  EXPECT_FALSE(CBS_get_u8_length_prefixed(&data, &prefixed));

  CBS_init(&data, kData2, sizeof(kData2));
  EXPECT_FALSE(CBS_get_u16_length_prefixed(&data, &prefixed));

  CBS_init(&data, kData3, sizeof(kData3));
  EXPECT_FALSE(CBS_get_u24_length_prefixed(&data, &prefixed));
}

TEST(CBSTest, GetUntilFirst) {
  static const uint8_t kData[] = {0, 1, 2, 3, 0, 1, 2, 3};
  CBS data;
  CBS_init(&data, kData, sizeof(kData));

  CBS prefix;
  EXPECT_FALSE(CBS_get_until_first(&data, &prefix, 4));
  EXPECT_EQ(CBS_data(&data), kData);
  EXPECT_EQ(CBS_len(&data), sizeof(kData));

  ASSERT_TRUE(CBS_get_until_first(&data, &prefix, 0));
  EXPECT_EQ(CBS_len(&prefix), 0u);
  EXPECT_EQ(CBS_data(&data), kData);
  EXPECT_EQ(CBS_len(&data), sizeof(kData));

  ASSERT_TRUE(CBS_get_until_first(&data, &prefix, 2));
  EXPECT_EQ(CBS_data(&prefix), kData);
  EXPECT_EQ(CBS_len(&prefix), 2u);
  EXPECT_EQ(CBS_data(&data), kData + 2);
  EXPECT_EQ(CBS_len(&data), sizeof(kData) - 2);
}

TEST(CBSTest, GetASN1) {
  static const uint8_t kData1[] = {0x30, 2, 1, 2};
  static const uint8_t kData2[] = {0x30, 3, 1, 2};
  static const uint8_t kData3[] = {0x30, 0x80};
  static const uint8_t kData4[] = {0x30, 0x81, 1, 1};
  static const uint8_t kData5[4 + 0x80] = {0x30, 0x82, 0, 0x80};
  static const uint8_t kData6[] = {0xa1, 3, 0x4, 1, 1};
  static const uint8_t kData7[] = {0xa1, 3, 0x4, 2, 1};
  static const uint8_t kData8[] = {0xa1, 3, 0x2, 1, 1};
  static const uint8_t kData9[] = {0xa1, 3, 0x2, 1, 0xff};

  CBS data, contents;
  int present;
  uint64_t value;

  CBS_init(&data, kData1, sizeof(kData1));
  EXPECT_FALSE(CBS_peek_asn1_tag(&data, CBS_ASN1_BOOLEAN));
  EXPECT_TRUE(CBS_peek_asn1_tag(&data, CBS_ASN1_SEQUENCE));

  ASSERT_TRUE(CBS_get_asn1(&data, &contents, CBS_ASN1_SEQUENCE));
  EXPECT_EQ(Bytes("\x01\x02"), Bytes(CBS_data(&contents), CBS_len(&contents)));

  CBS_init(&data, kData2, sizeof(kData2));
  // data is truncated
  EXPECT_FALSE(CBS_get_asn1(&data, &contents, CBS_ASN1_SEQUENCE));

  CBS_init(&data, kData3, sizeof(kData3));
  // zero byte length of length
  EXPECT_FALSE(CBS_get_asn1(&data, &contents, CBS_ASN1_SEQUENCE));

  CBS_init(&data, kData4, sizeof(kData4));
  // long form mistakenly used.
  EXPECT_FALSE(CBS_get_asn1(&data, &contents, CBS_ASN1_SEQUENCE));

  CBS_init(&data, kData5, sizeof(kData5));
  // length takes too many bytes.
  EXPECT_FALSE(CBS_get_asn1(&data, &contents, CBS_ASN1_SEQUENCE));

  CBS_init(&data, kData1, sizeof(kData1));
  // wrong tag.
  EXPECT_FALSE(CBS_get_asn1(&data, &contents, 0x31));

  CBS_init(&data, NULL, 0);
  // peek at empty data.
  EXPECT_FALSE(CBS_peek_asn1_tag(&data, CBS_ASN1_SEQUENCE));

  CBS_init(&data, NULL, 0);
  // optional elements at empty data.
  ASSERT_TRUE(CBS_get_optional_asn1(
      &data, &contents, &present,
      CBS_ASN1_CONTEXT_SPECIFIC | CBS_ASN1_CONSTRUCTED | 0));
  EXPECT_FALSE(present);
  ASSERT_TRUE(CBS_get_optional_asn1_octet_string(
      &data, &contents, &present,
      CBS_ASN1_CONTEXT_SPECIFIC | CBS_ASN1_CONSTRUCTED | 0));
  EXPECT_FALSE(present);
  EXPECT_EQ(0u, CBS_len(&contents));
  ASSERT_TRUE(CBS_get_optional_asn1_octet_string(
      &data, &contents, NULL,
      CBS_ASN1_CONTEXT_SPECIFIC | CBS_ASN1_CONSTRUCTED | 0));
  EXPECT_EQ(0u, CBS_len(&contents));
  ASSERT_TRUE(CBS_get_optional_asn1_uint64(
      &data, &value, CBS_ASN1_CONTEXT_SPECIFIC | CBS_ASN1_CONSTRUCTED | 0, 42));
  EXPECT_EQ(42u, value);

  CBS_init(&data, kData6, sizeof(kData6));
  // optional element.
  ASSERT_TRUE(CBS_get_optional_asn1(
      &data, &contents, &present,
      CBS_ASN1_CONTEXT_SPECIFIC | CBS_ASN1_CONSTRUCTED | 0));
  EXPECT_FALSE(present);
  ASSERT_TRUE(CBS_get_optional_asn1(
      &data, &contents, &present,
      CBS_ASN1_CONTEXT_SPECIFIC | CBS_ASN1_CONSTRUCTED | 1));
  EXPECT_TRUE(present);
  EXPECT_EQ(Bytes("\x04\x01\x01"),
            Bytes(CBS_data(&contents), CBS_len(&contents)));

  CBS_init(&data, kData6, sizeof(kData6));
  // optional octet string.
  ASSERT_TRUE(CBS_get_optional_asn1_octet_string(
      &data, &contents, &present,
      CBS_ASN1_CONTEXT_SPECIFIC | CBS_ASN1_CONSTRUCTED | 0));
  EXPECT_FALSE(present);
  EXPECT_EQ(0u, CBS_len(&contents));
  ASSERT_TRUE(CBS_get_optional_asn1_octet_string(
      &data, &contents, &present,
      CBS_ASN1_CONTEXT_SPECIFIC | CBS_ASN1_CONSTRUCTED | 1));
  EXPECT_TRUE(present);
  EXPECT_EQ(Bytes("\x01"), Bytes(CBS_data(&contents), CBS_len(&contents)));

  CBS_init(&data, kData7, sizeof(kData7));
  // invalid optional octet string.
  EXPECT_FALSE(CBS_get_optional_asn1_octet_string(
      &data, &contents, &present,
      CBS_ASN1_CONTEXT_SPECIFIC | CBS_ASN1_CONSTRUCTED | 1));

  CBS_init(&data, kData8, sizeof(kData8));
  // optional integer.
  ASSERT_TRUE(CBS_get_optional_asn1_uint64(
      &data, &value, CBS_ASN1_CONTEXT_SPECIFIC | CBS_ASN1_CONSTRUCTED | 0, 42));
  EXPECT_EQ(42u, value);
  ASSERT_TRUE(CBS_get_optional_asn1_uint64(
      &data, &value, CBS_ASN1_CONTEXT_SPECIFIC | CBS_ASN1_CONSTRUCTED | 1, 42));
  EXPECT_EQ(1u, value);

  CBS_init(&data, kData9, sizeof(kData9));
  // invalid optional integer.
  EXPECT_FALSE(CBS_get_optional_asn1_uint64(
      &data, &value, CBS_ASN1_CONTEXT_SPECIFIC | CBS_ASN1_CONSTRUCTED | 1, 42));

  CBS_ASN1_TAG tag;
  CBS_init(&data, kData1, sizeof(kData1));
  ASSERT_TRUE(CBS_get_any_asn1(&data, &contents, &tag));
  EXPECT_EQ(CBS_ASN1_SEQUENCE, tag);
  EXPECT_EQ(Bytes("\x01\x02"), Bytes(CBS_data(&contents), CBS_len(&contents)));

  size_t header_len;
  CBS_init(&data, kData1, sizeof(kData1));
  ASSERT_TRUE(CBS_get_any_asn1_element(&data, &contents, &tag, &header_len));
  EXPECT_EQ(CBS_ASN1_SEQUENCE, tag);
  EXPECT_EQ(2u, header_len);
  EXPECT_EQ(Bytes("\x30\x02\x01\x02"),
            Bytes(CBS_data(&contents), CBS_len(&contents)));
}

TEST(CBSTest, ParseASN1Tag) {
  const struct {
    bool ok;
    CBS_ASN1_TAG tag;
    std::vector<uint8_t> in;
  } kTests[] = {
      {true, CBS_ASN1_SEQUENCE, {0x30, 0}},
      {true, CBS_ASN1_CONTEXT_SPECIFIC | CBS_ASN1_CONSTRUCTED | 4, {0xa4, 0}},
      {true, CBS_ASN1_APPLICATION | 30, {0x5e, 0}},
      {true, CBS_ASN1_APPLICATION | 31, {0x5f, 0x1f, 0}},
      {true, CBS_ASN1_APPLICATION | 32, {0x5f, 0x20, 0}},
      {true,
       CBS_ASN1_PRIVATE | CBS_ASN1_CONSTRUCTED | 0x1fffffff,
       {0xff, 0x81, 0xff, 0xff, 0xff, 0x7f, 0}},
      // Tag number fits in |uint32_t| but not |CBS_ASN1_TAG_NUMBER_MASK|.
      {false, 0, {0xff, 0x82, 0xff, 0xff, 0xff, 0x7f, 0}},
      // Tag number does not fit in |uint32_t|.
      {false, 0, {0xff, 0x90, 0x80, 0x80, 0x80, 0, 0}},
      // Tag number is not minimally-encoded
      {false, 0, {0x5f, 0x80, 0x1f, 0}},
      // Tag number should have used short form.
      {false, 0, {0x5f, 0x80, 0x1e, 0}},
  };
  for (const auto &t : kTests) {
    SCOPED_TRACE(Bytes(t.in));
    CBS_ASN1_TAG tag;
    CBS cbs, child;
    CBS_init(&cbs, t.in.data(), t.in.size());
    ASSERT_EQ(t.ok, !!CBS_get_any_asn1(&cbs, &child, &tag));
    if (t.ok) {
      EXPECT_EQ(t.tag, tag);
      EXPECT_EQ(0u, CBS_len(&child));
      EXPECT_EQ(0u, CBS_len(&cbs));

      CBS_init(&cbs, t.in.data(), t.in.size());
      EXPECT_TRUE(CBS_peek_asn1_tag(&cbs, t.tag));
      EXPECT_FALSE(CBS_peek_asn1_tag(&cbs, t.tag + 1));

      EXPECT_TRUE(CBS_get_asn1(&cbs, &child, t.tag));
      EXPECT_EQ(0u, CBS_len(&child));
      EXPECT_EQ(0u, CBS_len(&cbs));

      CBS_init(&cbs, t.in.data(), t.in.size());
      EXPECT_FALSE(CBS_get_asn1(&cbs, &child, t.tag + 1));
    }
  }
}

TEST(CBSTest, GetOptionalASN1Bool) {
  static const uint8_t kTrue[] = {0x0a, 3, CBS_ASN1_BOOLEAN, 1, 0xff};
  static const uint8_t kFalse[] = {0x0a, 3, CBS_ASN1_BOOLEAN, 1, 0x00};
  static const uint8_t kInvalid[] = {0x0a, 3, CBS_ASN1_BOOLEAN, 1, 0x01};

  CBS data;
  CBS_init(&data, NULL, 0);
  int val = 2;
  ASSERT_TRUE(CBS_get_optional_asn1_bool(&data, &val, 0x0a, 0));
  EXPECT_EQ(0, val);

  CBS_init(&data, kTrue, sizeof(kTrue));
  val = 2;
  ASSERT_TRUE(CBS_get_optional_asn1_bool(&data, &val, 0x0a, 0));
  EXPECT_EQ(1, val);

  CBS_init(&data, kFalse, sizeof(kFalse));
  val = 2;
  ASSERT_TRUE(CBS_get_optional_asn1_bool(&data, &val, 0x0a, 1));
  EXPECT_EQ(0, val);

  CBS_init(&data, kInvalid, sizeof(kInvalid));
  EXPECT_FALSE(CBS_get_optional_asn1_bool(&data, &val, 0x0a, 1));
}

// Test that CBB_init may be used on an uninitialized input.
TEST(CBBTest, InitUninitialized) {
  CBB cbb;
  ASSERT_TRUE(CBB_init(&cbb, 100));
  CBB_cleanup(&cbb);
}

TEST(CBBTest, Basic) {
  static const uint8_t kExpected[] = {
      0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a,
      0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14,
      0x03, 0x02, 0x0a, 0x09, 0x08, 0x07, 0x12, 0x11, 0x10, 0x0f,
      0x0e, 0x0d, 0x0c, 0x0b, 0x00, 0x00, 0x00, 0x00};
  uint8_t *buf;
  size_t buf_len;

  bssl::ScopedCBB cbb;
  ASSERT_TRUE(CBB_init(cbb.get(), 100));
  cbb.Reset();

  ASSERT_TRUE(CBB_init(cbb.get(), 0));
  ASSERT_TRUE(CBB_add_zeros(cbb.get(), 0));
  ASSERT_TRUE(CBB_add_u8(cbb.get(), 1));
  ASSERT_TRUE(CBB_add_u16(cbb.get(), 0x203));
  ASSERT_TRUE(CBB_add_u24(cbb.get(), 0x40506));
  ASSERT_TRUE(CBB_add_u32(cbb.get(), 0x708090a));
  ASSERT_TRUE(CBB_add_u64(cbb.get(), 0xb0c0d0e0f101112));
  ASSERT_TRUE(CBB_add_bytes(cbb.get(), (const uint8_t *)"\x13\x14", 2));
  ASSERT_TRUE(CBB_add_u16le(cbb.get(), 0x203));
  ASSERT_TRUE(CBB_add_u32le(cbb.get(), 0x708090a));
  ASSERT_TRUE(CBB_add_u64le(cbb.get(), 0xb0c0d0e0f101112));
  ASSERT_TRUE(CBB_add_zeros(cbb.get(), 4));
  ASSERT_TRUE(CBB_finish(cbb.get(), &buf, &buf_len));

  bssl::UniquePtr<uint8_t> scoper(buf);
  EXPECT_EQ(Bytes(kExpected), Bytes(buf, buf_len));
}

TEST(CBBTest, Fixed) {
  CBB cbb;
  uint8_t buf[1];
  uint8_t *out_buf;
  size_t out_size;

  ASSERT_TRUE(CBB_init_fixed(&cbb, NULL, 0));
  ASSERT_TRUE(CBB_finish(&cbb, &out_buf, &out_size));
  EXPECT_EQ(NULL, out_buf);
  EXPECT_EQ(0u, out_size);

  ASSERT_TRUE(CBB_init_fixed(&cbb, buf, 1));
  ASSERT_TRUE(CBB_add_u8(&cbb, 1));
  ASSERT_TRUE(CBB_finish(&cbb, &out_buf, &out_size));
  EXPECT_EQ(buf, out_buf);
  EXPECT_EQ(1u, out_size);
  EXPECT_EQ(1u, buf[0]);

  ASSERT_TRUE(CBB_init_fixed(&cbb, buf, 1));
  ASSERT_TRUE(CBB_add_u8(&cbb, 1));
  EXPECT_FALSE(CBB_add_u8(&cbb, 2));
  // We do not need |CBB_cleanup| or |bssl::ScopedCBB| here because a fixed
  // |CBB| has no allocations. Leak-checking tools will confirm there was
  // nothing to clean up.

  // However, it should be harmless to call |CBB_cleanup|.
  CBB cbb2;
  ASSERT_TRUE(CBB_init_fixed(&cbb2, buf, 1));
  ASSERT_TRUE(CBB_add_u8(&cbb2, 1));
  EXPECT_FALSE(CBB_add_u8(&cbb2, 2));
  CBB_cleanup(&cbb2);
}

// Test that calling CBB_finish on a child does nothing.
TEST(CBBTest, FinishChild) {
  CBB child;
  uint8_t *out_buf;
  size_t out_size;

  bssl::ScopedCBB cbb;
  ASSERT_TRUE(CBB_init(cbb.get(), 16));
  ASSERT_TRUE(CBB_add_u8_length_prefixed(cbb.get(), &child));

  EXPECT_FALSE(CBB_finish(&child, &out_buf, &out_size));

  ASSERT_TRUE(CBB_finish(cbb.get(), &out_buf, &out_size));
  bssl::UniquePtr<uint8_t> scoper(out_buf);
  ASSERT_EQ(1u, out_size);
  EXPECT_EQ(0u, out_buf[0]);
}

TEST(CBBTest, Prefixed) {
  static const uint8_t kExpected[] = {0, 1, 1, 0, 2, 2, 3, 0, 0, 3,
                                      4, 5, 6, 5, 4, 1, 0, 1, 2};
  uint8_t *buf;
  size_t buf_len;
  bssl::ScopedCBB cbb;
  CBB contents, inner_contents, inner_inner_contents;
  ASSERT_TRUE(CBB_init(cbb.get(), 0));
  EXPECT_EQ(0u, CBB_len(cbb.get()));
  ASSERT_TRUE(CBB_add_u8_length_prefixed(cbb.get(), &contents));
  ASSERT_TRUE(CBB_add_u8_length_prefixed(cbb.get(), &contents));
  ASSERT_TRUE(CBB_add_u8(&contents, 1));
  EXPECT_EQ(1u, CBB_len(&contents));
  ASSERT_TRUE(CBB_flush(cbb.get()));
  EXPECT_EQ(3u, CBB_len(cbb.get()));
  ASSERT_TRUE(CBB_add_u16_length_prefixed(cbb.get(), &contents));
  ASSERT_TRUE(CBB_add_u16(&contents, 0x203));
  ASSERT_TRUE(CBB_add_u24_length_prefixed(cbb.get(), &contents));
  ASSERT_TRUE(CBB_add_u24(&contents, 0x40506));
  ASSERT_TRUE(CBB_add_u8_length_prefixed(cbb.get(), &contents));
  ASSERT_TRUE(CBB_add_u8_length_prefixed(&contents, &inner_contents));
  ASSERT_TRUE(CBB_add_u8(&inner_contents, 1));
  ASSERT_TRUE(
      CBB_add_u16_length_prefixed(&inner_contents, &inner_inner_contents));
  ASSERT_TRUE(CBB_add_u8(&inner_inner_contents, 2));
  ASSERT_TRUE(CBB_finish(cbb.get(), &buf, &buf_len));

  bssl::UniquePtr<uint8_t> scoper(buf);
  EXPECT_EQ(Bytes(kExpected), Bytes(buf, buf_len));
}

TEST(CBBTest, DiscardChild) {
  bssl::ScopedCBB cbb;
  CBB contents, inner_contents, inner_inner_contents;

  ASSERT_TRUE(CBB_init(cbb.get(), 0));
  ASSERT_TRUE(CBB_add_u8(cbb.get(), 0xaa));

  // Discarding |cbb|'s children preserves the byte written.
  CBB_discard_child(cbb.get());

  ASSERT_TRUE(CBB_add_u8_length_prefixed(cbb.get(), &contents));
  ASSERT_TRUE(CBB_add_u8_length_prefixed(cbb.get(), &contents));
  ASSERT_TRUE(CBB_add_u8(&contents, 0xbb));
  ASSERT_TRUE(CBB_add_u16_length_prefixed(cbb.get(), &contents));
  ASSERT_TRUE(CBB_add_u16(&contents, 0xcccc));
  ASSERT_TRUE(CBB_add_u24_length_prefixed(cbb.get(), &contents));
  ASSERT_TRUE(CBB_add_u24(&contents, 0xdddddd));
  ASSERT_TRUE(CBB_add_u8_length_prefixed(cbb.get(), &contents));
  ASSERT_TRUE(CBB_add_u8(&contents, 0xff));
  ASSERT_TRUE(CBB_add_u8_length_prefixed(&contents, &inner_contents));
  ASSERT_TRUE(CBB_add_u8(&inner_contents, 0x42));
  ASSERT_TRUE(
      CBB_add_u16_length_prefixed(&inner_contents, &inner_inner_contents));
  ASSERT_TRUE(CBB_add_u8(&inner_inner_contents, 0x99));

  // Discard everything from |inner_contents| down.
  CBB_discard_child(&contents);

  uint8_t *buf;
  size_t buf_len;
  ASSERT_TRUE(CBB_finish(cbb.get(), &buf, &buf_len));
  bssl::UniquePtr<uint8_t> scoper(buf);

  static const uint8_t kExpected[] = {
        0xaa,
        0,
        1, 0xbb,
        0, 2, 0xcc, 0xcc,
        0, 0, 3, 0xdd, 0xdd, 0xdd,
        1, 0xff,
  };
  EXPECT_EQ(Bytes(kExpected), Bytes(buf, buf_len));
}

TEST(CBBTest, Misuse) {
  bssl::ScopedCBB cbb;
  CBB child, contents;
  uint8_t *buf;
  size_t buf_len;

  ASSERT_TRUE(CBB_init(cbb.get(), 0));
  ASSERT_TRUE(CBB_add_u8_length_prefixed(cbb.get(), &child));
  ASSERT_TRUE(CBB_add_u8(&child, 1));
  ASSERT_TRUE(CBB_add_u8(cbb.get(), 2));

  // Since we wrote to |cbb|, |child| is now invalid and attempts to write to
  // it should fail.
  EXPECT_FALSE(CBB_add_u8(&child, 1));
  EXPECT_FALSE(CBB_add_u16(&child, 1));
  EXPECT_FALSE(CBB_add_u24(&child, 1));
  EXPECT_FALSE(CBB_add_u8_length_prefixed(&child, &contents));
  EXPECT_FALSE(CBB_add_u16_length_prefixed(&child, &contents));
  EXPECT_FALSE(CBB_add_asn1(&child, &contents, 1));
  EXPECT_FALSE(CBB_add_bytes(&child, (const uint8_t*) "a", 1));

  ASSERT_TRUE(CBB_finish(cbb.get(), &buf, &buf_len));
  bssl::UniquePtr<uint8_t> scoper(buf);

  EXPECT_EQ(Bytes("\x01\x01\x02"), Bytes(buf, buf_len));
}

TEST(CBBTest, ASN1) {
  static const uint8_t kExpected[] = {
      // SEQUENCE { 1 2 3 }
      0x30, 3, 1, 2, 3,
      // [4 CONSTRUCTED] { 4 5 6 }
      0xa4, 3, 4, 5, 6,
      // [APPLICATION 30 PRIMITIVE] { 7 8 9 }
      0x5e, 3, 7, 8, 9,
      // [APPLICATION 31 PRIMITIVE] { 10 11 12 }
      0x5f, 0x1f, 3, 10, 11, 12,
      // [PRIVATE 2^29-1 CONSTRUCTED] { 13 14 15 }
      0xff, 0x81, 0xff, 0xff, 0xff, 0x7f, 3, 13, 14, 15,
  };
  uint8_t *buf;
  size_t buf_len;
  bssl::ScopedCBB cbb;
  CBB contents, inner_contents;

  ASSERT_TRUE(CBB_init(cbb.get(), 0));
  ASSERT_TRUE(CBB_add_asn1(cbb.get(), &contents, CBS_ASN1_SEQUENCE));
  ASSERT_TRUE(CBB_add_bytes(&contents, (const uint8_t *)"\x01\x02\x03", 3));
  ASSERT_TRUE(
      CBB_add_asn1(cbb.get(), &contents,
                   CBS_ASN1_CONTEXT_SPECIFIC | CBS_ASN1_CONSTRUCTED | 4));
  ASSERT_TRUE(CBB_add_bytes(&contents, (const uint8_t *)"\x04\x05\x06", 3));
  ASSERT_TRUE(
      CBB_add_asn1(cbb.get(), &contents,
                   CBS_ASN1_APPLICATION | 30));
  ASSERT_TRUE(CBB_add_bytes(&contents, (const uint8_t *)"\x07\x08\x09", 3));
  ASSERT_TRUE(
      CBB_add_asn1(cbb.get(), &contents,
                   CBS_ASN1_APPLICATION | 31));
  ASSERT_TRUE(CBB_add_bytes(&contents, (const uint8_t *)"\x0a\x0b\x0c", 3));
  ASSERT_TRUE(
      CBB_add_asn1(cbb.get(), &contents,
                   CBS_ASN1_PRIVATE | CBS_ASN1_CONSTRUCTED | 0x1fffffff));
  ASSERT_TRUE(CBB_add_bytes(&contents, (const uint8_t *)"\x0d\x0e\x0f", 3));
  ASSERT_TRUE(CBB_finish(cbb.get(), &buf, &buf_len));
  bssl::UniquePtr<uint8_t> scoper(buf);

  EXPECT_EQ(Bytes(kExpected), Bytes(buf, buf_len));

  std::vector<uint8_t> test_data(100000, 0x42);
  ASSERT_TRUE(CBB_init(cbb.get(), 0));
  ASSERT_TRUE(CBB_add_asn1(cbb.get(), &contents, CBS_ASN1_SEQUENCE));
  ASSERT_TRUE(CBB_add_bytes(&contents, test_data.data(), 130));
  ASSERT_TRUE(CBB_finish(cbb.get(), &buf, &buf_len));
  scoper.reset(buf);

  ASSERT_EQ(3u + 130u, buf_len);
  EXPECT_EQ(Bytes("\x30\x81\x82"), Bytes(buf, 3));
  EXPECT_EQ(Bytes(test_data.data(), 130), Bytes(buf + 3, 130));

  ASSERT_TRUE(CBB_init(cbb.get(), 0));
  ASSERT_TRUE(CBB_add_asn1(cbb.get(), &contents, CBS_ASN1_SEQUENCE));
  ASSERT_TRUE(CBB_add_bytes(&contents, test_data.data(), 1000));
  ASSERT_TRUE(CBB_finish(cbb.get(), &buf, &buf_len));
  scoper.reset(buf);

  ASSERT_EQ(4u + 1000u, buf_len);
  EXPECT_EQ(Bytes("\x30\x82\x03\xe8"), Bytes(buf, 4));
  EXPECT_EQ(Bytes(test_data.data(), 1000), Bytes(buf + 4, 1000));

  ASSERT_TRUE(CBB_init(cbb.get(), 0));
  ASSERT_TRUE(CBB_add_asn1(cbb.get(), &contents, CBS_ASN1_SEQUENCE));
  ASSERT_TRUE(CBB_add_asn1(&contents, &inner_contents, CBS_ASN1_SEQUENCE));
  ASSERT_TRUE(CBB_add_bytes(&inner_contents, test_data.data(), 100000));
  ASSERT_TRUE(CBB_finish(cbb.get(), &buf, &buf_len));
  scoper.reset(buf);

  ASSERT_EQ(5u + 5u + 100000u, buf_len);
  EXPECT_EQ(Bytes("\x30\x83\x01\x86\xa5\x30\x83\x01\x86\xa0"), Bytes(buf, 10));
  EXPECT_EQ(Bytes(test_data.data(), test_data.size()), Bytes(buf + 10, 100000));
}

static void ExpectBerConvert(const char *name,
                             bssl::Span<const uint8_t> der_expected,
                             bssl::Span<const uint8_t> ber) {
  SCOPED_TRACE(name);
  CBS in, out;
  uint8_t *storage;

  CBS_init(&in, ber.data(), ber.size());
  ASSERT_TRUE(CBS_asn1_ber_to_der(&in, &out, &storage));
  bssl::UniquePtr<uint8_t> scoper(storage);

  EXPECT_EQ(Bytes(der_expected), Bytes(CBS_data(&out), CBS_len(&out)));
  if (storage != nullptr) {
    EXPECT_NE(Bytes(der_expected), Bytes(ber));
  } else {
    EXPECT_EQ(Bytes(der_expected), Bytes(ber));
  }
}

// kIndefBER contains a SEQUENCE with an indefinite length.
static const uint8_t kIndefBER[] = {0x30, 0x80, 0x01, 0x01, 0x02, 0x00, 0x00};
static const uint8_t kIndefDER[] = {0x30, 0x03, 0x01, 0x01, 0x02};

// kIndefBER2 contains a constructed [APPLICATION 31] with an indefinite
// length.
static const uint8_t kIndefBER2[] = {0x7f, 0x1f, 0x80, 0x01,
                                     0x01, 0x02, 0x00, 0x00};
static const uint8_t kIndefDER2[] = {0x7f, 0x1f, 0x03, 0x01, 0x01, 0x02};

// kOctetStringBER contains an indefinite length OCTET STRING with two parts.
// These parts need to be concatenated in DER form.
static const uint8_t kOctetStringBER[] = {0x24, 0x80, 0x04, 0x02, 0,    1,
                                          0x04, 0x02, 2,    3,    0x00, 0x00};
static const uint8_t kOctetStringDER[] = {0x04, 0x04, 0, 1, 2, 3};


// kWrappedIndefBER contains indefinite-length SEQUENCE, wrapped
// and followed by valid DER. This tests that we correctly identify BER nested
// inside DER.
//
//  SEQUENCE {
//    SEQUENCE {
//      SEQUENCE indefinite {}
//    }
//    SEQUENCE {}
//  }
static const uint8_t kWrappedIndefBER[] = {0x30, 0x08, 0x30, 0x04, 0x30,
                                           0x80, 0x00, 0x00, 0x30, 0x00};
static const uint8_t kWrappedIndefDER[] = {0x30, 0x06, 0x30, 0x02,
                                           0x30, 0x00, 0x30, 0x00};

// kConstructedStringBER contains a deeply-nested constructed OCTET STRING.
// The BER conversion collapses this to one level deep, but not completely.
static const uint8_t kConstructedStringBER[] = {
  0xa0, 0x10, 0x24, 0x06, 0x04, 0x01, 0x00, 0x04, 0x01,
  0x01, 0x24, 0x06, 0x04, 0x01, 0x02, 0x04, 0x01, 0x03,
};
static const uint8_t kConstructedStringDER[] = {
  0xa0, 0x08, 0x04, 0x02, 0x00, 0x01, 0x04, 0x02, 0x02, 0x03,
};

// kWrappedConstructedStringBER contains a constructed OCTET STRING, wrapped
// and followed by valid DER. This tests that we correctly identify BER nested
// inside DER.
//
//  SEQUENCE {
//    SEQUENCE {
//      [OCTET_STRING CONSTRUCTED] {
//        OCTET_STRING {}
//      }
//    }
//    SEQUENCE {}
//  }
static const uint8_t kWrappedConstructedStringBER[] = {
  0x30, 0x08, 0x30, 0x04, 0x24, 0x02, 0x04, 0x00, 0x30, 0x00};
static const uint8_t kWrappedConstructedStringDER[] = {
  0x30, 0x06, 0x30, 0x02, 0x04, 0x00, 0x30, 0x00};

// kConstructedBitString contains a BER constructed BIT STRING. These are not
// supported and thus are left unchanged.
static const uint8_t kConstructedBitStringBER[] = {
  0x23, 0x0a, 0x03, 0x03, 0x00, 0x12, 0x34, 0x03, 0x03, 0x00, 0x56, 0x78};

TEST(CBSTest, BerConvert) {
  static const uint8_t kSimpleBER[] = {0x01, 0x01, 0x00};

  // kNonMinimalLengthBER has a non-minimally encoded length.
  static const uint8_t kNonMinimalLengthBER[] = {0x02, 0x82, 0x00, 0x01, 0x01};
  static const uint8_t kNonMinimalLengthDER[] = {0x02, 0x01, 0x01};

  // kNSSBER is part of a PKCS#12 message generated by NSS that uses indefinite
  // length elements extensively.
  static const uint8_t kNSSBER[] = {
      0x30, 0x80, 0x02, 0x01, 0x03, 0x30, 0x80, 0x06, 0x09, 0x2a, 0x86, 0x48,
      0x86, 0xf7, 0x0d, 0x01, 0x07, 0x01, 0xa0, 0x80, 0x24, 0x80, 0x04, 0x04,
      0x01, 0x02, 0x03, 0x04, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x30, 0x39,
      0x30, 0x21, 0x30, 0x09, 0x06, 0x05, 0x2b, 0x0e, 0x03, 0x02, 0x1a, 0x05,
      0x00, 0x04, 0x14, 0x84, 0x98, 0xfc, 0x66, 0x33, 0xee, 0xba, 0xe7, 0x90,
      0xc1, 0xb6, 0xe8, 0x8f, 0xfe, 0x1d, 0xc5, 0xa5, 0x97, 0x93, 0x3e, 0x04,
      0x10, 0x38, 0x62, 0xc6, 0x44, 0x12, 0xd5, 0x30, 0x00, 0xf8, 0xf2, 0x1b,
      0xf0, 0x6e, 0x10, 0x9b, 0xb8, 0x02, 0x02, 0x07, 0xd0, 0x00, 0x00,
  };

  static const uint8_t kNSSDER[] = {
      0x30, 0x53, 0x02, 0x01, 0x03, 0x30, 0x13, 0x06, 0x09, 0x2a, 0x86,
      0x48, 0x86, 0xf7, 0x0d, 0x01, 0x07, 0x01, 0xa0, 0x06, 0x04, 0x04,
      0x01, 0x02, 0x03, 0x04, 0x30, 0x39, 0x30, 0x21, 0x30, 0x09, 0x06,
      0x05, 0x2b, 0x0e, 0x03, 0x02, 0x1a, 0x05, 0x00, 0x04, 0x14, 0x84,
      0x98, 0xfc, 0x66, 0x33, 0xee, 0xba, 0xe7, 0x90, 0xc1, 0xb6, 0xe8,
      0x8f, 0xfe, 0x1d, 0xc5, 0xa5, 0x97, 0x93, 0x3e, 0x04, 0x10, 0x38,
      0x62, 0xc6, 0x44, 0x12, 0xd5, 0x30, 0x00, 0xf8, 0xf2, 0x1b, 0xf0,
      0x6e, 0x10, 0x9b, 0xb8, 0x02, 0x02, 0x07, 0xd0,
  };

  ExpectBerConvert("kSimpleBER", kSimpleBER, kSimpleBER);
  ExpectBerConvert("kNonMinimalLengthBER", kNonMinimalLengthDER,
                   kNonMinimalLengthBER);
  ExpectBerConvert("kIndefBER", kIndefDER, kIndefBER);
  ExpectBerConvert("kIndefBER2", kIndefDER2, kIndefBER2);
  ExpectBerConvert("kOctetStringBER", kOctetStringDER, kOctetStringBER);
  ExpectBerConvert("kNSSBER", kNSSDER, kNSSBER);
  ExpectBerConvert("kConstructedStringBER", kConstructedStringDER,
                   kConstructedStringBER);
  ExpectBerConvert("kConstructedBitStringBER", kConstructedBitStringBER,
                   kConstructedBitStringBER);
  ExpectBerConvert("kWrappedIndefBER", kWrappedIndefDER, kWrappedIndefBER);
  ExpectBerConvert("kWrappedConstructedStringBER", kWrappedConstructedStringDER,
                   kWrappedConstructedStringBER);

  // indef_overflow is 200 levels deep of an indefinite-length-encoded SEQUENCE.
  // This will exceed our recursion limits and fail to be converted.
  std::vector<uint8_t> indef_overflow;
  for (int i = 0; i < 200; i++) {
    indef_overflow.push_back(0x30);
    indef_overflow.push_back(0x80);
  }
  for (int i = 0; i < 200; i++) {
    indef_overflow.push_back(0x00);
    indef_overflow.push_back(0x00);
  }
  CBS in, out;
  CBS_init(&in, indef_overflow.data(), indef_overflow.size());
  uint8_t *storage;
  ASSERT_FALSE(CBS_asn1_ber_to_der(&in, &out, &storage));
}

TEST(CBSTest, IndefiniteBerASN1Macros) {
  ExpectParse(d2i_ASN1_SEQUENCE_ANY,
              std::vector<uint8_t>(kIndefBER, kIndefBER + sizeof(kIndefBER)),
              true);
  ExpectParse(d2i_ASN1_TYPE,
              std::vector<uint8_t>(kIndefBER2, kIndefBER2 + sizeof(kIndefBER2)),
              true);
  ExpectParse(d2i_ASN1_SEQUENCE_ANY,
              std::vector<uint8_t>(kWrappedIndefBER,
                                   kWrappedIndefBER + sizeof(kWrappedIndefBER)),
              true);

  // Below use BER constructed strings.
  ExpectParse(d2i_ASN1_OCTET_STRING,
              std::vector<uint8_t>(kOctetStringBER,
                                   kOctetStringBER + sizeof(kOctetStringBER)),
              true);
  ExpectParse(d2i_ASN1_TYPE,
              std::vector<uint8_t>(kConstructedStringBER,
                                   kConstructedStringBER + sizeof(kConstructedStringBER)),
              true);
  ExpectParse(d2i_ASN1_SEQUENCE_ANY,
            std::vector<uint8_t>(kWrappedConstructedStringBER,
                                 kWrappedConstructedStringBER + sizeof(kWrappedConstructedStringBER)),
            true);
  ExpectParse(d2i_ASN1_BIT_STRING,
            std::vector<uint8_t>(kConstructedBitStringBER,
                                 kConstructedBitStringBER + sizeof(kConstructedBitStringBER)),
            true);
}

struct BERTest {
  const char *in_hex;
  bool ok;
  bool ber_found;
  bool indefinite;
  CBS_ASN1_TAG tag;
};

static const BERTest kBERTests[] = {
    // Trivial cases, also valid DER.
    {"0100", true, false, false, 1},
    {"020101", true, false, false, 2},

    // Non-minimally encoded lengths.
    {"02810101", true, true, false, 2},
    {"0282000101", true, true, false, 2},
    {"028300000101", true, true, false, 2},
    {"02840000000101", true, true, false, 2},
    // Technically valid BER, but not handled.
    {"02850000000101", false, false, false, 0},

    // Indefinite length, but not constructed.
    {"0280", false, false, false, 0},
    // Indefinite length.
    {"2280", true, true, true, CBS_ASN1_CONSTRUCTED | 2},
    // Indefinite length with multi-byte tag.
    {"bf1f80", true, true, true,
     CBS_ASN1_CONSTRUCTED | CBS_ASN1_CONTEXT_SPECIFIC | 31},
    // Invalid extended tag zero (X.690 8.1.2.4.2.c)
    {"3f0000", false, false, false, 0},
    // Should be a low-number tag form, even in BER.
    {"1f0100", false, false, false, 0},
    {"1f4000", true, false, false, 0x40},
    // Non-minimal tags are invalid, even in BER.
    {"1f804000", false, false, false, 0},

    // EOCs and other forms of tag [UNIVERSAL 0] are rejected as elements.
    {"0000", false, false, false, 0},
    {"000100", false, false, false, 0},
    {"00800000", false, false, false, 0},
    {"2000", false, false, false, 0},
};

TEST(CBSTest, BERElementTest) {
  for (const auto &test : kBERTests) {
    SCOPED_TRACE(test.in_hex);

    std::vector<uint8_t> in_bytes;
    ASSERT_TRUE(DecodeHex(&in_bytes, test.in_hex));
    CBS in(in_bytes);
    CBS out;
    CBS_ASN1_TAG tag;
    size_t header_len;
    int ber_found;
    int indefinite;
    int ok = CBS_get_any_ber_asn1_element(&in, &out, &tag, &header_len,
                                          &ber_found, &indefinite);
    ASSERT_TRUE((ok == 1) == test.ok);
    if (!test.ok) {
      continue;
    }

    EXPECT_EQ(test.ber_found ? 1 : 0, ber_found);
    EXPECT_EQ(test.indefinite ? 1 : 0, indefinite);
    EXPECT_LE(header_len, in_bytes.size());
    EXPECT_EQ(CBS_len(&out), in_bytes.size());
    EXPECT_EQ(CBS_len(&in), 0u);
    EXPECT_EQ(Bytes(out), Bytes(in_bytes));
    EXPECT_EQ(tag, test.tag);
  }
}

struct ImplicitStringTest {
  const char *in;
  size_t in_len;
  bool ok;
  const char *out;
  size_t out_len;
};

static const ImplicitStringTest kImplicitStringTests[] = {
    // A properly-encoded string.
    {"\x80\x03\x61\x61\x61", 5, true, "aaa", 3},
    // An implicit-tagged string.
    {"\xa0\x09\x04\x01\x61\x04\x01\x61\x04\x01\x61", 11, true, "aaa", 3},
    // |CBS_get_asn1_implicit_string| only accepts one level deep of nesting.
    {"\xa0\x0b\x24\x06\x04\x01\x61\x04\x01\x61\x04\x01\x61", 13, false, nullptr,
     0},
    // The outer tag must match.
    {"\x81\x03\x61\x61\x61", 5, false, nullptr, 0},
    {"\xa1\x09\x04\x01\x61\x04\x01\x61\x04\x01\x61", 11, false, nullptr, 0},
    // The inner tag must match.
    {"\xa1\x09\x0c\x01\x61\x0c\x01\x61\x0c\x01\x61", 11, false, nullptr, 0},
};

TEST(CBSTest, ImplicitString) {
  for (const auto &test : kImplicitStringTests) {
    SCOPED_TRACE(Bytes(test.in, test.in_len));
    uint8_t *storage = nullptr;
    CBS in, out;
    CBS_init(&in, reinterpret_cast<const uint8_t *>(test.in), test.in_len);
    int ok = CBS_get_asn1_implicit_string(&in, &out, &storage,
                                          CBS_ASN1_CONTEXT_SPECIFIC | 0,
                                          CBS_ASN1_OCTETSTRING);
    bssl::UniquePtr<uint8_t> scoper(storage);
    EXPECT_EQ(test.ok, static_cast<bool>(ok));

    if (ok) {
      EXPECT_EQ(Bytes(test.out, test.out_len),
                Bytes(CBS_data(&out), CBS_len(&out)));
    }
  }
}

struct ASN1Uint64Test {
  uint64_t value;
  const char *encoding;
  size_t encoding_len;
};

static const ASN1Uint64Test kASN1Uint64Tests[] = {
    {0, "\x02\x01\x00", 3},
    {1, "\x02\x01\x01", 3},
    {127, "\x02\x01\x7f", 3},
    {128, "\x02\x02\x00\x80", 4},
    {0xdeadbeef, "\x02\x05\x00\xde\xad\xbe\xef", 7},
    {UINT64_C(0x0102030405060708),
     "\x02\x08\x01\x02\x03\x04\x05\x06\x07\x08", 10},
    {UINT64_C(0xffffffffffffffff),
      "\x02\x09\x00\xff\xff\xff\xff\xff\xff\xff\xff", 11},
};

struct ASN1InvalidUint64Test {
  const char *encoding;
  size_t encoding_len;
  bool overflow;
};

static const ASN1InvalidUint64Test kASN1InvalidUint64Tests[] = {
    // Bad tag.
    {"\x03\x01\x00", 3, false},
    // Empty contents.
    {"\x02\x00", 2, false},
    // Negative number.
    {"\x02\x01\x80", 3, false},
    // Overflow.
    {"\x02\x09\x01\x00\x00\x00\x00\x00\x00\x00\x00", 11, true},
    // Leading zeros.
    {"\x02\x02\x00\x01", 4, false},
};

TEST(CBSTest, ASN1Uint64) {
  for (const ASN1Uint64Test &test : kASN1Uint64Tests) {
    SCOPED_TRACE(Bytes(test.encoding, test.encoding_len));
    SCOPED_TRACE(test.value);
    CBS cbs;
    uint64_t value;
    uint8_t *out;
    size_t len;

    CBS_init(&cbs, (const uint8_t *)test.encoding, test.encoding_len);
    ASSERT_TRUE(CBS_get_asn1_uint64(&cbs, &value));
    EXPECT_EQ(0u, CBS_len(&cbs));
    EXPECT_EQ(test.value, value);

    CBS child;
    int is_negative;
    CBS_init(&cbs, (const uint8_t *)test.encoding, test.encoding_len);
    ASSERT_TRUE(CBS_get_asn1(&cbs, &child, CBS_ASN1_INTEGER));
    EXPECT_TRUE(CBS_is_valid_asn1_integer(&child, &is_negative));
    EXPECT_EQ(0, is_negative);
    EXPECT_TRUE(CBS_is_unsigned_asn1_integer(&child));

    {
      bssl::ScopedCBB cbb;
      ASSERT_TRUE(CBB_init(cbb.get(), 0));
      ASSERT_TRUE(CBB_add_asn1_uint64(cbb.get(), test.value));
      ASSERT_TRUE(CBB_finish(cbb.get(), &out, &len));
      bssl::UniquePtr<uint8_t> scoper(out);
      EXPECT_EQ(Bytes(test.encoding, test.encoding_len), Bytes(out, len));
    }

    {
      // Overwrite the tag.
      bssl::ScopedCBB cbb;
      ASSERT_TRUE(CBB_init(cbb.get(), 0));
      ASSERT_TRUE(CBB_add_asn1_uint64_with_tag(cbb.get(), test.value,
                                               CBS_ASN1_CONTEXT_SPECIFIC | 1));
      ASSERT_TRUE(CBB_finish(cbb.get(), &out, &len));
      bssl::UniquePtr<uint8_t> scoper(out);
      std::vector<uint8_t> expected(test.encoding,
                                    test.encoding + test.encoding_len);
      expected[0] = 0x81;
      EXPECT_EQ(Bytes(expected), Bytes(out, len));
    }
  }

  for (const ASN1InvalidUint64Test &test : kASN1InvalidUint64Tests) {
    SCOPED_TRACE(Bytes(test.encoding, test.encoding_len));
    CBS cbs;
    uint64_t value;

    CBS_init(&cbs, (const uint8_t *)test.encoding, test.encoding_len);
    EXPECT_FALSE(CBS_get_asn1_uint64(&cbs, &value));

    CBS_init(&cbs, (const uint8_t *)test.encoding, test.encoding_len);
    CBS child;
    if (CBS_get_asn1(&cbs, &child, CBS_ASN1_INTEGER)) {
      EXPECT_EQ(test.overflow, !!CBS_is_unsigned_asn1_integer(&child));
    }
  }
}

struct ASN1Int64Test {
  int64_t value;
  const char *encoding;
  size_t encoding_len;
};

static const ASN1Int64Test kASN1Int64Tests[] = {
    {0, "\x02\x01\x00", 3},
    {1, "\x02\x01\x01", 3},
    {-1, "\x02\x01\xff", 3},
    {127, "\x02\x01\x7f", 3},
    {-127, "\x02\x01\x81", 3},
    {128, "\x02\x02\x00\x80", 4},
    {-128, "\x02\x01\x80", 3},
    {129, "\x02\x02\x00\x81", 4},
    {-129, "\x02\x02\xff\x7f", 4},
    {0xdeadbeef, "\x02\x05\x00\xde\xad\xbe\xef", 7},
    {INT64_C(0x0102030405060708), "\x02\x08\x01\x02\x03\x04\x05\x06\x07\x08",
     10},
    {INT64_MIN, "\x02\x08\x80\x00\x00\x00\x00\x00\x00\x00", 10},
    {INT64_MAX, "\x02\x08\x7f\xff\xff\xff\xff\xff\xff\xff", 10},
};

struct ASN1InvalidInt64Test {
  const char *encoding;
  size_t encoding_len;
  bool overflow;
};

static const ASN1InvalidInt64Test kASN1InvalidInt64Tests[] = {
    // Bad tag.
    {"\x03\x01\x00", 3, false},
    // Empty contents.
    {"\x02\x00", 2, false},
    // Overflow.
    {"\x02\x09\x01\x00\x00\x00\x00\x00\x00\x00\x00", 11, true},
    // Underflow.
    {"\x02\x09\x08\xff\xff\xff\xff\xff\xff\xff\xff", 11, true},
    // Leading zeros.
    {"\x02\x02\x00\x01", 4, false},
    // Leading 0xff.
    {"\x02\x02\xff\xff", 4, false},
};

TEST(CBSTest, ASN1Int64) {
  for (const ASN1Int64Test &test : kASN1Int64Tests) {
    SCOPED_TRACE(Bytes(test.encoding, test.encoding_len));
    SCOPED_TRACE(test.value);
    CBS cbs;
    int64_t value;
    uint8_t *out;
    size_t len;

    CBS_init(&cbs, (const uint8_t *)test.encoding, test.encoding_len);
    ASSERT_TRUE(CBS_get_asn1_int64(&cbs, &value));
    EXPECT_EQ(0u, CBS_len(&cbs));
    EXPECT_EQ(test.value, value);

    CBS child;
    int is_negative;
    CBS_init(&cbs, (const uint8_t *)test.encoding, test.encoding_len);
    ASSERT_TRUE(CBS_get_asn1(&cbs, &child, CBS_ASN1_INTEGER));
    EXPECT_TRUE(CBS_is_valid_asn1_integer(&child, &is_negative));
    EXPECT_EQ(test.value < 0, !!is_negative);
    EXPECT_EQ(test.value >= 0, !!CBS_is_unsigned_asn1_integer(&child));

    {
      bssl::ScopedCBB cbb;
      ASSERT_TRUE(CBB_init(cbb.get(), 0));
      ASSERT_TRUE(CBB_add_asn1_int64(cbb.get(), test.value));
      ASSERT_TRUE(CBB_finish(cbb.get(), &out, &len));
      bssl::UniquePtr<uint8_t> scoper(out);
      EXPECT_EQ(Bytes(test.encoding, test.encoding_len), Bytes(out, len));
    }

    {
      // Overwrite the tag.
      bssl::ScopedCBB cbb;
      ASSERT_TRUE(CBB_init(cbb.get(), 0));
      ASSERT_TRUE(CBB_add_asn1_int64_with_tag(cbb.get(), test.value,
                                              CBS_ASN1_CONTEXT_SPECIFIC | 1));
      ASSERT_TRUE(CBB_finish(cbb.get(), &out, &len));
      bssl::UniquePtr<uint8_t> scoper(out);
      std::vector<uint8_t> expected(test.encoding,
                                    test.encoding + test.encoding_len);
      expected[0] = 0x81;
      EXPECT_EQ(Bytes(expected), Bytes(out, len));
    }
  }

  for (const ASN1InvalidInt64Test &test : kASN1InvalidInt64Tests) {
    SCOPED_TRACE(Bytes(test.encoding, test.encoding_len));
    CBS cbs;
    int64_t value;

    CBS_init(&cbs, (const uint8_t *)test.encoding, test.encoding_len);
    EXPECT_FALSE(CBS_get_asn1_int64(&cbs, &value));

    CBS_init(&cbs, (const uint8_t *)test.encoding, test.encoding_len);
    CBS child;
    if (CBS_get_asn1(&cbs, &child, CBS_ASN1_INTEGER)) {
      EXPECT_EQ(test.overflow, !!CBS_is_valid_asn1_integer(&child, NULL));
    }
  }
}

TEST(CBBTest, Zero) {
  CBB cbb;
  CBB_zero(&cbb);
  // Calling |CBB_cleanup| on a zero-state |CBB| must not crash.
  CBB_cleanup(&cbb);
}

TEST(CBBTest, Reserve) {
  uint8_t buf[10];
  uint8_t *ptr;
  size_t len;
  bssl::ScopedCBB cbb;
  ASSERT_TRUE(CBB_init_fixed(cbb.get(), buf, sizeof(buf)));
  // Too large.
  EXPECT_FALSE(CBB_reserve(cbb.get(), &ptr, 11));

  cbb.Reset();
  ASSERT_TRUE(CBB_init_fixed(cbb.get(), buf, sizeof(buf)));
  // Successfully reserve the entire space.
  ASSERT_TRUE(CBB_reserve(cbb.get(), &ptr, 10));
  EXPECT_EQ(buf, ptr);
  // Advancing under the maximum bytes is legal.
  ASSERT_TRUE(CBB_did_write(cbb.get(), 5));
  ASSERT_TRUE(CBB_finish(cbb.get(), NULL, &len));
  EXPECT_EQ(5u, len);
}

// Test that CBB errors are sticky; once on operation on CBB fails, all
// subsequent ones do.
TEST(CBBTest, StickyError) {
  // Write an input that exceeds the limit for its length prefix.
  bssl::ScopedCBB cbb;
  CBB child;
  static const uint8_t kZeros[256] = {0};
  ASSERT_TRUE(CBB_init(cbb.get(), 0));
  ASSERT_TRUE(CBB_add_u8_length_prefixed(cbb.get(), &child));
  ASSERT_TRUE(CBB_add_bytes(&child, kZeros, sizeof(kZeros)));
  ASSERT_FALSE(CBB_flush(cbb.get()));

  // All future operations should fail.
  uint8_t *ptr;
  size_t len;
  EXPECT_FALSE(CBB_add_u8(cbb.get(), 0));
  EXPECT_FALSE(CBB_finish(cbb.get(), &ptr, &len));

  // Write an input that cannot fit in a fixed CBB.
  cbb.Reset();
  uint8_t buf;
  ASSERT_TRUE(CBB_init_fixed(cbb.get(), &buf, 1));
  ASSERT_FALSE(CBB_add_bytes(cbb.get(), kZeros, sizeof(kZeros)));

  // All future operations should fail.
  EXPECT_FALSE(CBB_add_u8(cbb.get(), 0));
  EXPECT_FALSE(CBB_finish(cbb.get(), &ptr, &len));

  // Write a u32 that cannot fit in a u24.
  cbb.Reset();
  ASSERT_TRUE(CBB_init(cbb.get(), 0));
  ASSERT_FALSE(CBB_add_u24(cbb.get(), 1u << 24));

  // All future operations should fail.
  EXPECT_FALSE(CBB_add_u8(cbb.get(), 0));
  EXPECT_FALSE(CBB_finish(cbb.get(), &ptr, &len));
}

TEST(CBSTest, BitString) {
  static const std::vector<uint8_t> kValidBitStrings[] = {
      {0x00},                                      // 0 bits
      {0x07, 0x80},                                // 1 bit
      {0x04, 0xf0},                                // 4 bits
      {0x00, 0xff},                                // 8 bits
      {0x06, 0xff, 0xff, 0xff, 0xff, 0xff, 0xc0},  // 42 bits
  };
  for (const auto& test : kValidBitStrings) {
    SCOPED_TRACE(Bytes(test.data(), test.size()));
    CBS cbs;
    CBS_init(&cbs, test.data(), test.size());
    EXPECT_TRUE(CBS_is_valid_asn1_bitstring(&cbs));
  }

  static const std::vector<uint8_t> kInvalidBitStrings[] = {
      // BIT STRINGs always have a leading byte.
      std::vector<uint8_t>{},
      // It's not possible to take an unused bit off the empty string.
      {0x01},
      // There can be at most 7 unused bits.
      {0x08, 0xff},
      {0xff, 0xff},
      // All unused bits must be cleared.
      {0x06, 0xff, 0xc1},
  };
  for (const auto& test : kInvalidBitStrings) {
    SCOPED_TRACE(Bytes(test.data(), test.size()));
    CBS cbs;
    CBS_init(&cbs, test.data(), test.size());
    EXPECT_FALSE(CBS_is_valid_asn1_bitstring(&cbs));

    // CBS_asn1_bitstring_has_bit returns false on invalid inputs.
    EXPECT_FALSE(CBS_asn1_bitstring_has_bit(&cbs, 0));
  }

  static const struct {
    std::vector<uint8_t> in;
    unsigned bit;
    bool bit_set;
  } kBitTests[] = {
      // Basic tests.
      {{0x00}, 0, false},
      {{0x07, 0x80}, 0, true},
      {{0x06, 0x0f, 0x40}, 0, false},
      {{0x06, 0x0f, 0x40}, 1, false},
      {{0x06, 0x0f, 0x40}, 2, false},
      {{0x06, 0x0f, 0x40}, 3, false},
      {{0x06, 0x0f, 0x40}, 4, true},
      {{0x06, 0x0f, 0x40}, 5, true},
      {{0x06, 0x0f, 0x40}, 6, true},
      {{0x06, 0x0f, 0x40}, 7, true},
      {{0x06, 0x0f, 0x40}, 8, false},
      {{0x06, 0x0f, 0x40}, 9, true},
      // Out-of-bounds bits return 0.
      {{0x06, 0x0f, 0x40}, 10, false},
      {{0x06, 0x0f, 0x40}, 15, false},
      {{0x06, 0x0f, 0x40}, 16, false},
      {{0x06, 0x0f, 0x40}, 1000, false},
  };
  for (const auto& test : kBitTests) {
    SCOPED_TRACE(Bytes(test.in.data(), test.in.size()));
    SCOPED_TRACE(test.bit);
    CBS cbs;
    CBS_init(&cbs, test.in.data(), test.in.size());
    EXPECT_EQ(static_cast<int>(test.bit_set),
              CBS_asn1_bitstring_has_bit(&cbs, test.bit));
  }
}

TEST(CBBTest, AddOIDFromText) {
  const struct {
    const char *text;
    std::vector<uint8_t> der;
  } kValidOIDs[] = {
      // Some valid values.
      {"0.0", {0x00}},
      {"0.2.3.4", {0x2, 0x3, 0x4}},
      {"1.2.3.4", {0x2a, 0x3, 0x4}},
      {"2.2.3.4", {0x52, 0x3, 0x4}},
      {"1.2.840.113554.4.1.72585",
       {0x2a, 0x86, 0x48, 0x86, 0xf7, 0x12, 0x04, 0x01, 0x84, 0xb7, 0x09}},
      // Test edge cases around the first component.
      {"0.39", {0x27}},
      {"1.0", {0x28}},
      {"1.39", {0x4f}},
      {"2.0", {0x50}},
      {"2.1", {0x51}},
      {"2.40", {0x78}},
      // Edge cases near an overflow.
      {"1.2.18446744073709551615",
       {0x2a, 0x81, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x7f}},
      {"2.18446744073709551535",
       {0x81, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x7f}},
  };

  const char *kInvalidTexts[] = {
      // Invalid second component.
      "0.40",
      "1.40",
      // Invalid first component.
      "3.1",
      // The empty string is not an OID.
      "",
      // No empty components.
      ".1.2.3.4.5",
      "1..2.3.4.5",
      "1.2.3.4.5.",
      // There must be at least two components.
      "1",
      // No extra leading zeros.
      "00.1.2.3.4",
      "01.1.2.3.4",
      // Overflow for both components or 40*A + B.
      "1.2.18446744073709551616",
      "2.18446744073709551536",
  };

  const struct {
    std::vector<uint8_t> der;
    // If true, |der| is valid but has a component that exceeds 2^64-1.
    bool overflow;
  } kInvalidDER[] = {
      // The empty string is not an OID.
      {{}, false},
      // Non-minimal representation.
      {{0x80, 0x01}, false},
      // Unterminated integer.
      {{0x01, 0x02, 0x83}, false},
      // Overflow. This is the DER representation of
      // 1.2.840.113554.4.1.72585.18446744073709551616. (The final value is
      // 2^64.)
      {{0x2a, 0x86, 0x48, 0x86, 0xf7, 0x12, 0x04, 0x01, 0x84, 0xb7, 0x09,
        0x82, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x00},
       true},
  };

  for (const auto &t : kValidOIDs) {
    SCOPED_TRACE(t.text);

    bssl::ScopedCBB cbb;
    ASSERT_TRUE(CBB_init(cbb.get(), 0));
    ASSERT_TRUE(CBB_add_asn1_oid_from_text(cbb.get(), t.text, strlen(t.text)));
    uint8_t *out;
    size_t len;
    ASSERT_TRUE(CBB_finish(cbb.get(), &out, &len));
    bssl::UniquePtr<uint8_t> free_out(out);
    EXPECT_EQ(Bytes(t.der), Bytes(out, len));

    CBS cbs;
    CBS_init(&cbs, t.der.data(), t.der.size());
    bssl::UniquePtr<char> text(CBS_asn1_oid_to_text(&cbs));
    ASSERT_TRUE(text.get());
    EXPECT_STREQ(t.text, text.get());

    EXPECT_TRUE(CBS_is_valid_asn1_oid(&cbs));
  }

  for (const char *t : kInvalidTexts) {
    SCOPED_TRACE(t);
    bssl::ScopedCBB cbb;
    ASSERT_TRUE(CBB_init(cbb.get(), 0));
    EXPECT_FALSE(CBB_add_asn1_oid_from_text(cbb.get(), t, strlen(t)));
  }

  for (const auto &t : kInvalidDER) {
    SCOPED_TRACE(Bytes(t.der));
    CBS cbs;
    CBS_init(&cbs, t.der.data(), t.der.size());
    bssl::UniquePtr<char> text(CBS_asn1_oid_to_text(&cbs));
    EXPECT_FALSE(text);
    EXPECT_EQ(t.overflow ? 1 : 0, CBS_is_valid_asn1_oid(&cbs));
  }
}

TEST(CBBTest, FlushASN1SetOf) {
  const struct {
    std::vector<uint8_t> in, out;
  } kValidInputs[] = {
    // No elements.
    {{}, {}},
    // One element.
    {{0x30, 0x00}, {0x30, 0x00}},
    // Two identical elements.
    {{0x30, 0x00, 0x30, 0x00}, {0x30, 0x00, 0x30, 0x00}},
    // clang-format off
    {{0x30, 0x02, 0x00, 0x00,
      0x30, 0x00,
      0x01, 0x00,
      0x30, 0x02, 0x00, 0x00,
      0x30, 0x03, 0x00, 0x00, 0x00,
      0x30, 0x00,
      0x30, 0x03, 0x00, 0x00, 0x01,
      0x30, 0x01, 0x00,
      0x01, 0x01, 0x00},
     {0x01, 0x00,
      0x01, 0x01, 0x00,
      0x30, 0x00,
      0x30, 0x00,
      0x30, 0x01, 0x00,
      0x30, 0x02, 0x00, 0x00,
      0x30, 0x02, 0x00, 0x00,
      0x30, 0x03, 0x00, 0x00, 0x00,
      0x30, 0x03, 0x00, 0x00, 0x01}},
    // clang-format on
  };

  for (const auto &t : kValidInputs) {
    SCOPED_TRACE(Bytes(t.in));

    bssl::ScopedCBB cbb;
    CBB child;
    ASSERT_TRUE(CBB_init(cbb.get(), 0));
    ASSERT_TRUE(CBB_add_asn1(cbb.get(), &child, CBS_ASN1_SET));
    ASSERT_TRUE(CBB_add_bytes(&child, t.in.data(), t.in.size()));
    ASSERT_TRUE(CBB_flush_asn1_set_of(&child));
    EXPECT_EQ(Bytes(t.out), Bytes(CBB_data(&child), CBB_len(&child)));

    // Running it again should be idempotent.
    ASSERT_TRUE(CBB_flush_asn1_set_of(&child));
    EXPECT_EQ(Bytes(t.out), Bytes(CBB_data(&child), CBB_len(&child)));

    // The ASN.1 header remain intact.
    ASSERT_TRUE(CBB_flush(cbb.get()));
    EXPECT_EQ(0x31, CBB_data(cbb.get())[0]);
  }

  const std::vector<uint8_t> kInvalidInputs[] = {
    {0x30},
    {0x30, 0x01},
    {0x30, 0x00, 0x30, 0x00, 0x30, 0x01},
  };

  for (const auto &t : kInvalidInputs) {
    SCOPED_TRACE(Bytes(t));

    bssl::ScopedCBB cbb;
    CBB child;
    ASSERT_TRUE(CBB_init(cbb.get(), 0));
    ASSERT_TRUE(CBB_add_asn1(cbb.get(), &child, CBS_ASN1_SET));
    ASSERT_TRUE(CBB_add_bytes(&child, t.data(), t.size()));
    EXPECT_FALSE(CBB_flush_asn1_set_of(&child));
  }
}

template <class T>
static std::vector<uint8_t> LiteralToBytes(const T *str) {
  std::vector<uint8_t> ret;
  for (; *str != 0; str++) {
    for (size_t i = 0; i < sizeof(T); i++) {
      ret.push_back(static_cast<uint8_t>(*str >> (8 * (sizeof(T) - 1 - i))));
    }
  }
  return ret;
}

static std::vector<uint32_t> LiteralToCodePoints(const char32_t *str) {
  std::vector<uint32_t> ret;
  for (; *str != 0; str++) {
    ret.push_back(static_cast<uint32_t>(*str));
  }
  return ret;
}

TEST(CBBTest, Unicode) {
  struct {
    int (*decode)(CBS *, uint32_t *);
    int (*encode)(CBB *, uint32_t);
    std::vector<uint8_t> in;
    std::vector<uint32_t> out;
    bool ok;
  } kTests[] = {
      {cbs_get_utf8, cbb_add_utf8,
       // This test string captures all four cases in UTF-8.
       LiteralToBytes(u8"Hello, 世界! ¡Hola, 🌎!"),
       LiteralToCodePoints(U"Hello, 世界! ¡Hola, 🌎!"), true},

      // Some invalid inputs adapted from
      // http://www.cl.cam.ac.uk/~mgk25/ucs/examples/UTF-8-test.txt
      // 2.1  First possible sequence of a certain length. (5- and 6-bit
      // sequences no longer exist.)
      {cbs_get_utf8, cbb_add_utf8, {0xf8, 0x88, 0x80, 0x80, 0x80}, {}, false},
      {cbs_get_utf8,
       cbb_add_utf8,
       {0xfc, 0x84, 0x80, 0x80, 0x80, 0x80},
       {},
       false},
      // 3.1  Unexpected continuation bytes.
      {cbs_get_utf8, cbb_add_utf8, {0x80}, {}, false},
      {cbs_get_utf8, cbb_add_utf8, {0xbf}, {}, false},
      // 3.2  Lonely start characters.
      {cbs_get_utf8, cbb_add_utf8, {0xc0, ' '}, {}, false},
      {cbs_get_utf8, cbb_add_utf8, {0xe0, ' '}, {}, false},
      {cbs_get_utf8, cbb_add_utf8, {0xf0, ' '}, {}, false},
      // 3.3  Sequences with last continuation byte missing
      {cbs_get_utf8, cbb_add_utf8, {0xc0}, {}, false},
      {cbs_get_utf8, cbb_add_utf8, {0xe0, 0x80}, {}, false},
      {cbs_get_utf8, cbb_add_utf8, {0xf0, 0x80, 0x80}, {}, false},
      // Variation of the above with unexpected spaces.
      {cbs_get_utf8, cbb_add_utf8, {0xe0, 0x80, ' '}, {}, false},
      {cbs_get_utf8, cbb_add_utf8, {0xf0, 0x80, 0x80, ' '}, {}, false},
      // 4.1  Examples of an overlong ASCII character
      {cbs_get_utf8, cbb_add_utf8, {0xc0, 0xaf}, {}, false},
      {cbs_get_utf8, cbb_add_utf8, {0xe0, 0x80, 0xaf}, {}, false},
      {cbs_get_utf8, cbb_add_utf8, {0xf0, 0x80, 0x80, 0xaf}, {}, false},
      // 4.2  Maximum overlong sequences
      {cbs_get_utf8, cbb_add_utf8, {0xc1, 0xbf}, {}, false},
      {cbs_get_utf8, cbb_add_utf8, {0xe0, 0x9f, 0xbf}, {}, false},
      {cbs_get_utf8, cbb_add_utf8, {0xf0, 0x8f, 0xbf, 0xbf}, {}, false},
      // 4.3  Overlong representation of the NUL character
      {cbs_get_utf8, cbb_add_utf8, {0xc0, 0x80}, {}, false},
      {cbs_get_utf8, cbb_add_utf8, {0xe0, 0x80, 0x80}, {}, false},
      {cbs_get_utf8, cbb_add_utf8, {0xf0, 0x80, 0x80, 0x80}, {}, false},
      // 5.1  Single UTF-16 surrogates
      {cbs_get_utf8, cbb_add_utf8, {0xed, 0xa0, 0x80}, {}, false},
      {cbs_get_utf8, cbb_add_utf8, {0xed, 0xad, 0xbf}, {}, false},
      {cbs_get_utf8, cbb_add_utf8, {0xed, 0xae, 0x80}, {}, false},
      {cbs_get_utf8, cbb_add_utf8, {0xed, 0xb0, 0x80}, {}, false},
      {cbs_get_utf8, cbb_add_utf8, {0xed, 0xbe, 0x80}, {}, false},
      {cbs_get_utf8, cbb_add_utf8, {0xed, 0xbf, 0xbf}, {}, false},
      // 5.2  Paired UTF-16 surrogates
      {cbs_get_utf8,
       cbb_add_utf8,
       {0xed, 0xa0, 0x80, 0xed, 0xb0, 0x80},
       {},
       false},
      {cbs_get_utf8,
       cbb_add_utf8,
       {0xed, 0xa0, 0x80, 0xed, 0xbf, 0xbf},
       {},
       false},
      {cbs_get_utf8,
       cbb_add_utf8,
       {0xed, 0xad, 0xbf, 0xed, 0xb0, 0x80},
       {},
       false},
      {cbs_get_utf8,
       cbb_add_utf8,
       {0xed, 0xad, 0xbf, 0xed, 0xbf, 0xbf},
       {},
       false},
      {cbs_get_utf8,
       cbb_add_utf8,
       {0xed, 0xae, 0x80, 0xed, 0xb0, 0x80},
       {},
       false},
      {cbs_get_utf8,
       cbb_add_utf8,
       {0xed, 0xae, 0x80, 0xed, 0xbf, 0xbf},
       {},
       false},
      {cbs_get_utf8,
       cbb_add_utf8,
       {0xed, 0xaf, 0xbf, 0xed, 0xb0, 0x80},
       {},
       false},
      {cbs_get_utf8,
       cbb_add_utf8,
       {0xed, 0xaf, 0xbf, 0xed, 0xbf, 0xbf},
       {},
       false},
      // 5.3  Noncharacter code positions
      {cbs_get_utf8, cbb_add_utf8, {0xef, 0xbf, 0xbe}, {}, false},
      {cbs_get_utf8, cbb_add_utf8, {0xef, 0xbf, 0xbf}, {}, false},
      {cbs_get_utf8, cbb_add_utf8, {0xef, 0xb7, 0x90}, {}, false},
      {cbs_get_utf8, cbb_add_utf8, {0xef, 0xb7, 0xaf}, {}, false},
      {cbs_get_utf8, cbb_add_utf8, {0xf0, 0x9f, 0xbf, 0xbe}, {}, false},
      {cbs_get_utf8, cbb_add_utf8, {0xf0, 0x9f, 0xbf, 0xbf}, {}, false},

      {cbs_get_latin1, cbb_add_latin1, LiteralToBytes("\xa1Hola!"),
       LiteralToCodePoints(U"¡Hola!"), true},

      // UCS-2 matches UTF-16 on the BMP.
      {cbs_get_ucs2_be, cbb_add_ucs2_be, LiteralToBytes(u"Hello, 世界!"),
       LiteralToCodePoints(U"Hello, 世界!"), true},
      // It does not support characters beyond the BMP.
      {cbs_get_ucs2_be, cbb_add_ucs2_be,
       LiteralToBytes(u"Hello, 世界! ¡Hola, 🌎!"),
       LiteralToCodePoints(U"Hello, 世界! ¡Hola, "), false},
      // Unpaired surrogates and non-characters are also rejected.
      {cbs_get_ucs2_be, cbb_add_ucs2_be, {0xd8, 0x00}, {}, false},
      {cbs_get_ucs2_be, cbb_add_ucs2_be, {0xff, 0xfe}, {}, false},

      {cbs_get_utf32_be, cbb_add_utf32_be,
       LiteralToBytes(U"Hello, 世界! ¡Hola, 🌎!"),
       LiteralToCodePoints(U"Hello, 世界! ¡Hola, 🌎!"), true},
      // Unpaired surrogates and non-characters are rejected.
      {cbs_get_utf32_be, cbb_add_utf32_be, {0x00, 0x00, 0xd8, 0x00}, {}, false},
      {cbs_get_utf32_be, cbb_add_utf32_be, {0x00, 0x00, 0xff, 0xfe}, {}, false},

      // Test that the NUL character can be encoded.
      {cbs_get_latin1, cbb_add_latin1, {0}, {0}, true},
      {cbs_get_utf8, cbb_add_utf8, {0}, {0}, true},
      {cbs_get_ucs2_be, cbb_add_ucs2_be, {0, 0}, {0}, true},
      {cbs_get_utf32_be, cbb_add_utf32_be, {0, 0, 0, 0}, {0}, true},
  };
  for (const auto &t : kTests) {
    SCOPED_TRACE(Bytes(t.in));

    // Test decoding.
    CBS cbs;
    CBS_init(&cbs, t.in.data(), t.in.size());
    std::vector<uint32_t> out;
    bool ok = true;
    while (CBS_len(&cbs) != 0) {
      uint32_t u;
      if (!t.decode(&cbs, &u)) {
        ok = false;
        break;
      }
      out.push_back(u);
    }
    EXPECT_EQ(t.ok, ok);
    EXPECT_EQ(t.out, out);

    // Test encoding.
    if (t.ok) {
      bssl::ScopedCBB cbb;
      ASSERT_TRUE(CBB_init(cbb.get(), 0));
      for (uint32_t u : t.out) {
        ASSERT_TRUE(t.encode(cbb.get(), u));
      }
      EXPECT_EQ(Bytes(t.in), Bytes(CBB_data(cbb.get()), CBB_len(cbb.get())));
    }
  }

  static const uint32_t kBadCodePoints[] = {
    // Surrogate pairs.
    0xd800,
    0xdfff,
    // Non-characters.
    0xfffe,
    0xffff,
    0xfdd0,
    0x1fffe,
    0x1ffff,
    // Too big.
    0x110000,
  };
  bssl::ScopedCBB cbb;
  ASSERT_TRUE(CBB_init(cbb.get(), 0));
  for (uint32_t v : kBadCodePoints) {
    SCOPED_TRACE(v);
    EXPECT_FALSE(cbb_add_utf8(cbb.get(), v));
    EXPECT_FALSE(cbb_add_latin1(cbb.get(), v));
    EXPECT_FALSE(cbb_add_ucs2_be(cbb.get(), v));
    EXPECT_FALSE(cbb_add_utf32_be(cbb.get(), v));
  }

  // Additional values that are out of range.
  EXPECT_FALSE(cbb_add_latin1(cbb.get(), 0x100));
  EXPECT_FALSE(cbb_add_ucs2_be(cbb.get(), 0x10000));

  EXPECT_EQ(1u, cbb_get_utf8_len(0));
  EXPECT_EQ(1u, cbb_get_utf8_len(0x7f));
  EXPECT_EQ(2u, cbb_get_utf8_len(0x80));
  EXPECT_EQ(2u, cbb_get_utf8_len(0x7ff));
  EXPECT_EQ(3u, cbb_get_utf8_len(0x800));
  EXPECT_EQ(3u, cbb_get_utf8_len(0xffff));
  EXPECT_EQ(4u, cbb_get_utf8_len(0x10000));
  EXPECT_EQ(4u, cbb_get_utf8_len(0x10ffff));
}

TEST(CBSTest, BogusTime) {
  static const struct {
    const char *timestring;
  } kBogusTimeTests[] = {
      {""},
      {"invalidtimesZ"},
      {"Z"},
      {"0000"},
      {"9999Z"},
      {"00000000000000000000000000000Z"},
      {"19491231235959"},
      {"500101000000.001Z"},
      {"500101000000+6"},
      {"-1970010100000Z"},
      {"7a0101000000Z"},
      {"20500101000000-6"},
      {"20500101000000.001"},
      {"20500229000000Z"},
      {"220229000000Z"},
      {"20500132000000Z"},
      {"220132000000Z"},
      {"20500332000000Z"},
      {"220332000000Z"},
      {"20500532000000Z"},
      {"220532000000Z"},
      {"20500732000000Z"},
      {"220732000000Z"},
      {"20500832000000Z"},
      {"220832000000Z"},
      {"20501032000000Z"},
      {"221032000000Z"},
      {"20501232000000Z"},
      {"221232000000Z"},
      {"20500431000000Z"},
      {"220431000000Z"},
      {"20500631000000Z"},
      {"220631000000Z"},
      {"20500931000000Z"},
      {"220931000000Z"},
      {"20501131000000Z"},
      {"221131000000Z"},
      {"20501100000000Z"},
      {"221100000000Z"},
      {"19500101000000+0600"},
  };
  for (const auto &t : kBogusTimeTests) {
    SCOPED_TRACE(t.timestring);
    CBS cbs;
    CBS_init(&cbs, (const uint8_t *)t.timestring, strlen(t.timestring));
    EXPECT_FALSE(CBS_parse_generalized_time(&cbs, NULL,
                                            /*allow_timezone_offset=*/0));
    EXPECT_FALSE(CBS_parse_utc_time(&cbs, NULL, /*allow_timezone_offset=*/1));
  }
  static const struct {
    const char *timestring;
  } kUTCTZTests[] = {
      {"480711220333-0700"},
      {"140704000000-0700"},
      {"480222202332-0500"},
      {"480726113216-0000"},
      {"480726113216-2359"},
  };
  for (const auto &t : kUTCTZTests) {
    SCOPED_TRACE(t.timestring);
    CBS cbs;
    CBS_init(&cbs, (const uint8_t *)t.timestring, strlen(t.timestring));
    EXPECT_FALSE(CBS_parse_generalized_time(&cbs, NULL,
                                            /*allow_timezone_offset=*/0));
    EXPECT_FALSE(CBS_parse_generalized_time(&cbs, NULL,
                                            /*allow_timezone_offset=*/1));
    EXPECT_TRUE(CBS_parse_utc_time(&cbs, NULL, /*allow_timezone_offset=*/1));
    EXPECT_FALSE(CBS_parse_utc_time(&cbs, NULL, /*allow_timezone_offset=*/0));
  }
  static const struct {
    const char *timestring;
  } kBogusUTCTZTests[] = {
      {"480711220333-0160"},
      {"140704000000-9999"},
      {"480222202332-2400"},
  };
  for (const auto &t : kBogusUTCTZTests) {
    SCOPED_TRACE(t.timestring);
    CBS cbs;
    CBS_init(&cbs, (const uint8_t *)t.timestring, strlen(t.timestring));
    EXPECT_FALSE(CBS_parse_generalized_time(&cbs, NULL,
                                            /*allow_timezone_offset=*/0));
    EXPECT_FALSE(CBS_parse_utc_time(&cbs, NULL, /*allow_timezone_offset=*/1));
  }
  static const struct {
    const char *timestring;
  } kGenTZTests[] = {
      {"20480711220333-0000"},
      {"20140704000000-0100"},
      {"20460311174630-0300"},
      {"20140704000000-2359"},
  };
  for (const auto &t : kGenTZTests) {
    SCOPED_TRACE(t.timestring);
    CBS cbs;
    CBS_init(&cbs, (const uint8_t *)t.timestring, strlen(t.timestring));
    EXPECT_FALSE(CBS_parse_generalized_time(&cbs, NULL,
                                            /*allow_timezone_offset=*/0));
    EXPECT_TRUE(CBS_parse_generalized_time(&cbs, NULL,
                                           /*allow_timezone_offset=*/1));
    EXPECT_FALSE(CBS_parse_utc_time(&cbs, NULL, /*allow_timezone_offset=*/1));
    EXPECT_FALSE(CBS_parse_utc_time(&cbs, NULL, /*allow_timezone_offset=*/0));
  }
  static const struct {
    const char *timestring;
  } kBogusGenTZTests[] = {
      {"20480222202332-2400"},
      {"20140704000000-9999"},
      {"20480726113216-0160"},
  };
  for (const auto &t : kBogusGenTZTests) {
    SCOPED_TRACE(t.timestring);
    CBS cbs;
    CBS_init(&cbs, (const uint8_t *)t.timestring, strlen(t.timestring));
    EXPECT_FALSE(CBS_parse_generalized_time(&cbs, NULL,
                                            /*allow_timezone_offset=*/0));
    EXPECT_FALSE(CBS_parse_utc_time(&cbs, NULL, /*allow_timezone_offset=*/1));
  }
}

TEST(CBSTest, GetU64Decimal) {
  const struct {
    uint64_t val;
    const char *text;
  } kTests[] = {
      {0, "0"},
      {1, "1"},
      {123456, "123456"},
      // 2^64 - 1
      {UINT64_C(18446744073709551615), "18446744073709551615"},
  };
  for (const auto &t : kTests) {
    SCOPED_TRACE(t.text);
    CBS cbs;
    CBS_init(&cbs, reinterpret_cast<const uint8_t*>(t.text), strlen(t.text));
    uint64_t v;
    ASSERT_TRUE(CBS_get_u64_decimal(&cbs, &v));
    EXPECT_EQ(v, t.val);
    EXPECT_EQ(CBS_data(&cbs),
              reinterpret_cast<const uint8_t *>(t.text) + strlen(t.text));
    EXPECT_EQ(CBS_len(&cbs), 0u);

    std::string str(t.text);
    str += "Z";
    CBS_init(&cbs, reinterpret_cast<const uint8_t *>(str.data()), str.size());
    ASSERT_TRUE(CBS_get_u64_decimal(&cbs, &v));
    EXPECT_EQ(v, t.val);
    EXPECT_EQ(CBS_data(&cbs),
              reinterpret_cast<const uint8_t *>(str.data()) + strlen(t.text));
    EXPECT_EQ(CBS_len(&cbs), 1u);
  }

  static const char *kInvalidTests[] = {
      "",
      "nope",
      "-1",
      // 2^64
      "18446744073709551616",
      // Overflows at multiplying by 10.
      "18446744073709551620",
  };
  for (const char *invalid : kInvalidTests) {
    SCOPED_TRACE(invalid);
    CBS cbs;
    CBS_init(&cbs, reinterpret_cast<const uint8_t *>(invalid), strlen(invalid));
    uint64_t v;
    EXPECT_FALSE(CBS_get_u64_decimal(&cbs, &v));
  }
}
