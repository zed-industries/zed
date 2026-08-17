// Copyright (c) 1998-2005 The OpenSSL Project.  All rights reserved.
// SPDX-License-Identifier: Apache-2.0

#include <openssl/ecdsa.h>

#include <vector>

#include <gtest/gtest.h>

#include <openssl/bn.h>
#include <openssl/crypto.h>
#include <openssl/ec.h>
#include <openssl/err.h>
#include <openssl/evp.h>
#include <openssl/mem.h>
#include <openssl/nid.h>
#include <openssl/rand.h>

#include "../ec/internal.h"
#include "../../test/file_test.h"
#include "../../test/test_util.h"


// Though we do not support secp160r1, it is reachable from the deprecated
// custom curve APIs and has some unique properties (n is larger than p with the
// difference crossing a word boundary on 32-bit), so test it explicitly.
static bssl::UniquePtr<EC_GROUP> NewSecp160r1Group() {
  static const char kP[] = "ffffffffffffffffffffffffffffffff7fffffff";
  static const char kA[] = "ffffffffffffffffffffffffffffffff7ffffffc";
  static const char kB[] = "1c97befc54bd7a8b65acf89f81d4d4adc565fa45";
  static const char kX[] = "4a96b5688ef573284664698968c38bb913cbfc82";
  static const char kY[] = "23a628553168947d59dcc912042351377ac5fb32";
  static const char kN[] = "0100000000000000000001f4c8f927aed3ca752257";

  bssl::UniquePtr<BIGNUM> p = HexToBIGNUM(kP), a = HexToBIGNUM(kA),
                          b = HexToBIGNUM(kB), x = HexToBIGNUM(kX),
                          y = HexToBIGNUM(kY), n = HexToBIGNUM(kN);
  if (!p || !a || !b || !x || !y || !n) {
    return nullptr;
  }

  bssl::UniquePtr<EC_GROUP> group(
      EC_GROUP_new_curve_GFp(p.get(), a.get(), b.get(), nullptr));
  if (!group) {
    return nullptr;
  }
  bssl::UniquePtr<EC_POINT> g(EC_POINT_new(group.get()));
  if (!g ||
      !EC_POINT_set_affine_coordinates_GFp(group.get(), g.get(), x.get(),
                                           y.get(), nullptr) ||
      !EC_GROUP_set_generator(group.get(), g.get(), n.get(), BN_value_one())) {
    return nullptr;
  }
  return group;
}

enum API {
  kEncodedAPI,
  kRawAPI,
};

// VerifyECDSASig checks that verifying |ecdsa_sig| gives |expected_result|.
static void VerifyECDSASig(API api, const uint8_t *digest, size_t digest_len,
                           const ECDSA_SIG *ecdsa_sig, EC_KEY *eckey,
                           int expected_result) {
  switch (api) {
    case kEncodedAPI: {
      uint8_t *der;
      size_t der_len;
      ASSERT_TRUE(ECDSA_SIG_to_bytes(&der, &der_len, ecdsa_sig));
      bssl::UniquePtr<uint8_t> delete_der(der);
      EXPECT_EQ(expected_result,
                ECDSA_verify(0, digest, digest_len, der, der_len, eckey));
      break;
    }

    case kRawAPI:
      EXPECT_EQ(expected_result,
                ECDSA_do_verify(digest, digest_len, ecdsa_sig, eckey));
      break;

    default:
      FAIL() << "Unknown API type.";
  }
}

// TestTamperedSig verifies that signature verification fails when a valid
// signature is tampered with. |ecdsa_sig| must be a valid signature, which will
// be modified.
static void TestTamperedSig(API api, const uint8_t *digest,
                            size_t digest_len, ECDSA_SIG *ecdsa_sig,
                            EC_KEY *eckey, const BIGNUM *order) {
  SCOPED_TRACE(api);
  // Modify a single byte of the signature: to ensure we don't
  // garble the ASN1 structure, we read the raw signature and
  // modify a byte in one of the bignums directly.

  // Store the two BIGNUMs in raw_buf.
  size_t r_len = BN_num_bytes(ecdsa_sig->r);
  size_t s_len = BN_num_bytes(ecdsa_sig->s);
  size_t bn_len = BN_num_bytes(order);
  ASSERT_LE(r_len, bn_len);
  ASSERT_LE(s_len, bn_len);
  size_t buf_len = 2 * bn_len;
  std::vector<uint8_t> raw_buf(buf_len);
  // Pad the bignums with leading zeroes.
  ASSERT_TRUE(BN_bn2bin_padded(raw_buf.data(), bn_len, ecdsa_sig->r));
  ASSERT_TRUE(BN_bn2bin_padded(raw_buf.data() + bn_len, bn_len, ecdsa_sig->s));

  // Modify a single byte in the buffer.
  size_t offset = raw_buf[10] % buf_len;
  uint8_t dirt = raw_buf[11] ? raw_buf[11] : 1;
  raw_buf[offset] ^= dirt;
  // Now read the BIGNUMs back in from raw_buf.
  ASSERT_TRUE(BN_bin2bn(raw_buf.data(), bn_len, ecdsa_sig->r));
  ASSERT_TRUE(BN_bin2bn(raw_buf.data() + bn_len, bn_len, ecdsa_sig->s));
  VerifyECDSASig(api, digest, digest_len, ecdsa_sig, eckey, 0);

  // Sanity check: Undo the modification and verify signature.
  raw_buf[offset] ^= dirt;
  ASSERT_TRUE(BN_bin2bn(raw_buf.data(), bn_len, ecdsa_sig->r));
  ASSERT_TRUE(BN_bin2bn(raw_buf.data() + bn_len, bn_len, ecdsa_sig->s));
  VerifyECDSASig(api, digest, digest_len, ecdsa_sig, eckey, 1);
}

TEST(ECDSATest, BuiltinCurves) {
  // Fill digest values with some random data.
  uint8_t digest[20], wrong_digest[20];
  ASSERT_TRUE(RAND_bytes(digest, 20));
  CONSTTIME_DECLASSIFY(digest, 20);
  ASSERT_TRUE(RAND_bytes(wrong_digest, 20));
  CONSTTIME_DECLASSIFY(wrong_digest, 20);

  static const struct {
    int nid;
    const char *name;
  } kCurves[] = {
      { NID_secp224r1, "secp224r1" },
      { NID_X9_62_prime256v1, "secp256r1" },
      { NID_secp384r1, "secp384r1" },
      { NID_secp521r1, "secp521r1" },
      { NID_secp160r1, "secp160r1" },
      { NID_secp256k1, "secp256k1" },
  };

  for (const auto &curve : kCurves) {
    SCOPED_TRACE(curve.name);

    bssl::UniquePtr<EC_GROUP> group;
    if (curve.nid == NID_secp160r1) {
      group = NewSecp160r1Group();
    } else {
      group.reset(EC_GROUP_new_by_curve_name(curve.nid));
    }
    ASSERT_TRUE(group);
    const BIGNUM *order = EC_GROUP_get0_order(group.get());

    // Create a new ECDSA key.
    bssl::UniquePtr<EC_KEY> eckey(EC_KEY_new());
    ASSERT_TRUE(eckey);
    ASSERT_TRUE(EC_KEY_set_group(eckey.get(), group.get()));
    ASSERT_TRUE(EC_KEY_generate_key(eckey.get()));

    // Create a second key.
    bssl::UniquePtr<EC_KEY> wrong_eckey(EC_KEY_new());
    ASSERT_TRUE(wrong_eckey);
    ASSERT_TRUE(EC_KEY_set_group(wrong_eckey.get(), group.get()));
    ASSERT_TRUE(EC_KEY_generate_key(wrong_eckey.get()));

    // Check the key.
    EXPECT_TRUE(EC_KEY_check_key(eckey.get()));
    bssl::UniquePtr<EVP_PKEY> ec_pkey(EVP_PKEY_new());
    ASSERT_TRUE(ec_pkey);
    ASSERT_TRUE(EVP_PKEY_set1_EC_KEY(ec_pkey.get(), eckey.get()));
    bssl::UniquePtr<EVP_PKEY_CTX> ec_key_ctx(
            EVP_PKEY_CTX_new(ec_pkey.get(), NULL));
    ASSERT_TRUE(ec_key_ctx);
    EXPECT_TRUE(EVP_PKEY_check(ec_key_ctx.get()));
    EXPECT_TRUE(EVP_PKEY_public_check((ec_key_ctx.get())));

    // Test ASN.1-encoded signatures.
    // Create a signature.
    std::vector<uint8_t> signature(ECDSA_size(eckey.get()));
    unsigned sig_len;
    ASSERT_TRUE(
        ECDSA_sign(0, digest, 20, signature.data(), &sig_len, eckey.get()));
    signature.resize(sig_len);

    // ECDSA signing should be non-deterministic. This does not verify k is
    // generated securely but at least checks it was randomized at all.
    std::vector<uint8_t> signature2(ECDSA_size(eckey.get()));
    ASSERT_TRUE(
        ECDSA_sign(0, digest, 20, signature2.data(), &sig_len, eckey.get()));
    signature2.resize(sig_len);
    EXPECT_NE(Bytes(signature), Bytes(signature2));

    // Verify the signature.
    EXPECT_TRUE(ECDSA_verify(0, digest, 20, signature.data(), signature.size(),
                             eckey.get()));

    // Verify the signature with the wrong key.
    EXPECT_FALSE(ECDSA_verify(0, digest, 20, signature.data(), signature.size(),
                              wrong_eckey.get()));
    ERR_clear_error();

    // Verify the signature using the wrong digest.
    EXPECT_FALSE(ECDSA_verify(0, wrong_digest, 20, signature.data(),
                              signature.size(), eckey.get()));
    ERR_clear_error();

    // Verify a truncated signature.
    EXPECT_FALSE(ECDSA_verify(0, digest, 20, signature.data(),
                              signature.size() - 1, eckey.get()));
    ERR_clear_error();

    // Verify a tampered signature.
    bssl::UniquePtr<ECDSA_SIG> ecdsa_sig(
        ECDSA_SIG_from_bytes(signature.data(), signature.size()));
    ASSERT_TRUE(ecdsa_sig);
    TestTamperedSig(kEncodedAPI, digest, 20, ecdsa_sig.get(), eckey.get(),
                    order);

    // Test ECDSA_SIG signing and verification.
    // Create a signature.
    ecdsa_sig.reset(ECDSA_do_sign(digest, 20, eckey.get()));
    ASSERT_TRUE(ecdsa_sig);

    // Verify the signature using the correct key.
    EXPECT_TRUE(ECDSA_do_verify(digest, 20, ecdsa_sig.get(), eckey.get()));

    // Verify the signature with the wrong key.
    EXPECT_FALSE(
        ECDSA_do_verify(digest, 20, ecdsa_sig.get(), wrong_eckey.get()));
    ERR_clear_error();

    // Verify the signature using the wrong digest.
    EXPECT_FALSE(
        ECDSA_do_verify(wrong_digest, 20, ecdsa_sig.get(), eckey.get()));
    ERR_clear_error();

    // Verify a tampered signature.
    TestTamperedSig(kRawAPI, digest, 20, ecdsa_sig.get(), eckey.get(), order);
  }
}

static size_t BitsToBytes(size_t bits) {
  return (bits / 8) + (7 + (bits % 8)) / 8;
}

TEST(ECDSATest, MaxSigLen) {
  static const size_t kBits[] = {224, 256, 384, 521, 10000};
  for (size_t bits : kBits) {
    SCOPED_TRACE(bits);
    size_t order_len = BitsToBytes(bits);

    // Create the largest possible |ECDSA_SIG| of the given constraints.
    bssl::UniquePtr<ECDSA_SIG> sig(ECDSA_SIG_new());
    ASSERT_TRUE(sig);
    std::vector<uint8_t> bytes(order_len, 0xff);
    ASSERT_TRUE(BN_bin2bn(bytes.data(), bytes.size(), sig->r));
    ASSERT_TRUE(BN_bin2bn(bytes.data(), bytes.size(), sig->s));
    // Serialize it.
    uint8_t *der;
    size_t der_len;
    ASSERT_TRUE(ECDSA_SIG_to_bytes(&der, &der_len, sig.get()));
    OPENSSL_free(der);

    EXPECT_EQ(der_len, ECDSA_SIG_max_len(order_len));
  }
}

static bssl::UniquePtr<EC_GROUP> GetCurve(FileTest *t, const char *key) {
  std::string curve_name;
  if (!t->GetAttribute(&curve_name, key)) {
    return nullptr;
  }

  if (curve_name == "P-224") {
    return bssl::UniquePtr<EC_GROUP>(const_cast<EC_GROUP *>(EC_group_p224()));
  }
  if (curve_name == "P-256") {
    return bssl::UniquePtr<EC_GROUP>(const_cast<EC_GROUP *>(EC_group_p256()));
  }
  if (curve_name == "P-384") {
    return bssl::UniquePtr<EC_GROUP>(const_cast<EC_GROUP *>(EC_group_p384()));
  }
  if (curve_name == "P-521") {
    return bssl::UniquePtr<EC_GROUP>(const_cast<EC_GROUP *>(EC_group_p521()));
  }
  if (curve_name == "secp256k1") {
    return bssl::UniquePtr<EC_GROUP>(EC_GROUP_new_by_curve_name(NID_secp256k1));
  }
  if (curve_name == "secp160r1") {
    return NewSecp160r1Group();
  }

  ADD_FAILURE() << "Unknown curve: " << curve_name;
  return nullptr;
}

static bssl::UniquePtr<EC_GROUP> MakeCustomClone(const EC_GROUP *group) {
  bssl::UniquePtr<BN_CTX> ctx(BN_CTX_new());
  bssl::UniquePtr<BIGNUM> p(BN_new()), a(BN_new()), b(BN_new()), x(BN_new()),
      y(BN_new());
  if (!ctx || !p || !a || !b || !x || !y ||
      !EC_GROUP_get_curve_GFp(group, p.get(), a.get(), b.get(), ctx.get()) ||
      !EC_POINT_get_affine_coordinates_GFp(
          group, EC_GROUP_get0_generator(group), x.get(), y.get(), ctx.get())) {
    return nullptr;
  }
  bssl::UniquePtr<EC_GROUP> ret(
      EC_GROUP_new_curve_GFp(p.get(), a.get(), b.get(), ctx.get()));
  if (!ret) {
    return nullptr;
  }
  bssl::UniquePtr<EC_POINT> g(EC_POINT_new(ret.get()));
  if (!g ||
      !EC_POINT_set_affine_coordinates_GFp(ret.get(), g.get(), x.get(), y.get(),
                                           ctx.get()) ||
      !EC_GROUP_set_generator(ret.get(), g.get(), EC_GROUP_get0_order(group),
                              BN_value_one())) {
    return nullptr;
  }
  return ret;
}

static bssl::UniquePtr<BIGNUM> GetBIGNUM(FileTest *t, const char *key) {
  std::vector<uint8_t> bytes;
  if (!t->GetBytes(&bytes, key)) {
    return nullptr;
  }

  return bssl::UniquePtr<BIGNUM>(BN_bin2bn(bytes.data(), bytes.size(), nullptr));
}

TEST(ECDSATest, VerifyTestVectors) {
  FileTestGTest("crypto/fipsmodule/ecdsa/ecdsa_verify_tests.txt",
                [](FileTest *t) {
    for (bool custom_group : {false, true}) {
      SCOPED_TRACE(custom_group);
      bssl::UniquePtr<EC_GROUP> group = GetCurve(t, "Curve");
      ASSERT_TRUE(group);
      if (custom_group) {
        group = MakeCustomClone(group.get());
        ASSERT_TRUE(group);
      }
      bssl::UniquePtr<BIGNUM> x = GetBIGNUM(t, "X");
      ASSERT_TRUE(x);
      bssl::UniquePtr<BIGNUM> y = GetBIGNUM(t, "Y");
      ASSERT_TRUE(y);
      bssl::UniquePtr<BIGNUM> r = GetBIGNUM(t, "R");
      ASSERT_TRUE(r);
      bssl::UniquePtr<BIGNUM> s = GetBIGNUM(t, "S");
      ASSERT_TRUE(s);
      std::vector<uint8_t> digest;
      ASSERT_TRUE(t->GetBytes(&digest, "Digest"));

      bssl::UniquePtr<EC_KEY> key(EC_KEY_new());
      ASSERT_TRUE(key);
      bssl::UniquePtr<EC_POINT> pub_key(EC_POINT_new(group.get()));
      ASSERT_TRUE(pub_key);
      bssl::UniquePtr<ECDSA_SIG> sig(ECDSA_SIG_new());
      ASSERT_TRUE(sig);
      ASSERT_TRUE(EC_KEY_set_group(key.get(), group.get()));
      ASSERT_TRUE(EC_POINT_set_affine_coordinates_GFp(
          group.get(), pub_key.get(), x.get(), y.get(), nullptr));
      ASSERT_TRUE(EC_KEY_set_public_key(key.get(), pub_key.get()));
      ASSERT_TRUE(BN_copy(sig->r, r.get()));
      ASSERT_TRUE(BN_copy(sig->s, s.get()));

      EXPECT_EQ(
          t->HasAttribute("Invalid") ? 0 : 1,
          ECDSA_do_verify(digest.data(), digest.size(), sig.get(), key.get()));
    }
  });
}

TEST(ECDSATest, SignTestVectors) {
  FileTestGTest("crypto/fipsmodule/ecdsa/ecdsa_sign_tests.txt",
                [](FileTest *t) {
    for (bool custom_group : {false, true}) {
      SCOPED_TRACE(custom_group);
      bssl::UniquePtr<EC_GROUP> group = GetCurve(t, "Curve");
      ASSERT_TRUE(group);
      if (custom_group) {
        group = MakeCustomClone(group.get());
        ASSERT_TRUE(group);
      }
      bssl::UniquePtr<BIGNUM> priv_key = GetBIGNUM(t, "Private");
      ASSERT_TRUE(priv_key);
      bssl::UniquePtr<BIGNUM> x = GetBIGNUM(t, "X");
      ASSERT_TRUE(x);
      bssl::UniquePtr<BIGNUM> y = GetBIGNUM(t, "Y");
      ASSERT_TRUE(y);
      std::vector<uint8_t> k;
      ASSERT_TRUE(t->GetBytes(&k, "K"));
      bssl::UniquePtr<BIGNUM> r = GetBIGNUM(t, "R");
      ASSERT_TRUE(r);
      bssl::UniquePtr<BIGNUM> s = GetBIGNUM(t, "S");
      ASSERT_TRUE(s);
      std::vector<uint8_t> digest;
      ASSERT_TRUE(t->GetBytes(&digest, "Digest"));

      bssl::UniquePtr<EC_KEY> key(EC_KEY_new());
      ASSERT_TRUE(key);
      bssl::UniquePtr<EC_POINT> pub_key(EC_POINT_new(group.get()));
      ASSERT_TRUE(pub_key);
      ASSERT_TRUE(EC_KEY_set_group(key.get(), group.get()));
      ASSERT_TRUE(EC_KEY_set_private_key(key.get(), priv_key.get()));
      ASSERT_TRUE(EC_POINT_set_affine_coordinates_GFp(
          group.get(), pub_key.get(), x.get(), y.get(), nullptr));
      ASSERT_TRUE(EC_KEY_set_public_key(key.get(), pub_key.get()));
      ASSERT_TRUE(EC_KEY_check_key(key.get()));

      bssl::UniquePtr<EVP_PKEY> ec_pkey(EVP_PKEY_new());
      ASSERT_TRUE(ec_pkey);
      ASSERT_TRUE(EVP_PKEY_set1_EC_KEY(ec_pkey.get(), key.get()));
      bssl::UniquePtr<EVP_PKEY_CTX> ec_key_ctx(
              EVP_PKEY_CTX_new(ec_pkey.get(), NULL));
      ASSERT_TRUE(ec_key_ctx);
      EXPECT_TRUE(EVP_PKEY_check(ec_key_ctx.get()));
      EXPECT_TRUE(EVP_PKEY_public_check((ec_key_ctx.get())));

      bssl::UniquePtr<ECDSA_SIG> sig(
          ECDSA_sign_with_nonce_and_leak_private_key_for_testing(
              digest.data(), digest.size(), key.get(), k.data(), k.size()));
      ASSERT_TRUE(sig);

      EXPECT_EQ(0, BN_cmp(r.get(), sig->r));
      EXPECT_EQ(0, BN_cmp(s.get(), sig->s));
    }
  });
}
