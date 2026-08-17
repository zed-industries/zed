// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC

#include <openssl/evp.h>
#include <openssl/rand.h>
#include <openssl/sha.h>
#include <gtest/gtest.h>

#include <openssl/digest.h>
#include "../../test/file_test.h"
#include "../../test/test_util.h"
#include "internal.h"

// Set values for input/output lengths used in
// |NISTTestVectors_SHAKESqueeze| test function
#define RAND_BYTES              256
#define RAND_OUT_BYTES          256

#define RAND_BYTES_x4           34
#define RAND_OUT_BLCKS          6

#define BATCHED_x4              4
#define NUM_TESTS               10
// Table containing the length of the output to squeeze for the
// initial call, followed by a output length for each subsequent call.
static const struct {
    size_t startsz, incsz;
} stride_tests[] = {
    // Test Edge Cases for SHAKE128 with blocksize of 168B
    { 1, 1 },
    { 8, 8 },
    { 9, 9 },
    { 10, 10 },
    { 1, 168 },
    { 1, 168/2 },
    { 1, 168/2-1 },
    { 1, 168/2+1 },
    { 1, 168*3 },
    { 168/2 - 1, 168 },
    { 168/2 - 1, 168-1 },
    { 168/2 - 1, 168+1 },
    { 168/2, 168 },
    { 168/2, 168-1 },
    { 168/2, 168+1 },
    { 168/2 + 1, 168 },
    { 168/2 + 1, 168-1 },
    { 168/2 + 1, 168+1 },
    { 168, 2 },
    { 168, 168 },
    { 168-1, 168 },
    { 168-1, 168-1 },
    { 168-1, 168+1 },
    { 168+1, 168 },
    { 168+1, 168-1 },
    { 168+1, 168+1 },
    { 168*3, 168 },
    { 168*3, 168 + 1 },
    { 168*3, 168 - 1 },
    { 168*3, 168/2 },
    { 168*3, 168/2 + 1 },
    { 168*3, 168/2 - 1 },
    // Test Edge Cases for SHAKE256 with blocksize of 136B
    { 1, 136 },
    { 1, 136/2 },
    { 1, 136/2-1 },
    { 1, 136/2+1 },
    { 1, 136*3 },
    { 8, 8 },
    { 9, 9 },
    { 10, 10 },
    { 136/2 - 1, 136 },
    { 136/2 - 1, 136-1 },
    { 136/2 - 1, 136+1 },
    { 136/2, 136 },
    { 136/2, 136-1 },
    { 136/2, 136+1 },
    { 136/2 + 1, 136 },
    { 136/2 + 1, 136-1 },
    { 136/2 + 1, 136+1 },
    { 136, 2 },
    { 136, 136 },
    { 136-1, 136 },
    { 136-1, 136-1 },
    { 136-1, 136+1 },
    { 136+1, 136 },
    { 136+1, 136-1 },
    { 136+1, 136+1 },
    { 136*3, 136 },
    { 136*3, 136 + 1 },
    { 136*3, 136 - 1 },
    { 136*3, 136/2 },
    { 136*3, 136/2 + 1 },
    { 136*3, 136/2 - 1 }
};

// SHA3TestVector corresponds to one test case of the NIST published file
// SHA3_256ShortMsg.txt.
// https://csrc.nist.gov/projects/cryptographic-algorithm-validation-program/secure-hashing
class SHA3TestVector {
 public:
  explicit SHA3TestVector() = default;
  ~SHA3TestVector() = default;

  bool ReadFromFileTest(FileTest *t);

  void NISTTestVectors(const EVP_MD *algorithm) const {
    uint32_t digest_length;
    std::unique_ptr<uint8_t[]> digest(new uint8_t[EVP_MD_size(algorithm)]);
    bssl::ScopedEVP_MD_CTX ctx;

    // Test the correctness via the Init, Update and Final Digest APIs.
    ASSERT_TRUE(EVP_DigestInit(ctx.get(), algorithm));
    ASSERT_TRUE(EVP_DigestUpdate(ctx.get(), msg_.data(), len_ / 8));
    ASSERT_TRUE(EVP_DigestFinal(ctx.get(), digest.get(), &digest_length));

    ASSERT_EQ(Bytes(digest.get(), EVP_MD_size(algorithm)),
              Bytes(digest_.data(), EVP_MD_size(algorithm)));

    // Test XOF-specific Digest functions with non XOF algorithms
    // Assert failure when |EVP_DigestSqueeze| or |EVP_DigestFinalXOF|
    // are called with digests different from XOF digests
    ASSERT_TRUE(EVP_DigestInit(ctx.get(), algorithm));
    ASSERT_TRUE(EVP_DigestUpdate(ctx.get(), msg_.data(), msg_.size()));
    ASSERT_FALSE(EVP_DigestSqueeze(ctx.get(), digest.get(), digest_length));
    ASSERT_FALSE(EVP_DigestFinalXOF(ctx.get(), digest.get(), digest_length));
  }

  void NISTTestVectors_SingleShot(const EVP_MD *algorithm) const {
    uint32_t digest_length;
    std::unique_ptr<uint8_t[]> digest(new uint8_t[EVP_MD_size(algorithm)]);

    // Test the correctness via the Single-Shot EVP_Digest APIs.
    ASSERT_TRUE(EVP_Digest(msg_.data(), len_ / 8, digest.get(), &digest_length,
                           algorithm, nullptr));

    ASSERT_EQ(Bytes(digest.get(), EVP_MD_size(algorithm)),
              Bytes(digest_.data(), EVP_MD_size(algorithm)));
  }

  void NISTTestVectors_SHAKE(const EVP_MD *algorithm) const {
    uint32_t digest_length = out_len_ / 8;
    std::unique_ptr<uint8_t[]> digest(new uint8_t[digest_length]);
    bssl::ScopedEVP_MD_CTX ctx;

    // Test the incremental EVP API
    ASSERT_TRUE(EVP_DigestInit(ctx.get(), algorithm));
    ASSERT_TRUE(EVP_DigestUpdate(ctx.get(), msg_.data(), msg_.size()));
    ASSERT_TRUE(EVP_DigestFinalXOF(ctx.get(), digest.get(), digest_length));
    EXPECT_EQ(Bytes(digest.get(), digest_length),
              Bytes(digest_.data(), digest_length));

    // Test the one-shot
    ASSERT_TRUE(EVP_Digest(msg_.data(), msg_.size(), digest.get(),
                           &digest_length, algorithm, nullptr));
    EXPECT_EQ(Bytes(digest.get(), digest_length),
              Bytes(digest_.data(), digest_length));
  }

  // Test SHAKE Squeeze functionality through |EVP_Digest| APIs
  void NISTTestVectors_SHAKESqueeze(const EVP_MD *algorithm) const {
    uint8_t random_bytes[RAND_BYTES];
    size_t sqd_bytes = 0, cur_test = 0, to_sq_bytes = 0;

    uint32_t digest_length = out_len_ / 8;
    std::unique_ptr<uint8_t[]> digest(new uint8_t[digest_length]);
    std::unique_ptr<uint8_t[]> digest_stream(new uint8_t[RAND_OUT_BYTES]);
    std::unique_ptr<uint8_t[]> digest_signle_shot(new uint8_t[RAND_OUT_BYTES]);

    bssl::ScopedEVP_MD_CTX ctx;

    // Test Final XOF
    // Assert fail when |EVP_DigestFinalXOF| is called as a streaming API
    ASSERT_TRUE(EVP_DigestInit(ctx.get(), algorithm));
    ASSERT_TRUE(EVP_DigestUpdate(ctx.get(), msg_.data(), msg_.size()));
    ASSERT_TRUE(EVP_DigestFinalXOF(ctx.get(), digest.get(), digest_length));
    EXPECT_EQ(Bytes(digest.get(), digest_length),
              Bytes(digest_.data(), digest_length));

    ASSERT_FALSE(EVP_DigestFinalXOF(ctx.get(), digest.get(), digest_length));
    ASSERT_FALSE(EVP_DigestSqueeze(ctx.get(), digest.get(), digest_length));

    // Test the one-shot
    // Assert success when |EVP_Digest| is called
    OPENSSL_memset(digest.get(), 0, digest_length);
    ASSERT_TRUE(EVP_Digest(msg_.data(), msg_.size(), digest.get(),
                           &digest_length, algorithm, nullptr));
    EXPECT_EQ(Bytes(digest.get(), digest_length),
              Bytes(digest_.data(), digest_length));

    // Test Final
    // Assert fail when |EVP_DigestFinal| is called for XOF algorithms
    OPENSSL_memset(digest.get(), 0, digest_length);
    ASSERT_TRUE(EVP_DigestInit(ctx.get(), algorithm));
    ASSERT_TRUE(EVP_DigestUpdate(ctx.get(), msg_.data(), msg_.size()));

    ASSERT_FALSE(EVP_DigestFinal(ctx.get(), digest.get(), &digest_length));
    ASSERT_FALSE(EVP_DigestFinalXOF(ctx.get(), digest.get(), digest_length));

    ASSERT_TRUE(EVP_DigestInit(ctx.get(), algorithm));
    ASSERT_TRUE(EVP_DigestUpdate(ctx.get(), msg_.data(), msg_.size()));
    ASSERT_TRUE(EVP_DigestFinalXOF(ctx.get(), digest.get(), digest_length));

    EXPECT_EQ(Bytes(digest.get(), digest_length),
              Bytes(digest_.data(), digest_length));

    ASSERT_FALSE(EVP_DigestFinalXOF(ctx.get(), digest.get(), digest_length));

    // Test Final XOF after Squeeze
    // Assert fail when |EVP_DigestFinalXOF| is called after |EVP_DigestSqueeze|
    ASSERT_TRUE(EVP_DigestInit_ex(ctx.get(), algorithm, NULL));
    ASSERT_TRUE(EVP_DigestSqueeze(ctx.get(), digest.get(), digest_length/2));
    ASSERT_FALSE(EVP_DigestFinalXOF(ctx.get(), digest.get() + digest_length/2,
                                                      digest_length/2));

    // Test Update after Squeeze
    // Assert fail when |EVP_DigestUpdate| is called after |EVP_DigestSqueeze|
    ASSERT_TRUE(EVP_DigestInit_ex(ctx.get(), algorithm, NULL));
    ASSERT_TRUE(EVP_DigestSqueeze(ctx.get(), digest.get(), digest_length));
    ASSERT_FALSE(EVP_DigestUpdate(ctx.get(), msg_.data(), msg_.size()));

    // Test Absorb
    // Assert success when |EVP_DigestUpdate| is called byte-by-byte
    OPENSSL_memset(digest.get(), 0, digest_length);
    ASSERT_TRUE(EVP_DigestInit_ex(ctx.get(), algorithm, NULL));
    ASSERT_TRUE(EVP_DigestUpdate(ctx.get(), nullptr, 0));
    for (const char p : msg_) {
        ASSERT_TRUE(EVP_DigestUpdate(ctx.get(), &p, 1));
    }

    ASSERT_TRUE(EVP_DigestFinalXOF(ctx.get(), digest.get(), digest_length));
    EXPECT_EQ(Bytes(digest.get(), digest_length),
              Bytes(digest_.data(), digest_length));

    // Test Squeeze
    // Assert success when |EVP_DigestSqueeze| is called byte-by-byte
    OPENSSL_memset(digest.get(), 0, digest_length);
    ASSERT_TRUE(EVP_DigestInit(ctx.get(), algorithm));
    ASSERT_TRUE(EVP_DigestUpdate(ctx.get(), msg_.data(), msg_.size()));

    for (size_t i = 0; i < digest_length; i++) {
      ASSERT_TRUE(EVP_DigestSqueeze(ctx.get(), digest.get() + i, 1));
    }

    EXPECT_EQ(Bytes(digest.get(), digest_length),
              Bytes(digest_.data(), digest_length));

    // Test Squeeze
    // Assert success when |EVP_DigestSqueeze| is called in set byte increments
    for (cur_test = 0, sqd_bytes = 0; cur_test < (int) (sizeof(stride_tests)/sizeof(stride_tests[0])); cur_test++, sqd_bytes = 0) {
      to_sq_bytes = stride_tests[cur_test].startsz;
      OPENSSL_memset(digest.get(), 0, digest_length);
      ASSERT_TRUE(EVP_DigestInit_ex(ctx.get(), algorithm, NULL));
      ASSERT_TRUE(EVP_DigestUpdate(ctx.get(), msg_.data(),  msg_.size()));

        while (sqd_bytes < digest_length) {
          if ((sqd_bytes + to_sq_bytes) > digest_length) {
            to_sq_bytes = digest_length - sqd_bytes;
          }
          ASSERT_TRUE(EVP_DigestSqueeze(ctx.get(), digest.get() + sqd_bytes, to_sq_bytes));
          sqd_bytes += to_sq_bytes;
          to_sq_bytes = stride_tests[cur_test].incsz;
        }
      EXPECT_EQ(Bytes(digest.get(), digest_length),
            Bytes(digest_.data(), digest_length));
    }

    // Test Squeeze Exhaustive
    // Assert success when |EVP_DigestSqueeze| is called in all possible byte increments
    for (to_sq_bytes = 1; to_sq_bytes < digest_length; to_sq_bytes++) {
      OPENSSL_memset(digest.get(), 0, digest_length);
      ASSERT_TRUE(EVP_DigestInit_ex(ctx.get(), algorithm, NULL));
      ASSERT_TRUE(EVP_DigestUpdate(ctx.get(), msg_.data(),  msg_.size()));

      for (sqd_bytes = 0; sqd_bytes <= digest_length - to_sq_bytes; sqd_bytes+=to_sq_bytes) {
        ASSERT_TRUE(EVP_DigestSqueeze(ctx.get(), digest.get() + sqd_bytes, to_sq_bytes));
      }

      if ((digest_length - sqd_bytes) > 0) {
        ASSERT_TRUE(EVP_DigestSqueeze(ctx.get(), digest.get() + sqd_bytes, digest_length - sqd_bytes));
      }

      EXPECT_EQ(Bytes(digest.get(), digest_length),
            Bytes(digest_.data(), digest_length));
    }
    
    // Test Squeeze with random Input
    // Assert success when |EVP_DigestSqueeze| is called on a random message
    ASSERT_TRUE(RAND_bytes(random_bytes, RAND_BYTES));

    ASSERT_TRUE(EVP_DigestInit_ex(ctx.get(), algorithm, NULL));
    ASSERT_TRUE(EVP_DigestUpdate(ctx.get(), random_bytes, RAND_BYTES));

    for (size_t i = 0; i < RAND_OUT_BYTES; i++) {
      ASSERT_TRUE(EVP_DigestSqueeze(ctx.get(), digest_stream.get() + i, 1));
    }

    ASSERT_TRUE(EVP_DigestInit_ex(ctx.get(), algorithm, NULL));
    ASSERT_TRUE(EVP_DigestUpdate(ctx.get(), random_bytes, RAND_BYTES));
    ASSERT_TRUE(EVP_DigestFinalXOF(ctx.get(), digest_signle_shot.get(), RAND_OUT_BYTES));

    EXPECT_EQ(EncodeHex(bssl::MakeConstSpan(digest_stream.get(), RAND_OUT_BYTES)),
                EncodeHex(bssl::MakeConstSpan(digest_signle_shot.get(), RAND_OUT_BYTES)));
    
    // Test Squeeze with random Input
    // Assert success when |EVP_DigestSqueeze| is called on a random message
    // in set byte increments
    for (cur_test = 0, sqd_bytes = 0; cur_test < (int) (sizeof(stride_tests)/sizeof(stride_tests[0])); cur_test++, sqd_bytes = 0) {
      to_sq_bytes = stride_tests[cur_test].startsz;
      OPENSSL_memset(digest_stream.get(), 0, RAND_OUT_BYTES);
      OPENSSL_memset(digest_signle_shot.get(), 0, RAND_OUT_BYTES);

      ASSERT_TRUE(RAND_bytes(random_bytes, RAND_BYTES));

      // Incremental Squeezes
      ASSERT_TRUE(EVP_DigestInit(ctx.get(), algorithm));
      ASSERT_TRUE(EVP_DigestUpdate(ctx.get(), random_bytes,  RAND_BYTES));

      while (sqd_bytes < RAND_OUT_BYTES) {
          if ((sqd_bytes + to_sq_bytes) > RAND_OUT_BYTES) {
              to_sq_bytes = RAND_OUT_BYTES - sqd_bytes;
          }
          ASSERT_TRUE(EVP_DigestSqueeze(ctx.get(), digest_stream.get() + sqd_bytes, to_sq_bytes));
          sqd_bytes += to_sq_bytes;
          to_sq_bytes = stride_tests[cur_test].incsz;
      }

      // Single-Shot Squeeze
      ASSERT_TRUE(EVP_DigestInit_ex(ctx.get(), algorithm, NULL));
      ASSERT_TRUE(EVP_DigestUpdate(ctx.get(), random_bytes, RAND_BYTES));
      ASSERT_TRUE(EVP_DigestFinalXOF(ctx.get(), digest_signle_shot.get(), RAND_OUT_BYTES));

      EXPECT_EQ(EncodeHex(bssl::MakeConstSpan(digest_stream.get(), RAND_OUT_BYTES)),
              EncodeHex(bssl::MakeConstSpan(digest_signle_shot.get(), RAND_OUT_BYTES)));
    }

    // Test Final XOF without Update
    // Assert fail when |EVP_DigestFinalXOF| is called as a streaming API
    OPENSSL_memset(digest_signle_shot.get(), 0, RAND_OUT_BYTES);
    OPENSSL_memset(digest_stream.get(), 0, RAND_OUT_BYTES);

    ASSERT_TRUE(EVP_DigestInit(ctx.get(), algorithm));
    ASSERT_TRUE(EVP_DigestFinalXOF(ctx.get(), digest_signle_shot.get(), RAND_OUT_BYTES));

    ASSERT_TRUE(EVP_DigestInit(ctx.get(), algorithm));
    ASSERT_TRUE(EVP_DigestSqueeze(ctx.get(), digest_stream.get(), RAND_OUT_BYTES/2));
    ASSERT_TRUE(EVP_DigestSqueeze(ctx.get(), digest_stream.get() + RAND_OUT_BYTES/2,
                                                      RAND_OUT_BYTES/2));

    EXPECT_EQ(EncodeHex(bssl::MakeConstSpan(digest_stream.get(), RAND_OUT_BYTES)),
            EncodeHex(bssl::MakeConstSpan(digest_signle_shot.get(), RAND_OUT_BYTES)));
}

 private:
  uint32_t len_;
  uint32_t out_len_;
  std::vector<uint8_t> msg_;
  std::vector<uint8_t> digest_;
};

// Read the |key| attribute from |file_test| and convert it to an integer.
template <typename T>
bool FileTestReadInt(FileTest *file_test, T *out, const std::string &key) {
  std::string s;
  return file_test->GetAttribute(&s, key) &&
         testing::internal::ParseInt32(
             testing::Message() << "The value " << s.data()
                                << " is not convertable to an integer.",
             s.data(), (int *)out);
}

bool SHA3TestVector::ReadFromFileTest(FileTest *t) {
  if (t->HasAttribute("Outputlen")) {
    if (!FileTestReadInt(t, &out_len_, "Outputlen")) {
      return false;
    }
  }

  if (t->HasAttribute("Len")) {
    if (!FileTestReadInt(t, &len_, "Len")) {
      return false;
    }
  }

  if (!t->GetBytes(&msg_, "Msg") || !t->GetBytes(&digest_, "MD")) {
    return false;
  }

  return true;
}

TEST(SHA3Test, NISTTestVectors) {
  FileTestGTest("crypto/fipsmodule/sha/testvectors/SHA3_224ShortMsg.txt",
                [](FileTest *t) {
                  SHA3TestVector test_vec;
                  EXPECT_TRUE(test_vec.ReadFromFileTest(t));
                  const EVP_MD *algorithm = EVP_sha3_224();
                  test_vec.NISTTestVectors(algorithm);
                });
  FileTestGTest("crypto/fipsmodule/sha/testvectors/SHA3_256ShortMsg.txt",
                [](FileTest *t) {
                  SHA3TestVector test_vec;
                  EXPECT_TRUE(test_vec.ReadFromFileTest(t));
                  const EVP_MD *algorithm = EVP_sha3_256();
                  test_vec.NISTTestVectors(algorithm);
                });
  FileTestGTest("crypto/fipsmodule/sha/testvectors/SHA3_384ShortMsg.txt",
                [](FileTest *t) {
                  SHA3TestVector test_vec;
                  EXPECT_TRUE(test_vec.ReadFromFileTest(t));
                  const EVP_MD *algorithm = EVP_sha3_384();
                  test_vec.NISTTestVectors(algorithm);
                });
  FileTestGTest("crypto/fipsmodule/sha/testvectors/SHA3_512ShortMsg.txt",
                [](FileTest *t) {
                  SHA3TestVector test_vec;
                  EXPECT_TRUE(test_vec.ReadFromFileTest(t));
                  const EVP_MD *algorithm = EVP_sha3_512();
                  test_vec.NISTTestVectors(algorithm);
                });

  FileTestGTest("crypto/fipsmodule/sha/testvectors/SHA3_224LongMsg.txt",
                [](FileTest *t) {
                  SHA3TestVector test_vec;
                  EXPECT_TRUE(test_vec.ReadFromFileTest(t));
                  const EVP_MD *algorithm = EVP_sha3_224();
                  test_vec.NISTTestVectors(algorithm);
                });
  FileTestGTest("crypto/fipsmodule/sha/testvectors/SHA3_256LongMsg.txt",
                [](FileTest *t) {
                  SHA3TestVector test_vec;
                  EXPECT_TRUE(test_vec.ReadFromFileTest(t));
                  const EVP_MD *algorithm = EVP_sha3_256();
                  test_vec.NISTTestVectors(algorithm);
                });
  FileTestGTest("crypto/fipsmodule/sha/testvectors/SHA3_384LongMsg.txt",
                [](FileTest *t) {
                  SHA3TestVector test_vec;
                  EXPECT_TRUE(test_vec.ReadFromFileTest(t));
                  const EVP_MD *algorithm = EVP_sha3_384();
                  test_vec.NISTTestVectors(algorithm);
                });
  FileTestGTest("crypto/fipsmodule/sha/testvectors/SHA3_512LongMsg.txt",
                [](FileTest *t) {
                  SHA3TestVector test_vec;
                  EXPECT_TRUE(test_vec.ReadFromFileTest(t));
                  const EVP_MD *algorithm = EVP_sha3_512();
                  test_vec.NISTTestVectors(algorithm);
                });
}

TEST(SHA3Test, NISTTestVectors_SingleShot) {
  FileTestGTest("crypto/fipsmodule/sha/testvectors/SHA3_224ShortMsg.txt",
                [](FileTest *t) {
                  SHA3TestVector test_vec;
                  EXPECT_TRUE(test_vec.ReadFromFileTest(t));
                  const EVP_MD *algorithm = EVP_sha3_224();
                  test_vec.NISTTestVectors_SingleShot(algorithm);
                });
  FileTestGTest("crypto/fipsmodule/sha/testvectors/SHA3_256ShortMsg.txt",
                [](FileTest *t) {
                  SHA3TestVector test_vec;
                  EXPECT_TRUE(test_vec.ReadFromFileTest(t));
                  const EVP_MD *algorithm = EVP_sha3_256();
                  test_vec.NISTTestVectors_SingleShot(algorithm);
                });
  FileTestGTest("crypto/fipsmodule/sha/testvectors/SHA3_384ShortMsg.txt",
                [](FileTest *t) {
                  SHA3TestVector test_vec;
                  EXPECT_TRUE(test_vec.ReadFromFileTest(t));
                  const EVP_MD *algorithm = EVP_sha3_384();
                  test_vec.NISTTestVectors_SingleShot(algorithm);
                });
  FileTestGTest("crypto/fipsmodule/sha/testvectors/SHA3_512ShortMsg.txt",
                [](FileTest *t) {
                  SHA3TestVector test_vec;
                  EXPECT_TRUE(test_vec.ReadFromFileTest(t));
                  const EVP_MD *algorithm = EVP_sha3_512();
                  test_vec.NISTTestVectors_SingleShot(algorithm);
                });
}

TEST(KeccakInternalTest, SqueezeOutputBufferOverflow) {
  EVP_MD_unstable_sha3_enable(true);

  KECCAK1600_CTX ctx;
  std::vector<uint8_t> out;
  std::vector<uint8_t> canary(8);
  std::fill(canary.begin(), canary.end(), 0xff);

  const size_t out_lens[] = {
      0, 1, 2, 3, 4, 5, 6, 7, 8, (1 << 5), (1 << 16) + 1};
  for (auto out_len : out_lens) {
    EXPECT_TRUE(SHA3_Init(&ctx, SHA3_384_DIGEST_BITLENGTH));
    out.resize(out_len + canary.size());
    std::copy(canary.begin(), canary.end(), out.end() - canary.size());
    Keccak1600_Squeeze(ctx.A, out.data(), out_len, ctx.block_size, 1);
    EXPECT_TRUE(std::equal(out.end() - canary.size(), out.end(),
                           canary.begin()) == true);
  }

  EVP_MD_unstable_sha3_enable(false);
}

// Test x4 batched SHAKE against 4 consecutive SHAKE calls
// Assert success when digest and digest_x4 values are equal
TEST(SHAKETest_x4, RandomMessages) {
  KECCAK1600_CTX_x4 ctx;

  uint8_t random_in[BATCHED_x4][RAND_BYTES_x4];
  uint8_t digest[BATCHED_x4][RAND_OUT_BLCKS * SHAKE128_BLOCKSIZE];
  uint8_t digest_x4[BATCHED_x4][RAND_OUT_BLCKS * SHAKE128_BLOCKSIZE];

  // Test |SHAKE128_Init_x4|, |SHAKE128_Absorb_once_x4|, and |SHAKE128_Squeezeblocks_x4| functions
  // Assert success when digest and digest_x4 values are equal
  for (int i = 0; i < NUM_TESTS; i++) {
    for (int j = 0; j < BATCHED_x4; j++) {
      OPENSSL_memset(digest[j], 0, RAND_OUT_BLCKS * SHAKE128_BLOCKSIZE);
      OPENSSL_memset(digest_x4[j], 0, RAND_OUT_BLCKS * SHAKE128_BLOCKSIZE);

      ASSERT_TRUE(RAND_bytes(random_in[j], RAND_BYTES_x4));
      ASSERT_TRUE(SHAKE128(random_in[j], RAND_BYTES_x4, digest[j],
                                              RAND_OUT_BLCKS * SHAKE128_BLOCKSIZE));
    }

    // Compute one batched x4 SHAKE128
    ASSERT_TRUE(SHAKE128_Init_x4(&ctx));
    ASSERT_TRUE(SHAKE128_Absorb_once_x4(&ctx, random_in[0], random_in[1], random_in[2], random_in[3],
                                                                                          RAND_BYTES_x4));
    ASSERT_TRUE(SHAKE128_Squeezeblocks_x4(digest_x4[0], digest_x4[1], digest_x4[2], digest_x4[3],
                                                        &ctx, RAND_OUT_BLCKS));

    for (int j = 0; j < BATCHED_x4; j++) {
      EXPECT_EQ(Bytes(digest_x4[j], RAND_OUT_BLCKS * SHAKE128_BLOCKSIZE),
                Bytes(digest[j], RAND_OUT_BLCKS * SHAKE128_BLOCKSIZE));
    }
  }

  // Test |SHAKE256_x4| function
  // Assert success when digest and digest_x4 values are equal
  for (int i = 0; i < NUM_TESTS; i++) {
    for (int j = 0; j < BATCHED_x4; j++) {
      OPENSSL_memset(digest[j], 0, RAND_OUT_BLCKS);
      OPENSSL_memset(digest_x4[j], 0, RAND_OUT_BLCKS);

      ASSERT_TRUE(RAND_bytes(random_in[j], RAND_BYTES_x4));
      SHAKE256(random_in[j], RAND_BYTES_x4, digest[j], RAND_OUT_BLCKS);
    }

    // Compute one batched x4 SHAKE128
    ASSERT_TRUE(SHAKE256_x4(random_in[0], random_in[1], random_in[2], random_in[3], RAND_BYTES_x4,
                            digest_x4[0], digest_x4[1], digest_x4[2], digest_x4[3], RAND_OUT_BLCKS));

    for (int j = 0; j < BATCHED_x4; j++) {
      EXPECT_EQ(EncodeHex(bssl::MakeConstSpan(digest_x4[j], RAND_OUT_BLCKS)),
                EncodeHex(bssl::MakeConstSpan(digest[j], RAND_OUT_BLCKS)));
    }
  }
}

TEST(SHAKETest, NISTTestVectors) {
  FileTestGTest("crypto/fipsmodule/sha/testvectors/SHAKE128VariableOut.txt",
                [](FileTest *t) {
                  SHA3TestVector test_vec;
                  EXPECT_TRUE(test_vec.ReadFromFileTest(t));
                  test_vec.NISTTestVectors_SHAKE(EVP_shake128());
                });
  FileTestGTest("crypto/fipsmodule/sha/testvectors/SHAKE256VariableOut.txt",
                [](FileTest *t) {
                  SHA3TestVector test_vec;
                  EXPECT_TRUE(test_vec.ReadFromFileTest(t));
                  test_vec.NISTTestVectors_SHAKE(EVP_shake256());
                });
  FileTestGTest("crypto/fipsmodule/sha/testvectors/SHAKE128VariableOut.txt",
                [](FileTest *t) {
                  SHA3TestVector test_vec;
                  EXPECT_TRUE(test_vec.ReadFromFileTest(t));
                  test_vec.NISTTestVectors_SHAKESqueeze(EVP_shake128());
                });
  FileTestGTest("crypto/fipsmodule/sha/testvectors/SHAKE256VariableOut.txt",
                [](FileTest *t) {
                  SHA3TestVector test_vec;
                  EXPECT_TRUE(test_vec.ReadFromFileTest(t));
                  test_vec.NISTTestVectors_SHAKESqueeze(EVP_shake256());
                });
}
