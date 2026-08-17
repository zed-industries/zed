//===-- Utility class to manipulate fixed point numbers. --*- C++ -*-=========//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef LLVM_LIBC_SRC___SUPPORT_FIXED_POINT_FX_BITS_H
#define LLVM_LIBC_SRC___SUPPORT_FIXED_POINT_FX_BITS_H

#include "include/llvm-libc-macros/stdfix-macros.h"
#include "src/__support/CPP/bit.h"
#include "src/__support/CPP/limits.h" // numeric_limits
#include "src/__support/CPP/type_traits.h"
#include "src/__support/macros/attributes.h"   // LIBC_INLINE
#include "src/__support/macros/config.h"       // LIBC_NAMESPACE_DECL
#include "src/__support/macros/null_check.h"   // LIBC_CRASH_ON_VALUE
#include "src/__support/macros/optimization.h" // LIBC_UNLIKELY
#include "src/__support/math_extras.h"

#include "fx_rep.h"

#ifdef LIBC_COMPILER_HAS_FIXED_POINT

namespace LIBC_NAMESPACE_DECL {
namespace fixed_point {

template <typename T> struct FXBits {
private:
  using fx_rep = FXRep<T>;
  using StorageType = typename fx_rep::StorageType;

  StorageType value;

  static_assert(fx_rep::FRACTION_LEN > 0);

  static constexpr size_t FRACTION_OFFSET = 0; // Just for completeness
  static constexpr size_t INTEGRAL_OFFSET =
      fx_rep::INTEGRAL_LEN == 0 ? 0 : fx_rep::FRACTION_LEN;
  static constexpr size_t SIGN_OFFSET =
      fx_rep::SIGN_LEN == 0
          ? 0
          : ((sizeof(StorageType) * CHAR_BIT) - fx_rep::SIGN_LEN);

  static constexpr StorageType FRACTION_MASK =
      mask_trailing_ones<StorageType, fx_rep::FRACTION_LEN>()
      << FRACTION_OFFSET;
  static constexpr StorageType INTEGRAL_MASK =
      mask_trailing_ones<StorageType, fx_rep::INTEGRAL_LEN>()
      << INTEGRAL_OFFSET;
  static constexpr StorageType SIGN_MASK =
      (fx_rep::SIGN_LEN == 0 ? 0 : StorageType(1) << SIGN_OFFSET);

  // mask for <integral | fraction>
  static constexpr StorageType VALUE_MASK = INTEGRAL_MASK | FRACTION_MASK;

  // mask for <sign | integral | fraction>
  static constexpr StorageType TOTAL_MASK = SIGN_MASK | VALUE_MASK;

public:
  LIBC_INLINE constexpr FXBits() = default;

  template <typename XType> LIBC_INLINE constexpr explicit FXBits(XType x) {
    using Unqual = typename cpp::remove_cv_t<XType>;
    if constexpr (cpp::is_same_v<Unqual, T>) {
      value = cpp::bit_cast<StorageType>(x);
    } else if constexpr (cpp::is_same_v<Unqual, StorageType>) {
      value = x;
    } else {
      // We don't want accidental type promotions/conversions, so we require
      // exact type match.
      static_assert(cpp::always_false<XType>);
    }
  }

  LIBC_INLINE constexpr StorageType get_fraction() {
    return (value & FRACTION_MASK) >> FRACTION_OFFSET;
  }

  LIBC_INLINE constexpr StorageType get_integral() {
    return (value & INTEGRAL_MASK) >> INTEGRAL_OFFSET;
  }

  // returns complete bitstring representation the fixed point number
  // the bitstring is of the form: padding | sign | integral | fraction
  LIBC_INLINE constexpr StorageType get_bits() {
    return (value & TOTAL_MASK) >> FRACTION_OFFSET;
  }

  // TODO: replace bool with Sign
  LIBC_INLINE constexpr bool get_sign() {
    return static_cast<bool>((value & SIGN_MASK) >> SIGN_OFFSET);
  }

  // This represents the effective negative exponent applied to this number
  LIBC_INLINE constexpr int get_exponent() { return fx_rep::FRACTION_LEN; }

  LIBC_INLINE constexpr void set_fraction(StorageType fraction) {
    value = (value & (~FRACTION_MASK)) |
            ((fraction << FRACTION_OFFSET) & FRACTION_MASK);
  }

  LIBC_INLINE constexpr void set_integral(StorageType integral) {
    value = (value & (~INTEGRAL_MASK)) |
            ((integral << INTEGRAL_OFFSET) & INTEGRAL_MASK);
  }

  // TODO: replace bool with Sign
  LIBC_INLINE constexpr void set_sign(bool sign) {
    value = (value & (~SIGN_MASK)) |
            ((static_cast<StorageType>(sign) << SIGN_OFFSET) & SIGN_MASK);
  }

  LIBC_INLINE constexpr T get_val() const { return cpp::bit_cast<T>(value); }
};

// Bit-wise operations are not available for fixed point types yet.
template <typename T>
LIBC_INLINE constexpr cpp::enable_if_t<cpp::is_fixed_point_v<T>, T>
bit_and(T x, T y) {
  using BitType = typename FXRep<T>::StorageType;
  BitType x_bit = cpp::bit_cast<BitType>(x);
  BitType y_bit = cpp::bit_cast<BitType>(y);
  // For some reason, bit_cast cannot deduce BitType from the input.
  return cpp::bit_cast<T, BitType>(x_bit & y_bit);
}

template <typename T>
LIBC_INLINE constexpr cpp::enable_if_t<cpp::is_fixed_point_v<T>, T>
bit_or(T x, T y) {
  using BitType = typename FXRep<T>::StorageType;
  BitType x_bit = cpp::bit_cast<BitType>(x);
  BitType y_bit = cpp::bit_cast<BitType>(y);
  // For some reason, bit_cast cannot deduce BitType from the input.
  return cpp::bit_cast<T, BitType>(x_bit | y_bit);
}

template <typename T>
LIBC_INLINE constexpr cpp::enable_if_t<cpp::is_fixed_point_v<T>, T>
bit_not(T x) {
  using BitType = typename FXRep<T>::StorageType;
  BitType x_bit = cpp::bit_cast<BitType>(x);
  // For some reason, bit_cast cannot deduce BitType from the input.
  return cpp::bit_cast<T, BitType>(static_cast<BitType>(~x_bit));
}

template <typename T> LIBC_INLINE constexpr T abs(T x) {
  using FXRep = FXRep<T>;
  if constexpr (FXRep::SIGN_LEN == 0)
    return x;
  else {
    if (LIBC_UNLIKELY(x == FXRep::MIN()))
      return FXRep::MAX();
    return (x < FXRep::ZERO() ? -x : x);
  }
}

// Round-to-nearest, tie-to-(+Inf)
template <typename T> LIBC_INLINE constexpr T round(T x, int n) {
  using FXRep = FXRep<T>;
  if (LIBC_UNLIKELY(n < 0))
    n = 0;
  if (LIBC_UNLIKELY(n >= FXRep::FRACTION_LEN))
    return x;

  T round_bit = FXRep::EPS() << (FXRep::FRACTION_LEN - n - 1);
  // Check for overflow.
  if (LIBC_UNLIKELY(FXRep::MAX() - round_bit < x))
    return FXRep::MAX();

  T all_ones = bit_not(FXRep::ZERO());

  int shift = FXRep::FRACTION_LEN - n;
  T rounding_mask =
      (shift == FXRep::TOTAL_LEN) ? FXRep::ZERO() : (all_ones << shift);
  return bit_and((x + round_bit), rounding_mask);
}

// count leading sign bits
// TODO: support fixed_point_padding
template <typename T>
LIBC_INLINE constexpr cpp::enable_if_t<cpp::is_fixed_point_v<T>, int>
countls(T f) {
  using FXRep = FXRep<T>;
  using BitType = typename FXRep::StorageType;
  using FXBits = FXBits<T>;

  if constexpr (FXRep::SIGN_LEN > 0) {
    if (f < 0)
      f = bit_not(f);
  }

  BitType value_bits = FXBits(f).get_bits();
  return cpp::countl_zero(value_bits) - FXRep::SIGN_LEN;
}

// fixed-point to integer conversion
template <typename T, typename XType>
LIBC_INLINE constexpr cpp::enable_if_t<cpp::is_fixed_point_v<T>, XType>
bitsfx(T f) {
  return cpp::bit_cast<XType, T>(f);
}

// divide the two fixed-point types and return an integer result
template <typename T, typename XType>
LIBC_INLINE constexpr cpp::enable_if_t<cpp::is_fixed_point_v<T>, XType>
idiv(T x, T y) {
  using FXBits = FXBits<T>;
  using FXRep = FXRep<T>;
  using CompType = typename FXRep::CompType;

  // If the value of the second operand of the / operator is zero, the
  // behavior is undefined. Ref: ISO/IEC TR 18037:2008(E) p.g. 16
  LIBC_CRASH_ON_VALUE(y, FXRep::ZERO());

  CompType x_comp = static_cast<CompType>(FXBits(x).get_bits());
  CompType y_comp = static_cast<CompType>(FXBits(y).get_bits());

  // If an integer result of one of these functions overflows, the behavior is
  // undefined. Ref: ISO/IEC TR 18037:2008(E) p.g. 16
  CompType result = x_comp / y_comp;

  return static_cast<XType>(result);
}

} // namespace fixed_point
} // namespace LIBC_NAMESPACE_DECL

#endif // LIBC_COMPILER_HAS_FIXED_POINT

#endif // LLVM_LIBC_SRC___SUPPORT_FIXED_POINT_FX_BITS_H
