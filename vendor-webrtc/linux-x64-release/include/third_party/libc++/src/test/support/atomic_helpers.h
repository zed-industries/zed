//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef ATOMIC_HELPERS_H
#define ATOMIC_HELPERS_H

#include <cassert>
#include <cstdint>
#include <cstddef>
#include <type_traits>

#include "test_macros.h"

#if defined(TEST_COMPILER_CLANG)
#  define TEST_ATOMIC_CHAR_LOCK_FREE __CLANG_ATOMIC_CHAR_LOCK_FREE
#  define TEST_ATOMIC_SHORT_LOCK_FREE __CLANG_ATOMIC_SHORT_LOCK_FREE
#  define TEST_ATOMIC_INT_LOCK_FREE __CLANG_ATOMIC_INT_LOCK_FREE
#  define TEST_ATOMIC_LONG_LOCK_FREE __CLANG_ATOMIC_LONG_LOCK_FREE
#  define TEST_ATOMIC_LLONG_LOCK_FREE __CLANG_ATOMIC_LLONG_LOCK_FREE
#  define TEST_ATOMIC_POINTER_LOCK_FREE __CLANG_ATOMIC_POINTER_LOCK_FREE
#elif defined(TEST_COMPILER_GCC)
#  define TEST_ATOMIC_CHAR_LOCK_FREE __GCC_ATOMIC_CHAR_LOCK_FREE
#  define TEST_ATOMIC_SHORT_LOCK_FREE __GCC_ATOMIC_SHORT_LOCK_FREE
#  define TEST_ATOMIC_INT_LOCK_FREE __GCC_ATOMIC_INT_LOCK_FREE
#  define TEST_ATOMIC_LONG_LOCK_FREE __GCC_ATOMIC_LONG_LOCK_FREE
#  define TEST_ATOMIC_LLONG_LOCK_FREE __GCC_ATOMIC_LLONG_LOCK_FREE
#  define TEST_ATOMIC_POINTER_LOCK_FREE __GCC_ATOMIC_POINTER_LOCK_FREE
#elif defined(TEST_COMPILER_MSVC)
// This is lifted from STL/stl/inc/atomic on github for the purposes of
// keeping the tests compiling for MSVC's STL. It's not a perfect solution
// but at least the tests will keep running.
//
// Note MSVC's STL never produces a type that is sometimes lock free, but not always lock free.
template <class T, size_t Size = sizeof(T)>
constexpr int msvc_is_lock_free_macro_value() {
  return (Size <= 8 && (Size & (Size - 1)) == 0) ? 2 : 0;
}
#  define TEST_ATOMIC_CHAR_LOCK_FREE ::msvc_is_lock_free_macro_value<char>()
#  define TEST_ATOMIC_SHORT_LOCK_FREE ::msvc_is_lock_free_macro_value<short>()
#  define TEST_ATOMIC_INT_LOCK_FREE ::msvc_is_lock_free_macro_value<int>()
#  define TEST_ATOMIC_LONG_LOCK_FREE ::msvc_is_lock_free_macro_value<long>()
#  define TEST_ATOMIC_LLONG_LOCK_FREE ::msvc_is_lock_free_macro_value<long long>()
#  define TEST_ATOMIC_POINTER_LOCK_FREE ::msvc_is_lock_free_macro_value<void*>()
#else
#  error "Unknown compiler"
#endif

#ifdef TEST_COMPILER_CLANG
#  pragma clang diagnostic push
#  pragma clang diagnostic ignored "-Wc++11-extensions"
#endif

enum class LockFreeStatus : int { unknown = -1, never = 0, sometimes = 1, always = 2 };

// We should really be checking whether the alignment of T is greater-than-or-equal-to the alignment required
// for T to be atomic, but this is basically impossible to implement portably. Instead, we assume that any type
// aligned to at least its size is going to be atomic if there exists atomic operations for that size at all,
// which is true on most platforms. This technically reduces our test coverage in the sense that if a type has
// an alignment requirement less than its size but could still be made lockfree, LockFreeStatusInfo will report
// that we don't know whether it is lockfree or not.
#define COMPARE_TYPES(T, FundamentalT) (sizeof(T) == sizeof(FundamentalT) && TEST_ALIGNOF(T) >= sizeof(T))

template <class T>
struct LockFreeStatusInfo {
  static const LockFreeStatus value = LockFreeStatus(
      COMPARE_TYPES(T, char)
          ? TEST_ATOMIC_CHAR_LOCK_FREE
          : (COMPARE_TYPES(T, short)
                 ? TEST_ATOMIC_SHORT_LOCK_FREE
                 : (COMPARE_TYPES(T, int)
                        ? TEST_ATOMIC_INT_LOCK_FREE
                        : (COMPARE_TYPES(T, long)
                               ? TEST_ATOMIC_LONG_LOCK_FREE
                               : (COMPARE_TYPES(T, long long)
                                      ? TEST_ATOMIC_LLONG_LOCK_FREE
                                      : (COMPARE_TYPES(T, void*) ? TEST_ATOMIC_POINTER_LOCK_FREE : -1))))));

  static const bool status_known = LockFreeStatusInfo::value != LockFreeStatus::unknown;
};

#undef COMPARE_TYPES

// This doesn't work in C++03 due to issues with scoped enumerations. Just disable the test.
#if TEST_STD_VER >= 11
static_assert(LockFreeStatusInfo<char>::status_known, "");
static_assert(LockFreeStatusInfo<short>::status_known, "");
static_assert(LockFreeStatusInfo<int>::status_known, "");
static_assert(LockFreeStatusInfo<long>::status_known, "");
static_assert(LockFreeStatusInfo<void*>::status_known, "");

// long long is a bit funky: on some platforms, its alignment is 4 bytes but its size is
// 8 bytes. In that case, atomics may or may not be lockfree based on their address.
static_assert(alignof(long long) == sizeof(long long) ? LockFreeStatusInfo<long long>::status_known : true, "");

// Those should always be lock free: hardcode some expected values to make sure our tests are actually
// testing something meaningful.
static_assert(LockFreeStatusInfo<char>::value == LockFreeStatus::always, "");
static_assert(LockFreeStatusInfo<short>::value == LockFreeStatus::always, "");
static_assert(LockFreeStatusInfo<int>::value == LockFreeStatus::always, "");
#endif

// These macros are somewhat suprising to use, since they take the values 0, 1, or 2.
// To make the tests clearer, get rid of them in preference of LockFreeStatusInfo.
#undef TEST_ATOMIC_CHAR_LOCK_FREE
#undef TEST_ATOMIC_SHORT_LOCK_FREE
#undef TEST_ATOMIC_INT_LOCK_FREE
#undef TEST_ATOMIC_LONG_LOCK_FREE
#undef TEST_ATOMIC_LLONG_LOCK_FREE
#undef TEST_ATOMIC_POINTER_LOCK_FREE

#ifdef TEST_COMPILER_CLANG
#  pragma clang diagnostic pop
#endif

struct UserAtomicType {
  int i;

  explicit UserAtomicType(int d = 0) TEST_NOEXCEPT : i(d) {}

  friend bool operator==(const UserAtomicType& x, const UserAtomicType& y) { return x.i == y.i; }
};

/*

Enable these once we have P0528

struct WeirdUserAtomicType
{
    char i, j, k; // the 3 chars of doom

    explicit WeirdUserAtomicType(int d = 0) TEST_NOEXCEPT : i(d) {}

    friend bool operator==(const WeirdUserAtomicType& x, const WeirdUserAtomicType& y)
    { return x.i == y.i; }
};

struct PaddedUserAtomicType
{
    char i; int j; // probably lock-free?

    explicit PaddedUserAtomicType(int d = 0) TEST_NOEXCEPT : i(d) {}

    friend bool operator==(const PaddedUserAtomicType& x, const PaddedUserAtomicType& y)
    { return x.i == y.i; }
};

*/

struct LargeUserAtomicType {
  int a[128]; /* decidedly not lock-free */

  LargeUserAtomicType(int d = 0) TEST_NOEXCEPT {
    for (auto&& e : a)
      e = d++;
  }

  friend bool operator==(LargeUserAtomicType const& x, LargeUserAtomicType const& y) TEST_NOEXCEPT {
    for (int i = 0; i < 128; ++i)
      if (x.a[i] != y.a[i])
        return false;
    return true;
  }
};

template <template <class TestArg> class TestFunctor>
struct TestEachIntegralType {
  void operator()() const {
    TestFunctor<char>()();
    TestFunctor<signed char>()();
    TestFunctor<unsigned char>()();
    TestFunctor<short>()();
    TestFunctor<unsigned short>()();
    TestFunctor<int>()();
    TestFunctor<unsigned int>()();
    TestFunctor<long>()();
    TestFunctor<unsigned long>()();
    TestFunctor<long long>()();
    TestFunctor<unsigned long long>()();
    TestFunctor<wchar_t>()();
#if TEST_STD_VER > 17 && defined(__cpp_char8_t)
    TestFunctor<char8_t>()();
#endif
    TestFunctor<char16_t>()();
    TestFunctor<char32_t>()();
    TestFunctor<std::int8_t>()();
    TestFunctor<std::uint8_t>()();
    TestFunctor<std::int16_t>()();
    TestFunctor<std::uint16_t>()();
    TestFunctor<std::int32_t>()();
    TestFunctor<std::uint32_t>()();
    TestFunctor<std::int64_t>()();
    TestFunctor<std::uint64_t>()();
  }
};

template <template <class TestArg> class TestFunctor>
struct TestEachFloatingPointType {
  void operator()() const {
    TestFunctor<float>()();
    TestFunctor<double>()();
    TestFunctor<long double>()();
  }
};

template <template <class TestArg> class TestFunctor>
struct TestEachPointerType {
  void operator()() const {
    TestFunctor<int*>()();
    TestFunctor<const int*>()();
  }
};

template <template <class TestArg> class TestFunctor>
struct TestEachAtomicType {
  void operator()() const {
    TestEachIntegralType<TestFunctor>()();
    TestEachPointerType<TestFunctor>()();
    TestFunctor<UserAtomicType>()();
    /*
            Note: These aren't going to be lock-free,
            so some libatomic.a is necessary.
        */
    TestFunctor<LargeUserAtomicType>()();
    /*
    Enable these once we have P0528

        TestFunctor<PaddedUserAtomicType>()();
        TestFunctor<WeirdUserAtomicType>()();
*/
    TestFunctor<float>()();
    TestFunctor<double>()();
  }
};

#endif // ATOMIC_HELPERS_H
