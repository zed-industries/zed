//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef TEST_HASH_H
#define TEST_HASH_H

#include <cstddef>
#include <utility>

template <class T>
class test_hash {
  int data_;

public:
  explicit test_hash(int data = 0) : data_(data) {}

  std::size_t operator()(const T& x) const { return std::hash<T>()(x); }

  bool operator==(const test_hash& c) const { return data_ == c.data_; }
};

#endif // TEST_HASH_H
