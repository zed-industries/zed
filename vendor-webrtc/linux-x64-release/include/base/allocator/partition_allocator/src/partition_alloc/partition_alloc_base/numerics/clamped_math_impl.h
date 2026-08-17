// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef PARTITION_ALLOC_PARTITION_ALLOC_BASE_NUMERICS_CLAMPED_MATH_IMPL_H_
#define PARTITION_ALLOC_PARTITION_ALLOC_BASE_NUMERICS_CLAMPED_MATH_IMPL_H_

#include <climits>
#include <cmath>
#include <cstddef>
#include <cstdint>
#include <cstdlib>
#include <limits>
#include <type_traits>

#include "partition_alloc/partition_alloc_base/numerics/checked_math.h"
#include "partition_alloc/partition_alloc_base/numerics/safe_conversions.h"
#include "partition_alloc/partition_alloc_base/numerics/safe_math_shared_impl.h"

namespace partition_alloc::internal::base::internal {

template <typename T,
          typename std::enable_if<std::is_integral_v<T> &&
                                  std::is_signed_v<T>>::type* = nullptr>
constexpr T SaturatedNegWrapper(T value) {
  return PA_IsConstantEvaluated() || !ClampedNegFastOp<T>::is_supported
             ? (NegateWrapper(value) != std::numeric_limits<T>::lowest()
                    ? NegateWrapper(value)
                    : std::numeric_limits<T>::max())
             : ClampedNegFastOp<T>::Do(value);
}

template <typename T,
          typename std::enable_if<std::is_integral_v<T> &&
                                  !std::is_signed_v<T>>::type* = nullptr>
constexpr T SaturatedNegWrapper(T value) {
  return T(0);
}

template <typename T,
          typename std::enable_if<std::is_floating_point_v<T>>::type* = nullptr>
constexpr T SaturatedNegWrapper(T value) {
  return -value;
}

template <typename T,
          typename std::enable_if<std::is_integral_v<T>>::type* = nullptr>
constexpr T SaturatedAbsWrapper(T value) {
  // The calculation below is a static identity for unsigned types, but for
  // signed integer types it provides a non-branching, saturated absolute value.
  // This works because SafeUnsignedAbs() returns an unsigned type, which can
  // represent the absolute value of all negative numbers of an equal-width
  // integer type. The call to IsValueNegative() then detects overflow in the
  // special case of numeric_limits<T>::min(), by evaluating the bit pattern as
  // a signed integer value. If it is the overflow case, we end up subtracting
  // one from the unsigned result, thus saturating to numeric_limits<T>::max().
  return static_cast<T>(
      SafeUnsignedAbs(value) -
      IsValueNegative<T>(static_cast<T>(SafeUnsignedAbs(value))));
}

template <typename T,
          typename std::enable_if<std::is_floating_point_v<T>>::type* = nullptr>
constexpr T SaturatedAbsWrapper(T value) {
  return value < 0 ? -value : value;
}

template <typename T, typename U, class Enable = void>
struct ClampedAddOp {};

template <typename T, typename U>
struct ClampedAddOp<T,
                    U,
                    typename std::enable_if<std::is_integral_v<T> &&
                                            std::is_integral_v<U>>::type> {
  using result_type = typename MaxExponentPromotion<T, U>::type;
  template <typename V = result_type>
  static constexpr V Do(T x, U y) {
    if (!PA_IsConstantEvaluated() && ClampedAddFastOp<T, U>::is_supported) {
      return ClampedAddFastOp<T, U>::template Do<V>(x, y);
    }

    static_assert(std::is_same_v<V, result_type> ||
                      IsTypeInRangeForNumericType<U, V>::value,
                  "The saturation result cannot be determined from the "
                  "provided types.");
    const V saturated = CommonMaxOrMin<V>(IsValueNegative(y));
    V result = {};
    if (CheckedAddOp<T, U>::Do(x, y, &result)) [[likely]] {
      return result;
    }
    return saturated;
  }
};

template <typename T, typename U, class Enable = void>
struct ClampedSubOp {};

template <typename T, typename U>
struct ClampedSubOp<T,
                    U,
                    typename std::enable_if<std::is_integral_v<T> &&
                                            std::is_integral_v<U>>::type> {
  using result_type = typename MaxExponentPromotion<T, U>::type;
  template <typename V = result_type>
  static constexpr V Do(T x, U y) {
    if (!PA_IsConstantEvaluated() && ClampedSubFastOp<T, U>::is_supported) {
      return ClampedSubFastOp<T, U>::template Do<V>(x, y);
    }

    static_assert(std::is_same_v<V, result_type> ||
                      IsTypeInRangeForNumericType<U, V>::value,
                  "The saturation result cannot be determined from the "
                  "provided types.");
    const V saturated = CommonMaxOrMin<V>(!IsValueNegative(y));
    V result = {};
    if (CheckedSubOp<T, U>::Do(x, y, &result)) [[likely]] {
      return result;
    }
    return saturated;
  }
};

template <typename T, typename U, class Enable = void>
struct ClampedMulOp {};

template <typename T, typename U>
struct ClampedMulOp<T,
                    U,
                    typename std::enable_if<std::is_integral_v<T> &&
                                            std::is_integral_v<U>>::type> {
  using result_type = typename MaxExponentPromotion<T, U>::type;
  template <typename V = result_type>
  static constexpr V Do(T x, U y) {
    if (!PA_IsConstantEvaluated() && ClampedMulFastOp<T, U>::is_supported) {
      return ClampedMulFastOp<T, U>::template Do<V>(x, y);
    }

    V result = {};
    const V saturated =
        CommonMaxOrMin<V>(IsValueNegative(x) ^ IsValueNegative(y));
    if (CheckedMulOp<T, U>::Do(x, y, &result)) [[likely]] {
      return result;
    }
    return saturated;
  }
};

template <typename T, typename U, class Enable = void>
struct ClampedDivOp {};

template <typename T, typename U>
struct ClampedDivOp<T,
                    U,
                    typename std::enable_if<std::is_integral_v<T> &&
                                            std::is_integral_v<U>>::type> {
  using result_type = typename MaxExponentPromotion<T, U>::type;
  template <typename V = result_type>
  static constexpr V Do(T x, U y) {
    V result = {};
    if ((CheckedDivOp<T, U>::Do(x, y, &result))) [[likely]] {
      return result;
    }
    // Saturation goes to max, min, or NaN (if x is zero).
    return x ? CommonMaxOrMin<V>(IsValueNegative(x) ^ IsValueNegative(y))
             : SaturationDefaultLimits<V>::NaN();
  }
};

template <typename T, typename U, class Enable = void>
struct ClampedModOp {};

template <typename T, typename U>
struct ClampedModOp<T,
                    U,
                    typename std::enable_if<std::is_integral_v<T> &&
                                            std::is_integral_v<U>>::type> {
  using result_type = typename MaxExponentPromotion<T, U>::type;
  template <typename V = result_type>
  static constexpr V Do(T x, U y) {
    V result = {};
    if (CheckedModOp<T, U>::Do(x, y, &result)) [[likely]] {
      return result;
    }
    return x;
  }
};

template <typename T, typename U, class Enable = void>
struct ClampedLshOp {};

// Left shift. Non-zero values saturate in the direction of the sign. A zero
// shifted by any value always results in zero.
template <typename T, typename U>
struct ClampedLshOp<T,
                    U,
                    typename std::enable_if<std::is_integral_v<T> &&
                                            std::is_integral_v<U>>::type> {
  using result_type = T;
  template <typename V = result_type>
  static constexpr V Do(T x, U shift) {
    static_assert(!std::is_signed_v<U>, "Shift value must be unsigned.");
    if (shift < std::numeric_limits<T>::digits) [[likely]] {
      // Shift as unsigned to avoid undefined behavior.
      V result = static_cast<V>(as_unsigned(x) << shift);
      // If the shift can be reversed, we know it was valid.
      if (result >> shift == x) [[likely]] {
        return result;
      }
    }
    return x ? CommonMaxOrMin<V>(IsValueNegative(x)) : 0;
  }
};

template <typename T, typename U, class Enable = void>
struct ClampedRshOp {};

// Right shift. Negative values saturate to -1. Positive or 0 saturates to 0.
template <typename T, typename U>
struct ClampedRshOp<T,
                    U,
                    typename std::enable_if<std::is_integral_v<T> &&
                                            std::is_integral_v<U>>::type> {
  using result_type = T;
  template <typename V = result_type>
  static constexpr V Do(T x, U shift) {
    static_assert(!std::is_signed_v<U>, "Shift value must be unsigned.");
    // Signed right shift is odd, because it saturates to -1 or 0.
    const V saturated = as_unsigned(V(0)) - IsValueNegative(x);
    if (shift < IntegerBitsPlusSign<T>::value) [[likely]] {
      return saturated_cast<V>(x >> shift);
    }
    return saturated;
  }
};

template <typename T, typename U, class Enable = void>
struct ClampedAndOp {};

template <typename T, typename U>
struct ClampedAndOp<T,
                    U,
                    typename std::enable_if<std::is_integral_v<T> &&
                                            std::is_integral_v<U>>::type> {
  using result_type = typename std::make_unsigned<
      typename MaxExponentPromotion<T, U>::type>::type;
  template <typename V>
  static constexpr V Do(T x, U y) {
    return static_cast<result_type>(x) & static_cast<result_type>(y);
  }
};

template <typename T, typename U, class Enable = void>
struct ClampedOrOp {};

// For simplicity we promote to unsigned integers.
template <typename T, typename U>
struct ClampedOrOp<T,
                   U,
                   typename std::enable_if<std::is_integral_v<T> &&
                                           std::is_integral_v<U>>::type> {
  using result_type = typename std::make_unsigned<
      typename MaxExponentPromotion<T, U>::type>::type;
  template <typename V>
  static constexpr V Do(T x, U y) {
    return static_cast<result_type>(x) | static_cast<result_type>(y);
  }
};

template <typename T, typename U, class Enable = void>
struct ClampedXorOp {};

// For simplicity we support only unsigned integers.
template <typename T, typename U>
struct ClampedXorOp<T,
                    U,
                    typename std::enable_if<std::is_integral_v<T> &&
                                            std::is_integral_v<U>>::type> {
  using result_type = typename std::make_unsigned<
      typename MaxExponentPromotion<T, U>::type>::type;
  template <typename V>
  static constexpr V Do(T x, U y) {
    return static_cast<result_type>(x) ^ static_cast<result_type>(y);
  }
};

template <typename T, typename U, class Enable = void>
struct ClampedMaxOp {};

template <typename T, typename U>
struct ClampedMaxOp<T,
                    U,
                    typename std::enable_if<std::is_arithmetic_v<T> &&
                                            std::is_arithmetic_v<U>>::type> {
  using result_type = typename MaxExponentPromotion<T, U>::type;
  template <typename V = result_type>
  static constexpr V Do(T x, U y) {
    return IsGreater<T, U>::Test(x, y) ? saturated_cast<V>(x)
                                       : saturated_cast<V>(y);
  }
};

template <typename T, typename U, class Enable = void>
struct ClampedMinOp {};

template <typename T, typename U>
struct ClampedMinOp<T,
                    U,
                    typename std::enable_if<std::is_arithmetic_v<T> &&
                                            std::is_arithmetic_v<U>>::type> {
  using result_type = typename LowestValuePromotion<T, U>::type;
  template <typename V = result_type>
  static constexpr V Do(T x, U y) {
    return IsLess<T, U>::Test(x, y) ? saturated_cast<V>(x)
                                    : saturated_cast<V>(y);
  }
};

// This is just boilerplate that wraps the standard floating point arithmetic.
// A macro isn't the nicest solution, but it beats rewriting these repeatedly.
#define PA_BASE_FLOAT_ARITHMETIC_OPS(NAME, OP)                      \
  template <typename T, typename U>                                 \
  struct Clamped##NAME##Op<                                         \
      T, U,                                                         \
      typename std::enable_if<std::is_floating_point_v<T> ||        \
                              std::is_floating_point_v<U>>::type> { \
    using result_type = typename MaxExponentPromotion<T, U>::type;  \
    template <typename V = result_type>                             \
    static constexpr V Do(T x, U y) {                               \
      return saturated_cast<V>(x OP y);                             \
    }                                                               \
  };

PA_BASE_FLOAT_ARITHMETIC_OPS(Add, +)
PA_BASE_FLOAT_ARITHMETIC_OPS(Sub, -)
PA_BASE_FLOAT_ARITHMETIC_OPS(Mul, *)
PA_BASE_FLOAT_ARITHMETIC_OPS(Div, /)

#undef PA_BASE_FLOAT_ARITHMETIC_OPS

}  // namespace partition_alloc::internal::base::internal

#endif  // PARTITION_ALLOC_PARTITION_ALLOC_BASE_NUMERICS_CLAMPED_MATH_IMPL_H_
