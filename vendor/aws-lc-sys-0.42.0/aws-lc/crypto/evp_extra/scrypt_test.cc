// Copyright (c) 2017, Google Inc.
// SPDX-License-Identifier: ISC

#include <stdlib.h>
#include <string.h>

#include <vector>

#include <gtest/gtest.h>

#include <openssl/err.h>
#include <openssl/evp.h>

#include "../test/file_test.h"
#include "../test/test_util.h"


static bool GetUint64(FileTest *t, uint64_t *out, const char *name) {
  std::string str;
  if (!t->GetAttribute(&str, name)) {
    return false;
  }

  char *endptr;
  *out = strtoull(str.data(), &endptr, 10);
  return !str.empty() && *endptr == '\0';
}

TEST(ScryptTest, TestVectors) {
  FileTestGTest("crypto/evp_extra/scrypt_tests.txt", [](FileTest *t) {
    std::vector<uint8_t> password, salt, key;
    uint64_t N, r, p, max_mem = 0;
    ASSERT_TRUE(t->GetBytes(&password, "Password"));
    ASSERT_TRUE(t->GetBytes(&salt, "Salt"));
    ASSERT_TRUE(t->GetBytes(&key, "Key"));
    ASSERT_TRUE(GetUint64(t, &N, "N"));
    ASSERT_TRUE(GetUint64(t, &r, "r"));
    ASSERT_TRUE(GetUint64(t, &p, "p"));
    if (t->HasAttribute("MaxMemory")) {
      ASSERT_TRUE(GetUint64(t, &max_mem, "MaxMemory"));
    }

    std::vector<uint8_t> result(key.size());
    ASSERT_TRUE(EVP_PBE_scrypt(reinterpret_cast<const char *>(password.data()),
                               password.size(), salt.data(), salt.size(), N, r,
                               p, max_mem, result.data(), result.size()));
    EXPECT_EQ(Bytes(key), Bytes(result));
  });
}

TEST(ScryptTest, MemoryLimit) {
  static const char kPassword[] = "pleaseletmein";
  static const char kSalt[] = "SodiumChloride";

  // This test requires more than 1GB to run.
  uint8_t key[64];
  EXPECT_FALSE(EVP_PBE_scrypt(kPassword, strlen(kPassword),
                              reinterpret_cast<const uint8_t *>(kSalt),
                              strlen(kSalt), 1048576 /* N */, 8 /* r */,
                              1 /* p */, 0 /* max_mem */, key, sizeof(key)));
  EXPECT_TRUE(
      ErrorEquals(ERR_get_error(), ERR_LIB_EVP, EVP_R_MEMORY_LIMIT_EXCEEDED));
}

TEST(ScryptTest, InvalidParameters) {
  uint8_t key[64];

  // p and r are non-zero.
  EXPECT_FALSE(EVP_PBE_scrypt(nullptr, 0, nullptr, 0, 1024 /* N */, 0 /* r */,
                              1 /* p */, 0 /* max_mem */, key, sizeof(key)));
  EXPECT_FALSE(EVP_PBE_scrypt(nullptr, 0, nullptr, 0, 1024 /* N */, 8 /* r */,
                              0 /* p */, 0 /* max_mem */, key, sizeof(key)));

  // N must be a power of 2 > 1.
  EXPECT_FALSE(EVP_PBE_scrypt(nullptr, 0, nullptr, 0, 0 /* N */, 8 /* r */,
                              1 /* p */, 0 /* max_mem */, key, sizeof(key)));
  EXPECT_FALSE(EVP_PBE_scrypt(nullptr, 0, nullptr, 0, 1 /* N */, 8 /* r */,
                              1 /* p */, 0 /* max_mem */, key, sizeof(key)));
  EXPECT_FALSE(EVP_PBE_scrypt(nullptr, 0, nullptr, 0, 1023 /* N */, 8 /* r */,
                              1 /* p */, 0 /* max_mem */, key, sizeof(key)));
  EXPECT_TRUE(EVP_PBE_scrypt(nullptr, 0, nullptr, 0, 1024 /* N */, 8 /* r */,
                              1 /* p */, 0 /* max_mem */, key, sizeof(key)));
  EXPECT_FALSE(EVP_PBE_scrypt(nullptr, 0, nullptr, 0, 1025 /* N */, 8 /* r */,
                              1 /* p */, 0 /* max_mem */, key, sizeof(key)));

  // N must be below 2^(128 * r / 8).
  EXPECT_FALSE(EVP_PBE_scrypt(nullptr, 0, nullptr, 0, 65536 /* N */, 1 /* r */,
                              1 /* p */, 0 /* max_mem */, key, sizeof(key)));
  EXPECT_TRUE(EVP_PBE_scrypt(nullptr, 0, nullptr, 0, 32768 /* N */, 1 /* r */,
                             1 /* p */, 0 /* max_mem */, key, sizeof(key)));
}
