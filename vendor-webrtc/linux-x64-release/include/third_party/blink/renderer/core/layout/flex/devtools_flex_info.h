// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_FLEX_DEVTOOLS_FLEX_INFO_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_FLEX_DEVTOOLS_FLEX_INFO_H_

#include "third_party/blink/renderer/core/layout/geometry/physical_rect.h"
#include "third_party/blink/renderer/platform/geometry/layout_unit.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {
struct DevtoolsFlexInfo {
  struct Item {
    Item(PhysicalRect rect, LayoutUnit baseline)
        : rect(rect), baseline(baseline) {}
    PhysicalRect rect;
    LayoutUnit baseline;
  };
  struct Line {
    Vector<Item> items;
  };
  Vector<Line> lines;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_FLEX_DEVTOOLS_FLEX_INFO_H_
