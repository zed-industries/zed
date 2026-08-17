//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef TEST_LIBCXX_RANGES_RANGE_ADAPTORS_RANGE_COPY_WRAP_TYPES_H
#define TEST_LIBCXX_RANGES_RANGE_ADAPTORS_RANGE_COPY_WRAP_TYPES_H

#include <ranges>
#include <cassert>
#include <concepts>
#include <type_traits>

#include "test_macros.h"

// NOTE: These types are strongly tied to the implementation of __movable_box. See the documentation
//       in __movable_box for the meaning of optimizations #1 and #2.

// Copy constructible, but neither copyable nor nothrow_copy/move_constructible. This uses the primary template.
struct CopyConstructible {
  constexpr CopyConstructible() = default;
  constexpr explicit CopyConstructible(int x) : value(x) {}
  CopyConstructible(CopyConstructible const&) noexcept(false) = default;
  CopyConstructible& operator=(CopyConstructible const&)      = delete;

  int value = -1;
};
static_assert(!std::copyable<CopyConstructible>);
static_assert(!std::is_nothrow_copy_constructible_v<CopyConstructible>);
static_assert(!std::movable<CopyConstructible>);
static_assert(!std::is_nothrow_move_constructible_v<CopyConstructible>);

// Copy constructible and movable, but not copyable. This uses the primary template, however we're
// still able to use the native move-assignment operator in this case.
struct CopyConstructibleMovable {
  constexpr CopyConstructibleMovable() = default;
  constexpr explicit CopyConstructibleMovable(int x) : value(x) {}
  CopyConstructibleMovable(CopyConstructibleMovable const&) noexcept(false) = default;
  CopyConstructibleMovable(CopyConstructibleMovable&&) noexcept(false)      = default;
  CopyConstructibleMovable& operator=(CopyConstructibleMovable const&)      = delete;

  constexpr CopyConstructibleMovable& operator=(CopyConstructibleMovable&& other) {
    value           = other.value;
    did_move_assign = true;
    return *this;
  }

  int value            = -1;
  bool did_move_assign = false;
};

// Copyable type that is not nothrow_copy/move_constructible.
// This triggers optimization #1 for the copy assignment and the move assignment.
struct Copyable {
  constexpr Copyable() = default;
  constexpr explicit Copyable(int x) : value(x) {}
  Copyable(Copyable const&) noexcept(false) = default;

  constexpr Copyable& operator=(Copyable const& other) noexcept(false) {
    value           = other.value;
    did_copy_assign = true;
    return *this;
  }

  constexpr Copyable& operator=(Copyable&& other) noexcept(false) {
    value           = other.value;
    did_move_assign = true;
    return *this;
  }

  int value            = -1;
  bool did_copy_assign = false;
  bool did_move_assign = false;
};
static_assert(std::copyable<Copyable>);
static_assert(!std::is_nothrow_copy_constructible_v<Copyable>);
static_assert(std::movable<Copyable>);
static_assert(!std::is_nothrow_move_constructible_v<Copyable>);

// Non-copyable type that is nothrow_copy_constructible and nothrow_move_constructible.
// This triggers optimization #2 for the copy assignment and the move assignment.
struct NothrowCopyConstructible {
  constexpr NothrowCopyConstructible() = default;
  constexpr explicit NothrowCopyConstructible(int x) : value(x) {}
  NothrowCopyConstructible(NothrowCopyConstructible const&) noexcept   = default;
  NothrowCopyConstructible(NothrowCopyConstructible&&) noexcept        = default;
  NothrowCopyConstructible& operator=(NothrowCopyConstructible const&) = delete;

  int value = -1;
};
static_assert(!std::copyable<NothrowCopyConstructible>);
static_assert(std::is_nothrow_copy_constructible_v<NothrowCopyConstructible>);
static_assert(!std::movable<NothrowCopyConstructible>);
static_assert(std::is_nothrow_move_constructible_v<NothrowCopyConstructible>);

// Non-copyable type that is nothrow_copy_constructible, and that is movable but NOT nothrow_move_constructible.
// This triggers optimization #2 for the copy assignment, and optimization #1 for the move assignment.
struct MovableNothrowCopyConstructible {
  constexpr MovableNothrowCopyConstructible() = default;
  constexpr explicit MovableNothrowCopyConstructible(int x) : value(x) {}
  MovableNothrowCopyConstructible(MovableNothrowCopyConstructible const&) noexcept   = default;
  MovableNothrowCopyConstructible(MovableNothrowCopyConstructible&&) noexcept(false) = default;
  constexpr MovableNothrowCopyConstructible& operator=(MovableNothrowCopyConstructible&& other) {
    value           = other.value;
    did_move_assign = true;
    return *this;
  }

  int value            = -1;
  bool did_move_assign = false;
};
static_assert(!std::copyable<MovableNothrowCopyConstructible>);
static_assert(std::is_nothrow_copy_constructible_v<MovableNothrowCopyConstructible>);
static_assert(std::movable<MovableNothrowCopyConstructible>);
static_assert(!std::is_nothrow_move_constructible_v<MovableNothrowCopyConstructible>);

#if !defined(TEST_HAS_NO_EXCEPTIONS)
// A type that we can make throw when copied from. This is used to create a
// copyable-box in the empty state.
static constexpr int THROW_WHEN_COPIED_FROM = 999;
struct ThrowsOnCopy {
  constexpr ThrowsOnCopy() = default;
  constexpr explicit ThrowsOnCopy(int x) : value(x) {}
  ThrowsOnCopy(ThrowsOnCopy const& other) {
    if (other.value == THROW_WHEN_COPIED_FROM)
      throw 0;
    else
      value = other.value;
  }

  ThrowsOnCopy& operator=(ThrowsOnCopy const&) = delete; // prevent from being copyable

  int value = -1;
};

// Creates an empty box. The only way to do that is to try assigning one box
// to another and have that fail due to an exception when calling the copy
// constructor. The assigned-to box will then be in the empty state.
inline std::ranges::__movable_box<ThrowsOnCopy> create_empty_box() {
  std::ranges::__movable_box<ThrowsOnCopy> box1;
  std::ranges::__movable_box<ThrowsOnCopy> box2(std::in_place, THROW_WHEN_COPIED_FROM);
  try {
    box1 = box2; // throws during assignment, which is implemented as a call to the copy ctor
  } catch (...) {
    // now, box1 is empty
    assert(!box1.__has_value());
    return box1;
  }
  assert(false && "should never be reached");
  return box1; // to silence warning about missing return in non-void function
}
#endif // !defined(TEST_HAS_NO_EXCEPTIONS)

#endif // TEST_LIBCXX_RANGES_RANGE_ADAPTORS_RANGE_COPY_WRAP_TYPES_H
