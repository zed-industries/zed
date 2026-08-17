// Copyright (c) 2016, Google Inc.
// SPDX-License-Identifier: ISC

#include <stdio.h>

#include <utility>
#include <vector>

#include <gtest/gtest.h>

#include <openssl/bn.h>
#include <openssl/bytestring.h>
#include <openssl/crypto.h>
#include <openssl/ec.h>
#include <openssl/ec_key.h>
#include <openssl/ecdh.h>
#include <openssl/err.h>
#include <openssl/evp.h>
#include <openssl/nid.h>
#include <openssl/sha.h>

#include "../fipsmodule/ec/internal.h"

#include "../test/file_test.h"
#include "../test/test_util.h"
#include "../test/wycheproof_util.h"


static const EC_GROUP *GetCurve(FileTest *t, const char *key) {
  std::string curve_name;
  if (!t->GetAttribute(&curve_name, key)) {
    return nullptr;
  }

  if (curve_name == "P-224") {
    return EC_group_p224();
  }
  if (curve_name == "P-256") {
    return EC_group_p256();
  }
  if (curve_name == "P-384") {
    return EC_group_p384();
  }
  if (curve_name == "P-521") {
    return EC_group_p521();
  }
  if (curve_name == "secp256k1") {
    return EC_group_secp256k1();
  }

  t->PrintLine("Unknown curve '%s'", curve_name.c_str());
  return nullptr;
}

static bssl::UniquePtr<BIGNUM> GetBIGNUM(FileTest *t, const char *key) {
  std::vector<uint8_t> bytes;
  if (!t->GetBytes(&bytes, key)) {
    return nullptr;
  }

  return bssl::UniquePtr<BIGNUM>(BN_bin2bn(bytes.data(), bytes.size(), nullptr));
}

TEST(ECDHTest, TestVectors) {
  FileTestGTest("crypto/ecdh_extra/ecdh_tests.txt", [](FileTest *t) {
    const EC_GROUP *group = GetCurve(t, "Curve");
    ASSERT_TRUE(group);
    bssl::UniquePtr<BIGNUM> priv_key = GetBIGNUM(t, "Private");
    ASSERT_TRUE(priv_key);
    bssl::UniquePtr<BIGNUM> x = GetBIGNUM(t, "X");
    ASSERT_TRUE(x);
    bssl::UniquePtr<BIGNUM> y = GetBIGNUM(t, "Y");
    ASSERT_TRUE(y);
    bssl::UniquePtr<BIGNUM> peer_x = GetBIGNUM(t, "PeerX");
    ASSERT_TRUE(peer_x);
    bssl::UniquePtr<BIGNUM> peer_y = GetBIGNUM(t, "PeerY");
    ASSERT_TRUE(peer_y);
    std::vector<uint8_t> z;
    ASSERT_TRUE(t->GetBytes(&z, "Z"));

    bssl::UniquePtr<EC_KEY> key(EC_KEY_new());
    ASSERT_TRUE(key);
    bssl::UniquePtr<EC_POINT> pub_key(EC_POINT_new(group));
    ASSERT_TRUE(pub_key);
    bssl::UniquePtr<EC_POINT> peer_pub_key(EC_POINT_new(group));
    ASSERT_TRUE(peer_pub_key);
    ASSERT_TRUE(EC_KEY_set_group(key.get(), group));
    ASSERT_TRUE(EC_KEY_set_private_key(key.get(), priv_key.get()));
    ASSERT_TRUE(EC_POINT_set_affine_coordinates_GFp(group, pub_key.get(),
                                                    x.get(), y.get(), nullptr));
    ASSERT_TRUE(EC_POINT_set_affine_coordinates_GFp(
        group, peer_pub_key.get(), peer_x.get(), peer_y.get(), nullptr));
    ASSERT_TRUE(EC_KEY_set_public_key(key.get(), pub_key.get()));
    ASSERT_TRUE(EC_KEY_check_key(key.get()));

    // Check EVP_PKEY_check and EVP_PKEY_public_check
    bssl::UniquePtr<EVP_PKEY> ec_pkey(EVP_PKEY_new());
    ASSERT_TRUE(ec_pkey);
    ASSERT_TRUE(EVP_PKEY_set1_EC_KEY(ec_pkey.get(), key.get()));
    bssl::UniquePtr<EVP_PKEY_CTX> ec_key_ctx(
            EVP_PKEY_CTX_new(ec_pkey.get(), NULL));
    ASSERT_TRUE(ec_key_ctx);
    ASSERT_TRUE(EVP_PKEY_check(ec_key_ctx.get()));
    ASSERT_TRUE(EVP_PKEY_public_check((ec_key_ctx.get())));

    std::vector<uint8_t> actual_z;
    // Make |actual_z| larger than expected to ensure |ECDH_compute_key| returns
    // the right amount of data.
    actual_z.resize(z.size() + 1);
    int ret = ECDH_compute_key(actual_z.data(), actual_z.size(),
                               peer_pub_key.get(), key.get(), nullptr);
    ASSERT_GE(ret, 0);
    EXPECT_EQ(Bytes(z), Bytes(actual_z.data(), static_cast<size_t>(ret)));

    // Test |ECDH_compute_key| truncates.
    actual_z.resize(z.size() - 1);
    ret = ECDH_compute_key(actual_z.data(), actual_z.size(), peer_pub_key.get(),
                           key.get(), nullptr);
    ASSERT_GE(ret, 0);
    EXPECT_EQ(Bytes(z.data(), z.size() - 1),
              Bytes(actual_z.data(), static_cast<size_t>(ret)));

    // Test that |ECDH_compute_key_fips| hashes as expected.
    uint8_t digest[SHA256_DIGEST_LENGTH], expected_digest[SHA256_DIGEST_LENGTH];
    ASSERT_TRUE(ECDH_compute_key_fips(digest, sizeof(digest),
                                      peer_pub_key.get(), key.get()));
    SHA256(z.data(), z.size(), expected_digest);
    EXPECT_EQ(Bytes(digest), Bytes(expected_digest));
  });
}

// Returns 1 if the curve defined by |nid| is using Montgomery representation
// for field elements (based on the build configuration). Returns 0 otherwise.
static int is_curve_using_mont_felem_impl(int nid) {
  if (nid == NID_secp224r1) {
#if defined(BORINGSSL_HAS_UINT128) && !defined(OPENSSL_SMALL)
    return 0;
#endif
  } else if (nid == NID_secp521r1) {
#if !defined(OPENSSL_SMALL)
    return 0;
#endif
  }
  return 1;
}

// The following test is adapted from ECTest.LargeXCoordinateVectors
TEST(ECDHTest, InvalidPubKeyLargeCoord) {
  bssl::UniquePtr<BN_CTX> ctx(BN_CTX_new());
  ASSERT_TRUE(ctx);

  FileTestGTest("crypto/fipsmodule/ec/large_x_coordinate_points.txt",
                [&](FileTest *t) {
    int ret;
	const EC_GROUP *group = GetCurve(t, "Curve");
    ASSERT_TRUE(group);
    bssl::UniquePtr<BIGNUM> x = GetBIGNUM(t, "X");
    ASSERT_TRUE(x);
    bssl::UniquePtr<BIGNUM> xpp = GetBIGNUM(t, "XplusP");
    ASSERT_TRUE(xpp);
    bssl::UniquePtr<BIGNUM> y = GetBIGNUM(t, "Y");
    ASSERT_TRUE(y);
    bssl::UniquePtr<EC_KEY> peer_key(EC_KEY_new());
    ASSERT_TRUE(peer_key);
    bssl::UniquePtr<EC_POINT> pub_key(EC_POINT_new(group));
    ASSERT_TRUE(pub_key);
    bssl::UniquePtr<EC_KEY> priv_key(EC_KEY_new());
    // Own private key
    ASSERT_TRUE(priv_key);
    ASSERT_TRUE(EC_KEY_set_group(priv_key.get(), group));
    // Generate a generic ec key.
    EC_KEY_generate_key(priv_key.get());

    size_t len = BN_num_bytes(&group->field.N); // Modulus byte-length
    std::vector<uint8_t> shared_key((group->curve_name == NID_secp521r1) ?
                                    SHA512_DIGEST_LENGTH : len);

    ASSERT_TRUE(EC_KEY_set_group(peer_key.get(), group));

    // |EC_POINT_set_affine_coordinates_GFp| sets given (x, y) according to the
    // form the curve is using. If the curve is using Montgomery form, |x| and
    // |y| will be converted to Montgomery form.
    ASSERT_TRUE(EC_POINT_set_affine_coordinates_GFp(
                  group, pub_key.get(), x.get(), y.get(), nullptr));
    ASSERT_TRUE(EC_KEY_set_public_key(peer_key.get(), pub_key.get()));
    ASSERT_TRUE(ECDH_compute_key_fips(
          shared_key.data(), shared_key.size(),
          EC_KEY_get0_public_key(peer_key.get()), priv_key.get()));

    // Ensure the pointers were not affected.
    ASSERT_TRUE(peer_key.get());
    ASSERT_TRUE(pub_key.get());

    // Set the raw point directly with the BIGNUM coordinates.
    // Note that both are in little-endian byte order.
    OPENSSL_memcpy(peer_key.get()->pub_key->raw.X.words,
                   x.get()->d, len);
    OPENSSL_memcpy(peer_key.get()->pub_key->raw.Y.words,
                   y.get()->d, len);
    OPENSSL_memset(peer_key.get()->pub_key->raw.Z.words, 0, len);
    peer_key.get()->pub_key->raw.Z.words[0] = 1;

    // |ECDH_compute_key_fips| calls |EC_KEY_check_fips| that calls
    // |EC_KEY_check_key| function which checks if the computed key point is on
    // the curve (among other checks). If the curve uses Montgomery form then
    // the point-on-curve check will fail because we set the raw point
    // coordinates in regular form above.
    ret = ECDH_compute_key_fips(shared_key.data(), shared_key.size(),
                                EC_KEY_get0_public_key(peer_key.get()),
                                priv_key.get());

    int curve_nid = group->curve_name;
    if (!is_curve_using_mont_felem_impl(curve_nid)) {
      ASSERT_TRUE(ret);
    } else {
      ASSERT_FALSE(ret);
      // Fails in |EC_KEY_check_fips|.
      EXPECT_EQ(EC_R_PUBLIC_KEY_VALIDATION_FAILED,
                ERR_GET_REASON(ERR_peek_last_error()));
    }
    ASSERT_TRUE(peer_key.get());
    ASSERT_TRUE(pub_key.get());

    // Now replace the x-coordinate with the larger one, x+p;
    OPENSSL_memcpy(peer_key.get()->pub_key->raw.X.words,
                   xpp.get()->d, len);
    ret = ECDH_compute_key_fips(shared_key.data(), shared_key.size(),
                                EC_KEY_get0_public_key(peer_key.get()),
                                priv_key.get());
    ASSERT_FALSE(ret);
    EXPECT_EQ(EC_R_PUBLIC_KEY_VALIDATION_FAILED,
              ERR_GET_REASON(ERR_peek_last_error()));

    ASSERT_TRUE(peer_key.get());
    ASSERT_TRUE(pub_key.get());
  });
}

static void RunWycheproofTest(FileTest *t) {
  t->IgnoreInstruction("encoding");

  const EC_GROUP *group = GetWycheproofCurve(t, "curve", true);
  ASSERT_TRUE(group);
  bssl::UniquePtr<BIGNUM> priv_key = GetWycheproofBIGNUM(t, "private", false);
  ASSERT_TRUE(priv_key);
  std::vector<uint8_t> peer_spki;
  ASSERT_TRUE(t->GetBytes(&peer_spki, "public"));
  WycheproofResult result;
  ASSERT_TRUE(GetWycheproofResult(t, &result));
  std::vector<uint8_t> shared;
  ASSERT_TRUE(t->GetBytes(&shared, "shared"));
  // BoringSSL supports compressed coordinates.
  bool is_valid = result.IsValid({"CompressedPoint", "UnnamedCurve", "UnusedParam"});

  // Wycheproof stores the peer key in an SPKI to mimic a Java API mistake.
  // This is non-standard and error-prone.
  CBS cbs;
  CBS_init(&cbs, peer_spki.data(), peer_spki.size());
  bssl::UniquePtr<EVP_PKEY> peer_evp(EVP_parse_public_key(&cbs));
  if (!peer_evp || CBS_len(&cbs) != 0) {
    if (!peer_evp && result.raw_result == WycheproofRawResult::kAcceptable &&
        std::find(result.flags.begin(), result.flags.end(), "UnusedParam") !=
            result.flags.end()) {
      // The proof flags are not granular enough, and we do validate some
      // parameters for unnamed / explicit curves.
      //
      // So skip tests where the result was acceptable and UnusedParam was a flag if we rejected the
      // public key.
      t->SkipCurrent();
      return;
    }
    EXPECT_FALSE(is_valid);
    return;
  }
  EC_KEY *peer_ec = EVP_PKEY_get0_EC_KEY(peer_evp.get());
  ASSERT_TRUE(peer_ec);
  OPENSSL_BEGIN_ALLOW_DEPRECATED
  ASSERT_EQ(peer_ec, EVP_PKEY_get0(peer_evp.get()));
  if (std::find(result.flags.begin(), result.flags.end(), "UnnamedCurve") !=
      result.flags.end()) {
    ASSERT_EQ(1, EC_KEY_decoded_from_explicit_params(peer_ec));
  } else {
    ASSERT_EQ(0, EC_KEY_decoded_from_explicit_params(peer_ec));
  }
  OPENSSL_END_ALLOW_DEPRECATED
  bssl::UniquePtr<EC_KEY> key(EC_KEY_new());
  ASSERT_TRUE(key);
  ASSERT_TRUE(EC_KEY_set_group(key.get(), group));
  ASSERT_TRUE(EC_KEY_set_private_key(key.get(), priv_key.get()));

  std::vector<uint8_t> actual((EC_GROUP_get_degree(group) + 7) / 8);
  int ret =
      ECDH_compute_key(actual.data(), actual.size(),
                       EC_KEY_get0_public_key(peer_ec), key.get(), nullptr);
  if (is_valid) {
    EXPECT_EQ(static_cast<int>(actual.size()), ret);
    EXPECT_EQ(Bytes(shared), Bytes(actual.data(), static_cast<size_t>(ret)));
  } else {
    EXPECT_EQ(-1, ret);
  }
}

TEST(ECDHTest, WycheproofP224) {
  FileTestGTest("third_party/wycheproof_testvectors/ecdh_secp224r1_test.txt",
                RunWycheproofTest);
}

TEST(ECDHTest, WycheproofP256) {
  FileTestGTest("third_party/wycheproof_testvectors/ecdh_secp256r1_test.txt",
                RunWycheproofTest);
}

TEST(ECDHTest, WycheproofP384) {
  FileTestGTest("third_party/wycheproof_testvectors/ecdh_secp384r1_test.txt",
                RunWycheproofTest);
}

TEST(ECDHTest, WycheproofP512) {
  FileTestGTest("third_party/wycheproof_testvectors/ecdh_secp521r1_test.txt",
                RunWycheproofTest);
}

// MakeCustomGroup returns an |EC_GROUP| containing a non-standard group. (P-256
// with the wrong generator.)
static bssl::UniquePtr<EC_GROUP> MakeCustomGroup() {
  static const uint8_t kP[] = {
      0xff, 0xff, 0xff, 0xff, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00,
      0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xff, 0xff,
      0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
  };
  static const uint8_t kA[] = {
      0xff, 0xff, 0xff, 0xff, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00,
      0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xff, 0xff,
      0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xfc,
  };
  static const uint8_t kB[] = {
      0x5a, 0xc6, 0x35, 0xd8, 0xaa, 0x3a, 0x93, 0xe7, 0xb3, 0xeb, 0xbd,
      0x55, 0x76, 0x98, 0x86, 0xbc, 0x65, 0x1d, 0x06, 0xb0, 0xcc, 0x53,
      0xb0, 0xf6, 0x3b, 0xce, 0x3c, 0x3e, 0x27, 0xd2, 0x60, 0x4b,
  };
  static const uint8_t kX[] = {
      0xe6, 0x2b, 0x69, 0xe2, 0xbf, 0x65, 0x9f, 0x97, 0xbe, 0x2f, 0x1e,
      0x0d, 0x94, 0x8a, 0x4c, 0xd5, 0x97, 0x6b, 0xb7, 0xa9, 0x1e, 0x0d,
      0x46, 0xfb, 0xdd, 0xa9, 0xa9, 0x1e, 0x9d, 0xdc, 0xba, 0x5a,
  };
  static const uint8_t kY[] = {
      0x01, 0xe7, 0xd6, 0x97, 0xa8, 0x0a, 0x18, 0xf9, 0xc3, 0xc4, 0xa3,
      0x1e, 0x56, 0xe2, 0x7c, 0x83, 0x48, 0xdb, 0x16, 0x1a, 0x1c, 0xf5,
      0x1d, 0x7e, 0xf1, 0x94, 0x2d, 0x4b, 0xcf, 0x72, 0x22, 0xc1,
  };
  static const uint8_t kOrder[] = {
      0xff, 0xff, 0xff, 0xff, 0x00, 0x00, 0x00, 0x00, 0xff, 0xff, 0xff,
      0xff, 0xff, 0xff, 0xff, 0xff, 0xbc, 0xe6, 0xfa, 0xad, 0xa7, 0x17,
      0x9e, 0x84, 0xf3, 0xb9, 0xca, 0xc2, 0xfc, 0x63, 0x25, 0x51,
  };
  bssl::UniquePtr<BN_CTX> ctx(BN_CTX_new());
  bssl::UniquePtr<BIGNUM> p(BN_bin2bn(kP, sizeof(kP), nullptr));
  bssl::UniquePtr<BIGNUM> a(BN_bin2bn(kA, sizeof(kA), nullptr));
  bssl::UniquePtr<BIGNUM> b(BN_bin2bn(kB, sizeof(kB), nullptr));
  bssl::UniquePtr<BIGNUM> x(BN_bin2bn(kX, sizeof(kX), nullptr));
  bssl::UniquePtr<BIGNUM> y(BN_bin2bn(kY, sizeof(kY), nullptr));
  bssl::UniquePtr<BIGNUM> order(BN_bin2bn(kOrder, sizeof(kOrder), nullptr));
  if (!ctx || !p || !a || !b || !x || !y || !order) {
    return nullptr;
  }
  bssl::UniquePtr<EC_GROUP> group(
      EC_GROUP_new_curve_GFp(p.get(), a.get(), b.get(), ctx.get()));
  if (!group) {
    return nullptr;
  }
  bssl::UniquePtr<EC_POINT> generator(EC_POINT_new(group.get()));
  if (!generator ||
      !EC_POINT_set_affine_coordinates_GFp(group.get(), generator.get(),
                                           x.get(), y.get(), ctx.get()) ||
      !EC_GROUP_set_generator(group.get(), generator.get(), order.get(),
                              BN_value_one())) {
    return nullptr;
  }
  return group;
}

TEST(ECDHTest, GroupMismatch) {
  const size_t num_curves = EC_get_builtin_curves(nullptr, 0);
  std::vector<EC_builtin_curve> curves(num_curves);
  EC_get_builtin_curves(curves.data(), num_curves);

  // Instantiate all the built-in curves.
  std::vector<bssl::UniquePtr<EC_GROUP>> groups;
  for (const auto &curve : curves) {
    groups.emplace_back(EC_GROUP_new_by_curve_name(curve.nid));
    ASSERT_TRUE(groups.back());
  }

  // Also create some arbitrary group. (This is P-256 with the wrong generator.)
  groups.push_back(MakeCustomGroup());
  ASSERT_TRUE(groups.back());

  for (const auto &a : groups) {
    for (const auto &b : groups) {
      if (a.get() == b.get()) {
        continue;
      }

      bssl::UniquePtr<EC_KEY> key(EC_KEY_new());
      ASSERT_TRUE(key);
      ASSERT_TRUE(EC_KEY_set_group(key.get(), a.get()));
      ASSERT_TRUE(EC_KEY_generate_key(key.get()));

      // ECDH across the groups should not work.
      char out[64];
      const EC_POINT *peer = EC_GROUP_get0_generator(b.get());
      EXPECT_EQ(-1,
                ECDH_compute_key(out, sizeof(out), peer, key.get(), nullptr));
      ERR_clear_error();
    }
  }
}
