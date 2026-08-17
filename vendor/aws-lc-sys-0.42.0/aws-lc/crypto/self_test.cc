// Copyright (c) 2018, Google Inc.
// SPDX-License-Identifier: ISC

#include <openssl/crypto.h>

#include <gtest/gtest.h>


TEST(SelfTests, KAT) {
#if !defined(_MSC_VER)
  EXPECT_TRUE(BORINGSSL_self_test());
#endif
}
