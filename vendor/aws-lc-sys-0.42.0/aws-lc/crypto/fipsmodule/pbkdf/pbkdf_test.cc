// Copyright (c) 2015, Google Inc.
// SPDX-License-Identifier: ISC

#include <gtest/gtest.h>

#include <openssl/crypto.h>
#include <openssl/digest.h>
#include <openssl/evp.h>

#include "../../internal.h"
#include "../../test/test_util.h"


// Tests deriving a key using an empty password (specified both as NULL and as
// non-NULL). Note that NULL has special meaning to HMAC initialization.
TEST(PBKDFTest, EmptyPassword) {
  const uint8_t kKey[] = {0xa3, 0x3d, 0xdd, 0xc3, 0x04, 0x78, 0x18,
                          0x55, 0x15, 0x31, 0x1f, 0x87, 0x52, 0x89,
                          0x5d, 0x36, 0xea, 0x43, 0x63, 0xa2};
  uint8_t key[sizeof(kKey)];

  ASSERT_TRUE(PKCS5_PBKDF2_HMAC(NULL, 0, (const uint8_t *)"salt", 4, 1,
                                EVP_sha1(), sizeof(kKey), key));
  EXPECT_EQ(Bytes(kKey), Bytes(key));

  ASSERT_TRUE(PKCS5_PBKDF2_HMAC("", 0, (const uint8_t *)"salt", 4, 1,
                                EVP_sha1(), sizeof(kKey), key));
  EXPECT_EQ(Bytes(kKey), Bytes(key));
}

// Tests deriving a key using an empty salt. Note that the expectation was
// generated using OpenSSL itself, and hence is not verified.
TEST(PBKDFTest, EmptySalt) {
  const uint8_t kKey[] = {0x8b, 0xc2, 0xf9, 0x16, 0x7a, 0x81, 0xcd, 0xcf,
                          0xad, 0x12, 0x35, 0xcd, 0x90, 0x47, 0xf1, 0x13,
                          0x62, 0x71, 0xc1, 0xf9, 0x78, 0xfc, 0xfc, 0xb3,
                          0x5e, 0x22, 0xdb, 0xea, 0xfa, 0x46, 0x34, 0xf6};
  uint8_t key[sizeof(kKey)];

  ASSERT_TRUE(PKCS5_PBKDF2_HMAC("password", 8, NULL, 0, 2, EVP_sha256(),
                                sizeof(kKey), key));
  EXPECT_EQ(Bytes(kKey), Bytes(key));

  ASSERT_TRUE(PKCS5_PBKDF2_HMAC("password", 8, (const uint8_t *)"", 0, 2,
                                EVP_sha256(), sizeof(kKey), key));
  EXPECT_EQ(Bytes(kKey), Bytes(key));
}

// Exercises test vectors taken from https://tools.ietf.org/html/rfc6070.
// Note that each of these test vectors uses SHA-1 as the digest.
TEST(PBKDFTest, RFC6070Vectors) {
  const uint8_t kKey1[] = {0x0c, 0x60, 0xc8, 0x0f, 0x96, 0x1f, 0x0e,
                           0x71, 0xf3, 0xa9, 0xb5, 0x24, 0xaf, 0x60,
                           0x12, 0x06, 0x2f, 0xe0, 0x37, 0xa6};
  const uint8_t kKey2[] = {0xea, 0x6c, 0x01, 0x4d, 0xc7, 0x2d, 0x6f,
                           0x8c, 0xcd, 0x1e, 0xd9, 0x2a, 0xce, 0x1d,
                           0x41, 0xf0, 0xd8, 0xde, 0x89, 0x57};
  const uint8_t kKey3[] = {0x56, 0xfa, 0x6a, 0xa7, 0x55, 0x48, 0x09, 0x9d,
                           0xcc, 0x37, 0xd7, 0xf0, 0x34, 0x25, 0xe0, 0xc3};
  const uint8_t kKey4[] = {0x3d, 0x2e, 0xec, 0x4f, 0xe4, 0x1c, 0x84, 0x9b,
                           0x80, 0xc8, 0xd8, 0x36, 0x62, 0xc0, 0xe4, 0x4a,
                           0x8b, 0x29, 0x1a, 0x96, 0x4c, 0xf2, 0xf0, 0x70,
                           0x38};
  uint8_t key[sizeof(kKey4)];
  static_assert(sizeof(key) >= sizeof(kKey1), "output too small");
  static_assert(sizeof(key) >= sizeof(kKey2), "output too small");
  static_assert(sizeof(key) >= sizeof(kKey3), "output too small");

  ASSERT_TRUE(PKCS5_PBKDF2_HMAC("password", 8, (const uint8_t *)"salt", 4, 1,
                                EVP_sha1(), sizeof(kKey1), key));
  EXPECT_EQ(Bytes(kKey1), Bytes(key, sizeof(kKey1)));

  ASSERT_TRUE(PKCS5_PBKDF2_HMAC("password", 8, (const uint8_t *)"salt", 4, 2,
                                EVP_sha1(), sizeof(kKey2), key));
  EXPECT_EQ(Bytes(kKey2), Bytes(key, sizeof(kKey2)));

  ASSERT_TRUE(PKCS5_PBKDF2_HMAC("pass\0word", 9, (const uint8_t *)"sa\0lt", 5,
                                4096, EVP_sha1(), sizeof(kKey3), key));
  EXPECT_EQ(Bytes(kKey3), Bytes(key, sizeof(kKey3)));

  ASSERT_TRUE(PKCS5_PBKDF2_HMAC("passwordPASSWORDpassword", 24,
                    (const uint8_t *)"saltSALTsaltSALTsaltSALTsaltSALTsalt", 36,
                    4096, EVP_sha1(), sizeof(kKey4), key));
  EXPECT_EQ(Bytes(kKey4), Bytes(key, sizeof(kKey4)));
}

// Tests key derivation using SHA-2 digests.
TEST(PBKDFTest, SHA2) {
  // This test was taken from:
  // http://stackoverflow.com/questions/5130513/pbkdf2-hmac-sha2-test-vectors.
  const uint8_t kKey1[] = {0xae, 0x4d, 0x0c, 0x95, 0xaf, 0x6b, 0x46, 0xd3,
                           0x2d, 0x0a, 0xdf, 0xf9, 0x28, 0xf0, 0x6d, 0xd0,
                           0x2a, 0x30, 0x3f, 0x8e, 0xf3, 0xc2, 0x51, 0xdf,
                           0xd6, 0xe2, 0xd8, 0x5a, 0x95, 0x47, 0x4c, 0x43};

  // This test was taken from:
  // http://stackoverflow.com/questions/15593184/pbkdf2-hmac-sha-512-test-vectors.
  const uint8_t kKey2[] = {
      0x8c, 0x05, 0x11, 0xf4, 0xc6, 0xe5, 0x97, 0xc6, 0xac, 0x63, 0x15,
      0xd8, 0xf0, 0x36, 0x2e, 0x22, 0x5f, 0x3c, 0x50, 0x14, 0x95, 0xba,
      0x23, 0xb8, 0x68, 0xc0, 0x05, 0x17, 0x4d, 0xc4, 0xee, 0x71, 0x11,
      0x5b, 0x59, 0xf9, 0xe6, 0x0c, 0xd9, 0x53, 0x2f, 0xa3, 0x3e, 0x0f,
      0x75, 0xae, 0xfe, 0x30, 0x22, 0x5c, 0x58, 0x3a, 0x18, 0x6c, 0xd8,
      0x2b, 0xd4, 0xda, 0xea, 0x97, 0x24, 0xa3, 0xd3, 0xb8};

  uint8_t key[sizeof(kKey2)];
  static_assert(sizeof(key) >= sizeof(kKey1), "output too small");

  ASSERT_TRUE(PKCS5_PBKDF2_HMAC("password", 8, (const uint8_t *)"salt", 4, 2,
                                EVP_sha256(), sizeof(kKey1), key));
  EXPECT_EQ(Bytes(kKey1), Bytes(key, sizeof(kKey1)));

  ASSERT_TRUE(
      PKCS5_PBKDF2_HMAC("passwordPASSWORDpassword", 24,
                        (const uint8_t *)"saltSALTsaltSALTsaltSALTsaltSALTsalt",
                        36, 4096, EVP_sha512(), sizeof(kKey2), key));
  EXPECT_EQ(Bytes(kKey2), Bytes(key, sizeof(kKey2)));
}

// Tests key derivation using iterations=0.
//
// RFC 2898 defines the iteration count (c) as a "positive integer". So doing a
// key derivation with iterations=0 is ill-defined and should result in a
// failure.
TEST(PBKDFTest, ZeroIterations) {
  static const char kPassword[] = "password";
  const size_t password_len = strlen(kPassword);
  static const uint8_t kSalt[] = {1, 2, 3, 4};
  const size_t salt_len = sizeof(kSalt);
  const EVP_MD *digest = EVP_sha1();

  uint8_t key[10] = {0};
  const size_t key_len = sizeof(key);

  // Verify that calling with iterations=1 works.
  ASSERT_TRUE(PKCS5_PBKDF2_HMAC(kPassword, password_len, kSalt, salt_len,
                                1 /* iterations */, digest, key_len, key));

  // Flip the first key byte (so can later test if it got set).
  const uint8_t expected_first_byte = key[0];
  key[0] = ~key[0];

  // However calling it with iterations=0 fails.
  ASSERT_FALSE(PKCS5_PBKDF2_HMAC(kPassword, password_len, kSalt, salt_len,
                                 0 /* iterations */, digest, key_len, key));

  // For backwards compatibility, the iterations == 0 case still fills in
  // the out key.
  EXPECT_EQ(expected_first_byte, key[0]);
}
