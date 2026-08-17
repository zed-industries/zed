//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef TEST_SUPPORT_TYPE_CLASSIFICATION_SWAPPABLE_H
#define TEST_SUPPORT_TYPE_CLASSIFICATION_SWAPPABLE_H

#include <concepts>

// `adl_swappable` indicates it's been swapped using ADL by maintaining a pointer to itself that
// isn't a part of the exchange. This is well-formed since we say that two `adl_swappable` objects
// are equal only if their respective `value` subobjects are equal and their respective `this`
// subobjects store the addresses of those respective `adl_swappable` objects.
class lvalue_adl_swappable {
public:
  lvalue_adl_swappable() = default;

  constexpr lvalue_adl_swappable(int value) noexcept : value_(value) {}

  constexpr lvalue_adl_swappable(lvalue_adl_swappable&& other) noexcept
      : value_(std::move(other.value_)),
        this_(this) {}

  constexpr lvalue_adl_swappable(lvalue_adl_swappable const& other) noexcept
      : value_(other.value_),
        this_(this) {}

  constexpr lvalue_adl_swappable&
  operator=(lvalue_adl_swappable other) noexcept {
    value_ = other.value_;
    return *this;
  }

  friend constexpr void swap(lvalue_adl_swappable& x,
                             lvalue_adl_swappable& y) noexcept {
    std::ranges::swap(x.value_, y.value_);
  }

  constexpr bool operator==(lvalue_adl_swappable const& other) const noexcept {
    return value_ == other.value_ && this_ == this && other.this_ == &other;
  }

private:
  int value_{};
  lvalue_adl_swappable* this_ = this;
};

class lvalue_rvalue_adl_swappable {
public:
  lvalue_rvalue_adl_swappable() = default;

  constexpr lvalue_rvalue_adl_swappable(int value) noexcept : value_(value) {}

  constexpr
  lvalue_rvalue_adl_swappable(lvalue_rvalue_adl_swappable&& other) noexcept
      : value_(std::move(other.value_)),
        this_(this) {}

  constexpr
  lvalue_rvalue_adl_swappable(lvalue_rvalue_adl_swappable const& other) noexcept
      : value_(other.value_),
        this_(this) {}

  constexpr lvalue_rvalue_adl_swappable&
  operator=(lvalue_rvalue_adl_swappable other) noexcept {
    value_ = other.value_;
    return *this;
  }

  friend constexpr void swap(lvalue_rvalue_adl_swappable& x,
                             lvalue_rvalue_adl_swappable&& y) noexcept {
    std::ranges::swap(x.value_, y.value_);
  }

  constexpr bool
  operator==(lvalue_rvalue_adl_swappable const& other) const noexcept {
    return value_ == other.value_ && this_ == this && other.this_ == &other;
  }

private:
  int value_{};
  lvalue_rvalue_adl_swappable* this_ = this;
};

class rvalue_lvalue_adl_swappable {
public:
  rvalue_lvalue_adl_swappable() = default;

  constexpr rvalue_lvalue_adl_swappable(int value) noexcept : value_(value) {}

  constexpr
  rvalue_lvalue_adl_swappable(rvalue_lvalue_adl_swappable&& other) noexcept
      : value_(std::move(other.value_)),
        this_(this) {}

  constexpr
  rvalue_lvalue_adl_swappable(rvalue_lvalue_adl_swappable const& other) noexcept
      : value_(other.value_),
        this_(this) {}

  constexpr rvalue_lvalue_adl_swappable&
  operator=(rvalue_lvalue_adl_swappable other) noexcept {
    value_ = other.value_;
    return *this;
  }

  friend constexpr void swap(rvalue_lvalue_adl_swappable&& x,
                             rvalue_lvalue_adl_swappable& y) noexcept {
    std::ranges::swap(x.value_, y.value_);
  }

  constexpr bool
  operator==(rvalue_lvalue_adl_swappable const& other) const noexcept {
    return value_ == other.value_ && this_ == this && other.this_ == &other;
  }

private:
  int value_{};
  rvalue_lvalue_adl_swappable* this_ = this;
};

class rvalue_adl_swappable {
public:
  rvalue_adl_swappable() = default;

  constexpr rvalue_adl_swappable(int value) noexcept : value_(value) {}

  constexpr rvalue_adl_swappable(rvalue_adl_swappable&& other) noexcept
      : value_(std::move(other.value_)),
        this_(this) {}

  constexpr rvalue_adl_swappable(rvalue_adl_swappable const& other) noexcept
      : value_(other.value_),
        this_(this) {}

  constexpr rvalue_adl_swappable&
  operator=(rvalue_adl_swappable other) noexcept {
    value_ = other.value_;
    return *this;
  }

  friend constexpr void swap(rvalue_adl_swappable&& x,
                             rvalue_adl_swappable&& y) noexcept {
    std::ranges::swap(x.value_, y.value_);
  }

  constexpr bool operator==(rvalue_adl_swappable const& other) const noexcept {
    return value_ == other.value_ && this_ == this && other.this_ == &other;
  }

private:
  int value_{};
  rvalue_adl_swappable* this_ = this;
};

class non_move_constructible_adl_swappable {
public:
  non_move_constructible_adl_swappable() = default;

  constexpr non_move_constructible_adl_swappable(int value) noexcept
      : value_(value) {}

  constexpr non_move_constructible_adl_swappable(
      non_move_constructible_adl_swappable&& other) noexcept
      : value_(std::move(other.value_)),
        this_(this) {}

  constexpr non_move_constructible_adl_swappable(
      non_move_constructible_adl_swappable const& other) noexcept
      : value_(other.value_),
        this_(this) {}

  constexpr non_move_constructible_adl_swappable&
  operator=(non_move_constructible_adl_swappable other) noexcept {
    value_ = other.value_;
    return *this;
  }

  friend constexpr void swap(non_move_constructible_adl_swappable& x,
                             non_move_constructible_adl_swappable& y) noexcept {
    std::ranges::swap(x.value_, y.value_);
  }

  constexpr bool
  operator==(non_move_constructible_adl_swappable const& other) const noexcept {
    return value_ == other.value_ && this_ == this && other.this_ == &other;
  }

private:
  int value_{};
  non_move_constructible_adl_swappable* this_ = this;
};

class non_move_assignable_adl_swappable {
public:
  non_move_assignable_adl_swappable() = default;

  constexpr non_move_assignable_adl_swappable(int value) noexcept
      : value_(value) {}

  non_move_assignable_adl_swappable(non_move_assignable_adl_swappable&& other) =
      delete;

  constexpr non_move_assignable_adl_swappable(
      non_move_assignable_adl_swappable const& other) noexcept
      : value_(other.value_),
        this_(this) {}

  constexpr non_move_assignable_adl_swappable&
  operator=(non_move_assignable_adl_swappable&& other) noexcept = delete;

  friend constexpr void swap(non_move_assignable_adl_swappable& x,
                             non_move_assignable_adl_swappable& y) noexcept {
    std::ranges::swap(x.value_, y.value_);
  }

  constexpr bool
  operator==(non_move_assignable_adl_swappable const& other) const noexcept {
    return value_ == other.value_ && this_ == this && other.this_ == &other;
  }

private:
  int value_{};
  non_move_assignable_adl_swappable* this_ = this;
};

class throwable_adl_swappable {
public:
  throwable_adl_swappable() = default;

  constexpr throwable_adl_swappable(int value) noexcept : value_(value) {}

  constexpr throwable_adl_swappable(throwable_adl_swappable&& other) noexcept
      : value_(std::move(other.value_)),
        this_(this) {}

  constexpr
  throwable_adl_swappable(throwable_adl_swappable const& other) noexcept
      : value_(other.value_),
        this_(this) {}

  constexpr throwable_adl_swappable&
  operator=(throwable_adl_swappable other) noexcept {
    value_ = other.value_;
    return *this;
  }

  friend constexpr void swap(throwable_adl_swappable& X,
                             throwable_adl_swappable& Y) noexcept(false) {
    std::ranges::swap(X.value_, Y.value_);
  }

  constexpr bool
  operator==(throwable_adl_swappable const& other) const noexcept {
    return value_ == other.value_ && this_ == this && other.this_ == &other;
  }

private:
  int value_{};
  throwable_adl_swappable* this_ = this;
};

#endif // TEST_SUPPORT_TYPE_CLASSIFICATION_SWAPPABLE_H
