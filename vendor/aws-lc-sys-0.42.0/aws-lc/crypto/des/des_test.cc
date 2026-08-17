// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC

#include <openssl/des.h>

#include <gtest/gtest.h>

TEST(DESTest, WeakKeys) {
  // The all 2 key is not weak and has odd parity
  DES_cblock validKey =  {{2, 2, 2, 2, 2, 2, 2, 2}};
  EXPECT_FALSE(DES_is_weak_key(&validKey));
  DES_key_schedule des;
  EXPECT_EQ(0, DES_set_key(&validKey, &des));

  // Weak key example from SP 800-67r2 section 3.3.2
  static const DES_cblock weakKey = {{0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01}};
  EXPECT_TRUE(DES_is_weak_key(&weakKey));
  EXPECT_EQ(-2, DES_set_key(&weakKey, &des));
}

// If it wasn't for MSVC this could be __builtin_popcount, if this was C++20
// it could be std::popcount
static int countSetBits(uint8_t n) {
  int count = 0;
  while (n) {
    count += n & 1;
    n >>= 1;
  }
  return count;
}

TEST(DESTest, Parity) {
  // The all 2 key is not weak and has odd parity for each byte
  DES_cblock key = {{2, 2, 2, 2, 2, 2, 2, 2}};
  DES_key_schedule des;
  int result = DES_set_key(&key, &des);
  EXPECT_EQ(result, 0);

  for (uint8_t i = 0; i < 255; i++) {
    key.bytes[0] = i;
    result = DES_set_key(&key, &des);
    int bitsSet = countSetBits(i);
    int oddParity = bitsSet % 2 == 1;
    if (oddParity) {
      EXPECT_EQ(result, 0);
    } else {
      EXPECT_EQ(result, -1);
    }
  }
}
