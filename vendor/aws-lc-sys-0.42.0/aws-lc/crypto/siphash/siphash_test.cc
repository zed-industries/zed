// Copyright (c) 2019, Google Inc.
// SPDX-License-Identifier: ISC

#include <stdint.h>

#include <gtest/gtest.h>

#include <openssl/siphash.h>

#include "../test/file_test.h"
#include "../test/test_util.h"

TEST(SipHash, Basic) {
  // This is the example from appendix A of the SipHash paper.
  uint8_t key_bytes[16];
  for (unsigned i = 0; i < 16; i++) {
    key_bytes[i] = i;
  }
  uint64_t key[2];
  memcpy(key, key_bytes, sizeof(key));

  uint8_t input[15];
  for (unsigned i = 0; i < sizeof(input); i++) {
    input[i] = i;
  }

  EXPECT_EQ(UINT64_C(0xa129ca6149be45e5),
            SIPHASH_24(key, input, sizeof(input)));
}

TEST(SipHash, Vectors) {
  FileTestGTest("crypto/siphash/siphash_tests.txt", [](FileTest *t) {
    std::vector<uint8_t> key, msg, hash_bytes;
    ASSERT_TRUE(t->GetBytes(&key, "KEY"));
    ASSERT_TRUE(t->GetBytes(&msg, "IN"));
    ASSERT_TRUE(t->GetBytes(&hash_bytes, "HASH"));
    ASSERT_EQ(16u, key.size());
    ASSERT_EQ(8u, hash_bytes.size());
    uint64_t hash = CRYPTO_load_u64_le(hash_bytes.data());

    uint64_t key_words[2];
    memcpy(key_words, key.data(), key.size());
    uint64_t result = SIPHASH_24(key_words, msg.data(), msg.size());
    EXPECT_EQ(result, hash);
  });
}
