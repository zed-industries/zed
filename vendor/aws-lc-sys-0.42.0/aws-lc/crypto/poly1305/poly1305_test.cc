// Copyright (c) 2015, Google Inc.
// SPDX-License-Identifier: ISC

#include <stdio.h>
#include <string.h>

#include <vector>

#include <gtest/gtest.h>

#include <openssl/poly1305.h>

#include "../internal.h"
#include "../test/file_test.h"
#include "../test/test_util.h"


static void TestSIMD(unsigned excess, const std::vector<uint8_t> &key,
                     const std::vector<uint8_t> &in,
                     const std::vector<uint8_t> &mac) {
  poly1305_state state;
  CRYPTO_poly1305_init(&state, key.data());

  size_t done = 0;

  // Feed 16 bytes in. Some implementations begin in non-SIMD mode and upgrade
  // on-demand. Stress the upgrade path.
  size_t todo = 16;
  if (todo > in.size()) {
    todo = in.size();
  }
  CRYPTO_poly1305_update(&state, in.data(), todo);
  done += todo;

  for (;;) {
    // Feed 128 + |excess| bytes to test SIMD mode.
    if (done + 128 + excess > in.size()) {
      break;
    }
    CRYPTO_poly1305_update(&state, in.data() + done, 128 + excess);
    done += 128 + excess;

    // Feed |excess| bytes to ensure SIMD mode can handle short inputs.
    if (done + excess > in.size()) {
      break;
    }
    CRYPTO_poly1305_update(&state, in.data() + done, excess);
    done += excess;
  }

  // Consume the remainder and finish.
  CRYPTO_poly1305_update(&state, in.data() + done, in.size() - done);

  uint8_t out[16];
  CRYPTO_poly1305_finish(&state, out);
  EXPECT_EQ(Bytes(out), Bytes(mac)) << "SIMD pattern " << excess << " failed.";
}

TEST(Poly1305Test, TestVectors) {
  FileTestGTest("crypto/poly1305/poly1305_tests.txt", [](FileTest *t) {
    std::vector<uint8_t> key, in, mac;
    ASSERT_TRUE(t->GetBytes(&key, "Key"));
    ASSERT_TRUE(t->GetBytes(&in, "Input"));
    ASSERT_TRUE(t->GetBytes(&mac, "MAC"));
    ASSERT_EQ(32u, key.size());
    ASSERT_EQ(16u, mac.size());

    // Test single-shot operation.
    poly1305_state state;
    CRYPTO_poly1305_init(&state, key.data());
    CRYPTO_poly1305_update(&state, in.data(), in.size());
    uint8_t out[16];
    CRYPTO_poly1305_finish(&state, out);
    EXPECT_EQ(Bytes(out), Bytes(mac)) << "Single-shot Poly1305 failed.";

    // Test streaming byte-by-byte.
    CRYPTO_poly1305_init(&state, key.data());
    for (size_t i = 0; i < in.size(); i++) {
      CRYPTO_poly1305_update(&state, &in[i], 1);
    }
    CRYPTO_poly1305_finish(&state, out);
    EXPECT_EQ(Bytes(out), Bytes(mac)) << "Streaming Poly1305 failed.";

    // Test |CRYPTO_poly1305_init| and |CRYPTO_poly1305_finish| work on
    // unaligned values.
    alignas(8) uint8_t unaligned_key[32 + 1];
    OPENSSL_memcpy(unaligned_key + 1, key.data(), 32);
    CRYPTO_poly1305_init(&state, unaligned_key + 1);
    CRYPTO_poly1305_update(&state, in.data(), in.size());
    alignas(8) uint8_t unaligned_out[16 + 1];
    CRYPTO_poly1305_finish(&state, unaligned_out + 1);
    EXPECT_EQ(Bytes(unaligned_out + 1, 16), Bytes(mac))
        << "Unaligned Poly1305 failed.";

    // Test SIMD stress patterns. OpenSSL's AVX2 assembly needs a multiple of
    // four blocks, so test up to three blocks of excess.
    TestSIMD(0, key, in, mac);
    TestSIMD(16, key, in, mac);
    TestSIMD(32, key, in, mac);
    TestSIMD(48, key, in, mac);
  });
}
