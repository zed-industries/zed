// Copyright (C) 1995-1998 Eric Young (eay@cryptsoft.com)
// The DSS routines are based on patches supplied by Steven Schoch <schoch@sheba.arc.nasa.gov>.
// SPDX-License-Identifier: Apache-2.0

#include <openssl/dsa.h>

#include <stdio.h>
#include <string.h>

#include <vector>

#include <gtest/gtest.h>

#include <openssl/bn.h>
#include <openssl/crypto.h>
#include <openssl/err.h>
#include <openssl/pem.h>
#include <openssl/span.h>

#include "../test/test_util.h"


// The following values are taken from the updated Appendix 5 to FIPS PUB 186
// and also appear in Appendix 5 to FIPS PUB 186-1.

static const uint8_t seed[20] = {
    0xd5, 0x01, 0x4e, 0x4b, 0x60, 0xef, 0x2b, 0xa8, 0xb6, 0x21, 0x1b,
    0x40, 0x62, 0xba, 0x32, 0x24, 0xe0, 0x42, 0x7d, 0xd3,
};

static const uint8_t fips_p[] = {
    0x8d, 0xf2, 0xa4, 0x94, 0x49, 0x22, 0x76, 0xaa, 0x3d, 0x25, 0x75,
    0x9b, 0xb0, 0x68, 0x69, 0xcb, 0xea, 0xc0, 0xd8, 0x3a, 0xfb, 0x8d,
    0x0c, 0xf7, 0xcb, 0xb8, 0x32, 0x4f, 0x0d, 0x78, 0x82, 0xe5, 0xd0,
    0x76, 0x2f, 0xc5, 0xb7, 0x21, 0x0e, 0xaf, 0xc2, 0xe9, 0xad, 0xac,
    0x32, 0xab, 0x7a, 0xac, 0x49, 0x69, 0x3d, 0xfb, 0xf8, 0x37, 0x24,
    0xc2, 0xec, 0x07, 0x36, 0xee, 0x31, 0xc8, 0x02, 0x91,
};

static const uint8_t fips_q[] = {
    0xc7, 0x73, 0x21, 0x8c, 0x73, 0x7e, 0xc8, 0xee, 0x99, 0x3b, 0x4f,
    0x2d, 0xed, 0x30, 0xf4, 0x8e, 0xda, 0xce, 0x91, 0x5f,
};

static const uint8_t fips_g[] = {
    0x62, 0x6d, 0x02, 0x78, 0x39, 0xea, 0x0a, 0x13, 0x41, 0x31, 0x63,
    0xa5, 0x5b, 0x4c, 0xb5, 0x00, 0x29, 0x9d, 0x55, 0x22, 0x95, 0x6c,
    0xef, 0xcb, 0x3b, 0xff, 0x10, 0xf3, 0x99, 0xce, 0x2c, 0x2e, 0x71,
    0xcb, 0x9d, 0xe5, 0xfa, 0x24, 0xba, 0xbf, 0x58, 0xe5, 0xb7, 0x95,
    0x21, 0x92, 0x5c, 0x9c, 0xc4, 0x2e, 0x9f, 0x6f, 0x46, 0x4b, 0x08,
    0x8c, 0xc5, 0x72, 0xaf, 0x53, 0xe6, 0xd7, 0x88, 0x02,
};

static const uint8_t fips_x[] = {
    0x20, 0x70, 0xb3, 0x22, 0x3d, 0xba, 0x37, 0x2f, 0xde, 0x1c, 0x0f,
    0xfc, 0x7b, 0x2e, 0x3b, 0x49, 0x8b, 0x26, 0x06, 0x14,
};

static const uint8_t fips_y[] = {
    0x19, 0x13, 0x18, 0x71, 0xd7, 0x5b, 0x16, 0x12, 0xa8, 0x19, 0xf2,
    0x9d, 0x78, 0xd1, 0xb0, 0xd7, 0x34, 0x6f, 0x7a, 0xa7, 0x7b, 0xb6,
    0x2a, 0x85, 0x9b, 0xfd, 0x6c, 0x56, 0x75, 0xda, 0x9d, 0x21, 0x2d,
    0x3a, 0x36, 0xef, 0x16, 0x72, 0xef, 0x66, 0x0b, 0x8c, 0x7c, 0x25,
    0x5c, 0xc0, 0xec, 0x74, 0x85, 0x8f, 0xba, 0x33, 0xf4, 0x4c, 0x06,
    0x69, 0x96, 0x30, 0xa7, 0x6b, 0x03, 0x0e, 0xe3, 0x33,
};

static const uint8_t fips_digest[] = {
    0xa9, 0x99, 0x3e, 0x36, 0x47, 0x06, 0x81, 0x6a, 0xba, 0x3e, 0x25,
    0x71, 0x78, 0x50, 0xc2, 0x6c, 0x9c, 0xd0, 0xd8, 0x9d,
};

// fips_sig is a DER-encoded version of the r and s values in FIPS PUB 186-1.
static const uint8_t fips_sig[] = {
    0x30, 0x2d, 0x02, 0x15, 0x00, 0x8b, 0xac, 0x1a, 0xb6, 0x64, 0x10,
    0x43, 0x5c, 0xb7, 0x18, 0x1f, 0x95, 0xb1, 0x6a, 0xb9, 0x7c, 0x92,
    0xb3, 0x41, 0xc0, 0x02, 0x14, 0x41, 0xe2, 0x34, 0x5f, 0x1f, 0x56,
    0xdf, 0x24, 0x58, 0xf4, 0x26, 0xd1, 0x55, 0xb4, 0xba, 0x2d, 0xb6,
    0xdc, 0xd8, 0xc8,
};

// fips_sig_negative is fips_sig with r encoded as a negative number.
static const uint8_t fips_sig_negative[] = {
    0x30, 0x2c, 0x02, 0x14, 0x8b, 0xac, 0x1a, 0xb6, 0x64, 0x10, 0x43,
    0x5c, 0xb7, 0x18, 0x1f, 0x95, 0xb1, 0x6a, 0xb9, 0x7c, 0x92, 0xb3,
    0x41, 0xc0, 0x02, 0x14, 0x41, 0xe2, 0x34, 0x5f, 0x1f, 0x56, 0xdf,
    0x24, 0x58, 0xf4, 0x26, 0xd1, 0x55, 0xb4, 0xba, 0x2d, 0xb6, 0xdc,
    0xd8, 0xc8,
};

// fip_sig_extra is fips_sig with trailing data.
static const uint8_t fips_sig_extra[] = {
    0x30, 0x2d, 0x02, 0x15, 0x00, 0x8b, 0xac, 0x1a, 0xb6, 0x64, 0x10,
    0x43, 0x5c, 0xb7, 0x18, 0x1f, 0x95, 0xb1, 0x6a, 0xb9, 0x7c, 0x92,
    0xb3, 0x41, 0xc0, 0x02, 0x14, 0x41, 0xe2, 0x34, 0x5f, 0x1f, 0x56,
    0xdf, 0x24, 0x58, 0xf4, 0x26, 0xd1, 0x55, 0xb4, 0xba, 0x2d, 0xb6,
    0xdc, 0xd8, 0xc8, 0x00,
};

// fips_sig_lengths is fips_sig with a non-minimally encoded length.
static const uint8_t fips_sig_bad_length[] = {
    0x30, 0x81, 0x2d, 0x02, 0x15, 0x00, 0x8b, 0xac, 0x1a, 0xb6, 0x64,
    0x10, 0x43, 0x5c, 0xb7, 0x18, 0x1f, 0x95, 0xb1, 0x6a, 0xb9, 0x7c,
    0x92, 0xb3, 0x41, 0xc0, 0x02, 0x14, 0x41, 0xe2, 0x34, 0x5f, 0x1f,
    0x56, 0xdf, 0x24, 0x58, 0xf4, 0x26, 0xd1, 0x55, 0xb4, 0xba, 0x2d,
    0xb6, 0xdc, 0xd8, 0xc8, 0x00,
};

// fips_sig_bad_r is fips_sig with a bad r value.
static const uint8_t fips_sig_bad_r[] = {
    0x30, 0x2d, 0x02, 0x15, 0x00, 0x8c, 0xac, 0x1a, 0xb6, 0x64, 0x10,
    0x43, 0x5c, 0xb7, 0x18, 0x1f, 0x95, 0xb1, 0x6a, 0xb9, 0x7c, 0x92,
    0xb3, 0x41, 0xc0, 0x02, 0x14, 0x41, 0xe2, 0x34, 0x5f, 0x1f, 0x56,
    0xdf, 0x24, 0x58, 0xf4, 0x26, 0xd1, 0x55, 0xb4, 0xba, 0x2d, 0xb6,
    0xdc, 0xd8, 0xc8,
};

static bssl::UniquePtr<DSA> GetFIPSDSAGroup(void) {
  bssl::UniquePtr<DSA> dsa(DSA_new());
  if (!dsa) {
    return nullptr;
  }
  bssl::UniquePtr<BIGNUM> p(BN_bin2bn(fips_p, sizeof(fips_p), nullptr));
  bssl::UniquePtr<BIGNUM> q(BN_bin2bn(fips_q, sizeof(fips_q), nullptr));
  bssl::UniquePtr<BIGNUM> g(BN_bin2bn(fips_g, sizeof(fips_g), nullptr));
  if (!p || !q || !g || !DSA_set0_pqg(dsa.get(), p.get(), q.get(), g.get())) {
    return nullptr;
  }
  // |DSA_set0_pqg| takes ownership.
  p.release();
  q.release();
  g.release();
  return dsa;
}

static bssl::UniquePtr<DSA> GetFIPSDSA(void) {
  bssl::UniquePtr<DSA> dsa = GetFIPSDSAGroup();
  if (!dsa) {
    return nullptr;
  }
  bssl::UniquePtr<BIGNUM> pub_key(BN_bin2bn(fips_y, sizeof(fips_y), nullptr));
  bssl::UniquePtr<BIGNUM> priv_key(BN_bin2bn(fips_x, sizeof(fips_x), nullptr));
  if (!pub_key || !priv_key ||
      !DSA_set0_key(dsa.get(), pub_key.get(), priv_key.get())) {
    return nullptr;
  }
  // |DSA_set0_key| takes ownership.
  pub_key.release();
  priv_key.release();
  return dsa;
}

TEST(DSATest, Generate) {
  bssl::UniquePtr<DSA> dsa(DSA_new());
  ASSERT_TRUE(dsa);
  int counter;
  unsigned long h;
  ASSERT_TRUE(DSA_generate_parameters_ex(dsa.get(), 512, seed, 20, &counter, &h,
                                         nullptr));
  EXPECT_EQ(counter, 105);
  EXPECT_EQ(h, 2u);

  auto expect_bn_bytes = [](const char *msg, const BIGNUM *bn,
                            bssl::Span<const uint8_t> bytes) {
    std::vector<uint8_t> buf(BN_num_bytes(bn));
    BN_bn2bin(bn, buf.data());
    EXPECT_EQ(Bytes(buf), Bytes(bytes)) << msg;
  };
  expect_bn_bytes("q value is wrong", DSA_get0_q(dsa.get()), fips_q);
  expect_bn_bytes("p value is wrong", DSA_get0_p(dsa.get()), fips_p);
  expect_bn_bytes("g value is wrong", DSA_get0_g(dsa.get()), fips_g);

  ASSERT_TRUE(DSA_generate_key(dsa.get()));

  std::vector<uint8_t> sig(DSA_size(dsa.get()));
  unsigned sig_len;
  ASSERT_TRUE(DSA_sign(0, fips_digest, sizeof(fips_digest), sig.data(),
                       &sig_len, dsa.get()));

  EXPECT_EQ(1, DSA_verify(0, fips_digest, sizeof(fips_digest), sig.data(),
                          sig_len, dsa.get()));
}

TEST(DSATest, GenerateParamsTooLarge) {
  bssl::UniquePtr<DSA> dsa(DSA_new());
  ASSERT_TRUE(dsa);
  EXPECT_FALSE(DSA_generate_parameters_ex(
      dsa.get(), 10001, /*seed=*/nullptr, /*seed_len=*/0,
      /*out_counter=*/nullptr, /*out_h=*/nullptr,
      /*cb=*/nullptr));
}

TEST(DSATest, GenerateKeyTooLarge) {
  bssl::UniquePtr<DSA> dsa = GetFIPSDSA();
  ASSERT_TRUE(dsa);
  bssl::UniquePtr<BIGNUM> large_p(BN_new());
  ASSERT_TRUE(large_p);
  ASSERT_TRUE(BN_set_bit(large_p.get(), 10001));
  ASSERT_TRUE(BN_set_bit(large_p.get(), 0));
  ASSERT_TRUE(DSA_set0_pqg(dsa.get(), /*p=*/large_p.get(), /*q=*/nullptr,
                           /*g=*/nullptr));
  large_p.release();  // |DSA_set0_pqg| takes ownership on success.

  // Don't generate DSA keys if the group is too large.
  EXPECT_FALSE(DSA_generate_key(dsa.get()));
}

TEST(DSATest, Verify) {
  bssl::UniquePtr<DSA> dsa = GetFIPSDSA();
  ASSERT_TRUE(dsa);

  EXPECT_EQ(1, DSA_verify(0, fips_digest, sizeof(fips_digest), fips_sig,
                          sizeof(fips_sig), dsa.get()));
  EXPECT_EQ(-1,
            DSA_verify(0, fips_digest, sizeof(fips_digest), fips_sig_negative,
                       sizeof(fips_sig_negative), dsa.get()));
  EXPECT_EQ(-1, DSA_verify(0, fips_digest, sizeof(fips_digest), fips_sig_extra,
                           sizeof(fips_sig_extra), dsa.get()));
  EXPECT_EQ(-1,
            DSA_verify(0, fips_digest, sizeof(fips_digest), fips_sig_bad_length,
                       sizeof(fips_sig_bad_length), dsa.get()));
  EXPECT_EQ(0, DSA_verify(0, fips_digest, sizeof(fips_digest), fips_sig_bad_r,
                          sizeof(fips_sig_bad_r), dsa.get()));
}

TEST(DSATest, InvalidGroup) {
  bssl::UniquePtr<DSA> dsa = GetFIPSDSA();
  ASSERT_TRUE(dsa);
  bssl::UniquePtr<BIGNUM> zero(BN_new());
  ASSERT_TRUE(zero);
  ASSERT_TRUE(DSA_set0_pqg(dsa.get(), /*p=*/nullptr, /*q=*/nullptr,
                           /*g=*/zero.release()));

  std::vector<uint8_t> sig(DSA_size(dsa.get()));
  unsigned sig_len;
  static const uint8_t kDigest[32] = {0};
  EXPECT_FALSE(
      DSA_sign(0, kDigest, sizeof(kDigest), sig.data(), &sig_len, dsa.get()));
  EXPECT_TRUE(
      ErrorEquals(ERR_get_error(), ERR_LIB_DSA, DSA_R_INVALID_PARAMETERS));
}

// Signing and verifying should cleanly fail when the DSA object is empty.
TEST(DSATest, MissingParameters) {
  bssl::UniquePtr<DSA> dsa(DSA_new());
  ASSERT_TRUE(dsa);
  EXPECT_EQ(-1, DSA_verify(0, fips_digest, sizeof(fips_digest), fips_sig,
                           sizeof(fips_sig), dsa.get()));

  std::vector<uint8_t> sig(DSA_size(dsa.get()));
  unsigned sig_len;
  EXPECT_FALSE(DSA_sign(0, fips_digest, sizeof(fips_digest), sig.data(),
                        &sig_len, dsa.get()));
}

// Verifying should cleanly fail when the public key is missing.
TEST(DSATest, MissingPublic) {
  bssl::UniquePtr<DSA> dsa = GetFIPSDSAGroup();
  ASSERT_TRUE(dsa);
  EXPECT_EQ(-1, DSA_verify(0, fips_digest, sizeof(fips_digest), fips_sig,
                           sizeof(fips_sig), dsa.get()));
}

// Signing should cleanly fail when the private key is missing.
TEST(DSATest, MissingPrivate) {
  bssl::UniquePtr<DSA> dsa = GetFIPSDSAGroup();
  ASSERT_TRUE(dsa);

  std::vector<uint8_t> sig(DSA_size(dsa.get()));
  unsigned sig_len;
  EXPECT_FALSE(DSA_sign(0, fips_digest, sizeof(fips_digest), sig.data(),
                        &sig_len, dsa.get()));
}

// A zero private key is invalid and can cause signing to loop forever.
TEST(DSATest, ZeroPrivateKey) {
  bssl::UniquePtr<DSA> dsa = GetFIPSDSA();
  ASSERT_TRUE(dsa);
  bssl::UniquePtr<BIGNUM> zero(BN_new());
  ASSERT_TRUE(zero);
  ASSERT_TRUE(DSA_set0_key(dsa.get(), /*pub_key=*/nullptr,
                           /*priv_key=*/zero.release()));

  static const uint8_t kZeroDigest[32] = {0};
  std::vector<uint8_t> sig(DSA_size(dsa.get()));
  unsigned sig_len;
  EXPECT_FALSE(DSA_sign(0, kZeroDigest, sizeof(kZeroDigest), sig.data(),
                        &sig_len, dsa.get()));
}

// If the "field" is actually a ring and the "generator" of the multiplicative
// subgroup is actually nilpotent with low degree, DSA signing never completes.
// Test that we give up in the infinite loop.
TEST(DSATest, NilpotentGenerator) {
  static const char kPEM[] = R"(
-----BEGIN DSA PRIVATE KEY-----
MGECAQACFQHH+MnFXh4NNlZiV/zUVb5a5ib3kwIVAOP8ZOKvDwabKzEr/moq3y1z
E3vJAhUAl/2Ylx9fWbzHdh1URsc/c6IM/TECAQECFCsjU4AZRcuks45g1NMOUeCB
Epvg
-----END DSA PRIVATE KEY-----
)";
  bssl::UniquePtr<BIO> bio(BIO_new_mem_buf(kPEM, sizeof(kPEM)));
  ASSERT_TRUE(bio);
  bssl::UniquePtr<DSA> dsa(
      PEM_read_bio_DSAPrivateKey(bio.get(), nullptr, nullptr, nullptr));
  ASSERT_TRUE(dsa);

  std::vector<uint8_t> sig(DSA_size(dsa.get()));
  unsigned sig_len;
  EXPECT_FALSE(DSA_sign(0, fips_digest, sizeof(fips_digest), sig.data(),
                        &sig_len, dsa.get()));
}

TEST(DSATest, Overwrite) {
  // Load an arbitrary DSA private key and use it.
  static const char kPEM[] = R"(
-----BEGIN DSA PRIVATE KEY-----
MIIDTgIBAAKCAQEAyH68EuravtF+7PTFBtWJkwjmp0YJmh8e2Cdpu8ci3dZf87rk
GwXzfqYkAEkW5H4Hp0cxdICKFiqfxjSaiEauOrNV+nXWZS634hZ9H47I8HnAVS0p
5MmSmPJ7NNUowymMpyB6M6hfqHl/1pZd7avbTmnzb2SZ0kw0WLWJo6vMekepYWv9
3o1Xove4ci00hnkr7Qo9Bh/+z84jgeT2/MTdsCVtbuMv/mbcYLhCKVWPBozDZr/D
qwhGTlomsTRvP3WIbem3b5eYhQaPuMsKiAzntcinoxQXWrIoZB+xJyF/sI013uBI
i9ePSxY3704U4QGxVM0aR/6fzORz5kh8ZjhhywIdAI9YBUR6eoGevUaLq++qXiYW
TgXBXlyqE32ESbkCggEBAL/c5GerO5g25D0QsfgVIJtlZHQOwYauuWoUudaQiyf6
VhWLBNNTAGldkFGdtxsA42uqqZSXCki25LvN6PscGGvFy8oPWaa9TGt+l9Z5ZZiV
ShNpg71V9YuImsPB3BrQ4L6nZLfhBt6InzJ6KqjDNdg7u6lgnFKue7l6khzqNxbM
RgxHWMq7PkhMcl+RzpqbiGcxSHqraxldutqCWsnZzhKh4d4GdunuRY8GiFo0Axkb
Kn0Il3zm81ewv08F/ocu+IZQEzxTyR8YRQ99MLVbnwhVxndEdLjjetCX82l+/uEY
5fdUy0thR8odcDsvUc/tT57I+yhnno80HbpUUNw2+/sCggEAdh1wp/9CifYIp6T8
P/rIus6KberZ2Pv/n0bl+Gv8AoToA0zhZXIfY2l0TtanKmdLqPIvjqkN0v6zGSs+
+ahR1QzMQnK718mcsQmB4X6iP5LKgJ/t0g8LrDOxc/cNycmHq76MmF9RN5NEBz4+
PAnRIftm/b0UQflP6uy3gRQP2X7P8ZebCytOPKTZC4oLyCtvPevSkCiiauq/RGjL
k6xqRgLxMtmuyhT+dcVbtllV1p1xd9Bppnk17/kR5VCefo/e/7DHu163izRDW8tx
SrEmiVyVkRijY3bVZii7LPfMz5eEAWEDJRuFwyNv3i6j7CKeZw2d/hzu370Ua28F
s2lmkAIcLIFUDFrbC2nViaB5ATM9ARKk6F2QwnCfGCyZ6A==
-----END DSA PRIVATE KEY-----
)";
  bssl::UniquePtr<BIO> bio(BIO_new_mem_buf(kPEM, sizeof(kPEM)));
  ASSERT_TRUE(bio);
  bssl::UniquePtr<DSA> dsa(
      PEM_read_bio_DSAPrivateKey(bio.get(), nullptr, nullptr, nullptr));
  ASSERT_TRUE(dsa);

  std::vector<uint8_t> sig(DSA_size(dsa.get()));
  unsigned sig_len;
  ASSERT_TRUE(DSA_sign(0, fips_digest, sizeof(fips_digest), sig.data(),
                       &sig_len, dsa.get()));
  sig.resize(sig_len);
  EXPECT_EQ(1, DSA_verify(0, fips_digest, sizeof(fips_digest), sig.data(),
                          sig.size(), dsa.get()));

  // Overwrite it with the sample key.
  bssl::UniquePtr<BIGNUM> p(BN_bin2bn(fips_p, sizeof(fips_p), nullptr));
  ASSERT_TRUE(p);
  bssl::UniquePtr<BIGNUM> q(BN_bin2bn(fips_q, sizeof(fips_q), nullptr));
  ASSERT_TRUE(q);
  bssl::UniquePtr<BIGNUM> g(BN_bin2bn(fips_g, sizeof(fips_g), nullptr));
  ASSERT_TRUE(g);
  ASSERT_TRUE(DSA_set0_pqg(dsa.get(), p.get(), q.get(), g.get()));
  // |DSA_set0_pqg| takes ownership on success.
  p.release();
  q.release();
  g.release();
  bssl::UniquePtr<BIGNUM> pub_key(BN_bin2bn(fips_y, sizeof(fips_y), nullptr));
  ASSERT_TRUE(pub_key);
  bssl::UniquePtr<BIGNUM> priv_key(BN_bin2bn(fips_x, sizeof(fips_x), nullptr));
  ASSERT_TRUE(priv_key);
  ASSERT_TRUE(DSA_set0_key(dsa.get(), pub_key.get(), priv_key.get()));
  // |DSA_set0_key| takes ownership on success.
  pub_key.release();
  priv_key.release();

  // The key should now work correctly for the new parameters.
  EXPECT_EQ(1, DSA_verify(0, fips_digest, sizeof(fips_digest), fips_sig,
                          sizeof(fips_sig), dsa.get()));

  // Test signing by verifying it round-trips through the real key.
  sig.resize(DSA_size(dsa.get()));
  ASSERT_TRUE(DSA_sign(0, fips_digest, sizeof(fips_digest), sig.data(),
                       &sig_len, dsa.get()));
  sig.resize(sig_len);
  dsa = GetFIPSDSA();
  ASSERT_TRUE(dsa);
  EXPECT_EQ(1, DSA_verify(0, fips_digest, sizeof(fips_digest), sig.data(),
                          sig.size(), dsa.get()));
}

TEST(DSATest, DSAPrint) {
  bssl::UniquePtr<DSA> dsa = GetFIPSDSA();
  ASSERT_TRUE(dsa);
  bssl::UniquePtr<BIO> bio(BIO_new(BIO_s_mem()));
  ASSERT_TRUE(bio);

  DSA_print(bio.get(), dsa.get(), 4);
  const uint8_t *data;
  size_t len;
  BIO_mem_contents(bio.get(), &data, &len);

  const char *expected = ""
      "    Private-Key: (512 bit)\n"
      "    priv:\n"
      "        20:70:b3:22:3d:ba:37:2f:de:1c:0f:fc:7b:2e:3b:\n"
      "        49:8b:26:06:14\n"
      "    pub:\n"
      "        19:13:18:71:d7:5b:16:12:a8:19:f2:9d:78:d1:b0:\n"
      "        d7:34:6f:7a:a7:7b:b6:2a:85:9b:fd:6c:56:75:da:\n"
      "        9d:21:2d:3a:36:ef:16:72:ef:66:0b:8c:7c:25:5c:\n"
      "        c0:ec:74:85:8f:ba:33:f4:4c:06:69:96:30:a7:6b:\n"
      "        03:0e:e3:33\n"
      "    P:\n"
      "        00:8d:f2:a4:94:49:22:76:aa:3d:25:75:9b:b0:68:\n"
      "        69:cb:ea:c0:d8:3a:fb:8d:0c:f7:cb:b8:32:4f:0d:\n"
      "        78:82:e5:d0:76:2f:c5:b7:21:0e:af:c2:e9:ad:ac:\n"
      "        32:ab:7a:ac:49:69:3d:fb:f8:37:24:c2:ec:07:36:\n"
      "        ee:31:c8:02:91\n"
      "    Q:\n"
      "        00:c7:73:21:8c:73:7e:c8:ee:99:3b:4f:2d:ed:30:\n"
      "        f4:8e:da:ce:91:5f\n"
      "    G:\n"
      "        62:6d:02:78:39:ea:0a:13:41:31:63:a5:5b:4c:b5:\n"
      "        00:29:9d:55:22:95:6c:ef:cb:3b:ff:10:f3:99:ce:\n"
      "        2c:2e:71:cb:9d:e5:fa:24:ba:bf:58:e5:b7:95:21:\n"
      "        92:5c:9c:c4:2e:9f:6f:46:4b:08:8c:c5:72:af:53:\n"
      "        e6:d7:88:02\n";
  ASSERT_EQ(Bytes(expected), Bytes(data, len));

#if !defined(OPENSSL_ANDROID)
  // On Android, when running from an APK, |tmpfile| does not work. See
  // b/36991167#comment8.
  TempFILE tmp = createTempFILE();
  ASSERT_TRUE(tmp);
  ASSERT_TRUE(DSA_print_fp(tmp.get(), dsa.get(), 4));
  fseek(tmp.get(), 0, SEEK_END);
  long fileSize = ftell(tmp.get());
  ASSERT_GT(fileSize, 0);
  rewind(tmp.get());
  std::unique_ptr<uint8_t[]> buf(new uint8_t[fileSize]);
  size_t bytesRead = fread(buf.get(), 1, fileSize, tmp.get());
  ASSERT_EQ(bytesRead, (size_t)fileSize);
  ASSERT_EQ(Bytes(expected), Bytes(buf.get(), fileSize));
#endif
}
