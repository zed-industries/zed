// Copyright 2025 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_GEOMETRY_SKIA_GEOMETRY_UTILS_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_GEOMETRY_SKIA_GEOMETRY_UTILS_H_

#include <concepts>

#include "third_party/skia/include/core/SkScalar.h"
#include "ui/gfx/geometry/clamp_float_geometry.h"
#include "ui/gfx/geometry/point_f.h"

namespace blink {

// Convert non-finite values to zero.
template <typename T>
  requires std::floating_point<T>
inline T ClampNonFiniteToZero(T f) {
  return std::isfinite(f) ? f : 0;
}

// Convert any non-finite values in the point to zero.
inline gfx::PointF ClampNonFiniteToZero(const gfx::PointF& point) {
  return {ClampNonFiniteToZero(point.x()), ClampNonFiniteToZero(point.y())};
}

// Convert any non-finite values in the point to safe enough float.
inline gfx::PointF ClampNonFiniteToSafeFloat(const gfx::PointF& point) {
  return {gfx::ClampFloatGeometry(point.x()),
          gfx::ClampFloatGeometry(point.y())};
}

inline bool WebCoreFloatNearlyEqual(float a, float b) {
  return SkScalarNearlyEqual(ClampNonFiniteToZero(a), ClampNonFiniteToZero(b));
}

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_GEOMETRY_SKIA_GEOMETRY_UTILS_H_
