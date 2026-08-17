// Copyright (c) 2015, Google Inc.
// SPDX-License-Identifier: ISC

#include <stdint.h>
#include <string.h>

#include <gtest/gtest.h>

#include <openssl/curve25519.h>

#include "../../internal.h"
#include "../../test/file_test.h"
#include "../../test/test_util.h"


TEST(Ed25519Test, TestVectors) {
  FileTestGTest("crypto/fipsmodule/curve25519/ed25519_tests.txt", [](FileTest *t) {
    std::vector<uint8_t> private_key, public_key, message, expected_signature;
    ASSERT_TRUE(t->GetBytes(&private_key, "PRIV"));
    ASSERT_EQ(64u, private_key.size());
    ASSERT_TRUE(t->GetBytes(&public_key, "PUB"));
    ASSERT_EQ(32u, public_key.size());
    ASSERT_TRUE(t->GetBytes(&message, "MESSAGE"));
    ASSERT_TRUE(t->GetBytes(&expected_signature, "SIG"));
    ASSERT_EQ(64u, expected_signature.size());

    // Signing should not leak the private key or the message.
    CONSTTIME_SECRET(private_key.data(), private_key.size());
    CONSTTIME_SECRET(message.data(), message.size());
    uint8_t signature[64];
    ASSERT_TRUE(ED25519_sign(signature, message.data(), message.size(),
                             private_key.data()));
    CONSTTIME_DECLASSIFY(signature, sizeof(signature));
    CONSTTIME_DECLASSIFY(message.data(), message.size());

    EXPECT_EQ(Bytes(expected_signature), Bytes(signature));
    EXPECT_TRUE(ED25519_verify(message.data(), message.size(), signature,
                               public_key.data()));
  });
}

TEST(Ed25519Test, Malleability) {
  // https://tools.ietf.org/html/rfc8032#section-5.1.7 adds an additional test
  // that s be in [0, order). This prevents someone from adding a multiple of
  // order to s and obtaining a second valid signature for the same message.
  static const uint8_t kMsg[] = {0x54, 0x65, 0x73, 0x74};
  static const uint8_t kSig[] = {
      0x7c, 0x38, 0xe0, 0x26, 0xf2, 0x9e, 0x14, 0xaa, 0xbd, 0x05, 0x9a,
      0x0f, 0x2d, 0xb8, 0xb0, 0xcd, 0x78, 0x30, 0x40, 0x60, 0x9a, 0x8b,
      0xe6, 0x84, 0xdb, 0x12, 0xf8, 0x2a, 0x27, 0x77, 0x4a, 0xb0, 0x67,
      0x65, 0x4b, 0xce, 0x38, 0x32, 0xc2, 0xd7, 0x6f, 0x8f, 0x6f, 0x5d,
      0xaf, 0xc0, 0x8d, 0x93, 0x39, 0xd4, 0xee, 0xf6, 0x76, 0x57, 0x33,
      0x36, 0xa5, 0xc5, 0x1e, 0xb6, 0xf9, 0x46, 0xb3, 0x1d,
  };
  static const uint8_t kPub[] = {
      0x7d, 0x4d, 0x0e, 0x7f, 0x61, 0x53, 0xa6, 0x9b, 0x62, 0x42, 0xb5,
      0x22, 0xab, 0xbe, 0xe6, 0x85, 0xfd, 0xa4, 0x42, 0x0f, 0x88, 0x34,
      0xb1, 0x08, 0xc3, 0xbd, 0xae, 0x36, 0x9e, 0xf5, 0x49, 0xfa,
  };

  EXPECT_FALSE(ED25519_verify(kMsg, sizeof(kMsg), kSig, kPub));

  // The following inputs try to exercise the boundaries of the order check,
  // where s is near the order above and below. EdDSA hashes the public key with
  // the message, which frustrates constructing actual boundary cases. Instead,
  // these inputs were found by randomly generating signatures. kSigValid had
  // the highest s value. kSigInvalid had the lowest s value, and then the order
  // was added.
  //
  // This isn't ideal, but it is sensitive to the most significant 32 bits.
  //
  // The private key seed for kPub2 is
  // a59a4130fcfd293c9737db8f14177ce034305cf34bdc4346f24b4d262e07b5c2.
  static const uint8_t kPub2[] = {
      0x10, 0x0f, 0xdf, 0x47, 0xfb, 0x94, 0xf1, 0x53, 0x6a, 0x4f, 0x7c,
      0x3f, 0xda, 0x27, 0x38, 0x3f, 0xa0, 0x33, 0x75, 0xa8, 0xf5, 0x27,
      0xc5, 0x37, 0xe6, 0xf1, 0x70, 0x3c, 0x47, 0xf9, 0x4f, 0x86};
  static const uint8_t kMsgValid[] = {
      0x12, 0x4e, 0x58, 0x3f, 0x8b, 0x8e, 0xca, 0x58, 0xbb, 0x29, 0xc2,
      0x71, 0xb4, 0x1d, 0x36, 0x98, 0x6b, 0xbc, 0x45, 0x54, 0x1f, 0x8e,
      0x51, 0xf9, 0xcb, 0x01, 0x33, 0xec, 0xa4, 0x47, 0x60, 0x1e};
  static const uint8_t kSigValid[] = {
      0xda, 0xc1, 0x19, 0xd6, 0xca, 0x87, 0xfc, 0x59, 0xae, 0x61, 0x1c,
      0x15, 0x70, 0x48, 0xf4, 0xd4, 0xfc, 0x93, 0x2a, 0x14, 0x9d, 0xbe,
      0x20, 0xec, 0x6e, 0xff, 0xd1, 0x43, 0x6a, 0xbf, 0x83, 0xea, 0x05,
      0xc7, 0xdf, 0x0f, 0xef, 0x06, 0x14, 0x72, 0x41, 0x25, 0x91, 0x13,
      0x90, 0x9b, 0xc7, 0x1b, 0xd3, 0xc5, 0x3b, 0xa4, 0x46, 0x4f, 0xfc,
      0xad, 0x3c, 0x09, 0x68, 0xf2, 0xff, 0xff, 0xff, 0x0f};
  static const uint8_t kMsgInvalid[] = {
      0x6a, 0x0b, 0xc2, 0xb0, 0x05, 0x7c, 0xed, 0xfc, 0x0f, 0xa2, 0xe3,
      0xf7, 0xf7, 0xd3, 0x92, 0x79, 0xb3, 0x0f, 0x45, 0x4a, 0x69, 0xdf,
      0xd1, 0x11, 0x7c, 0x75, 0x8d, 0x86, 0xb1, 0x9d, 0x85, 0xe0};
  static const uint8_t kSigInvalid[] = {
      0x09, 0x71, 0xf8, 0x6d, 0x2c, 0x9c, 0x78, 0x58, 0x25, 0x24, 0xa1,
      0x03, 0xcb, 0x9c, 0xf9, 0x49, 0x52, 0x2a, 0xe5, 0x28, 0xf8, 0x05,
      0x4d, 0xc2, 0x01, 0x07, 0xd9, 0x99, 0xbe, 0x67, 0x3f, 0xf4, 0xe2,
      0x5e, 0xbf, 0x2f, 0x29, 0x28, 0x76, 0x6b, 0x12, 0x48, 0xbe, 0xc6,
      0xe9, 0x16, 0x97, 0x77, 0x5f, 0x84, 0x46, 0x63, 0x9e, 0xde, 0x46,
      0xad, 0x4d, 0xf4, 0x05, 0x30, 0x00, 0x00, 0x00, 0x10};

  EXPECT_TRUE(ED25519_verify(kMsgValid, sizeof(kMsgValid), kSigValid, kPub2));
  EXPECT_FALSE(
      ED25519_verify(kMsgInvalid, sizeof(kMsgInvalid), kSigInvalid, kPub2));
}

TEST(Ed25519Test, KeypairFromSeed) {
  uint8_t public_key1[32], private_key1[64];
  ED25519_keypair(public_key1, private_key1);

  uint8_t seed[32];
  OPENSSL_memcpy(seed, private_key1, sizeof(seed));
  CONSTTIME_SECRET(seed, sizeof(seed));

  uint8_t public_key2[32], private_key2[64];
  ED25519_keypair_from_seed(public_key2, private_key2, seed);
  CONSTTIME_DECLASSIFY(public_key2, sizeof(public_key2));
  CONSTTIME_DECLASSIFY(private_key2, sizeof(private_key2));

  EXPECT_EQ(Bytes(public_key1), Bytes(public_key2));
  EXPECT_EQ(Bytes(private_key1), Bytes(private_key2));
}

TEST(Ed25519phTest, TestVectors) {
  FileTestGTest("crypto/fipsmodule/curve25519/ed25519ph_tests.txt", [](FileTest *t) {
    std::vector<uint8_t> seed, q, message, context, expected_signature;
    ASSERT_TRUE(t->GetBytes(&seed, "SEED"));
    ASSERT_EQ(32u, seed.size());
    ASSERT_TRUE(t->GetBytes(&q, "Q"));
    ASSERT_EQ(32u, q.size());
    ASSERT_TRUE(t->GetBytes(&message, "MESSAGE"));
    ASSERT_TRUE(t->GetBytes(&expected_signature, "SIGNATURE"));
    ASSERT_EQ(64u, expected_signature.size());

    if (t->HasAttribute("CONTEXT")) {
        t->GetBytes(&context, "CONTEXT");
    } else {
        context = std::vector<uint8_t>();
    }

    uint8_t private_key[ED25519_PRIVATE_KEY_LEN] = {0};
    uint8_t public_key[ED25519_PUBLIC_KEY_LEN] = {0};

    ED25519_keypair_from_seed(public_key, private_key, seed.data());
    ASSERT_EQ(Bytes(q), Bytes(public_key));

    // Signing should not leak the private key or the message.
    CONSTTIME_SECRET(&private_key[0], sizeof(private_key));
    CONSTTIME_SECRET(message.data(), message.size());
    CONSTTIME_SECRET(context.data(), context.size());
    uint8_t signature[64];
    ASSERT_TRUE(ED25519ph_sign(signature, message.data(), message.size(),
                             private_key, context.data(), context.size()));
    CONSTTIME_DECLASSIFY(signature, sizeof(signature));
    CONSTTIME_DECLASSIFY(message.data(), message.size());
    CONSTTIME_DECLASSIFY(context.data(), context.size());

    EXPECT_EQ(Bytes(expected_signature), Bytes(signature));
    EXPECT_TRUE(ED25519ph_verify(message.data(), message.size(), signature,
                               public_key, context.data(), context.size()));
  });
}

TEST(Ed25519ctxTest, TestVectors) {
  FileTestGTest("crypto/fipsmodule/curve25519/ed25519ctx_tests.txt", [](FileTest *t) {
    std::vector<uint8_t> seed, q, message, context, expected_signature;
    ASSERT_TRUE(t->GetBytes(&seed, "SEED"));
    ASSERT_EQ(32u, seed.size());
    ASSERT_TRUE(t->GetBytes(&q, "Q"));
    ASSERT_EQ(32u, q.size());
    ASSERT_TRUE(t->GetBytes(&message, "MESSAGE"));
    ASSERT_TRUE(t->GetBytes(&expected_signature, "SIGNATURE"));
    ASSERT_EQ(64u, expected_signature.size());

    if (t->HasAttribute("CONTEXT")) {
        t->GetBytes(&context, "CONTEXT");
    } else {
        context = std::vector<uint8_t>();
    }

    uint8_t private_key[ED25519_PRIVATE_KEY_LEN] = {0};
    uint8_t public_key[ED25519_PUBLIC_KEY_LEN] = {0};

    ED25519_keypair_from_seed(public_key, private_key, seed.data());
    ASSERT_EQ(Bytes(q), Bytes(public_key));

    // Signing should not leak the private key or the message.
    CONSTTIME_SECRET(&private_key[0], sizeof(private_key));
    CONSTTIME_SECRET(message.data(), message.size());
    CONSTTIME_SECRET(context.data(), context.size());
    uint8_t signature[64];
    ASSERT_TRUE(ED25519ctx_sign(signature, message.data(), message.size(),
                                private_key, context.data(), context.size()));
    CONSTTIME_DECLASSIFY(signature, sizeof(signature));
    CONSTTIME_DECLASSIFY(message.data(), message.size());
    CONSTTIME_DECLASSIFY(context.data(), context.size());

    EXPECT_EQ(Bytes(expected_signature), Bytes(signature));
    EXPECT_TRUE(ED25519ctx_verify(message.data(), message.size(), signature,
                                  public_key, context.data(), context.size()));
  });
}

TEST(Ed25519ctxTest, EmptyContext) {
  uint8_t private_key[ED25519_PRIVATE_KEY_LEN] = {0};
  uint8_t public_key[ED25519_PUBLIC_KEY_LEN] = {0};
  uint8_t signature[ED25519_SIGNATURE_LEN] = {0};
  const uint8_t message[3] = {'f', 'o', 'o'};

  ED25519_keypair(public_key, private_key);

  EXPECT_FALSE(ED25519ctx_sign(signature, message, sizeof(message), private_key, NULL, 0));
  EXPECT_FALSE(ED25519ctx_verify(message, sizeof(message), signature, public_key, NULL, 0));
}

