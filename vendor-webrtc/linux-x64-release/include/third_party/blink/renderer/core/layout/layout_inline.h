/*
 * Copyright (C) 1999 Lars Knoll (knoll@kde.org)
 *           (C) 1999 Antti Koivisto (koivisto@kde.org)
 * Copyright (C) 2003, 2004, 2005, 2006, 2007, 2008, 2009 Apple Inc.
 *               All rights reserved.
 *
 * This library is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Library General Public
 * License as published by the Free Software Foundation; either
 * version 2 of the License, or (at your option) any later version.
 *
 * This library is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Library General Public License for more details.
 *
 * You should have received a copy of the GNU Library General Public License
 * along with this library; see the file COPYING.LIB.  If not, write to
 * the Free Software Foundation, Inc., 51 Franklin Street, Fifth Floor,
 * Boston, MA 02110-1301, USA.
 *
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_LAYOUT_INLINE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_LAYOUT_INLINE_H_

#include "base/check_op.h"
#include "base/notreached.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/editing/forward.h"
#include "third_party/blink/renderer/core/layout/layout_box_model_object.h"

namespace blink {

class LayoutBlockFlow;
class InlineCursor;

// LayoutInline is the LayoutObject associated with display: inline.
// This is called an "inline box" in CSS 2.1.
// http://www.w3.org/TR/CSS2/visuren.html#inline-boxes
//
// It is also the base class for content that behaves in similar way (like
// quotes and "display: ruby").
//
// Note that LayoutInline is always 'inline-level' but other LayoutObject
// can be 'inline-level', which is why it's stored as a boolean on LayoutObject
// (see LayoutObject::isInline()).
//
// For performance and memory consumption, this class ignores some inline-boxes
// during line layout because they don't impact layout (they still exist and are
// inserted into the layout tree). An example of this is
//             <span><span>Text</span></span>
// where the 2 spans have the same size as the inner text-node so they can be
// ignored for layout purpose, generating a single inline-box instead of 3.
// One downside of this optimization is that we have extra work to do when
// asking for bounding rects (see generateLineBoxRects).
// This optimization is called "culled inline" in the code.
//
// LayoutInlines are expected to be laid out by their containing
// LayoutBlockFlow. See LayoutBlockFlow::layoutInlineChildren.
//
//
// ***** CONTINUATIONS AND ANONYMOUS LAYOUTBLOCKFLOWS *****
// LayoutInline enforces the following invariant:
// "All in-flow children of an inline box are inline."
// When a non-inline child is inserted, LayoutInline::addChild splits the inline
// and potentially enclosing inlines too. It then wraps layout objects into
// anonymous block-flow containers. This creates complexity in the code as:
// - a DOM node can have several associated LayoutObjects (we don't currently
//   expose this information to the DOM code though).
// - more importantly, nodes that are parent/child in the DOM have no natural
//   relationship anymore (see example below).
// In order to do a correct tree walk over this synthetic tree, a single linked
// list is stored called *continuation*. See splitFlow() about how it is
// populated during LayoutInline split.
//
// Continuations can only be a LayoutInline or an anonymous LayoutBlockFlow.
// That's why continuations are handled by LayoutBoxModelObject (common class
// between the 2). See LayoutBoxModelObject::continuation and setContinuation.
//
// Let's take the following example:
//   <!DOCTYPE html>
//   <b>Bold inline.<div>Bold block.</div>More bold inlines.</b>
//
// The generated layout tree is:
//   LayoutBlockFlow {HTML}
//    LayoutBlockFlow {BODY}
//      LayoutBlockFlow (anonymous)
//        LayoutInline {B}
//          LayoutText {#text}
//            text run: "Bold inline."
//      LayoutBlockFlow (anonymous)
//        LayoutBlockFlow {DIV}
//          LayoutText {#text}
//            text run: "Bold block."
//      LayoutBlockFlow (anonymous)
//        LayoutInline {B}
//          LayoutText {#text}
//            text run: "More bold inlines."
//
// The insertion of the <div> inside the <b> forces the latter to be split
// into 2 LayoutInlines and the insertion of anonymous LayoutBlockFlows. The 2
// LayoutInlines are done so that we can apply the correct (bold) style to both
// sides of the <div>. The continuation chain starts with the first
// LayoutInline {B}, continues to the middle anonymous LayoutBlockFlow and
// finishes with the last LayoutInline {B}.
//
// Note that the middle anonymous LayoutBlockFlow duplicates the content.
// TODO(jchaffraix): Find out why we made the decision to always insert the
//                   anonymous LayoutBlockFlows.
//
// This section was inspired by an older article by Dave Hyatt:
// https://www.webkit.org/blog/115/webcore-rendering-ii-blocks-and-inlines/
class CORE_EXPORT LayoutInline : public LayoutBoxModelObject {
 public:
  explicit LayoutInline(Element*);

  void Trace(Visitor*) const override;

  static LayoutInline* CreateAnonymous(Document*);

  LayoutObject* FirstChild() const {
    NOT_DESTROYED();
    DCHECK_EQ(Children(), VirtualChildren());
    return Children()->FirstChild();
  }
  LayoutObject* LastChild() const {
    NOT_DESTROYED();
    DCHECK_EQ(Children(), VirtualChildren());
    return Children()->LastChild();
  }

  // If you have a LayoutInline, use firstChild or lastChild instead.
  void SlowFirstChild() const = delete;
  void SlowLastChild() const = delete;

  void AddChild(LayoutObject* new_child,
                LayoutObject* before_child = nullptr) override;

  // A block-in-inline became floated or out-of-flow positioned. The anonymous
  // wrapper around it may therefore need to be removed, if it no longer
  // contains any in-flow blocks at all.
  void BlockInInlineBecameFloatingOrOutOfFlow(
      LayoutBlockFlow* anonymous_block_child);

  Element* GetNode() const {
    NOT_DESTROYED();
    return To<Element>(LayoutBoxModelObject::GetNode());
  }

  LayoutUnit MarginLeft() const final;
  LayoutUnit MarginRight() const final;
  LayoutUnit MarginTop() const final;
  LayoutUnit MarginBottom() const final;

  // Returns the bounding box of all quads returned by `LocalQuadsForSelf`.
  gfx::RectF LocalBoundingBoxRectF() const;

  gfx::RectF LocalBoundingBoxRectForAccessibility() const final;

  PhysicalRect PhysicalLinesBoundingBox() const;
  PhysicalRect LinesVisualOverflowBoundingBox() const;
  PhysicalRect VisualOverflowRect() const final;

  bool HasInlineFragments() const final;
  wtf_size_t FirstInlineFragmentItemIndex() const final;
  void ClearFirstInlineFragmentItemIndex() final;
  void SetFirstInlineFragmentItemIndex(wtf_size_t) final;

  void AddOutlineRects(OutlineRectCollector&,
                       OutlineInfo*,
                       const PhysicalOffset& additional_offset,
                       OutlineType) const override;

  bool AlwaysCreateLineBoxes() const {
    NOT_DESTROYED();
    return AlwaysCreateLineBoxesForLayoutInline() &&
           !IsInLayoutNGInlineFormattingContext();
  }
  void SetAlwaysCreateLineBoxes(bool always_create_line_boxes = true) {
    NOT_DESTROYED();
    DCHECK(!IsInLayoutNGInlineFormattingContext());
    SetAlwaysCreateLineBoxesForLayoutInline(always_create_line_boxes);
  }

  // True if this inline box should force creation of PhysicalBoxFragment.
  bool ShouldCreateBoxFragment() const {
    NOT_DESTROYED();
    return AlwaysCreateLineBoxesForLayoutInline() &&
           IsInLayoutNGInlineFormattingContext();
  }
  void SetShouldCreateBoxFragment(bool value = true) {
    NOT_DESTROYED();
    DCHECK(IsInLayoutNGInlineFormattingContext());
    SetAlwaysCreateLineBoxesForLayoutInline(value);
  }
  void UpdateShouldCreateBoxFragment();

  PhysicalRect LocalCaretRect(int) const final;

  // When this LayoutInline doesn't generate line boxes of its own, regenerate
  // the rects of the line boxes and hit test the rects.
  // |parent_cursor| is used to limit the regenerated rects to be from
  // descendant fragments of |parent_cursor|.
  bool HitTestCulledInline(HitTestResult&,
                           const HitTestLocation&,
                           const PhysicalOffset& accumulated_offset,
                           const InlineCursor& parent_cursor);

  PhysicalOffset FirstLineBoxTopLeft() const {
    NOT_DESTROYED();
    return FirstLineBoxTopLeftInternal().value_or(PhysicalOffset());
  }

  PhysicalRect AbsoluteBoundingBoxRectHandlingEmptyInline(
      MapCoordinatesFlags = 0) const final;

  const char* GetName() const override {
    NOT_DESTROYED();
    return "LayoutInline";
  }

  PhysicalRect DebugRect() const override;

 protected:
  void WillBeDestroyed() override;

  void InLayoutNGInlineFormattingContextWillChange(bool) final;

  void StyleDidChange(StyleDifference, const ComputedStyle* old_style) override;

  void InvalidateDisplayItemClients(PaintInvalidationReason) const override;

  void QuadsInAncestorInternal(Vector<gfx::QuadF>&,
                               const LayoutBoxModelObject* ancestor,
                               MapCoordinatesFlags) const override;

 private:
  bool AbsoluteTransformDependsOnPoint(const LayoutObject& object) const;
  void QuadsForSelfInternal(Vector<gfx::QuadF>& quads,
                            const LayoutBoxModelObject* ancestor,
                            MapCoordinatesFlags mode,
                            bool map_to_ancestor) const;

  LayoutObjectChildList* VirtualChildren() final {
    NOT_DESTROYED();
    return Children();
  }
  const LayoutObjectChildList* VirtualChildren() const final {
    NOT_DESTROYED();
    return Children();
  }
  const LayoutObjectChildList* Children() const {
    NOT_DESTROYED();
    return &children_;
  }
  LayoutObjectChildList* Children() {
    NOT_DESTROYED();
    return &children_;
  }

  bool IsLayoutInline() const final {
    NOT_DESTROYED();
    return true;
  }

  // Compute the initial value of |ShouldCreateBoxFragment()| for this
  // LayoutInline. It maybe flipped to true later for other conditions.
  bool ComputeInitialShouldCreateBoxFragment() const;
  bool ComputeInitialShouldCreateBoxFragment(const ComputedStyle& style) const;

  PhysicalRect CulledInlineVisualOverflowBoundingBox() const;

  // PhysicalRectCollector should be like a function:
  // void (const PhysicalRect&).
  template <typename PhysicalRectCollector>
  void CollectLineBoxRects(const PhysicalRectCollector&) const;

  void AddChildIgnoringContinuation(LayoutObject* new_child,
                                    LayoutObject* before_child = nullptr) final;
  void AddChildAsBlockInInline(LayoutObject* new_child,
                               LayoutObject* before_child);

  // Create an anonymous block for block children of this inline.
  LayoutBlockFlow* CreateAnonymousContainerForBlockChildren() const;
  LayoutBox* CreateAnonymousBoxToSplit(
      const LayoutBox* box_to_split) const final;

  void MarkMayHaveAnchorQuery() final;

  void Paint(const PaintInfo&) const override;

  bool NodeAtPoint(HitTestResult&,
                   const HitTestLocation&,
                   const PhysicalOffset& accumulated_offset,
                   HitTestPhase) final;

  PaintLayerType LayerTypeRequired() const override;

  LayoutUnit OffsetLeft(const Element*) const final;
  LayoutUnit OffsetTop(const Element*) const final;
  LayoutUnit OffsetWidth() const final;
  LayoutUnit OffsetHeight() const final;

  bool MapToVisualRectInAncestorSpaceInternal(
      const LayoutBoxModelObject* ancestor,
      TransformState&,
      VisualRectFlags = kDefaultVisualRectFlags) const final;

  PositionWithAffinity PositionForPoint(const PhysicalOffset&) const override;

  void DirtyLinesFromChangedChild(LayoutObject*) final;

  // TODO(leviw): This should probably be an int. We don't snap equivalent lines
  // to different heights.
  LayoutUnit FirstLineHeight() const final;

  void ChildBecameNonInline(LayoutObject* child) final;

  void UpdateHitTestResult(HitTestResult&, const PhysicalOffset&) const final;

  void ImageChanged(WrappedImagePtr, CanDeferInvalidation) final;

  void AddDraggableRegions(Vector<DraggableRegionValue>&) final;

  void UpdateFromStyle() final;
  bool AnonymousHasStylePropagationOverride() final {
    NOT_DESTROYED();
    return true;
  }

  std::optional<PhysicalOffset> FirstLineBoxTopLeftInternal() const;
  PhysicalOffset AnchorPhysicalLocation() const;

  LayoutObjectChildList children_;

  // The index of the first fragment item associated with this object in
  // |FragmentItems::Items()|. Zero means there are no such item.
  // Valid only when IsInLayoutNGInlineFormattingContext().
  wtf_size_t first_fragment_item_index_ = 0u;
};

inline wtf_size_t LayoutInline::FirstInlineFragmentItemIndex() const {
  NOT_DESTROYED();
  if (!IsInLayoutNGInlineFormattingContext())
    return 0u;
  return first_fragment_item_index_;
}

template <>
struct DowncastTraits<LayoutInline> {
  static bool AllowFrom(const LayoutObject& object) {
    return object.IsLayoutInline();
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_LAYOUT_INLINE_H_
