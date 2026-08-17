// Copyright (c) 2014, Google Inc.
// SPDX-License-Identifier: ISC

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include <vector>

#include <gtest/gtest.h>

#include <openssl/bn.h>
#include <openssl/bytestring.h>
#include <openssl/crypto.h>
#include <openssl/ec.h>
#include <openssl/ec_key.h>
#include <openssl/ecdsa.h>
#include <openssl/evp.h>
#include <openssl/rand.h>
#include <openssl/err.h>
#include <openssl/evp.h>
#include <openssl/mem.h>
#include <openssl/nid.h>
#include <openssl/obj.h>
#include <openssl/span.h>

#include "../../ec_extra/internal.h"
#include "../../test/file_test.h"
#include "../../test/test_util.h"
#include "../bn/internal.h"
#include "internal.h"


// kECKeyWithoutPublic is an ECPrivateKey with the optional publicKey field
// omitted.
static const uint8_t kECKeyWithoutPublic[] = {
  0x30, 0x31, 0x02, 0x01, 0x01, 0x04, 0x20, 0xc6, 0xc1, 0xaa, 0xda, 0x15, 0xb0,
  0x76, 0x61, 0xf8, 0x14, 0x2c, 0x6c, 0xaf, 0x0f, 0xdb, 0x24, 0x1a, 0xff, 0x2e,
  0xfe, 0x46, 0xc0, 0x93, 0x8b, 0x74, 0xf2, 0xbc, 0xc5, 0x30, 0x52, 0xb0, 0x77,
  0xa0, 0x0a, 0x06, 0x08, 0x2a, 0x86, 0x48, 0xce, 0x3d, 0x03, 0x01, 0x07,
};

// kECKeySpecifiedCurve is the above key with P-256's parameters explicitly
// spelled out rather than using a named curve.
static const uint8_t kECKeySpecifiedCurve[] = {
    0x30, 0x82, 0x01, 0x22, 0x02, 0x01, 0x01, 0x04, 0x20, 0xc6, 0xc1, 0xaa,
    0xda, 0x15, 0xb0, 0x76, 0x61, 0xf8, 0x14, 0x2c, 0x6c, 0xaf, 0x0f, 0xdb,
    0x24, 0x1a, 0xff, 0x2e, 0xfe, 0x46, 0xc0, 0x93, 0x8b, 0x74, 0xf2, 0xbc,
    0xc5, 0x30, 0x52, 0xb0, 0x77, 0xa0, 0x81, 0xfa, 0x30, 0x81, 0xf7, 0x02,
    0x01, 0x01, 0x30, 0x2c, 0x06, 0x07, 0x2a, 0x86, 0x48, 0xce, 0x3d, 0x01,
    0x01, 0x02, 0x21, 0x00, 0xff, 0xff, 0xff, 0xff, 0x00, 0x00, 0x00, 0x01,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0x30, 0x5b, 0x04, 0x20, 0xff, 0xff, 0xff, 0xff, 0x00, 0x00, 0x00, 0x01,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xfc,
    0x04, 0x20, 0x5a, 0xc6, 0x35, 0xd8, 0xaa, 0x3a, 0x93, 0xe7, 0xb3, 0xeb,
    0xbd, 0x55, 0x76, 0x98, 0x86, 0xbc, 0x65, 0x1d, 0x06, 0xb0, 0xcc, 0x53,
    0xb0, 0xf6, 0x3b, 0xce, 0x3c, 0x3e, 0x27, 0xd2, 0x60, 0x4b, 0x03, 0x15,
    0x00, 0xc4, 0x9d, 0x36, 0x08, 0x86, 0xe7, 0x04, 0x93, 0x6a, 0x66, 0x78,
    0xe1, 0x13, 0x9d, 0x26, 0xb7, 0x81, 0x9f, 0x7e, 0x90, 0x04, 0x41, 0x04,
    0x6b, 0x17, 0xd1, 0xf2, 0xe1, 0x2c, 0x42, 0x47, 0xf8, 0xbc, 0xe6, 0xe5,
    0x63, 0xa4, 0x40, 0xf2, 0x77, 0x03, 0x7d, 0x81, 0x2d, 0xeb, 0x33, 0xa0,
    0xf4, 0xa1, 0x39, 0x45, 0xd8, 0x98, 0xc2, 0x96, 0x4f, 0xe3, 0x42, 0xe2,
    0xfe, 0x1a, 0x7f, 0x9b, 0x8e, 0xe7, 0xeb, 0x4a, 0x7c, 0x0f, 0x9e, 0x16,
    0x2b, 0xce, 0x33, 0x57, 0x6b, 0x31, 0x5e, 0xce, 0xcb, 0xb6, 0x40, 0x68,
    0x37, 0xbf, 0x51, 0xf5, 0x02, 0x21, 0x00, 0xff, 0xff, 0xff, 0xff, 0x00,
    0x00, 0x00, 0x00, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xbc,
    0xe6, 0xfa, 0xad, 0xa7, 0x17, 0x9e, 0x84, 0xf3, 0xb9, 0xca, 0xc2, 0xfc,
    0x63, 0x25, 0x51, 0x02, 0x01, 0x01,
};

// kECKeyMissingZeros is an ECPrivateKey containing a degenerate P-256 key where
// the private key is one. The private key is incorrectly encoded without zero
// padding.
static const uint8_t kECKeyMissingZeros[] = {
  0x30, 0x58, 0x02, 0x01, 0x01, 0x04, 0x01, 0x01, 0xa0, 0x0a, 0x06, 0x08, 0x2a,
  0x86, 0x48, 0xce, 0x3d, 0x03, 0x01, 0x07, 0xa1, 0x44, 0x03, 0x42, 0x00, 0x04,
  0x6b, 0x17, 0xd1, 0xf2, 0xe1, 0x2c, 0x42, 0x47, 0xf8, 0xbc, 0xe6, 0xe5, 0x63,
  0xa4, 0x40, 0xf2, 0x77, 0x03, 0x7d, 0x81, 0x2d, 0xeb, 0x33, 0xa0, 0xf4, 0xa1,
  0x39, 0x45, 0xd8, 0x98, 0xc2, 0x96, 0x4f, 0xe3, 0x42, 0xe2, 0xfe, 0x1a, 0x7f,
  0x9b, 0x8e, 0xe7, 0xeb, 0x4a, 0x7c, 0x0f, 0x9e, 0x16, 0x2b, 0xce, 0x33, 0x57,
  0x6b, 0x31, 0x5e, 0xce, 0xcb, 0xb6, 0x40, 0x68, 0x37, 0xbf, 0x51, 0xf5,
};

// kECKeyMissingZeros is an ECPrivateKey containing a degenerate P-256 key where
// the private key is one. The private key is encoded with the required zero
// padding.
static const uint8_t kECKeyWithZeros[] = {
  0x30, 0x77, 0x02, 0x01, 0x01, 0x04, 0x20, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
  0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
  0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01,
  0xa0, 0x0a, 0x06, 0x08, 0x2a, 0x86, 0x48, 0xce, 0x3d, 0x03, 0x01, 0x07, 0xa1,
  0x44, 0x03, 0x42, 0x00, 0x04, 0x6b, 0x17, 0xd1, 0xf2, 0xe1, 0x2c, 0x42, 0x47,
  0xf8, 0xbc, 0xe6, 0xe5, 0x63, 0xa4, 0x40, 0xf2, 0x77, 0x03, 0x7d, 0x81, 0x2d,
  0xeb, 0x33, 0xa0, 0xf4, 0xa1, 0x39, 0x45, 0xd8, 0x98, 0xc2, 0x96, 0x4f, 0xe3,
  0x42, 0xe2, 0xfe, 0x1a, 0x7f, 0x9b, 0x8e, 0xe7, 0xeb, 0x4a, 0x7c, 0x0f, 0x9e,
  0x16, 0x2b, 0xce, 0x33, 0x57, 0x6b, 0x31, 0x5e, 0xce, 0xcb, 0xb6, 0x40, 0x68,
  0x37, 0xbf, 0x51, 0xf5,
};

// DecodeECPrivateKey decodes |in| as an ECPrivateKey structure and returns the
// result or nullptr on error.
static bssl::UniquePtr<EC_KEY> DecodeECPrivateKey(const uint8_t *in,
                                                  size_t in_len) {
  CBS cbs;
  CBS_init(&cbs, in, in_len);
  bssl::UniquePtr<EC_KEY> ret(EC_KEY_parse_private_key(&cbs, NULL));
  if (!ret || CBS_len(&cbs) != 0) {
    return nullptr;
  }
  return ret;
}

// EncodeECPrivateKey encodes |key| as an ECPrivateKey structure into |*out|. It
// returns true on success or false on error.
static bool EncodeECPrivateKey(std::vector<uint8_t> *out, const EC_KEY *key) {
  bssl::ScopedCBB cbb;
  uint8_t *der;
  size_t der_len;
  if (!CBB_init(cbb.get(), 0) ||
      !EC_KEY_marshal_private_key(cbb.get(), key, EC_KEY_get_enc_flags(key)) ||
      !CBB_finish(cbb.get(), &der, &der_len)) {
    return false;
  }
  out->assign(der, der + der_len);
  OPENSSL_free(der);
  return true;
}

static bool EncodeECPoint(std::vector<uint8_t> *out, const EC_GROUP *group,
                          const EC_POINT *p, point_conversion_form_t form) {
  size_t len = EC_POINT_point2oct(group, p, form, nullptr, 0, nullptr);
  if (len == 0) {
    return false;
  }

  out->resize(len);
  len = EC_POINT_point2oct(group, p, form, out->data(), out->size(), nullptr);
  if (len != out->size()) {
    return false;
  }

  return true;
}

TEST(ECTest, Encoding) {
  bssl::UniquePtr<EC_KEY> key =
      DecodeECPrivateKey(kECKeyWithoutPublic, sizeof(kECKeyWithoutPublic));
  ASSERT_TRUE(key);

  // Test that the encoding round-trips.
  std::vector<uint8_t> out;
  ASSERT_TRUE(EncodeECPrivateKey(&out, key.get()));
  EXPECT_EQ(Bytes(kECKeyWithoutPublic), Bytes(out.data(), out.size()));

  const EC_POINT *pub_key = EC_KEY_get0_public_key(key.get());
  ASSERT_TRUE(pub_key) << "Public key missing";

  bssl::UniquePtr<BIGNUM> x(BN_new());
  bssl::UniquePtr<BIGNUM> y(BN_new());
  ASSERT_TRUE(x);
  ASSERT_TRUE(y);
  ASSERT_TRUE(EC_POINT_get_affine_coordinates_GFp(
      EC_KEY_get0_group(key.get()), pub_key, x.get(), y.get(), NULL));
  bssl::UniquePtr<char> x_hex(BN_bn2hex(x.get()));
  bssl::UniquePtr<char> y_hex(BN_bn2hex(y.get()));
  ASSERT_TRUE(x_hex);
  ASSERT_TRUE(y_hex);

  EXPECT_STREQ(
      "C81561ECF2E54EDEFE6617DB1C7A34A70744DDB261F269B83DACFCD2ADE5A681",
      x_hex.get());
  EXPECT_STREQ(
      "E0E2AFA3F9B6ABE4C698EF6495F1BE49A3196C5056ACB3763FE4507EEC596E88",
      y_hex.get());
}

// P-{224,256,384,521} test vectors, taken from CAVP
// (CAVP 20.1 - KASValidityTest_ECCStaticUnified_KDFConcat_NOKC)
// https://csrc.nist.gov/projects/cryptographic-algorithm-validation-program/key-management

static const uint8_t kP224PublicKey_uncompressed_0x02[] = {
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

static const uint8_t kP224PublicKey_compressed_0x02[] = {
  0x02,
  /* x-coordinate */
  0xd6, 0xf5, 0xf0, 0x6e, 0xf4, 0xc5, 0x56, 0x0a, 0xff, 0x8f, 0x49, 0x90,
  0xef, 0xdb, 0xa5, 0x9a, 0xf8, 0xa8, 0xd3, 0x77, 0x0d, 0x80, 0x14, 0x6a,
  0xc5, 0x82, 0x78, 0x85
};

static const uint8_t kP224PublicKey_hybrid_0x02[] = {
  /* uncompressed */
  0x06,
  /* x-coordinate */
  0xd6, 0xf5, 0xf0, 0x6e, 0xf4, 0xc5, 0x56, 0x0a, 0xff, 0x8f, 0x49, 0x90,
  0xef, 0xdb, 0xa5, 0x9a, 0xf8, 0xa8, 0xd3, 0x77, 0x0d, 0x80, 0x14, 0x6a,
  0xc5, 0x82, 0x78, 0x85,
  /* y-coordinate */
  0xe0, 0x43, 0xae, 0x7b, 0xae, 0xa3, 0x77, 0x28, 0x60, 0x39, 0xc0, 0x7c,
  0x04, 0x1b, 0x7a, 0x3b, 0x5d, 0x76, 0x96, 0xda, 0xdd, 0xa7, 0x05, 0x1a,
  0xd6, 0x45, 0xa3, 0xea
};

static const uint8_t kP224PublicKey_uncompressed_0x03[] = {
  /* uncompressed */
  0x04,
  /* x-coordinate */
  0xd6, 0xf5, 0xf0, 0x6e, 0xf4, 0xc5, 0x56, 0x0a, 0xff, 0x8f, 0x49, 0x90,
  0xef, 0xdb, 0xa5, 0x9a, 0xf8, 0xa8, 0xd3, 0x77, 0x0d, 0x80, 0x14, 0x6a,
  0xc5, 0x82, 0x78, 0x85,
  /* y-coordinate */
  0x1f, 0xbc, 0x51, 0x84, 0x51, 0x5c, 0x88, 0xd7, 0x9f, 0xc6, 0x3f, 0x83,
  0xfb, 0xe4, 0x85, 0xc3, 0xa2, 0x89, 0x69, 0x25, 0x22, 0x58, 0xfa, 0xe5,
  0x29, 0xba, 0x5c, 0x17
};

static const uint8_t kP224PublicKey_compressed_0x03[] = {
  0x03,
  /* x-coordinate */
  0xd6, 0xf5, 0xf0, 0x6e, 0xf4, 0xc5, 0x56, 0x0a, 0xff, 0x8f, 0x49, 0x90,
  0xef, 0xdb, 0xa5, 0x9a, 0xf8, 0xa8, 0xd3, 0x77, 0x0d, 0x80, 0x14, 0x6a,
  0xc5, 0x82, 0x78, 0x85
};

static const uint8_t kP224PublicKey_hybrid_0x03[] = {
  /* uncompressed */
  0x07,
  /* x-coordinate */
  0xd6, 0xf5, 0xf0, 0x6e, 0xf4, 0xc5, 0x56, 0x0a, 0xff, 0x8f, 0x49, 0x90,
  0xef, 0xdb, 0xa5, 0x9a, 0xf8, 0xa8, 0xd3, 0x77, 0x0d, 0x80, 0x14, 0x6a,
  0xc5, 0x82, 0x78, 0x85,
  /* y-coordinate */
  0x1f, 0xbc, 0x51, 0x84, 0x51, 0x5c, 0x88, 0xd7, 0x9f, 0xc6, 0x3f, 0x83,
  0xfb, 0xe4, 0x85, 0xc3, 0xa2, 0x89, 0x69, 0x25, 0x22, 0x58, 0xfa, 0xe5,
  0x29, 0xba, 0x5c, 0x17
};

static const uint8_t kP256PublicKey_uncompressed_0x02[] = {
  /* uncompressed */
  0x04,
  /* x-coordinate */
  0xe1, 0x5a, 0x44, 0x72, 0x91, 0xf0, 0x84, 0xfe, 0x88, 0x7a, 0x6c, 0x2c,
  0x03, 0x22, 0x9a, 0xf3, 0x04, 0x8a, 0x5d, 0xfe, 0x84, 0x73, 0x70, 0xc9,
  0x3f, 0x92, 0x72, 0x9b, 0x31, 0xc5, 0x5f, 0x7b,
  /* y-coordinate */
  0xc9, 0x53, 0x67, 0xc0, 0xd2, 0x90, 0x46, 0x86, 0x61, 0x8b, 0xf6, 0xf2,
  0xd9, 0x0b, 0x7c, 0xcb, 0x31, 0xb0, 0xb4, 0x8c, 0x60, 0xc0, 0x28, 0x55,
  0x6d, 0x1d, 0x3a, 0xbf, 0xdc, 0xd3, 0x1e, 0x42
};

static const uint8_t kP256PublicKey_compressed_0x02[] = {
  0x02,
  /* x-coordinate */
  0xe1, 0x5a, 0x44, 0x72, 0x91, 0xf0, 0x84, 0xfe, 0x88, 0x7a, 0x6c, 0x2c,
  0x03, 0x22, 0x9a, 0xf3, 0x04, 0x8a, 0x5d, 0xfe, 0x84, 0x73, 0x70, 0xc9,
  0x3f, 0x92, 0x72, 0x9b, 0x31, 0xc5, 0x5f, 0x7b
};

static const uint8_t kP256PublicKey_hybrid_0x02[] = {
  /* uncompressed */
  0x06,
  /* x-coordinate */
  0xe1, 0x5a, 0x44, 0x72, 0x91, 0xf0, 0x84, 0xfe, 0x88, 0x7a, 0x6c, 0x2c,
  0x03, 0x22, 0x9a, 0xf3, 0x04, 0x8a, 0x5d, 0xfe, 0x84, 0x73, 0x70, 0xc9,
  0x3f, 0x92, 0x72, 0x9b, 0x31, 0xc5, 0x5f, 0x7b,
  /* y-coordinate */
  0xc9, 0x53, 0x67, 0xc0, 0xd2, 0x90, 0x46, 0x86, 0x61, 0x8b, 0xf6, 0xf2,
  0xd9, 0x0b, 0x7c, 0xcb, 0x31, 0xb0, 0xb4, 0x8c, 0x60, 0xc0, 0x28, 0x55,
  0x6d, 0x1d, 0x3a, 0xbf, 0xdc, 0xd3, 0x1e, 0x42
};

static const uint8_t kP256PublicKey_uncompressed_0x03[] = {
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

static const uint8_t kP256PublicKey_compressed_0x03[] = {
  0x03,
  /* x-coordinate */
  0xe1, 0x5a, 0x44, 0x72, 0x91, 0xf0, 0x84, 0xfe, 0x88, 0x7a, 0x6c, 0x2c,
  0x03, 0x22, 0x9a, 0xf3, 0x04, 0x8a, 0x5d, 0xfe, 0x84, 0x73, 0x70, 0xc9,
  0x3f, 0x92, 0x72, 0x9b, 0x31, 0xc5, 0x5f, 0x7b
};

static const uint8_t kP256PublicKey_hybrid_0x03[] = {
  /* uncompressed */
  0x07,
  /* x-coordinate */
  0xe1, 0x5a, 0x44, 0x72, 0x91, 0xf0, 0x84, 0xfe, 0x88, 0x7a, 0x6c, 0x2c,
  0x03, 0x22, 0x9a, 0xf3, 0x04, 0x8a, 0x5d, 0xfe, 0x84, 0x73, 0x70, 0xc9,
  0x3f, 0x92, 0x72, 0x9b, 0x31, 0xc5, 0x5f, 0x7b,
  /* y-coordinate */
  0x36, 0xac, 0x98, 0x3e, 0x2d, 0x6f, 0xb9, 0x7a, 0x9e, 0x74, 0x09, 0x0d,
  0x26, 0xf4, 0x83, 0x34, 0xce, 0x4f, 0x4b, 0x74, 0x9f, 0x3f, 0xd7, 0xaa,
  0x92, 0xe2, 0xc5, 0x40, 0x23, 0x2c, 0xe1, 0xbd
};

static const uint8_t kP384PublicKey_uncompressed_0x02[] = {
  /* uncompressed */
  0x04,
  /* x-coordinate */
  0xe4, 0xe7, 0x0e, 0x43, 0xc6, 0xd0, 0x43, 0x46, 0xdd, 0xd7, 0x62, 0xa6,
  0x14, 0x17, 0x6d, 0x22, 0x78, 0xb0, 0x47, 0xc5, 0xec, 0x28, 0x64, 0x84,
  0x65, 0xf2, 0xa3, 0x90, 0xf6, 0xdd, 0x6b, 0xba, 0x54, 0xb9, 0x0b, 0x1e,
  0x62, 0xb3, 0x91, 0x85, 0xf8, 0xf3, 0x95, 0xf6, 0x65, 0x73, 0x6d, 0x1d,
  /* y-coordinate */
  0x06, 0x9d, 0x5d, 0x8c, 0x95, 0x31, 0xad, 0xa9, 0xe7, 0xea, 0x2a, 0x66,
  0xac, 0x5f, 0xe6, 0xe4, 0xe0, 0x4e, 0x0d, 0x77, 0x5b, 0xa0, 0x71, 0xd7,
  0xc2, 0xbf, 0x5a, 0x00, 0xf1, 0x7c, 0xc0, 0x0b, 0xf4, 0x29, 0xfa, 0x4d,
  0xf3, 0x07, 0x3d, 0x93, 0xa8, 0xb2, 0xb3, 0xd1, 0xf2, 0x32, 0x31, 0xde
};

static const uint8_t kP384PublicKey_compressed_0x02[] = {
  0x02,
  /* x-coordinate */
  0xe4, 0xe7, 0x0e, 0x43, 0xc6, 0xd0, 0x43, 0x46, 0xdd, 0xd7, 0x62, 0xa6,
  0x14, 0x17, 0x6d, 0x22, 0x78, 0xb0, 0x47, 0xc5, 0xec, 0x28, 0x64, 0x84,
  0x65, 0xf2, 0xa3, 0x90, 0xf6, 0xdd, 0x6b, 0xba, 0x54, 0xb9, 0x0b, 0x1e,
  0x62, 0xb3, 0x91, 0x85, 0xf8, 0xf3, 0x95, 0xf6, 0x65, 0x73, 0x6d, 0x1d
};

static const uint8_t kP384PublicKey_hybrid_0x02[] = {
  /* uncompressed */
  0x06,
  /* x-coordinate */
  0xe4, 0xe7, 0x0e, 0x43, 0xc6, 0xd0, 0x43, 0x46, 0xdd, 0xd7, 0x62, 0xa6,
  0x14, 0x17, 0x6d, 0x22, 0x78, 0xb0, 0x47, 0xc5, 0xec, 0x28, 0x64, 0x84,
  0x65, 0xf2, 0xa3, 0x90, 0xf6, 0xdd, 0x6b, 0xba, 0x54, 0xb9, 0x0b, 0x1e,
  0x62, 0xb3, 0x91, 0x85, 0xf8, 0xf3, 0x95, 0xf6, 0x65, 0x73, 0x6d, 0x1d,
  /* y-coordinate */
  0x06, 0x9d, 0x5d, 0x8c, 0x95, 0x31, 0xad, 0xa9, 0xe7, 0xea, 0x2a, 0x66,
  0xac, 0x5f, 0xe6, 0xe4, 0xe0, 0x4e, 0x0d, 0x77, 0x5b, 0xa0, 0x71, 0xd7,
  0xc2, 0xbf, 0x5a, 0x00, 0xf1, 0x7c, 0xc0, 0x0b, 0xf4, 0x29, 0xfa, 0x4d,
  0xf3, 0x07, 0x3d, 0x93, 0xa8, 0xb2, 0xb3, 0xd1, 0xf2, 0x32, 0x31, 0xde
};

static const uint8_t kP384PublicKey_uncompressed_0x03[] = {
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

static const uint8_t kP384PublicKey_compressed_0x03[] = {
  0x03,
  /* x-coordinate */
  0xe4, 0xe7, 0x0e, 0x43, 0xc6, 0xd0, 0x43, 0x46, 0xdd, 0xd7, 0x62, 0xa6,
  0x14, 0x17, 0x6d, 0x22, 0x78, 0xb0, 0x47, 0xc5, 0xec, 0x28, 0x64, 0x84,
  0x65, 0xf2, 0xa3, 0x90, 0xf6, 0xdd, 0x6b, 0xba, 0x54, 0xb9, 0x0b, 0x1e,
  0x62, 0xb3, 0x91, 0x85, 0xf8, 0xf3, 0x95, 0xf6, 0x65, 0x73, 0x6d, 0x1d
};

static const uint8_t kP384PublicKey_hybrid_0x03[] = {
  /* uncompressed */
  0x07,
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

static const uint8_t kP521PublicKey_uncompressed_0x02[] = {
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
  0x00, 0xe4, 0x45, 0x33, 0xe8, 0x7f, 0xa9, 0x74, 0x64, 0xcd, 0x2b, 0x7d,
  0xc0, 0xcd, 0x65, 0xb9, 0x27, 0xc6, 0xc6, 0x2e, 0xe7, 0x33, 0x68, 0x86,
  0x72, 0xa2, 0x05, 0xf7, 0x4b, 0xd8, 0x2c, 0x51, 0x1b, 0x89, 0xb0, 0xb9,
  0xb8, 0x06, 0x0d, 0xb1, 0x30, 0xf0, 0x11, 0x92, 0x9e, 0x63, 0x86, 0x8c,
  0x57, 0xaa, 0xb5, 0x2a, 0xae, 0xec, 0xf2, 0xe1, 0xc0, 0x93, 0x62, 0xd1,
  0x1c, 0x5d, 0x57, 0x90, 0x0a, 0x3c
};

static const uint8_t kP521PublicKey_compressed_0x02[] = {
  0x02,
  /* x-coordinate */
  0x01, 0x03, 0x7e, 0x95, 0xff, 0x8e, 0x40, 0x31, 0xe0, 0xb0, 0x36, 0x1c,
  0x58, 0xc0, 0x62, 0x61, 0x39, 0x56, 0xaa, 0x30, 0x77, 0x0c, 0xed, 0x17,
  0x15, 0xed, 0x1b, 0x4d, 0x34, 0x29, 0x33, 0x0f, 0xac, 0x2f, 0xc5, 0xc9,
  0x3a, 0x69, 0xf7, 0x98, 0x63, 0x3a, 0x15, 0x75, 0x5e, 0x2d, 0xb8, 0x65,
  0x09, 0x87, 0xf5, 0x75, 0x85, 0xcd, 0xe3, 0x51, 0x6b, 0x6d, 0xd0, 0xfc,
  0x9f, 0x5f, 0xb4, 0xf8, 0xe7, 0x7b
};

static const uint8_t kP521PublicKey_hybrid_0x02[] = {
  /* uncompressed */
  0x06,
  /* x-coordinate */
  0x01, 0x03, 0x7e, 0x95, 0xff, 0x8e, 0x40, 0x31, 0xe0, 0xb0, 0x36, 0x1c,
  0x58, 0xc0, 0x62, 0x61, 0x39, 0x56, 0xaa, 0x30, 0x77, 0x0c, 0xed, 0x17,
  0x15, 0xed, 0x1b, 0x4d, 0x34, 0x29, 0x33, 0x0f, 0xac, 0x2f, 0xc5, 0xc9,
  0x3a, 0x69, 0xf7, 0x98, 0x63, 0x3a, 0x15, 0x75, 0x5e, 0x2d, 0xb8, 0x65,
  0x09, 0x87, 0xf5, 0x75, 0x85, 0xcd, 0xe3, 0x51, 0x6b, 0x6d, 0xd0, 0xfc,
  0x9f, 0x5f, 0xb4, 0xf8, 0xe7, 0x7b,
  /* y-coordinate */
  0x00, 0xe4, 0x45, 0x33, 0xe8, 0x7f, 0xa9, 0x74, 0x64, 0xcd, 0x2b, 0x7d,
  0xc0, 0xcd, 0x65, 0xb9, 0x27, 0xc6, 0xc6, 0x2e, 0xe7, 0x33, 0x68, 0x86,
  0x72, 0xa2, 0x05, 0xf7, 0x4b, 0xd8, 0x2c, 0x51, 0x1b, 0x89, 0xb0, 0xb9,
  0xb8, 0x06, 0x0d, 0xb1, 0x30, 0xf0, 0x11, 0x92, 0x9e, 0x63, 0x86, 0x8c,
  0x57, 0xaa, 0xb5, 0x2a, 0xae, 0xec, 0xf2, 0xe1, 0xc0, 0x93, 0x62, 0xd1,
  0x1c, 0x5d, 0x57, 0x90, 0x0a, 0x3c
};

static const uint8_t kP521PublicKey_uncompressed_0x03[] = {
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

static const uint8_t kP521PublicKey_compressed_0x03[] = {
  0x03,
  /* x-coordinate */
  0x01, 0x03, 0x7e, 0x95, 0xff, 0x8e, 0x40, 0x31, 0xe0, 0xb0, 0x36, 0x1c,
  0x58, 0xc0, 0x62, 0x61, 0x39, 0x56, 0xaa, 0x30, 0x77, 0x0c, 0xed, 0x17,
  0x15, 0xed, 0x1b, 0x4d, 0x34, 0x29, 0x33, 0x0f, 0xac, 0x2f, 0xc5, 0xc9,
  0x3a, 0x69, 0xf7, 0x98, 0x63, 0x3a, 0x15, 0x75, 0x5e, 0x2d, 0xb8, 0x65,
  0x09, 0x87, 0xf5, 0x75, 0x85, 0xcd, 0xe3, 0x51, 0x6b, 0x6d, 0xd0, 0xfc,
  0x9f, 0x5f, 0xb4, 0xf8, 0xe7, 0x7b
};

static const uint8_t kP521PublicKey_hybrid_0x03[] = {
  /* uncompressed */
  0x07,
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

static const uint8_t ksecp256k1PublicKey_uncompressed_0x02[] = {
  /* uncompressed */
  0x04,
  /* x-coordinate */
  0xc5, 0xea, 0xe6, 0x37, 0xf3, 0xbd, 0x76, 0xad, 0x09, 0x64, 0x54, 0x9d,
  0x52, 0xa6, 0x00, 0x46, 0x7e, 0xdb, 0x30, 0x3d, 0x9c, 0x32, 0xa8, 0xab,
  0x12, 0xd0, 0xed, 0x0a, 0x88, 0x67, 0x59, 0x0b,
  /* y-coordinate */
  0xfc, 0x97, 0x38, 0x6b, 0xc9, 0x8f, 0xf5, 0xfc, 0x2d, 0xa5, 0x77, 0x96,
  0x62, 0xd2, 0x72, 0x69, 0x6a, 0xd2, 0xac, 0xa3, 0x7b, 0x4d, 0x5c, 0x84,
  0x6c, 0xa4, 0x2c, 0xec, 0xb2, 0x4c, 0x3d, 0x94
};

static const uint8_t ksecp256k1PublicKey_compressed_0x02[] = {
  0x02,
  /* x-coordinate */
  0xc5, 0xea, 0xe6, 0x37, 0xf3, 0xbd, 0x76, 0xad, 0x09, 0x64, 0x54, 0x9d,
  0x52, 0xa6, 0x00, 0x46, 0x7e, 0xdb, 0x30, 0x3d, 0x9c, 0x32, 0xa8, 0xab,
  0x12, 0xd0, 0xed, 0x0a, 0x88, 0x67, 0x59, 0x0b
};

static const uint8_t ksecp256k1PublicKey_uncompressed_0x03[] = {
  /* uncompressed */
  0x04,
  /* x-coordinate */
  0xad, 0xa8, 0x37, 0xe6, 0x83, 0x94, 0x67, 0xbf, 0x79, 0xa8, 0xa8, 0x3b,
  0x17, 0x3d, 0x4a, 0x56, 0x07, 0xa0, 0x57, 0x66, 0x19, 0xc6, 0x67, 0x56,
  0xa2, 0x48, 0x8c, 0x6d, 0xff, 0xda, 0xf2, 0xa9,
  /* y-coordinate */
  0x50, 0xd1, 0x4b, 0xff, 0x7a, 0x83, 0xb7, 0x02, 0x4c, 0xeb, 0x29, 0x2e,
  0xc8, 0x32, 0xa0, 0x16, 0xc5, 0x83, 0x74, 0x80, 0x1a, 0xf6, 0xc8, 0xb8,
  0xb8, 0x1d, 0x6a, 0xa6, 0xdc, 0xae, 0xfe, 0x63
};

static const uint8_t ksecp256k1PublicKey_compressed_0x03[] = {
  0x03,
  /* x-coordinate */
  0xad, 0xa8, 0x37, 0xe6, 0x83, 0x94, 0x67, 0xbf, 0x79, 0xa8, 0xa8, 0x3b,
  0x17, 0x3d, 0x4a, 0x56, 0x07, 0xa0, 0x57, 0x66, 0x19, 0xc6, 0x67, 0x56,
  0xa2, 0x48, 0x8c, 0x6d, 0xff, 0xda, 0xf2, 0xa9
};

struct ECPublicKeyTestInput {
  const uint8_t *input_key;
  size_t input_key_len;
  point_conversion_form_t encode_conv_form;
  const uint8_t *expected_output_key;
  size_t expected_output_key_len;
  int nid;
} kDecodeAndEncodeInputs[] = {
    // Test 1: decode uncompressed |EC_KEY|, and then encode with the same
    // |conv_form|.
    {kP224PublicKey_uncompressed_0x02, sizeof(kP224PublicKey_uncompressed_0x02),
     POINT_CONVERSION_UNCOMPRESSED, kP224PublicKey_uncompressed_0x02,
     sizeof(kP224PublicKey_uncompressed_0x02), NID_secp224r1},
    {kP256PublicKey_uncompressed_0x02, sizeof(kP256PublicKey_uncompressed_0x02),
     POINT_CONVERSION_UNCOMPRESSED, kP256PublicKey_uncompressed_0x02,
     sizeof(kP256PublicKey_uncompressed_0x02), NID_X9_62_prime256v1},
    {kP384PublicKey_uncompressed_0x02, sizeof(kP384PublicKey_uncompressed_0x02),
     POINT_CONVERSION_UNCOMPRESSED, kP384PublicKey_uncompressed_0x02,
     sizeof(kP384PublicKey_uncompressed_0x02), NID_secp384r1},
    {kP521PublicKey_uncompressed_0x02, sizeof(kP521PublicKey_uncompressed_0x02),
     POINT_CONVERSION_UNCOMPRESSED, kP521PublicKey_uncompressed_0x02,
     sizeof(kP521PublicKey_uncompressed_0x02), NID_secp521r1},
    {ksecp256k1PublicKey_uncompressed_0x02,
     sizeof(ksecp256k1PublicKey_uncompressed_0x02),
     POINT_CONVERSION_UNCOMPRESSED, ksecp256k1PublicKey_uncompressed_0x02,
     sizeof(ksecp256k1PublicKey_uncompressed_0x02), NID_secp256k1},
    {kP224PublicKey_uncompressed_0x03, sizeof(kP224PublicKey_uncompressed_0x03),
     POINT_CONVERSION_UNCOMPRESSED, kP224PublicKey_uncompressed_0x03,
     sizeof(kP224PublicKey_uncompressed_0x03), NID_secp224r1},
    {kP256PublicKey_uncompressed_0x03, sizeof(kP256PublicKey_uncompressed_0x03),
     POINT_CONVERSION_UNCOMPRESSED, kP256PublicKey_uncompressed_0x03,
     sizeof(kP256PublicKey_uncompressed_0x03), NID_X9_62_prime256v1},
    {kP384PublicKey_uncompressed_0x03, sizeof(kP384PublicKey_uncompressed_0x03),
     POINT_CONVERSION_UNCOMPRESSED, kP384PublicKey_uncompressed_0x03,
     sizeof(kP384PublicKey_uncompressed_0x03), NID_secp384r1},
    {kP521PublicKey_uncompressed_0x03, sizeof(kP521PublicKey_uncompressed_0x03),
     POINT_CONVERSION_UNCOMPRESSED, kP521PublicKey_uncompressed_0x03,
     sizeof(kP521PublicKey_uncompressed_0x03), NID_secp521r1},
    {ksecp256k1PublicKey_uncompressed_0x03,
     sizeof(ksecp256k1PublicKey_uncompressed_0x03),
     POINT_CONVERSION_UNCOMPRESSED, ksecp256k1PublicKey_uncompressed_0x03,
     sizeof(ksecp256k1PublicKey_uncompressed_0x03), NID_secp256k1},
    // Test 2: decode compressed |EC_KEY|, and then encode with the same
    // |conv_form|.
    {kP224PublicKey_compressed_0x02, sizeof(kP224PublicKey_compressed_0x02),
     POINT_CONVERSION_COMPRESSED, kP224PublicKey_compressed_0x02,
     sizeof(kP224PublicKey_compressed_0x02), NID_secp224r1},
    {kP256PublicKey_compressed_0x02, sizeof(kP256PublicKey_compressed_0x02),
     POINT_CONVERSION_COMPRESSED, kP256PublicKey_compressed_0x02,
     sizeof(kP256PublicKey_compressed_0x02), NID_X9_62_prime256v1},
    {kP384PublicKey_compressed_0x02, sizeof(kP384PublicKey_compressed_0x02),
     POINT_CONVERSION_COMPRESSED, kP384PublicKey_compressed_0x02,
     sizeof(kP384PublicKey_compressed_0x02), NID_secp384r1},
    {kP521PublicKey_compressed_0x02, sizeof(kP521PublicKey_compressed_0x02),
     POINT_CONVERSION_COMPRESSED, kP521PublicKey_compressed_0x02,
     sizeof(kP521PublicKey_compressed_0x02), NID_secp521r1},
    {ksecp256k1PublicKey_compressed_0x02,
     sizeof(ksecp256k1PublicKey_compressed_0x02), POINT_CONVERSION_COMPRESSED,
     ksecp256k1PublicKey_compressed_0x02,
     sizeof(ksecp256k1PublicKey_compressed_0x02), NID_secp256k1},
    {kP224PublicKey_compressed_0x03, sizeof(kP224PublicKey_compressed_0x03),
     POINT_CONVERSION_COMPRESSED, kP224PublicKey_compressed_0x03,
     sizeof(kP224PublicKey_compressed_0x03), NID_secp224r1},
    {kP256PublicKey_compressed_0x03, sizeof(kP256PublicKey_compressed_0x03),
     POINT_CONVERSION_COMPRESSED, kP256PublicKey_compressed_0x03,
     sizeof(kP256PublicKey_compressed_0x03), NID_X9_62_prime256v1},
    {kP384PublicKey_compressed_0x03, sizeof(kP384PublicKey_compressed_0x03),
     POINT_CONVERSION_COMPRESSED, kP384PublicKey_compressed_0x03,
     sizeof(kP384PublicKey_compressed_0x03), NID_secp384r1},
    {kP521PublicKey_compressed_0x03, sizeof(kP521PublicKey_compressed_0x03),
     POINT_CONVERSION_COMPRESSED, kP521PublicKey_compressed_0x03,
     sizeof(kP521PublicKey_compressed_0x03), NID_secp521r1},
    {ksecp256k1PublicKey_compressed_0x03,
     sizeof(ksecp256k1PublicKey_compressed_0x03), POINT_CONVERSION_COMPRESSED,
     ksecp256k1PublicKey_compressed_0x03,
     sizeof(ksecp256k1PublicKey_compressed_0x03), NID_secp256k1},
    // Test 3: decode compressed |EC_KEY|, and then encode with uncompressed
    // |conv_form|.
    {kP224PublicKey_compressed_0x02, sizeof(kP224PublicKey_compressed_0x02),
     POINT_CONVERSION_UNCOMPRESSED, kP224PublicKey_uncompressed_0x02,
     sizeof(kP224PublicKey_uncompressed_0x02), NID_secp224r1},
    {kP256PublicKey_compressed_0x02, sizeof(kP256PublicKey_compressed_0x02),
     POINT_CONVERSION_UNCOMPRESSED, kP256PublicKey_uncompressed_0x02,
     sizeof(kP256PublicKey_uncompressed_0x02), NID_X9_62_prime256v1},
    {kP384PublicKey_compressed_0x02, sizeof(kP384PublicKey_compressed_0x02),
     POINT_CONVERSION_UNCOMPRESSED, kP384PublicKey_uncompressed_0x02,
     sizeof(kP384PublicKey_uncompressed_0x02), NID_secp384r1},
    {kP521PublicKey_compressed_0x02, sizeof(kP521PublicKey_compressed_0x02),
     POINT_CONVERSION_UNCOMPRESSED, kP521PublicKey_uncompressed_0x02,
     sizeof(kP521PublicKey_uncompressed_0x02), NID_secp521r1},
    {ksecp256k1PublicKey_compressed_0x02,
     sizeof(ksecp256k1PublicKey_compressed_0x02), POINT_CONVERSION_UNCOMPRESSED,
     ksecp256k1PublicKey_uncompressed_0x02,
     sizeof(ksecp256k1PublicKey_uncompressed_0x02), NID_secp256k1},
    {kP224PublicKey_compressed_0x03, sizeof(kP224PublicKey_compressed_0x03),
     POINT_CONVERSION_UNCOMPRESSED, kP224PublicKey_uncompressed_0x03,
     sizeof(kP224PublicKey_uncompressed_0x03), NID_secp224r1},
    {kP256PublicKey_compressed_0x03, sizeof(kP256PublicKey_compressed_0x03),
     POINT_CONVERSION_UNCOMPRESSED, kP256PublicKey_uncompressed_0x03,
     sizeof(kP256PublicKey_uncompressed_0x03), NID_X9_62_prime256v1},
    {kP384PublicKey_compressed_0x03, sizeof(kP384PublicKey_compressed_0x03),
     POINT_CONVERSION_UNCOMPRESSED, kP384PublicKey_uncompressed_0x03,
     sizeof(kP384PublicKey_uncompressed_0x03), NID_secp384r1},
    {kP521PublicKey_compressed_0x03, sizeof(kP521PublicKey_compressed_0x03),
     POINT_CONVERSION_UNCOMPRESSED, kP521PublicKey_uncompressed_0x03,
     sizeof(kP521PublicKey_uncompressed_0x03), NID_secp521r1},
    {ksecp256k1PublicKey_compressed_0x03,
     sizeof(ksecp256k1PublicKey_compressed_0x03), POINT_CONVERSION_UNCOMPRESSED,
     ksecp256k1PublicKey_uncompressed_0x03,
     sizeof(ksecp256k1PublicKey_uncompressed_0x03), NID_secp256k1},
    // Test 4: decode uncompressed |EC_KEY|, and then encode with compressed
    // |conv_form|.
    {kP224PublicKey_uncompressed_0x02, sizeof(kP224PublicKey_uncompressed_0x02),
     POINT_CONVERSION_COMPRESSED, kP224PublicKey_compressed_0x02,
     sizeof(kP224PublicKey_compressed_0x02), NID_secp224r1},
    {kP256PublicKey_uncompressed_0x02, sizeof(kP256PublicKey_uncompressed_0x02),
     POINT_CONVERSION_COMPRESSED, kP256PublicKey_compressed_0x02,
     sizeof(kP256PublicKey_compressed_0x02), NID_X9_62_prime256v1},
    {kP384PublicKey_uncompressed_0x02, sizeof(kP384PublicKey_uncompressed_0x02),
     POINT_CONVERSION_COMPRESSED, kP384PublicKey_compressed_0x02,
     sizeof(kP384PublicKey_compressed_0x02), NID_secp384r1},
    {kP521PublicKey_uncompressed_0x02, sizeof(kP521PublicKey_uncompressed_0x02),
     POINT_CONVERSION_COMPRESSED, kP521PublicKey_compressed_0x02,
     sizeof(kP521PublicKey_compressed_0x02), NID_secp521r1},
    {ksecp256k1PublicKey_uncompressed_0x02,
     sizeof(ksecp256k1PublicKey_uncompressed_0x02), POINT_CONVERSION_COMPRESSED,
     ksecp256k1PublicKey_compressed_0x02,
     sizeof(ksecp256k1PublicKey_compressed_0x02), NID_secp256k1},
    {kP224PublicKey_uncompressed_0x03, sizeof(kP224PublicKey_uncompressed_0x03),
     POINT_CONVERSION_COMPRESSED, kP224PublicKey_compressed_0x03,
     sizeof(kP224PublicKey_compressed_0x03), NID_secp224r1},
    {kP256PublicKey_uncompressed_0x03, sizeof(kP256PublicKey_uncompressed_0x03),
     POINT_CONVERSION_COMPRESSED, kP256PublicKey_compressed_0x03,
     sizeof(kP256PublicKey_compressed_0x03), NID_X9_62_prime256v1},
    {kP384PublicKey_uncompressed_0x03, sizeof(kP384PublicKey_uncompressed_0x03),
     POINT_CONVERSION_COMPRESSED, kP384PublicKey_compressed_0x03,
     sizeof(kP384PublicKey_compressed_0x03), NID_secp384r1},
    {kP521PublicKey_uncompressed_0x03, sizeof(kP521PublicKey_uncompressed_0x03),
     POINT_CONVERSION_COMPRESSED, kP521PublicKey_compressed_0x03,
     sizeof(kP521PublicKey_compressed_0x03), NID_secp521r1},
    {ksecp256k1PublicKey_uncompressed_0x03,
     sizeof(ksecp256k1PublicKey_uncompressed_0x03), POINT_CONVERSION_COMPRESSED,
     ksecp256k1PublicKey_compressed_0x03,
     sizeof(ksecp256k1PublicKey_compressed_0x03), NID_secp256k1},
    // Test 5: decode uncompressed |EC_KEY|, and then encode with
    // |POINT_CONVERSION_HYBRID|.
    {kP224PublicKey_uncompressed_0x02, sizeof(kP224PublicKey_uncompressed_0x02),
     POINT_CONVERSION_HYBRID, kP224PublicKey_hybrid_0x02,
     sizeof(kP224PublicKey_hybrid_0x02), NID_secp224r1},
    {kP224PublicKey_uncompressed_0x03, sizeof(kP224PublicKey_uncompressed_0x03),
     POINT_CONVERSION_HYBRID, kP224PublicKey_hybrid_0x03,
     sizeof(kP224PublicKey_hybrid_0x03), NID_secp224r1},
    {kP256PublicKey_uncompressed_0x02, sizeof(kP256PublicKey_uncompressed_0x02),
     POINT_CONVERSION_HYBRID, kP256PublicKey_hybrid_0x02,
     sizeof(kP256PublicKey_hybrid_0x02), NID_X9_62_prime256v1},
    {kP256PublicKey_uncompressed_0x03, sizeof(kP256PublicKey_uncompressed_0x03),
     POINT_CONVERSION_HYBRID, kP256PublicKey_hybrid_0x03,
     sizeof(kP256PublicKey_hybrid_0x03), NID_X9_62_prime256v1},
    {kP384PublicKey_uncompressed_0x02, sizeof(kP384PublicKey_uncompressed_0x02),
     POINT_CONVERSION_HYBRID, kP384PublicKey_hybrid_0x02,
     sizeof(kP384PublicKey_hybrid_0x02), NID_secp384r1},
    {kP384PublicKey_uncompressed_0x03, sizeof(kP384PublicKey_uncompressed_0x03),
     POINT_CONVERSION_HYBRID, kP384PublicKey_hybrid_0x03,
     sizeof(kP384PublicKey_hybrid_0x03), NID_secp384r1},
    {kP521PublicKey_uncompressed_0x02, sizeof(kP521PublicKey_uncompressed_0x02),
     POINT_CONVERSION_HYBRID, kP521PublicKey_hybrid_0x02,
     sizeof(kP521PublicKey_hybrid_0x02), NID_secp521r1},
    {kP521PublicKey_uncompressed_0x03, sizeof(kP521PublicKey_uncompressed_0x03),
     POINT_CONVERSION_HYBRID, kP521PublicKey_hybrid_0x03,
     sizeof(kP521PublicKey_hybrid_0x03), NID_secp521r1},
    // Test 5: decode hybrid |EC_KEY|, and then encode with
    // |POINT_CONVERSION_UNCOMPRESSED|.
    {kP224PublicKey_hybrid_0x02, sizeof(kP224PublicKey_hybrid_0x02),
     POINT_CONVERSION_UNCOMPRESSED, kP224PublicKey_uncompressed_0x02,
     sizeof(kP224PublicKey_uncompressed_0x02), NID_secp224r1},
    {kP224PublicKey_hybrid_0x03, sizeof(kP224PublicKey_hybrid_0x03),
     POINT_CONVERSION_UNCOMPRESSED, kP224PublicKey_uncompressed_0x03,
     sizeof(kP224PublicKey_uncompressed_0x03), NID_secp224r1},
    {kP256PublicKey_hybrid_0x02, sizeof(kP256PublicKey_hybrid_0x02),
     POINT_CONVERSION_UNCOMPRESSED, kP256PublicKey_uncompressed_0x02,
     sizeof(kP256PublicKey_uncompressed_0x02), NID_X9_62_prime256v1},
    {kP256PublicKey_hybrid_0x03, sizeof(kP256PublicKey_hybrid_0x03),
     POINT_CONVERSION_UNCOMPRESSED, kP256PublicKey_uncompressed_0x03,
     sizeof(kP256PublicKey_uncompressed_0x03), NID_X9_62_prime256v1},
    {kP384PublicKey_hybrid_0x02, sizeof(kP384PublicKey_hybrid_0x02),
     POINT_CONVERSION_UNCOMPRESSED, kP384PublicKey_uncompressed_0x02,
     sizeof(kP384PublicKey_uncompressed_0x02), NID_secp384r1},
    {kP384PublicKey_hybrid_0x03, sizeof(kP384PublicKey_hybrid_0x03),
     POINT_CONVERSION_UNCOMPRESSED, kP384PublicKey_uncompressed_0x03,
     sizeof(kP384PublicKey_uncompressed_0x03), NID_secp384r1},
    {kP521PublicKey_hybrid_0x02, sizeof(kP521PublicKey_hybrid_0x02),
     POINT_CONVERSION_UNCOMPRESSED, kP521PublicKey_uncompressed_0x02,
     sizeof(kP521PublicKey_uncompressed_0x02), NID_secp521r1},
    {kP521PublicKey_hybrid_0x03, sizeof(kP521PublicKey_hybrid_0x03),
     POINT_CONVERSION_UNCOMPRESSED, kP521PublicKey_uncompressed_0x03,
     sizeof(kP521PublicKey_uncompressed_0x03), NID_secp521r1},
};

class ECPublicKeyTest : public testing::TestWithParam<ECPublicKeyTestInput> {};

// This is to test |EC_KEY| decode using |o2i_ECPublicKey| and encode using
// |i2o_ECPublicKey|.
TEST_P(ECPublicKeyTest, DecodeAndEncode) {
  const auto &param = GetParam();

  // Generate |ec_key|.
  EC_KEY *ec_key = EC_KEY_new();
  ASSERT_TRUE(ec_key);
  bssl::UniquePtr<EC_KEY> ec_key_ptr(ec_key);
  EC_GROUP *group = EC_GROUP_new_by_curve_name(param.nid);
  ASSERT_TRUE(group);
  ASSERT_TRUE(EC_KEY_set_group(ec_key, group));
  const uint8_t *inp = &param.input_key[0];
  // Decoding an EC point.
  o2i_ECPublicKey(&ec_key, &inp, param.input_key_len);
  // On successful exit of |o2i_ECPublicKey|, |*inp| is advanced by |len| bytes.
  ASSERT_EQ(&param.input_key[0] + param.input_key_len, inp);
  // Set |conv_form| of |ec_key|.
  EC_KEY_set_conv_form(ec_key, param.encode_conv_form);
  // Encoding |ec_key| to bytes.
  // The 1st call of |i2o_ECPublicKey| is to tell the number of bytes in the
  // result, whether written or not.
  size_t len1 = i2o_ECPublicKey(ec_key, nullptr);
  ASSERT_EQ(len1, param.expected_output_key_len);
  uint8_t *p = nullptr;
  // The 2nd call of |i2o_ECPublicKey| is to write the number of bytes specified
  // by |len1|.
  size_t len2 = i2o_ECPublicKey(ec_key, &p);
  EXPECT_EQ(len2, param.expected_output_key_len);
  EXPECT_EQ(Bytes(param.expected_output_key, param.expected_output_key_len),
            Bytes(p, len2));

  // All the above should succeed, but |ec_key|'s assigned reference to the
  // |EC_GROUP| is one of the default static methods. Since these are static,
  // both references to |group| should retain the default
  // |POINT_CONVERSION_UNCOMPRESSED|. We don't encourage relying on |EC_GROUP|
  // to retain any information regarding the |conv_form|, but
  // |EC_GROUP_new_by_curve_name_mutable| is available for this specific
  // use-case.
  EXPECT_EQ(EC_KEY_get_conv_form(ec_key), param.encode_conv_form);
  EXPECT_EQ(EC_GROUP_get_point_conversion_form(EC_KEY_get0_group(ec_key)),
            POINT_CONVERSION_UNCOMPRESSED);
  EXPECT_EQ(EC_GROUP_get_point_conversion_form(group),
            POINT_CONVERSION_UNCOMPRESSED);

  OPENSSL_free(p);
}

TEST_P(ECPublicKeyTest, DecodeAndEncodeMutable) {
  const auto &param = GetParam();

  EC_KEY *ec_key = EC_KEY_new();
  ASSERT_TRUE(ec_key);
  bssl::UniquePtr<EC_KEY> ec_key_ptr(ec_key);
  bssl::UniquePtr<EC_GROUP> group(
      EC_GROUP_new_by_curve_name_mutable(param.nid));
  ASSERT_TRUE(group);

  ASSERT_TRUE(EC_KEY_set_group(ec_key, group.get()));
  const uint8_t *inp = &param.input_key[0];
  o2i_ECPublicKey(&ec_key, &inp, param.input_key_len);
  ASSERT_EQ(&param.input_key[0] + param.input_key_len, inp);

  // Set |conv_form| of |ec_key|.
  EC_KEY_set_conv_form(ec_key, param.encode_conv_form);

  size_t len1 = i2o_ECPublicKey(ec_key, nullptr);
  ASSERT_EQ(len1, param.expected_output_key_len);
  uint8_t *p = nullptr;
  size_t len2 = i2o_ECPublicKey(ec_key, &p);
  EXPECT_EQ(len2, param.expected_output_key_len);
  EXPECT_EQ(Bytes(param.expected_output_key, param.expected_output_key_len),
            Bytes(p, len2));

  // All the above should succeed, but the original |conv_form| for |group|
  // should not be changed with |EC_KEY_set_conv_form|. The |group| reference
  // assigned to |EC_KEY| was duplicated with |EC_GROUP_dup|, and is a different
  // pointer reference from |group|.
  // |group| should retain the default |POINT_CONVERSION_UNCOMPRESSED|.
  EXPECT_EQ(EC_KEY_get_conv_form(ec_key), param.encode_conv_form);
  EXPECT_EQ(EC_GROUP_get_point_conversion_form(EC_KEY_get0_group(ec_key)),
            param.encode_conv_form);
  EXPECT_EQ(EC_GROUP_get_point_conversion_form(group.get()),
            POINT_CONVERSION_UNCOMPRESSED);

  OPENSSL_free(p);
}

TEST_P(ECPublicKeyTest, MutableECGroup) {
  const auto &param = GetParam();

  bssl::UniquePtr<EC_GROUP> group(
      EC_GROUP_new_by_curve_name_mutable(param.nid));
  ASSERT_TRUE(group);

  bssl::UniquePtr<EC_POINT> point(EC_POINT_new(group.get()));
  ASSERT_TRUE(point.get());
  ASSERT_TRUE(EC_POINT_oct2point(group.get(), point.get(), param.input_key,
                                 param.input_key_len, nullptr));

  EC_GROUP_set_point_conversion_form(group.get(), param.encode_conv_form);

  // Use the saved conversion form in |group|. This should only work with
  // |EC_GROUP_new_by_curve_name_mutable|.
  std::vector<uint8_t> serialized;
  ASSERT_TRUE(EncodeECPoint(&serialized, group.get(), point.get(),
                            EC_GROUP_get_point_conversion_form(group.get())));
  EXPECT_EQ(Bytes(param.expected_output_key, param.expected_output_key_len),
            Bytes(serialized));
}

INSTANTIATE_TEST_SUITE_P(All, ECPublicKeyTest,
                         testing::ValuesIn(kDecodeAndEncodeInputs));

// The 1st byte should be |0x04| to indicate this is uncompressed ECPublicKey.
// This test is modified from |kP224PublicKey_uncompressed_0x02|.
static const uint8_t kP224PublicKey_wrong_uncompressed_byte[] = {
  /* wrong uncompressed byte */
  0x01,
  /* x-coordinate */
  0xd6, 0xf5, 0xf0, 0x6e, 0xf4, 0xc5, 0x56, 0x0a, 0xff, 0x8f, 0x49, 0x90,
  0xef, 0xdb, 0xa5, 0x9a, 0xf8, 0xa8, 0xd3, 0x77, 0x0d, 0x80, 0x14, 0x6a,
  0xc5, 0x82, 0x78, 0x85,
  /* y-coordinate */
  0xe0, 0x43, 0xae, 0x7b, 0xae, 0xa3, 0x77, 0x28, 0x60, 0x39, 0xc0, 0x7c,
  0x04, 0x1b, 0x7a, 0x3b, 0x5d, 0x76, 0x96, 0xda, 0xdd, 0xa7, 0x05, 0x1a,
  0xd6, 0x45, 0xa3, 0xea
};

// The last byte should be |0xea| instead of |0xe1|.
// This test is modified from |kP224PublicKey_uncompressed_0x02|.
static const uint8_t kP224PublicKey_uncompressed_wrong_y[] = {
  /* uncompressed */
  0x04,
  /* x-coordinate */
  0xd6, 0xf5, 0xf0, 0x6e, 0xf4, 0xc5, 0x56, 0x0a, 0xff, 0x8f, 0x49, 0x90,
  0xef, 0xdb, 0xa5, 0x9a, 0xf8, 0xa8, 0xd3, 0x77, 0x0d, 0x80, 0x14, 0x6a,
  0xc5, 0x82, 0x78, 0x85,
  /* y-coordinate */
  0xe0, 0x43, 0xae, 0x7b, 0xae, 0xa3, 0x77, 0x28, 0x60, 0x39, 0xc0, 0x7c,
  0x04, 0x1b, 0x7a, 0x3b, 0x5d, 0x76, 0x96, 0xda, 0xdd, 0xa7, 0x05, 0x1a,
  0xd6, 0x45, 0xa3, 0xe1
};

// The last byte |0xe1| should not exist.
// This test is modified from |kP224PublicKey_uncompressed_0x02|.
static const uint8_t kP224PublicKey_uncompressed_too_long[] = {
  /* uncompressed */
  0x04,
  /* x-coordinate */
  0xd6, 0xf5, 0xf0, 0x6e, 0xf4, 0xc5, 0x56, 0x0a, 0xff, 0x8f, 0x49, 0x90,
  0xef, 0xdb, 0xa5, 0x9a, 0xf8, 0xa8, 0xd3, 0x77, 0x0d, 0x80, 0x14, 0x6a,
  0xc5, 0x82, 0x78, 0x85,
  /* y-coordinate */
  0xe0, 0x43, 0xae, 0x7b, 0xae, 0xa3, 0x77, 0x28, 0x60, 0x39, 0xc0, 0x7c,
  0x04, 0x1b, 0x7a, 0x3b, 0x5d, 0x76, 0x96, 0xda, 0xdd, 0xa7, 0x05, 0x1a,
  0xd6, 0x45, 0xa3, 0xea,
  /* extra but not needed bytes */
  0xe1
};

// Additional one byte |0xea| should be appended to this array.
// This test is modified from |kP224PublicKey_uncompressed_0x02|.
static const uint8_t kP224PublicKey_uncompressed_too_short[] = {
  /* uncompressed */
  0x04,
  /* x-coordinate */
  0xd6, 0xf5, 0xf0, 0x6e, 0xf4, 0xc5, 0x56, 0x0a, 0xff, 0x8f, 0x49, 0x90,
  0xef, 0xdb, 0xa5, 0x9a, 0xf8, 0xa8, 0xd3, 0x77, 0x0d, 0x80, 0x14, 0x6a,
  0xc5, 0x82, 0x78, 0x85,
  /* y-coordinate */
  0xe0, 0x43, 0xae, 0x7b, 0xae, 0xa3, 0x77, 0x28, 0x60, 0x39, 0xc0, 0x7c,
  0x04, 0x1b, 0x7a, 0x3b, 0x5d, 0x76, 0x96, 0xda, 0xdd, 0xa7, 0x05, 0x1a,
  0xd6, 0x45, 0xa3
};

// The 1st byte should be 0x02.
// This test is modified from |kP224PublicKey_compressed_0x02|.
static const uint8_t kP224PublicKey_wrong_compressed_byte[] = {
  0x01,
  /* x-coordinate */
  0xd6, 0xf5, 0xf0, 0x6e, 0xf4, 0xc5, 0x56, 0x0a, 0xff, 0x8f, 0x49, 0x90,
  0xef, 0xdb, 0xa5, 0x9a, 0xf8, 0xa8, 0xd3, 0x77, 0x0d, 0x80, 0x14, 0x6a,
  0xc5, 0x82, 0x78, 0x85
};

// The last byte should be |0x85| instead of |0x87|.
// This test is modified from |kP224PublicKey_compressed_0x02|.
static const uint8_t kP224PublicKey_compressed_wrong_x[] = {
  0x02,
  /* x-coordinate */
  0xd6, 0xf5, 0xf0, 0x6e, 0xf4, 0xc5, 0x56, 0x0a, 0xff, 0x8f, 0x49, 0x90,
  0xef, 0xdb, 0xa5, 0x9a, 0xf8, 0xa8, 0xd3, 0x77, 0x0d, 0x80, 0x14, 0x6a,
  0xc5, 0x82, 0x78, 0x87
};

// Additional one byte |0x85| should be appended to this array.
// This test is modified from |kP224PublicKey_compressed_0x02|.
static const uint8_t kP224PublicKey_compressed_too_short[] = {
  0x02,
  /* x-coordinate */
  0xd6, 0xf5, 0xf0, 0x6e, 0xf4, 0xc5, 0x56, 0x0a, 0xff, 0x8f, 0x49, 0x90,
  0xef, 0xdb, 0xa5, 0x9a, 0xf8, 0xa8, 0xd3, 0x77, 0x0d, 0x80, 0x14, 0x6a,
  0xc5, 0x82, 0x78
};

// The last byte |0xe1| should not exist.
// This test is modified from |kP224PublicKey_compressed_0x02|.
static const uint8_t kP224PublicKey_compressed_too_long[] = {
  0x02,
  /* x-coordinate */
  0xd6, 0xf5, 0xf0, 0x6e, 0xf4, 0xc5, 0x56, 0x0a, 0xff, 0x8f, 0x49, 0x90,
  0xef, 0xdb, 0xa5, 0x9a, 0xf8, 0xa8, 0xd3, 0x77, 0x0d, 0x80, 0x14, 0x6a,
  0xc5, 0x82, 0x78, 0x85,
  /* extra but not needed bytes */
  0xe1
};

struct InvalidECPublicKey {
  const uint8_t *input_key;
  size_t input_key_len;
  int nid;
} kInvalidECPublicKeyInputs[] = {
  /* Test 0: point at infinity. */
  {
    (const uint8_t *)"\x00",
    1,
    NID_X9_62_prime256v1
  },
  /* Test 1: incorrect compresion representation. */
  {
    kP224PublicKey_wrong_compressed_byte,
    sizeof(kP224PublicKey_wrong_compressed_byte),
    NID_secp224r1
  },
  {
    kP224PublicKey_wrong_uncompressed_byte,
    sizeof(kP224PublicKey_wrong_uncompressed_byte),
    NID_secp224r1
  },
  /* Test 2: incorrect NID. */
  {
    kP224PublicKey_uncompressed_0x02,
    sizeof(kP224PublicKey_uncompressed_0x02),
    NID_secp521r1
  },
  {
    kP224PublicKey_compressed_0x02,
    sizeof(kP224PublicKey_compressed_0x02),
    NID_secp521r1
  },
  /* Test 3: bytes are too long, too short or wrong. */
  {
    kP224PublicKey_compressed_too_long,
    sizeof(kP224PublicKey_compressed_too_long),
    NID_secp224r1
  },
  {
    kP224PublicKey_compressed_too_short,
    sizeof(kP224PublicKey_compressed_too_short),
    NID_secp224r1
  },
  {
    kP224PublicKey_compressed_wrong_x,
    sizeof(kP224PublicKey_compressed_wrong_x),
    NID_secp224r1
  },
  {
    kP224PublicKey_uncompressed_too_long,
    sizeof(kP224PublicKey_uncompressed_too_long),
    NID_secp224r1
  },
  {
    kP224PublicKey_uncompressed_too_short,
    sizeof(kP224PublicKey_uncompressed_too_short),
    NID_secp224r1
  },
  {
    kP224PublicKey_uncompressed_wrong_y,
    sizeof(kP224PublicKey_uncompressed_wrong_y),
    NID_secp224r1
  }
};

class ECPublicKeyInvalidTest : public testing::TestWithParam<InvalidECPublicKey> {};

// This is to test |EC_KEY| failing to decode some bytes using |o2i_ECPublicKey|.
TEST_P(ECPublicKeyInvalidTest, Decode) {
  const auto &param = GetParam();
  const auto input_key = param.input_key;
  const auto input_key_len = param.input_key_len;
  const auto nid = param.nid;
  // Generate |ec_key|.
  EC_KEY *ec_key = EC_KEY_new();
  ASSERT_TRUE(ec_key);
  bssl::UniquePtr<EC_KEY> ec_key_ptr(ec_key);
  EC_GROUP *group = EC_GROUP_new_by_curve_name(nid);
  ASSERT_TRUE(group);
  ASSERT_TRUE(EC_KEY_set_group(ec_key, group));
  const uint8_t *inp = &input_key[0];
  // Decoding an EC point should fail and return NULL.
  ASSERT_TRUE(o2i_ECPublicKey(&ec_key, &inp, input_key_len) == nullptr);
  ERR_clear_error();
}

INSTANTIATE_TEST_SUITE_P(All, ECPublicKeyInvalidTest,
                         testing::ValuesIn(kInvalidECPublicKeyInputs));

TEST(ECTest, ZeroPadding) {
  // Check that the correct encoding round-trips.
  bssl::UniquePtr<EC_KEY> key =
      DecodeECPrivateKey(kECKeyWithZeros, sizeof(kECKeyWithZeros));
  ASSERT_TRUE(key);
  std::vector<uint8_t> out;
  EXPECT_TRUE(EncodeECPrivateKey(&out, key.get()));
  EXPECT_EQ(Bytes(kECKeyWithZeros), Bytes(out.data(), out.size()));

  // Keys without leading zeros also parse, but they encode correctly.
  key = DecodeECPrivateKey(kECKeyMissingZeros, sizeof(kECKeyMissingZeros));
  ASSERT_TRUE(key);
  EXPECT_TRUE(EncodeECPrivateKey(&out, key.get()));
  EXPECT_EQ(Bytes(kECKeyWithZeros), Bytes(out.data(), out.size()));
}

TEST(ECTest, SpecifiedCurve) {
  // Test keys with specified curves may be decoded.
  bssl::UniquePtr<EC_KEY> key =
      DecodeECPrivateKey(kECKeySpecifiedCurve, sizeof(kECKeySpecifiedCurve));
  ASSERT_TRUE(key);

  // The group should have been interpreted as P-256.
  EXPECT_EQ(NID_X9_62_prime256v1,
            EC_GROUP_get_curve_name(EC_KEY_get0_group(key.get())));

  // Encoding the key should still use named form.
  std::vector<uint8_t> out;
  EXPECT_TRUE(EncodeECPrivateKey(&out, key.get()));
  EXPECT_EQ(Bytes(kECKeyWithoutPublic), Bytes(out.data(), out.size()));
}

// An arbitrary curve which is identical to P-256.
static const uint8_t kP256P[] = {
    0xff, 0xff, 0xff, 0xff, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xff, 0xff,
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
};
static const uint8_t kP256A[] = {
    0xff, 0xff, 0xff, 0xff, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xff, 0xff,
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xfc,
};
static const uint8_t kP256B[] = {
    0x5a, 0xc6, 0x35, 0xd8, 0xaa, 0x3a, 0x93, 0xe7, 0xb3, 0xeb, 0xbd,
    0x55, 0x76, 0x98, 0x86, 0xbc, 0x65, 0x1d, 0x06, 0xb0, 0xcc, 0x53,
    0xb0, 0xf6, 0x3b, 0xce, 0x3c, 0x3e, 0x27, 0xd2, 0x60, 0x4b,
};
static const uint8_t kP256X[] = {
    0x6b, 0x17, 0xd1, 0xf2, 0xe1, 0x2c, 0x42, 0x47, 0xf8, 0xbc, 0xe6,
    0xe5, 0x63, 0xa4, 0x40, 0xf2, 0x77, 0x03, 0x7d, 0x81, 0x2d, 0xeb,
    0x33, 0xa0, 0xf4, 0xa1, 0x39, 0x45, 0xd8, 0x98, 0xc2, 0x96,
};
static const uint8_t kP256Y[] = {
    0x4f, 0xe3, 0x42, 0xe2, 0xfe, 0x1a, 0x7f, 0x9b, 0x8e, 0xe7, 0xeb,
    0x4a, 0x7c, 0x0f, 0x9e, 0x16, 0x2b, 0xce, 0x33, 0x57, 0x6b, 0x31,
    0x5e, 0xce, 0xcb, 0xb6, 0x40, 0x68, 0x37, 0xbf, 0x51, 0xf5,
};
static const uint8_t kP256Order[] = {
    0xff, 0xff, 0xff, 0xff, 0x00, 0x00, 0x00, 0x00, 0xff, 0xff, 0xff,
    0xff, 0xff, 0xff, 0xff, 0xff, 0xbc, 0xe6, 0xfa, 0xad, 0xa7, 0x17,
    0x9e, 0x84, 0xf3, 0xb9, 0xca, 0xc2, 0xfc, 0x63, 0x25, 0x51,
};

TEST(ECTest, ArbitraryCurve) {
  // Make a P-256 key and extract the affine coordinates.
  bssl::UniquePtr<EC_KEY> key(EC_KEY_new_by_curve_name(NID_X9_62_prime256v1));
  ASSERT_TRUE(key);
  ASSERT_TRUE(EC_KEY_generate_key(key.get()));

  bssl::UniquePtr<BN_CTX> ctx(BN_CTX_new());
  ASSERT_TRUE(ctx);
  bssl::UniquePtr<BIGNUM> p(BN_bin2bn(kP256P, sizeof(kP256P), nullptr));
  ASSERT_TRUE(p);
  bssl::UniquePtr<BIGNUM> a(BN_bin2bn(kP256A, sizeof(kP256A), nullptr));
  ASSERT_TRUE(a);
  bssl::UniquePtr<BIGNUM> b(BN_bin2bn(kP256B, sizeof(kP256B), nullptr));
  ASSERT_TRUE(b);
  bssl::UniquePtr<BIGNUM> gx(BN_bin2bn(kP256X, sizeof(kP256X), nullptr));
  ASSERT_TRUE(gx);
  bssl::UniquePtr<BIGNUM> gy(BN_bin2bn(kP256Y, sizeof(kP256Y), nullptr));
  ASSERT_TRUE(gy);
  bssl::UniquePtr<BIGNUM> order(
      BN_bin2bn(kP256Order, sizeof(kP256Order), nullptr));
  ASSERT_TRUE(order);

  bssl::UniquePtr<EC_GROUP> group(
      EC_GROUP_new_curve_GFp(p.get(), a.get(), b.get(), ctx.get()));
  ASSERT_TRUE(group);
  bssl::UniquePtr<EC_POINT> generator(EC_POINT_new(group.get()));
  ASSERT_TRUE(generator);
  ASSERT_TRUE(EC_POINT_set_affine_coordinates_GFp(
      group.get(), generator.get(), gx.get(), gy.get(), ctx.get()));
  ASSERT_TRUE(EC_GROUP_set_generator(group.get(), generator.get(), order.get(),
                                     BN_value_one()));

  // |group| should not have a curve name.
  EXPECT_EQ(NID_undef, EC_GROUP_get_curve_name(group.get()));

  // Copy |key| to |key2| using |group|.
  bssl::UniquePtr<EC_KEY> key2(EC_KEY_new());
  ASSERT_TRUE(key2);
  bssl::UniquePtr<EC_POINT> point(EC_POINT_new(group.get()));
  ASSERT_TRUE(point);
  bssl::UniquePtr<BIGNUM> x(BN_new()), y(BN_new());
  ASSERT_TRUE(x);
  ASSERT_TRUE(EC_KEY_set_group(key2.get(), group.get()));
  ASSERT_TRUE(
      EC_KEY_set_private_key(key2.get(), EC_KEY_get0_private_key(key.get())));
  ASSERT_TRUE(EC_POINT_get_affine_coordinates_GFp(
      EC_KEY_get0_group(key.get()), EC_KEY_get0_public_key(key.get()), x.get(),
      y.get(), nullptr));
  ASSERT_TRUE(EC_POINT_set_affine_coordinates_GFp(group.get(), point.get(),
                                                  x.get(), y.get(), nullptr));
  ASSERT_TRUE(EC_KEY_set_public_key(key2.get(), point.get()));

  // The key must be valid according to the new group too.
  EXPECT_TRUE(EC_KEY_check_key(key2.get()));

  bssl::UniquePtr<EVP_PKEY> ec_pkey(EVP_PKEY_new());
  ASSERT_TRUE(ec_pkey);
  ASSERT_TRUE(EVP_PKEY_set1_EC_KEY(ec_pkey.get(), key2.get()));
  bssl::UniquePtr<EVP_PKEY_CTX> ec_key_ctx(
          EVP_PKEY_CTX_new(ec_pkey.get(), NULL));
  ASSERT_TRUE(ec_key_ctx);
  EXPECT_TRUE(EVP_PKEY_check(ec_key_ctx.get()));
  EXPECT_TRUE(EVP_PKEY_public_check((ec_key_ctx.get())));

  // Make a second instance of |group|.
  bssl::UniquePtr<EC_GROUP> group2(
      EC_GROUP_new_curve_GFp(p.get(), a.get(), b.get(), ctx.get()));
  ASSERT_TRUE(group2);
  bssl::UniquePtr<EC_POINT> generator2(EC_POINT_new(group2.get()));
  ASSERT_TRUE(generator2);
  ASSERT_TRUE(EC_POINT_set_affine_coordinates_GFp(
      group2.get(), generator2.get(), gx.get(), gy.get(), ctx.get()));
  ASSERT_TRUE(EC_GROUP_set_generator(group2.get(), generator2.get(),
                                     order.get(), BN_value_one()));

  EXPECT_EQ(0, EC_GROUP_cmp(group.get(), group.get(), nullptr));
  EXPECT_EQ(0, EC_GROUP_cmp(group2.get(), group.get(), nullptr));

  // group3 uses the wrong generator.
  bssl::UniquePtr<EC_GROUP> group3(
      EC_GROUP_new_curve_GFp(p.get(), a.get(), b.get(), ctx.get()));
  ASSERT_TRUE(group3);
  bssl::UniquePtr<EC_POINT> generator3(EC_POINT_new(group3.get()));
  ASSERT_TRUE(generator3);
  ASSERT_TRUE(EC_POINT_set_affine_coordinates_GFp(
      group3.get(), generator3.get(), x.get(), y.get(), ctx.get()));
  ASSERT_TRUE(EC_GROUP_set_generator(group3.get(), generator3.get(),
                                     order.get(), BN_value_one()));

  EXPECT_NE(0, EC_GROUP_cmp(group.get(), group3.get(), NULL));

#if !defined(BORINGSSL_SHARED_LIBRARY)
  // group4 has non-minimal components that do not fit in |EC_SCALAR| and the
  // future |EC_FELEM|.
  ASSERT_TRUE(bn_resize_words(p.get(), 32));
  ASSERT_TRUE(bn_resize_words(a.get(), 32));
  ASSERT_TRUE(bn_resize_words(b.get(), 32));
  ASSERT_TRUE(bn_resize_words(gx.get(), 32));
  ASSERT_TRUE(bn_resize_words(gy.get(), 32));
  ASSERT_TRUE(bn_resize_words(order.get(), 32));

  bssl::UniquePtr<EC_GROUP> group4(
      EC_GROUP_new_curve_GFp(p.get(), a.get(), b.get(), ctx.get()));
  ASSERT_TRUE(group4);
  bssl::UniquePtr<EC_POINT> generator4(EC_POINT_new(group4.get()));
  ASSERT_TRUE(generator4);
  ASSERT_TRUE(EC_POINT_set_affine_coordinates_GFp(
      group4.get(), generator4.get(), gx.get(), gy.get(), ctx.get()));
  ASSERT_TRUE(EC_GROUP_set_generator(group4.get(), generator4.get(),
                                     order.get(), BN_value_one()));

  EXPECT_EQ(0, EC_GROUP_cmp(group.get(), group4.get(), NULL));
#endif

  // group5 is the same group, but the curve coefficients are passed in
  // unreduced and the caller does not pass in a |BN_CTX|.
  ASSERT_TRUE(BN_sub(a.get(), a.get(), p.get()));
  ASSERT_TRUE(BN_add(b.get(), b.get(), p.get()));
  bssl::UniquePtr<EC_GROUP> group5(
      EC_GROUP_new_curve_GFp(p.get(), a.get(), b.get(), NULL));
  ASSERT_TRUE(group5);
  bssl::UniquePtr<EC_POINT> generator5(EC_POINT_new(group5.get()));
  ASSERT_TRUE(generator5);
  ASSERT_TRUE(EC_POINT_set_affine_coordinates_GFp(
      group5.get(), generator5.get(), gx.get(), gy.get(), ctx.get()));
  ASSERT_TRUE(EC_GROUP_set_generator(group5.get(), generator5.get(),
                                     order.get(), BN_value_one()));

  EXPECT_EQ(0, EC_GROUP_cmp(group.get(), group.get(), NULL));
  EXPECT_EQ(0, EC_GROUP_cmp(group5.get(), group.get(), NULL));
}

TEST(ECTest, BIGNUMConvert) {
  bssl::UniquePtr<BN_CTX> ctx(BN_CTX_new());
  ASSERT_TRUE(ctx);
  bssl::UniquePtr<BIGNUM> p(BN_bin2bn(kP256P, sizeof(kP256P), nullptr));
  ASSERT_TRUE(p);
  bssl::UniquePtr<BIGNUM> a(BN_bin2bn(kP256A, sizeof(kP256A), nullptr));
  ASSERT_TRUE(a);
  bssl::UniquePtr<BIGNUM> b(BN_bin2bn(kP256B, sizeof(kP256B), nullptr));
  ASSERT_TRUE(b);
  bssl::UniquePtr<BIGNUM> gx(BN_bin2bn(kP256X, sizeof(kP256X), nullptr));
  ASSERT_TRUE(gx);
  bssl::UniquePtr<BIGNUM> gy(BN_bin2bn(kP256Y, sizeof(kP256Y), nullptr));
  ASSERT_TRUE(gy);
  bssl::UniquePtr<BIGNUM> order(
      BN_bin2bn(kP256Order, sizeof(kP256Order), nullptr));
  ASSERT_TRUE(order);

  bssl::UniquePtr<EC_GROUP> group(
      EC_GROUP_new_curve_GFp(p.get(), a.get(), b.get(), ctx.get()));
  ASSERT_TRUE(group);
  bssl::UniquePtr<EC_POINT> generator(EC_POINT_new(group.get()));
  ASSERT_TRUE(generator);
  ASSERT_TRUE(EC_POINT_set_affine_coordinates_GFp(
      group.get(), generator.get(), gx.get(), gy.get(), ctx.get()));
  ASSERT_TRUE(EC_GROUP_set_generator(group.get(), generator.get(), order.get(),
                                     BN_value_one()));

  // Make a second instance of |group|.
  bssl::UniquePtr<EC_GROUP> group2(
      EC_GROUP_new_curve_GFp(p.get(), a.get(), b.get(), ctx.get()));
  ASSERT_TRUE(group2);
  bssl::UniquePtr<EC_POINT> generator2(EC_POINT_new(group2.get()));
  ASSERT_TRUE(generator2);
  ASSERT_TRUE(EC_POINT_set_affine_coordinates_GFp(
      group2.get(), generator2.get(), gx.get(), gy.get(), ctx.get()));
  ASSERT_TRUE(EC_GROUP_set_generator(group2.get(), generator2.get(),
                                     order.get(), BN_value_one()));

  // Convert |EC_POINT| to |BIGNUM| in uncompressed format with
  // |EC_POINT_point2bn| and ensure results are the same.
  bssl::UniquePtr<BIGNUM> converted_bignum(
      EC_POINT_point2bn(group.get(), EC_GROUP_get0_generator(group.get()),
                        POINT_CONVERSION_UNCOMPRESSED, nullptr, nullptr));
  ASSERT_TRUE(converted_bignum);
  bssl::UniquePtr<BIGNUM> converted_bignum2(
      EC_POINT_point2bn(group2.get(), EC_GROUP_get0_generator(group2.get()),
                        POINT_CONVERSION_UNCOMPRESSED, nullptr, nullptr));
  ASSERT_TRUE(converted_bignum2);
  EXPECT_EQ(0, BN_cmp(converted_bignum.get(), converted_bignum2.get()));

  // Convert |BIGNUM| back to |EC_POINTS| with |EC_POINT_bn2point| and ensure
  // output is identical to the original.
  bssl::UniquePtr<EC_POINT> converted_generator(
      EC_POINT_bn2point(group.get(), converted_bignum.get(), nullptr, nullptr));
  ASSERT_TRUE(converted_generator);
  EXPECT_EQ(0, EC_POINT_cmp(group.get(), EC_GROUP_get0_generator(group.get()),
                            converted_generator.get(), nullptr));
  bssl::UniquePtr<EC_POINT> converted_generator2(EC_POINT_bn2point(
      group2.get(), converted_bignum2.get(), nullptr, nullptr));
  ASSERT_TRUE(converted_generator2);
  EXPECT_EQ(0, EC_POINT_cmp(group2.get(), EC_GROUP_get0_generator(group2.get()),
                            converted_generator2.get(), nullptr));

  // Convert |EC_POINT|s in compressed format with |EC_POINT_point2bn| and
  // ensure results are the same.
  converted_bignum.reset(
      EC_POINT_point2bn(group.get(), EC_GROUP_get0_generator(group.get()),
                        POINT_CONVERSION_COMPRESSED, nullptr, nullptr));
  ASSERT_TRUE(converted_bignum);
  converted_bignum2.reset(
      EC_POINT_point2bn(group2.get(), EC_GROUP_get0_generator(group2.get()),
                        POINT_CONVERSION_COMPRESSED, nullptr, nullptr));
  ASSERT_TRUE(converted_bignum2);
  EXPECT_EQ(0, BN_cmp(converted_bignum.get(), converted_bignum2.get()));

  // Convert |BIGNUM| back to |EC_POINTS| with |EC_POINT_bn2point| and ensure
  // output is identical to the original.
  converted_generator.reset(
      EC_POINT_bn2point(group.get(), converted_bignum.get(), nullptr, nullptr));
  ASSERT_TRUE(converted_generator);
  EXPECT_EQ(0, EC_POINT_cmp(group.get(), EC_GROUP_get0_generator(group.get()),
                            converted_generator.get(), nullptr));
  converted_generator2.reset(EC_POINT_bn2point(
      group2.get(), converted_bignum2.get(), nullptr, nullptr));
  ASSERT_TRUE(converted_generator2);
  EXPECT_EQ(0, EC_POINT_cmp(group2.get(), EC_GROUP_get0_generator(group2.get()),
                            converted_generator2.get(), nullptr));

  // Test specific openssl/openssl#10329 case for |BN_zero|.
  bssl::UniquePtr<BIGNUM> zero(BN_new());
  BN_zero(zero.get());
  bssl::UniquePtr<EC_POINT> infinity(
      EC_POINT_bn2point(group.get(), zero.get(), nullptr, nullptr));
  ASSERT_TRUE(infinity);
}

TEST(ECTest, SetKeyWithoutGroup) {
  bssl::UniquePtr<EC_KEY> key(EC_KEY_new());
  ASSERT_TRUE(key);

  // Private keys may not be configured without a group.
  EXPECT_FALSE(EC_KEY_set_private_key(key.get(), BN_value_one()));

  // Public keys may not be configured without a group.
  EXPECT_FALSE(EC_KEY_set_public_key(key.get(),
                                     EC_GROUP_get0_generator(EC_group_p256())));
}

TEST(ECTest, SetNULLKey) {
  bssl::UniquePtr<EC_KEY> key(EC_KEY_new_by_curve_name(NID_X9_62_prime256v1));
  ASSERT_TRUE(key);

  EXPECT_TRUE(EC_KEY_set_public_key(
      key.get(), EC_GROUP_get0_generator(EC_KEY_get0_group(key.get()))));
  EXPECT_TRUE(EC_KEY_get0_public_key(key.get()));

  // Setting a NULL public-key should clear the public-key and return zero, in
  // order to match OpenSSL behaviour exactly.
  EXPECT_FALSE(EC_KEY_set_public_key(key.get(), nullptr));
  EXPECT_FALSE(EC_KEY_get0_public_key(key.get()));
}

TEST(ECTest, PointAtInfinity) {
  bssl::UniquePtr<EC_KEY> key(EC_KEY_new_by_curve_name(NID_X9_62_prime256v1));
  ASSERT_TRUE(key);

  bssl::UniquePtr<EC_POINT> inf(
      EC_POINT_new(EC_KEY_get0_group(key.get())));
  ASSERT_TRUE(inf);
  ASSERT_TRUE(
      EC_POINT_set_to_infinity(EC_KEY_get0_group(key.get()), inf.get()));
  // Configuring a public key with the point at infinity is invalid.
  EXPECT_FALSE(EC_KEY_set_public_key(key.get(), inf.get()));
}

TEST(ECTest, GroupMismatch) {
  bssl::UniquePtr<EC_KEY> key(EC_KEY_new_by_curve_name(NID_secp384r1));
  ASSERT_TRUE(key);

  // Changing a key's group is invalid.
  EXPECT_FALSE(EC_KEY_set_group(key.get(), EC_group_p256()));

  // Configuring a public key with the wrong group is invalid.
  EXPECT_FALSE(EC_KEY_set_public_key(key.get(),
                                     EC_GROUP_get0_generator(EC_group_p256())));
}

TEST(ECTest, EmptyKey) {
  bssl::UniquePtr<EC_KEY> key(EC_KEY_new());
  ASSERT_TRUE(key);
  EXPECT_FALSE(EC_KEY_get0_group(key.get()));
  EXPECT_FALSE(EC_KEY_get0_public_key(key.get()));
  EXPECT_FALSE(EC_KEY_get0_private_key(key.get()));
}

// Test that point arithmetic works with custom curves using an arbitrary |a|,
// rather than -3, as is common (and more efficient).
TEST(ECTest, BrainpoolP256r1) {
  static const char kP[] =
      "a9fb57dba1eea9bc3e660a909d838d726e3bf623d52620282013481d1f6e5377";
  static const char kA[] =
      "7d5a0975fc2c3057eef67530417affe7fb8055c126dc5c6ce94a4b44f330b5d9";
  static const char kB[] =
      "26dc5c6ce94a4b44f330b5d9bbd77cbf958416295cf7e1ce6bccdc18ff8c07b6";
  static const char kX[] =
      "8bd2aeb9cb7e57cb2c4b482ffc81b7afb9de27e1e3bd23c23a4453bd9ace3262";
  static const char kY[] =
      "547ef835c3dac4fd97f8461a14611dc9c27745132ded8e545c1d54c72f046997";
  static const char kN[] =
      "a9fb57dba1eea9bc3e660a909d838d718c397aa3b561a6f7901e0e82974856a7";
  static const char kD[] =
      "0da21d76fed40dd82ac3314cce91abb585b5c4246e902b238a839609ea1e7ce1";
  static const char kQX[] =
      "3a55e0341cab50452fe27b8a87e4775dec7a9daca94b0d84ad1e9f85b53ea513";
  static const char kQY[] =
      "40088146b33bbbe81b092b41146774b35dd478cf056437cfb35ef0df2d269339";

  bssl::UniquePtr<BIGNUM> p = HexToBIGNUM(kP), a = HexToBIGNUM(kA),
                          b = HexToBIGNUM(kB), x = HexToBIGNUM(kX),
                          y = HexToBIGNUM(kY), n = HexToBIGNUM(kN),
                          d = HexToBIGNUM(kD), qx = HexToBIGNUM(kQX),
                          qy = HexToBIGNUM(kQY);
  ASSERT_TRUE(p && a && b && x && y && n && d && qx && qy);

  bssl::UniquePtr<EC_GROUP> group(
      EC_GROUP_new_curve_GFp(p.get(), a.get(), b.get(), nullptr));
  ASSERT_TRUE(group);
  bssl::UniquePtr<EC_POINT> g(EC_POINT_new(group.get()));
  ASSERT_TRUE(g);
  ASSERT_TRUE(EC_POINT_set_affine_coordinates_GFp(group.get(), g.get(), x.get(),
                                                  y.get(), nullptr));
  ASSERT_TRUE(
      EC_GROUP_set_generator(group.get(), g.get(), n.get(), BN_value_one()));

  bssl::UniquePtr<EC_POINT> q(EC_POINT_new(group.get()));
  ASSERT_TRUE(q);
  ASSERT_TRUE(
      EC_POINT_mul(group.get(), q.get(), d.get(), nullptr, nullptr, nullptr));
  ASSERT_TRUE(EC_POINT_get_affine_coordinates_GFp(group.get(), q.get(), x.get(),
                                                  y.get(), nullptr));
  EXPECT_EQ(0, BN_cmp(x.get(), qx.get()));
  EXPECT_EQ(0, BN_cmp(y.get(), qy.get()));
}

#if !defined(AWSLC_FIPS)

TEST(ECTest, SmallGroupOrder) {
  // Make a P-224 key and corrupt the group order to be small in order to fail
  // |EC_KEY_generate_key|.
  bssl::UniquePtr<EC_KEY> key(EC_KEY_new_by_curve_name(NID_secp224r1));
  ASSERT_TRUE(key);
  ASSERT_TRUE(EC_KEY_generate_key(key.get()));

  bssl::UniquePtr<EC_GROUP> group_org(EC_GROUP_new_by_curve_name(NID_secp224r1));
  ASSERT_TRUE(group_org);
  bssl::UniquePtr<BN_CTX> ctx(BN_CTX_new());
  ASSERT_TRUE(ctx);
  bssl::UniquePtr<BIGNUM> p(BN_new());
  ASSERT_TRUE(p);
  bssl::UniquePtr<BIGNUM> a(BN_new());
  ASSERT_TRUE(a);
  bssl::UniquePtr<BIGNUM> b(BN_new());
  ASSERT_TRUE(b);
  bssl::UniquePtr<BIGNUM> order(BN_new());
  ASSERT_TRUE(order);
  ASSERT_TRUE(BN_copy(order.get(), EC_GROUP_get0_order(group_org.get())));
  ASSERT_TRUE(EC_GROUP_get_curve_GFp(group_org.get(),
                                     p.get(), a.get(), b.get(), ctx.get()));

  // Set a new group with p, a, b
  bssl::UniquePtr<EC_GROUP> group(
      EC_GROUP_new_curve_GFp(p.get(), a.get(), b.get(), ctx.get()));
  ASSERT_TRUE(group);
  // The generator has to be created using the new group so they match when calling
  // |EC_GROUP_set_generator|
  bssl::UniquePtr<EC_POINT> generator(EC_POINT_new(group.get()));
  ASSERT_TRUE(generator);
  // Get the original group's generator's coordinates.
  bssl::UniquePtr<BIGNUM> gx(BN_new());
  ASSERT_TRUE(gx);
  bssl::UniquePtr<BIGNUM> gy(BN_new());
  ASSERT_TRUE(gy);
  EXPECT_TRUE(EC_POINT_get_affine_coordinates_GFp(
      group_org.get(), EC_GROUP_get0_generator(group_org.get()), gx.get(), gy.get(), ctx.get()));
  // Set the coordinates of the new generator.
  ASSERT_TRUE(EC_POINT_set_affine_coordinates_GFp(
      group.get(), generator.get(), gx.get(), gy.get(), ctx.get()));
  ASSERT_TRUE(EC_GROUP_set_generator(group.get(), generator.get(), order.get(),
                                     BN_value_one()));

  // Create a key2 with the new group and make the order value 7
  bssl::UniquePtr<EC_KEY> key2(EC_KEY_new());
  ASSERT_TRUE(key2);
  ASSERT_TRUE(EC_KEY_set_group(key2.get(), group.get()));
  BN_clear(&key2.get()->group->order.N);
  ASSERT_TRUE(BN_set_word(&key2.get()->group->order.N, 7));
  ASSERT_FALSE(EC_KEY_generate_key_fips(key2.get()));
}

#else
// AWSLCAndroidTestRunner does not take tests that do |ASSERT_DEATH| very well.
// GTEST issue: https://github.com/google/googletest/issues/1496.
#if !defined(OPENSSL_ANDROID) && !defined(AWSLC_FIPS_FAILURE_CALLBACK)

TEST(ECDeathTest, SmallGroupOrderAndDie) {
  // Make a P-224 key and corrupt the group order to be small in order to fail
  // |EC_KEY_generate_key|.
  bssl::UniquePtr<EC_KEY> key(EC_KEY_new_by_curve_name(NID_secp224r1));
  ASSERT_TRUE(key);
  ASSERT_TRUE(EC_KEY_generate_key(key.get()));

  bssl::UniquePtr<EC_GROUP> group_org(EC_GROUP_new_by_curve_name(NID_secp224r1));
  ASSERT_TRUE(group_org);
  bssl::UniquePtr<BN_CTX> ctx(BN_CTX_new());
  ASSERT_TRUE(ctx);
  bssl::UniquePtr<BIGNUM> p(BN_new());
  ASSERT_TRUE(p);
  bssl::UniquePtr<BIGNUM> a(BN_new());
  ASSERT_TRUE(a);
  bssl::UniquePtr<BIGNUM> b(BN_new());
  ASSERT_TRUE(b);
  bssl::UniquePtr<BIGNUM> order(BN_new());
  ASSERT_TRUE(order);
  ASSERT_TRUE(BN_copy(order.get(), EC_GROUP_get0_order(group_org.get())));
  ASSERT_TRUE(EC_GROUP_get_curve_GFp(group_org.get(),
                                     p.get(), a.get(), b.get(), ctx.get()));

  // Set a new group with p, a, b
  bssl::UniquePtr<EC_GROUP> group(
      EC_GROUP_new_curve_GFp(p.get(), a.get(), b.get(), ctx.get()));
  ASSERT_TRUE(group);
  // The generator has to be created using the new group so they match when calling
  // |EC_GROUP_set_generator|
  bssl::UniquePtr<EC_POINT> generator(EC_POINT_new(group.get()));
  ASSERT_TRUE(generator);
  // Get the original group's generator's coordinates.
  bssl::UniquePtr<BIGNUM> gx(BN_new());
  ASSERT_TRUE(gx);
  bssl::UniquePtr<BIGNUM> gy(BN_new());
  ASSERT_TRUE(gy);
  EXPECT_TRUE(EC_POINT_get_affine_coordinates_GFp(
      group_org.get(), EC_GROUP_get0_generator(group_org.get()), gx.get(), gy.get(), ctx.get()));
  // Set the coordinates of the new generator.
  ASSERT_TRUE(EC_POINT_set_affine_coordinates_GFp(
      group.get(), generator.get(), gx.get(), gy.get(), ctx.get()));
  ASSERT_TRUE(EC_GROUP_set_generator(group.get(), generator.get(), order.get(),
                                     BN_value_one()));

  // Create a key2 with the new group and make the order value 7
  bssl::UniquePtr<EC_KEY> key2(EC_KEY_new());
  ASSERT_TRUE(key2);
  ASSERT_TRUE(EC_KEY_set_group(key2.get(), group.get()));
  BN_clear(&key2.get()->group->order.N);
  ASSERT_TRUE(BN_set_word(&key2.get()->group->order.N, 7));
  ASSERT_DEATH_IF_SUPPORTED(EC_KEY_generate_key_fips(key2.get()), "");
}

#endif
#endif

struct CurveParam {
  int nid;
  bool mutable_group;
};

class ECCurveTest : public testing::TestWithParam<CurveParam> {
 public:
  EC_GROUP *group() const { return group_.get(); }

  void SetUp() override {
    if(GetParam().mutable_group) {
      group_.reset(EC_GROUP_new_by_curve_name_mutable(GetParam().nid));
      ASSERT_TRUE(group_);
    } else {
      group_.reset(EC_GROUP_new_by_curve_name(GetParam().nid));
      ASSERT_TRUE(group_);
    }
  }

 private:
  bssl::UniquePtr<EC_GROUP> group_{};
};

TEST_P(ECCurveTest, SetAffine) {
  // Generate an EC_KEY.
  bssl::UniquePtr<EC_KEY> key(EC_KEY_new_by_curve_name(GetParam().nid));
  ASSERT_TRUE(key);
  ASSERT_TRUE(EC_KEY_generate_key(key.get()));

  // Get the public key's coordinates.
  bssl::UniquePtr<BIGNUM> x(BN_new());
  ASSERT_TRUE(x);
  bssl::UniquePtr<BIGNUM> y(BN_new());
  ASSERT_TRUE(y);
  bssl::UniquePtr<BIGNUM> p(BN_new());
  ASSERT_TRUE(p);
  EXPECT_TRUE(EC_POINT_get_affine_coordinates_GFp(
      group(), EC_KEY_get0_public_key(key.get()), x.get(), y.get(), nullptr));
  EXPECT_TRUE(
      EC_GROUP_get_curve_GFp(group(), p.get(), nullptr, nullptr, nullptr));

  // Points on the curve should be accepted.
  auto point = bssl::UniquePtr<EC_POINT>(EC_POINT_new(group()));
  ASSERT_TRUE(point);
  EXPECT_TRUE(EC_POINT_set_affine_coordinates_GFp(group(), point.get(), x.get(),
                                                  y.get(), nullptr));

  // Subtract one from |y| to make the point no longer on the curve.
  EXPECT_TRUE(BN_sub(y.get(), y.get(), BN_value_one()));

  // Points not on the curve should be rejected.
  bssl::UniquePtr<EC_POINT> invalid_point(EC_POINT_new(group()));
  ASSERT_TRUE(invalid_point);
  EXPECT_FALSE(EC_POINT_set_affine_coordinates_GFp(group(), invalid_point.get(),
                                                   x.get(), y.get(), nullptr));

  // Coordinates out of range should be rejected.
  EXPECT_TRUE(BN_add(y.get(), y.get(), BN_value_one()));
  EXPECT_TRUE(BN_add(y.get(), y.get(), p.get()));

  EXPECT_FALSE(EC_POINT_set_affine_coordinates_GFp(group(), invalid_point.get(),
                                                   x.get(), y.get(), nullptr));
  EXPECT_FALSE(
      EC_KEY_set_public_key_affine_coordinates(key.get(), x.get(), y.get()));
}

TEST_P(ECCurveTest, IsOnCurve) {
  bssl::UniquePtr<EC_KEY> key(EC_KEY_new_by_curve_name(GetParam().nid));
  ASSERT_TRUE(key);
  ASSERT_TRUE(EC_KEY_generate_key(key.get()));

  // The generated point is on the curve.
  EXPECT_TRUE(EC_POINT_is_on_curve(group(), EC_KEY_get0_public_key(key.get()),
                                   nullptr));

  bssl::UniquePtr<EC_POINT> p(EC_POINT_new(group()));
  ASSERT_TRUE(p);
  ASSERT_TRUE(EC_POINT_copy(p.get(), EC_KEY_get0_public_key(key.get())));

  // This should never happen outside of a bug, but |EC_POINT_is_on_curve|
  // rejects points not on the curve.
  OPENSSL_memset(&p->raw.X, 0, sizeof(p->raw.X));
  EXPECT_FALSE(EC_POINT_is_on_curve(group(), p.get(), nullptr));

  // The point at infinity is always on the curve.
  ASSERT_TRUE(EC_POINT_copy(p.get(), EC_KEY_get0_public_key(key.get())));
  OPENSSL_memset(&p->raw.Z, 0, sizeof(p->raw.Z));
  EXPECT_TRUE(EC_POINT_is_on_curve(group(), p.get(), nullptr));
}

TEST_P(ECCurveTest, Compare) {
  bssl::UniquePtr<EC_KEY> key1(EC_KEY_new_by_curve_name(GetParam().nid));
  ASSERT_TRUE(key1);
  ASSERT_TRUE(EC_KEY_generate_key(key1.get()));
  const EC_POINT *pub1 = EC_KEY_get0_public_key(key1.get());

  bssl::UniquePtr<EC_KEY> key2(EC_KEY_new_by_curve_name(GetParam().nid));
  ASSERT_TRUE(key2);
  ASSERT_TRUE(EC_KEY_generate_key(key2.get()));
  const EC_POINT *pub2 = EC_KEY_get0_public_key(key2.get());

  // Two different points should not compare as equal.
  EXPECT_EQ(1, EC_POINT_cmp(group(), pub1, pub2, nullptr));

  // Serialize |pub1| and parse it back out. This gives a point in affine
  // coordinates.
  std::vector<uint8_t> serialized;
  ASSERT_TRUE(
      EncodeECPoint(&serialized, group(), pub1, POINT_CONVERSION_UNCOMPRESSED));
  bssl::UniquePtr<EC_POINT> p(EC_POINT_new(group()));
  ASSERT_TRUE(p);
  ASSERT_TRUE(EC_POINT_oct2point(group(), p.get(), serialized.data(),
                                 serialized.size(), nullptr));

  // The points should be equal.
  EXPECT_EQ(0, EC_POINT_cmp(group(), p.get(), pub1, nullptr));

  // Add something to the point. It no longer compares as equal.
  ASSERT_TRUE(EC_POINT_add(group(), p.get(), p.get(), pub2, nullptr));
  EXPECT_EQ(1, EC_POINT_cmp(group(), p.get(), pub1, nullptr));

  // Negate |pub2|. It should no longer compare as equal. This tests that we
  // check both x and y coordinate.
  bssl::UniquePtr<EC_POINT> q(EC_POINT_new(group()));
  ASSERT_TRUE(q);
  ASSERT_TRUE(EC_POINT_copy(q.get(), pub2));
  ASSERT_TRUE(EC_POINT_invert(group(), q.get(), nullptr));
  EXPECT_EQ(1, EC_POINT_cmp(group(), q.get(), pub2, nullptr));

  // Return |p| to the original value. It should be equal to |pub1| again.
  ASSERT_TRUE(EC_POINT_add(group(), p.get(), p.get(), q.get(), nullptr));
  EXPECT_EQ(0, EC_POINT_cmp(group(), p.get(), pub1, nullptr));

  // Infinity compares as equal to itself, but not other points.
  bssl::UniquePtr<EC_POINT> inf1(EC_POINT_new(group())),
      inf2(EC_POINT_new(group()));
  ASSERT_TRUE(inf1);
  ASSERT_TRUE(inf2);
  ASSERT_TRUE(EC_POINT_set_to_infinity(group(), inf1.get()));
  // |q| is currently -|pub2|.
  ASSERT_TRUE(EC_POINT_add(group(), inf2.get(), pub2, q.get(), nullptr));
  EXPECT_EQ(0, EC_POINT_cmp(group(), inf1.get(), inf2.get(), nullptr));
  EXPECT_EQ(1, EC_POINT_cmp(group(), inf1.get(), p.get(), nullptr));
}

TEST_P(ECCurveTest, GenerateFIPS) {
  // Generate an EC_KEY.
  bssl::UniquePtr<EC_KEY> key(EC_KEY_new_by_curve_name(GetParam().nid));
  ASSERT_TRUE(key);
  ASSERT_TRUE(EC_KEY_generate_key_fips(key.get()));
}

TEST_P(ECCurveTest, AddingEqualPoints) {
  bssl::UniquePtr<EC_KEY> key(EC_KEY_new_by_curve_name(GetParam().nid));
  ASSERT_TRUE(key);
  ASSERT_TRUE(EC_KEY_generate_key(key.get()));

  bssl::UniquePtr<EC_POINT> p1(EC_POINT_new(group()));
  ASSERT_TRUE(p1);
  ASSERT_TRUE(EC_POINT_copy(p1.get(), EC_KEY_get0_public_key(key.get())));

  bssl::UniquePtr<EC_POINT> p2(EC_POINT_new(group()));
  ASSERT_TRUE(p2);
  ASSERT_TRUE(EC_POINT_copy(p2.get(), EC_KEY_get0_public_key(key.get())));

  bssl::UniquePtr<EC_POINT> double_p1(EC_POINT_new(group()));
  ASSERT_TRUE(double_p1);
  bssl::UniquePtr<BN_CTX> ctx(BN_CTX_new());
  ASSERT_TRUE(ctx);
  ASSERT_TRUE(EC_POINT_dbl(group(), double_p1.get(), p1.get(), ctx.get()));

  bssl::UniquePtr<EC_POINT> p1_plus_p2(EC_POINT_new(group()));
  ASSERT_TRUE(p1_plus_p2);
  ASSERT_TRUE(
      EC_POINT_add(group(), p1_plus_p2.get(), p1.get(), p2.get(), ctx.get()));

  EXPECT_EQ(0,
            EC_POINT_cmp(group(), double_p1.get(), p1_plus_p2.get(), ctx.get()))
      << "A+A != 2A";
}

TEST_P(ECCurveTest, MulZero) {
  bssl::UniquePtr<EC_POINT> point(EC_POINT_new(group()));
  ASSERT_TRUE(point);
  bssl::UniquePtr<BIGNUM> zero(BN_new());
  ASSERT_TRUE(zero);
  BN_zero(zero.get());
  ASSERT_TRUE(EC_POINT_mul(group(), point.get(), zero.get(), nullptr, nullptr,
                           nullptr));

  EXPECT_TRUE(EC_POINT_is_at_infinity(group(), point.get()))
      << "g * 0 did not return point at infinity.";

  // Test that zero times an arbitrary point is also infinity. The generator is
  // used as the arbitrary point.
  bssl::UniquePtr<EC_POINT> generator(EC_POINT_new(group()));
  ASSERT_TRUE(generator);
  ASSERT_TRUE(EC_POINT_mul(group(), generator.get(), BN_value_one(), nullptr,
                           nullptr, nullptr));
  ASSERT_TRUE(EC_POINT_mul(group(), point.get(), nullptr, generator.get(),
                           zero.get(), nullptr));

  EXPECT_TRUE(EC_POINT_is_at_infinity(group(), point.get()))
      << "p * 0 did not return point at infinity.";
}

// Test that multiplying by the order produces ∞ and, moreover, that callers may
// do so. |EC_POINT_mul| is almost exclusively used with reduced scalars, with
// this exception. This comes from consumers following NIST SP 800-56A section
// 5.6.2.3.2. (Though all our curves have cofactor one, so this check isn't
// useful.)
TEST_P(ECCurveTest, MulOrder) {
  // Test that g × order = ∞.
  bssl::UniquePtr<EC_POINT> point(EC_POINT_new(group()));
  ASSERT_TRUE(point);
  ASSERT_TRUE(EC_POINT_mul(group(), point.get(), EC_GROUP_get0_order(group()),
                           nullptr, nullptr, nullptr));

  EXPECT_TRUE(EC_POINT_is_at_infinity(group(), point.get()))
      << "g * order did not return point at infinity.";

  // Test that p × order = ∞, for some arbitrary p.
  bssl::UniquePtr<BIGNUM> forty_two(BN_new());
  ASSERT_TRUE(forty_two);
  ASSERT_TRUE(BN_set_word(forty_two.get(), 42));
  ASSERT_TRUE(EC_POINT_mul(group(), point.get(), forty_two.get(), nullptr,
                           nullptr, nullptr));
  ASSERT_TRUE(EC_POINT_mul(group(), point.get(), nullptr, point.get(),
                           EC_GROUP_get0_order(group()), nullptr));

  EXPECT_TRUE(EC_POINT_is_at_infinity(group(), point.get()))
      << "p * order did not return point at infinity.";
}

// Test that |EC_POINT_mul| works with out-of-range scalars. The operation will
// not be constant-time, but we'll compute the right answer.
TEST_P(ECCurveTest, MulOutOfRange) {
  bssl::UniquePtr<BIGNUM> n_minus_one(BN_dup(EC_GROUP_get0_order(group())));
  ASSERT_TRUE(n_minus_one);
  ASSERT_TRUE(BN_sub_word(n_minus_one.get(), 1));

  bssl::UniquePtr<BIGNUM> minus_one(BN_new());
  ASSERT_TRUE(minus_one);
  ASSERT_TRUE(BN_one(minus_one.get()));
  BN_set_negative(minus_one.get(), 1);

  bssl::UniquePtr<BIGNUM> seven(BN_new());
  ASSERT_TRUE(seven);
  ASSERT_TRUE(BN_set_word(seven.get(), 7));

  bssl::UniquePtr<BIGNUM> ten_n_plus_seven(
      BN_dup(EC_GROUP_get0_order(group())));
  ASSERT_TRUE(ten_n_plus_seven);
  ASSERT_TRUE(BN_mul_word(ten_n_plus_seven.get(), 10));
  ASSERT_TRUE(BN_add_word(ten_n_plus_seven.get(), 7));

  bssl::UniquePtr<EC_POINT> point1(EC_POINT_new(group())),
      point2(EC_POINT_new(group()));
  ASSERT_TRUE(point1);
  ASSERT_TRUE(point2);

  ASSERT_TRUE(EC_POINT_mul(group(), point1.get(), n_minus_one.get(), nullptr,
                           nullptr, nullptr));
  ASSERT_TRUE(EC_POINT_mul(group(), point2.get(), minus_one.get(), nullptr,
                           nullptr, nullptr));
  EXPECT_EQ(0, EC_POINT_cmp(group(), point1.get(), point2.get(), nullptr))
      << "-1 * G and (n-1) * G did not give the same result";

  ASSERT_TRUE(EC_POINT_mul(group(), point1.get(), seven.get(), nullptr, nullptr,
                           nullptr));
  ASSERT_TRUE(EC_POINT_mul(group(), point2.get(), ten_n_plus_seven.get(),
                           nullptr, nullptr, nullptr));
  EXPECT_EQ(0, EC_POINT_cmp(group(), point1.get(), point2.get(), nullptr))
      << "7 * G and (10n + 7) * G did not give the same result";
}

// Test that 10×∞ + G = G.
TEST_P(ECCurveTest, Mul) {
  bssl::UniquePtr<EC_POINT> p(EC_POINT_new(group()));
  ASSERT_TRUE(p);
  bssl::UniquePtr<EC_POINT> result(EC_POINT_new(group()));
  ASSERT_TRUE(result);
  bssl::UniquePtr<BIGNUM> n(BN_new());
  ASSERT_TRUE(n);
  ASSERT_TRUE(EC_POINT_set_to_infinity(group(), p.get()));
  ASSERT_TRUE(BN_set_word(n.get(), 10));

  // First check that 10×∞ = ∞.
  ASSERT_TRUE(
      EC_POINT_mul(group(), result.get(), nullptr, p.get(), n.get(), nullptr));
  EXPECT_TRUE(EC_POINT_is_at_infinity(group(), result.get()));

  // Now check that 10×∞ + G = G.
  const EC_POINT *generator = EC_GROUP_get0_generator(group());
  ASSERT_TRUE(EC_POINT_mul(group(), result.get(), BN_value_one(), p.get(),
                           n.get(), nullptr));
  EXPECT_EQ(0, EC_POINT_cmp(group(), result.get(), generator, nullptr));
}

TEST_P(ECCurveTest, MulNonMinimal) {
  bssl::UniquePtr<BIGNUM> forty_two(BN_new());
  ASSERT_TRUE(forty_two);
  ASSERT_TRUE(BN_set_word(forty_two.get(), 42));

  // Compute g × 42.
  bssl::UniquePtr<EC_POINT> point(EC_POINT_new(group()));
  ASSERT_TRUE(point);
  ASSERT_TRUE(EC_POINT_mul(group(), point.get(), forty_two.get(), nullptr,
                           nullptr, nullptr));

  // Compute it again with a non-minimal 42, much larger than the scalar.
  ASSERT_TRUE(bn_resize_words(forty_two.get(), 64));

  bssl::UniquePtr<EC_POINT> point2(EC_POINT_new(group()));
  ASSERT_TRUE(point2);
  ASSERT_TRUE(EC_POINT_mul(group(), point2.get(), forty_two.get(), nullptr,
                           nullptr, nullptr));
  EXPECT_EQ(0, EC_POINT_cmp(group(), point.get(), point2.get(), nullptr));
}

// Test that EC_KEY_set_private_key rejects invalid values.
TEST_P(ECCurveTest, SetInvalidPrivateKey) {
  bssl::UniquePtr<EC_KEY> key(EC_KEY_new_by_curve_name(GetParam().nid));
  ASSERT_TRUE(key);

  bssl::UniquePtr<BIGNUM> bn(BN_dup(BN_value_one()));
  ASSERT_TRUE(bn);
  BN_set_negative(bn.get(), 1);
  EXPECT_FALSE(EC_KEY_set_private_key(key.get(), bn.get()))
      << "Unexpectedly set a key of -1";
  ERR_clear_error();

  ASSERT_TRUE(
      BN_copy(bn.get(), EC_GROUP_get0_order(EC_KEY_get0_group(key.get()))));
  EXPECT_FALSE(EC_KEY_set_private_key(key.get(), bn.get()))
      << "Unexpectedly set a key of the group order.";
  ERR_clear_error();

  BN_zero(bn.get());
  EXPECT_FALSE(EC_KEY_set_private_key(key.get(), bn.get()))
      << "Unexpectedly set a key of 0";
  ERR_clear_error();
}

TEST_P(ECCurveTest, IgnoreOct2PointReturnValue) {
  bssl::UniquePtr<BIGNUM> forty_two(BN_new());
  ASSERT_TRUE(forty_two);
  ASSERT_TRUE(BN_set_word(forty_two.get(), 42));

  // Compute g × 42.
  bssl::UniquePtr<EC_POINT> point(EC_POINT_new(group()));
  ASSERT_TRUE(point);
  ASSERT_TRUE(EC_POINT_mul(group(), point.get(), forty_two.get(), nullptr,
                           nullptr, nullptr));

  // Serialize the point.
  std::vector<uint8_t> serialized;
  ASSERT_TRUE(EncodeECPoint(&serialized, group(), point.get(),
                            POINT_CONVERSION_UNCOMPRESSED));

  // Create a serialized point that is not on the curve.
  serialized[serialized.size() - 1]++;

  ASSERT_FALSE(EC_POINT_oct2point(group(), point.get(), serialized.data(),
                                  serialized.size(), nullptr));
  // After a failure, |point| should have been set to the generator to defend
  // against code that doesn't check the return value.
  ASSERT_EQ(0, EC_POINT_cmp(group(), point.get(),
                            EC_GROUP_get0_generator(group()), nullptr));
}

TEST_P(ECCurveTest, DoubleSpecialCase) {
  const EC_POINT *g = EC_GROUP_get0_generator(group());

  bssl::UniquePtr<EC_POINT> two_g(EC_POINT_new(group()));
  ASSERT_TRUE(two_g);
  ASSERT_TRUE(EC_POINT_dbl(group(), two_g.get(), g, nullptr));

  bssl::UniquePtr<EC_POINT> p(EC_POINT_new(group()));
  ASSERT_TRUE(p);
  ASSERT_TRUE(EC_POINT_mul(group(), p.get(), BN_value_one(), g, BN_value_one(),
                           nullptr));
  EXPECT_EQ(0, EC_POINT_cmp(group(), p.get(), two_g.get(), nullptr));

  EC_SCALAR one;
  ASSERT_TRUE(ec_bignum_to_scalar(group(), &one, BN_value_one()));
  ASSERT_TRUE(
      ec_point_mul_scalar_public(group(), &p->raw, &one, &g->raw, &one));
  EXPECT_EQ(0, EC_POINT_cmp(group(), p.get(), two_g.get(), nullptr));
}

// This a regression test for a P-224 bug, but we may as well run it for all
// curves.
TEST_P(ECCurveTest, P224Bug) {
  // P = -G
  const EC_POINT *g = EC_GROUP_get0_generator(group());
  bssl::UniquePtr<EC_POINT> p(EC_POINT_dup(g, group()));
  ASSERT_TRUE(p);
  ASSERT_TRUE(EC_POINT_invert(group(), p.get(), nullptr));

  // Compute 31 * P + 32 * G = G
  bssl::UniquePtr<EC_POINT> ret(EC_POINT_new(group()));
  ASSERT_TRUE(ret);
  bssl::UniquePtr<BIGNUM> bn31(BN_new()), bn32(BN_new());
  ASSERT_TRUE(bn31);
  ASSERT_TRUE(bn32);
  ASSERT_TRUE(BN_set_word(bn31.get(), 31));
  ASSERT_TRUE(BN_set_word(bn32.get(), 32));
  ASSERT_TRUE(EC_POINT_mul(group(), ret.get(), bn32.get(), p.get(), bn31.get(),
                           nullptr));
  EXPECT_EQ(0, EC_POINT_cmp(group(), ret.get(), g, nullptr));

  // Repeat the computation with |ec_point_mul_scalar_public|, which ties the
  // additions together.
  EC_SCALAR sc31, sc32;
  ASSERT_TRUE(ec_bignum_to_scalar(group(), &sc31, bn31.get()));
  ASSERT_TRUE(ec_bignum_to_scalar(group(), &sc32, bn32.get()));
  ASSERT_TRUE(
      ec_point_mul_scalar_public(group(), &ret->raw, &sc32, &p->raw, &sc31));
  EXPECT_EQ(0, EC_POINT_cmp(group(), ret.get(), g, nullptr));
}

TEST_P(ECCurveTest, GPlusMinusG) {
  const EC_POINT *g = EC_GROUP_get0_generator(group());

  bssl::UniquePtr<EC_POINT> p(EC_POINT_dup(g, group()));
  ASSERT_TRUE(p);
  ASSERT_TRUE(EC_POINT_invert(group(), p.get(), nullptr));

  bssl::UniquePtr<EC_POINT> sum(EC_POINT_new(group()));
  ASSERT_TRUE(sum);
  ASSERT_TRUE(EC_POINT_add(group(), sum.get(), g, p.get(), nullptr));
  EXPECT_TRUE(EC_POINT_is_at_infinity(group(), sum.get()));
}

// Test that checks we encode or decode the point at infinity like OpenSSl.
TEST_P(ECCurveTest, EncodeInfinity) {
  // The point at infinity is encoded as a single zero byte in OpenSSL, and we
  // are forced to support it.
  static const uint8_t kInfinity[] = {0};
  bssl::UniquePtr<EC_POINT> inf(EC_POINT_new(group()));
  ASSERT_TRUE(inf);
  EXPECT_TRUE(EC_POINT_oct2point(group(), inf.get(), kInfinity,
                                 sizeof(kInfinity), nullptr));

  // Encoding it should succeed and set to 0.
  ASSERT_TRUE(EC_POINT_set_to_infinity(group(), inf.get()));
  uint8_t buf[128];
  // Tweak buf[0] to another value to ensure that it's set to 0.
  buf[0] = 1;
  EXPECT_EQ(
      1u, EC_POINT_point2oct(group(), inf.get(), POINT_CONVERSION_UNCOMPRESSED,
                             buf, sizeof(buf), nullptr));
  EXPECT_EQ(buf[0], 0);

  // Measuring the length of the encoding should succeed.
  EXPECT_EQ(
      1u, EC_POINT_point2oct(group(), inf.get(), POINT_CONVERSION_UNCOMPRESSED,
                             nullptr, 0, nullptr));
}

TEST_P(ECCurveTest, ECGroupConvForm) {
  bssl::UniquePtr<BIGNUM> one(BN_new());
  ASSERT_TRUE(one);
  ASSERT_TRUE(BN_set_word(one.get(), 1));

  // Ruby depends on |EC_GROUP| to save the used compression format, so we
  // replicate that scenario. This won't work with our default static curves.
  bssl::UniquePtr<EC_GROUP> group2(EC_GROUP_dup(group()));
  EC_GROUP_set_point_conversion_form(group2.get(), POINT_CONVERSION_COMPRESSED);

  // Compute g × 1.
  bssl::UniquePtr<EC_POINT> point(EC_POINT_new(group()));
  ASSERT_TRUE(point);
  ASSERT_TRUE(
      EC_POINT_mul(group(), point.get(), one.get(), nullptr, nullptr, nullptr));

  // Serialize the points.
  std::vector<uint8_t> group1_serialized;
  std::vector<uint8_t> group2_serialized;
  ASSERT_TRUE(EncodeECPoint(&group1_serialized, group(), point.get(),
                            EC_GROUP_get_point_conversion_form(group())));
  ASSERT_TRUE(EncodeECPoint(&group2_serialized, group2.get(), point.get(),
                            EC_GROUP_get_point_conversion_form(group2.get())));

  if (GetParam().mutable_group) {
    EXPECT_NE(Bytes(group1_serialized), Bytes(group2_serialized));
  } else {
    // |EC_GROUP_set_point_conversion_form| is a no-op when using our default
    // static groups.
    EXPECT_EQ(Bytes(group1_serialized), Bytes(group2_serialized));
  }
}

static std::vector<CurveParam> AllCurves() {
  const size_t num_curves = EC_get_builtin_curves(nullptr, 0);
  std::vector<EC_builtin_curve> curves(num_curves);
  EC_get_builtin_curves(curves.data(), num_curves);
  std::vector<CurveParam> nids;
  for (const auto &curve : curves) {
    // Curve test parameter to use static groups.
    CurveParam curve_param = { curve.nid, false };
    nids.push_back(curve_param);

    // Curve test parameter to use mutable groups.
    curve_param.mutable_group = true;
    nids.push_back(curve_param);
  }
  return nids;
}

static std::string CurveToString(
    const testing::TestParamInfo<CurveParam> &params) {
  return std::string(OBJ_nid2sn(params.param.nid)) +
         std::string(params.param.mutable_group ? "_mutable" : "_static_curve");
}

INSTANTIATE_TEST_SUITE_P(All, ECCurveTest, testing::ValuesIn(AllCurves()),
                         CurveToString);

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

  return bssl::UniquePtr<BIGNUM>(
      BN_bin2bn(bytes.data(), bytes.size(), nullptr));
}

static bool HasSuffix(const char *str, const char *suffix) {
  size_t suffix_len = strlen(suffix);
  size_t str_len = strlen(str);
  if (str_len < suffix_len) {
    return false;
  }
  return strcmp(str + str_len - suffix_len, suffix) == 0;
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

// Test for out-of-range coordinates in public-key validation in
// |EC_KEY_check_fips|. This test can only be exercised when the coordinates
// in the raw point are not in Montgomery representation, which is the case
// for P-224 in some builds (see below) and for P-521.
TEST(ECTest, LargeXCoordinateVectors) {
  int line;
  const char *file;

  bssl::UniquePtr<BN_CTX> ctx(BN_CTX_new());
  ASSERT_TRUE(ctx);

  FileTestGTest("crypto/fipsmodule/ec/large_x_coordinate_points.txt",
                [&](FileTest *t) {
	const EC_GROUP *group = GetCurve(t, "Curve");
    ASSERT_TRUE(group);
    bssl::UniquePtr<BIGNUM> x = GetBIGNUM(t, "X");
    ASSERT_TRUE(x);
    bssl::UniquePtr<BIGNUM> xpp = GetBIGNUM(t, "XplusP");
    ASSERT_TRUE(xpp);
    bssl::UniquePtr<BIGNUM> y = GetBIGNUM(t, "Y");
    ASSERT_TRUE(y);
    bssl::UniquePtr<EC_KEY> key(EC_KEY_new());
    ASSERT_TRUE(key);
    bssl::UniquePtr<EC_POINT> pub_key(EC_POINT_new(group));
    ASSERT_TRUE(pub_key);

    ASSERT_TRUE(EC_KEY_set_group(key.get(), group));

    // |EC_POINT_set_affine_coordinates_GFp| sets given (x, y) according to the
    // form the curve is using. If the curve is using Montgomery form, |x| and
    // |y| will be converted to Montgomery form.
    ASSERT_TRUE(EC_POINT_set_affine_coordinates_GFp(
                    group, pub_key.get(), x.get(), y.get(), nullptr));
    ASSERT_TRUE(EC_KEY_set_public_key(key.get(), pub_key.get()));
    ASSERT_TRUE(EC_KEY_check_fips(key.get()));

    bssl::UniquePtr<EVP_PKEY> ec_pkey(EVP_PKEY_new());
    ASSERT_TRUE(ec_pkey);
    ASSERT_TRUE(EVP_PKEY_set1_EC_KEY(ec_pkey.get(), key.get()));
    bssl::UniquePtr<EVP_PKEY_CTX> ec_key_ctx(
            EVP_PKEY_CTX_new(ec_pkey.get(), NULL));
    ASSERT_TRUE(ec_key_ctx);

    // Only valid public key exists, EVP_PKEY_check should fail
    ASSERT_FALSE(EVP_PKEY_check(ec_key_ctx.get()));
    ASSERT_TRUE(EVP_PKEY_public_check((ec_key_ctx.get())));

    // Set the raw point directly with the BIGNUM coordinates.
    // Note that both are in little-endian byte order.
    OPENSSL_memcpy(key.get()->pub_key->raw.X.words,
                   x.get()->d, BN_BYTES * group->field.N.width);
    OPENSSL_memcpy(key.get()->pub_key->raw.Y.words,
                   y.get()->d, BN_BYTES * group->field.N.width);
    OPENSSL_memset(key.get()->pub_key->raw.Z.words, 0, BN_BYTES * group->field.N.width);
    key.get()->pub_key->raw.Z.words[0] = 1;

    // |EC_KEY_check_fips| first calls the |EC_KEY_check_key| function that
    // checks if the key point is on the curve (among other checks). If the
    // curve uses Montgomery form the point-on-curve check will fail because
    // we set the raw point coordinates in regular form above.
    int curve_nid = group->curve_name;
    if (!is_curve_using_mont_felem_impl(curve_nid)) {
      ASSERT_TRUE(EC_KEY_check_fips(key.get()));
    } else {
      ASSERT_FALSE(EC_KEY_check_fips(key.get()));
      EXPECT_EQ(EC_R_POINT_IS_NOT_ON_CURVE,
                ERR_GET_REASON(ERR_peek_last_error_line(&file, &line)));
      EXPECT_PRED2(HasSuffix, file, "ec_key.c"); // within EC_KEY_check_key
    }

    // Now replace the x-coordinate with the larger one, x+p.
    OPENSSL_memcpy(key.get()->pub_key->raw.X.words,
                   xpp.get()->d, BN_BYTES * group->field.N.width);
    // We expect |EC_KEY_check_fips| to always fail when given key with x > p.
    ASSERT_FALSE(EC_KEY_check_fips(key.get()));

    // But the failure is for different reasons in case of curves using the
    // Montgomery form versus those that don't, as explained above.
    if (!is_curve_using_mont_felem_impl(curve_nid)) {
      EXPECT_EQ(EC_R_COORDINATES_OUT_OF_RANGE,
                ERR_GET_REASON(ERR_peek_last_error_line(&file, &line)));
      EXPECT_PRED2(HasSuffix, file, "ec_key.c"); // within EC_KEY_check_fips
    } else {
      EXPECT_EQ(EC_R_POINT_IS_NOT_ON_CURVE,
                ERR_GET_REASON(ERR_peek_last_error_line(&file, &line)));
      EXPECT_PRED2(HasSuffix, file, "ec_key.c"); // within EC_KEY_check_key
    }
  });
}

TEST(ECTest, ScalarBaseMultVectors) {
  bssl::UniquePtr<BN_CTX> ctx(BN_CTX_new());
  ASSERT_TRUE(ctx);

  FileTestGTest("crypto/fipsmodule/ec/ec_scalar_base_mult_tests.txt",
                [&](FileTest *t) {
    const EC_GROUP *group = GetCurve(t, "Curve");
    ASSERT_TRUE(group);
    bssl::UniquePtr<BIGNUM> n = GetBIGNUM(t, "N");
    ASSERT_TRUE(n);
    bssl::UniquePtr<BIGNUM> x = GetBIGNUM(t, "X");
    ASSERT_TRUE(x);
    bssl::UniquePtr<BIGNUM> y = GetBIGNUM(t, "Y");
    ASSERT_TRUE(y);
    bool is_infinity = BN_is_zero(x.get()) && BN_is_zero(y.get());

    bssl::UniquePtr<BIGNUM> px(BN_new());
    ASSERT_TRUE(px);
    bssl::UniquePtr<BIGNUM> py(BN_new());
    ASSERT_TRUE(py);
    auto check_point = [&](const EC_POINT *p) {
      if (is_infinity) {
        EXPECT_TRUE(EC_POINT_is_at_infinity(group, p));
      } else {
        ASSERT_TRUE(EC_POINT_get_affine_coordinates_GFp(
            group, p, px.get(), py.get(), ctx.get()));
        EXPECT_EQ(0, BN_cmp(x.get(), px.get()));
        EXPECT_EQ(0, BN_cmp(y.get(), py.get()));
      }
    };

    const EC_POINT *g = EC_GROUP_get0_generator(group);
    bssl::UniquePtr<EC_POINT> p(EC_POINT_new(group));
    ASSERT_TRUE(p);
    // Test single-point multiplication.
    ASSERT_TRUE(EC_POINT_mul(group, p.get(), n.get(), nullptr, nullptr,
                             ctx.get()));
    check_point(p.get());

    ASSERT_TRUE(EC_POINT_mul(group, p.get(), nullptr, g, n.get(), ctx.get()));
    check_point(p.get());
  });
}

// These tests take a very long time, but are worth running when we make
// non-trivial changes to the EC code.
TEST(ECTest, DISABLED_ScalarBaseMultVectorsTwoPoint) {
  bssl::UniquePtr<BN_CTX> ctx(BN_CTX_new());
  ASSERT_TRUE(ctx);

  FileTestGTest("crypto/fipsmodule/ec/ec_scalar_base_mult_tests.txt",
                [&](FileTest *t) {
    const EC_GROUP *group = GetCurve(t, "Curve");
    ASSERT_TRUE(group);
    bssl::UniquePtr<BIGNUM> n = GetBIGNUM(t, "N");
    ASSERT_TRUE(n);
    bssl::UniquePtr<BIGNUM> x = GetBIGNUM(t, "X");
    ASSERT_TRUE(x);
    bssl::UniquePtr<BIGNUM> y = GetBIGNUM(t, "Y");
    ASSERT_TRUE(y);
    bool is_infinity = BN_is_zero(x.get()) && BN_is_zero(y.get());

    bssl::UniquePtr<BIGNUM> px(BN_new());
    ASSERT_TRUE(px);
    bssl::UniquePtr<BIGNUM> py(BN_new());
    ASSERT_TRUE(py);
    auto check_point = [&](const EC_POINT *p) {
      if (is_infinity) {
        EXPECT_TRUE(EC_POINT_is_at_infinity(group, p));
      } else {
        ASSERT_TRUE(EC_POINT_get_affine_coordinates_GFp(
            group, p, px.get(), py.get(), ctx.get()));
        EXPECT_EQ(0, BN_cmp(x.get(), px.get()));
        EXPECT_EQ(0, BN_cmp(y.get(), py.get()));
      }
    };

    const EC_POINT *g = EC_GROUP_get0_generator(group);
    bssl::UniquePtr<EC_POINT> p(EC_POINT_new(group));
    ASSERT_TRUE(p);
    bssl::UniquePtr<BIGNUM> a(BN_new()), b(BN_new());
    for (int i = -64; i < 64; i++) {
      SCOPED_TRACE(i);
      ASSERT_TRUE(BN_set_word(a.get(), abs(i)));
      if (i < 0) {
        ASSERT_TRUE(BN_sub(a.get(), EC_GROUP_get0_order(group), a.get()));
      }

      ASSERT_TRUE(BN_copy(b.get(), n.get()));
      ASSERT_TRUE(BN_sub(b.get(), b.get(), a.get()));
      if (BN_is_negative(b.get())) {
        ASSERT_TRUE(BN_add(b.get(), b.get(), EC_GROUP_get0_order(group)));
      }

      ASSERT_TRUE(EC_POINT_mul(group, p.get(), a.get(), g, b.get(), ctx.get()));
      check_point(p.get());

      EC_SCALAR a_scalar, b_scalar;
      ASSERT_TRUE(ec_bignum_to_scalar(group, &a_scalar, a.get()));
      ASSERT_TRUE(ec_bignum_to_scalar(group, &b_scalar, b.get()));
      ASSERT_TRUE(ec_point_mul_scalar_public(group, &p->raw, &a_scalar, &g->raw,
                                             &b_scalar));
      check_point(p.get());
    }
  });
}

TEST(ECTest, DeriveFromSecret) {
  struct DeriveTest {
    const EC_GROUP *group;
    std::vector<uint8_t> secret;
    std::vector<uint8_t> expected_priv;
    std::vector<uint8_t> expected_pub;
  };
  const DeriveTest kDeriveTests[] = {
      {EC_group_p256(), HexToBytes(""),
       HexToBytes(
           "b98a86a71efb51ebdac4759937b977e9b0c05224675bb2b6a58ba306e237f4b8"),
       HexToBytes(
           "04fbe6cab439918e00231a2ff073cdc25823998864a9eb36f809095a1a919ece875"
           "a145803fbe89a6cde53936e3c6d9c253ed3d38f5f58cae455c27e95645ceda9")},
      {EC_group_p256(), HexToBytes("123456"),
       HexToBytes(
           "44a72bc62087b88e5ab7126766177ed0d8f1ed09ad066cd746527fc201105a7e"),
       HexToBytes(
           "04ec0555cd76e991fef7f5504343937d0f38696db3360a4854052cb0d84a377a5a0"
           "ff64c352755c28692b4ae085c2b817db9a1eddbd22e9cf39c12751e0870791b")},
      {EC_group_p256(), HexToBytes("00000000000000000000000000000000"),
       HexToBytes(
           "7ca1e2c83e6a5f2c1b3e7d58180226f269930c4b9fbe2a275096079630b7c57d"),
       HexToBytes(
           "0442ef70c8fc0fbe383ed0a0da36f39f9a590f3feebc07863cc858c9a8ef0465731"
           "0408c249bd4d61929c54b71ffe056e6b4fa1eb537039b43d1c175f0ceab0f89")},
      {EC_group_p256(),
       HexToBytes(
           "de9c9b35543aaa0fba039e34e8ca9695da3225c7161c9e3a8c70356cac28c780"),
       HexToBytes(
           "659f5abf3b62b9931c29d6ed0722efd2349fa56f54e708cf3272f620f1bc44d0"),
       HexToBytes(
           "046741f806b593bf3a3d4a9d76bdcb9b0d7874633cbea8f42c05e78561f7e8ec362"
           "b9b6f1913ded796fbdafe7f210cea897ac22a4e580c06a60f2659fd09f1830f")},
      {EC_group_p384(), HexToBytes("123456"),
       HexToBytes("95cd90d548997de090c7622708eccb7edc1b1bd78d2422235ad97406dada"
                  "076555309da200096f6e4b36c46002beee89"),
       HexToBytes(
           "04007b2d026aa7636fa912c3f970d62bb6c10fa81c8f3290ed90b2d701696d1c6b9"
           "5af88ce13e962996a7ac37e16527cb5d69bd081b8641d07634cf84b438600ec9434"
           "15ac6bd7a0236f7ab0ea31ece67df03fa11646ea2b75e73d1b5e45b75c18")},
  };

  for (const auto &test : kDeriveTests) {
    SCOPED_TRACE(Bytes(test.secret));

    bssl::UniquePtr<EC_KEY> key(EC_KEY_derive_from_secret(
        test.group, test.secret.data(), test.secret.size()));
    ASSERT_TRUE(key);

    std::vector<uint8_t> priv(BN_num_bytes(EC_GROUP_get0_order(test.group)));
    ASSERT_TRUE(BN_bn2bin_padded(priv.data(), priv.size(),
                                 EC_KEY_get0_private_key(key.get())));
    EXPECT_EQ(Bytes(priv), Bytes(test.expected_priv));

    uint8_t *pub = nullptr;
    size_t pub_len =
        EC_KEY_key2buf(key.get(), POINT_CONVERSION_UNCOMPRESSED, &pub, nullptr);
    bssl::UniquePtr<uint8_t> free_pub(pub);
    EXPECT_NE(pub_len, 0u);
    EXPECT_EQ(Bytes(pub, pub_len), Bytes(test.expected_pub));
  }
}

TEST(ECTest, HashToCurve) {
  auto hash_to_curve_p384_sha512_draft07 =
      [](const EC_GROUP *group, EC_POINT *out, const uint8_t *dst,
         size_t dst_len, const uint8_t *msg, size_t msg_len) -> int {
    if (EC_GROUP_cmp(group, out->group, NULL) != 0) {
      return 0;
    }
    return ec_hash_to_curve_p384_xmd_sha512_sswu_draft07(group, &out->raw, dst,
                                                         dst_len, msg, msg_len);
  };

  struct HashToCurveTest {
    int (*hash_to_curve)(const EC_GROUP *group, EC_POINT *out,
                         const uint8_t *dst, size_t dst_len, const uint8_t *msg,
                         size_t msg_len);
    const EC_GROUP *group;
    const char *dst;
    const char *msg;
    const char *x_hex;
    const char *y_hex;
  };
  const HashToCurveTest kTests[] = {
      // See RFC 9380, appendix J.1.1.
      {&EC_hash_to_curve_p256_xmd_sha256_sswu, EC_group_p256(),
       "QUUX-V01-CS02-with-P256_XMD:SHA-256_SSWU_RO_", "",
       "2c15230b26dbc6fc9a37051158c95b79656e17a1a920b11394ca91"
       "c44247d3e4",
       "8a7a74985cc5c776cdfe4b1f19884970453912e9d31528c060be9a"
       "b5c43e8415"},
      {&EC_hash_to_curve_p256_xmd_sha256_sswu, EC_group_p256(),
       "QUUX-V01-CS02-with-P256_XMD:SHA-256_SSWU_RO_", "abc",
       "0bb8b87485551aa43ed54f009230450b492fead5f1cc91658775da"
       "c4a3388a0f",
       "5c41b3d0731a27a7b14bc0bf0ccded2d8751f83493404c84a88e71"
       "ffd424212e"},
      {&EC_hash_to_curve_p256_xmd_sha256_sswu, EC_group_p256(),
       "QUUX-V01-CS02-with-P256_XMD:SHA-256_SSWU_RO_", "abcdef0123456789",
       "65038ac8f2b1def042a5df0b33b1f4eca6bff7cb0f9c6c15268118"
       "64e544ed80",
       "cad44d40a656e7aff4002a8de287abc8ae0482b5ae825822bb870d"
       "6df9b56ca3"},
      {&EC_hash_to_curve_p256_xmd_sha256_sswu, EC_group_p256(),
       "QUUX-V01-CS02-with-P256_XMD:SHA-256_SSWU_RO_",
       "q128_qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqq"
       "qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqq"
       "qqqqqqqqqqqqqqqqqqqqqqqqq",
       "4be61ee205094282ba8a2042bcb48d88dfbb609301c49aa8b07853"
       "3dc65a0b5d",
       "98f8df449a072c4721d241a3b1236d3caccba603f916ca680f4539"
       "d2bfb3c29e"},
      {&EC_hash_to_curve_p256_xmd_sha256_sswu, EC_group_p256(),
       "QUUX-V01-CS02-with-P256_XMD:SHA-256_SSWU_RO_",
       "a512_aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
       "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
       "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
       "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
       "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
       "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
       "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
       "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
       "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
       "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
       "457ae2981f70ca85d8e24c308b14db22f3e3862c5ea0f652ca38b5"
       "e49cd64bc5",
       "ecb9f0eadc9aeed232dabc53235368c1394c78de05dd96893eefa6"
       "2b0f4757dc"},

      // See draft-irtf-cfrg-hash-to-curve-07, appendix G.2.1.
      {hash_to_curve_p384_sha512_draft07, EC_group_p384(),
       "P384_XMD:SHA-512_SSWU_RO_TESTGEN", "",
       "2fc0b9efdd63a8e43b4db88dc12f03c798f6fd91bccac0c9096185"
       "4386e58fdc54fc2a01f0f358759054ce1f9b762025",
       "949b936fabb72cdb02cd7980b86cb6a3adf286658e81301648851d"
       "b8a49d9bec00ccb57698d559fc5960fa5030a8e54b"},
      {hash_to_curve_p384_sha512_draft07, EC_group_p384(),
       "P384_XMD:SHA-512_SSWU_RO_TESTGEN", "abc",
       "4f3338035391e8ce8ce40c974136f0edc97f392ffd44a643338741"
       "8ed1b8c2603487e1688ec151f048fbc6b2c138c92f",
       "152b90aef6558be328a3168855fb1906452e7167b0f7c8a56ff9d4"
       "fa87d6fb522cdf8e409db54418b2c764fd26260757"},
      {hash_to_curve_p384_sha512_draft07, EC_group_p384(),
       "P384_XMD:SHA-512_SSWU_RO_TESTGEN", "abcdef0123456789",
       "e9e5d7ac397e123d060ad44301cbc8eb972f6e64ebcff29dcc9b9a"
       "10357902aace2240c580fec85e5b427d98b4e80703",
       "916cb8963521ad75105be43cc4148e5a5bbb4fcf107f1577e4f7fa"
       "3ca58cd786aa76890c8e687d2353393bc16c78ec4d"},
      {hash_to_curve_p384_sha512_draft07, EC_group_p384(),
       "P384_XMD:SHA-512_SSWU_RO_TESTGEN",
       "a512_aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
       "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
       "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
       "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
       "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
       "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
       "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
       "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
       "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
       "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
       "41941db59a7b8b633bd5bfa462f1e29a9f18e5a341445d90fc6eb9"
       "37f2913224287b9dfb64742851f760eb14ca115ff9",
       "1510e764f1be968d661b7aaecb26a6d38c98e5205ca150f0ae426d"
       "2c3983c68e3a9ffb283c6ae4891d891b5705500475"},
  };

  for (const auto &test : kTests) {
    SCOPED_TRACE(test.dst);
    SCOPED_TRACE(test.msg);

    bssl::UniquePtr<EC_POINT> p(EC_POINT_new(test.group));
    ASSERT_TRUE(p);
    ASSERT_TRUE(test.hash_to_curve(
        test.group, p.get(), reinterpret_cast<const uint8_t *>(test.dst),
        strlen(test.dst), reinterpret_cast<const uint8_t *>(test.msg),
        strlen(test.msg)));

    std::vector<uint8_t> buf;
    ASSERT_TRUE(EncodeECPoint(&buf, test.group, p.get(),
                              POINT_CONVERSION_UNCOMPRESSED));
    size_t field_len = (buf.size() - 1) / 2;
    EXPECT_EQ(test.x_hex,
              EncodeHex(bssl::MakeConstSpan(buf).subspan(1, field_len)));
    EXPECT_EQ(test.y_hex, EncodeHex(bssl::MakeConstSpan(buf).subspan(
                              1 + field_len, field_len)));
  }

  // hash-to-curve functions should check for the wrong group.
  EC_JACOBIAN raw;
  bssl::UniquePtr<EC_POINT> p_p384(EC_POINT_new(EC_group_p384()));
  ASSERT_TRUE(p_p384);
  bssl::UniquePtr<EC_POINT> p_p224(EC_POINT_new(EC_group_p224()));
  ASSERT_TRUE(p_p224);
  static const uint8_t kDST[] = {0, 1, 2, 3};
  static const uint8_t kMessage[] = {4, 5, 6, 7};
  EXPECT_FALSE(ec_hash_to_curve_p384_xmd_sha384_sswu(
      EC_group_p224(), &raw, kDST, sizeof(kDST), kMessage, sizeof(kMessage)));
  EXPECT_FALSE(EC_hash_to_curve_p384_xmd_sha384_sswu(
      EC_group_p224(), p_p224.get(), kDST, sizeof(kDST), kMessage,
      sizeof(kMessage)));
  EXPECT_FALSE(EC_hash_to_curve_p384_xmd_sha384_sswu(
      EC_group_p224(), p_p384.get(), kDST, sizeof(kDST), kMessage,
      sizeof(kMessage)));
  EXPECT_FALSE(EC_hash_to_curve_p384_xmd_sha384_sswu(
      EC_group_p384(), p_p224.get(), kDST, sizeof(kDST), kMessage,
      sizeof(kMessage)));

  // Zero-length DSTs are not allowed.
  EXPECT_FALSE(ec_hash_to_curve_p384_xmd_sha384_sswu(
      EC_group_p384(), &raw, nullptr, 0, kMessage, sizeof(kMessage)));
}

TEST(ECTest, HashToScalar) {
  struct HashToScalarTest {
    int (*hash_to_scalar)(const EC_GROUP *group, EC_SCALAR *out,
                          const uint8_t *dst, size_t dst_len,
                          const uint8_t *msg, size_t msg_len);
    const EC_GROUP *group;
    const char *dst;
    const char *msg;
    const char *result_hex;
  };
  const HashToScalarTest kTests[] = {
      {&ec_hash_to_scalar_p384_xmd_sha512_draft07, EC_group_p384(),
       "P384_XMD:SHA-512_SCALAR_TEST", "",
       "9687acc2de56c3cf94c0e05b6811a21aa480092254ec0532bdce63"
       "140ecd340f09dc2d45d77e21fb0aa76f7707b8a676"},
      {&ec_hash_to_scalar_p384_xmd_sha512_draft07, EC_group_p384(),
       "P384_XMD:SHA-512_SCALAR_TEST", "abcdef0123456789",
       "8f8076022a68233cbcecaceae68c2068f132724f001caa78619eff"
       "1ffc58fa871db73fe9034fc9cf853c384ed34b5666"},
      {&ec_hash_to_scalar_p384_xmd_sha512_draft07, EC_group_p384(),
       "P384_XMD:SHA-512_SCALAR_TEST",
       "a512_aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
       "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
       "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
       "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
       "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
       "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
       "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
       "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
       "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
       "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
       "750f2fae7d2b2f41ac737d180c1d4363d85a1504798b4976d40921"
       "1ddb3651c13a5b4daba9975cdfce18336791131915"},
  };

  for (const auto &test : kTests) {
    SCOPED_TRACE(test.dst);
    SCOPED_TRACE(test.msg);

    EC_SCALAR scalar;
    ASSERT_TRUE(test.hash_to_scalar(
        test.group, &scalar, reinterpret_cast<const uint8_t *>(test.dst),
        strlen(test.dst), reinterpret_cast<const uint8_t *>(test.msg),
        strlen(test.msg)));
    uint8_t buf[EC_MAX_BYTES];
    size_t len;
    ec_scalar_to_bytes(test.group, buf, &len, &scalar);
    EXPECT_EQ(test.result_hex, EncodeHex(bssl::MakeConstSpan(buf, len)));
  }

  // hash-to-scalar functions should check for the wrong group.
  EC_SCALAR scalar;
  static const uint8_t kDST[] = {0, 1, 2, 3};
  static const uint8_t kMessage[] = {4, 5, 6, 7};
  EXPECT_FALSE(ec_hash_to_scalar_p384_xmd_sha512_draft07(
      EC_group_p224(), &scalar, kDST, sizeof(kDST), kMessage,
      sizeof(kMessage)));
}

TEST(ECTest, FelemBytes) {
  std::tuple<int,int, int>  test_cases[2] = {
          std::make_tuple(NID_secp384r1, P384_EC_FELEM_BYTES, P384_EC_FELEM_WORDS),
          std::make_tuple(NID_secp521r1, P521_EC_FELEM_BYTES, P521_EC_FELEM_WORDS)
  };

  for(size_t i = 0; i < sizeof(test_cases)/sizeof(std::tuple<int,int,int>); i++) {
    int nid = std::get<0>(test_cases[i]);
    int expected_felem_bytes = std::get<1>(test_cases[i]);
    int expected_felem_words = std::get<2>(test_cases[i]);

    ASSERT_TRUE(expected_felem_bytes <= EC_MAX_BYTES);
    ASSERT_TRUE(expected_felem_words <= EC_MAX_WORDS);
    if( 0 == (expected_felem_bytes % BN_BYTES)) {
      ASSERT_EQ(expected_felem_words, expected_felem_bytes / BN_BYTES);
    } else {
      ASSERT_EQ(expected_felem_words, 1 + (expected_felem_bytes / BN_BYTES));
    }

    bssl::UniquePtr<EC_GROUP> test_group(EC_GROUP_new_by_curve_name(nid));
    ASSERT_TRUE(test_group);
    ASSERT_EQ(test_group.get()->field.N.width, expected_felem_words);
  }
}

static ECDSA_SIG * ecdsa_sign_sig(const unsigned char *dgst, int dgstlen,
                                  const BIGNUM *in_kinv, const BIGNUM *in_r,
                                  EC_KEY *ec) {
  // To track whether custom implementation was called
  EC_KEY_set_ex_data(ec, 1, (void*)"ecdsa_sign_sig");
  return nullptr;
}

static int ecdsa_sign(int type, const unsigned char *dgst, int dgstlen,
                      unsigned char *sig, unsigned int *siglen,
                      const BIGNUM *kinv, const BIGNUM *r, EC_KEY *ec) {

  ECDSA_SIG *ret = ECDSA_do_sign(dgst, dgstlen, ec);
  if (!ret) {
    *siglen = 0;
    return 0;
  }

  CBB cbb;
  CBB_init_fixed(&cbb, sig, ECDSA_size(ec));
  size_t len;
  if (!ECDSA_SIG_marshal(&cbb, ret) ||
      !CBB_finish(&cbb, nullptr, &len)) {
    ECDSA_SIG_free(ret);
    OPENSSL_PUT_ERROR(ECDSA, ECDSA_R_ENCODE_ERROR);
    *siglen = 0;
    return 0;
  }

  *siglen = (unsigned)len;

  // To track whether custom implementation was called
  EC_KEY_set_ex_data(ec, 0, (void*)"ecdsa_sign");

  ECDSA_SIG_free(ret);
  return 1;
}

static void openvpn_extkey_ec_finish(EC_KEY *ec)
{
  const EC_KEY_METHOD *ec_meth = EC_KEY_get_method(ec);
  EC_KEY_METHOD_free((EC_KEY_METHOD *) ec_meth);
}

TEST(ECTest, ECKEYMETHOD) {
  bssl::UniquePtr<EC_KEY> ec(EC_KEY_new());
  ASSERT_TRUE(ec.get());

  EC_KEY_METHOD *ec_method;
  ec_method = EC_KEY_METHOD_new(EC_KEY_OpenSSL());
  ASSERT_TRUE(ec_method);
  // We zero initialize the default struct
  ASSERT_FALSE(ec_method->finish && ec_method->sign);

  // Can only set these fields
  EC_KEY_METHOD_set_init(ec_method, NULL, openvpn_extkey_ec_finish,
                         NULL, NULL, NULL, NULL);
  ASSERT_TRUE(ec_method->finish);
  //  Checking Sign
  EC_KEY_METHOD_set_sign(ec_method, ecdsa_sign, NULL, NULL);
  ASSERT_TRUE(ec_method->sign);

  bssl::UniquePtr<EC_GROUP> group(EC_GROUP_new_by_curve_name(NID_secp224r1));
  ASSERT_TRUE(group.get());
  ASSERT_TRUE(EC_KEY_set_group(ec.get(), group.get()));
  ASSERT_TRUE(EC_KEY_generate_key(ec.get()));

  // Should get freed with EC_KEY once assigned through
  // |openvpn_extkey_ec_finish|
  ASSERT_TRUE(EC_KEY_set_method(ec.get(), ec_method));
  ASSERT_TRUE(EC_KEY_check_key(ec.get()));

  bssl::UniquePtr<EVP_PKEY> ec_key(EVP_PKEY_new());
  ASSERT_TRUE(ec_key.get());
  EVP_PKEY_assign_EC_KEY(ec_key.get(), ec.get());
  // EVP_PKEY_assign_EC_KEY doesn't up the reference, so do that here for proper test cleanup
  ASSERT_TRUE(EC_KEY_up_ref(ec.get()));
  bssl::UniquePtr<EVP_PKEY_CTX> ec_key_ctx(EVP_PKEY_CTX_new(ec_key.get(), NULL));
  ASSERT_TRUE(ec_key_ctx.get());

  // Do a signature, should call custom openvpn_extkey_ec_finish
  uint8_t digest[20];
  ASSERT_TRUE(RAND_bytes(digest, 20));
  CONSTTIME_DECLASSIFY(digest, 20);
  std::vector<uint8_t> signature(ECDSA_size(ec.get()));
  size_t sig_len = ECDSA_size(ec.get());
  ASSERT_TRUE(EVP_PKEY_sign_init(ec_key_ctx.get()));
  ASSERT_TRUE(EVP_PKEY_sign(ec_key_ctx.get(), signature.data(),
                            &sig_len, digest, 20));
  signature.resize(sig_len);

  ASSERT_STREQ(static_cast<const char*>(EC_KEY_get_ex_data(ec.get(), 0))
               , "ecdsa_sign");
  // Verify the signature
  EXPECT_TRUE(ECDSA_verify(0, digest, 20, signature.data(), signature.size(),
                           ec.get()));

  // Now test the sign_sig pointer
  EC_KEY_METHOD_set_sign(ec_method, NULL, NULL, ecdsa_sign_sig);
  ASSERT_TRUE(ec_method->sign_sig && !ec_method->sign);

  ECDSA_do_sign(digest, 20, ec.get());
  ASSERT_STREQ(static_cast<const char*>(EC_KEY_get_ex_data(ec.get(), 1)),
               "ecdsa_sign_sig");

  // Flags
  ASSERT_FALSE(EC_KEY_is_opaque(ec.get()));
  EC_KEY_METHOD_set_flags(ec_method, ECDSA_FLAG_OPAQUE);
  ASSERT_TRUE(EC_KEY_is_opaque(ec.get()));
}

TEST(ECTest, ECEngine) {
  ENGINE *engine = ENGINE_new();
  ASSERT_TRUE(engine);
  ASSERT_FALSE(ENGINE_get_EC(engine));

  EC_KEY_METHOD *eng_funcs = EC_KEY_METHOD_new(NULL);
  ASSERT_TRUE(eng_funcs);
  EC_KEY_METHOD_set_sign(eng_funcs, NULL, NULL, ecdsa_sign_sig);

  ASSERT_TRUE(ENGINE_set_EC(engine, eng_funcs));
  ASSERT_TRUE(ENGINE_get_EC(engine));

  EC_KEY *key = EC_KEY_new_method(engine);
  ASSERT_TRUE(key);

  // Call custom Engine implementation
  ECDSA_do_sign(NULL, 0, key);
  ASSERT_STREQ(static_cast<const char*>(EC_KEY_get_ex_data(key, 1))
  , "ecdsa_sign_sig");

  EC_KEY_free(key);
  ENGINE_free(engine);
  EC_KEY_METHOD_free(eng_funcs);
}

TEST(ECTest, ECPKParmatersBio) {
  bssl::UniquePtr<BIO> bio(BIO_new(BIO_s_mem()));

  EXPECT_TRUE(i2d_ECPKParameters_bio(bio.get(), EC_group_p256()));
  EXPECT_EQ(d2i_ECPKParameters_bio(bio.get(), nullptr), EC_group_p256());

  EXPECT_TRUE(i2d_ECPKParameters_bio(bio.get(), EC_group_secp256k1()));
  EXPECT_EQ(d2i_ECPKParameters_bio(bio.get(), nullptr), EC_group_secp256k1());
}

TEST(ECTest, MutableCustomECGroup) {
  bssl::UniquePtr<BN_CTX> ctx(BN_CTX_new());
  ASSERT_TRUE(ctx);
  bssl::UniquePtr<BIGNUM> p(BN_bin2bn(kP256P, sizeof(kP256P), nullptr));
  ASSERT_TRUE(p);
  bssl::UniquePtr<BIGNUM> a(BN_bin2bn(kP256A, sizeof(kP256A), nullptr));
  ASSERT_TRUE(a);
  bssl::UniquePtr<BIGNUM> b(BN_bin2bn(kP256B, sizeof(kP256B), nullptr));
  ASSERT_TRUE(b);
  bssl::UniquePtr<BIGNUM> gx(BN_bin2bn(kP256X, sizeof(kP256X), nullptr));
  ASSERT_TRUE(gx);
  bssl::UniquePtr<BIGNUM> gy(BN_bin2bn(kP256Y, sizeof(kP256Y), nullptr));
  ASSERT_TRUE(gy);
  bssl::UniquePtr<BIGNUM> order(
      BN_bin2bn(kP256Order, sizeof(kP256Order), nullptr));
  ASSERT_TRUE(order);

  bssl::UniquePtr<EC_GROUP> group(
      EC_GROUP_new_curve_GFp(p.get(), a.get(), b.get(), ctx.get()));
  ASSERT_TRUE(group);
  bssl::UniquePtr<EC_POINT> generator(EC_POINT_new(group.get()));
  ASSERT_TRUE(generator);
  ASSERT_TRUE(EC_POINT_set_affine_coordinates_GFp(
      group.get(), generator.get(), gx.get(), gy.get(), ctx.get()));
  ASSERT_TRUE(EC_GROUP_set_generator(group.get(), generator.get(), order.get(),
                                     BN_value_one()));


  // Initialize an |EC_POINT| on the corresponding curve.
  bssl::UniquePtr<EC_POINT> point(EC_POINT_new(group.get()));
  ASSERT_TRUE(EC_POINT_oct2point(
      group.get(), point.get(), kP256PublicKey_uncompressed_0x02,
      sizeof(kP256PublicKey_uncompressed_0x02), nullptr));

  EC_GROUP_set_point_conversion_form(group.get(), POINT_CONVERSION_COMPRESSED);

  // Use the saved conversion form in |group|. This should only work with
  // |EC_GROUP_new_by_curve_name_mutable|.
  std::vector<uint8_t> serialized;
  ASSERT_TRUE(EncodeECPoint(&serialized, group.get(), point.get(),
                            EC_GROUP_get_point_conversion_form(group.get())));
  EXPECT_EQ(Bytes(kP256PublicKey_compressed_0x02,
                  sizeof(kP256PublicKey_compressed_0x02)),
            Bytes(serialized));

  serialized.clear();
  EC_GROUP_set_point_conversion_form(group.get(),
                                     POINT_CONVERSION_UNCOMPRESSED);
  ASSERT_TRUE(EncodeECPoint(&serialized, group.get(), point.get(),
                            EC_GROUP_get_point_conversion_form(group.get())));
  EXPECT_EQ(Bytes(kP256PublicKey_uncompressed_0x02,
                  sizeof(kP256PublicKey_uncompressed_0x02)),
            Bytes(serialized));
}
