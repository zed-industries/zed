// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_GEOMETRY_GEOMETRY_UTIL_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_GEOMETRY_GEOMETRY_UTIL_H_

#include <algorithm>
#include <cmath>
#include <limits>

namespace blink {

namespace geometry_util {

// Returns the minimum of |a| and |b|. If either operand is NaN, then NaN is
// returned, consistent with Math.min() in JavaScript.
inline double NanSafeMin(double a, double b) {
  if (std::isnan(a) || std::isnan(b)) {
    return std::numeric_limits<double>::quiet_NaN();
  }
  return std::min(a, b);
}

// Returns the maximum of |a| and |b|. If either operand is NaN, then NaN is
// returned, consistent with Math.max() in JavaScript.
inline double NanSafeMax(double a, double b) {
  if (std::isnan(a) || std::isnan(b)) {
    return std::numeric_limits<double>::quiet_NaN();
  }
  return std::max(a, b);
}

}  // namespace geometry_util

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_GEOMETRY_GEOMETRY_UTIL_H_
