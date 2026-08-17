//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef TEST_STD_CONTAINERS_SEQUENCES_VECTOR_VECTOR_MODIFIERS_COMMON_H
#define TEST_STD_CONTAINERS_SEQUENCES_VECTOR_VECTOR_MODIFIERS_COMMON_H

#include "test_macros.h"

#include <type_traits> // for __libcpp_is_trivially_relocatable

#ifndef TEST_HAS_NO_EXCEPTIONS
struct Throws {
  Throws() : v_(0) {}
  Throws(int v) : v_(v) {}
  Throws(const Throws& rhs) : v_(rhs.v_) {
    if (sThrows)
      throw 1;
  }
  Throws(Throws&& rhs) : v_(rhs.v_) {
    if (sThrows)
      throw 1;
  }
  Throws& operator=(const Throws& rhs) {
    v_ = rhs.v_;
    return *this;
  }
  Throws& operator=(Throws&& rhs) {
    v_ = rhs.v_;
    return *this;
  }
  int v_;
  static bool sThrows;
};

bool Throws::sThrows = false;
#endif

struct Tracker {
  int copy_assignments = 0;
  int move_assignments = 0;
};

struct TrackedAssignment {
  Tracker* tracker_;
  TEST_CONSTEXPR_CXX14 explicit TrackedAssignment(Tracker* tracker) : tracker_(tracker) {}

  TrackedAssignment(TrackedAssignment const&) = default;
  TrackedAssignment(TrackedAssignment&&)      = default;

  TEST_CONSTEXPR_CXX14 TrackedAssignment& operator=(TrackedAssignment const&) {
    tracker_->copy_assignments++;
    return *this;
  }
  TEST_CONSTEXPR_CXX14 TrackedAssignment& operator=(TrackedAssignment&&) {
    tracker_->move_assignments++;
    return *this;
  }
};

struct NonTriviallyRelocatable {
  int value_;
  TEST_CONSTEXPR NonTriviallyRelocatable() : value_(0) {}
  TEST_CONSTEXPR explicit NonTriviallyRelocatable(int v) : value_(v) {}
  TEST_CONSTEXPR NonTriviallyRelocatable(NonTriviallyRelocatable const& other) : value_(other.value_) {}
  TEST_CONSTEXPR NonTriviallyRelocatable(NonTriviallyRelocatable&& other) : value_(other.value_) {}
  TEST_CONSTEXPR_CXX14 NonTriviallyRelocatable& operator=(NonTriviallyRelocatable const& other) {
    value_ = other.value_;
    return *this;
  }
  TEST_CONSTEXPR_CXX14 NonTriviallyRelocatable& operator=(NonTriviallyRelocatable&& other) {
    value_ = other.value_;
    return *this;
  }

  TEST_CONSTEXPR_CXX14 friend bool operator==(NonTriviallyRelocatable const& a, NonTriviallyRelocatable const& b) {
    return a.value_ == b.value_;
  }
};
LIBCPP_STATIC_ASSERT(!std::__libcpp_is_trivially_relocatable<NonTriviallyRelocatable>::value, "");

#endif // TEST_STD_CONTAINERS_SEQUENCES_VECTOR_VECTOR_MODIFIERS_COMMON_H
