//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef TEST_LIBCXX_RANGES_RANGE_ADAPTORS_RANGE_CHUNK_BY_TYPES_H
#define TEST_LIBCXX_RANGES_RANGE_ADAPTORS_RANGE_CHUNK_BY_TYPES_H

struct ThrowOnCopyPred {
  ThrowOnCopyPred() = default;
  ThrowOnCopyPred(const ThrowOnCopyPred&) { throw 0; }
  ThrowOnCopyPred& operator=(const ThrowOnCopyPred&) = delete;

  ThrowOnCopyPred(ThrowOnCopyPred&&)            = default;
  ThrowOnCopyPred& operator=(ThrowOnCopyPred&&) = default;

  bool operator()(int x, int y) const { return x != y; }
};

#endif // TEST_LIBCXX_RANGES_RANGE_ADAPTORS_RANGE_CHUNK_BY_TYPES_H
