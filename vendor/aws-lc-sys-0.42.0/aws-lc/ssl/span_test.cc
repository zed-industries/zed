// Copyright (c) 2017, Google Inc.
// SPDX-License-Identifier: ISC

#include <stdio.h>
#include <vector>

#include <gtest/gtest.h>

#include <openssl/ssl.h>

BSSL_NAMESPACE_BEGIN
namespace {

static void TestCtor(Span<int> s, const int *ptr, size_t size) {
  EXPECT_EQ(s.data(), ptr);
  EXPECT_EQ(s.size(), size);
}

static void TestConstCtor(Span<const int> s, const int *ptr, size_t size) {
  EXPECT_EQ(s.data(), ptr);
  EXPECT_EQ(s.size(), size);
}

TEST(SpanTest, CtorEmpty) {
  Span<int> s;
  TestCtor(s, nullptr, 0);
}

TEST(SpanTest, CtorFromPtrAndSize) {
  std::vector<int> v = {7, 8, 9, 10};
  Span<int> s(v.data(), v.size());
  TestCtor(s, v.data(), v.size());
}

TEST(SpanTest, CtorFromVector) {
  std::vector<int> v = {1, 2};
  // Const ctor is implicit.
  TestConstCtor(v, v.data(), v.size());
  // Mutable is explicit.
  Span<int> s(v);
  TestCtor(s, v.data(), v.size());
}

TEST(SpanTest, CtorConstFromArray) {
  int v[] = {10, 11};
  // Array ctor is implicit for const and mutable T.
  TestConstCtor(v, v, 2);
  TestCtor(v, v, 2);
}

TEST(SpanTest, MakeSpan) {
  std::vector<int> v = {100, 200, 300};
  TestCtor(MakeSpan(v), v.data(), v.size());
  TestCtor(MakeSpan(v.data(), v.size()), v.data(), v.size());
  TestConstCtor(MakeSpan(v.data(), v.size()), v.data(), v.size());
  TestConstCtor(MakeSpan(v), v.data(), v.size());
}

TEST(SpanTest, MakeConstSpan) {
  std::vector<int> v = {100, 200, 300};
  TestConstCtor(MakeConstSpan(v), v.data(), v.size());
  TestConstCtor(MakeConstSpan(v.data(), v.size()), v.data(), v.size());
  // But not:
  // TestConstCtor(MakeSpan(v), v.data(), v.size());
}

TEST(SpanTest, Accessor) {
  std::vector<int> v({42, 23, 5, 101, 80});
  Span<int> s(v);
  for (size_t i = 0; i < s.size(); ++i) {
    EXPECT_EQ(s[i], v[i]);
    EXPECT_EQ(s.at(i), v.at(i));
  }
  EXPECT_EQ(s.begin(), v.data());
  EXPECT_EQ(s.end(), v.data() + v.size());
}

}  // namespace
BSSL_NAMESPACE_END
