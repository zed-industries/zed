// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_PAGE_SCROLLING_STICKY_POSITION_SCROLLING_CONSTRAINTS_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_PAGE_SCROLLING_STICKY_POSITION_SCROLLING_CONSTRAINTS_H_

#include <optional>

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/layout/geometry/physical_rect.h"
#include "third_party/blink/renderer/platform/geometry/layout_unit.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/member.h"

namespace blink {

class LayoutBoxModelObject;
class PaintLayer;

// Encapsulates the constraint information for a position: sticky element and
// does calculation of the sticky offset for a given overflow clip rectangle.
//
// To avoid slowing down scrolling we cannot make the offset calculation a
// layout-inducing event. Instead constraint information is cached during layout
// and used as the scroll position changes to determine the current offset. In
// most cases the only information that is needed is the sticky element's layout
// rectangle and its containing block rectangle (both respective to the nearest
// ancestor scroller which the element is sticking to), and the set of sticky
// edge constraints (i.e. the distance from each edge the element should stick).
//
// For a given (non-cached) overflow clip rectangle, calculating the current
// offset in most cases just requires sliding the (cached) sticky element
// rectangle until it satisfies the (cached) sticky edge constraints for the
// overflow clip rectangle, whilst not letting the sticky element rectangle
// escape its (cached) containing block rect. For example, consider the
// following situation (where positions are relative to the scroll container):
//
//    +---------------------+ <-- Containing Block (150x70 at 10,0)
//    | +-----------------+ |
//    | |    top: 50px;   |<-- Sticky Box (130x10 at 20,0)
//    | +-----------------+ |
//    |                     |
//  +-------------------------+ <-- Overflow Clip Rectangle (170x60 at 0,50)
//  | |                     | |
//  | |                     | |
//  | +---------------------+ |
//  |                         |
//  |                         |
//  |                         |
//  +-------------------------+
//
// Here the cached sticky box would be moved downwards to try and be at position
// (20,100) - 50 pixels down from the top of the clip rectangle. However doing
// so would take it outside the cached containing block rectangle, so the final
// sticky position would be capped to (20,20).
//
// Unfortunately this approach breaks down in the presence of nested sticky
// elements, as the cached locations would be moved by ancestor sticky elements.
// Consider:
//
//  +------------------------+ <-- Outer sticky (top: 10px, 150x50 at 0,0)
//  |  +------------------+  |
//  |  |                  | <-- Inner sticky (top: 25px, 100x20 at 20,0)
//  |  +------------------+  |
//  |                        |
//  +------------------------+
//
// Assume the overflow clip rectangle is centered perfectly over the outer
// sticky. We would then want to move the outer sticky element down by 10
// pixels, and the inner sticky element down by only 15 pixels - because it is
// already being shifted by its ancestor. To correctly handle such situations we
// apply more complicated logic which is explained in the implementation of
// |ComputeStickyOffset|.
struct CORE_EXPORT StickyPositionScrollingConstraints final
    : public GarbageCollected<StickyPositionScrollingConstraints> {
 public:
  // Computes the sticky offset for a given scroll position of the containing
  // scroll container. When the scroll position changed in a ScrollableArea,
  // this method must be called for all affected sticky objects in pre-tree
  // order.
  void ComputeStickyOffset(const gfx::PointF& scroll_position);

  // Returns the last-computed offset of the sticky box from its original
  // position before scroll.
  PhysicalOffset StickyOffset() const { return sticky_offset_; }

  void Trace(Visitor* visitor) const;

  std::optional<LayoutUnit> left_inset;
  std::optional<LayoutUnit> right_inset;
  std::optional<LayoutUnit> top_inset;
  std::optional<LayoutUnit> bottom_inset;

  // The rectangle in which the sticky box is able to be positioned. This may be
  // smaller than the scroller viewport due to things like padding.
  PhysicalRect constraining_rect;

  // The containing block rect and sticky box rect are the basic components
  // for calculating the sticky offset to apply after a scroll. Consider the
  // following setup:
  //
  // <scroll-container>
  //   <containing-block> (*)
  //     <sticky-element>
  //
  // (*) <containing-block> may be the same as <scroll-container>.

  // The layout position of the containing block relative to the scroll
  // container. When calculating the sticky offset it is used to ensure the
  // sticky element stays bounded by its containing block.
  PhysicalRect scroll_container_relative_containing_block_rect;

  // The layout position of the sticky element relative to the scroll container.
  // When calculating the sticky offset it is used to determine how large the
  // offset needs to be to satisfy the sticky constraints.
  PhysicalRect scroll_container_relative_sticky_box_rect;

  // In the case of nested sticky elements the layout position of the sticky
  // element and its containing block are not accurate (as they are affected by
  // ancestor sticky offsets). To ensure a correct sticky offset calculation in
  // that case we must track any sticky ancestors between the sticky element and
  // its containing block, and between its containing block and the overflow
  // clip ancestor.
  //
  // See the implementation of |ComputeStickyOffset| for documentation on how
  // these ancestors are used to correct the offset calculation.
  Member<const LayoutBoxModelObject> nearest_sticky_layer_shifting_sticky_box =
      nullptr;
  Member<const LayoutBoxModelObject>
      nearest_sticky_layer_shifting_containing_block = nullptr;

  // These fields cache the result of
  // PaintLayer::ContainingScrollContainerLayer().
  Member<const PaintLayer> containing_scroll_container_layer = nullptr;
  bool is_fixed_to_view = false;

 private:
  // For performance we cache our accumulated sticky offset to allow descendant
  // sticky elements to offset their constraint rects. Because we can either
  // affect a descendant element's sticky box constraint rect or containing
  // block constraint rect, we need to accumulate two offsets.

  // The sticky box offset accumulates the chain of sticky elements that are
  // between this sticky element and its containing block. Any descendant using
  // |total_sticky_box_sticky_offset_| has the same containing block as this
  // element, so |total_sticky_box_sticky_offset_| does not accumulate
  // containing block sticky offsets. For example, consider the following chain:
  //
  // <div style="position: sticky;">
  //   <div id="outerInline" style="position: sticky; display: inline;">
  //     <div id="innerInline" style="position: sticky; display: inline;"><div>
  //   </div>
  // </div>
  //
  // In the above example, both outerInline and innerInline have the same
  // containing block - the outermost <div>.
  PhysicalOffset total_sticky_box_sticky_offset_;

  // The containing block offset accumulates all sticky-related offsets between
  // this element and the ancestor scroller. If this element is a containing
  // block shifting ancestor for some descendant, it shifts the descendant's
  // constraint rects by its entire offset.
  PhysicalOffset total_containing_block_sticky_offset_;

  // This is the real sticky offset which is  total_sticky_box_sticky_offset_ -
  // AncestorStickyBoxOffset(). It's stored to avoid access to the
  // Member<PaintLayer> fields from StickyOffset() in case it's called during
  // layout. See the TODO in LayoutBoxModelObject::StickyPositionOffset().
  PhysicalOffset sticky_offset_;

  PhysicalOffset AncestorStickyBoxOffset() const;
  PhysicalOffset AncestorContainingBlockOffset() const;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_PAGE_SCROLLING_STICKY_POSITION_SCROLLING_CONSTRAINTS_H_
