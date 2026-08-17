// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_EXCLUSIONS_LAYOUT_OPPORTUNITY_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_EXCLUSIONS_LAYOUT_OPPORTUNITY_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/layout/geometry/bfc_rect.h"
#include "third_party/blink/renderer/core/layout/exclusions/line_layout_opportunity.h"
#include "third_party/blink/renderer/core/layout/exclusions/shape_exclusions.h"

namespace blink {

class ConstraintSpace;

// This struct represents an 2D-area where a LogicalFragment can fit within the
// exclusion space. A layout opportunity is produced by the exclusion space by
// calling FindLayoutOpportunity, or AllLayoutOpportunities.
//
// Its coordinates are relative to the BFC.
struct CORE_EXPORT LayoutOpportunity final {
  DISALLOW_NEW();

 public:
  LayoutOpportunity()
      : rect(BfcOffset(LayoutUnit::Min(), LayoutUnit::Min()),
             BfcOffset(LayoutUnit::Max(), LayoutUnit::Max())) {}
  explicit LayoutOpportunity(const BfcRect& rect,
                             const ShapeExclusions* shape_exclusions = nullptr)
      : rect(rect), shape_exclusions(shape_exclusions) {}

  void Trace(Visitor* visitor) const { visitor->Trace(shape_exclusions); }

  // Rectangle in BFC coordinates that represents this opportunity.
  BfcRect rect;

  // The shape exclusions hold all of the adjacent exclusions which may affect
  // the line layout opportunity when queried. May be null if no shapes are
  // present.
  Member<const ShapeExclusions> shape_exclusions;

  // Returns if the opportunity has any shapes which may affect a line layout
  // opportunity.
  bool HasShapeExclusions() const { return shape_exclusions != nullptr; }

  // Returns if the given delta (relative to the start of the opportunity) will
  // be below any shapes.
  bool IsBlockDeltaBelowShapes(LayoutUnit block_delta) const;

  // Calculates a line layout opportunity which takes into account any shapes
  // which may affect the available inline size for the line breaker.
  LineLayoutOpportunity ComputeLineLayoutOpportunity(
      const ConstraintSpace& space,
      LayoutUnit line_block_size,
      LayoutUnit block_delta) const {
    return LineLayoutOpportunity(
        ComputeLineLeftOffset(space, line_block_size, block_delta),
        ComputeLineRightOffset(space, line_block_size, block_delta),
        rect.LineStartOffset(), rect.LineEndOffset(),
        rect.BlockStartOffset() + block_delta, line_block_size);
  }

  bool operator==(const LayoutOpportunity& other) const;
  bool operator!=(const LayoutOpportunity& other) const {
    return !operator==(other);
  }

 private:
  LayoutUnit ComputeLineLeftOffset(const ConstraintSpace&,
                                   LayoutUnit line_block_size,
                                   LayoutUnit block_delta) const;
  LayoutUnit ComputeLineRightOffset(const ConstraintSpace&,
                                    LayoutUnit line_block_size,
                                    LayoutUnit block_delta) const;
};

CORE_EXPORT std::ostream& operator<<(std::ostream& os,
                                     const LayoutOpportunity& opportunity);

}  // namespace blink

WTF_ALLOW_CLEAR_UNUSED_SLOTS_WITH_MEM_FUNCTIONS(blink::LayoutOpportunity)

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_EXCLUSIONS_LAYOUT_OPPORTUNITY_H_
