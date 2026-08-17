// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC

#include <openssl/bn.h>
#include <openssl/rand.h>
#include "./internal.h"

#include <gtest/gtest.h>

TEST(BNAssertTest, Assert_fits_in_bytes_large) {
// TODO: Update Android test harness
#if !defined(NDEBUG) && !defined(OPENSSL_ANDROID)
  bssl::UniquePtr<BIGNUM> x(BN_new());
  uint8_t input[255];
  OPENSSL_memset(input, 0, sizeof(input));
  input[0] = 0xaa;
  input[1] = 0x01;
  input[254] = 0x01;
  ASSERT_TRUE(BN_le2bn(input, sizeof(input), x.get()));
  for (size_t i = 255; i < 260; i++) {
    bn_assert_fits_in_bytes(x.get(), i);
  }
  for (size_t i = 247; i < 255; i++) {
    EXPECT_DEATH_IF_SUPPORTED(bn_assert_fits_in_bytes(x.get(), i), "");
  }
#endif
}

TEST(BNAssertTest, Assert_fits_in_bytes_small) {
#if !defined(NDEBUG) && !defined(OPENSSL_ANDROID)
  bssl::UniquePtr<BIGNUM> x(BN_new());
  uint8_t input[8];
  OPENSSL_memset(input, 0, sizeof(input));
  input[0] = 0xaa;
  input[1] = 0xbb;
  input[2] = 0xcc;
  ASSERT_TRUE(BN_le2bn(input, sizeof(input), x.get()));
  for (size_t i = 3; i < 10; i++) {
    bn_assert_fits_in_bytes(x.get(), i);
  }
  for (size_t i = 0; i < 3; i++) {
    EXPECT_DEATH_IF_SUPPORTED(bn_assert_fits_in_bytes(x.get(), i), "");
  }
#endif
}

TEST(BNAssertTest, Assert_fits_in_bytes_zero) {
#if !defined(NDEBUG) && !defined(OPENSSL_ANDROID)
  bssl::UniquePtr<BIGNUM> x(BN_new());
  uint8_t input[8];
  OPENSSL_memset(input, 0, sizeof(input));
  ASSERT_TRUE(BN_le2bn(input, sizeof(input), x.get()));

  for (size_t i = 0; i < 10; i++) {
    bn_assert_fits_in_bytes(x.get(), i);
  }
#endif
}

TEST(BNAssertTest, Assert_fits_in_bytes_boundary) {
#if !defined(NDEBUG) && !defined(OPENSSL_ANDROID)
  bssl::UniquePtr<BIGNUM> x(BN_new());
  uint8_t input[8];
  OPENSSL_memset(input, 0, sizeof(input));
  for (size_t i = 0; i < sizeof(input); i++) {
    input[i] = i * (i + 1) & 0xff;
  }
  ASSERT_TRUE(BN_le2bn(input, sizeof(input), x.get()));
  for (size_t i = 8; i < 18; i++) {
    bn_assert_fits_in_bytes(x.get(), i);
  }
  for (size_t i = 0; i < 8; i++) {
    EXPECT_DEATH_IF_SUPPORTED(bn_assert_fits_in_bytes(x.get(), i), "");
  }
#endif
}
