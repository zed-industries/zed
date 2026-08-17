//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef TEST_STD_UTILITIES_MEMORY_UTIL_SMARTPTR_SHARED_CREATE_TYPES_H
#define TEST_STD_UTILITIES_MEMORY_UTIL_SMARTPTR_SHARED_CREATE_TYPES_H

#include <cassert>
#include <cstddef>
#include <exception>

#include "test_macros.h"

struct DestroyInReverseOrder {
  static void reset() { global_count_ = 0; }
  static int alive() { return global_count_; }

  DestroyInReverseOrder()
    : DestroyInReverseOrder(&global_count_)
  { }

  constexpr DestroyInReverseOrder(int* count)
    : count_(count), value_(*count)
  { ++*count_; }

  constexpr DestroyInReverseOrder(DestroyInReverseOrder const& other)
    : count_(other.count_), value_(*other.count_)
  { ++*count_; }

  constexpr int value() const { return value_; }

  // Ensure that we destroy these objects in the reverse order as they were created.
  constexpr ~DestroyInReverseOrder() {
    --*count_;
    assert(*count_ == value_);
  }

private:
  int* count_;
  int value_;
  static int global_count_;
};

int DestroyInReverseOrder::global_count_ = 0;

struct NonMovable {
  NonMovable() = default;
  NonMovable(NonMovable&&) = delete;
};

struct CountCopies {
  static void reset() { global_count_ = 0; }
  static int copies() { return global_count_; }

  constexpr CountCopies() : copies_(&global_count_) { }
  constexpr CountCopies(int* counter) : copies_(counter) { }
  constexpr CountCopies(CountCopies const& other) : copies_(other.copies_) { ++*copies_; }

private:
  int* copies_;
  static int global_count_;
};

int CountCopies::global_count_ = 0;

struct alignas(alignof(std::max_align_t) * 2) OverAligned { };

struct MaxAligned {
  std::max_align_t foo;
};

#ifndef TEST_HAS_NO_EXCEPTIONS
struct ThrowOnConstruction {
  struct exception : std::exception { };

  ThrowOnConstruction() { on_construct(); }
  ThrowOnConstruction(ThrowOnConstruction const&) { on_construct(); }

  static void reset() { throw_after_ = -1; }
  static void throw_after(int n) { throw_after_ = n; }

private:
  static int throw_after_;
  void on_construct() {
    if (throw_after_ == 0)
      throw exception{};

    if (throw_after_ != -1)
      --throw_after_;
  }
};

int ThrowOnConstruction::throw_after_ = -1;
#endif // TEST_HAS_NO_EXCEPTIONS

#endif // TEST_STD_UTILITIES_MEMORY_UTIL_SMARTPTR_SHARED_CREATE_TYPES_H
