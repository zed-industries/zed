// Copyright (c) 2020, Google Inc.
// SPDX-License-Identifier: ISC

#include <assert.h>
#include <stdio.h>
#include <string.h>
#include <time.h>

#include <algorithm>
#include <limits>
#include <string>
#include <tuple>
#include <utility>
#include <vector>

#include <gtest/gtest.h>

#include <openssl/bytestring.h>
#include <openssl/curve25519.h>
#include <openssl/evp.h>
#include <openssl/mem.h>
#include <openssl/rand.h>
#include <openssl/sha.h>
#include <openssl/trust_token.h>

#include "../ec_extra/internal.h"
#include "../fipsmodule/ec/internal.h"
#include "../internal.h"
#include "../test/test_util.h"
#include "internal.h"


BSSL_NAMESPACE_BEGIN

namespace {

const uint8_t kMessage[] = "MSG";

TEST(TrustTokenTest, KeyGenExp1) {
  uint8_t priv_key[TRUST_TOKEN_MAX_PRIVATE_KEY_SIZE];
  uint8_t pub_key[TRUST_TOKEN_MAX_PUBLIC_KEY_SIZE];
  size_t priv_key_len, pub_key_len;
  ASSERT_TRUE(TRUST_TOKEN_generate_key(
      TRUST_TOKEN_experiment_v1(), priv_key, &priv_key_len,
      TRUST_TOKEN_MAX_PRIVATE_KEY_SIZE, pub_key, &pub_key_len,
      TRUST_TOKEN_MAX_PUBLIC_KEY_SIZE, 0x0001));
  ASSERT_EQ(292u, priv_key_len);
  ASSERT_EQ(301u, pub_key_len);

  const uint8_t kKeygenSecret[] = "SEED";
  ASSERT_TRUE(TRUST_TOKEN_derive_key_from_secret(
      TRUST_TOKEN_experiment_v1(), priv_key, &priv_key_len,
      TRUST_TOKEN_MAX_PRIVATE_KEY_SIZE, pub_key, &pub_key_len,
      TRUST_TOKEN_MAX_PUBLIC_KEY_SIZE, 0x0001, kKeygenSecret,
      sizeof(kKeygenSecret) - 1));

  const uint8_t kExpectedPriv[] = {
      0x00, 0x00, 0x00, 0x01, 0x98, 0xaa, 0x32, 0xfc, 0x5f, 0x83, 0x35, 0xea,
      0x57, 0x4f, 0x9e, 0x61, 0x48, 0x6e, 0x89, 0x9d, 0x3d, 0xaa, 0x38, 0x5d,
      0xd0, 0x06, 0x96, 0x62, 0xe8, 0x0b, 0xd6, 0x5f, 0x12, 0xa4, 0xcc, 0xa9,
      0xb5, 0x20, 0x1b, 0x13, 0x8c, 0x1c, 0xaf, 0x36, 0x1b, 0xab, 0x0c, 0xc6,
      0xac, 0x38, 0xae, 0x96, 0x3d, 0x14, 0x9d, 0xb8, 0x8d, 0xf4, 0x7f, 0xe2,
      0x7d, 0xeb, 0x17, 0xc2, 0xbc, 0x63, 0x42, 0x93, 0x94, 0xe4, 0x97, 0xbf,
      0x97, 0xea, 0x02, 0x40, 0xac, 0xb6, 0xa5, 0x03, 0x4c, 0x6b, 0x4c, 0xb8,
      0x8c, 0xf4, 0x66, 0x1b, 0x4e, 0x02, 0x45, 0xf9, 0xcd, 0xb6, 0x0f, 0x59,
      0x09, 0x21, 0x03, 0x7e, 0x92, 0x1f, 0x3f, 0x40, 0x83, 0x50, 0xe3, 0xdc,
      0x9e, 0x6f, 0x65, 0xc5, 0xbd, 0x2c, 0x7d, 0xab, 0x74, 0x49, 0xc8, 0xa2,
      0x3c, 0xab, 0xcb, 0x4d, 0x63, 0x73, 0x81, 0x2b, 0xb2, 0x1e, 0x00, 0x8f,
      0x00, 0xb8, 0xd8, 0xb4, 0x5d, 0xc4, 0x3f, 0x3d, 0xa8, 0x4f, 0x4c, 0x72,
      0x0e, 0x20, 0x17, 0x4b, 0xac, 0x14, 0x8f, 0xb2, 0xa5, 0x20, 0x41, 0x2b,
      0xf7, 0x62, 0x25, 0x6a, 0xd6, 0x41, 0x26, 0x62, 0x10, 0xc1, 0xbc, 0x42,
      0xac, 0x54, 0x1b, 0x75, 0x05, 0xd6, 0x53, 0xb1, 0x7b, 0x84, 0x6a, 0x7b,
      0x5b, 0x2a, 0x34, 0x6e, 0x43, 0x4b, 0x43, 0xcc, 0x6c, 0xdb, 0x1d, 0x02,
      0x34, 0x7f, 0xd1, 0xe8, 0xfd, 0x42, 0x2c, 0xd9, 0x14, 0xdb, 0xd6, 0xf4,
      0xad, 0xb5, 0xe4, 0xac, 0xdd, 0x7e, 0xb5, 0x4c, 0x3f, 0x59, 0x24, 0xfa,
      0x04, 0xd9, 0xb6, 0xd2, 0xb7, 0x7d, 0xf1, 0xfa, 0x13, 0xc0, 0x4d, 0xd5,
      0xca, 0x3a, 0x4e, 0xa8, 0xdd, 0xa9, 0xfc, 0xcb, 0x06, 0xb2, 0xde, 0x4b,
      0x2a, 0x86, 0xbb, 0x0d, 0x41, 0xb6, 0x3d, 0xfb, 0x49, 0xc8, 0xdf, 0x9a,
      0x48, 0xe5, 0x68, 0x8a, 0xfc, 0x86, 0x9c, 0x79, 0x5a, 0x79, 0xc1, 0x09,
      0x33, 0x53, 0xdc, 0x3d, 0xe9, 0x93, 0x7c, 0x5b, 0x72, 0xf7, 0xa0, 0x8a,
      0x1f, 0x07, 0x6c, 0x38, 0x3c, 0x99, 0x0b, 0xe4, 0x4e, 0xa4, 0xbd, 0x41,
      0x1f, 0x83, 0xa6, 0xd3
  };
  ASSERT_EQ(Bytes(kExpectedPriv, sizeof(kExpectedPriv)),
            Bytes(priv_key, priv_key_len));

  const uint8_t kExpectedPub[] = {
      0x00, 0x00, 0x00, 0x01, 0x00, 0x61, 0x04, 0x5e, 0x06, 0x6b, 0x7b, 0xfd,
      0x54, 0x01, 0xe0, 0xd2, 0xb5, 0x12, 0xce, 0x48, 0x16, 0x66, 0xb2, 0xdf,
      0xfd, 0xa8, 0x38, 0x7c, 0x1f, 0x45, 0x1a, 0xb8, 0x21, 0x52, 0x17, 0x25,
      0xbb, 0x0b, 0x00, 0xd4, 0xa1, 0xbc, 0x28, 0xd9, 0x08, 0x36, 0x98, 0xb2,
      0x17, 0xd3, 0xb5, 0xad, 0xb6, 0x4e, 0x03, 0x5f, 0xd3, 0x66, 0x2c, 0x58,
      0x1c, 0xcc, 0xc6, 0x23, 0xa4, 0xf9, 0xa2, 0x7e, 0xb0, 0xe4, 0xd3, 0x95,
      0x41, 0x6f, 0xba, 0x23, 0x4a, 0x82, 0x93, 0x29, 0x73, 0x75, 0x38, 0x85,
      0x64, 0x9c, 0xaa, 0x12, 0x6d, 0x7d, 0xcd, 0x52, 0x02, 0x91, 0x9f, 0xa9,
      0xee, 0x4b, 0xfd, 0x68, 0x97, 0x40, 0xdc, 0x00, 0x61, 0x04, 0x14, 0x16,
      0x39, 0xf9, 0x63, 0x66, 0x94, 0x03, 0xfa, 0x0b, 0xbf, 0xca, 0x5a, 0x39,
      0x9f, 0x27, 0x5b, 0x3f, 0x69, 0x7a, 0xc9, 0xf7, 0x25, 0x7c, 0x84, 0x9e,
      0x1d, 0x61, 0x5a, 0x24, 0x53, 0xf2, 0x4a, 0x9d, 0xe9, 0x05, 0x53, 0xfd,
      0x12, 0x01, 0x2d, 0x9a, 0x69, 0x50, 0x74, 0x82, 0xa3, 0x45, 0x73, 0xdc,
      0x34, 0x36, 0x31, 0x44, 0x07, 0x0c, 0xda, 0x13, 0xbe, 0x94, 0x37, 0x65,
      0xa0, 0xab, 0x16, 0x52, 0x90, 0xe5, 0x8a, 0x03, 0xe5, 0x98, 0x79, 0x14,
      0x79, 0xd5, 0x17, 0xee, 0xd4, 0xb8, 0xda, 0x77, 0x76, 0x03, 0x20, 0x2a,
      0x7e, 0x3b, 0x76, 0x0b, 0x23, 0xb7, 0x72, 0x77, 0xb2, 0xeb, 0x00, 0x61,
      0x04, 0x68, 0x18, 0x4d, 0x23, 0x23, 0xf4, 0x45, 0xb8, 0x81, 0x0d, 0xa4,
      0x5d, 0x0b, 0x9e, 0x08, 0xfb, 0x45, 0xfb, 0x96, 0x29, 0x43, 0x2f, 0xab,
      0x93, 0x04, 0x4c, 0x04, 0xb6, 0x5e, 0x27, 0xf5, 0x39, 0x66, 0x94, 0x15,
      0x1d, 0xb1, 0x1c, 0x7c, 0x27, 0x6f, 0xa5, 0x19, 0x0c, 0x30, 0x12, 0xcc,
      0x77, 0x7f, 0x10, 0xa9, 0x7c, 0xe4, 0x08, 0x77, 0x3c, 0xd3, 0x6f, 0xa4,
      0xf4, 0xaf, 0xf1, 0x9d, 0x14, 0x1d, 0xd0, 0x02, 0x33, 0x50, 0x55, 0x00,
      0x6a, 0x47, 0x96, 0xe1, 0x8b, 0x4e, 0x44, 0x41, 0xad, 0xb3, 0xea, 0x0d,
      0x0d, 0xd5, 0x73, 0x8e, 0x62, 0x67, 0x8a, 0xb4, 0xe7, 0x5d, 0x17, 0xa9,
      0x24};
  ASSERT_EQ(Bytes(kExpectedPub, sizeof(kExpectedPub)),
            Bytes(pub_key, pub_key_len));
}

TEST(TrustTokenTest, KeyGenExp2VOPRF) {
  uint8_t priv_key[TRUST_TOKEN_MAX_PRIVATE_KEY_SIZE];
  uint8_t pub_key[TRUST_TOKEN_MAX_PUBLIC_KEY_SIZE];
  size_t priv_key_len, pub_key_len;
  ASSERT_TRUE(TRUST_TOKEN_generate_key(
      TRUST_TOKEN_experiment_v2_voprf(), priv_key, &priv_key_len,
      TRUST_TOKEN_MAX_PRIVATE_KEY_SIZE, pub_key, &pub_key_len,
      TRUST_TOKEN_MAX_PUBLIC_KEY_SIZE, 0x0001));
  ASSERT_EQ(52u, priv_key_len);
  ASSERT_EQ(101u, pub_key_len);

  const uint8_t kKeygenSecret[] = "SEED";
  ASSERT_TRUE(TRUST_TOKEN_derive_key_from_secret(
      TRUST_TOKEN_experiment_v2_voprf(), priv_key, &priv_key_len,
      TRUST_TOKEN_MAX_PRIVATE_KEY_SIZE, pub_key, &pub_key_len,
      TRUST_TOKEN_MAX_PUBLIC_KEY_SIZE, 0x0001, kKeygenSecret,
      sizeof(kKeygenSecret) - 1));

  const uint8_t kExpectedPriv[] = {
      0x00, 0x00, 0x00, 0x01, 0x0b, 0xe2, 0xc4, 0x73, 0x92, 0xe7, 0xf8,
      0x3e, 0xba, 0xab, 0x85, 0xa7, 0x77, 0xd7, 0x0a, 0x02, 0xc5, 0x36,
      0xfe, 0x62, 0xa3, 0xca, 0x01, 0x75, 0xc7, 0x62, 0x19, 0xc7, 0xf0,
      0x30, 0xc5, 0x14, 0x60, 0x13, 0x97, 0x4f, 0x63, 0x05, 0x37, 0x92,
      0x7b, 0x76, 0x8e, 0x9f, 0xd0, 0x1a, 0x74, 0x44
  };
  ASSERT_EQ(Bytes(kExpectedPriv, sizeof(kExpectedPriv)),
            Bytes(priv_key, priv_key_len));

  const uint8_t kExpectedPub[] = {
      0x00, 0x00, 0x00, 0x01, 0x04, 0x2c, 0x9c, 0x11, 0xc1, 0xe5, 0x52, 0x59,
      0x0b, 0x6d, 0x88, 0x8b, 0x6e, 0x28, 0xe8, 0xc5, 0xa3, 0xbe, 0x48, 0x18,
      0xf7, 0x1d, 0x31, 0xcf, 0xa2, 0x6e, 0x2a, 0xd6, 0xcb, 0x83, 0x26, 0x04,
      0xbd, 0x93, 0x67, 0xe4, 0x53, 0xf6, 0x11, 0x7d, 0x45, 0xe9, 0xfe, 0x27,
      0x33, 0x90, 0xdb, 0x1b, 0xfc, 0x9b, 0x31, 0x4d, 0x39, 0x1f, 0x1f, 0x8c,
      0x43, 0x06, 0x70, 0x2c, 0x84, 0xdc, 0x23, 0x18, 0xc7, 0x6a, 0x58, 0xcf,
      0x9e, 0xc1, 0xfa, 0xf2, 0x30, 0xdd, 0xad, 0x62, 0x24, 0xde, 0x11, 0xc1,
      0xba, 0x8d, 0xc3, 0x4f, 0xfb, 0xe5, 0xa5, 0xd4, 0x37, 0xba, 0x3b, 0x70,
      0xc0, 0xc3, 0xef, 0x20, 0x43
  };
  ASSERT_EQ(Bytes(kExpectedPub, sizeof(kExpectedPub)),
            Bytes(pub_key, pub_key_len));
}

TEST(TrustTokenTest, KeyGenExp2PMB) {
  uint8_t priv_key[TRUST_TOKEN_MAX_PRIVATE_KEY_SIZE];
  uint8_t pub_key[TRUST_TOKEN_MAX_PUBLIC_KEY_SIZE];
  size_t priv_key_len, pub_key_len;
  ASSERT_TRUE(TRUST_TOKEN_generate_key(
      TRUST_TOKEN_experiment_v2_pmb(), priv_key, &priv_key_len,
      TRUST_TOKEN_MAX_PRIVATE_KEY_SIZE, pub_key, &pub_key_len,
      TRUST_TOKEN_MAX_PUBLIC_KEY_SIZE, 0x0001));
  ASSERT_EQ(292u, priv_key_len);
  ASSERT_EQ(295u, pub_key_len);

  const uint8_t kKeygenSecret[] = "SEED";
  ASSERT_TRUE(TRUST_TOKEN_derive_key_from_secret(
      TRUST_TOKEN_experiment_v2_pmb(), priv_key, &priv_key_len,
      TRUST_TOKEN_MAX_PRIVATE_KEY_SIZE, pub_key, &pub_key_len,
      TRUST_TOKEN_MAX_PUBLIC_KEY_SIZE, 0x0001, kKeygenSecret,
      sizeof(kKeygenSecret) - 1));

  const uint8_t kExpectedPriv[] = {
      0x00, 0x00, 0x00, 0x01, 0x1b, 0x74, 0xdc, 0xf0, 0xa9, 0xa7, 0x6c, 0xfb,
      0x41, 0xef, 0xfa, 0x65, 0x52, 0xc9, 0x86, 0x4e, 0xfb, 0x16, 0x9d, 0xea,
      0x62, 0x3f, 0x47, 0xab, 0x1f, 0x1b, 0x05, 0xf2, 0x4f, 0x05, 0xfe, 0x64,
      0xb7, 0xe8, 0xcd, 0x2a, 0x10, 0xfa, 0xa2, 0x48, 0x3f, 0x0e, 0x8b, 0x94,
      0x39, 0xf1, 0xe7, 0x53, 0xe9, 0x50, 0x29, 0xe2, 0xb7, 0x0e, 0xc0, 0x94,
      0xa9, 0xd3, 0xef, 0x64, 0x10, 0x1d, 0x08, 0xd0, 0x60, 0xcb, 0x6d, 0x97,
      0x68, 0xc7, 0x04, 0x92, 0x07, 0xb2, 0x22, 0x83, 0xf7, 0xd9, 0x9b, 0x2c,
      0xf2, 0x52, 0x34, 0x0c, 0x42, 0x31, 0x47, 0x41, 0x19, 0xb9, 0xee, 0xfc,
      0x46, 0xbd, 0x14, 0xce, 0x42, 0xd7, 0x43, 0xc8, 0x32, 0x3b, 0x24, 0xed,
      0xdc, 0x69, 0xa3, 0x8e, 0x29, 0x01, 0xbe, 0xae, 0x24, 0x39, 0x14, 0xa7,
      0x52, 0xe5, 0xd5, 0xff, 0x9a, 0xc4, 0x15, 0x79, 0x29, 0x4c, 0x9b, 0x4e,
      0xfc, 0x61, 0xf2, 0x12, 0x6f, 0x4f, 0xd3, 0x96, 0x28, 0xb0, 0x79, 0xf0,
      0x4e, 0x6e, 0x7d, 0x56, 0x19, 0x1b, 0xc2, 0xd7, 0xf9, 0x3a, 0x58, 0x06,
      0xe5, 0xec, 0xa4, 0x33, 0x14, 0x1c, 0x78, 0x0c, 0x83, 0x94, 0x34, 0x22,
      0x5a, 0x8e, 0x2e, 0xa1, 0x72, 0x4a, 0x03, 0x35, 0xfe, 0x46, 0x92, 0x41,
      0x6b, 0xe6, 0x4b, 0x3f, 0xf0, 0xe7, 0x0b, 0xb5, 0xf3, 0x66, 0x6c, 0xc6,
      0x14, 0xcf, 0xce, 0x32, 0x0a, 0x2c, 0x28, 0xba, 0x4e, 0xb9, 0x75, 0x4a,
      0xa9, 0x2d, 0xb0, 0x8c, 0xd0, 0x62, 0x52, 0x29, 0x1f, 0x12, 0xfd, 0xfb,
      0xd3, 0x2a, 0x36, 0x0f, 0x89, 0x32, 0x86, 0x25, 0x56, 0xb9, 0xe7, 0x3c,
      0xeb, 0xb4, 0x84, 0x41, 0x2b, 0xa8, 0xf3, 0xa5, 0x3d, 0xfe, 0x56, 0x94,
      0x5b, 0x74, 0xb3, 0x5b, 0x27, 0x3f, 0xe7, 0xcf, 0xe4, 0xf8, 0x15, 0x95,
      0x2a, 0xd2, 0x5f, 0x92, 0xb4, 0x6a, 0x89, 0xa5, 0x54, 0xbd, 0x27, 0x5e,
      0xeb, 0x43, 0x07, 0x9b, 0x2b, 0x8b, 0x22, 0x59, 0x13, 0x4b, 0x9c, 0x56,
      0xd8, 0x63, 0xd9, 0xe6, 0x85, 0x15, 0x2c, 0x82, 0x52, 0x40, 0x8f, 0xb1,
      0xe7, 0x56, 0x07, 0x98
  };
  ASSERT_EQ(Bytes(kExpectedPriv, sizeof(kExpectedPriv)),
            Bytes(priv_key, priv_key_len));

  const uint8_t kExpectedPub[] = {
      0x00, 0x00, 0x00, 0x01, 0x04, 0x48, 0xb1, 0x2d, 0xdd, 0x03, 0x32, 0xeb,
      0x93, 0x31, 0x3d, 0x59, 0x74, 0xf0, 0xcf, 0xaa, 0xa5, 0x39, 0x5f, 0x53,
      0xc4, 0x94, 0x98, 0xbe, 0x8f, 0x22, 0xd7, 0x30, 0xde, 0x1e, 0xb4, 0xf3,
      0x32, 0x23, 0x90, 0x0b, 0xa6, 0x37, 0x4a, 0x4b, 0x44, 0xb3, 0x26, 0x52,
      0x93, 0x7b, 0x4b, 0xa4, 0x79, 0xe8, 0x77, 0x6a, 0x19, 0x81, 0x2a, 0xdd,
      0x91, 0xfb, 0x90, 0x8b, 0x24, 0xb5, 0xbe, 0x20, 0x2e, 0xe8, 0xbc, 0xd3,
      0x83, 0x6c, 0xa8, 0xc5, 0xa1, 0x9a, 0x5b, 0x5e, 0x60, 0xda, 0x45, 0x2e,
      0x31, 0x7f, 0x54, 0x0e, 0x14, 0x40, 0xd2, 0x4d, 0x40, 0x2e, 0x21, 0x79,
      0xfc, 0x77, 0xdd, 0xc7, 0x2d, 0x04, 0xfe, 0xc6, 0xe3, 0xcf, 0x99, 0xef,
      0x88, 0xab, 0x76, 0x86, 0x16, 0x14, 0xed, 0x72, 0x35, 0xa7, 0x05, 0x13,
      0x9f, 0x2c, 0x53, 0xd5, 0xdf, 0x66, 0x75, 0x2e, 0x68, 0xdc, 0xd4, 0xc4,
      0x00, 0x36, 0x08, 0x6d, 0xb7, 0x15, 0xf7, 0xe5, 0x32, 0x59, 0x81, 0x16,
      0x57, 0xaa, 0x72, 0x06, 0xf0, 0xad, 0xd1, 0x85, 0xa0, 0x04, 0xd4, 0x11,
      0x95, 0x1d, 0xac, 0x0b, 0x25, 0xbe, 0x59, 0xa2, 0xb3, 0x30, 0xee, 0x97,
      0x07, 0x2a, 0x51, 0x15, 0xc1, 0x8d, 0xa8, 0xa6, 0x57, 0x9a, 0x4e, 0xbf,
      0xd7, 0x2d, 0x35, 0x07, 0x6b, 0xd6, 0xc9, 0x3c, 0xe4, 0xcf, 0x0b, 0x14,
      0x3e, 0x10, 0x51, 0x77, 0xd6, 0x84, 0x04, 0xbe, 0xd1, 0xd5, 0xa8, 0xf3,
      0x9d, 0x1d, 0x4f, 0xc1, 0xc9, 0xf1, 0x0c, 0x6d, 0xb6, 0xcb, 0xe2, 0x05,
      0x0b, 0x9c, 0x7a, 0x3a, 0x9a, 0x99, 0xe9, 0xa1, 0x93, 0xdc, 0x72, 0x2e,
      0xef, 0xf3, 0x8d, 0xb9, 0x7b, 0xb0, 0x19, 0x24, 0x95, 0x0d, 0x68, 0xa7,
      0xe0, 0xaa, 0x0b, 0xb1, 0xd1, 0xcc, 0x52, 0x14, 0xf9, 0x6c, 0x91, 0x59,
      0xe4, 0xe1, 0x9b, 0xf9, 0x12, 0x39, 0xb1, 0x79, 0xbb, 0x21, 0x92, 0x00,
      0xa4, 0x89, 0xf5, 0xbd, 0xd7, 0x89, 0x27, 0x40, 0xdc, 0xb1, 0x09, 0x38,
      0x63, 0x91, 0x8c, 0xa5, 0x27, 0x27, 0x97, 0x39, 0x35, 0xfa, 0x1a, 0x8a,
      0xa7, 0xe5, 0xc4, 0xd8, 0xbf, 0xe7, 0xbe
  };
  ASSERT_EQ(Bytes(kExpectedPub, sizeof(kExpectedPub)),
            Bytes(pub_key, pub_key_len));
}

// Test that H in |TRUST_TOKEN_experiment_v1| was computed correctly.
TEST(TrustTokenTest, HExp1) {
  const EC_GROUP *group = EC_group_p384();
  const uint8_t kHGen[] = "generator";
  const uint8_t kHLabel[] = "PMBTokens Experiment V1 HashH";

  bssl::UniquePtr<EC_POINT> expected_h(EC_POINT_new(group));
  ASSERT_TRUE(expected_h);
  ASSERT_TRUE(ec_hash_to_curve_p384_xmd_sha512_sswu_draft07(
      group, &expected_h->raw, kHLabel, sizeof(kHLabel), kHGen, sizeof(kHGen)));
  uint8_t expected_bytes[1 + 2 * EC_MAX_BYTES];
  size_t expected_len =
      EC_POINT_point2oct(group, expected_h.get(), POINT_CONVERSION_UNCOMPRESSED,
                         expected_bytes, sizeof(expected_bytes), nullptr);

  uint8_t h[97];
  ASSERT_TRUE(pmbtoken_exp1_get_h_for_testing(h));
  EXPECT_EQ(Bytes(h), Bytes(expected_bytes, expected_len));
}

// Test that H in |TRUST_TOKEN_experiment_v2_pmb| was computed correctly.
TEST(TrustTokenTest, HExp2) {
  const EC_GROUP *group = EC_group_p384();
  const uint8_t kHGen[] = "generator";
  const uint8_t kHLabel[] = "PMBTokens Experiment V2 HashH";

  bssl::UniquePtr<EC_POINT> expected_h(EC_POINT_new(group));
  ASSERT_TRUE(expected_h);
  ASSERT_TRUE(ec_hash_to_curve_p384_xmd_sha512_sswu_draft07(
      group, &expected_h->raw, kHLabel, sizeof(kHLabel), kHGen, sizeof(kHGen)));
  uint8_t expected_bytes[1 + 2 * EC_MAX_BYTES];
  size_t expected_len =
      EC_POINT_point2oct(group, expected_h.get(), POINT_CONVERSION_UNCOMPRESSED,
                         expected_bytes, sizeof(expected_bytes), nullptr);

  uint8_t h[97];
  ASSERT_TRUE(pmbtoken_exp2_get_h_for_testing(h));
  EXPECT_EQ(Bytes(h), Bytes(expected_bytes, expected_len));
}

// Test that H in |TRUST_TOKEN_pst_v1_pmb| was computed correctly.
TEST(TrustTokenTest, HPST1) {
  const EC_GROUP *group = EC_GROUP_new_by_curve_name(NID_secp384r1);
  ASSERT_TRUE(group);

  const uint8_t kHGen[] = "generator";
  const uint8_t kHLabel[] = "PMBTokens PST V1 HashH";

  bssl::UniquePtr<EC_POINT> expected_h(EC_POINT_new(group));
  ASSERT_TRUE(expected_h);
  ASSERT_TRUE(ec_hash_to_curve_p384_xmd_sha384_sswu(
      group, &expected_h->raw, kHLabel, sizeof(kHLabel), kHGen, sizeof(kHGen)));
  uint8_t expected_bytes[1 + 2 * EC_MAX_BYTES];
  size_t expected_len =
      EC_POINT_point2oct(group, expected_h.get(), POINT_CONVERSION_UNCOMPRESSED,
                         expected_bytes, sizeof(expected_bytes), nullptr);

  uint8_t h[97];
  ASSERT_TRUE(pmbtoken_pst1_get_h_for_testing(h));
  EXPECT_EQ(Bytes(h), Bytes(expected_bytes, expected_len));
}

static int ec_point_uncompressed_from_compressed(
    const EC_GROUP *group, uint8_t out[EC_MAX_UNCOMPRESSED], size_t *out_len,
    const uint8_t *in, size_t len) {
  bssl::UniquePtr<EC_POINT> point(EC_POINT_new(group));
  if (!point ||
      !EC_POINT_oct2point(group, point.get(), in, len, nullptr)) {
    return 0;
  }

  *out_len =
      EC_POINT_point2oct(group, point.get(), POINT_CONVERSION_UNCOMPRESSED, out,
                         EC_MAX_UNCOMPRESSED, nullptr);
  return 1;
}

static bool setup_voprf_test_key(const EC_GROUP *group,
                                 TRUST_TOKEN_ISSUER_KEY *out) {
  static const uint8_t kPrivateKey[] = {
      0x05, 0x16, 0x46, 0xb9, 0xe6, 0xe7, 0xa7, 0x1a, 0xe2, 0x7c, 0x1e, 0x1d,
      0x0b, 0x87, 0xb4, 0x38, 0x1d, 0xb6, 0xd3, 0x59, 0x5e, 0xee, 0xb1, 0xad,
      0xb4, 0x15, 0x79, 0xad, 0xbf, 0x99, 0x2f, 0x42, 0x78, 0xf9, 0x01, 0x6e,
      0xaf, 0xc9, 0x44, 0xed, 0xaa, 0x2b, 0x43, 0x18, 0x35, 0x81, 0x77, 0x9d
  };

  static const uint8_t kPublicKey[] = {
      0x03, 0x1d, 0x68, 0x96, 0x86, 0xc6, 0x11, 0x99, 0x1b, 0x55,
      0xf1, 0xa1, 0xd8, 0xf4, 0x30, 0x5c, 0xcd, 0x6c, 0xb7, 0x19,
      0x44, 0x6f, 0x66, 0x0a, 0x30, 0xdb, 0x61, 0xb7, 0xaa, 0x87,
      0xb4, 0x6a, 0xcf, 0x59, 0xb7, 0xc0, 0xd4, 0xa9, 0x07, 0x7b,
      0x3d, 0xa2, 0x1c, 0x25, 0xdd, 0x48, 0x22, 0x29, 0xa0
  };

  if (!ec_scalar_from_bytes(group, &out->xs, kPrivateKey,
                            sizeof(kPrivateKey))) {
    return false;
  }

  bssl::UniquePtr<EC_POINT> pub(EC_POINT_new(group));
  return pub &&
         EC_POINT_oct2point(group, pub.get(), kPublicKey, sizeof(kPublicKey),
                            nullptr) &&
         ec_jacobian_to_affine(group, &out->pubs, &pub->raw);
}

TEST(TrustTokenTest, PSTV1VOPRFTestVector1) {
  const EC_GROUP *group = EC_GROUP_new_by_curve_name(NID_secp384r1);
  TRUST_TOKEN_ISSUER_KEY key;
  ASSERT_TRUE(setup_voprf_test_key(group, &key));

  static const uint8_t kBlindedElement[] = {
      0x02, 0xd3, 0x38, 0xc0, 0x5c, 0xbe, 0xcb, 0x82, 0xde, 0x13,
      0xd6, 0x70, 0x0f, 0x09, 0xcb, 0x61, 0x19, 0x05, 0x43, 0xa7,
      0xb7, 0xe2, 0xc6, 0xcd, 0x4f, 0xca, 0x56, 0x88, 0x7e, 0x56,
      0x4e, 0xa8, 0x26, 0x53, 0xb2, 0x7f, 0xda, 0xd3, 0x83, 0x99,
      0x5e, 0xa6, 0xd0, 0x2c, 0xf2, 0x6d, 0x0e, 0x24, 0xd9
  };

  static const uint8_t kEvaluatedElement[] = {
      0x02, 0xa7, 0xbb, 0xa5, 0x89, 0xb3, 0xe8, 0x67, 0x2a, 0xa1,
      0x9e, 0x8f, 0xd2, 0x58, 0xde, 0x2e, 0x6a, 0xae, 0x20, 0x10,
      0x1c, 0x8d, 0x76, 0x12, 0x46, 0xde, 0x97, 0xa6, 0xb5, 0xee,
      0x9c, 0xf1, 0x05, 0xfe, 0xbc, 0xe4, 0x32, 0x7a, 0x32, 0x62,
      0x55, 0xa3, 0xc6, 0x04, 0xf6, 0x3f, 0x60, 0x0e, 0xf6
  };

  static const uint8_t kProof[] = {
      0xbf, 0xc6, 0xcf, 0x38, 0x59, 0x12, 0x7f, 0x5f, 0xe2, 0x55, 0x48, 0x85,
      0x98, 0x56, 0xd6, 0xb7, 0xfa, 0x1c, 0x74, 0x59, 0xf0, 0xba, 0x57, 0x12,
      0xa8, 0x06, 0xfc, 0x09, 0x1a, 0x30, 0x00, 0xc4, 0x2d, 0x8b, 0xa3, 0x4f,
      0xf4, 0x5f, 0x32, 0xa5, 0x2e, 0x40, 0x53, 0x3e, 0xfd, 0x2a, 0x03, 0xbc,
      0x87, 0xf3, 0xbf, 0x4f, 0x9f, 0x58, 0x02, 0x82, 0x97, 0xcc, 0xb9, 0xcc,
      0xb1, 0x8a, 0xe7, 0x18, 0x2b, 0xcd, 0x1e, 0xf2, 0x39, 0xdf, 0x77, 0xe3,
      0xbe, 0x65, 0xef, 0x14, 0x7f, 0x3a, 0xcf, 0x8b, 0xc9, 0xcb, 0xfc, 0x55,
      0x24, 0xb7, 0x02, 0x26, 0x34, 0x14, 0xf0, 0x43, 0xe3, 0xb7, 0xca, 0x2e
  };

  static const uint8_t kProofScalar[] = {
      0x80, 0x3d, 0x95, 0x5f, 0x0e, 0x07, 0x3a, 0x04, 0xaa, 0x5d, 0x92, 0xb3,
      0xfb, 0x73, 0x9f, 0x56, 0xf9, 0xdb, 0x00, 0x12, 0x66, 0x67, 0x7f, 0x62,
      0xc0, 0x95, 0x02, 0x1d, 0xb0, 0x18, 0xcd, 0x8c, 0xbb, 0x55, 0x94, 0x1d,
      0x40, 0x73, 0x69, 0x8c, 0xe4, 0x5c, 0x40, 0x5d, 0x13, 0x48, 0xb7, 0xb1
  };

  uint8_t blinded_buf[EC_MAX_UNCOMPRESSED];
  size_t blinded_len;
  ASSERT_TRUE(ec_point_uncompressed_from_compressed(
      group, blinded_buf, &blinded_len, kBlindedElement,
      sizeof(kBlindedElement)));

  CBS sign_input;
  CBS_init(&sign_input, blinded_buf, blinded_len);
  bssl::ScopedCBB response;
  ASSERT_TRUE(CBB_init(response.get(), 0));
  ASSERT_TRUE(voprf_pst1_sign_with_proof_scalar_for_testing(
      &key, response.get(), &sign_input, /*num_requested=*/1,
      /*num_to_issue=*/1,
      /*private_metadata=*/0, kProofScalar, sizeof(kProofScalar)));

  uint8_t evaluated_buf[EC_MAX_UNCOMPRESSED];
  size_t evaluated_len;
  ASSERT_TRUE(ec_point_uncompressed_from_compressed(
      group, evaluated_buf, &evaluated_len, kEvaluatedElement,
      sizeof(kEvaluatedElement)));

  bssl::ScopedCBB expected_response;
  ASSERT_TRUE(CBB_init(expected_response.get(), 0));
  ASSERT_TRUE(
      CBB_add_bytes(expected_response.get(), evaluated_buf, evaluated_len));
  ASSERT_TRUE(CBB_add_u16(expected_response.get(), sizeof(kProof)));
  ASSERT_TRUE(CBB_add_bytes(expected_response.get(), kProof, sizeof(kProof)));
  ASSERT_TRUE(CBB_flush(expected_response.get()));

  ASSERT_EQ(Bytes(CBB_data(expected_response.get()),
                  CBB_len(expected_response.get())),
            Bytes(CBB_data(response.get()), CBB_len(response.get())));
}

TEST(TrustTokenTest, PSTV1VOPRFTestVector2) {
  const EC_GROUP *group = EC_GROUP_new_by_curve_name(NID_secp384r1);
  TRUST_TOKEN_ISSUER_KEY key;
  ASSERT_TRUE(setup_voprf_test_key(group, &key));

  static const uint8_t kBlindedElement[] = {
      0x02, 0xf2, 0x74, 0x69, 0xe0, 0x59, 0x88, 0x6f, 0x22, 0x1b,
      0xe5, 0xf2, 0xcc, 0xa0, 0x3d, 0x2b, 0xdc, 0x61, 0xe5, 0x52,
      0x21, 0x72, 0x1c, 0x3b, 0x3e, 0x56, 0xfc, 0x01, 0x2e, 0x36,
      0xd3, 0x1a, 0xe5, 0xf8, 0xdc, 0x05, 0x81, 0x09, 0x59, 0x15,
      0x56, 0xa6, 0xdb, 0xd3, 0xa8, 0xc6, 0x9c, 0x43, 0x3b
  };

  static const uint8_t kEvaluatedElement[] = {
      0x03, 0xf1, 0x6f, 0x90, 0x39, 0x47, 0x03, 0x54, 0x00, 0xe9,
      0x6b, 0x7f, 0x53, 0x1a, 0x38, 0xd4, 0xa0, 0x7a, 0xc8, 0x9a,
      0x80, 0xf8, 0x9d, 0x86, 0xa1, 0xbf, 0x08, 0x9c, 0x52, 0x5a,
      0x92, 0xc7, 0xf4, 0x73, 0x37, 0x29, 0xca, 0x30, 0xc5, 0x6c,
      0xe7, 0x8b, 0x1a, 0xb4, 0xf7, 0xd9, 0x2d, 0xb8, 0xb4
  };

  static const uint8_t kProof[] = {
      0xd0, 0x05, 0xd6, 0xda, 0xaa, 0xd7, 0x57, 0x14, 0x14, 0xc1, 0xe0,
      0xc7, 0x5f, 0x7e, 0x57, 0xf2, 0x11, 0x3c, 0xa9, 0xf4, 0x60, 0x4e,
      0x84, 0xbc, 0x90, 0xf9, 0xbe, 0x52, 0xda, 0x89, 0x6f, 0xff, 0x3b,
      0xee, 0x49, 0x6d, 0xcd, 0xe2, 0xa5, 0x78, 0xae, 0x9d, 0xf3, 0x15,
      0x03, 0x25, 0x85, 0xf8, 0x01, 0xfb, 0x21, 0xc6, 0x08, 0x0a, 0xc0,
      0x56, 0x72, 0xb2, 0x91, 0xe5, 0x75, 0xa4, 0x02, 0x95, 0xb3, 0x06,
      0xd9, 0x67, 0x71, 0x7b, 0x28, 0xe0, 0x8f, 0xcc, 0x8a, 0xd1, 0xca,
      0xb4, 0x78, 0x45, 0xd1, 0x6a, 0xf7, 0x3b, 0x3e, 0x64, 0x3d, 0xdc,
      0xc1, 0x91, 0x20, 0x8e, 0x71, 0xc6, 0x46, 0x30
  };

  static const uint8_t kProofScalar[] = {
      0x80, 0x3d, 0x95, 0x5f, 0x0e, 0x07, 0x3a, 0x04, 0xaa, 0x5d, 0x92, 0xb3,
      0xfb, 0x73, 0x9f, 0x56, 0xf9, 0xdb, 0x00, 0x12, 0x66, 0x67, 0x7f, 0x62,
      0xc0, 0x95, 0x02, 0x1d, 0xb0, 0x18, 0xcd, 0x8c, 0xbb, 0x55, 0x94, 0x1d,
      0x40, 0x73, 0x69, 0x8c, 0xe4, 0x5c, 0x40, 0x5d, 0x13, 0x48, 0xb7, 0xb1
  };

  uint8_t blinded_buf[EC_MAX_UNCOMPRESSED];
  size_t blinded_len;
  ASSERT_TRUE(ec_point_uncompressed_from_compressed(
      group, blinded_buf, &blinded_len, kBlindedElement,
      sizeof(kBlindedElement)));

  CBS sign_input;
  CBS_init(&sign_input, blinded_buf, blinded_len);
  bssl::ScopedCBB response;
  ASSERT_TRUE(CBB_init(response.get(), 0));
  ASSERT_TRUE(voprf_pst1_sign_with_proof_scalar_for_testing(
      &key, response.get(), &sign_input, /*num_requested=*/1,
      /*num_to_issue=*/1,
      /*private_metadata=*/0, kProofScalar, sizeof(kProofScalar)));

  uint8_t evaluated_buf[EC_MAX_UNCOMPRESSED];
  size_t evaluated_len;
  ASSERT_TRUE(ec_point_uncompressed_from_compressed(
      group, evaluated_buf, &evaluated_len, kEvaluatedElement,
      sizeof(kEvaluatedElement)));

  bssl::ScopedCBB expected_response;
  ASSERT_TRUE(CBB_init(expected_response.get(), 0));
  ASSERT_TRUE(
      CBB_add_bytes(expected_response.get(), evaluated_buf, evaluated_len));
  ASSERT_TRUE(CBB_add_u16(expected_response.get(), sizeof(kProof)));
  ASSERT_TRUE(CBB_add_bytes(expected_response.get(), kProof, sizeof(kProof)));
  ASSERT_TRUE(CBB_flush(expected_response.get()));

  ASSERT_EQ(Bytes(CBB_data(expected_response.get()),
                  CBB_len(expected_response.get())),
            Bytes(CBB_data(response.get()), CBB_len(response.get())));
}

TEST(TrustTokenTest, PSTV1VOPRFTestVector3) {
  const EC_GROUP *group = EC_GROUP_new_by_curve_name(NID_secp384r1);
  TRUST_TOKEN_ISSUER_KEY key;
  ASSERT_TRUE(setup_voprf_test_key(group, &key));

  static const uint8_t kBlindedElement1[] = {
      0x02, 0xd3, 0x38, 0xc0, 0x5c, 0xbe, 0xcb, 0x82, 0xde, 0x13,
      0xd6, 0x70, 0x0f, 0x09, 0xcb, 0x61, 0x19, 0x05, 0x43, 0xa7,
      0xb7, 0xe2, 0xc6, 0xcd, 0x4f, 0xca, 0x56, 0x88, 0x7e, 0x56,
      0x4e, 0xa8, 0x26, 0x53, 0xb2, 0x7f, 0xda, 0xd3, 0x83, 0x99,
      0x5e, 0xa6, 0xd0, 0x2c, 0xf2, 0x6d, 0x0e, 0x24, 0xd9
  };
  static const uint8_t kBlindedElement2[] = {
      0x02, 0xfa, 0x02, 0x47, 0x0d, 0x7f, 0x15, 0x10, 0x18, 0xb4,
      0x1e, 0x82, 0x22, 0x3c, 0x32, 0xfa, 0xd8, 0x24, 0xde, 0x6a,
      0xd4, 0xb5, 0xce, 0x9f, 0x8e, 0x9f, 0x98, 0x08, 0x3c, 0x9a,
      0x72, 0x6d, 0xe9, 0xa1, 0xfc, 0x39, 0xd7, 0xa0, 0xcb, 0x6f,
      0x4f, 0x18, 0x8d, 0xd9, 0xce, 0xa0, 0x14, 0x74, 0xcd
  };

  static const uint8_t kEvaluatedElement1[] = {
      0x02, 0xa7, 0xbb, 0xa5, 0x89, 0xb3, 0xe8, 0x67, 0x2a, 0xa1,
      0x9e, 0x8f, 0xd2, 0x58, 0xde, 0x2e, 0x6a, 0xae, 0x20, 0x10,
      0x1c, 0x8d, 0x76, 0x12, 0x46, 0xde, 0x97, 0xa6, 0xb5, 0xee,
      0x9c, 0xf1, 0x05, 0xfe, 0xbc, 0xe4, 0x32, 0x7a, 0x32, 0x62,
      0x55, 0xa3, 0xc6, 0x04, 0xf6, 0x3f, 0x60, 0x0e, 0xf6
  };

  static const uint8_t kEvaluatedElement2[] = {
      0x02, 0x8e, 0x9e, 0x11, 0x56, 0x25, 0xff, 0x4c, 0x2f, 0x07,
      0xbf, 0x87, 0xce, 0x3f, 0xd7, 0x3f, 0xc7, 0x79, 0x94, 0xa7,
      0xa0, 0xc1, 0xdf, 0x03, 0xd2, 0xa6, 0x30, 0xa3, 0xd8, 0x45,
      0x93, 0x0e, 0x2e, 0x63, 0xa1, 0x65, 0xb1, 0x14, 0xd9, 0x8f,
      0xe3, 0x4e, 0x61, 0xb6, 0x8d, 0x23, 0xc0, 0xb5, 0x0a
  };

  static const uint8_t kProof[] = {
      0x6d, 0x8d, 0xcb, 0xd2, 0xfc, 0x95, 0x55, 0x0a, 0x02, 0x21, 0x1f,
      0xb7, 0x8a, 0xfd, 0x01, 0x39, 0x33, 0xf3, 0x07, 0xd2, 0x1e, 0x7d,
      0x85, 0x5b, 0x0b, 0x1e, 0xd0, 0xaf, 0x78, 0x07, 0x6d, 0x81, 0x37,
      0xad, 0x8b, 0x0a, 0x1b, 0xfa, 0x05, 0x67, 0x6d, 0x32, 0x52, 0x49,
      0xc1, 0xdb, 0xb9, 0xa5, 0x2b, 0xd8, 0x1b, 0x1c, 0x2b, 0x7b, 0x0e,
      0xfc, 0x77, 0xcf, 0x7b, 0x27, 0x8e, 0x1c, 0x94, 0x7f, 0x62, 0x83,
      0xf1, 0xd4, 0xc5, 0x13, 0x05, 0x3f, 0xc0, 0xad, 0x19, 0xe0, 0x26,
      0xfb, 0x0c, 0x30, 0x65, 0x4b, 0x53, 0xd9, 0xce, 0xa4, 0xb8, 0x7b,
      0x03, 0x72, 0x71, 0xb5, 0xd2, 0xe2, 0xd0, 0xea
  };

  static const uint8_t kProofScalar[] = {
      0xa0, 0x97, 0xe7, 0x22, 0xed, 0x24, 0x27, 0xde, 0x86, 0x96,
      0x69, 0x10, 0xac, 0xba, 0x9f, 0x5c, 0x35, 0x0e, 0x80, 0x40,
      0xf8, 0x28, 0xbf, 0x6c, 0xec, 0xa2, 0x74, 0x05, 0x42, 0x0c,
      0xdf, 0x3d, 0x63, 0xcb, 0x3a, 0xef, 0x00, 0x5f, 0x40, 0xba,
      0x51, 0x94, 0x3c, 0x80, 0x26, 0x87, 0x79, 0x63
  };

  uint8_t blinded_buf[2*EC_MAX_UNCOMPRESSED];
  size_t blinded_len;
  ASSERT_TRUE(ec_point_uncompressed_from_compressed(
      group, blinded_buf, &blinded_len, kBlindedElement1,
      sizeof(kBlindedElement1)));
  size_t offset = blinded_len;
  ASSERT_TRUE(ec_point_uncompressed_from_compressed(
      group, blinded_buf + offset, &blinded_len, kBlindedElement2,
      sizeof(kBlindedElement2)));

  CBS sign_input;
  CBS_init(&sign_input, blinded_buf, offset + blinded_len);
  bssl::ScopedCBB response;
  ASSERT_TRUE(CBB_init(response.get(), 0));
  ASSERT_TRUE(voprf_pst1_sign_with_proof_scalar_for_testing(
      &key, response.get(), &sign_input, /*num_requested=*/2,
      /*num_to_issue=*/2,
      /*private_metadata=*/0, kProofScalar, sizeof(kProofScalar)));

  uint8_t evaluated_buf[2 * EC_MAX_UNCOMPRESSED];
  size_t evaluated_len;
  ASSERT_TRUE(ec_point_uncompressed_from_compressed(
      group, evaluated_buf, &evaluated_len, kEvaluatedElement1,
      sizeof(kEvaluatedElement1)));
  offset = evaluated_len;
  ASSERT_TRUE(ec_point_uncompressed_from_compressed(
      group, evaluated_buf + offset, &evaluated_len, kEvaluatedElement2,
      sizeof(kEvaluatedElement2)));

  bssl::ScopedCBB expected_response;
  ASSERT_TRUE(CBB_init(expected_response.get(), 0));
  ASSERT_TRUE(CBB_add_bytes(expected_response.get(), evaluated_buf,
                            offset + evaluated_len));
  ASSERT_TRUE(CBB_add_u16(expected_response.get(), sizeof(kProof)));
  ASSERT_TRUE(CBB_add_bytes(expected_response.get(), kProof, sizeof(kProof)));
  ASSERT_TRUE(CBB_flush(expected_response.get()));

  ASSERT_EQ(Bytes(CBB_data(expected_response.get()),
                  CBB_len(expected_response.get())),
            Bytes(CBB_data(response.get()), CBB_len(response.get())));
}

static std::vector<const TRUST_TOKEN_METHOD *> AllMethods() {
  return {
    TRUST_TOKEN_experiment_v1(),
    TRUST_TOKEN_experiment_v2_voprf(),
    TRUST_TOKEN_experiment_v2_pmb(),
    TRUST_TOKEN_pst_v1_voprf(),
    TRUST_TOKEN_pst_v1_pmb()
  };
}

class TrustTokenProtocolTestBase : public ::testing::Test {
 public:
  explicit TrustTokenProtocolTestBase(const TRUST_TOKEN_METHOD *method_arg,
                                      bool use_msg)
      : method_(method_arg), use_msg_(use_msg) {}

  // KeyID returns the key ID associated with key index |i|.
  static uint32_t KeyID(size_t i) {
    assert(i <= UINT32_MAX);
    // Use a different value from the indices to that we do not mix them up.
    return static_cast<uint32_t>(7 + i);
  }

  const TRUST_TOKEN_METHOD *method() const { return method_; }

  bool use_message() const { return use_msg_; }

 protected:
  void SetupContexts() {
    client.reset(TRUST_TOKEN_CLIENT_new(method(), client_max_batchsize));
    ASSERT_TRUE(client);
    issuer.reset(TRUST_TOKEN_ISSUER_new(method(), issuer_max_batchsize));
    ASSERT_TRUE(issuer);

    for (size_t i = 0; i < method()->max_keys; i++) {
      uint8_t priv_key[TRUST_TOKEN_MAX_PRIVATE_KEY_SIZE];
      uint8_t pub_key[TRUST_TOKEN_MAX_PUBLIC_KEY_SIZE];
      size_t priv_key_len, pub_key_len, key_index;
      ASSERT_TRUE(TRUST_TOKEN_generate_key(
          method(), priv_key, &priv_key_len, TRUST_TOKEN_MAX_PRIVATE_KEY_SIZE,
          pub_key, &pub_key_len, TRUST_TOKEN_MAX_PUBLIC_KEY_SIZE, KeyID(i)));
      ASSERT_TRUE(TRUST_TOKEN_CLIENT_add_key(client.get(), &key_index, pub_key,
                                             pub_key_len));
      ASSERT_EQ(i, key_index);
      ASSERT_TRUE(
          TRUST_TOKEN_ISSUER_add_key(issuer.get(), priv_key, priv_key_len));
    }

    uint8_t public_key[32], private_key[64];
    ED25519_keypair(public_key, private_key);
    bssl::UniquePtr<EVP_PKEY> priv(EVP_PKEY_new_raw_private_key(
        EVP_PKEY_ED25519, nullptr, private_key, 32));
    ASSERT_TRUE(priv);
    bssl::UniquePtr<EVP_PKEY> pub(
        EVP_PKEY_new_raw_public_key(EVP_PKEY_ED25519, nullptr, public_key, 32));
    ASSERT_TRUE(pub);

    TRUST_TOKEN_CLIENT_set_srr_key(client.get(), pub.get());
    TRUST_TOKEN_ISSUER_set_srr_key(issuer.get(), priv.get());
    RAND_bytes(metadata_key, sizeof(metadata_key));
    ASSERT_TRUE(TRUST_TOKEN_ISSUER_set_metadata_key(issuer.get(), metadata_key,
                                                    sizeof(metadata_key)));
  }

  const TRUST_TOKEN_METHOD *method_;
  bool use_msg_;
  uint16_t client_max_batchsize = 10;
  uint16_t issuer_max_batchsize = 10;
  bssl::UniquePtr<TRUST_TOKEN_CLIENT> client;
  bssl::UniquePtr<TRUST_TOKEN_ISSUER> issuer;
  uint8_t metadata_key[32];
};

class TrustTokenProtocolTest
    : public TrustTokenProtocolTestBase,
      public testing::WithParamInterface<
          std::tuple<const TRUST_TOKEN_METHOD *, bool>> {
 public:
  TrustTokenProtocolTest()
      : TrustTokenProtocolTestBase(std::get<0>(GetParam()),
                                   std::get<1>(GetParam())) {}
};

INSTANTIATE_TEST_SUITE_P(TrustTokenAllProtocolTest, TrustTokenProtocolTest,
                         testing::Combine(testing::ValuesIn(AllMethods()),
                                          testing::Bool()));

TEST_P(TrustTokenProtocolTest, InvalidToken) {
  ASSERT_NO_FATAL_FAILURE(SetupContexts());

  uint8_t *issue_msg = NULL, *issue_resp = NULL;
  size_t msg_len, resp_len;

  size_t key_index;
  size_t tokens_issued;
  if (use_message()) {
    ASSERT_TRUE(TRUST_TOKEN_CLIENT_begin_issuance_over_message(
        client.get(), &issue_msg, &msg_len, 1, kMessage, sizeof(kMessage)));
  } else {
    ASSERT_TRUE(TRUST_TOKEN_CLIENT_begin_issuance(client.get(), &issue_msg,
                                                  &msg_len, 1));
  }
  bssl::UniquePtr<uint8_t> free_issue_msg(issue_msg);
  ASSERT_TRUE(TRUST_TOKEN_ISSUER_issue(
      issuer.get(), &issue_resp, &resp_len, &tokens_issued, issue_msg, msg_len,
      /*public_metadata=*/KeyID(0), /*private_metadata=*/0,
      /*max_issuance=*/10));
  bssl::UniquePtr<uint8_t> free_msg(issue_resp);
  bssl::UniquePtr<STACK_OF(TRUST_TOKEN)> tokens(
      TRUST_TOKEN_CLIENT_finish_issuance(client.get(), &key_index, issue_resp,
                                         resp_len));
  ASSERT_TRUE(tokens);

  for (TRUST_TOKEN *token : tokens.get()) {
    // Corrupt the token.
    token->data[0] ^= 0x42;

    uint8_t *redeem_msg = NULL, *redeem_resp = NULL;
    ASSERT_TRUE(TRUST_TOKEN_CLIENT_begin_redemption(
        client.get(), &redeem_msg, &msg_len, token, NULL, 0, 0));
    bssl::UniquePtr<uint8_t> free_redeem_msg(redeem_msg);
    uint32_t public_value;
    uint8_t private_value;
    TRUST_TOKEN *rtoken;
    uint8_t *client_data;
    size_t client_data_len;
    if (use_message()) {
      ASSERT_FALSE(TRUST_TOKEN_ISSUER_redeem_over_message(
          issuer.get(), &public_value, &private_value, &rtoken, &client_data,
          &client_data_len, redeem_msg, msg_len, kMessage, sizeof(kMessage)));
    } else {
      ASSERT_FALSE(TRUST_TOKEN_ISSUER_redeem(
          issuer.get(), &public_value, &private_value, &rtoken, &client_data,
          &client_data_len, redeem_msg, msg_len));
    }
    bssl::UniquePtr<uint8_t> free_redeem_resp(redeem_resp);
  }
}

TEST_P(TrustTokenProtocolTest, TruncatedIssuanceRequest) {
  ASSERT_NO_FATAL_FAILURE(SetupContexts());

  uint8_t *issue_msg = NULL, *issue_resp = NULL;
  size_t msg_len, resp_len;
  if (use_message()) {
    ASSERT_TRUE(TRUST_TOKEN_CLIENT_begin_issuance_over_message(
        client.get(), &issue_msg, &msg_len, 10, kMessage, sizeof(kMessage)));
  } else {
    ASSERT_TRUE(TRUST_TOKEN_CLIENT_begin_issuance(client.get(), &issue_msg,
                                                  &msg_len, 10));
  }
  bssl::UniquePtr<uint8_t> free_issue_msg(issue_msg);
  msg_len = 10;
  size_t tokens_issued;
  ASSERT_FALSE(TRUST_TOKEN_ISSUER_issue(
      issuer.get(), &issue_resp, &resp_len, &tokens_issued, issue_msg, msg_len,
      /*public_metadata=*/KeyID(0), /*private_metadata=*/0,
      /*max_issuance=*/10));
  bssl::UniquePtr<uint8_t> free_msg(issue_resp);
}

TEST_P(TrustTokenProtocolTest, TruncatedIssuanceResponse) {
  ASSERT_NO_FATAL_FAILURE(SetupContexts());

  uint8_t *issue_msg = NULL, *issue_resp = NULL;
  size_t msg_len, resp_len;
  if (use_message()) {
    ASSERT_TRUE(TRUST_TOKEN_CLIENT_begin_issuance_over_message(
        client.get(), &issue_msg, &msg_len, 10, kMessage, sizeof(kMessage)));
  } else {
    ASSERT_TRUE(TRUST_TOKEN_CLIENT_begin_issuance(client.get(), &issue_msg,
                                                  &msg_len, 10));
  }
  bssl::UniquePtr<uint8_t> free_issue_msg(issue_msg);
  size_t tokens_issued;
  ASSERT_TRUE(TRUST_TOKEN_ISSUER_issue(
      issuer.get(), &issue_resp, &resp_len, &tokens_issued, issue_msg, msg_len,
      /*public_metadata=*/KeyID(0), /*private_metadata=*/0,
      /*max_issuance=*/10));
  bssl::UniquePtr<uint8_t> free_msg(issue_resp);
  resp_len = 10;
  size_t key_index;
  bssl::UniquePtr<STACK_OF(TRUST_TOKEN)> tokens(
      TRUST_TOKEN_CLIENT_finish_issuance(client.get(), &key_index, issue_resp,
                                         resp_len));
  ASSERT_FALSE(tokens);
}

TEST_P(TrustTokenProtocolTest, ExtraDataIssuanceResponse) {
  ASSERT_NO_FATAL_FAILURE(SetupContexts());

  uint8_t *request = NULL, *response = NULL;
  size_t request_len, response_len;
  if (use_message()) {
    ASSERT_TRUE(TRUST_TOKEN_CLIENT_begin_issuance_over_message(
        client.get(), &request, &request_len, 10, kMessage, sizeof(kMessage)));
  } else {
    ASSERT_TRUE(TRUST_TOKEN_CLIENT_begin_issuance(client.get(), &request,
                                                  &request_len, 10));
  }
  bssl::UniquePtr<uint8_t> free_request(request);
  size_t tokens_issued;
  ASSERT_TRUE(TRUST_TOKEN_ISSUER_issue(issuer.get(), &response, &response_len,
                                       &tokens_issued, request, request_len,
                                       /*public_metadata=*/KeyID(0),
                                       /*private_metadata=*/0,
                                       /*max_issuance=*/10));
  bssl::UniquePtr<uint8_t> free_response(response);
  std::vector<uint8_t> response2(response, response + response_len);
  response2.push_back(0);
  size_t key_index;
  bssl::UniquePtr<STACK_OF(TRUST_TOKEN)> tokens(
      TRUST_TOKEN_CLIENT_finish_issuance(client.get(), &key_index,
                                         response2.data(), response2.size()));
  ASSERT_FALSE(tokens);
}

TEST_P(TrustTokenProtocolTest, TruncatedRedemptionRequest) {
  ASSERT_NO_FATAL_FAILURE(SetupContexts());

  uint8_t *issue_msg = NULL, *issue_resp = NULL;
  size_t msg_len, resp_len;
  if (use_message()) {
    ASSERT_TRUE(TRUST_TOKEN_CLIENT_begin_issuance_over_message(
        client.get(), &issue_msg, &msg_len, 10, kMessage, sizeof(kMessage)));
  } else {
    ASSERT_TRUE(TRUST_TOKEN_CLIENT_begin_issuance(client.get(), &issue_msg,
                                                  &msg_len, 10));
  }
  bssl::UniquePtr<uint8_t> free_issue_msg(issue_msg);
  size_t tokens_issued;
  ASSERT_TRUE(TRUST_TOKEN_ISSUER_issue(
      issuer.get(), &issue_resp, &resp_len, &tokens_issued, issue_msg, msg_len,
      /*public_metadata=*/KeyID(0), /*private_metadata=*/0,
      /*max_issuance=*/10));
  bssl::UniquePtr<uint8_t> free_msg(issue_resp);
  size_t key_index;
  bssl::UniquePtr<STACK_OF(TRUST_TOKEN)> tokens(
      TRUST_TOKEN_CLIENT_finish_issuance(client.get(), &key_index, issue_resp,
                                         resp_len));
  ASSERT_TRUE(tokens);

  for (TRUST_TOKEN *token : tokens.get()) {
    const uint8_t kClientData[] = "\x70TEST CLIENT DATA";
    uint64_t kRedemptionTime = (method()->has_srr ? 13374242 : 0);

    uint8_t *redeem_msg = NULL;
    ASSERT_TRUE(TRUST_TOKEN_CLIENT_begin_redemption(
        client.get(), &redeem_msg, &msg_len, token, kClientData,
        sizeof(kClientData) - 1, kRedemptionTime));
    bssl::UniquePtr<uint8_t> free_redeem_msg(redeem_msg);
    msg_len = 10;

    uint32_t public_value;
    uint8_t private_value;
    TRUST_TOKEN *rtoken;
    uint8_t *client_data;
    size_t client_data_len;
    if (use_message()) {
      ASSERT_FALSE(TRUST_TOKEN_ISSUER_redeem_over_message(
          issuer.get(), &public_value, &private_value, &rtoken, &client_data,
          &client_data_len, redeem_msg, msg_len, kMessage, sizeof(kMessage)));
    } else {
      ASSERT_FALSE(TRUST_TOKEN_ISSUER_redeem(
          issuer.get(), &public_value, &private_value, &rtoken, &client_data,
          &client_data_len, redeem_msg, msg_len));
    }
  }
}

TEST_P(TrustTokenProtocolTest, IssuedWithBadKeyID) {
  client.reset(TRUST_TOKEN_CLIENT_new(method(), client_max_batchsize));
  ASSERT_TRUE(client);
  issuer.reset(TRUST_TOKEN_ISSUER_new(method(), issuer_max_batchsize));
  ASSERT_TRUE(issuer);

  // We configure the client and the issuer with different key IDs and test
  // that the client notices.
  const uint32_t kClientKeyID = 0;
  const uint32_t kIssuerKeyID = 42;

  uint8_t priv_key[TRUST_TOKEN_MAX_PRIVATE_KEY_SIZE];
  uint8_t pub_key[TRUST_TOKEN_MAX_PUBLIC_KEY_SIZE];
  size_t priv_key_len, pub_key_len, key_index;
  ASSERT_TRUE(TRUST_TOKEN_generate_key(
      method(), priv_key, &priv_key_len, TRUST_TOKEN_MAX_PRIVATE_KEY_SIZE,
      pub_key, &pub_key_len, TRUST_TOKEN_MAX_PUBLIC_KEY_SIZE, kClientKeyID));
  ASSERT_TRUE(TRUST_TOKEN_CLIENT_add_key(client.get(), &key_index, pub_key,
                                         pub_key_len));
  ASSERT_EQ(0UL, key_index);

  ASSERT_TRUE(TRUST_TOKEN_generate_key(
      method(), priv_key, &priv_key_len, TRUST_TOKEN_MAX_PRIVATE_KEY_SIZE,
      pub_key, &pub_key_len, TRUST_TOKEN_MAX_PUBLIC_KEY_SIZE, kIssuerKeyID));
  ASSERT_TRUE(TRUST_TOKEN_ISSUER_add_key(issuer.get(), priv_key, priv_key_len));


  uint8_t public_key[32], private_key[64];
  ED25519_keypair(public_key, private_key);
  bssl::UniquePtr<EVP_PKEY> priv(
      EVP_PKEY_new_raw_private_key(EVP_PKEY_ED25519, nullptr, private_key, 32));
  ASSERT_TRUE(priv);
  bssl::UniquePtr<EVP_PKEY> pub(
      EVP_PKEY_new_raw_public_key(EVP_PKEY_ED25519, nullptr, public_key, 32));
  ASSERT_TRUE(pub);

  TRUST_TOKEN_CLIENT_set_srr_key(client.get(), pub.get());
  TRUST_TOKEN_ISSUER_set_srr_key(issuer.get(), priv.get());
  RAND_bytes(metadata_key, sizeof(metadata_key));
  ASSERT_TRUE(TRUST_TOKEN_ISSUER_set_metadata_key(issuer.get(), metadata_key,
                                                  sizeof(metadata_key)));


  uint8_t *issue_msg = NULL, *issue_resp = NULL;
  size_t msg_len, resp_len;
  if (use_message()) {
    ASSERT_TRUE(TRUST_TOKEN_CLIENT_begin_issuance_over_message(
        client.get(), &issue_msg, &msg_len, 10, kMessage, sizeof(kMessage)));
  } else {
    ASSERT_TRUE(TRUST_TOKEN_CLIENT_begin_issuance(client.get(), &issue_msg,
                                                  &msg_len, 10));
  }
  bssl::UniquePtr<uint8_t> free_issue_msg(issue_msg);
  size_t tokens_issued;
  ASSERT_TRUE(TRUST_TOKEN_ISSUER_issue(
      issuer.get(), &issue_resp, &resp_len, &tokens_issued, issue_msg, msg_len,
      /*public_metadata=*/42, /*private_metadata=*/0, /*max_issuance=*/10));
  bssl::UniquePtr<uint8_t> free_msg(issue_resp);
  bssl::UniquePtr<STACK_OF(TRUST_TOKEN)> tokens(
      TRUST_TOKEN_CLIENT_finish_issuance(client.get(), &key_index, issue_resp,
                                         resp_len));
  ASSERT_FALSE(tokens);
}

class TrustTokenMetadataTest
    : public TrustTokenProtocolTestBase,
      public testing::WithParamInterface<
    std::tuple<const TRUST_TOKEN_METHOD *, bool, int, bool>> {
 public:
  TrustTokenMetadataTest()
      : TrustTokenProtocolTestBase(std::get<0>(GetParam()),
                                   std::get<1>(GetParam())) {}

  int public_metadata() { return std::get<2>(GetParam()); }
  bool private_metadata() { return std::get<3>(GetParam()); }
};

TEST_P(TrustTokenMetadataTest, SetAndGetMetadata) {
  ASSERT_NO_FATAL_FAILURE(SetupContexts());

  uint8_t *issue_msg = NULL, *issue_resp = NULL;
  size_t msg_len, resp_len;
  if (use_message()) {
    ASSERT_TRUE(TRUST_TOKEN_CLIENT_begin_issuance_over_message(
        client.get(), &issue_msg, &msg_len, 10, kMessage, sizeof(kMessage)));
  } else {
    ASSERT_TRUE(TRUST_TOKEN_CLIENT_begin_issuance(client.get(), &issue_msg,
                                                  &msg_len, 10));
  }
  bssl::UniquePtr<uint8_t> free_issue_msg(issue_msg);
  size_t tokens_issued;
  bool result = TRUST_TOKEN_ISSUER_issue(
      issuer.get(), &issue_resp, &resp_len, &tokens_issued, issue_msg, msg_len,
      public_metadata(), private_metadata(), /*max_issuance=*/1);
  if (!method()->has_private_metadata && private_metadata()) {
    ASSERT_FALSE(result);
    return;
  }
  ASSERT_TRUE(result);
  bssl::UniquePtr<uint8_t> free_msg(issue_resp);
  size_t key_index;
  bssl::UniquePtr<STACK_OF(TRUST_TOKEN)> tokens(
      TRUST_TOKEN_CLIENT_finish_issuance(client.get(), &key_index, issue_resp,
                                         resp_len));
  ASSERT_TRUE(tokens);
  EXPECT_EQ(1u, sk_TRUST_TOKEN_num(tokens.get()));

  for (TRUST_TOKEN *token : tokens.get()) {
    const uint8_t kClientData[] = "\x70TEST CLIENT DATA";
    uint64_t kRedemptionTime = (method()->has_srr ? 13374242 : 0);

    uint8_t *redeem_msg = NULL;
    ASSERT_TRUE(TRUST_TOKEN_CLIENT_begin_redemption(
        client.get(), &redeem_msg, &msg_len, token, kClientData,
        sizeof(kClientData) - 1, kRedemptionTime));
    bssl::UniquePtr<uint8_t> free_redeem_msg(redeem_msg);
    uint32_t public_value;
    uint8_t private_value;
    TRUST_TOKEN *rtoken;
    uint8_t *client_data;
    size_t client_data_len;
    if (use_message()) {
      ASSERT_TRUE(TRUST_TOKEN_ISSUER_redeem_over_message(
          issuer.get(), &public_value, &private_value, &rtoken, &client_data,
          &client_data_len, redeem_msg, msg_len, kMessage, sizeof(kMessage)));
    } else {
      ASSERT_TRUE(TRUST_TOKEN_ISSUER_redeem(
          issuer.get(), &public_value, &private_value, &rtoken, &client_data,
          &client_data_len, redeem_msg, msg_len));
    }
    bssl::UniquePtr<uint8_t> free_client_data(client_data);
    bssl::UniquePtr<TRUST_TOKEN> free_rtoken(rtoken);

    ASSERT_EQ(Bytes(kClientData, sizeof(kClientData) - 1),
              Bytes(client_data, client_data_len));
    ASSERT_EQ(public_value, static_cast<uint32_t>(public_metadata()));
    ASSERT_EQ(private_value, private_metadata());
  }
}

TEST_P(TrustTokenMetadataTest, TooManyRequests) {
  if (!method()->has_private_metadata && private_metadata()) {
    return;
  }

  issuer_max_batchsize = 1;
  ASSERT_NO_FATAL_FAILURE(SetupContexts());

  uint8_t *issue_msg = NULL, *issue_resp = NULL;
  size_t msg_len, resp_len;
  if (use_message()) {
    ASSERT_TRUE(TRUST_TOKEN_CLIENT_begin_issuance_over_message(
        client.get(), &issue_msg, &msg_len, 10, kMessage, sizeof(kMessage)));
  } else {
    ASSERT_TRUE(TRUST_TOKEN_CLIENT_begin_issuance(client.get(), &issue_msg,
                                                  &msg_len, 10));
  }
  bssl::UniquePtr<uint8_t> free_issue_msg(issue_msg);
  size_t tokens_issued;
  ASSERT_TRUE(TRUST_TOKEN_ISSUER_issue(
      issuer.get(), &issue_resp, &resp_len, &tokens_issued, issue_msg, msg_len,
      public_metadata(), private_metadata(), /*max_issuance=*/1));
  bssl::UniquePtr<uint8_t> free_msg(issue_resp);
  ASSERT_EQ(tokens_issued, issuer_max_batchsize);
  size_t key_index;
  bssl::UniquePtr<STACK_OF(TRUST_TOKEN)> tokens(
      TRUST_TOKEN_CLIENT_finish_issuance(client.get(), &key_index, issue_resp,
                                         resp_len));
  ASSERT_TRUE(tokens);
  ASSERT_EQ(sk_TRUST_TOKEN_num(tokens.get()), 1UL);
}


TEST_P(TrustTokenMetadataTest, TruncatedProof) {
  if (!method()->has_private_metadata && private_metadata()) {
    return;
  }

  ASSERT_NO_FATAL_FAILURE(SetupContexts());

  uint8_t *issue_msg = NULL, *issue_resp = NULL;
  size_t msg_len, resp_len;
  if (use_message()) {
    ASSERT_TRUE(TRUST_TOKEN_CLIENT_begin_issuance_over_message(
        client.get(), &issue_msg, &msg_len, 10, kMessage, sizeof(kMessage)));
  } else {
    ASSERT_TRUE(TRUST_TOKEN_CLIENT_begin_issuance(client.get(), &issue_msg,
                                                  &msg_len, 10));
  }
  bssl::UniquePtr<uint8_t> free_issue_msg(issue_msg);
  size_t tokens_issued;
  ASSERT_TRUE(TRUST_TOKEN_ISSUER_issue(
      issuer.get(), &issue_resp, &resp_len, &tokens_issued, issue_msg, msg_len,
      public_metadata(), private_metadata(), /*max_issuance=*/1));
  bssl::UniquePtr<uint8_t> free_msg(issue_resp);

  CBS real_response;
  CBS_init(&real_response, issue_resp, resp_len);
  uint16_t count;
  uint32_t parsed_public_metadata;
  bssl::ScopedCBB bad_response;
  ASSERT_TRUE(CBB_init(bad_response.get(), 0));
  ASSERT_TRUE(CBS_get_u16(&real_response, &count));
  ASSERT_TRUE(CBB_add_u16(bad_response.get(), count));
  ASSERT_TRUE(CBS_get_u32(&real_response, &parsed_public_metadata));
  ASSERT_TRUE(CBB_add_u32(bad_response.get(), parsed_public_metadata));

  const size_t kP384PointLen = 1 + 2 * (384 / 8);
  size_t token_length = TRUST_TOKEN_NONCE_SIZE + 2 * kP384PointLen;
  if (method() == TRUST_TOKEN_experiment_v1()) {
    token_length += 4;
  }
  if (method() == TRUST_TOKEN_experiment_v2_voprf() ||
      method() == TRUST_TOKEN_pst_v1_voprf()) {
    token_length = kP384PointLen;
  }
  for (size_t i = 0; i < count; i++) {
    ASSERT_TRUE(CBB_add_bytes(bad_response.get(), CBS_data(&real_response),
                              token_length));
    ASSERT_TRUE(CBS_skip(&real_response, token_length));
  }

  CBS tmp;
  ASSERT_TRUE(CBS_get_u16_length_prefixed(&real_response, &tmp));
  CBB dleq;
  ASSERT_TRUE(CBB_add_u16_length_prefixed(bad_response.get(), &dleq));
  ASSERT_TRUE(CBB_add_bytes(&dleq, CBS_data(&tmp), CBS_len(&tmp) - 2));
  ASSERT_TRUE(CBB_flush(bad_response.get()));

  uint8_t *bad_buf;
  size_t bad_len;
  ASSERT_TRUE(CBB_finish(bad_response.get(), &bad_buf, &bad_len));
  bssl::UniquePtr<uint8_t> free_bad(bad_buf);

  size_t key_index;
  bssl::UniquePtr<STACK_OF(TRUST_TOKEN)> tokens(
      TRUST_TOKEN_CLIENT_finish_issuance(client.get(), &key_index, bad_buf,
                                         bad_len));
  ASSERT_FALSE(tokens);
}

TEST_P(TrustTokenMetadataTest, ExcessDataProof) {
  if (!method()->has_private_metadata && private_metadata()) {
    return;
  }

  ASSERT_NO_FATAL_FAILURE(SetupContexts());

  uint8_t *issue_msg = NULL, *issue_resp = NULL;
  size_t msg_len, resp_len;
  if (use_message()) {
    ASSERT_TRUE(TRUST_TOKEN_CLIENT_begin_issuance_over_message(
        client.get(), &issue_msg, &msg_len, 10, kMessage, sizeof(kMessage)));
  } else {
    ASSERT_TRUE(TRUST_TOKEN_CLIENT_begin_issuance(client.get(), &issue_msg,
                                                  &msg_len, 10));
  }
  bssl::UniquePtr<uint8_t> free_issue_msg(issue_msg);
  size_t tokens_issued;
  ASSERT_TRUE(TRUST_TOKEN_ISSUER_issue(
      issuer.get(), &issue_resp, &resp_len, &tokens_issued, issue_msg, msg_len,
      public_metadata(), private_metadata(), /*max_issuance=*/1));
  bssl::UniquePtr<uint8_t> free_msg(issue_resp);

  CBS real_response;
  CBS_init(&real_response, issue_resp, resp_len);
  uint16_t count;
  uint32_t parsed_public_metadata;
  bssl::ScopedCBB bad_response;
  ASSERT_TRUE(CBB_init(bad_response.get(), 0));
  ASSERT_TRUE(CBS_get_u16(&real_response, &count));
  ASSERT_TRUE(CBB_add_u16(bad_response.get(), count));
  ASSERT_TRUE(CBS_get_u32(&real_response, &parsed_public_metadata));
  ASSERT_TRUE(CBB_add_u32(bad_response.get(), parsed_public_metadata));

  const size_t kP384PointLen = 1 + 2 * (384 / 8);
  size_t token_length = TRUST_TOKEN_NONCE_SIZE + 2 * kP384PointLen;
  if (method() == TRUST_TOKEN_experiment_v1()) {
    token_length += 4;
  }
  if (method() == TRUST_TOKEN_experiment_v2_voprf() ||
      method() == TRUST_TOKEN_pst_v1_voprf()) {
    token_length = kP384PointLen;
  }
  for (size_t i = 0; i < count; i++) {
    ASSERT_TRUE(CBB_add_bytes(bad_response.get(), CBS_data(&real_response),
                              token_length));
    ASSERT_TRUE(CBS_skip(&real_response, token_length));
  }

  CBS tmp;
  ASSERT_TRUE(CBS_get_u16_length_prefixed(&real_response, &tmp));
  CBB dleq;
  ASSERT_TRUE(CBB_add_u16_length_prefixed(bad_response.get(), &dleq));
  ASSERT_TRUE(CBB_add_bytes(&dleq, CBS_data(&tmp), CBS_len(&tmp)));
  ASSERT_TRUE(CBB_add_u16(&dleq, 42));
  ASSERT_TRUE(CBB_flush(bad_response.get()));

  uint8_t *bad_buf;
  size_t bad_len;
  ASSERT_TRUE(CBB_finish(bad_response.get(), &bad_buf, &bad_len));
  bssl::UniquePtr<uint8_t> free_bad(bad_buf);

  size_t key_index;
  bssl::UniquePtr<STACK_OF(TRUST_TOKEN)> tokens(
      TRUST_TOKEN_CLIENT_finish_issuance(client.get(), &key_index, bad_buf,
                                         bad_len));
  ASSERT_FALSE(tokens);
}

INSTANTIATE_TEST_SUITE_P(
    TrustTokenAllMetadataTest, TrustTokenMetadataTest,
    testing::Combine(testing::ValuesIn(AllMethods()),
                     testing::Bool(),
                     testing::Values(TrustTokenProtocolTest::KeyID(0),
                                     TrustTokenProtocolTest::KeyID(1),
                                     TrustTokenProtocolTest::KeyID(2)),
                     testing::Bool()));

class TrustTokenBadKeyTest
    : public TrustTokenProtocolTestBase,
      public testing::WithParamInterface<
          std::tuple<const TRUST_TOKEN_METHOD *, bool, bool, int>> {
 public:
  TrustTokenBadKeyTest()
      : TrustTokenProtocolTestBase(std::get<0>(GetParam()),
                                   std::get<1>(GetParam())) {}

  bool private_metadata() { return std::get<2>(GetParam()); }
  int corrupted_key() { return std::get<3>(GetParam()); }
};

TEST_P(TrustTokenBadKeyTest, BadKey) {
  // For versions without private metadata, only corruptions of 'xs' (the 4th
  // entry in |scalars| below) result in a bad key, as the other scalars are
  // unused internally.
  if (!method()->has_private_metadata &&
      (private_metadata() || corrupted_key() != 4)) {
    return;
  }

  ASSERT_NO_FATAL_FAILURE(SetupContexts());

  uint8_t *issue_msg = NULL, *issue_resp = NULL;
  size_t msg_len, resp_len;
  if (use_message()) {
    ASSERT_TRUE(TRUST_TOKEN_CLIENT_begin_issuance_over_message(
        client.get(), &issue_msg, &msg_len, 10, kMessage, sizeof(kMessage)));
  } else {
    ASSERT_TRUE(TRUST_TOKEN_CLIENT_begin_issuance(client.get(), &issue_msg,
                                                  &msg_len, 10));
  }
  bssl::UniquePtr<uint8_t> free_issue_msg(issue_msg);

  struct trust_token_issuer_key_st *key = &issuer->keys[0];
  EC_SCALAR *scalars[] = {&key->key.x0, &key->key.y0, &key->key.x1,
                          &key->key.y1, &key->key.xs, &key->key.ys};

  // Corrupt private key scalar.
  scalars[corrupted_key()]->words[0] ^= 42;

  size_t tokens_issued;
  ASSERT_TRUE(TRUST_TOKEN_ISSUER_issue(
      issuer.get(), &issue_resp, &resp_len, &tokens_issued, issue_msg, msg_len,
      /*public_metadata=*/7, private_metadata(), /*max_issuance=*/1));
  bssl::UniquePtr<uint8_t> free_msg(issue_resp);
  size_t key_index;
  bssl::UniquePtr<STACK_OF(TRUST_TOKEN)> tokens(
      TRUST_TOKEN_CLIENT_finish_issuance(client.get(), &key_index, issue_resp,
                                         resp_len));

  // If the unused private key is corrupted, then the DLEQ proof should succeed.
  if ((corrupted_key() / 2 == 0 && private_metadata() == true) ||
      (corrupted_key() / 2 == 1 && private_metadata() == false)) {
    ASSERT_TRUE(tokens);
  } else {
    ASSERT_FALSE(tokens);
  }
}

INSTANTIATE_TEST_SUITE_P(TrustTokenAllBadKeyTest, TrustTokenBadKeyTest,
                         testing::Combine(testing::ValuesIn(AllMethods()),
                                          testing::Bool(),
                                          testing::Bool(),
                                          testing::Values(0, 1, 2, 3, 4, 5)));

}  // namespace

TEST(TrustTokenTest, ShortMetadataKeyRejected) {
  bssl::UniquePtr<TRUST_TOKEN_ISSUER> issuer(
      TRUST_TOKEN_ISSUER_new(TRUST_TOKEN_experiment_v1(), 10));
  ASSERT_TRUE(issuer);

  // A 31-byte key must be rejected.
  uint8_t short_key[31];
  RAND_bytes(short_key, sizeof(short_key));
  ASSERT_FALSE(TRUST_TOKEN_ISSUER_set_metadata_key(issuer.get(), short_key,
                                                    sizeof(short_key)));

  // A 32-byte key must be accepted.
  uint8_t good_key[32];
  RAND_bytes(good_key, sizeof(good_key));
  ASSERT_TRUE(TRUST_TOKEN_ISSUER_set_metadata_key(issuer.get(), good_key,
                                                   sizeof(good_key)));
}

BSSL_NAMESPACE_END
