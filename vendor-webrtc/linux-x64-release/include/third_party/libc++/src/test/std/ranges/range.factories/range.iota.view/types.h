//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef TEST_STD_RANGES_RANGE_FACTORIES_RANGE_IOTA_VIEW_TYPES_H
#define TEST_STD_RANGES_RANGE_FACTORIES_RANGE_IOTA_VIEW_TYPES_H

#include "test_macros.h"

struct SomeInt {
  using difference_type = int;

  int value_;
  constexpr explicit SomeInt(int value = 0) : value_(value) {}

  auto operator<=>(const SomeInt&) const = default;

  friend constexpr SomeInt& operator+=(SomeInt &lhs, const SomeInt& rhs) {
    lhs.value_ += rhs.value_; return lhs;
  }
  friend constexpr SomeInt& operator-=(SomeInt &lhs, const SomeInt& rhs) {
    lhs.value_ -= rhs.value_; return lhs;
  }

  friend constexpr SomeInt& operator+=(SomeInt &lhs, difference_type rhs) {
    lhs.value_ += rhs; return lhs;
  }
  friend constexpr SomeInt& operator-=(SomeInt &lhs, difference_type rhs) {
    lhs.value_ -= rhs; return lhs;
  }

  friend constexpr SomeInt operator+(SomeInt lhs, SomeInt rhs) {
    return SomeInt{lhs.value_ + rhs.value_};
  }
  friend constexpr int operator-(SomeInt lhs, SomeInt rhs) {
    return lhs.value_ - rhs.value_;
  }

  friend constexpr SomeInt operator+(SomeInt lhs, difference_type rhs) {
    return SomeInt{lhs.value_ + rhs};
  }
  friend constexpr int operator-(SomeInt lhs, difference_type rhs) {
    return lhs.value_ - rhs;
  }

  friend constexpr SomeInt operator+(difference_type lhs, SomeInt rhs) {
    return SomeInt{lhs + rhs.value_};
  }
  friend constexpr int operator-(difference_type lhs, SomeInt rhs) {
    return lhs - rhs.value_;
  }

  constexpr SomeInt& operator++() { ++value_; return *this; }
  constexpr SomeInt  operator++(int) { auto tmp = *this; ++value_; return tmp; }
  constexpr SomeInt& operator--() { --value_; return *this; }
  constexpr SomeInt  operator--(int) { auto tmp = *this; --value_; return tmp; }
};

template<class T>
struct IntComparableWith {
  using difference_type = std::iter_difference_t<T>;

  T value_;
  constexpr explicit IntComparableWith(T value = T()) : value_(value) {}

  friend constexpr bool operator==(IntComparableWith lhs, IntComparableWith rhs) {
    return lhs.value_ == rhs.value_;
  }
  friend constexpr bool operator==(IntComparableWith lhs, T rhs) {
    return lhs.value_ == rhs;
  }
  friend constexpr bool operator==(T lhs, IntComparableWith rhs) {
    return lhs == rhs.value_;
  }

  friend constexpr IntComparableWith operator+(IntComparableWith lhs, IntComparableWith rhs) {
    return IntComparableWith{lhs.value_ + rhs.value_};
  }
  friend constexpr difference_type operator-(IntComparableWith lhs, IntComparableWith rhs) {
    return lhs.value_ - rhs.value_;
  }

  constexpr IntComparableWith& operator++() { ++value_; return *this; }
  constexpr IntComparableWith  operator++(int) { auto tmp = *this; ++value_; return tmp; }
  constexpr IntComparableWith  operator--() { --value_; return *this; }
};
template <class T>
IntComparableWith(T) -> IntComparableWith<T>;

template<class T>
struct IntSentinelWith {
  using difference_type = std::iter_difference_t<T>;

  T value_;
  constexpr explicit IntSentinelWith(T value = T()) : value_(value) {}

  friend constexpr bool operator==(IntSentinelWith lhs, IntSentinelWith rhs) {
    return lhs.value_ == rhs.value_;
  }
  friend constexpr bool operator==(IntSentinelWith lhs, T rhs) {
    return lhs.value_ == rhs;
  }
  friend constexpr bool operator==(T lhs, IntSentinelWith rhs) {
    return lhs == rhs.value_;
  }

  friend constexpr IntSentinelWith operator+(IntSentinelWith lhs, IntSentinelWith rhs) {
    return IntSentinelWith{lhs.value_ + rhs.value_};
  }
  friend constexpr difference_type operator-(IntSentinelWith lhs, IntSentinelWith rhs) {
    return lhs.value_ - rhs.value_;
  }
  friend constexpr difference_type operator-(IntSentinelWith lhs, T rhs) {
    return lhs.value_ - rhs;
  }
  friend constexpr difference_type operator-(T lhs, IntSentinelWith rhs) {
    return lhs - rhs.value_;
  }

  constexpr IntSentinelWith& operator++() { ++value_; return *this; }
  constexpr IntSentinelWith  operator++(int) { auto tmp = *this; ++value_; return tmp; }
  constexpr IntSentinelWith  operator--() { --value_; return *this; }
};
template <class T>
IntSentinelWith(T) -> IntSentinelWith<T>;

struct NotIncrementable {
  using difference_type = int;

  int value_;
  constexpr explicit NotIncrementable(int value = 0) : value_(value) {}

  bool operator==(const NotIncrementable&) const = default;

  friend constexpr NotIncrementable& operator+=(NotIncrementable &lhs, const NotIncrementable& rhs) {
    lhs.value_ += rhs.value_; return lhs;
  }
  friend constexpr NotIncrementable& operator-=(NotIncrementable &lhs, const NotIncrementable& rhs) {
    lhs.value_ -= rhs.value_; return lhs;
  }

  friend constexpr NotIncrementable operator+(NotIncrementable lhs, NotIncrementable rhs) {
    return NotIncrementable{lhs.value_ + rhs.value_};
  }
  friend constexpr int operator-(NotIncrementable lhs, NotIncrementable rhs) {
    return lhs.value_ - rhs.value_;
  }

  constexpr NotIncrementable& operator++()    { ++value_; return *this; }
  constexpr void              operator++(int) { ++value_;               }
  constexpr NotIncrementable& operator--()    { --value_; return *this; }
};
static_assert(!std::incrementable<NotIncrementable>);

struct NotDecrementable {
  using difference_type = int;

  int value_;
  constexpr explicit NotDecrementable(int value = 0) : value_(value) {}

  bool operator==(const NotDecrementable&) const = default;

  friend constexpr NotDecrementable& operator+=(NotDecrementable &lhs, const NotDecrementable& rhs) {
    lhs.value_ += rhs.value_; return lhs;
  }
  friend constexpr NotDecrementable& operator-=(NotDecrementable &lhs, const NotDecrementable& rhs) {
    lhs.value_ -= rhs.value_; return lhs;
  }

  friend constexpr NotDecrementable operator+(NotDecrementable lhs, NotDecrementable rhs) {
    return NotDecrementable{lhs.value_ + rhs.value_};
  }
  friend constexpr int operator-(NotDecrementable lhs, NotDecrementable rhs) {
    return lhs.value_ - rhs.value_;
  }

  constexpr NotDecrementable& operator++()    { ++value_; return *this; }
  constexpr void              operator++(int) { ++value_;               }
};

enum CtorKind { DefaultTo42, ValueCtor };

template<CtorKind CK>
struct Int42 {
  using difference_type = int;

  int value_;
  constexpr explicit Int42(int value) : value_(value) {}
  constexpr explicit Int42() requires (CK == DefaultTo42)
    : value_(42) {}

  bool operator==(const Int42&) const = default;

  friend constexpr Int42& operator+=(Int42 &lhs, const Int42& rhs) {
    lhs.value_ += rhs.value_; return lhs;
  }
  friend constexpr Int42& operator-=(Int42 &lhs, const Int42& rhs) {
    lhs.value_ -= rhs.value_; return lhs;
  }

  friend constexpr Int42 operator+(Int42 lhs, Int42 rhs) {
    return Int42{lhs.value_ + rhs.value_};
  }
  friend constexpr int operator-(Int42 lhs, Int42 rhs) {
    return lhs.value_ - rhs.value_;
  }

  constexpr Int42& operator++()    { ++value_; return *this; }
  constexpr void   operator++(int) { ++value_;               }
};

#endif // TEST_STD_RANGES_RANGE_FACTORIES_RANGE_IOTA_VIEW_TYPES_H
