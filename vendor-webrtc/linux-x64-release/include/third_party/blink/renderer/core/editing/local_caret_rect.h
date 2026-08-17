// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_LOCAL_CARET_RECT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_LOCAL_CARET_RECT_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/editing/editing_boundary.h"
#include "third_party/blink/renderer/core/editing/forward.h"
#include "third_party/blink/renderer/core/layout/geometry/physical_rect.h"

namespace blink {

class LayoutObject;
class PhysicalBoxFragment;

// A transient struct representing a caret rect local to |layout_object|.
struct LocalCaretRect {
  STACK_ALLOCATED();

 public:
  const LayoutObject* layout_object = nullptr;
  PhysicalRect rect;
  const PhysicalBoxFragment* root_box_fragment = nullptr;

  LocalCaretRect() = default;
  LocalCaretRect(const LayoutObject* layout_object,
                 const PhysicalRect& rect,
                 const PhysicalBoxFragment* root_box_fragment = nullptr)
      : layout_object(layout_object),
        rect(rect),
        root_box_fragment(root_box_fragment) {}

  bool IsEmpty() const { return !layout_object || rect.IsEmpty(); }
};

// Rect is local to the returned layoutObject
CORE_EXPORT LocalCaretRect LocalCaretRectOfPosition(
    const PositionWithAffinity&,
    EditingBoundaryCrossingRule = kCanCrossEditingBoundary);
CORE_EXPORT LocalCaretRect LocalCaretRectOfPosition(
    const PositionInFlatTreeWithAffinity&,
    EditingBoundaryCrossingRule = kCanCrossEditingBoundary);

LocalCaretRect LocalSelectionRectOfPosition(const PositionWithAffinity&);

// Bounds of (possibly transformed) caret in absolute coords
CORE_EXPORT gfx::Rect AbsoluteCaretBoundsOf(
    const PositionWithAffinity&,
    EditingBoundaryCrossingRule rule = kCanCrossEditingBoundary);
CORE_EXPORT gfx::Rect AbsoluteCaretBoundsOf(
    const PositionInFlatTreeWithAffinity&);

CORE_EXPORT gfx::Rect AbsoluteSelectionBoundsOf(const VisiblePosition&);

// Exposed to tests only. Implemented in local_caret_rect_test.cc.
bool operator==(const LocalCaretRect&, const LocalCaretRect&);
std::ostream& operator<<(std::ostream&, const LocalCaretRect&);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_LOCAL_CARET_RECT_H_
