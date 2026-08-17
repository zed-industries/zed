// Copyright (c) 2014, Google Inc.
// SPDX-License-Identifier: ISC

#include <stdint.h>
#include <string.h>

#include <vector>

#include <gtest/gtest.h>

#include <openssl/aead.h>
#include <openssl/cipher.h>
#include <openssl/err.h>

#include "../fipsmodule/cipher/internal.h"
#include "../internal.h"
#include "../test/abi_test.h"
#include "../test/file_test.h"
#include "../test/test_util.h"
#include "../test/wycheproof_util.h"
#include "./internal.h"
#include "internal.h"

// kLimitedImplementation indicates that tests that assume a generic AEAD
// interface should not be performed. For example, the key-wrap AEADs only
// handle inputs that are a multiple of eight bytes in length and the TLS CBC
// AEADs have the concept of “direction”.
constexpr uint32_t kLimitedImplementation = 1 << 0;
// kCanTruncateTags indicates that the AEAD supports truncatating tags to
// arbitrary lengths.
constexpr uint32_t kCanTruncateTags = 1 << 1;
// kVariableNonce indicates that the AEAD supports a variable-length nonce.
constexpr uint32_t kVariableNonce = 1 << 2;
// kNondeterministic indicates that the AEAD performs randomised encryption thus
// one cannot assume that encrypting the same data will result in the same
// ciphertext.
constexpr uint32_t kNondeterministic = 1 << 7;

// RequiresADLength encodes an AD length requirement into flags.
constexpr uint32_t RequiresADLength(size_t length) {
  // If we had a more recent C++ version we could assert that the length is
  // sufficiently small with:
  //
  // if (length >= 16) {
  //  __builtin_unreachable();
  // }
  return (length & 0xf) << 3;
}

// RequiredADLength returns the AD length requirement encoded in |flags|, or
// zero if there isn't one.
constexpr size_t RequiredADLength(uint32_t flags) { return (flags >> 3) & 0xf; }

constexpr uint32_t RequiresMinimumTagLength(size_t length) {
  // See above for statically checking the size at compile time with future C++
  // versions.
  return (length & 0xf) << 8;
}

constexpr size_t MinimumTagLength(uint32_t flags) {
  return ((flags >> 8) & 0xf) == 0 ? 1 : ((flags >> 8) & 0xf);
}

struct KnownAEAD {
  const char name[40];
  const EVP_AEAD *(*func)(void);
  const char *test_vectors;
  uint32_t flags;
};

static const struct KnownAEAD kAEADs[] = {
    {"AES_128_GCM", EVP_aead_aes_128_gcm, "aes_128_gcm_tests.txt",
     kCanTruncateTags | kVariableNonce},

    {"AES_128_GCM_NIST", EVP_aead_aes_128_gcm, "nist_cavp/aes_128_gcm.txt",
     kCanTruncateTags | kVariableNonce},

    {"AES_192_GCM", EVP_aead_aes_192_gcm, "aes_192_gcm_tests.txt",
     kCanTruncateTags | kVariableNonce},

    {"AES_256_GCM", EVP_aead_aes_256_gcm, "aes_256_gcm_tests.txt",
     kCanTruncateTags | kVariableNonce},

    {"AES_256_GCM_NIST", EVP_aead_aes_256_gcm, "nist_cavp/aes_256_gcm.txt",
     kCanTruncateTags | kVariableNonce},

    {"AES_128_GCM_SIV", EVP_aead_aes_128_gcm_siv, "aes_128_gcm_siv_tests.txt",
     0},

    {"AES_256_GCM_SIV", EVP_aead_aes_256_gcm_siv, "aes_256_gcm_siv_tests.txt",
     0},

    {"AES_128_GCM_RandomNonce", EVP_aead_aes_128_gcm_randnonce,
     "aes_128_gcm_randnonce_tests.txt",
     kNondeterministic | kCanTruncateTags | RequiresMinimumTagLength(13)},

    {"AES_256_GCM_RandomNonce", EVP_aead_aes_256_gcm_randnonce,
     "aes_256_gcm_randnonce_tests.txt",
     kNondeterministic | kCanTruncateTags | RequiresMinimumTagLength(13)},

    {"ChaCha20Poly1305", EVP_aead_chacha20_poly1305,
     "chacha20_poly1305_tests.txt", kCanTruncateTags},

    {"XChaCha20Poly1305", EVP_aead_xchacha20_poly1305,
     "xchacha20_poly1305_tests.txt", kCanTruncateTags},

    {"AES_128_CBC_SHA1_TLS", EVP_aead_aes_128_cbc_sha1_tls,
     "aes_128_cbc_sha1_tls_tests.txt",
     kLimitedImplementation | RequiresADLength(11)},

    {"AES_128_CBC_SHA1_TLSImplicitIV",
     EVP_aead_aes_128_cbc_sha1_tls_implicit_iv,
     "aes_128_cbc_sha1_tls_implicit_iv_tests.txt",
     kLimitedImplementation | RequiresADLength(11)},

    {"AES_128_CBC_SHA256_TLS", EVP_aead_aes_128_cbc_sha256_tls,
     "aes_128_cbc_sha256_tls_tests.txt",
     kLimitedImplementation | RequiresADLength(11)},

    {"AES_128_CBC_SHA256_TLSImplicitIV",
     EVP_aead_aes_128_cbc_sha256_tls_implicit_iv,
     "aes_128_cbc_sha256_tls_implicit_iv_tests.txt",
     kLimitedImplementation | RequiresADLength(11)},

  {"AES_256_CBC_SHA384_TLS", EVP_aead_aes_256_cbc_sha384_tls,
     "aes_256_cbc_sha384_tls_tests.txt",
     kLimitedImplementation | RequiresADLength(11)},

    {"AES_256_CBC_SHA1_TLS", EVP_aead_aes_256_cbc_sha1_tls,
     "aes_256_cbc_sha1_tls_tests.txt",
     kLimitedImplementation | RequiresADLength(11)},

    {"AES_256_CBC_SHA1_TLSImplicitIV",
     EVP_aead_aes_256_cbc_sha1_tls_implicit_iv,
     "aes_256_cbc_sha1_tls_implicit_iv_tests.txt",
     kLimitedImplementation | RequiresADLength(11)},

    {"DES_EDE3_CBC_SHA1_TLS", EVP_aead_des_ede3_cbc_sha1_tls,
     "des_ede3_cbc_sha1_tls_tests.txt",
     kLimitedImplementation | RequiresADLength(11)},

    {"DES_EDE3_CBC_SHA1_TLSImplicitIV",
     EVP_aead_des_ede3_cbc_sha1_tls_implicit_iv,
     "des_ede3_cbc_sha1_tls_implicit_iv_tests.txt",
     kLimitedImplementation | RequiresADLength(11)},

    {"AES_128_CTR_HMAC_SHA256", EVP_aead_aes_128_ctr_hmac_sha256,
     "aes_128_ctr_hmac_sha256.txt", kCanTruncateTags},

    {"AES_256_CTR_HMAC_SHA256", EVP_aead_aes_256_ctr_hmac_sha256,
     "aes_256_ctr_hmac_sha256.txt", kCanTruncateTags},

    {"AES_128_CCM_BLUETOOTH", EVP_aead_aes_128_ccm_bluetooth,
     "aes_128_ccm_bluetooth_tests.txt", 0},

    {"AES_128_CCM_BLUETOOTH_8", EVP_aead_aes_128_ccm_bluetooth_8,
     "aes_128_ccm_bluetooth_8_tests.txt", 0},

    {"AES_128_CCM_Matter", EVP_aead_aes_128_ccm_matter,
     "aes_128_ccm_matter_tests.txt", 0},
};

class PerAEADTest : public testing::TestWithParam<KnownAEAD> {
 public:
  const EVP_AEAD *aead() { return GetParam().func(); }
};

INSTANTIATE_TEST_SUITE_P(All, PerAEADTest, testing::ValuesIn(kAEADs),
                         [](const testing::TestParamInfo<KnownAEAD> &params)
                             -> std::string { return params.param.name; });

// Tests an AEAD against a series of test vectors from a file, using the
// FileTest format. As an example, here's a valid test case:
//
//   KEY: 5a19f3173586b4c42f8412f4d5a786531b3231753e9e00998aec12fda8df10e4
//   NONCE: 978105dfce667bf4
//   IN: 6a4583908d
//   AD: b654574932
//   CT: 5294265a60
//   TAG: 1d45758621762e061368e68868e2f929
TEST_P(PerAEADTest, TestVector) {
  std::string test_vectors = "crypto/cipher_extra/test/";
  test_vectors += GetParam().test_vectors;
  FileTestGTest(test_vectors.c_str(), [&](FileTest *t) {
    std::vector<uint8_t> key, nonce, in, ad, ct, tag;
    ASSERT_TRUE(t->GetBytes(&key, "KEY"));
    ASSERT_TRUE(t->GetBytes(&nonce, "NONCE"));
    ASSERT_TRUE(t->GetBytes(&in, "IN"));
    ASSERT_TRUE(t->GetBytes(&ad, "AD"));
    ASSERT_TRUE(t->GetBytes(&ct, "CT"));
    ASSERT_TRUE(t->GetBytes(&tag, "TAG"));
    size_t tag_len = tag.size();
    if (t->HasAttribute("TAG_LEN")) {
      // Legacy AEADs are MAC-then-encrypt and may include padding in the TAG
      // field. TAG_LEN contains the actual size of the digest in that case.
      std::string tag_len_str;
      ASSERT_TRUE(t->GetAttribute(&tag_len_str, "TAG_LEN"));
      tag_len = strtoul(tag_len_str.c_str(), nullptr, 10);
      ASSERT_TRUE(tag_len);
    }

    bssl::ScopedEVP_AEAD_CTX ctx;
    ASSERT_TRUE(EVP_AEAD_CTX_init_with_direction(
        ctx.get(), aead(), key.data(), key.size(), tag_len, evp_aead_seal));

    std::vector<uint8_t> out(in.size() + EVP_AEAD_max_overhead(aead()));
    if (!t->HasAttribute("NO_SEAL") &&
        !(GetParam().flags & kNondeterministic)) {
      size_t out_len;
      ASSERT_TRUE(EVP_AEAD_CTX_seal(ctx.get(), out.data(), &out_len, out.size(),
                                    nonce.data(), nonce.size(), in.data(),
                                    in.size(), ad.data(), ad.size()));
      out.resize(out_len);

      ASSERT_EQ(out.size(), ct.size() + tag.size());
      EXPECT_EQ(Bytes(ct), Bytes(out.data(), ct.size()));
      EXPECT_EQ(Bytes(tag), Bytes(out.data() + ct.size(), tag.size()));
    } else {
      out.resize(ct.size() + tag.size());
      OPENSSL_memcpy(out.data(), ct.data(), ct.size());
      OPENSSL_memcpy(out.data() + ct.size(), tag.data(), tag.size());
    }

    // The "stateful" AEADs for implementing pre-AEAD cipher suites need to be
    // reset after each operation.
    ctx.Reset();
    ASSERT_TRUE(EVP_AEAD_CTX_init_with_direction(
        ctx.get(), aead(), key.data(), key.size(), tag_len, evp_aead_open));

    std::vector<uint8_t> out2(out.size());
    size_t out2_len;
    int ret = EVP_AEAD_CTX_open(ctx.get(), out2.data(), &out2_len, out2.size(),
                                nonce.data(), nonce.size(), out.data(),
                                out.size(), ad.data(), ad.size());
    if (t->HasAttribute("FAILS")) {
      ASSERT_FALSE(ret) << "Decrypted bad data.";
      ERR_clear_error();
      return;
    }

    ASSERT_TRUE(ret) << "Failed to decrypt.";
    out2.resize(out2_len);
    EXPECT_EQ(Bytes(in), Bytes(out2));

    // The "stateful" AEADs for implementing pre-AEAD cipher suites need to be
    // reset after each operation.
    ctx.Reset();
    ASSERT_TRUE(EVP_AEAD_CTX_init_with_direction(
        ctx.get(), aead(), key.data(), key.size(), tag_len, evp_aead_open));

    // Garbage at the end isn't ignored.
    out.push_back(0);
    out2.resize(out.size());
    EXPECT_FALSE(EVP_AEAD_CTX_open(
        ctx.get(), out2.data(), &out2_len, out2.size(), nonce.data(),
        nonce.size(), out.data(), out.size(), ad.data(), ad.size()))
        << "Decrypted bad data with trailing garbage.";
    ERR_clear_error();

    // The "stateful" AEADs for implementing pre-AEAD cipher suites need to be
    // reset after each operation.
    ctx.Reset();
    ASSERT_TRUE(EVP_AEAD_CTX_init_with_direction(
        ctx.get(), aead(), key.data(), key.size(), tag_len, evp_aead_open));

    // Verify integrity is checked.
    out[0] ^= 0x80;
    out.resize(out.size() - 1);
    out2.resize(out.size());
    EXPECT_FALSE(EVP_AEAD_CTX_open(
        ctx.get(), out2.data(), &out2_len, out2.size(), nonce.data(),
        nonce.size(), out.data(), out.size(), ad.data(), ad.size()))
        << "Decrypted bad data with corrupted byte.";
    ERR_clear_error();
  });
}

struct KnownTLSLegacyAEAD {
  const char name[40];
  const EVP_CIPHER *(*func)(void);
  const char *test_vectors;
  uint32_t flags;
};

static const struct KnownTLSLegacyAEAD kTLSLegacyAEADs[] = {
    {"AES_128_CBC_SHA1_TLS", EVP_aes_128_cbc_hmac_sha1,
     "aes_128_cbc_sha1_tls_stitch_tests.txt",
     RequiresADLength(EVP_AEAD_TLS1_AAD_LEN)},

    {"AES_128_CBC_SHA1_TLS_IMPLICIT_IV", EVP_aes_128_cbc_hmac_sha1,
     "aes_128_cbc_sha1_tls_stitch_implicit_iv_tests.txt",
     RequiresADLength(EVP_AEAD_TLS1_AAD_LEN)},

    {"AES_128_CBC_SHA256_TLS", EVP_aes_128_cbc_hmac_sha256,
     "aes_128_cbc_sha256_tls_stitch_tests.txt",
     RequiresADLength(EVP_AEAD_TLS1_AAD_LEN)},

    {"AES_128_CBC_SHA256_TLS_IMPLICIT_IV", EVP_aes_128_cbc_hmac_sha256,
     "aes_128_cbc_sha256_tls_stitch_implicit_iv_tests.txt",
     RequiresADLength(EVP_AEAD_TLS1_AAD_LEN)},

    {"AES_256_CBC_SHA1_TLS", EVP_aes_256_cbc_hmac_sha1,
     "aes_256_cbc_sha1_tls_stitch_tests.txt",
     RequiresADLength(EVP_AEAD_TLS1_AAD_LEN)},

    {"AES_256_CBC_SHA1_TLS_IMPLICIT_IV", EVP_aes_256_cbc_hmac_sha1,
     "aes_256_cbc_sha1_tls_stitch_implicit_iv_tests.txt",
     RequiresADLength(EVP_AEAD_TLS1_AAD_LEN)},

    {"AES_256_CBC_SHA256_TLS", EVP_aes_256_cbc_hmac_sha256,
     "aes_256_cbc_sha256_tls_stitch_tests.txt",
     RequiresADLength(EVP_AEAD_TLS1_AAD_LEN)},

    {"AES_256_CBC_SHA256_TLS_IMPLICIT_IV", EVP_aes_256_cbc_hmac_sha256,
     "aes_256_cbc_sha256_tls_stitch_implicit_iv_tests.txt",
     RequiresADLength(EVP_AEAD_TLS1_AAD_LEN)},
};

class PerTLSLegacyAEADTest : public testing::TestWithParam<KnownTLSLegacyAEAD> {
};

INSTANTIATE_TEST_SUITE_P(
    All, PerTLSLegacyAEADTest, testing::ValuesIn(kTLSLegacyAEADs),
    [](const testing::TestParamInfo<KnownTLSLegacyAEAD> &params)
        -> std::string { return params.param.name; });

static void set_MAC_key(EVP_CIPHER_CTX *ctx, uint8_t *key,
                        size_t mac_key_size) {
  // In each TLS session, only need to set EVP_CTRL_AEAD_SET_MAC_KEY once.
  ASSERT_TRUE(
      EVP_CIPHER_CTX_ctrl(ctx, EVP_CTRL_AEAD_SET_MAC_KEY, mac_key_size, key))
      << "EVP_CTRL_AEAD_SET_MAC_KEY failed: "
      << ERR_reason_error_string(ERR_get_error());
}

static void set_TLS1_AAD(EVP_CIPHER_CTX *ctx, uint8_t *ad) {
  // In each TLS session, ad should be set before each read/write operation
  // because it includes sequence_num. Below ctrl returns hmac|pad len for
  // encryption. For decryption, it returns digest size.
  EVP_CIPHER_CTX_ctrl(ctx, EVP_CTRL_AEAD_TLS1_AAD, EVP_AEAD_TLS1_AAD_LEN, ad);
}

// |EVP_aes_128/256_cbc_hmac_sha1/256| tests.
// Tests a TLS specific legacy AEAD against a series of test vectors from a
// file, using the FileTest format. As an example, here's a valid test case:
//
// # DIGEST: 918c0df73de553b5bdffb7365f93a430292f6eea
// KEY: 2993a340b9b3c589c7481df3f4183aa23fd8d7efd88503f78b8ed1c8e9ba2fd6773e0d0c
// NONCE: 302a5f47e037446f5891d77df660ed82
// IN: 302a5f47e037446f5891d77df660ed8286d641b877
// AD: 97b684e0fb56f97c3903020015
// CT: 6854710b76c79c9be84b3669fc4f05b17e67a9cc14
// TAG: 9c6998bf3b172670c5f8613f479641468bbed4c68d093940c92de4
// TAG_LEN: 20
TEST_P(PerTLSLegacyAEADTest, TestVector) {
  const EVP_CIPHER *cipher = GetParam().func();
  if (!cipher) {
    // This condition can happen when
    // 1. !defined(AES_CBC_HMAC_SHA_STITCH).
    // 2. |hwaes_capable| returns false.
    //    -- Checked by Intel SDE. CPU is "mrm", "nhm", "pnr", "slt" and "p4p".
    return;
  }
  std::string test_vectors = "crypto/cipher_extra/test/";
  test_vectors += GetParam().test_vectors;
  FileTestGTest(test_vectors.c_str(), [&](FileTest *t) {
    std::vector<uint8_t> key, nonce, in, ad, ct, tag;
    ASSERT_TRUE(t->GetBytes(&key, "KEY"));
    ASSERT_TRUE(t->GetBytes(&nonce, "NONCE"));
    ASSERT_TRUE(t->GetBytes(&in, "IN"));
    ASSERT_TRUE(t->GetBytes(&ad, "AD"));
    ASSERT_TRUE(t->GetBytes(&ct, "CT"));
    ASSERT_TRUE(t->GetBytes(&tag, "TAG"));
    size_t tag_len = tag.size();
    if (t->HasAttribute("TAG_LEN")) {
      // Legacy AEADs are MAC-then-encrypt and may include padding in the TAG
      // field. TAG_LEN contains the actual size of the digest in that case.
      std::string tag_len_str;
      ASSERT_TRUE(t->GetAttribute(&tag_len_str, "TAG_LEN"));
      tag_len = strtoul(tag_len_str.c_str(), nullptr, 10);
      ASSERT_TRUE(tag_len);
    }
    bool explicit_iv = !nonce.empty();
    size_t aes_block_size = EVP_CIPHER_block_size(cipher);
    ;
    size_t key_block_size = EVP_CIPHER_key_length(cipher);
    size_t e_iv_len = 0;
    bssl::ScopedEVP_CIPHER_CTX ctx;

    std::vector<uint8_t> encrypted(ct.size() + tag.size());
    // The |key| is Mac key + AES key + IV.
    size_t mac_key_size = tag_len;
    const uint8_t *aes_key = key.data() + mac_key_size;
    std::vector<uint8_t> iv(aes_block_size);
    if (explicit_iv) {
      e_iv_len = aes_block_size;
      OPENSSL_memcpy(iv.data(), nonce.data(), nonce.size());
    } else {
      OPENSSL_memcpy(iv.data(), key.data() + mac_key_size + key_block_size,
                     aes_block_size);
    }
    if (!t->HasAttribute("NO_SEAL")) {
      // Even without |EVP_CIPHER_CTX_set_padding|, |EVP_Cipher| returns error
      // code because |EVP_aes_128/256_cbc_hmac_sha1/256| does not automatically
      // pad the input.
      ASSERT_TRUE(EVP_CIPHER_CTX_set_padding(ctx.get(), EVP_CIPH_NO_PADDING));
      ASSERT_TRUE(
          EVP_EncryptInit_ex(ctx.get(), cipher, nullptr, aes_key, iv.data()));
      set_MAC_key(ctx.get(), key.data(), mac_key_size);
      // |EVP_aes_128/256_cbc_hmac_sha1/256| encrypts a TLS record, which should
      // have space for explicit_iv(if applicable), payload, tag(hmac and
      // padding).
      std::vector<uint8_t> record(in.size() + tag.size(), 0);
      OPENSSL_memcpy(record.data(), in.data(), in.size());
      set_TLS1_AAD(ctx.get(), ad.data());
      ASSERT_TRUE(
          EVP_Cipher(ctx.get(), encrypted.data(), record.data(), record.size()))
          << "EVP_Cipher encryption failed: "
          << ERR_reason_error_string(ERR_get_error());
      EXPECT_EQ(Bytes(ct), Bytes(encrypted.data(), ct.size()));
      EXPECT_EQ(Bytes(tag), Bytes(encrypted.data() + ct.size(), tag.size()));
    } else {
      encrypted.resize(ct.size() + tag.size());
      OPENSSL_memcpy(encrypted.data(), ct.data(), ct.size());
      OPENSSL_memcpy(encrypted.data() + ct.size(), tag.data(), tag.size());
    }

    // Decryption side(TLS client/server) always has a separated EVP_CIPHER_CTX.
    bssl::ScopedEVP_CIPHER_CTX decrypt_ctx;
    ASSERT_TRUE(
        EVP_CIPHER_CTX_set_padding(decrypt_ctx.get(), EVP_CIPH_NO_PADDING));
    ASSERT_TRUE(EVP_DecryptInit_ex(decrypt_ctx.get(), cipher, nullptr, aes_key,
                                   iv.data()));
    set_MAC_key(decrypt_ctx.get(), key.data(), mac_key_size);
    set_TLS1_AAD(decrypt_ctx.get(), ad.data());
    std::vector<uint8_t> decrypted(encrypted.size());
    int ret = EVP_Cipher(decrypt_ctx.get(), decrypted.data(), encrypted.data(),
                         encrypted.size());
    if (t->HasAttribute("FAILS")) {
      ASSERT_TRUE(ret <= 0) << "Decrypted bad data.";
      ERR_clear_error();
      return;
    }

    // Integrity check.
    ASSERT_EQ(ret, 1) << "EVP_Cipher integrity check failed.";
    // Check payload data.
    size_t payload_len = in.size() - e_iv_len;
    std::vector<uint8_t> expect(payload_len, 0);
    OPENSSL_memcpy(expect.data(), in.data() + e_iv_len, payload_len);
    std::vector<uint8_t> actual(payload_len, 0);
    OPENSSL_memcpy(actual.data(), decrypted.data() + e_iv_len, payload_len);
    ASSERT_EQ(Bytes(expect), Bytes(actual));
    // Integrity check after modifying some bytes of the cipher text.
    // Some |in| only include explicit iv(if applicable) without payload.
    if (in.size() > e_iv_len) {
      // Modify the cipher text.
      encrypted[EVP_CIPHER_block_size(cipher)] ^= 0x80;
      set_TLS1_AAD(decrypt_ctx.get(), ad.data());
      ASSERT_FALSE(EVP_Cipher(decrypt_ctx.get(), decrypted.data(),
                              encrypted.data(), encrypted.size()));
      ASSERT_EQ(ERR_GET_REASON(ERR_get_error()), CIPHER_R_BAD_DECRYPT);
      ERR_clear_error();
      // Recover the cipher text.
      encrypted[EVP_CIPHER_block_size(cipher)] ^= 0x80;
    }

    // |EVP_aes_128/256_cbc_hmac_sha1/256| requires the input to be the integral
    // multiple of AES_BLOCK_SIZE.
    encrypted.resize(encrypted.size() + 1);
    set_TLS1_AAD(decrypt_ctx.get(), ad.data());
    ASSERT_FALSE(EVP_Cipher(decrypt_ctx.get(), decrypted.data(),
                            encrypted.data(), encrypted.size()));
    ASSERT_EQ(ERR_GET_REASON(ERR_get_error()),
              CIPHER_R_DATA_NOT_MULTIPLE_OF_BLOCK_LENGTH);
    ERR_clear_error();

    // Garbage at the end isn't ignored.
    encrypted.resize(encrypted.size() + EVP_CIPHER_block_size(cipher) - 1);
    decrypted.resize(encrypted.size());
    set_TLS1_AAD(decrypt_ctx.get(), ad.data());
    ASSERT_FALSE(EVP_Cipher(decrypt_ctx.get(), decrypted.data(),
                            encrypted.data(), encrypted.size()));
    ASSERT_EQ(ERR_GET_REASON(ERR_get_error()), CIPHER_R_BAD_DECRYPT);
    ERR_clear_error();
  });
}

TEST_P(PerAEADTest, TestExtraInput) {
  const KnownAEAD &aead_config = GetParam();
  if (!aead()->seal_scatter_supports_extra_in) {
    return;
  }

  const std::string test_vectors =
      "crypto/cipher_extra/test/" + std::string(aead_config.test_vectors);
  FileTestGTest(test_vectors.c_str(), [&](FileTest *t) {
    if (t->HasAttribute("NO_SEAL") || t->HasAttribute("FAILS") ||
        (aead_config.flags & kNondeterministic)) {
      t->SkipCurrent();
      return;
    }

    std::vector<uint8_t> key, nonce, in, ad, ct, tag;
    ASSERT_TRUE(t->GetBytes(&key, "KEY"));
    ASSERT_TRUE(t->GetBytes(&nonce, "NONCE"));
    ASSERT_TRUE(t->GetBytes(&in, "IN"));
    ASSERT_TRUE(t->GetBytes(&ad, "AD"));
    ASSERT_TRUE(t->GetBytes(&ct, "CT"));
    ASSERT_TRUE(t->GetBytes(&tag, "TAG"));

    bssl::ScopedEVP_AEAD_CTX ctx;
    ASSERT_TRUE(EVP_AEAD_CTX_init(ctx.get(), aead(), key.data(), key.size(),
                                  tag.size(), nullptr));
    std::vector<uint8_t> out_tag(EVP_AEAD_max_overhead(aead()) + in.size());
    std::vector<uint8_t> out(in.size());

    for (size_t extra_in_size = 0; extra_in_size < in.size(); extra_in_size++) {
      size_t tag_bytes_written;
      SCOPED_TRACE(extra_in_size);
      ASSERT_TRUE(EVP_AEAD_CTX_seal_scatter(
          ctx.get(), out.data(), out_tag.data(), &tag_bytes_written,
          out_tag.size(), nonce.data(), nonce.size(), in.data(),
          in.size() - extra_in_size, in.data() + in.size() - extra_in_size,
          extra_in_size, ad.data(), ad.size()));

      ASSERT_EQ(tag_bytes_written, extra_in_size + tag.size());

      memcpy(out.data() + in.size() - extra_in_size, out_tag.data(),
             extra_in_size);

      EXPECT_EQ(Bytes(ct), Bytes(out.data(), in.size()));
      EXPECT_EQ(Bytes(tag), Bytes(out_tag.data() + extra_in_size,
                                  tag_bytes_written - extra_in_size));
    }
  });
}

TEST_P(PerAEADTest, TestVectorScatterGather) {
  std::string test_vectors = "crypto/cipher_extra/test/";
  const KnownAEAD &aead_config = GetParam();
  test_vectors += aead_config.test_vectors;
  FileTestGTest(test_vectors.c_str(), [&](FileTest *t) {
    std::vector<uint8_t> key, nonce, in, ad, ct, tag;
    ASSERT_TRUE(t->GetBytes(&key, "KEY"));
    ASSERT_TRUE(t->GetBytes(&nonce, "NONCE"));
    ASSERT_TRUE(t->GetBytes(&in, "IN"));
    ASSERT_TRUE(t->GetBytes(&ad, "AD"));
    ASSERT_TRUE(t->GetBytes(&ct, "CT"));
    ASSERT_TRUE(t->GetBytes(&tag, "TAG"));
    size_t tag_len = tag.size();
    if (t->HasAttribute("TAG_LEN")) {
      // Legacy AEADs are MAC-then-encrypt and may include padding in the TAG
      // field. TAG_LEN contains the actual size of the digest in that case.
      std::string tag_len_str;
      ASSERT_TRUE(t->GetAttribute(&tag_len_str, "TAG_LEN"));
      tag_len = strtoul(tag_len_str.c_str(), nullptr, 10);
      ASSERT_TRUE(tag_len);
    }

    bssl::ScopedEVP_AEAD_CTX ctx;
    ASSERT_TRUE(EVP_AEAD_CTX_init_with_direction(
        ctx.get(), aead(), key.data(), key.size(), tag_len, evp_aead_seal));

    std::vector<uint8_t> out(in.size());
    std::vector<uint8_t> out_tag(EVP_AEAD_max_overhead(aead()));
    if (!t->HasAttribute("NO_SEAL") &&
        !(aead_config.flags & kNondeterministic)) {
      size_t out_tag_len;
      ASSERT_TRUE(EVP_AEAD_CTX_seal_scatter(
          ctx.get(), out.data(), out_tag.data(), &out_tag_len, out_tag.size(),
          nonce.data(), nonce.size(), in.data(), in.size(), nullptr, 0,
          ad.data(), ad.size()));
      out_tag.resize(out_tag_len);

      ASSERT_EQ(out.size(), ct.size());
      ASSERT_EQ(out_tag.size(), tag.size());
      EXPECT_EQ(Bytes(ct), Bytes(out.data(), ct.size()));
      EXPECT_EQ(Bytes(tag), Bytes(out_tag.data(), tag.size()));
    } else {
      out.resize(ct.size());
      out_tag.resize(tag.size());
      OPENSSL_memcpy(out.data(), ct.data(), ct.size());
      OPENSSL_memcpy(out_tag.data(), tag.data(), tag.size());
    }

    // The "stateful" AEADs for implementing pre-AEAD cipher suites need to be
    // reset after each operation.
    ctx.Reset();
    ASSERT_TRUE(EVP_AEAD_CTX_init_with_direction(
        ctx.get(), aead(), key.data(), key.size(), tag_len, evp_aead_open));

    std::vector<uint8_t> out2(out.size());
    int ret = EVP_AEAD_CTX_open_gather(
        ctx.get(), out2.data(), nonce.data(), nonce.size(), out.data(),
        out.size(), out_tag.data(), out_tag.size(), ad.data(), ad.size());

    // Skip decryption for AEADs that don't implement open_gather().
    if (!ret) {
      uint32_t err = ERR_peek_error();
      if (ERR_GET_LIB(err) == ERR_LIB_CIPHER &&
          ERR_GET_REASON(err) == CIPHER_R_CTRL_NOT_IMPLEMENTED) {
        t->SkipCurrent();
        return;
      }
    }

    if (t->HasAttribute("FAILS")) {
      ASSERT_FALSE(ret) << "Decrypted bad data";
      ERR_clear_error();
      return;
    }

    ASSERT_TRUE(ret) << "Failed to decrypt: "
                     << ERR_reason_error_string(ERR_get_error());
    EXPECT_EQ(Bytes(in), Bytes(out2));

    // The "stateful" AEADs for implementing pre-AEAD cipher suites need to be
    // reset after each operation.
    ctx.Reset();
    ASSERT_TRUE(EVP_AEAD_CTX_init_with_direction(
        ctx.get(), aead(), key.data(), key.size(), tag_len, evp_aead_open));

    // Garbage at the end isn't ignored.
    out_tag.push_back(0);
    out2.resize(out.size());
    EXPECT_FALSE(EVP_AEAD_CTX_open_gather(
        ctx.get(), out2.data(), nonce.data(), nonce.size(), out.data(),
        out.size(), out_tag.data(), out_tag.size(), ad.data(), ad.size()))
        << "Decrypted bad data with trailing garbage.";
    ERR_clear_error();

    // The "stateful" AEADs for implementing pre-AEAD cipher suites need to be
    // reset after each operation.
    ctx.Reset();
    ASSERT_TRUE(EVP_AEAD_CTX_init_with_direction(
        ctx.get(), aead(), key.data(), key.size(), tag_len, evp_aead_open));

    // Verify integrity is checked.
    out_tag[0] ^= 0x80;
    out_tag.resize(out_tag.size() - 1);
    out2.resize(out.size());
    EXPECT_FALSE(EVP_AEAD_CTX_open_gather(
        ctx.get(), out2.data(), nonce.data(), nonce.size(), out.data(),
        out.size(), out_tag.data(), out_tag.size(), ad.data(), ad.size()))
        << "Decrypted bad data with corrupted byte.";
    ERR_clear_error();

    ctx.Reset();
    ASSERT_TRUE(EVP_AEAD_CTX_init_with_direction(
        ctx.get(), aead(), key.data(), key.size(), tag_len, evp_aead_open));

    // Check edge case for tag length.
    EXPECT_FALSE(EVP_AEAD_CTX_open_gather(
        ctx.get(), out2.data(), nonce.data(), nonce.size(), out.data(),
        out.size(), out_tag.data(), 0, ad.data(), ad.size()))
        << "Decrypted bad data with corrupted byte.";
    ERR_clear_error();
  });
}

TEST_P(PerAEADTest, CleanupAfterInitFailure) {
  uint8_t key[EVP_AEAD_MAX_KEY_LENGTH];
  OPENSSL_memset(key, 0, sizeof(key));
  const size_t key_len = EVP_AEAD_key_length(aead());
  ASSERT_GE(sizeof(key), key_len);

  EVP_AEAD_CTX ctx;
  ASSERT_FALSE(EVP_AEAD_CTX_init(
      &ctx, aead(), key, key_len,
      9999 /* a silly tag length to trigger an error */, NULL /* ENGINE */));
  ERR_clear_error();

  // Running a second, failed _init should not cause a memory leak.
  ASSERT_FALSE(EVP_AEAD_CTX_init(
      &ctx, aead(), key, key_len,
      9999 /* a silly tag length to trigger an error */, NULL /* ENGINE */));
  ERR_clear_error();

  // Calling _cleanup on an |EVP_AEAD_CTX| after a failed _init should be a
  // no-op.
  EVP_AEAD_CTX_cleanup(&ctx);
}

TEST_P(PerAEADTest, TruncatedTags) {
  if (!(GetParam().flags & kCanTruncateTags)) {
    return;
  }

  uint8_t key[EVP_AEAD_MAX_KEY_LENGTH];
  OPENSSL_memset(key, 0, sizeof(key));
  const size_t key_len = EVP_AEAD_key_length(aead());
  ASSERT_GE(sizeof(key), key_len);

  uint8_t nonce[EVP_AEAD_MAX_NONCE_LENGTH];
  OPENSSL_memset(nonce, 0, sizeof(nonce));
  const size_t nonce_len = EVP_AEAD_nonce_length(aead());
  ASSERT_GE(sizeof(nonce), nonce_len);

  const size_t tag_len = MinimumTagLength(GetParam().flags);
  bssl::ScopedEVP_AEAD_CTX ctx;
  ASSERT_TRUE(EVP_AEAD_CTX_init(ctx.get(), aead(), key, key_len, tag_len,
                                NULL /* ENGINE */));

  const uint8_t plaintext[1] = {'A'};

  uint8_t ciphertext[128];
  size_t ciphertext_len;
  constexpr uint8_t kSentinel = 42;
  OPENSSL_memset(ciphertext, kSentinel, sizeof(ciphertext));

  ASSERT_TRUE(EVP_AEAD_CTX_seal(ctx.get(), ciphertext, &ciphertext_len,
                                sizeof(ciphertext), nonce, nonce_len, plaintext,
                                sizeof(plaintext), nullptr /* ad */, 0));

  for (size_t i = ciphertext_len; i < sizeof(ciphertext); i++) {
    // Sealing must not write past where it said it did.
    EXPECT_EQ(kSentinel, ciphertext[i])
        << "Sealing wrote off the end of the buffer.";
  }

  const size_t overhead_used = ciphertext_len - sizeof(plaintext);
  const size_t expected_overhead =
      tag_len + EVP_AEAD_max_overhead(aead()) - EVP_AEAD_max_tag_len(aead());
  EXPECT_EQ(overhead_used, expected_overhead)
      << "AEAD is probably ignoring request to truncate tags.";

  uint8_t plaintext2[sizeof(plaintext) + 16];
  OPENSSL_memset(plaintext2, kSentinel, sizeof(plaintext2));

  size_t plaintext2_len;
  ASSERT_TRUE(EVP_AEAD_CTX_open(
      ctx.get(), plaintext2, &plaintext2_len, sizeof(plaintext2), nonce,
      nonce_len, ciphertext, ciphertext_len, nullptr /* ad */, 0))
      << "Opening with truncated tag didn't work.";

  for (size_t i = plaintext2_len; i < sizeof(plaintext2); i++) {
    // Likewise, opening should also stay within bounds.
    EXPECT_EQ(kSentinel, plaintext2[i])
        << "Opening wrote off the end of the buffer.";
  }

  EXPECT_EQ(Bytes(plaintext), Bytes(plaintext2, plaintext2_len));
}

TEST_P(PerAEADTest, AliasedBuffers) {
  if (GetParam().flags & kLimitedImplementation) {
    return;
  }

  const size_t key_len = EVP_AEAD_key_length(aead());
  const size_t nonce_len = EVP_AEAD_nonce_length(aead());
  const size_t max_overhead = EVP_AEAD_max_overhead(aead());

  std::vector<uint8_t> key(key_len, 'a');
  bssl::ScopedEVP_AEAD_CTX ctx;
  ASSERT_TRUE(EVP_AEAD_CTX_init(ctx.get(), aead(), key.data(), key_len,
                                EVP_AEAD_DEFAULT_TAG_LENGTH, nullptr));

  static const uint8_t kPlaintext[260] =
      "testing123456testing123456testing123456testing123456testing123456testing"
      "123456testing123456testing123456testing123456testing123456testing123456t"
      "esting123456testing123456testing123456testing123456testing123456testing1"
      "23456testing123456testing123456testing12345";
  const std::vector<size_t> offsets = {
      0,  1,  2,  8,  15, 16,  17,  31,  32,  33,  63,
      64, 65, 95, 96, 97, 127, 128, 129, 255, 256, 257,
  };

  std::vector<uint8_t> nonce(nonce_len, 'b');
  std::vector<uint8_t> valid_encryption(sizeof(kPlaintext) + max_overhead);
  size_t valid_encryption_len;
  ASSERT_TRUE(EVP_AEAD_CTX_seal(
      ctx.get(), valid_encryption.data(), &valid_encryption_len,
      sizeof(kPlaintext) + max_overhead, nonce.data(), nonce_len, kPlaintext,
      sizeof(kPlaintext), nullptr, 0))
      << "EVP_AEAD_CTX_seal failed with disjoint buffers.";

  // Test with out != in which we expect to fail.
  std::vector<uint8_t> buffer(2 + valid_encryption_len);
  uint8_t *in = buffer.data() + 1;
  uint8_t *out1 = buffer.data();
  uint8_t *out2 = buffer.data() + 2;

  OPENSSL_memcpy(in, kPlaintext, sizeof(kPlaintext));
  size_t out_len;
  EXPECT_FALSE(EVP_AEAD_CTX_seal(
      ctx.get(), out1 /* in - 1 */, &out_len, sizeof(kPlaintext) + max_overhead,
      nonce.data(), nonce_len, in, sizeof(kPlaintext), nullptr, 0));
  EXPECT_FALSE(EVP_AEAD_CTX_seal(
      ctx.get(), out2 /* in + 1 */, &out_len, sizeof(kPlaintext) + max_overhead,
      nonce.data(), nonce_len, in, sizeof(kPlaintext), nullptr, 0));
  ERR_clear_error();

  OPENSSL_memcpy(in, valid_encryption.data(), valid_encryption_len);
  EXPECT_FALSE(EVP_AEAD_CTX_open(ctx.get(), out1 /* in - 1 */, &out_len,
                                 valid_encryption_len, nonce.data(), nonce_len,
                                 in, valid_encryption_len, nullptr, 0));
  EXPECT_FALSE(EVP_AEAD_CTX_open(ctx.get(), out2 /* in + 1 */, &out_len,
                                 valid_encryption_len, nonce.data(), nonce_len,
                                 in, valid_encryption_len, nullptr, 0));
  ERR_clear_error();

  // Test with out == in, which we expect to work.
  OPENSSL_memcpy(in, kPlaintext, sizeof(kPlaintext));

  ASSERT_TRUE(EVP_AEAD_CTX_seal(ctx.get(), in, &out_len,
                                sizeof(kPlaintext) + max_overhead, nonce.data(),
                                nonce_len, in, sizeof(kPlaintext), nullptr, 0));

  if (!(GetParam().flags & kNondeterministic)) {
    EXPECT_EQ(Bytes(valid_encryption.data(), valid_encryption_len),
              Bytes(in, out_len));
  }

  OPENSSL_memcpy(in, valid_encryption.data(), valid_encryption_len);
  ASSERT_TRUE(EVP_AEAD_CTX_open(ctx.get(), in, &out_len, valid_encryption_len,
                                nonce.data(), nonce_len, in,
                                valid_encryption_len, nullptr, 0));
  EXPECT_EQ(Bytes(kPlaintext), Bytes(in, out_len));
}

#if defined(__BIGGEST_ALIGNMENT__)
#define UNALIGNED_TEST_ALIGNMENT __BIGGEST_ALIGNMENT__
#else
#define UNALIGNED_TEST_ALIGNMENT 8
#endif // defined(__BIGGEST_ALIGNMENT__)

TEST_P(PerAEADTest, UnalignedInput) {
  alignas(UNALIGNED_TEST_ALIGNMENT) uint8_t key[EVP_AEAD_MAX_KEY_LENGTH + 1];
  alignas(UNALIGNED_TEST_ALIGNMENT) uint8_t nonce[EVP_AEAD_MAX_NONCE_LENGTH + 1];
  alignas(UNALIGNED_TEST_ALIGNMENT) uint8_t plaintext[32 + 1];
  alignas(UNALIGNED_TEST_ALIGNMENT) uint8_t ad[32 + 1];
  OPENSSL_memset(key, 'K', sizeof(key));
  OPENSSL_memset(nonce, 'N', sizeof(nonce));
  OPENSSL_memset(plaintext, 'P', sizeof(plaintext));
  OPENSSL_memset(ad, 'A', sizeof(ad));
  const size_t key_len = EVP_AEAD_key_length(aead());
  ASSERT_GE(sizeof(key) - 1, key_len);
  const size_t nonce_len = EVP_AEAD_nonce_length(aead());
  ASSERT_GE(sizeof(nonce) - 1, nonce_len);
  const size_t ad_len = RequiredADLength(GetParam().flags) != 0
                            ? RequiredADLength(GetParam().flags)
                            : sizeof(ad) - 1;
  ASSERT_GE(sizeof(ad) - 1, ad_len);

  // Encrypt some input.
  bssl::ScopedEVP_AEAD_CTX ctx;
  ASSERT_TRUE(EVP_AEAD_CTX_init_with_direction(
      ctx.get(), aead(), key + 1, key_len, EVP_AEAD_DEFAULT_TAG_LENGTH,
      evp_aead_seal));
  alignas(UNALIGNED_TEST_ALIGNMENT) uint8_t ciphertext[sizeof(plaintext) + EVP_AEAD_MAX_OVERHEAD];
  size_t ciphertext_len;
  ASSERT_TRUE(EVP_AEAD_CTX_seal(ctx.get(), ciphertext + 1, &ciphertext_len,
                                sizeof(ciphertext) - 1, nonce + 1, nonce_len,
                                plaintext + 1, sizeof(plaintext) - 1, ad + 1,
                                ad_len));

  // It must successfully decrypt.
  alignas(UNALIGNED_TEST_ALIGNMENT) uint8_t out[sizeof(ciphertext)];
  ctx.Reset();
  ASSERT_TRUE(EVP_AEAD_CTX_init_with_direction(
      ctx.get(), aead(), key + 1, key_len, EVP_AEAD_DEFAULT_TAG_LENGTH,
      evp_aead_open));
  size_t out_len;
  ASSERT_TRUE(EVP_AEAD_CTX_open(ctx.get(), out + 1, &out_len, sizeof(out) - 1,
                                nonce + 1, nonce_len, ciphertext + 1,
                                ciphertext_len, ad + 1, ad_len));
  EXPECT_EQ(Bytes(plaintext + 1, sizeof(plaintext) - 1),
            Bytes(out + 1, out_len));
}

TEST_P(PerAEADTest, Overflow) {
  uint8_t key[EVP_AEAD_MAX_KEY_LENGTH];
  OPENSSL_memset(key, 'K', sizeof(key));

  bssl::ScopedEVP_AEAD_CTX ctx;
  const size_t max_tag_len = EVP_AEAD_max_tag_len(aead());
  ASSERT_TRUE(EVP_AEAD_CTX_init_with_direction(ctx.get(), aead(), key,
                                               EVP_AEAD_key_length(aead()),
                                               max_tag_len, evp_aead_seal));

  uint8_t plaintext[1] = {0};
  uint8_t ciphertext[1024] = {0};
  size_t ciphertext_len;
  // The AEAD must not overflow when calculating the ciphertext length.
  ASSERT_FALSE(EVP_AEAD_CTX_seal(
      ctx.get(), ciphertext, &ciphertext_len, sizeof(ciphertext), nullptr, 0,
      plaintext, std::numeric_limits<size_t>::max() - max_tag_len + 1, nullptr,
      0));
  ERR_clear_error();

  // (Can't test the scatter interface because it'll attempt to zero the output
  // buffer on error and the primary output buffer is implicitly the same size
  // as the input.)
}

TEST_P(PerAEADTest, InvalidNonceLength) {
  size_t valid_nonce_len = EVP_AEAD_nonce_length(aead());
  std::vector<size_t> nonce_lens;
  if (valid_nonce_len != 0) {
    // Other than the implicit IV TLS "AEAD"s, none of our AEADs allow empty
    // nonces. In particular, although AES-GCM was incorrectly specified with
    // variable-length nonces, it does not allow the empty nonce.
    nonce_lens.push_back(0);
  }
  if (!(GetParam().flags & kVariableNonce)) {
    nonce_lens.push_back(valid_nonce_len + 1);
    if (valid_nonce_len != 0) {
      nonce_lens.push_back(valid_nonce_len - 1);
    }
  }

  static const uint8_t kZeros[EVP_AEAD_MAX_KEY_LENGTH] = {0};
  const size_t ad_len = RequiredADLength(GetParam().flags) != 0
                            ? RequiredADLength(GetParam().flags)
                            : 16;
  ASSERT_LE(ad_len, sizeof(kZeros));

  for (size_t nonce_len : nonce_lens) {
    SCOPED_TRACE(nonce_len);
    uint8_t buf[256];
    size_t len;
    std::vector<uint8_t> nonce(nonce_len);
    bssl::ScopedEVP_AEAD_CTX ctx;
    ASSERT_TRUE(EVP_AEAD_CTX_init_with_direction(
        ctx.get(), aead(), kZeros, EVP_AEAD_key_length(aead()),
        EVP_AEAD_DEFAULT_TAG_LENGTH, evp_aead_seal));

    EXPECT_FALSE(EVP_AEAD_CTX_seal(ctx.get(), buf, &len, sizeof(buf),
                                   nonce.data(), nonce.size(), nullptr /* in */,
                                   0, kZeros /* ad */, ad_len));
    uint32_t err = ERR_get_error();
    // TODO(davidben): Merge these errors. https://crbug.com/boringssl/129.
    if (!ErrorEquals(err, ERR_LIB_CIPHER, CIPHER_R_UNSUPPORTED_NONCE_SIZE)) {
      EXPECT_TRUE(
          ErrorEquals(err, ERR_LIB_CIPHER, CIPHER_R_INVALID_NONCE_SIZE));
    }

    ctx.Reset();
    ASSERT_TRUE(EVP_AEAD_CTX_init_with_direction(
        ctx.get(), aead(), kZeros, EVP_AEAD_key_length(aead()),
        EVP_AEAD_DEFAULT_TAG_LENGTH, evp_aead_open));
    EXPECT_FALSE(EVP_AEAD_CTX_open(ctx.get(), buf, &len, sizeof(buf),
                                   nonce.data(), nonce.size(), kZeros /* in */,
                                   sizeof(kZeros), kZeros /* ad */, ad_len));
    err = ERR_get_error();
    if (!ErrorEquals(err, ERR_LIB_CIPHER, CIPHER_R_UNSUPPORTED_NONCE_SIZE)) {
      EXPECT_TRUE(
          ErrorEquals(err, ERR_LIB_CIPHER, CIPHER_R_INVALID_NONCE_SIZE));
    }
  }
}

#if defined(SUPPORTS_ABI_TEST) && !defined(BORINGSSL_RELEASE_BUILD)
// CHECK_ABI can't pass enums, i.e. |evp_aead_seal| and |evp_aead_open|. Thus
// these two wrappers.
static int aead_ctx_init_for_seal(EVP_AEAD_CTX *ctx, const EVP_AEAD *aead,
                                  const uint8_t *key, size_t key_len) {
  return EVP_AEAD_CTX_init_with_direction(ctx, aead, key, key_len, 0,
                                          evp_aead_seal);
}

static int aead_ctx_init_for_open(EVP_AEAD_CTX *ctx, const EVP_AEAD *aead,
                                  const uint8_t *key, size_t key_len) {
  return EVP_AEAD_CTX_init_with_direction(ctx, aead, key, key_len, 0,
                                          evp_aead_open);
}

// CHECK_ABI can pass, at most, eight arguments. Thus these wrappers that
// figure out the output length from the input length, and take the nonce length
// from the configuration of the AEAD.
static int aead_ctx_seal(EVP_AEAD_CTX *ctx, uint8_t *out_ciphertext,
                         size_t *out_ciphertext_len, const uint8_t *nonce,
                         const uint8_t *plaintext, size_t plaintext_len,
                         const uint8_t *ad, size_t ad_len) {
  const size_t nonce_len = EVP_AEAD_nonce_length(EVP_AEAD_CTX_aead(ctx));
  return EVP_AEAD_CTX_seal(ctx, out_ciphertext, out_ciphertext_len,
                           plaintext_len + EVP_AEAD_MAX_OVERHEAD, nonce,
                           nonce_len, plaintext, plaintext_len, ad, ad_len);
}

static int aead_ctx_open(EVP_AEAD_CTX *ctx, uint8_t *out_plaintext,
                         size_t *out_plaintext_len, const uint8_t *nonce,
                         const uint8_t *ciphertext, size_t ciphertext_len,
                         const uint8_t *ad, size_t ad_len) {
  const size_t nonce_len = EVP_AEAD_nonce_length(EVP_AEAD_CTX_aead(ctx));
  return EVP_AEAD_CTX_open(ctx, out_plaintext, out_plaintext_len,
                           ciphertext_len, nonce, nonce_len, ciphertext,
                           ciphertext_len, ad, ad_len);
}

TEST_P(PerAEADTest, ABI) {
  uint8_t key[EVP_AEAD_MAX_KEY_LENGTH];
  OPENSSL_memset(key, 'K', sizeof(key));
  const size_t key_len = EVP_AEAD_key_length(aead());
  ASSERT_LE(key_len, sizeof(key));

  bssl::ScopedEVP_AEAD_CTX ctx_seal;
  ASSERT_TRUE(
      CHECK_ABI(aead_ctx_init_for_seal, ctx_seal.get(), aead(), key, key_len));

  bssl::ScopedEVP_AEAD_CTX ctx_open;
  ASSERT_TRUE(
      CHECK_ABI(aead_ctx_init_for_open, ctx_open.get(), aead(), key, key_len));

  alignas(2) uint8_t plaintext[512];
  OPENSSL_memset(plaintext, 'P', sizeof(plaintext));

  alignas(2) uint8_t ad_buf[512];
  OPENSSL_memset(ad_buf, 'A', sizeof(ad_buf));
  const uint8_t *const ad = ad_buf + 1;
  ASSERT_LE(RequiredADLength(GetParam().flags), sizeof(ad_buf) - 1);
  const size_t ad_len = RequiredADLength(GetParam().flags) != 0
                            ? RequiredADLength(GetParam().flags)
                            : sizeof(ad_buf) - 1;

  uint8_t nonce[EVP_AEAD_MAX_NONCE_LENGTH];
  OPENSSL_memset(nonce, 'N', sizeof(nonce));
  const size_t nonce_len = EVP_AEAD_nonce_length(aead());
  ASSERT_LE(nonce_len, sizeof(nonce));

  alignas(2) uint8_t ciphertext[sizeof(plaintext) + EVP_AEAD_MAX_OVERHEAD + 1];
  size_t ciphertext_len;
  // Knock plaintext, ciphertext, and AD off alignment and give odd lengths for
  // plaintext and AD. This hopefully triggers any edge-cases in the assembly.
  ASSERT_TRUE(CHECK_ABI(aead_ctx_seal, ctx_seal.get(), ciphertext + 1,
                        &ciphertext_len, nonce, plaintext + 1,
                        sizeof(plaintext) - 1, ad, ad_len));

  alignas(2) uint8_t plaintext2[sizeof(ciphertext) + 1];
  size_t plaintext2_len;
  ASSERT_TRUE(CHECK_ABI(aead_ctx_open, ctx_open.get(), plaintext2 + 1,
                        &plaintext2_len, nonce, ciphertext + 1, ciphertext_len,
                        ad, ad_len));

  EXPECT_EQ(Bytes(plaintext + 1, sizeof(plaintext) - 1),
            Bytes(plaintext2 + 1, plaintext2_len));
}

TEST(ChaChaPoly1305Test, ABI) {
  if (!chacha20_poly1305_asm_capable()) {
    return;
  }

  std::unique_ptr<uint8_t[]> buf(new uint8_t[1024]);
  for (size_t len = 0; len <= 1024; len += 5) {
    SCOPED_TRACE(len);
    union chacha20_poly1305_open_data open_ctx = {};
    CHECK_ABI(chacha20_poly1305_open, buf.get(), buf.get(), len, buf.get(),
              len % 128, &open_ctx);
  }

  for (size_t len = 0; len <= 1024; len += 5) {
    SCOPED_TRACE(len);
    union chacha20_poly1305_seal_data seal_ctx = {};
    CHECK_ABI(chacha20_poly1305_seal, buf.get(), buf.get(), len, buf.get(),
              len % 128, &seal_ctx);
  }
}
#endif  // SUPPORTS_ABI_TEST

TEST(AEADTest, AESCCMLargeAD) {
  static const std::vector<uint8_t> kKey(16, 'A');
  static const std::vector<uint8_t> kNonce(13, 'N');
  static const std::vector<uint8_t> kAD(65536, 'D');
  static const std::vector<uint8_t> kPlaintext = {
      0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07,
      0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f};
  static const std::vector<uint8_t> kCiphertext = {
      0xa2, 0x12, 0x3f, 0x0b, 0x07, 0xd5, 0x02, 0xff,
      0xa9, 0xcd, 0xa0, 0xf3, 0x69, 0x1c, 0x49, 0x0c};
  static const std::vector<uint8_t> kTag = {0x4a, 0x31, 0x82, 0x96};

  // Test AES-128-CCM-Bluetooth.
  bssl::ScopedEVP_AEAD_CTX ctx;
  ASSERT_TRUE(EVP_AEAD_CTX_init(ctx.get(), EVP_aead_aes_128_ccm_bluetooth(),
                                kKey.data(), kKey.size(),
                                EVP_AEAD_DEFAULT_TAG_LENGTH, nullptr));

  std::vector<uint8_t> out(kCiphertext.size() + kTag.size());
  size_t out_len;
  EXPECT_TRUE(EVP_AEAD_CTX_seal(ctx.get(), out.data(), &out_len, out.size(),
                                kNonce.data(), kNonce.size(), kPlaintext.data(),
                                kPlaintext.size(), kAD.data(), kAD.size()));

  ASSERT_EQ(out_len, kCiphertext.size() + kTag.size());
  EXPECT_EQ(Bytes(kCiphertext), Bytes(out.data(), kCiphertext.size()));
  EXPECT_EQ(Bytes(kTag), Bytes(out.data() + kCiphertext.size(), kTag.size()));

  EXPECT_TRUE(EVP_AEAD_CTX_open(ctx.get(), out.data(), &out_len, out.size(),
                                kNonce.data(), kNonce.size(), out.data(),
                                out.size(), kAD.data(), kAD.size()));

  ASSERT_EQ(out_len, kPlaintext.size());
  EXPECT_EQ(Bytes(kPlaintext), Bytes(out.data(), kPlaintext.size()));
}

static void RunWycheproofTestCase(FileTest *t, const EVP_AEAD *aead) {
  t->IgnoreInstruction("ivSize");

  std::vector<uint8_t> aad, ct, iv, key, msg, tag;
  ASSERT_TRUE(t->GetBytes(&aad, "aad"));
  ASSERT_TRUE(t->GetBytes(&ct, "ct"));
  ASSERT_TRUE(t->GetBytes(&iv, "iv"));
  ASSERT_TRUE(t->GetBytes(&key, "key"));
  ASSERT_TRUE(t->GetBytes(&msg, "msg"));
  ASSERT_TRUE(t->GetBytes(&tag, "tag"));
  std::string tag_size_str;
  ASSERT_TRUE(t->GetInstruction(&tag_size_str, "tagSize"));
  size_t tag_size = static_cast<size_t>(atoi(tag_size_str.c_str()));
  ASSERT_EQ(0u, tag_size % 8);
  tag_size /= 8;
  WycheproofResult result;
  ASSERT_TRUE(GetWycheproofResult(t, &result));

  std::vector<uint8_t> ct_and_tag = ct;
  ct_and_tag.insert(ct_and_tag.end(), tag.begin(), tag.end());

  bssl::ScopedEVP_AEAD_CTX ctx;
  ASSERT_TRUE(EVP_AEAD_CTX_init(ctx.get(), aead, key.data(), key.size(),
                                tag_size, nullptr));
  std::vector<uint8_t> out(msg.size());
  size_t out_len;
  // Wycheproof tags small AES-GCM IVs as "acceptable" and otherwise does not
  // use it in AEADs. Any AES-GCM IV that isn't 96 bits is absurd, but our API
  // supports those, so we treat SmallIv tests as valid.
  if (result.IsValid({"SmallIv"})) {
    // Decryption should succeed.
    ASSERT_TRUE(EVP_AEAD_CTX_open(ctx.get(), out.data(), &out_len, out.size(),
                                  iv.data(), iv.size(), ct_and_tag.data(),
                                  ct_and_tag.size(), aad.data(), aad.size()));
    EXPECT_EQ(Bytes(msg), Bytes(out.data(), out_len));

    // Decryption in-place should succeed.
    out = ct_and_tag;
    ASSERT_TRUE(EVP_AEAD_CTX_open(ctx.get(), out.data(), &out_len, out.size(),
                                  iv.data(), iv.size(), out.data(), out.size(),
                                  aad.data(), aad.size()));
    EXPECT_EQ(Bytes(msg), Bytes(out.data(), out_len));

    // AEADs are deterministic, so encryption should produce the same result.
    out.resize(ct_and_tag.size());
    ASSERT_TRUE(EVP_AEAD_CTX_seal(ctx.get(), out.data(), &out_len, out.size(),
                                  iv.data(), iv.size(), msg.data(), msg.size(),
                                  aad.data(), aad.size()));
    EXPECT_EQ(Bytes(ct_and_tag), Bytes(out.data(), out_len));

    // Encrypt in-place.
    out = msg;
    out.resize(ct_and_tag.size());
    ASSERT_TRUE(EVP_AEAD_CTX_seal(ctx.get(), out.data(), &out_len, out.size(),
                                  iv.data(), iv.size(), out.data(), msg.size(),
                                  aad.data(), aad.size()));
    EXPECT_EQ(Bytes(ct_and_tag), Bytes(out.data(), out_len));
  } else {
    // Decryption should fail.
    EXPECT_FALSE(EVP_AEAD_CTX_open(ctx.get(), out.data(), &out_len, out.size(),
                                   iv.data(), iv.size(), ct_and_tag.data(),
                                   ct_and_tag.size(), aad.data(), aad.size()));

    // Decryption in-place should also fail.
    out = ct_and_tag;
    EXPECT_FALSE(EVP_AEAD_CTX_open(ctx.get(), out.data(), &out_len, out.size(),
                                   iv.data(), iv.size(), out.data(), out.size(),
                                   aad.data(), aad.size()));
  }
}

TEST(AEADTest, WycheproofAESGCMSIV) {
  FileTestGTest("third_party/wycheproof_testvectors/aes_gcm_siv_test.txt",
                [](FileTest *t) {
                  std::string key_size_str;
                  ASSERT_TRUE(t->GetInstruction(&key_size_str, "keySize"));
                  const EVP_AEAD *aead;
                  switch (atoi(key_size_str.c_str())) {
                    case 128:
                      aead = EVP_aead_aes_128_gcm_siv();
                      break;
                    case 256:
                      aead = EVP_aead_aes_256_gcm_siv();
                      break;
                    default:
                      FAIL() << "Unknown key size: " << key_size_str;
                  }

                  RunWycheproofTestCase(t, aead);
                });
}

//= third_party/vectors/vectors_spec.md#wycheproof
//= type=test
//# AWS-LC MUST test against `testvectors_v1/aes_gcm_test.txt`.
TEST(AEADTest, WycheproofAESGCM) {
  FileTestGTest("third_party/vectors/converted/wycheproof/testvectors_v1/aes_gcm_test.txt",
                [](FileTest *t) {
                  std::string key_size_str;
                  ASSERT_TRUE(t->GetInstruction(&key_size_str, "keySize"));
                  const EVP_AEAD *aead;
                  switch (atoi(key_size_str.c_str())) {
                    case 128:
                      aead = EVP_aead_aes_128_gcm();
                      break;
                    case 192:
                      aead = EVP_aead_aes_192_gcm();
                      break;
                    case 256:
                      aead = EVP_aead_aes_256_gcm();
                      break;
                    default:
                      FAIL() << "Unknown key size: " << key_size_str;
                  }

                  RunWycheproofTestCase(t, aead);
                });
}

TEST(AEADTest, WycheproofChaCha20Poly1305) {
  FileTestGTest("third_party/wycheproof_testvectors/chacha20_poly1305_test.txt",
                [](FileTest *t) {
                  t->IgnoreInstruction("keySize");
                  RunWycheproofTestCase(t, EVP_aead_chacha20_poly1305());
                });
}

TEST(AEADTest, WycheproofXChaCha20Poly1305) {
  FileTestGTest(
      "third_party/wycheproof_testvectors/xchacha20_poly1305_test.txt",
      [](FileTest *t) {
        t->IgnoreInstruction("keySize");
        RunWycheproofTestCase(t, EVP_aead_xchacha20_poly1305());
      });
}

TEST(AEADTest, FreeNull) { EVP_AEAD_CTX_free(nullptr); }

// Deterministic IV generation for AES-GCM 256.
TEST(AEADTest, AEADAES256GCMDetIVGen) {
  EXPECT_FALSE(EVP_AEAD_get_iv_from_ipv4_nanosecs(0, 0, nullptr));

  uint32_t ip_address = UINT32_C(0xcdfbf267);  // amazon.com when I checked.
  uint64_t fake_time = UINT64_C(0x1122334455667788);
  uint8_t out[FIPS_AES_GCM_NONCE_LENGTH] = {0};
  uint8_t expected[FIPS_AES_GCM_NONCE_LENGTH] = {
      // Note: Little-endian byte representation.
      0x67, 0xf2, 0xfb, 0xcd, 0x88, 0x77, 0x66, 0x55, 0x44, 0x33, 0x22, 0x11};

  EXPECT_TRUE(EVP_AEAD_get_iv_from_ipv4_nanosecs(ip_address, fake_time, out));
  EXPECT_EQ(Bytes(out, sizeof(out)), Bytes(expected, sizeof(expected)));
}

static int awslc_encrypt(EVP_AEAD_CTX *ctx, uint8_t *nonce, uint8_t *ct,
                         uint8_t *pt) {
  size_t ct_len = 0;
  GTEST_LOG_(INFO) << "awslc_encrypt: Ctx.State Location: " << &ctx->state;
  if (EVP_AEAD_CTX_seal(ctx, ct, &ct_len, 32, nonce, 12, pt, 16, NULL, 0) !=
      1) {
    return 1;
  }

  return 0;
}

static int awslc_decrypt(const EVP_AEAD *cipher, uint8_t ct[32], uint8_t *key,
                         size_t key_len, uint8_t nonce[12], uint8_t pt[16]) {
  EVP_AEAD_CTX ctx;
  size_t pt_len = 0;

  EVP_AEAD_CTX_zero(&ctx);
  if (EVP_AEAD_CTX_init(&ctx, cipher, key, key_len, 16, NULL) != 1) {
    return 1;
  }
  GTEST_LOG_(INFO) << "awslc_decrypt: Ctx.State Location: " << &ctx.state;

  if (EVP_AEAD_CTX_open(&ctx, pt, &pt_len, 16, nonce, 12, ct, 32, NULL, 0) !=
      1) {
    return 1;
  }

  return 0;
}

TEST(AEADTest, TestGCMSIV128Change16Alignment) {
  uint8_t key[16] = {0};
  uint8_t nonce[12] = {0};
  uint8_t pt[16] = {0};
  uint8_t ct[32] = {0};
  EVP_AEAD_CTX *encrypt_ctx_128 =
      (EVP_AEAD_CTX *)malloc(sizeof(EVP_AEAD_CTX) + 8);
  ASSERT_TRUE(encrypt_ctx_128);

  const EVP_AEAD *cipher_128 = EVP_aead_aes_128_gcm_siv();

  EVP_AEAD_CTX_zero(encrypt_ctx_128);
  ASSERT_TRUE(EVP_AEAD_CTX_init(encrypt_ctx_128, cipher_128, key, 16, 16, NULL))
      << ERR_error_string(ERR_get_error(), NULL);
  ASSERT_FALSE(awslc_encrypt(encrypt_ctx_128, nonce, ct, pt))
      << ERR_error_string(ERR_get_error(), NULL);
  ASSERT_FALSE(awslc_decrypt(cipher_128, ct, key, 16, nonce, pt))
      << ERR_error_string(ERR_get_error(), NULL);

  GTEST_LOG_(INFO) << "Orig. Ctx.State Location: " << &encrypt_ctx_128->state;
  EVP_AEAD_CTX *moved_encrypt_ctx_128 =
      (EVP_AEAD_CTX *)(((uint8_t *)encrypt_ctx_128) + 8);
  // The destination pointer is offset into the allocation so that it aliases
  // the |state| subobject; GCC / fortify-headers infer the subobject size and
  // report a `stringop-overflow` false positive (see aws-lc#3083).
  // -Wstringop-overflow was introduced in GCC 7; older GCC versions reject
  // the pragma with -Werror=pragmas.
#if defined(__GNUC__) && !defined(__clang__) && (__GNUC__ >= 7)
#pragma GCC diagnostic push
#pragma GCC diagnostic ignored "-Wstringop-overflow"
#endif
  memmove(moved_encrypt_ctx_128, encrypt_ctx_128, sizeof(EVP_AEAD_CTX));
#if defined(__GNUC__) && !defined(__clang__) && (__GNUC__ >= 7)
#pragma GCC diagnostic pop
#endif
  GTEST_LOG_(INFO) << "Moved Ctx.State Location: "
                   << &moved_encrypt_ctx_128->state;

  if (awslc_encrypt(moved_encrypt_ctx_128, nonce, ct, pt) != 1) {
    if (x86_64_assembly_implementation_FOR_TESTING()) {
      FAIL() << "Expected failure in awslc_encrypt";
    }
  } else {
    if (!x86_64_assembly_implementation_FOR_TESTING()) {
      FAIL() << "Failure in awslc_encrypt";
    }
    uint32_t err = ERR_get_error();
    EXPECT_EQ(ERR_R_CIPHER_LIB, ERR_GET_LIB(err));
    EXPECT_EQ(CIPHER_R_ALIGNMENT_CHANGED, ERR_GET_REASON(err));
  }
  free(encrypt_ctx_128);
}

TEST(AEADTest, TestGCMSIV256Change16Alignment) {
  uint8_t nonce[12] = {0};
  uint8_t key[32] = {0};
  uint8_t pt[16] = {0};
  uint8_t ct[32] = {0};
  EVP_AEAD_CTX *encrypt_ctx_256 =
      (EVP_AEAD_CTX *)malloc(sizeof(EVP_AEAD_CTX) + 8);
  ASSERT_TRUE(encrypt_ctx_256);

  const EVP_AEAD *cipher_256 = EVP_aead_aes_256_gcm_siv();

  EVP_AEAD_CTX_zero(encrypt_ctx_256);
  ASSERT_TRUE(EVP_AEAD_CTX_init(encrypt_ctx_256, cipher_256, key, 32, 16, NULL))
      << ERR_error_string(ERR_get_error(), NULL);
  ASSERT_FALSE(awslc_encrypt(encrypt_ctx_256, nonce, ct, pt))
      << ERR_error_string(ERR_get_error(), NULL);
  ASSERT_FALSE(awslc_decrypt(cipher_256, ct, key, 32, nonce, pt))
      << ERR_error_string(ERR_get_error(), NULL);

  GTEST_LOG_(INFO) << "Orig. Ctx.State Location: " << &encrypt_ctx_256->state;
  EVP_AEAD_CTX *moved_encrypt_ctx_256 =
      (EVP_AEAD_CTX *)(((uint8_t *)encrypt_ctx_256) + 8);
  // See TestGCMSIV128Change16Alignment for why this pragma is needed.
#if defined(__GNUC__) && !defined(__clang__) && (__GNUC__ >= 7)
#pragma GCC diagnostic push
#pragma GCC diagnostic ignored "-Wstringop-overflow"
#endif
  memmove(moved_encrypt_ctx_256, encrypt_ctx_256, sizeof(EVP_AEAD_CTX));
#if defined(__GNUC__) && !defined(__clang__) && (__GNUC__ >= 7)
#pragma GCC diagnostic pop
#endif
  GTEST_LOG_(INFO) << "Moved Ctx.State Location: "
                   << &moved_encrypt_ctx_256->state;

  if (awslc_encrypt(moved_encrypt_ctx_256, nonce, ct, pt) != 1) {
    if (x86_64_assembly_implementation_FOR_TESTING()) {
      FAIL() << "Expected failure in awslc_encrypt";
    }
  } else {
    if (!x86_64_assembly_implementation_FOR_TESTING()) {
      FAIL() << "Failure in awslc_encrypt";
    }
    uint32_t err = ERR_get_error();
    EXPECT_EQ(ERR_R_CIPHER_LIB, ERR_GET_LIB(err));
    EXPECT_EQ(CIPHER_R_ALIGNMENT_CHANGED, ERR_GET_REASON(err));
  }
  free(encrypt_ctx_256);
}

TEST(AEADTest, TestMonotonicityCheck) {

  static const uint8_t kEvpAeadCtxKey[32] = {0};

  // Only the tls13() ciphers have monotonicity checks
  const EVP_AEAD *aeads_to_test[] = {  EVP_aead_aes_128_gcm_tls13(), EVP_aead_aes_256_gcm_tls13() };

  for (const EVP_AEAD *cipher : aeads_to_test) {
    bssl::ScopedEVP_AEAD_CTX encrypt_ctx;

    ASSERT_TRUE(EVP_AEAD_CTX_init(encrypt_ctx.get(), cipher, kEvpAeadCtxKey, cipher->key_len, 16, NULL))
        << ERR_error_string(ERR_get_error(), NULL);

    uint8_t nonce[12] = {0};
    uint8_t last_byte = sizeof(nonce) - 1;
    uint8_t plaintext[16] = {0};
    uint8_t ciphertext[32] = {0};
    size_t out_len = 0;

    // Checks that sequence numbers are allowed to increment by more than one
    // as long as monotonicity is preserved. Here the implicit IV is presumed
    // to be a zero-filled array. That lets us update the nonce value directly
    // with an increasing sequence number.
    for (size_t sequence_num = 0; sequence_num <= 255; sequence_num+=10) {
      nonce[last_byte] = sequence_num;
      ASSERT_TRUE(EVP_AEAD_CTX_seal(encrypt_ctx.get(), ciphertext, &out_len,
                                    sizeof(ciphertext), nonce, sizeof(nonce), plaintext,
                                    sizeof(plaintext), nullptr /* ad */, 0));
    }

    // Attempting to encrypt with a decreased sequence number causes the monotonicity check to fail.
    nonce[last_byte] = 0;
    ASSERT_FALSE(EVP_AEAD_CTX_seal(encrypt_ctx.get(), ciphertext, &out_len,
                               sizeof(ciphertext), nonce, sizeof(nonce), plaintext,
                               sizeof(plaintext), nullptr /* ad */, 0));
  }
}

struct EvpAeadCtxSerdeTestParams {
  const char *name;
  const EVP_AEAD *cipher;
  const uint8_t *key;
  const size_t key_len;
  const size_t tag_len;
  uint16_t expect_id;
};

class EvpAeadCtxSerdeTest
    : public testing::TestWithParam<EvpAeadCtxSerdeTestParams> {};

static const uint8_t kEvpAeadCtxKey[80] = {
    0x03, 0xeb, 0x1d, 0xb2, 0x2c, 0xa8, 0xc0, 0x3b, 0x29, 0x9c, 0x66, 0xe5,
    0xdd, 0xb7, 0x70, 0x6c, 0x39, 0x86, 0x71, 0x94, 0x79, 0x5c, 0xf5, 0x88,
    0xde, 0xd9, 0x05, 0x1f, 0x28, 0x96, 0x86, 0x28, 0x01, 0xb0, 0x59, 0x11,
    0xb0, 0x3f, 0x35, 0xe6, 0xb5, 0x2f, 0x3b, 0xee, 0xbc, 0xf9, 0x11, 0xb1,
    0x9e, 0x58, 0xf6, 0xb7, 0xf3, 0x3e, 0x5b, 0x66, 0x28, 0x85, 0x0c, 0x66,
    0x2b, 0x75, 0xb7, 0x86, 0xfd, 0xa4, 0x2d, 0x4b, 0x8c, 0xe0, 0x9a, 0x58,
    0xbf, 0xc6, 0x22, 0x4c, 0x39, 0x25, 0x66, 0xfd
};

static const EvpAeadCtxSerdeTestParams kEvpAeadCtxSerde[] = {
    {"EVP_aead_aes_128_gcm", EVP_aead_aes_128_gcm(), kEvpAeadCtxKey, 16, 16,
     16},
    {"EVP_aead_aes_192_gcm", EVP_aead_aes_192_gcm(), kEvpAeadCtxKey, 24, 16,
     17},
    {"EVP_aead_aes_256_gcm", EVP_aead_aes_256_gcm(), kEvpAeadCtxKey, 32, 16,
     18},
    {"EVP_aead_chacha20_poly1305", EVP_aead_chacha20_poly1305(), kEvpAeadCtxKey,
     32, 16, 5},
    {"EVP_aead_xchacha20_poly1305", EVP_aead_xchacha20_poly1305(),
     kEvpAeadCtxKey, 32, 16, 6},
    {"EVP_aead_aes_128_ctr_hmac_sha256", EVP_aead_aes_128_ctr_hmac_sha256(),
     kEvpAeadCtxKey, 48, 32, 1},
    {"EVP_aead_aes_256_ctr_hmac_sha256", EVP_aead_aes_256_ctr_hmac_sha256(),
     kEvpAeadCtxKey, 64, 32, 2},
    {"EVP_aead_aes_128_gcm_siv", EVP_aead_aes_128_gcm_siv(), kEvpAeadCtxKey, 16,
     16, 3},
    {"EVP_aead_aes_256_gcm_siv", EVP_aead_aes_256_gcm_siv(), kEvpAeadCtxKey, 32,
     16, 4},
    {"EVP_aead_aes_128_gcm_randnonce", EVP_aead_aes_128_gcm_randnonce(),
     kEvpAeadCtxKey, 16, 28, 19},
    {"EVP_aead_aes_256_gcm_randnonce", EVP_aead_aes_256_gcm_randnonce(),
     kEvpAeadCtxKey, 32, 28, 20},
    {"EVP_aead_aes_128_ccm_bluetooth", EVP_aead_aes_128_ccm_bluetooth(),
     kEvpAeadCtxKey, 16, 4, 25},
    {"EVP_aead_aes_128_ccm_bluetooth_8", EVP_aead_aes_128_ccm_bluetooth_8(),
     kEvpAeadCtxKey, 16, 8, 26},
    {"EVP_aead_aes_128_ccm_matter", EVP_aead_aes_128_ccm_matter(),
     kEvpAeadCtxKey, 16, 16, 27},
    {"EVP_aead_aes_128_cbc_sha1_tls", EVP_aead_aes_128_cbc_sha1_tls(),
     kEvpAeadCtxKey, 36, 20, 7},
    {"EVP_aead_aes_128_cbc_sha1_tls_implicit_iv",
     EVP_aead_aes_128_cbc_sha1_tls_implicit_iv(), kEvpAeadCtxKey, 52, 20, 8},
    {"EVP_aead_aes_256_cbc_sha1_tls", EVP_aead_aes_256_cbc_sha1_tls(),
     kEvpAeadCtxKey, 52, 20, 9},
    {"EVP_aead_aes_256_cbc_sha1_tls_implicit_iv",
     EVP_aead_aes_256_cbc_sha1_tls_implicit_iv(), kEvpAeadCtxKey, 68, 20, 10},
    {"EVP_aead_aes_128_cbc_sha256_tls", EVP_aead_aes_128_cbc_sha256_tls(),
     kEvpAeadCtxKey, 48, 32, 11},
    {"EVP_aead_aes_128_cbc_sha256_tls_implicit_iv",
     EVP_aead_aes_128_cbc_sha256_tls_implicit_iv(), kEvpAeadCtxKey, 64, 32, 12},
    {"EVP_aead_aes_256_cbc_sha384_tls", EVP_aead_aes_256_cbc_sha384_tls(),
     kEvpAeadCtxKey, 80, 48, 28},
    {"EVP_aead_des_ede3_cbc_sha1_tls", EVP_aead_des_ede3_cbc_sha1_tls(),
     kEvpAeadCtxKey, 44, 20, 13},
    {"EVP_aead_des_ede3_cbc_sha1_tls_implicit_iv",
     EVP_aead_des_ede3_cbc_sha1_tls_implicit_iv(), kEvpAeadCtxKey, 52, 20, 14},
    {"EVP_aead_null_sha1_tls", EVP_aead_null_sha1_tls(), kEvpAeadCtxKey, 20, 20,
     15},
    {"EVP_aead_aes_128_gcm_tls12", EVP_aead_aes_128_gcm_tls12(), kEvpAeadCtxKey,
     16, 16, 21},
    {"EVP_aead_aes_256_gcm_tls12", EVP_aead_aes_256_gcm_tls12(), kEvpAeadCtxKey,
     32, 16, 22},
    {"EVP_aead_aes_128_gcm_tls13", EVP_aead_aes_128_gcm_tls13(), kEvpAeadCtxKey,
     16, 16, 23},
    {"EVP_aead_aes_256_gcm_tls13", EVP_aead_aes_256_gcm_tls13(), kEvpAeadCtxKey,
     32, 16, 24}};

INSTANTIATE_TEST_SUITE_P(
    EvpAeadCtxSerdeTests, EvpAeadCtxSerdeTest,
    testing::ValuesIn(kEvpAeadCtxSerde),
    [](const testing::TestParamInfo<EvpAeadCtxSerdeTestParams> &params)
        -> std::string { return params.param.name; });

TEST_P(EvpAeadCtxSerdeTest, Roundtrips) {
  const ParamType &params = GetParam();

  const evp_aead_direction_t directions[] = {evp_aead_open, evp_aead_seal};

  for (evp_aead_direction_t direction : directions) {
    bssl::ScopedEVP_AEAD_CTX ctx;
    bssl::ScopedEVP_AEAD_CTX ctx2;

    ASSERT_TRUE(EVP_AEAD_CTX_init_with_direction(ctx.get(), params.cipher,
                                                 params.key, params.key_len,
                                                 params.tag_len, direction));
    ASSERT_TRUE(EVP_AEAD_CTX_init_with_direction(ctx2.get(), params.cipher,
                                                 params.key, params.key_len,
                                                 params.tag_len, direction));

    bssl::ScopedCBB cbb;
    CBB_init(cbb.get(), 1024);
    CBS cbs;

    ASSERT_TRUE(EVP_AEAD_CTX_serialize_state(ctx.get(), cbb.get()));
    CBS_init(&cbs, CBB_data(cbb.get()), CBB_len(cbb.get()));
    ASSERT_TRUE(EVP_AEAD_CTX_deserialize_state(ctx2.get(), &cbs));
  }
}

TEST_P(EvpAeadCtxSerdeTest, FailUnknownSerdeVersion) {
  // A minimal DER encoding with the serialization version set to 42.
  // SEQUENCE {
  //   INTEGER { 42 }
  // }
  static const size_t INVALID_VERSION_DER_LEN = 5;
  static const uint8_t INVALID_VERSION_DER[INVALID_VERSION_DER_LEN] = {
      0x30, 0x03, 0x02, 0x01, 0x2a};

  const ParamType &params = GetParam();

  const evp_aead_direction_t directions[] = {evp_aead_open, evp_aead_seal};

  for (const evp_aead_direction_t direction : directions) {
    bssl::ScopedEVP_AEAD_CTX ctx;

    ASSERT_TRUE(EVP_AEAD_CTX_init_with_direction(ctx.get(), params.cipher,
                                                 params.key, params.key_len,
                                                 params.tag_len, direction));

    CBS cbs;
    CBS_init(&cbs, INVALID_VERSION_DER, INVALID_VERSION_DER_LEN);
    ASSERT_FALSE(EVP_AEAD_CTX_deserialize_state(ctx.get(), &cbs));
    ASSERT_EQ(ERR_GET_LIB(ERR_peek_error()), ERR_LIB_CIPHER);
    ASSERT_EQ(ERR_GET_REASON(ERR_get_error()),
              CIPHER_R_SERIALIZATION_INVALID_SERDE_VERSION);
  }
}

TEST_P(EvpAeadCtxSerdeTest, FailUnknownCipherId) {
  // A minimal DER encoding with the serialization version set to 1,
  // and the cipher identifer set to 65535.
  // SEQUENCE {
  //   INTEGER { 1 }
  //   INTEGER { 65535 }
  // }
  static const size_t INVALID_CIPHER_ID_DER_LEN = 10;
  static const uint8_t INVALID_CIPHER_ID_DER[INVALID_CIPHER_ID_DER_LEN] = {
      0x30, 0x08, 0x02, 0x01, 0x01, 0x02, 0x03, 0x00, 0xff, 0xff};

  const ParamType &params = GetParam();

  const evp_aead_direction_t directions[] = {evp_aead_open, evp_aead_seal};

  for (const evp_aead_direction_t direction : directions) {
    bssl::ScopedEVP_AEAD_CTX ctx;

    ASSERT_TRUE(EVP_AEAD_CTX_init_with_direction(ctx.get(), params.cipher,
                                                 params.key, params.key_len,
                                                 params.tag_len, direction));

    CBS cbs;
    CBS_init(&cbs, INVALID_CIPHER_ID_DER, INVALID_CIPHER_ID_DER_LEN);
    ASSERT_FALSE(EVP_AEAD_CTX_deserialize_state(ctx.get(), &cbs));
    ASSERT_EQ(ERR_GET_LIB(ERR_peek_error()), ERR_LIB_CIPHER);
    ASSERT_EQ(ERR_GET_REASON(ERR_get_error()),
              CIPHER_R_SERIALIZATION_INVALID_CIPHER_ID);
  }
}

TEST(EvpAeadCtxSerdeTest, ID) {
  bool identifiers[AEAD_MAX_ID + 1] = {false};
  for (EvpAeadCtxSerdeTestParams params : kEvpAeadCtxSerde) {
    bssl::ScopedEVP_AEAD_CTX ctx;
    ASSERT_TRUE(EVP_AEAD_CTX_init_with_direction(
        ctx.get(), params.cipher, params.key, params.key_len, params.tag_len,
        evp_aead_open));
    uint16_t id = EVP_AEAD_CTX_get_aead_id(ctx.get());
    ASSERT_EQ(params.expect_id, id);
    // No cipher must have the same identifier
    ASSERT_FALSE(identifiers[id]);
    identifiers[id] = true;
  }

  // Nothing should have the unknown identifier (0)
  ASSERT_FALSE(identifiers[0]);

  // If our test coverage is good we should have all possible id's covered
  // otherwise we are missing coverage.
  for (size_t id = 1; id <= AEAD_MAX_ID; id++) {
    ASSERT_TRUE(identifiers[id]);
  }
}
