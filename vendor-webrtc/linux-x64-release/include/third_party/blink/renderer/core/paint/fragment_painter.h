// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_FRAGMENT_PAINTER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_FRAGMENT_PAINTER_H_

#include "third_party/blink/renderer/core/layout/physical_box_fragment.h"
#include "third_party/blink/renderer/platform/geometry/physical_offset.h"

namespace blink {

struct PaintInfo;

// Generic fragment painter for paint logic shared between all types of
// fragments. LayoutNG version of ObjectPainter.
class FragmentPainter {
  STACK_ALLOCATED();

 public:
  FragmentPainter(const PhysicalBoxFragment& box,
                  const DisplayItemClient& display_item_client)
      : box_fragment_(box), display_item_client_(display_item_client) {}

  // |style_to_use| may be from other objects than |box_fragment_|. When
  // painting outlines for a block in a continuation chain, its style does not
  // have the `outline` property set.
  void PaintOutline(const PaintInfo&,
                    const PhysicalOffset& paint_offset,
                    const ComputedStyle& style_to_use);

  void AddURLRectIfNeeded(const PaintInfo&, const PhysicalOffset& paint_offset);

 private:
  const PhysicalBoxFragment& PhysicalFragment() const { return box_fragment_; }
  const DisplayItemClient& GetDisplayItemClient() const {
    return display_item_client_;
  }

  const PhysicalBoxFragment& box_fragment_;
  const DisplayItemClient& display_item_client_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_FRAGMENT_PAINTER_H_
