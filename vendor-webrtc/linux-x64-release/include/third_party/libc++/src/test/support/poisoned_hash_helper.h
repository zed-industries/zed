// -*- C++ -*-
//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef SUPPORT_POISONED_HASH_HELPER_H
#define SUPPORT_POISONED_HASH_HELPER_H

#include <functional>
#include <cassert>
#include <cstddef>
#include <type_traits>
#include <utility>

#include "test_macros.h"
#include "type_algorithms.h"

template <class Hash, class Key, class Res = decltype(std::declval<Hash&>()(std::declval<Key>()))>
constexpr bool can_hash_impl(int) {
  return std::is_same<Res, std::size_t>::value;
}
template <class, class>
constexpr bool can_hash_impl(long) {
  return false;
}
template <class Hash, class Key>
constexpr bool can_hash() {
  return can_hash_impl<Hash, Key>(0);
}

template <class To>
struct ConvertibleToSimple {
  operator To() const { return To{}; }
};

template <class To>
struct ConvertibleTo {
  To to{};
  operator To&() & { return to; }
  operator To const&() const& { return to; }
  operator To&&() && { return std::move(to); }
  operator To const&&() const&& { return std::move(to); }
};

// Test that the specified Hash meets the requirements of an enabled hash
template <class Key, class Hash = std::hash<Key>>
TEST_CONSTEXPR_CXX20 void test_hash_enabled(Key const& key = Key{}) {
  static_assert(std::is_destructible<Hash>::value, "");

  // Enabled hash requirements
  static_assert(std::is_default_constructible<Hash>::value, "");
  static_assert(std::is_copy_constructible<Hash>::value, "");
  static_assert(std::is_move_constructible<Hash>::value, "");
  static_assert(std::is_copy_assignable<Hash>::value, "");
  static_assert(std::is_move_assignable<Hash>::value, "");

#if TEST_STD_VER > 14
  static_assert(std::is_swappable<Hash>::value, "");
#elif defined(_LIBCPP_VERSION)
  static_assert(std::__is_swappable_v<Hash>, "");
#endif

  // Hashable requirements
  static_assert(can_hash<Hash, Key&>(), "");
  static_assert(can_hash<Hash, Key const&>(), "");
  static_assert(can_hash<Hash, Key&&>(), "");
  static_assert(can_hash<Hash const, Key&>(), "");
  static_assert(can_hash<Hash const, Key const&>(), "");
  static_assert(can_hash<Hash const, Key&&>(), "");

  static_assert(can_hash<Hash, ConvertibleToSimple<Key>&>(), "");
  static_assert(can_hash<Hash, ConvertibleToSimple<Key> const&>(), "");
  static_assert(can_hash<Hash, ConvertibleToSimple<Key>&&>(), "");

  static_assert(can_hash<Hash, ConvertibleTo<Key>&>(), "");
  static_assert(can_hash<Hash, ConvertibleTo<Key> const&>(), "");
  static_assert(can_hash<Hash, ConvertibleTo<Key>&&>(), "");
  static_assert(can_hash<Hash, ConvertibleTo<Key> const&&>(), "");

  const Hash h{};
  assert(h(key) == h(key));
}

// Test that the specified Hash meets the requirements of a disabled hash.
template <class Key, class Hash = std::hash<Key>>
void test_hash_disabled() {
  // Disabled hash requirements
  static_assert(!std::is_default_constructible<Hash>::value, "");
  static_assert(!std::is_copy_constructible<Hash>::value, "");
  static_assert(!std::is_move_constructible<Hash>::value, "");
  static_assert(!std::is_copy_assignable<Hash>::value, "");
  static_assert(!std::is_move_assignable<Hash>::value, "");

  static_assert(
      !std::is_function<typename std::remove_pointer<typename std::remove_reference<Hash>::type>::type>::value, "");

  // Hashable requirements
  static_assert(!can_hash<Hash, Key&>(), "");
  static_assert(!can_hash<Hash, Key const&>(), "");
  static_assert(!can_hash<Hash, Key&&>(), "");
  static_assert(!can_hash<Hash const, Key&>(), "");
  static_assert(!can_hash<Hash const, Key const&>(), "");
  static_assert(!can_hash<Hash const, Key&&>(), "");

  static_assert(!can_hash<Hash, ConvertibleToSimple<Key>&>(), "");
  static_assert(!can_hash<Hash, ConvertibleToSimple<Key> const&>(), "");
  static_assert(!can_hash<Hash, ConvertibleToSimple<Key>&&>(), "");

  static_assert(!can_hash<Hash, ConvertibleTo<Key>&>(), "");
  static_assert(!can_hash<Hash, ConvertibleTo<Key> const&>(), "");
  static_assert(!can_hash<Hash, ConvertibleTo<Key>&&>(), "");
  static_assert(!can_hash<Hash, ConvertibleTo<Key> const&&>(), "");
}

enum Enum {};
enum EnumClass : bool {};
struct Class {};

// Each header that declares the std::hash template provides enabled
// specializations of std::hash for std::nullptr_t and all cv-unqualified
// arithmetic, enumeration, and pointer types.
#if TEST_STD_VER >= 17
using MaybeNullptr = types::type_list<std::nullptr_t>;
#else
using MaybeNullptr = types::type_list<>;
#endif
using LibraryHashTypes = types::
    concatenate_t<types::arithmetic_types, types::type_list<Enum, EnumClass, void*, void const*, Class*>, MaybeNullptr>;

struct TestHashEnabled {
  template <class T>
  void operator()() const {
    test_hash_enabled<T>();
  }
};

// Test that each of the library hash specializations for arithmetic types,
// enum types, and pointer types are available and enabled.
template <class Types = LibraryHashTypes>
void test_library_hash_specializations_available() {
  types::for_each(Types(), TestHashEnabled());
}

#endif // SUPPORT_POISONED_HASH_HELPER_H
