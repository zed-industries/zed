// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_GEOMETRY_INFINITE_INT_RECT_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_GEOMETRY_INFINITE_INT_RECT_H_

#include <limits>

#include "third_party/blink/renderer/platform/geometry/layout_unit.h"
#include "ui/gfx/geometry/rect.h"

namespace blink {

// Returns a big enough rect that can contain all reasonable rendered results.
// The rect can be used as a "non-clipping" clip rect. The rect can be
// modified to clip at one or more sides, e.g.
//   gfx::Rect r = InfiniteIntRect();
//   r.set_width(clip_right - r.x());
constexpr gfx::Rect InfiniteIntRect() {
  constexpr int kInfiniteXY = LayoutUnit::Min().ToInt() / 4;
  constexpr int kInfiniteWH = LayoutUnit::Max().ToInt() / 2;
  // The values above ensure that any value between kInfiniteXY, and
  // kInfiniteXY + kInvalidateWH can be converted among int, float, and
  // LayoutUnit losslessly.
  static_assert(kInfiniteXY >= -(1 << std::numeric_limits<float>::digits));
  static_assert(kInfiniteXY + kInfiniteWH <=
                (1 << std::numeric_limits<float>::digits));
  return gfx::Rect(kInfiniteXY, kInfiniteXY, kInfiniteWH, kInfiniteWH);
}

// See physical_rect_test.cc for tests.

}  // namespace blink
#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_GEOMETRY_INFINITE_INT_RECT_H_
