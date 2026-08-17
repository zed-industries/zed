// Copyright (c) 2019, Google Inc.
// SPDX-License-Identifier: ISC

#include <openssl/cipher.h>

#include <gtest/gtest.h>

#include "../../internal.h"
#include "../../test/test_util.h"

struct CastTestCase {
  uint8_t key[16];
  uint8_t plaintext[16];
  uint8_t iv[8];
  uint8_t ecb_ciphertext[16];
  uint8_t cbc_ciphertext[24];
};

static const CastTestCase kTests[] = {
    // Randomly generated test cases. Checked against vanilla OpenSSL.
    {
        {0xbb, 0x56, 0xb1, 0x27, 0x7c, 0x4c, 0xdd, 0x5a, 0x99, 0x90, 0x1e, 0x6f,
         0xeb, 0x36, 0x6c, 0xf3},
        {0xa6, 0x5b, 0xe0, 0x99, 0xad, 0x5d, 0x91, 0x98, 0x37, 0xc1, 0xa4, 0x7f,
         0x01, 0x24, 0x9a, 0x6b},
        {0xd5, 0x8a, 0x5c, 0x29, 0xeb, 0xee, 0xed, 0x76},
        {0x01, 0x8d, 0x1b, 0x42, 0xb8, 0x77, 0xc8, 0x84, 0x25, 0x7d, 0xd4, 0x89,
         0x8d, 0xc1, 0xbc, 0x2a},
        {0xc1, 0x05, 0xa1, 0x9a, 0xb4, 0xc4, 0xd0, 0x15,
         0x9d, 0xfd, 0xea, 0xd0, 0xc3, 0x54, 0xe5, 0x33,
         0x26, 0xac, 0x25, 0xf3, 0x48, 0xbc, 0xf6, 0xa2},
    },
    {
        {0x5d, 0x98, 0xa9, 0xd2, 0x27, 0x5d, 0xc8, 0x8c, 0x8c, 0xee, 0x23, 0x7f,
         0x8e, 0x2b, 0xd4, 0x8d},
        {0x60, 0xec, 0x31, 0xda, 0x25, 0x07, 0x02, 0x14, 0x84, 0x44, 0x96, 0xa6,
         0x04, 0x81, 0xca, 0x4e},
        {0x96, 0x4c, 0xa4, 0x07, 0xee, 0x1c, 0xd1, 0xfb},
        {0x58, 0x62, 0x29, 0x62, 0x23, 0x69, 0x9e, 0xe8, 0x27, 0xc2, 0xcd, 0x5b,
         0x35, 0xf1, 0xdf, 0xa4},
        {0x1c, 0xd0, 0x29, 0xe5, 0xf3, 0xdb, 0x65, 0x60,
         0x05, 0xde, 0x01, 0x2b, 0x10, 0x09, 0x44, 0x56,
         0x59, 0x44, 0x00, 0x26, 0xdb, 0xb3, 0x2d, 0x98},
    },
};

TEST(CAST, ECB) {
  unsigned test_num = 0;
  for (const auto &test : kTests) {
    test_num++;
    SCOPED_TRACE(test_num);

    uint8_t out[sizeof(test.ecb_ciphertext)];
    int out_bytes, final_bytes;

    bssl::ScopedEVP_CIPHER_CTX ctx;
    ASSERT_TRUE(EVP_EncryptInit_ex(ctx.get(), EVP_cast5_ecb(), nullptr,
                                   test.key, nullptr));
    ASSERT_TRUE(EVP_CIPHER_CTX_set_padding(ctx.get(), 0 /* no padding */));
    ASSERT_TRUE(EVP_EncryptUpdate(ctx.get(), out, &out_bytes, test.plaintext,
                                  sizeof(test.plaintext)));
    ASSERT_TRUE(EVP_EncryptFinal_ex(ctx.get(), out + out_bytes, &final_bytes));
    EXPECT_EQ(static_cast<size_t>(out_bytes + final_bytes),
              sizeof(test.plaintext));
    EXPECT_EQ(Bytes(test.ecb_ciphertext), Bytes(out));

    bssl::ScopedEVP_CIPHER_CTX decrypt_ctx;
    ASSERT_TRUE(EVP_DecryptInit_ex(decrypt_ctx.get(), EVP_cast5_ecb(), nullptr,
                                   test.key, nullptr));
    ASSERT_TRUE(
        EVP_CIPHER_CTX_set_padding(decrypt_ctx.get(), 0 /* no padding */));
    ASSERT_TRUE(EVP_DecryptUpdate(decrypt_ctx.get(), out, &out_bytes,
                                  test.ecb_ciphertext,
                                  sizeof(test.ecb_ciphertext)));
    ASSERT_TRUE(
        EVP_DecryptFinal_ex(decrypt_ctx.get(), out + out_bytes, &final_bytes));
    EXPECT_EQ(static_cast<size_t>(out_bytes + final_bytes),
              sizeof(test.plaintext));
    EXPECT_EQ(Bytes(test.plaintext), Bytes(out));
  }
}

TEST(CAST, CBC) {
  unsigned test_num = 0;
  for (const auto &test : kTests) {
    test_num++;
    SCOPED_TRACE(test_num);

    uint8_t out[sizeof(test.cbc_ciphertext)];
    int out_bytes, final_bytes;

    bssl::ScopedEVP_CIPHER_CTX ctx;
    ASSERT_TRUE(EVP_EncryptInit_ex(ctx.get(), EVP_cast5_cbc(), nullptr,
                                   test.key, test.iv));
    ASSERT_TRUE(EVP_EncryptUpdate(ctx.get(), out, &out_bytes, test.plaintext,
                                  sizeof(test.plaintext)));
    EXPECT_TRUE(EVP_EncryptFinal_ex(ctx.get(), out + out_bytes, &final_bytes));
    EXPECT_EQ(static_cast<size_t>(out_bytes + final_bytes),
              sizeof(test.cbc_ciphertext));
    EXPECT_EQ(Bytes(test.cbc_ciphertext), Bytes(out));

    bssl::ScopedEVP_CIPHER_CTX decrypt_ctx;
    ASSERT_TRUE(EVP_DecryptInit_ex(decrypt_ctx.get(), EVP_cast5_cbc(), nullptr,
                                   test.key, test.iv));
    ASSERT_TRUE(EVP_DecryptUpdate(decrypt_ctx.get(), out, &out_bytes,
                                  test.cbc_ciphertext,
                                  sizeof(test.cbc_ciphertext)));
    EXPECT_TRUE(
        EVP_DecryptFinal_ex(decrypt_ctx.get(), out + out_bytes, &final_bytes));
    EXPECT_EQ(static_cast<size_t>(out_bytes + final_bytes),
              sizeof(test.plaintext));
    EXPECT_EQ(Bytes(test.plaintext), Bytes(out, out_bytes + final_bytes));
  }
}
