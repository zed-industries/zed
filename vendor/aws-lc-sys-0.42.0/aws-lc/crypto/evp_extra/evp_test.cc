// Written by Dr Stephen N Henson (steve@openssl.org) for the OpenSSL project.
// Copyright (c) 2015 The OpenSSL Project.  All rights reserved.
// SPDX-License-Identifier: Apache-2.0

#include <openssl/curve25519.h>
#include <openssl/ec_key.h>
#include <openssl/evp.h>

#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "../fipsmodule/evp/internal.h"
#include "internal.h"

OPENSSL_MSVC_PRAGMA(warning(push))
OPENSSL_MSVC_PRAGMA(warning(disable: 4702))

#include <map>
#include <string>
#include <utility>
#include <vector>

OPENSSL_MSVC_PRAGMA(warning(pop))

#include <gtest/gtest.h>

#include <openssl/bn.h>
#include <openssl/bytestring.h>
#include <openssl/crypto.h>
#include <openssl/digest.h>
#include <openssl/dh.h>
#include <openssl/dsa.h>
#include <openssl/err.h>
#include <openssl/rsa.h>

#include "../test/file_test.h"
#include "../test/test_util.h"
#include "../test/wycheproof_util.h"


// evp_test dispatches between multiple test types. PublicKey and PrivateKey
// tests take a key name parameter and key information. If the test is
// successful, the key is saved under that key name. Decrypt, Sign, and Verify
// tests take a previously imported key name as parameter and test their
// respective operations.

static const EVP_MD *GetDigest(const std::string &name) {
  if (name == "MD5") {
    return EVP_md5();
  } else if (name == "SHA1") {
    return EVP_sha1();
  } else if (name == "SHA224") {
    return EVP_sha224();
  } else if (name == "SHA256") {
    return EVP_sha256();
  } else if (name == "SHA384") {
    return EVP_sha384();
  } else if (name == "SHA512") {
    return EVP_sha512();
  } else if (name == "SHA512/224") {
    return EVP_sha512_224();
  } else if (name == "SHA512/256") {
    return EVP_sha512_256();
  } else if (name == "SHA3-224") {
    return EVP_sha3_224();
  } else if (name == "SHA3-256") {
    return EVP_sha3_256();
  } else if (name == "SHA3-384") {
    return EVP_sha3_384();
  } else if (name == "SHA3-512") {
    return EVP_sha3_512();
  } else if (name == "SHAKE128") {
    return EVP_shake128();
  } else if (name == "SHAKE256") {
    return EVP_shake256();
  }
  ADD_FAILURE() << "Unknown digest: " << name;
  return nullptr;
}

static int GetKeyType(const std::string &name) {
  if (name == "RSA") {
    return EVP_PKEY_RSA;
  }
  if (name == "EC") {
    return EVP_PKEY_EC;
  }
  if (name == "DSA") {
    return EVP_PKEY_DSA;
  }
  if (name == "Ed25519") {
    return EVP_PKEY_ED25519;
  }
  if (name == "X25519") {
    return EVP_PKEY_X25519;
  }
  ADD_FAILURE() << "Unknown key type: " << name;
  return EVP_PKEY_NONE;
}

static bool GetRSAPadding(FileTest *t, int *out, const std::string &name) {
  if (name == "PKCS1") {
    *out = RSA_PKCS1_PADDING;
    return true;
  }
  if (name == "PSS") {
    *out = RSA_PKCS1_PSS_PADDING;
    return true;
  }
  if (name == "OAEP") {
    *out = RSA_PKCS1_OAEP_PADDING;
    return true;
  }
  if (name == "None") {
    *out = RSA_NO_PADDING;
    return true;
  }
  ADD_FAILURE() << "Unknown RSA padding mode: " << name;
  return false;
}

using KeyMap = std::map<std::string, bssl::UniquePtr<EVP_PKEY>>;

enum class KeyRole { kPublic, kPrivate };

static void CheckRSAParam(FileTest *t, const std::string &attr_name,
                          const EVP_PKEY *pkey,
                          const BIGNUM *(*rsa_getter)(const RSA *)) {
  SCOPED_TRACE(attr_name);
  if (t->HasAttribute(attr_name)) {
    bssl::UniquePtr<BIGNUM> want =
        HexToBIGNUM(t->GetAttributeOrDie(attr_name).c_str());
    ASSERT_TRUE(want);

    const RSA *rsa = EVP_PKEY_get0_RSA(pkey);
    ASSERT_TRUE(rsa);
    const BIGNUM *got = rsa_getter(rsa);
    ASSERT_TRUE(got);
    EXPECT_EQ(BN_cmp(want.get(), got), 0)
        << "wanted: " << BIGNUMToHex(want.get())
        << "\ngot: " << BIGNUMToHex(got);
  }
  // We have many test RSA keys so, for now, don't require that all RSA keys
  // list out these parameters. That is, the absence of an RSA parameter does
  // not currently assert that we omit them.
}

static bool ImportKey(FileTest *t, KeyMap *key_map, KeyRole key_role,
                      int (*marshal_func)(CBB *cbb, const EVP_PKEY *key)) {
  auto parse_func = key_role == KeyRole::kPublic ? &EVP_parse_public_key
                                                 : &EVP_parse_private_key;
  if (key_role == KeyRole::kPublic) {
    marshal_func = &EVP_marshal_public_key;
  }

  // This test will first import the key from all available methods, then check
  // that all properties on all keys match.
  std::vector<std::pair<std::string, bssl::UniquePtr<EVP_PKEY>>> keys;

  // Parse from SPKI or PKCS#8.
  std::vector<uint8_t> input;
  if (!t->GetBytes(&input, "Input")) {
    return false;
  }
  CBS cbs;
  CBS_init(&cbs, input.data(), input.size());
  bssl::UniquePtr<EVP_PKEY> new_key(parse_func(&cbs));
  if (new_key == nullptr || CBS_len(&cbs) != 0) {
    return false;
  }
  keys.emplace_back(key_role == KeyRole::kPublic ? "spki" : "pkcs8",
                    std::move(new_key));

  std::string key_type_str;
  if (!t->GetAttribute(&key_type_str, "Type")) {
    return false;
  }
  int key_type = GetKeyType(key_type_str);

  // Import as a raw key.
  if (key_role == KeyRole::kPublic && t->HasAttribute("RawPublic")) {
    std::vector<uint8_t> raw;
    if (!t->GetBytes(&raw, "RawPublic")) {
      return false;
    }
    new_key.reset(
        EVP_PKEY_new_raw_public_key(key_type, nullptr, raw.data(), raw.size()));
    if (new_key == nullptr) {
      return false;
    }
    keys.emplace_back("raw public", std::move(new_key));
  }
  if (key_role == KeyRole::kPrivate && t->HasAttribute("RawPrivate")) {
    std::vector<uint8_t> raw;
    if (!t->GetBytes(&raw, "RawPrivate")) {
      return false;
    }
    new_key.reset(EVP_PKEY_new_raw_private_key(key_type, nullptr, raw.data(),
                                               raw.size()));
    if (new_key == nullptr) {
      return false;
    }
    keys.emplace_back("raw private", std::move(new_key));
  }

  // Import RSA key from parameters.
  if (key_type == EVP_PKEY_RSA) {
    if (key_role == KeyRole::kPublic && t->HasAttribute("RSAParamN") &&
        t->HasAttribute("RSAParamE")) {
      bssl::UniquePtr<BIGNUM> n =
          HexToBIGNUM(t->GetAttributeOrDie("RSAParamN").c_str());
      bssl::UniquePtr<BIGNUM> e =
          HexToBIGNUM(t->GetAttributeOrDie("RSAParamE").c_str());
      if (n == nullptr || e == nullptr) {
        return false;
      }
      bssl::UniquePtr<RSA> rsa(RSA_new_public_key(n.get(), e.get()));
      new_key.reset(EVP_PKEY_new());
      if (rsa == nullptr || new_key == nullptr ||
          !EVP_PKEY_set1_RSA(new_key.get(), rsa.get())) {
        return false;
      }
      keys.emplace_back("RSA public params", std::move(new_key));
    }
    if (key_role == KeyRole::kPrivate && t->HasAttribute("RSAParamN") &&
        t->HasAttribute("RSAParamE") && t->HasAttribute("RSAParamD") &&
        t->HasAttribute("RSAParamP") && t->HasAttribute("RSAParamQ") &&
        t->HasAttribute("RSAParamDMP1") && t->HasAttribute("RSAParamDMQ1") &&
        t->HasAttribute("RSAParamIQMP")) {
      bssl::UniquePtr<BIGNUM> n =
          HexToBIGNUM(t->GetAttributeOrDie("RSAParamN").c_str());
      bssl::UniquePtr<BIGNUM> e =
          HexToBIGNUM(t->GetAttributeOrDie("RSAParamE").c_str());
      bssl::UniquePtr<BIGNUM> d =
          HexToBIGNUM(t->GetAttributeOrDie("RSAParamD").c_str());
      bssl::UniquePtr<BIGNUM> p =
          HexToBIGNUM(t->GetAttributeOrDie("RSAParamP").c_str());
      bssl::UniquePtr<BIGNUM> q =
          HexToBIGNUM(t->GetAttributeOrDie("RSAParamQ").c_str());
      bssl::UniquePtr<BIGNUM> dmp1 =
          HexToBIGNUM(t->GetAttributeOrDie("RSAParamDMP1").c_str());
      bssl::UniquePtr<BIGNUM> dmq1 =
          HexToBIGNUM(t->GetAttributeOrDie("RSAParamDMQ1").c_str());
      bssl::UniquePtr<BIGNUM> iqmp =
          HexToBIGNUM(t->GetAttributeOrDie("RSAParamIQMP").c_str());
      if (n == nullptr || e == nullptr) {
        return false;
      }
      bssl::UniquePtr<RSA> rsa(RSA_new_private_key(n.get(), e.get(), d.get(),
                                                   p.get(), q.get(), dmp1.get(),
                                                   dmq1.get(), iqmp.get()));
      new_key.reset(EVP_PKEY_new());
      if (rsa == nullptr || new_key == nullptr ||
          !EVP_PKEY_set1_RSA(new_key.get(), rsa.get())) {
        return false;
      }
      keys.emplace_back("RSA private params", std::move(new_key));
    }
  }

  // Check properties of the keys.
  for (const auto &entry : keys) {
    const std::string &name = entry.first;
    const bssl::UniquePtr<EVP_PKEY> &pkey = entry.second;
    SCOPED_TRACE(name);

    EXPECT_EQ(key_type, EVP_PKEY_id(pkey.get()));

    if (t->HasAttribute("Bits")) {
      EXPECT_EQ(EVP_PKEY_bits(pkey.get()),
                atoi(t->GetAttributeOrDie("Bits").c_str()));
    }

    if (EVP_PKEY_id(pkey.get()) == EVP_PKEY_EC) {
      EC_KEY *ec_key = EVP_PKEY_get0_EC_KEY(pkey.get());
      OPENSSL_BEGIN_ALLOW_DEPRECATED
      if (t->HasAttribute("ExpectFromExplicitParams")) {
        EXPECT_EQ(1, EC_KEY_decoded_from_explicit_params(ec_key));
      } else {
        EXPECT_EQ(0, EC_KEY_decoded_from_explicit_params(ec_key));
      }
      OPENSSL_END_ALLOW_DEPRECATED
    }

    CheckRSAParam(t, "RSAParamN", pkey.get(), RSA_get0_n);
    CheckRSAParam(t, "RSAParamE", pkey.get(), RSA_get0_e);
    CheckRSAParam(t, "RSAParamD", pkey.get(), RSA_get0_d);
    CheckRSAParam(t, "RSAParamP", pkey.get(), RSA_get0_p);
    CheckRSAParam(t, "RSAParamQ", pkey.get(), RSA_get0_q);
    CheckRSAParam(t, "RSAParamDMP1", pkey.get(), RSA_get0_dmp1);
    CheckRSAParam(t, "RSAParamDMQ1", pkey.get(), RSA_get0_dmq1);
    CheckRSAParam(t, "RSAParamIQMP", pkey.get(), RSA_get0_iqmp);

    // All keys must compare equal.
    EXPECT_EQ(EVP_PKEY_cmp(pkey.get(), keys.front().second.get()), 1);

    // The key must re-encode correctly.
    bssl::ScopedCBB cbb;
    if (!CBB_init(cbb.get(), 0) || !marshal_func(cbb.get(), pkey.get())) {
      return false;
    }
    std::vector<uint8_t> output = input;
    if (t->HasAttribute("Output") && !t->GetBytes(&output, "Output")) {
      return false;
    }
    EXPECT_EQ(Bytes(output), Bytes(CBB_data(cbb.get()), CBB_len(cbb.get())))
        << "Re-encoding the key did not match.";

    if (t->HasAttribute("RawPrivate")) {
      std::vector<uint8_t> expected;
      if (!t->GetBytes(&expected, "RawPrivate")) {
        return false;
      }

      std::vector<uint8_t> raw;
      size_t len;
      if (!EVP_PKEY_get_raw_private_key(pkey.get(), nullptr, &len)) {
        return false;
      }
      raw.resize(len);
      if (!EVP_PKEY_get_raw_private_key(pkey.get(), raw.data(), &len)) {
        return false;
      }
      raw.resize(len);
      EXPECT_EQ(Bytes(raw), Bytes(expected));

      // Short buffers should be rejected.
      raw.resize(len - 1);
      len = raw.size();
      EXPECT_FALSE(EVP_PKEY_get_raw_private_key(pkey.get(), raw.data(), &len));
    } else {
      size_t len;
      EXPECT_FALSE(EVP_PKEY_get_raw_private_key(pkey.get(), nullptr, &len));
    }

    if (t->HasAttribute("RawPublic")) {
      std::vector<uint8_t> expected;
      if (!t->GetBytes(&expected, "RawPublic")) {
        return false;
      }

      std::vector<uint8_t> raw;
      size_t len;
      if (!EVP_PKEY_get_raw_public_key(pkey.get(), nullptr, &len)) {
        return false;
      }
      raw.resize(len);
      if (!EVP_PKEY_get_raw_public_key(pkey.get(), raw.data(), &len)) {
        return false;
      }
      raw.resize(len);
      EXPECT_EQ(Bytes(raw), Bytes(expected));

      // Short buffers should be rejected.
      raw.resize(len - 1);
      len = raw.size();
      EXPECT_FALSE(EVP_PKEY_get_raw_public_key(pkey.get(), raw.data(), &len));
    } else {
      size_t len;
      EXPECT_FALSE(EVP_PKEY_get_raw_public_key(pkey.get(), nullptr, &len));
    }
  }

  // Save the key for future tests.
  const std::string &key_name = t->GetParameter();
  EXPECT_EQ(0u, key_map->count(key_name)) << "Duplicate key: " << key_name;
  (*key_map)[key_name] = std::move(keys.front().second);
  return true;
}

static bool GetOptionalBignum(FileTest *t, bssl::UniquePtr<BIGNUM> *out,
                              const std::string &key) {
  if (!t->HasAttribute(key)) {
    *out = nullptr;
    return true;
  }

  std::vector<uint8_t> bytes;
  if (!t->GetBytes(&bytes, key)) {
    return false;
  }

  out->reset(BN_bin2bn(bytes.data(), bytes.size(), nullptr));
  return *out != nullptr;
}

static bool ImportDHKey(FileTest *t, KeyMap *key_map) {
  bssl::UniquePtr<BIGNUM> p, q, g, pub_key, priv_key;
  if (!GetOptionalBignum(t, &p, "P") ||  //
      !GetOptionalBignum(t, &q, "Q") ||  //
      !GetOptionalBignum(t, &g, "G") ||
      !GetOptionalBignum(t, &pub_key, "Public") ||
      !GetOptionalBignum(t, &priv_key, "Private")) {
    return false;
  }

  bssl::UniquePtr<DH> dh(DH_new());
  if (dh == nullptr || !DH_set0_pqg(dh.get(), p.get(), q.get(), g.get())) {
    return false;
  }
  // |DH_set0_pqg| takes ownership on success.
  p.release();
  q.release();
  g.release();

  if (!DH_set0_key(dh.get(), pub_key.get(), priv_key.get())) {
    return false;
  }
  // |DH_set0_key| takes ownership on success.
  pub_key.release();
  priv_key.release();

  bssl::UniquePtr<EVP_PKEY> pkey(EVP_PKEY_new());
  if (pkey == nullptr || !EVP_PKEY_set1_DH(pkey.get(), dh.get())) {
    return false;
  }

  // Save the key for future tests.
  const std::string &key_name = t->GetParameter();
  EXPECT_EQ(0u, key_map->count(key_name)) << "Duplicate key: " << key_name;
  (*key_map)[key_name] = std::move(pkey);
  return true;
}

// SetupContext configures |ctx| based on attributes in |t|, with the exception
// of the signing digest which must be configured externally.
static bool SetupContext(FileTest *t, KeyMap *key_map, EVP_PKEY_CTX *ctx) {
  if (t->HasAttribute("RSAPadding")) {
    int padding;
    if (!GetRSAPadding(t, &padding, t->GetAttributeOrDie("RSAPadding")) ||
        !EVP_PKEY_CTX_set_rsa_padding(ctx, padding)) {
      return false;
    }
  }
  if (t->HasAttribute("PSSSaltLength") &&
      !EVP_PKEY_CTX_set_rsa_pss_saltlen(
          ctx, atoi(t->GetAttributeOrDie("PSSSaltLength").c_str()))) {
    return false;
  }
  if (t->HasAttribute("MGF1Digest")) {
    const EVP_MD *digest = GetDigest(t->GetAttributeOrDie("MGF1Digest"));
    if (digest == nullptr || !EVP_PKEY_CTX_set_rsa_mgf1_md(ctx, digest)) {
      return false;
    }
  }
  if (t->HasAttribute("OAEPDigest")) {
    const EVP_MD *digest = GetDigest(t->GetAttributeOrDie("OAEPDigest"));
    if (digest == nullptr || !EVP_PKEY_CTX_set_rsa_oaep_md(ctx, digest)) {
      return false;
    }
  }
  if (t->HasAttribute("OAEPLabel")) {
    std::vector<uint8_t> label;
    if (!t->GetBytes(&label, "OAEPLabel")) {
      return false;
    }
    // For historical reasons, |EVP_PKEY_CTX_set0_rsa_oaep_label| expects to be
    // take ownership of the input.
    bssl::UniquePtr<uint8_t> buf(reinterpret_cast<uint8_t *>(
        OPENSSL_memdup(label.data(), label.size())));
    if (!buf ||
        !EVP_PKEY_CTX_set0_rsa_oaep_label(ctx, buf.get(), label.size())) {
      return false;
    }
    buf.release();
  }
  if (t->HasAttribute("DerivePeer")) {
    std::string derive_peer = t->GetAttributeOrDie("DerivePeer");
    if (key_map->count(derive_peer) == 0) {
      ADD_FAILURE() << "Could not find key " << derive_peer;
      return false;
    }
    EVP_PKEY *derive_peer_key = (*key_map)[derive_peer].get();
    if (!EVP_PKEY_derive_set_peer(ctx, derive_peer_key)) {
      return false;
    }
  }
  if (t->HasAttribute("DiffieHellmanPad") && !EVP_PKEY_CTX_set_dh_pad(ctx, 1)) {
    return false;
  }
  return true;
}

static bool TestDerive(FileTest *t, KeyMap *key_map, EVP_PKEY *key) {
  bssl::UniquePtr<EVP_PKEY_CTX> ctx(EVP_PKEY_CTX_new(key, nullptr));
  if (!ctx ||
      !EVP_PKEY_derive_init(ctx.get()) ||
      !SetupContext(t, key_map, ctx.get())) {
    return false;
  }

  bssl::UniquePtr<EVP_PKEY_CTX> copy(EVP_PKEY_CTX_dup(ctx.get()));
  if (!copy) {
    return false;
  }

  for (EVP_PKEY_CTX *pctx : {ctx.get(), copy.get()}) {
    size_t len;
    std::vector<uint8_t> actual, output;
    if (!EVP_PKEY_derive(pctx, nullptr, &len)) {
      return false;
    }
    actual.resize(len);
    if (!EVP_PKEY_derive(pctx, actual.data(), &len)) {
      return false;
    }
    actual.resize(len);

    // Defer looking up the attribute so Error works properly.
    if (!t->GetBytes(&output, "Output")) {
      return false;
    }
    EXPECT_EQ(Bytes(output), Bytes(actual));

    // Test when the buffer is too large.
    actual.resize(len + 1);
    len = actual.size();
    if (!EVP_PKEY_derive(pctx, actual.data(), &len)) {
      return false;
    }
    actual.resize(len);
    EXPECT_EQ(Bytes(output), Bytes(actual));

    // Test when the buffer is too small.
    actual.resize(len - 1);
    len = actual.size();
    if (t->HasAttribute("SmallBufferTruncates")) {
      if (!EVP_PKEY_derive(pctx, actual.data(), &len)) {
        return false;
      }
      actual.resize(len);
      EXPECT_EQ(Bytes(output.data(), len), Bytes(actual));
    } else {
      EXPECT_FALSE(EVP_PKEY_derive(pctx, actual.data(), &len));
      ERR_clear_error();
    }
  }
  return true;
}

static int EVP_marshal_private_key_version_one(CBB *cbb, const EVP_PKEY *key) {
  return EVP_marshal_private_key(cbb, key);
}

static int EVP_marshal_private_key_version_two(CBB *cbb, const EVP_PKEY *key) {
  return EVP_marshal_private_key_v2(cbb, key);
}

static void VerifyEVPSignOut(std::string key_name, std::vector<uint8_t> input,
                            std::vector<uint8_t> actual, std::vector<uint8_t> output,
                            EVP_MD_CTX *ctx, size_t len) {

  // Unless not compatible, verify EVP_DigestSign() with EVP_DigestVerify instead of comparing outputs
  // This allows us to test the correctness of non-deterministic outputs (e.g. for ECDSA).
  if (key_name.find("Ed25519") != std::string::npos) {
    EXPECT_EQ(Bytes(output), Bytes(actual));
  } else {
    EXPECT_TRUE(!EVP_DigestVerify(ctx, actual.data(), len, input.data(), input.size()));
  }
}

static bool TestEVP(FileTest *t, KeyMap *key_map) {
  if (t->GetType() == "PrivateKey") {
    int (*marshal_func)(CBB * cbb, const EVP_PKEY *key) =
        EVP_marshal_private_key;
    std::string version;
    if (t->HasAttribute("PKCS8VersionOut") && t->GetAttribute(&version, "PKCS8VersionOut")) {
      if (version == "1") {
        marshal_func = EVP_marshal_private_key_version_one;
      } else if (version == "2") {
        marshal_func = EVP_marshal_private_key_version_two;
      } else {
        return false;
      }
    }
    return ImportKey(t, key_map, KeyRole::kPrivate, marshal_func);
  }

  if (t->GetType() == "PublicKey") {
    return ImportKey(t, key_map, KeyRole::kPublic, EVP_marshal_public_key);
  }

  if (t->GetType() == "DHKey") {
    return ImportDHKey(t, key_map);
  }

  // Load the key.
  const std::string &key_name = t->GetParameter();
  if (key_map->count(key_name) == 0) {
    ADD_FAILURE() << "Could not find key " << key_name;
    return false;
  }
  EVP_PKEY *key = (*key_map)[key_name].get();

  int (*key_op_init)(EVP_PKEY_CTX *ctx) = nullptr;
  int (*key_op)(EVP_PKEY_CTX *ctx, uint8_t *out, size_t *out_len,
                const uint8_t *in, size_t in_len) = nullptr;
  int (*md_op_init)(EVP_MD_CTX * ctx, EVP_PKEY_CTX * *pctx, const EVP_MD *type,
                    ENGINE *e, EVP_PKEY *pkey) = nullptr;
  bool is_verify = false;
  if (t->GetType() == "Decrypt") {
    key_op_init = EVP_PKEY_decrypt_init;
    key_op = EVP_PKEY_decrypt;
  } else if (t->GetType() == "Sign") {
    key_op_init = EVP_PKEY_sign_init;
    key_op = EVP_PKEY_sign;
  } else if (t->GetType() == "Verify") {
    key_op_init = EVP_PKEY_verify_init;
    is_verify = true;
  } else if (t->GetType() == "SignMessage") {
    md_op_init = EVP_DigestSignInit;
  } else if (t->GetType() == "VerifyMessage") {
    md_op_init = EVP_DigestVerifyInit;
    is_verify = true;
  } else if (t->GetType() == "Encrypt") {
    key_op_init = EVP_PKEY_encrypt_init;
    key_op = EVP_PKEY_encrypt;
  } else if (t->GetType() == "Derive") {
    return TestDerive(t, key_map, key);
  } else {
    ADD_FAILURE() << "Unknown test " << t->GetType();
    return false;
  }

  const EVP_MD *digest = nullptr;
  if (t->HasAttribute("Digest")) {
    digest = GetDigest(t->GetAttributeOrDie("Digest"));
    if (digest == nullptr) {
      return false;
    }
  }

  // For verify tests, the "output" is the signature. Read it now so that, for
  // tests which expect a failure in SetupContext, the attribute is still
  // consumed.
  std::vector<uint8_t> input, actual, output;
  if (!t->GetBytes(&input, "Input") ||
      (is_verify && !t->GetBytes(&output, "Output"))) {
    return false;
  }

  if (md_op_init) {
    bssl::ScopedEVP_MD_CTX ctx, copy;
    EVP_PKEY_CTX *pctx;
    if (!md_op_init(ctx.get(), &pctx, digest, nullptr, key) ||
        !SetupContext(t, key_map, pctx) ||
        !EVP_MD_CTX_copy_ex(copy.get(), ctx.get())) {
      return false;
    }

    if (is_verify) {
      return EVP_DigestVerify(ctx.get(), output.data(), output.size(),
                              input.data(), input.size()) &&
             EVP_DigestVerify(copy.get(), output.data(), output.size(),
                              input.data(), input.size());
    }

    size_t len;
    if (!EVP_DigestSign(ctx.get(), nullptr, &len, input.data(), input.size())) {
      return false;
    }
    actual.resize(len);
    if (!EVP_DigestSign(ctx.get(), actual.data(), &len, input.data(),
                        input.size()) ||
        !t->GetBytes(&output, "Output")) {
      return false;
    }
    actual.resize(len);
    VerifyEVPSignOut(key_name, input, actual, output, ctx.get(), len);

    // Repeat the test with |copy|, to check |EVP_MD_CTX_copy_ex| duplicated
    // everything.
    if (!EVP_DigestSign(copy.get(), nullptr, &len, input.data(),
                        input.size())) {
      return false;
    }
    actual.resize(len);
    if (!EVP_DigestSign(copy.get(), actual.data(), &len, input.data(),
                        input.size()) ||
        !t->GetBytes(&output, "Output")) {
      return false;
    }
    actual.resize(len);
    VerifyEVPSignOut(key_name, std::move(input), std::move(actual),
      std::move(output), ctx.get(), len);
    return true;
  }

  bssl::UniquePtr<EVP_PKEY_CTX> ctx(EVP_PKEY_CTX_new(key, nullptr));
  if (!ctx ||
      !key_op_init(ctx.get()) ||
      (digest != nullptr &&
       !EVP_PKEY_CTX_set_signature_md(ctx.get(), digest)) ||
      !SetupContext(t, key_map, ctx.get())) {
    return false;
  }

  bssl::UniquePtr<EVP_PKEY_CTX> copy(EVP_PKEY_CTX_dup(ctx.get()));
  if (!copy) {
    return false;
  }

  if (is_verify) {
    return EVP_PKEY_verify(ctx.get(), output.data(), output.size(),
                           input.data(), input.size()) &&
           EVP_PKEY_verify(copy.get(), output.data(), output.size(),
                           input.data(), input.size());
  }

  for (EVP_PKEY_CTX *pctx : {ctx.get(), copy.get()}) {
    size_t len;
    if (!key_op(pctx, nullptr, &len, input.data(), input.size())) {
      return false;
    }
    actual.resize(len);
    if (!key_op(pctx, actual.data(), &len, input.data(), input.size())) {
      return false;
    }

    if (t->HasAttribute("CheckDecrypt")) {
      // Encryption is non-deterministic, so we check by decrypting.
      size_t plaintext_len;
      bssl::UniquePtr<EVP_PKEY_CTX> decrypt_ctx(EVP_PKEY_CTX_new(key, nullptr));
      if (!decrypt_ctx ||
          !EVP_PKEY_decrypt_init(decrypt_ctx.get()) ||
          (digest != nullptr &&
           !EVP_PKEY_CTX_set_signature_md(decrypt_ctx.get(), digest)) ||
          !SetupContext(t, key_map, decrypt_ctx.get()) ||
          !EVP_PKEY_decrypt(decrypt_ctx.get(), nullptr, &plaintext_len,
                            actual.data(), actual.size())) {
        return false;
      }
      output.resize(plaintext_len);
      if (!EVP_PKEY_decrypt(decrypt_ctx.get(), output.data(), &plaintext_len,
                            actual.data(), actual.size())) {
        ADD_FAILURE() << "Could not decrypt result.";
        return false;
      }
      output.resize(plaintext_len);
      EXPECT_EQ(Bytes(input), Bytes(output)) << "Decrypted result mismatch.";
    } else if (t->HasAttribute("CheckVerify")) {
      // Some signature schemes are non-deterministic, so we check by verifying.
      bssl::UniquePtr<EVP_PKEY_CTX> verify_ctx(EVP_PKEY_CTX_new(key, nullptr));
      if (!verify_ctx ||
          !EVP_PKEY_verify_init(verify_ctx.get()) ||
          (digest != nullptr &&
           !EVP_PKEY_CTX_set_signature_md(verify_ctx.get(), digest)) ||
          !SetupContext(t, key_map, verify_ctx.get())) {
        return false;
      }
      if (t->HasAttribute("VerifyPSSSaltLength")) {
        if (!EVP_PKEY_CTX_set_rsa_pss_saltlen(
                verify_ctx.get(),
                atoi(t->GetAttributeOrDie("VerifyPSSSaltLength").c_str()))) {
          return false;
        }
      }
      EXPECT_TRUE(EVP_PKEY_verify(verify_ctx.get(), actual.data(),
                                  actual.size(), input.data(), input.size()))
          << "Could not verify result.";
    } else {
      // By default, check by comparing the result against Output.
      if (!t->GetBytes(&output, "Output")) {
        return false;
      }
      actual.resize(len);
      EXPECT_EQ(Bytes(output), Bytes(actual));
    }
  }
  return true;
}

TEST(EVPTest, TestVectors) {
  KeyMap key_map;
  FileTestGTest("crypto/evp_extra/evp_tests.txt", [&](FileTest *t) {
    bool result = TestEVP(t, &key_map);
    if (t->HasAttribute("Error")) {
      ASSERT_FALSE(result) << "Operation unexpectedly succeeded.";
      uint32_t err = ERR_peek_error();
      EXPECT_EQ(t->GetAttributeOrDie("Error"), ERR_reason_error_string(err));
    } else if (!result) {
      ADD_FAILURE() << "Operation unexpectedly failed.";
    }
  });
}

static void RunWycheproofVerifyTest(const char *path) {
  SCOPED_TRACE(path);
  FileTestGTest(path, [](FileTest *t) {
    t->IgnoreAllUnusedInstructions();

    std::vector<uint8_t> der;
    // Try publicKeyDer first (Wycheproof v1), fall back to keyDer (Wycheproof v0)
    if (t->HasInstruction("publicKeyDer")) {
      ASSERT_TRUE(t->GetInstructionBytes(&der, "publicKeyDer"));
    } else {
      ASSERT_TRUE(t->GetInstructionBytes(&der, "keyDer"));
    }
    CBS cbs;
    CBS_init(&cbs, der.data(), der.size());
    bssl::UniquePtr<EVP_PKEY> key(EVP_parse_public_key(&cbs));
    ASSERT_TRUE(key);

    const EVP_MD *md = nullptr;
    if (t->HasInstruction("sha")) {
      md = GetWycheproofDigest(t, "sha", true);
      ASSERT_TRUE(md);
    }

    bool is_pss = t->HasInstruction("mgf");
    const EVP_MD *mgf1_md = nullptr;
    int pss_salt_len = -1;
    if (is_pss) {
      ASSERT_EQ("MGF1", t->GetInstructionOrDie("mgf"));
      mgf1_md = GetWycheproofDigest(t, "mgfSha", true);

      std::string s_len;
      ASSERT_TRUE(t->GetInstruction(&s_len, "sLen"));
      pss_salt_len = atoi(s_len.c_str());
    }

    std::vector<uint8_t> msg;
    ASSERT_TRUE(t->GetBytes(&msg, "msg"));
    std::vector<uint8_t> sig;
    ASSERT_TRUE(t->GetBytes(&sig, "sig"));
    WycheproofResult result;
    ASSERT_TRUE(GetWycheproofResult(t, &result));

    if (EVP_PKEY_id(key.get()) == EVP_PKEY_DSA) {
      // DSA is deprecated and is not usable via EVP.
      DSA *dsa = EVP_PKEY_get0_DSA(key.get());
      OPENSSL_BEGIN_ALLOW_DEPRECATED
      ASSERT_EQ(dsa, EVP_PKEY_get0(key.get()));
      OPENSSL_END_ALLOW_DEPRECATED
      uint8_t digest[EVP_MAX_MD_SIZE];
      unsigned digest_len;
      ASSERT_TRUE(
          EVP_Digest(msg.data(), msg.size(), digest, &digest_len, md, nullptr));
      int valid;
      bool sig_ok = DSA_check_signature(&valid, digest, digest_len, sig.data(),
                                        sig.size(), dsa) &&
                    valid;
      EXPECT_EQ(sig_ok, result.IsValid());
    } else {
      bssl::ScopedEVP_MD_CTX ctx;
      EVP_PKEY_CTX *pctx;
      ASSERT_TRUE(
          EVP_DigestVerifyInit(ctx.get(), &pctx, md, nullptr, key.get()));
      if (is_pss) {
        ASSERT_TRUE(EVP_PKEY_CTX_set_rsa_padding(pctx, RSA_PKCS1_PSS_PADDING));
        ASSERT_TRUE(EVP_PKEY_CTX_set_rsa_mgf1_md(pctx, mgf1_md));
        ASSERT_TRUE(EVP_PKEY_CTX_set_rsa_pss_saltlen(pctx, pss_salt_len));
      }
      int ret = EVP_DigestVerify(ctx.get(), sig.data(), sig.size(), msg.data(),
                                 msg.size());
      // BoringSSL does not enforce policies on weak keys and leaves it to the
      // caller.
      EXPECT_EQ(ret,
                result.IsValid({"SmallModulus", "SmallPublicKey", "WeakHash"})
                    ? 1
                    : 0);
    }
  });
}

//= third_party/vectors/vectors_spec.md#wycheproof
//# AWS-LC MUST test against `testvectors_v1/dsa_2048_224_sha224_test.txt`.
//# AWS-LC MUST test against `testvectors_v1/dsa_2048_224_sha256_test.txt`.
//# AWS-LC MUST test against `testvectors_v1/dsa_2048_256_sha256_test.txt`.
//# AWS-LC MUST test against `testvectors_v1/dsa_3072_256_sha256_test.txt`.
TEST(EVPTest, WycheproofDSA) {
  RunWycheproofVerifyTest(
      "third_party/vectors/converted/wycheproof/testvectors_v1/dsa_2048_224_sha224_test.txt");
  RunWycheproofVerifyTest(
      "third_party/vectors/converted/wycheproof/testvectors_v1/dsa_2048_224_sha256_test.txt");
  RunWycheproofVerifyTest(
      "third_party/vectors/converted/wycheproof/testvectors_v1/dsa_2048_256_sha256_test.txt");
  RunWycheproofVerifyTest(
      "third_party/vectors/converted/wycheproof/testvectors_v1/dsa_3072_256_sha256_test.txt");
}

//= third_party/vectors/vectors_spec.md#wycheproof
//# AWS-LC MUST test against `testvectors_v1/ecdsa_secp224r1_sha224_test.txt`.
//# AWS-LC MUST test against `testvectors_v1/ecdsa_secp224r1_sha256_test.txt`.
//# AWS-LC MUST test against `testvectors_v1/ecdsa_secp224r1_sha512_test.txt`.
TEST(EVPTest, WycheproofECDSAP224) {
  RunWycheproofVerifyTest(
      "third_party/vectors/converted/wycheproof/testvectors_v1/ecdsa_secp224r1_sha224_test.txt");
  RunWycheproofVerifyTest(
      "third_party/vectors/converted/wycheproof/testvectors_v1/ecdsa_secp224r1_sha256_test.txt");
  RunWycheproofVerifyTest(
      "third_party/vectors/converted/wycheproof/testvectors_v1/ecdsa_secp224r1_sha512_test.txt");
}

//= third_party/vectors/vectors_spec.md#wycheproof
//# AWS-LC MUST test against `testvectors_v1/ecdsa_secp256r1_sha256_test.txt`.
//# AWS-LC MUST test against `testvectors_v1/ecdsa_secp256r1_sha512_test.txt`.
TEST(EVPTest, WycheproofECDSAP256) {
  RunWycheproofVerifyTest(
      "third_party/vectors/converted/wycheproof/testvectors_v1/ecdsa_secp256r1_sha256_test.txt");
  RunWycheproofVerifyTest(
      "third_party/vectors/converted/wycheproof/testvectors_v1/ecdsa_secp256r1_sha512_test.txt");
}

//= third_party/vectors/vectors_spec.md#wycheproof
//# AWS-LC MUST test against `testvectors_v1/ecdsa_secp384r1_sha384_test.txt`.
//# AWS-LC MUST test against `testvectors_v1/ecdsa_secp384r1_sha512_test.txt`.
TEST(EVPTest, WycheproofECDSAP384) {
  RunWycheproofVerifyTest(
      "third_party/vectors/converted/wycheproof/testvectors_v1/ecdsa_secp384r1_sha384_test.txt");
  RunWycheproofVerifyTest(
      "third_party/vectors/converted/wycheproof/testvectors_v1/ecdsa_secp384r1_sha512_test.txt");
}

//= third_party/vectors/vectors_spec.md#wycheproof
//# AWS-LC MUST test against `testvectors_v1/ecdsa_secp521r1_sha512_test.txt`.
TEST(EVPTest, WycheproofECDSAP521) {
  RunWycheproofVerifyTest(
      "third_party/vectors/converted/wycheproof/testvectors_v1/ecdsa_secp521r1_sha512_test.txt");
}

//= third_party/vectors/vectors_spec.md#wycheproof
//# AWS-LC MUST test against `testvectors_v1/ecdsa_secp256k1_sha256_test.txt`.
//# AWS-LC MUST test against `testvectors_v1/ecdsa_secp256k1_sha512_test.txt`.
TEST(EVPTest, WycheproofECDSAsecp256k1) {
  RunWycheproofVerifyTest(
      "third_party/vectors/converted/wycheproof/testvectors_v1/ecdsa_secp256k1_sha256_test.txt");
  RunWycheproofVerifyTest(
      "third_party/vectors/converted/wycheproof/testvectors_v1/ecdsa_secp256k1_sha512_test.txt");
}

//= third_party/vectors/vectors_spec.md#wycheproof
//# AWS-LC MUST test against `testvectors_v1/ed25519_test.txt`.
TEST(EVPTest, WycheproofEdDSA) {
  RunWycheproofVerifyTest(
      "third_party/vectors/converted/wycheproof/testvectors_v1/ed25519_test.txt");
}

//= third_party/vectors/vectors_spec.md#wycheproof
//# AWS-LC MUST test against `testvectors_v1/rsa_signature_2048_sha224_test.txt`.
//# AWS-LC MUST test against `testvectors_v1/rsa_signature_2048_sha256_test.txt`.
//# AWS-LC MUST test against `testvectors_v1/rsa_signature_2048_sha384_test.txt`.
//# AWS-LC MUST test against `testvectors_v1/rsa_signature_2048_sha512_test.txt`.
//# AWS-LC MUST test against `testvectors_v1/rsa_signature_3072_sha256_test.txt`.
//# AWS-LC MUST test against `testvectors_v1/rsa_signature_3072_sha384_test.txt`.
//# AWS-LC MUST test against `testvectors_v1/rsa_signature_3072_sha512_test.txt`.
//# AWS-LC MUST test against `testvectors_v1/rsa_signature_4096_sha384_test.txt`.
//# AWS-LC MUST test against `testvectors_v1/rsa_signature_4096_sha512_test.txt`.
//# AWS-LC MUST test against `testvectors_v1/rsa_signature_8192_sha256_test.txt`.
//# AWS-LC MUST test against `testvectors_v1/rsa_signature_8192_sha384_test.txt`.
//# AWS-LC MUST test against `testvectors_v1/rsa_signature_8192_sha512_test.txt`.
TEST(EVPTest, WycheproofRSAPKCS1) {
  RunWycheproofVerifyTest(
      "third_party/vectors/converted/wycheproof/testvectors_v1/rsa_signature_2048_sha224_test.txt");
  RunWycheproofVerifyTest(
      "third_party/vectors/converted/wycheproof/testvectors_v1/rsa_signature_2048_sha256_test.txt");
  RunWycheproofVerifyTest(
      "third_party/vectors/converted/wycheproof/testvectors_v1/rsa_signature_2048_sha384_test.txt");
  RunWycheproofVerifyTest(
      "third_party/vectors/converted/wycheproof/testvectors_v1/rsa_signature_2048_sha512_test.txt");
  RunWycheproofVerifyTest(
      "third_party/vectors/converted/wycheproof/testvectors_v1/rsa_signature_3072_sha256_test.txt");
  RunWycheproofVerifyTest(
      "third_party/vectors/converted/wycheproof/testvectors_v1/rsa_signature_3072_sha384_test.txt");
  RunWycheproofVerifyTest(
      "third_party/vectors/converted/wycheproof/testvectors_v1/rsa_signature_3072_sha512_test.txt");
  RunWycheproofVerifyTest(
      "third_party/vectors/converted/wycheproof/testvectors_v1/rsa_signature_4096_sha384_test.txt");
  RunWycheproofVerifyTest(
      "third_party/vectors/converted/wycheproof/testvectors_v1/rsa_signature_4096_sha512_test.txt");
  RunWycheproofVerifyTest(
      "third_party/vectors/converted/wycheproof/testvectors_v1/rsa_signature_8192_sha256_test.txt");
  RunWycheproofVerifyTest(
      "third_party/vectors/converted/wycheproof/testvectors_v1/rsa_signature_8192_sha384_test.txt");
  RunWycheproofVerifyTest(
      "third_party/vectors/converted/wycheproof/testvectors_v1/rsa_signature_8192_sha512_test.txt");
  // Note: rsa_signature_test.txt (377 tests) is not available in the new
  // upstream. The specific test files above provide comprehensive coverage
  // (2169 tests total across all key sizes and hash functions).
}

//= third_party/vectors/vectors_spec.md#wycheproof
//# AWS-LC MUST test against `testvectors_v1/rsa_pkcs1_1024_sig_gen_test.txt`.
//# AWS-LC MUST test against `testvectors_v1/rsa_pkcs1_1536_sig_gen_test.txt`.
//# AWS-LC MUST test against `testvectors_v1/rsa_pkcs1_2048_sig_gen_test.txt`.
//# AWS-LC MUST test against `testvectors_v1/rsa_pkcs1_3072_sig_gen_test.txt`.
//# AWS-LC MUST test against `testvectors_v1/rsa_pkcs1_4096_sig_gen_test.txt`.
static void RunWycheproofRSAPKCS1SignTest(const char *path) {
  FileTestGTest(path,
      [](FileTest *t) {
        t->IgnoreAllUnusedInstructions();

        std::vector<uint8_t> pkcs8;
        ASSERT_TRUE(t->GetInstructionBytes(&pkcs8, "privateKeyPkcs8"));
        CBS cbs;
        CBS_init(&cbs, pkcs8.data(), pkcs8.size());
        bssl::UniquePtr<EVP_PKEY> key(EVP_parse_private_key(&cbs));
        ASSERT_TRUE(key);

        const EVP_MD *md = GetWycheproofDigest(t, "sha", true);
        ASSERT_TRUE(md);

        std::vector<uint8_t> msg, sig;
        ASSERT_TRUE(t->GetBytes(&msg, "msg"));
        ASSERT_TRUE(t->GetBytes(&sig, "sig"));
        WycheproofResult result;
        ASSERT_TRUE(GetWycheproofResult(t, &result));

        bssl::ScopedEVP_MD_CTX ctx;
        EVP_PKEY_CTX *pctx;
        ASSERT_TRUE(
            EVP_DigestSignInit(ctx.get(), &pctx, md, nullptr, key.get()));
        std::vector<uint8_t> out(EVP_PKEY_size(key.get()));
        size_t len = out.size();
        int ret =
            EVP_DigestSign(ctx.get(), out.data(), &len, msg.data(), msg.size());
        // BoringSSL does not enforce policies on weak keys and leaves it to the
        // caller.
        bool is_valid =
            result.IsValid({"SmallModulus", "SmallPublicKey", "WeakHash"});
        EXPECT_EQ(ret, is_valid ? 1 : 0);
        if (is_valid) {
          out.resize(len);
          EXPECT_EQ(Bytes(sig), Bytes(out));
        }
      });
}

TEST(EVPTest, WycheproofRSAPKCS1Sign) {
  RunWycheproofRSAPKCS1SignTest(
      "third_party/vectors/converted/wycheproof/testvectors_v1/rsa_pkcs1_1024_sig_gen_test.txt");
  RunWycheproofRSAPKCS1SignTest(
      "third_party/vectors/converted/wycheproof/testvectors_v1/rsa_pkcs1_1536_sig_gen_test.txt");
  RunWycheproofRSAPKCS1SignTest(
      "third_party/vectors/converted/wycheproof/testvectors_v1/rsa_pkcs1_2048_sig_gen_test.txt");
  RunWycheproofRSAPKCS1SignTest(
      "third_party/vectors/converted/wycheproof/testvectors_v1/rsa_pkcs1_3072_sig_gen_test.txt");
  RunWycheproofRSAPKCS1SignTest(
      "third_party/vectors/converted/wycheproof/testvectors_v1/rsa_pkcs1_4096_sig_gen_test.txt");
  // Note: rsa_sig_gen_misc_test.txt (158 tests with 1024-4096 bit keys) is
  // not available in the new upstream. The new test files above provide
  // equivalent coverage split by key size.
}

TEST(EVPTest, WycheproofRSAPSS) {
  RunWycheproofVerifyTest(
      "third_party/wycheproof_testvectors/rsa_pss_2048_sha1_mgf1_20_test.txt");
  RunWycheproofVerifyTest(
      "third_party/wycheproof_testvectors/rsa_pss_2048_sha256_mgf1_0_test.txt");
  RunWycheproofVerifyTest(
      "third_party/wycheproof_testvectors/"
      "rsa_pss_2048_sha256_mgf1_32_test.txt");
  RunWycheproofVerifyTest(
      "third_party/wycheproof_testvectors/"
      "rsa_pss_3072_sha256_mgf1_32_test.txt");
  RunWycheproofVerifyTest(
      "third_party/wycheproof_testvectors/"
      "rsa_pss_4096_sha256_mgf1_32_test.txt");
  RunWycheproofVerifyTest(
      "third_party/wycheproof_testvectors/"
      "rsa_pss_4096_sha512_mgf1_32_test.txt");
  RunWycheproofVerifyTest(
      "third_party/wycheproof_testvectors/rsa_pss_misc_test.txt");
}

static void RunWycheproofDecryptTest(
    const char *path,
    std::function<void(FileTest *, EVP_PKEY_CTX *)> setup_cb) {
  FileTestGTest(path, [&](FileTest *t) {
    t->IgnoreAllUnusedInstructions();

    std::vector<uint8_t> pkcs8;
    ASSERT_TRUE(t->GetInstructionBytes(&pkcs8, "privateKeyPkcs8"));
    CBS cbs;
    CBS_init(&cbs, pkcs8.data(), pkcs8.size());
    bssl::UniquePtr<EVP_PKEY> key(EVP_parse_private_key(&cbs));
    ASSERT_TRUE(key);

    std::vector<uint8_t> ct, msg;
    ASSERT_TRUE(t->GetBytes(&ct, "ct"));
    ASSERT_TRUE(t->GetBytes(&msg, "msg"));
    WycheproofResult result;
    ASSERT_TRUE(GetWycheproofResult(t, &result));

    bssl::UniquePtr<EVP_PKEY_CTX> ctx(EVP_PKEY_CTX_new(key.get(), nullptr));
    ASSERT_TRUE(ctx);
    ASSERT_TRUE(EVP_PKEY_decrypt_init(ctx.get()));
    ASSERT_NO_FATAL_FAILURE(setup_cb(t, ctx.get()));
    std::vector<uint8_t> out(EVP_PKEY_size(key.get()));
    size_t len = out.size();
    int ret =
        EVP_PKEY_decrypt(ctx.get(), out.data(), &len, ct.data(), ct.size());
    // BoringSSL does not enforce policies on weak keys and leaves it to the
    // caller.
    bool is_valid = result.IsValid({"SmallModulus"});

    // AWS-LC enforces FIPS 800-56B Rev. 2 §7.1.2.1 which requires 1 < c < (n-1).
    // But Wycheproof mistakenly marks some vectors with c values outside this range as valid.
    if (is_valid) {
      const RSA *rsa = EVP_PKEY_get0_RSA(key.get());
      const BIGNUM *n = RSA_get0_n(rsa);
      bssl::UniquePtr<BIGNUM> c(BN_bin2bn(ct.data(), ct.size(), nullptr));
      bssl::UniquePtr<BIGNUM> n_minus_one(BN_dup(n));
      ASSERT_TRUE(c && n_minus_one);
      ASSERT_TRUE(BN_sub_word(n_minus_one.get(), 1));
      if (BN_is_zero(c.get()) || BN_is_one(c.get()) ||
          BN_cmp(c.get(), n_minus_one.get()) >= 0) {
        is_valid = false;
      }
    }

    EXPECT_EQ(ret, is_valid ? 1 : 0);
    if (is_valid) {
      out.resize(len);
      EXPECT_EQ(Bytes(msg), Bytes(out));
    }
  });
}

static void RunWycheproofOAEPTest(const char *path) {
  RunWycheproofDecryptTest(path, [](FileTest *t, EVP_PKEY_CTX *ctx) {
    const EVP_MD *md = GetWycheproofDigest(t, "sha", true);
    ASSERT_TRUE(md);
    const EVP_MD *mgf1_md = GetWycheproofDigest(t, "mgfSha", true);
    ASSERT_TRUE(mgf1_md);
    std::vector<uint8_t> label;
    ASSERT_TRUE(t->GetBytes(&label, "label"));

    ASSERT_TRUE(EVP_PKEY_CTX_set_rsa_padding(ctx, RSA_PKCS1_OAEP_PADDING));
    ASSERT_TRUE(EVP_PKEY_CTX_set_rsa_oaep_md(ctx, md));
    ASSERT_TRUE(EVP_PKEY_CTX_set_rsa_mgf1_md(ctx, mgf1_md));
    bssl::UniquePtr<uint8_t> label_copy(
        static_cast<uint8_t *>(OPENSSL_memdup(label.data(), label.size())));
    ASSERT_TRUE(label_copy || label.empty());
    ASSERT_TRUE(
        EVP_PKEY_CTX_set0_rsa_oaep_label(ctx, label_copy.get(), label.size()));
    // |EVP_PKEY_CTX_set0_rsa_oaep_label| takes ownership on success.
    label_copy.release();
  });
}

TEST(EVPTest, WycheproofRSAOAEP2048) {
  RunWycheproofOAEPTest(
      "third_party/wycheproof_testvectors/"
      "rsa_oaep_2048_sha1_mgf1sha1_test.txt");
  RunWycheproofOAEPTest(
      "third_party/wycheproof_testvectors/"
      "rsa_oaep_2048_sha224_mgf1sha1_test.txt");
  RunWycheproofOAEPTest(
      "third_party/wycheproof_testvectors/"
      "rsa_oaep_2048_sha224_mgf1sha224_test.txt");
  RunWycheproofOAEPTest(
      "third_party/wycheproof_testvectors/"
      "rsa_oaep_2048_sha256_mgf1sha1_test.txt");
  RunWycheproofOAEPTest(
      "third_party/wycheproof_testvectors/"
      "rsa_oaep_2048_sha256_mgf1sha256_test.txt");
  RunWycheproofOAEPTest(
      "third_party/wycheproof_testvectors/"
      "rsa_oaep_2048_sha384_mgf1sha1_test.txt");
  RunWycheproofOAEPTest(
      "third_party/wycheproof_testvectors/"
      "rsa_oaep_2048_sha384_mgf1sha384_test.txt");
  RunWycheproofOAEPTest(
      "third_party/wycheproof_testvectors/"
      "rsa_oaep_2048_sha512_mgf1sha1_test.txt");
  RunWycheproofOAEPTest(
      "third_party/wycheproof_testvectors/"
      "rsa_oaep_2048_sha512_mgf1sha512_test.txt");
}

TEST(EVPTest, WycheproofRSAOAEP3072) {
  RunWycheproofOAEPTest(
      "third_party/wycheproof_testvectors/"
      "rsa_oaep_3072_sha256_mgf1sha1_test.txt");
  RunWycheproofOAEPTest(
      "third_party/wycheproof_testvectors/"
      "rsa_oaep_3072_sha256_mgf1sha256_test.txt");
  RunWycheproofOAEPTest(
      "third_party/wycheproof_testvectors/"
      "rsa_oaep_3072_sha512_mgf1sha1_test.txt");
  RunWycheproofOAEPTest(
      "third_party/wycheproof_testvectors/"
      "rsa_oaep_3072_sha512_mgf1sha512_test.txt");
}

TEST(EVPTest, WycheproofRSAOAEP4096) {
  RunWycheproofOAEPTest(
      "third_party/wycheproof_testvectors/"
      "rsa_oaep_4096_sha256_mgf1sha1_test.txt");
  RunWycheproofOAEPTest(
      "third_party/wycheproof_testvectors/"
      "rsa_oaep_4096_sha256_mgf1sha256_test.txt");
  RunWycheproofOAEPTest(
      "third_party/wycheproof_testvectors/"
      "rsa_oaep_4096_sha512_mgf1sha1_test.txt");
  RunWycheproofOAEPTest(
      "third_party/wycheproof_testvectors/"
      "rsa_oaep_4096_sha512_mgf1sha512_test.txt");
}

TEST(EVPTest, WycheproofRSAOAEPMisc) {
  RunWycheproofOAEPTest(
      "third_party/wycheproof_testvectors/rsa_oaep_misc_test.txt");
}

static void RunWycheproofPKCS1DecryptTest(const char *path) {
  RunWycheproofDecryptTest(path, [](FileTest *t, EVP_PKEY_CTX *ctx) {
    // No setup needed. PKCS#1 is, sadly, the default.
  });
}

//= third_party/vectors/vectors_spec.md#wycheproof
//# AWS-LC MUST test against `testvectors_v1/rsa_pkcs1_2048_test.txt`.
//# AWS-LC MUST test against `testvectors_v1/rsa_pkcs1_3072_test.txt`.
//# AWS-LC MUST test against `testvectors_v1/rsa_pkcs1_4096_test.txt`.
TEST(EVPTest, WycheproofRSAPKCS1Decrypt) {
  RunWycheproofPKCS1DecryptTest(
      "third_party/vectors/converted/wycheproof/testvectors_v1/rsa_pkcs1_2048_test.txt");
  RunWycheproofPKCS1DecryptTest(
      "third_party/vectors/converted/wycheproof/testvectors_v1/rsa_pkcs1_3072_test.txt");
  RunWycheproofPKCS1DecryptTest(
      "third_party/vectors/converted/wycheproof/testvectors_v1/rsa_pkcs1_4096_test.txt");
}

struct ectlsencodedpoint_test_data {
    const uint8_t *public_key;
    size_t public_key_size;
    const uint8_t *private_key;
    size_t private_key_size;
    const uint8_t *expected_shared_secret;
    size_t expected_shared_secret_size;
    int key_type;
    int curve_nid;
};

static EVP_PKEY * instantiate_public_key(int key_type, int curve_nid) {

  EVP_PKEY *pkey = NULL;

  if (NID_X25519 == curve_nid) {
    pkey = EVP_PKEY_new();
    EXPECT_TRUE(pkey);
    EXPECT_TRUE(EVP_PKEY_set_type(pkey, key_type));
  }
  else {
    EC_KEY *ec_key = EC_KEY_new_by_curve_name(curve_nid);
    EXPECT_TRUE(ec_key);
    pkey = EVP_PKEY_new();
    EXPECT_TRUE(pkey);
    EXPECT_TRUE(EVP_PKEY_assign(pkey, EVP_PKEY_EC, (EC_KEY *) ec_key));
  }

  return pkey;
}

static EVP_PKEY * instantiate_and_set_public_key(const uint8_t *public_key,
  size_t public_key_size, int curve_nid) {

  EVP_PKEY *pkey = NULL;

  if (NID_X25519 != curve_nid) {
    EC_KEY *ec_key = EC_KEY_new_by_curve_name(curve_nid);
    EXPECT_TRUE(ec_key);
    const EC_GROUP *ec_key_group = EC_KEY_get0_group(ec_key);
    EXPECT_TRUE(ec_key_group);
    EC_POINT *ec_point = EC_POINT_new(ec_key_group);
    EXPECT_TRUE(ec_point);
    EXPECT_TRUE(EC_POINT_oct2point(ec_key_group, ec_point, public_key,
      public_key_size, NULL));
    EXPECT_TRUE(EC_KEY_set_public_key(ec_key, ec_point));
    pkey = EVP_PKEY_new();
    EXPECT_TRUE(pkey);
    EXPECT_TRUE(EVP_PKEY_assign(pkey, EVP_PKEY_EC, (EC_KEY *) ec_key));
    EC_POINT_free(ec_point);
  }

  return pkey;
}

static EVP_PKEY * instantiate_and_set_private_key(const uint8_t *private_key,
  size_t private_key_size, int key_type, int curve_nid) {

  EVP_PKEY *pkey = NULL;
  OPENSSL_BEGIN_ALLOW_DEPRECATED
  EXPECT_FALSE(EVP_PKEY_get0(pkey));
  OPENSSL_END_ALLOW_DEPRECATED

  if (NID_X25519 == curve_nid) {
    pkey = EVP_PKEY_new_raw_private_key(curve_nid, nullptr, private_key,
      private_key_size);
    EXPECT_TRUE(pkey);
  }
  else {
    EC_KEY *ec_key = EC_KEY_new_by_curve_name(curve_nid);
    EXPECT_TRUE(ec_key);
    BIGNUM *private_key_bn = BN_bin2bn(private_key, private_key_size, NULL);
    EXPECT_TRUE(private_key_bn);
    EXPECT_TRUE(EC_KEY_set_private_key(ec_key, private_key_bn));
    BN_free(private_key_bn);
    pkey = EVP_PKEY_new();
    EXPECT_TRUE(pkey);
    OPENSSL_BEGIN_ALLOW_DEPRECATED
    EXPECT_FALSE(EVP_PKEY_get0(pkey));
    EXPECT_TRUE(EVP_PKEY_assign(pkey, key_type, (EC_KEY *) ec_key));
    EXPECT_EQ(ec_key, EVP_PKEY_get0(pkey));
    OPENSSL_END_ALLOW_DEPRECATED
  }

  return pkey;
}

TEST(EVPTest, ECTLSEncodedPoint) {

    // TLS wire-encoding format
    // (https://tools.ietf.org/html/rfc8422#section-5.4)
    // x25519: u-coordinate
    // NIST curves: 0x04 || x-coordinate || y-coordinate

    // Taken from https://tools.ietf.org/html/rfc7748#section-5.2
    static const uint8_t kX25519PublicKey[] = {
      0xe6, 0xdb, 0x68, 0x67, 0x58, 0x30, 0x30, 0xdb, 0x35, 0x94, 0xc1, 0xa4,
      0x24, 0xb1, 0x5f, 0x7c, 0x72, 0x66, 0x24, 0xec, 0x26, 0xb3, 0x35, 0x3b,
      0x10, 0xa9, 0x03, 0xa6, 0xd0, 0xab, 0x1c, 0x4c
    };
    static const uint8_t kX25519PrivateKey[] = {
      0xa5, 0x46, 0xe3, 0x6b, 0xf0, 0x52, 0x7c, 0x9d, 0x3b, 0x16, 0x15, 0x4b,
      0x82, 0x46, 0x5e, 0xdd, 0x62, 0x14, 0x4c, 0x0a, 0xc1, 0xfc, 0x5a, 0x18,
      0x50, 0x6a, 0x22, 0x44, 0xba, 0x44, 0x9a, 0xc4
    };
    static const uint8_t kX25519ExpectedSharedSecret[] = {
      0xc3, 0xda, 0x55, 0x37, 0x9d, 0xe9, 0xc6, 0x90, 0x8e, 0x94, 0xea, 0x4d,
      0xf2, 0x8d, 0x08, 0x4f, 0x32, 0xec, 0xcf, 0x03, 0x49, 0x1c, 0x71, 0xf7,
      0x54, 0xb4, 0x07, 0x55, 0x77, 0xa2, 0x85, 0x52
    };

    struct ectlsencodedpoint_test_data x25519_test_data = {
      kX25519PublicKey, // public_key
      X25519_PUBLIC_VALUE_LEN, // public_key_size
      kX25519PrivateKey, // private_key
      X25519_PRIVATE_KEY_LEN, // private_key_size
      kX25519ExpectedSharedSecret, // expected_shared_secret
      X25519_SHARED_KEY_LEN, // expected_shared_secret_size
      EVP_PKEY_X25519, // key_type
      NID_X25519 // curve_nid
    };

    // P-{224,256,384,521} test vectors, taken from CAVP
    // (CAVP 20.1 - KASValidityTest_ECCStaticUnified_KDFConcat_NOKC)
    // https://csrc.nist.gov/projects/cryptographic-algorithm-validation-program/key-management

    static const uint8_t kP224PublicKey[] = {
      /* uncompressed */
      0x04,
      /* x-coordinate */
      0xd6, 0xf5, 0xf0, 0x6e, 0xf4, 0xc5, 0x56, 0x0a, 0xff, 0x8f, 0x49, 0x90,
      0xef, 0xdb, 0xa5, 0x9a, 0xf8, 0xa8, 0xd3, 0x77, 0x0d, 0x80, 0x14, 0x6a,
      0xc5, 0x82, 0x78, 0x85,
      /* y-coordinate */
      0xe0, 0x43, 0xae, 0x7b, 0xae, 0xa3, 0x77, 0x28, 0x60, 0x39, 0xc0, 0x7c,
      0x04, 0x1b, 0x7a, 0x3b, 0x5d, 0x76, 0x96, 0xda, 0xdd, 0xa7, 0x05, 0x1a,
      0xd6, 0x45, 0xa3, 0xea
    };
    static const uint8_t kP224PrivateKey[] = {
      0xc7, 0x39, 0x45, 0x68, 0x8b, 0x3d, 0xbb, 0xc6, 0xc2, 0xe7, 0x54, 0x75,
      0xdf, 0x61, 0xd1, 0x44, 0x9d, 0x05, 0xf9, 0x64, 0x49, 0x62, 0x92, 0x67,
      0xf2, 0x19, 0x5d, 0xaf
    };
    static const uint8_t kP224ExpectedSharedSecret[] = {
      0x50, 0x28, 0xd8, 0xa1, 0x62, 0xfe, 0xac, 0xbd, 0xfa, 0x5e, 0xca, 0x8c,
      0xdf, 0x50, 0xcc, 0xb9, 0xe0, 0x7c, 0x6b, 0x7f, 0x96, 0xa8, 0xa8, 0x93,
      0x24, 0xdd, 0xed, 0x7a
    };

    struct ectlsencodedpoint_test_data p224_test_data = {
      kP224PublicKey, // public_key
      (1 + 28 + 28), // public_key_size
      kP224PrivateKey, // private_key
      28, // private_key_size
      kP224ExpectedSharedSecret, // expected_shared_secret
      28, // expected_shared_secret_size
      EVP_PKEY_EC, // key_type
      NID_secp224r1 // curve_nid
    };

    static const uint8_t kP256PublicKey[] = {
      /* uncompressed */
      0x04,
      /* x-coordinate */
      0xe1, 0x5a, 0x44, 0x72, 0x91, 0xf0, 0x84, 0xfe, 0x88, 0x7a, 0x6c, 0x2c,
      0x03, 0x22, 0x9a, 0xf3, 0x04, 0x8a, 0x5d, 0xfe, 0x84, 0x73, 0x70, 0xc9,
      0x3f, 0x92, 0x72, 0x9b, 0x31, 0xc5, 0x5f, 0x7b,
      /* y-coordinate */
      0x36, 0xac, 0x98, 0x3e, 0x2d, 0x6f, 0xb9, 0x7a, 0x9e, 0x74, 0x09, 0x0d,
      0x26, 0xf4, 0x83, 0x34, 0xce, 0x4f, 0x4b, 0x74, 0x9f, 0x3f, 0xd7, 0xaa,
      0x92, 0xe2, 0xc5, 0x40, 0x23, 0x2c, 0xe1, 0xbd
    };
    static const uint8_t kP256PrivateKey[] = {
      0x4c, 0xab, 0xbc, 0x3f, 0xad, 0x44, 0x43, 0xcd, 0xa1, 0x36, 0x46, 0x39,
      0x1e, 0x08, 0xbd, 0xa9, 0xd5, 0x29, 0xe1, 0x03, 0x96, 0xc0, 0xcb, 0xd2,
      0xde, 0x9c, 0x1c, 0x73, 0xaf, 0xaa, 0x32, 0x99
    };
    static const uint8_t kP256ExpectedSharedSecret[] = {
      0x89, 0x00, 0x1b, 0x34, 0x36, 0xf7, 0xe6, 0x6b, 0x00, 0x8d, 0x68, 0xa6,
      0xc4, 0x7e, 0x01, 0x82, 0x49, 0x49, 0x4b, 0x92, 0x33, 0x92, 0x1b, 0x80,
      0x7a, 0x75, 0x49, 0xd3, 0xad, 0xe2, 0x01, 0xa2
    };

    struct ectlsencodedpoint_test_data p256_test_data = {
      kP256PublicKey, // public_key
      (1 + 32 + 32), // public_key_size
      kP256PrivateKey, // private_key
      32, // private_key_size
      kP256ExpectedSharedSecret, // expected_shared_secret
      32, // expected_shared_secret_size
      EVP_PKEY_EC, // key_type
      NID_X9_62_prime256v1 // curve_nid
    };

    static const uint8_t kP384PublicKey[] = {
      /* uncompressed */
      0x04,
      /* x-coordinate */
      0xe4, 0xe7, 0x0e, 0x43, 0xc6, 0xd0, 0x43, 0x46, 0xdd, 0xd7, 0x62, 0xa6,
      0x14, 0x17, 0x6d, 0x22, 0x78, 0xb0, 0x47, 0xc5, 0xec, 0x28, 0x64, 0x84,
      0x65, 0xf2, 0xa3, 0x90, 0xf6, 0xdd, 0x6b, 0xba, 0x54, 0xb9, 0x0b, 0x1e,
      0x62, 0xb3, 0x91, 0x85, 0xf8, 0xf3, 0x95, 0xf6, 0x65, 0x73, 0x6d, 0x1d,
      /* y-coordinate */
      0xf9, 0x62, 0xa2, 0x73, 0x6a, 0xce, 0x52, 0x56, 0x18, 0x15, 0xd5, 0x99,
      0x53, 0xa0, 0x19, 0x1b, 0x1f, 0xb1, 0xf2, 0x88, 0xa4, 0x5f, 0x8e, 0x28,
      0x3d, 0x40, 0xa5, 0xff, 0x0e, 0x83, 0x3f, 0xf3, 0x0b, 0xd6, 0x05, 0xb1,
      0x0c, 0xf8, 0xc2, 0x6c, 0x57, 0x4d, 0x4c, 0x2f, 0x0d, 0xcd, 0xce, 0x21
    };
    static const uint8_t kP384PrivateKey[] = {
      0x08, 0x95, 0x0a, 0xc9, 0x2e, 0x16, 0xce, 0x9e, 0x50, 0xed, 0xe3, 0x65,
      0x00, 0x3c, 0xb6, 0x2c, 0xea, 0x61, 0x03, 0xcf, 0xe5, 0x35, 0xfa, 0xb3,
      0xdc, 0x6f, 0x01, 0x45, 0xf3, 0x8e, 0xf1, 0x1c, 0x10, 0x3e, 0xf1, 0x40,
      0x79, 0x7e, 0x4f, 0x1e, 0x5f, 0x05, 0x3f, 0x8e, 0x83, 0x0c, 0xa7, 0xd9
    };
    static const uint8_t kP384ExpectedSharedSecret[] = {
      0x4b, 0x3c, 0xda, 0x1c, 0xef, 0xb6, 0x8d, 0x0a, 0x2e, 0xf3, 0x53, 0x04,
      0xa9, 0xb0, 0xca, 0x1d, 0x8c, 0xda, 0x8b, 0xdf, 0xc8, 0x01, 0x09, 0x8c,
      0xf7, 0x3c, 0x21, 0x8e, 0x65, 0x67, 0x22, 0xc3, 0x64, 0x96, 0x9a, 0x2a,
      0x1f, 0x57, 0xd1, 0x93, 0x03, 0x95, 0x98, 0x22, 0x7e, 0xf2, 0xb5, 0x18
    };

    struct ectlsencodedpoint_test_data p384_test_data = {
      kP384PublicKey, // public_key
      (1 + 48 + 48), // public_key_size
      kP384PrivateKey, // private_key
      48, // private_key_size
      kP384ExpectedSharedSecret, // expected_shared_secret
      48, // expected_shared_secret_size
      EVP_PKEY_EC, // key_type
      NID_secp384r1 // curve_nid
    };

    static const uint8_t kP521PublicKey[] = {
      /* uncompressed */
      0x04,
      /* x-coordinate */
      0x01, 0x03, 0x7e, 0x95, 0xff, 0x8e, 0x40, 0x31, 0xe0, 0xb0, 0x36, 0x1c,
      0x58, 0xc0, 0x62, 0x61, 0x39, 0x56, 0xaa, 0x30, 0x77, 0x0c, 0xed, 0x17,
      0x15, 0xed, 0x1b, 0x4d, 0x34, 0x29, 0x33, 0x0f, 0xac, 0x2f, 0xc5, 0xc9,
      0x3a, 0x69, 0xf7, 0x98, 0x63, 0x3a, 0x15, 0x75, 0x5e, 0x2d, 0xb8, 0x65,
      0x09, 0x87, 0xf5, 0x75, 0x85, 0xcd, 0xe3, 0x51, 0x6b, 0x6d, 0xd0, 0xfc,
      0x9f, 0x5f, 0xb4, 0xf8, 0xe7, 0x7b,
      /* y-coordinate */
      0x01, 0x1b, 0xba, 0xcc, 0x17, 0x80, 0x56, 0x8b, 0x9b, 0x32, 0xd4, 0x82,
      0x3f, 0x32, 0x9a, 0x46, 0xd8, 0x39, 0x39, 0xd1, 0x18, 0xcc, 0x97, 0x79,
      0x8d, 0x5d, 0xfa, 0x08, 0xb4, 0x27, 0xd3, 0xae, 0xe4, 0x76, 0x4f, 0x46,
      0x47, 0xf9, 0xf2, 0x4e, 0xcf, 0x0f, 0xee, 0x6d, 0x61, 0x9c, 0x79, 0x73,
      0xa8, 0x55, 0x4a, 0xd5, 0x51, 0x13, 0x0d, 0x1e, 0x3f, 0x6c, 0x9d, 0x2e,
      0xe3, 0xa2, 0xa8, 0x6f, 0xf5, 0xc3
    };
    static const uint8_t kP521PrivateKey[] = {
      0x01, 0xab, 0x4b, 0x1a, 0x8b, 0x60, 0xbb, 0x40, 0x23, 0xd6, 0x55, 0x05,
      0x0f, 0x0a, 0xd5, 0xd6, 0xe1, 0xbf, 0x5b, 0xc5, 0x23, 0x90, 0x2a, 0x2f,
      0x59, 0x69, 0x3e, 0xd0, 0xb9, 0x4f, 0x3c, 0x61, 0x06, 0xde, 0xb5, 0x92,
      0xe0, 0xf1, 0x74, 0xa7, 0x8b, 0xbd, 0xef, 0x23, 0xec, 0xeb, 0x23, 0xfc,
      0x97, 0x4b, 0x1c, 0xf5, 0x6a, 0x37, 0x73, 0x66, 0x6a, 0xfc, 0x76, 0x6f,
      0x3d, 0xdc, 0xb4, 0xc2, 0x92, 0xd0
    };
    static const uint8_t kP521ExpectedSharedSecret[] = {
      0x01, 0x1e, 0x28, 0x45, 0xc3, 0x2d, 0x1e, 0x49, 0xfc, 0x6a, 0x0e, 0x3c,
      0xc8, 0x05, 0xc0, 0x98, 0x45, 0x11, 0xb0, 0x7f, 0xf6, 0xea, 0x41, 0xe1,
      0xe1, 0x12, 0xee, 0x9c, 0x40, 0x8c, 0x74, 0xc3, 0x53, 0x5c, 0x97, 0xf2,
      0xf1, 0x8d, 0x62, 0xf4, 0x3d, 0x27, 0x21, 0x40, 0x7b, 0x82, 0x13, 0xd0,
      0x0b, 0xd3, 0x58, 0x86, 0x6a, 0x33, 0xc6, 0x0c, 0x67, 0x51, 0xd2, 0xdc,
      0x23, 0x50, 0x06, 0x15, 0xb2, 0xba
    };

    struct ectlsencodedpoint_test_data p521_test_data = {
      kP521PublicKey, // public_key
      (1 + 66 + 66), // public_key_size
      kP521PrivateKey, // private_key
      66, // private_key_size
      kP521ExpectedSharedSecret, // expected_shared_secret
      66, // expected_shared_secret_size
      EVP_PKEY_EC, // key_type
      NID_secp521r1 // curve_nid
    };

    ectlsencodedpoint_test_data test_data_all[] = {x25519_test_data,
      p224_test_data, p256_test_data, p384_test_data, p521_test_data};

    uint8_t *output = nullptr;
    uint8_t *shared_secret = nullptr;
    EVP_PKEY_CTX *pkey_ctx = nullptr;
    EVP_PKEY *pkey_public = nullptr;
    EVP_PKEY *pkey_private = nullptr;

    for (ectlsencodedpoint_test_data test_data : test_data_all) {
      size_t output_size = 0;
      size_t shared_secret_size = 0;

      pkey_private = instantiate_and_set_private_key(test_data.private_key,
        test_data.private_key_size, test_data.key_type, test_data.curve_nid);
      ASSERT_TRUE(pkey_private);
      pkey_public = instantiate_public_key(test_data.key_type,
        test_data.curve_nid);
      ASSERT_TRUE(pkey_public);

      // Test we can parse EC point into an EVP_PKEY object
      ASSERT_TRUE(EVP_PKEY_set1_tls_encodedpoint(pkey_public,
        test_data.public_key, test_data.public_key_size));

      // Test we can successfully perform a ECDH key derivation using the
      // parsed public key and a corresponding private key
      pkey_ctx = EVP_PKEY_CTX_new(pkey_private, nullptr);
      ASSERT_TRUE(pkey_ctx);
      ASSERT_TRUE(EVP_PKEY_derive_init(pkey_ctx));
      ASSERT_TRUE(EVP_PKEY_derive_set_peer(pkey_ctx, pkey_public));
      ASSERT_TRUE(EVP_PKEY_derive(pkey_ctx, nullptr, &shared_secret_size));
      EXPECT_EQ(shared_secret_size, test_data.expected_shared_secret_size);
      shared_secret = (uint8_t *) OPENSSL_malloc(shared_secret_size);
      ASSERT_TRUE(shared_secret);
      ASSERT_TRUE(EVP_PKEY_derive(pkey_ctx, shared_secret,
        &shared_secret_size));
      EXPECT_EQ(shared_secret_size, test_data.expected_shared_secret_size);
      EXPECT_EQ(Bytes(shared_secret, shared_secret_size),
        Bytes(test_data.expected_shared_secret, shared_secret_size));

      // Test we can write EC point from the EVP_PKEY object to wire format
      output_size = EVP_PKEY_get1_tls_encodedpoint(pkey_public, &output);
      EXPECT_EQ(output_size, test_data.public_key_size);
      EXPECT_EQ(Bytes(output, output_size),
        Bytes(test_data.public_key, output_size));

      OPENSSL_free(output);
      OPENSSL_free(shared_secret);
      EVP_PKEY_CTX_free(pkey_ctx);
      EVP_PKEY_free(pkey_public);
      EVP_PKEY_free(pkey_private);
    }

    // Above tests explore the happy path. Now test that some invalid
    // input parameters are handled gracefully and with no crashes.
    for (ectlsencodedpoint_test_data test_data : test_data_all) {

      pkey_public = instantiate_public_key(test_data.key_type,
        test_data.curve_nid);
      ASSERT_TRUE(pkey_public);

      // pkey = NULL should result in |ERR_R_PASSED_NULL_PARAMETER| being passed
      // back for both functions.
      ASSERT_FALSE(EVP_PKEY_set1_tls_encodedpoint(nullptr,
        test_data.public_key, test_data.public_key_size));
      EXPECT_EQ(ERR_R_PASSED_NULL_PARAMETER,
        ERR_GET_REASON(ERR_peek_last_error()));
      ERR_clear_error();
      ASSERT_FALSE(EVP_PKEY_get1_tls_encodedpoint(nullptr, &output));
      EXPECT_EQ(ERR_R_PASSED_NULL_PARAMETER,
        ERR_GET_REASON(ERR_peek_last_error()));
      ERR_clear_error();

      // For |EVP_PKEY_get1_tls_encodedpoint| if out_ptr = NULL, we should also
      // expect |ERR_R_PASSED_NULL_PARAMETER| being passed back.
      ASSERT_FALSE(EVP_PKEY_get1_tls_encodedpoint(pkey_public, nullptr));
      EXPECT_EQ(ERR_R_PASSED_NULL_PARAMETER,
        ERR_GET_REASON(ERR_peek_last_error()));
      ERR_clear_error();

      // For |EVP_PKEY_set1_tls_encodedpoint| if in = NULL or len < 1, we should
      // expect |ERR_R_PASSED_NULL_PARAMETER| or |EVP_R_INVALID_PARAMETERS|,
      // respectively.
      ASSERT_FALSE(EVP_PKEY_set1_tls_encodedpoint(pkey_public,
        nullptr, test_data.public_key_size));
      EXPECT_EQ(ERR_R_PASSED_NULL_PARAMETER,
        ERR_GET_REASON(ERR_peek_last_error()));
      ERR_clear_error();
      ASSERT_FALSE(EVP_PKEY_set1_tls_encodedpoint(pkey_public,
        test_data.public_key, 0));
      EXPECT_EQ(EVP_R_INVALID_PARAMETERS,
        ERR_GET_REASON(ERR_peek_last_error()));
      ERR_clear_error();

      EVP_PKEY_free(pkey_public);
    }

    // Test various unsupported key types are rejected
    int key_types_not_supported[] = {EVP_PKEY_RSA, EVP_PKEY_DSA,
      EVP_PKEY_ED25519};
    const uint8_t not_supported[] = {'n','o','t',' ','s','u','p','p','o','r',
      't','e','d'};
    size_t not_supported_size = 13; // specific size irrelevant
    uint8_t *not_supported_out = nullptr;
    bssl::UniquePtr<EVP_PKEY> pkey_key_type_not_supported(EVP_PKEY_new());

    for (int key_type : key_types_not_supported) {
      ASSERT_TRUE(pkey_key_type_not_supported.get());
      ASSERT_TRUE(EVP_PKEY_set_type(pkey_key_type_not_supported.get(),
        key_type));

      ASSERT_FALSE(EVP_PKEY_set1_tls_encodedpoint(
        pkey_key_type_not_supported.get(), not_supported, not_supported_size));
      EXPECT_EQ(EVP_R_UNSUPPORTED_PUBLIC_KEY_TYPE,
        ERR_GET_REASON(ERR_peek_last_error()));
      ERR_clear_error();

      ASSERT_FALSE(EVP_PKEY_get1_tls_encodedpoint(
        pkey_key_type_not_supported.get(), &not_supported_out));
      EXPECT_EQ(EVP_R_UNSUPPORTED_PUBLIC_KEY_TYPE,
        ERR_GET_REASON(ERR_peek_last_error()));
      ERR_clear_error();
    }

    // Test compressed encoded EC point is rejected
    static const uint8_t kP256PublicKeyCompressed[] = {
      /* uncompressed + parity bit */
      0x03,
      /* x-coordinate */
      0xe1, 0x5a, 0x44, 0x72, 0x91, 0xf0, 0x84, 0xfe, 0x88, 0x7a, 0x6c, 0x2c,
      0x03, 0x22, 0x9a, 0xf3, 0x04, 0x8a, 0x5d, 0xfe, 0x84, 0x73, 0x70, 0xc9,
      0x3f, 0x92, 0x72, 0x9b, 0x31, 0xc5, 0x5f, 0x7b,
    };

    bssl::UniquePtr<EVP_PKEY> pkey_public_compressed(instantiate_public_key(
      EVP_PKEY_EC, NID_X9_62_prime256v1));
    ASSERT_TRUE(pkey_public_compressed);

    ASSERT_FALSE(EVP_PKEY_set1_tls_encodedpoint(pkey_public_compressed.get(),
      kP256PublicKeyCompressed, 1 + 32));
    EXPECT_EQ(ERR_R_EVP_LIB,
      ERR_GET_REASON(ERR_peek_last_error()));
    ERR_clear_error();

    uint8_t *output_compressed = NULL;
    bssl::UniquePtr<EVP_PKEY> pkey_public_compressed_set(
      instantiate_and_set_public_key(kP256PublicKeyCompressed, 1 + 32,
        NID_X9_62_prime256v1));
    EC_KEY_set_conv_form(EVP_PKEY_get0_EC_KEY(pkey_public_compressed_set.get()),
      POINT_CONVERSION_COMPRESSED);
    ASSERT_TRUE(pkey_public_compressed_set.get());

    ASSERT_FALSE(EVP_PKEY_get1_tls_encodedpoint(
      pkey_public_compressed_set.get(), &output_compressed));
    EXPECT_EQ(ERR_R_EVP_LIB,
      ERR_GET_REASON(ERR_peek_last_error()));
    ERR_clear_error();
}

TEST(EVPTest, PKEY_set_type_str) {
  bssl::UniquePtr<EVP_PKEY> pkey(EVP_PKEY_new());
  /* Test case 1: Assign RSA algorithm */
  ASSERT_TRUE(EVP_PKEY_set_type_str(pkey.get(), "RSA", 3));
  ASSERT_EQ(pkey->type, EVP_PKEY_RSA);

  /* Test case 2: Assign EC algorithm */
  ASSERT_TRUE(EVP_PKEY_set_type_str(pkey.get(), "EC", 2));
  ASSERT_EQ(pkey->type, EVP_PKEY_EC);

  /* Test case 3: Assign non-existent algorithm */
  ASSERT_FALSE(EVP_PKEY_set_type_str(pkey.get(), "Nonsense", 8));
}

TEST(EVPTest, PKEY_asn1_find) {
  int pkey_id, pkey_base_id, pkey_flags;
  const char *pinfo, *pem_str;

  /* Test case 1: Find RSA algorithm */
  const EVP_PKEY_ASN1_METHOD* ameth = EVP_PKEY_asn1_find(NULL, EVP_PKEY_RSA);
  ASSERT_TRUE(ameth);
  ASSERT_TRUE(EVP_PKEY_asn1_get0_info(&pkey_id, &pkey_base_id, &pkey_flags, &pinfo, &pem_str, ameth));
  ASSERT_EQ(pkey_id, EVP_PKEY_RSA);
  ASSERT_EQ(pkey_base_id, EVP_PKEY_RSA);
  ASSERT_EQ(0, pkey_flags);
  ASSERT_STREQ("RSA", pem_str);
  ASSERT_STREQ("OpenSSL RSA method", pinfo);

  /* Test case 2: Find EC algorithm */
  ameth = EVP_PKEY_asn1_find(NULL, EVP_PKEY_EC);
  ASSERT_TRUE(ameth);
  ASSERT_TRUE(EVP_PKEY_asn1_get0_info(&pkey_id, &pkey_base_id, &pkey_flags, &pinfo, &pem_str, ameth));
  ASSERT_EQ(pkey_id, EVP_PKEY_EC);
  ASSERT_EQ(pkey_base_id, EVP_PKEY_EC);
  ASSERT_EQ(0, pkey_flags);
  ASSERT_STREQ("EC", pem_str);
  ASSERT_STREQ("OpenSSL EC algorithm", pinfo);

  /* Test case 3: Find non-existent algorithm */
  ameth = EVP_PKEY_asn1_find(NULL, EVP_PKEY_NONE);
  ASSERT_FALSE(ameth);
  ASSERT_FALSE(EVP_PKEY_asn1_get0_info(&pkey_id, &pkey_base_id, &pkey_flags, &pinfo, &pem_str, ameth));
}

TEST(EVPTest, PKEY_asn1_find_str) {
  int pkey_id, pkey_base_id, pkey_flags;
  const char *pinfo, *pem_str;

  /* Test case 1: Find RSA algorithm */
  const EVP_PKEY_ASN1_METHOD* ameth = EVP_PKEY_asn1_find_str(NULL, "RSA", 3);
  ASSERT_TRUE(ameth);
  ASSERT_TRUE(EVP_PKEY_asn1_get0_info(&pkey_id, &pkey_base_id, &pkey_flags, &pinfo, &pem_str, ameth));
  ASSERT_EQ(pkey_id, EVP_PKEY_RSA);
  ASSERT_EQ(pkey_base_id, EVP_PKEY_RSA);
  ASSERT_EQ(0, pkey_flags);
  ASSERT_STREQ("RSA", pem_str);
  ASSERT_STREQ("OpenSSL RSA method", pinfo);

  /* Test case 2: Find EC algorithm */
  ameth = EVP_PKEY_asn1_find_str(NULL, "EC", 2);
  ASSERT_TRUE(ameth);
  ASSERT_TRUE(EVP_PKEY_asn1_get0_info(&pkey_id, &pkey_base_id, &pkey_flags, &pinfo, &pem_str, ameth));
  ASSERT_EQ(pkey_id, EVP_PKEY_EC);
  ASSERT_EQ(pkey_base_id, EVP_PKEY_EC);
  ASSERT_EQ(0, pkey_flags);
  ASSERT_STREQ("EC", pem_str);
  ASSERT_STREQ("OpenSSL EC algorithm", pinfo);

  /* Test case 3: Find non-existent algorithm */
  ameth = EVP_PKEY_asn1_find_str(NULL, "Nonsense", 8);
  ASSERT_FALSE(ameth);
  ASSERT_FALSE(EVP_PKEY_asn1_get0_info(&pkey_id, &pkey_base_id, &pkey_flags, &pinfo, &pem_str, ameth));
}

TEST(EVPTest, ED25519PH) {
  const uint8_t message[] = {0x72, 0x61, 0x63, 0x63, 0x6f, 0x6f, 0x6e};
  const uint8_t context[] = {0x73, 0x6e, 0x65, 0x61, 0x6b, 0x79};
  const uint8_t message_sha512[] = {
      0x50, 0xcf, 0x03, 0x79, 0x8c, 0xb2, 0xfb, 0x0f, 0xf1, 0x3d, 0xc6,
      0x4c, 0x7c, 0xf0, 0x89, 0x8f, 0xfe, 0x90, 0x9d, 0xfd, 0xa5, 0x22,
      0xdd, 0x22, 0xf4, 0x10, 0x8f, 0xa0, 0x1b, 0x8f, 0x29, 0x15, 0x98,
      0x60, 0xf2, 0x80, 0x0e, 0x7c, 0x93, 0x3c, 0x7c, 0x6e, 0x4c, 0xb1,
      0xf9, 0x3f, 0x33, 0xbe, 0x43, 0xa3, 0xd4, 0x1c, 0x86, 0x92, 0x2b,
      0x32, 0xaf, 0x89, 0xa2, 0xa4, 0xa3, 0xe2, 0xf1, 0x92};

  bssl::UniquePtr<EVP_PKEY> pkey(nullptr);
  bssl::UniquePtr<EVP_PKEY> pubkey(nullptr);
  bssl::ScopedCBB marshalled_private_key;
  bssl::ScopedCBB marshalled_public_key;
  uint8_t signature[ED25519_SIGNATURE_LEN] = {0};
  size_t signature_len = ED25519_SIGNATURE_LEN;
  uint8_t working_signature[ED25519_SIGNATURE_LEN] = {0};
  size_t working_signature_len = ED25519_SIGNATURE_LEN;

  {
    bssl::UniquePtr<EVP_PKEY_CTX> ctx(
        EVP_PKEY_CTX_new_id(EVP_PKEY_ED25519PH, nullptr));
    ASSERT_FALSE(EVP_PKEY_keygen_init(ctx.get()));
  }

  {
    bssl::UniquePtr<EVP_PKEY_CTX> ctx(
        EVP_PKEY_CTX_new_id(EVP_PKEY_ED25519, nullptr));
    ASSERT_TRUE(EVP_PKEY_keygen_init(ctx.get()));
    EVP_PKEY *pkey_ptr = nullptr;
    ASSERT_TRUE(EVP_PKEY_keygen(ctx.get(), &pkey_ptr));
    ASSERT_NE(pkey_ptr, nullptr);
    pkey.reset(pkey_ptr);  // now owns pkey_ptr
    // marshal the keys
    ASSERT_TRUE(CBB_init(marshalled_private_key.get(), 0));
    ASSERT_TRUE(CBB_init(marshalled_public_key.get(), 0));
    ASSERT_TRUE(
        EVP_marshal_private_key(marshalled_private_key.get(), pkey.get()));
    ASSERT_TRUE(
        EVP_marshal_public_key(marshalled_public_key.get(), pkey.get()));
  }

  {
    uint8_t raw_key[ED25519_PRIVATE_KEY_SEED_LEN];
    size_t raw_key_len = sizeof(raw_key);
    ASSERT_TRUE(EVP_PKEY_get_raw_private_key(pkey.get(), raw_key, &raw_key_len));

    EVP_PKEY *rk = EVP_PKEY_new_raw_private_key(EVP_PKEY_ED25519PH, nullptr, raw_key, raw_key_len);
    ASSERT_TRUE(rk);
    pkey.reset(rk);
    ASSERT_EQ(EVP_PKEY_ED25519PH, EVP_PKEY_id(pkey.get()));

    bssl::ScopedCBB temp;
    ASSERT_TRUE(CBB_init(temp.get(), 0));
    ASSERT_FALSE(EVP_marshal_private_key(temp.get(), pkey.get()));
  }

  {
    uint8_t raw_key[ED25519_PUBLIC_KEY_LEN];
    size_t raw_key_len = sizeof(raw_key);
    ASSERT_TRUE(EVP_PKEY_get_raw_public_key(pkey.get(), raw_key, &raw_key_len));
    
    EVP_PKEY *rk = EVP_PKEY_new_raw_public_key(EVP_PKEY_ED25519PH, nullptr, raw_key, raw_key_len);
    ASSERT_TRUE(rk);
    pubkey.reset(rk);
    ASSERT_EQ(EVP_PKEY_ED25519PH, EVP_PKEY_id(pubkey.get()));

    bssl::ScopedCBB temp;
    ASSERT_TRUE(CBB_init(temp.get(), 0));
    ASSERT_FALSE(EVP_marshal_public_key(temp.get(), pubkey.get()));
  }

  // prehash signature w/ context gen and verify
  {
    bssl::UniquePtr<EVP_MD_CTX> md_ctx(EVP_MD_CTX_new());
    EVP_PKEY_CTX *pctx = nullptr;

    ASSERT_TRUE(EVP_DigestSignInit(md_ctx.get(), &pctx, EVP_sha512(), nullptr,
                                   pkey.get()));

    ASSERT_TRUE(
        EVP_PKEY_CTX_set1_signature_context_string(pctx, context, sizeof(context)));
    const uint8_t *sctx = NULL;
    size_t sctx_len = 0;
    ASSERT_TRUE(EVP_PKEY_CTX_get0_signature_context(pctx, &sctx, &sctx_len));
    ASSERT_TRUE(sctx);
    ASSERT_NE(sctx, context);
    ASSERT_EQ(Bytes(context, sizeof(context)), Bytes(sctx, sctx_len));

    ASSERT_TRUE(EVP_DigestSignUpdate(md_ctx.get(), &message[0], 3));
    ASSERT_TRUE(
        EVP_DigestSignUpdate(md_ctx.get(), &message[3], sizeof(message) - 3));
    ASSERT_TRUE(EVP_DigestSignFinal(md_ctx.get(), signature,
                                    &signature_len));
    ASSERT_EQ(signature_len, (size_t)ED25519_SIGNATURE_LEN);

    ASSERT_TRUE(EVP_DigestVerifyInit(md_ctx.get(), &pctx, EVP_sha512(), nullptr,
                                     pubkey.get()));
    ASSERT_TRUE(
        EVP_PKEY_CTX_set1_signature_context_string(pctx, context, sizeof(context)));
    ASSERT_TRUE(EVP_DigestVerifyUpdate(md_ctx.get(), &message[0], 3));
    ASSERT_TRUE(
        EVP_DigestVerifyUpdate(md_ctx.get(), &message[3], sizeof(message) - 3));
    ASSERT_TRUE(EVP_DigestVerifyFinal(md_ctx.get(),
                                      signature, signature_len));
  }

  // prehash signature gen and verify w/ context using EVP_PKEY_sign and
  // EVP_PKEY_verify directly
  {
    bssl::UniquePtr<EVP_PKEY_CTX> ctx(EVP_PKEY_CTX_new(pkey.get(), nullptr));
    ASSERT_TRUE(ctx.get());
    ASSERT_TRUE(EVP_PKEY_sign_init(ctx.get()));
    ASSERT_TRUE(EVP_PKEY_CTX_set1_signature_context_string(ctx.get(), context,
                                                   sizeof(context)));
    ASSERT_TRUE(EVP_PKEY_sign(ctx.get(), working_signature, &working_signature_len, message_sha512, sizeof(message_sha512)));
    ASSERT_EQ(working_signature_len, (size_t)ED25519_SIGNATURE_LEN);
    
    ctx.reset(EVP_PKEY_CTX_new(pubkey.get(), nullptr));
    ASSERT_TRUE(ctx.get());
    ASSERT_TRUE(EVP_PKEY_verify_init(ctx.get()));
    ASSERT_TRUE(EVP_PKEY_CTX_set1_signature_context_string(ctx.get(), context,
                                                   sizeof(context)));
    ASSERT_TRUE(EVP_PKEY_verify(ctx.get(), working_signature,
                                working_signature_len, message_sha512,
                                sizeof(message_sha512)));

    ASSERT_EQ(Bytes(signature, signature_len),
              Bytes(working_signature, working_signature_len));
  }

  // prehash signature gen and verify
  {
    bssl::UniquePtr<EVP_MD_CTX> md_ctx(EVP_MD_CTX_new());
    EVP_PKEY_CTX *pctx;

    ASSERT_TRUE(EVP_DigestSignInit(md_ctx.get(), &pctx, EVP_sha512(), nullptr,
                                   pkey.get()));

    const uint8_t *sctx = NULL;
    size_t sctx_len = 0;
    ASSERT_TRUE(EVP_PKEY_CTX_get0_signature_context(pctx, &sctx, &sctx_len));
    ASSERT_EQ(sctx, nullptr);
    ASSERT_EQ(sctx_len, (size_t)0);

    ASSERT_TRUE(EVP_DigestSignUpdate(md_ctx.get(), &message[0], 3));
    ASSERT_TRUE(
        EVP_DigestSignUpdate(md_ctx.get(), &message[3], sizeof(message) - 3));
    ASSERT_TRUE(EVP_DigestSignFinal(md_ctx.get(), working_signature,
                                    &working_signature_len));
    ASSERT_EQ(working_signature_len, (size_t)ED25519_SIGNATURE_LEN);

    ASSERT_TRUE(EVP_DigestVerifyInit(md_ctx.get(), nullptr, EVP_sha512(),
                                     nullptr, pubkey.get()));
    ASSERT_TRUE(EVP_DigestVerifyUpdate(md_ctx.get(), message, 3));
    ASSERT_TRUE(
        EVP_DigestVerifyUpdate(md_ctx.get(), &message[3], sizeof(message) - 3));
    ASSERT_TRUE(EVP_DigestVerifyFinal(md_ctx.get(), working_signature,
                                      working_signature_len));
  }

  // Pre-hash signature w/ context should not match Pre-hash signature w/o context
  ASSERT_NE(Bytes(signature, signature_len),
            Bytes(working_signature, working_signature_len));


  // prehash signature gen and verify with EVP_PKEY_sign and EVP_PKEY_verify directly
  {
    OPENSSL_memcpy(signature, working_signature, working_signature_len);
    signature_len = working_signature_len;

    bssl::UniquePtr<EVP_PKEY_CTX> ctx(EVP_PKEY_CTX_new(pkey.get(), nullptr));
    ASSERT_TRUE(ctx.get());
    ASSERT_TRUE(EVP_PKEY_sign_init(ctx.get()));
    ASSERT_TRUE(EVP_PKEY_sign(ctx.get(), working_signature, &working_signature_len, message_sha512, sizeof(message_sha512)));
    ASSERT_EQ(working_signature_len, (size_t)ED25519_SIGNATURE_LEN);

    ctx.reset(EVP_PKEY_CTX_new(pubkey.get(), nullptr));
    ASSERT_TRUE(ctx.get());
    ASSERT_TRUE(EVP_PKEY_verify_init(ctx.get()));
    ASSERT_TRUE(EVP_PKEY_verify(ctx.get(), working_signature, working_signature_len, message_sha512, sizeof(message_sha512)));

    ASSERT_EQ(Bytes(signature, signature_len),
              Bytes(working_signature, working_signature_len));
  }

  
  {
    CBS cbs;
    CBS_init(&cbs, CBB_data(marshalled_private_key.get()),
             CBB_len(marshalled_private_key.get()));
    EVP_PKEY *parsed = EVP_parse_private_key(&cbs);
    ASSERT_TRUE(parsed);
    pkey.reset(parsed);
    ASSERT_EQ(EVP_PKEY_ED25519, EVP_PKEY_id(pkey.get()));
  }

  {
    CBS cbs;
    CBS_init(&cbs, CBB_data(marshalled_public_key.get()),
             CBB_len(marshalled_public_key.get()));
    EVP_PKEY *parsed = EVP_parse_public_key(&cbs);
    ASSERT_TRUE(parsed);
    pubkey.reset(parsed);
    ASSERT_EQ(EVP_PKEY_ED25519, EVP_PKEY_id(pubkey.get()));
  }

  // pure signature gen and verify
  {
    bssl::UniquePtr<EVP_MD_CTX> md_ctx(EVP_MD_CTX_new());
    ASSERT_TRUE(EVP_DigestSignInit(md_ctx.get(), nullptr, nullptr, nullptr,
                                   pkey.get()));
    ASSERT_TRUE(EVP_DigestSign(md_ctx.get(), working_signature,
                               &working_signature_len, message, sizeof(message)));
    ASSERT_EQ(working_signature_len, (size_t)ED25519_SIGNATURE_LEN);

    ASSERT_TRUE(EVP_DigestVerifyInit(md_ctx.get(), nullptr, nullptr, nullptr,
                                     pubkey.get()));
    ASSERT_TRUE(EVP_DigestVerify(md_ctx.get(), working_signature,
                                 working_signature_len, message, sizeof(message)));
  }

  // pure signature shouldn't match a pre-hash signature w/o context
  ASSERT_NE(Bytes(signature, signature_len),
            Bytes(working_signature, working_signature_len));
}

TEST(EVPTest, ASN1MethodCheckPemStrLengthInvariant) {
  for (int i = 0; i < EVP_PKEY_asn1_get_count(); i++) {
    SCOPED_TRACE(i);
    const EVP_PKEY_ASN1_METHOD *method = EVP_PKEY_asn1_get0(i);
    ASSERT_NE(method, nullptr);
    ASSERT_NE(method->pem_str, nullptr);
    EXPECT_LE(OPENSSL_strnlen(method->pem_str, strlen(method->pem_str)+1), MAX_PEM_STR_LEN);
  }
}

TEST(EVPTest, Ed25519phTestVectors) {
  FileTestGTest("crypto/fipsmodule/curve25519/ed25519ph_tests.txt", [](FileTest *t) {
    std::vector<uint8_t> seed, q, message, context, expected_signature;
    ASSERT_TRUE(t->GetBytes(&seed, "SEED"));
    ASSERT_EQ(32u, seed.size());
    ASSERT_TRUE(t->GetBytes(&q, "Q"));
    ASSERT_EQ(32u, q.size());
    ASSERT_TRUE(t->GetBytes(&message, "MESSAGE"));
    ASSERT_TRUE(t->GetBytes(&expected_signature, "SIGNATURE"));
    ASSERT_EQ(64u, expected_signature.size());

    if (t->HasAttribute("CONTEXT")) {
        t->GetBytes(&context, "CONTEXT");
    } else {
        context = std::vector<uint8_t>();
    }

    bssl::UniquePtr<EVP_PKEY> pkey(EVP_PKEY_new_raw_private_key(EVP_PKEY_ED25519PH, nullptr, seed.data(), seed.size()));
    bssl::UniquePtr<EVP_PKEY> pubkey(EVP_PKEY_new_raw_public_key(EVP_PKEY_ED25519PH, nullptr, q.data(), q.size()));
    ASSERT_TRUE(pkey.get());
    ASSERT_TRUE(pubkey.get());
    ASSERT_EQ(EVP_PKEY_ED25519PH, EVP_PKEY_id(pkey.get()));
    ASSERT_EQ(EVP_PKEY_ED25519PH, EVP_PKEY_id(pubkey.get()));

    bssl::UniquePtr<EVP_MD_CTX> md_ctx(EVP_MD_CTX_new());
    EVP_PKEY_CTX *pctx = nullptr;
    uint8_t signature[ED25519_SIGNATURE_LEN] = {};
    size_t signature_len = ED25519_SIGNATURE_LEN;

    ASSERT_TRUE(EVP_DigestSignInit(md_ctx.get(), &pctx, EVP_sha512(), nullptr,
                                   pkey.get()));
    ASSERT_TRUE(
        EVP_PKEY_CTX_set1_signature_context_string(pctx, context.data(), context.size()));
    ASSERT_TRUE(EVP_DigestSignUpdate(md_ctx.get(), message.data(), message.size()));
    ASSERT_TRUE(EVP_DigestSignFinal(md_ctx.get(), signature,
                                    &signature_len));
    ASSERT_EQ(signature_len, (size_t)ED25519_SIGNATURE_LEN);
    ASSERT_EQ(Bytes(expected_signature), Bytes(signature, signature_len));

    ASSERT_TRUE(EVP_DigestVerifyInit(md_ctx.get(), &pctx, EVP_sha512(), nullptr,
                                     pubkey.get()));
    ASSERT_TRUE(
        EVP_PKEY_CTX_set1_signature_context_string(pctx, context.data(), context.size()));
    ASSERT_TRUE(EVP_DigestVerifyUpdate(md_ctx.get(), message.data(), message.size()));
    ASSERT_TRUE(EVP_DigestVerifyFinal(md_ctx.get(), signature,
                                      signature_len));
  });
}

TEST(EVPTest, SignUndersizedBuffer) {
  // EC: undersized buffer should be rejected.
  {
    bssl::UniquePtr<EC_KEY> ec(EC_KEY_new_by_curve_name(NID_X9_62_prime256v1));
    ASSERT_TRUE(ec);
    ASSERT_TRUE(EC_KEY_generate_key(ec.get()));
    bssl::UniquePtr<EVP_PKEY> key(EVP_PKEY_new());
    ASSERT_TRUE(key);
    ASSERT_TRUE(EVP_PKEY_set1_EC_KEY(key.get(), ec.get()));

    uint8_t digest[32] = {0};
    bssl::UniquePtr<EVP_PKEY_CTX> ctx(EVP_PKEY_CTX_new(key.get(), nullptr));
    ASSERT_TRUE(ctx);
    ASSERT_TRUE(EVP_PKEY_sign_init(ctx.get()));

    size_t siglen = 0;
    ASSERT_EQ(1, EVP_PKEY_sign(ctx.get(), NULL, &siglen, digest, 32));
    ASSERT_GT(siglen, (size_t)0);

    std::vector<uint8_t> sig(siglen);
    size_t too_small = 1;
    EXPECT_FALSE(EVP_PKEY_sign(ctx.get(), sig.data(), &too_small, digest, 32));
    EXPECT_EQ(EVP_R_BUFFER_TOO_SMALL,
              ERR_GET_REASON(ERR_peek_last_error()));
    ERR_clear_error();
  }

  // RSA: undersized buffer should be rejected.
  {
    bssl::UniquePtr<RSA> rsa(RSA_new());
    ASSERT_TRUE(rsa);
    bssl::UniquePtr<BIGNUM> e(BN_new());
    ASSERT_TRUE(e);
    ASSERT_TRUE(BN_set_word(e.get(), RSA_F4));
    ASSERT_TRUE(RSA_generate_key_ex(rsa.get(), 2048, e.get(), nullptr));
    bssl::UniquePtr<EVP_PKEY> key(EVP_PKEY_new());
    ASSERT_TRUE(key);
    ASSERT_TRUE(EVP_PKEY_set1_RSA(key.get(), rsa.get()));

    uint8_t digest[32] = {0};
    bssl::UniquePtr<EVP_PKEY_CTX> ctx(EVP_PKEY_CTX_new(key.get(), nullptr));
    ASSERT_TRUE(ctx);
    ASSERT_TRUE(EVP_PKEY_sign_init(ctx.get()));
    ASSERT_TRUE(EVP_PKEY_CTX_set_rsa_padding(ctx.get(), RSA_PKCS1_PADDING));
    ASSERT_TRUE(EVP_PKEY_CTX_set_signature_md(ctx.get(), EVP_sha256()));

    size_t siglen = 0;
    ASSERT_EQ(1, EVP_PKEY_sign(ctx.get(), NULL, &siglen, digest, 32));
    ASSERT_GT(siglen, (size_t)0);

    std::vector<uint8_t> sig(siglen);
    size_t too_small = 1;
    EXPECT_FALSE(EVP_PKEY_sign(ctx.get(), sig.data(), &too_small, digest, 32));
    EXPECT_EQ(EVP_R_BUFFER_TOO_SMALL,
              ERR_GET_REASON(ERR_peek_last_error()));
    ERR_clear_error();
  }
}
