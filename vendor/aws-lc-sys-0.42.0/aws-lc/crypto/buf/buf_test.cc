// Copyright (c) 2017, Google Inc.
// SPDX-License-Identifier: ISC

#include <openssl/buf.h>

#include <string.h>

#include <string>

#include <gtest/gtest.h>


TEST(BufTest, Basic) {
  bssl::UniquePtr<BUF_MEM> buf(BUF_MEM_new());
  ASSERT_TRUE(buf);
  EXPECT_EQ(0u, buf->length);

  // Use BUF_MEM_reserve to increase buf->max.
  ASSERT_TRUE(BUF_MEM_reserve(buf.get(), 200));
  EXPECT_GE(buf->max, 200u);
  EXPECT_EQ(0u, buf->length);

  // BUF_MEM_reserve with a smaller cap is a no-op.
  size_t old_max = buf->max;
  ASSERT_TRUE(BUF_MEM_reserve(buf.get(), 100));
  EXPECT_EQ(old_max, buf->max);
  EXPECT_EQ(0u, buf->length);

  // BUF_MEM_grow can increase the length without reallocating.
  ASSERT_EQ(100u, BUF_MEM_grow(buf.get(), 100));
  EXPECT_EQ(100u, buf->length);
  EXPECT_EQ(old_max, buf->max);
  memset(buf->data, 'A', buf->length);

  // If BUF_MEM_reserve reallocates, it preserves the contents.
  ASSERT_TRUE(BUF_MEM_reserve(buf.get(), old_max + 1));
  ASSERT_GE(buf->max, old_max + 1);
  EXPECT_EQ(100u, buf->length);
  for (size_t i = 0; i < 100; i++) {
    EXPECT_EQ('A', buf->data[i]);
  }

  // BUF_MEM_grow should zero everything beyond buf->length.
  memset(buf->data, 'B', buf->max);
  ASSERT_EQ(150u, BUF_MEM_grow(buf.get(), 150));
  EXPECT_EQ(150u, buf->length);
  for (size_t i = 0; i < 100; i++) {
    EXPECT_EQ('B', buf->data[i]);
  }
  for (size_t i = 100; i < 150; i++) {
    EXPECT_EQ(0, buf->data[i]);
  }

  // BUF_MEM_grow can rellocate if necessary.
  size_t new_len = buf->max + 1;
  ASSERT_EQ(new_len, BUF_MEM_grow(buf.get(), new_len));
  EXPECT_GE(buf->max, new_len);
  EXPECT_EQ(new_len, buf->length);
  for (size_t i = 0; i < 100; i++) {
    EXPECT_EQ('B', buf->data[i]);
  }
  for (size_t i = 100; i < new_len; i++) {
    EXPECT_EQ(0, buf->data[i]);
  }

  // BUF_MEM_grow can shink.
  ASSERT_EQ(50u, BUF_MEM_grow(buf.get(), 50));
  EXPECT_EQ(50u, buf->length);
  for (size_t i = 0; i < 50; i++) {
    EXPECT_EQ('B', buf->data[i]);
  }
}

TEST(BufTest, Append) {
  bssl::UniquePtr<BUF_MEM> buf(BUF_MEM_new());
  ASSERT_TRUE(buf);

  ASSERT_TRUE(BUF_MEM_append(buf.get(), nullptr, 0));
  ASSERT_TRUE(BUF_MEM_append(buf.get(), "hello ", 6));
  ASSERT_TRUE(BUF_MEM_append(buf.get(), nullptr, 0));
  ASSERT_TRUE(BUF_MEM_append(buf.get(), "world", 5));
  std::string str(128, 'A');
  ASSERT_TRUE(BUF_MEM_append(buf.get(), str.data(), str.size()));

  EXPECT_EQ("hello world" + str, std::string(buf->data, buf->length));
}
