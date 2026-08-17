// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_GEOMETRY_MATH_FUNCTIONS_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_GEOMETRY_MATH_FUNCTIONS_H_

#include <cfloat>
#include <cmath>
#include <optional>
#include <utility>

#include "base/notreached.h"
#include "ui/gfx/geometry/sin_cos_degrees.h"

namespace blink {

namespace {

template <class ValueType>
std::pair<ValueType, ValueType> GetNearestMultiples(ValueType a, ValueType b) {
  using std::swap;
  bool is_negative = a < 0.0;
  a = std::abs(a);
  b = std::abs(b);
  // To get rid of rounding and range problems we use std::fmod
  // to get the lower to A multiple of B.
  ValueType c = -std::fmod(a, b);
  ValueType lower = a + c;
  // Sort a, b, c by increasing magnitude so that a + b + c later can
  // avoid rounding problems (e.g., if one number is very small compared to
  // other).
  if (std::abs(a) > std::abs(c)) {
    swap(a, c);
  }
  if (std::abs(a) > std::abs(b)) {
    swap(a, b);
  }
  if (std::abs(b) > std::abs(c)) {
    swap(b, c);
  }
  ValueType upper = a + b + c;
  if (is_negative) {
    swap(lower, upper);
    lower = -lower;
    upper = -upper;
  }
  return {lower, upper};
}

template <class OperatorType, typename ValueType>
std::optional<ValueType> PreCheckSteppedValueFunctionArguments(OperatorType op,
                                                               ValueType a,
                                                               ValueType b) {
  // In round(A, B), if B is 0, the result is NaN.
  // In mod(A, B) or rem(A, B), if B is 0, the result is NaN.
  // If A and B are both infinite, the result is NaN.
  if (b == 0.0 || (std::isinf(a) && std::isinf(b))) {
    return std::numeric_limits<ValueType>::quiet_NaN();
  }
  // If A is exactly equal to an integer multiple of B,
  // round() resolves to A exactly.
  // If A is infinite but B is finite, the result is the same infinity.
  if (OperatorType::kRoundNearest <= op && op <= OperatorType::kRoundToZero &&
      (std::fmod(a, b) == 0.0 || (std::isinf(a) && !std::isinf(b)))) {
    return a;
  }
  // In mod(A, B) or rem(A, B), if A is infinite, the result is NaN.
  if (OperatorType::kMod <= op && op <= OperatorType::kRem && std::isinf(a)) {
    return std::numeric_limits<ValueType>::quiet_NaN();
  }
  return {};
}

template <typename T>
  requires std::floating_point<T>
T TanDegrees(T degrees) {
  // Use table values for tan() if possible.
  // We pick a pretty arbitrary limit that should be safe.
  if (degrees > -90000000.0 && degrees < 90000000.0) {
    // Make sure 0, 45, 90, 135, 180, 225 and 270 degrees get exact results.
    T n45degrees = degrees / 45.0;
    int octant = static_cast<int>(n45degrees);
    if (octant == n45degrees) {
      constexpr std::array<T, 8> kTanN45 = {
          /* 0deg */ 0.0,
          /* 45deg */ 1.0,
          /* 90deg */ std::numeric_limits<T>::infinity(),
          /* 135deg */ -1.0,
          /* 180deg */ 0.0,
          /* 225deg */ 1.0,
          /* 270deg */ -std::numeric_limits<T>::infinity(),
          /* 315deg */ -1.0,
      };
      return kTanN45[octant & 7];
    }
  }
  // Slow path for non-table cases.
  T x = Deg2rad(degrees);
  return std::tan(x);
}

}  // namespace

template <class OperatorType, typename ValueType>
  requires std::is_enum_v<OperatorType> && std::floating_point<ValueType>
ValueType EvaluateTrigonometricFunction(
    OperatorType op,
    ValueType a,
    std::optional<ValueType>(b) = std::nullopt) {
  switch (op) {
    case OperatorType::kSin: {
      return gfx::SinCosDegrees(a).sin;
    }
    case OperatorType::kCos: {
      return gfx::SinCosDegrees(a).cos;
    }
    case OperatorType::kTan: {
      return TanDegrees(a);
    }
    case OperatorType::kAsin: {
      ValueType value = Rad2deg(std::asin(a));
      DCHECK(value >= -90 && value <= 90 || std::isnan(value));
      return value;
    }
    case OperatorType::kAcos: {
      ValueType value = Rad2deg(std::acos(a));
      DCHECK(value >= 0 && value <= 180 || std::isnan(value));
      return value;
    }
    case OperatorType::kAtan: {
      ValueType value = Rad2deg(std::atan(a));
      DCHECK(value >= -90 && value <= 90 || std::isnan(value));
      return value;
    }
    case OperatorType::kAtan2: {
      DCHECK(b.has_value());
      ValueType value = Rad2deg(std::atan2(a, b.value()));
      DCHECK(value >= -180 && value <= 180 || std::isnan(value));
      return value;
    }
    default:
      NOTREACHED();
  }
}

template <class OperatorType, typename ValueType>
  requires std::is_enum_v<OperatorType> && std::floating_point<ValueType>
ValueType EvaluateSteppedValueFunction(OperatorType op,
                                       ValueType a,
                                       ValueType b) {
  // https://drafts.csswg.org/css-values/#round-infinities
  std::optional<ValueType> pre_check =
      PreCheckSteppedValueFunctionArguments(op, a, b);
  if (pre_check.has_value()) {
    return pre_check.value();
  }
  // If A is finite but B is infinite, the result depends
  // on the <rounding-strategy> and the sign of A:
  // -- nearest, to-zero
  // If A is positive or +0.0, return +0.0. Otherwise, return -0.0.
  // -- up
  // If A is positive (not zero), return +∞. If A is +0.0, return +0.0.
  // Otherwise, return -0.0. down If A is negative (not zero), return −∞. If A
  // is -0.0, return -0.0. Otherwise, return +0.0. If A is infinite but B is
  // finite, the result is the same infinity.
  auto [lower, upper] = GetNearestMultiples(a, b);
  switch (op) {
    case OperatorType::kRoundNearest: {
      if (!std::isinf(a) && std::isinf(b)) {
        return std::signbit(a) ? -0.0 : +0.0;
      } else {
        // In the negative case we need to swap lower and upper for the nearest
        // rounding. This also means tie-breaking should pick the lower rather
        // than upper,
        const bool a_is_negative = a < 0.0;
        if (a_is_negative) {
          using std::swap;
          swap(lower, upper);
        }
        const ValueType distance = std::abs(std::fmod(a, b));
        const ValueType half_b = std::abs(b) / 2;
        if (distance < half_b || (a_is_negative && distance == half_b)) {
          return lower;
        } else {
          return upper;
        }
      }
    }
    case OperatorType::kRoundUp: {
      if (!std::isinf(a) && std::isinf(b)) {
        if (!a) {
          return a;
        } else {
          return std::signbit(a) ? -0.0
                                 : std::numeric_limits<ValueType>::infinity();
        }
      } else {
        return upper;
      }
    }
    case OperatorType::kRoundDown: {
      if (!std::isinf(a) && std::isinf(b)) {
        if (!a) {
          return a;
        } else {
          return std::signbit(a) ? -std::numeric_limits<ValueType>::infinity()
                                 : +0.0;
        }
      } else {
        return lower;
      }
    }
    case OperatorType::kRoundToZero: {
      if (!std::isinf(a) && std::isinf(b)) {
        return std::signbit(a) ? -0.0 : +0.0;
      } else {
        return std::abs(upper) < std::abs(lower) ? upper : lower;
      }
    }
    case OperatorType::kMod: {
      // In mod(A, B) only, if B is infinite and A has opposite sign to B
      // (including an oppositely-signed zero), the result is NaN.
      if (std::isinf(b) && std::signbit(a) != std::signbit(b)) {
        return std::numeric_limits<ValueType>::quiet_NaN();
      }
      // If both arguments are positive or both are negative:
      // the value of the function is equal to the value of A
      // shifted by the integer multiple of B that brings
      // the value between zero and B.
      // If the A value and the B step are on opposite sides of zero:
      // mod() (short for “modulus”) continues to choose the integer
      // multiple of B that puts the value between zero and B.
      // std::fmod - the returned value has the same sign as A
      // and is less than B in magnitude.
      ValueType result = std::fmod(a, b);
      if (std::signbit(result) != std::signbit(b)) {
        // When the absolute values of arguments are the same, but they
        // appear on different sides from zero, the result of std::fmod will be
        // either -0 (e.g. mod(-1, 1)), or +0 (e.g. mod(1, -1)), we need to swap
        // the sign of the resulting zero to match the sign of the B.
        if (std::abs(a) == std::abs(b)) {
          return -result;
        }
        // If the result is on opposite side of zero from B,
        // put it between 0 and B. As the result of std::fmod is less
        // than B in magnitude, adding B would perform a correct shift.
        return result + b;
      }
      return result;
    }
    case OperatorType::kRem: {
      // If both arguments are positive or both are negative:
      // the value of the function is equal to the value of A
      // shifted by the integer multiple of B that brings
      // the value between zero and B.
      // If the A value and the B step are on opposite sides of zero:
      // rem() (short for "remainder") chooses the integer multiple of B
      // that puts the value between zero and -B,
      // avoiding changing the sign of the value.
      // std::fmod - the returned value has the same sign as A
      // and is less than B in magnitude.
      return std::fmod(a, b);
    }
    default:
      NOTREACHED();
  }
}

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_GEOMETRY_MATH_FUNCTIONS_H_
