// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_FRAME_SET_LAYOUT_DATA_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_FRAME_SET_LAYOUT_DATA_H_

#include "third_party/blink/renderer/platform/geometry/layout_unit.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {

// An instance of FrameSetLayoutData is produced by FrameSetLayoutAlgorithm,
// and is owned by BoxFragmentBuilder and PhysicalBoxFragment. It is used
// by FrameSetPainter and resize handling of HTMLFrameSetElement.
struct FrameSetLayoutData {
  // Frame grid sizes.
  Vector<LayoutUnit> col_sizes;
  Vector<LayoutUnit> row_sizes;

  // Border existence.
  Vector<bool> col_allow_border;
  Vector<bool> row_allow_border;

  int border_thickness;
  bool has_border_color;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_FRAME_SET_LAYOUT_DATA_H_
