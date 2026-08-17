// Copyright (c) 2015, Google Inc.
// SPDX-License-Identifier: ISC

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include <algorithm>
#include <memory>
#include <vector>

#include <gtest/gtest.h>

#include <openssl/aes.h>
#include <openssl/cipher.h>
#include <openssl/rand.h>

#include "../../internal.h"
#include "../../test/abi_test.h"
#include "../../test/file_test.h"
#include "../../test/test_util.h"
#include "../../test/wycheproof_util.h"
#include "../cpucap/internal.h"
#include "internal.h"

// All test vectors use the default IV, so test both with implicit and
// explicit IV.
//
// TODO(davidben): Find test vectors that use a different IV.
static const uint8_t kDefaultIV[] = {
    0xa6, 0xa6, 0xa6, 0xa6, 0xa6, 0xa6, 0xa6, 0xa6,
};

static void TestRaw(FileTest *t) {
  std::vector<uint8_t> key, plaintext, ciphertext;
  ASSERT_TRUE(t->GetBytes(&key, "Key"));
  ASSERT_TRUE(t->GetBytes(&plaintext, "Plaintext"));
  ASSERT_TRUE(t->GetBytes(&ciphertext, "Ciphertext"));

  ASSERT_EQ(static_cast<unsigned>(AES_BLOCK_SIZE), plaintext.size());
  ASSERT_EQ(static_cast<unsigned>(AES_BLOCK_SIZE), ciphertext.size());

  AES_KEY aes_key;
  ASSERT_EQ(0, AES_set_encrypt_key(key.data(), 8 * key.size(), &aes_key));

  // Test encryption.
  uint8_t block[AES_BLOCK_SIZE];
  AES_encrypt(plaintext.data(), block, &aes_key);
  EXPECT_EQ(Bytes(ciphertext), Bytes(block));

  // Test in-place encryption.
  OPENSSL_memcpy(block, plaintext.data(), AES_BLOCK_SIZE);
  AES_encrypt(block, block, &aes_key);
  EXPECT_EQ(Bytes(ciphertext), Bytes(block));

  ASSERT_EQ(0, AES_set_decrypt_key(key.data(), 8 * key.size(), &aes_key));

  // Test decryption.
  AES_decrypt(ciphertext.data(), block, &aes_key);
  EXPECT_EQ(Bytes(plaintext), Bytes(block));

  // Test in-place decryption.
  OPENSSL_memcpy(block, ciphertext.data(), AES_BLOCK_SIZE);
  AES_decrypt(block, block, &aes_key);
  EXPECT_EQ(Bytes(plaintext), Bytes(block));
}

static void TestKeyWrap(FileTest *t) {
  std::vector<uint8_t> key, plaintext, ciphertext;
  ASSERT_TRUE(t->GetBytes(&key, "Key"));
  ASSERT_TRUE(t->GetBytes(&plaintext, "Plaintext"));
  ASSERT_TRUE(t->GetBytes(&ciphertext, "Ciphertext"));

  ASSERT_EQ(plaintext.size() + 8, ciphertext.size())
      << "Invalid Plaintext and Ciphertext lengths.";

  // Test encryption.
  AES_KEY aes_key;
  ASSERT_EQ(0, AES_set_encrypt_key(key.data(), 8 * key.size(), &aes_key));

  // Test with implicit IV.
  std::unique_ptr<uint8_t[]> buf(new uint8_t[ciphertext.size()]);
  int len = AES_wrap_key(&aes_key, nullptr /* iv */, buf.get(),
                         plaintext.data(), plaintext.size());
  ASSERT_GE(len, 0);
  EXPECT_EQ(Bytes(ciphertext), Bytes(buf.get(), static_cast<size_t>(len)));

  // Test with explicit IV.
  OPENSSL_memset(buf.get(), 0, ciphertext.size());
  len = AES_wrap_key(&aes_key, kDefaultIV, buf.get(), plaintext.data(),
                     plaintext.size());
  ASSERT_GE(len, 0);
  EXPECT_EQ(Bytes(ciphertext), Bytes(buf.get(), static_cast<size_t>(len)));

  // Test decryption.
  ASSERT_EQ(0, AES_set_decrypt_key(key.data(), 8 * key.size(), &aes_key));

  // Test with implicit IV.
  buf.reset(new uint8_t[plaintext.size()]);
  len = AES_unwrap_key(&aes_key, nullptr /* iv */, buf.get(), ciphertext.data(),
                       ciphertext.size());
  ASSERT_GE(len, 0);
  EXPECT_EQ(Bytes(plaintext), Bytes(buf.get(), static_cast<size_t>(len)));

  // Test with explicit IV.
  OPENSSL_memset(buf.get(), 0, plaintext.size());
  len = AES_unwrap_key(&aes_key, kDefaultIV, buf.get(), ciphertext.data(),
                       ciphertext.size());
  ASSERT_GE(len, 0);

  // Test corrupted ciphertext.
  ciphertext[0] ^= 1;
  EXPECT_EQ(-1, AES_unwrap_key(&aes_key, nullptr /* iv */, buf.get(),
                               ciphertext.data(), ciphertext.size()));
}

static void TestEVPKeyWrap(FileTest *t) {
  std::vector<uint8_t> key, plaintext, ciphertext;
  ASSERT_TRUE(t->GetBytes(&key, "Key"));
  ASSERT_TRUE(t->GetBytes(&plaintext, "Plaintext"));
  ASSERT_TRUE(t->GetBytes(&ciphertext, "Ciphertext"));

  // Only 256 bit keys are supported for key wrap from EVP_CIPHER at the moment.
  if (key.size() != 32) {
    return;
  }

  const EVP_CIPHER *cipher = EVP_aes_256_wrap();

  ASSERT_EQ(plaintext.size() + 8, ciphertext.size())
      << "Invalid Plaintext and Ciphertext lengths.";

  // Test encryption.
  std::vector<uint8_t> out(ciphertext.size());
  int len;
  // Test with implicit IV.
  bssl::ScopedEVP_CIPHER_CTX ctx;
  ASSERT_TRUE(
      EVP_EncryptInit_ex(ctx.get(), cipher, nullptr, key.data(), nullptr));
  ASSERT_TRUE(EVP_EncryptUpdate(ctx.get(), out.data(), &len, plaintext.data(),
                                plaintext.size()));
  ASSERT_GE(len, 0);
  ASSERT_TRUE(EVP_EncryptFinal(ctx.get(), out.data(), &len));
  EXPECT_EQ(Bytes(ciphertext), Bytes(out));

  // Test with explicit IV.
  ctx.Reset();
  ASSERT_TRUE(
      EVP_EncryptInit_ex(ctx.get(), cipher, nullptr, key.data(), kDefaultIV));
  ASSERT_TRUE(EVP_EncryptUpdate(ctx.get(), out.data(), &len, plaintext.data(),
                                plaintext.size()));
  ASSERT_GE(len, 0);
  ASSERT_TRUE(EVP_EncryptFinal(ctx.get(), out.data(), &len));
  EXPECT_EQ(Bytes(ciphertext), Bytes(out));

  // Test decryption.
  out.clear();
  out.resize(plaintext.size());
  ctx.Reset();
  // Test with implicit IV.
  ASSERT_TRUE(
      EVP_DecryptInit_ex(ctx.get(), cipher, nullptr, key.data(), nullptr));
  ASSERT_TRUE(EVP_DecryptUpdate(ctx.get(), out.data(), &len, ciphertext.data(),
                                ciphertext.size()));
  out.resize(len);
  ASSERT_TRUE(EVP_EncryptFinal(ctx.get(), out.data(), &len));
  EXPECT_EQ(Bytes(plaintext), Bytes(out));

  // Test with explicit IV.
  ctx.Reset();
  ASSERT_TRUE(
      EVP_DecryptInit_ex(ctx.get(), cipher, nullptr, key.data(), kDefaultIV));
  ASSERT_TRUE(EVP_DecryptUpdate(ctx.get(), out.data(), &len, ciphertext.data(),
                                ciphertext.size()));
  out.resize(len);
  ASSERT_TRUE(EVP_EncryptFinal(ctx.get(), out.data(), &len));
  EXPECT_EQ(Bytes(plaintext), Bytes(out));

  // Test corrupted ciphertext.
  ctx.Reset();
  ciphertext[0] ^= 1;
  ASSERT_TRUE(
      EVP_DecryptInit_ex(ctx.get(), cipher, nullptr, key.data(), nullptr));
  EXPECT_FALSE(EVP_DecryptUpdate(ctx.get(), out.data(), &len, ciphertext.data(),
                                 ciphertext.size()));
}

static void TestKeyWrapWithPadding(FileTest *t) {
  std::vector<uint8_t> key, plaintext, ciphertext;
  ASSERT_TRUE(t->GetBytes(&key, "Key"));
  ASSERT_TRUE(t->GetBytes(&plaintext, "Plaintext"));
  ASSERT_TRUE(t->GetBytes(&ciphertext, "Ciphertext"));

  // Test encryption.
  AES_KEY aes_key;
  ASSERT_EQ(0, AES_set_encrypt_key(key.data(), 8 * key.size(), &aes_key));
  std::unique_ptr<uint8_t[]> buf(new uint8_t[plaintext.size() + 15]);
  size_t len;
  ASSERT_TRUE(AES_wrap_key_padded(&aes_key, buf.get(), &len,
                                  plaintext.size() + 15, plaintext.data(),
                                  plaintext.size()));
  EXPECT_EQ(Bytes(ciphertext), Bytes(buf.get(), static_cast<size_t>(len)));

  // Test decryption
  ASSERT_EQ(0, AES_set_decrypt_key(key.data(), 8 * key.size(), &aes_key));
  buf.reset(new uint8_t[ciphertext.size() - 8]);
  ASSERT_TRUE(AES_unwrap_key_padded(&aes_key, buf.get(), &len,
                                    ciphertext.size() - 8, ciphertext.data(),
                                    ciphertext.size()));
  ASSERT_EQ(len, plaintext.size());
  EXPECT_EQ(Bytes(plaintext), Bytes(buf.get(), static_cast<size_t>(len)));
}

TEST(AESTest, TestVectors) {
  FileTestGTest("crypto/fipsmodule/aes/aes_tests.txt", [](FileTest *t) {
    if (t->GetParameter() == "Raw") {
      TestRaw(t);
    } else if (t->GetParameter() == "KeyWrap") {
      TestKeyWrap(t);
      TestEVPKeyWrap(t);
    } else if (t->GetParameter() == "KeyWrapWithPadding") {
      TestKeyWrapWithPadding(t);
    } else {
      ADD_FAILURE() << "Unknown mode " << t->GetParameter();
    }
  });
}

TEST(AESTest, WycheproofKeyWrap) {
  FileTestGTest("third_party/wycheproof_testvectors/kw_test.txt",
                [](FileTest *t) {
    std::string key_size;
    ASSERT_TRUE(t->GetInstruction(&key_size, "keySize"));
    std::vector<uint8_t> ct, key, msg;
    ASSERT_TRUE(t->GetBytes(&ct, "ct"));
    ASSERT_TRUE(t->GetBytes(&key, "key"));
    ASSERT_TRUE(t->GetBytes(&msg, "msg"));
    ASSERT_EQ(static_cast<unsigned>(atoi(key_size.c_str())), key.size() * 8);
    WycheproofResult result;
    ASSERT_TRUE(GetWycheproofResult(t, &result));

    if (result.IsValid()) {
      ASSERT_GE(ct.size(), 8u);

      AES_KEY aes;
      ASSERT_EQ(0, AES_set_decrypt_key(key.data(), 8 * key.size(), &aes));
      std::vector<uint8_t> out(ct.size() - 8);
      int len = AES_unwrap_key(&aes, nullptr, out.data(), ct.data(), ct.size());
      ASSERT_EQ(static_cast<int>(out.size()), len);
      EXPECT_EQ(Bytes(msg), Bytes(out));

      out.resize(msg.size() + 8);
      ASSERT_EQ(0, AES_set_encrypt_key(key.data(), 8 * key.size(), &aes));
      len = AES_wrap_key(&aes, nullptr, out.data(), msg.data(), msg.size());
      ASSERT_EQ(static_cast<int>(out.size()), len);
      EXPECT_EQ(Bytes(ct), Bytes(out));
    } else {
      AES_KEY aes;
      ASSERT_EQ(0, AES_set_decrypt_key(key.data(), 8 * key.size(), &aes));
      std::vector<uint8_t> out(ct.size() < 8 ? 0 : ct.size() - 8);
      int len = AES_unwrap_key(&aes, nullptr, out.data(), ct.data(), ct.size());
      EXPECT_EQ(-1, len);
    }
  });
}

TEST(AESTest, WycheproofEVPKeyWrap) {
  FileTestGTest("third_party/wycheproof_testvectors/kw_test.txt",
                [](FileTest *t) {
    std::string key_size;
    ASSERT_TRUE(t->GetInstruction(&key_size, "keySize"));
    std::vector<uint8_t> ct, key, msg;
    ASSERT_TRUE(t->GetBytes(&ct, "ct"));
    ASSERT_TRUE(t->GetBytes(&key, "key"));
    ASSERT_TRUE(t->GetBytes(&msg, "msg"));
    ASSERT_EQ(static_cast<unsigned>(atoi(key_size.c_str())), key.size() * 8);
    WycheproofResult result;
    ASSERT_TRUE(GetWycheproofResult(t, &result));

    // Only 256 bit keys are supported for key wrap from EVP_CIPHER at the
    // moment.
    if (key.size() != 32) {
      return;
    }

    const EVP_CIPHER *cipher = EVP_aes_256_wrap();

    if (result.IsValid()) {
      ASSERT_GE(ct.size(), 8u);

      bssl::ScopedEVP_CIPHER_CTX ctx;
      std::vector<uint8_t> out(ct.size() - 8);
      int len;
      ASSERT_TRUE(
        EVP_DecryptInit_ex(ctx.get(), cipher, nullptr, key.data(), nullptr));
      ASSERT_TRUE(EVP_DecryptUpdate(ctx.get(), out.data(), &len, ct.data(),
                                ct.size()));
      ASSERT_EQ(static_cast<int>(out.size()), len);
      ASSERT_TRUE(EVP_EncryptFinal(ctx.get(), out.data(), &len));
      EXPECT_EQ(Bytes(msg), Bytes(out));

      ctx.Reset();
      out.resize(msg.size() + 8);
      ASSERT_TRUE(
        EVP_EncryptInit_ex(ctx.get(), cipher, nullptr, key.data(), nullptr));
      ASSERT_TRUE(EVP_EncryptUpdate(ctx.get(), out.data(), &len, msg.data(),
                                msg.size()));
      ASSERT_EQ(static_cast<int>(out.size()), len);
      ASSERT_TRUE(EVP_EncryptFinal(ctx.get(), out.data(), &len));
      EXPECT_EQ(Bytes(ct), Bytes(out));
    } else {
      bssl::ScopedEVP_CIPHER_CTX ctx;
      std::vector<uint8_t> out(ct.size() < 8 ? 0 : ct.size() - 8);
      int len;
      ASSERT_TRUE(
        EVP_DecryptInit_ex(ctx.get(), cipher, nullptr, key.data(), nullptr));
      if (!ct.empty()) {
        EXPECT_FALSE(EVP_DecryptUpdate(ctx.get(), out.data(), &len, ct.data(),
                                       ct.size()));
        // There is no "Final" function for |EVP_aes_256_wrap|, so this will
        // always return 1.
        EXPECT_TRUE(EVP_EncryptFinal(ctx.get(), out.data(), &len));
      } else {
        // The EVP version of AES-KEY Wrap will return 1 if the ciphertext is
        // NULL. This is consistent with OpenSSL behaviour.
        EXPECT_EQ(EVP_DecryptUpdate(ctx.get(), out.data(), &len, ct.data(),
                                       ct.size()), 1);
        EXPECT_TRUE(EVP_EncryptFinal(ctx.get(), out.data(), &len));
      }
    }
  });
}

TEST(AESTest, WycheproofKeyWrapWithPadding) {
  FileTestGTest("third_party/wycheproof_testvectors/kwp_test.txt",
                [](FileTest *t) {
    std::string key_size;
    ASSERT_TRUE(t->GetInstruction(&key_size, "keySize"));
    std::vector<uint8_t> ct, key, msg;
    ASSERT_TRUE(t->GetBytes(&ct, "ct"));
    ASSERT_TRUE(t->GetBytes(&key, "key"));
    ASSERT_TRUE(t->GetBytes(&msg, "msg"));
    ASSERT_EQ(static_cast<unsigned>(atoi(key_size.c_str())), key.size() * 8);
    WycheproofResult result;
    ASSERT_TRUE(GetWycheproofResult(t, &result));

    // Wycheproof contains test vectors with empty messages that it believes
    // should pass. However, both RFC 5649 and SP 800-38F section 5.3.1 say that
    // the minimum length is one. Therefore we consider test cases with an empty
    // message to be invalid.
    //
    // Wycheproof marks various weak parameters as acceptable. We do not enforce
    // policy in the library, so we map those flags to valid.
    if (result.IsValid({"SmallKey", "WeakWrapping"}) && !msg.empty()) {
      AES_KEY aes;
      ASSERT_EQ(0, AES_set_decrypt_key(key.data(), 8 * key.size(), &aes));
      std::vector<uint8_t> out(ct.size() - 8);
      size_t len;
      ASSERT_TRUE(AES_unwrap_key_padded(&aes, out.data(), &len, ct.size() - 8,
                                        ct.data(), ct.size()));
      EXPECT_EQ(Bytes(msg), Bytes(out.data(), len));

      out.resize(msg.size() + 15);
      ASSERT_EQ(0, AES_set_encrypt_key(key.data(), 8 * key.size(), &aes));
      ASSERT_TRUE(AES_wrap_key_padded(&aes, out.data(), &len, msg.size() + 15,
                                      msg.data(), msg.size()));
      EXPECT_EQ(Bytes(ct), Bytes(out.data(), len));
    } else {
      AES_KEY aes;
      ASSERT_EQ(0, AES_set_decrypt_key(key.data(), 8 * key.size(), &aes));
      std::vector<uint8_t> out(ct.size());
      size_t len;
      ASSERT_FALSE(AES_unwrap_key_padded(&aes, out.data(), &len, ct.size(),
                                         ct.data(), ct.size()));
    }
  });
}

TEST(AESTest, WrapBadLengths) {
  uint8_t key[128/8] = {0};
  AES_KEY aes;
  ASSERT_EQ(0, AES_set_encrypt_key(key, 128, &aes));

  // Input lengths to |AES_wrap_key| must be a multiple of 8 and at least 16.
  static const size_t kLengths[] = {0, 1,  2,  3,  4,  5,  6,  7, 8,
                                    9, 10, 11, 12, 13, 14, 15, 20};
  for (size_t len : kLengths) {
    SCOPED_TRACE(len);
    std::vector<uint8_t> in(len);
    std::vector<uint8_t> out(len + 8);
    EXPECT_EQ(-1,
              AES_wrap_key(&aes, nullptr, out.data(), in.data(), in.size()));
  }
}

TEST(AESTest, InvalidKeySize) {
  static const uint8_t kZero[8] = {0};
  AES_KEY key;
  EXPECT_LT(AES_set_encrypt_key(kZero, 42, &key), 0);
  EXPECT_LT(AES_set_decrypt_key(kZero, 42, &key), 0);
}

#if defined(SUPPORTS_ABI_TEST)
TEST(AESTest, ABI) {
  for (int bits : {128, 192, 256}) {
    SCOPED_TRACE(bits);
    const uint8_t kKey[256/8] = {0};
    AES_KEY key;
    uint8_t block[AES_BLOCK_SIZE];
    uint8_t buf[AES_BLOCK_SIZE * 64] = {0};
    std::vector<int> block_counts;
    if (bits == 128) {
      block_counts = {0, 1, 2, 3, 4, 8, 16, 31};
    } else {
      // Unwind tests are very slow. Assume that the various input sizes do not
      // differ significantly by round count for ABI purposes.
      block_counts = {0, 1, 8};
    }

    if (bsaes_capable()) {
      ASSERT_EQ(vpaes_set_encrypt_key(kKey, bits, &key), 0);
      CHECK_ABI(vpaes_encrypt_key_to_bsaes, &key, &key);
      for (size_t blocks : block_counts) {
        SCOPED_TRACE(blocks);
        if (blocks != 0) {
          CHECK_ABI(bsaes_ctr32_encrypt_blocks, buf, buf, blocks, &key, block);
        }
      }

      ASSERT_EQ(vpaes_set_decrypt_key(kKey, bits, &key), 0);
      CHECK_ABI(vpaes_decrypt_key_to_bsaes, &key, &key);
      for (size_t blocks : block_counts) {
        SCOPED_TRACE(blocks);
        CHECK_ABI(bsaes_cbc_encrypt, buf, buf, AES_BLOCK_SIZE * blocks, &key,
                  block, AES_DECRYPT);
      }
    }

    if (vpaes_capable()) {
      ASSERT_EQ(CHECK_ABI(vpaes_set_encrypt_key, kKey, bits, &key), 0);
      CHECK_ABI(vpaes_encrypt, block, block, &key);
      for (size_t blocks : block_counts) {
        SCOPED_TRACE(blocks);
#if defined(VPAES_CBC)
        CHECK_ABI(vpaes_cbc_encrypt, buf, buf, AES_BLOCK_SIZE * blocks, &key,
                  block, AES_ENCRYPT);
#endif
#if defined(VPAES_CTR32)
        CHECK_ABI(vpaes_ctr32_encrypt_blocks, buf, buf, blocks, &key, block);
#endif
      }

      ASSERT_EQ(CHECK_ABI(vpaes_set_decrypt_key, kKey, bits, &key), 0);
      CHECK_ABI(vpaes_decrypt, block, block, &key);
#if defined(VPAES_CBC)
      for (size_t blocks : block_counts) {
        SCOPED_TRACE(blocks);
        CHECK_ABI(vpaes_cbc_encrypt, buf, buf, AES_BLOCK_SIZE * blocks, &key,
                  block, AES_DECRYPT);
      }
#endif  // VPAES_CBC
    }

    if (hwaes_capable()) {
      ASSERT_EQ(CHECK_ABI(aes_hw_set_encrypt_key, kKey, bits, &key), 0);
      CHECK_ABI(aes_hw_encrypt, block, block, &key);
      for (size_t blocks : block_counts) {
        SCOPED_TRACE(blocks);
        CHECK_ABI(aes_hw_cbc_encrypt, buf, buf, AES_BLOCK_SIZE * blocks, &key,
                  block, AES_ENCRYPT);
        if (blocks == 0) {
          // Without this initialization, valgrind complains
          // about using an unitialized value.
          for (size_t i = 0; i < 64; i++) {
            buf[i] = i;
          }
          std::string buf_before = testing::PrintToString(Bytes(buf,64));
          CHECK_ABI(aes_hw_ctr32_encrypt_blocks, buf, buf, blocks, &key, block);
          EXPECT_EQ(buf_before, testing::PrintToString(Bytes(buf,64)));
        }

        CHECK_ABI(aes_hw_ctr32_encrypt_blocks, buf, buf, blocks, &key, block);
#if defined(HWAES_ECB)
        CHECK_ABI(aes_hw_ecb_encrypt, buf, buf, AES_BLOCK_SIZE * blocks, &key,
                  AES_ENCRYPT);
#endif
      }

      ASSERT_EQ(CHECK_ABI(aes_hw_set_decrypt_key, kKey, bits, &key), 0);
      CHECK_ABI(aes_hw_decrypt, block, block, &key);
      for (size_t blocks : block_counts) {
        SCOPED_TRACE(blocks);
        CHECK_ABI(aes_hw_cbc_encrypt, buf, buf, AES_BLOCK_SIZE * blocks, &key,
                  block, AES_DECRYPT);
#if defined(HWAES_ECB)
        CHECK_ABI(aes_hw_ecb_encrypt, buf, buf, AES_BLOCK_SIZE * blocks, &key,
                  AES_DECRYPT);
#endif
      }
    }
  }
}
#endif  // SUPPORTS_ABI_TEST

#if defined(BSAES) && !defined(BORINGSSL_SHARED_LIBRARY)
static Bytes AESKeyToBytes(const AES_KEY *key) {
  return Bytes(reinterpret_cast<const uint8_t *>(key), sizeof(*key));
}

static uint8_t aes_ref_sub_byte(uint8_t b) {
  static const uint8_t kSBox[256] = {
      0x63, 0x7c, 0x77, 0x7b, 0xf2, 0x6b, 0x6f, 0xc5, 0x30, 0x01, 0x67, 0x2b,
      0xfe, 0xd7, 0xab, 0x76, 0xca, 0x82, 0xc9, 0x7d, 0xfa, 0x59, 0x47, 0xf0,
      0xad, 0xd4, 0xa2, 0xaf, 0x9c, 0xa4, 0x72, 0xc0, 0xb7, 0xfd, 0x93, 0x26,
      0x36, 0x3f, 0xf7, 0xcc, 0x34, 0xa5, 0xe5, 0xf1, 0x71, 0xd8, 0x31, 0x15,
      0x04, 0xc7, 0x23, 0xc3, 0x18, 0x96, 0x05, 0x9a, 0x07, 0x12, 0x80, 0xe2,
      0xeb, 0x27, 0xb2, 0x75, 0x09, 0x83, 0x2c, 0x1a, 0x1b, 0x6e, 0x5a, 0xa0,
      0x52, 0x3b, 0xd6, 0xb3, 0x29, 0xe3, 0x2f, 0x84, 0x53, 0xd1, 0x00, 0xed,
      0x20, 0xfc, 0xb1, 0x5b, 0x6a, 0xcb, 0xbe, 0x39, 0x4a, 0x4c, 0x58, 0xcf,
      0xd0, 0xef, 0xaa, 0xfb, 0x43, 0x4d, 0x33, 0x85, 0x45, 0xf9, 0x02, 0x7f,
      0x50, 0x3c, 0x9f, 0xa8, 0x51, 0xa3, 0x40, 0x8f, 0x92, 0x9d, 0x38, 0xf5,
      0xbc, 0xb6, 0xda, 0x21, 0x10, 0xff, 0xf3, 0xd2, 0xcd, 0x0c, 0x13, 0xec,
      0x5f, 0x97, 0x44, 0x17, 0xc4, 0xa7, 0x7e, 0x3d, 0x64, 0x5d, 0x19, 0x73,
      0x60, 0x81, 0x4f, 0xdc, 0x22, 0x2a, 0x90, 0x88, 0x46, 0xee, 0xb8, 0x14,
      0xde, 0x5e, 0x0b, 0xdb, 0xe0, 0x32, 0x3a, 0x0a, 0x49, 0x06, 0x24, 0x5c,
      0xc2, 0xd3, 0xac, 0x62, 0x91, 0x95, 0xe4, 0x79, 0xe7, 0xc8, 0x37, 0x6d,
      0x8d, 0xd5, 0x4e, 0xa9, 0x6c, 0x56, 0xf4, 0xea, 0x65, 0x7a, 0xae, 0x08,
      0xba, 0x78, 0x25, 0x2e, 0x1c, 0xa6, 0xb4, 0xc6, 0xe8, 0xdd, 0x74, 0x1f,
      0x4b, 0xbd, 0x8b, 0x8a, 0x70, 0x3e, 0xb5, 0x66, 0x48, 0x03, 0xf6, 0x0e,
      0x61, 0x35, 0x57, 0xb9, 0x86, 0xc1, 0x1d, 0x9e, 0xe1, 0xf8, 0x98, 0x11,
      0x69, 0xd9, 0x8e, 0x94, 0x9b, 0x1e, 0x87, 0xe9, 0xce, 0x55, 0x28, 0xdf,
      0x8c, 0xa1, 0x89, 0x0d, 0xbf, 0xe6, 0x42, 0x68, 0x41, 0x99, 0x2d, 0x0f,
      0xb0, 0x54, 0xbb, 0x16,
  };
  return kSBox[b];
}

static uint32_t aes_ref_sub_word(uint32_t in) {
  uint32_t a0 = aes_ref_sub_byte(in);
  uint32_t a1 = aes_ref_sub_byte(in >> 8);
  uint32_t a2 = aes_ref_sub_byte(in >> 16);
  uint32_t a3 = aes_ref_sub_byte(in >> 24);
  return a0 | (a1 << 8) | (a2 << 16) | (a3 << 24);
}

static int aes_ref_set_encrypt_key(const uint8_t *key, int key_bits,
                                   AES_KEY *out) {
  static const uint32_t kRCon[10] = {0x01, 0x02, 0x04, 0x08, 0x10,
                                     0x20, 0x40, 0x80, 0x1b, 0x36};
  switch (key_bits) {
    case 128:
      out->rounds = 10;
      break;
    case 192:
      out->rounds = 12;
      break;
    case 256:
      out->rounds = 14;
      break;
    default:
      return 1;
  }

  size_t words = key_bits / 32;
  size_t num_subkey_words = (out->rounds + 1) * 4;
  OPENSSL_memcpy(out->rd_key, key, words * sizeof(uint32_t));
  for (size_t i = words; i < num_subkey_words; i++) {
    uint32_t tmp = out->rd_key[i - 1];
    if (i % words == 0) {
      tmp = aes_ref_sub_word(CRYPTO_rotr_u32(tmp, 8)) ^ kRCon[(i / words) - 1];
    } else if (key_bits == 256 && i % 4 == 0) {
      tmp = aes_ref_sub_word(tmp);
    }
    out->rd_key[i] = tmp ^ out->rd_key[i - words];
  }

  // The ARM bsaes implementation expects all the keys to be byteswapped.
  for (size_t i = 0; i < num_subkey_words; i++) {
    out->rd_key[i] = CRYPTO_bswap4(out->rd_key[i]);
  }

  return 0;
}

static void aes_ref_inv_mix_columns(uint32_t block[4]) {
  // This tables was generated with the following Python script:
  // clang-format off
/*
def mul_unreduced(a, b):
  c = 0
  for i in range(8):
    if b & (1 << i):
      c ^= a << i
  return c

def mul(a, b):
  c = mul_unreduced(a, b)
  # c's highest term is at most x^14.
  c = (c & 0xff) ^ mul_unreduced(c >> 8, 0b00011011)
  # c's highest term is at most x^10.
  c = (c & 0xff) ^ mul_unreduced(c >> 8, 0b00011011)
  # c's highest term is at most x^7.
  assert (c >> 8) == 0
  return c

def inv_mix_column(a):
  ret = 0
  for b in [0x0e, 0x09, 0x0d, 0x0b]:
    ret <<= 8
    ret |= mul(a, b)
  return ret

body = ", ".join("0x%08x" % inv_mix_column(a) for a in range(256))
print("static const uint32_t kTable[256] = {%s};\n" % body)
*/
  // clang-format on

  // kInvMixColumn[i] is the result of InvMixColumns applied to a column
  // containing [i, 0, 0, 0]. (The contributions of the other positions are
  // computed by rotating bytes.)
  static const uint32_t kInvMixColumn[256] = {
      0x00000000, 0x0e090d0b, 0x1c121a16, 0x121b171d, 0x3824342c, 0x362d3927,
      0x24362e3a, 0x2a3f2331, 0x70486858, 0x7e416553, 0x6c5a724e, 0x62537f45,
      0x486c5c74, 0x4665517f, 0x547e4662, 0x5a774b69, 0xe090d0b0, 0xee99ddbb,
      0xfc82caa6, 0xf28bc7ad, 0xd8b4e49c, 0xd6bde997, 0xc4a6fe8a, 0xcaaff381,
      0x90d8b8e8, 0x9ed1b5e3, 0x8ccaa2fe, 0x82c3aff5, 0xa8fc8cc4, 0xa6f581cf,
      0xb4ee96d2, 0xbae79bd9, 0xdb3bbb7b, 0xd532b670, 0xc729a16d, 0xc920ac66,
      0xe31f8f57, 0xed16825c, 0xff0d9541, 0xf104984a, 0xab73d323, 0xa57ade28,
      0xb761c935, 0xb968c43e, 0x9357e70f, 0x9d5eea04, 0x8f45fd19, 0x814cf012,
      0x3bab6bcb, 0x35a266c0, 0x27b971dd, 0x29b07cd6, 0x038f5fe7, 0x0d8652ec,
      0x1f9d45f1, 0x119448fa, 0x4be30393, 0x45ea0e98, 0x57f11985, 0x59f8148e,
      0x73c737bf, 0x7dce3ab4, 0x6fd52da9, 0x61dc20a2, 0xad766df6, 0xa37f60fd,
      0xb16477e0, 0xbf6d7aeb, 0x955259da, 0x9b5b54d1, 0x894043cc, 0x87494ec7,
      0xdd3e05ae, 0xd33708a5, 0xc12c1fb8, 0xcf2512b3, 0xe51a3182, 0xeb133c89,
      0xf9082b94, 0xf701269f, 0x4de6bd46, 0x43efb04d, 0x51f4a750, 0x5ffdaa5b,
      0x75c2896a, 0x7bcb8461, 0x69d0937c, 0x67d99e77, 0x3daed51e, 0x33a7d815,
      0x21bccf08, 0x2fb5c203, 0x058ae132, 0x0b83ec39, 0x1998fb24, 0x1791f62f,
      0x764dd68d, 0x7844db86, 0x6a5fcc9b, 0x6456c190, 0x4e69e2a1, 0x4060efaa,
      0x527bf8b7, 0x5c72f5bc, 0x0605bed5, 0x080cb3de, 0x1a17a4c3, 0x141ea9c8,
      0x3e218af9, 0x302887f2, 0x223390ef, 0x2c3a9de4, 0x96dd063d, 0x98d40b36,
      0x8acf1c2b, 0x84c61120, 0xaef93211, 0xa0f03f1a, 0xb2eb2807, 0xbce2250c,
      0xe6956e65, 0xe89c636e, 0xfa877473, 0xf48e7978, 0xdeb15a49, 0xd0b85742,
      0xc2a3405f, 0xccaa4d54, 0x41ecdaf7, 0x4fe5d7fc, 0x5dfec0e1, 0x53f7cdea,
      0x79c8eedb, 0x77c1e3d0, 0x65daf4cd, 0x6bd3f9c6, 0x31a4b2af, 0x3fadbfa4,
      0x2db6a8b9, 0x23bfa5b2, 0x09808683, 0x07898b88, 0x15929c95, 0x1b9b919e,
      0xa17c0a47, 0xaf75074c, 0xbd6e1051, 0xb3671d5a, 0x99583e6b, 0x97513360,
      0x854a247d, 0x8b432976, 0xd134621f, 0xdf3d6f14, 0xcd267809, 0xc32f7502,
      0xe9105633, 0xe7195b38, 0xf5024c25, 0xfb0b412e, 0x9ad7618c, 0x94de6c87,
      0x86c57b9a, 0x88cc7691, 0xa2f355a0, 0xacfa58ab, 0xbee14fb6, 0xb0e842bd,
      0xea9f09d4, 0xe49604df, 0xf68d13c2, 0xf8841ec9, 0xd2bb3df8, 0xdcb230f3,
      0xcea927ee, 0xc0a02ae5, 0x7a47b13c, 0x744ebc37, 0x6655ab2a, 0x685ca621,
      0x42638510, 0x4c6a881b, 0x5e719f06, 0x5078920d, 0x0a0fd964, 0x0406d46f,
      0x161dc372, 0x1814ce79, 0x322bed48, 0x3c22e043, 0x2e39f75e, 0x2030fa55,
      0xec9ab701, 0xe293ba0a, 0xf088ad17, 0xfe81a01c, 0xd4be832d, 0xdab78e26,
      0xc8ac993b, 0xc6a59430, 0x9cd2df59, 0x92dbd252, 0x80c0c54f, 0x8ec9c844,
      0xa4f6eb75, 0xaaffe67e, 0xb8e4f163, 0xb6edfc68, 0x0c0a67b1, 0x02036aba,
      0x10187da7, 0x1e1170ac, 0x342e539d, 0x3a275e96, 0x283c498b, 0x26354480,
      0x7c420fe9, 0x724b02e2, 0x605015ff, 0x6e5918f4, 0x44663bc5, 0x4a6f36ce,
      0x587421d3, 0x567d2cd8, 0x37a10c7a, 0x39a80171, 0x2bb3166c, 0x25ba1b67,
      0x0f853856, 0x018c355d, 0x13972240, 0x1d9e2f4b, 0x47e96422, 0x49e06929,
      0x5bfb7e34, 0x55f2733f, 0x7fcd500e, 0x71c45d05, 0x63df4a18, 0x6dd64713,
      0xd731dcca, 0xd938d1c1, 0xcb23c6dc, 0xc52acbd7, 0xef15e8e6, 0xe11ce5ed,
      0xf307f2f0, 0xfd0efffb, 0xa779b492, 0xa970b999, 0xbb6bae84, 0xb562a38f,
      0x9f5d80be, 0x91548db5, 0x834f9aa8, 0x8d4697a3};

  // Note |block| is byte-swapped so block[i] >> 24 is the first element of
  // block[i]. (See |aes_ref_set_encrypt_key|).
  for (size_t i = 0; i < 4; i++) {
    uint32_t in = block[i];
    block[i] = kInvMixColumn[in >> 24];
    block[i] ^= CRYPTO_rotr_u32(kInvMixColumn[(in >> 16) & 0xff], 8);
    block[i] ^= CRYPTO_rotr_u32(kInvMixColumn[(in >> 8) & 0xff], 16);
    block[i] ^= CRYPTO_rotr_u32(kInvMixColumn[in & 0xff], 24);
  }
}

static int aes_ref_set_decrypt_key(const uint8_t *key, int bits, AES_KEY *out) {
  if (aes_ref_set_encrypt_key(key, bits, out) != 0) {
    return 1;
  }

  // bsaes expects the decryption round keys in reverse order. Note there are
  // |out->rounds + 1| round keys.
  for (size_t i = 0; i < out->rounds / 2; i++) {
    std::swap(out->rd_key[4 * i], out->rd_key[4 * (out->rounds - i)]);
    std::swap(out->rd_key[4 * i + 1], out->rd_key[4 * (out->rounds - i) + 1]);
    std::swap(out->rd_key[4 * i + 2], out->rd_key[4 * (out->rounds - i) + 2]);
    std::swap(out->rd_key[4 * i + 3], out->rd_key[4 * (out->rounds - i) + 3]);
  }

  // bsaes expects round keys other than the first and last to have
  // InvMixColumns applied.
  for (size_t i = 1; i < out->rounds; i++) {
    aes_ref_inv_mix_columns(out->rd_key + 4 * i);
  }

  return 0;
}


TEST(AESTest, VPAESToBSAESConvert) {
  if (!vpaes_capable()) {
    GTEST_SKIP();
  }

  const int kNumIterations = 1000;
  for (int i = 0; i < kNumIterations; i++) {
    uint8_t key[256 / 8];
    RAND_bytes(key, sizeof(key));
    SCOPED_TRACE(Bytes(key));
    for (unsigned bits : {128u, 192u, 256u}) {
      SCOPED_TRACE(bits);
      for (bool enc : {false, true}) {
        SCOPED_TRACE(enc);
        AES_KEY ref, vpaes, bsaes;
        OPENSSL_memset(&ref, 0xaa, sizeof(ref));
        OPENSSL_memset(&vpaes, 0xaa, sizeof(vpaes));
        OPENSSL_memset(&bsaes, 0xaa, sizeof(bsaes));

        if (enc) {
          ASSERT_EQ(0, aes_ref_set_encrypt_key(key, bits, &ref));
          ASSERT_EQ(0, vpaes_set_encrypt_key(key, bits, &vpaes));
          vpaes_encrypt_key_to_bsaes(&bsaes, &vpaes);
        } else {
          ASSERT_EQ(0, aes_ref_set_decrypt_key(key, bits, &ref));
          ASSERT_EQ(0, vpaes_set_decrypt_key(key, bits, &vpaes));
          vpaes_decrypt_key_to_bsaes(&bsaes, &vpaes);
        }

        // Although not fatal, stop running if this fails, otherwise we'll spam
        // the user's console.
        ASSERT_EQ(AESKeyToBytes(&ref), AESKeyToBytes(&bsaes));

        // Repeat the test in-place.
        OPENSSL_memcpy(&bsaes, &vpaes, sizeof(AES_KEY));
        if (enc) {
          vpaes_encrypt_key_to_bsaes(&bsaes, &bsaes);
        } else {
          vpaes_decrypt_key_to_bsaes(&bsaes, &bsaes);
        }

        ASSERT_EQ(AESKeyToBytes(&ref), AESKeyToBytes(&bsaes));
      }
    }
  }
}
#endif  // BSAES && !SHARED_LIBRARY
