// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_OBJECT_PAINTER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_OBJECT_PAINTER_H_

#include "cc/input/hit_test_opaqueness.h"
#include "third_party/blink/renderer/core/style/computed_style_constants.h"
#include "third_party/blink/renderer/platform/geometry/physical_offset.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace gfx {
class Rect;
}

namespace blink {

class DisplayItemClient;
class LayoutObject;
struct PaintInfo;

class ObjectPainter {
  STACK_ALLOCATED();

 public:
  ObjectPainter(const LayoutObject& layout_object)
      : layout_object_(layout_object) {}

  void PaintOutline(const PaintInfo&, const PhysicalOffset& paint_offset);
  void PaintInlineChildrenOutlines(const PaintInfo&);
  void AddURLRectIfNeeded(const PaintInfo&, const PhysicalOffset& paint_offset);

  // Paints the object atomically as if it created a new stacking context, for:
  // - inline blocks, inline tables, inline-level replaced elements (Section
  //   7.2.1.4 in http://www.w3.org/TR/CSS2/zindex.html#painting-order),
  // - non-positioned floating objects (Section 5 in
  //   http://www.w3.org/TR/CSS2/zindex.html#painting-order),
  // - flex items (http://www.w3.org/TR/css-flexbox-1/#painting),
  // - grid items (http://www.w3.org/TR/css-grid-1/#z-order),
  // - custom scrollbar parts.
  // Also see core/paint/README.md.
  //
  // It is expected that the caller will call this function independent of the
  // value of paintInfo.phase, and this function will do atomic paint (for
  // kForeground), normal paint (for kSelection and kTextClip) or nothing (other
  // paint phases) according to paintInfo.phase.
  void PaintAllPhasesAtomically(const PaintInfo&);

  // Hit test data has two purposes:
  // 1. Expands the bounds of the current paint chunk for hit test;
  // 2. Stores special hit test data, e.g. special touch action.
  // This should be called in the proper paint phase (background for
  // LayoutBoxes, foreground for line boxes and SVG) even if there is no other
  // painted content.
  void RecordHitTestData(const PaintInfo&,
                         const gfx::Rect& paint_rect,
                         const DisplayItemClient&);

  cc::HitTestOpaqueness GetHitTestOpaqueness() const;

  // If true, we should record hit test data for the second purpose described
  // above. As an optimization, some callers of RecordHitTestData() doesn't
  // need to call it just for the first purpose. For example, a text fragment
  // is always contained by some line box, thus the painter checks this
  // function before calling RecordHitTestData().
  bool ShouldRecordSpecialHitTestData(const PaintInfo&);

 private:
  const LayoutObject& layout_object_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_OBJECT_PAINTER_H_
