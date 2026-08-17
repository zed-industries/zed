// Copyright (c) 2020, Google Inc.
// SPDX-License-Identifier: ISC

#if !defined(_DEFAULT_SOURCE)
#define _DEFAULT_SOURCE  // Needed for getentropy on musl and glibc
#endif

#include <openssl/rand.h>

#include "internal.h"

#if defined(OPENSSL_RAND_CCRANDOMGENERATEBYTES)

#include <gtest/gtest.h>

#include <openssl/span.h>

#include <CommonCrypto/CommonRandom.h>

#include "../test/test_util.h"

// This test is, strictly speaking, flaky, but we use large enough buffers
// that the probability of failing when we should pass is negligible.

TEST(CCRandomGenerateBytesTest, NotObviouslyBroken) {
  static const uint8_t kZeros[256] = {0};

  uint8_t buf1[256] = {0}, buf2[256] = {0}, buf3[256] = {0};

  EXPECT_EQ(CCRandomGenerateBytes(buf1, sizeof(buf1)), kCCSuccess);
  EXPECT_EQ(CCRandomGenerateBytes(buf2, sizeof(buf2)), kCCSuccess);
  EXPECT_NE(Bytes(buf1), Bytes(buf2));
  EXPECT_NE(Bytes(buf1), Bytes(kZeros));
  EXPECT_NE(Bytes(buf2), Bytes(kZeros));

  // Ensure that the implementation is not simply returning the memory unchanged.
  memcpy(buf3, buf1, sizeof(buf3));
  EXPECT_EQ(CCRandomGenerateBytes(buf1, sizeof(buf1)), kCCSuccess);
  EXPECT_NE(Bytes(buf1), Bytes(buf3));

  // |CCRandomGenerateBytes| supports bigger inputs.
  uint8_t buf4[1024] = {0}, buf5[1024] = {0};
  EXPECT_EQ(CCRandomGenerateBytes(buf4, sizeof(buf4)), kCCSuccess);
  EXPECT_EQ(CCRandomGenerateBytes(buf5, sizeof(buf5)), kCCSuccess);
  EXPECT_NE(Bytes(buf4), Bytes(buf5));
}

#endif // defined(OPENSSL_RAND_CCRANDOMGENERATEBYTES)
