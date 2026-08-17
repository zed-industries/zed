// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC

#include "internal.h"

#include <gtest/gtest.h>
#include <openssl/dh.h>
#include <openssl/dsa.h>
#include <openssl/ec_key.h>
#include <openssl/err.h>
#include <openssl/evp.h>
#include <openssl/kdf.h>
#include <openssl/rsa.h>

#include "../../internal.h"
#include "../../test/test_util.h"
#include "internal.h"

class EvpPkeyCtxCtrlStrTest : public ::testing::Test {
 protected:
  void SetUp() override {}

  void TearDown() override {}
};

class EvpPkeyCtxCtrlStrParamTest : public testing::TestWithParam<const char *> {
protected:
  void SetUp() override {}

  void TearDown() override {}
};

static bssl::UniquePtr<EVP_PKEY_CTX> gen_RSA() {
  EVP_PKEY *raw = nullptr;
  bssl::UniquePtr<EVP_PKEY_CTX> keygen_ctx(
      EVP_PKEY_CTX_new_id(EVP_PKEY_RSA, nullptr));
  if (!EVP_PKEY_keygen_init(keygen_ctx.get()) ||
      !EVP_PKEY_CTX_set_rsa_keygen_bits(keygen_ctx.get(), 2048) ||
      !EVP_PKEY_keygen(keygen_ctx.get(), &raw)) {
    return nullptr;
  }
  bssl::UniquePtr<EVP_PKEY> pkey(raw);
  return bssl::UniquePtr<EVP_PKEY_CTX>(EVP_PKEY_CTX_new(pkey.get(), nullptr));
}

TEST_F(EvpPkeyCtxCtrlStrTest, RsaMissingValue) {
  // Create a EVP_PKEY_CTX with a newly generated RSA key
  bssl::UniquePtr<EVP_PKEY_CTX> ctx = gen_RSA();
  ASSERT_TRUE(ctx);
  EXPECT_FALSE(EVP_PKEY_CTX_ctrl_str(ctx.get(), "rsa_padding_mode", nullptr));
  unsigned long err = ERR_get_error();
  EXPECT_EQ(ERR_GET_LIB(err), ERR_LIB_EVP);
  EXPECT_EQ(ERR_GET_REASON(err), RSA_R_VALUE_MISSING);
}

TEST_F(EvpPkeyCtxCtrlStrTest, RsaPaddingModeValid) {
  bssl::UniquePtr<EVP_PKEY_CTX> ctx = gen_RSA();
  ASSERT_TRUE(ctx);

  int padding = 0;

  // Padding for sign
  ASSERT_TRUE(EVP_PKEY_sign_init(ctx.get()));

  EXPECT_TRUE(EVP_PKEY_CTX_ctrl_str(ctx.get(), "rsa_padding_mode", "pkcs1"));
  EXPECT_TRUE(EVP_PKEY_CTX_get_rsa_padding(ctx.get(), &padding));
  EXPECT_EQ(padding, RSA_PKCS1_PADDING);

  EXPECT_EQ(EVP_PKEY_CTX_ctrl_str(ctx.get(), "rsa_padding_mode", "none"), 1);
  EXPECT_TRUE(EVP_PKEY_CTX_get_rsa_padding(ctx.get(), &padding));
  EXPECT_EQ(padding, RSA_NO_PADDING);

  EXPECT_EQ(EVP_PKEY_CTX_ctrl_str(ctx.get(), "rsa_padding_mode", "pss"), 1);
  EXPECT_TRUE(EVP_PKEY_CTX_get_rsa_padding(ctx.get(), &padding));
  EXPECT_EQ(padding, RSA_PKCS1_PSS_PADDING);

  // Padding for encrypt
  ASSERT_TRUE(EVP_PKEY_encrypt_init(ctx.get()));

  EXPECT_EQ(EVP_PKEY_CTX_ctrl_str(ctx.get(), "rsa_padding_mode", "oaep"), 1);
  EXPECT_TRUE(EVP_PKEY_CTX_get_rsa_padding(ctx.get(), &padding));
  EXPECT_EQ(padding, RSA_PKCS1_OAEP_PADDING);

  EXPECT_EQ(EVP_PKEY_CTX_ctrl_str(ctx.get(), "rsa_padding_mode", "oeap"), 1);
  EXPECT_TRUE(EVP_PKEY_CTX_get_rsa_padding(ctx.get(), &padding));
  EXPECT_EQ(padding, RSA_PKCS1_OAEP_PADDING);

  EXPECT_EQ(EVP_PKEY_CTX_ctrl_str(ctx.get(), "rsa_padding_mode", "nonsense"),
            -2);
}

TEST_F(EvpPkeyCtxCtrlStrTest, RsaPssSaltlen) {
  // Create a EVP_PKEY_CTX with a newly generated RSA key
  bssl::UniquePtr<EVP_PKEY_CTX> ctx = gen_RSA();
  ASSERT_TRUE(ctx);

  ASSERT_TRUE(EVP_PKEY_sign_init(ctx.get()));
  EXPECT_EQ(EVP_PKEY_CTX_ctrl_str(ctx.get(), "rsa_padding_mode", "pss"), 1);
  EXPECT_EQ(EVP_PKEY_CTX_ctrl_str(ctx.get(), "rsa_pss_saltlen", "128"), 1);

  int saltlen = 0;
  EXPECT_EQ(EVP_PKEY_CTX_get_rsa_pss_saltlen(ctx.get(), &saltlen), 1);
  EXPECT_EQ(saltlen, 128);

  EXPECT_EQ(EVP_PKEY_CTX_ctrl_str(ctx.get(), "rsa_pss_saltlen", "digest"), 1);
  EXPECT_EQ(EVP_PKEY_CTX_get_rsa_pss_saltlen(ctx.get(), &saltlen), 1);
  EXPECT_EQ(saltlen, -1);

  EXPECT_EQ(EVP_PKEY_CTX_ctrl_str(ctx.get(), "rsa_pss_saltlen", "-3"), -2);
}

TEST_F(EvpPkeyCtxCtrlStrTest, RsaKeygenBits) {
  // Create a EVP_PKEY_CTX with a newly generated RSA key
  EVP_PKEY *raw = nullptr;
  bssl::UniquePtr<EVP_PKEY_CTX> ctx(EVP_PKEY_CTX_new_id(EVP_PKEY_RSA, nullptr));
  ASSERT_TRUE(ctx);
  ASSERT_TRUE(EVP_PKEY_keygen_init(ctx.get()));
  ASSERT_EQ(EVP_PKEY_CTX_ctrl_str(ctx.get(), "rsa_keygen_bits", "2048"), 1);
  ASSERT_EQ(EVP_PKEY_CTX_ctrl_str(ctx.get(), "rsa_keygen_bits", "-3"), -2);
  ASSERT_TRUE(EVP_PKEY_keygen(ctx.get(), &raw));
  bssl::UniquePtr<EVP_PKEY> pkey(raw);
  ASSERT_TRUE(pkey);

  ASSERT_EQ(EVP_PKEY_bits(pkey.get()), 2048);
}

TEST_F(EvpPkeyCtxCtrlStrTest, RsaKeygenPubexp) {
  // Create a EVP_PKEY_CTX with a newly generated RSA key
  bssl::UniquePtr<EVP_PKEY_CTX> ctx(EVP_PKEY_CTX_new_id(EVP_PKEY_RSA, nullptr));
  ASSERT_TRUE(ctx);
  ASSERT_TRUE(EVP_PKEY_keygen_init(ctx.get()));
#if defined(BORINGSSL_FIPS)
  ASSERT_EQ(EVP_PKEY_CTX_ctrl_str(ctx.get(), "rsa_keygen_pubexp", "65537"), 1);
  ASSERT_EQ(EVP_PKEY_CTX_ctrl_str(ctx.get(), "rsa_keygen_pubexp", "729"), 0);
#else
  ASSERT_EQ(EVP_PKEY_CTX_ctrl_str(ctx.get(), "rsa_keygen_pubexp", "729"), 1);
#endif
  ASSERT_EQ(EVP_PKEY_CTX_ctrl_str(ctx.get(), "rsa_keygen_pubexp", "gg"), -2);
  EVP_PKEY *raw = nullptr;
  ASSERT_TRUE(EVP_PKEY_keygen(ctx.get(), &raw));
  bssl::UniquePtr<EVP_PKEY> pkey(raw);
  ASSERT_TRUE(pkey);

  bssl::UniquePtr<RSA> rsa_key(EVP_PKEY_get1_RSA(pkey.get()));
  ASSERT_TRUE(rsa_key);
  const BIGNUM *const_pe_bn = RSA_get0_e(rsa_key.get());
  ASSERT_TRUE(const_pe_bn != nullptr);
#if defined(BORINGSSL_FIPS)
  const uint64_t expected_pe = 65537;
#else
  const uint64_t expected_pe = 729;
#endif
  uint64_t pe_u64;
  ASSERT_TRUE(BN_get_u64(const_pe_bn, &pe_u64));
  EXPECT_EQ(pe_u64, expected_pe);

}

TEST_F(EvpPkeyCtxCtrlStrTest, RsaMgf1Md) {
  bssl::UniquePtr<EVP_PKEY_CTX> ctx = gen_RSA();
  ASSERT_TRUE(ctx);

  ASSERT_TRUE(EVP_PKEY_sign_init(ctx.get()));
  ASSERT_TRUE(EVP_PKEY_CTX_set_rsa_padding(ctx.get(), RSA_PKCS1_PSS_PADDING));
  ASSERT_EQ(EVP_PKEY_CTX_ctrl_str(ctx.get(), "rsa_mgf1_md", "sha256"), 1);

  const EVP_MD *out_md;
  ASSERT_TRUE(EVP_PKEY_CTX_get_rsa_mgf1_md(ctx.get(), &out_md));
  ASSERT_STREQ(EVP_MD_name(out_md), "SHA256");
}

// rsa_oaep_md
TEST_F(EvpPkeyCtxCtrlStrTest, RsaOaepMd) {
  bssl::UniquePtr<EVP_PKEY_CTX> ctx = gen_RSA();
  ASSERT_TRUE(ctx);

  ASSERT_TRUE(EVP_PKEY_encrypt_init(ctx.get()));
  ASSERT_TRUE(EVP_PKEY_CTX_set_rsa_padding(ctx.get(), RSA_PKCS1_OAEP_PADDING));
  ASSERT_EQ(EVP_PKEY_CTX_ctrl_str(ctx.get(), "rsa_oaep_md", "sha256"), 1);

  const EVP_MD *out_md;
  ASSERT_TRUE(EVP_PKEY_CTX_get_rsa_oaep_md(ctx.get(), &out_md));
  ASSERT_STREQ(EVP_MD_name(out_md), "SHA256");
}

TEST_F(EvpPkeyCtxCtrlStrTest, RsaOaepLabel) {
  bssl::UniquePtr<EVP_PKEY_CTX> ctx = gen_RSA();
  ASSERT_TRUE(ctx);

  ASSERT_TRUE(EVP_PKEY_encrypt_init(ctx.get()));
  ASSERT_TRUE(EVP_PKEY_CTX_set_rsa_padding(ctx.get(), RSA_PKCS1_OAEP_PADDING));
  ASSERT_TRUE(EVP_PKEY_CTX_set_rsa_oaep_md(ctx.get(), EVP_sha256()));
  ASSERT_EQ(EVP_PKEY_CTX_ctrl_str(ctx.get(), "rsa_oaep_label", "aabb11"), 1);
  ASSERT_EQ(EVP_PKEY_CTX_ctrl_str(ctx.get(), "rsa_oaep_label", "gg"), 0);

  const char expected_label[4] = "\xaa\xbb\x11";
  const uint8_t *actual_label;
  ASSERT_TRUE(EVP_PKEY_CTX_get0_rsa_oaep_label(ctx.get(), &actual_label));

  ASSERT_TRUE(OPENSSL_memcmp(actual_label, expected_label, 3) == 0);
}

TEST_P(EvpPkeyCtxCtrlStrParamTest, EcParamgenCurve) {
  const char* name = GetParam();

  // Create a EVP_PKEY_CTX with a newly generated EC key
  EVP_PKEY *raw = nullptr;
  bssl::UniquePtr<EVP_PKEY_CTX> ctx(EVP_PKEY_CTX_new_id(EVP_PKEY_EC, nullptr));
  ASSERT_TRUE(ctx);
  ASSERT_TRUE(EVP_PKEY_keygen_init(ctx.get()));
  ASSERT_EQ(EVP_PKEY_CTX_ctrl_str(ctx.get(), "ec_paramgen_curve", name), 1);

  ASSERT_TRUE(EVP_PKEY_keygen(ctx.get(), &raw));
  bssl::UniquePtr<EVP_PKEY> pkey(raw);
  ASSERT_TRUE(pkey);

  const EC_KEY* ec_key = EVP_PKEY_get0_EC_KEY(pkey.get());
  ASSERT_TRUE(ec_key != nullptr);
  const EC_GROUP* ec_group = EC_KEY_get0_group(ec_key);
  ASSERT_TRUE(ec_group != nullptr);
  ASSERT_EQ(NID_X9_62_prime256v1, EC_GROUP_get_curve_name(ec_group));
}

INSTANTIATE_TEST_SUITE_P(EcParamgenCurve,
                         EvpPkeyCtxCtrlStrParamTest,
                         testing::Values("P-256", "prime256v1"));


TEST_F(EvpPkeyCtxCtrlStrTest, EcParamEnc) {
  // Create a EVP_PKEY_CTX with a newly generated EC key
  bssl::UniquePtr<EVP_PKEY_CTX> ctx(EVP_PKEY_CTX_new_id(EVP_PKEY_EC, nullptr));
  ASSERT_TRUE(ctx);
  ASSERT_TRUE(EVP_PKEY_keygen_init(ctx.get()));
  ASSERT_EQ(EVP_PKEY_CTX_ctrl_str(ctx.get(), "ec_param_enc", "named_curve"), 1);

  // The `EVP_PKEY_CTX_set_ec_param_enc` function that is called does not
  // alter any state, so there's nothing to verify afterward.
}

TEST_F(EvpPkeyCtxCtrlStrTest, DhPad) {
  // Create a EVP_PKEY_CTX with a newly generated DH
  bssl::UniquePtr<EVP_PKEY_CTX> ctx(EVP_PKEY_CTX_new_id(EVP_PKEY_DH, nullptr));
  ASSERT_TRUE(ctx);
  ASSERT_TRUE(EVP_PKEY_derive_init(ctx.get()));
  ASSERT_EQ(EVP_PKEY_CTX_ctrl_str(ctx.get(), "dh_pad", "17"), 1);

  // There is no function to retrieve the DH pad value.
}

TEST_F(EvpPkeyCtxCtrlStrTest, DhParamGen) {
  // Create a EVP_PKEY_CTX with a newly generated DH
  bssl::UniquePtr<EVP_PKEY_CTX> ctx(EVP_PKEY_CTX_new_id(EVP_PKEY_DH, nullptr));
  ASSERT_TRUE(ctx);
  ASSERT_TRUE(EVP_PKEY_paramgen_init(ctx.get()));

  ASSERT_EQ(EVP_PKEY_CTX_ctrl_str(ctx.get(), "dh_paramgen_prime_len", "256"), 1);
  ASSERT_NE(EVP_PKEY_CTX_ctrl_str(ctx.get(), "dh_paramgen_prime_len", "gg"), 1);
  ASSERT_NE(EVP_PKEY_CTX_ctrl_str(ctx.get(), "dh_paramgen_prime_len", "255"), 1);

  ASSERT_EQ(EVP_PKEY_CTX_ctrl_str(ctx.get(), "dh_paramgen_generator", "5"), 1);
  ASSERT_NE(EVP_PKEY_CTX_ctrl_str(ctx.get(), "dh_paramgen_prime_len", "gg"), 1);
  ASSERT_NE(EVP_PKEY_CTX_ctrl_str(ctx.get(), "dh_paramgen_prime_len", "1"), 1);

  EVP_PKEY* raw = nullptr;
  ASSERT_EQ(EVP_PKEY_paramgen(ctx.get(), &raw), 1);
  // Only parameters have been generated, but no key has actually been set.
  EXPECT_FALSE(EVP_PKEY_param_check(ctx.get()));
  bssl::UniquePtr<EVP_PKEY> pkey(raw);
  ASSERT_TRUE(raw);

  const DH* dh = EVP_PKEY_get0_DH(pkey.get());
  ASSERT_TRUE(dh);
  const BIGNUM* p = DH_get0_p(dh);
  ASSERT_TRUE(p);
  unsigned p_size = BN_num_bits(p);
  ASSERT_EQ(p_size, 256u);
}

static const char *hkdf_hexsalt = "000102030405060708090a0b0c";
static const char *hkdf_hexinfo = "f0f1f2f3f4f5f6f7f8f9";
static const char *hkdf_hexkey = "0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b";
static const char *hkdf_hex_expected_okm =
    "3cb25f25faacd57a90434f64d0362f2a2d2d0a90cf1a5a4c5db02d56ecc4c5bf34007208d5"
    "b887185865";
static const char *hkdf_hex_expected_prk =
    "077709362c2e32df0ddc3f0dc47bba6390b6c73bb50f9c3122ec844ad7c2b3e5";

TEST_F(EvpPkeyCtxCtrlStrTest, HkdfHex) {
  // Test Cases from RFC 5869.

  bssl::UniquePtr<EVP_PKEY_CTX> ctx(
      EVP_PKEY_CTX_new_id(EVP_PKEY_HKDF, nullptr));
  ASSERT_TRUE(ctx);
  ASSERT_TRUE(EVP_PKEY_derive_init(ctx.get()));
  ASSERT_EQ(EVP_PKEY_CTX_ctrl_str(ctx.get(), "mode", "EXTRACT_AND_EXPAND"), 1);
  ASSERT_EQ(EVP_PKEY_CTX_ctrl_str(ctx.get(), "md", "SHA256"), 1);
  ASSERT_EQ(EVP_PKEY_CTX_ctrl_str(ctx.get(), "hexsalt", hkdf_hexsalt), 1);
  ASSERT_EQ(EVP_PKEY_CTX_ctrl_str(ctx.get(), "hexinfo", hkdf_hexinfo), 1);
  ASSERT_EQ(EVP_PKEY_CTX_ctrl_str(ctx.get(), "hexkey", hkdf_hexkey), 1);

  size_t okm_len;
  bssl::UniquePtr<uint8_t> expected_okm(
      OPENSSL_hexstr2buf(hkdf_hex_expected_okm, &okm_len));
  ASSERT_TRUE(expected_okm);

  bssl::UniquePtr<uint8_t> actual_okm(
      static_cast<uint8_t *>(OPENSSL_zalloc(okm_len)));
  ASSERT_TRUE(actual_okm);

  ASSERT_TRUE(EVP_PKEY_derive(ctx.get(), actual_okm.get(), &okm_len));

  ASSERT_EQ(OPENSSL_memcmp(actual_okm.get(), expected_okm.get(), okm_len), 0);
}

TEST_F(EvpPkeyCtxCtrlStrTest, HkdfRaw) {
  // Test Cases from RFC 5869.

  bssl::UniquePtr<EVP_PKEY_CTX> ctx(
      EVP_PKEY_CTX_new_id(EVP_PKEY_HKDF, nullptr));
  ASSERT_TRUE(ctx);
  ASSERT_TRUE(EVP_PKEY_derive_init(ctx.get()));
  ASSERT_EQ(EVP_PKEY_CTX_ctrl_str(ctx.get(), "mode", "EXTRACT_AND_EXPAND"), 1);
  ASSERT_EQ(EVP_PKEY_CTX_ctrl_str(ctx.get(), "md", "SHA256"), 1);

  // The salt in the KAT contains a 0-byte so "salt" cannot be used.
  ASSERT_EQ(
      EVP_PKEY_CTX_ctrl_str(ctx.get(), "hexsalt", "000102030405060708090a0b0c"),
      1);


  size_t len;
  bssl::UniquePtr<uint8_t> info_parsed(OPENSSL_hexstr2buf(hkdf_hexinfo, &len));
  bssl::UniquePtr<uint8_t> info((uint8_t*)OPENSSL_zalloc(len+1));
  OPENSSL_memcpy(info.get(), info_parsed.get(), len);

  ASSERT_EQ(EVP_PKEY_CTX_ctrl_str(ctx.get(), "info",
                                  reinterpret_cast<const char *>(info.get())),
            1);
  bssl::UniquePtr<uint8_t> key_parsed(OPENSSL_hexstr2buf(hkdf_hexkey, &len));
  bssl::UniquePtr<uint8_t> key((uint8_t*)OPENSSL_zalloc(len+1));
  OPENSSL_memcpy(key.get(), key_parsed.get(), len);

  ASSERT_EQ(EVP_PKEY_CTX_ctrl_str(ctx.get(), "key",
                                  reinterpret_cast<const char *>(key.get())),
            1);

  size_t okm_len;
  bssl::UniquePtr<uint8_t> expected_okm(
      OPENSSL_hexstr2buf(hkdf_hex_expected_okm, &okm_len));
  ASSERT_TRUE(expected_okm);

  bssl::UniquePtr<uint8_t> actual_okm(
      static_cast<uint8_t *>(OPENSSL_zalloc(okm_len)));
  ASSERT_TRUE(actual_okm);

  ASSERT_TRUE(EVP_PKEY_derive(ctx.get(), actual_okm.get(), &okm_len));

  ASSERT_EQ(OPENSSL_memcmp(actual_okm.get(), expected_okm.get(), okm_len), 0);
}

TEST_F(EvpPkeyCtxCtrlStrTest, HkdfExtract) {
  // Test Cases from RFC 5869.

  bssl::UniquePtr<EVP_PKEY_CTX> ctx(
      EVP_PKEY_CTX_new_id(EVP_PKEY_HKDF, nullptr));
  ASSERT_TRUE(ctx);
  ASSERT_TRUE(EVP_PKEY_derive_init(ctx.get()));
  ASSERT_EQ(EVP_PKEY_CTX_ctrl_str(ctx.get(), "mode", "EXTRACT_ONLY"), 1);
  ASSERT_EQ(EVP_PKEY_CTX_ctrl_str(ctx.get(), "md", "SHA256"), 1);
  ASSERT_EQ(EVP_PKEY_CTX_ctrl_str(ctx.get(), "hexsalt", hkdf_hexsalt), 1);
  ASSERT_EQ(EVP_PKEY_CTX_ctrl_str(ctx.get(), "hexinfo", hkdf_hexinfo), 1);
  ASSERT_EQ(EVP_PKEY_CTX_ctrl_str(ctx.get(), "hexkey", hkdf_hexkey), 1);

  size_t prk_len;
  bssl::UniquePtr<uint8_t> expected_prk(
      OPENSSL_hexstr2buf(hkdf_hex_expected_prk, &prk_len));
  ASSERT_TRUE(expected_prk);

  bssl::UniquePtr<uint8_t> actual_prk(
      static_cast<uint8_t *>(OPENSSL_zalloc(prk_len)));
  ASSERT_TRUE(actual_prk);

  ASSERT_TRUE(EVP_PKEY_derive(ctx.get(), actual_prk.get(), &prk_len));

  ASSERT_EQ(OPENSSL_memcmp(actual_prk.get(), expected_prk.get(), prk_len), 0);
}

static const char *hmac_hexkey = "0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b";

TEST_F(EvpPkeyCtxCtrlStrTest, HMACKey) {

  bssl::UniquePtr<EVP_PKEY> pkey_hex;
  {
    bssl::UniquePtr<EVP_PKEY_CTX> ctx_hex(EVP_PKEY_CTX_new_id(EVP_PKEY_HMAC, NULL));
    ASSERT_TRUE(ctx_hex);
    ASSERT_TRUE(EVP_PKEY_keygen_init(ctx_hex.get()));

    ASSERT_NE(1, EVP_PKEY_CTX_ctrl_str(ctx_hex.get(), "hexkey", "nonsense"));
    ASSERT_TRUE(EVP_PKEY_CTX_ctrl_str(ctx_hex.get(), "hexkey", hmac_hexkey));
    EVP_PKEY* my_pkey = NULL;
    ASSERT_TRUE(EVP_PKEY_keygen(ctx_hex.get(), &my_pkey));
    pkey_hex.reset(my_pkey);
    ASSERT_TRUE(pkey_hex);
  }

  bssl::UniquePtr<EVP_PKEY> pkey_raw;
  {
    bssl::UniquePtr<EVP_PKEY_CTX> ctx_hex(EVP_PKEY_CTX_new_id(EVP_PKEY_HMAC, NULL));
    ASSERT_TRUE(ctx_hex);
    ASSERT_TRUE(EVP_PKEY_keygen_init(ctx_hex.get()));

    std::vector<uint8_t> raw_key;
    DecodeHex(&raw_key, hmac_hexkey);
    raw_key.push_back(0);
    ASSERT_TRUE(EVP_PKEY_CTX_ctrl_str(ctx_hex.get(), "key", (char*)raw_key.data()));
    EVP_PKEY* my_pkey = NULL;
    ASSERT_TRUE(EVP_PKEY_keygen(ctx_hex.get(), &my_pkey));
    pkey_raw.reset(my_pkey);
    ASSERT_TRUE(pkey_raw);
  }

  ASSERT_TRUE(EVP_PKEY_cmp(pkey_hex.get(), pkey_raw.get()));
}

TEST_F(EvpPkeyCtxCtrlStrTest, HMACKeyCtxDup) {
  // Test that EVP_PKEY_CTX_dup works correctly with HMAC keys.
  // This tests the hmac_copy function to ensure it copies the key correctly.
  // Generate from duplicate FIRST, then from original, to detect if copy
  // corrupts the source context.

  // Get the expected key bytes
  std::vector<uint8_t> expected_key;
  DecodeHex(&expected_key, hmac_hexkey);
  ASSERT_GT(expected_key.size(), 0u);

  bssl::UniquePtr<EVP_PKEY_CTX> ctx(EVP_PKEY_CTX_new_id(EVP_PKEY_HMAC, NULL));
  ASSERT_TRUE(ctx);
  ASSERT_TRUE(EVP_PKEY_keygen_init(ctx.get()));
  ASSERT_TRUE(EVP_PKEY_CTX_ctrl_str(ctx.get(), "hexkey", hmac_hexkey));

  // Duplicate the context
  bssl::UniquePtr<EVP_PKEY_CTX> ctx_dup(EVP_PKEY_CTX_dup(ctx.get()));
  ASSERT_TRUE(ctx_dup);

  // Generate from DUPLICATE first - this verifies the key was copied
  EVP_PKEY* pkey_dup_raw = NULL;
  ASSERT_TRUE(EVP_PKEY_keygen(ctx_dup.get(), &pkey_dup_raw));
  bssl::UniquePtr<EVP_PKEY> pkey_dup(pkey_dup_raw);
  ASSERT_TRUE(pkey_dup);

  // Generate from ORIGINAL - verifies copy didn't corrupt source
  EVP_PKEY* pkey_orig_raw = NULL;
  ASSERT_TRUE(EVP_PKEY_keygen(ctx.get(), &pkey_orig_raw));
  bssl::UniquePtr<EVP_PKEY> pkey_orig(pkey_orig_raw);
  ASSERT_TRUE(pkey_orig);

  // Verify both keys have the correct length and content using raw key access
  // EVP_PKEY_cmp returns -2 for HMAC (no pub_cmp method), so we must compare
  // the raw key bytes directly.
  size_t dup_key_len = 0;
  ASSERT_TRUE(EVP_PKEY_get_raw_private_key(pkey_dup.get(), NULL, &dup_key_len));
  ASSERT_EQ(dup_key_len, expected_key.size());

  size_t orig_key_len = 0;
  ASSERT_TRUE(EVP_PKEY_get_raw_private_key(pkey_orig.get(), NULL, &orig_key_len));
  ASSERT_EQ(orig_key_len, expected_key.size());

  std::vector<uint8_t> dup_key(dup_key_len);
  ASSERT_TRUE(EVP_PKEY_get_raw_private_key(pkey_dup.get(), dup_key.data(), &dup_key_len));

  std::vector<uint8_t> orig_key(orig_key_len);
  ASSERT_TRUE(EVP_PKEY_get_raw_private_key(pkey_orig.get(), orig_key.data(), &orig_key_len));

  // Both keys should match each other and the expected key
  ASSERT_EQ(orig_key, expected_key);
  ASSERT_EQ(dup_key, expected_key);
}




static void verify_DSA(const DSA* dsa, unsigned psize, unsigned qsize) {
  const BIGNUM* p = DSA_get0_p(dsa);
  EXPECT_TRUE(p != NULL);
  if (p == NULL) {
    return;
  }
  EXPECT_EQ(BN_num_bytes(p), psize);
  const BIGNUM* q = DSA_get0_q(dsa);
  EXPECT_TRUE(q != NULL);
  if (q == NULL) {
    return;
  }
  EXPECT_EQ(BN_num_bytes(q), qsize);
}


TEST_F(EvpPkeyCtxCtrlStrTest, DSAParamGen) {

  {
    bssl::UniquePtr<EVP_PKEY_CTX> ctx(
        EVP_PKEY_CTX_new_id(EVP_PKEY_DSA, nullptr));
    ASSERT_TRUE(ctx);
    ASSERT_TRUE(EVP_PKEY_paramgen_init(ctx.get()));
    ASSERT_EQ(EVP_PKEY_CTX_ctrl_str(ctx.get(), "dsa_paramgen_bits", "512"), 1);
    ASSERT_NE(EVP_PKEY_CTX_ctrl_str(ctx.get(), "dsa_paramgen_bits", "256"), 1);
    ASSERT_NE(EVP_PKEY_CTX_ctrl_str(ctx.get(), "dsa_paramgen_bits", "a125"), 1);
    ASSERT_EQ(EVP_PKEY_CTX_ctrl_str(ctx.get(), "dsa_paramgen_md", "SHA1"), 1);
    ASSERT_NE(EVP_PKEY_CTX_ctrl_str(ctx.get(), "dsa_paramgen_md", "sha123"), 1);

    EVP_PKEY *pkey_raw = NULL;
    EVP_PKEY_paramgen(ctx.get(), &pkey_raw);
    bssl::UniquePtr<EVP_PKEY> pkey(pkey_raw);
    ASSERT_TRUE(pkey);

    DSA *dsa_raw = EVP_PKEY_get0_DSA(pkey_raw);
    ASSERT_TRUE(dsa_raw != NULL);
    verify_DSA(dsa_raw, 512 / 8, 160 / 8);
  }

  {
    bssl::UniquePtr<EVP_PKEY_CTX> ctx(
        EVP_PKEY_CTX_new_id(EVP_PKEY_DSA, nullptr));
    ASSERT_TRUE(ctx);
    ASSERT_TRUE(EVP_PKEY_paramgen_init(ctx.get()));
    ASSERT_EQ(EVP_PKEY_CTX_ctrl_str(ctx.get(), "dsa_paramgen_bits", "768"), 1);
    ASSERT_EQ(EVP_PKEY_CTX_ctrl_str(ctx.get(), "dsa_paramgen_q_bits", "224"), 1);
    ASSERT_NE(EVP_PKEY_CTX_ctrl_str(ctx.get(), "dsa_paramgen_q_bits", "128"), 1);
    ASSERT_NE(EVP_PKEY_CTX_ctrl_str(ctx.get(), "dsa_paramgen_q_bits", "aghj"), 1);

    EVP_PKEY *pkey_raw = NULL;
    EVP_PKEY_paramgen(ctx.get(), &pkey_raw);
    bssl::UniquePtr<EVP_PKEY> pkey(pkey_raw);
    ASSERT_TRUE(pkey);

    DSA *dsa_raw = EVP_PKEY_get0_DSA(pkey_raw);
    ASSERT_TRUE(dsa_raw != NULL);
    verify_DSA(dsa_raw, 768 / 8, 224 / 8);
  }

  {
    bssl::UniquePtr<EVP_PKEY_CTX> ctx(
        EVP_PKEY_CTX_new_id(EVP_PKEY_DSA, nullptr));
    ASSERT_TRUE(ctx);
    ASSERT_TRUE(EVP_PKEY_paramgen_init(ctx.get()));
    ASSERT_EQ(EVP_PKEY_CTX_ctrl_str(ctx.get(), "dsa_paramgen_bits", "512"), 1);
    ASSERT_EQ(EVP_PKEY_CTX_ctrl_str(ctx.get(), "dsa_paramgen_q_bits", "160"), 1);
    // MD takes precedence over qbits
    ASSERT_EQ(EVP_PKEY_CTX_ctrl_str(ctx.get(), "dsa_paramgen_md", "SHA256"), 1);

    EVP_PKEY *pkey_raw = NULL;
    EVP_PKEY_paramgen(ctx.get(), &pkey_raw);
    bssl::UniquePtr<EVP_PKEY> pkey(pkey_raw);
    ASSERT_TRUE(pkey);

    DSA *dsa_raw = EVP_PKEY_get0_DSA(pkey_raw);
    ASSERT_TRUE(dsa_raw != NULL);
    verify_DSA(dsa_raw, 512 / 8, 256 / 8);
  }
}
