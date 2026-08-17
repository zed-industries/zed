// Written by Dr Stephen N Henson (steve@openssl.org) for the OpenSSL project.
// Copyright (c) 2015 The OpenSSL Project.  All rights reserved.
// SPDX-License-Identifier: Apache-2.0

#include <limits.h>
#include <stdlib.h>
#include <string.h>

#include <algorithm>
#include <string>
#include <vector>

#include <gtest/gtest.h>

#include <openssl/aes.h>
#include <openssl/cipher.h>
#include <openssl/err.h>
#include <openssl/nid.h>
#include <openssl/rand.h>
#include <openssl/sha.h>
#include <openssl/span.h>

#include "../fipsmodule/cipher/internal.h"
#include "../internal.h"
#include "../test/file_test.h"
#include "../test/test_util.h"
#include "../test/wycheproof_util.h"
#include "./internal.h"


static const EVP_CIPHER *GetCipher(const std::string &name) {
  if (name == "DES-CBC") {
    return EVP_des_cbc();
  } else if (name == "DES-ECB") {
    return EVP_des_ecb();
  } else if (name == "DES-EDE") {
    return EVP_des_ede();
  } else if (name == "DES-EDE3") {
    return EVP_des_ede3();
  } else if (name == "DES-EDE-CBC") {
    return EVP_des_ede_cbc();
  } else if (name == "DES-EDE3-CBC") {
    return EVP_des_ede3_cbc();
  } else if (name == "RC4") {
    return EVP_rc4();
  } else if (name == "AES-128-ECB") {
    return EVP_aes_128_ecb();
  } else if (name == "AES-256-ECB") {
    return EVP_aes_256_ecb();
  } else if (name == "AES-128-CBC") {
    return EVP_aes_128_cbc();
  } else if (name == "AES-128-GCM") {
    return EVP_aes_128_gcm();
  } else if (name == "AES-128-OFB") {
    return EVP_aes_128_ofb();
  } else if (name == "AES-192-CBC") {
    return EVP_aes_192_cbc();
  } else if (name == "AES-192-CTR") {
    return EVP_aes_192_ctr();
  } else if (name == "AES-192-ECB") {
    return EVP_aes_192_ecb();
  } else if (name == "AES-192-GCM") {
    return EVP_aes_192_gcm();
  } else if (name == "AES-192-OFB") {
    return EVP_aes_192_ofb();
  } else if (name == "AES-256-CBC") {
    return EVP_aes_256_cbc();
  } else if (name == "AES-128-CTR") {
    return EVP_aes_128_ctr();
  } else if (name == "AES-256-CTR") {
    return EVP_aes_256_ctr();
  } else if (name == "AES-256-GCM") {
    return EVP_aes_256_gcm();
  } else if (name == "AES-256-OFB") {
    return EVP_aes_256_ofb();
  } else if (name == "AES-128-CCM") {
    return EVP_aes_128_ccm();
  } else if (name == "AES-192-CCM") {
    return EVP_aes_192_ccm();
  } else if (name == "AES-256-CCM") {
    return EVP_aes_256_ccm();
  }
  return nullptr;
}

enum class Operation {
  // kBoth tests both encryption and decryption.
  kBoth,
  // kEncrypt tests encryption. The result of encryption should always
  // successfully decrypt, so this should only be used if the test file has a
  // matching decrypt-only vector.
  kEncrypt,
  // kDecrypt tests decryption. This should only be used if the test file has a
  // matching encrypt-only input, or if multiple ciphertexts are valid for
  // a given plaintext and this is a non-canonical ciphertext.
  kDecrypt,
  // kInvalidDecrypt tests decryption and expects it to fail, e.g. due to
  // invalid tag or padding.
  kInvalidDecrypt,
};

static const char *OperationToString(Operation op) {
  switch (op) {
    case Operation::kBoth:
      return "Both";
    case Operation::kEncrypt:
      return "Encrypt";
    case Operation::kDecrypt:
      return "Decrypt";
    case Operation::kInvalidDecrypt:
      return "InvalidDecrypt";
  }
  abort();
}

// MaybeCopyCipherContext, if |copy| is true, replaces |*ctx| with a, hopefully
// equivalent, copy of it.
static bool MaybeCopyCipherContext(bool copy,
                                   bssl::UniquePtr<EVP_CIPHER_CTX> *ctx) {
  if (!copy) {
    return true;
  }
  bssl::UniquePtr<EVP_CIPHER_CTX> ctx2(EVP_CIPHER_CTX_new());
  if (!ctx2 || !EVP_CIPHER_CTX_copy(ctx2.get(), ctx->get())) {
    return false;
  }
  *ctx = std::move(ctx2);
  return true;
}

static void TestCipherAPI(const EVP_CIPHER *cipher, Operation op, bool padding,
                          bool copy, bool in_place, bool use_evp_cipher,
                          size_t chunk_size, bssl::Span<const uint8_t> key,
                          bssl::Span<const uint8_t> iv,
                          bssl::Span<const uint8_t> plaintext,
                          bssl::Span<const uint8_t> ciphertext,
                          bssl::Span<const uint8_t> aad,
                          bssl::Span<const uint8_t> tag) {
  bool encrypt = op == Operation::kEncrypt;
  bool is_custom_cipher =
      EVP_CIPHER_flags(cipher) & EVP_CIPH_FLAG_CUSTOM_CIPHER;
  bssl::Span<const uint8_t> in = encrypt ? plaintext : ciphertext;
  bssl::Span<const uint8_t> expected = encrypt ? ciphertext : plaintext;
  bool is_aead = EVP_CIPHER_flags(cipher) & EVP_CIPH_FLAG_AEAD_CIPHER;
  bool is_ccm = EVP_CIPHER_mode(cipher) == EVP_CIPH_CCM_MODE;

  // Some |EVP_CIPHER|s take a variable-length key, and need to first be
  // configured with the key length, which requires configuring the cipher.
  bssl::UniquePtr<EVP_CIPHER_CTX> ctx(EVP_CIPHER_CTX_new());
  ASSERT_TRUE(ctx);
  ASSERT_TRUE(EVP_CipherInit_ex(ctx.get(), cipher, /*engine=*/nullptr,
                                /*key=*/nullptr, /*iv=*/nullptr,
                                encrypt ? 1 : 0));
  ASSERT_TRUE(EVP_CIPHER_CTX_set_key_length(ctx.get(), key.size()));
  if (!padding) {
    ASSERT_TRUE(EVP_CIPHER_CTX_set_padding(ctx.get(), 0));
  }

  // Configure the key.
  ASSERT_TRUE(MaybeCopyCipherContext(copy, &ctx));
  ASSERT_TRUE(EVP_CipherInit_ex(ctx.get(), /*cipher=*/nullptr,
                                /*engine=*/nullptr, key.data(), /*iv=*/nullptr,
                                /*enc=*/-1));

  // Configure the IV to run the actual operation. Callers that wish to use a
  // key for multiple, potentially concurrent, operations will likely copy at
  // this point. The |EVP_CIPHER_CTX| API uses the same type to represent a
  // pre-computed key schedule and a streaming operation.
  ASSERT_TRUE(MaybeCopyCipherContext(copy, &ctx));
  if (is_aead) {
    ASSERT_LE(iv.size(), size_t{INT_MAX});
    ASSERT_TRUE(EVP_CIPHER_CTX_ctrl(ctx.get(), EVP_CTRL_AEAD_SET_IVLEN,
                                    static_cast<int>(iv.size()), nullptr));
    ASSERT_EQ(EVP_CIPHER_CTX_iv_length(ctx.get()), iv.size());
  } else {
    ASSERT_EQ(iv.size(), EVP_CIPHER_CTX_iv_length(ctx.get()));
  }
  // CCM needs tag length (M) set via EVP_CTRL_AEAD_SET_TAG during encryption.
  if ((is_aead && !encrypt) || is_ccm) {
    ASSERT_TRUE(EVP_CIPHER_CTX_ctrl(ctx.get(), EVP_CTRL_AEAD_SET_TAG,
                                    tag.size(), encrypt ? nullptr
                                                        : const_cast<uint8_t *>(tag.data())));
  }
  ASSERT_TRUE(EVP_CipherInit_ex(ctx.get(), /*cipher=*/nullptr,
                                /*engine=*/nullptr,
                                /*key=*/nullptr, iv.data(), /*enc=*/-1));

  // CCM requires the full length of the plaintext to be known ahead of time.
  if (is_ccm) {
    int len;
    if (use_evp_cipher) {
      len = EVP_Cipher(ctx.get(), nullptr, nullptr, in.size());
    } else {
      ASSERT_TRUE(EVP_CipherUpdate(ctx.get(), nullptr, &len, nullptr,
                                   in.size()));
    }
    ASSERT_EQ(len, (int) in.size());
  }

  // Note: the deprecated |EVP_CIPHER|-based AEAD API is sensitive to whether
  // parameters are NULL, so it is important to skip the |in| and |aad|
  // |EVP_CipherUpdate| calls when empty.
  while (!aad.empty()) {
    size_t todo =
        chunk_size == 0 ? aad.size() : std::min(aad.size(), chunk_size);
    if (use_evp_cipher) {
      // AEADs always use the "custom cipher" return value convention. Passing a
      // null output pointer triggers the AAD logic.
      ASSERT_TRUE(is_custom_cipher);
      ASSERT_EQ(static_cast<int>(todo),
                EVP_Cipher(ctx.get(), nullptr, aad.data(), todo));
    } else {
      int len;
      ASSERT_TRUE(EVP_CipherUpdate(ctx.get(), nullptr, &len, aad.data(), todo));
      // Although it doesn't output anything, |EVP_CipherUpdate| should claim to
      // output the input length.
      EXPECT_EQ(len, static_cast<int>(todo));
    }
    aad = aad.subspan(todo);
  }

  // Set up the output buffer.
  size_t max_out = in.size();
  size_t block_size = EVP_CIPHER_CTX_block_size(ctx.get());
  if (block_size > 1 &&
      (EVP_CIPHER_CTX_flags(ctx.get()) & EVP_CIPH_NO_PADDING) == 0 &&
      EVP_CIPHER_CTX_encrypting(ctx.get())) {
    max_out += block_size - (max_out % block_size);
  }
  std::vector<uint8_t> result;
  max_out ? result.resize(max_out) : result.reserve(1);
  if (in_place) {
    std::copy(in.begin(), in.end(), result.begin());
    in = bssl::MakeConstSpan(result).first(in.size());
  }

  // An idiosyncrasy of OpenSSL's AES-CCM implementation is that it checks the
  // tag during the one and only |EVP_CipherUpdate| so we have to handle that.
  bool is_invalid_ccm = is_ccm && op == Operation::kInvalidDecrypt;

  size_t total = 0;
  int len;
  do {
    size_t todo = chunk_size == 0 ? in.size() : std::min(in.size(), chunk_size);
    EXPECT_LE(todo, static_cast<size_t>(INT_MAX));
    ASSERT_TRUE(MaybeCopyCipherContext(copy, &ctx));
    if (use_evp_cipher) {
      // |EVP_Cipher| sometimes returns the number of bytes written, or -1 on
      // error, and sometimes 1 or 0, implicitly writing |in_len| bytes.
      if (is_custom_cipher) {
        len = EVP_Cipher(ctx.get(), result.data() + total, in.data(), todo);
        if (is_invalid_ccm) {
          ASSERT_EQ(len, -1);
          break;
        }
      } else {
        ASSERT_EQ(
            1, EVP_Cipher(ctx.get(), result.data() + total, in.data(), todo));
        len = static_cast<int>(todo);
      }
    } else {
      int expected_ret = is_invalid_ccm ? 0 : 1;
      ASSERT_EQ(expected_ret, EVP_CipherUpdate(ctx.get(), result.data() + total,
                                               &len, in.data(),
                                               static_cast<int>(todo)));
    }
    ASSERT_GE(len, 0);
    total += static_cast<size_t>(len);
    in = in.subspan(todo);
  } while (!in.empty());
  len = -1;
  if (op == Operation::kInvalidDecrypt) {
    if (use_evp_cipher) {
      // Only the "custom cipher" return value convention can report failures.
      // Passing all nulls should act like |EVP_CipherFinal_ex|.
      ASSERT_TRUE(is_custom_cipher);
      int expected_ret = is_ccm ? 0 : -1;
      EXPECT_EQ(expected_ret, EVP_Cipher(ctx.get(), result.data() + total,
                                         nullptr, 0));
    } else {
      // Invalid padding and invalid tags all appear as a failed
      // |EVP_CipherFinal_ex|. In CCM, this happens in |EVP_CipherUpdate|.
      int expected_ret = is_ccm ? 1 : 0;
      EXPECT_EQ(expected_ret, EVP_CipherFinal_ex(ctx.get(),
                                                 result.data() + total, &len));
    }
  } else {
    if (use_evp_cipher) {
      if (is_custom_cipher) {
        // Only the "custom cipher" convention has an |EVP_CipherFinal_ex|
        // equivalent.
        len = EVP_Cipher(ctx.get(), result.data() + total, nullptr, 0);
      } else {
        len = 0;
      }
    } else {
      ASSERT_TRUE(EVP_CipherFinal_ex(ctx.get(), result.data() + total, &len));
    }
    ASSERT_GE(len, 0);
    total += static_cast<size_t>(len);
    result.resize(total);
    EXPECT_EQ(Bytes(expected), Bytes(result));
    if (encrypt && is_aead) {
      uint8_t rtag[16];
      ASSERT_LE(tag.size(), sizeof(rtag));
      ASSERT_TRUE(MaybeCopyCipherContext(copy, &ctx));
      ASSERT_TRUE(EVP_CIPHER_CTX_ctrl(ctx.get(), EVP_CTRL_AEAD_GET_TAG,
                                      tag.size(), rtag));
      EXPECT_EQ(Bytes(tag), Bytes(rtag, tag.size()));
    }
  }
}

static void TestLowLevelAPI(
    const EVP_CIPHER *cipher, Operation op, bool in_place, size_t chunk_size,
    bssl::Span<const uint8_t> key, bssl::Span<const uint8_t> iv,
    bssl::Span<const uint8_t> plaintext, bssl::Span<const uint8_t> ciphertext) {
  bool encrypt = op == Operation::kEncrypt;
  bssl::Span<const uint8_t> in = encrypt ? plaintext : ciphertext;
  bssl::Span<const uint8_t> expected = encrypt ? ciphertext : plaintext;
  int nid = EVP_CIPHER_nid(cipher);
  bool is_ctr = nid == NID_aes_128_ctr || nid == NID_aes_192_ctr ||
                nid == NID_aes_256_ctr;
  bool is_cbc = nid == NID_aes_128_cbc || nid == NID_aes_192_cbc ||
                nid == NID_aes_256_cbc;
  bool is_ofb = nid == NID_aes_128_ofb128 || nid == NID_aes_192_ofb128 ||
                nid == NID_aes_256_ofb128;
  if (!is_ctr && !is_cbc && !is_ofb) {
    return;
  }

  // Invalid ciphertexts are not possible in any of the ciphers where this API
  // applies.
  ASSERT_NE(op, Operation::kInvalidDecrypt);

  AES_KEY aes;
  if (encrypt || !is_cbc) {
    ASSERT_EQ(0, AES_set_encrypt_key(key.data(), key.size() * 8, &aes));
  } else {
    ASSERT_EQ(0, AES_set_decrypt_key(key.data(), key.size() * 8, &aes));
  }

  std::vector<uint8_t> result;
  if (in_place) {
    result.assign(in.begin(), in.end());
  } else {
    result.resize(expected.size());
  }
  bssl::Span<uint8_t> out = bssl::MakeSpan(result);
  // Input and output sizes for all the low-level APIs should match.
  ASSERT_EQ(in.size(), out.size());

  // The low-level APIs all use block-size IVs.
  ASSERT_EQ(iv.size(), size_t{AES_BLOCK_SIZE});
  uint8_t ivec[AES_BLOCK_SIZE];
  OPENSSL_memcpy(ivec, iv.data(), iv.size());

  if (is_ctr) {
    unsigned num = 0;
    uint8_t ecount_buf[AES_BLOCK_SIZE];
    if (chunk_size == 0) {
      AES_ctr128_encrypt(in.data(), out.data(), in.size(), &aes, ivec,
                         ecount_buf, &num);
    } else {
      do {
        size_t todo = std::min(in.size(), chunk_size);
        AES_ctr128_encrypt(in.data(), out.data(), todo, &aes, ivec, ecount_buf,
                           &num);
        in = in.subspan(todo);
        out = out.subspan(todo);
      } while (!in.empty());
    }
    EXPECT_EQ(Bytes(expected), Bytes(result));
  } else if (is_cbc && chunk_size % AES_BLOCK_SIZE == 0) {
    // Note |AES_cbc_encrypt| requires block-aligned chunks.
    if (chunk_size == 0) {
      AES_cbc_encrypt(in.data(), out.data(), in.size(), &aes, ivec, encrypt);
    } else {
      do {
        size_t todo = std::min(in.size(), chunk_size);
        AES_cbc_encrypt(in.data(), out.data(), todo, &aes, ivec, encrypt);
        in = in.subspan(todo);
        out = out.subspan(todo);
      } while (!in.empty());
    }
    EXPECT_EQ(Bytes(expected), Bytes(result));
  } else if (is_ofb) {
    int num = 0;
    if (chunk_size == 0) {
      AES_ofb128_encrypt(in.data(), out.data(), in.size(), &aes, ivec, &num);
    } else {
      do {
        size_t todo = std::min(in.size(), chunk_size);
        AES_ofb128_encrypt(in.data(), out.data(), todo, &aes, ivec, &num);
        in = in.subspan(todo);
        out = out.subspan(todo);
      } while (!in.empty());
    }
    EXPECT_EQ(Bytes(expected), Bytes(result));
  }
}

static void TestCipher(const EVP_CIPHER *cipher, Operation input_op,
                       bool padding, bssl::Span<const uint8_t> key,
                       bssl::Span<const uint8_t> iv,
                       bssl::Span<const uint8_t> plaintext,
                       bssl::Span<const uint8_t> ciphertext,
                       bssl::Span<const uint8_t> aad,
                       bssl::Span<const uint8_t> tag) {
  size_t block_size = EVP_CIPHER_block_size(cipher);
  bool is_ccm = EVP_CIPHER_mode(cipher) == EVP_CIPH_CCM_MODE;
  std::vector<Operation> ops;
  if (input_op == Operation::kBoth) {
    ops = {Operation::kEncrypt, Operation::kDecrypt};
  } else {
    ops = {input_op};
  }
  for (Operation op : ops) {
    SCOPED_TRACE(OperationToString(op));
    // Zero indicates a single-shot API.
    static const size_t kChunkSizes[] = {0,  1,  2,  5,  7,  8,  9,  15, 16,
                                         17, 31, 32, 33, 63, 64, 65, 512};
    for (size_t chunk_size : kChunkSizes) {
      SCOPED_TRACE(chunk_size);
      if (chunk_size > plaintext.size() && chunk_size > ciphertext.size() &&
          chunk_size > aad.size()) {
        continue;
      }
      // CCM only supports single-shot operations.
      if (is_ccm && chunk_size != 0) {
        continue;
      }
      for (bool in_place : {false, true}) {
        SCOPED_TRACE(in_place);
        for (bool copy : {false, true}) {
          SCOPED_TRACE(copy);
          TestCipherAPI(cipher, op, padding, copy, in_place,
                        /*use_evp_cipher=*/false, chunk_size, key, iv,
                        plaintext, ciphertext, aad, tag);
          if (!padding && chunk_size % block_size == 0) {
            TestCipherAPI(cipher, op, padding, copy, in_place,
                          /*use_evp_cipher=*/true, chunk_size, key, iv,
                          plaintext, ciphertext, aad, tag);
          }
        }
        if (!padding) {
          TestLowLevelAPI(cipher, op, in_place, chunk_size, key, iv, plaintext,
                          ciphertext);
        }
      }
    }
  }
}

static void CipherFileTest(FileTest *t) {
  std::string cipher_str;
  ASSERT_TRUE(t->GetAttribute(&cipher_str, "Cipher"));
  const EVP_CIPHER *cipher = GetCipher(cipher_str);
  ASSERT_TRUE(cipher);

  std::vector<uint8_t> key, iv, plaintext, ciphertext, aad, tag;
  // Force an allocation of the underlying data-store so that v.data() is
  // non-NULL even for empty test vectors.
  plaintext.reserve(1);
  ciphertext.reserve(1);
  ASSERT_TRUE(t->GetBytes(&key, "Key"));
  ASSERT_TRUE(t->GetBytes(&plaintext, "Plaintext"));
  ASSERT_TRUE(t->GetBytes(&ciphertext, "Ciphertext"));
  if (EVP_CIPHER_iv_length(cipher) > 0) {
    ASSERT_TRUE(t->GetBytes(&iv, "IV"));
  }
  if (EVP_CIPHER_flags(cipher) & EVP_CIPH_FLAG_AEAD_CIPHER) {
    ASSERT_TRUE(t->GetBytes(&aad, "AAD"));
    ASSERT_TRUE(t->GetBytes(&tag, "Tag"));
  }

  Operation op = Operation::kBoth;
  if (t->HasAttribute("Operation")) {
    const std::string &str = t->GetAttributeOrDie("Operation");
    if (str == "Encrypt" || str == "ENCRYPT") {
      op = Operation::kEncrypt;
    } else if (str == "Decrypt" || str == "DECRYPT") {
      op = Operation::kDecrypt;
    } else if (str == "InvalidDecrypt") {
      op = Operation::kInvalidDecrypt;
    } else {
      FAIL() << "Unknown operation: " << str;
    }
  }

  TestCipher(cipher, op, /*padding=*/false, key, iv, plaintext, ciphertext, aad,
             tag);
}

TEST(CipherTest, TestVectors) {
  FileTestGTest("crypto/cipher_extra/test/cipher_tests.txt", CipherFileTest);
}

TEST(CipherTest, AES_CCM) {
  FileTestGTest("crypto/cipher_extra/test/aes_ccm_test.txt", CipherFileTest);
}

TEST(CipherTest, CAVP_AES_128_CBC) {
  FileTestGTest("crypto/cipher_extra/test/nist_cavp/aes_128_cbc.txt",
                CipherFileTest);
}

TEST(CipherTest, CAVP_AES_128_CTR) {
  FileTestGTest("crypto/cipher_extra/test/nist_cavp/aes_128_ctr.txt",
                CipherFileTest);
}

TEST(CipherTest, CAVP_AES_192_CBC) {
  FileTestGTest("crypto/cipher_extra/test/nist_cavp/aes_192_cbc.txt",
                CipherFileTest);
}

TEST(CipherTest, CAVP_AES_192_CTR) {
  FileTestGTest("crypto/cipher_extra/test/nist_cavp/aes_192_ctr.txt",
                CipherFileTest);
}

TEST(CipherTest, CAVP_AES_256_CBC) {
  FileTestGTest("crypto/cipher_extra/test/nist_cavp/aes_256_cbc.txt",
                CipherFileTest);
}

TEST(CipherTest, CAVP_AES_256_CTR) {
  FileTestGTest("crypto/cipher_extra/test/nist_cavp/aes_256_ctr.txt",
                CipherFileTest);
}

TEST(CipherTest, CAVP_TDES_CBC) {
  FileTestGTest("crypto/cipher_extra/test/nist_cavp/tdes_cbc.txt",
                CipherFileTest);
}

TEST(CipherTest, CAVP_TDES_ECB) {
  FileTestGTest("crypto/cipher_extra/test/nist_cavp/tdes_ecb.txt",
                CipherFileTest);
}

struct AeadCipherParams {
  const char name[40];
  const EVP_CIPHER *(*func)(void);
  const char *test_vectors;
};

static const struct AeadCipherParams AeadCiphers[] = {
  {"ChaCha20Poly1305", EVP_chacha20_poly1305, "chacha20_poly1305_tests.txt"},
  {"AES_128_CCM_BLUETOOTH", EVP_aes_128_ccm, "aes_128_ccm_bluetooth_tests.txt"},
  {"AES_128_CCM_BLUETOOTH_8", EVP_aes_128_ccm,
   "aes_128_ccm_bluetooth_8_tests.txt"},
  {"AES_128_CCM_Matter", EVP_aes_128_ccm, "aes_128_ccm_matter_tests.txt"},
};

class AeadCipherTest : public testing::TestWithParam<AeadCipherParams> {
public:
  const EVP_CIPHER *getTestCipher() {
    return GetParam().func();
  }
};

INSTANTIATE_TEST_SUITE_P(All, AeadCipherTest, testing::ValuesIn(AeadCiphers),
                         [](const testing::TestParamInfo<AeadCipherParams> &params)
                         -> std::string { return params.param.name; });

TEST_P(AeadCipherTest, TestVector) {
  std::string test_vectors = "crypto/cipher_extra/test/";
  test_vectors += GetParam().test_vectors;
  FileTestGTest(test_vectors.c_str(), [&](FileTest *t) {
    const EVP_CIPHER *cipher = getTestCipher();
    ASSERT_TRUE(cipher);

    std::vector<uint8_t> key, iv, plaintext, ciphertext, aad, tag;
    ASSERT_TRUE(t->GetBytes(&key, "KEY"));
    ASSERT_TRUE(t->GetBytes(&plaintext, "IN"));
    ASSERT_TRUE(t->GetBytes(&ciphertext, "CT"));
    if (EVP_CIPHER_iv_length(cipher) > 0) {
      ASSERT_TRUE(t->GetBytes(&iv, "NONCE"));
    }
    ASSERT_TRUE(EVP_CIPHER_flags(cipher) & EVP_CIPH_FLAG_AEAD_CIPHER);
    ASSERT_TRUE(t->GetBytes(&aad, "AD"));
    ASSERT_TRUE(t->GetBytes(&tag, "TAG"));

    Operation op = Operation::kBoth;
    TestCipher(cipher, op, /*padding=*/false, key, iv, plaintext, ciphertext,
               aad, tag);
  });
}

// Given a size_t, return true if it's valid. These hardcoded validators are
// necessary because the Wychefproof test vectors are not consistent about
// setting the right validity flags.
typedef bool(*validate_f)(size_t);

static void WycheproofFileTest(FileTest *t, std::vector<uint8_t> key,
  std::vector<uint8_t> iv, std::vector<uint8_t> msg, std::vector<uint8_t> ct,
  std::vector<uint8_t> aad, std::vector<uint8_t> tag, WycheproofResult result,
  const EVP_CIPHER *cipher, validate_f iv_validate, validate_f tag_validate) {
  std::unique_ptr<EVP_CIPHER_CTX, decltype(&EVP_CIPHER_CTX_free)> uctx(
          EVP_CIPHER_CTX_new(), EVP_CIPHER_CTX_free);
  EVP_CIPHER_CTX *ctx = uctx.get();
  ASSERT_TRUE(ctx);

  bool is_ccm = EVP_CIPHER_mode(cipher) == EVP_CIPH_CCM_MODE;
  bool valid_iv = iv_validate(iv.size());
  bool valid_tag = tag_validate(tag.size());

  // Initialize without the key/iv
  ASSERT_TRUE(EVP_EncryptInit_ex(ctx, cipher, NULL, NULL, NULL));

  // Set the IV size
  int res;
  res = EVP_CIPHER_CTX_ctrl(ctx, EVP_CTRL_AEAD_SET_IVLEN, iv.size(), NULL);

  if (!valid_iv && !result.IsValid()) {
    // Check that we correctly reject the IV and skip.
    // Otherwise, the cipher will simply just use IV=0.
    ASSERT_EQ(res, 0);
    t->SkipCurrent();
    return;
  } else {
    ASSERT_EQ(res, 1);
  }

  // Set the tag size for CCM
  if (is_ccm) {
    res = EVP_CIPHER_CTX_ctrl(ctx, EVP_CTRL_AEAD_SET_TAG, tag.size(),
                              nullptr);
    if (!valid_tag && !result.IsValid()) {
      ASSERT_FALSE(res);
      t->SkipCurrent();
      return;
    } else {
      ASSERT_TRUE(res);
    }
  }

  // Initialize with the key/iv
  ASSERT_TRUE(EVP_EncryptInit_ex(ctx, nullptr, nullptr, key.data(), iv.data()));

  // Set the message length for CCM
  int out_len = 0;
  if (is_ccm) {
    ASSERT_TRUE(EVP_CipherUpdate(ctx, nullptr, &out_len, nullptr, msg.size()));
    ASSERT_EQ(out_len, (int) msg.size());
  }

  // Insert AAD
  uint8_t junk_buf[1];
  uint8_t *in = aad.empty() ? junk_buf : aad.data();
  ASSERT_TRUE(EVP_EncryptUpdate(ctx, /*out*/ nullptr, &out_len, in, aad.size()));
  ASSERT_EQ(out_len, (int) aad.size());

  // Insert plaintext
  std::vector<uint8_t> computed_ct(ct.size());

  in = msg.empty() ? junk_buf : msg.data();
  uint8_t *out = computed_ct.empty() ? junk_buf : computed_ct.data();
  out_len = 0;
  ASSERT_TRUE(EVP_EncryptUpdate(ctx, out, &out_len, in, msg.size()));
  ASSERT_EQ(out_len, (int) msg.size());

  // Finish the cipher
  out_len = 0;
  out = computed_ct.empty() ?
          junk_buf : computed_ct.data() + computed_ct.size();
  ASSERT_TRUE(EVP_EncryptFinal(ctx, out, &out_len));
  ASSERT_EQ(out_len, 0);

  // Get the tag
  std::vector<uint8_t> computed_tag(tag.size());
  ASSERT_TRUE(EVP_CIPHER_CTX_ctrl(ctx, EVP_CTRL_AEAD_GET_TAG, tag.size(),
                            computed_tag.data()));

  // Initialize the decrypt context
  std::unique_ptr<EVP_CIPHER_CTX, decltype(&EVP_CIPHER_CTX_free)> udctx(
          EVP_CIPHER_CTX_new(), EVP_CIPHER_CTX_free);
  EVP_CIPHER_CTX *dctx = udctx.get();
  ASSERT_TRUE(dctx);
  ASSERT_TRUE(EVP_DecryptInit_ex(dctx, cipher, nullptr, nullptr, nullptr));

  // Set the CTRL parameters
  ASSERT_TRUE(EVP_CIPHER_CTX_ctrl(dctx, EVP_CTRL_AEAD_SET_IVLEN, iv.size(),
                                  nullptr));
  ASSERT_TRUE(EVP_CIPHER_CTX_ctrl(dctx, EVP_CTRL_AEAD_SET_TAG, tag.size(),
                                  tag.data()));

  // Initialize with the key/iv
  ASSERT_TRUE(EVP_DecryptInit_ex(dctx, NULL, NULL, key.data(), iv.data()));

  // Set the message length for CCM
  if (is_ccm) {
    ASSERT_TRUE(EVP_CipherUpdate(dctx, nullptr, &out_len, nullptr, msg.size()));
    ASSERT_EQ(out_len, (int) msg.size());
  }

  // Insert AAD
  out_len = 0;
  in = aad.empty() ? junk_buf : aad.data();
  ASSERT_TRUE(EVP_DecryptUpdate(dctx, NULL, &out_len, in, aad.size()));
  ASSERT_EQ(out_len, (int) aad.size());

  // Insert ciphertext
  std::vector<uint8_t> computed_pt(msg.size());
  in = ct.empty() ? junk_buf : ct.data();
  out_len = 0;
  bool is_invalid_ccm = is_ccm && !result.IsValid();
  int expected_res = is_invalid_ccm ? 0 : 1;
  res = EVP_DecryptUpdate(dctx, computed_pt.data(), &out_len, in, ct.size());
  ASSERT_EQ(expected_res, res);
  if (!is_invalid_ccm) {
    ASSERT_EQ((int) msg.size(), out_len);
  }

  // Finish decryption
  out_len = 0;
  out = computed_pt.empty() ?
          junk_buf : computed_pt.data() + computed_pt.size();
  expected_res = is_invalid_ccm ? 1 : result.IsValid() ? 1 : 0;
  res = EVP_DecryptFinal(dctx, out, &out_len);

  // Tag validation, a valid result should end with a valid tag
  ASSERT_EQ(expected_res, res);
  ASSERT_EQ(out_len, 0);

  // Valid results should match the KATs
  if (result.IsValid()) {
    ASSERT_EQ(Bytes(tag.data(), tag.size()),
              Bytes(computed_tag.data(), computed_tag.size()));
    ASSERT_EQ(Bytes(msg.data(), msg.size()),
              Bytes(computed_pt.data(), computed_pt.size()));
    ASSERT_EQ(Bytes(ct.data(), ct.size()),
              Bytes(computed_ct.data(), computed_ct.size()));
  }
}

static bool ChaCha20Poly1305IvValidate(size_t iv_size) {
  return iv_size == 12;
}

static bool ChaCha20Poly1305TagValidate(size_t tag_size) {
  return tag_size <= 16;
}

TEST(CipherTest, WycheproofChaCha20Poly1305) {
  std::string test_vectors =
          "third_party/wycheproof_testvectors/chacha20_poly1305_test.txt";
  FileTestGTest(test_vectors.c_str(), [&](FileTest *t) {
    t->IgnoreInstruction("type");
    t->IgnoreInstruction("tagSize");
    t->IgnoreInstruction("keySize");
    t->IgnoreInstruction("ivSize");

    std::vector<uint8_t> key, iv, msg, ct, aad, tag;
    ASSERT_TRUE(t->GetBytes(&key, "key"));
    ASSERT_TRUE(t->GetBytes(&iv, "iv"));
    ASSERT_TRUE(t->GetBytes(&msg, "msg"));
    ASSERT_TRUE(t->GetBytes(&ct, "ct"));
    ASSERT_TRUE(t->GetBytes(&aad, "aad"));
    ASSERT_TRUE(t->GetBytes(&tag, "tag"));

    WycheproofResult result;
    ASSERT_TRUE(GetWycheproofResult(t, &result));
    const EVP_CIPHER *cipher = EVP_chacha20_poly1305();
    ASSERT_TRUE(cipher);
    WycheproofFileTest(t, key, iv, msg, ct, aad, tag, result, cipher,
                       &ChaCha20Poly1305IvValidate,
                       &ChaCha20Poly1305TagValidate);
  });
}

static bool AesCcmIvValidate(size_t iv_size) {
  size_t L = 15 - iv_size;
  if (L < 2 || L > 8) {
    return false;
  }
  return true;
}

static bool AesCcmTagValidate(size_t tag_size) {
  if ((tag_size & 1) || tag_size < 4 || tag_size > 16) {
    return false;
  }
  return true;
}

TEST(CipherTest, WycheproofAesCcm) {
  std::string test_vectors =
          "third_party/wycheproof_testvectors/aes_ccm_test.txt";
  FileTestGTest(test_vectors.c_str(), [&](FileTest *t) {
    t->IgnoreInstruction("type");
    t->IgnoreInstruction("tagSize");
    t->IgnoreInstruction("ivSize");

    std::vector<uint8_t> key, iv, msg, ct, aad, tag;
    ASSERT_TRUE(t->GetBytes(&key, "key"));
    ASSERT_TRUE(t->GetBytes(&iv, "iv"));
    ASSERT_TRUE(t->GetBytes(&msg, "msg"));
    ASSERT_TRUE(t->GetBytes(&ct, "ct"));
    ASSERT_TRUE(t->GetBytes(&aad, "aad"));
    ASSERT_TRUE(t->GetBytes(&tag, "tag"));

    WycheproofResult result;
    ASSERT_TRUE(GetWycheproofResult(t, &result));

    std::string key_size;
    ASSERT_TRUE(t->GetInstruction(&key_size, "keySize"));
    uint32_t key_bits = std::stoi(key_size);
    const EVP_CIPHER *cipher;
    switch (key_bits) {
      case 128:
        cipher = EVP_aes_128_ccm();
        break;
      case 192:
        cipher = EVP_aes_192_ccm();
        break;
      case 256:
        cipher = EVP_aes_256_ccm();
        break;
      default:
        FAIL();
    }
    ASSERT_TRUE(cipher);
    WycheproofFileTest(t, key, iv, msg, ct, aad, tag, result, cipher,
                       &AesCcmIvValidate, &AesCcmTagValidate);
  });
}

TEST(CipherTest, WycheproofAESCBC) {
  FileTestGTest("third_party/wycheproof_testvectors/aes_cbc_pkcs5_test.txt",
                [](FileTest *t) {
                  t->IgnoreInstruction("type");
                  t->IgnoreInstruction("ivSize");

                  std::string key_size;
                  ASSERT_TRUE(t->GetInstruction(&key_size, "keySize"));
                  const EVP_CIPHER *cipher;
                  switch (atoi(key_size.c_str())) {
                    case 128:
                      cipher = EVP_aes_128_cbc();
                      break;
                    case 192:
                      cipher = EVP_aes_192_cbc();
                      break;
                    case 256:
                      cipher = EVP_aes_256_cbc();
                      break;
                    default:
                      FAIL() << "Unsupported key size: " << key_size;
                  }

                  std::vector<uint8_t> key, iv, msg, ct;
                  ASSERT_TRUE(t->GetBytes(&key, "key"));
                  ASSERT_TRUE(t->GetBytes(&iv, "iv"));
                  ASSERT_TRUE(t->GetBytes(&msg, "msg"));
                  ASSERT_TRUE(t->GetBytes(&ct, "ct"));
                  WycheproofResult result;
                  ASSERT_TRUE(GetWycheproofResult(t, &result));
                  TestCipher(cipher,
                             result.IsValid() ? Operation::kBoth
                                              : Operation::kInvalidDecrypt,
                             /*padding=*/true, key, iv, msg, ct, /*aad=*/{},
                             /*tag=*/{});
                });
}

TEST(CipherTest, SHA1WithSecretSuffix) {
  uint8_t buf[SHA_CBLOCK * 4];
  RAND_bytes(buf, sizeof(buf));
  // Hashing should run in time independent of the bytes.
  CONSTTIME_SECRET(buf, sizeof(buf));

  // Exhaustively testing interesting cases in this function is cubic in the
  // block size, so we test in 3-byte increments.
  constexpr size_t kSkip = 3;
  // This value should be less than 8 to test the edge case when the 8-byte
  // length wraps to the next block.
  static_assert(kSkip < 8, "kSkip is too large");

  // |EVP_final_with_secret_suffix_sha1| is sensitive to the public length of
  // the partial block previously hashed. In TLS, this is the HMAC prefix, the
  // header, and the public minimum padding length.
  for (size_t prefix = 0; prefix < SHA_CBLOCK; prefix += kSkip) {
    SCOPED_TRACE(prefix);
    // The first block is treated differently, so we run with up to three
    // blocks of length variability.
    for (size_t max_len = 0; max_len < 3 * SHA_CBLOCK; max_len += kSkip) {
      SCOPED_TRACE(max_len);
      for (size_t len = 0; len <= max_len; len += kSkip) {
        SCOPED_TRACE(len);

        uint8_t expected[SHA_DIGEST_LENGTH];
        SHA1(buf, prefix + len, expected);
        CONSTTIME_DECLASSIFY(expected, sizeof(expected));

        // Make a copy of the secret length to avoid interfering with the loop.
        size_t secret_len = len;
        CONSTTIME_SECRET(&secret_len, sizeof(secret_len));

        SHA_CTX ctx;
        SHA1_Init(&ctx);
        SHA1_Update(&ctx, buf, prefix);
        uint8_t computed[SHA_DIGEST_LENGTH];
        ASSERT_TRUE(EVP_final_with_secret_suffix_sha1(
            &ctx, computed, buf + prefix, secret_len, max_len));

        CONSTTIME_DECLASSIFY(computed, sizeof(computed));
        EXPECT_EQ(Bytes(expected), Bytes(computed));
      }
    }
  }
}

TEST(CipherTest, SHA256WithSecretSuffix) {
  uint8_t buf[SHA256_CBLOCK * 4];
  RAND_bytes(buf, sizeof(buf));
  // Hashing should run in time independent of the bytes.
  CONSTTIME_SECRET(buf, sizeof(buf));

  // Exhaustively testing interesting cases in this function is cubic in the
  // block size, so we test in 3-byte increments.
  constexpr size_t kSkip = 3;
  // This value should be less than 8 to test the edge case when the 8-byte
  // length wraps to the next block.
  static_assert(kSkip < 8, "kSkip is too large");

  // |EVP_final_with_secret_suffix_sha256| is sensitive to the public length of
  // the partial block previously hashed. In TLS, this is the HMAC prefix, the
  // header, and the public minimum padding length.
  for (size_t prefix = 0; prefix < SHA256_CBLOCK; prefix += kSkip) {
    SCOPED_TRACE(prefix);
    // The first block is treated differently, so we run with up to three
    // blocks of length variability.
    for (size_t max_len = 0; max_len < 3 * SHA256_CBLOCK; max_len += kSkip) {
      SCOPED_TRACE(max_len);
      for (size_t len = 0; len <= max_len; len += kSkip) {
        SCOPED_TRACE(len);

        uint8_t expected[SHA256_DIGEST_LENGTH];
        SHA256(buf, prefix + len, expected);
        CONSTTIME_DECLASSIFY(expected, sizeof(expected));

        // Make a copy of the secret length to avoid interfering with the loop.
        size_t secret_len = len;
        CONSTTIME_SECRET(&secret_len, sizeof(secret_len));

        SHA256_CTX ctx;
        SHA256_Init(&ctx);
        SHA256_Update(&ctx, buf, prefix);
        uint8_t computed[SHA256_DIGEST_LENGTH];
        ASSERT_TRUE(EVP_final_with_secret_suffix_sha256(
            &ctx, computed, buf + prefix, secret_len, max_len));

        CONSTTIME_DECLASSIFY(computed, sizeof(computed));
        EXPECT_EQ(Bytes(expected), Bytes(computed));
      }
    }
  }
}

TEST(CipherTest, SHA384WithSecretSuffix) {
  uint8_t buf[SHA384_CBLOCK * 4];
  RAND_bytes(buf, sizeof(buf));
  // Hashing should run in time independent of the bytes.
  CONSTTIME_SECRET(buf, sizeof(buf));

  // Exhaustively testing interesting cases in this function is cubic in the
  // block size, so we test in 7-byte increments.
  constexpr size_t kSkip = 7;
  // This value should be less than 16 to test the edge case when the 16-byte
  // length wraps to the next block.
  static_assert(kSkip < 16, "kSkip is too large");

  // |EVP_final_with_secret_suffix_sha384| is sensitive to the public length of
  // the partial block previously hashed. In TLS, this is the HMAC prefix, the
  // header, and the public minimum padding length.
  for (size_t prefix = 0; prefix < SHA384_CBLOCK; prefix += kSkip) {
    SCOPED_TRACE(prefix);
    // The first block is treated differently, so we run with up to three
    // blocks of length variability.
    for (size_t max_len = 0; max_len < 3 * SHA384_CBLOCK; max_len += kSkip) {
      SCOPED_TRACE(max_len);
      for (size_t len = 0; len <= max_len; len += kSkip) {
        SCOPED_TRACE(len);

        uint8_t expected[SHA384_DIGEST_LENGTH];
        SHA384(buf, prefix + len, expected);
        CONSTTIME_DECLASSIFY(expected, sizeof(expected));

        // Make a copy of the secret length to avoid interfering with the loop.
        size_t secret_len = len;
        CONSTTIME_SECRET(&secret_len, sizeof(secret_len));

        SHA512_CTX ctx;
        SHA384_Init(&ctx);
        SHA384_Update(&ctx, buf, prefix);
        uint8_t computed[SHA384_DIGEST_LENGTH];
        ASSERT_TRUE(EVP_final_with_secret_suffix_sha384(
            &ctx, computed, buf + prefix, secret_len, max_len));

        CONSTTIME_DECLASSIFY(computed, sizeof(computed));
        EXPECT_EQ(Bytes(expected), Bytes(computed));
      }
    }
  }
}


TEST(CipherTest, GetCipher) {
  const auto test_get_cipher = [](int nid, const char *name) {
    EXPECT_EQ(nid, EVP_CIPHER_nid(EVP_get_cipherbynid(nid)));
    EXPECT_EQ(nid, EVP_CIPHER_nid(EVP_get_cipherbyname(name)));
  };

  // canonical names
  test_get_cipher(NID_aes_128_cbc, "aes-128-cbc");
  test_get_cipher(NID_aes_128_cfb128, "aes-128-cfb");
  test_get_cipher(NID_aes_128_ctr, "aes-128-ctr");
  test_get_cipher(NID_aes_128_ecb, "aes-128-ecb");
  test_get_cipher(NID_aes_128_gcm, "aes-128-gcm");
  test_get_cipher(NID_aes_128_ofb128, "aes-128-ofb");
  test_get_cipher(NID_aes_192_cbc, "aes-192-cbc");
  test_get_cipher(NID_aes_192_cfb128, "aes-192-cfb");
  test_get_cipher(NID_aes_192_ctr, "aes-192-ctr");
  test_get_cipher(NID_aes_192_ecb, "aes-192-ecb");
  test_get_cipher(NID_aes_192_gcm, "aes-192-gcm");
  test_get_cipher(NID_aes_192_ofb128, "aes-192-ofb");
  test_get_cipher(NID_aes_256_cbc, "aes-256-cbc");
  test_get_cipher(NID_aes_256_cfb128, "aes-256-cfb");
  test_get_cipher(NID_aes_256_ctr, "aes-256-ctr");
  test_get_cipher(NID_aes_256_ecb, "aes-256-ecb");
  test_get_cipher(NID_aes_256_gcm, "aes-256-gcm");
  test_get_cipher(NID_aes_256_ofb128, "aes-256-ofb");
  test_get_cipher(NID_aes_256_xts, "aes-256-xts");
  test_get_cipher(NID_chacha20_poly1305, "chacha20-poly1305");
  test_get_cipher(NID_des_cbc, "des-cbc");
  test_get_cipher(NID_des_ecb, "des-ecb");
  test_get_cipher(NID_des_ede_cbc, "des-ede-cbc");
  test_get_cipher(NID_des_ede_ecb, "des-ede");
  test_get_cipher(NID_des_ede3_cbc, "des-ede3-cbc");
  test_get_cipher(NID_rc2_cbc, "rc2-cbc");
  test_get_cipher(NID_rc4, "rc4");
  test_get_cipher(NID_bf_cbc, "bf-cbc");
  test_get_cipher(NID_bf_cfb64, "bf-cfb");
  test_get_cipher(NID_bf_ecb, "bf-ecb");

  // aliases
  test_get_cipher(NID_des_ede3_cbc, "des-ede3-cbc");
  test_get_cipher(NID_des_cbc, "DES");
  test_get_cipher(NID_aes_256_cbc, "aes256");
  test_get_cipher(NID_aes_128_cbc, "aes128");
  test_get_cipher(NID_aes_128_gcm, "id-aes128-gcm");
  test_get_cipher(NID_aes_192_gcm, "id-aes192-gcm");
  test_get_cipher(NID_aes_256_gcm, "id-aes256-gcm");

  // error case
  EXPECT_FALSE(EVP_get_cipherbyname(nullptr));
  EXPECT_FALSE(EVP_get_cipherbyname("vigenère"));
  EXPECT_FALSE(EVP_get_cipherbynid(-1));
  EXPECT_FALSE(EVP_CIPHER_block_size(nullptr));
  EXPECT_FALSE(EVP_CIPHER_key_length(nullptr));
  EXPECT_FALSE(EVP_CIPHER_iv_length(nullptr));
}

// Test the AES-GCM EVP_CIPHER's internal IV management APIs. OpenSSH uses these
// APIs.
TEST(CipherTest, GCMIncrementingIV) {
  const EVP_CIPHER *kCipher = EVP_aes_128_gcm();
  static const uint8_t kKey[16] = {0, 1, 2,  3,  4,  5,  6,  7,
                                   8, 9, 10, 11, 12, 13, 14, 15};
  static const uint8_t kInput[] = {'h', 'e', 'l', 'l', 'o'};

  auto expect_iv = [&](EVP_CIPHER_CTX *ctx, bssl::Span<const uint8_t> iv,
                       bool enc) {
    // Make a reference ciphertext.
    bssl::ScopedEVP_CIPHER_CTX ref;
    ASSERT_TRUE(EVP_EncryptInit_ex(ref.get(), kCipher, /*impl=*/nullptr, kKey,
                                   /*iv=*/nullptr));
    ASSERT_TRUE(EVP_CIPHER_CTX_ctrl(ref.get(), EVP_CTRL_AEAD_SET_IVLEN,
                                    static_cast<int>(iv.size()), nullptr));
    ASSERT_TRUE(EVP_EncryptInit_ex(ref.get(), /*cipher=*/nullptr,
                                   /*impl=*/nullptr, /*key=*/nullptr,
                                   iv.data()));
    uint8_t ciphertext[sizeof(kInput)];
    int ciphertext_len;
    ASSERT_TRUE(EVP_EncryptUpdate(ref.get(), ciphertext, &ciphertext_len,
                                  kInput, sizeof(kInput)));
    int extra_len;
    ASSERT_TRUE(EVP_EncryptFinal_ex(ref.get(), nullptr, &extra_len));
    ASSERT_EQ(extra_len, 0);
    uint8_t tag[16];
    ASSERT_TRUE(EVP_CIPHER_CTX_ctrl(ref.get(), EVP_CTRL_AEAD_GET_TAG,
                                    sizeof(tag), tag));

    if (enc) {
      uint8_t actual[sizeof(kInput)];
      int actual_len;
      ASSERT_TRUE(
          EVP_EncryptUpdate(ctx, actual, &actual_len, kInput, sizeof(kInput)));
      ASSERT_TRUE(EVP_EncryptFinal_ex(ctx, nullptr, &extra_len));
      ASSERT_EQ(extra_len, 0);
      EXPECT_EQ(Bytes(actual, actual_len), Bytes(ciphertext, ciphertext_len));
      uint8_t actual_tag[16];
      ASSERT_TRUE(EVP_CIPHER_CTX_ctrl(ctx, EVP_CTRL_AEAD_GET_TAG,
                                      sizeof(actual_tag), actual_tag));
      EXPECT_EQ(Bytes(actual_tag), Bytes(tag));
    } else {
      ASSERT_TRUE(EVP_CIPHER_CTX_ctrl(ctx, EVP_CTRL_AEAD_SET_TAG, sizeof(tag),
                                      const_cast<uint8_t *>(tag)));
      uint8_t actual[sizeof(kInput)];
      int actual_len;
      ASSERT_TRUE(EVP_DecryptUpdate(ctx, actual, &actual_len, ciphertext,
                                    sizeof(ciphertext)));
      ASSERT_TRUE(EVP_DecryptFinal_ex(ctx, nullptr, &extra_len));
      ASSERT_EQ(extra_len, 0);
      EXPECT_EQ(Bytes(actual, actual_len), Bytes(kInput));
    }
  };

  {
    // Passing in a fixed IV length of -1 sets the whole IV, but then configures
    // |EVP_CIPHER_CTX| to increment the bottom 8 bytes of the IV.
    static const uint8_t kIV1[12] = {1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12};
    static const uint8_t kIV2[12] = {1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 13};
    static const uint8_t kIV3[12] = {1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 14};
    static const uint8_t kIV4[12] = {1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 15};

    bssl::ScopedEVP_CIPHER_CTX ctx;
    ASSERT_TRUE(EVP_EncryptInit_ex(ctx.get(), kCipher, /*impl=*/nullptr, kKey,
                                   /*iv=*/nullptr));
    ASSERT_TRUE(EVP_CIPHER_CTX_ctrl(ctx.get(), EVP_CTRL_AEAD_SET_IV_FIXED, -1,
                                    const_cast<uint8_t *>(kIV1)));

    // EVP_CTRL_GCM_IV_GEN both configures and returns the IV.
    uint8_t iv[12];
    ASSERT_TRUE(
        EVP_CIPHER_CTX_ctrl(ctx.get(), EVP_CTRL_GCM_IV_GEN, sizeof(iv), iv));
    EXPECT_EQ(Bytes(iv), Bytes(kIV1));
    ASSERT_NO_FATAL_FAILURE(expect_iv(ctx.get(), kIV1, /*enc=*/true));

    // Continuing to run EVP_CTRL_GCM_IV_GEN should increment the IV.
    ASSERT_TRUE(
        EVP_CIPHER_CTX_ctrl(ctx.get(), EVP_CTRL_GCM_IV_GEN, sizeof(iv), iv));
    EXPECT_EQ(Bytes(iv), Bytes(kIV2));
    ASSERT_NO_FATAL_FAILURE(expect_iv(ctx.get(), kIV2, /*enc=*/true));

    // Passing in a shorter length outputs the suffix portion.
    uint8_t suffix[8];
    ASSERT_TRUE(EVP_CIPHER_CTX_ctrl(ctx.get(), EVP_CTRL_GCM_IV_GEN,
                                    sizeof(suffix), suffix));
    EXPECT_EQ(Bytes(suffix),
              Bytes(bssl::MakeConstSpan(kIV3).last(sizeof(suffix))));
    ASSERT_NO_FATAL_FAILURE(expect_iv(ctx.get(), kIV3, /*enc=*/true));

    // A length of -1 returns the whole IV.
    ASSERT_TRUE(EVP_CIPHER_CTX_ctrl(ctx.get(), EVP_CTRL_GCM_IV_GEN, -1, iv));
    EXPECT_EQ(Bytes(iv), Bytes(kIV4));
    ASSERT_NO_FATAL_FAILURE(expect_iv(ctx.get(), kIV4, /*enc=*/true));
  }

  {
    // Similar to the above, but for decrypting.
    static const uint8_t kIV1[12] = {1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12};
    static const uint8_t kIV2[12] = {1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 13};

    bssl::ScopedEVP_CIPHER_CTX ctx;
    ASSERT_TRUE(EVP_DecryptInit_ex(ctx.get(), kCipher, /*impl=*/nullptr, kKey,
                                   /*iv=*/nullptr));
    ASSERT_TRUE(EVP_CIPHER_CTX_ctrl(ctx.get(), EVP_CTRL_AEAD_SET_IV_FIXED, -1,
                                    const_cast<uint8_t *>(kIV1)));

    uint8_t iv[12];
    ASSERT_TRUE(
        EVP_CIPHER_CTX_ctrl(ctx.get(), EVP_CTRL_GCM_IV_GEN, sizeof(iv), iv));
    EXPECT_EQ(Bytes(iv), Bytes(kIV1));
    ASSERT_NO_FATAL_FAILURE(expect_iv(ctx.get(), kIV1, /*enc=*/false));

    ASSERT_TRUE(
        EVP_CIPHER_CTX_ctrl(ctx.get(), EVP_CTRL_GCM_IV_GEN, sizeof(iv), iv));
    EXPECT_EQ(Bytes(iv), Bytes(kIV2));
    ASSERT_NO_FATAL_FAILURE(expect_iv(ctx.get(), kIV2, /*enc=*/false));
  }

  {
    // Test that only the bottom 8 bytes are used as a counter.
    static const uint8_t kIV1[12] = {0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
                                     0xff, 0xff, 0xff, 0xff, 0xff, 0xff};
    static const uint8_t kIV2[12] = {0xff, 0xff, 0xff, 0xff, 0x00, 0x00,
                                     0x00, 0x00, 0x00, 0x00, 0x00, 0x00};
    static const uint8_t kIV3[12] = {0xff, 0xff, 0xff, 0xff, 0x00, 0x00,
                                     0x00, 0x00, 0x00, 0x00, 0x00, 0x01};

    bssl::ScopedEVP_CIPHER_CTX ctx;
    ASSERT_TRUE(EVP_EncryptInit_ex(ctx.get(), kCipher, /*impl=*/nullptr, kKey,
                                   /*iv=*/nullptr));
    ASSERT_TRUE(EVP_CIPHER_CTX_ctrl(ctx.get(), EVP_CTRL_AEAD_SET_IV_FIXED, -1,
                                    const_cast<uint8_t *>(kIV1)));

    uint8_t iv[12];
    ASSERT_TRUE(
        EVP_CIPHER_CTX_ctrl(ctx.get(), EVP_CTRL_GCM_IV_GEN, sizeof(iv), iv));
    EXPECT_EQ(Bytes(iv), Bytes(kIV1));
    ASSERT_NO_FATAL_FAILURE(expect_iv(ctx.get(), kIV1, /*enc=*/true));

    ASSERT_TRUE(
        EVP_CIPHER_CTX_ctrl(ctx.get(), EVP_CTRL_GCM_IV_GEN, sizeof(iv), iv));
    EXPECT_EQ(Bytes(iv), Bytes(kIV2));
    ASSERT_NO_FATAL_FAILURE(expect_iv(ctx.get(), kIV2, /*enc=*/true));

    ASSERT_TRUE(
        EVP_CIPHER_CTX_ctrl(ctx.get(), EVP_CTRL_GCM_IV_GEN, sizeof(iv), iv));
    EXPECT_EQ(Bytes(iv), Bytes(kIV3));
    ASSERT_NO_FATAL_FAILURE(expect_iv(ctx.get(), kIV3, /*enc=*/true));
  }

  {
    // Test with a longer IV length.
    static const uint8_t kIV1[16] = {0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
                                     0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
                                     0xff, 0xff, 0xff, 0xff};
    static const uint8_t kIV2[16] = {0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
                                     0xff, 0xff, 0x00, 0x00, 0x00, 0x00,
                                     0x00, 0x00, 0x00, 0x00};
    static const uint8_t kIV3[16] = {0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
                                     0xff, 0xff, 0x00, 0x00, 0x00, 0x00,
                                     0x00, 0x00, 0x00, 0x01};

    bssl::ScopedEVP_CIPHER_CTX ctx;
    ASSERT_TRUE(EVP_EncryptInit_ex(ctx.get(), kCipher, /*impl=*/nullptr, kKey,
                                   /*iv=*/nullptr));
    ASSERT_TRUE(EVP_CIPHER_CTX_ctrl(ctx.get(), EVP_CTRL_AEAD_SET_IVLEN,
                                    sizeof(kIV1), nullptr));
    ASSERT_TRUE(EVP_CIPHER_CTX_ctrl(ctx.get(), EVP_CTRL_AEAD_SET_IV_FIXED, -1,
                                    const_cast<uint8_t *>(kIV1)));

    uint8_t iv[16];
    ASSERT_TRUE(
        EVP_CIPHER_CTX_ctrl(ctx.get(), EVP_CTRL_GCM_IV_GEN, sizeof(iv), iv));
    EXPECT_EQ(Bytes(iv), Bytes(kIV1));
    ASSERT_NO_FATAL_FAILURE(expect_iv(ctx.get(), kIV1, /*enc=*/true));

    ASSERT_TRUE(
        EVP_CIPHER_CTX_ctrl(ctx.get(), EVP_CTRL_GCM_IV_GEN, sizeof(iv), iv));
    EXPECT_EQ(Bytes(iv), Bytes(kIV2));
    ASSERT_NO_FATAL_FAILURE(expect_iv(ctx.get(), kIV2, /*enc=*/true));

    ASSERT_TRUE(
        EVP_CIPHER_CTX_ctrl(ctx.get(), EVP_CTRL_GCM_IV_GEN, sizeof(iv), iv));
    EXPECT_EQ(Bytes(iv), Bytes(kIV3));
    ASSERT_NO_FATAL_FAILURE(expect_iv(ctx.get(), kIV3, /*enc=*/true));
  }

  {
    // When decrypting, callers are expected to configure the fixed half and
    // invocation half separately. The two will get stitched together into the
    // final IV.
    const uint8_t kIV[12] = {1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12};

    bssl::ScopedEVP_CIPHER_CTX ctx;
    ASSERT_TRUE(EVP_DecryptInit_ex(ctx.get(), kCipher, /*impl=*/nullptr, kKey,
                                   /*iv=*/nullptr));
    ASSERT_TRUE(EVP_CIPHER_CTX_ctrl(ctx.get(), EVP_CTRL_AEAD_SET_IV_FIXED, 4,
                                    const_cast<uint8_t *>(kIV)));
    ASSERT_TRUE(EVP_CIPHER_CTX_ctrl(ctx.get(), EVP_CTRL_GCM_SET_IV_INV, 8,
                                    const_cast<uint8_t *>(kIV + 4)));
    // EVP_CTRL_GCM_SET_IV_INV is sufficient to configure the IV. There is no
    // need to call EVP_CTRL_GCM_IV_GEN.
    ASSERT_NO_FATAL_FAILURE(expect_iv(ctx.get(), kIV, /*enc=*/false));
  }

  {
    // Stitching together a decryption IV that exceeds the standard IV length.
    const uint8_t kIV[16] = {1, 2,  3,  4,  5,  6,  7,  8,
                             9, 10, 11, 12, 13, 14, 15, 16};

    bssl::ScopedEVP_CIPHER_CTX ctx;
    ASSERT_TRUE(EVP_DecryptInit_ex(ctx.get(), kCipher, /*impl=*/nullptr, kKey,
                                   /*iv=*/nullptr));
    ASSERT_TRUE(EVP_CIPHER_CTX_ctrl(ctx.get(), EVP_CTRL_AEAD_SET_IVLEN,
                                    sizeof(kIV), nullptr));

    ASSERT_TRUE(EVP_CIPHER_CTX_ctrl(ctx.get(), EVP_CTRL_AEAD_SET_IV_FIXED, 4,
                                    const_cast<uint8_t *>(kIV)));
    ASSERT_TRUE(EVP_CIPHER_CTX_ctrl(ctx.get(), EVP_CTRL_GCM_SET_IV_INV, 12,
                                    const_cast<uint8_t *>(kIV + 4)));
    // EVP_CTRL_GCM_SET_IV_INV is sufficient to configure the IV. There is no
    // need to call EVP_CTRL_GCM_IV_GEN.
    ASSERT_NO_FATAL_FAILURE(expect_iv(ctx.get(), kIV, /*enc=*/false));
  }

  {
    // Fixed IVs must be at least 4 bytes and admit at least an 8 byte counter.
    const uint8_t kIV[16] = {1, 2,  3,  4,  5,  6,  7,  8,
                             9, 10, 11, 12, 13, 14, 15, 16};

    bssl::ScopedEVP_CIPHER_CTX ctx;
    ASSERT_TRUE(EVP_DecryptInit_ex(ctx.get(), kCipher, /*impl=*/nullptr, kKey,
                                   /*iv=*/nullptr));

    // This means the default IV length only allows a 4/8 split.
    EXPECT_FALSE(EVP_CIPHER_CTX_ctrl(ctx.get(), EVP_CTRL_AEAD_SET_IV_FIXED, 0,
                                     const_cast<uint8_t *>(kIV)));
    EXPECT_FALSE(EVP_CIPHER_CTX_ctrl(ctx.get(), EVP_CTRL_AEAD_SET_IV_FIXED, 3,
                                     const_cast<uint8_t *>(kIV)));
    EXPECT_TRUE(EVP_CIPHER_CTX_ctrl(ctx.get(), EVP_CTRL_AEAD_SET_IV_FIXED, 4,
                                    const_cast<uint8_t *>(kIV)));
    EXPECT_FALSE(EVP_CIPHER_CTX_ctrl(ctx.get(), EVP_CTRL_AEAD_SET_IV_FIXED, 5,
                                     const_cast<uint8_t *>(kIV)));
    EXPECT_FALSE(EVP_CIPHER_CTX_ctrl(ctx.get(), EVP_CTRL_AEAD_SET_IV_FIXED, 16,
                                     const_cast<uint8_t *>(kIV)));

    // A longer IV allows a wider range.
    ASSERT_TRUE(
        EVP_CIPHER_CTX_ctrl(ctx.get(), EVP_CTRL_AEAD_SET_IVLEN, 16, nullptr));
    EXPECT_FALSE(EVP_CIPHER_CTX_ctrl(ctx.get(), EVP_CTRL_AEAD_SET_IV_FIXED, 0,
                                     const_cast<uint8_t *>(kIV)));
    EXPECT_FALSE(EVP_CIPHER_CTX_ctrl(ctx.get(), EVP_CTRL_AEAD_SET_IV_FIXED, 3,
                                     const_cast<uint8_t *>(kIV)));
    EXPECT_TRUE(EVP_CIPHER_CTX_ctrl(ctx.get(), EVP_CTRL_AEAD_SET_IV_FIXED, 4,
                                    const_cast<uint8_t *>(kIV)));
    EXPECT_TRUE(EVP_CIPHER_CTX_ctrl(ctx.get(), EVP_CTRL_AEAD_SET_IV_FIXED, 6,
                                    const_cast<uint8_t *>(kIV)));
    EXPECT_TRUE(EVP_CIPHER_CTX_ctrl(ctx.get(), EVP_CTRL_AEAD_SET_IV_FIXED, 8,
                                    const_cast<uint8_t *>(kIV)));
    EXPECT_FALSE(EVP_CIPHER_CTX_ctrl(ctx.get(), EVP_CTRL_AEAD_SET_IV_FIXED, 9,
                                     const_cast<uint8_t *>(kIV)));
    EXPECT_FALSE(EVP_CIPHER_CTX_ctrl(ctx.get(), EVP_CTRL_AEAD_SET_IV_FIXED, 16,
                                     const_cast<uint8_t *>(kIV)));
  }

  {
    // When encrypting, setting a fixed IV randomizes the counter portion.
    const uint8_t kFixedIV[4] = {1, 2, 3, 4};
    bssl::ScopedEVP_CIPHER_CTX ctx;
    ASSERT_TRUE(EVP_EncryptInit_ex(ctx.get(), kCipher, /*impl=*/nullptr, kKey,
                                   /*iv=*/nullptr));
    ASSERT_TRUE(EVP_CIPHER_CTX_ctrl(ctx.get(), EVP_CTRL_AEAD_SET_IV_FIXED,
                                    sizeof(kFixedIV),
                                    const_cast<uint8_t *>(kFixedIV)));
    uint8_t counter[8];
    ASSERT_TRUE(EVP_CIPHER_CTX_ctrl(ctx.get(), EVP_CTRL_GCM_IV_GEN,
                                    sizeof(counter), counter));

    uint8_t iv[12];
    memcpy(iv, kFixedIV, sizeof(kFixedIV));
    memcpy(iv + sizeof(kFixedIV), counter, sizeof(counter));
    ASSERT_NO_FATAL_FAILURE(expect_iv(ctx.get(), iv, /*enc=*/true));

    // The counter continues to act as a counter.
    uint8_t counter2[8];
    ASSERT_TRUE(EVP_CIPHER_CTX_ctrl(ctx.get(), EVP_CTRL_GCM_IV_GEN,
                                    sizeof(counter2), counter2));
    EXPECT_EQ(CRYPTO_load_u64_be(counter2), CRYPTO_load_u64_be(counter) + 1);
    memcpy(iv + sizeof(kFixedIV), counter2, sizeof(counter2));
    ASSERT_NO_FATAL_FAILURE(expect_iv(ctx.get(), iv, /*enc=*/true));
  }

  {
    // Same as above, but with a larger IV.
    const uint8_t kFixedIV[8] = {1, 2, 3, 4, 5, 6, 7, 8};
    bssl::ScopedEVP_CIPHER_CTX ctx;
    ASSERT_TRUE(EVP_EncryptInit_ex(ctx.get(), kCipher, /*impl=*/nullptr, kKey,
                                   /*iv=*/nullptr));
    ASSERT_TRUE(EVP_CIPHER_CTX_ctrl(ctx.get(), EVP_CTRL_AEAD_SET_IVLEN,
                                    sizeof(kFixedIV) + 8, nullptr));
    ASSERT_TRUE(EVP_CIPHER_CTX_ctrl(ctx.get(), EVP_CTRL_AEAD_SET_IV_FIXED,
                                    sizeof(kFixedIV),
                                    const_cast<uint8_t *>(kFixedIV)));
    uint8_t counter[8];
    ASSERT_TRUE(EVP_CIPHER_CTX_ctrl(ctx.get(), EVP_CTRL_GCM_IV_GEN,
                                    sizeof(counter), counter));

    uint8_t iv[16];
    memcpy(iv, kFixedIV, sizeof(kFixedIV));
    memcpy(iv + sizeof(kFixedIV), counter, sizeof(counter));
    ASSERT_NO_FATAL_FAILURE(expect_iv(ctx.get(), iv, /*enc=*/true));

    // The counter continues to act as a counter.
    uint8_t counter2[8];
    ASSERT_TRUE(EVP_CIPHER_CTX_ctrl(ctx.get(), EVP_CTRL_GCM_IV_GEN,
                                    sizeof(counter2), counter2));
    EXPECT_EQ(CRYPTO_load_u64_be(counter2), CRYPTO_load_u64_be(counter) + 1);
    memcpy(iv + sizeof(kFixedIV), counter2, sizeof(counter2));
    ASSERT_NO_FATAL_FAILURE(expect_iv(ctx.get(), iv, /*enc=*/true));
  }
}

#define CHECK_ERROR(function, err) \
    ERR_clear_error();                 \
    EXPECT_FALSE(function);                          \
    EXPECT_EQ(err, ERR_GET_REASON(ERR_peek_last_error()));

TEST(CipherTest, Empty_EVP_CIPHER_CTX_V1187459157) {
  int in_len = 10;
  std::vector<uint8_t> in_vec(in_len);
  int out_len = in_len + 256;
  std::vector<uint8_t> out_vec(out_len);

  CHECK_ERROR(EVP_EncryptUpdate(nullptr, out_vec.data(), &out_len, in_vec.data(), in_len), ERR_R_PASSED_NULL_PARAMETER);

  bssl::UniquePtr<EVP_CIPHER_CTX> ctx(EVP_CIPHER_CTX_new());
  CHECK_ERROR(EVP_EncryptUpdate(ctx.get(), out_vec.data(), &out_len, in_vec.data(), in_len), ERR_R_SHOULD_NOT_HAVE_BEEN_CALLED);
  CHECK_ERROR(EVP_EncryptFinal(ctx.get(), out_vec.data(), &out_len), ERR_R_SHOULD_NOT_HAVE_BEEN_CALLED);
  CHECK_ERROR(EVP_DecryptUpdate(ctx.get(), out_vec.data(), &out_len, in_vec.data(), in_len), ERR_R_SHOULD_NOT_HAVE_BEEN_CALLED);
  CHECK_ERROR(EVP_DecryptFinal(ctx.get(), out_vec.data(), &out_len), ERR_R_SHOULD_NOT_HAVE_BEEN_CALLED);
}

// Mock cipher whose |EVP_CTRL_INIT| handler always fails. It's needed to
// exercise the error paths in |EVP_CipherInit_ex| and |EVP_CIPHER_CTX_copy|.
// In addition, the mock needs |ctx_size| != 0, otherwise |cipher_data| is
// not allocated before the ctrl callback runs. This is needed below for
// InitErrorPathReleasesCipherData, to test |cipher_data| is free'd.
static int mock_failing_init(EVP_CIPHER_CTX *, const uint8_t *, const uint8_t *,
                             int) {
  return 1;
}

static int mock_failing_cipher(EVP_CIPHER_CTX *, uint8_t *, const uint8_t *,
                               size_t) {
  return 1;
}

static int mock_failing_ctrl(EVP_CIPHER_CTX *, int type, int, void *) {
  if (type == EVP_CTRL_INIT) {
    return 0;
  }
  return -1;
}

static int mock_failing_copy_ctrl(EVP_CIPHER_CTX *, int type, int, void *) {
  if (type == EVP_CTRL_INIT) {
    return 1;
  }
  if (type == EVP_CTRL_COPY) {
    return 0;
  }
  return -1;
}

static const EVP_CIPHER kFailingInitCipher = {
  NID_undef,
  1,
  16,
  0,
  128,
  EVP_CIPH_STREAM_CIPHER | EVP_CIPH_CTRL_INIT,
  mock_failing_init,
  mock_failing_cipher,
  nullptr,
  mock_failing_ctrl,
};

static const EVP_CIPHER kFailingCopyCipher = {
  NID_undef,
  1,
  16,
  0,
  128,
  EVP_CIPH_STREAM_CIPHER | EVP_CIPH_CTRL_INIT | EVP_CIPH_CUSTOM_COPY,
  mock_failing_init,
  mock_failing_cipher,
  nullptr,
  mock_failing_copy_ctrl,
};

// On |EVP_CipherInit_ex| error path at |EVP_CTRL_INIT|, the context must not be
// left with an orphaned |cipher_data|. Otherwise a subsequent
// |EVP_CipherInit_ex| call on the same context would overwrite and leak it. Not
// exactly how one would probably use this API, but who knows.
TEST(CipherTest, InitErrorPathReleasesCipherData) {
  bssl::UniquePtr<EVP_CIPHER_CTX> ctx(EVP_CIPHER_CTX_new());
  ASSERT_TRUE(ctx);

  std::vector<uint8_t> key(16, 0);
  EXPECT_FALSE(EVP_CipherInit_ex(ctx.get(), &kFailingInitCipher, nullptr,
                                 key.data(), nullptr, 1));
  EXPECT_EQ(ctx->cipher, nullptr);
  EXPECT_EQ(ctx->cipher_data, nullptr);

  // A follow-up init with a real cipher must still work and must not depend on
  // leaked state from the failed attempt.
  EXPECT_TRUE(EVP_CipherInit_ex(ctx.get(), EVP_aes_128_ecb(), nullptr,
                                key.data(), nullptr, 1));
}

// |EVP_CIPHER_CTX_copy| shares the same shape of error path for
// |EVP_CTRL_COPY| failures as |EVP_CTRL_INIT| above. The destination's
// |cipher_data| must not be orphaned when the custom copy callback fails.
TEST(CipherTest, CopyErrorPathReleasesCipherData) {
  bssl::UniquePtr<EVP_CIPHER_CTX> src(EVP_CIPHER_CTX_new());
  ASSERT_TRUE(src);
  std::vector<uint8_t> key(16, 0);
  ASSERT_TRUE(EVP_CipherInit_ex(src.get(), &kFailingCopyCipher, nullptr,
                                key.data(), nullptr, 1));

  bssl::UniquePtr<EVP_CIPHER_CTX> dst(EVP_CIPHER_CTX_new());
  ASSERT_TRUE(dst);
  EXPECT_FALSE(EVP_CIPHER_CTX_copy(dst.get(), src.get()));
  EXPECT_EQ(dst->cipher, nullptr);
  EXPECT_EQ(dst->cipher_data, nullptr);

  // A follow-up init with a real cipher on |dst| must still work and must not
  // depend on leaked state from the failed copy.
  EXPECT_TRUE(EVP_CipherInit_ex(dst.get(), EVP_aes_128_ecb(), nullptr,
                                key.data(), nullptr, 1));
}

struct CipherInfo {
  const char *name;
  const EVP_CIPHER *(*func)(void);
};

static const CipherInfo kAllCiphers[] = {
    {"AES-128-ECB", EVP_aes_128_ecb},
    {"AES-128-CBC", EVP_aes_128_cbc},
    {"AES-128-CTR", EVP_aes_128_ctr},
    {"AES-128-OFB", EVP_aes_128_ofb},
    {"AES-128-GCM", EVP_aes_128_gcm},
    {"AES-128-CCM", EVP_aes_128_ccm},
    {"AES-192-ECB", EVP_aes_192_ecb},
    {"AES-192-CBC", EVP_aes_192_cbc},
    {"AES-192-CTR", EVP_aes_192_ctr},
    {"AES-192-OFB", EVP_aes_192_ofb},
    {"AES-192-GCM", EVP_aes_192_gcm},
    {"AES-192-CCM", EVP_aes_192_ccm},
    {"AES-256-ECB", EVP_aes_256_ecb},
    {"AES-256-CBC", EVP_aes_256_cbc},
    {"AES-256-CTR", EVP_aes_256_ctr},
    {"AES-256-OFB", EVP_aes_256_ofb},
    {"AES-256-GCM", EVP_aes_256_gcm},
    {"AES-256-CCM", EVP_aes_256_ccm},
    {"AES-256-XTS", EVP_aes_256_xts},
    {"DES-CBC", EVP_des_cbc},
    {"DES-ECB", EVP_des_ecb},
    {"DES-EDE", EVP_des_ede},
    {"DES-EDE3", EVP_des_ede3},
    {"DES-EDE-CBC", EVP_des_ede_cbc},
    {"DES-EDE3-CBC", EVP_des_ede3_cbc},
    {"ChaCha20-Poly1305", EVP_chacha20_poly1305},
};

class RandomizedCipherTest : public testing::TestWithParam<CipherInfo> {};

INSTANTIATE_TEST_SUITE_P(
    All, RandomizedCipherTest, testing::ValuesIn(kAllCiphers),
    [](const testing::TestParamInfo<CipherInfo> &info) -> std::string {
      std::string name = info.param.name;
      std::replace(name.begin(), name.end(), '-', '_');
      return name;
    });

TEST_P(RandomizedCipherTest, EncryptDecrypt) {
  if (runtimeEmulationIsIntelSde() && addressSanitizerIsEnabled()) {
    GTEST_SKIP() << "Test not supported under Intel SDE + ASAN";
  }

  // This is an arbitrary max input size that I chose such that
  // the test runs for about 2 seconds.
  const size_t max_input_size = 1601;
  const size_t num_tests_per_input_size = 10;

  const EVP_CIPHER *cipher = GetParam().func();
  ASSERT_TRUE(cipher);

  const size_t key_len = EVP_CIPHER_key_length(cipher);
  const size_t iv_len = EVP_CIPHER_iv_length(cipher);
  const size_t block_size = EVP_CIPHER_block_size(cipher);
  const uint32_t mode = EVP_CIPHER_mode(cipher);
  const bool is_aead = EVP_CIPHER_flags(cipher) & EVP_CIPH_FLAG_AEAD_CIPHER;
  const bool is_ccm = mode == EVP_CIPH_CCM_MODE;
  const bool is_xts = mode == EVP_CIPH_XTS_MODE;

  for (size_t pt_len = 0; pt_len <= max_input_size; pt_len++) {
    // Filter based on cipher mode constraints.
    if (is_xts && pt_len < 16) {
      continue;  // XTS requires at least 16 bytes
    }
    if (is_ccm && pt_len == 0) {
      continue;  // CCM requires non-empty plaintext for tag
    }
    if ((mode == EVP_CIPH_ECB_MODE || mode == EVP_CIPH_CBC_MODE) &&
        pt_len % block_size != 0) {
      continue;  // Block modes without padding require aligned input
    }

    for (size_t i = 0; i < num_tests_per_input_size; i++) {

      // Generate random key, IV, plaintext, aad.
      std::vector<uint8_t> key(key_len);
      std::vector<uint8_t> iv(iv_len);
      std::vector<uint8_t> plaintext(pt_len);
      std::vector<uint8_t> aad;

      RAND_bytes(key.data(), key_len);
      if (iv_len > 0) {
        RAND_bytes(iv.data(), iv_len);
      }
      if (pt_len > 0) {
        RAND_bytes(plaintext.data(), pt_len);
      }
      if (is_aead) {
        aad.resize(16);
        RAND_bytes(aad.data(), aad.size());
      }

      // Encrypt
      bssl::UniquePtr<EVP_CIPHER_CTX> ctx(EVP_CIPHER_CTX_new());
      ASSERT_TRUE(ctx);
      ASSERT_TRUE(EVP_EncryptInit_ex(ctx.get(), cipher, nullptr, nullptr, nullptr));
      if (!is_xts) {
        ASSERT_TRUE(EVP_CIPHER_CTX_set_padding(ctx.get(), 0));
      }

      if (is_ccm) {
        ASSERT_TRUE(EVP_CIPHER_CTX_ctrl(ctx.get(), EVP_CTRL_AEAD_SET_TAG, 16, nullptr));
      }

      ASSERT_TRUE(EVP_EncryptInit_ex(ctx.get(), nullptr, nullptr, key.data(),
                                     iv_len > 0 ? iv.data() : nullptr));

      if (is_ccm) {
        int len = 0;
        ASSERT_TRUE(EVP_CipherUpdate(ctx.get(), nullptr, &len, nullptr, pt_len));
      }

      if (!aad.empty()) {
        int len = 0;
        ASSERT_TRUE(EVP_EncryptUpdate(ctx.get(), nullptr, &len, aad.data(), aad.size()));
      }

      size_t max_ct_len = pt_len + block_size;
      std::vector<uint8_t> ciphertext(max_ct_len);
      int out_len = 0;
      if (pt_len > 0) {
        ASSERT_TRUE(EVP_EncryptUpdate(ctx.get(), ciphertext.data(), &out_len,
                                      plaintext.data(), pt_len));
      }
      size_t ct_len = out_len;

      int final_len = 0;
      ASSERT_TRUE(EVP_EncryptFinal_ex(ctx.get(), ciphertext.data() + ct_len, &final_len));
      ct_len += final_len;
      ciphertext.resize(ct_len);

      std::vector<uint8_t> tag;
      if (is_aead) {
        tag.resize(16);
        ASSERT_TRUE(EVP_CIPHER_CTX_ctrl(ctx.get(), EVP_CTRL_AEAD_GET_TAG,
                                        tag.size(), tag.data()));
      }

      // Decrypt
      ctx.reset(EVP_CIPHER_CTX_new());
      ASSERT_TRUE(ctx);
      ASSERT_TRUE(EVP_DecryptInit_ex(ctx.get(), cipher, nullptr, nullptr, nullptr));
      if (!is_xts) {
        ASSERT_TRUE(EVP_CIPHER_CTX_set_padding(ctx.get(), 0));
      }

      if (is_aead) {
        ASSERT_TRUE(EVP_CIPHER_CTX_ctrl(ctx.get(), EVP_CTRL_AEAD_SET_TAG,
                                        tag.size(), tag.data()));
      }

      ASSERT_TRUE(EVP_DecryptInit_ex(ctx.get(), nullptr, nullptr, key.data(),
                                     iv_len > 0 ? iv.data() : nullptr));

      if (is_ccm) {
        int len = 0;
        ASSERT_TRUE(EVP_CipherUpdate(ctx.get(), nullptr, &len, nullptr, ct_len));
      }

      if (!aad.empty()) {
        int len = 0;
        ASSERT_TRUE(EVP_DecryptUpdate(ctx.get(), nullptr, &len, aad.data(), aad.size()));
      }

      std::vector<uint8_t> decrypted(ct_len + block_size);
      out_len = 0;
      if (ct_len > 0) {
        int ret = EVP_DecryptUpdate(ctx.get(), decrypted.data(), &out_len,
                                    ciphertext.data(), ct_len);
        ASSERT_TRUE(ret);
      }
      size_t dec_len = out_len;

      final_len = 0;
      ASSERT_TRUE(EVP_DecryptFinal_ex(ctx.get(), decrypted.data() + dec_len, &final_len));
      dec_len += final_len;
      decrypted.resize(dec_len);

      // Verify the original plaintext is the same as the decrypted one.
      EXPECT_EQ(Bytes(plaintext), Bytes(decrypted));
    }
  }
}
