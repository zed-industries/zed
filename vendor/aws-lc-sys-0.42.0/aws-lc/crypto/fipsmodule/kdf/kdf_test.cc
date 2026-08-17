// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC

#include <openssl/digest.h>
#include <openssl/kdf.h>
#include <vector>
#include "../../test/file_test.h"
#include "../../test/test_util.h"

#include <gtest/gtest.h>

TEST(SSKDFTest, TestVectors) {
  FileTestGTest("crypto/fipsmodule/kdf/test/sskdf.txt", [](FileTest *t) {
    const EVP_MD *md;
    std::string hash;
    ASSERT_TRUE(t->GetAttribute(&hash, "HASH"));
    if (hash == "SHA1") {
      md = EVP_sha1();
    } else if (hash == "SHA-224") {
      md = EVP_sha224();
    } else if (hash == "SHA-256") {
      md = EVP_sha256();
    } else if (hash == "SHA-384") {
      md = EVP_sha384();
    } else if (hash == "SHA-512") {
      md = EVP_sha512();
    } else {
      FAIL() << "Unknown HASH=" + hash;
    }

    std::vector<uint8_t> secret, info, expect;

    ASSERT_TRUE(t->GetBytes(&secret, "SECRET"));
    if (t->HasAttribute("INFO")) {
      ASSERT_TRUE(t->GetBytes(&info, "INFO"));
    } else {
      info = std::vector<uint8_t>(0);
    }
    ASSERT_TRUE(t->GetBytes(&expect, "EXPECT"));

    std::vector<uint8_t> out(expect.size());

    std::string variant;
    ASSERT_TRUE(t->GetAttribute(&variant, "VARIANT"));
    if (variant == "DIGEST") {
      ASSERT_TRUE(SSKDF_digest(out.data(), out.size(), md, secret.data(),
                               secret.size(), info.data(), info.size()));
    } else if (variant == "HMAC") {
      if (t->HasAttribute("SALT")) {
        std::vector<uint8_t> salt;
        ASSERT_TRUE(t->GetBytes(&salt, "SALT"));
        ASSERT_TRUE(SSKDF_hmac(out.data(), out.size(), md, secret.data(),
                               secret.size(), info.data(), info.size(),
                               salt.data(), salt.size()));
      } else {
        ASSERT_TRUE(SSKDF_hmac(out.data(), out.size(), md, secret.data(),
                               secret.size(), info.data(), info.size(), NULL,
                               0));
      }
    }
    ASSERT_EQ(Bytes(expect.data(), expect.size()),
              Bytes(out.data(), out.size()));
  });
}

TEST(SSKDFTest, DigestNegativeTests) {
  const uint8_t secret[16] = {0};
  std::vector<uint8_t> out(16);

  // NULL output
  ASSERT_FALSE(SSKDF_digest(NULL, out.size(), EVP_sha256(), &secret[0],
                            sizeof(secret), NULL, 0));

  // zero-length output
  ASSERT_FALSE(SSKDF_digest(out.data(), 0, EVP_sha256(), &secret[0],
                            sizeof(secret), NULL, 0));

  // NULL digest
  ASSERT_FALSE(SSKDF_digest(out.data(), out.size(), NULL, &secret[0],
                            sizeof(secret), NULL, 0));

  // NULL secret
  ASSERT_FALSE(
      SSKDF_digest(out.data(), out.size(), EVP_sha256(), NULL, 0, NULL, 0));

  // zero-length secret
  ASSERT_FALSE(SSKDF_digest(out.data(), out.size(), EVP_sha256(), &secret[0], 0,
                            NULL, 0));
}

TEST(SSKDFTest, HMACNegativeTests) {
  const uint8_t secret[16] = {0};
  std::vector<uint8_t> out(16);

  // NULL output
  ASSERT_FALSE(SSKDF_hmac(NULL, out.size(), EVP_sha256(), &secret[0],
                          sizeof(secret), NULL, 0, NULL, 0));

  // zero-length output
  ASSERT_FALSE(SSKDF_hmac(out.data(), 0, EVP_sha256(), &secret[0],
                          sizeof(secret), NULL, 0, NULL, 0));

  // NULL digest
  ASSERT_FALSE(SSKDF_hmac(out.data(), out.size(), NULL, &secret[0],
                          sizeof(secret), NULL, 0, NULL, 0));

  // NULL secret
  ASSERT_FALSE(SSKDF_hmac(out.data(), out.size(), EVP_sha256(), NULL, 0, NULL,
                          0, NULL, 0));

  // zero-length secret
  ASSERT_FALSE(SSKDF_hmac(out.data(), out.size(), EVP_sha256(), &secret[0], 0,
                          NULL, 0, NULL, 0));
}

TEST(KBKDFCounterTest, TestVectors) {
  FileTestGTest(
      "crypto/fipsmodule/kdf/test/kbkdf_counter.txt", [](FileTest *t) {
        const EVP_MD *md;
        std::string hash;
        ASSERT_TRUE(t->GetAttribute(&hash, "HASH"));
        if (hash == "SHA1") {
          md = EVP_sha1();
        } else if (hash == "SHA-224") {
          md = EVP_sha224();
        } else if (hash == "SHA-256") {
          md = EVP_sha256();
        } else if (hash == "SHA-384") {
          md = EVP_sha384();
        } else if (hash == "SHA-512") {
          md = EVP_sha512();
        } else {
          FAIL() << "Unknown HASH=" + hash;
        }

        std::vector<uint8_t> secret, info, expect;

        ASSERT_TRUE(t->GetBytes(&secret, "SECRET"));
        if (t->HasAttribute("INFO")) {
          ASSERT_TRUE(t->GetBytes(&info, "INFO"));
        } else {
          info = std::vector<uint8_t>(0);
        }
        ASSERT_TRUE(t->GetBytes(&expect, "EXPECT"));

        std::vector<uint8_t> out(expect.size());

        ASSERT_TRUE(KBKDF_ctr_hmac(out.data(), out.size(), md, secret.data(),
                                   secret.size(), info.data(), info.size()));
        ASSERT_EQ(Bytes(expect.data(), expect.size()),
                  Bytes(out.data(), out.size()));
      });
}

TEST(KBKDFCounterTest, NegativeTests) {
  const uint8_t secret[16] = {0};
  std::vector<uint8_t> out(16);

  // NULL output
  ASSERT_FALSE(KBKDF_ctr_hmac(NULL, out.size(), EVP_sha256(), &secret[0],
                              sizeof(secret), NULL, 0));

  // zero-length output
  ASSERT_FALSE(KBKDF_ctr_hmac(out.data(), 0, EVP_sha256(), &secret[0],
                              sizeof(secret), NULL, 0));

  // NULL Digest
  ASSERT_FALSE(KBKDF_ctr_hmac(out.data(), out.size(), NULL, &secret[0],
                              sizeof(secret), NULL, 0));

  // NULL secret
  ASSERT_FALSE(KBKDF_ctr_hmac(out.data(), out.size(), EVP_sha256(), NULL,
                              sizeof(secret), NULL, 0));

  // zero-length secret
  ASSERT_FALSE(KBKDF_ctr_hmac(out.data(), out.size(), EVP_sha256(), &secret[0],
                              0, NULL, 0));
}
