//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef SUPPORT_OPERATOR_HIJACKER_H
#define SUPPORT_OPERATOR_HIJACKER_H

#include <cstddef>
#include <functional>
#include <memory>
#include <string>
#include <type_traits>

#include "test_macros.h"

/// Helper struct to test ADL-hijacking in containers.
///
/// The class has some additional operations to be usable in all containers.
struct operator_hijacker {
  TEST_CONSTEXPR bool operator<(const operator_hijacker&) const { return true; }
  TEST_CONSTEXPR bool operator==(const operator_hijacker&) const { return true; }
  TEST_CONSTEXPR int operator()() const { return 42; }

  template <typename T>
  friend void operator&(T&&) = delete;
  template <class T, class U>
  friend void operator,(T&&, U&&) = delete;
  template <class T, class U>
  friend void operator&&(T&&, U&&) = delete;
  template <class T, class U>
  friend void operator||(T&&, U&&) = delete;
};

static_assert(std::is_trivially_copyable<operator_hijacker>::value &&     //
                  std::is_copy_constructible<operator_hijacker>::value && //
                  std::is_move_constructible<operator_hijacker>::value && //
                  std::is_copy_assignable<operator_hijacker>::value &&    //
                  std::is_move_assignable<operator_hijacker>::value,      //
              "does not satisfy the requirements for atomic<operator_hijacker>");

template <>
struct std::hash<operator_hijacker> {
  std::size_t operator()(const operator_hijacker&) const { return 0; }
};

template <class T>
struct operator_hijacker_allocator : std::allocator<T>, operator_hijacker {
#if TEST_STD_VER <= 17
  struct rebind {
    typedef operator_hijacker_allocator<T> other;
  };
#endif
};

template <class CharT>
struct operator_hijacker_char_traits : std::char_traits<CharT>, operator_hijacker {};

#endif // SUPPORT_OPERATOR_HIJACKER_H
