// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_GEOMETRY_BLEND_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_GEOMETRY_BLEND_H_

#include "third_party/blink/renderer/platform/geometry/layout_unit.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/wtf/math_extras.h"
#include "ui/gfx/geometry/point_f.h"

#include <type_traits>

namespace blink {

inline int Blend(int from, int to, double progress) {
  return static_cast<int>(lround(from + (to - from) * progress));
}

// For unsigned types.
template <typename T>
inline T Blend(T from, T to, double progress) {
  static_assert(std::is_integral<T>::value,
                "blend can only be used with integer types");
  return ClampTo<T>(round(to > from ? from + (to - from) * progress
                                    : from - (from - to) * progress));
}

inline double Blend(double from, double to, double progress) {
  return from + (to - from) * progress;
}

inline float Blend(float from, float to, double progress) {
  return static_cast<float>(from + (to - from) * progress);
}

inline LayoutUnit Blend(LayoutUnit from, LayoutUnit to, double progress) {
  return LayoutUnit(from + (to - from) * progress);
}

inline gfx::PointF Blend(const gfx::PointF& from,
                         const gfx::PointF& to,
                         double progress) {
  return gfx::PointF(Blend(from.x(), to.x(), progress),
                     Blend(from.y(), to.y(), progress));
}

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_GEOMETRY_BLEND_H_
