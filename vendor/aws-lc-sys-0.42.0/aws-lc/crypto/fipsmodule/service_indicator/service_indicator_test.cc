// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC


#include <gtest/gtest.h>

#include <openssl/aead.h>
#include <openssl/aes.h>
#include <openssl/bn.h>
#include <openssl/cipher.h>
#include <openssl/cmac.h>
#include <openssl/crypto.h>
#include <openssl/ctrdrbg.h>
#include <openssl/curve25519.h>
#include <openssl/dh.h>
#include <openssl/digest.h>
#include <openssl/ec.h>
#include <openssl/ecdsa.h>
#include <openssl/ecdh.h>
#include <openssl/evp.h>
#include <openssl/hmac.h>
#include <openssl/hkdf.h>
#include <openssl/kdf.h>
#include <openssl/md4.h>
#include <openssl/md5.h>
#include <openssl/rand.h>
#include <openssl/rsa.h>
#include <openssl/service_indicator.h>
#include <openssl/sshkdf.h>
#include <openssl/err.h>

#include "../ml_dsa/ml_dsa.h"
#include "../pqdsa/internal.h"
#include "../../test/abi_test.h"
#include "../../test/test_util.h"
#include "../bn/internal.h"
#include "../hmac/internal.h"
#include "../rand/internal.h"
#include "../sha/internal.h"
#include "../rsa/internal.h"
#include "../ec/internal.h"

static const uint8_t kAESKey[16] = {
    'A','W','S','-','L','C','C','r','y','p','t','o',' ','K', 'e','y'};
static const uint8_t kPlaintext[64] = {
    'A','W','S','-','L','C','C','r','y','p','t','o','M','o','d','u','l','e',
    ' ','F','I','P','S',' ','K','A','T',' ','E','n','c','r','y','p','t','i',
    'o','n',' ','a','n','d',' ','D','e','c','r','y','p','t','i','o','n',' ',
    'P','l','a','i','n','t','e','x','t','!'};

#if defined(AWSLC_FIPS)

// kEVPKeyGenShouldCallFIPSFunctions determines whether |EVP_PKEY_keygen_*|
// functions should call the FIPS versions of the key-generation functions.
// AWS-LC sets this to true.
static const bool kEVPKeyGenShouldCallFIPSFunctions = true;

// kCurveSecp256k1Supported determines whether secp256k1 tests should be run.
// AWS-LC sets this to true.
static const bool kCurveSecp256k1Supported = true;

// kEVPDeriveSetsServiceIndicator is true if `EVP_PKEY_derive` should set the
// service indicator for some algorithms.
// AWS-LC sets this to true.
static const bool kEVPDeriveSetsServiceIndicator = true;

template <typename T>
class TestWithNoErrors : public testing::TestWithParam<T> {
  void TearDown() override {
    if (ERR_peek_error() != 0) {
      auto f = [](const char *str, size_t len, void *unused) -> int {
        fprintf(stderr, "%s\n", str);
        return 1;
      };
      ERR_print_errors_cb(f, nullptr);
      ADD_FAILURE();
      ADD_FAILURE();
    }
  }
};

static const uint8_t kAESKey_192[24] = {
    'A','W','S','-','L','C','C','r','y','p','t','o',' ','1', '9','2', '-','b',
    'i','t',' ','K','e','y'
};

static const uint8_t kAESKey_256[32] = {
    'A','W','S','-','L','C','C','r','y','p','t','o',' ','2', '5','6', '-','b',
    'i','t',' ','L','o','n','g',' ','K','e','y','!','!','!'
};

static const uint8_t kAESIV[AES_BLOCK_SIZE] = {0};

static bssl::UniquePtr<DH> GetDH() {
  // kFFDHE2048PrivateKeyData is a 225-bit value. (225 because that's the
  // minimum private key size in
  // https://tools.ietf.org/html/rfc7919#appendix-A.1.)
  static const uint8_t kFFDHE2048PrivateKey[] = {
      0x01, 0x91, 0x17, 0x3f, 0x2a, 0x05, 0x70, 0x18, 0x7e, 0xc4,
      0x22, 0xee, 0xb7, 0x0a, 0x15, 0x2f, 0x39, 0x64, 0x58, 0xf3,
      0xb8, 0x18, 0x7b, 0xe3, 0x6b, 0xd3, 0x8a, 0x4f, 0xa1};
  bssl::UniquePtr<BIGNUM> priv(
      BN_bin2bn(kFFDHE2048PrivateKey, sizeof(kFFDHE2048PrivateKey), nullptr));
  if (!priv) {
    return nullptr;
  }
  bssl::UniquePtr<DH> dh(DH_get_rfc7919_2048());
  if (!dh || !DH_set0_key(dh.get(), nullptr, priv.get())) {
    return nullptr;
  }
  priv.release();  // |DH_set0_key| takes ownership on success.
  return dh;
}

static void DoCipherFinal(EVP_CIPHER_CTX *ctx, std::vector<uint8_t> *out,
                     bssl::Span<const uint8_t> in, FIPSStatus expect_approved) {
  FIPSStatus approved = AWSLC_NOT_APPROVED;
  size_t max_out = in.size();
  if (EVP_CIPHER_CTX_encrypting(ctx)) {
    unsigned block_size = EVP_CIPHER_CTX_block_size(ctx);
    max_out += block_size - (max_out % block_size);
  }
  out->resize(max_out);

  size_t total = 0;
  int len = 0;
  CALL_SERVICE_AND_CHECK_APPROVED(approved,
    EVP_CipherUpdate(ctx, out->data(), &len, in.data(), in.size()));
  ASSERT_EQ(approved, AWSLC_NOT_APPROVED);
  total += static_cast<size_t>(len);
  // Check if the overall service is approved by checking |EVP_CipherFinal_ex|,
  // which should be the last part of the service.
  CALL_SERVICE_AND_CHECK_APPROVED(
      approved, EVP_CipherFinal_ex(ctx, out->data() + total, &len));
  total += static_cast<size_t>(len);
  ASSERT_EQ(approved, expect_approved);
  out->resize(total);
}

static const uint8_t kTDES_EDE3_CipherText[64] = {
    0x2a, 0x17, 0x79, 0x5a, 0x9b, 0x1d, 0xd8, 0x72, 0x06, 0xc6, 0xe7,
    0x55, 0x14, 0xaa, 0x7b, 0x2a, 0x6e, 0xfc, 0x71, 0x29, 0xff, 0x9b,
    0x67, 0x73, 0x7c, 0x9e, 0x15, 0x74, 0x80, 0xc8, 0x2f, 0xca, 0x93,
    0xaa, 0x8e, 0xba, 0x2c, 0x48, 0x88, 0x51, 0xc7, 0xa4, 0xf4, 0xe3,
    0x2b, 0x33, 0xe5, 0xa1, 0x58, 0x0a, 0x08, 0x3c, 0xb9, 0xf6, 0xf1,
    0x20, 0x67, 0x02, 0x49, 0xa0, 0x92, 0x18, 0xde, 0x2b};

static const uint8_t kTDES_EDE3_CBCCipherText[64] = {
    0x2a, 0x17, 0x79, 0x5a, 0x9b, 0x1d, 0xd8, 0x72, 0xbf, 0x3f, 0xfd,
    0xe4, 0x0d, 0x66, 0x33, 0x49, 0x3b, 0x8c, 0xa6, 0xd0, 0x0a, 0x66,
    0xae, 0xf1, 0xd9, 0xa7, 0xd6, 0xfb, 0xa2, 0x39, 0x6f, 0xf6, 0x1b,
    0x8f, 0x67, 0xe1, 0x2b, 0x58, 0x1c, 0xb6, 0xa2, 0xec, 0xb3, 0xc2,
    0xe6, 0xd1, 0xcc, 0x11, 0x05, 0xdd, 0xee, 0x9d, 0x87, 0x95, 0xe9,
    0x58, 0xc7, 0xef, 0xa4, 0x6d, 0x5e, 0xd6, 0x57, 0x01};

// AES-OFB is not an approved service, and is only used to test we are not
// validating un-approved services correctly.
static const uint8_t kAESOFBCiphertext[64] = {
    0x49, 0xf5, 0x6a, 0x7d, 0x3e, 0xd7, 0xb2, 0x47, 0x35, 0xca, 0x54,
    0xf5, 0xf1, 0xb8, 0xd1, 0x48, 0x8e, 0x47, 0x09, 0x95, 0xd5, 0xa0,
    0xc6, 0xa3, 0xe4, 0x94, 0xaf, 0xd4, 0x1b, 0x64, 0x25, 0x65, 0x28,
    0x9e, 0x82, 0xba, 0x92, 0xca, 0x75, 0xb3, 0xf3, 0x78, 0x44, 0x87,
    0xd6, 0x11, 0xf9, 0x22, 0xa3, 0xf3, 0xc6, 0x1d, 0x30, 0x00, 0x5b,
    0x77, 0x18, 0x38, 0x39, 0x08, 0x5e, 0x0a, 0x56, 0x6b};

static const uint8_t kAESECBCiphertext[64] = {
    0xa4, 0xc1, 0x5c, 0x51, 0x2a, 0x2e, 0x2a, 0xda, 0xd9, 0x02, 0x23,
    0xe7, 0xa9, 0x34, 0x9d, 0xd8, 0x15, 0xc5, 0xf5, 0x55, 0x8e, 0xb0,
    0x29, 0x95, 0x48, 0x6c, 0x7f, 0xa9, 0x47, 0x19, 0x0b, 0x54, 0xe5,
    0x0f, 0x05, 0x76, 0xbb, 0xd0, 0x1a, 0x6c, 0xab, 0xe9, 0xfd, 0x5b,
    0xd8, 0x0b, 0x0a, 0xbd, 0x7f, 0xea, 0xda, 0x52, 0x07, 0x65, 0x13,
    0x6c, 0xbe, 0xfc, 0x36, 0x82, 0x4b, 0x6a, 0xc3, 0xd5};

static const uint8_t kAESECBCiphertext_192[64] = {
    0x1d, 0xc8, 0xaa, 0xa7, 0x29, 0x01, 0x17, 0x09, 0x72, 0xc6, 0xe9,
    0x63, 0x02, 0x9d, 0xeb, 0x01, 0xeb, 0xc0, 0xda, 0x82, 0x6c, 0x30,
    0x7d, 0x60, 0x1b, 0x3e, 0xc7, 0x7b, 0xe3, 0x18, 0xa2, 0x43, 0x59,
    0x15, 0x4a, 0xe4, 0x8a, 0x84, 0xda, 0x16, 0x90, 0x7b, 0xfa, 0x64,
    0x37, 0x62, 0x19, 0xf1, 0x95, 0x11, 0x61, 0x84, 0xb0, 0x70, 0x49,
    0x72, 0x9f, 0xe7, 0x3a, 0x18, 0x99, 0x01, 0xba, 0xb0};

static const uint8_t kAESECBCiphertext_256[64] = {
    0x6f, 0x2d, 0x6d, 0x7a, 0xc1, 0x8f, 0x00, 0x9f, 0x2d, 0xcf, 0xba,
    0xe6, 0x4f, 0xdd, 0xe0, 0x09, 0x5b, 0xf3, 0xa4, 0xaf, 0xce, 0x45,
    0x49, 0x6e, 0x28, 0x7b, 0x48, 0x57, 0xb5, 0xf5, 0xd8, 0x05, 0x16,
    0x0f, 0xea, 0x21, 0x0c, 0x39, 0x78, 0xee, 0x9e, 0x57, 0x3c, 0x40,
    0x11, 0x9c, 0xd9, 0x34, 0x97, 0xb9, 0xa6, 0x06, 0x40, 0x60, 0xa2,
    0x0c, 0x01, 0xe3, 0x9c, 0xda, 0x3e, 0xad, 0x99, 0x3d};

static const uint8_t kAESCBCCiphertext[64] = {
    0xa4, 0xc1, 0x5c, 0x51, 0x2a, 0x2e, 0x2a, 0xda, 0xd9, 0x02, 0x23,
    0xe7, 0xa9, 0x34, 0x9d, 0xd8, 0x5c, 0xb3, 0x65, 0x54, 0x72, 0xc8,
    0x06, 0xf1, 0x36, 0xc3, 0x97, 0x73, 0x87, 0xca, 0x44, 0x99, 0x21,
    0xb8, 0xdb, 0x93, 0x22, 0x00, 0x89, 0x7c, 0x1c, 0xea, 0x36, 0x23,
    0x18, 0xdb, 0xc1, 0x52, 0x8c, 0x23, 0x66, 0x11, 0x0d, 0xa8, 0xe9,
    0xb8, 0x08, 0x8b, 0xaa, 0x81, 0x47, 0x01, 0xa4, 0x4f};

static const uint8_t kAESCBCCiphertext_192[64] = {
    0x1d, 0xc8, 0xaa, 0xa7, 0x29, 0x01, 0x17, 0x09, 0x72, 0xc6, 0xe9,
    0x63, 0x02, 0x9d, 0xeb, 0x01, 0xb4, 0x48, 0xa8, 0x00, 0x94, 0x46,
    0x7f, 0xe3, 0xc1, 0x24, 0xea, 0x41, 0xa0, 0x2b, 0x47, 0x2f, 0xae,
    0x19, 0xce, 0x0d, 0xfa, 0x90, 0x45, 0x85, 0xce, 0xc4, 0x21, 0x0c,
    0x74, 0x38, 0x13, 0xfd, 0x64, 0xba, 0x58, 0x10, 0x37, 0x53, 0x48,
    0x66, 0x02, 0x76, 0xfb, 0xb1, 0x3a, 0x19, 0xce, 0x61};

static const uint8_t kAESCBCCiphertext_256[64] = {
    0x6f, 0x2d, 0x6d, 0x7a, 0xc1, 0x8f, 0x00, 0x9f, 0x2d, 0xcf, 0xba,
    0xe6, 0x4f, 0xdd, 0xe0, 0x09, 0x9e, 0xa8, 0x28, 0xdc, 0x27, 0xde,
    0x89, 0x26, 0xc7, 0x94, 0x6a, 0xbf, 0xb6, 0x94, 0x05, 0x08, 0x6c,
    0x39, 0x07, 0x52, 0xfa, 0x7b, 0xca, 0x7d, 0x9b, 0xbf, 0xb2, 0x43,
    0x2b, 0x69, 0xee, 0xc5, 0x68, 0x4c, 0xdd, 0x62, 0xae, 0x8d, 0x7e,
    0x71, 0x0c, 0x8f, 0x11, 0xce, 0x1d, 0x8b, 0xee, 0x94};

static const uint8_t kAESCTRCiphertext[64] = {
    0x49, 0xf5, 0x6a, 0x7d, 0x3e, 0xd7, 0xb2, 0x47, 0x35, 0xca, 0x54,
    0xf5, 0xf1, 0xb8, 0xd1, 0x48, 0xb0, 0x18, 0xc4, 0x5e, 0xeb, 0x42,
    0xfd, 0x10, 0x49, 0x1f, 0x2b, 0x11, 0xe9, 0xb0, 0x07, 0xa4, 0x00,
    0x56, 0xec, 0x25, 0x53, 0x4d, 0x70, 0x98, 0x38, 0x85, 0x5d, 0x54,
    0xab, 0x2c, 0x19, 0x13, 0x6d, 0xf3, 0x0e, 0x6f, 0x48, 0x2f, 0xab,
    0xe1, 0x82, 0xd4, 0x30, 0xa9, 0x16, 0x73, 0x93, 0xc3};

static const uint8_t kAESCTRCiphertext_192[64] = {
    0x72, 0x7d, 0xbb, 0xd4, 0x8b, 0x16, 0x8b, 0x19, 0xa4, 0xeb, 0xa6,
    0xfa, 0xa0, 0xd0, 0x2b, 0xbb, 0x9b, 0x1f, 0xbf, 0x4d, 0x67, 0xfb,
    0xea, 0x89, 0x16, 0xd7, 0xa4, 0xb6, 0xbe, 0x1a, 0x78, 0x1c, 0x3d,
    0x44, 0x49, 0xa0, 0xf2, 0xb2, 0xb3, 0x82, 0x0f, 0xdd, 0xac, 0xd6,
    0xea, 0x6e, 0x1f, 0x09, 0x8d, 0xa5, 0xdb, 0x4f, 0x3f, 0x97, 0x90,
    0x26, 0xed, 0xf6, 0xbb, 0x62, 0xb3, 0x6f, 0x52, 0x67};

static const uint8_t kAESCTRCiphertext_256[64] = {
    0x4a, 0x87, 0x44, 0x09, 0xf4, 0x1d, 0x80, 0x94, 0x51, 0x9a, 0xe4,
    0x89, 0x49, 0xcb, 0x98, 0x0d, 0x27, 0xc5, 0xba, 0x20, 0x00, 0x45,
    0xbb, 0x29, 0x75, 0xc0, 0xb7, 0x23, 0x0d, 0x81, 0x9f, 0x43, 0xaa,
    0x78, 0x89, 0xc0, 0xc4, 0x6d, 0x99, 0x0d, 0xb8, 0x9b, 0xc3, 0x25,
    0xa6, 0xd1, 0x7c, 0x98, 0x3e, 0xff, 0x06, 0x59, 0x41, 0xcf, 0xb2,
    0xd5, 0x2f, 0x95, 0xea, 0x83, 0xb1, 0x42, 0xb8, 0xb2};

static const uint8_t kAESCFBCiphertext[64] = {
    0x49, 0xf5, 0x6a, 0x7d, 0x3e, 0xd7, 0xb2, 0x47, 0x35, 0xca, 0x54,
    0xf5, 0xf1, 0xb8, 0xd1, 0x48, 0x01, 0xdc, 0xba, 0x43, 0x3a, 0x7b,
    0xbf, 0x84, 0x91, 0x49, 0xc5, 0xc9, 0xd6, 0xcf, 0x6a, 0x2c, 0x3a,
    0x66, 0x99, 0x68, 0xe3, 0xd0, 0x56, 0x05, 0xe7, 0x99, 0x7f, 0xc3,
    0xbc, 0x09, 0x13, 0xa6, 0xf0, 0xde, 0x17, 0xf4, 0x85, 0x9a, 0xee,
    0x29, 0xc3, 0x77, 0xab, 0xc4, 0xf6, 0xdb, 0xae, 0x24};

static const uint8_t kAESCCMCiphertext[64 + 4] = {
    0x7a, 0x02, 0x5d, 0x48, 0x02, 0x44, 0x78, 0x7f, 0xb4, 0x71, 0x74, 0x7b,
    0xec, 0x4d, 0x90, 0x29, 0x7b, 0xa7, 0x65, 0xbb, 0x3e, 0x80, 0x41, 0x7e,
    0xab, 0xb4, 0x58, 0x22, 0x4f, 0x86, 0xcd, 0xcc, 0xc2, 0x12, 0xeb, 0x36,
    0x39, 0x89, 0xe3, 0x66, 0x2a, 0xbf, 0xe3, 0x6c, 0x95, 0x60, 0x13, 0x9e,
    0x93, 0xcc, 0xb4, 0x06, 0xbe, 0xaf, 0x3f, 0xba, 0x13, 0x73, 0x09, 0x92,
    0xd1, 0x80, 0x73, 0xb3, 0xc3, 0xa3, 0xa4, 0x8b,
};

static const uint8_t kAESKWCiphertext[72] = {
    0x44, 0xec, 0x7d, 0x92, 0x2c, 0x9f, 0xf3, 0xe8, 0xac, 0xb1, 0xea, 0x3d,
    0x0a, 0xc7, 0x51, 0x27, 0xe8, 0x03, 0x11, 0x78, 0xe5, 0xaf, 0x8d, 0xb1,
    0x70, 0x96, 0x2e, 0xfa, 0x05, 0x48, 0x48, 0x99, 0x1a, 0x58, 0xcc, 0xfe,
    0x11, 0x36, 0x5d, 0x49, 0x98, 0x1e, 0xbb, 0xd6, 0x0b, 0xf5, 0xb9, 0x64,
    0xa4, 0x30, 0x3e, 0x60, 0xf6, 0xc5, 0xff, 0x82, 0x30, 0x9a, 0xa7, 0x48,
    0x82, 0xe2, 0x00, 0xc1, 0xe9, 0xc2, 0x73, 0x6f, 0xbc, 0x89, 0x66, 0x9d};

static const uint8_t kAESKWCiphertext_256[72] = {
    0x27, 0x5b, 0x2b, 0x05, 0x32, 0xc9, 0xc3, 0x67, 0xde, 0x16, 0xce, 0xd7,
    0xa8, 0x03, 0xc3, 0x58, 0x64, 0x8e, 0x8d, 0x53, 0x2c, 0x80, 0xac, 0x6f,
    0xf7, 0x43, 0x2a, 0xfb, 0xb5, 0x1a, 0x53, 0xaf, 0x86, 0x3c, 0xce, 0x0d,
    0x92, 0xd6, 0xce, 0x41, 0x78, 0x67, 0x8a, 0x67, 0x80, 0xbd, 0x0d, 0xa7,
    0x00, 0xb6, 0xeb, 0x3c, 0x4c, 0x68, 0xb7, 0x03, 0x14, 0x89, 0xbf, 0xe7,
    0x30, 0x41, 0x09, 0xb5, 0xe4, 0xf4, 0x91, 0x22, 0xc6, 0x2c, 0xee, 0x4a};

static const uint8_t kAESKWPCiphertext[72] = {
    0x29, 0x5e, 0xb9, 0xea, 0x96, 0xa7, 0xa5, 0xca, 0xfa, 0xeb, 0xda, 0x78,
    0x13, 0xea, 0x83, 0xca, 0x41, 0xdb, 0x4d, 0x36, 0x7d, 0x39, 0x8a, 0xd6,
    0xef, 0xd3, 0xd2, 0x2d, 0x3a, 0xc8, 0x55, 0xc8, 0x73, 0xd7, 0x79, 0x55,
    0xad, 0xc0, 0xce, 0xad, 0x12, 0x54, 0x51, 0xf0, 0x70, 0x76, 0xff, 0xe7,
    0x0c, 0xb2, 0x8e, 0xdd, 0xb6, 0x9a, 0x27, 0x74, 0x98, 0x28, 0xe0, 0xfa,
    0x11, 0xe6, 0x3f, 0x86, 0x93, 0x23, 0xf8, 0x0d, 0xcb, 0xaf, 0x2b, 0xb7};

static const uint8_t kAESCMACOutput[16] = {0xe7, 0x32, 0x43, 0xb4, 0xae, 0x79,
                                           0x08, 0x86, 0xe7, 0x9f, 0x0d, 0x3f,
                                           0x88, 0x3f, 0x1a, 0xfd};

// AES-XTS test vector from
// https://csrc.nist.gov/projects/cryptographic-algorithm-validation-program

static const uint8_t kAESXTSKey_256[64] = {
    0x1e, 0xa6, 0x61, 0xc5, 0x8d, 0x94, 0x3a, 0x0e, 0x48, 0x01, 0xe4, 0x2f,
    0x4b, 0x09, 0x47, 0x14, 0x9e, 0x7f, 0x9f, 0x8e, 0x3e, 0x68, 0xd0, 0xc7,
    0x50, 0x52, 0x10, 0xbd, 0x31, 0x1a, 0x0e, 0x7c, 0xd6, 0xe1, 0x3f, 0xfd,
    0xf2, 0x41, 0x8d, 0x8d, 0x19, 0x11, 0xc0, 0x04, 0xcd, 0xa5, 0x8d, 0xa3,
    0xd6, 0x19, 0xb7, 0xe2, 0xb9, 0x14, 0x1e, 0x58, 0x31, 0x8e, 0xea, 0x39,
    0x2c, 0xf4, 0x1b, 0x08
};

static const uint8_t kAESXTSIV_256[16] = {
    0xad, 0xf8, 0xd9, 0x26, 0x27, 0x46, 0x4a, 0xd2, 0xf0, 0x42, 0x8e, 0x84,
    0xa9, 0xf8, 0x75, 0x64
};

static const uint8_t kAESXTSPlaintext_256[32] = {
    0x2e, 0xed, 0xea, 0x52, 0xcd, 0x82, 0x15, 0xe1, 0xac, 0xc6, 0x47, 0xe8,
    0x10, 0xbb, 0xc3, 0x64, 0x2e, 0x87, 0x28, 0x7f, 0x8d, 0x2e, 0x57, 0xe3,
    0x6c, 0x0a, 0x24, 0xfb, 0xc1, 0x2a, 0x20, 0x2e
};

static const uint8_t kAESXTSCiphertext_256[32] = {
    0xcb, 0xaa, 0xd0, 0xe2, 0xf6, 0xce, 0xa3, 0xf5, 0x0b, 0x37, 0xf9, 0x34,
    0xd4, 0x6a, 0x9b, 0x13, 0x0b, 0x9d, 0x54, 0xf0, 0x7e, 0x34, 0xf3, 0x6a,
    0xf7, 0x93, 0xe8, 0x6f, 0x73, 0xc6, 0xd7, 0xdb
};

const uint8_t kDHOutput[2048 / 8] = {
    0x83, 0xf0, 0xd8, 0x4f, 0xdb, 0xe7, 0x65, 0xb6, 0x80, 0x6f, 0xa3, 0x22,
    0x9b, 0x33, 0x1c, 0x87, 0x89, 0xc8, 0x1d, 0x2c, 0xa1, 0xba, 0xa3, 0xb8,
    0xdf, 0xad, 0x42, 0xea, 0x9a, 0x75, 0xfe, 0xbf, 0xc1, 0xa8, 0xf6, 0xda,
    0xec, 0xdf, 0x48, 0x61, 0x7d, 0x7f, 0x3d, 0xab, 0xbd, 0xda, 0xd1, 0xd3,
    0xd8, 0xaf, 0x44, 0x4a, 0xba, 0x3f, 0x0e, 0x99, 0x8d, 0x11, 0xdc, 0x63,
    0xb1, 0xe0, 0x65, 0xf2, 0xb9, 0x82, 0x81, 0x8c, 0x88, 0x75, 0x8f, 0xa0,
    0x94, 0x52, 0x2a, 0x2f, 0x2d, 0x10, 0xb1, 0xf4, 0xd2, 0xdd, 0x0f, 0x8a,
    0x7e, 0x49, 0x7b, 0x1e, 0xfd, 0x8c, 0x78, 0xf9, 0x11, 0xdf, 0x80, 0x8b,
    0x2e, 0x86, 0x34, 0xbf, 0x4b, 0xca, 0x13, 0x3e, 0x85, 0x63, 0xeb, 0xe4,
    0xff, 0xec, 0xb0, 0xe8, 0x83, 0xf6, 0x2c, 0x45, 0x21, 0x90, 0x34, 0x9c,
    0x9d, 0x9d, 0xfe, 0x1a, 0x48, 0x53, 0xef, 0x97, 0xd5, 0xea, 0x6a, 0x65,
    0xf5, 0xe9, 0x9f, 0x91, 0x4f, 0xb4, 0x43, 0xe7, 0x1f, 0x0a, 0x2e, 0xdb,
    0xe6, 0x84, 0x30, 0xdb, 0xad, 0xe4, 0xaf, 0x2c, 0xf9, 0x93, 0xe8, 0x0a,
    0xab, 0x7f, 0x1c, 0xde, 0xb3, 0x80, 0xb6, 0x02, 0x42, 0xba, 0x18, 0x0d,
    0x0f, 0xc2, 0x1d, 0xa4, 0x4b, 0x2b, 0x84, 0x74, 0x10, 0x97, 0x6d, 0xdc,
    0xfa, 0x99, 0xdc, 0xba, 0xf2, 0xcb, 0x1b, 0xe8, 0x1a, 0xba, 0x0c, 0x67,
    0x60, 0x07, 0x87, 0xcc, 0xc6, 0x0d, 0xef, 0x56, 0x07, 0x80, 0x55, 0xae,
    0x03, 0xa3, 0x62, 0x31, 0x4c, 0x50, 0xf7, 0xf6, 0x87, 0xb3, 0x8d, 0xe2,
    0x11, 0x86, 0xe7, 0x9d, 0x98, 0x3c, 0x2a, 0x6c, 0x8a, 0xf0, 0xa7, 0x73,
    0x33, 0x07, 0x4e, 0x70, 0xee, 0x14, 0x4b, 0xa3, 0xf7, 0x4f, 0x8f, 0x1a,
    0xa2, 0xf6, 0xd1, 0xeb, 0x4d, 0x04, 0xf9, 0x4c, 0x07, 0x36, 0xb1, 0x46,
    0x53, 0x55, 0xb1, 0x23};

static const uint8_t kOutput_md4[MD4_DIGEST_LENGTH] = {
    0xab, 0x6b, 0xda, 0x84, 0xc0, 0x6b, 0xd0, 0x1d,
    0x19, 0xc0, 0x08, 0x11, 0x07, 0x8d, 0xce, 0x0e};

static const uint8_t kOutput_md5[MD5_DIGEST_LENGTH] = {
    0xe9, 0x70, 0xa2, 0xf7, 0x9c, 0x55, 0x57, 0xac,
    0x4e, 0x7f, 0x6b, 0xbc, 0xa3, 0xb9, 0xb7, 0xdb};

static const uint8_t kOutput_sha1[SHA_DIGEST_LENGTH] = {
    0xaa, 0x18, 0x71, 0x34, 0x00, 0x71, 0x67, 0x9f, 0xa1, 0x6d,
    0x20, 0x82, 0x91, 0x0f, 0x53, 0x0a, 0xcd, 0x6e, 0xa4, 0x34};

static const uint8_t kOutput_sha224[SHA224_DIGEST_LENGTH] = {
    0x5f, 0x1a, 0x9e, 0x68, 0x4c, 0xb7, 0x42, 0x68, 0xa0, 0x8b,
    0x87, 0xd7, 0x96, 0xb6, 0xcf, 0x1e, 0x4f, 0x85, 0x1c, 0x47,
    0xe9, 0x29, 0xb3, 0xb2, 0x73, 0x72, 0xd2, 0x69};

static const uint8_t kOutput_sha256[SHA256_DIGEST_LENGTH] = {
    0xe7, 0x63, 0x1c, 0xbb, 0x12, 0xb5, 0xbf, 0x4f, 0x99, 0x05, 0x9d,
    0x40, 0x15, 0x55, 0x34, 0x9c, 0x26, 0x36, 0xd2, 0xfe, 0x6a, 0xd6,
    0x26, 0xb4, 0x9d, 0x33, 0x07, 0xf5, 0xe6, 0x29, 0x13, 0x92};

static const uint8_t kOutput_sha384[SHA384_DIGEST_LENGTH] = {
    0x15, 0x81, 0x48, 0x8d, 0x95, 0xf2, 0x66, 0x84, 0x65, 0x94, 0x3e, 0xb9,
    0x8c, 0xda, 0x36, 0x30, 0x2a, 0x85, 0xc0, 0xcd, 0xec, 0x38, 0xa0, 0x1f,
    0x72, 0xe2, 0x68, 0xfe, 0x4e, 0xdb, 0x27, 0x8b, 0x50, 0x15, 0xe0, 0x24,
    0xc3, 0x65, 0xd1, 0x66, 0x2a, 0x3e, 0xe7, 0x00, 0x16, 0x51, 0xf5, 0x18};

static const uint8_t kOutput_sha512[SHA512_DIGEST_LENGTH] = {
    0x71, 0xcc, 0xec, 0x03, 0xf8, 0x76, 0xf4, 0x0b, 0xf1, 0x1b, 0x89,
    0x27, 0x83, 0xa1, 0x70, 0x02, 0x00, 0x2b, 0xe9, 0x3c, 0x3c, 0x65,
    0x12, 0xb9, 0xa8, 0x8c, 0xc5, 0x9d, 0xae, 0x3c, 0x73, 0x43, 0x76,
    0x4d, 0x98, 0xed, 0xd0, 0xbe, 0xb4, 0xf9, 0x0b, 0x5c, 0x5d, 0x34,
    0x46, 0x30, 0x18, 0xc2, 0x05, 0x88, 0x8a, 0x3c, 0x25, 0xcc, 0x06,
    0xf8, 0x73, 0xb9, 0xe4, 0x18, 0xa8, 0xc2, 0xf0, 0xe5};

static const uint8_t kOutput_sha512_224[SHA512_224_DIGEST_LENGTH] = {
    0xbf, 0xee, 0x89, 0x08, 0x8c, 0x9a, 0x4e, 0xa4, 0x79, 0x22, 0x6e,
    0x17, 0x9f, 0x41, 0x53, 0x06, 0xc9, 0x1e, 0x58, 0x75, 0x22, 0xfd,
    0x89, 0x0a, 0xe2, 0xbf, 0x35, 0x8e};

static const uint8_t kOutput_sha512_256[SHA512_256_DIGEST_LENGTH] = {
    0x1a, 0x78, 0x68, 0x6b, 0x69, 0x6d, 0x28, 0x14, 0x6b, 0x37, 0x11,
    0x2d, 0xfb, 0x72, 0x35, 0xfa, 0xc1, 0xc4, 0x5f, 0x5c, 0x49, 0x91,
    0x08, 0x95, 0x0b, 0x0f, 0xc9, 0x88, 0x44, 0x12, 0x01, 0x6a};

static const uint8_t kOutput_sha3_224[SHA3_224_DIGEST_LENGTH] = {
    0xd4, 0x7e, 0x2d, 0xca, 0xf9, 0x36, 0x7a, 0x73, 0x2f, 0x9b, 0x42, 0x46,
    0x25, 0x49, 0x29, 0x68, 0xfa, 0x2c, 0xc7, 0xd0, 0xb0, 0x11, 0x1c, 0x86,
    0xa6, 0xc0, 0xa1, 0x29};

static const uint8_t kOutput_sha3_256[SHA3_256_DIGEST_LENGTH] = {
    0x4a, 0x95, 0x1c, 0x1e, 0xd1, 0x58, 0x5f, 0xa3, 0xcf, 0x77, 0x24, 0x73,
    0x7b, 0xd2, 0x28, 0x55, 0x9f, 0xa5, 0xe8, 0xc6, 0x58, 0x99, 0xe3, 0xb1,
    0x88, 0x17, 0xd6, 0xc4, 0x1d, 0x3e, 0xa8, 0x4c};

static const uint8_t kOutput_sha3_384[SHA3_384_DIGEST_LENGTH] = {
    0x19, 0x97, 0xad, 0xa6, 0x45, 0x40, 0x3d, 0x10, 0xda, 0xe6, 0xd4, 0xfd,
    0xe1, 0xd3, 0x2b, 0x1b, 0xd6, 0xdb, 0x0c, 0xdb, 0xca, 0x6f, 0xae, 0x58,
    0xbf, 0x75, 0x9a, 0xf6, 0x97, 0xc6, 0xb4, 0xb4, 0xbf, 0xef, 0x3c, 0x2d,
    0xb1, 0xb3, 0x4a, 0x1d, 0xd9, 0x69, 0x58, 0x25, 0x5b, 0xd0, 0xb6, 0xad};

static const uint8_t kOutput_sha3_512[SHA3_512_DIGEST_LENGTH] = {
    0x36, 0xe5, 0xa2, 0x70, 0xa4, 0xd1, 0xc3, 0x76, 0xc6, 0x44, 0xe6, 0x00,
    0x49, 0xae, 0x7d, 0x83, 0x21, 0xdc, 0xab, 0x2e, 0xa2, 0xe3, 0x96, 0xc2,
    0xeb, 0xe6, 0x61, 0x14, 0x95, 0xd6, 0x6a, 0xf2, 0xf0, 0xa0, 0x4e, 0x93,
    0x14, 0x2f, 0x02, 0x6a, 0xdb, 0xae, 0xbd, 0x76, 0x4e, 0xb9, 0x52, 0x88,
    0x85, 0x3c, 0x64, 0xa1, 0x56, 0x6f, 0xeb, 0x76, 0x25, 0x9a, 0x4a, 0x44,
    0x23, 0xf7, 0xcf, 0x46};

// NOTE: SHAKE is a variable-length XOF; this number is chosen somewhat
//       arbitrarily for testing.
static const size_t SHAKE_OUTPUT_LENGTH = 64;

static const uint8_t kOutput_shake128[SHAKE_OUTPUT_LENGTH] = {
    0x22, 0xfe, 0x51, 0xb7, 0x9c, 0x28, 0x1c, 0x0e, 0xfc, 0x66, 0x58, 0x6a,
    0xa1, 0x60, 0x85, 0x0b, 0xe6, 0xeb, 0x20, 0x0b, 0xdb, 0x0c, 0xe7, 0xfe,
    0x49, 0x51, 0xcd, 0xc2, 0x92, 0x3f, 0xfc, 0xf8, 0xcb, 0x4b, 0x19, 0xce,
    0x80, 0x9f, 0x1f, 0xbf, 0x10, 0xf1, 0x74, 0x38, 0x7a, 0x19, 0xd0, 0xca,
    0x52, 0xf2, 0xf3, 0xd0, 0x77, 0x08, 0xe2, 0x1e, 0x20, 0x2d, 0x57, 0x25,
    0x8b, 0xd5, 0xca, 0x66};

static const uint8_t kOutput_shake256[SHAKE_OUTPUT_LENGTH] = {
    0xfc, 0xd1, 0x32, 0xd0, 0x02, 0x43, 0x7c, 0x31, 0xb2, 0x78, 0xdf, 0x34,
    0x74, 0xc8, 0x9b, 0x77, 0x08, 0x14, 0x9d, 0xde, 0x69, 0x79, 0xb5, 0x58,
    0x98, 0x01, 0x69, 0xaa, 0x64, 0x11, 0x04, 0xbe, 0xa2, 0x5f, 0xf1, 0x29,
    0x9b, 0x94, 0x03, 0x4a, 0x1e, 0x82, 0xf0, 0x9e, 0xee, 0x9b, 0xa0, 0xe3,
    0xe1, 0x5f, 0x9c, 0x13, 0xb7, 0x52, 0xef, 0x3c, 0x96, 0xf3, 0xf8, 0xf3,
    0x1f, 0x59, 0x7e, 0x41};

static const uint8_t kHMACOutput_sha1[SHA_DIGEST_LENGTH] = {
    0x34, 0xac, 0x50, 0x9b, 0xa9, 0x4c, 0x39, 0xef, 0x45, 0xa0,
    0x6b, 0xdc, 0xfc, 0xbd, 0x3d, 0x42, 0xe8, 0x0a, 0x97, 0x86};

static const uint8_t kHMACOutput_sha224[SHA224_DIGEST_LENGTH] = {
    0x30, 0x62, 0x97, 0x45, 0x9e, 0xea, 0x62, 0xe4, 0x5d, 0xbb,
    0x7d, 0x25, 0x3f, 0x77, 0x0f, 0x9d, 0xa4, 0xbd, 0x17, 0x96,
    0x23, 0x53, 0xe1, 0x76, 0xf3, 0xf8, 0x9b, 0x74};

static const uint8_t kHMACOutput_sha256[SHA256_DIGEST_LENGTH] = {
    0x68, 0x33, 0x3e, 0x74, 0x9a, 0x49, 0xab, 0x77, 0xb4, 0x1a, 0x40,
    0xd8, 0x55, 0x07, 0xa7, 0xb6, 0x48, 0xa1, 0xa5, 0xa9, 0xd1, 0x7b,
    0x85, 0xe9, 0x33, 0x09, 0x16, 0x79, 0xcc, 0xe9, 0x29, 0x97};

static const uint8_t kHMACOutput_sha384[SHA384_DIGEST_LENGTH] = {
    0xcc, 0x39, 0x22, 0x0e, 0x9f, 0x2e, 0x26, 0x4a, 0xb5, 0xf8, 0x4a, 0x0f,
    0x73, 0x51, 0x26, 0x1a, 0xf2, 0xef, 0x15, 0xf3, 0x5f, 0x77, 0xce, 0xbb,
    0x4c, 0x69, 0x86, 0x0e, 0x1f, 0x5c, 0x4d, 0xc9, 0x96, 0xd9, 0xed, 0x74,
    0x6c, 0x45, 0x05, 0x7a, 0x0e, 0x3f, 0x36, 0x8a, 0xda, 0x2a, 0x35, 0xf9};

static const uint8_t kHMACOutput_sha512[SHA512_DIGEST_LENGTH] = {
    0x4c, 0x09, 0x46, 0x50, 0x7c, 0xb3, 0xa1, 0xfa, 0xbc, 0xf2, 0xc4,
    0x4f, 0x1e, 0x3d, 0xa9, 0x0b, 0x29, 0x4e, 0x12, 0x09, 0x09, 0x32,
    0xde, 0x82, 0xa0, 0xab, 0xf6, 0x5e, 0x66, 0x19, 0xd0, 0x86, 0x9a,
    0x92, 0xe3, 0xf9, 0x13, 0xa7, 0xe6, 0xfc, 0x1a, 0x2e, 0x50, 0xda,
    0xf6, 0x8f, 0xb2, 0xd5, 0xb2, 0x6e, 0x97, 0x82, 0x25, 0x5a, 0x1e,
    0xbf, 0x9b, 0x99, 0x8c, 0xf0, 0x37, 0xe6, 0x3d, 0x40};

static const uint8_t kHMACOutput_sha512_224[SHA512_224_DIGEST_LENGTH] = {
    0xb7, 0x55, 0xfb, 0x59, 0x58, 0xa0, 0xf9, 0xa8, 0x94, 0xc2, 0x91,
    0x6b, 0xd3, 0xfc, 0xa2, 0xbc, 0xd2, 0x91, 0x09, 0xcb, 0x22, 0x0c,
    0x04, 0xc9, 0x21, 0xc1, 0x96, 0x62};

static const uint8_t kHMACOutput_sha512_256[SHA512_256_DIGEST_LENGTH] = {
    0x9c, 0x95, 0x9c, 0x03, 0xc9, 0x8c, 0x90, 0xee, 0x7a, 0xff, 0xed,
    0x26, 0xba, 0x75, 0x90, 0xd0, 0xb9, 0xd4, 0x09, 0xf5, 0x22, 0xd6,
    0xb6, 0xab, 0xa8, 0xb9, 0xae, 0x01, 0x06, 0x37, 0x8f, 0xd1};

static const uint8_t kDRBGEntropy[48] = {
    'B', 'C', 'M', ' ', 'K', 'n', 'o', 'w', 'n', ' ', 'A', 'n',
    's', 'w', 'e', 'r', ' ', 'T', 'e', 's', 't', ' ', 'D', 'B',
    'R', 'G', ' ', 'I', 'n', 'i', 't', 'i', 'a', 'l', ' ', 'E',
    'n', 't', 'r', 'o', 'p', 'y', ' ', ' ', ' ', ' ', ' ', ' '};

static const uint8_t kDRBGPersonalization[18] = {'B', 'C', 'M', 'P', 'e', 'r',
                                                 's', 'o', 'n', 'a', 'l', 'i',
                                                 'z', 'a', 't', 'i', 'o', 'n'};

static const uint8_t kDRBGAD[16] = {'B', 'C', 'M', ' ', 'D', 'R', 'B', 'G',
                                    ' ', 'K', 'A', 'T', ' ', 'A', 'D', ' '};

const uint8_t kDRBGOutput[64] = {
    0x1d, 0x63, 0xdf, 0x05, 0x51, 0x49, 0x22, 0x46, 0xcd, 0x9b, 0xc5,
    0xbb, 0xf1, 0x5d, 0x44, 0xae, 0x13, 0x78, 0xb1, 0xe4, 0x7c, 0xf1,
    0x96, 0x33, 0x3d, 0x60, 0xb6, 0x29, 0xd4, 0xbb, 0x6b, 0x44, 0xf9,
    0xef, 0xd9, 0xf4, 0xa2, 0xba, 0x48, 0xea, 0x39, 0x75, 0x59, 0x32,
    0xf7, 0x31, 0x2c, 0x98, 0x14, 0x2b, 0x49, 0xdf, 0x02, 0xb6, 0x5d,
    0x71, 0x09, 0x50, 0xdb, 0x23, 0xdb, 0xe5, 0x22, 0x95};

static const uint8_t kDRBGEntropy2[48] = {
    'B', 'C', 'M', ' ', 'K', 'n', 'o', 'w', 'n', ' ', 'A', 'n',
    's', 'w', 'e', 'r', ' ', 'T', 'e', 's', 't', ' ', 'D', 'B',
    'R', 'G', ' ', 'R', 'e', 's', 'e', 'e', 'd', ' ', 'E', 'n',
    't', 'r', 'o', 'p', 'y', ' ', ' ', ' ', ' ', ' ', ' ', ' '};

static const uint8_t kDRBGReseedOutput[64] = {
    0xa4, 0x77, 0x05, 0xdb, 0x14, 0x11, 0x76, 0x71, 0x42, 0x5b, 0xd8,
    0xd7, 0xa5, 0x4f, 0x8b, 0x39, 0xf2, 0x10, 0x4a, 0x50, 0x5b, 0xa2,
    0xc8, 0xf0, 0xbb, 0x3e, 0xa1, 0xa5, 0x90, 0x7d, 0x54, 0xd9, 0xc6,
    0xb0, 0x96, 0xc0, 0x2b, 0x7e, 0x9b, 0xc9, 0xa1, 0xdd, 0x78, 0x2e,
    0xd5, 0xa8, 0x66, 0x16, 0xbd, 0x18, 0x3c, 0xf2, 0xaa, 0x7a, 0x2b,
    0x37, 0xf9, 0xab, 0x35, 0x64, 0x15, 0x01, 0x3f, 0xc4,
};

static const uint8_t kTLSSecret[32] = {
    0xbf, 0xe4, 0xb7, 0xe0, 0x26, 0x55, 0x5f, 0x6a, 0xdf, 0x5d, 0x27,
    0xd6, 0x89, 0x99, 0x2a, 0xd6, 0xf7, 0x65, 0x66, 0x07, 0x4b, 0x55,
    0x5f, 0x64, 0x55, 0xcd, 0xd5, 0x77, 0xa4, 0xc7, 0x09, 0x61,
};
static const char kTLSLabel[] = "FIPS self test";
static const char extendedMasterSecretLabel[] = "extended master secret";

static const uint8_t kTLSSeed1[16] = {
    0x8f, 0x0d, 0xe8, 0xb6, 0x90, 0x8f, 0xb1, 0xd2,
    0x6d, 0x51, 0xf4, 0x79, 0x18, 0x63, 0x51, 0x65,
};
static const uint8_t kTLSSeed2[16] = {
    0x7d, 0x24, 0x1a, 0x9d, 0x3c, 0x59, 0xbf, 0x3c,
    0x31, 0x1e, 0x2b, 0x21, 0x41, 0x8d, 0x32, 0x81,
};

static const uint8_t kTLSOutput1_mdsha1[32] = {
    0x36, 0xa9, 0x31, 0xb0, 0x43, 0xe3, 0x64, 0x72, 0xb9, 0x47, 0x54,
    0x0d, 0x8a, 0xfc, 0xe3, 0x5c, 0x1c, 0x15, 0x67, 0x7e, 0xa3, 0x5d,
    0xf2, 0x3a, 0x57, 0xfd, 0x50, 0x16, 0xe1, 0xa4, 0xa6, 0x37,
};

static const uint8_t kTLSOutput_md[32] = {
    0x79, 0xef, 0x46, 0xc4, 0x35, 0xbc, 0xe5, 0xda, 0xd3, 0x66, 0x91,
    0xdc, 0x86, 0x09, 0x41, 0x66, 0xf2, 0x0c, 0xeb, 0xe6, 0xab, 0x5c,
    0x58, 0xf4, 0x65, 0xce, 0x2f, 0x5f, 0x4b, 0x34, 0x1e, 0xa1,
};

static const uint8_t kTLSOutput_sha1[32] = {
    0xbb, 0x0a, 0x73, 0x52, 0xf8, 0x85, 0xd7, 0xbd, 0x12, 0x34, 0x78,
    0x3b, 0x54, 0x4c, 0x75, 0xfe, 0xd7, 0x23, 0x6e, 0x22, 0x3f, 0x42,
    0x34, 0x99, 0x57, 0x6b, 0x14, 0xc4, 0xc8, 0xae, 0x9f, 0x4c,
};

static const uint8_t kTLSOutput_sha224[32] = {
    0xdd, 0xaf, 0x6f, 0xaa, 0xd9, 0x2b, 0x3d, 0xb9, 0x46, 0x4c, 0x55,
    0x8a, 0xf7, 0xa6, 0x9b, 0x0b, 0x35, 0xcc, 0x07, 0xa7, 0x55, 0x5b,
    0x5e, 0x39, 0x12, 0xc0, 0xd4, 0x30, 0xdf, 0x0c, 0xdf, 0x6b,
};

static const uint8_t kTLSOutput1_sha256[32] = {
    0x67, 0x85, 0xde, 0x60, 0xfc, 0x0a, 0x83, 0xe9, 0xa2, 0x2a, 0xb3,
    0xf0, 0x27, 0x0c, 0xba, 0xf7, 0xfa, 0x82, 0x3d, 0x14, 0x77, 0x1d,
    0x86, 0x29, 0x79, 0x39, 0x77, 0x8a, 0xd5, 0x0e, 0x9d, 0x32,
};

static const uint8_t kTLSOutput1_sha384[32] = {
    0x75, 0x15, 0x3f, 0x44, 0x7a, 0xfd, 0x34, 0xed, 0x2b, 0x67, 0xbc,
    0xd8, 0x57, 0x96, 0xab, 0xff, 0xf4, 0x0c, 0x05, 0x94, 0x02, 0x23,
    0x81, 0xbf, 0x0e, 0xd2, 0xec, 0x7c, 0xe0, 0xa7, 0xc3, 0x7d,
};

static const uint8_t kTLSOutput1_sha512[32] = {
    0x68, 0xb9, 0xc8, 0x4c, 0xf5, 0x51, 0xfc, 0x7a, 0x1f, 0x6c, 0xe5,
    0x43, 0x73, 0x80, 0x53, 0x7c, 0xae, 0x76, 0x55, 0x67, 0xe0, 0x79,
    0xbf, 0x3a, 0x53, 0x71, 0xb7, 0x9c, 0xb5, 0x03, 0x15, 0x3f,
};

static const uint8_t kTLSOutput2_mdsha1[32] = {
    0x21, 0x72, 0x18, 0xbe, 0x5a, 0xdc, 0xf7, 0x29, 0x1e, 0x81, 0x15,
    0x46, 0x8d, 0x7f, 0x7e, 0x93, 0xac, 0xe5, 0x45, 0x26, 0x1a, 0x17,
    0x7c, 0x3a, 0xd4, 0x17, 0xaa, 0xe6, 0xfc, 0x15, 0x55, 0x69
};

static const uint8_t kTLSOutput2_sha256[32] = {
    0xfc, 0xa0, 0x34, 0x55, 0x73, 0x01, 0x22, 0x19, 0x93, 0x40, 0x56,
    0x09, 0xfc, 0x8e, 0x42, 0xe4, 0x1a, 0x0c, 0xfa, 0x55, 0xaf, 0x19,
    0xbb, 0x38, 0x64, 0x63, 0x4b, 0xfb, 0x79, 0x19, 0x8a, 0xfc
};

static const uint8_t kTLSOutput2_sha384[32] = {
    0xc5, 0x37, 0xd2, 0x5e, 0x6d, 0xaf, 0x50, 0xd2, 0x1e, 0xe6, 0xd6,
    0x26, 0x50, 0xbc, 0x36, 0xb3, 0xc5, 0xf9, 0x1c, 0x8f, 0x59, 0xfd,
    0xf9, 0x0e, 0xcb, 0xe4, 0x0b, 0xa9, 0xaf, 0xa5, 0x48, 0x01
};

static const uint8_t kTLSOutput2_sha512[32] = {
    0x12, 0xfe, 0x4f, 0xd9, 0x98, 0x64, 0x27, 0x3f, 0x82, 0xbb, 0xde,
    0x87, 0x1b, 0x43, 0x01, 0xc2, 0x6c, 0x9b, 0xaa, 0x89, 0xd0, 0x47,
    0x3d, 0x56, 0xa8, 0xf5, 0x9f, 0x2e, 0x8d, 0xbb, 0x77, 0x57
};

static const uint8_t kAESGCMCiphertext_128[64 + 16] = {
    0x38, 0x71, 0xcb, 0x61, 0x70, 0x60, 0x13, 0x8b, 0x2f, 0x91, 0x09, 0x7f,
    0x83, 0x20, 0x0f, 0x1f, 0x71, 0xe2, 0x47, 0x46, 0x6f, 0x5f, 0xa8, 0xad,
    0xa8, 0xfc, 0x0a, 0xfd, 0x36, 0x65, 0x84, 0x90, 0x28, 0x2b, 0xcb, 0x4f,
    0x68, 0xae, 0x09, 0xba, 0xae, 0xdd, 0xdb, 0x91, 0xcc, 0x38, 0xb3, 0xad,
    0x10, 0x84, 0xb8, 0x45, 0x36, 0xf3, 0x96, 0xb4, 0xef, 0xba, 0xda, 0x10,
    0xf8, 0x8b, 0xf3, 0xda, 0x91, 0x1f, 0x8c, 0xd8, 0x39, 0x7b, 0x1c, 0xfd,
    0xe7, 0x99, 0x7d, 0xb7, 0x22, 0x69, 0x67, 0xbd,
};

static const uint8_t kAESGCMCiphertext_192[64 + 16] = {
    0x05, 0x63, 0x6e, 0xe4, 0xd1, 0x9f, 0xd0, 0x91, 0x18, 0xc9, 0xf8, 0xfd,
    0xc2, 0x62, 0x09, 0x05, 0x91, 0xb4, 0x92, 0x66, 0x18, 0xe7, 0x93, 0x6a,
    0xc7, 0xde, 0x81, 0x36, 0x93, 0x79, 0x45, 0x34, 0xc0, 0x6d, 0x14, 0x94,
    0x93, 0x39, 0x2b, 0x7f, 0x4f, 0x10, 0x1c, 0xa5, 0xfe, 0x3b, 0x37, 0xd7,
    0x0a, 0x98, 0xd7, 0xb5, 0xe0, 0xdc, 0xe4, 0x9f, 0x36, 0x40, 0xad, 0x03,
    0xbf, 0x53, 0xe0, 0x7c, 0x3f, 0x57, 0x4f, 0x80, 0x99, 0xe6, 0x90, 0x4e,
    0x59, 0x2e, 0xe0, 0x76, 0x53, 0x09, 0xc3, 0xd3,
};

static const uint8_t kAESGCMCiphertext_256[64 + 16] = {
    0x92, 0x5f, 0xae, 0x84, 0xe7, 0x40, 0xfa, 0x1e, 0xaf, 0x8f, 0x97, 0x0e,
    0x8e, 0xdd, 0x6a, 0x94, 0x22, 0xee, 0x4f, 0x70, 0x66, 0xbf, 0xb1, 0x99,
    0x05, 0xbd, 0xd0, 0xd7, 0x91, 0x54, 0xaf, 0xe1, 0x52, 0xc9, 0x4e, 0x55,
    0xa5, 0x23, 0x62, 0x8b, 0x23, 0x40, 0x90, 0x56, 0xe0, 0x68, 0x63, 0xe5,
    0x7e, 0x5b, 0xbe, 0x96, 0x7b, 0xc4, 0x16, 0xf9, 0xbe, 0x18, 0x06, 0x79,
    0x8f, 0x99, 0x35, 0xe3, 0x2a, 0x82, 0xb5, 0x5e, 0x8a, 0x06, 0xbe, 0x99,
    0x57, 0xb1, 0x76, 0xe1, 0xc5, 0xaa, 0x82, 0xe7,
};

static const struct AEADTestVector {
  const char* name;
  const EVP_AEAD *aead;
  const uint8_t *key;
  const int key_length;
  const uint8_t *expected_ciphertext;
  const int cipher_text_length;
  const FIPSStatus expect_approved;
  const bool test_repeat_nonce;
} kAEADTestVectors[] = {
    // Internal IV usage of AES-GCM is approved.
    {
        "AES-GCM 128-bit key internal iv test",
        EVP_aead_aes_128_gcm_randnonce(),
        kAESKey,
        sizeof(kAESKey),
        nullptr,
        0,
        AWSLC_APPROVED,
        false,
    },
    {
        "AES-GCM 256-bit key internal iv test",
        EVP_aead_aes_256_gcm_randnonce(),
        kAESKey_256,
        sizeof(kAESKey_256),
        nullptr,
        0,
        AWSLC_APPROVED,
        false,
    },
    // External IV usage of AES-GCM is not approved unless used within a TLS
    // context.
    {
        "Generic AES-GCM 128-bit key external iv test",
        EVP_aead_aes_128_gcm(),
        kAESKey,
        sizeof(kAESKey),
        kAESGCMCiphertext_128,
        sizeof(kAESGCMCiphertext_128),
        AWSLC_NOT_APPROVED,
        false,
    },
    {
        "Generic AES-GCM 192-bit key external iv test",
        EVP_aead_aes_192_gcm(),
        kAESKey_192,
        24,
        kAESGCMCiphertext_192,
        sizeof(kAESGCMCiphertext_192),
        AWSLC_NOT_APPROVED,
        false,
    },
    {
        "Generic AES-GCM 256-bit key external iv test",
        EVP_aead_aes_256_gcm(),
        kAESKey_256,
        sizeof(kAESKey_256),
        kAESGCMCiphertext_256,
        sizeof(kAESGCMCiphertext_256),
        AWSLC_NOT_APPROVED,
        false,
    },
    // External IV usage of AEAD AES-GCM APIs specific for TLS is approved.
    {
        "TLS1.2 AES-GCM 128-bit key external iv test",
        EVP_aead_aes_128_gcm_tls12(),
        kAESKey,
        sizeof(kAESKey),
        kAESGCMCiphertext_128,
        sizeof(kAESGCMCiphertext_128),
        AWSLC_APPROVED,
        true,
    },
    {
        "TLS1.2 AES-GCM 256-bit key external iv test",
        EVP_aead_aes_256_gcm_tls12(),
        kAESKey_256,
        sizeof(kAESKey_256),
        kAESGCMCiphertext_256,
        sizeof(kAESGCMCiphertext_256),
        AWSLC_APPROVED,
        true,
    },
    {
        "TLS1.3 AES-GCM 128-bit key external iv test",
        EVP_aead_aes_128_gcm_tls13(),
        kAESKey,
        sizeof(kAESKey),
        kAESGCMCiphertext_128,
        sizeof(kAESGCMCiphertext_128),
        AWSLC_APPROVED,
        true,
    },
    {
        "TLS1.3 AES-GCM 256-bit key external iv test",
        EVP_aead_aes_256_gcm_tls13(),
        kAESKey_256,
        sizeof(kAESKey_256),
        kAESGCMCiphertext_256,
        sizeof(kAESGCMCiphertext_256),
        AWSLC_APPROVED,
        true,
    },
    // 128 bit keys with 32 bit tag lengths are approved for AES-CCM.
    {
        "AES-CCM 128-bit key test",
        EVP_aead_aes_128_ccm_bluetooth(),
        kAESKey,
        sizeof(kAESKey),
        kAESCCMCiphertext,
        sizeof(kAESCCMCiphertext),
        AWSLC_APPROVED,
        false,
    },
};

class AEADServiceIndicatorTest : public TestWithNoErrors<AEADTestVector> {};

INSTANTIATE_TEST_SUITE_P(All, AEADServiceIndicatorTest,
                         testing::ValuesIn(kAEADTestVectors));

TEST_P(AEADServiceIndicatorTest, EVP_AEAD) {
  const AEADTestVector &test = GetParam();
  SCOPED_TRACE(test.name);

  FIPSStatus approved = AWSLC_NOT_APPROVED;

  bssl::ScopedEVP_AEAD_CTX aead_ctx;
  std::vector<uint8_t> nonce(EVP_AEAD_nonce_length(test.aead), 0);
  std::vector<uint8_t> encrypt_output(256);
  std::vector<uint8_t> decrypt_output(256);
  size_t out_len;

  // Test running the EVP_AEAD_CTX interfaces one by one directly, and check
  // |EVP_AEAD_CTX_seal| and |EVP_AEAD_CTX_open| for approval at the end.
  // |EVP_AEAD_CTX_init| should not be approved because the function does not
  // indicate that a service has been fully completed yet.
  CALL_SERVICE_AND_CHECK_APPROVED(approved,
      ASSERT_TRUE(EVP_AEAD_CTX_init(aead_ctx.get(), test.aead, test.key,
                                  test.key_length, 0, nullptr)));
  EXPECT_EQ(approved, AWSLC_NOT_APPROVED);
  CALL_SERVICE_AND_CHECK_APPROVED(approved,
      ASSERT_TRUE(EVP_AEAD_CTX_seal(aead_ctx.get(), encrypt_output.data(),
                                  &out_len, encrypt_output.size(), nonce.data(),
                                  EVP_AEAD_nonce_length(test.aead), kPlaintext,
                                  sizeof(kPlaintext), nullptr, 0)));
  EXPECT_EQ(approved, test.expect_approved);
  encrypt_output.resize(out_len);
  if (test.expected_ciphertext) {
    EXPECT_EQ(Bytes(test.expected_ciphertext, test.cipher_text_length),
              Bytes(encrypt_output));
  }

  CALL_SERVICE_AND_CHECK_APPROVED(approved, ASSERT_TRUE(
      EVP_AEAD_CTX_open(aead_ctx.get(), decrypt_output.data(),&out_len,
            decrypt_output.size(), nonce.data(), nonce.size(),
            encrypt_output.data(), out_len, nullptr, 0)));
  // Decryption doesn't have nonce uniqueness requirements and so is always
  // approved for approved key lengths.
  EXPECT_EQ(approved, test.key_length != 24 ? AWSLC_APPROVED
                                            : AWSLC_NOT_APPROVED);
  decrypt_output.resize(out_len);
  EXPECT_EQ(Bytes(kPlaintext), Bytes(decrypt_output));

  // Second call when encrypting using the same nonce for AES-GCM TLS specific
  // functions should fail and return |AWSLC_NOT_APPROVED|.
  if (test.test_repeat_nonce) {
    CALL_SERVICE_AND_CHECK_APPROVED(
        approved, ASSERT_FALSE(
        EVP_AEAD_CTX_seal(aead_ctx.get(), encrypt_output.data(), &out_len,
                          encrypt_output.size(), nonce.data(), nonce.size(),
                          kPlaintext, sizeof(kPlaintext), nullptr, 0)));
    EXPECT_EQ(approved, AWSLC_NOT_APPROVED);
    EXPECT_TRUE(
        ErrorEquals(ERR_get_error(), ERR_LIB_CIPHER, CIPHER_R_INVALID_NONCE));
  }
}

static const struct CipherTestVector {
  const EVP_CIPHER *cipher;
  const uint8_t *key;
  const int key_length;
  const uint8_t *iv;
  const int iv_length;
  const uint8_t *plaintext;
  const int plaintext_length;
  const uint8_t *expected_ciphertext;
  const int cipher_text_length;
  const FIPSStatus expect_approved;
} kTestVectors[] = {
    {
        EVP_aes_128_ecb(),
        kAESKey,
        sizeof(kAESKey),
        nullptr,
        0,
        kPlaintext,
        sizeof(kPlaintext),
        kAESECBCiphertext,
        sizeof(kAESECBCiphertext),
        AWSLC_APPROVED,
    },
    {
        EVP_aes_192_ecb(),
        kAESKey_192,
        sizeof(kAESKey_192),
        nullptr,
        0,
        kPlaintext,
        sizeof(kPlaintext),
        kAESECBCiphertext_192,
        sizeof(kAESECBCiphertext_192),
        AWSLC_APPROVED,
    },
    {
        EVP_aes_256_ecb(),
        kAESKey_256,
        sizeof(kAESKey_256),
        nullptr,
        0,
        kPlaintext,
        sizeof(kPlaintext),
        kAESECBCiphertext_256,
        sizeof(kAESECBCiphertext_256),
        AWSLC_APPROVED,
    },
    {
        EVP_aes_128_cbc(),
        kAESKey,
        sizeof(kAESKey),
        kAESIV,
        sizeof(kAESIV),
        kPlaintext,
        sizeof(kPlaintext),
        kAESCBCCiphertext,
        sizeof(kAESCBCCiphertext),
        AWSLC_APPROVED,
    },
    {
        EVP_aes_192_cbc(),
        kAESKey_192,
        sizeof(kAESKey_192),
        kAESIV,
        sizeof(kAESIV),
        kPlaintext,
        sizeof(kPlaintext),
        kAESCBCCiphertext_192,
        sizeof(kAESCBCCiphertext_192),
        AWSLC_APPROVED,
    },
    {
        EVP_aes_256_cbc(),
        kAESKey_256,
        sizeof(kAESKey_256),
        kAESIV,
        sizeof(kAESIV),
        kPlaintext,
        sizeof(kPlaintext),
        kAESCBCCiphertext_256,
        sizeof(kAESCBCCiphertext_256),
        AWSLC_APPROVED,
    },
    {
        EVP_aes_128_ctr(),
        kAESKey,
        sizeof(kAESKey),
        kAESIV,
        sizeof(kAESIV),
        kPlaintext,
        sizeof(kPlaintext),
        kAESCTRCiphertext,
        sizeof(kAESCTRCiphertext),
        AWSLC_APPROVED,
    },
    {
        EVP_aes_192_ctr(),
        kAESKey_192,
        sizeof(kAESKey_192),
        kAESIV,
        sizeof(kAESIV),
        kPlaintext,
        sizeof(kPlaintext),
        kAESCTRCiphertext_192,
        sizeof(kAESCTRCiphertext_192),
        AWSLC_APPROVED,
    },
    {
        EVP_aes_256_ctr(),
        kAESKey_256,
        sizeof(kAESKey_256),
        kAESIV,
        sizeof(kAESIV),
        kPlaintext,
        sizeof(kPlaintext),
        kAESCTRCiphertext_256,
        sizeof(kAESCTRCiphertext_256),
        AWSLC_APPROVED,
    },
    {
        EVP_aes_128_ofb(),
        kAESKey,
        sizeof(kAESKey),
        kAESIV,
        sizeof(kAESIV),
        kPlaintext,
        sizeof(kPlaintext),
        kAESOFBCiphertext,
        sizeof(kAESOFBCiphertext),
        AWSLC_NOT_APPROVED,
    },
    {
        EVP_aes_256_xts(),
        kAESXTSKey_256,
        sizeof(kAESXTSKey_256),
        kAESXTSIV_256,
        sizeof(kAESXTSIV_256),
        kAESXTSPlaintext_256,
        sizeof(kAESXTSPlaintext_256),
        kAESXTSCiphertext_256,
        sizeof(kAESXTSCiphertext_256),
        AWSLC_APPROVED,
    },
    {
        EVP_aes_256_wrap(),
        kAESKey_256,
        sizeof(kAESKey_256),
        nullptr,
        0,
        kPlaintext,
        sizeof(kPlaintext),
        kAESKWCiphertext_256,
        sizeof(kAESKWCiphertext_256),
        AWSLC_NOT_APPROVED,
    },
    {
        EVP_des_ede3(),
        kAESKey_192,
        sizeof(kAESKey_192),
        nullptr,
        0,
        kPlaintext,
        sizeof(kPlaintext),
        kTDES_EDE3_CipherText,
        sizeof(kTDES_EDE3_CipherText),
        AWSLC_NOT_APPROVED,
    },
    {
        EVP_des_ede3_cbc(),
        kAESKey_192,
        sizeof(kAESKey_192),
        nullptr,
        0,
        kPlaintext,
        sizeof(kPlaintext),
        kTDES_EDE3_CBCCipherText,
        sizeof(kTDES_EDE3_CBCCipherText),
        AWSLC_NOT_APPROVED,
    },
};

class EVPServiceIndicatorTest : public TestWithNoErrors<CipherTestVector> {};

static void TestOperation(const EVP_CIPHER *cipher, bool encrypt,
                          const std::vector<uint8_t> key,
                          const std::vector<uint8_t> iv,
                          const std::vector<uint8_t> plaintext,
                          const std::vector<uint8_t> ciphertext,
                          FIPSStatus expect_approved) {
  FIPSStatus approved = AWSLC_NOT_APPROVED;
  std::vector<uint8_t> in, out;
  if (encrypt) {
    in = plaintext;
    out = ciphertext;
  } else {
    in = ciphertext;
    out = plaintext;
  }

  bssl::ScopedEVP_CIPHER_CTX ctx;
  // Test running the EVP_Cipher interfaces one by one directly, and check
  // |EVP_EncryptFinal_ex| and |EVP_DecryptFinal_ex| for approval at the end.
  CALL_SERVICE_AND_CHECK_APPROVED(approved,
    ASSERT_TRUE(EVP_CipherInit_ex(ctx.get(), cipher, nullptr, nullptr, nullptr,
                                encrypt ? 1 : 0)));
  EXPECT_EQ(approved, AWSLC_NOT_APPROVED);
  if (iv.size() > 0) {
    // IV specified for the test, so the context's IV length should match.
    ASSERT_LE(EVP_CIPHER_CTX_iv_length(ctx.get()), iv.size());
  } else {
    // IV not specified, and the context defaults to the standard AES IV length
    // even if we're not going to use it.
    ASSERT_LE(EVP_CIPHER_CTX_iv_length(ctx.get()), sizeof(kAESIV));
  }

  ASSERT_TRUE(EVP_CIPHER_CTX_set_key_length(ctx.get(), key.size()));
  CALL_SERVICE_AND_CHECK_APPROVED(approved,
    ASSERT_TRUE(EVP_CipherInit_ex(ctx.get(), cipher, nullptr, key.data(),
                                iv.data(), encrypt ? 1 : 0)));
  EXPECT_EQ(approved, AWSLC_NOT_APPROVED);
  ASSERT_TRUE(EVP_CIPHER_CTX_set_padding(ctx.get(), 0));
  std::vector<uint8_t> encrypt_result;
  DoCipherFinal(ctx.get(), &encrypt_result, in, expect_approved);
  EXPECT_EQ(Bytes(out), Bytes(encrypt_result));

  // Test using the one-shot |EVP_Cipher| function for approval.
  bssl::ScopedEVP_CIPHER_CTX ctx2;
  uint8_t output[256];
  CALL_SERVICE_AND_CHECK_APPROVED(approved,
    ASSERT_TRUE(EVP_CipherInit_ex(ctx2.get(), cipher, nullptr, key.data(),
                                iv.data(), encrypt ? 1 : 0)));
  EXPECT_EQ(approved, AWSLC_NOT_APPROVED);
  CALL_SERVICE_AND_CHECK_APPROVED(
      approved, EVP_Cipher(ctx2.get(), output, in.data(), in.size()));
  EXPECT_EQ(approved, expect_approved);
  EXPECT_EQ(Bytes(out), Bytes(output, out.size()));
}

INSTANTIATE_TEST_SUITE_P(All, EVPServiceIndicatorTest,
                         testing::ValuesIn(kTestVectors));

TEST_P(EVPServiceIndicatorTest, EVP_Ciphers) {
  const CipherTestVector &test = GetParam();

  const EVP_CIPHER *cipher = test.cipher;
  std::vector<uint8_t> key(test.key, test.key + test.key_length);
  std::vector<uint8_t> iv(test.iv, test.iv + test.iv_length);
  std::vector<uint8_t> plaintext(test.plaintext,
                                 test.plaintext + test.plaintext_length);
  std::vector<uint8_t> ciphertext(
      test.expected_ciphertext,
      test.expected_ciphertext + test.cipher_text_length);

  TestOperation(cipher, true /* encrypt */, key, iv, plaintext, ciphertext,
                test.expect_approved);
  TestOperation(cipher, false /* decrypt */, key, iv, plaintext, ciphertext,
                test.expect_approved);
}

static const struct DigestTestVector {
  // name is the name of the digest test.
  const char *name;
  // length of digest.
  const int length;
  // func is the digest to test.
  const EVP_MD *(*func)();
  // one_shot_func is the convenience one-shot version of the digest.
  uint8_t *(*one_shot_func)(const uint8_t *, size_t, uint8_t *);
  // expected_digest is the expected digest.
  const uint8_t *expected_digest;
  // expected to be approved or not.
  const FIPSStatus expect_approved;
} kDigestTestVectors[] = {
    {
        "MD4",
        MD4_DIGEST_LENGTH,
        &EVP_md4,
        &MD4,
        kOutput_md4,
        AWSLC_NOT_APPROVED,
    },
    {
        "MD5",
        MD5_DIGEST_LENGTH,
        &EVP_md5,
        &MD5,
        kOutput_md5,
        AWSLC_NOT_APPROVED,
    },
    {
        "SHA-1",
        SHA_DIGEST_LENGTH,
        &EVP_sha1,
        &SHA1,
        kOutput_sha1,
        AWSLC_APPROVED,
    },
    {
        "SHA-224",
        SHA224_DIGEST_LENGTH,
        &EVP_sha224,
        &SHA224,
        kOutput_sha224,
        AWSLC_APPROVED,
    },
    {
        "SHA-256",
        SHA256_DIGEST_LENGTH,
        &EVP_sha256,
        &SHA256,
        kOutput_sha256,
        AWSLC_APPROVED,
    },
    {
        "SHA-384",
        SHA384_DIGEST_LENGTH,
        &EVP_sha384,
        &SHA384,
        kOutput_sha384,
        AWSLC_APPROVED,
    },
    {
        "SHA-512",
        SHA512_DIGEST_LENGTH,
        &EVP_sha512,
        &SHA512,
        kOutput_sha512,
        AWSLC_APPROVED,
    },
    {
        "SHA-512/224",
        SHA512_224_DIGEST_LENGTH,
        &EVP_sha512_224,
        &SHA512_224,
        kOutput_sha512_224,
        AWSLC_APPROVED,
    },
    {
        "SHA-512/256",
        SHA512_256_DIGEST_LENGTH,
        &EVP_sha512_256,
        &SHA512_256,
        kOutput_sha512_256,
        AWSLC_APPROVED,
    },
    {
        "SHA3-224",
        SHA3_224_DIGEST_LENGTH,
        &EVP_sha3_224,
        &SHA3_224,
        kOutput_sha3_224,
        AWSLC_APPROVED,
    },
    {
        "SHA3-256",
        SHA3_256_DIGEST_LENGTH,
        &EVP_sha3_256,
        &SHA3_256,
        kOutput_sha3_256,
        AWSLC_APPROVED,
    },
    {
        "SHA3-384",
        SHA3_384_DIGEST_LENGTH,
        &EVP_sha3_384,
        &SHA3_384,
        kOutput_sha3_384,
        AWSLC_APPROVED,
    },
    {
        "SHA3-512",
        SHA3_512_DIGEST_LENGTH,
        &EVP_sha3_512,
        &SHA3_512,
        kOutput_sha3_512,
        AWSLC_APPROVED,
    },
};

class EVPMDServiceIndicatorTest : public TestWithNoErrors<DigestTestVector> {};

INSTANTIATE_TEST_SUITE_P(All, EVPMDServiceIndicatorTest,
                         testing::ValuesIn(kDigestTestVectors));

TEST_P(EVPMDServiceIndicatorTest, EVP_Digests) {
  const DigestTestVector &test = GetParam();
  SCOPED_TRACE(test.name);

  FIPSStatus approved = AWSLC_NOT_APPROVED;
  bssl::ScopedEVP_MD_CTX ctx;
  std::vector<uint8_t> digest(test.length);
  unsigned digest_len;

  // Test running the EVP_Digest interfaces one by one directly, and check
  // |EVP_DigestFinal_ex| for approval at the end. |EVP_DigestInit_ex| and
  // |EVP_DigestUpdate| should not be approved, because the functions do not
  // indicate that a service has been fully completed yet.
  CALL_SERVICE_AND_CHECK_APPROVED(approved,
      ASSERT_TRUE(EVP_DigestInit_ex(ctx.get(), test.func(), nullptr)));
  EXPECT_EQ(approved, AWSLC_NOT_APPROVED);
  CALL_SERVICE_AND_CHECK_APPROVED(approved,
      ASSERT_TRUE(EVP_DigestUpdate(ctx.get(), kPlaintext, sizeof(kPlaintext))));
  EXPECT_EQ(approved, AWSLC_NOT_APPROVED);
  CALL_SERVICE_AND_CHECK_APPROVED(approved,
      ASSERT_TRUE(EVP_DigestFinal_ex(ctx.get(), digest.data(), &digest_len)));
  EXPECT_EQ(approved, test.expect_approved);
  EXPECT_EQ(Bytes(test.expected_digest, digest_len), Bytes(digest));

  // Test using the one-shot |EVP_Digest| function for approval.
  CALL_SERVICE_AND_CHECK_APPROVED(approved,
      ASSERT_TRUE(EVP_Digest(kPlaintext, sizeof(kPlaintext), digest.data(),
                           &digest_len, test.func(), nullptr)));
  EXPECT_EQ(approved, test.expect_approved);
  EXPECT_EQ(Bytes(test.expected_digest, test.length), Bytes(digest));

  // Test using the one-shot API for approval.
  CALL_SERVICE_AND_CHECK_APPROVED(
      approved,
      test.one_shot_func(kPlaintext, sizeof(kPlaintext), digest.data()));
  EXPECT_EQ(approved, test.expect_approved);
  EXPECT_EQ(Bytes(test.expected_digest, test.length), Bytes(digest));
}

static const struct XofTestVector {
  // name is the name of the digest test.
  const char *name;
  // output length to specify in XOF finalization
  const int length;
  // func is the digest to test.
  const EVP_MD *(*func)();
  // one_shot_func is the convenience one-shot version of the digest.
  uint8_t *(*one_shot_func)(const uint8_t *, size_t, uint8_t *, size_t);
  // expected_digest is the expected digest.
  const uint8_t *expected_digest;
  // expected to be approved or not.
  const FIPSStatus expect_approved;
} kXofTestVectors[] = {
    {
        "SHAKE128",
        SHAKE_OUTPUT_LENGTH,
        &EVP_shake128,
        &SHAKE128,
        kOutput_shake128,
        AWSLC_APPROVED,
    },
    {
        "SHAKE256",
        SHAKE_OUTPUT_LENGTH,
        &EVP_shake256,
        &SHAKE256,
        kOutput_shake256,
        AWSLC_APPROVED,
    },
};

class EVPXOFServiceIndicatorTest : public TestWithNoErrors<XofTestVector> {};

INSTANTIATE_TEST_SUITE_P(All, EVPXOFServiceIndicatorTest,
                         testing::ValuesIn(kXofTestVectors));

TEST_P(EVPXOFServiceIndicatorTest, EVP_Xofs) {
  const XofTestVector &test = GetParam();
  SCOPED_TRACE(test.name);

  FIPSStatus approved = AWSLC_NOT_APPROVED;
  bssl::ScopedEVP_MD_CTX ctx;
  std::vector<uint8_t> digest(test.length);

  // Test running the EVP_Digest interfaces one by one directly, and check
  // |EVP_DigestFinalXOF| for approval at the end. |EVP_DigestInit_ex| and
  // |EVP_DigestUpdate| should not be approved, because the functions do not
  // indicate that a service has been fully completed yet.
  CALL_SERVICE_AND_CHECK_APPROVED(approved,
      ASSERT_TRUE(EVP_DigestInit_ex(ctx.get(), test.func(), nullptr)));
  EXPECT_EQ(approved, AWSLC_NOT_APPROVED);
  CALL_SERVICE_AND_CHECK_APPROVED(approved,
      ASSERT_TRUE(EVP_DigestUpdate(ctx.get(), kPlaintext, sizeof(kPlaintext))));
  EXPECT_EQ(approved, AWSLC_NOT_APPROVED);
  EXPECT_TRUE(EVP_MD_flags(ctx->digest) & EVP_MD_FLAG_XOF);
  CALL_SERVICE_AND_CHECK_APPROVED(approved,
      ASSERT_TRUE(EVP_DigestFinalXOF(ctx.get(), digest.data(), test.length)));
  EXPECT_EQ(approved, test.expect_approved);
  EXPECT_EQ(Bytes(test.expected_digest, test.length), Bytes(digest));

  // Test using the one-shot |EVP_Digest| function for approval.
  unsigned digest_len = test.length;
  CALL_SERVICE_AND_CHECK_APPROVED(approved,
      ASSERT_TRUE(EVP_Digest(kPlaintext, sizeof(kPlaintext), digest.data(),
                           &digest_len, test.func(), nullptr)));
  EXPECT_EQ(approved, test.expect_approved);
  EXPECT_EQ(Bytes(test.expected_digest, test.length), Bytes(digest));

  // Test using the one-shot API for approval.
  CALL_SERVICE_AND_CHECK_APPROVED(
      approved,
      test.one_shot_func(kPlaintext, sizeof(kPlaintext), digest.data(), test.length));
  EXPECT_EQ(approved, test.expect_approved);
  EXPECT_EQ(Bytes(test.expected_digest, test.length), Bytes(digest));
}

static const struct HMACTestVector {
  // func is the hash function for HMAC to test.
  const EVP_MD *(*func)(void);
  // expected_digest is the expected digest.
  const uint8_t *expected_digest;
  // expected to be approved or not.
  const FIPSStatus expect_approved;
} kHMACTestVectors[] = {
    { EVP_sha1, kHMACOutput_sha1, AWSLC_APPROVED },
    { EVP_sha224, kHMACOutput_sha224, AWSLC_APPROVED },
    { EVP_sha256, kHMACOutput_sha256, AWSLC_APPROVED },
    { EVP_sha384, kHMACOutput_sha384, AWSLC_APPROVED },
    { EVP_sha512, kHMACOutput_sha512, AWSLC_APPROVED },
    { EVP_sha512_224, kHMACOutput_sha512_224, AWSLC_APPROVED },
    { EVP_sha512_256, kHMACOutput_sha512_256, AWSLC_APPROVED }
};

class HMACServiceIndicatorTest : public TestWithNoErrors<HMACTestVector> {};

INSTANTIATE_TEST_SUITE_P(All, HMACServiceIndicatorTest,
                         testing::ValuesIn(kHMACTestVectors));

// TODO: |EVP_PKEY_HMAC| does not indicate support for FIPS yet. Mark
// |EVP_PKEY_HMAC| as approved by modifying
// |EVP_DigestSign_verify_service_indicator| and add to the list of approved
// FIPS APIs.
TEST_P(HMACServiceIndicatorTest, HMACTest) {
  const HMACTestVector &test = GetParam();

  FIPSStatus approved = AWSLC_NOT_APPROVED;
  // The key is deliberately long in order to trigger digesting it down to a
  // block size. This tests that doing so does not cause the indicator to be
  // mistakenly set in |HMAC_Init_ex|.
  const uint8_t kHMACKey[512] = {0};
  const EVP_MD *const digest = test.func();
  const unsigned expected_mac_len = EVP_MD_size(digest);
  std::vector<uint8_t> mac(expected_mac_len);

  // Test running the HMAC interfaces one by one directly, and check
  // |HMAC_Final| for approval at the end. |HMAC_Init_ex| and |HMAC_Update|
  // should not be approved, because the functions do not indicate that a
  // service has been fully completed yet.
  unsigned mac_len;
  bssl::ScopedHMAC_CTX ctx;
  CALL_SERVICE_AND_CHECK_APPROVED(
      approved, ASSERT_TRUE(
      HMAC_Init_ex(ctx.get(), kHMACKey, sizeof(kHMACKey), digest, nullptr)));
  EXPECT_EQ(approved, AWSLC_NOT_APPROVED);
  CALL_SERVICE_AND_CHECK_APPROVED(approved,
      ASSERT_TRUE(HMAC_Update(ctx.get(), kPlaintext, sizeof(kPlaintext))));
  EXPECT_EQ(approved, AWSLC_NOT_APPROVED);
  CALL_SERVICE_AND_CHECK_APPROVED(approved,
      ASSERT_TRUE(HMAC_Final(ctx.get(), mac.data(), &mac_len)));
  EXPECT_EQ(approved, test.expect_approved);
  EXPECT_EQ(Bytes(test.expected_digest, expected_mac_len),
            Bytes(mac.data(), mac_len));

  // Test using the one-shot API for approval.
  CALL_SERVICE_AND_CHECK_APPROVED(
      approved, ASSERT_TRUE(HMAC(digest, kHMACKey, sizeof(kHMACKey), kPlaintext,
                            sizeof(kPlaintext), mac.data(), &mac_len)));
  EXPECT_EQ(approved, test.expect_approved);
  EXPECT_EQ(Bytes(test.expected_digest, expected_mac_len),
            Bytes(mac.data(), mac_len));

  // Test using the one-shot non-approved internal API HMAC_with_precompute
  CALL_SERVICE_AND_CHECK_APPROVED(
      approved, ASSERT_TRUE(HMAC_with_precompute(
                    digest, kHMACKey, sizeof(kHMACKey), kPlaintext,
                    sizeof(kPlaintext), mac.data(), &mac_len)));
  EXPECT_EQ(approved, AWSLC_NOT_APPROVED);
  EXPECT_EQ(Bytes(test.expected_digest, expected_mac_len),
            Bytes(mac.data(), mac_len));

  // Test using precomputed keys
  // First, extract the precomputed key
  ctx.Reset();
  uint8_t precomputed_key[HMAC_MAX_PRECOMPUTED_KEY_SIZE];
  size_t precomputed_key_len = HMAC_MAX_PRECOMPUTED_KEY_SIZE;
  CALL_SERVICE_AND_CHECK_APPROVED(
      approved, ASSERT_TRUE(
      HMAC_Init_ex(ctx.get(), kHMACKey, sizeof(kHMACKey), digest, nullptr)));
  EXPECT_EQ(approved, AWSLC_NOT_APPROVED);
  CALL_SERVICE_AND_CHECK_APPROVED(
      approved, ASSERT_TRUE(HMAC_set_precomputed_key_export(ctx.get())));
  EXPECT_EQ(approved, AWSLC_NOT_APPROVED);
  CALL_SERVICE_AND_CHECK_APPROVED(
      approved, ASSERT_TRUE(HMAC_get_precomputed_key(
                    ctx.get(), precomputed_key, &precomputed_key_len)));
  EXPECT_EQ(approved, AWSLC_NOT_APPROVED);
  // Second, use the precomputed key to compute the hash
  ctx.Reset();
  CALL_SERVICE_AND_CHECK_APPROVED(
      approved, ASSERT_TRUE(HMAC_Init_from_precomputed_key(
                    ctx.get(), precomputed_key, precomputed_key_len, digest)));
  EXPECT_EQ(approved, AWSLC_NOT_APPROVED);
  CALL_SERVICE_AND_CHECK_APPROVED(approved,
                                  ASSERT_TRUE(HMAC_Update(ctx.get(), kPlaintext, sizeof(kPlaintext))));
  EXPECT_EQ(approved, AWSLC_NOT_APPROVED);
  CALL_SERVICE_AND_CHECK_APPROVED(approved,
                                  ASSERT_TRUE(HMAC_Final(ctx.get(), mac.data(), &mac_len)));
  EXPECT_EQ(approved, test.expect_approved);
  EXPECT_EQ(Bytes(test.expected_digest, expected_mac_len),
            Bytes(mac.data(), mac_len));
}

const uint8_t kHKDF_ikm_tc1[] = {   // RFC 5869 Test Case 1
    0x0b, 0x0b, 0x0b, 0x0b, 0x0b, 0x0b, 0x0b, 0x0b, 0x0b, 0x0b, 0x0b, 0x0b,
    0x0b, 0x0b, 0x0b, 0x0b, 0x0b, 0x0b, 0x0b, 0x0b, 0x0b, 0x0b
};
const uint8_t kHKDF_salt_tc1[] = {
    0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c
};
const uint8_t kHKDF_info_tc1[] = {
    0xf0, 0xf1, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7, 0xf8, 0xf9
};
const uint8_t kHKDF_okm_tc1_md5[] = {   // Used for negative testing only.
    0xb2, 0x22, 0xc9, 0xdb, 0x38, 0xd1, 0x7b, 0x2f, 0xea, 0x8b, 0x3b, 0xb5,
    0x11, 0xc0, 0xd6, 0xd8, 0x60, 0x49, 0xef, 0x48, 0x1b, 0xa7, 0x06, 0x5c,
    0xa5, 0xc6, 0x42, 0x26, 0x18, 0xed, 0x9c, 0xc9, 0x14, 0x49, 0x00, 0xe2,
    0xc7, 0x2b, 0x6a, 0x86, 0x3a, 0x31
};
const uint8_t kHKDF_okm_tc1_sha1[] = {
    0xd6, 0x00, 0x0f, 0xfb, 0x5b, 0x50, 0xbd, 0x39, 0x70, 0xb2, 0x60, 0x01,
    0x77, 0x98, 0xfb, 0x9c, 0x8d, 0xf9, 0xce, 0x2e, 0x2c, 0x16, 0xb6, 0xcd,
    0x70, 0x9c, 0xca, 0x07, 0xdc, 0x3c, 0xf9, 0xcf, 0x26, 0xd6, 0xc6, 0xd7,
    0x50, 0xd0, 0xaa, 0xf5, 0xac, 0x94
};
const uint8_t kHKDF_okm_tc1_sha224[] = {
    0x2f, 0x21, 0xcd, 0x7c, 0xbc, 0x81, 0x8c, 0xa5, 0xc5, 0x61, 0xb9, 0x33,
    0x72, 0x8e, 0x2e, 0x08, 0xe1, 0x54, 0xa8, 0x7e, 0x14, 0x32, 0x39, 0x9a,
    0x82, 0x0d, 0xee, 0x13, 0xaa, 0x22, 0x2d, 0x0c, 0xee, 0x61, 0x52, 0xfa,
    0x53, 0x9a, 0xb7, 0x0f, 0x8e, 0x80
};
const uint8_t kHKDF_okm_tc1_sha256[] = {
    0x3c, 0xb2, 0x5f, 0x25, 0xfa, 0xac, 0xd5, 0x7a, 0x90, 0x43, 0x4f, 0x64,
    0xd0, 0x36, 0x2f, 0x2a, 0x2d, 0x2d, 0x0a, 0x90, 0xcf, 0x1a, 0x5a, 0x4c,
    0x5d, 0xb0, 0x2d, 0x56, 0xec, 0xc4, 0xc5, 0xbf, 0x34, 0x00, 0x72, 0x08,
    0xd5, 0xb8, 0x87, 0x18, 0x58, 0x65
};
const uint8_t kHKDF_okm_tc1_sha384[] = {
    0x9b, 0x50, 0x97, 0xa8, 0x60, 0x38, 0xb8, 0x05, 0x30, 0x90, 0x76, 0xa4,
    0x4b, 0x3a, 0x9f, 0x38, 0x06, 0x3e, 0x25, 0xb5, 0x16, 0xdc, 0xbf, 0x36,
    0x9f, 0x39, 0x4c, 0xfa, 0xb4, 0x36, 0x85, 0xf7, 0x48, 0xb6, 0x45, 0x77,
    0x63, 0xe4, 0xf0, 0x20, 0x4f, 0xc5
};
const uint8_t kHKDF_okm_tc1_sha512[] = {
    0x83, 0x23, 0x90, 0x08, 0x6c, 0xda, 0x71, 0xfb, 0x47, 0x62, 0x5b, 0xb5,
    0xce, 0xb1, 0x68, 0xe4, 0xc8, 0xe2, 0x6a, 0x1a, 0x16, 0xed, 0x34, 0xd9,
    0xfc, 0x7f, 0xe9, 0x2c, 0x14, 0x81, 0x57, 0x93, 0x38, 0xda, 0x36, 0x2c,
    0xb8, 0xd9, 0xf9, 0x25, 0xd7, 0xcb
};
const uint8_t kHKDF_okm_tc1_sha512_224[] = {
    0xf8, 0xd9, 0x56, 0xe1, 0x52, 0xb0, 0xfb, 0xa8, 0x31, 0xba, 0xc4,
    0x00, 0xf1, 0xa5, 0xaf, 0x54, 0x98, 0x2b, 0x91, 0xdb, 0x3d, 0x96,
    0xae, 0x21, 0xa7, 0x56, 0x55, 0xef, 0xf1, 0x72, 0x5f, 0x92, 0x8e,
    0x49, 0x1c, 0x63, 0xf3, 0xae, 0xdb, 0x40, 0x82, 0x96
};
const uint8_t kHKDF_okm_tc1_sha512_256[] = {
    0x78, 0x9a, 0x93, 0xe5, 0x67, 0xa1, 0x86, 0x1d, 0xe4, 0x49, 0x34,
    0x2b, 0x2d, 0x67, 0x4c, 0x0d, 0xf7, 0x37, 0xfd, 0x8a, 0xdc, 0xe2,
    0xa8, 0xe1, 0x84, 0x32, 0x37, 0xc1, 0x93, 0x8a, 0xc4, 0x13, 0x04,
    0x4b, 0x49, 0x6c, 0xe2, 0x67, 0xa1, 0x98, 0xeb, 0xe3
};

const uint8_t kHKDF_ikm_tc2[] = {   // RFC 5869 Test Case 2
    0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b,
    0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17,
    0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20, 0x21, 0x22, 0x23,
    0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2a, 0x2b, 0x2c, 0x2d, 0x2e, 0x2f,
    0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3a, 0x3b,
    0x3c, 0x3d, 0x3e, 0x3f, 0x40, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47,
    0x48, 0x49, 0x4a, 0x4b, 0x4c, 0x4d, 0x4e, 0x4f
};
const uint8_t kHKDF_salt_tc2[] = {
    0x60, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x6a, 0x6b,
    0x6c, 0x6d, 0x6e, 0x6f, 0x70, 0x71, 0x72, 0x73, 0x74, 0x75, 0x76, 0x77,
    0x78, 0x79, 0x7a, 0x7b, 0x7c, 0x7d, 0x7e, 0x7f, 0x80, 0x81, 0x82, 0x83,
    0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8a, 0x8b, 0x8c, 0x8d, 0x8e, 0x8f,
    0x90, 0x91, 0x92, 0x93, 0x94, 0x95, 0x96, 0x97, 0x98, 0x99, 0x9a, 0x9b,
    0x9c, 0x9d, 0x9e, 0x9f, 0xa0, 0xa1, 0xa2, 0xa3, 0xa4, 0xa5, 0xa6, 0xa7,
    0xa8, 0xa9, 0xaa, 0xab, 0xac, 0xad, 0xae, 0xaf
};
const uint8_t kHKDF_info_tc2[] = {
    0xb0, 0xb1, 0xb2, 0xb3, 0xb4, 0xb5, 0xb6, 0xb7, 0xb8, 0xb9, 0xba, 0xbb,
    0xbc, 0xbd, 0xbe, 0xbf, 0xc0, 0xc1, 0xc2, 0xc3, 0xc4, 0xc5, 0xc6, 0xc7,
    0xc8, 0xc9, 0xca, 0xcb, 0xcc, 0xcd, 0xce, 0xcf, 0xd0, 0xd1, 0xd2, 0xd3,
    0xd4, 0xd5, 0xd6, 0xd7, 0xd8, 0xd9, 0xda, 0xdb, 0xdc, 0xdd, 0xde, 0xdf,
    0xe0, 0xe1, 0xe2, 0xe3, 0xe4, 0xe5, 0xe6, 0xe7, 0xe8, 0xe9, 0xea, 0xeb,
    0xec, 0xed, 0xee, 0xef, 0xf0, 0xf1, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7,
    0xf8, 0xf9, 0xfa, 0xfb, 0xfc, 0xfd, 0xfe, 0xff
};
const uint8_t kHKDF_okm_tc2_sha1[] = {
    0x0b, 0xd7, 0x70, 0xa7, 0x4d, 0x11, 0x60, 0xf7, 0xc9, 0xf1, 0x2c, 0xd5,
    0x91, 0x2a, 0x06, 0xeb, 0xff, 0x6a, 0xdc, 0xae, 0x89, 0x9d, 0x92, 0x19,
    0x1f, 0xe4, 0x30, 0x56, 0x73, 0xba, 0x2f, 0xfe, 0x8f, 0xa3, 0xf1, 0xa4,
    0xe5, 0xad, 0x79, 0xf3, 0xf3, 0x34, 0xb3, 0xb2, 0x02, 0xb2, 0x17, 0x3c,
    0x48, 0x6e, 0xa3, 0x7c, 0xe3, 0xd3, 0x97, 0xed, 0x03, 0x4c, 0x7f, 0x9d,
    0xfe, 0xb1, 0x5c, 0x5e, 0x92, 0x73, 0x36, 0xd0, 0x44, 0x1f, 0x4c, 0x43,
    0x00, 0xe2, 0xcf, 0xf0, 0xd0, 0x90, 0x0b, 0x52, 0xd3, 0xb4
};
const uint8_t kHKDF_okm_tc2_sha224[] = {
    0x3e, 0x49, 0x70, 0x3c, 0x24, 0x3a, 0x38, 0x94, 0x91, 0x63, 0x49, 0xb5,
    0x2a, 0x8f, 0x55, 0xc7, 0xc1, 0x60, 0x45, 0x2f, 0x97, 0xb2, 0x87, 0x0f,
    0x04, 0xba, 0x92, 0x4b, 0xa9, 0x05, 0x6a, 0xb3, 0x51, 0x76, 0x5b, 0x04,
    0x20, 0x72, 0x31, 0x15, 0x8d, 0xcb, 0x03, 0xd0, 0xc7, 0xd4, 0x27, 0xcb,
    0x2b, 0x7e, 0x06, 0x01, 0x79, 0x45, 0x9f, 0x9d, 0xaf, 0xfe, 0xe0, 0x5e,
    0x87, 0x05, 0x11, 0x3f, 0x7b, 0xc4, 0x5b, 0x4f, 0x45, 0x26, 0x01, 0xd8,
    0x84, 0xdf, 0x6d, 0xfd, 0x4f, 0xf9, 0xda, 0xcf, 0xde, 0x69
};
const uint8_t kHKDF_okm_tc2_sha256[] = {
    0xb1, 0x1e, 0x39, 0x8d, 0xc8, 0x03, 0x27, 0xa1, 0xc8, 0xe7, 0xf7, 0x8c,
    0x59, 0x6a, 0x49, 0x34, 0x4f, 0x01, 0x2e, 0xda, 0x2d, 0x4e, 0xfa, 0xd8,
    0xa0, 0x50, 0xcc, 0x4c, 0x19, 0xaf, 0xa9, 0x7c, 0x59, 0x04, 0x5a, 0x99,
    0xca, 0xc7, 0x82, 0x72, 0x71, 0xcb, 0x41, 0xc6, 0x5e, 0x59, 0x0e, 0x09,
    0xda, 0x32, 0x75, 0x60, 0x0c, 0x2f, 0x09, 0xb8, 0x36, 0x77, 0x93, 0xa9,
    0xac, 0xa3, 0xdb, 0x71, 0xcc, 0x30, 0xc5, 0x81, 0x79, 0xec, 0x3e, 0x87,
    0xc1, 0x4c, 0x01, 0xd5, 0xc1, 0xf3, 0x43, 0x4f, 0x1d, 0x87
};
const uint8_t kHKDF_okm_tc2_sha384[] = {
    0x48, 0x4c, 0xa0, 0x52, 0xb8, 0xcc, 0x72, 0x4f, 0xd1, 0xc4, 0xec, 0x64,
    0xd5, 0x7b, 0x4e, 0x81, 0x8c, 0x7e, 0x25, 0xa8, 0xe0, 0xf4, 0x56, 0x9e,
    0xd7, 0x2a, 0x6a, 0x05, 0xfe, 0x06, 0x49, 0xee, 0xbf, 0x69, 0xf8, 0xd5,
    0xc8, 0x32, 0x85, 0x6b, 0xf4, 0xe4, 0xfb, 0xc1, 0x79, 0x67, 0xd5, 0x49,
    0x75, 0x32, 0x4a, 0x94, 0x98, 0x7f, 0x7f, 0x41, 0x83, 0x58, 0x17, 0xd8,
    0x99, 0x4f, 0xdb, 0xd6, 0xf4, 0xc0, 0x9c, 0x55, 0x00, 0xdc, 0xa2, 0x4a,
    0x56, 0x22, 0x2f, 0xea, 0x53, 0xd8, 0x96, 0x7a, 0x8b, 0x2e
};
const uint8_t kHKDF_okm_tc2_sha512[] = {
    0xce, 0x6c, 0x97, 0x19, 0x28, 0x05, 0xb3, 0x46, 0xe6, 0x16, 0x1e, 0x82,
    0x1e, 0xd1, 0x65, 0x67, 0x3b, 0x84, 0xf4, 0x00, 0xa2, 0xb5, 0x14, 0xb2,
    0xfe, 0x23, 0xd8, 0x4c, 0xd1, 0x89, 0xdd, 0xf1, 0xb6, 0x95, 0xb4, 0x8c,
    0xbd, 0x1c, 0x83, 0x88, 0x44, 0x11, 0x37, 0xb3, 0xce, 0x28, 0xf1, 0x6a,
    0xa6, 0x4b, 0xa3, 0x3b, 0xa4, 0x66, 0xb2, 0x4d, 0xf6, 0xcf, 0xcb, 0x02,
    0x1e, 0xcf, 0xf2, 0x35, 0xf6, 0xa2, 0x05, 0x6c, 0xe3, 0xaf, 0x1d, 0xe4,
    0x4d, 0x57, 0x20, 0x97, 0xa8, 0x50, 0x5d, 0x9e, 0x7a, 0x93
};
const uint8_t kHKDF_okm_tc2_sha512_224[] = {
    0xb2, 0xfd, 0x77, 0xf9, 0xcf, 0xb6, 0xda, 0x40, 0x10, 0x23, 0x8b,
    0xa6, 0x21, 0x7a, 0x1b, 0xc7, 0xb7, 0x70, 0xa3, 0x85, 0x28, 0x4b,
    0x8e, 0x54, 0xd6, 0x8d, 0x40, 0x8c, 0xce, 0x4a, 0x42, 0xc3, 0xc5,
    0xde, 0x16, 0x91, 0x59, 0x93, 0x1e, 0x13, 0x24, 0x8b
};
const uint8_t kHKDF_okm_tc2_sha512_256[] = {
    0x9f, 0xde, 0x11, 0xa4, 0x63, 0x48, 0xac, 0x1a, 0xba, 0xdf, 0xd2,
    0xff, 0xb6, 0x0d, 0x85, 0x26, 0x58, 0x3f, 0xc8, 0x3e, 0x08, 0xb8,
    0x8a, 0x6e, 0xdc, 0x2d, 0xc6, 0x95, 0xad, 0x61, 0x5d, 0xe3, 0xbe,
    0x8e, 0xd2, 0xe1, 0xfe, 0x5b, 0xc8, 0x38, 0xf7, 0x13
};

const uint8_t kHKDF_ikm_tc3[] = {   // RFC 5869 Test Case 3
    0x0b, 0x0b, 0x0b, 0x0b, 0x0b, 0x0b, 0x0b, 0x0b, 0x0b, 0x0b, 0x0b, 0x0b,
    0x0b, 0x0b, 0x0b, 0x0b, 0x0b, 0x0b, 0x0b, 0x0b, 0x0b, 0x0b
};
const uint8_t kHKDF_salt_tc3[] = {0};    // No salt
const uint8_t kHKDF_info_tc3[] = {0};    // No info
const uint8_t kHKDF_okm_tc3_sha1[] = {
    0x0a, 0xc1, 0xaf, 0x70, 0x02, 0xb3, 0xd7, 0x61, 0xd1, 0xe5, 0x52, 0x98,
    0xda, 0x9d, 0x05, 0x06, 0xb9, 0xae, 0x52, 0x05, 0x72, 0x20, 0xa3, 0x06,
    0xe0, 0x7b, 0x6b, 0x87, 0xe8, 0xdf, 0x21, 0xd0, 0xea, 0x00, 0x03, 0x3d,
    0xe0, 0x39, 0x84, 0xd3, 0x49, 0x18
};
const uint8_t kHKDF_okm_tc3_sha224[] = {
    0x2a, 0x26, 0x80, 0x83, 0xea, 0x78, 0x7e, 0x06, 0x60, 0x4a, 0x58, 0x45,
    0xf1, 0xa5, 0x35, 0x44, 0xdd, 0x78, 0x47, 0xbd, 0x6f, 0xb7, 0x4a, 0xdf,
    0xcc, 0x11, 0x78, 0xba, 0xac, 0x5a, 0x0f, 0xe7, 0x40, 0x76, 0xf8, 0x93,
    0x59, 0x71, 0xc0, 0x0c, 0x2b, 0x19
};
const uint8_t kHKDF_okm_tc3_sha256[] = {
    0x8d, 0xa4, 0xe7, 0x75, 0xa5, 0x63, 0xc1, 0x8f, 0x71, 0x5f, 0x80, 0x2a,
    0x06, 0x3c, 0x5a, 0x31, 0xb8, 0xa1, 0x1f, 0x5c, 0x5e, 0xe1, 0x87, 0x9e,
    0xc3, 0x45, 0x4e, 0x5f, 0x3c, 0x73, 0x8d, 0x2d, 0x9d, 0x20, 0x13, 0x95,
    0xfa, 0xa4, 0xb6, 0x1a, 0x96, 0xc8
};
const uint8_t kHKDF_okm_tc3_sha384[] = {
    0xc8, 0xc9, 0x6e, 0x71, 0x0f, 0x89, 0xb0, 0xd7, 0x99, 0x0b, 0xca, 0x68,
    0xbc, 0xde, 0xc8, 0xcf, 0x85, 0x40, 0x62, 0xe5, 0x4c, 0x73, 0xa7, 0xab,
    0xc7, 0x43, 0xfa, 0xde, 0x9b, 0x24, 0x2d, 0xaa, 0xcc, 0x1c, 0xea, 0x56,
    0x70, 0x41, 0x5b, 0x52, 0x84, 0x9c
};
const uint8_t kHKDF_okm_tc3_sha512[] = {
    0xf5, 0xfa, 0x02, 0xb1, 0x82, 0x98, 0xa7, 0x2a, 0x8c, 0x23, 0x89, 0x8a,
    0x87, 0x03, 0x47, 0x2c, 0x6e, 0xb1, 0x79, 0xdc, 0x20, 0x4c, 0x03, 0x42,
    0x5c, 0x97, 0x0e, 0x3b, 0x16, 0x4b, 0xf9, 0x0f, 0xff, 0x22, 0xd0, 0x48,
    0x36, 0xd0, 0xe2, 0x34, 0x3b, 0xac
};
const uint8_t kHKDF_okm_tc3_sha512_224[] = {
    0x7c, 0x21, 0xff, 0xc6, 0x05, 0x69, 0x03, 0xdd, 0x09, 0xf1, 0x31,
    0xd3, 0x36, 0xb4, 0x20, 0x41, 0x5f, 0x17, 0xb0, 0x50, 0x3b, 0xa3,
    0x23, 0x55, 0xe6, 0x79, 0xaf, 0x0f, 0x6e, 0xb6, 0x44, 0x39, 0x20,
    0x77, 0x94, 0x40, 0x09, 0x43, 0xb5, 0x3a, 0x17, 0x83
};
const uint8_t kHKDF_okm_tc3_sha512_256[] = {
    0xfa, 0x6f, 0xf4, 0x5b, 0x2f, 0xc4, 0xf0, 0xf4, 0x98, 0x83, 0xd9,
    0xc4, 0xc9, 0xf9, 0xed, 0xfb, 0x53, 0xce, 0xbb, 0x3f, 0x9f, 0xaa,
    0xc5, 0x71, 0x31, 0x9c, 0x7b, 0xd1, 0x7d, 0x37, 0x1a, 0x0a, 0xbc,
    0xa6, 0x5d, 0x85, 0xeb, 0x3d, 0x41, 0x49, 0x51, 0x58
};

const uint8_t kHKDF_ikm_tc4[] = {   // RFC 5869 Test Case 4
    0x0b, 0x0b, 0x0b, 0x0b, 0x0b, 0x0b, 0x0b, 0x0b, 0x0b, 0x0b, 0x0b
};
const uint8_t kHKDF_salt_tc4[] = {
    0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c
};
const uint8_t kHKDF_info_tc4[] = {
    0xf0, 0xf1, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7, 0xf8, 0xf9
};
const uint8_t kHKDF_okm_tc4_sha1[] = {
    0x08, 0x5a, 0x01, 0xea, 0x1b, 0x10, 0xf3, 0x69, 0x33, 0x06, 0x8b, 0x56,
    0xef, 0xa5, 0xad, 0x81, 0xa4, 0xf1, 0x4b, 0x82, 0x2f, 0x5b, 0x09, 0x15,
    0x68, 0xa9, 0xcd, 0xd4, 0xf1, 0x55, 0xfd, 0xa2, 0xc2, 0x2e, 0x42, 0x24,
    0x78, 0xd3, 0x05, 0xf3, 0xf8, 0x96
};
const uint8_t kHKDF_okm_tc4_sha224[] = {
    0x7f, 0xc8, 0xae, 0x03, 0x35, 0xed, 0x46, 0x8c, 0xef, 0x56, 0xbe, 0x09,
    0x1f, 0x64, 0x78, 0xa1, 0xaa, 0xe8, 0x4c, 0x0d, 0xa5, 0x4c, 0xe5, 0x17,
    0x6a, 0xa3, 0x89, 0x46, 0xc7, 0x9e, 0x21, 0x0e, 0xa3, 0x2a, 0x44, 0x87,
    0xe2, 0x13, 0x84, 0x05, 0xc3, 0x40
};
const uint8_t kHKDF_okm_tc4_sha256[] = {
    0x58, 0xdc, 0xe1, 0x0d, 0x58, 0x01, 0xcd, 0xfd, 0xa8, 0x31, 0x72, 0x6b,
    0xfe, 0xbc, 0xb7, 0x43, 0xd1, 0x4a, 0x7e, 0xe8, 0x3a, 0xa0, 0x57, 0xa9,
    0x3d, 0x59, 0xb0, 0xa1, 0x31, 0x7f, 0xf0, 0x9d, 0x10, 0x5c, 0xce, 0xcf,
    0x53, 0x56, 0x92, 0xb1, 0x4d, 0xd5
};
const uint8_t kHKDF_okm_tc4_sha384[] = {
    0xfb, 0x7e, 0x67, 0x43, 0xeb, 0x42, 0xcd, 0xe9, 0x6f, 0x1b, 0x70, 0x77,
    0x89, 0x52, 0xab, 0x75, 0x48, 0xca, 0xfe, 0x53, 0x24, 0x9f, 0x7f, 0xfe,
    0x14, 0x97, 0xa1, 0x63, 0x5b, 0x20, 0x1f, 0xf1, 0x85, 0xb9, 0x3e, 0x95,
    0x19, 0x92, 0xd8, 0x58, 0xf1, 0x1a
};
const uint8_t kHKDF_okm_tc4_sha512[] = {
    0x74, 0x13, 0xe8, 0x99, 0x7e, 0x02, 0x06, 0x10, 0xfb, 0xf6, 0x82, 0x3f,
    0x2c, 0xe1, 0x4b, 0xff, 0x01, 0x87, 0x5d, 0xb1, 0xca, 0x55, 0xf6, 0x8c,
    0xfc, 0xf3, 0x95, 0x4d, 0xc8, 0xaf, 0xf5, 0x35, 0x59, 0xbd, 0x5e, 0x30,
    0x28, 0xb0, 0x80, 0xf7, 0xc0, 0x68
};
const uint8_t kHKDF_okm_tc4_sha512_224[] = {
    0x80, 0x86, 0x34, 0xf8, 0x71, 0x34, 0xbc, 0xb6, 0x9b, 0xfb, 0xd2,
    0x17, 0x2c, 0x91, 0xd2, 0x2b, 0x6b, 0xdf, 0x11, 0x63, 0x4f, 0x66,
    0x4e, 0x60, 0x45, 0x03, 0xac, 0x55, 0x90, 0x7c, 0x71, 0x16, 0x5e,
    0xbe, 0xfe, 0x17, 0xce, 0xf1, 0xef, 0xe8, 0x23, 0xa3
};
const uint8_t kHKDF_okm_tc4_sha512_256[] = {
    0xce, 0xa7, 0x08, 0xf9, 0xe8, 0x3b, 0x5b, 0x33, 0x39, 0x59, 0x9b,
    0xcf, 0x6b, 0x97, 0x08, 0xde, 0x5e, 0xdf, 0x23, 0xab, 0xb5, 0x95,
    0xfc, 0xbb, 0xcc, 0xb5, 0xf5, 0x18, 0x70, 0x1e, 0x7b, 0x72, 0x07,
    0x74, 0xa8, 0xef, 0xa7, 0x9b, 0x99, 0x46, 0xb3, 0x1f
};

// RFC Test Case 5 repeats the inputs from RFC 5869 Test Case 2; RFC Test Case 6
// repeats the inputs from RFC 5869 Test Case 3.

const uint8_t kHKDF_ikm_tc7[] = {   // RFC 5869 Test Case 7
    0x0c, 0x0c, 0x0c, 0x0c, 0x0c, 0x0c, 0x0c, 0x0c, 0x0c, 0x0c, 0x0c, 0x0c,
    0x0c, 0x0c, 0x0c, 0x0c, 0x0c, 0x0c, 0x0c, 0x0c, 0x0c, 0x0c
};
// Salt for Test Case 7 is not specified (NULL). HKDF will use HashLen 0x00
// bytes instead.
const uint8_t kHKDF_info_tc7[] = {0};    // No info
const uint8_t kHKDF_okm_tc7_sha1[] = {
    0x2c, 0x91, 0x11, 0x72, 0x04, 0xd7, 0x45, 0xf3, 0x50, 0x0d, 0x63, 0x6a,
    0x62, 0xf6, 0x4f, 0x0a, 0xb3, 0xba, 0xe5, 0x48, 0xaa, 0x53, 0xd4, 0x23,
    0xb0, 0xd1, 0xf2, 0x7e, 0xbb, 0xa6, 0xf5, 0xe5, 0x67, 0x3a, 0x08, 0x1d,
    0x70, 0xcc, 0xe7, 0xac, 0xfc, 0x48
};
const uint8_t kHKDF_okm_tc7_sha224[] = {
    0xca, 0x84, 0x01, 0xe6, 0x45, 0xb3, 0xa5, 0x8e, 0x00, 0x99, 0x28, 0x57,
    0xfe, 0x00, 0x38, 0xcb, 0x1b, 0xf8, 0xdc, 0x51, 0xed, 0xf0, 0x52, 0x33,
    0x6c, 0x08, 0xf3, 0xbe, 0xd6, 0x82, 0xc8, 0x3e, 0x77, 0x80, 0x3c, 0xdd,
    0x16, 0xd1, 0x56, 0xbb, 0x8a, 0x30
};
const uint8_t kHKDF_okm_tc7_sha256[] = {
    0x59, 0x68, 0x99, 0x17, 0x9a, 0xb1, 0xbc, 0x00, 0xa7, 0xc0, 0x37, 0x86,
    0xff, 0x43, 0xee, 0x53, 0x50, 0x04, 0xbe, 0x2b, 0xb9, 0xbe, 0x68, 0xbc,
    0x14, 0x06, 0x63, 0x6f, 0x54, 0xbd, 0x33, 0x8a, 0x66, 0xa2, 0x37, 0xba,
    0x2a, 0xcb, 0xce, 0xe3, 0xc9, 0xa7
};
const uint8_t kHKDF_okm_tc7_sha384[] = {
    0x6a, 0xd7, 0xc7, 0x26, 0xc8, 0x40, 0x09, 0x54, 0x6a, 0x76, 0xe0, 0x54,
    0x5d, 0xf2, 0x66, 0x78, 0x7e, 0x2b, 0x2c, 0xd6, 0xca, 0x43, 0x73, 0xa1,
    0xf3, 0x14, 0x50, 0xa7, 0xbd, 0xf9, 0x48, 0x2b, 0xfa, 0xb8, 0x11, 0xf5,
    0x54, 0x20, 0x0e, 0xad, 0x8f, 0x53
};
const uint8_t kHKDF_okm_tc7_sha512[] = {
    0x14, 0x07, 0xd4, 0x60, 0x13, 0xd9, 0x8b, 0xc6, 0xde, 0xce, 0xfc, 0xfe,
    0xe5, 0x5f, 0x0f, 0x90, 0xb0, 0xc7, 0xf6, 0x3d, 0x68, 0xeb, 0x1a, 0x80,
    0xea, 0xf0, 0x7e, 0x95, 0x3c, 0xfc, 0x0a, 0x3a, 0x52, 0x40, 0xa1, 0x55,
    0xd6, 0xe4, 0xda, 0xa9, 0x65, 0xbb
};
const uint8_t kHKDF_okm_tc7_sha512_224[] = {
    0xb2, 0xf0, 0x98, 0x31, 0x2a, 0xd3, 0xfe, 0xee, 0x46, 0xe9, 0x0f,
    0x1b, 0x90, 0x6a, 0x20, 0xa1, 0xab, 0xee, 0x95, 0xbb, 0xcd, 0xf8,
    0x16, 0x30, 0xc7, 0x1c, 0x2b, 0x46, 0xc6, 0xc6, 0x15, 0xbe, 0x23,
    0x54, 0x38, 0x2f, 0x42, 0x56, 0x4d, 0xee, 0x56, 0x4d
};
const uint8_t kHKDF_okm_tc7_sha512_256[] = {
    0x6e, 0x15, 0x36, 0x0b, 0x08, 0x47, 0xd9, 0xef, 0x32, 0xa4, 0xa8,
    0x0d, 0x5e, 0x1f, 0x58, 0xce, 0xb3, 0xe9, 0x01, 0xf9, 0x29, 0x80,
    0x4e, 0xcf, 0x01, 0x6a, 0x8c, 0xf3, 0x59, 0x18, 0xb5, 0xdb, 0x99,
    0x8d, 0x1f, 0x09, 0x1e, 0x83, 0x67, 0xa1, 0x82, 0x62
};

static const struct HKDFTestVector {
    // func is the hash function for HMAC to test.
    const EVP_MD *(*func)(void);
    const uint8_t *ikm; // Initial Keying Material
    const size_t ikm_size;
    const uint8_t *salt;    // Salt
    const size_t salt_size;
    const uint8_t *info;    // "Other Info", the sequel to Salt.
    const size_t info_size;
    const uint8_t *expected_output;    // Expected Output Keying Material
    const uint8_t output_len;
    const FIPSStatus expect_approved;
} kHKDFTestVectors[] = {
    // RFC 5869 Test Case 1
    {
        EVP_md5,
        kHKDF_ikm_tc1, sizeof(kHKDF_ikm_tc1),
        kHKDF_salt_tc1, sizeof(kHKDF_salt_tc1),
        kHKDF_info_tc1, sizeof(kHKDF_info_tc1),
        kHKDF_okm_tc1_md5, sizeof(kHKDF_okm_tc1_md5),
        AWSLC_NOT_APPROVED,
    },
    {
        EVP_sha1,
        kHKDF_ikm_tc1, sizeof(kHKDF_ikm_tc1),
        kHKDF_salt_tc1, sizeof(kHKDF_salt_tc1),
        kHKDF_info_tc1, sizeof(kHKDF_info_tc1),
        kHKDF_okm_tc1_sha1, sizeof(kHKDF_okm_tc1_sha1),
        AWSLC_APPROVED,
    },
    {
        EVP_sha224,
        kHKDF_ikm_tc1, sizeof(kHKDF_ikm_tc1),
        kHKDF_salt_tc1, sizeof(kHKDF_salt_tc1),
        kHKDF_info_tc1, sizeof(kHKDF_info_tc1),
        kHKDF_okm_tc1_sha224, sizeof(kHKDF_okm_tc1_sha224),
        AWSLC_APPROVED,
    },
    {
        EVP_sha256,
        kHKDF_ikm_tc1, sizeof(kHKDF_ikm_tc1),
        kHKDF_salt_tc1, sizeof(kHKDF_salt_tc1),
        kHKDF_info_tc1, sizeof(kHKDF_info_tc1),
        kHKDF_okm_tc1_sha256, sizeof(kHKDF_okm_tc1_sha256),
        AWSLC_APPROVED,
    },
    {
        EVP_sha384,
        kHKDF_ikm_tc1, sizeof(kHKDF_ikm_tc1),
        kHKDF_salt_tc1, sizeof(kHKDF_salt_tc1),
        kHKDF_info_tc1, sizeof(kHKDF_info_tc1),
        kHKDF_okm_tc1_sha384, sizeof(kHKDF_okm_tc1_sha384),
        AWSLC_APPROVED,
    },
    {
        EVP_sha512,
        kHKDF_ikm_tc1, sizeof(kHKDF_ikm_tc1),
        kHKDF_salt_tc1, sizeof(kHKDF_salt_tc1),
        kHKDF_info_tc1, sizeof(kHKDF_info_tc1),
        kHKDF_okm_tc1_sha512, sizeof(kHKDF_okm_tc1_sha512),
        AWSLC_APPROVED,
    },
    {
        EVP_sha512_224,
        kHKDF_ikm_tc1, sizeof(kHKDF_ikm_tc1),
        kHKDF_salt_tc1, sizeof(kHKDF_salt_tc1),
        kHKDF_info_tc1, sizeof(kHKDF_info_tc1),
        kHKDF_okm_tc1_sha512_224, sizeof(kHKDF_okm_tc1_sha512_224),
        AWSLC_APPROVED,
    },
    {
        EVP_sha512_256,
        kHKDF_ikm_tc1, sizeof(kHKDF_ikm_tc1),
        kHKDF_salt_tc1, sizeof(kHKDF_salt_tc1),
        kHKDF_info_tc1, sizeof(kHKDF_info_tc1),
        kHKDF_okm_tc1_sha512_256, sizeof(kHKDF_okm_tc1_sha512_256),
        AWSLC_APPROVED,
    },


    // RFC 5869 Test Case 2
    {
        EVP_sha1,
        kHKDF_ikm_tc2, sizeof(kHKDF_ikm_tc2),
        kHKDF_salt_tc2, sizeof(kHKDF_salt_tc2),
        kHKDF_info_tc2, sizeof(kHKDF_info_tc2),
        kHKDF_okm_tc2_sha1, sizeof(kHKDF_okm_tc2_sha1),
        AWSLC_APPROVED,
    },
    {
        EVP_sha224,
        kHKDF_ikm_tc2, sizeof(kHKDF_ikm_tc2),
        kHKDF_salt_tc2, sizeof(kHKDF_salt_tc2),
        kHKDF_info_tc2, sizeof(kHKDF_info_tc2),
        kHKDF_okm_tc2_sha224, sizeof(kHKDF_okm_tc2_sha224),
        AWSLC_APPROVED,
    },
    {
        EVP_sha256,
        kHKDF_ikm_tc2, sizeof(kHKDF_ikm_tc2),
        kHKDF_salt_tc2, sizeof(kHKDF_salt_tc2),
        kHKDF_info_tc2, sizeof(kHKDF_info_tc2),
        kHKDF_okm_tc2_sha256, sizeof(kHKDF_okm_tc2_sha256),
        AWSLC_APPROVED,
    },
    {
        EVP_sha384,
        kHKDF_ikm_tc2, sizeof(kHKDF_ikm_tc2),
        kHKDF_salt_tc2, sizeof(kHKDF_salt_tc2),
        kHKDF_info_tc2, sizeof(kHKDF_info_tc2),
        kHKDF_okm_tc2_sha384, sizeof(kHKDF_okm_tc2_sha384),
        AWSLC_APPROVED,
    },
    {
        EVP_sha512,
        kHKDF_ikm_tc2, sizeof(kHKDF_ikm_tc2),
        kHKDF_salt_tc2, sizeof(kHKDF_salt_tc2),
        kHKDF_info_tc2, sizeof(kHKDF_info_tc2),
        kHKDF_okm_tc2_sha512, sizeof(kHKDF_okm_tc2_sha512),
        AWSLC_APPROVED,
    },
    {
        EVP_sha512_224,
        kHKDF_ikm_tc2, sizeof(kHKDF_ikm_tc2),
        kHKDF_salt_tc2, sizeof(kHKDF_salt_tc2),
        kHKDF_info_tc2, sizeof(kHKDF_info_tc2),
        kHKDF_okm_tc2_sha512_224, sizeof(kHKDF_okm_tc2_sha512_224),
        AWSLC_APPROVED,
    },
    {
        EVP_sha512_256,
        kHKDF_ikm_tc2, sizeof(kHKDF_ikm_tc2),
        kHKDF_salt_tc2, sizeof(kHKDF_salt_tc2),
        kHKDF_info_tc2, sizeof(kHKDF_info_tc2),
        kHKDF_okm_tc2_sha512_256, sizeof(kHKDF_okm_tc2_sha512_256),
        AWSLC_APPROVED,
    },

    // RFC 5869 Test Case 3
    //
    // Salt and Info are zero-length arrays, which aren't allowed in standard
    // C, thus the hard-coded sizes in this section.
    {
        EVP_sha1,
        kHKDF_ikm_tc3, sizeof(kHKDF_ikm_tc3),
        kHKDF_salt_tc3, 0,
        kHKDF_info_tc3, 0,
        kHKDF_okm_tc3_sha1, sizeof(kHKDF_okm_tc3_sha1),
        AWSLC_NOT_APPROVED,
    },
    {
        EVP_sha224,
        kHKDF_ikm_tc3, sizeof(kHKDF_ikm_tc3),
        kHKDF_salt_tc3, 0,
        kHKDF_info_tc3, 0,
        kHKDF_okm_tc3_sha224, sizeof(kHKDF_okm_tc3_sha224),
        AWSLC_NOT_APPROVED,
    },
    {
        EVP_sha256,
        kHKDF_ikm_tc3, sizeof(kHKDF_ikm_tc3),
        kHKDF_salt_tc3, 0,
        kHKDF_info_tc3, 0,
        kHKDF_okm_tc3_sha256, sizeof(kHKDF_okm_tc3_sha256),
        AWSLC_NOT_APPROVED,
    },
    {
        EVP_sha384,
        kHKDF_ikm_tc3, sizeof(kHKDF_ikm_tc3),
        kHKDF_salt_tc3, 0,
        kHKDF_info_tc3, 0,
        kHKDF_okm_tc3_sha384, sizeof(kHKDF_okm_tc3_sha384),
        AWSLC_NOT_APPROVED,
    },
    {
        EVP_sha512,
        kHKDF_ikm_tc3, sizeof(kHKDF_ikm_tc3),
        kHKDF_salt_tc3, 0,
        kHKDF_info_tc3, 0,
        kHKDF_okm_tc3_sha512, sizeof(kHKDF_okm_tc3_sha512),
        AWSLC_NOT_APPROVED,
    },
    {
        EVP_sha512_224,
        kHKDF_ikm_tc3, sizeof(kHKDF_ikm_tc3),
        kHKDF_salt_tc3, 0,
        kHKDF_info_tc3, 0,
        kHKDF_okm_tc3_sha512_224, sizeof(kHKDF_okm_tc3_sha512_224),
        AWSLC_NOT_APPROVED,
    },
    {
        EVP_sha512_256,
        kHKDF_ikm_tc3, sizeof(kHKDF_ikm_tc3),
        kHKDF_salt_tc3, 0,
        kHKDF_salt_tc3, 0,
        kHKDF_okm_tc3_sha512_256, sizeof(kHKDF_okm_tc3_sha512_256),
        AWSLC_NOT_APPROVED,
    },

    // RFC 5869 Test Case 4
    {
        EVP_sha1,
        kHKDF_ikm_tc4, sizeof(kHKDF_ikm_tc4),
        kHKDF_salt_tc4, sizeof(kHKDF_salt_tc4),
        kHKDF_info_tc4, sizeof(kHKDF_info_tc4),
        kHKDF_okm_tc4_sha1, sizeof(kHKDF_okm_tc4_sha1),
        AWSLC_APPROVED,
    },
    {
        EVP_sha224,
        kHKDF_ikm_tc4, sizeof(kHKDF_ikm_tc4),
        kHKDF_salt_tc4, sizeof(kHKDF_salt_tc4),
        kHKDF_info_tc4, sizeof(kHKDF_info_tc4),
        kHKDF_okm_tc4_sha224, sizeof(kHKDF_okm_tc4_sha224),
        AWSLC_APPROVED,
    },
    {
        EVP_sha256,
        kHKDF_ikm_tc4, sizeof(kHKDF_ikm_tc4),
        kHKDF_salt_tc4, sizeof(kHKDF_salt_tc4),
        kHKDF_info_tc4, sizeof(kHKDF_info_tc4),
        kHKDF_okm_tc4_sha256, sizeof(kHKDF_okm_tc4_sha256),
        AWSLC_APPROVED,
    },
    {
        EVP_sha384,
        kHKDF_ikm_tc4, sizeof(kHKDF_ikm_tc4),
        kHKDF_salt_tc4, sizeof(kHKDF_salt_tc4),
        kHKDF_info_tc4, sizeof(kHKDF_info_tc4),
        kHKDF_okm_tc4_sha384, sizeof(kHKDF_okm_tc4_sha384),
        AWSLC_APPROVED,
    },
    {
        EVP_sha512,
        kHKDF_ikm_tc4, sizeof(kHKDF_ikm_tc4),
        kHKDF_salt_tc4, sizeof(kHKDF_salt_tc4),
        kHKDF_info_tc4, sizeof(kHKDF_info_tc4),
        kHKDF_okm_tc4_sha512, sizeof(kHKDF_okm_tc4_sha512),
        AWSLC_APPROVED,
    },
    {
        EVP_sha512_224,
        kHKDF_ikm_tc4, sizeof(kHKDF_ikm_tc4),
        kHKDF_salt_tc4, sizeof(kHKDF_salt_tc4),
        kHKDF_info_tc4, sizeof(kHKDF_info_tc4),
        kHKDF_okm_tc4_sha512_224, sizeof(kHKDF_okm_tc4_sha512_224),
        AWSLC_APPROVED,
    },
    {
        EVP_sha512_256,
        kHKDF_ikm_tc4, sizeof(kHKDF_ikm_tc4),
        kHKDF_salt_tc4, sizeof(kHKDF_salt_tc4),
        kHKDF_info_tc4, sizeof(kHKDF_info_tc4),
        kHKDF_okm_tc4_sha512_256, sizeof(kHKDF_okm_tc4_sha512_256),
        AWSLC_APPROVED,
    },

    // RFC Test Case 5 repeats the inputs from RFC 5869 Test Case 2; RFC Test
    // Case 6 repeats the inputs from RFC 5869 Test Case 3.

    // RFC 5869 Test Case 7
    //
    // Info is a zero-length array, thus the hard-coded length.
    {
        EVP_sha1,
        kHKDF_ikm_tc7, sizeof(kHKDF_ikm_tc7),
        NULL, 0,
        kHKDF_info_tc7, 0,
        kHKDF_okm_tc7_sha1, sizeof(kHKDF_okm_tc7_sha1),
        AWSLC_NOT_APPROVED,
    },
    {
        EVP_sha224,
        kHKDF_ikm_tc7, sizeof(kHKDF_ikm_tc7),
        NULL, 0,
        kHKDF_info_tc7, 0,
        kHKDF_okm_tc7_sha224, sizeof(kHKDF_okm_tc7_sha224),
        AWSLC_NOT_APPROVED,
    },
    {
        EVP_sha256,
        kHKDF_ikm_tc7, sizeof(kHKDF_ikm_tc7),
        NULL, 0,
        kHKDF_info_tc7, 0,
        kHKDF_okm_tc7_sha256, sizeof(kHKDF_okm_tc7_sha256),
        AWSLC_NOT_APPROVED,
    },
    {
        EVP_sha384,
        kHKDF_ikm_tc7, sizeof(kHKDF_ikm_tc7),
        NULL, 0,
        kHKDF_info_tc7, 0,
        kHKDF_okm_tc7_sha384, sizeof(kHKDF_okm_tc7_sha384),
        AWSLC_NOT_APPROVED,
    },
    {
        EVP_sha512,
        kHKDF_ikm_tc7, sizeof(kHKDF_ikm_tc7),
        NULL, 0,
        kHKDF_info_tc7, 0,
        kHKDF_okm_tc7_sha512, sizeof(kHKDF_okm_tc7_sha512),
        AWSLC_NOT_APPROVED,
    },
    {
        EVP_sha512_224,
        kHKDF_ikm_tc7, sizeof(kHKDF_ikm_tc7),
        NULL, 0,
        kHKDF_info_tc7, 0,
        kHKDF_okm_tc7_sha512_224, sizeof(kHKDF_okm_tc7_sha512_224),
        AWSLC_NOT_APPROVED,
    },
    {
        EVP_sha512_256,
        kHKDF_ikm_tc7, sizeof(kHKDF_ikm_tc7),
        NULL, 0,
        kHKDF_info_tc7, 0,
        kHKDF_okm_tc7_sha512_256, sizeof(kHKDF_okm_tc7_sha512_256),
        AWSLC_NOT_APPROVED,
    },
};

// Index into the kHKDFTestVectors array; used in the EVP_HKDF_Extract and
// EVP_HKDF_Expand tests, below.
#define EVP_HKDF_TEST_EXTRACT_EXPAND 3
#define EVP_HKDF_TEST_EXTRACT_EXPAND_FAIL 0

class HKDF_ServiceIndicatorTest : public TestWithNoErrors<HKDFTestVector> {};

INSTANTIATE_TEST_SUITE_P(All, HKDF_ServiceIndicatorTest,
                         testing::ValuesIn(kHKDFTestVectors));

TEST_P(HKDF_ServiceIndicatorTest, HKDFTest) {
  const HKDFTestVector &test = GetParam();

  FIPSStatus approved = AWSLC_NOT_APPROVED;

  uint8_t output[sizeof(kHKDF_okm_tc2_sha256)];   // largest test output size
  CALL_SERVICE_AND_CHECK_APPROVED(
    approved, ASSERT_TRUE(HKDF(output, test.output_len, test.func(),
                               test.ikm, test.ikm_size,
                               test.salt, test.salt_size,
                               test.info, test.info_size)));
  EXPECT_EQ(Bytes(test.expected_output, test.output_len),
            Bytes(output, test.output_len));
  EXPECT_EQ(approved, test.expect_approved);
}

TEST(HKDF_ServiceIndicatorTest, NegativeTests) {
  FIPSStatus status = AWSLC_APPROVED;

  // Setting |out_len| to (256 * 254 + 1) implies n = 255 in |HKDF_expand|.
  // This should cause a failure and no service indicator set to approved.
  uint8_t output[sizeof(kHKDF_okm_tc1_sha256)];
  CALL_SERVICE_AND_CHECK_APPROVED(
    status, ASSERT_FALSE(HKDF(output, (256 * 254 + 1), EVP_sha256(),
                               kHKDF_ikm_tc1, sizeof(kHKDF_ikm_tc1),
                               kHKDF_salt_tc1, sizeof(kHKDF_salt_tc1),
                               kHKDF_info_tc1, sizeof(kHKDF_info_tc1))));
  EXPECT_EQ(status, AWSLC_NOT_APPROVED);

  status = AWSLC_APPROVED;
  CALL_SERVICE_AND_CHECK_APPROVED(
    status, ASSERT_FALSE(HKDF_expand(output, (256 * 254 + 1), EVP_sha256(),
                               kHKDF_ikm_tc1, sizeof(kHKDF_ikm_tc1),
                               kHKDF_info_tc1, sizeof(kHKDF_info_tc1))));
  EXPECT_EQ(status, AWSLC_NOT_APPROVED);

  // The above short-circuits fast in |HKDF_expand|. But currently, there is no
  // way to force an error in the rest of the function logic when going through
  // the public API.
}

class EVP_HKDF_ServiceIndicatorTest : public TestWithNoErrors<HKDFTestVector> {};

INSTANTIATE_TEST_SUITE_P(All, EVP_HKDF_ServiceIndicatorTest,
                         testing::ValuesIn(kHKDFTestVectors));

TEST_P(EVP_HKDF_ServiceIndicatorTest, EVP_HKDFTest) {
  const HKDFTestVector &test = GetParam();

  FIPSStatus approved = AWSLC_NOT_APPROVED;

  uint8_t output[sizeof(kHKDF_okm_tc2_sha256)];   // largest test output size
  EVP_PKEY_CTX *pctx;
  size_t outlen = test.output_len;

  pctx = EVP_PKEY_CTX_new_id(EVP_PKEY_HKDF, NULL);
  EXPECT_NE(pctx, nullptr);
  EXPECT_TRUE(EVP_PKEY_derive_init(pctx));
  EXPECT_TRUE(EVP_PKEY_CTX_hkdf_mode(pctx,
                                     EVP_PKEY_HKDEF_MODE_EXTRACT_AND_EXPAND));
  EXPECT_TRUE(EVP_PKEY_CTX_set_hkdf_md(pctx, test.func()));
  EXPECT_TRUE(EVP_PKEY_CTX_set1_hkdf_key(pctx, test.ikm, test.ikm_size));
  EXPECT_TRUE(EVP_PKEY_CTX_set1_hkdf_salt(pctx, test.salt, test.salt_size));
  EXPECT_TRUE(EVP_PKEY_CTX_add1_hkdf_info(pctx, test.info, test.info_size));

  CALL_SERVICE_AND_CHECK_APPROVED(
    approved, ASSERT_TRUE(EVP_PKEY_derive(pctx, output, &outlen)));
  EXPECT_EQ(outlen, test.output_len);
  EXPECT_EQ(Bytes(test.expected_output, test.output_len),
            Bytes(output, test.output_len));
  EXPECT_EQ(approved, test.expect_approved);

  if (pctx != NULL) {
    EVP_PKEY_CTX_free(pctx);
  }
}

// Test only HKDF's Extract phase, which is not approved on its own.
TEST(EVP_HKDF_ServiceIndicatorTest, EVP_HKDF_Extract) {
  const HKDFTestVector &test = kHKDFTestVectors[EVP_HKDF_TEST_EXTRACT_EXPAND];
  FIPSStatus approved = AWSLC_NOT_APPROVED;
  uint8_t output[sizeof(kHKDF_okm_tc2_sha256)];  // largest test output size
  EVP_PKEY_CTX *pctx;
  size_t outlen = test.output_len;

  pctx = EVP_PKEY_CTX_new_id(EVP_PKEY_HKDF, NULL);
  EXPECT_NE(pctx, nullptr);
  EXPECT_TRUE(EVP_PKEY_derive_init(pctx));
  EXPECT_TRUE(EVP_PKEY_CTX_hkdf_mode(pctx,
                                     EVP_PKEY_HKDEF_MODE_EXTRACT_ONLY));
  EXPECT_TRUE(EVP_PKEY_CTX_set_hkdf_md(pctx, test.func()));
  EXPECT_TRUE(EVP_PKEY_CTX_set1_hkdf_key(pctx, test.ikm, test.ikm_size));
  EXPECT_TRUE(EVP_PKEY_CTX_set1_hkdf_salt(pctx, test.salt, test.salt_size));
  EXPECT_TRUE(EVP_PKEY_CTX_add1_hkdf_info(pctx, test.info, test.info_size));

  CALL_SERVICE_AND_CHECK_APPROVED(
    approved, ASSERT_TRUE(EVP_PKEY_derive(pctx, output, &outlen)));
  EXPECT_EQ(approved, AWSLC_NOT_APPROVED);

  if (pctx != NULL) {
    EVP_PKEY_CTX_free(pctx);
  }
}

// Test only HKDF's Expand phase, which is approved as a "KBKDF in Feedback
// Mode" per NIST SP800-108r1.
TEST(EVP_HKDF_ServiceIndicatorTest, EVP_HKDF_Expand) {
    const HKDFTestVector &test = kHKDFTestVectors[EVP_HKDF_TEST_EXTRACT_EXPAND];
    FIPSStatus approved = AWSLC_NOT_APPROVED;
    uint8_t output[sizeof(kHKDF_okm_tc2_sha256)];  // largest test output size
    EVP_PKEY_CTX *pctx;
    size_t outlen = test.output_len;

    // Positive test; HKDF_Expand() with an allowed hash (SHA256) is approved.
    pctx = EVP_PKEY_CTX_new_id(EVP_PKEY_HKDF, NULL);
    EXPECT_NE(pctx, nullptr);
    EXPECT_TRUE(EVP_PKEY_derive_init(pctx));
    EXPECT_TRUE(EVP_PKEY_CTX_hkdf_mode(pctx,
                                       EVP_PKEY_HKDEF_MODE_EXPAND_ONLY));
    EXPECT_TRUE(EVP_PKEY_CTX_set_hkdf_md(pctx, test.func()));
    EXPECT_TRUE(EVP_PKEY_CTX_set1_hkdf_key(pctx, test.ikm, test.ikm_size));
    EXPECT_TRUE(EVP_PKEY_CTX_set1_hkdf_salt(pctx, test.salt, test.salt_size));
    EXPECT_TRUE(EVP_PKEY_CTX_add1_hkdf_info(pctx, test.info, test.info_size));

    CALL_SERVICE_AND_CHECK_APPROVED(
        approved, ASSERT_TRUE(EVP_PKEY_derive(pctx, output, &outlen)));
    EXPECT_EQ(approved, AWSLC_APPROVED);

    if (pctx != NULL) {
        EVP_PKEY_CTX_free(pctx);
    }

    // Negative test; HKDF_Expand() with a disallowed hash (MD5).
    const HKDFTestVector &bad_test = kHKDFTestVectors[EVP_HKDF_TEST_EXTRACT_EXPAND_FAIL];
    pctx = EVP_PKEY_CTX_new_id(EVP_PKEY_HKDF, NULL);
    EXPECT_NE(pctx, nullptr);
    EXPECT_TRUE(EVP_PKEY_derive_init(pctx));
    EXPECT_TRUE(EVP_PKEY_CTX_hkdf_mode(pctx,
                                       EVP_PKEY_HKDEF_MODE_EXPAND_ONLY));
    EXPECT_TRUE(EVP_PKEY_CTX_set_hkdf_md(pctx, bad_test.func()));
    EXPECT_TRUE(EVP_PKEY_CTX_set1_hkdf_key(pctx, bad_test.ikm, bad_test.ikm_size));
    EXPECT_TRUE(EVP_PKEY_CTX_set1_hkdf_salt(pctx, bad_test.salt, bad_test.salt_size));
    EXPECT_TRUE(EVP_PKEY_CTX_add1_hkdf_info(pctx, bad_test.info, bad_test.info_size));

    CALL_SERVICE_AND_CHECK_APPROVED(
        approved, ASSERT_TRUE(EVP_PKEY_derive(pctx, output, &outlen)));
    EXPECT_EQ(approved, AWSLC_NOT_APPROVED);

    if (pctx != NULL) {
        EVP_PKEY_CTX_free(pctx);
    }
}

// RSA tests are not parameterized with the |kRSATestVectors| as key
// generation for RSA is time consuming.
TEST(ServiceIndicatorTest, RSAKeyGen) {
  FIPSStatus approved = AWSLC_NOT_APPROVED;
  bssl::UniquePtr<RSA> rsa(RSA_new());
  ASSERT_TRUE(rsa);

  // |RSA_generate_key_fips| may only be used for bits >= 2048 && bits % 128 == 0
  for (const size_t bits : {512, 1024, 2520, 3071}) {
    SCOPED_TRACE(bits);

    rsa.reset(RSA_new());
    CALL_SERVICE_AND_CHECK_APPROVED(approved,
        EXPECT_FALSE(RSA_generate_key_fips(rsa.get(), bits, nullptr)));
    EXPECT_EQ(approved, AWSLC_NOT_APPROVED);
  }

  // Test that we can generate keys with supported lengths,
  // larger key sizes are supported but are omitted for time.
  for (const size_t bits : {2048, 3072, 4096, 6144, 8192}) {
    SCOPED_TRACE(bits);

    rsa.reset(RSA_new());
    CALL_SERVICE_AND_CHECK_APPROVED(approved,
        EXPECT_TRUE( RSA_generate_key_fips(rsa.get(), bits, nullptr)));
    EXPECT_EQ(approved, AWSLC_APPROVED);
    EXPECT_EQ(bits, RSA_bits(rsa.get()));
  }

  // Test running the EVP_PKEY_keygen interfaces one by one directly, and check
  // |EVP_PKEY_keygen| for approval at the end. |EVP_PKEY_keygen_init| should
  // not be approved because it does not indicate an entire service has been
  // completed.
  bssl::UniquePtr<EVP_PKEY_CTX> ctx(EVP_PKEY_CTX_new_id(EVP_PKEY_RSA, nullptr));
  EVP_PKEY *raw = nullptr;
  bssl::UniquePtr<EVP_PKEY> pkey(raw);
  ASSERT_TRUE(ctx);

  if (kEVPKeyGenShouldCallFIPSFunctions) {
    // Test various unapproved key sizes of RSA.
    for (const size_t bits : {512, 1024, 3071, 4095}) {
      SCOPED_TRACE(bits);
      CALL_SERVICE_AND_CHECK_APPROVED(
          approved, ASSERT_TRUE(EVP_PKEY_keygen_init(ctx.get())));
      EXPECT_EQ(approved, AWSLC_NOT_APPROVED);
      ASSERT_TRUE(EVP_PKEY_CTX_set_rsa_keygen_bits(ctx.get(), bits));
      CALL_SERVICE_AND_CHECK_APPROVED(
          approved, ASSERT_FALSE(EVP_PKEY_keygen(ctx.get(), &raw)));
      EXPECT_EQ(approved, AWSLC_NOT_APPROVED);
      pkey.reset(raw);
      raw = nullptr;
    }

    // Test various approved key sizes of RSA.
    for (const size_t bits : {2048, 3072, 4096, 6144, 8192}) {
      SCOPED_TRACE(bits);
      CALL_SERVICE_AND_CHECK_APPROVED(
          approved, ASSERT_TRUE(EVP_PKEY_keygen_init(ctx.get())));
      EXPECT_EQ(approved, AWSLC_NOT_APPROVED);
      ASSERT_TRUE(EVP_PKEY_CTX_set_rsa_keygen_bits(ctx.get(), bits));
      CALL_SERVICE_AND_CHECK_APPROVED(
          approved, ASSERT_TRUE(EVP_PKEY_keygen(ctx.get(), &raw)));
      EXPECT_EQ(approved, AWSLC_APPROVED);
      pkey.reset(raw);
      raw = nullptr;
    }
  }
}

struct RSATestVector {
  // key_size is the input rsa key size.
  const int key_size;
  // md_func is the digest to test.
  const EVP_MD *(*func)();
  // whether to use pss testing or not.
  const bool use_pss;
  // expected to be approved or not for signature generation.
  const FIPSStatus sig_gen_expect_approved;
  // expected to be approved or not for signature verification.
  const FIPSStatus sig_ver_expect_approved;
};
struct RSATestVector kRSATestVectors[] = {
    // RSA test cases that are not approved in any case.
    { 512, &EVP_sha1, false, AWSLC_NOT_APPROVED, AWSLC_NOT_APPROVED },
    // PSS with hashLen == saltLen is not possible for 512-bit modulus.
    { 1024, &EVP_md5, false, AWSLC_NOT_APPROVED, AWSLC_NOT_APPROVED },
    { 1536, &EVP_sha256, false, AWSLC_NOT_APPROVED, AWSLC_NOT_APPROVED },
    { 1536, &EVP_sha512, true, AWSLC_NOT_APPROVED, AWSLC_NOT_APPROVED },
    { 2048, &EVP_md5, false, AWSLC_NOT_APPROVED, AWSLC_NOT_APPROVED },
    { 4096, &EVP_md5, false, AWSLC_NOT_APPROVED, AWSLC_NOT_APPROVED },

    // RSA test cases that are approved.
    { 1024, &EVP_sha1, false, AWSLC_NOT_APPROVED, AWSLC_APPROVED },
    { 1024, &EVP_sha224, false, AWSLC_NOT_APPROVED, AWSLC_APPROVED },
    { 1024, &EVP_sha256, false, AWSLC_NOT_APPROVED, AWSLC_APPROVED },
    { 1024, &EVP_sha384, false, AWSLC_NOT_APPROVED, AWSLC_APPROVED },
    { 1024, &EVP_sha512, false, AWSLC_NOT_APPROVED, AWSLC_APPROVED },
    { 1024, &EVP_sha512_224, false, AWSLC_NOT_APPROVED, AWSLC_APPROVED },
    { 1024, &EVP_sha512_256, false, AWSLC_NOT_APPROVED, AWSLC_APPROVED },
    { 1024, &EVP_sha3_224, false, AWSLC_NOT_APPROVED, AWSLC_APPROVED },
    { 1024, &EVP_sha3_256, false, AWSLC_NOT_APPROVED, AWSLC_APPROVED },
    { 1024, &EVP_sha3_384, false, AWSLC_NOT_APPROVED, AWSLC_APPROVED },

    { 1024, &EVP_sha1, true, AWSLC_NOT_APPROVED, AWSLC_APPROVED },
    { 1024, &EVP_sha224, true, AWSLC_NOT_APPROVED, AWSLC_APPROVED },
    { 1024, &EVP_sha256, true, AWSLC_NOT_APPROVED, AWSLC_APPROVED },
    { 1024, &EVP_sha384, true, AWSLC_NOT_APPROVED, AWSLC_APPROVED },
    { 1024, &EVP_sha512_224, true, AWSLC_NOT_APPROVED, AWSLC_APPROVED },
    { 1024, &EVP_sha512_256, true, AWSLC_NOT_APPROVED, AWSLC_APPROVED },
    { 1024, &EVP_sha3_224, true, AWSLC_NOT_APPROVED, AWSLC_APPROVED },
    { 1024, &EVP_sha3_256, true, AWSLC_NOT_APPROVED, AWSLC_APPROVED },
    { 1024, &EVP_sha3_384, true, AWSLC_NOT_APPROVED, AWSLC_APPROVED },
    // PSS with hashLen == saltLen is not possible for 1024-bit modulus and
    // SHA-512. This means we can't test it here because the API won't work.

    { 2048, &EVP_sha1, false, AWSLC_NOT_APPROVED, AWSLC_APPROVED },
    { 2048, &EVP_sha224, false, AWSLC_APPROVED, AWSLC_APPROVED },
    { 2048, &EVP_sha256, false, AWSLC_APPROVED, AWSLC_APPROVED },
    { 2048, &EVP_sha384, false, AWSLC_APPROVED, AWSLC_APPROVED },
    { 2048, &EVP_sha512, false, AWSLC_APPROVED, AWSLC_APPROVED },
    { 2048, &EVP_sha512_224, false, AWSLC_APPROVED, AWSLC_APPROVED },
    { 2048, &EVP_sha512_256, false, AWSLC_APPROVED, AWSLC_APPROVED },
    { 2048, &EVP_sha3_224, false, AWSLC_APPROVED, AWSLC_APPROVED },
    { 2048, &EVP_sha3_256, false, AWSLC_APPROVED, AWSLC_APPROVED },
    { 2048, &EVP_sha3_384, false, AWSLC_APPROVED, AWSLC_APPROVED },
    { 2048, &EVP_sha3_512, false, AWSLC_APPROVED, AWSLC_APPROVED },

    { 2048, &EVP_sha1, true, AWSLC_NOT_APPROVED, AWSLC_APPROVED },
    { 2048, &EVP_sha224, true, AWSLC_APPROVED, AWSLC_APPROVED },
    { 2048, &EVP_sha256, true, AWSLC_APPROVED, AWSLC_APPROVED },
    { 2048, &EVP_sha384, true, AWSLC_APPROVED, AWSLC_APPROVED },
    { 2048, &EVP_sha512, true, AWSLC_APPROVED, AWSLC_APPROVED },
    { 2048, &EVP_sha512_224, true, AWSLC_APPROVED, AWSLC_APPROVED },
    { 2048, &EVP_sha512_256, true, AWSLC_APPROVED, AWSLC_APPROVED },
    { 2048, &EVP_sha3_224, true, AWSLC_APPROVED, AWSLC_APPROVED },
    { 2048, &EVP_sha3_256, true, AWSLC_APPROVED, AWSLC_APPROVED },
    { 2048, &EVP_sha3_384, true, AWSLC_APPROVED, AWSLC_APPROVED },
    { 2048, &EVP_sha3_512, true, AWSLC_APPROVED, AWSLC_APPROVED },

    { 3072, &EVP_sha1, false, AWSLC_NOT_APPROVED, AWSLC_APPROVED },
    { 3072, &EVP_sha224, false, AWSLC_APPROVED, AWSLC_APPROVED },
    { 3072, &EVP_sha256, false, AWSLC_APPROVED, AWSLC_APPROVED },
    { 3072, &EVP_sha384, false, AWSLC_APPROVED, AWSLC_APPROVED },
    { 3072, &EVP_sha512, false, AWSLC_APPROVED, AWSLC_APPROVED },
    { 3072, &EVP_sha512_224, false, AWSLC_APPROVED, AWSLC_APPROVED },
    { 3072, &EVP_sha512_256, false, AWSLC_APPROVED, AWSLC_APPROVED },
    { 3072, &EVP_sha3_224, false, AWSLC_APPROVED, AWSLC_APPROVED },
    { 3072, &EVP_sha3_256, false, AWSLC_APPROVED, AWSLC_APPROVED },
    { 3072, &EVP_sha3_384, false, AWSLC_APPROVED, AWSLC_APPROVED },
    { 3072, &EVP_sha3_512, false, AWSLC_APPROVED, AWSLC_APPROVED },

    { 3072, &EVP_sha1, true, AWSLC_NOT_APPROVED, AWSLC_APPROVED },
    { 3072, &EVP_sha224, true, AWSLC_APPROVED, AWSLC_APPROVED },
    { 3072, &EVP_sha256, true, AWSLC_APPROVED, AWSLC_APPROVED },
    { 3072, &EVP_sha384, true, AWSLC_APPROVED, AWSLC_APPROVED },
    { 3072, &EVP_sha512, true, AWSLC_APPROVED, AWSLC_APPROVED },
    { 3072, &EVP_sha512_224, true, AWSLC_APPROVED, AWSLC_APPROVED },
    { 3072, &EVP_sha512_256, true, AWSLC_APPROVED, AWSLC_APPROVED },
    { 3072, &EVP_sha3_224, true, AWSLC_APPROVED, AWSLC_APPROVED },
    { 3072, &EVP_sha3_256, true, AWSLC_APPROVED, AWSLC_APPROVED },
    { 3072, &EVP_sha3_384, true, AWSLC_APPROVED, AWSLC_APPROVED },
    { 3072, &EVP_sha3_512, true, AWSLC_APPROVED, AWSLC_APPROVED },

    { 4096, &EVP_sha1, false, AWSLC_NOT_APPROVED, AWSLC_APPROVED },
    { 4096, &EVP_sha224, false, AWSLC_APPROVED, AWSLC_APPROVED },
    { 4096, &EVP_sha256, false, AWSLC_APPROVED, AWSLC_APPROVED },
    { 4096, &EVP_sha384, false, AWSLC_APPROVED, AWSLC_APPROVED },
    { 4096, &EVP_sha512, false, AWSLC_APPROVED, AWSLC_APPROVED },
    { 4096, &EVP_sha512_224, false, AWSLC_APPROVED, AWSLC_APPROVED },
    { 4096, &EVP_sha512_256, false, AWSLC_APPROVED, AWSLC_APPROVED },
    { 4096, &EVP_sha3_224, false, AWSLC_APPROVED, AWSLC_APPROVED },
    { 4096, &EVP_sha3_256, false, AWSLC_APPROVED, AWSLC_APPROVED },
    { 4096, &EVP_sha3_384, false, AWSLC_APPROVED, AWSLC_APPROVED },
    { 4096, &EVP_sha3_512, false, AWSLC_APPROVED, AWSLC_APPROVED },

    { 4096, &EVP_sha1, true, AWSLC_NOT_APPROVED, AWSLC_APPROVED },
    { 4096, &EVP_sha224, true, AWSLC_APPROVED, AWSLC_APPROVED },
    { 4096, &EVP_sha256, true, AWSLC_APPROVED, AWSLC_APPROVED },
    { 4096, &EVP_sha384, true, AWSLC_APPROVED, AWSLC_APPROVED },
    { 4096, &EVP_sha512, true, AWSLC_APPROVED, AWSLC_APPROVED },
    { 4096, &EVP_sha512_224, true, AWSLC_APPROVED, AWSLC_APPROVED },
    { 4096, &EVP_sha512_256, true, AWSLC_APPROVED, AWSLC_APPROVED },
    { 4096, &EVP_sha3_224, true, AWSLC_APPROVED, AWSLC_APPROVED },
    { 4096, &EVP_sha3_256, true, AWSLC_APPROVED, AWSLC_APPROVED },
    { 4096, &EVP_sha3_384, true, AWSLC_APPROVED, AWSLC_APPROVED },
    { 4096, &EVP_sha3_512, true, AWSLC_APPROVED, AWSLC_APPROVED },

    { 6144, &EVP_sha1, false, AWSLC_NOT_APPROVED, AWSLC_APPROVED },
    { 6144, &EVP_sha224, false, AWSLC_APPROVED, AWSLC_APPROVED },
    { 6144, &EVP_sha256, false, AWSLC_APPROVED, AWSLC_APPROVED },
    { 6144, &EVP_sha384, false, AWSLC_APPROVED, AWSLC_APPROVED },
    { 6144, &EVP_sha512, false, AWSLC_APPROVED, AWSLC_APPROVED },
    { 6144, &EVP_sha512_224, false, AWSLC_APPROVED, AWSLC_APPROVED },
    { 6144, &EVP_sha512_256, false, AWSLC_APPROVED, AWSLC_APPROVED },
    { 6144, &EVP_sha3_224, false, AWSLC_APPROVED, AWSLC_APPROVED },
    { 6144, &EVP_sha3_256, false, AWSLC_APPROVED, AWSLC_APPROVED },
    { 6144, &EVP_sha3_384, false, AWSLC_APPROVED, AWSLC_APPROVED },
    { 6144, &EVP_sha3_512, false, AWSLC_APPROVED, AWSLC_APPROVED },

    { 6144, &EVP_sha1, true, AWSLC_NOT_APPROVED, AWSLC_APPROVED },
    { 6144, &EVP_sha224, true, AWSLC_APPROVED, AWSLC_APPROVED },
    { 6144, &EVP_sha256, true, AWSLC_APPROVED, AWSLC_APPROVED },
    { 6144, &EVP_sha384, true, AWSLC_APPROVED, AWSLC_APPROVED },
    { 6144, &EVP_sha512, true, AWSLC_APPROVED, AWSLC_APPROVED },
    { 6144, &EVP_sha512_224, true, AWSLC_APPROVED, AWSLC_APPROVED },
    { 6144, &EVP_sha512_256, true, AWSLC_APPROVED, AWSLC_APPROVED },
    { 6144, &EVP_sha3_224, true, AWSLC_APPROVED, AWSLC_APPROVED },
    { 6144, &EVP_sha3_256, true, AWSLC_APPROVED, AWSLC_APPROVED },
    { 6144, &EVP_sha3_384, true, AWSLC_APPROVED, AWSLC_APPROVED },
    { 6144, &EVP_sha3_512, true, AWSLC_APPROVED, AWSLC_APPROVED },

    { 8192, &EVP_sha1, false, AWSLC_NOT_APPROVED, AWSLC_APPROVED },
    { 8192, &EVP_sha224, false, AWSLC_APPROVED, AWSLC_APPROVED },
    { 8192, &EVP_sha256, false, AWSLC_APPROVED, AWSLC_APPROVED },
    { 8192, &EVP_sha384, false, AWSLC_APPROVED, AWSLC_APPROVED },
    { 8192, &EVP_sha512, false, AWSLC_APPROVED, AWSLC_APPROVED },
    { 8192, &EVP_sha512_224, false, AWSLC_APPROVED, AWSLC_APPROVED },
    { 8192, &EVP_sha512_256, false, AWSLC_APPROVED, AWSLC_APPROVED },
    { 8192, &EVP_sha3_224, false, AWSLC_APPROVED, AWSLC_APPROVED },
    { 8192, &EVP_sha3_256, false, AWSLC_APPROVED, AWSLC_APPROVED },
    { 8192, &EVP_sha3_384, false, AWSLC_APPROVED, AWSLC_APPROVED },
    { 8192, &EVP_sha3_512, false, AWSLC_APPROVED, AWSLC_APPROVED },

    { 8192, &EVP_sha1, true, AWSLC_NOT_APPROVED, AWSLC_APPROVED },
    { 8192, &EVP_sha224, true, AWSLC_APPROVED, AWSLC_APPROVED },
    { 8192, &EVP_sha256, true, AWSLC_APPROVED, AWSLC_APPROVED },
    { 8192, &EVP_sha384, true, AWSLC_APPROVED, AWSLC_APPROVED },
    { 8192, &EVP_sha512, true, AWSLC_APPROVED, AWSLC_APPROVED },
    { 8192, &EVP_sha512_224, true, AWSLC_APPROVED, AWSLC_APPROVED },
    { 8192, &EVP_sha512_256, true, AWSLC_APPROVED, AWSLC_APPROVED },
    { 8192, &EVP_sha3_224, true, AWSLC_APPROVED, AWSLC_APPROVED },
    { 8192, &EVP_sha3_256, true, AWSLC_APPROVED, AWSLC_APPROVED },
    { 8192, &EVP_sha3_384, true, AWSLC_APPROVED, AWSLC_APPROVED },
    { 8192, &EVP_sha3_512, true, AWSLC_APPROVED, AWSLC_APPROVED },
};

class RSAServiceIndicatorTest : public TestWithNoErrors<RSATestVector> {};

INSTANTIATE_TEST_SUITE_P(All, RSAServiceIndicatorTest,
                         testing::ValuesIn(kRSATestVectors));

static std::map<unsigned, bssl::UniquePtr<RSA>> &CachedRSAKeys() {
  static std::map<unsigned, bssl::UniquePtr<RSA>> keys;
  return keys;
}


static RSA *GetRSAKey(unsigned bits) {
  auto it = CachedRSAKeys().find(bits);
  if (it != CachedRSAKeys().end()) {
    return it->second.get();
  }

  bssl::UniquePtr<BIGNUM> e(BN_new());
  if (!e || !BN_set_word(e.get(), RSA_F4)) {
    abort();
  }

  bssl::UniquePtr<RSA> key(RSA_new());
  if (!key || !RSA_generate_key_ex(key.get(), bits, e.get(), nullptr)) {
    abort();
  }

  RSA *const ret = key.get();
  CachedRSAKeys().emplace(static_cast<unsigned>(bits), std::move(key));
  return ret;
}

static void AssignRSAPSSKey(EVP_PKEY *pkey, unsigned bits) {
  RSA *rsa = GetRSAKey(bits);
  if (rsa == NULL || pkey == NULL) {
    abort();
  }

  // When using |EVP_PKEY_assign| to assign |RSA| to |EVP_PKEY|, the pointer
  // will get assigned to |EVP_PKEY| and get freed along with it. This will not
  // up the reference to RSA unlike |EVP_PKEY_assign_RSA|! So we do that after.
  if (!EVP_PKEY_assign(pkey, EVP_PKEY_RSA_PSS, rsa)) {
    abort();
  }

  RSA_up_ref(rsa);
}

TEST_P(RSAServiceIndicatorTest, RSASigGen) {
  const RSATestVector &test = GetParam();
  SCOPED_TRACE(test.key_size);

  bssl::UniquePtr<EVP_PKEY> pkey(EVP_PKEY_new());
  ASSERT_TRUE(pkey);
  RSA *rsa = nullptr;
  if(test.use_pss) {
    AssignRSAPSSKey(pkey.get(), test.key_size);
  } else {
    rsa = GetRSAKey(test.key_size);
    ASSERT_TRUE(EVP_PKEY_set1_RSA(pkey.get(), rsa));
 }

  // Test running the EVP_DigestSign interfaces one by one directly, and check
  // |EVP_DigestSignFinal| for approval at the end. |EVP_DigestSignInit|, and
  // |EVP_DigestSignUpdate| should not be approved because they do not indicate
  // an entire service has been completed.
  FIPSStatus approved = AWSLC_NOT_APPROVED;
  bssl::ScopedEVP_MD_CTX md_ctx;
  EVP_PKEY_CTX *pctx = nullptr;
  size_t sig_len;
  CALL_SERVICE_AND_CHECK_APPROVED(approved,
        ASSERT_TRUE(EVP_DigestSignInit(md_ctx.get(), &pctx, test.func(),
                                       nullptr, pkey.get())));
  EXPECT_EQ(approved, AWSLC_NOT_APPROVED);
  if (test.use_pss) {
    CALL_SERVICE_AND_CHECK_APPROVED(approved,
        ASSERT_TRUE(EVP_PKEY_CTX_set_rsa_padding(pctx, RSA_PKCS1_PSS_PADDING)));
    EXPECT_EQ(approved, AWSLC_NOT_APPROVED);
    CALL_SERVICE_AND_CHECK_APPROVED(
        approved, ASSERT_TRUE(EVP_PKEY_CTX_set_rsa_pss_saltlen(pctx, -1)));
    EXPECT_EQ(approved, AWSLC_NOT_APPROVED);
  }
  CALL_SERVICE_AND_CHECK_APPROVED(
      approved, ASSERT_TRUE(EVP_DigestSignUpdate(md_ctx.get(), kPlaintext,
                                                 sizeof(kPlaintext))));
  EXPECT_EQ(approved, AWSLC_NOT_APPROVED);
  // Determine the size of the signature. The first call of
  // |EVP_DigestSignFinal| should not return an approval check because no crypto
  // is being done when |nullptr| is inputted in the |*out_sig| field.
  CALL_SERVICE_AND_CHECK_APPROVED(
      approved, ASSERT_TRUE(EVP_DigestSignFinal(md_ctx.get(), nullptr,
                                                &sig_len)));
  EXPECT_EQ(approved, AWSLC_NOT_APPROVED);
  std::vector<uint8_t> signature(sig_len);
  // The second call performs the actual operation.
  CALL_SERVICE_AND_CHECK_APPROVED(
      approved, ASSERT_TRUE(EVP_DigestSignFinal(md_ctx.get(), signature.data(),
                                                &sig_len)));
  EXPECT_EQ(approved, test.sig_gen_expect_approved);

  md_ctx.Reset();
  std::vector<uint8_t> oneshot_output(sig_len);
  CALL_SERVICE_AND_CHECK_APPROVED(
      approved, ASSERT_TRUE(EVP_DigestSignInit(md_ctx.get(), &pctx, test.func(),
                                               nullptr, pkey.get())));
  EXPECT_EQ(approved, AWSLC_NOT_APPROVED);

  if (test.use_pss) {
    CALL_SERVICE_AND_CHECK_APPROVED(
        approved,
        ASSERT_TRUE(EVP_PKEY_CTX_set_rsa_padding(pctx, RSA_PKCS1_PSS_PADDING)));
    EXPECT_EQ(approved, AWSLC_NOT_APPROVED);
    CALL_SERVICE_AND_CHECK_APPROVED(
        approved, ASSERT_TRUE(EVP_PKEY_CTX_set_rsa_pss_saltlen(pctx, -1)));
    EXPECT_EQ(approved, AWSLC_NOT_APPROVED);
  }

  // Test the one-shot |EVP_DigestSign| function to determine size.
  // This should not set the indicator.
  CALL_SERVICE_AND_CHECK_APPROVED(
      approved,
      ASSERT_TRUE(EVP_DigestSign(md_ctx.get(), nullptr, &sig_len, nullptr, 0)));
  EXPECT_EQ(approved, AWSLC_NOT_APPROVED);

  // Now test using the one-shot |EVP_DigestSign| function for approval.
  CALL_SERVICE_AND_CHECK_APPROVED(approved,
      ASSERT_TRUE(EVP_DigestSign(md_ctx.get(), oneshot_output.data(), &sig_len,
                                 kPlaintext, sizeof(kPlaintext))));
  EXPECT_EQ(approved, test.sig_gen_expect_approved);

  if (test.use_pss) {
    // Odd configurations of PSS, for example where the salt length is not equal
    // to the hash length, are not approved.
    md_ctx.Reset();
    ASSERT_TRUE(EVP_DigestSignInit(md_ctx.get(), &pctx, test.func(), nullptr,
                                   pkey.get()));
    ASSERT_TRUE(EVP_PKEY_CTX_set_rsa_padding(pctx, RSA_PKCS1_PSS_PADDING));
    ASSERT_TRUE(EVP_PKEY_CTX_set_rsa_pss_saltlen(pctx, 10));
    CALL_SERVICE_AND_CHECK_APPROVED(approved,
        ASSERT_TRUE(EVP_DigestSign(md_ctx.get(), oneshot_output.data(),
                                   &sig_len, kPlaintext, sizeof(kPlaintext))));
    EXPECT_EQ(approved, AWSLC_NOT_APPROVED);
  }
}

TEST_P(RSAServiceIndicatorTest, RSASigVer) {
  const RSATestVector &test = GetParam();

  bssl::UniquePtr<EVP_PKEY> pkey(EVP_PKEY_new());
  ASSERT_TRUE(pkey);

  RSA *rsa = nullptr;
  if(test.use_pss) {
    AssignRSAPSSKey(pkey.get(), test.key_size);
  } else {
    rsa = GetRSAKey(test.key_size);
    ASSERT_TRUE(EVP_PKEY_set1_RSA(pkey.get(), rsa));
 }

  std::vector<uint8_t> signature;
  size_t sig_len;
  bssl::ScopedEVP_MD_CTX md_ctx;
  EVP_PKEY_CTX *pctx = nullptr;
  ASSERT_TRUE(EVP_DigestSignInit(md_ctx.get(), &pctx, test.func(), nullptr,
                                 pkey.get()));
  if (test.use_pss) {
    ASSERT_TRUE(EVP_PKEY_CTX_set_rsa_padding(pctx, RSA_PKCS1_PSS_PADDING));
    ASSERT_TRUE(EVP_PKEY_CTX_set_rsa_pss_saltlen(pctx, -1));
  }
  ASSERT_TRUE(EVP_DigestSign(md_ctx.get(), nullptr, &sig_len, nullptr, 0));
  signature.resize(sig_len);
  ASSERT_TRUE(EVP_DigestSign(md_ctx.get(), signature.data(), &sig_len,
                             kPlaintext, sizeof(kPlaintext)));
  signature.resize(sig_len);

  // Service Indicator approval checks for RSA signature verification.

  // Test running the EVP_DigestVerify interfaces one by one directly, and check
  // |EVP_DigestVerifyFinal| for approval at the end. |EVP_DigestVerifyInit|,
  // |EVP_DigestVerifyUpdate| should not be approved because they do not
  // indicate an entire service has been done.
  FIPSStatus approved = AWSLC_NOT_APPROVED;
  md_ctx.Reset();
  CALL_SERVICE_AND_CHECK_APPROVED(approved,
      ASSERT_TRUE(EVP_DigestVerifyInit(md_ctx.get(), &pctx, test.func(),
                                       nullptr, pkey.get())));
  EXPECT_EQ(approved, AWSLC_NOT_APPROVED);
  if (test.use_pss) {
    CALL_SERVICE_AND_CHECK_APPROVED(approved,
      ASSERT_TRUE(EVP_PKEY_CTX_set_rsa_padding(pctx, RSA_PKCS1_PSS_PADDING)));
    EXPECT_EQ(approved, AWSLC_NOT_APPROVED);
    ASSERT_TRUE(EVP_PKEY_CTX_set_rsa_pss_saltlen(pctx, -1));
  }
  CALL_SERVICE_AND_CHECK_APPROVED(approved,
      ASSERT_TRUE(EVP_DigestVerifyUpdate(md_ctx.get(), kPlaintext,
                                         sizeof(kPlaintext))));
  EXPECT_EQ(approved, AWSLC_NOT_APPROVED);
  CALL_SERVICE_AND_CHECK_APPROVED(approved,
      ASSERT_TRUE(EVP_DigestVerifyFinal(md_ctx.get(), signature.data(),
                                        signature.size())));
  EXPECT_EQ(approved, test.sig_ver_expect_approved);

  // Test using the one-shot |EVP_DigestVerify| function for approval.
  md_ctx.Reset();
  CALL_SERVICE_AND_CHECK_APPROVED(approved,
      ASSERT_TRUE(EVP_DigestVerifyInit(md_ctx.get(), &pctx, test.func(),
                                       nullptr, pkey.get())));
  EXPECT_EQ(approved, AWSLC_NOT_APPROVED);
  if (test.use_pss) {
    CALL_SERVICE_AND_CHECK_APPROVED(approved,
      ASSERT_TRUE(EVP_PKEY_CTX_set_rsa_padding(pctx, RSA_PKCS1_PSS_PADDING)));
    EXPECT_EQ(approved, AWSLC_NOT_APPROVED);
    ASSERT_TRUE(EVP_PKEY_CTX_set_rsa_pss_saltlen(pctx, -1));
  }
  CALL_SERVICE_AND_CHECK_APPROVED(approved,
      ASSERT_TRUE(EVP_DigestVerify(md_ctx.get(), signature.data(),
                                   signature.size(), kPlaintext,
                                   sizeof(kPlaintext))));
  EXPECT_EQ(approved, test.sig_ver_expect_approved);
}

// Test that |EVP_DigestSignFinal| and |EVP_DigestSignVerify| are approved with
// manually constructing using the context setting functions.
// AWS-LC uses this to test support of the function |EVP_MD_CTX_set_pkey_ctx|.
TEST_P(RSAServiceIndicatorTest, ManualRSASignVerify) {
  const RSATestVector &test = GetParam();

  FIPSStatus approved = AWSLC_NOT_APPROVED;

  bssl::UniquePtr<EVP_PKEY> pkey(EVP_PKEY_new());
  ASSERT_TRUE(pkey);

  RSA *rsa = nullptr;
  if(test.use_pss) {
    AssignRSAPSSKey(pkey.get(), test.key_size);
  } else {
    rsa = GetRSAKey(test.key_size);
    ASSERT_TRUE(EVP_PKEY_set1_RSA(pkey.get(), rsa));
 }

  bssl::ScopedEVP_MD_CTX ctx;
  ASSERT_TRUE(EVP_DigestInit(ctx.get(), test.func()));
  ASSERT_TRUE(EVP_DigestUpdate(ctx.get(), kPlaintext, sizeof(kPlaintext)));

  // Manual construction for signing.
  bssl::UniquePtr<EVP_PKEY_CTX> pctx(EVP_PKEY_CTX_new(pkey.get(), nullptr));
  ASSERT_TRUE(EVP_PKEY_sign_init(pctx.get()));
  ASSERT_TRUE(EVP_PKEY_CTX_set_signature_md(pctx.get(), test.func()));
  if (test.use_pss) {
    ASSERT_TRUE(EVP_PKEY_CTX_set_rsa_padding(pctx.get(),
                                             RSA_PKCS1_PSS_PADDING));
    ASSERT_TRUE(EVP_PKEY_CTX_set_rsa_pss_saltlen(pctx.get(), -1));
  }
  EVP_MD_CTX_set_pkey_ctx(ctx.get(), pctx.get());
  // Determine the size of the signature.
  size_t sig_len = 0;
  CALL_SERVICE_AND_CHECK_APPROVED(approved,
    ASSERT_TRUE(EVP_DigestSignFinal(ctx.get(), nullptr, &sig_len)));
  ASSERT_EQ(approved, AWSLC_NOT_APPROVED);

  std::vector<uint8_t> sig;
  sig.resize(sig_len);
  CALL_SERVICE_AND_CHECK_APPROVED(approved,
    EVP_DigestSignFinal(ctx.get(), sig.data(), &sig_len));
  ASSERT_EQ(approved, test.sig_gen_expect_approved);
  sig.resize(sig_len);

  // Manual construction for verification.
  ASSERT_TRUE(EVP_PKEY_verify_init(pctx.get()));
  ASSERT_TRUE(EVP_PKEY_CTX_set_signature_md(pctx.get(), test.func()));
  EVP_MD_CTX_set_pkey_ctx(ctx.get(), pctx.get());

  CALL_SERVICE_AND_CHECK_APPROVED(approved,
            EVP_DigestVerifyFinal(ctx.get(), sig.data(), sig_len));
  ASSERT_EQ(approved, test.sig_ver_expect_approved);
}

static int custom_sign(int max_out, const uint8_t *in, uint8_t *out, RSA *rsa,
                       int padding) {
  return 0;
}

static int custom_finish(RSA *rsa) {
  const RSA_METHOD *meth = RSA_get_method(rsa);
  RSA_meth_free((RSA_METHOD *) meth);
  return 1;
}

TEST_P(RSAServiceIndicatorTest, RSAMethod) {
  const RSATestVector &test = GetParam();

  FIPSStatus approved = AWSLC_NOT_APPROVED;

  bssl::UniquePtr<EVP_PKEY> pkey(EVP_PKEY_new());
  ASSERT_TRUE(pkey);

  RSA *rsa = nullptr;
  if(test.use_pss) {
    AssignRSAPSSKey(pkey.get(), test.key_size);
  } else {
    rsa = GetRSAKey(test.key_size);
    ASSERT_TRUE(EVP_PKEY_set1_RSA(pkey.get(), rsa));
  }

  // If meth is previously set, avoid overwriting cached RSA key
  if (!EVP_PKEY_get0_RSA(pkey.get())->meth->sign_raw) {
    RSA_METHOD *meth = RSA_meth_new(NULL, 0);
    RSA_meth_set_priv_enc(meth, custom_sign);
    RSA_meth_set_finish(meth, custom_finish);
    RSA_set_method(EVP_PKEY_get0_RSA(pkey.get()), meth);
  }

  bssl::ScopedEVP_MD_CTX ctx;
  ASSERT_TRUE(EVP_DigestInit(ctx.get(), test.func()));
  ASSERT_TRUE(EVP_DigestUpdate(ctx.get(), kPlaintext, sizeof(kPlaintext)));

  // Manual construction for signing.
  bssl::UniquePtr<EVP_PKEY_CTX> pctx(EVP_PKEY_CTX_new(pkey.get(), nullptr));
  ASSERT_TRUE(EVP_PKEY_sign_init(pctx.get()));
  ASSERT_TRUE(EVP_PKEY_CTX_set_signature_md(pctx.get(), test.func()));
  if (test.use_pss) {
    ASSERT_TRUE(EVP_PKEY_CTX_set_rsa_padding(pctx.get(),
                                             RSA_PKCS1_PSS_PADDING));
    ASSERT_TRUE(EVP_PKEY_CTX_set_rsa_pss_saltlen(pctx.get(), -1));
  }
  EVP_MD_CTX_set_pkey_ctx(ctx.get(), pctx.get());
  // Determine the size of the signature.
  size_t sig_len = 0;
  CALL_SERVICE_AND_CHECK_APPROVED(approved,
                                  ASSERT_TRUE(EVP_DigestSignFinal(ctx.get(), nullptr, &sig_len)));
  ASSERT_EQ(approved, AWSLC_NOT_APPROVED);

  std::vector<uint8_t> sig;
  sig.resize(sig_len);
  // Custom sign will be called, never approved
  CALL_SERVICE_AND_CHECK_APPROVED(approved,
                                  EVP_DigestSignFinal(ctx.get(), sig.data(), &sig_len));

  ASSERT_EQ(approved, AWSLC_NOT_APPROVED);
  sig.resize(sig_len);
}

struct ECDSATestVector {
  // nid is the input curve nid.
  const int nid;
  // md_func is the digest to test.
  const EVP_MD *(*func)();
  // expected to be approved or not for key generation.
  const FIPSStatus key_check_expect_approved;
  // expected to be approved or not for signature generation.
  const FIPSStatus sig_gen_expect_approved;
  // expected to be approved or not for signature verification.
  const FIPSStatus sig_ver_expect_approved;
};

static const struct ECDSATestVector kECDSATestVectors[] = {
    // Only the following NIDs for |EC_GROUP| are creatable with
    // |EC_GROUP_new_by_curve_name|, and |NID_secp256k1| will only work if
    // |kCurveSecp256k1Supported| is true.
    {NID_secp224r1, &EVP_sha1, AWSLC_APPROVED, AWSLC_NOT_APPROVED,
     AWSLC_APPROVED},
    {NID_secp224r1, &EVP_sha224, AWSLC_APPROVED, AWSLC_APPROVED,
     AWSLC_APPROVED},
    {NID_secp224r1, &EVP_sha256, AWSLC_APPROVED, AWSLC_APPROVED,
     AWSLC_APPROVED},
    {NID_secp224r1, &EVP_sha384, AWSLC_APPROVED, AWSLC_APPROVED,
     AWSLC_APPROVED},
    {NID_secp224r1, &EVP_sha512, AWSLC_APPROVED, AWSLC_APPROVED,
     AWSLC_APPROVED},
    {NID_secp224r1, &EVP_sha512_224, AWSLC_APPROVED, AWSLC_APPROVED,
     AWSLC_APPROVED},
    {NID_secp224r1, &EVP_sha512_256, AWSLC_APPROVED, AWSLC_APPROVED,
     AWSLC_APPROVED},
    {NID_secp224r1, &EVP_sha3_224, AWSLC_APPROVED, AWSLC_APPROVED,
     AWSLC_APPROVED},
    {NID_secp224r1, &EVP_sha3_256, AWSLC_APPROVED, AWSLC_APPROVED,
     AWSLC_APPROVED},
    {NID_secp224r1, &EVP_sha3_384, AWSLC_APPROVED, AWSLC_APPROVED,
     AWSLC_APPROVED},
    {NID_secp224r1, &EVP_sha3_512, AWSLC_APPROVED, AWSLC_APPROVED,
     AWSLC_APPROVED},

    {NID_X9_62_prime256v1, &EVP_sha1, AWSLC_APPROVED, AWSLC_NOT_APPROVED,
     AWSLC_APPROVED},
    {NID_X9_62_prime256v1, &EVP_sha224, AWSLC_APPROVED, AWSLC_APPROVED,
     AWSLC_APPROVED},
    {NID_X9_62_prime256v1, &EVP_sha256, AWSLC_APPROVED, AWSLC_APPROVED,
     AWSLC_APPROVED},
    {NID_X9_62_prime256v1, &EVP_sha384, AWSLC_APPROVED, AWSLC_APPROVED,
     AWSLC_APPROVED},
    {NID_X9_62_prime256v1, &EVP_sha512, AWSLC_APPROVED, AWSLC_APPROVED,
     AWSLC_APPROVED},
    {NID_X9_62_prime256v1, &EVP_sha512_224, AWSLC_APPROVED, AWSLC_APPROVED,
     AWSLC_APPROVED},
    {NID_X9_62_prime256v1, &EVP_sha512_256, AWSLC_APPROVED, AWSLC_APPROVED,
     AWSLC_APPROVED},
    {NID_X9_62_prime256v1, &EVP_sha3_224, AWSLC_APPROVED, AWSLC_APPROVED,
     AWSLC_APPROVED},
    {NID_X9_62_prime256v1, &EVP_sha3_256, AWSLC_APPROVED, AWSLC_APPROVED,
     AWSLC_APPROVED},
    {NID_X9_62_prime256v1, &EVP_sha3_384, AWSLC_APPROVED, AWSLC_APPROVED,
     AWSLC_APPROVED},
    {NID_X9_62_prime256v1, &EVP_sha3_512, AWSLC_APPROVED, AWSLC_APPROVED,
     AWSLC_APPROVED},

    {NID_secp384r1, &EVP_sha1, AWSLC_APPROVED, AWSLC_NOT_APPROVED,
     AWSLC_APPROVED},
    {NID_secp384r1, &EVP_sha224, AWSLC_APPROVED, AWSLC_APPROVED,
     AWSLC_APPROVED},
    {NID_secp384r1, &EVP_sha256, AWSLC_APPROVED, AWSLC_APPROVED,
     AWSLC_APPROVED},
    {NID_secp384r1, &EVP_sha384, AWSLC_APPROVED, AWSLC_APPROVED,
     AWSLC_APPROVED},
    {NID_secp384r1, &EVP_sha512, AWSLC_APPROVED, AWSLC_APPROVED,
     AWSLC_APPROVED},
    {NID_secp384r1, &EVP_sha512_224, AWSLC_APPROVED, AWSLC_APPROVED,
     AWSLC_APPROVED},
    {NID_secp384r1, &EVP_sha512_256, AWSLC_APPROVED, AWSLC_APPROVED,
     AWSLC_APPROVED},
    {NID_secp384r1, &EVP_sha3_224, AWSLC_APPROVED, AWSLC_APPROVED,
     AWSLC_APPROVED},
    {NID_secp384r1, &EVP_sha3_256, AWSLC_APPROVED, AWSLC_APPROVED,
     AWSLC_APPROVED},
    {NID_secp384r1, &EVP_sha3_384, AWSLC_APPROVED, AWSLC_APPROVED,
     AWSLC_APPROVED},
    {NID_secp384r1, &EVP_sha3_512, AWSLC_APPROVED, AWSLC_APPROVED,
     AWSLC_APPROVED},

    {NID_secp521r1, &EVP_sha1, AWSLC_APPROVED, AWSLC_NOT_APPROVED,
     AWSLC_APPROVED},
    {NID_secp521r1, &EVP_sha224, AWSLC_APPROVED, AWSLC_APPROVED,
     AWSLC_APPROVED},
    {NID_secp521r1, &EVP_sha256, AWSLC_APPROVED, AWSLC_APPROVED,
     AWSLC_APPROVED},
    {NID_secp521r1, &EVP_sha384, AWSLC_APPROVED, AWSLC_APPROVED,
     AWSLC_APPROVED},
    {NID_secp521r1, &EVP_sha512, AWSLC_APPROVED, AWSLC_APPROVED,
     AWSLC_APPROVED},
    {NID_secp521r1, &EVP_sha512_224, AWSLC_APPROVED, AWSLC_APPROVED,
     AWSLC_APPROVED},
    {NID_secp521r1, &EVP_sha512_256, AWSLC_APPROVED, AWSLC_APPROVED,
     AWSLC_APPROVED},
    {NID_secp521r1, &EVP_sha3_224, AWSLC_APPROVED, AWSLC_APPROVED,
     AWSLC_APPROVED},
    {NID_secp521r1, &EVP_sha3_256, AWSLC_APPROVED, AWSLC_APPROVED,
     AWSLC_APPROVED},
    {NID_secp521r1, &EVP_sha3_384, AWSLC_APPROVED, AWSLC_APPROVED,
     AWSLC_APPROVED},
    {NID_secp521r1, &EVP_sha3_512, AWSLC_APPROVED, AWSLC_APPROVED,
     AWSLC_APPROVED},

    {NID_secp256k1, &EVP_sha1, AWSLC_NOT_APPROVED, AWSLC_NOT_APPROVED,
     AWSLC_NOT_APPROVED},
    {NID_secp256k1, &EVP_sha224, AWSLC_NOT_APPROVED, AWSLC_NOT_APPROVED,
     AWSLC_NOT_APPROVED},
    {NID_secp256k1, &EVP_sha256, AWSLC_NOT_APPROVED, AWSLC_NOT_APPROVED,
     AWSLC_NOT_APPROVED},
    {NID_secp256k1, &EVP_sha384, AWSLC_NOT_APPROVED, AWSLC_NOT_APPROVED,
     AWSLC_NOT_APPROVED},
    {NID_secp256k1, &EVP_sha512, AWSLC_NOT_APPROVED, AWSLC_NOT_APPROVED,
     AWSLC_NOT_APPROVED},
    {NID_secp256k1, &EVP_sha512_224, AWSLC_NOT_APPROVED, AWSLC_NOT_APPROVED,
     AWSLC_NOT_APPROVED},
    {NID_secp256k1, &EVP_sha512_256, AWSLC_NOT_APPROVED, AWSLC_NOT_APPROVED,
     AWSLC_NOT_APPROVED},
    {NID_secp256k1, &EVP_sha3_224, AWSLC_NOT_APPROVED, AWSLC_NOT_APPROVED,
     AWSLC_NOT_APPROVED},
    {NID_secp256k1, &EVP_sha3_256, AWSLC_NOT_APPROVED, AWSLC_NOT_APPROVED,
     AWSLC_NOT_APPROVED},
    {NID_secp256k1, &EVP_sha3_384, AWSLC_NOT_APPROVED, AWSLC_NOT_APPROVED,
     AWSLC_NOT_APPROVED},
    {NID_secp256k1, &EVP_sha3_512, AWSLC_NOT_APPROVED, AWSLC_NOT_APPROVED,
     AWSLC_NOT_APPROVED},
};

class ECDSAServiceIndicatorTest : public TestWithNoErrors<ECDSATestVector> {};

INSTANTIATE_TEST_SUITE_P(All, ECDSAServiceIndicatorTest,
                         testing::ValuesIn(kECDSATestVectors));

TEST_P(ECDSAServiceIndicatorTest, ECDSAKeyCheck) {
  const ECDSATestVector &test = GetParam();
  if (test.nid == NID_secp256k1 && !kCurveSecp256k1Supported) {
    GTEST_SKIP();
  }

  FIPSStatus approved = AWSLC_NOT_APPROVED;

  // Test service indicator approval for |EC_KEY_generate_key_fips| and
  // |EC_KEY_check_fips|.
  bssl::UniquePtr<EC_KEY> key(EC_KEY_new_by_curve_name(test.nid));
  CALL_SERVICE_AND_CHECK_APPROVED(
      approved, ASSERT_TRUE(EC_KEY_generate_key_fips(key.get())));
  EXPECT_EQ(approved, test.key_check_expect_approved);
  CALL_SERVICE_AND_CHECK_APPROVED(approved,
                                  ASSERT_TRUE(EC_KEY_check_fips(key.get())));
  EXPECT_EQ(approved, test.key_check_expect_approved);

  // See if |EC_KEY_check_fips| still returns approval with only the public
  // component.
  bssl::UniquePtr<EC_KEY> key_only_public(EC_KEY_new_by_curve_name(test.nid));
  ASSERT_TRUE(EC_KEY_set_public_key(key_only_public.get(),
                                    EC_KEY_get0_public_key(key.get())));
  CALL_SERVICE_AND_CHECK_APPROVED(
      approved, ASSERT_TRUE(EC_KEY_check_fips(key_only_public.get())));
  EXPECT_EQ(approved, test.key_check_expect_approved);

  if (kEVPKeyGenShouldCallFIPSFunctions) {
    // Test running the EVP_PKEY_keygen interfaces one by one directly, and
    // check |EVP_PKEY_keygen| for approval at the end. |EVP_PKEY_keygen_init|
    // should not be approved because it does not indicate that an entire
    // service has been completed.
    bssl::UniquePtr<EVP_PKEY_CTX> ctx(
        EVP_PKEY_CTX_new_id(EVP_PKEY_EC, nullptr));
    EVP_PKEY *raw = nullptr;
    ASSERT_TRUE(ctx);
    ASSERT_TRUE(EVP_PKEY_keygen_init(ctx.get()));
    ASSERT_TRUE(EVP_PKEY_CTX_set_ec_paramgen_curve_nid(ctx.get(), test.nid));
    CALL_SERVICE_AND_CHECK_APPROVED(approved,
        ASSERT_TRUE(EVP_PKEY_keygen(ctx.get(), &raw)));
    EXPECT_EQ(approved, test.key_check_expect_approved);

    EVP_PKEY_free(raw);
  }
}

TEST_P(ECDSAServiceIndicatorTest, ECDSASigGen) {
  const ECDSATestVector &test = GetParam();
  if (test.nid == NID_secp256k1 && !kCurveSecp256k1Supported) {
    GTEST_SKIP();
  }

  FIPSStatus approved = AWSLC_NOT_APPROVED;

  const EC_GROUP *group = EC_GROUP_new_by_curve_name(test.nid);
  bssl::UniquePtr<EC_KEY> eckey(EC_KEY_new());
  bssl::UniquePtr<EVP_PKEY> pkey(EVP_PKEY_new());
  bssl::ScopedEVP_MD_CTX md_ctx;
  ASSERT_TRUE(eckey);
  ASSERT_TRUE(EC_KEY_set_group(eckey.get(), group));

  // Generate a generic EC key.
  ASSERT_TRUE(EC_KEY_generate_key(eckey.get()));
  ASSERT_TRUE(EVP_PKEY_set1_EC_KEY(pkey.get(), eckey.get()));

  // Test running the EVP_DigestSign interfaces one by one directly, and check
  // |EVP_DigestSignFinal| for approval at the end. |EVP_DigestSignInit|,
  // |EVP_DigestSignUpdate| should not be approved because they do not indicate
  // an entire service has been done.
  CALL_SERVICE_AND_CHECK_APPROVED(
      approved, ASSERT_TRUE(EVP_DigestSignInit(md_ctx.get(), nullptr,
                                               test.func(), nullptr,
                                               pkey.get())));
  EXPECT_EQ(approved, AWSLC_NOT_APPROVED);
  CALL_SERVICE_AND_CHECK_APPROVED(approved,
      ASSERT_TRUE(EVP_DigestSignUpdate(md_ctx.get(), kPlaintext,
                                       sizeof(kPlaintext))));
  EXPECT_EQ(approved, AWSLC_NOT_APPROVED);
  // Determine the size of the signature. The first call of
  // |EVP_DigestSignFinal| should not return an approval check because no crypto
  // is being done when |nullptr| is given as the |out_sig| field.
  size_t max_sig_len;
  CALL_SERVICE_AND_CHECK_APPROVED(approved,
      ASSERT_TRUE(EVP_DigestSignFinal(md_ctx.get(), nullptr, &max_sig_len)));
  EXPECT_EQ(approved, AWSLC_NOT_APPROVED);
  std::vector<uint8_t> signature(max_sig_len);
  // The second call performs the actual operation.
  size_t sig_len = max_sig_len;
  CALL_SERVICE_AND_CHECK_APPROVED(approved,
      ASSERT_TRUE(EVP_DigestSignFinal(md_ctx.get(), signature.data(),
                                      &sig_len)));
  ASSERT_LE(sig_len, signature.size());
  EXPECT_EQ(approved, test.sig_gen_expect_approved);

  // Test using the one-shot |EVP_DigestSign| function for approval.
  md_ctx.Reset();
  CALL_SERVICE_AND_CHECK_APPROVED(approved,
      ASSERT_TRUE(EVP_DigestSignInit(md_ctx.get(), nullptr, test.func(),
                                     nullptr, pkey.get())));
  EXPECT_EQ(approved, AWSLC_NOT_APPROVED);
  sig_len = max_sig_len;
  CALL_SERVICE_AND_CHECK_APPROVED(approved,
      ASSERT_TRUE(EVP_DigestSign(md_ctx.get(), signature.data(), &sig_len,
                                 kPlaintext, sizeof(kPlaintext))));
  ASSERT_LE(sig_len, signature.size());
  EXPECT_EQ(approved, test.sig_gen_expect_approved);
}

TEST_P(ECDSAServiceIndicatorTest, ECDSASigVer) {
  const ECDSATestVector &test = GetParam();
  if (test.nid == NID_secp256k1 && !kCurveSecp256k1Supported) {
    GTEST_SKIP();
  }

  FIPSStatus approved = AWSLC_NOT_APPROVED;

  const EC_GROUP *group = EC_GROUP_new_by_curve_name(test.nid);
  bssl::UniquePtr<EC_KEY> eckey(EC_KEY_new());
  bssl::UniquePtr<EVP_PKEY> pkey(EVP_PKEY_new());
  bssl::ScopedEVP_MD_CTX md_ctx;
  ASSERT_TRUE(eckey);
  ASSERT_TRUE(EC_KEY_set_group(eckey.get(), group));

  // Generate ECDSA signatures for ECDSA verification.
  ASSERT_TRUE(EC_KEY_generate_key(eckey.get()));
  ASSERT_TRUE(EVP_PKEY_set1_EC_KEY(pkey.get(), eckey.get()));
  std::vector<uint8_t> signature;
  size_t sig_len = 0;
  ASSERT_TRUE(EVP_DigestSignInit(md_ctx.get(), nullptr, test.func(), nullptr,
                                 pkey.get()));
  ASSERT_TRUE(EVP_DigestSignFinal(md_ctx.get(), nullptr, &sig_len));
  signature.resize(sig_len);
  ASSERT_TRUE(EVP_DigestSign(md_ctx.get(), signature.data(), &sig_len,
                             kPlaintext, sizeof(kPlaintext)));
  signature.resize(sig_len);

  // Service Indicator approval checks for ECDSA signature verification.

  // Test running the EVP_DigestVerify interfaces one by one directly, and check
  // |EVP_DigestVerifyFinal| for approval at the end. |EVP_DigestVerifyInit|,
  // |EVP_DigestVerifyUpdate| should not be approved because they do not
  // indicate an entire service has been done.
  md_ctx.Reset();
  CALL_SERVICE_AND_CHECK_APPROVED(approved,
      ASSERT_TRUE(EVP_DigestVerifyInit(md_ctx.get(), nullptr, test.func(),
                                       nullptr, pkey.get())));
  EXPECT_EQ(approved, AWSLC_NOT_APPROVED);
  CALL_SERVICE_AND_CHECK_APPROVED(approved,
      ASSERT_TRUE(EVP_DigestVerifyUpdate(md_ctx.get(), kPlaintext,
                                         sizeof(kPlaintext))));
  EXPECT_EQ(approved, AWSLC_NOT_APPROVED);
  CALL_SERVICE_AND_CHECK_APPROVED(approved,
      ASSERT_TRUE(EVP_DigestVerifyFinal(md_ctx.get(), signature.data(),
                                        signature.size())));
  EXPECT_EQ(approved, test.sig_ver_expect_approved);

  // Test using the one-shot |EVP_DigestVerify| function for approval.
  md_ctx.Reset();
  CALL_SERVICE_AND_CHECK_APPROVED(approved,
      ASSERT_TRUE(EVP_DigestVerifyInit(md_ctx.get(), nullptr, test.func(),
                                       nullptr, pkey.get())));
  EXPECT_EQ(approved, AWSLC_NOT_APPROVED);
  CALL_SERVICE_AND_CHECK_APPROVED(approved,
      ASSERT_TRUE(EVP_DigestVerify(md_ctx.get(), signature.data(),
                                   signature.size(), kPlaintext,
                                   sizeof(kPlaintext))));
  EXPECT_EQ(approved, test.sig_ver_expect_approved);
}

// Test that |EVP_DigestSignFinal| and |EVP_DigestSignVerify| are approved with
// manually constructing using the context setting functions.
TEST_P(ECDSAServiceIndicatorTest, ManualECDSASignVerify) {
  const ECDSATestVector &test = GetParam();

  FIPSStatus approved = AWSLC_NOT_APPROVED;

  bssl::ScopedEVP_MD_CTX ctx;
  ASSERT_TRUE(EVP_DigestInit(ctx.get(), test.func()));
  ASSERT_TRUE(EVP_DigestUpdate(ctx.get(), kPlaintext, sizeof(kPlaintext)));

  const EC_GROUP *group = EC_GROUP_new_by_curve_name(test.nid);
  bssl::UniquePtr<EC_KEY> eckey(EC_KEY_new());
  bssl::UniquePtr<EVP_PKEY> pkey(EVP_PKEY_new());
  bssl::ScopedEVP_MD_CTX md_ctx;
  ASSERT_TRUE(eckey);
  ASSERT_TRUE(EC_KEY_set_group(eckey.get(), group));

  // Generate a generic ec key.
  EC_KEY_generate_key(eckey.get());
  ASSERT_TRUE(EVP_PKEY_set1_EC_KEY(pkey.get(), eckey.get()));

  // Manual construction for signing.
  bssl::UniquePtr<EVP_PKEY_CTX> pctx(EVP_PKEY_CTX_new(pkey.get(), nullptr));
  ASSERT_TRUE(EVP_PKEY_sign_init(pctx.get()));
  ASSERT_TRUE(EVP_PKEY_CTX_set_signature_md(pctx.get(), test.func()));
  EVP_MD_CTX_set_pkey_ctx(ctx.get(), pctx.get());
  // Determine the size of the signature.
  size_t sig_len = 0;
  CALL_SERVICE_AND_CHECK_APPROVED(approved,
      ASSERT_TRUE(EVP_DigestSignFinal(ctx.get(), nullptr, &sig_len)));
  EXPECT_EQ(approved, AWSLC_NOT_APPROVED);

  std::vector<uint8_t> sig;
  sig.resize(sig_len);
  CALL_SERVICE_AND_CHECK_APPROVED(approved,
      ASSERT_TRUE(EVP_DigestSignFinal(ctx.get(), sig.data(), &sig_len)));
  EXPECT_EQ(approved, test.sig_gen_expect_approved);
  sig.resize(sig_len);

  // Manual construction for verification.
  ASSERT_TRUE(EVP_PKEY_verify_init(pctx.get()));
  ASSERT_TRUE(EVP_PKEY_CTX_set_signature_md(pctx.get(), test.func()));
  EVP_MD_CTX_set_pkey_ctx(ctx.get(), pctx.get());

  CALL_SERVICE_AND_CHECK_APPROVED(approved,
      ASSERT_TRUE(EVP_DigestVerifyFinal(ctx.get(), sig.data(), sig_len)));
  EXPECT_EQ(approved, test.sig_ver_expect_approved);
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

static void ecdsa_finish(EC_KEY *ec)
{
  const EC_KEY_METHOD *ec_meth = EC_KEY_get_method(ec);
  EC_KEY_METHOD_free((EC_KEY_METHOD *) ec_meth);
}

TEST_P(ECDSAServiceIndicatorTest, ECKeyMethod) {
  const ECDSATestVector &test = GetParam();
  if (test.nid == NID_secp256k1 && !kCurveSecp256k1Supported) {
    GTEST_SKIP();
  }

  FIPSStatus approved = AWSLC_NOT_APPROVED;

  const EC_GROUP *group = EC_GROUP_new_by_curve_name(test.nid);
  bssl::UniquePtr<EC_KEY> eckey(EC_KEY_new());
  bssl::UniquePtr<EVP_PKEY> pkey(EVP_PKEY_new());
  bssl::ScopedEVP_MD_CTX md_ctx;
  ASSERT_TRUE(eckey);
  ASSERT_TRUE(EC_KEY_set_group(eckey.get(), group));

  EC_KEY_METHOD *meth = EC_KEY_METHOD_new(NULL);
  EC_KEY_METHOD_set_sign(meth, ecdsa_sign, NULL, NULL);
  EC_KEY_METHOD_set_init(meth, NULL, ecdsa_finish, NULL, NULL, NULL, NULL);
  EC_KEY_set_method(eckey.get(), meth);

  // Generate an EC key.
  ASSERT_TRUE(EC_KEY_generate_key(eckey.get()));
  ASSERT_TRUE(EVP_PKEY_set1_EC_KEY(pkey.get(), eckey.get()));

  // Test running the EVP_DigestSign interfaces one by one directly, and check
  // |EVP_DigestSignFinal| for approval at the end. |EVP_DigestSignInit|,
  // |EVP_DigestSignUpdate| should not be approved because they do not indicate
  // an entire service has been done.
  CALL_SERVICE_AND_CHECK_APPROVED(
          approved, ASSERT_TRUE(EVP_DigestSignInit(md_ctx.get(), nullptr,
                                                   test.func(), nullptr,
                                                   pkey.get())));
  EXPECT_EQ(approved, AWSLC_NOT_APPROVED);
  CALL_SERVICE_AND_CHECK_APPROVED(approved,
                                  ASSERT_TRUE(EVP_DigestSignUpdate(md_ctx.get(),
                                                                   kPlaintext,
                                                                   sizeof(kPlaintext))));
  EXPECT_EQ(approved, AWSLC_NOT_APPROVED);
  // Determine the size of the signature. The first call of
  // |EVP_DigestSignFinal| should not return an approval check because no crypto
  // is being done when |nullptr| is given as the |out_sig| field.
  size_t max_sig_len;
  CALL_SERVICE_AND_CHECK_APPROVED(approved,
                                  ASSERT_TRUE(EVP_DigestSignFinal(md_ctx.get(),
                                                                  nullptr,
                                                                  &max_sig_len)));
  EXPECT_EQ(approved, AWSLC_NOT_APPROVED);
  std::vector<uint8_t> signature(max_sig_len);
  // The second call performs the actual operation and should not return an
  // approval because custom sign functionality is defined.
  size_t sig_len = max_sig_len;
  CALL_SERVICE_AND_CHECK_APPROVED(approved,
                                  ASSERT_TRUE(EVP_DigestSignFinal(md_ctx.get(),
                                                                  signature.data(),
                                                                  &sig_len)));
  ASSERT_STREQ(static_cast<const char*>(EC_KEY_get_ex_data(eckey.get(), 0)), "ecdsa_sign");

  ASSERT_LE(sig_len, signature.size());
  EXPECT_EQ(approved, AWSLC_NOT_APPROVED);

  // Test using the one-shot |EVP_DigestSign| function for approval. It should
  // not return an approval because custom sign functionality is defined.
  md_ctx.Reset();
  CALL_SERVICE_AND_CHECK_APPROVED(approved,
                                  ASSERT_TRUE(EVP_DigestSignInit(md_ctx.get(),
                                                                 nullptr,
                                                                 test.func(),
                                                                 nullptr,
                                                                 pkey.get())));
  EXPECT_EQ(approved, AWSLC_NOT_APPROVED);
  sig_len = max_sig_len;
  EC_KEY_set_ex_data(eckey.get(), 0, (void*) "");
  ASSERT_STREQ(static_cast<const char*>(EC_KEY_get_ex_data(eckey.get(), 0)), "");

  CALL_SERVICE_AND_CHECK_APPROVED(approved,
                                  ASSERT_TRUE(EVP_DigestSign(md_ctx.get(),
                                                             signature.data(),
                                                             &sig_len,
                                                             kPlaintext,
                                                             sizeof(kPlaintext))));
  ASSERT_STREQ(static_cast<const char*>(EC_KEY_get_ex_data(eckey.get(), 0)), "ecdsa_sign");

  ASSERT_LE(sig_len, signature.size());
  EXPECT_EQ(approved, AWSLC_NOT_APPROVED);
}

struct ECDHTestVector {
  // nid is the input curve nid.
  const int nid;
  // md_func is the digest to test.
  const int digest_length;
  // expected to be approved or not.
  const FIPSStatus expect_approved;
};
static const struct ECDHTestVector kECDHTestVectors[] = {
    // Only the following NIDs for |EC_GROUP| are creatable with
    // |EC_GROUP_new_by_curve_name|.
    // |ECDH_compute_key_fips| fails directly when an invalid hash length is
    // inputted.
    { NID_secp224r1, SHA224_DIGEST_LENGTH, AWSLC_APPROVED },
    { NID_secp224r1, SHA256_DIGEST_LENGTH, AWSLC_APPROVED },
    { NID_secp224r1, SHA384_DIGEST_LENGTH, AWSLC_APPROVED },
    { NID_secp224r1, SHA512_DIGEST_LENGTH, AWSLC_APPROVED },

    { NID_X9_62_prime256v1, SHA224_DIGEST_LENGTH, AWSLC_APPROVED },
    { NID_X9_62_prime256v1, SHA256_DIGEST_LENGTH, AWSLC_APPROVED },
    { NID_X9_62_prime256v1, SHA384_DIGEST_LENGTH, AWSLC_APPROVED },
    { NID_X9_62_prime256v1, SHA512_DIGEST_LENGTH, AWSLC_APPROVED },

    { NID_secp384r1, SHA224_DIGEST_LENGTH, AWSLC_APPROVED },
    { NID_secp384r1, SHA256_DIGEST_LENGTH, AWSLC_APPROVED },
    { NID_secp384r1, SHA384_DIGEST_LENGTH, AWSLC_APPROVED },
    { NID_secp384r1, SHA512_DIGEST_LENGTH, AWSLC_APPROVED },

    { NID_secp521r1, SHA224_DIGEST_LENGTH, AWSLC_APPROVED },
    { NID_secp521r1, SHA256_DIGEST_LENGTH, AWSLC_APPROVED },
    { NID_secp521r1, SHA384_DIGEST_LENGTH, AWSLC_APPROVED },
    { NID_secp521r1, SHA512_DIGEST_LENGTH, AWSLC_APPROVED },

    { NID_secp256k1, SHA224_DIGEST_LENGTH, AWSLC_NOT_APPROVED },
    { NID_secp256k1, SHA256_DIGEST_LENGTH, AWSLC_NOT_APPROVED },
    { NID_secp256k1, SHA384_DIGEST_LENGTH, AWSLC_NOT_APPROVED },
    { NID_secp256k1, SHA512_DIGEST_LENGTH, AWSLC_NOT_APPROVED },
};

class ECDH_ServiceIndicatorTest : public TestWithNoErrors<ECDHTestVector> {};

INSTANTIATE_TEST_SUITE_P(All, ECDH_ServiceIndicatorTest,
                         testing::ValuesIn(kECDHTestVectors));

TEST_P(ECDH_ServiceIndicatorTest, ECDH) {
  const ECDHTestVector &test = GetParam();
  if (test.nid == NID_secp256k1 && !kCurveSecp256k1Supported) {
    GTEST_SKIP();
  }

  FIPSStatus approved = AWSLC_NOT_APPROVED;

  const EC_GROUP *group = EC_GROUP_new_by_curve_name(test.nid);
  bssl::UniquePtr<EC_KEY> our_key(EC_KEY_new());
  bssl::UniquePtr<EC_KEY> peer_key(EC_KEY_new());
  bssl::ScopedEVP_MD_CTX md_ctx;
  ASSERT_TRUE(our_key);
  ASSERT_TRUE(peer_key);

  // Generate two generic ec key pairs.
  ASSERT_TRUE(EC_KEY_set_group(our_key.get(), group));
  ASSERT_TRUE(EC_KEY_generate_key(our_key.get()));
  ASSERT_TRUE(EC_KEY_check_key(our_key.get()));

  ASSERT_TRUE(EC_KEY_set_group(peer_key.get(), group));
  ASSERT_TRUE(EC_KEY_generate_key(peer_key.get()));
  ASSERT_TRUE(EC_KEY_check_key(peer_key.get()));

  // Test that |ECDH_compute_key_fips| has service indicator approval as
  // expected.
  std::vector<uint8_t> digest(test.digest_length);
  CALL_SERVICE_AND_CHECK_APPROVED(approved,
      ASSERT_TRUE(ECDH_compute_key_fips(digest.data(), digest.size(),
                                        EC_KEY_get0_public_key(peer_key.get()),
                                        our_key.get())));
  EXPECT_EQ(approved, test.expect_approved);

  // Test running the EVP_PKEY_derive interfaces one by one directly, and check
  // |EVP_PKEY_derive| for approval at the end. |EVP_PKEY_derive_init| and
  // |EVP_PKEY_derive_set_peer| should not be approved because they do not
  // indicate an entire service has been done.
  bssl::UniquePtr<EVP_PKEY> our_pkey(EVP_PKEY_new());
  ASSERT_TRUE(EVP_PKEY_set1_EC_KEY(our_pkey.get(), our_key.get()));
  bssl::UniquePtr<EVP_PKEY_CTX> our_ctx(
      EVP_PKEY_CTX_new(our_pkey.get(), nullptr));
  bssl::UniquePtr<EVP_PKEY> peer_pkey(EVP_PKEY_new());
  ASSERT_TRUE(EVP_PKEY_set1_EC_KEY(peer_pkey.get(), peer_key.get()));

  CALL_SERVICE_AND_CHECK_APPROVED(approved,
      ASSERT_TRUE(EVP_PKEY_derive_init(our_ctx.get())));
  EXPECT_EQ(approved, AWSLC_NOT_APPROVED);
  CALL_SERVICE_AND_CHECK_APPROVED(approved,
      ASSERT_TRUE(EVP_PKEY_derive_set_peer(our_ctx.get(), peer_pkey.get())));
  EXPECT_EQ(approved, AWSLC_NOT_APPROVED);
  // Determine the size of the output key. The first call of |EVP_PKEY_derive|
  // should not return an approval check because no crypto is being done when
  // |nullptr| is inputted in the |*key| field
  size_t out_len = 0;
  CALL_SERVICE_AND_CHECK_APPROVED(
      approved, ASSERT_TRUE(EVP_PKEY_derive(our_ctx.get(), nullptr, &out_len)));
  EXPECT_EQ(approved, AWSLC_NOT_APPROVED);
  std::vector<uint8_t> derive_output(out_len);
  CALL_SERVICE_AND_CHECK_APPROVED(approved,
      ASSERT_TRUE(EVP_PKEY_derive(our_ctx.get(), derive_output.data(),
                                  &out_len)));
  EXPECT_EQ(approved, kEVPDeriveSetsServiceIndicator
                          ? test.expect_approved
                          : AWSLC_NOT_APPROVED);
}

static const struct KDFTestVector {
  // func is the hash function for KDF to test.
  const EVP_MD *(*func)();
  const char *label;
  const size_t label_len;
  const uint8_t *expected_output;
  const FIPSStatus expect_approved;
} kKDFTestVectors[] = {
    {EVP_md5, kTLSLabel, sizeof(kTLSLabel), kTLSOutput_md, AWSLC_NOT_APPROVED},
    {EVP_sha1, kTLSLabel, sizeof(kTLSLabel), kTLSOutput_sha1,
     AWSLC_NOT_APPROVED},
    {EVP_md5_sha1, kTLSLabel, sizeof(kTLSLabel), kTLSOutput1_mdsha1,
     AWSLC_APPROVED},
    {EVP_md5_sha1, extendedMasterSecretLabel, sizeof(extendedMasterSecretLabel),
     kTLSOutput2_mdsha1, AWSLC_APPROVED},
    {EVP_sha224, kTLSLabel, sizeof(kTLSLabel), kTLSOutput_sha224,
     AWSLC_NOT_APPROVED},
    {EVP_sha256, kTLSLabel, sizeof(kTLSLabel), kTLSOutput1_sha256,
     AWSLC_NOT_APPROVED},
    {EVP_sha256, extendedMasterSecretLabel, sizeof(extendedMasterSecretLabel),
     kTLSOutput2_sha256, AWSLC_APPROVED},
    {EVP_sha384, kTLSLabel, sizeof(kTLSLabel), kTLSOutput1_sha384,
     AWSLC_NOT_APPROVED},
    {EVP_sha384, extendedMasterSecretLabel, sizeof(extendedMasterSecretLabel),
     kTLSOutput2_sha384, AWSLC_APPROVED},
    {EVP_sha512, kTLSLabel, sizeof(kTLSLabel), kTLSOutput1_sha512,
     AWSLC_NOT_APPROVED},
    {EVP_sha512, extendedMasterSecretLabel, sizeof(extendedMasterSecretLabel),
     kTLSOutput2_sha512, AWSLC_APPROVED}
};

class KDF_ServiceIndicatorTest : public TestWithNoErrors<KDFTestVector> {};

INSTANTIATE_TEST_SUITE_P(All, KDF_ServiceIndicatorTest,
                         testing::ValuesIn(kKDFTestVectors));

TEST_P(KDF_ServiceIndicatorTest, TLSKDF) {
  const KDFTestVector &test = GetParam();

  FIPSStatus approved = AWSLC_NOT_APPROVED;

  uint8_t output[32];
  CALL_SERVICE_AND_CHECK_APPROVED(
      approved, ASSERT_TRUE(CRYPTO_tls1_prf(test.func(), output, sizeof(output),
                                kTLSSecret, sizeof(kTLSSecret), test.label,
                                test.label_len, kTLSSeed1, sizeof(kTLSSeed1),
                                kTLSSeed2, sizeof(kTLSSeed2))));
  EXPECT_EQ(Bytes(test.expected_output, sizeof(output)),
            Bytes(output, sizeof(output)));
  EXPECT_EQ(approved, test.expect_approved);
}

// TLS 1.3 KDF (HKDF-Expand-Label) is approved under SHA2-256 and SHA2-384, and
// not approved for any other digest. Label / context contents do not affect
// approval state for TLS 1.3.
static const struct TLS13KDFTestVector {
  const EVP_MD *(*func)();
  const FIPSStatus expect_approved;
} kTLS13KDFTestVectors[] = {
    {EVP_sha1, AWSLC_NOT_APPROVED},
    {EVP_sha224, AWSLC_NOT_APPROVED},
    {EVP_sha256, AWSLC_APPROVED},
    {EVP_sha384, AWSLC_APPROVED},
    {EVP_sha512, AWSLC_NOT_APPROVED},
};

class TLS13KDF_ServiceIndicatorTest
    : public TestWithNoErrors<TLS13KDFTestVector> {};

INSTANTIATE_TEST_SUITE_P(All, TLS13KDF_ServiceIndicatorTest,
                         testing::ValuesIn(kTLS13KDFTestVectors));

TEST_P(TLS13KDF_ServiceIndicatorTest, HKDFExpandLabel) {
  const TLS13KDFTestVector &test = GetParam();
  const EVP_MD *digest = test.func();

  static const uint8_t kLabel[] = "c e traffic";
  // The HKDF-Expand-Label context is a transcript hash, so its length tracks
  // the digest in use (e.g. 48 bytes for SHA-384, 32 for SHA-256). Size it from
  // the digest rather than pinning it to 32 bytes so each parameterization
  // mirrors a real caller.
  uint8_t hash[EVP_MAX_MD_SIZE] = {0};
  const size_t hash_len = EVP_MD_size(digest);
  FIPSStatus approved = AWSLC_NOT_APPROVED;

  uint8_t output[32];
  CALL_SERVICE_AND_CHECK_APPROVED(
      approved,
      ASSERT_TRUE(CRYPTO_tls13_hkdf_expand_label(
          output, sizeof(output), digest, kTLSSecret, sizeof(kTLSSecret),
          kLabel, sizeof(kLabel) - 1, hash, hash_len)));
  EXPECT_EQ(approved, test.expect_approved);
}

static void CheckTLS13KDFRejectedNotApproved(
    size_t out_len, const uint8_t *label, size_t label_len,
    const uint8_t *hash, size_t hash_len, int expected_lib,
    int expected_reason) {
  uint8_t output[32] = {0};
  int result = 1;
  FIPSStatus approved = AWSLC_APPROVED;
  CALL_SERVICE_AND_CHECK_APPROVED(
      approved, result = CRYPTO_tls13_hkdf_expand_label(
                    output, out_len, EVP_sha256(), kTLSSecret,
                    sizeof(kTLSSecret), label, label_len, hash, hash_len));

  EXPECT_FALSE(result);
  EXPECT_EQ(approved, AWSLC_NOT_APPROVED);
  uint32_t err = ERR_get_error();
  EXPECT_TRUE(ErrorEquals(err, expected_lib, expected_reason))
      << ERR_error_string(err, nullptr);
  EXPECT_EQ(0u, ERR_get_error());
}

TEST(TLS13KDF_ServiceIndicatorNegativeTest, HKDFExpandLabelRejectsBounds) {
  static const uint8_t kLabel[] = "c e traffic";
  const size_t kOutputLen = 32;
  uint8_t hash[EVP_MAX_MD_SIZE] = {0};

  CheckTLS13KDFRejectedNotApproved(static_cast<size_t>(UINT16_MAX) + 1, kLabel,
                                   sizeof(kLabel) - 1, hash,
                                   EVP_MD_size(EVP_sha256()), ERR_LIB_CRYPTO,
                                   ERR_R_OVERFLOW);

  const std::vector<uint8_t> oversized_label(250);
  CheckTLS13KDFRejectedNotApproved(
      kOutputLen, oversized_label.data(), oversized_label.size(), hash,
      EVP_MD_size(EVP_sha256()), ERR_LIB_CRYPTO, ERR_R_OVERFLOW);

  const std::vector<uint8_t> oversized_hash(256);
  CheckTLS13KDFRejectedNotApproved(kOutputLen, kLabel, sizeof(kLabel) - 1,
                                   oversized_hash.data(),
                                   oversized_hash.size(), ERR_LIB_CRYPTO,
                                   ERR_R_OVERFLOW);

  const size_t hkdf_expand_limit = 255 * EVP_MD_size(EVP_sha256());
  std::vector<uint8_t> output(hkdf_expand_limit + 1);
  int result = 1;
  FIPSStatus approved = AWSLC_APPROVED;
  CALL_SERVICE_AND_CHECK_APPROVED(
      approved, result = CRYPTO_tls13_hkdf_expand_label(
                    output.data(), output.size(), EVP_sha256(), kTLSSecret,
                    sizeof(kTLSSecret), kLabel, sizeof(kLabel) - 1, hash,
                    EVP_MD_size(EVP_sha256())));

  EXPECT_FALSE(result);
  EXPECT_EQ(approved, AWSLC_NOT_APPROVED);
  uint32_t err = ERR_get_error();
  EXPECT_TRUE(ErrorEquals(err, ERR_LIB_HKDF, HKDF_R_OUTPUT_TOO_LARGE))
      << ERR_error_string(err, nullptr);
  EXPECT_EQ(0u, ERR_get_error());

  uint8_t approved_output[32];
  CALL_SERVICE_AND_CHECK_APPROVED(
      approved, ASSERT_TRUE(CRYPTO_tls13_hkdf_expand_label(
                    approved_output, sizeof(approved_output), EVP_sha256(),
                    kTLSSecret, sizeof(kTLSSecret), kLabel, sizeof(kLabel) - 1,
                    hash, EVP_MD_size(EVP_sha256()))));
  EXPECT_EQ(approved, AWSLC_APPROVED);
}

// PBKDF2 test data from RFC 6070.
//
// Set 1 - short password/salt; these are too short for FIPS, so they'll
//         return NOT_APPROVED
// Set 2 - long password/salt; APPROVED for FIPS
// Set 3 - Not included, it's another short password/salt test, to ensure the
//         password/salt are being handled as byte buffers rather than strings.
static const uint8_t kPBKDF2Password1[] = {
    'p', 'a', 's', 's', 'w', 'o', 'r', 'd'
};
static const uint8_t kPBKDF2Salt1[] = {'s', 'a', 'l', 't'};
static const uint8_t kPBKDF2Password2[] = {
    'p', 'a', 's', 's', 'w', 'o', 'r', 'd', 'P', 'A', 'S', 'S', 'W', 'O', 'R',
    'D', 'p', 'a', 's', 's', 'w', 'o', 'r', 'd'
};
static const uint8_t kPBKDF2Salt2[] = {
    's', 'a', 'l', 't', 'S', 'A', 'L', 'T', 's', 'a', 'l', 't', 'S', 'A', 'L',
    'T', 's', 'a', 'l', 't', 'S', 'A', 'L', 'T', 's', 'a', 'l', 't', 'S', 'A',
    'L', 'T', 's', 'a', 'l', 't'
};

static const uint8_t kPBKDF2DerivedKey1SHA1[] = {
    0x0c, 0x60, 0xc8, 0x0f, 0x96, 0x1f, 0x0e, 0x71, 0xf3, 0xa9, 0xb5, 0x24,
    0xaf, 0x60, 0x12, 0x06, 0x2f, 0xe0, 0x37, 0xa6  // 20 bytes
};
static const uint8_t kPBKDF2DerivedKey2SHA1[] = {
    0xea, 0x6c, 0x01, 0x4d, 0xc7, 0x2d, 0x6f, 0x8c, 0xcd, 0x1e, 0xd9, 0x2a,
    0xce, 0x1d, 0x41, 0xf0, 0xd8, 0xde, 0x89, 0x57  // 20 bytes
};
static const uint8_t kPBKDF2DerivedKey3SHA1[] = {
    0x4b, 0x00, 0x79, 0x01, 0xb7, 0x65, 0x48, 0x9a, 0xbe, 0xad, 0x49, 0xd9,
    0x26, 0xf7, 0x21, 0xd0, 0x65, 0xa4, 0x29, 0xc1  // 20 bytes
};
static const uint8_t kPBKDF2DerivedKey4SHA1[] = {
    0xee, 0xfe, 0x3d, 0x61, 0xcd, 0x4d, 0xa4, 0xe4, 0xe9, 0x94, 0x5b, 0x3d,
    0x6b, 0xa2, 0x15, 0x8c, 0x26, 0x34, 0xe9, 0x84  // 20 bytes
};
static const uint8_t kPBKDF2DerivedKey5SHA1[] = {
    0x3d, 0x2e, 0xec, 0x4f, 0xe4, 0x1c, 0x84, 0x9b, 0x80, 0xc8, 0xd8, 0x36,
    0x62, 0xc0, 0xe4, 0x4a, 0x8b, 0x29, 0x1a, 0x96, 0x4c, 0xf2, 0xf0, 0x70,
    0x38    // 25 bytes
};
static const uint8_t kPBKDF2DerivedKey6SHA1[] = {
    0xac, 0xf8, 0xb4, 0x67, 0x41, 0xc7, 0xf3, 0xd1, 0xa0, 0xc0, 0x08, 0xbe,
    0x9b, 0x23, 0x96, 0x78, 0xbd, 0x93, 0xda, 0x4a, 0x30, 0xd4, 0xfb, 0xf0,
    0x33    // 25 bytes
};

static const uint8_t kPBKDF2DerivedKey1SHA224[] = {
    0x3c, 0x19, 0x8c, 0xbd, 0xb9, 0x46, 0x4b, 0x78, 0x57, 0x96, 0x6b, 0xd0,
    0x5b, 0x7b, 0xc9, 0x2b, 0xc1, 0xcc, 0x4e, 0x6e  // 20 bytes
};
static const uint8_t kPBKDF2DerivedKey2SHA224[] = {
    0x93, 0x20, 0x0f, 0xfa, 0x96, 0xc5, 0x77, 0x6d, 0x38, 0xfa, 0x10, 0xab,
    0xdf, 0x8f, 0x5b, 0xfc, 0x00, 0x54, 0xb9, 0x71  // 20 bytes
};
static const uint8_t kPBKDF2DerivedKey3SHA224[] = {
    0x21, 0x8c, 0x45, 0x3b, 0xf9, 0x06, 0x35, 0xbd, 0x0a, 0x21, 0xa7, 0x5d,
    0x17, 0x27, 0x03, 0xff, 0x61, 0x08, 0xef, 0x60  // 20 bytes
};
static const uint8_t kPBKDF2DerivedKey4SHA224[] = {
    0xb4, 0x99, 0x25, 0x18, 0x4c, 0xb4, 0xb5, 0x59, 0xf3, 0x65, 0xe9, 0x4f,
    0xca, 0xfc, 0xd4, 0xcd, 0xb9, 0xf7, 0xae, 0xf4  // 20 bytes
};
static const uint8_t kPBKDF2DerivedKey5SHA224[] = {
    0x05, 0x6c, 0x4b, 0xa4, 0x38, 0xde, 0xd9, 0x1f, 0xc1, 0x4e, 0x05, 0x94,
    0xe6, 0xf5, 0x2b, 0x87, 0xe1, 0xf3, 0x69, 0x0c, 0x0d, 0xc0, 0xfb, 0xc0,
    0x57    // 25 bytes
};
static const uint8_t kPBKDF2DerivedKey6SHA224[] = {
    0x0f, 0x51, 0xe7, 0x77, 0x07, 0x88, 0x5e, 0x09, 0x20, 0xd7, 0x46, 0x6c,
    0x8f, 0xdf, 0xd6, 0x07, 0x38, 0x31, 0xde, 0xfe, 0x01, 0x29, 0x22, 0xbf,
    0x47    // 25 bytes
};

static const uint8_t kPBKDF2DerivedKey1SHA256[] = {
    0x12, 0x0f, 0xb6, 0xcf, 0xfc, 0xf8, 0xb3, 0x2c, 0x43, 0xe7, 0x22, 0x52,
    0x56, 0xc4, 0xf8, 0x37, 0xa8, 0x65, 0x48, 0xc9  // 20 bytes
};
static const uint8_t kPBKDF2DerivedKey2SHA256[] = {
    0xae, 0x4d, 0x0c, 0x95, 0xaf, 0x6b, 0x46, 0xd3, 0x2d, 0x0a, 0xdf, 0xf9,
    0x28, 0xf0, 0x6d, 0xd0, 0x2a, 0x30, 0x3f, 0x8e  // 20 bytes
};
static const uint8_t kPBKDF2DerivedKey3SHA256[] = {
    0xc5, 0xe4, 0x78, 0xd5, 0x92, 0x88, 0xc8, 0x41, 0xaa, 0x53, 0x0d, 0xb6,
    0x84, 0x5c, 0x4c, 0x8d, 0x96, 0x28, 0x93, 0xa0  // 20 bytes
};
static const uint8_t kPBKDF2DerivedKey4SHA256[] = {
    0xcf, 0x81, 0xc6, 0x6f, 0xe8, 0xcf, 0xc0, 0x4d, 0x1f, 0x31, 0xec, 0xb6,
    0x5d, 0xab, 0x40, 0x89, 0xf7, 0xf1, 0x79, 0xe8  // 20 bytes
};
static const uint8_t kPBKDF2DerivedKey5SHA256[] = {
    0x34, 0x8c, 0x89, 0xdb, 0xcb, 0xd3, 0x2b, 0x2f, 0x32, 0xd8, 0x14, 0xb8,
    0x11, 0x6e, 0x84, 0xcf, 0x2b, 0x17, 0x34, 0x7e, 0xbc, 0x18, 0x00, 0x18,
    0x1c    // 25 bytes
};
static const uint8_t kPBKDF2DerivedKey6SHA256[] = {
    0x09, 0x3e, 0x1a, 0xd8, 0x63, 0x30, 0x71, 0x9c, 0x17, 0xcf, 0xb0, 0x53,
    0x3e, 0x1f, 0xc8, 0x51, 0x29, 0x71, 0x54, 0x28, 0x5d, 0xf7, 0x8e, 0x41,
    0xaa    // 25 bytes
};

static const uint8_t kPBKDF2DerivedKey1SHA384[] = {
    0xc0, 0xe1, 0x4f, 0x06, 0xe4, 0x9e, 0x32, 0xd7, 0x3f, 0x9f, 0x52, 0xdd,
    0xf1, 0xd0, 0xc5, 0xc7, 0x19, 0x16, 0x09, 0x23  // 20 bytes
};
static const uint8_t kPBKDF2DerivedKey2SHA384[] = {
    0x54, 0xf7, 0x75, 0xc6, 0xd7, 0x90, 0xf2, 0x19, 0x30, 0x45, 0x91, 0x62,
    0xfc, 0x53, 0x5d, 0xbf, 0x04, 0xa9, 0x39, 0x18  // 20 bytes
};
static const uint8_t kPBKDF2DerivedKey3SHA384[] = {
    0x55, 0x97, 0x26, 0xbe, 0x38, 0xdb, 0x12, 0x5b, 0xc8, 0x5e, 0xd7, 0x89,
    0x5f, 0x6e, 0x3c, 0xf5, 0x74, 0xc7, 0xa0, 0x1c  // 20 bytes
};
static const uint8_t kPBKDF2DerivedKey4SHA384[] = {
    0xa7, 0xfd, 0xb3, 0x49, 0xba, 0x2b, 0xfa, 0x6b, 0xf6, 0x47, 0xbb, 0x01,
    0x61, 0xba, 0xe1, 0x32, 0x0d, 0xf2, 0x7e, 0x64  // 20 bytes
};
static const uint8_t kPBKDF2DerivedKey5SHA384[] = {
    0x81, 0x91, 0x43, 0xad, 0x66, 0xdf, 0x9a, 0x55, 0x25, 0x59, 0xb9, 0xe1,
    0x31, 0xc5, 0x2a, 0xe6, 0xc5, 0xc1, 0xb0, 0xee, 0xd1, 0x8f, 0x4d, 0x28,
    0x3b    // 25 bytes
};
static const uint8_t kPBKDF2DerivedKey6SHA384[] = {
    0xd6, 0xb7, 0x36, 0x38, 0xe3, 0x59, 0xee, 0x39, 0xae, 0x1b, 0x5c, 0x24,
    0xb2, 0x5c, 0x56, 0x14, 0x5b, 0x57, 0xb1, 0x75, 0xdc, 0x6f, 0x75, 0xb8,
    0x12    // 25 bytes
};

static const uint8_t kPBKDF2DerivedKey1SHA512[] = {
    0x86, 0x7f, 0x70, 0xcf, 0x1a, 0xde, 0x02, 0xcf, 0xf3, 0x75, 0x25, 0x99,
    0xa3, 0xa5, 0x3d, 0xc4, 0xaf, 0x34, 0xc7, 0xa6  // 20 bytes
};
static const uint8_t kPBKDF2DerivedKey2SHA512[] = {
    0xe1, 0xd9, 0xc1, 0x6a, 0xa6, 0x81, 0x70, 0x8a, 0x45, 0xf5, 0xc7, 0xc4,
    0xe2, 0x15, 0xce, 0xb6, 0x6e, 0x01, 0x1a, 0x2e  // 20 bytes
};
static const uint8_t kPBKDF2DerivedKey3SHA512[] = {
    0xd1, 0x97, 0xb1, 0xb3, 0x3d, 0xb0, 0x14, 0x3e, 0x01, 0x8b, 0x12, 0xf3,
    0xd1, 0xd1, 0x47, 0x9e, 0x6c, 0xde, 0xbd, 0xcc  // 20 bytes
};
static const uint8_t kPBKDF2DerivedKey4SHA512[] = {
    0x61, 0x80, 0xa3, 0xce, 0xab, 0xab, 0x45, 0xcc, 0x39, 0x64, 0x11, 0x2c,
    0x81, 0x1e, 0x01, 0x31, 0xbc, 0xa9, 0x3a, 0x35  // 20 bytes
};
static const uint8_t kPBKDF2DerivedKey5SHA512[] = {
    0x8c, 0x05, 0x11, 0xf4, 0xc6, 0xe5, 0x97, 0xc6, 0xac, 0x63, 0x15, 0xd8,
    0xf0, 0x36, 0x2e, 0x22, 0x5f, 0x3c, 0x50, 0x14, 0x95, 0xba, 0x23, 0xb8,
    0x68    // 25 bytes
};
static const uint8_t kPBKDF2DerivedKey6SHA512[] = {
    0x14, 0xe8, 0xb0, 0x63, 0x43, 0xf9, 0x04, 0xc6, 0xa8, 0x55, 0xcb, 0xe0,
    0x7b, 0xaf, 0xe6, 0xf8, 0xac, 0x13, 0x8f, 0xcb, 0x91, 0x2d, 0xbd, 0x33,
    0x49   // 25 bytes
};

static const uint8_t kPBKDF2DerivedKey1SHA512_224[] = {
    0xb3, 0x4a, 0xb6, 0x26, 0x27, 0x6a, 0x61, 0xce, 0x19, 0xd2,
    0xec, 0xb4, 0xc7, 0xe1, 0x5f, 0x81, 0x98, 0xa2, 0x98, 0x9a  // 20 bytes
};

static const uint8_t kPBKDF2DerivedKey2SHA512_224[] = {
    0xb8, 0x87, 0x8a, 0xc5, 0xe4, 0x50, 0x9c, 0x16, 0x5c, 0x1b,
    0x50, 0x89, 0x61, 0xfa, 0x3c, 0x3a, 0xfc, 0xef, 0x3f, 0x37  // 20 bytes
};

static const uint8_t kPBKDF2DerivedKey3SHA512_224[] = {
    0xed, 0x54, 0xaf, 0x69, 0x9c, 0xc3, 0x07, 0xe0, 0x89, 0x65,
    0x09, 0x8b, 0xda, 0x5f, 0xf4, 0xe4, 0x1e, 0xa1, 0x93, 0x1f  // 20 bytes
};

static const uint8_t kPBKDF2DerivedKey4SHA512_224[] = {
    0xca, 0x62, 0x62, 0xfa, 0x52, 0xea, 0xd9, 0x88, 0x78, 0x86,
    0x7e, 0x13, 0x75, 0xd0, 0xa5, 0x8a, 0x8a, 0xe0, 0x4d, 0x00  // 20 bytes
};

static const uint8_t kPBKDF2DerivedKey5SHA512_224[] = {
    0x57, 0x3d, 0xf9, 0x67, 0x62, 0xea, 0x7d, 0xa4, 0xf7,
    0x12, 0x31, 0x85, 0x9c, 0xa2, 0x82, 0xef, 0x48, 0x27,
    0x64, 0xad, 0x96, 0x71, 0xc5, 0x27, 0x5c  // 25 bytes
};

static const uint8_t kPBKDF2DerivedKey6SHA512_224[] = {
    0xcb, 0x42, 0xca, 0xcd, 0x9a, 0xe7, 0xc6, 0x27, 0xb3, 0xa5, 0x04, 0x86,
    0x75, 0xbc, 0xed, 0xd9, 0xdd, 0xf8, 0xa5, 0xfd, 0xfe, 0x7a, 0x6a, 0xc6,
    0xcc  // 25 bytes
};

static const uint8_t kPBKDF2DerivedKey1SHA512_256[] = {
    0x4b, 0x6a, 0x63, 0x11, 0x7d, 0x3e, 0xc0, 0x03, 0x26, 0x24, 0x61,
    0x60, 0x82, 0xc1, 0xc1, 0x91, 0x2f, 0x56, 0xfa, 0x5f // 20 bytes

};

static const uint8_t kPBKDF2DerivedKey2SHA512_256[] = {
    0xfc, 0xfd, 0x10, 0x8c, 0x99, 0xcc, 0x88, 0x8e, 0xc0, 0xaf,
    0x9f, 0x18, 0x48, 0x85, 0xaf, 0xf5, 0xf0, 0x2d, 0x19, 0xa9  // 20 bytes
};

static const uint8_t kPBKDF2DerivedKey3SHA512_256[] = {
    0xf2, 0xfb, 0xe5, 0xf8, 0xec, 0x36, 0x18, 0xbb, 0x14, 0x52,
    0x79, 0xa8, 0xc6, 0xa8, 0xdf, 0xa4, 0x76, 0xc2, 0x82, 0xa3  // 20 bytes
};

static const uint8_t kPBKDF2DerivedKey4SHA512_256[] = {
    0x25, 0xf5, 0xd3, 0x8f, 0x4b, 0x39, 0xd9, 0x2b, 0x43, 0x6e,
    0x52, 0xf8, 0xa9, 0x2f, 0xb6, 0x9d, 0xf7, 0x6f, 0xb0, 0x14  // 20 bytes
};

static const uint8_t kPBKDF2DerivedKey5SHA512_256[] = {
    0x31, 0xcf, 0x94, 0xe3, 0xd8, 0xe3, 0x6a, 0xa1, 0x8d,
    0x40, 0xad, 0x92, 0x65, 0x4a, 0xb8, 0x0f, 0x50, 0x0e,
    0xd7, 0xfb, 0x57, 0x5a, 0x22, 0x15, 0x54  // 25 bytes
};

static const uint8_t kPBKDF2DerivedKey6SHA512_256[] = {
    0x4d, 0x68, 0xef, 0xc6, 0x80, 0xd2, 0x30, 0x5d, 0x23,
    0x44, 0x9c, 0x92, 0xc4, 0x3b, 0x5b, 0xb7, 0x7f, 0x75,
    0x03, 0x4d, 0x95, 0xc5, 0x48, 0xaa, 0x44 // 25 bytes
};

static const struct PBKDF2TestVector {
    // func is the hash function for PBKDF2 to test.
    const EVP_MD *(*func)();
    const uint8_t *password;
    const size_t password_len;
    const uint8_t *salt;
    const size_t salt_len;
    const unsigned iterations;
    const size_t output_len;
    const uint8_t *expected_output;
    const FIPSStatus expect_approved;
} kPBKDF2TestVectors[] = {
    // SHA1 outputs
    {
        EVP_sha1,
        kPBKDF2Password1, sizeof(kPBKDF2Password1),
        kPBKDF2Salt1, sizeof(kPBKDF2Salt1),
        1,
        sizeof(kPBKDF2DerivedKey1SHA1), kPBKDF2DerivedKey1SHA1,
        AWSLC_NOT_APPROVED
    },
    {
        EVP_sha1,
        kPBKDF2Password1, sizeof(kPBKDF2Password1),
        kPBKDF2Salt1, sizeof(kPBKDF2Salt1),
        2,
        sizeof(kPBKDF2DerivedKey2SHA1), kPBKDF2DerivedKey2SHA1,
        AWSLC_NOT_APPROVED
    },
    {
        EVP_sha1,
        kPBKDF2Password1, sizeof(kPBKDF2Password1),
        kPBKDF2Salt1, sizeof(kPBKDF2Salt1),
        4096,
        sizeof(kPBKDF2DerivedKey3SHA1), kPBKDF2DerivedKey3SHA1,
        AWSLC_NOT_APPROVED
    },
    {
        EVP_sha1,
        kPBKDF2Password1, sizeof(kPBKDF2Password1),
        kPBKDF2Salt1, sizeof(kPBKDF2Salt1),
        16777216,
        sizeof(kPBKDF2DerivedKey4SHA1), kPBKDF2DerivedKey4SHA1,
        AWSLC_NOT_APPROVED
    },
    {
        EVP_sha1,
        kPBKDF2Password2, sizeof(kPBKDF2Password2),
        kPBKDF2Salt2, sizeof(kPBKDF2Salt2),
        4096,
        sizeof(kPBKDF2DerivedKey5SHA1), kPBKDF2DerivedKey5SHA1,
        AWSLC_APPROVED
    },
    {
        EVP_sha1,
        kPBKDF2Password2, sizeof(kPBKDF2Password2),
        kPBKDF2Salt2, sizeof(kPBKDF2Salt2),
        999,
        sizeof(kPBKDF2DerivedKey6SHA1), kPBKDF2DerivedKey6SHA1,
        AWSLC_NOT_APPROVED
    },

    // SHA224 outputs from
    // https://github.com/brycx/Test-Vector-Generation/pull/1
    {
        EVP_sha224,
        kPBKDF2Password1, sizeof(kPBKDF2Password1),
        kPBKDF2Salt1, sizeof(kPBKDF2Salt1),
        1,
        sizeof(kPBKDF2DerivedKey1SHA224), kPBKDF2DerivedKey1SHA224,
        AWSLC_NOT_APPROVED
    },
    {
        EVP_sha224,
        kPBKDF2Password1, sizeof(kPBKDF2Password1),
        kPBKDF2Salt1, sizeof(kPBKDF2Salt1),
        2,
        sizeof(kPBKDF2DerivedKey2SHA224), kPBKDF2DerivedKey2SHA224,
        AWSLC_NOT_APPROVED
    },
    {
        EVP_sha224,
        kPBKDF2Password1, sizeof(kPBKDF2Password1),
        kPBKDF2Salt1, sizeof(kPBKDF2Salt1),
        4096,
        sizeof(kPBKDF2DerivedKey3SHA224), kPBKDF2DerivedKey3SHA224,
        AWSLC_NOT_APPROVED
    },
    {
        EVP_sha224,
        kPBKDF2Password1, sizeof(kPBKDF2Password1),
        kPBKDF2Salt1, sizeof(kPBKDF2Salt1),
        16777216,
        sizeof(kPBKDF2DerivedKey4SHA224), kPBKDF2DerivedKey4SHA224,
        AWSLC_NOT_APPROVED
    },
    {
        EVP_sha224,
        kPBKDF2Password2, sizeof(kPBKDF2Password2),
        kPBKDF2Salt2, sizeof(kPBKDF2Salt2),
        4096,
        sizeof(kPBKDF2DerivedKey5SHA224), kPBKDF2DerivedKey5SHA224,
        AWSLC_APPROVED
    },
    {
        EVP_sha224,
        kPBKDF2Password2, sizeof(kPBKDF2Password2),
        kPBKDF2Salt2, sizeof(kPBKDF2Salt2),
        999,
        sizeof(kPBKDF2DerivedKey6SHA224), kPBKDF2DerivedKey6SHA224,
        AWSLC_NOT_APPROVED
    },

    // SHA256 outputs from
    // https://github.com/brycx/Test-Vector-Generation/blob/master/PBKDF2/pbkdf2-hmac-sha2-test-vectors.md
    {
        EVP_sha256,
        kPBKDF2Password1, sizeof(kPBKDF2Password1),
        kPBKDF2Salt1, sizeof(kPBKDF2Salt1),
        1,
        sizeof(kPBKDF2DerivedKey1SHA256), kPBKDF2DerivedKey1SHA256,
        AWSLC_NOT_APPROVED
    },
    {
        EVP_sha256,
        kPBKDF2Password1, sizeof(kPBKDF2Password1),
        kPBKDF2Salt1, sizeof(kPBKDF2Salt1),
        2,
        sizeof(kPBKDF2DerivedKey2SHA256), kPBKDF2DerivedKey2SHA256,
        AWSLC_NOT_APPROVED
    },
    {
        EVP_sha256,
        kPBKDF2Password1, sizeof(kPBKDF2Password1),
        kPBKDF2Salt1, sizeof(kPBKDF2Salt1),
        4096,
        sizeof(kPBKDF2DerivedKey3SHA256), kPBKDF2DerivedKey3SHA256,
        AWSLC_NOT_APPROVED
    },
    {
        EVP_sha256,
        kPBKDF2Password1, sizeof(kPBKDF2Password1),
        kPBKDF2Salt1, sizeof(kPBKDF2Salt1),
        16777216,
        sizeof(kPBKDF2DerivedKey4SHA256), kPBKDF2DerivedKey4SHA256,
        AWSLC_NOT_APPROVED
    },
    {
        EVP_sha256,
        kPBKDF2Password2, sizeof(kPBKDF2Password2),
        kPBKDF2Salt2, sizeof(kPBKDF2Salt2),
        4096,
        sizeof(kPBKDF2DerivedKey5SHA256), kPBKDF2DerivedKey5SHA256,
        AWSLC_APPROVED
    },
    {
        EVP_sha256,
        kPBKDF2Password2, sizeof(kPBKDF2Password2),
        kPBKDF2Salt2, sizeof(kPBKDF2Salt2),
        999,
        sizeof(kPBKDF2DerivedKey6SHA256), kPBKDF2DerivedKey6SHA256,
        AWSLC_NOT_APPROVED
    },

    // SHA384 outputs from
    // https://github.com/brycx/Test-Vector-Generation/blob/master/PBKDF2/pbkdf2-hmac-sha2-test-vectors.md
    {
        EVP_sha384,
        kPBKDF2Password1, sizeof(kPBKDF2Password1),
        kPBKDF2Salt1, sizeof(kPBKDF2Salt1),
        1,
        sizeof(kPBKDF2DerivedKey1SHA384), kPBKDF2DerivedKey1SHA384,
        AWSLC_NOT_APPROVED
    },
    {
        EVP_sha384,
        kPBKDF2Password1, sizeof(kPBKDF2Password1),
        kPBKDF2Salt1, sizeof(kPBKDF2Salt1),
        2,
        sizeof(kPBKDF2DerivedKey2SHA384), kPBKDF2DerivedKey2SHA384,
        AWSLC_NOT_APPROVED
    },
    {
        EVP_sha384,
        kPBKDF2Password1, sizeof(kPBKDF2Password1),
        kPBKDF2Salt1, sizeof(kPBKDF2Salt1),
        4096,
        sizeof(kPBKDF2DerivedKey3SHA384), kPBKDF2DerivedKey3SHA384,
        AWSLC_NOT_APPROVED
    },
    {
        EVP_sha384,
        kPBKDF2Password1, sizeof(kPBKDF2Password1),
        kPBKDF2Salt1, sizeof(kPBKDF2Salt1),
        16777216,
        sizeof(kPBKDF2DerivedKey4SHA384), kPBKDF2DerivedKey4SHA384,
        AWSLC_NOT_APPROVED
    },
    {
        EVP_sha384,
        kPBKDF2Password2, sizeof(kPBKDF2Password2),
        kPBKDF2Salt2, sizeof(kPBKDF2Salt2),
        4096,
        sizeof(kPBKDF2DerivedKey5SHA384), kPBKDF2DerivedKey5SHA384,
        AWSLC_APPROVED
    },
    {
        EVP_sha384,
        kPBKDF2Password2, sizeof(kPBKDF2Password2),
        kPBKDF2Salt2, sizeof(kPBKDF2Salt2),
        999,
        sizeof(kPBKDF2DerivedKey6SHA384), kPBKDF2DerivedKey6SHA384,
        AWSLC_NOT_APPROVED
    },

    // SHA512 outputs from
    // https://github.com/brycx/Test-Vector-Generation/blob/master/PBKDF2/pbkdf2-hmac-sha2-test-vectors.md
    {
        EVP_sha512,
        kPBKDF2Password1, sizeof(kPBKDF2Password1),
        kPBKDF2Salt1, sizeof(kPBKDF2Salt1),
        1,
        sizeof(kPBKDF2DerivedKey1SHA512), kPBKDF2DerivedKey1SHA512,
        AWSLC_NOT_APPROVED
    },
    {
        EVP_sha512,
        kPBKDF2Password1, sizeof(kPBKDF2Password1),
        kPBKDF2Salt1, sizeof(kPBKDF2Salt1),
        2,
        sizeof(kPBKDF2DerivedKey2SHA512), kPBKDF2DerivedKey2SHA512,
        AWSLC_NOT_APPROVED
    },
    {
        EVP_sha512,
        kPBKDF2Password1, sizeof(kPBKDF2Password1),
        kPBKDF2Salt1, sizeof(kPBKDF2Salt1),
        4096,
        sizeof(kPBKDF2DerivedKey3SHA512), kPBKDF2DerivedKey3SHA512,
        AWSLC_NOT_APPROVED
    },
    {
        EVP_sha512,
        kPBKDF2Password1, sizeof(kPBKDF2Password1),
        kPBKDF2Salt1, sizeof(kPBKDF2Salt1),
        16777216,
        sizeof(kPBKDF2DerivedKey4SHA512), kPBKDF2DerivedKey4SHA512,
        AWSLC_NOT_APPROVED
    },
    {
        EVP_sha512,
        kPBKDF2Password2, sizeof(kPBKDF2Password2),
        kPBKDF2Salt2, sizeof(kPBKDF2Salt2),
        4096,
        sizeof(kPBKDF2DerivedKey5SHA512), kPBKDF2DerivedKey5SHA512,
        AWSLC_APPROVED
    },
    {
        EVP_sha512,
        kPBKDF2Password2, sizeof(kPBKDF2Password2),
        kPBKDF2Salt2, sizeof(kPBKDF2Salt2),
        999,
        sizeof(kPBKDF2DerivedKey6SHA512), kPBKDF2DerivedKey6SHA512,
        AWSLC_NOT_APPROVED
    },

    // SHA512_224 using
    // https://github.com/brycx/Test-Vector-Generation/blob/master/PBKDF2/pbkdf2-hmac-sha2-test-vectors.md
    {
        EVP_sha512_224,
        kPBKDF2Password1, sizeof(kPBKDF2Password1),
        kPBKDF2Salt1, sizeof(kPBKDF2Salt1),
        1,
        sizeof(kPBKDF2DerivedKey1SHA512_224), kPBKDF2DerivedKey1SHA512_224,
        AWSLC_NOT_APPROVED
    },
    {
        EVP_sha512_224,
        kPBKDF2Password1, sizeof(kPBKDF2Password1),
        kPBKDF2Salt1, sizeof(kPBKDF2Salt1),
        2,
        sizeof(kPBKDF2DerivedKey2SHA512_224), kPBKDF2DerivedKey2SHA512_224,
        AWSLC_NOT_APPROVED
    },
    {
        EVP_sha512_224,
        kPBKDF2Password1, sizeof(kPBKDF2Password1),
        kPBKDF2Salt1, sizeof(kPBKDF2Salt1),
        4096,
        sizeof(kPBKDF2DerivedKey3SHA512_224), kPBKDF2DerivedKey3SHA512_224,
        AWSLC_NOT_APPROVED
    },
    {
        EVP_sha512_224,
        kPBKDF2Password1, sizeof(kPBKDF2Password1),
        kPBKDF2Salt1, sizeof(kPBKDF2Salt1),
        16777216,
        sizeof(kPBKDF2DerivedKey4SHA512_224), kPBKDF2DerivedKey4SHA512_224,
        AWSLC_NOT_APPROVED
    },
    {
        EVP_sha512_224,
        kPBKDF2Password2, sizeof(kPBKDF2Password2),
        kPBKDF2Salt2, sizeof(kPBKDF2Salt2),
        4096,
        sizeof(kPBKDF2DerivedKey5SHA512_224), kPBKDF2DerivedKey5SHA512_224,
        AWSLC_APPROVED
    },
    {
        EVP_sha512_224,
        kPBKDF2Password2, sizeof(kPBKDF2Password2),
        kPBKDF2Salt2, sizeof(kPBKDF2Salt2),
        999,
        sizeof(kPBKDF2DerivedKey6SHA512_224), kPBKDF2DerivedKey6SHA512_224,
        AWSLC_NOT_APPROVED
    },

    // SHA512_256 using
    // https://github.com/brycx/Test-Vector-Generation/blob/master/PBKDF2/pbkdf2-hmac-sha2-test-vectors.md
    {
        EVP_sha512_256,
        kPBKDF2Password1, sizeof(kPBKDF2Password1),
        kPBKDF2Salt1, sizeof(kPBKDF2Salt1),
        1,
        sizeof(kPBKDF2DerivedKey1SHA512_256), kPBKDF2DerivedKey1SHA512_256,
        AWSLC_NOT_APPROVED
    },
    {
        EVP_sha512_256,
        kPBKDF2Password1, sizeof(kPBKDF2Password1),
        kPBKDF2Salt1, sizeof(kPBKDF2Salt1),
        2,
        sizeof(kPBKDF2DerivedKey2SHA512_256), kPBKDF2DerivedKey2SHA512_256,
        AWSLC_NOT_APPROVED
    },
    {
        EVP_sha512_256,
        kPBKDF2Password1, sizeof(kPBKDF2Password1),
        kPBKDF2Salt1, sizeof(kPBKDF2Salt1),
        4096,
        sizeof(kPBKDF2DerivedKey3SHA512_256), kPBKDF2DerivedKey3SHA512_256,
        AWSLC_NOT_APPROVED
    },
    {
        EVP_sha512_256,
        kPBKDF2Password1, sizeof(kPBKDF2Password1),
        kPBKDF2Salt1, sizeof(kPBKDF2Salt1),
        16777216,
        sizeof(kPBKDF2DerivedKey4SHA512_256), kPBKDF2DerivedKey4SHA512_256,
        AWSLC_NOT_APPROVED
    },
    {
        EVP_sha512_256,
        kPBKDF2Password2, sizeof(kPBKDF2Password2),
        kPBKDF2Salt2, sizeof(kPBKDF2Salt2),
        4096,
        sizeof(kPBKDF2DerivedKey5SHA512_256), kPBKDF2DerivedKey5SHA512_256,
        AWSLC_APPROVED
    },
    {
        EVP_sha512_256,
        kPBKDF2Password2, sizeof(kPBKDF2Password2),
        kPBKDF2Salt2, sizeof(kPBKDF2Salt2),
        999,
        sizeof(kPBKDF2DerivedKey6SHA512_256), kPBKDF2DerivedKey6SHA512_256,
        AWSLC_NOT_APPROVED
    },
};

class PBKDF2_ServiceIndicatorTest : public TestWithNoErrors<PBKDF2TestVector> {
};

INSTANTIATE_TEST_SUITE_P(All, PBKDF2_ServiceIndicatorTest,
                         testing::ValuesIn(kPBKDF2TestVectors));

TEST_P(PBKDF2_ServiceIndicatorTest, PBKDF2) {
  const PBKDF2TestVector &test = GetParam();

  FIPSStatus approved = AWSLC_NOT_APPROVED;

  uint8_t output[sizeof(kPBKDF2DerivedKey5SHA1)];   // largest test vector output size
  CALL_SERVICE_AND_CHECK_APPROVED(
      approved, ASSERT_TRUE(PKCS5_PBKDF2_HMAC((const char *)test.password,
                                              test.password_len,
                                              test.salt, test.salt_len,
                                              test.iterations,
                                              test.func(), test.output_len,
                                              output)));
  EXPECT_EQ(Bytes(test.expected_output, test.output_len),
            Bytes(output, test.output_len));
  EXPECT_EQ(approved, test.expect_approved);
}

// SSHKDF test vector from NIST CAVS 14.1 test vectors
// https://csrc.nist.gov/Projects/Cryptographic-Algorithm-Validation-Program/Component-Testing#KDF135
static const char kSSHKDFtype = EVP_KDF_SSHKDF_TYPE_INITIAL_IV_CLI_TO_SRV;

const uint8_t kSSHKDFkeySHA1[] = {
    0x00, 0x00, 0x00, 0x80, 0x55, 0xba, 0xe9, 0x31, 0xc0, 0x7f, 0xd8, 0x24,
    0xbf, 0x10, 0xad, 0xd1, 0x90, 0x2b, 0x6f, 0xbc, 0x7c, 0x66, 0x53, 0x47,
    0x38, 0x34, 0x98, 0xa6, 0x86, 0x92, 0x9f, 0xf5, 0xa2, 0x5f, 0x8e, 0x40,
    0xcb, 0x66, 0x45, 0xea, 0x81, 0x4f, 0xb1, 0xa5, 0xe0, 0xa1, 0x1f, 0x85,
    0x2f, 0x86, 0x25, 0x56, 0x41, 0xe5, 0xed, 0x98, 0x6e, 0x83, 0xa7, 0x8b,
    0xc8, 0x26, 0x94, 0x80, 0xea, 0xc0, 0xb0, 0xdf, 0xd7, 0x70, 0xca, 0xb9,
    0x2e, 0x7a, 0x28, 0xdd, 0x87, 0xff, 0x45, 0x24, 0x66, 0xd6, 0xae, 0x86,
    0x7c, 0xea, 0xd6, 0x3b, 0x36, 0x6b, 0x1c, 0x28, 0x6e, 0x6c, 0x48, 0x11,
    0xa9, 0xf1, 0x4c, 0x27, 0xae, 0xa1, 0x4c, 0x51, 0x71, 0xd4, 0x9b, 0x78,
    0xc0, 0x6e, 0x37, 0x35, 0xd3, 0x6e, 0x6a, 0x3b, 0xe3, 0x21, 0xdd, 0x5f,
    0xc8, 0x23, 0x08, 0xf3, 0x4e, 0xe1, 0xcb, 0x17, 0xfb, 0xa9, 0x4a, 0x59
};
const uint8_t kSSHKDFxcghashSHA1[] = {
    0xa4, 0xeb, 0xd4, 0x59, 0x34, 0xf5, 0x67, 0x92, 0xb5, 0x11, 0x2d, 0xcd,
    0x75, 0xa1, 0x07, 0x5f, 0xdc, 0x88, 0x92, 0x45
};
const uint8_t kSSHKDFsessionSHA1[] = {
    0xa4, 0xeb, 0xd4, 0x59, 0x34, 0xf5, 0x67, 0x92, 0xb5, 0x11, 0x2d, 0xcd,
    0x75, 0xa1, 0x07, 0x5f, 0xdc, 0x88, 0x92, 0x45
};
const uint8_t kSSHKDFexpectedSHA1[] = {
    0xe2, 0xf6, 0x27, 0xc0, 0xb4, 0x3f, 0x1a, 0xc1
};

const uint8_t kSSHKDFkeySHA224[] = {
    0x00, 0x00, 0x00, 0x81, 0x00, 0x8d, 0xe6, 0x0d, 0xf0, 0x19, 0xc2, 0x39,
    0x66, 0xd2, 0x15, 0xd9, 0xb8, 0x49, 0x0a, 0xc4, 0x93, 0xdf, 0xae, 0x59,
    0xb9, 0x9d, 0xbe, 0xfd, 0xad, 0x81, 0xd2, 0xc9, 0xe7, 0x61, 0x20, 0x5c,
    0x93, 0xa6, 0x96, 0xdb, 0xd9, 0xe5, 0x38, 0xcc, 0x57, 0xcd, 0x3e, 0x24,
    0xc2, 0x79, 0x8d, 0x2c, 0x56, 0x56, 0x1d, 0x68, 0x03, 0xe8, 0xee, 0x24,
    0xe1, 0x12, 0xba, 0xbe, 0xf8, 0x4a, 0xd5, 0xa2, 0xc5, 0x71, 0xc5, 0x72,
    0x33, 0x9f, 0x2b, 0x38, 0xf1, 0x34, 0x51, 0x64, 0x31, 0x4f, 0x8f, 0x47,
    0x14, 0x04, 0x7f, 0x0c, 0x66, 0x65, 0x0f, 0x10, 0x05, 0x10, 0x44, 0xf8,
    0xdc, 0xd2, 0x56, 0xbf, 0xe8, 0x17, 0x13, 0x02, 0xa8, 0x1c, 0xe1, 0x3f,
    0x47, 0xf7, 0x37, 0x5d, 0xb8, 0x0a, 0x6b, 0xbf, 0x8c, 0xe7, 0xd8, 0xf9,
    0x6e, 0x03, 0xfc, 0x62, 0x75, 0xfd, 0x5d, 0xac, 0xfb, 0xdd, 0x16, 0x67,
    0x92
};
const uint8_t kSSHKDFxcghashSHA224[] = {
    0xe6, 0x9f, 0xbb, 0xee, 0x90, 0xf0, 0xcb, 0x7c, 0x57, 0x99, 0x6c, 0x6f,
    0x3f, 0x9e, 0xc4, 0xc7, 0xde, 0x9f, 0x0c, 0x43, 0xb7, 0xc9, 0x93, 0xec,
    0x3e, 0xc1, 0xd4, 0xca
};
const uint8_t kSSHKDFsessionSHA224[] = {
    0xe6, 0x9f, 0xbb, 0xee, 0x90, 0xf0, 0xcb, 0x7c, 0x57, 0x99, 0x6c, 0x6f,
    0x3f, 0x9e, 0xc4, 0xc7, 0xde, 0x9f, 0x0c, 0x43, 0xb7, 0xc9, 0x93, 0xec,
    0x3e, 0xc1, 0xd4, 0xca
};
const uint8_t kSSHKDFexpectedSHA224[] = {
    0x9f, 0xff, 0x6c, 0x6a, 0x6d, 0x1f, 0x5c, 0x31
};

static const uint8_t kSSHKDFkeySHA256[] = {
    0x00, 0x00, 0x00, 0x81, 0x00, 0x87, 0x5c, 0x55, 0x1c, 0xef, 0x52, 0x6a,
    0x4a, 0x8b, 0xe1, 0xa7, 0xdf, 0x27, 0xe9, 0xed, 0x35, 0x4b, 0xac, 0x9a,
    0xfb, 0x71, 0xf5, 0x3d, 0xba, 0xe9, 0x05, 0x67, 0x9d, 0x14, 0xf9, 0xfa,
    0xf2, 0x46, 0x9c, 0x53, 0x45, 0x7c, 0xf8, 0x0a, 0x36, 0x6b, 0xe2, 0x78,
    0x96, 0x5b, 0xa6, 0x25, 0x52, 0x76, 0xca, 0x2d, 0x9f, 0x4a, 0x97, 0xd2,
    0x71, 0xf7, 0x1e, 0x50, 0xd8, 0xa9, 0xec, 0x46, 0x25, 0x3a, 0x6a, 0x90,
    0x6a, 0xc2, 0xc5, 0xe4, 0xf4, 0x8b, 0x27, 0xa6, 0x3c, 0xe0, 0x8d, 0x80,
    0x39, 0x0a, 0x49, 0x2a, 0xa4, 0x3b, 0xad, 0x9d, 0x88, 0x2c, 0xca, 0xc2,
    0x3d, 0xac, 0x88, 0xbc, 0xad, 0xa4, 0xb4, 0xd4, 0x26, 0xa3, 0x62, 0x08,
    0x3d, 0xab, 0x65, 0x69, 0xc5, 0x4c, 0x22, 0x4d, 0xd2, 0xd8, 0x76, 0x43,
    0xaa, 0x22, 0x76, 0x93, 0xe1, 0x41, 0xad, 0x16, 0x30, 0xce, 0x13, 0x14,
    0x4e
};
static const uint8_t kSSHKDFxcghashSHA256[] = {
    0x0e, 0x68, 0x3f, 0xc8, 0xa9, 0xed, 0x7c, 0x2f, 0xf0, 0x2d, 0xef, 0x23,
    0xb2, 0x74, 0x5e, 0xbc, 0x99, 0xb2, 0x67, 0xda, 0xa8, 0x6a, 0x4a, 0xa7,
    0x69, 0x72, 0x39, 0x08, 0x82, 0x53, 0xf6, 0x42
};
static const uint8_t kSSHKDFsessionSHA256[] = {
    0x0e, 0x68, 0x3f, 0xc8, 0xa9, 0xed, 0x7c, 0x2f, 0xf0, 0x2d, 0xef, 0x23,
    0xb2, 0x74, 0x5e, 0xbc, 0x99, 0xb2, 0x67, 0xda, 0xa8, 0x6a, 0x4a, 0xa7,
    0x69, 0x72, 0x39, 0x08, 0x82, 0x53, 0xf6, 0x42
};
static const uint8_t kSSHKDFexpectedSHA256[] = {
    0x41, 0xff, 0x2e, 0xad, 0x16, 0x83, 0xf1, 0xe6
};

const uint8_t kSSHKDFkeySHA384[] = {
    0x00, 0x00, 0x00, 0x81, 0x00, 0x94, 0x14, 0x56, 0xbd, 0x72, 0x26, 0x7a,
    0x90, 0x69, 0x0f, 0xfc, 0x87, 0x35, 0x28, 0xf4, 0xb7, 0x63, 0x94, 0x43,
    0x1a, 0xce, 0xee, 0x1e, 0x24, 0xa7, 0xbe, 0xd4, 0x14, 0x56, 0x8d, 0x9b,
    0x97, 0xc8, 0x4c, 0xe1, 0x3d, 0x34, 0xa2, 0xb4, 0xa6, 0x3e, 0xf7, 0x35,
    0xba, 0xc2, 0x3a, 0xf0, 0xb7, 0xfa, 0x63, 0x4a, 0x9e, 0x56, 0xc2, 0xd7,
    0x75, 0xc7, 0x41, 0xa6, 0x1d, 0x63, 0x98, 0x13, 0x32, 0xf9, 0x02, 0x7d,
    0x3f, 0x52, 0xc4, 0xa9, 0xa3, 0xad, 0xb8, 0x3e, 0x96, 0xd3, 0x9f, 0x7e,
    0x6b, 0xb7, 0x25, 0x14, 0x79, 0x7d, 0xa3, 0x2f, 0x2f, 0x0e, 0xdb, 0x59,
    0xac, 0xcf, 0xc5, 0x8a, 0x49, 0xfc, 0x34, 0xb1, 0x98, 0xe0, 0x28, 0x5b,
    0x31, 0x03, 0x2a, 0xc9, 0xf0, 0x69, 0x07, 0xde, 0xf1, 0x96, 0xf5, 0x74,
    0x8b, 0xd3, 0x2c, 0xe2, 0x2a, 0x53, 0x83, 0xa1, 0xbb, 0xdb, 0xd3, 0x1f,
    0x24
};
const uint8_t kSSHKDFxcghashSHA384[] = {
    0xe0, 0xde, 0xe8, 0x0c, 0xcc, 0x16, 0x28, 0x84, 0x39, 0x39, 0x30, 0xad,
    0x20, 0x73, 0xd9, 0x21, 0x20, 0xc8, 0x04, 0x25, 0x41, 0x62, 0x44, 0x6b,
    0x7d, 0x04, 0x8f, 0x85, 0xa1, 0xa4, 0xdd, 0x7b, 0x63, 0x6a, 0x09, 0xb6,
    0x92, 0x52, 0xb8, 0x09, 0x52, 0xa0, 0x58, 0x1e, 0x94, 0x90, 0xee, 0x5a
};
const uint8_t kSSHKDFsessionSHA384[] = {
    0xe0, 0xde, 0xe8, 0x0c, 0xcc, 0x16, 0x28, 0x84, 0x39, 0x39, 0x30, 0xad,
    0x20, 0x73, 0xd9, 0x21, 0x20, 0xc8, 0x04, 0x25, 0x41, 0x62, 0x44, 0x6b,
    0x7d, 0x04, 0x8f, 0x85, 0xa1, 0xa4, 0xdd, 0x7b, 0x63, 0x6a, 0x09, 0xb6,
    0x92, 0x52, 0xb8, 0x09, 0x52, 0xa0, 0x58, 0x1e, 0x94, 0x90, 0xee, 0x5a
};
const uint8_t kSSHKDFexpectedSHA384[] = {
    0xd3, 0x1c, 0x16, 0xf6, 0x7b, 0x17, 0xbc, 0x69
};

const uint8_t kSSHKDFkeySHA512[] = {
    0x00, 0x00, 0x00, 0x80, 0x57, 0x53, 0x08, 0xca, 0x39, 0x57, 0x98, 0xbb,
    0x21, 0xec, 0x54, 0x38, 0xc4, 0x6a, 0x88, 0xff, 0xa3, 0xf7, 0xf7, 0x67,
    0x1c, 0x06, 0xf9, 0x24, 0xab, 0xf7, 0xc3, 0xcf, 0xb4, 0x6c, 0x78, 0xc0,
    0x25, 0x59, 0x6e, 0x4a, 0xba, 0x50, 0xc3, 0x27, 0x10, 0x89, 0x18, 0x4a,
    0x44, 0x7a, 0x57, 0x1a, 0xbb, 0x7f, 0x4a, 0x1b, 0x1c, 0x41, 0xf5, 0xd5,
    0xca, 0x80, 0x62, 0x94, 0x0d, 0x43, 0x69, 0x77, 0x85, 0x89, 0xfd, 0xe8,
    0x1a, 0x71, 0xb2, 0x22, 0x8f, 0x01, 0x8c, 0x4c, 0x83, 0x6c, 0xf3, 0x89,
    0xf8, 0x54, 0xf8, 0x6d, 0xe7, 0x1a, 0x68, 0xb1, 0x69, 0x3f, 0xe8, 0xff,
    0xa1, 0xc5, 0x9c, 0xe7, 0xe9, 0xf9, 0x22, 0x3d, 0xeb, 0xad, 0xa2, 0x56,
    0x6d, 0x2b, 0x0e, 0x56, 0x78, 0xa4, 0x8b, 0xfb, 0x53, 0x0e, 0x7b, 0xee,
    0x42, 0xbd, 0x2a, 0xc7, 0x30, 0x4a, 0x0a, 0x5a, 0xe3, 0x39, 0xa2, 0xcd
};
const uint8_t kSSHKDFxcghashSHA512[] = {
    0xa4, 0x12, 0x5a, 0xa9, 0x89, 0x80, 0x92, 0xca, 0x50, 0xc3, 0xc1, 0x63,
    0x1c, 0x03, 0xdc, 0xbc, 0x9d, 0xf9, 0x5c, 0xeb, 0xb4, 0x09, 0x88, 0x1e,
    0x58, 0x01, 0x08, 0xb6, 0xcc, 0x47, 0x04, 0xb7, 0x6c, 0xc7, 0x7b, 0x87,
    0x95, 0xfd, 0x59, 0x40, 0x56, 0x1e, 0x32, 0x24, 0xcc, 0x75, 0x84, 0x85,
    0x18, 0x99, 0x2b, 0xd8, 0xd9, 0xb7, 0x0f, 0xe0, 0xfc, 0x97, 0x7a, 0x47,
    0x60, 0x63, 0xc8, 0xbf
};
const uint8_t kSSHKDFsessionSHA512[] = {
    0xa4, 0x12, 0x5a, 0xa9, 0x89, 0x80, 0x92, 0xca, 0x50, 0xc3, 0xc1, 0x63,
    0x1c, 0x03, 0xdc, 0xbc, 0x9d, 0xf9, 0x5c, 0xeb, 0xb4, 0x09, 0x88, 0x1e,
    0x58, 0x01, 0x08, 0xb6, 0xcc, 0x47, 0x04, 0xb7, 0x6c, 0xc7, 0x7b, 0x87,
    0x95, 0xfd, 0x59, 0x40, 0x56, 0x1e, 0x32, 0x24, 0xcc, 0x75, 0x84, 0x85,
    0x18, 0x99, 0x2b, 0xd8, 0xd9, 0xb7, 0x0f, 0xe0, 0xfc, 0x97, 0x7a, 0x47,
    0x60, 0x63, 0xc8, 0xbf
};
const uint8_t kSSHKDFexpectedSHA512[] = {
    0x0e, 0x26, 0x93, 0xad, 0xe0, 0x52, 0x4a, 0xf8
};

static const struct SSHKDFTestVector {
    // func is the hash function for PBKDF2 to test.
    const EVP_MD *(*func)();
    const uint8_t *key;
    const size_t key_len;
    const uint8_t *xcghash;
    const size_t xcghash_len;
    const uint8_t *session_id;
    const size_t session_id_len;
    const char type;
    const size_t output_len;
    const uint8_t *expected_output;
    const FIPSStatus expect_approved;
} kSSHKDFTestVectors[] = {
    {
        EVP_sha1,
        kSSHKDFkeySHA1, sizeof(kSSHKDFkeySHA1),
        kSSHKDFxcghashSHA1, sizeof(kSSHKDFxcghashSHA1),
        kSSHKDFsessionSHA1, sizeof(kSSHKDFsessionSHA1),
        kSSHKDFtype,
        sizeof(kSSHKDFexpectedSHA1),
        kSSHKDFexpectedSHA1,
        AWSLC_APPROVED
    },
    {
        EVP_sha224,
        kSSHKDFkeySHA224, sizeof(kSSHKDFkeySHA224),
        kSSHKDFxcghashSHA224, sizeof(kSSHKDFxcghashSHA224),
        kSSHKDFsessionSHA224, sizeof(kSSHKDFsessionSHA224),
        kSSHKDFtype,
        sizeof(kSSHKDFexpectedSHA224),
        kSSHKDFexpectedSHA224,
        AWSLC_APPROVED
    },
    {
        EVP_sha256,
        kSSHKDFkeySHA256, sizeof(kSSHKDFkeySHA256),
        kSSHKDFxcghashSHA256, sizeof(kSSHKDFxcghashSHA256),
        kSSHKDFsessionSHA256, sizeof(kSSHKDFsessionSHA256),
        kSSHKDFtype,
        sizeof(kSSHKDFexpectedSHA256),
        kSSHKDFexpectedSHA256,
        AWSLC_APPROVED
    },
    {
        EVP_sha384,
        kSSHKDFkeySHA384, sizeof(kSSHKDFkeySHA384),
        kSSHKDFxcghashSHA384, sizeof(kSSHKDFxcghashSHA384),
        kSSHKDFsessionSHA384, sizeof(kSSHKDFsessionSHA384),
        kSSHKDFtype,
        sizeof(kSSHKDFexpectedSHA384),
        kSSHKDFexpectedSHA384,
        AWSLC_APPROVED
    },
    {
        EVP_sha512,
        kSSHKDFkeySHA512, sizeof(kSSHKDFkeySHA512),
        kSSHKDFxcghashSHA512, sizeof(kSSHKDFxcghashSHA512),
        kSSHKDFsessionSHA512, sizeof(kSSHKDFsessionSHA512),
        kSSHKDFtype,
        sizeof(kSSHKDFexpectedSHA512),
        kSSHKDFexpectedSHA512,
        AWSLC_APPROVED
    },
    {
        EVP_md5,
        kSSHKDFkeySHA256, sizeof(kSSHKDFkeySHA256),
        kSSHKDFxcghashSHA256, sizeof(kSSHKDFxcghashSHA256),
        kSSHKDFsessionSHA256, sizeof(kSSHKDFsessionSHA256),
        kSSHKDFtype,
        sizeof(kSSHKDFexpectedSHA256),
        kSSHKDFexpectedSHA256,  // Not actually, that's the SHA-256 data.
        AWSLC_NOT_APPROVED
    },
};

class SSHKDF_ServiceIndicatorTest : public TestWithNoErrors<SSHKDFTestVector> {
};

INSTANTIATE_TEST_SUITE_P(All, SSHKDF_ServiceIndicatorTest,
                         testing::ValuesIn(kSSHKDFTestVectors));

TEST_P(SSHKDF_ServiceIndicatorTest, SSHKDF) {
    const SSHKDFTestVector &test = GetParam();

    FIPSStatus approved = AWSLC_NOT_APPROVED;
    uint8_t output[sizeof(kSSHKDFexpectedSHA512)];   // largest test vector output size
    CALL_SERVICE_AND_CHECK_APPROVED(
        approved, ASSERT_TRUE(SSHKDF(test.func(), test.key, test.key_len,
            test.xcghash, test.xcghash_len,
            test.session_id, test.session_id_len,
            test.type,
            output, test.output_len)));
    if (test.expect_approved) {
        EXPECT_EQ(Bytes(test.expected_output, test.output_len),
                  Bytes(output, test.output_len));
    }
    EXPECT_EQ(approved, test.expect_approved);
}

TEST(ServiceIndicatorTest, CMAC) {
  FIPSStatus approved = AWSLC_NOT_APPROVED;

  bssl::UniquePtr<CMAC_CTX> ctx(CMAC_CTX_new());
  ASSERT_TRUE(ctx);

  // Test running the CMAC interfaces one by one directly, and check
  // |CMAC_Final| for approval at the end. |CMAC_Init| and |CMAC_Update|
  // should not be approved, because the functions do not indicate that a
  // service has been fully completed yet.
  CALL_SERVICE_AND_CHECK_APPROVED(approved,
      ASSERT_TRUE(CMAC_Init(ctx.get(), kAESKey, sizeof(kAESKey),
                            EVP_aes_128_cbc(), nullptr)));
  EXPECT_EQ(approved, AWSLC_NOT_APPROVED);
  CALL_SERVICE_AND_CHECK_APPROVED(approved,
      ASSERT_TRUE(CMAC_Update(ctx.get(), kPlaintext, sizeof(kPlaintext))));
  EXPECT_EQ(approved, AWSLC_NOT_APPROVED);

  uint8_t mac[16];
  size_t out_len;
  CALL_SERVICE_AND_CHECK_APPROVED(approved,
      ASSERT_TRUE(CMAC_Final(ctx.get(), mac, &out_len)));
  EXPECT_EQ(approved, AWSLC_APPROVED);
  EXPECT_EQ(Bytes(kAESCMACOutput), Bytes(mac));

  // Test using the one-shot API for approval.
  CALL_SERVICE_AND_CHECK_APPROVED(approved,
      ASSERT_TRUE(AES_CMAC(mac, kAESKey, sizeof(kAESKey), kPlaintext,
                           sizeof(kPlaintext))));
  EXPECT_EQ(Bytes(kAESCMACOutput), Bytes(mac));
  EXPECT_EQ(approved, AWSLC_APPROVED);
}

TEST(ServiceIndicatorTest, BasicTest) {
  FIPSStatus approved = AWSLC_NOT_APPROVED;

  bssl::ScopedEVP_AEAD_CTX aead_ctx;
  AES_KEY aes_key;
  uint8_t aes_iv[sizeof(kAESIV)];
  uint8_t output[256];
  uint8_t ofb_output[sizeof(kPlaintext)];
  size_t out_len;
  int num = 0;
  uint64_t counter_before, counter_after;

  ASSERT_TRUE(EVP_AEAD_CTX_init(aead_ctx.get(), EVP_aead_aes_128_gcm_randnonce(),
                                kAESKey, sizeof(kAESKey), 0, nullptr));
  // Because the service indicator gets initialised in
  // |FIPS_service_indicator_update_state|, which is called by all approved
  // services, the self_test run at the beginning would have updated it more
  // than once. The following test ensures that it's not zero and that it gets
  // updated by calling an approved service without calling
  // |FIPS_service_indicator_before_call| first, which can init the counter, but
  // instead calling |FIPS_service_indicator_after_call|
  // This test also ensures that the counter gets incremented once, i.e. it was
  // locked through the internal calls.
  counter_before = FIPS_service_indicator_after_call();
  ASSERT_NE(counter_before, (uint64_t)0);
  EVP_AEAD_CTX_seal(aead_ctx.get(), output, &out_len, sizeof(output), nullptr,
                    0, kPlaintext, sizeof(kPlaintext), nullptr, 0);
  counter_after = FIPS_service_indicator_after_call();
  ASSERT_EQ(counter_after, counter_before+1);

  // Call an approved service.
  CALL_SERVICE_AND_CHECK_APPROVED(approved,
      EVP_AEAD_CTX_seal(aead_ctx.get(), output, &out_len, sizeof(output),
                      nullptr, 0, kPlaintext, sizeof(kPlaintext), nullptr, 0));
  ASSERT_EQ(approved, AWSLC_APPROVED);

  // Call an approved service in a macro.
  CALL_SERVICE_AND_CHECK_APPROVED(approved,
      ASSERT_EQ(EVP_AEAD_CTX_seal(aead_ctx.get(), output, &out_len,
                                  sizeof(output), nullptr, 0, kPlaintext,
                                  sizeof(kPlaintext), nullptr, 0), 1));
  ASSERT_EQ(approved, AWSLC_APPROVED);

  // Call an approved service and compare expected return value.
  int return_val = 0;
  CALL_SERVICE_AND_CHECK_APPROVED(approved,
      return_val = EVP_AEAD_CTX_seal(aead_ctx.get(),  output, &out_len,
                                     sizeof(output), nullptr, 0, kPlaintext,
                                     sizeof(kPlaintext), nullptr, 0));
  ASSERT_EQ(return_val, 1);
  ASSERT_EQ(approved, AWSLC_APPROVED);

  // Call an approved service wrapped in an if statement.
  return_val = 0;
  CALL_SERVICE_AND_CHECK_APPROVED(approved,
    if(EVP_AEAD_CTX_seal(aead_ctx.get(), output, &out_len, sizeof(output),
         nullptr, 0, kPlaintext, sizeof(kPlaintext), nullptr, 0) == 1)
    {
      return_val = 1;
    }
  );
  ASSERT_EQ(return_val, 1);
  ASSERT_EQ(approved, AWSLC_APPROVED);

  // Fail an approved service on purpose.
  return_val = 0;
  CALL_SERVICE_AND_CHECK_APPROVED(approved,
      return_val = EVP_AEAD_CTX_seal(aead_ctx.get(), output, &out_len, 0,
                                     nullptr, 0, kPlaintext, sizeof(kPlaintext),
                                     nullptr, 0));
  ASSERT_EQ(return_val, 0);
  ASSERT_EQ(approved, AWSLC_NOT_APPROVED);

  // Fail an approved service on purpose while wrapped in an if statement.
  return_val = 0;
  CALL_SERVICE_AND_CHECK_APPROVED(approved,
    if(EVP_AEAD_CTX_seal(aead_ctx.get(), output, &out_len, 0,
        nullptr, 0, kPlaintext, sizeof(kPlaintext), nullptr, 0) == 1)
    {
      return_val = 1;
    }
  );
  ASSERT_EQ(return_val, 0);
  ASSERT_EQ(approved, AWSLC_NOT_APPROVED);

  // Call a non-approved service.
  memcpy(aes_iv, kAESIV, sizeof(kAESIV));
  ASSERT_TRUE(AES_set_encrypt_key(kAESKey, 8 * sizeof(kAESKey), &aes_key) == 0);
  CALL_SERVICE_AND_CHECK_APPROVED(approved,
      AES_ofb128_encrypt(kPlaintext, ofb_output, sizeof(kPlaintext), &aes_key,
                         aes_iv, &num));
  EXPECT_EQ(Bytes(kAESOFBCiphertext), Bytes(ofb_output));
  ASSERT_EQ(approved, AWSLC_NOT_APPROVED);
}

// Test the SHA interfaces one by one and check that only |*_Final| does the
// approval at the end.
TEST(ServiceIndicatorTest, SHA) {
  FIPSStatus approved = AWSLC_NOT_APPROVED;
  std::vector<uint8_t> digest;

  digest.resize(MD4_DIGEST_LENGTH);
  MD4_CTX md4_ctx;
  CALL_SERVICE_AND_CHECK_APPROVED(approved, ASSERT_TRUE(MD4_Init(&md4_ctx)));
  EXPECT_EQ(approved, AWSLC_NOT_APPROVED);
  CALL_SERVICE_AND_CHECK_APPROVED(approved,
      ASSERT_TRUE(MD4_Update(&md4_ctx, kPlaintext, sizeof(kPlaintext))));
  EXPECT_EQ(approved, AWSLC_NOT_APPROVED);
  CALL_SERVICE_AND_CHECK_APPROVED(approved,
      ASSERT_TRUE(MD4_Final(digest.data(), &md4_ctx)));
  EXPECT_EQ(Bytes(kOutput_md4), Bytes(digest));
  EXPECT_EQ(approved, AWSLC_NOT_APPROVED);

  digest.resize(MD5_DIGEST_LENGTH);
  MD5_CTX md5_ctx;
  CALL_SERVICE_AND_CHECK_APPROVED(approved, ASSERT_TRUE(MD5_Init(&md5_ctx)));
  EXPECT_EQ(approved, AWSLC_NOT_APPROVED);
  CALL_SERVICE_AND_CHECK_APPROVED(approved,
      ASSERT_TRUE(MD5_Update(&md5_ctx, kPlaintext, sizeof(kPlaintext))));
  EXPECT_EQ(approved, AWSLC_NOT_APPROVED);
  CALL_SERVICE_AND_CHECK_APPROVED(approved,
      ASSERT_TRUE(MD5_Final(digest.data(), &md5_ctx)));
  EXPECT_EQ(Bytes(kOutput_md5), Bytes(digest));
  EXPECT_EQ(approved, AWSLC_NOT_APPROVED);

  digest.resize(SHA_DIGEST_LENGTH);
  SHA_CTX sha_ctx;
  CALL_SERVICE_AND_CHECK_APPROVED(approved, ASSERT_TRUE(SHA1_Init(&sha_ctx)));
  EXPECT_EQ(approved, AWSLC_NOT_APPROVED);
  CALL_SERVICE_AND_CHECK_APPROVED(approved,
      ASSERT_TRUE(SHA1_Update(&sha_ctx, kPlaintext, sizeof(kPlaintext))));
  EXPECT_EQ(approved, AWSLC_NOT_APPROVED);
  CALL_SERVICE_AND_CHECK_APPROVED(approved,
      ASSERT_TRUE(SHA1_Final(digest.data(), &sha_ctx)));
  EXPECT_EQ(Bytes(kOutput_sha1), Bytes(digest));
  EXPECT_EQ(approved, AWSLC_APPROVED);

  digest.resize(SHA224_DIGEST_LENGTH);
  SHA256_CTX sha224_ctx;
  CALL_SERVICE_AND_CHECK_APPROVED(approved,
      ASSERT_TRUE(SHA224_Init(&sha224_ctx)));
  EXPECT_EQ(approved, AWSLC_NOT_APPROVED);
  CALL_SERVICE_AND_CHECK_APPROVED(approved,
      ASSERT_TRUE(SHA224_Update(&sha224_ctx, kPlaintext, sizeof(kPlaintext))));
  EXPECT_EQ(approved, AWSLC_NOT_APPROVED);
  CALL_SERVICE_AND_CHECK_APPROVED(approved,
      ASSERT_TRUE(SHA224_Final(digest.data(), &sha224_ctx)));
  EXPECT_EQ(Bytes(kOutput_sha224), Bytes(digest));
  EXPECT_EQ(approved, AWSLC_APPROVED);

  digest.resize(SHA256_DIGEST_LENGTH);
  SHA256_CTX sha256_ctx;
  CALL_SERVICE_AND_CHECK_APPROVED(approved,
      ASSERT_TRUE(SHA256_Init(&sha256_ctx)));
  EXPECT_EQ(approved, AWSLC_NOT_APPROVED);
  CALL_SERVICE_AND_CHECK_APPROVED(approved,
      ASSERT_TRUE(SHA256_Update(&sha256_ctx, kPlaintext, sizeof(kPlaintext))));
  EXPECT_EQ(approved, AWSLC_NOT_APPROVED);
  CALL_SERVICE_AND_CHECK_APPROVED(approved,
      ASSERT_TRUE(SHA256_Final(digest.data(), &sha256_ctx)));
  EXPECT_EQ(Bytes(kOutput_sha256), Bytes(digest));
  EXPECT_EQ(approved, AWSLC_APPROVED);

  digest.resize(SHA384_DIGEST_LENGTH);
  SHA512_CTX sha384_ctx;
  CALL_SERVICE_AND_CHECK_APPROVED(approved,
      ASSERT_TRUE(SHA384_Init(&sha384_ctx)));
  EXPECT_EQ(approved, AWSLC_NOT_APPROVED);
  CALL_SERVICE_AND_CHECK_APPROVED(approved,
      ASSERT_TRUE(SHA384_Update(&sha384_ctx, kPlaintext, sizeof(kPlaintext))));
  EXPECT_EQ(approved, AWSLC_NOT_APPROVED);
  CALL_SERVICE_AND_CHECK_APPROVED(approved,
      ASSERT_TRUE(SHA384_Final(digest.data(), &sha384_ctx)));
  EXPECT_EQ(Bytes(kOutput_sha384), Bytes(digest));
  EXPECT_EQ(approved, AWSLC_APPROVED);

  digest.resize(SHA512_DIGEST_LENGTH);
  SHA512_CTX sha512_ctx;
  CALL_SERVICE_AND_CHECK_APPROVED(approved,
      ASSERT_TRUE(SHA512_Init(&sha512_ctx)));
  EXPECT_EQ(approved, AWSLC_NOT_APPROVED);
  CALL_SERVICE_AND_CHECK_APPROVED(approved,
      ASSERT_TRUE(SHA512_Update(&sha512_ctx, kPlaintext, sizeof(kPlaintext))));
  EXPECT_EQ(approved, AWSLC_NOT_APPROVED);
  CALL_SERVICE_AND_CHECK_APPROVED(approved,
      ASSERT_TRUE(SHA512_Final(digest.data(), &sha512_ctx)));
  EXPECT_EQ(Bytes(kOutput_sha512), Bytes(digest));
  EXPECT_EQ(approved, AWSLC_APPROVED);

  digest.resize(SHA512_224_DIGEST_LENGTH);
  SHA512_CTX sha512_224_ctx;
  CALL_SERVICE_AND_CHECK_APPROVED(approved,
      ASSERT_TRUE(SHA512_224_Init(&sha512_224_ctx)));
  EXPECT_EQ(approved, AWSLC_NOT_APPROVED);
  CALL_SERVICE_AND_CHECK_APPROVED(approved,
      ASSERT_TRUE(SHA512_224_Update(&sha512_224_ctx, kPlaintext,
                                    sizeof(kPlaintext))));
  EXPECT_EQ(approved, AWSLC_NOT_APPROVED);
  CALL_SERVICE_AND_CHECK_APPROVED(approved,
      ASSERT_TRUE(SHA512_224_Final(digest.data(), &sha512_224_ctx)));
  EXPECT_EQ(Bytes(kOutput_sha512_224), Bytes(digest));
  EXPECT_EQ(approved, AWSLC_APPROVED);

  digest.resize(SHA512_256_DIGEST_LENGTH);
  SHA512_CTX sha512_256_ctx;
  CALL_SERVICE_AND_CHECK_APPROVED(approved,
      ASSERT_TRUE(SHA512_256_Init(&sha512_256_ctx)));
  EXPECT_EQ(approved, AWSLC_NOT_APPROVED);
  CALL_SERVICE_AND_CHECK_APPROVED(approved,
      ASSERT_TRUE(SHA512_256_Update(&sha512_256_ctx, kPlaintext,
                                    sizeof(kPlaintext))));
  EXPECT_EQ(approved, AWSLC_NOT_APPROVED);
  CALL_SERVICE_AND_CHECK_APPROVED(approved,
      ASSERT_TRUE(SHA512_256_Final(digest.data(), &sha512_256_ctx)));
  EXPECT_EQ(Bytes(kOutput_sha512_256), Bytes(digest));
  EXPECT_EQ(approved, AWSLC_APPROVED);
}

TEST(ServiceIndicatorTest, AESECB) {
  FIPSStatus approved = AWSLC_NOT_APPROVED;

  AES_KEY aes_key;
  uint8_t output[256];

  // AES-ECB Encryption KAT for 128 bit key.
  ASSERT_TRUE(AES_set_encrypt_key(kAESKey, 8 * sizeof(kAESKey), &aes_key) == 0);
  // AES_ecb_encrypt encrypts (or decrypts) a single, 16 byte block from in to
  // out.
  for (size_t i = 0; i < sizeof(kPlaintext) / AES_BLOCK_SIZE; i++) {
    CALL_SERVICE_AND_CHECK_APPROVED(
        approved,
        AES_ecb_encrypt(&kPlaintext[i * AES_BLOCK_SIZE],
                        &output[i * AES_BLOCK_SIZE], &aes_key, AES_ENCRYPT));
    EXPECT_EQ(approved, AWSLC_APPROVED);
  }
  EXPECT_EQ(Bytes(kAESECBCiphertext), Bytes(output, sizeof(kAESECBCiphertext)));

  // AES-ECB Decryption KAT for 128 bit key.
  ASSERT_TRUE(AES_set_decrypt_key(kAESKey, 8 * sizeof(kAESKey), &aes_key) == 0);
  for (size_t i = 0; i < sizeof(kPlaintext) / AES_BLOCK_SIZE; i++) {
    CALL_SERVICE_AND_CHECK_APPROVED(
        approved,
        AES_ecb_encrypt(&kAESECBCiphertext[i * AES_BLOCK_SIZE],
                        &output[i * AES_BLOCK_SIZE], &aes_key, AES_DECRYPT));
    EXPECT_EQ(approved, AWSLC_APPROVED);
  }
  EXPECT_EQ(Bytes(kPlaintext), Bytes(output, sizeof(kPlaintext)));

  // AES-ECB Encryption KAT for 192 bit key.
  ASSERT_TRUE(
      AES_set_encrypt_key(kAESKey_192, 8 * sizeof(kAESKey_192), &aes_key) == 0);
  for (size_t i = 0; i < sizeof(kPlaintext) / AES_BLOCK_SIZE; i++) {
    CALL_SERVICE_AND_CHECK_APPROVED(
        approved,
        AES_ecb_encrypt(&kPlaintext[i * AES_BLOCK_SIZE],
                        &output[i * AES_BLOCK_SIZE], &aes_key, AES_ENCRYPT));
    EXPECT_EQ(approved, AWSLC_APPROVED);
  }
  EXPECT_EQ(Bytes(kAESECBCiphertext_192),
            Bytes(output, sizeof(kAESECBCiphertext_192)));

  // AES-ECB Decryption KAT for 192 bit key.
  ASSERT_TRUE(
      AES_set_decrypt_key(kAESKey_192, 8 * sizeof(kAESKey_192), &aes_key) == 0);
  for (size_t i = 0; i < sizeof(kPlaintext) / AES_BLOCK_SIZE; i++) {
    CALL_SERVICE_AND_CHECK_APPROVED(
        approved,
        AES_ecb_encrypt(&kAESECBCiphertext_192[i * AES_BLOCK_SIZE],
                        &output[i * AES_BLOCK_SIZE], &aes_key, AES_DECRYPT));
    EXPECT_EQ(approved, AWSLC_APPROVED);
  }
  EXPECT_EQ(Bytes(kPlaintext), Bytes(output, sizeof(kPlaintext)));

  // AES-ECB Encryption KAT for 256 bit key.
  ASSERT_TRUE(
      AES_set_encrypt_key(kAESKey_256, 8 * sizeof(kAESKey_256), &aes_key) == 0);
  for (size_t i = 0; i < sizeof(kPlaintext) / AES_BLOCK_SIZE; i++) {
    CALL_SERVICE_AND_CHECK_APPROVED(
        approved,
        AES_ecb_encrypt(&kPlaintext[i * AES_BLOCK_SIZE],
                        &output[i * AES_BLOCK_SIZE], &aes_key, AES_ENCRYPT));
    EXPECT_EQ(approved, AWSLC_APPROVED);
  }
  EXPECT_EQ(Bytes(kAESECBCiphertext_256),
            Bytes(output, sizeof(kAESECBCiphertext_256)));

  // AES-ECB Decryption KAT for 256 bit key.
  ASSERT_TRUE(
      AES_set_decrypt_key(kAESKey_256, 8 * sizeof(kAESKey_256), &aes_key) == 0);
  for (size_t i = 0; i < sizeof(kPlaintext) / AES_BLOCK_SIZE; i++) {
    CALL_SERVICE_AND_CHECK_APPROVED(
        approved,
        AES_ecb_encrypt(&kAESECBCiphertext_256[i * AES_BLOCK_SIZE],
                        &output[i * AES_BLOCK_SIZE], &aes_key, AES_DECRYPT));
    EXPECT_EQ(approved, AWSLC_APPROVED);
  }
  EXPECT_EQ(Bytes(kPlaintext), Bytes(output, sizeof(kPlaintext)));
}

TEST(ServiceIndicatorTest, AESCBC) {
  FIPSStatus approved = AWSLC_NOT_APPROVED;

  AES_KEY aes_key;
  uint8_t aes_iv[sizeof(kAESIV)];
  uint8_t output[sizeof(kPlaintext)];

  // AES-CBC Encryption KAT for 128 bit key.
  memcpy(aes_iv, kAESIV, sizeof(aes_iv));
  ASSERT_TRUE(AES_set_encrypt_key(kAESKey, 8 * sizeof(kAESKey), &aes_key) == 0);
  CALL_SERVICE_AND_CHECK_APPROVED(
      approved, AES_cbc_encrypt(kPlaintext, output, sizeof(kPlaintext),
                                &aes_key, aes_iv, AES_ENCRYPT));
  EXPECT_EQ(Bytes(kAESCBCCiphertext), Bytes(output));
  EXPECT_EQ(approved, AWSLC_APPROVED);

  // AES-CBC Decryption KAT for 128 bit key.
  memcpy(aes_iv, kAESIV, sizeof(aes_iv));
  ASSERT_TRUE(AES_set_decrypt_key(kAESKey, 8 * sizeof(kAESKey), &aes_key) == 0);
  CALL_SERVICE_AND_CHECK_APPROVED(
      approved,
      AES_cbc_encrypt(kAESCBCCiphertext, output, sizeof(kAESCBCCiphertext),
                      &aes_key, aes_iv, AES_DECRYPT));
  EXPECT_EQ(Bytes(kPlaintext), Bytes(output));
  EXPECT_EQ(approved, AWSLC_APPROVED);

  // AES-CBC Encryption KAT for 192 bit key.
  memcpy(aes_iv, kAESIV, sizeof(aes_iv));
  ASSERT_TRUE(
      AES_set_encrypt_key(kAESKey_192, 8 * sizeof(kAESKey_192), &aes_key) == 0);
  CALL_SERVICE_AND_CHECK_APPROVED(
      approved, AES_cbc_encrypt(kPlaintext, output, sizeof(kPlaintext),
                                &aes_key, aes_iv, AES_ENCRYPT));
  EXPECT_EQ(Bytes(kAESCBCCiphertext_192), Bytes(output));
  EXPECT_EQ(approved, AWSLC_APPROVED);

  // AES-CBC Decryption KAT for 192 bit key.
  memcpy(aes_iv, kAESIV, sizeof(aes_iv));
  ASSERT_TRUE(
      AES_set_decrypt_key(kAESKey_192, 8 * sizeof(kAESKey_192), &aes_key) == 0);
  CALL_SERVICE_AND_CHECK_APPROVED(
      approved, AES_cbc_encrypt(kAESCBCCiphertext_192, output,
                                sizeof(kAESCBCCiphertext_192), &aes_key, aes_iv,
                                AES_DECRYPT));
  EXPECT_EQ(Bytes(kPlaintext), Bytes(output));
  EXPECT_EQ(approved, AWSLC_APPROVED);

  // AES-CBC Encryption KAT for 256 bit key.
  memcpy(aes_iv, kAESIV, sizeof(aes_iv));
  ASSERT_TRUE(
      AES_set_encrypt_key(kAESKey_256, 8 * sizeof(kAESKey_256), &aes_key) == 0);
  CALL_SERVICE_AND_CHECK_APPROVED(
      approved, AES_cbc_encrypt(kPlaintext, output, sizeof(kPlaintext),
                                &aes_key, aes_iv, AES_ENCRYPT));
  EXPECT_EQ(Bytes(kAESCBCCiphertext_256), Bytes(output));
  EXPECT_EQ(approved, AWSLC_APPROVED);

  // AES-CBC Decryption KAT for 256 bit key.
  memcpy(aes_iv, kAESIV, sizeof(aes_iv));
  ASSERT_TRUE(
      AES_set_decrypt_key(kAESKey_256, 8 * sizeof(kAESKey_256), &aes_key) == 0);
  CALL_SERVICE_AND_CHECK_APPROVED(
      approved, AES_cbc_encrypt(kAESCBCCiphertext_256, output,
                                sizeof(kAESCBCCiphertext_256), &aes_key, aes_iv,
                                AES_DECRYPT));
  EXPECT_EQ(Bytes(kPlaintext), Bytes(output));
  EXPECT_EQ(approved, AWSLC_APPROVED);
}

TEST(ServiceIndicatorTest, AESCTR) {
  FIPSStatus approved = AWSLC_NOT_APPROVED;

  AES_KEY aes_key;
  uint8_t aes_iv[sizeof(kAESIV)];
  uint8_t output[sizeof(kPlaintext)];
  unsigned num = 0;
  uint8_t ecount_buf[AES_BLOCK_SIZE];

  // AES-CTR Encryption KAT
  memcpy(aes_iv, kAESIV, sizeof(aes_iv));
  ASSERT_TRUE(AES_set_encrypt_key(kAESKey, 8 * sizeof(kAESKey), &aes_key) == 0);
  CALL_SERVICE_AND_CHECK_APPROVED(
      approved, AES_ctr128_encrypt(kPlaintext, output, sizeof(kPlaintext),
                                   &aes_key, aes_iv, ecount_buf, &num));
  EXPECT_EQ(Bytes(kAESCTRCiphertext), Bytes(output));
  EXPECT_EQ(approved, AWSLC_APPROVED);

  // AES-CTR Decryption KAT
  memcpy(aes_iv, kAESIV, sizeof(aes_iv));
  CALL_SERVICE_AND_CHECK_APPROVED(
      approved,
      AES_ctr128_encrypt(kAESCTRCiphertext, output, sizeof(kAESCTRCiphertext),
                         &aes_key, aes_iv, ecount_buf, &num));
  EXPECT_EQ(Bytes(kPlaintext), Bytes(output));
  EXPECT_EQ(approved, AWSLC_APPROVED);

  // AES-CTR Encryption KAT for 192 bit key.
  memcpy(aes_iv, kAESIV, sizeof(aes_iv));
  ASSERT_TRUE(
      AES_set_encrypt_key(kAESKey_192, 8 * sizeof(kAESKey_192), &aes_key) == 0);
  CALL_SERVICE_AND_CHECK_APPROVED(
      approved, AES_ctr128_encrypt(kPlaintext, output, sizeof(kPlaintext),
                                   &aes_key, aes_iv, ecount_buf, &num));
  EXPECT_EQ(Bytes(kAESCTRCiphertext_192), Bytes(output));
  EXPECT_EQ(approved, AWSLC_APPROVED);

  // AES-CTR Decryption KAT for 192 bit key.
  memcpy(aes_iv, kAESIV, sizeof(aes_iv));
  CALL_SERVICE_AND_CHECK_APPROVED(
      approved, AES_ctr128_encrypt(kAESCTRCiphertext_192, output,
                                   sizeof(kAESCTRCiphertext_192), &aes_key,
                                   aes_iv, ecount_buf, &num));
  EXPECT_EQ(Bytes(kPlaintext), Bytes(output));
  EXPECT_EQ(approved, AWSLC_APPROVED);

  // AES-CTR Encryption KAT for 256 bit key.
  memcpy(aes_iv, kAESIV, sizeof(aes_iv));
  ASSERT_TRUE(
      AES_set_encrypt_key(kAESKey_256, 8 * sizeof(kAESKey_256), &aes_key) == 0);
  CALL_SERVICE_AND_CHECK_APPROVED(
      approved, AES_ctr128_encrypt(kPlaintext, output, sizeof(kPlaintext),
                                   &aes_key, aes_iv, ecount_buf, &num));
  EXPECT_EQ(Bytes(kAESCTRCiphertext_256), Bytes(output));
  EXPECT_EQ(approved, AWSLC_APPROVED);

  // AES-CTR Decryption KAT for 256 bit key.
  memcpy(aes_iv, kAESIV, sizeof(aes_iv));
  CALL_SERVICE_AND_CHECK_APPROVED(
      approved, AES_ctr128_encrypt(kAESCTRCiphertext_256, output,
                                   sizeof(kAESCTRCiphertext_256), &aes_key,
                                   aes_iv, ecount_buf, &num));
  EXPECT_EQ(Bytes(kPlaintext), Bytes(output));
  EXPECT_EQ(approved, AWSLC_APPROVED);
}

TEST(ServiceIndicatorTest, AESOFB) {
  FIPSStatus approved = AWSLC_NOT_APPROVED;

  AES_KEY aes_key;
  uint8_t aes_iv[sizeof(kAESIV)];
  uint8_t output[sizeof(kPlaintext)];
  int num = 0;

  // AES-OFB Encryption KAT
  memcpy(aes_iv, kAESIV, sizeof(aes_iv));
  ASSERT_TRUE(AES_set_encrypt_key(kAESKey, 8 * sizeof(kAESKey), &aes_key) == 0);
  CALL_SERVICE_AND_CHECK_APPROVED(
      approved, AES_ofb128_encrypt(kPlaintext, output, sizeof(kPlaintext),
                                   &aes_key, aes_iv, &num));
  EXPECT_EQ(Bytes(kAESOFBCiphertext), Bytes(output));
  EXPECT_EQ(approved, AWSLC_NOT_APPROVED);

  // AES-OFB Decryption KAT
  memcpy(aes_iv, kAESIV, sizeof(aes_iv));
  CALL_SERVICE_AND_CHECK_APPROVED(
      approved,
      AES_ofb128_encrypt(kAESOFBCiphertext, output, sizeof(kAESOFBCiphertext),
                         &aes_key, aes_iv, &num));
  EXPECT_EQ(Bytes(kPlaintext), Bytes(output));
  EXPECT_EQ(approved, AWSLC_NOT_APPROVED);
}

TEST(ServiceIndicatorTest, AESCFB) {
  FIPSStatus approved = AWSLC_NOT_APPROVED;

  AES_KEY aes_key;
  uint8_t aes_iv[sizeof(kAESIV)];
  uint8_t output[sizeof(kPlaintext)];
  int num = 0;

  // AES-CFB Encryption KAT
  memcpy(aes_iv, kAESIV, sizeof(aes_iv));
  ASSERT_TRUE(AES_set_encrypt_key(kAESKey, 8 * sizeof(kAESKey), &aes_key) == 0);
  CALL_SERVICE_AND_CHECK_APPROVED(
      approved, AES_cfb128_encrypt(kPlaintext, output, sizeof(kPlaintext),
                                   &aes_key, aes_iv, &num, AES_ENCRYPT));
  EXPECT_EQ(Bytes(kAESCFBCiphertext), Bytes(output));
  EXPECT_EQ(approved, AWSLC_NOT_APPROVED);

  // AES-CFB Decryption KAT
  memcpy(aes_iv, kAESIV, sizeof(aes_iv));
  CALL_SERVICE_AND_CHECK_APPROVED(
      approved,
      AES_cfb128_encrypt(kAESCFBCiphertext, output, sizeof(kAESCFBCiphertext),
                         &aes_key, aes_iv, &num, AES_DECRYPT));
  EXPECT_EQ(Bytes(kPlaintext), Bytes(output));
  EXPECT_EQ(approved, AWSLC_NOT_APPROVED);
}

TEST(ServiceIndicatorTest, AESKW) {
  FIPSStatus approved = AWSLC_NOT_APPROVED;

  AES_KEY aes_key;
  uint8_t output[sizeof(kPlaintext) + 8];
  size_t outlen;

  // AES-KW Encryption KAT
  ASSERT_TRUE(AES_set_encrypt_key(kAESKey, 8 * sizeof(kAESKey), &aes_key) == 0);
  CALL_SERVICE_AND_CHECK_APPROVED(
      approved, outlen = AES_wrap_key(&aes_key, nullptr, output, kPlaintext,
                                      sizeof(kPlaintext)));
  ASSERT_EQ(outlen, sizeof(kAESKWCiphertext));
  EXPECT_EQ(Bytes(kAESKWCiphertext), Bytes(output));
  EXPECT_EQ(approved, AWSLC_APPROVED);

  // AES-KW Decryption KAT
  ASSERT_TRUE(AES_set_decrypt_key(kAESKey, 8 * sizeof(kAESKey), &aes_key) == 0);
  CALL_SERVICE_AND_CHECK_APPROVED(
      approved, outlen = AES_unwrap_key(&aes_key, nullptr, output,
                                        kAESKWCiphertext,
                                        sizeof(kAESKWCiphertext)));
  ASSERT_EQ(outlen, sizeof(kPlaintext));
  EXPECT_EQ(Bytes(kPlaintext), Bytes(output, sizeof(kPlaintext)));
  EXPECT_EQ(approved, AWSLC_APPROVED);
}

TEST(ServiceIndicatorTest, AESKWP) {
  FIPSStatus approved = AWSLC_NOT_APPROVED;

  AES_KEY aes_key;
  uint8_t output[sizeof(kPlaintext) + 15];
  size_t outlen;

  // AES-KWP Encryption KAT
  ASSERT_TRUE(AES_set_encrypt_key(kAESKey, 8 * sizeof(kAESKey), &aes_key) == 0);
  CALL_SERVICE_AND_CHECK_APPROVED(approved,
      ASSERT_TRUE(AES_wrap_key_padded(&aes_key, output, &outlen, sizeof(output),
                                      kPlaintext, sizeof(kPlaintext))));
  EXPECT_EQ(Bytes(kAESKWPCiphertext), Bytes(output, outlen));
  EXPECT_EQ(approved, AWSLC_APPROVED);

  // AES-KWP Decryption KAT
  ASSERT_TRUE(AES_set_decrypt_key(kAESKey, 8 * sizeof(kAESKey), &aes_key) == 0);
  CALL_SERVICE_AND_CHECK_APPROVED(approved,
      ASSERT_TRUE(AES_unwrap_key_padded(&aes_key, output, &outlen,
                                        sizeof(output), kAESKWPCiphertext,
                                        sizeof(kAESKWPCiphertext))));
  EXPECT_EQ(Bytes(kPlaintext), Bytes(output, outlen));
  EXPECT_EQ(approved, AWSLC_APPROVED);
}

TEST(ServiceIndicatorTest, AESXTS) {
  FIPSStatus approved = AWSLC_NOT_APPROVED;
  std::vector<uint8_t> key(
      kAESXTSKey_256,
      kAESXTSKey_256 + sizeof(kAESXTSKey_256));
  std::vector<uint8_t> iv(
      kAESXTSIV_256,
      kAESXTSIV_256 + sizeof(kAESXTSIV_256));
  std::vector<uint8_t> plaintext(
      kAESXTSPlaintext_256,
      kAESXTSPlaintext_256 + sizeof(kAESXTSPlaintext_256));
  std::vector<uint8_t> ciphertext(
      kAESXTSCiphertext_256,
      kAESXTSCiphertext_256 + sizeof(kAESXTSCiphertext_256));
  bssl::ScopedEVP_CIPHER_CTX ctx;

  CALL_SERVICE_AND_CHECK_APPROVED(approved,
    ASSERT_TRUE(EVP_CipherInit_ex(ctx.get(), EVP_aes_256_xts(), nullptr,
                                key.data(), iv.data(), 1)));
  EXPECT_EQ(approved, AWSLC_NOT_APPROVED);
  ASSERT_LE(EVP_CIPHER_CTX_iv_length(ctx.get()), iv.size());

  ASSERT_TRUE(EVP_CIPHER_CTX_set_key_length(ctx.get(), key.size()));
  CALL_SERVICE_AND_CHECK_APPROVED(approved,
    ASSERT_TRUE(EVP_CipherInit_ex(ctx.get(), EVP_aes_256_xts(), nullptr,
                                key.data(), iv.data(), 1)));
  EXPECT_EQ(approved, AWSLC_NOT_APPROVED);
  ASSERT_TRUE(EVP_CIPHER_CTX_set_padding(ctx.get(), 0));
  std::vector<uint8_t> encrypt_result;

  size_t max_out = plaintext.size();
  unsigned block_size = EVP_CIPHER_CTX_block_size(ctx.get());
  max_out += block_size - (max_out % block_size);
  encrypt_result.resize(max_out);
  size_t total = 0;
  int len = 0;

  // Result should be fully encrypted during |EVP_CipherUpdate| for AES-XTS.
  CALL_SERVICE_AND_CHECK_APPROVED(approved,
    EVP_CipherUpdate(ctx.get(), encrypt_result.data(), &len,
                          plaintext.data(), plaintext.size()));
  ASSERT_EQ(approved, AWSLC_NOT_APPROVED);
  total += static_cast<size_t>(len);
  encrypt_result.resize(total);
  EXPECT_EQ(Bytes(encrypt_result), Bytes(ciphertext));

  // Ensure |EVP_CipherFinal_ex| is a no-op, but only |*Final| functions
  // should indicate service indicator approval.
  CALL_SERVICE_AND_CHECK_APPROVED(approved,
    EVP_CipherFinal_ex(ctx.get(), encrypt_result.data() + total, &len));
  EXPECT_EQ(Bytes(encrypt_result), Bytes(ciphertext));
  EXPECT_EQ(0, len);

  ASSERT_EQ(approved, AWSLC_APPROVED);
}

TEST(ServiceIndicatorTest, FFDH) {
  FIPSStatus approved = AWSLC_NOT_APPROVED;

  // |DH_compute_key_padded| should be a non-approved service.
  bssl::UniquePtr<DH> dh(GetDH());
  uint8_t dh_out[sizeof(kDHOutput)];
  ASSERT_EQ(DH_size(dh.get()), static_cast<int>(sizeof(dh_out)));
  CALL_SERVICE_AND_CHECK_APPROVED(approved, ASSERT_EQ(DH_compute_key_padded(
                          dh_out, DH_get0_priv_key(dh.get()), dh.get()),
                          static_cast<int>(sizeof(dh_out))));
  EXPECT_EQ(Bytes(kDHOutput), Bytes(dh_out));
  EXPECT_EQ(approved, AWSLC_NOT_APPROVED);
}

TEST(ServiceIndicatorTest, DRBG) {
  FIPSStatus approved = AWSLC_NOT_APPROVED;
  CTR_DRBG_STATE drbg;
  uint8_t output[sizeof(kDRBGOutput)];

  // Test running the DRBG interfaces and check |CTR_DRBG_generate| for approval
  // at the end since it indicates a service is being done. |CTR_DRBG_init| and
  // |CTR_DRBG_reseed| should not be approved, because the functions do not
  // indicate that a service has been fully completed yet.
  CALL_SERVICE_AND_CHECK_APPROVED(approved,
      ASSERT_TRUE(CTR_DRBG_init(&drbg, kDRBGEntropy, kDRBGPersonalization,
                                sizeof(kDRBGPersonalization))));
  EXPECT_EQ(approved, AWSLC_NOT_APPROVED);
  CALL_SERVICE_AND_CHECK_APPROVED(approved,
      ASSERT_TRUE(CTR_DRBG_generate(&drbg, output, sizeof(kDRBGOutput), kDRBGAD,
                                    sizeof(kDRBGAD))));
  EXPECT_EQ(approved, AWSLC_APPROVED);
  EXPECT_EQ(Bytes(kDRBGOutput), Bytes(output));

  CALL_SERVICE_AND_CHECK_APPROVED(approved,
      ASSERT_TRUE(CTR_DRBG_reseed(&drbg, kDRBGEntropy2, kDRBGAD,
                                  sizeof(kDRBGAD))));
  EXPECT_EQ(approved, AWSLC_NOT_APPROVED);
  CALL_SERVICE_AND_CHECK_APPROVED(approved,
      ASSERT_TRUE(CTR_DRBG_generate(&drbg, output, sizeof(kDRBGReseedOutput),
                                    kDRBGAD, sizeof(kDRBGAD))));
  EXPECT_EQ(approved, AWSLC_APPROVED);
  EXPECT_EQ(Bytes(kDRBGReseedOutput), Bytes(output));

  CALL_SERVICE_AND_CHECK_APPROVED(
      approved, ASSERT_TRUE(RAND_bytes(output, sizeof(output))));
  EXPECT_EQ(approved, AWSLC_APPROVED);
}

static const struct SSKDFDigestTestVector {
  const EVP_MD *(*md)();
  const FIPSStatus expectation;
} kSSKDFDigestTestVectors[] = {{
                                   &EVP_sha1,
                                   AWSLC_APPROVED,
                               },
                               {
                                   &EVP_sha224,
                                   AWSLC_APPROVED,
                               },
                               {
                                   &EVP_sha256,
                                   AWSLC_APPROVED,
                               },
                               {
                                   &EVP_sha384,
                                   AWSLC_APPROVED,
                               },
                               {
                                   &EVP_sha512,
                                   AWSLC_APPROVED,
                               },
                               {
                                   &EVP_sha512_224,
                                   AWSLC_APPROVED,
                               },
                               {
                                   &EVP_sha512_256,
                                   AWSLC_APPROVED,
                               },
                               {
                                   &EVP_sha3_224,
                                   AWSLC_APPROVED,
                               },
                               {
                                   &EVP_sha3_256,
                                   AWSLC_APPROVED,
                               },
                               {
                                   &EVP_sha3_384,
                                   AWSLC_APPROVED,
                               },
                               {
                                   &EVP_sha3_512,
                                   AWSLC_APPROVED,
                               },
                               {
                                &EVP_md5,
                                AWSLC_NOT_APPROVED,
                               }};

class SSKDFDigestIndicatorTest : public TestWithNoErrors<SSKDFDigestTestVector> {};

INSTANTIATE_TEST_SUITE_P(All, SSKDFDigestIndicatorTest,
                         testing::ValuesIn(kSSKDFDigestTestVectors));


TEST_P(SSKDFDigestIndicatorTest, SSKDF) {
  const SSKDFDigestTestVector &vector = GetParam();

  const uint8_t secret[23] = {'A', 'W', 'S', '-', 'L', 'C', ' ', 'S',
                              'S', 'K', 'D', 'F', '-', 'D', 'I', 'G',
                              'E', 'S', 'T', ' ', 'K', 'E', 'Y'};
  const uint8_t info[19] = {'A', 'W', 'S', '-', 'L', 'C', ' ', 'S', 'S', 'K',
                            'D', 'F', '-', 'D', 'I', 'G', 'E', 'S', 'T'};
  uint8_t output[16] = {0};

  FIPSStatus approved = AWSLC_NOT_APPROVED;

  CALL_SERVICE_AND_CHECK_APPROVED(
      approved, ASSERT_TRUE(SSKDF_digest(
                    &output[0], sizeof(output), vector.md(), &secret[0],
                    sizeof(secret), &info[0], sizeof(info))));
  ASSERT_EQ(vector.expectation, approved);
}

static const struct SSKDFHmacTestVector {
  const EVP_MD *(*md)();
  const FIPSStatus expectation;
} kSSKDFHmacTestVectors[] = {{
                                 &EVP_sha1,
                                 AWSLC_APPROVED,
                             },
                             {
                                 &EVP_sha224,
                                 AWSLC_APPROVED,
                             },
                             {
                                 &EVP_sha256,
                                 AWSLC_APPROVED,
                             },
                             {
                                 &EVP_sha384,
                                 AWSLC_APPROVED,
                             },
                             {
                                 &EVP_sha512,
                                 AWSLC_APPROVED,
                             },
                             {
                                 &EVP_sha512_224,
                                 AWSLC_APPROVED,
                             },
                             {
                                 &EVP_sha512_256,
                                 AWSLC_APPROVED,
                             },
                             {
                                 &EVP_md5,
                                 AWSLC_NOT_APPROVED,
                             }};

class SSKDFHmacIndicatorTest : public TestWithNoErrors<SSKDFHmacTestVector> {};

INSTANTIATE_TEST_SUITE_P(All, SSKDFHmacIndicatorTest,
                         testing::ValuesIn(kSSKDFHmacTestVectors));


TEST_P(SSKDFHmacIndicatorTest, SSKDF) {
  const SSKDFHmacTestVector &vector = GetParam();

  const uint8_t secret[21] = {'A', 'W', 'S', '-', 'L', 'C', ' ',
                              'S', 'S', 'K', 'D', 'F', '-', 'H',
                              'M', 'A', 'C', ' ', 'K', 'E', 'Y'};
  const uint8_t info[17] = {'A', 'W', 'S', '-', 'L', 'C', ' ', 'S', 'S', 'K',
                            'D', 'F', '-', 'H', 'M', 'A', 'C'};
  const uint8_t salt[22] = {'A', 'W', 'S', '-', 'L', 'C', ' ', 'S',
                            'S', 'K', 'D', 'F', '-', 'H', 'M', 'A',
                            'C', ' ', 'S', 'A', 'L', 'T'};
  uint8_t output[16] = {0};

  FIPSStatus approved = AWSLC_NOT_APPROVED;

  CALL_SERVICE_AND_CHECK_APPROVED(
      approved, ASSERT_TRUE(SSKDF_hmac(&output[0], sizeof(output), vector.md(),
                                       &secret[0], sizeof(secret), &info[0],
                                       sizeof(info), &salt[0], sizeof(salt))));
  ASSERT_EQ(vector.expectation, approved);
}

static const struct KBKDFCtrHmacTestVector {
  const EVP_MD *(*md)();
  const FIPSStatus expectation;
} kKBKDFCtrHmacTestVectors[] = {{
                                    &EVP_sha1,
                                    AWSLC_APPROVED,
                                },
                                {
                                    &EVP_sha224,
                                    AWSLC_APPROVED,
                                },
                                {
                                    &EVP_sha256,
                                    AWSLC_APPROVED,
                                },
                                {
                                    &EVP_sha384,
                                    AWSLC_APPROVED,
                                },
                                {
                                    &EVP_sha512,
                                    AWSLC_APPROVED,
                                },
                                {
                                    &EVP_sha512_224,
                                    AWSLC_APPROVED,
                                },
                                {
                                    &EVP_sha512_256,
                                    AWSLC_APPROVED,
                                },
                                {
                                    &EVP_md5,
                                    AWSLC_NOT_APPROVED,
                                }};

class KBKDFCtrHmacIndicatorTest : public TestWithNoErrors<KBKDFCtrHmacTestVector> {};

INSTANTIATE_TEST_SUITE_P(All, KBKDFCtrHmacIndicatorTest,
                         testing::ValuesIn(kKBKDFCtrHmacTestVectors));

TEST_P(KBKDFCtrHmacIndicatorTest, KBKDF) {
  const KBKDFCtrHmacTestVector &vector = GetParam();

  // 14 bytes (112 bits) is the minimum key-derivation key that would be
  // approved if using an approved algorithm;
  const uint8_t secret[14] = {'K', 'B', 'K', 'D', 'F', '-', 'C',
                              'T', 'R', ' ', ' ', 'K', 'E', 'Y'};
  const uint8_t info[16] = {'A', 'W', 'S', '-', 'L', 'C', ' ', 'K',
                            'B', 'K', 'D', 'F', '-', 'C', 'T', 'R'};
  uint8_t output[16] = {0};

  FIPSStatus approved = AWSLC_NOT_APPROVED;

  CALL_SERVICE_AND_CHECK_APPROVED(
      approved, ASSERT_TRUE(KBKDF_ctr_hmac(
                    &output[0], sizeof(output), vector.md(), &secret[0],
                    sizeof(secret), &info[0], sizeof(info))));
  ASSERT_EQ(vector.expectation, approved);

  // Pass a secret key (key-derivation key) size of 13 bytes (104 bits)
  // regardless of algorithm, this is less then 112 bits minimum
  // required by SP 800-131Ar1, Section 8. So indicator should reflect
  // this being unapproved.
  CALL_SERVICE_AND_CHECK_APPROVED(
      approved,
      ASSERT_TRUE(KBKDF_ctr_hmac(&output[0], sizeof(output), vector.md(),
                                 &secret[0], 13, &info[0], sizeof(info))));
  ASSERT_EQ(AWSLC_NOT_APPROVED, approved);
}

TEST(ServiceIndicatorTest, ML_KEM) {
  for (int nid : {NID_MLKEM512, NID_MLKEM768, NID_MLKEM1024}) {
    bssl::UniquePtr<EVP_PKEY_CTX> ctx(
        EVP_PKEY_CTX_new_id(EVP_PKEY_KEM, nullptr));
    ASSERT_TRUE(EVP_PKEY_CTX_kem_set_params(ctx.get(), nid));
    ASSERT_TRUE(EVP_PKEY_keygen_init(ctx.get()));

    FIPSStatus approved = AWSLC_NOT_APPROVED;
    EVP_PKEY *raw = nullptr;
    // keygen for ML-KEM algorithms should be approved
    CALL_SERVICE_AND_CHECK_APPROVED(approved, EVP_PKEY_keygen(ctx.get(), &raw));
    bssl::UniquePtr<EVP_PKEY> pkey(raw);
    ASSERT_EQ(approved, AWSLC_APPROVED);

    size_t ciphertext_len = 0;
    size_t shared_secret_len = 0;

    ctx.reset(EVP_PKEY_CTX_new(pkey.get(), nullptr));

    approved = AWSLC_NOT_APPROVED;
    // encapsulate size check should not set indicator
    CALL_SERVICE_AND_CHECK_APPROVED(
        approved, EVP_PKEY_encapsulate(ctx.get(), nullptr, &ciphertext_len,
                                       nullptr, &shared_secret_len));
    ASSERT_EQ(approved, AWSLC_NOT_APPROVED);

    std::vector<uint8_t> ciphertext(ciphertext_len);
    std::vector<uint8_t> encap_shared_secret(shared_secret_len);

    // encapsulate should set indicator
    CALL_SERVICE_AND_CHECK_APPROVED(
        approved,
        EVP_PKEY_encapsulate(ctx.get(), ciphertext.data(), &ciphertext_len,
                             encap_shared_secret.data(), &shared_secret_len));
    ASSERT_EQ(approved, AWSLC_APPROVED);

    shared_secret_len = 0;
    approved = AWSLC_NOT_APPROVED;
    // decapsulate size check should not set indicator
    CALL_SERVICE_AND_CHECK_APPROVED(
        approved, EVP_PKEY_decapsulate(ctx.get(), nullptr, &shared_secret_len,
                                       ciphertext.data(), ciphertext.size()));
    ASSERT_EQ(approved, AWSLC_NOT_APPROVED);

    std::vector<uint8_t> decap_shared_secret(shared_secret_len);
    // decapsulate should set indicator
    CALL_SERVICE_AND_CHECK_APPROVED(
        approved, EVP_PKEY_decapsulate(ctx.get(), decap_shared_secret.data(),
                                       &shared_secret_len, ciphertext.data(),
                                       ciphertext.size()));
    ASSERT_EQ(approved, AWSLC_APPROVED);
    ASSERT_EQ(encap_shared_secret, decap_shared_secret);
  }
}

TEST(ServiceIndicatorTest, ED25519KeyGen) {
  FIPSStatus approved = AWSLC_NOT_APPROVED;
  uint8_t private_key[ED25519_PRIVATE_KEY_LEN] = {0};
  uint8_t public_key[ED25519_PUBLIC_KEY_LEN] = {0};
  CALL_SERVICE_AND_CHECK_APPROVED(approved,
                                  ED25519_keypair(public_key, private_key));
  ASSERT_EQ(AWSLC_APPROVED, approved);

  approved = AWSLC_NOT_APPROVED;

  bssl::UniquePtr<EVP_PKEY_CTX> ctx(
      EVP_PKEY_CTX_new_id(EVP_PKEY_ED25519, nullptr));
  EVP_PKEY *raw = nullptr;
  bssl::UniquePtr<EVP_PKEY> pkey(raw);
  ASSERT_TRUE(EVP_PKEY_keygen_init(ctx.get()));
  CALL_SERVICE_AND_CHECK_APPROVED(
      approved, ASSERT_TRUE(EVP_PKEY_keygen(ctx.get(), &raw)));
  ASSERT_EQ(AWSLC_APPROVED, approved);
  pkey.reset(raw);
}

TEST(ServiceIndicatorTest, ED25519SigGenVerify) {
  const uint8_t MESSAGE[15] = {'E', 'D', '2', '5', '5', '1', '9', ' ',
                               'M', 'E', 'S', 'S', 'A', 'G', 'E'};
  const uint8_t CONTEXT[6] = {'A', 'W', 'S', '-', 'L', 'C'};
  uint8_t digest[SHA512_DIGEST_LENGTH] = {
      0xcf, 0x83, 0xe1, 0x35, 0x7e, 0xef, 0xb8, 0xbd, 0xf1, 0x54, 0x28,
      0x50, 0xd6, 0x6d, 0x80, 0x07, 0xd6, 0x20, 0xe4, 0x05, 0x0b, 0x57,
      0x15, 0xdc, 0x83, 0xf4, 0xa9, 0x21, 0xd3, 0x6c, 0xe9, 0xce, 0x47,
      0xd0, 0xd1, 0x3c, 0x5d, 0x85, 0xf2, 0xb0, 0xff, 0x83, 0x18, 0xd2,
      0x87, 0x7e, 0xec, 0x2f, 0x63, 0xb9, 0x31, 0xbd, 0x47, 0x41, 0x7a,
      0x81, 0xa5, 0x38, 0x32, 0x7a, 0xf9, 0x27, 0xda, 0x3e};  // sha512 of empty
                                                              // string
  uint8_t private_key[ED25519_PRIVATE_KEY_LEN] = {0};
  uint8_t public_key[ED25519_PUBLIC_KEY_LEN] = {0};
  uint8_t signature[ED25519_SIGNATURE_LEN] = {0};
  ED25519_keypair(public_key, private_key);

  FIPSStatus approved = AWSLC_NOT_APPROVED;
  CALL_SERVICE_AND_CHECK_APPROVED(
      approved, ASSERT_TRUE(ED25519_sign(&signature[0], &MESSAGE[0],
                                         sizeof(MESSAGE), private_key)));
  ASSERT_EQ(AWSLC_APPROVED, approved);

  approved = AWSLC_NOT_APPROVED;
  CALL_SERVICE_AND_CHECK_APPROVED(
      approved, ASSERT_TRUE(ED25519_verify(&MESSAGE[0], sizeof(MESSAGE),
                                           signature, public_key)));
  ASSERT_EQ(AWSLC_APPROVED, approved);

  approved = AWSLC_NOT_APPROVED;
  CALL_SERVICE_AND_CHECK_APPROVED(
      approved,
      ASSERT_TRUE(ED25519ctx_sign(&signature[0], &MESSAGE[0], sizeof(MESSAGE),
                                  private_key, &CONTEXT[0], sizeof(CONTEXT))));
  ASSERT_EQ(AWSLC_NOT_APPROVED, approved);

  approved = AWSLC_NOT_APPROVED;
  CALL_SERVICE_AND_CHECK_APPROVED(
      approved,
      ASSERT_TRUE(ED25519ctx_verify(&MESSAGE[0], sizeof(MESSAGE), signature,
                                    public_key, &CONTEXT[0], sizeof(CONTEXT))));
  ASSERT_EQ(AWSLC_NOT_APPROVED, approved);

  approved = AWSLC_NOT_APPROVED;
  CALL_SERVICE_AND_CHECK_APPROVED(
      approved,
      ASSERT_TRUE(ED25519ph_sign(&signature[0], &MESSAGE[0], sizeof(MESSAGE),
                                 private_key, &CONTEXT[0], sizeof(CONTEXT))));
  ASSERT_EQ(AWSLC_APPROVED, approved);

  approved = AWSLC_NOT_APPROVED;
  CALL_SERVICE_AND_CHECK_APPROVED(
      approved,
      ASSERT_TRUE(ED25519ph_verify(&MESSAGE[0], sizeof(MESSAGE), signature,
                                   public_key, &CONTEXT[0], sizeof(CONTEXT))));
  ASSERT_EQ(AWSLC_APPROVED, approved);

  approved = AWSLC_NOT_APPROVED;
  CALL_SERVICE_AND_CHECK_APPROVED(
      approved,
      ASSERT_TRUE(ED25519ph_sign_digest(&signature[0], digest,
                                 private_key, &CONTEXT[0], sizeof(CONTEXT))));
  ASSERT_EQ(AWSLC_APPROVED, approved);

  approved = AWSLC_NOT_APPROVED;
  CALL_SERVICE_AND_CHECK_APPROVED(approved, ASSERT_TRUE(ED25519ph_verify_digest(
                                                digest, signature, public_key,
                                                &CONTEXT[0], sizeof(CONTEXT))));
  ASSERT_EQ(AWSLC_APPROVED, approved);

  bssl::UniquePtr<EVP_PKEY> pkey(EVP_PKEY_new_raw_private_key(
      EVP_PKEY_ED25519, NULL, &private_key[0], ED25519_PRIVATE_KEY_SEED_LEN));

  bssl::UniquePtr<EVP_MD_CTX> mdctx(EVP_MD_CTX_new());
  CALL_SERVICE_AND_CHECK_APPROVED(
      approved, EVP_DigestSignInit(mdctx.get(), NULL, NULL, NULL, pkey.get()));
  ASSERT_EQ(AWSLC_NOT_APPROVED, approved);
  size_t sig_out_len = sizeof(signature);
  CALL_SERVICE_AND_CHECK_APPROVED(
      approved,
      ASSERT_TRUE(EVP_DigestSign(mdctx.get(), &signature[0], &sig_out_len,
                                 &MESSAGE[0], sizeof(MESSAGE))));
  ASSERT_EQ(AWSLC_APPROVED, approved);
  ASSERT_EQ(sizeof(signature), sig_out_len);

  mdctx.reset(EVP_MD_CTX_new());
  ASSERT_TRUE(EVP_DigestVerifyInit(mdctx.get(), NULL, NULL, NULL, pkey.get()));
  approved = AWSLC_NOT_APPROVED;
  CALL_SERVICE_AND_CHECK_APPROVED(
      approved, ASSERT_TRUE(EVP_DigestVerify(mdctx.get(), &signature[0],
                                             sizeof(signature), &MESSAGE[0],
                                             sizeof(MESSAGE))));
  ASSERT_EQ(AWSLC_APPROVED, approved);
}

TEST(ServiceIndicatorTest, MLDSAKeyGen) {
  // Test EVP interface for each ML-DSA parameter set
  for (int nid : {NID_MLDSA44, NID_MLDSA65, NID_MLDSA87}) {
    bssl::UniquePtr<EVP_PKEY_CTX> ctx(
        EVP_PKEY_CTX_new_id(EVP_PKEY_PQDSA, nullptr));
    ASSERT_TRUE(EVP_PKEY_CTX_pqdsa_set_params(ctx.get(), nid));
    ASSERT_TRUE(EVP_PKEY_keygen_init(ctx.get()));

    FIPSStatus approved = AWSLC_NOT_APPROVED;
    EVP_PKEY *raw = nullptr;
    CALL_SERVICE_AND_CHECK_APPROVED(
        approved, EVP_PKEY_keygen(ctx.get(), &raw));
    bssl::UniquePtr<EVP_PKEY> pkey(raw);
    ASSERT_EQ(AWSLC_APPROVED, approved);
  }
}

TEST(ServiceIndicatorTest, MLDSASigGenVerify) {
  const uint8_t MESSAGE[15] = {'M', 'L', '-', 'D', 'S', 'A', ' ',
                               'M', 'E', 'S', 'S', 'A', 'G', 'E'};

  // Test EVP interface for all ML-DSA parameter sets
  for (int nid : {NID_MLDSA44, NID_MLDSA65, NID_MLDSA87}) {
    bssl::UniquePtr<EVP_PKEY_CTX> ctx(
        EVP_PKEY_CTX_new_id(EVP_PKEY_PQDSA, nullptr));
    ASSERT_TRUE(EVP_PKEY_CTX_pqdsa_set_params(ctx.get(), nid));
    ASSERT_TRUE(EVP_PKEY_keygen_init(ctx.get()));

    EVP_PKEY *raw = nullptr;
    ASSERT_TRUE(EVP_PKEY_keygen(ctx.get(), &raw));
    bssl::UniquePtr<EVP_PKEY> pkey(raw);

    // Test Method 1: EVP_DigestSign/EVP_DigestVerify (pure signatures)
    {
      bssl::UniquePtr<EVP_MD_CTX> mdctx(EVP_MD_CTX_new());
      FIPSStatus approved = AWSLC_NOT_APPROVED;
      
      CALL_SERVICE_AND_CHECK_APPROVED(
          approved, EVP_DigestSignInit(mdctx.get(), NULL, NULL, NULL, pkey.get()));
      ASSERT_EQ(AWSLC_NOT_APPROVED, approved);
      
      size_t sig_out_len = 0;
      CALL_SERVICE_AND_CHECK_APPROVED(
          approved,
          ASSERT_TRUE(EVP_DigestSign(mdctx.get(), nullptr, &sig_out_len,
                                     MESSAGE, sizeof(MESSAGE))));
      ASSERT_EQ(AWSLC_NOT_APPROVED, approved);

      std::vector<uint8_t> signature(sig_out_len);
      CALL_SERVICE_AND_CHECK_APPROVED(
          approved,
          ASSERT_TRUE(EVP_DigestSign(mdctx.get(), signature.data(), &sig_out_len,
                                     MESSAGE, sizeof(MESSAGE))));
      ASSERT_EQ(AWSLC_APPROVED, approved);

      mdctx.reset(EVP_MD_CTX_new());
      ASSERT_TRUE(EVP_DigestVerifyInit(mdctx.get(), NULL, NULL, NULL, pkey.get()));
      approved = AWSLC_NOT_APPROVED;
      CALL_SERVICE_AND_CHECK_APPROVED(
          approved, ASSERT_TRUE(EVP_DigestVerify(mdctx.get(), signature.data(),
                                                 sig_out_len, MESSAGE,
                                                 sizeof(MESSAGE))));
      ASSERT_EQ(AWSLC_APPROVED, approved);
    }

    // Test Method 2: EVP_PKEY_sign/EVP_PKEY_verify (pre-hash signatures)
    {
      // Compute mu for this EVP key
      size_t pk_len = 0;
      ASSERT_TRUE(EVP_PKEY_get_raw_public_key(pkey.get(), nullptr, &pk_len));
      std::vector<uint8_t> evp_pk(pk_len);
      ASSERT_TRUE(EVP_PKEY_get_raw_public_key(pkey.get(), evp_pk.data(), &pk_len));
      
      std::vector<uint8_t> evp_mu(64);
      std::vector<uint8_t> evp_tr(64);
      
      // Compute tr = SHAKE256(pk, 64)
      bssl::ScopedEVP_MD_CTX evp_md_ctx_pk;
      ASSERT_TRUE(EVP_DigestInit_ex(evp_md_ctx_pk.get(), EVP_shake256(), nullptr));
      ASSERT_TRUE(EVP_DigestUpdate(evp_md_ctx_pk.get(), evp_pk.data(), pk_len));
      ASSERT_TRUE(EVP_DigestFinalXOF(evp_md_ctx_pk.get(), evp_tr.data(), 64));

      // Compute mu = SHAKE256(tr || pre || MESSAGE, 64)
      uint8_t evp_pre[2] = {0x00, 0x00};
      bssl::ScopedEVP_MD_CTX evp_md_ctx_mu;
      ASSERT_TRUE(EVP_DigestInit_ex(evp_md_ctx_mu.get(), EVP_shake256(), nullptr));
      ASSERT_TRUE(EVP_DigestUpdate(evp_md_ctx_mu.get(), evp_tr.data(), 64));
      ASSERT_TRUE(EVP_DigestUpdate(evp_md_ctx_mu.get(), evp_pre, 2));
      ASSERT_TRUE(EVP_DigestUpdate(evp_md_ctx_mu.get(), MESSAGE, sizeof(MESSAGE)));
      ASSERT_TRUE(EVP_DigestFinalXOF(evp_md_ctx_mu.get(), evp_mu.data(), 64));

      bssl::UniquePtr<EVP_PKEY_CTX> sign_ctx(EVP_PKEY_CTX_new(pkey.get(), nullptr));
      FIPSStatus approved = AWSLC_NOT_APPROVED;
      
      CALL_SERVICE_AND_CHECK_APPROVED(
          approved, ASSERT_TRUE(EVP_PKEY_sign_init(sign_ctx.get())));
      ASSERT_EQ(AWSLC_NOT_APPROVED, approved);
      
      size_t sig_len = 0;
      CALL_SERVICE_AND_CHECK_APPROVED(
          approved, ASSERT_TRUE(EVP_PKEY_sign(sign_ctx.get(), nullptr, &sig_len,
                                              evp_mu.data(), evp_mu.size())));
      ASSERT_EQ(AWSLC_NOT_APPROVED, approved);

      std::vector<uint8_t> signature2(sig_len);
      CALL_SERVICE_AND_CHECK_APPROVED(
          approved, ASSERT_TRUE(EVP_PKEY_sign(sign_ctx.get(), signature2.data(), &sig_len,
                                              evp_mu.data(), evp_mu.size())));
      ASSERT_EQ(AWSLC_APPROVED, approved);

      CALL_SERVICE_AND_CHECK_APPROVED(
          approved, ASSERT_TRUE(EVP_PKEY_verify_init(sign_ctx.get())));
      ASSERT_EQ(AWSLC_NOT_APPROVED, approved);
      
      CALL_SERVICE_AND_CHECK_APPROVED(
          approved, ASSERT_TRUE(EVP_PKEY_verify(sign_ctx.get(), signature2.data(), sig_len,
                                                evp_mu.data(), evp_mu.size())));
      ASSERT_EQ(AWSLC_APPROVED, approved);
    }
  }
}

// Verifies that the awslc_version_string is as expected.
// Since this is running in FIPS mode it should end in FIPS
// Update this when the AWS-LC version number is modified
TEST(ServiceIndicatorTest, AWSLCVersionString) {
  ASSERT_STREQ(awslc_version_string(), "AWS-LC FIPS " AWSLC_VERSION_NUMBER_STRING);
}

#else
// Service indicator calls should not be used in non-FIPS builds. However, if
// used, the macro |CALL_SERVICE_AND_CHECK_APPROVED| will return
// |AWSLC_APPROVED|, but the direct calls to |FIPS_service_indicator_xxx|
// will not indicate an approved state.
TEST(ServiceIndicatorTest, BasicTest) {
   // Reset and check the initial state and counter.
  FIPSStatus approved = AWSLC_NOT_APPROVED;
  uint64_t before = FIPS_service_indicator_before_call();
  ASSERT_EQ(before, (uint64_t)0);

  // Call an approved service.
  bssl::ScopedEVP_AEAD_CTX aead_ctx;
  uint8_t nonce[EVP_AEAD_MAX_NONCE_LENGTH] = {0};
  uint8_t output[256];
  size_t out_len;
  ASSERT_TRUE(EVP_AEAD_CTX_init(aead_ctx.get(),
                                EVP_aead_aes_128_gcm_randnonce(), kAESKey,
                                sizeof(kAESKey), 0, nullptr));
  CALL_SERVICE_AND_CHECK_APPROVED(approved,
      EVP_AEAD_CTX_seal(aead_ctx.get(), output, &out_len, sizeof(output),
                      nullptr, 0, kPlaintext, sizeof(kPlaintext), nullptr, 0));
  // Macro should return true, to ensure FIPS/Non-FIPS compatibility.
  ASSERT_EQ(approved, AWSLC_APPROVED);

  // Call a non-approved service.
  ASSERT_TRUE(EVP_AEAD_CTX_init(aead_ctx.get(), EVP_aead_aes_128_gcm(),
                                kAESKey, sizeof(kAESKey), 0, nullptr));
  CALL_SERVICE_AND_CHECK_APPROVED(approved,
      EVP_AEAD_CTX_seal(aead_ctx.get(), output, &out_len, sizeof(output), nonce,
          EVP_AEAD_nonce_length(EVP_aead_aes_128_gcm()), kPlaintext,
          sizeof(kPlaintext), nullptr, 0));
  ASSERT_EQ(approved, AWSLC_APPROVED);
}


// Verifies that the awslc_version_string is as expected.
// Since this is not running in FIPS mode it shouldn't end in FIPS
// Update this when the AWS-LC version number is modified
TEST(ServiceIndicatorTest, AWSLCVersionString) {
  ASSERT_STREQ(awslc_version_string(), "AWS-LC " AWSLC_VERSION_NUMBER_STRING);
}
#endif // AWSLC_FIPS
