// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC

#include <gtest/gtest.h>

#include "stdlib.h"

#include <openssl/bio.h>
#include <openssl/bytestring.h>
#include <openssl/x509.h>

#include "../test/test_util.h"
#include "../internal.h"

#define MESSAGE_LEN 8 * 1024

struct MessageDigestParams {
  const char name[40];
  const EVP_MD *(*md)(void);
};

static const struct MessageDigestParams MessageDigests[] = {
    {"MD5", EVP_md5},
    {"SHA1", EVP_sha1},
    {"SHA224", EVP_sha224},
    {"SHA256", EVP_sha256},
    {"SHA284", EVP_sha384},
    {"SHA512", EVP_sha512},
    {"SHA512_224", EVP_sha512_224},
    {"SHA512_256", EVP_sha512_256},
    {"SHA3_224", EVP_sha3_224},
    {"SHA3_256", EVP_sha3_256},
    {"SHA3_384", EVP_sha3_384},
    {"SHA3_512", EVP_sha3_512},
};

class BIOMessageDigestTest
    : public testing::TestWithParam<MessageDigestParams> {};

INSTANTIATE_TEST_SUITE_P(
    PKCS7Test, BIOMessageDigestTest, testing::ValuesIn(MessageDigests),
    [](const testing::TestParamInfo<MessageDigestParams> &params)
        -> std::string { return params.param.name; });

TEST_P(BIOMessageDigestTest, Basic) {
  uint8_t message[MESSAGE_LEN];
  uint8_t buf[2 * MESSAGE_LEN];
  std::vector<uint8_t> message_vec;
  std::vector<uint8_t> buf_vec;
  bssl::UniquePtr<BIO> bio;
  bssl::UniquePtr<BIO> bio_md;
  bssl::UniquePtr<BIO> bio_mem;
  bssl::UniquePtr<EVP_MD_CTX> ctx;
  EVP_MD *get_md = nullptr;

  OPENSSL_memset(message, 'A', sizeof(message));
  OPENSSL_memset(buf, '\0', sizeof(buf));

  const EVP_MD *md = GetParam().md();
  ASSERT_TRUE(md);

  // Simple initialization and error cases
  bio_md.reset(BIO_new(BIO_f_md()));
  ASSERT_TRUE(bio_md);
  EXPECT_FALSE(BIO_get_md(bio_md.get(), &get_md));
  EXPECT_FALSE(BIO_reset(bio_md.get()));
  EXPECT_TRUE(BIO_set_md(bio_md.get(), md));
  EVP_MD_CTX *ctx_tmp;  // |bio_md| owns the context, we just take a ref here
  EXPECT_TRUE(BIO_get_md_ctx(bio_md.get(), &ctx_tmp));
  EXPECT_EQ(EVP_MD_type(md), EVP_MD_CTX_type(ctx_tmp));
  EXPECT_EQ(md, EVP_MD_CTX_md(ctx_tmp));  // for static *EVP_MD_CTX, ptrs equal

  // The following should fail due to the passing of NULL as the argument.
  EXPECT_FALSE(BIO_ctrl(bio_md.get(), BIO_C_GET_MD, 0, nullptr));
  EXPECT_FALSE(BIO_ctrl(bio_md.get(), BIO_C_SET_MD, 0, nullptr));
  EXPECT_FALSE(BIO_ctrl(bio_md.get(), BIO_C_GET_MD_CTX, 0, nullptr));

  // The following should fail due to no support for the underlying |BIO_ctrl|
  // implementations.
  EXPECT_FALSE(BIO_ctrl(bio_md.get(), BIO_C_SET_MD_CTX, 0, nullptr));
  EXPECT_FALSE(BIO_ctrl(bio_md.get(), BIO_C_DO_STATE_MACHINE, 0, nullptr));
  EXPECT_FALSE(BIO_ctrl(bio_md.get(), BIO_CTRL_DUP, 0, nullptr));
  EXPECT_FALSE(BIO_ctrl(bio_md.get(), BIO_CTRL_GET_CALLBACK, 0, nullptr));
  EXPECT_FALSE(BIO_ctrl(bio_md.get(), BIO_CTRL_SET_CALLBACK, 0, nullptr));

  // More error cases.
  EXPECT_FALSE(BIO_read(bio_md.get(), buf, 0));
  EXPECT_FALSE(BIO_write(bio_md.get(), buf, 0));
  EXPECT_EQ(0UL, BIO_number_read(bio_md.get()));
  EXPECT_EQ(0UL, BIO_number_written(bio_md.get()));
  EXPECT_FALSE(BIO_gets(bio_md.get(), (char *)buf, EVP_MD_size(md) - 1));

  // Briefly test |BIO_get_md|.
  EXPECT_TRUE(BIO_get_md(bio_md.get(), &get_md));
  EXPECT_EQ(md, get_md);

  // Pre-initialization IO should fail, but |BIO_get_md_ctx| should do init
  bio_md.reset(BIO_new(BIO_f_md()));
  ASSERT_TRUE(bio_md);
  bio_mem.reset(BIO_new(BIO_s_mem()));
  ASSERT_TRUE(bio_mem);
  bio.reset(BIO_push(bio_md.get(), bio_mem.get()));
  ASSERT_TRUE(bio);
  EXPECT_GT(1, BIO_write(bio.get(), message, sizeof(message)));
  EXPECT_GT(1, BIO_read(bio.get(), message, sizeof(message)));
  EXPECT_TRUE(BIO_get_md_ctx(bio_md.get(), &ctx_tmp));
  ASSERT_TRUE(EVP_DigestInit_ex(ctx_tmp, md, NULL));
  EXPECT_TRUE(BIO_write(bio.get(), message, sizeof(message)));
  EXPECT_TRUE(BIO_read(bio.get(), message, sizeof(message)));
  bio_md.release();   // |bio| took ownership
  bio_mem.release();  // |bio| took ownership

  // Write-through digest BIO
  bio_md.reset(BIO_new(BIO_f_md()));
  ASSERT_TRUE(bio_md);
  EXPECT_TRUE(BIO_set_md(bio_md.get(), md));
  bio_mem.reset(BIO_new(BIO_s_mem()));
  ASSERT_TRUE(bio_mem);
  bio.reset(BIO_push(bio_md.get(), bio_mem.get()));
  ASSERT_TRUE(bio);
  EXPECT_TRUE(BIO_write(bio.get(), message, sizeof(message)));
  int digest_len = BIO_gets(bio_md.get(), (char *)buf, sizeof(buf));
  ASSERT_GE(digest_len, 0);
  buf_vec.clear();
  buf_vec.insert(buf_vec.begin(), buf, buf + digest_len);
  OPENSSL_memset(buf, '\0', sizeof(buf));
  message_vec.clear();
  int rsize;
  while ((rsize = BIO_read(bio_mem.get(), buf, sizeof(buf))) > 0) {
    message_vec.insert(message_vec.end(), buf, buf + rsize);
  }
  ctx.reset(EVP_MD_CTX_new());
  ASSERT_TRUE(EVP_DigestInit_ex(ctx.get(), md, NULL));
  ASSERT_TRUE(
      EVP_DigestUpdate(ctx.get(), message_vec.data(), message_vec.size()));
  ASSERT_TRUE(EVP_DigestFinal_ex(ctx.get(), buf, reinterpret_cast<unsigned int*>(&digest_len)));
  EXPECT_EQ(Bytes(buf_vec.data(), buf_vec.size()), Bytes(buf, digest_len));
  bio_md.release();   // |bio| took ownership
  bio_mem.release();  // |bio| took ownership

  // Read-through digest BIO
  bio_md.reset(BIO_new(BIO_f_md()));
  ASSERT_TRUE(bio_md);
  EXPECT_TRUE(BIO_set_md(bio_md.get(), md));
  bio_mem.reset(BIO_new_mem_buf(message, sizeof(message)));
  ASSERT_TRUE(bio_mem);
  bio.reset(BIO_push(bio_md.get(), bio_mem.get()));
  ASSERT_TRUE(bio);
  message_vec.clear();
  OPENSSL_memset(buf, '\0', sizeof(buf));
  while ((rsize = BIO_read(bio.get(), buf, sizeof(buf))) > 0) {
    message_vec.insert(message_vec.begin(), buf, buf + rsize);
  }
  EXPECT_EQ(Bytes(message_vec.data(), message_vec.size()),
            Bytes(message, sizeof(message)));
  digest_len = BIO_gets(bio_md.get(), (char *)buf, sizeof(buf));
  ASSERT_GE(digest_len, 0);
  buf_vec.clear();
  buf_vec.insert(buf_vec.begin(), buf, buf + digest_len);
  ctx.reset(EVP_MD_CTX_new());
  ASSERT_TRUE(EVP_DigestInit_ex(ctx.get(), md, NULL));
  ASSERT_TRUE(
      EVP_DigestUpdate(ctx.get(), message_vec.data(), message_vec.size()));
  ASSERT_TRUE(EVP_DigestFinal_ex(ctx.get(), buf, reinterpret_cast<unsigned int*>(&digest_len)));
  EXPECT_EQ(Bytes(buf, digest_len), Bytes(buf_vec.data(), buf_vec.size()));
  EXPECT_EQ(Bytes(buf_vec.data(), buf_vec.size()), Bytes(buf, digest_len));
  // Resetting |bio_md| should reset digest state, elicit different digest
  // output
  EXPECT_TRUE(BIO_reset(bio.get()));
  digest_len = BIO_gets(bio_md.get(), (char *)buf, sizeof(buf));
  EXPECT_NE(Bytes(buf_vec.data(), buf_vec.size()), Bytes(buf, digest_len));
  bio_md.release();   // |bio| took ownership
  bio_mem.release();  // |bio| took ownership
}

TEST_P(BIOMessageDigestTest, Randomized) {
  uint8_t message_buf[MESSAGE_LEN];
  uint8_t digest_buf[EVP_MAX_MD_SIZE];
  std::vector<uint8_t> message;
  std::vector<uint8_t> expected_digest;
  bssl::UniquePtr<BIO> bio;
  bssl::UniquePtr<BIO> bio_md;
  bssl::UniquePtr<BIO> bio_mem;
  bssl::UniquePtr<EVP_MD_CTX> ctx;

  const EVP_MD *md = GetParam().md();
  ASSERT_TRUE(md);

  const size_t block_size = EVP_MD_block_size(md);
  srand(42);
  std::vector<std::vector<size_t>> io_patterns = {
      {},
      {0},
      {1},
      {8, 8, 8, 8},
      {block_size - 1, 1, block_size + 1, block_size, block_size - 1},
      {4, 1, 5, 3, 2, 0, 1, MESSAGE_LEN, 133, 4555, 22, 4, 7964, 1234},
  };
  std::vector<size_t> v(1000);
  std::generate(v.begin(), v.end(), [] { return rand() % MESSAGE_LEN; }); // NOLINT(clang-analyzer-security.insecureAPI.rand)
  io_patterns.push_back(std::move(v));

  for (auto io_pattern : io_patterns) {
    message.clear();
    expected_digest.clear();
    ctx.reset(EVP_MD_CTX_new());
    EVP_DigestInit_ex(ctx.get(), md, NULL);
    // Construct overall message and its expected expected_digest
    for (auto io_size : io_pattern) {
      ASSERT_LE(io_size, sizeof(message_buf));
      char c = rand() % 256; // NOLINT(clang-analyzer-security.insecureAPI.rand)
      OPENSSL_memset(message_buf, c, io_size);
      message.insert(message.end(), &message_buf[0], &message_buf[io_size]);
    }
    EVP_DigestUpdate(ctx.get(), message.data(), message.size());
    int digest_size;
    EVP_DigestFinal_ex(ctx.get(), digest_buf, reinterpret_cast<unsigned int*>(&digest_size));
    ASSERT_EQ(EVP_MD_CTX_size(ctx.get()), (unsigned int)digest_size);
    expected_digest.insert(expected_digest.begin(), &digest_buf[0],
                           &digest_buf[digest_size]);
    OPENSSL_cleanse(digest_buf, sizeof(digest_buf));

    // Write-through digest BIO, check against expectation
    bio_md.reset(BIO_new(BIO_f_md()));
    ASSERT_TRUE(bio_md);
    EXPECT_TRUE(BIO_set_md(bio_md.get(), md));
    bio_mem.reset(BIO_new(BIO_s_mem()));
    ASSERT_TRUE(bio_mem);
    bio.reset(BIO_push(bio_md.get(), bio_mem.get()));
    ASSERT_TRUE(bio);
    int pos = 0;
    for (auto io_size : io_pattern) {
      int wsize = BIO_write(bio.get(), (char *)(message.data() + pos), io_size);
      EXPECT_EQ((int)io_size, wsize);
      pos += io_size;
    }
    digest_size =
        BIO_gets(bio_md.get(), (char *)digest_buf, sizeof(digest_buf));
    ASSERT_GE(digest_size, 0);
    ASSERT_EQ(EVP_MD_CTX_size(ctx.get()), (unsigned int)digest_size);
    EXPECT_EQ(Bytes(expected_digest.data(), expected_digest.size()),
              Bytes(digest_buf, digest_size));
    OPENSSL_cleanse(digest_buf, sizeof(digest_buf));
    bio_md.release();   // |bio| took ownership
    bio_mem.release();  // |bio| took ownership

    // Read-through digest BIO, check against expectation
    bio_md.reset(BIO_new(BIO_f_md()));
    ASSERT_TRUE(bio_md);
    EXPECT_TRUE(BIO_set_md(bio_md.get(), md));
    bio_mem.reset(BIO_new_mem_buf(message.data(), message.size()));
    ASSERT_TRUE(bio_mem);
    bio.reset(BIO_push(bio_md.get(), bio_mem.get()));
    ASSERT_TRUE(bio);
    for (auto io_size : io_pattern) {
      int rsize = BIO_read(bio.get(), message_buf, io_size);
      EXPECT_EQ((int)io_size, rsize);
    }
    EXPECT_TRUE(BIO_eof(bio.get()));
    digest_size =
        BIO_gets(bio_md.get(), (char *)digest_buf, sizeof(digest_buf));
    ASSERT_GE(digest_size, 0);
    ASSERT_EQ(EVP_MD_CTX_size(ctx.get()), (unsigned int)digest_size);
    EXPECT_EQ(Bytes(expected_digest.data(), expected_digest.size()),
              Bytes(digest_buf, digest_size));
    OPENSSL_cleanse(digest_buf, sizeof(digest_buf));
    bio_md.release();   // |bio| took ownership
    bio_mem.release();  // |bio| took ownership
  }
}
