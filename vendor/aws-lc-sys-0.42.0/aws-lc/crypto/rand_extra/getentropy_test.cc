// Copyright (c) 2023, Google Inc.
// SPDX-License-Identifier: ISC

#if !defined(_DEFAULT_SOURCE)
#define _DEFAULT_SOURCE  // Needed for getentropy on musl and glibc
#endif

#include <openssl/rand.h>

#include "internal.h"

#if defined(OPENSSL_RAND_GETENTROPY)

#include <unistd.h>

#include <errno.h>

#if defined(OPENSSL_MACOS)
#include <sys/random.h>
#endif

#include <gtest/gtest.h>

#include <openssl/span.h>

#include "../test/test_util.h"

// This test is, strictly speaking, flaky, but we use large enough buffers
// that the probability of failing when we should pass is negligible.

TEST(GetEntropyTest, NotObviouslyBroken) {
  static const uint8_t kZeros[256] = {0};

  uint8_t buf1[256], buf2[256];

  EXPECT_EQ(getentropy(buf1, sizeof(buf1)), 0);
  EXPECT_EQ(getentropy(buf2, sizeof(buf2)), 0);
  EXPECT_NE(Bytes(buf1), Bytes(buf2));
  EXPECT_NE(Bytes(buf1), Bytes(kZeros));
  EXPECT_NE(Bytes(buf2), Bytes(kZeros));
  uint8_t buf3[256];
  // Ensure that the implementation is not simply returning the memory unchanged.
  memcpy(buf3, buf1, sizeof(buf3));
  EXPECT_EQ(getentropy(buf1, sizeof(buf1)), 0);
  EXPECT_NE(Bytes(buf1), Bytes(buf3));
  errno = 0;
  uint8_t toobig[257];
  // getentropy should fail returning -1 and setting errno to EIO if you request
  // more than 256 bytes of entropy. macOS's man page says EIO but it actually
  // returns EINVAL, so we accept either.
  EXPECT_EQ(getentropy(toobig, 257), -1);
  EXPECT_TRUE(errno == EIO || errno == EINVAL);
}
#endif
