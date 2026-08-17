// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef IPCZ_SRC_UTIL_SAFE_MATH_
#define IPCZ_SRC_UTIL_SAFE_MATH_

#include <limits>
#include <type_traits>

#include "third_party/abseil-cpp/absl/base/macros.h"
#include "third_party/abseil-cpp/absl/base/optimization.h"

namespace ipcz {

template <typename Dst, typename Src>
constexpr Dst checked_cast(Src value) {
  // This throws a compile-time error on evaluating the constexpr if it can be
  // determined at compile-time as failing, otherwise it will fail an
  // assertion at runtime.
  ABSL_HARDENING_ASSERT(
      ABSL_PREDICT_TRUE(value <= std::numeric_limits<Dst>::max()));
  return static_cast<Dst>(value);
}

template <typename Dst, typename Src>
constexpr Dst saturated_cast(Src value) {
  static_assert(std::is_unsigned_v<Src> && std::is_unsigned_v<Dst>,
                "saturated_cast only supports unsigned types");
  constexpr Dst kMaxDst = std::numeric_limits<Dst>::max();
  constexpr Src kMaxSrc = std::numeric_limits<Src>::max();
  if (ABSL_PREDICT_TRUE(kMaxDst >= kMaxSrc || value <= kMaxDst)) {
    return static_cast<Dst>(value);
  }
  return kMaxDst;
}

template <typename T>
constexpr T CheckAdd(T a, T b) {
  T result;
  const bool did_overflow = ABSL_PREDICT_FALSE(__builtin_add_overflow(a, b, &result));
  ABSL_HARDENING_ASSERT(!did_overflow);
  return result;
}

template <typename T>
constexpr T CheckMul(T a, T b) {
  T result;
  const bool did_overflow = ABSL_PREDICT_FALSE(__builtin_mul_overflow(a, b, &result));
  ABSL_HARDENING_ASSERT(!did_overflow);
  return result;
}

template <typename T>
T SaturatedAdd(T a, T b) {
  T result;
  if (!__builtin_add_overflow(a, b, &result)) {
    return result;
  }
  return std::numeric_limits<T>::max();
}

}  // namespace ipcz

#endif  // IPCZ_SRC_UTIL_SAFE_MATH_
