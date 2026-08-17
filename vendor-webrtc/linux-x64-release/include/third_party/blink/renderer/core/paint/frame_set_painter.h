// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_FRAME_SET_PAINTER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_FRAME_SET_PAINTER_H_

#include "third_party/blink/renderer/platform/geometry/physical_offset.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace gfx {
class Rect;
}

namespace blink {

class Color;
class DisplayItemClient;
class PhysicalBoxFragment;
struct AutoDarkMode;
struct PaintInfo;

class FrameSetPainter {
  STACK_ALLOCATED();

 public:
  FrameSetPainter(const PhysicalBoxFragment& box_fragment,
                  const DisplayItemClient& display_item_client)
      : box_fragment_(box_fragment),
        display_item_client_(display_item_client) {}
  void PaintObject(const PaintInfo&, const PhysicalOffset&);

 private:
  void PaintChildren(const PaintInfo& paint_info);
  void PaintBorders(const PaintInfo& paint_info,
                    const PhysicalOffset& paint_offset);
  void PaintRowBorder(const PaintInfo& paint_info,
                      const gfx::Rect& border_rect,
                      const Color& fill_color,
                      const AutoDarkMode& auto_dark_mode);
  void PaintColumnBorder(const PaintInfo& paint_info,
                         const gfx::Rect& border_rect,
                         const Color& fill_color,
                         const AutoDarkMode& auto_dark_mode);

  const PhysicalBoxFragment& box_fragment_;
  const DisplayItemClient& display_item_client_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_FRAME_SET_PAINTER_H_
