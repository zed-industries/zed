/*
 * Copyright (c) 2012, Google Inc. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are
 * met:
 *
 *     * Redistributions of source code must retain the above copyright
 * notice, this list of conditions and the following disclaimer.
 *     * Redistributions in binary form must reproduce the above
 * copyright notice, this list of conditions and the following disclaimer
 * in the documentation and/or other materials provided with the
 * distribution.
 *     * Neither the name of Google Inc. nor the names of its
 * contributors may be used to endorse or promote products derived from
 * this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 * "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 * LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
 * A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
 * OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 * SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
 * LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
 * DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
 * THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_GEOMETRY_LAYOUT_UNIT_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_GEOMETRY_LAYOUT_UNIT_H_

#include <climits>
#include <iosfwd>
#include <limits>
#include <optional>
#include <type_traits>

#include "base/check_op.h"
#include "base/compiler_specific.h"
#include "base/dcheck_is_on.h"
#include "base/logging.h"
#include "base/numerics/clamped_math.h"
#include "base/numerics/safe_conversions.h"
#include "build/build_config.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/forward.h"
#include "third_party/blink/renderer/platform/wtf/vector_traits.h"

namespace WTF {
class String;
}  // namespace WTF

namespace blink {

#if DCHECK_IS_ON()
#define REPORT_OVERFLOW(doesOverflow)                                          \
  DLOG_IF(ERROR, !(doesOverflow)) << "LayoutUnit overflow !(" << #doesOverflow \
                                  << ") in " << PRETTY_FUNCTION
#else
#define REPORT_OVERFLOW(doesOverflow) ((void)0)
#endif

//
// `FixedPoint` is a fixed-point math class template, with the number of bits
// for the fractional part as a template parameter.
//
// `LayoutUnit` is an instantiated class of the `FixedPoint`, storing multiples
// of 1/64 of a pixel in an `int32_t` storage.
// See: https://trac.webkit.org/wiki/LayoutUnit
//
// `TextRunLayoutUnit` stores multiples of 1/65536 of a pixel in the same
// storage, so it has less integral part than `LayoutUnit` (16/16 bits vs 26/6
// bits). Suitable for a run of text, but not for the whole layout space.
//
// `InlineLayoutUnit` stores the same precision as `TextRunLayoutUnit` using an
// `int64_t` storage. It can provide the text precision, and represent the whole
// layout space (and more, 48 bits vs 26 bits), but it's double-sized.
//
// Note, non-member functions and operators for `TextRunLayoutUnit` and
// `InlineLayoutUnit` are implemented as needed.
//
template <unsigned fractional_bits, typename Storage>
class PLATFORM_EXPORT FixedPoint {
  DISALLOW_NEW();

 public:
  using StorageType = Storage;
  using UnsignedStorageType = std::make_unsigned<Storage>::type;
  static constexpr unsigned kFractionalBits = fractional_bits;
  static constexpr unsigned kIntegralBits =
      sizeof(Storage) * 8 - kFractionalBits;
  static constexpr int kFixedPointDenominator = 1 << kFractionalBits;
  static constexpr Storage kRawValueMax = std::numeric_limits<Storage>::max();
  static constexpr Storage kRawValueMin = std::numeric_limits<Storage>::min();
  static constexpr Storage kIntMax = kRawValueMax / kFixedPointDenominator;
  static constexpr Storage kIntMin = kRawValueMin / kFixedPointDenominator;

  template <typename T>
  static constexpr Storage ClampRawValue(T raw_value) {
    return base::saturated_cast<Storage>(raw_value);
  }

  constexpr FixedPoint() : value_(0) {}

  // Creates a `FixedPoint` with the specified integer value.
  // If the specified value is smaller than `FixedPoint::Min()`, the new
  // `FixedPoint` is equivalent to `FixedPoint::Min()`.
  // If the specified value is greater than the maximum integer value which
  // `FixedPoint` can represent, the new `FixedPoint` is equivalent to
  // `FixedPoint(FixedPoint::kIntMax)` in 32-bit Arm, or is equivalent to
  // `FixedPoint::Max()` otherwise.
  constexpr explicit FixedPoint(std::signed_integral auto value)
    requires(sizeof(value) <= sizeof(int))
      : value_(0) {
    SaturatedSet(static_cast<int>(value));
  }
  constexpr explicit FixedPoint(std::unsigned_integral auto value)
    requires(sizeof(value) <= sizeof(int))
      : value_(0) {
    SaturatedSet(static_cast<unsigned>(value));
  }
  constexpr explicit FixedPoint(std::integral auto value)
    requires(sizeof(value) > sizeof(int))
      : value_(ClampRawValue(value * kFixedPointDenominator)) {}

  // The specified `value` is truncated to a multiple of `Epsilon()` near 0, and
  // is clamped by `Min()` and `Max()`. A NaN `value` produces `FixedPoint(0)`.
  constexpr explicit FixedPoint(float value)
      : value_(ClampRawValue(value * kFixedPointDenominator)) {}
  constexpr explicit FixedPoint(double value)
      : value_(ClampRawValue(value * kFixedPointDenominator)) {}

  // The specified `value` is rounded up to a multiple of `Epsilon()`, and is
  // clamped by `Min()` and `Max()`. A NaN `value` produces `FixedPoint(0)`.
  static FixedPoint FromFloatCeil(float value) {
    return FromRawValueWithClamp(ceilf(value * kFixedPointDenominator));
  }

  // The specified `value` is truncated to a multiple of `Epsilon()`, and is
  // clamped by `Min()` and `Max()`. A NaN `value` produces `FixedPoint(0)`.
  static FixedPoint FromFloatFloor(float value) {
    return FromRawValueWithClamp(floorf(value * kFixedPointDenominator));
  }

  // The specified `value` is rounded to a multiple of `Epsilon()`, and is
  // clamped by `Min()` and `Max()`. A NaN `value` produces `FixedPoint(0)`.
  static FixedPoint FromFloatRound(float value) {
    return FromRawValueWithClamp(roundf(value * kFixedPointDenominator));
  }

  static FixedPoint FromDoubleRound(double value) {
    return FromRawValueWithClamp(round(value * kFixedPointDenominator));
  }

  static constexpr FixedPoint FromRawValue(Storage raw_value) {
    FixedPoint v;
    v.value_ = raw_value;
    return v;
  }
  template <typename T>
  static constexpr FixedPoint FromRawValueWithClamp(T raw_value) {
    return FromRawValue(ClampRawValue(raw_value));
  }

  // Construct from a `FixedPoint` with different template parameters. Implicit
  // because it's lossless. For lossy conversions, use `To<>()` below instead.
  template <unsigned source_fractional_bits, typename SourceStorage>
    requires(
        sizeof(Storage) > sizeof(SourceStorage) &&
        kFractionalBits >= source_fractional_bits &&
        kIntegralBits >=
            FixedPoint<source_fractional_bits, SourceStorage>::kIntegralBits)
  FixedPoint(FixedPoint<source_fractional_bits, SourceStorage> source)
      : value_(static_cast<Storage>(source.RawValue())
               << (kFractionalBits - source_fractional_bits)) {}

  // Convert from a fixed point integer.
  template <unsigned source_fractional_bits>
    requires(source_fractional_bits == kFractionalBits)
  static constexpr FixedPoint FromFixed(Storage value) {
    return FromRawValue(value);
  }
  template <unsigned source_fractional_bits>
    requires(source_fractional_bits >= kFractionalBits)
  static constexpr FixedPoint FromFixed(std::integral auto value) {
    constexpr unsigned kBitsDiff = source_fractional_bits - kFractionalBits;
    return FromRawValueWithClamp(value >> kBitsDiff);
  }
  template <unsigned source_fractional_bits>
    requires(kFractionalBits > source_fractional_bits)
  static constexpr FixedPoint FromFixed(std::integral auto value) {
    constexpr unsigned kBitsDiff = kFractionalBits - source_fractional_bits;
    if (value >= kRawValueMax >> kBitsDiff) [[unlikely]] {
      return Max();
    }
    if (value <= kRawValueMin >> kBitsDiff) [[unlikely]] {
      return Min();
    }
    return FromRawValue(value << kBitsDiff);
  }

  // Convert to a `FixedPoint` with a different storage and/or precision.
  template <typename Target>
  constexpr Target To() const {
    return Target::template FromFixed<kFractionalBits>(RawValue());
  }

  // Convert to a different `FixedPoint` by ceiling the lost precisions (e.g.,
  // `InlineLayoutUnit` to `LayoutUnit`).
  template <typename Target>
    requires(Target::kFractionalBits < kFractionalBits &&
             sizeof(typename Target::StorageType) < sizeof(Storage))
  constexpr Target ToCeil() const {
    constexpr unsigned kBitsDiff = kFractionalBits - Target::kFractionalBits;
    Storage raw_value = RawValue() >> kBitsDiff;
    if (RawValue() & ((1 << kBitsDiff) - 1)) {
      ++raw_value;
    }
    return Target::FromRawValueWithClamp(raw_value);
  }

  constexpr Storage ToInt() const { return value_ / kFixedPointDenominator; }
  constexpr float ToFloat() const {
    return static_cast<float>(value_) / kFixedPointDenominator;
  }
  constexpr double ToDouble() const {
    return static_cast<double>(value_) / kFixedPointDenominator;
  }
  UnsignedStorageType ToUnsigned() const {
    REPORT_OVERFLOW(value_ >= 0);
    return ToInt();
  }

  // Conversion to int or unsigned is lossy. 'explicit' on these operators won't
  // work because there are also other implicit conversion paths (e.g. operator
  // bool then to int which would generate wrong result). Use toInt() and
  // toUnsigned() instead.
  operator int() const = delete;
  operator unsigned() const = delete;

  constexpr operator double() const { return ToDouble(); }
  constexpr operator float() const { return ToFloat(); }
  constexpr operator bool() const { return value_; }

  std::strong_ordering operator<=>(const FixedPoint&) const = default;
  std::partial_ordering operator<=>(double d) const { return ToDouble() <=> d; }
  std::partial_ordering operator<=>(float f) const { return ToFloat() <=> f; }

  FixedPoint operator++(int) {
    value_ = base::ClampAdd(value_, kFixedPointDenominator);
    return *this;
  }

  constexpr Storage RawValue() const { return value_; }
  inline void SetRawValue(int value) { value_ = value; }
  void SetRawValue(int64_t value) {
    if constexpr (sizeof(Storage) < sizeof(int64_t)) {
      REPORT_OVERFLOW(value > kRawValueMin && value < kRawValueMax);
    }
    value_ = static_cast<Storage>(value);
  }

  FixedPoint Abs() const { return FromRawValue(::abs(value_)); }
  Storage Ceil() const {
    if (value_ >= kRawValueMax - kFixedPointDenominator + 1) [[unlikely]] {
      return kIntMax;
    }

    if (value_ >= 0)
      return (value_ + kFixedPointDenominator - 1) / kFixedPointDenominator;
    return ToInt();
  }
  ALWAYS_INLINE Storage Round() const {
    return ToInt() + ((Fraction().RawValue() + (kFixedPointDenominator / 2)) >>
                      kFractionalBits);
  }

  Storage Floor() const {
    if (value_ <= kRawValueMin + kFixedPointDenominator - 1) [[unlikely]] {
      return kIntMin;
    }

    return value_ >> kFractionalBits;
  }

  FixedPoint ClampNegativeToZero() const {
    return value_ < 0 ? FixedPoint() : *this;
  }

  FixedPoint ClampPositiveToZero() const {
    return value_ > 0 ? FixedPoint() : *this;
  }

  FixedPoint ClampIndefiniteToZero() const {
    // We compare to |kFixedPointDenominator| here instead of |kIndefiniteSize|
    // as the operator== for LayoutUnit is inlined below.
    if (value_ == -kFixedPointDenominator)
      return FixedPoint();
    DCHECK_GE(value_, 0);
    return *this;
  }

  constexpr bool HasFraction() const {
    return RawValue() % kFixedPointDenominator;
  }
  constexpr bool IsInteger() const { return !HasFraction(); }

  FixedPoint Fraction() const {
    // Compute fraction using the mod operator to preserve the sign of the value
    // as it may affect rounding.
    return FromRawValue(RawValue() % kFixedPointDenominator);
  }

  bool MightBeSaturated() const {
    return RawValue() == kRawValueMax || RawValue() == kRawValueMin;
  }

  static constexpr float Epsilon() { return 1.0f / kFixedPointDenominator; }

  FixedPoint AddEpsilon() const {
    return FromRawValue(value_ < kRawValueMax ? value_ + 1 : value_);
  }

  static constexpr FixedPoint Max() { return FromRawValue(kRawValueMax); }
  static constexpr FixedPoint Min() { return FromRawValue(kRawValueMin); }

  // Versions of max/min that are slightly smaller/larger than max/min() to
  // allow for rounding without overflowing.
  static constexpr FixedPoint NearlyMax() {
    return FromRawValue(kRawValueMax - kFixedPointDenominator / 2);
  }
  static constexpr FixedPoint NearlyMin() {
    return FromRawValue(kRawValueMin + kFixedPointDenominator / 2);
  }

  static FixedPoint Clamp(double value) { return FromFloatFloor(value); }

  // Multiply by |m| and divide by |d| as a single ("fused") operation, avoiding
  // any saturation of the intermediate result. Rounding matches that of the
  // regular operations (i.e the result of the divide is rounded towards zero).
  FixedPoint MulDiv(FixedPoint m, FixedPoint d) const;

  // Return `std::nullopt` if `this` is the specified value.
  std::optional<FixedPoint> NullOptIf(FixedPoint null_value) const;
  std::optional<FixedPoint> NullOptIfMin() const { return NullOptIf(Min()); }

  WTF::String ToString() const;

 private:
#if defined(ARCH_CPU_ARM_FAMILY) && defined(ARCH_CPU_32_BITS) && \
    defined(COMPILER_GCC) && !BUILDFLAG(IS_NACL) && __OPTIMIZE__
  // If we're building ARM 32-bit on GCC we replace the C++ versions with some
  // native ARM assembly for speed.
  constexpr inline void SaturatedSet(int value) {
    if (std::is_constant_evaluated() || sizeof(Storage) > sizeof(int)) {
      SaturatedSetNonAsm(value);
    } else {
      SaturatedSetAsm(value);
    }
  }

  inline void SaturatedSetAsm(int value) {
    // Figure out how many bits are left for storing the integer part of
    // the fixed point number, and saturate our input to that
    enum { Saturate = 32 - kFractionalBits };

    int result;

    // The following ARM code will Saturate the passed value to the number of
    // bits used for the whole part of the fixed point representation, then
    // shift it up into place. This will result in the low
    // <kFractionalBits> bits all being 0's. When the value saturates
    // this gives a different result to from the C++ case; in the C++ code a
    // saturated value has all the low bits set to 1 (for a +ve number at
    // least). This cannot be done rapidly in ARM ... we live with the
    // difference, for the sake of speed.

    asm("ssat %[output],%[saturate],%[value]\n\t"
        "lsl  %[output],%[shift]"
        : [output] "=r"(result)
        : [value] "r"(value), [saturate] "n"(Saturate),
          [shift] "n"(kFractionalBits));

    value_ = result;
  }

  constexpr inline void SaturatedSet(unsigned value) {
    if (std::is_constant_evaluated() || sizeof(Storage) > sizeof(int)) {
      SaturatedSetNonAsm(value);
    } else {
      SaturatedSetAsm(value);
    }
  }

  inline void SaturatedSetAsm(unsigned value) {
    // Here we are being passed an unsigned value to saturate,
    // even though the result is returned as a signed integer. The ARM
    // instruction for unsigned saturation therefore needs to be given one
    // less bit (i.e. the sign bit) for the saturation to work correctly; hence
    // the '31' below.
    enum { Saturate = 31 - kFractionalBits };

    // The following ARM code will Saturate the passed value to the number of
    // bits used for the whole part of the fixed point representation, then
    // shift it up into place. This will result in the low
    // <kFractionalBits> bits all being 0's. When the value saturates
    // this gives a different result to from the C++ case; in the C++ code a
    // saturated value has all the low bits set to 1. This cannot be done
    // rapidly in ARM, so we live with the difference, for the sake of speed.

    int result;

    asm("usat %[output],%[saturate],%[value]\n\t"
        "lsl  %[output],%[shift]"
        : [output] "=r"(result)
        : [value] "r"(value), [saturate] "n"(Saturate),
          [shift] "n"(kFractionalBits));

    value_ = result;
  }
#else  // end of 32-bit ARM GCC
  ALWAYS_INLINE constexpr void SaturatedSet(int value) {
    SaturatedSetNonAsm(value);
  }

  ALWAYS_INLINE constexpr void SaturatedSet(unsigned value) {
    SaturatedSetNonAsm(value);
  }
#endif

  ALWAYS_INLINE constexpr void SaturatedSetNonAsm(int value) {
    if (value > kIntMax) {
      value_ = kRawValueMax;
    } else if (value < kIntMin) {
      value_ = kRawValueMin;
    } else {
      value_ = static_cast<UnsignedStorageType>(value) << kFractionalBits;
    }
  }

  ALWAYS_INLINE constexpr void SaturatedSetNonAsm(unsigned value) {
    if (value >= static_cast<UnsignedStorageType>(kIntMax)) {
      value_ = kRawValueMax;
    } else {
      value_ = static_cast<UnsignedStorageType>(value) << kFractionalBits;
    }
  }

  Storage value_;
};

using LayoutUnit = FixedPoint<6, int32_t>;
using TextRunLayoutUnit = FixedPoint<16, int32_t>;
using InlineLayoutUnit = FixedPoint<16, int64_t>;

// kIndefiniteSize is a special value used within layout code. It is typical
// within layout to have sizes which are only allowed to be non-negative or
// "indefinite". We use the value of "-1" to represent these indefinite values.
//
// It is common to clamp these indefinite values to zero.
// |LayoutUnit::ClampIndefiniteToZero| provides this functionality, and
// additionally DCHECKs that it isn't some other negative value.
inline constexpr LayoutUnit kIndefiniteSize(-1);

// TODO(kojii): Using three-way comparison (spaceship) operator for `int` makes
// too many cases ambiguous.
inline bool operator<=(const LayoutUnit& a, int b) {
  return a <= LayoutUnit(b);
}

inline bool operator<=(const int a, const LayoutUnit& b) {
  return LayoutUnit(a) <= b;
}

inline bool operator>=(const LayoutUnit& a, int b) {
  return a >= LayoutUnit(b);
}

inline bool operator>=(const int a, const LayoutUnit& b) {
  return LayoutUnit(a) >= b;
}

inline bool operator<(const LayoutUnit& a, int b) {
  return a < LayoutUnit(b);
}

inline bool operator<(const int a, const LayoutUnit& b) {
  return LayoutUnit(a) < b;
}

inline bool operator>(const LayoutUnit& a, int b) {
  return a > LayoutUnit(b);
}

inline bool operator>(const int a, const LayoutUnit& b) {
  return LayoutUnit(a) > b;
}

inline bool operator!=(const int a, const LayoutUnit& b) {
  return LayoutUnit(a) != b;
}

inline bool operator!=(const LayoutUnit& a, int b) {
  return a != LayoutUnit(b);
}

inline bool operator==(const LayoutUnit& a, int b) {
  return a == LayoutUnit(b);
}

inline bool operator==(const int a, const LayoutUnit& b) {
  return LayoutUnit(a) == b;
}

// For multiplication that's prone to overflow, this bounds it to
// `FixedPoint::Max()` and `FixedPoint::Min()`.
template <unsigned fractional_bits, typename RawValue>
  requires(std::is_same_v<RawValue, int32_t>)
inline FixedPoint<fractional_bits, RawValue> BoundedMultiply(
    const FixedPoint<fractional_bits, RawValue>& a,
    const FixedPoint<fractional_bits, RawValue>& b) {
  int64_t result =
      static_cast<int64_t>(a.RawValue()) * static_cast<int64_t>(b.RawValue()) /
      FixedPoint<fractional_bits, RawValue>::kFixedPointDenominator;
  int32_t high = static_cast<int32_t>(result >> 32);
  int32_t low = static_cast<int32_t>(result);
  uint32_t saturated =
      (static_cast<uint32_t>(a.RawValue() ^ b.RawValue()) >> 31) +
      FixedPoint<fractional_bits, RawValue>::kRawValueMax;
  // If the higher 32 bits does not match the lower 32 with sign extension the
  // operation overflowed.
  if (high != low >> 31)
    result = saturated;

  return FixedPoint<fractional_bits, RawValue>::FromRawValue(
      static_cast<RawValue>(result));
}

template <unsigned fractional_bits, typename RawValue>
  requires(std::is_same_v<RawValue, int32_t>)
inline FixedPoint<fractional_bits, RawValue> operator*(
    const FixedPoint<fractional_bits, RawValue>& a,
    const FixedPoint<fractional_bits, RawValue>& b) {
  return BoundedMultiply(a, b);
}

inline double operator*(const LayoutUnit& a, double b) {
  return a.ToDouble() * b;
}

inline float operator*(const LayoutUnit& a, float b) {
  return a.ToFloat() * b;
}

template <unsigned fractional_bits, typename RawValue>
inline FixedPoint<fractional_bits, RawValue> operator*(
    const FixedPoint<fractional_bits, RawValue> a,
    std::integral auto b) {
  return FixedPoint<fractional_bits, RawValue>::FromRawValue(
      base::ClampMul(a.RawValue(), b));
}

inline LayoutUnit operator*(std::integral auto a, const LayoutUnit& b) {
  return b * a;
}

constexpr float operator*(const float a, const LayoutUnit& b) {
  return a * b.ToFloat();
}

constexpr double operator*(const double a, const LayoutUnit& b) {
  return a * b.ToDouble();
}

template <unsigned fractional_bits, typename RawValue>
  requires(std::is_same_v<RawValue, int32_t>)
inline FixedPoint<fractional_bits, RawValue> operator/(
    const FixedPoint<fractional_bits, RawValue>& a,
    const FixedPoint<fractional_bits, RawValue>& b) {
  int64_t raw_val =
      static_cast<int64_t>(
          FixedPoint<fractional_bits, RawValue>::kFixedPointDenominator) *
      a.RawValue() / b.RawValue();
  return FixedPoint<fractional_bits, RawValue>::FromRawValueWithClamp(raw_val);
}

template <unsigned fractional_bits, typename RawValue>
inline FixedPoint<fractional_bits, RawValue>
FixedPoint<fractional_bits, RawValue>::MulDiv(FixedPoint m,
                                              FixedPoint d) const {
  int64_t n = static_cast<int64_t>(RawValue()) * m.RawValue();
  int64_t q = n / d.RawValue();
  return FromRawValueWithClamp(q);
}

constexpr float operator/(const LayoutUnit& a, float b) {
  return a.ToFloat() / b;
}

constexpr double operator/(const LayoutUnit& a, double b) {
  return a.ToDouble() / b;
}

template <unsigned fractional_bits, typename RawValue>
inline FixedPoint<fractional_bits, RawValue> operator/(
    const FixedPoint<fractional_bits, RawValue>& a,
    std::integral auto b) {
  return FixedPoint<fractional_bits, RawValue>::FromRawValue(a.RawValue() / b);
}

constexpr float operator/(const float a, const LayoutUnit& b) {
  return a / b.ToFloat();
}

constexpr double operator/(const double a, const LayoutUnit& b) {
  return a / b.ToDouble();
}

inline LayoutUnit operator/(std::integral auto a, const LayoutUnit& b) {
  return LayoutUnit(a) / b;
}

template <unsigned fractional_bits, typename RawValue>
ALWAYS_INLINE FixedPoint<fractional_bits, RawValue> operator+(
    const FixedPoint<fractional_bits, RawValue>& a,
    const FixedPoint<fractional_bits, RawValue>& b) {
  return FixedPoint<fractional_bits, RawValue>::FromRawValue(
      base::ClampAdd(a.RawValue(), b.RawValue()).RawValue());
}

inline LayoutUnit operator+(const LayoutUnit& a, std::integral auto b) {
  return a + LayoutUnit(b);
}

template <unsigned fractional_bits, typename RawValue>
inline float operator+(const FixedPoint<fractional_bits, RawValue>& a,
                       float b) {
  return a.ToFloat() + b;
}

inline double operator+(const LayoutUnit& a, double b) {
  return a.ToDouble() + b;
}

inline LayoutUnit operator+(std::integral auto a, const LayoutUnit& b) {
  return LayoutUnit(a) + b;
}

constexpr float operator+(const float a, const LayoutUnit& b) {
  return a + b.ToFloat();
}

constexpr double operator+(const double a, const LayoutUnit& b) {
  return a + b.ToDouble();
}

ALWAYS_INLINE LayoutUnit operator-(const LayoutUnit& a, const LayoutUnit& b) {
  return LayoutUnit::FromRawValue(
      base::ClampSub(a.RawValue(), b.RawValue()).RawValue());
}

inline LayoutUnit operator-(const LayoutUnit& a, std::integral auto b) {
  return a - LayoutUnit(b);
}

constexpr float operator-(const LayoutUnit& a, float b) {
  return a.ToFloat() - b;
}

constexpr double operator-(const LayoutUnit& a, double b) {
  return a.ToDouble() - b;
}

inline LayoutUnit operator-(std::integral auto a, const LayoutUnit& b) {
  return LayoutUnit(a) - b;
}

constexpr float operator-(const float a, const LayoutUnit& b) {
  return a - b.ToFloat();
}

template <unsigned fractional_bits, typename RawValue>
inline FixedPoint<fractional_bits, RawValue> operator-(
    const FixedPoint<fractional_bits, RawValue>& a) {
  return FixedPoint<fractional_bits, RawValue>::FromRawValue(
      (-base::MakeClampedNum(a.RawValue())).RawValue());
}

// Returns the remainder after a division with integer results.
// This calculates the modulo so that:
//   a = static_cast<int>(a / b) * b + IntMod(a, b).
inline LayoutUnit IntMod(const LayoutUnit& a, const LayoutUnit& b) {
  return LayoutUnit::FromRawValue(a.RawValue() % b.RawValue());
}

template <unsigned fractional_bits, typename RawValue, typename SourceStorage>
  requires(sizeof(SourceStorage) <= sizeof(RawValue))
inline FixedPoint<fractional_bits, RawValue>& operator+=(
    FixedPoint<fractional_bits, RawValue>& a,
    const FixedPoint<fractional_bits, SourceStorage>& b) {
  a.SetRawValue(base::ClampAdd(a.RawValue(), b.RawValue()).RawValue());
  return a;
}

template <unsigned fractional_bits, typename RawValue, typename SourceStorage>
  requires(sizeof(SourceStorage) <= sizeof(RawValue))
inline FixedPoint<fractional_bits, RawValue> operator+(
    const FixedPoint<fractional_bits, RawValue>& a,
    const FixedPoint<fractional_bits, SourceStorage>& b) {
  FixedPoint<fractional_bits, RawValue> r = a;
  r += b;
  return r;
}

inline LayoutUnit& operator+=(LayoutUnit& a, std::integral auto b) {
  a = a + LayoutUnit(b);
  return a;
}

inline LayoutUnit& operator+=(LayoutUnit& a, float b) {
  a = LayoutUnit(a + b);
  return a;
}

inline float& operator+=(float& a, const LayoutUnit& b) {
  a = a + b;
  return a;
}

inline LayoutUnit& operator-=(LayoutUnit& a, std::integral auto b) {
  a = a - LayoutUnit(b);
  return a;
}

template <unsigned fractional_bits, typename RawValue>
inline FixedPoint<fractional_bits, RawValue>& operator-=(
    FixedPoint<fractional_bits, RawValue>& a,
    const FixedPoint<fractional_bits, RawValue>& b) {
  a.SetRawValue(base::ClampSub(a.RawValue(), b.RawValue()).RawValue());
  return a;
}

inline LayoutUnit& operator-=(LayoutUnit& a, float b) {
  a = LayoutUnit(a - b);
  return a;
}

inline float& operator-=(float& a, const LayoutUnit& b) {
  a = a - b;
  return a;
}

inline LayoutUnit& operator*=(LayoutUnit& a, const LayoutUnit& b) {
  a = a * b;
  return a;
}

inline LayoutUnit& operator*=(LayoutUnit& a, float b) {
  a = LayoutUnit(a * b);
  return a;
}

inline float& operator*=(float& a, const LayoutUnit& b) {
  a = a * b;
  return a;
}

inline LayoutUnit& operator/=(LayoutUnit& a, const LayoutUnit& b) {
  a = a / b;
  return a;
}

inline LayoutUnit& operator/=(LayoutUnit& a, float b) {
  a = LayoutUnit(a / b);
  return a;
}

inline float& operator/=(float& a, const LayoutUnit& b) {
  a = a / b;
  return a;
}

inline int SnapSizeToPixel(LayoutUnit size, LayoutUnit location) {
  LayoutUnit fraction = location.Fraction();
  int result = (fraction + size).Round() - fraction.Round();
  if (result == 0 && (size.RawValue() > 4 || size.RawValue() < -4))
      [[unlikely]] {
    return size > 0 ? 1 : -1;
  }
  return result;
}

inline int SnapSizeToPixelAllowingZero(LayoutUnit size, LayoutUnit location) {
  LayoutUnit fraction = location.Fraction();
  return (fraction + size).Round() - fraction.Round();
}

inline int RoundToInt(LayoutUnit value) {
  return value.Round();
}

inline int FloorToInt(LayoutUnit value) {
  return value.Floor();
}

inline int CeilToInt(LayoutUnit value) {
  return value.Ceil();
}

inline LayoutUnit AbsoluteValue(const LayoutUnit& value) {
  return value.Abs();
}

template <unsigned fractional_bits, typename RawValue>
inline std::optional<FixedPoint<fractional_bits, RawValue>>
FixedPoint<fractional_bits, RawValue>::NullOptIf(FixedPoint null_value) const {
  if (*this == null_value) {
    return std::nullopt;
  }
  return *this;
}

#if defined(ARCH_CPU_ARM_FAMILY) && defined(ARCH_CPU_32_BITS) && \
    defined(COMPILER_GCC) && !BUILDFLAG(IS_NACL) && __OPTIMIZE__
inline int GetMaxSaturatedSetResultForTesting() {
  // For ARM Asm version the set function maxes out to the biggest
  // possible integer part with the fractional part zero'd out.
  // e.g. 0x7fffffc0.
  return LayoutUnit::kRawValueMax & ~(LayoutUnit::kFixedPointDenominator - 1);
}

inline int GetMinSaturatedSetResultForTesting() {
  return LayoutUnit::kRawValueMin;
}
#else
ALWAYS_INLINE int GetMaxSaturatedSetResultForTesting() {
  // For C version the set function maxes out to max int, this differs from
  // the ARM asm version.
  return LayoutUnit::kRawValueMax;
}

ALWAYS_INLINE int GetMinSaturatedSetResultForTesting() {
  return LayoutUnit::kRawValueMin;
}
#endif  // CPU(ARM) && COMPILER(GCC)

template <unsigned fractional_bits, typename RawValue>
PLATFORM_EXPORT std::ostream& operator<<(
    std::ostream&,
    const FixedPoint<fractional_bits, RawValue>&);

}  // namespace blink

WTF_ALLOW_MOVE_INIT_AND_COMPARE_WITH_MEM_FUNCTIONS(blink::LayoutUnit)

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_GEOMETRY_LAYOUT_UNIT_H_
