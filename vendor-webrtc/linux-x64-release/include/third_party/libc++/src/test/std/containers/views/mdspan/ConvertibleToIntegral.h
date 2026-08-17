//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef TEST_STD_CONTAINERS_VIEWS_MDSPAN_CONVERTIBLE_TO_INTEGRAL_H
#define TEST_STD_CONTAINERS_VIEWS_MDSPAN_CONVERTIBLE_TO_INTEGRAL_H

struct IntType {
  int val;
  constexpr IntType() = default;
  constexpr IntType(int v) noexcept : val(v) {}

  constexpr bool operator==(const IntType& rhs) const { return val == rhs.val; }
  constexpr operator int() const noexcept { return val; }
  constexpr operator unsigned char() const { return static_cast<unsigned char>(val); }
  constexpr operator signed char() const noexcept { return static_cast<signed char>(val); }
};

// only non-const convertible
struct IntTypeNC {
  int val;
  constexpr IntTypeNC() = default;
  constexpr IntTypeNC(int v) noexcept : val(v) {}

  constexpr bool operator==(const IntType& rhs) const { return val == rhs.val; }
  constexpr operator int() noexcept { return val; }
  constexpr operator unsigned() { return static_cast<unsigned>(val); }
  constexpr operator char() noexcept { return static_cast<char>(val); }
};

// weird configurability of convertibility to int
template <bool conv_c, bool conv_nc, bool ctor_nt_c, bool ctor_nt_nc>
struct IntConfig {
  int val;
  constexpr explicit IntConfig(int val_) : val(val_) {}
  constexpr operator int() noexcept(ctor_nt_nc)
    requires(conv_nc)
  {
    return val;
  }
  constexpr operator int() const noexcept(ctor_nt_c)
    requires(conv_c)
  {
    return val;
  }
};

#endif // TEST_STD_CONTAINERS_VIEWS_MDSPAN_CONVERTIBLE_TO_INTEGRAL_H
