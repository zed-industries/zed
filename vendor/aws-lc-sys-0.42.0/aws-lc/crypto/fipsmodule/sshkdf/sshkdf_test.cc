// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC

#include <gtest/gtest.h>

#include <openssl/crypto.h>
#include <openssl/digest.h>
#include <openssl/sshkdf.h>

#include "../../internal.h"
#include "../../test/file_test.h"
#include "../../test/test_util.h"

TEST(SSHKDFTest, SSHKDF_INPUT_INSANITY) {
    uint8_t not_empty[] = {'t', 'e', 's', 't'};
    size_t not_empty_len = sizeof(not_empty);
    uint8_t output[] = {0};
    size_t output_len = sizeof(output);
    const EVP_MD *md = EVP_sha256();  // Not actually used.

    ASSERT_FALSE(SSHKDF(nullptr, not_empty, not_empty_len,
                        not_empty, not_empty_len, not_empty, not_empty_len,
                        EVP_KDF_SSHKDF_TYPE_INITIAL_IV_CLI_TO_SRV,
                        output, output_len));

    ASSERT_FALSE(SSHKDF(md, nullptr, not_empty_len,
                        not_empty, not_empty_len, not_empty, not_empty_len,
                        EVP_KDF_SSHKDF_TYPE_INITIAL_IV_CLI_TO_SRV,
                        output, output_len));
    ASSERT_FALSE(SSHKDF(md, not_empty, 0,
                        not_empty, not_empty_len, not_empty, not_empty_len,
                        EVP_KDF_SSHKDF_TYPE_INITIAL_IV_CLI_TO_SRV,
                        output, output_len));

    ASSERT_FALSE(SSHKDF(md, not_empty, not_empty_len,
                        nullptr, not_empty_len, not_empty, not_empty_len,
                        EVP_KDF_SSHKDF_TYPE_INITIAL_IV_CLI_TO_SRV,
                        output, output_len));
    ASSERT_FALSE(SSHKDF(md, not_empty, not_empty_len,
                        not_empty, 0, not_empty, not_empty_len,
                        EVP_KDF_SSHKDF_TYPE_INITIAL_IV_CLI_TO_SRV,
                        output, output_len));

    ASSERT_FALSE(SSHKDF(md, not_empty, not_empty_len,
                        not_empty, not_empty_len, nullptr, not_empty_len,
                        EVP_KDF_SSHKDF_TYPE_INITIAL_IV_CLI_TO_SRV,
                        output, output_len));
    ASSERT_FALSE(SSHKDF(md, not_empty, not_empty_len,
                        not_empty, not_empty_len, not_empty, 0,
                        EVP_KDF_SSHKDF_TYPE_INITIAL_IV_CLI_TO_SRV,
                        output, output_len));

    ASSERT_FALSE(SSHKDF(md, not_empty, not_empty_len,
                        not_empty, not_empty_len, not_empty, not_empty_len,
                        EVP_KDF_SSHKDF_TYPE_INITIAL_IV_CLI_TO_SRV - 1,
                        output, output_len));
    ASSERT_FALSE(SSHKDF(md, not_empty, not_empty_len,
                        not_empty, not_empty_len, not_empty, not_empty_len,
                        EVP_KDF_SSHKDF_TYPE_INTEGRITY_KEY_SRV_TO_CLI + 1,
                        output, output_len));
}

static void RunTest(FileTest *t)
{
  std::string count;
  std::vector<uint8_t> key, xcghash, session_id, initial_iv_c2s, initial_iv_s2c,
    encryption_key_c2s, encryption_key_s2c, integrity_key_c2s,
    integrity_key_s2c;

  t->IgnoreAllUnusedInstructions();

  const EVP_MD *md = NULL;
  if (t->HasInstruction("SHA-1")) {
    md = EVP_sha1();
  } else if (t->HasInstruction("SHA-224")) {
    md = EVP_sha224();
  } else if (t->HasInstruction("SHA-256")) {
    md = EVP_sha256();
  } else if (t->HasInstruction("SHA-384")) {
    md = EVP_sha384();
  } else if (t->HasInstruction("SHA-512")) {
    md = EVP_sha512();
  } else {
    // Unknown/unsupported hash in test input. Did someone update the test
    // data without updating the code?
    ASSERT_NE(md, nullptr);
  }

  // These are all specified in bits.
  std::string iv_len_str, encryption_key_len_str;
  ASSERT_TRUE(t->GetInstruction(&iv_len_str, "IV length"));
  unsigned long iv_len = std::stoul(iv_len_str) / 8;
  ASSERT_TRUE(t->GetInstruction(&encryption_key_len_str, "encryption key length"));
  unsigned long encryption_key_len = std::stoul(encryption_key_len_str) / 8;

  ASSERT_TRUE(t->GetAttribute(&count, "COUNT"));
  ASSERT_TRUE(t->GetBytes(&key, "K"));
  ASSERT_TRUE(t->GetBytes(&xcghash, "H"));
  ASSERT_TRUE(t->GetBytes(&session_id, "session_id"));
  ASSERT_TRUE(t->GetBytes(&initial_iv_c2s, "Initial IV (client to server)"));
  ASSERT_TRUE(t->GetBytes(&initial_iv_s2c, "Initial IV (server to client)"));
  ASSERT_TRUE(t->GetBytes(&encryption_key_c2s, "Encryption key (client to server)"));
  ASSERT_TRUE(t->GetBytes(&encryption_key_s2c, "Encryption key (server to client)"));
  ASSERT_TRUE(t->GetBytes(&integrity_key_c2s, "Integrity key (client to server)"));
  ASSERT_TRUE(t->GetBytes(&integrity_key_s2c, "Integrity key (server to client)"));

  // The CAVP test data shows its work, repeatedly. Ignore these.
  t->IgnoreAttribute("K || H || K1");
  t->IgnoreAttribute("K || H || K1/2");
  t->IgnoreAttribute("K || H || X || session id");
  t->IgnoreAttribute("K || H || X || session id/2");
  t->IgnoreAttribute("K || H || X || session id/3");
  t->IgnoreAttribute("K || H || X || session id/4");
  t->IgnoreAttribute("K || H || X || session id/5");
  t->IgnoreAttribute("K || H || X || session id/6");
  t->IgnoreAttribute("K/2");
  t->IgnoreAttribute("K/3");
  t->IgnoreAttribute("K/4");
  t->IgnoreAttribute("K/5");
  t->IgnoreAttribute("K/6");
  t->IgnoreAttribute("K/7");
  t->IgnoreAttribute("K1");
  t->IgnoreAttribute("K1/2");
  t->IgnoreAttribute("K1/3");
  t->IgnoreAttribute("K1/4");
  t->IgnoreAttribute("K1/5");
  t->IgnoreAttribute("K1/6");
  t->IgnoreAttribute("K2");
  t->IgnoreAttribute("K2/2");
  t->IgnoreAttribute("X");
  t->IgnoreAttribute("X/2");
  t->IgnoreAttribute("X/3");
  t->IgnoreAttribute("X/4");
  t->IgnoreAttribute("X/5");
  t->IgnoreAttribute("X/6");

  // Why isn't this output length specified in the data file? It is a mystery.
  unsigned integrity_key_len = integrity_key_c2s.size();

  // Initial IVs
  uint8_t *output = static_cast<uint8_t *>(new uint8_t[iv_len]);

  ASSERT_TRUE(SSHKDF(md, key.data(), key.size(), xcghash.data(), xcghash.size(),
    session_id.data(), session_id.size(),
    EVP_KDF_SSHKDF_TYPE_INITIAL_IV_CLI_TO_SRV,
    output, iv_len));
  EXPECT_EQ(Bytes(initial_iv_c2s.data(), initial_iv_c2s.size()),
    Bytes(output, iv_len));

  ASSERT_TRUE(SSHKDF(md, key.data(), key.size(), xcghash.data(), xcghash.size(),
    session_id.data(), session_id.size(),
    EVP_KDF_SSHKDF_TYPE_INITIAL_IV_SRV_TO_CLI,
    output, iv_len));
  EXPECT_EQ(Bytes(initial_iv_s2c.data(), initial_iv_s2c.size()),
    Bytes(output, iv_len));

  delete[] output;
  output = NULL;

  // Encryption keys
  output = static_cast<uint8_t *>(new uint8_t[encryption_key_len]);

  ASSERT_TRUE(SSHKDF(md, key.data(), key.size(), xcghash.data(), xcghash.size(),
    session_id.data(), session_id.size(),
    EVP_KDF_SSHKDF_TYPE_ENCRYPTION_KEY_CLI_TO_SRV,
    output, encryption_key_len));
  EXPECT_EQ(Bytes(encryption_key_c2s.data(), encryption_key_c2s.size()),
    Bytes(output, encryption_key_len));

  ASSERT_TRUE(SSHKDF(md, key.data(), key.size(), xcghash.data(), xcghash.size(),
    session_id.data(), session_id.size(),
    EVP_KDF_SSHKDF_TYPE_ENCRYPTION_KEY_SRV_TO_CLI,
    output, encryption_key_len));
  EXPECT_EQ(Bytes(encryption_key_s2c.data(), encryption_key_s2c.size()),
    Bytes(output, encryption_key_len));

  delete[] output;
  output = NULL;

  // Integrity keys
  output = static_cast<uint8_t *>(new uint8_t[integrity_key_len]);

  ASSERT_TRUE(SSHKDF(md, key.data(), key.size(), xcghash.data(), xcghash.size(),
    session_id.data(), session_id.size(),
    EVP_KDF_SSHKDF_TYPE_INTEGRITY_KEY_CLI_TO_SRV,
    output, integrity_key_len));
  EXPECT_EQ(Bytes(integrity_key_c2s.data(), integrity_key_c2s.size()),
    Bytes(output, integrity_key_len));

  ASSERT_TRUE(SSHKDF(md, key.data(), key.size(), xcghash.data(), xcghash.size(),
    session_id.data(), session_id.size(),
    EVP_KDF_SSHKDF_TYPE_INTEGRITY_KEY_SRV_TO_CLI,
    output, integrity_key_len));
  EXPECT_EQ(Bytes(integrity_key_s2c.data(), integrity_key_s2c.size()),
    Bytes(output, integrity_key_len));

  delete[] output;
  output = NULL;
}

TEST(SSHKDFTest, KAT) {
  FileTestGTest("crypto/evp_extra/sshkdf_tests.txt", RunTest);
}
