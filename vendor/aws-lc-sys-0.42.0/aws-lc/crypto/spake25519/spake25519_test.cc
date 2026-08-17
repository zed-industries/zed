// Copyright (c) 2016, Google Inc.
// SPDX-License-Identifier: ISC

#include <openssl/curve25519.h>

#include <string>

#include <stdint.h>
#include <stdio.h>
#include <string.h>

#include <gtest/gtest.h>

#include "../internal.h"
#include "./internal.h"


// TODO(agl): add tests with fixed vectors once SPAKE2 is nailed down.

struct SPAKE2Run {
  bool Run() {
    bssl::UniquePtr<SPAKE2_CTX> alice(SPAKE2_CTX_new(
        spake2_role_alice,
        reinterpret_cast<const uint8_t *>(alice_names.first.data()),
        alice_names.first.size(),
        reinterpret_cast<const uint8_t *>(alice_names.second.data()),
        alice_names.second.size()));
    bssl::UniquePtr<SPAKE2_CTX> bob(SPAKE2_CTX_new(
        spake2_role_bob,
        reinterpret_cast<const uint8_t *>(bob_names.first.data()),
        bob_names.first.size(),
        reinterpret_cast<const uint8_t *>(bob_names.second.data()),
        bob_names.second.size()));

    if (!alice || !bob) {
      return false;
    }

    if (alice_disable_password_scalar_hack) {
      alice->disable_password_scalar_hack = 1;
    }
    if (bob_disable_password_scalar_hack) {
      bob->disable_password_scalar_hack = 1;
    }

    uint8_t alice_msg[SPAKE2_MAX_MSG_SIZE];
    uint8_t bob_msg[SPAKE2_MAX_MSG_SIZE];
    size_t alice_msg_len, bob_msg_len;

    if (!SPAKE2_generate_msg(
            alice.get(), alice_msg, &alice_msg_len, sizeof(alice_msg),
            reinterpret_cast<const uint8_t *>(alice_password.data()),
            alice_password.size()) ||
        !SPAKE2_generate_msg(
            bob.get(), bob_msg, &bob_msg_len, sizeof(bob_msg),
            reinterpret_cast<const uint8_t *>(bob_password.data()),
            bob_password.size())) {
      return false;
    }

    if (alice_corrupt_msg_bit >= 0 &&
        static_cast<size_t>(alice_corrupt_msg_bit) < 8 * alice_msg_len) {
      alice_msg[alice_corrupt_msg_bit/8] ^= 1 << (alice_corrupt_msg_bit & 7);
    }

    uint8_t alice_key[64], bob_key[64];
    size_t alice_key_len, bob_key_len;

    if (!SPAKE2_process_msg(alice.get(), alice_key, &alice_key_len,
                            sizeof(alice_key), bob_msg, bob_msg_len) ||
        !SPAKE2_process_msg(bob.get(), bob_key, &bob_key_len, sizeof(bob_key),
                            alice_msg, alice_msg_len)) {
      return false;
    }

    key_matches_ = (alice_key_len == bob_key_len &&
                    OPENSSL_memcmp(alice_key, bob_key, alice_key_len) == 0);

    return true;
  }

  bool key_matches() const {
    return key_matches_;
  }

  std::string alice_password = "password";
  std::string bob_password = "password";
  std::pair<std::string, std::string> alice_names = {"alice", "bob"};
  std::pair<std::string, std::string> bob_names = {"bob", "alice"};
  bool alice_disable_password_scalar_hack = false;
  bool bob_disable_password_scalar_hack = false;
  int alice_corrupt_msg_bit = -1;

 private:
  bool key_matches_ = false;
};

TEST(SPAKE25519Test, SPAKE2) {
  for (unsigned i = 0; i < 20; i++) {
    SPAKE2Run spake2;
    ASSERT_TRUE(spake2.Run());
    EXPECT_TRUE(spake2.key_matches());
  }
}

TEST(SPAKE25519Test, OldAlice) {
  for (unsigned i = 0; i < 20; i++) {
    SPAKE2Run spake2;
    spake2.alice_disable_password_scalar_hack = true;
    ASSERT_TRUE(spake2.Run());
    EXPECT_TRUE(spake2.key_matches());
  }
}

TEST(SPAKE25519Test, OldBob) {
  for (unsigned i = 0; i < 20; i++) {
    SPAKE2Run spake2;
    spake2.bob_disable_password_scalar_hack = true;
    ASSERT_TRUE(spake2.Run());
    EXPECT_TRUE(spake2.key_matches());
  }
}

TEST(SPAKE25519Test, WrongPassword) {
  SPAKE2Run spake2;
  spake2.bob_password = "wrong password";
  ASSERT_TRUE(spake2.Run());
  EXPECT_FALSE(spake2.key_matches()) << "Key matched for unequal passwords.";
}

TEST(SPAKE25519Test, WrongNames) {
  SPAKE2Run spake2;
  spake2.alice_names.second = "charlie";
  spake2.bob_names.second = "charlie";
  ASSERT_TRUE(spake2.Run());
  EXPECT_FALSE(spake2.key_matches()) << "Key matched for unequal names.";
}

TEST(SPAKE25519Test, CorruptMessages) {
  for (int i = 0; i < 8 * SPAKE2_MAX_MSG_SIZE; i++) {
    SPAKE2Run spake2;
    spake2.alice_corrupt_msg_bit = i;
    EXPECT_FALSE(spake2.Run() && spake2.key_matches())
        << "Passed after corrupting Alice's message, bit " << i;
  }
}
