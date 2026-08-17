/*
 * Copyright (C) 1999 Lars Knoll (knoll@kde.org)
 *           (C) 1999 Antti Koivisto (koivisto@kde.org)
 * Copyright (C) 2003, 2006, 2007 Apple Inc. All rights reserved.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_LAYOUT_BOX_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_LAYOUT_BOX_H_

#include <memory>

#include "base/check_op.h"
#include "base/dcheck_is_on.h"
#include "base/gtest_prod_util.h"
#include "base/notreached.h"
#include "third_party/blink/public/mojom/scroll/scroll_into_view_params.mojom-blink-forward.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/layout/geometry/physical_rect.h"
#include "third_party/blink/renderer/core/layout/layout_box_model_object.h"
#include "third_party/blink/renderer/core/layout/layout_object.h"
#include "third_party/blink/renderer/core/layout/min_max_sizes.h"
#include "third_party/blink/renderer/core/layout/min_max_sizes_cache.h"
#include "third_party/blink/renderer/core/layout/overflow_model.h"
#include "third_party/blink/renderer/core/paint/paint_layer_scrollable_area.h"
#include "third_party/blink/renderer/core/scroll/scrollbar_theme.h"
#include "third_party/blink/renderer/core/style/style_overflow_clip_margin.h"
#include "third_party/blink/renderer/platform/graphics/overlay_scrollbar_clip_behavior.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_set.h"

namespace blink {

class AnchorPositionScrollData;
class BlockBreakToken;
class ColumnSpannerPath;
class ConstraintSpace;
class CustomLayoutChild;
class EarlyBreak;
class Element;
class LayoutMultiColumnSpannerPlaceholder;
class LayoutResult;
class MeasureCache;
class PhysicalBoxFragment;
class ShapeOutsideInfo;
class WritingModeConverter;
enum class LayoutCacheStatus;
struct FragmentGeometry;
struct NonOverflowingScrollRange;
struct PaintInfo;
struct PhysicalBoxStrut;

enum BackgroundRectType {
  kBackgroundPaintedExtent,
  kBackgroundKnownOpaqueRect,
};

enum ShouldClampToContentBox { kDoNotClampToContentBox, kClampToContentBox };

enum ShouldIncludeScrollbarGutter {
  kExcludeScrollbarGutter,
  kIncludeScrollbarGutter
};

struct LayoutBoxRareData final : public GarbageCollected<LayoutBoxRareData> {
 public:
  LayoutBoxRareData();
  LayoutBoxRareData(const LayoutBoxRareData&) = delete;
  LayoutBoxRareData& operator=(const LayoutBoxRareData&) = delete;

  void Trace(Visitor* visitor) const;

  // For spanners, the spanner placeholder that lays us out within the multicol
  // container.
  Member<LayoutMultiColumnSpannerPlaceholder> spanner_placeholder_;

  bool has_override_containing_block_content_logical_width_ : 1;
  bool has_previous_content_box_rect_ : 1;

  LayoutUnit override_containing_block_content_logical_width_;

  // Used by BoxPaintInvalidator. Stores the previous content rect after the
  // last paint invalidation. It's valid if has_previous_content_box_rect_ is
  // true.
  PhysicalRect previous_physical_content_box_rect_;

  // Used by CSSLayoutDefinition::Instance::Layout. Represents the script
  // object for this box that web developers can query style, and perform
  // layout upon. Only created if IsCustomItem() is true.
  Member<CustomLayoutChild> layout_child_;
};

// LayoutBox implements the full CSS box model.
//
// LayoutBoxModelObject only introduces some abstractions for LayoutInline and
// LayoutBox. The logic for the model is in LayoutBox, e.g. the storage for the
// rectangle and offset forming the CSS box (frame_location_ and frame_size_)
// and the getters for the different boxes.
//
// LayoutBox is also the uppermost class to support scrollbars, however the
// logic is delegated to PaintLayerScrollableArea.
// Per the CSS specification, scrollbars should "be inserted between the inner
// border edge and the outer padding edge".
// (see http://www.w3.org/TR/CSS21/visufx.html#overflow)
// Also the scrollbar width / height are removed from the content box. Taking
// the following example:
//
// <!DOCTYPE html>
// <style>
// ::-webkit-scrollbar {
//     /* Force non-overlay scrollbars */
//     width: 10px;
//     height: 20px;
// }
// </style>
// <div style="overflow:scroll; width: 100px; height: 100px">
//
// The <div>'s content box is not 100x100 as specified in the style but 90x80 as
// we remove the scrollbars from the box.
//
// The presence of scrollbars is determined by the 'overflow' property and can
// be conditioned on having scrollable overflow (see OverflowModel for more
// details on how we track overflow).
//
// There are 2 types of scrollbars:
// - non-overlay scrollbars take space from the content box.
// - overlay scrollbars don't and just overlay hang off from the border box,
//   potentially overlapping with the padding box's content.
// For more details on scrollbars, see PaintLayerScrollableArea.
//
//
// ***** THE BOX MODEL *****
// The CSS box model is based on a series of nested boxes:
// http://www.w3.org/TR/CSS21/box.html
//
//       |----------------------------------------------------|
//       |                                                    |
//       |                   margin-top                       |
//       |                                                    |
//       |     |-----------------------------------------|    |
//       |     |                                         |    |
//       |     |             border-top                  |    |
//       |     |                                         |    |
//       |     |    |--------------------------|----|    |    |
//       |     |    |                          |    |    |    |
//       |     |    |       padding-top        |####|    |    |
//       |     |    |                          |####|    |    |
//       |     |    |    |----------------|    |####|    |    |
//       |     |    |    |                |    |    |    |    |
//       | ML  | BL | PL |  content box   | PR | SW | BR | MR |
//       |     |    |    |                |    |    |    |    |
//       |     |    |    |----------------|    |    |    |    |
//       |     |    |                          |    |    |    |
//       |     |    |      padding-bottom      |    |    |    |
//       |     |    |--------------------------|----|    |    |
//       |     |    |                      ####|    |    |    |
//       |     |    |     scrollbar height ####| SC |    |    |
//       |     |    |                      ####|    |    |    |
//       |     |    |-------------------------------|    |    |
//       |     |                                         |    |
//       |     |           border-bottom                 |    |
//       |     |                                         |    |
//       |     |-----------------------------------------|    |
//       |                                                    |
//       |                 margin-bottom                      |
//       |                                                    |
//       |----------------------------------------------------|
//
// BL = border-left
// BR = border-right
// ML = margin-left
// MR = margin-right
// PL = padding-left
// PR = padding-right
// SC = scroll corner (contains UI for resizing (see the 'resize' property)
// SW = scrollbar width
//
// Note that the vertical scrollbar (if existing) will be on the left in
// right-to-left direction and horizontal writing-mode. The horizontal scrollbar
// (if existing) is always at the bottom.
//
// Those are just the boxes from the CSS model. Extra boxes are tracked by Blink
// (e.g. the overflows). Thus it is paramount to know which box a function is
// manipulating. Also of critical importance is the coordinate system used (see
// the COORDINATE SYSTEMS section in LayoutBoxModelObject).
class CORE_EXPORT LayoutBox : public LayoutBoxModelObject {
 public:
  explicit LayoutBox(ContainerNode*);
  void Trace(Visitor*) const override;

  PaintLayerType LayerTypeRequired() const override;

  bool BackgroundIsKnownToBeOpaqueInRect(
      const PhysicalRect& local_rect) const override;

  virtual bool BackgroundShouldAlwaysBeClipped() const {
    NOT_DESTROYED();
    return false;
  }

  // Use this with caution! No type checking is done!
  LayoutBox* FirstChildBox() const;
  LayoutBox* LastChildBox() const;

  // Returns the LogicalRect of this box for LocationContainer()'s writing-mode.
  // The coordinate origin is the border corner of the LocationContainer().
  // This function doesn't take into account of TextDirection.
  LogicalRect LogicalRectInContainer() const;

  // Returns the inline-size for this box's writing-mode.  It might be
  // different from container's writing-mode.
  LayoutUnit LogicalWidth() const {
    NOT_DESTROYED();
    PhysicalSize size = Size();
    return StyleRef().IsHorizontalWritingMode() ? size.width : size.height;
  }
  // Returns the block-size for this box's writing-mode.  It might be
  // different from container's writing-mode.
  LayoutUnit LogicalHeight() const {
    NOT_DESTROYED();
    PhysicalSize size = Size();
    return StyleRef().IsHorizontalWritingMode() ? size.height : size.width;
  }

  LayoutUnit LogicalHeightForEmptyLine() const {
    NOT_DESTROYED();
    return FirstLineHeight();
  }

  virtual PhysicalSize Size() const;

  void SetLocation(const DeprecatedLayoutPoint& location) {
    NOT_DESTROYED();
    if (location == frame_location_) {
      return;
    }
    frame_location_ = location;
    LocationChanged();
  }

  // The ancestor box that this object's PhysicalLocation is relative to.
  virtual LayoutBox* LocationContainer() const;

  // Note that those functions have their origin at this box's CSS border box.
  // As such their location doesn't account for 'top'/'left'.
  PhysicalRect PhysicalBorderBoxRect() const {
    NOT_DESTROYED();
    return PhysicalRect(PhysicalOffset(), Size());
  }

  // Client rect and padding box rect are the same concept.
  DISABLE_CFI_PERF PhysicalRect PhysicalPaddingBoxRect() const {
    NOT_DESTROYED();
    return PhysicalRect(ClientLeft(), ClientTop(), ClientWidth(),
                        ClientHeight());
  }

  // The content area of the box (excludes padding - and intrinsic padding for
  // table cells, etc... - and scrollbars and border).
  DISABLE_CFI_PERF PhysicalRect PhysicalContentBoxRect() const {
    NOT_DESTROYED();
    return PhysicalRect(ContentLeft(), ContentTop(), ContentWidth(),
                        ContentHeight());
  }
  PhysicalOffset PhysicalContentBoxOffset() const {
    NOT_DESTROYED();
    return PhysicalOffset(ContentLeft(), ContentTop());
  }
  PhysicalSize PhysicalContentBoxSize() const {
    NOT_DESTROYED();
    return PhysicalSize(ContentWidth(), ContentHeight());
  }
  // The content box converted to absolute coords (taking transforms into
  // account).
  gfx::QuadF AbsoluteContentQuad(MapCoordinatesFlags = 0) const;

  // The enclosing rectangle of the background with given opacity requirement.
  PhysicalRect PhysicalBackgroundRect(BackgroundRectType) const;

  // This returns the content area of the box (excluding padding and border).
  // The only difference with contentBoxRect is that ComputedCSSContentBoxRect
  // does include the intrinsic padding in the content box as this is what some
  // callers expect (like getComputedStyle).
  PhysicalRect ComputedCSSContentBoxRect() const {
    NOT_DESTROYED();
    return PhysicalRect(
        BorderLeft() + ComputedCSSPaddingLeft(),
        BorderTop() + ComputedCSSPaddingTop(),
        ClientWidth() - ComputedCSSPaddingLeft() - ComputedCSSPaddingRight(),
        ClientHeight() - ComputedCSSPaddingTop() - ComputedCSSPaddingBottom());
  }

  void AddOutlineRects(OutlineRectCollector&,
                       OutlineInfo*,
                       const PhysicalOffset& additional_offset,
                       OutlineType) const override;

  // Use this with caution! No type checking is done!
  LayoutBox* PreviousSiblingBox() const;
  LayoutBox* NextSiblingBox() const;
  LayoutBox* ParentBox() const;

  // Return the previous sibling column set or spanner placeholder. Only to be
  // used on multicol container children.
  LayoutBox* PreviousSiblingMultiColumnBox() const;
  // Return the next sibling column set or spanner placeholder. Only to be used
  // on multicol container children.
  LayoutBox* NextSiblingMultiColumnBox() const;

  bool CanResize() const;

  DISABLE_CFI_PERF PhysicalRect NoOverflowRect() const {
    NOT_DESTROYED();
    return PhysicalPaddingBoxRect();
  }
  PhysicalRect ScrollableOverflowRect() const {
    NOT_DESTROYED();
    DCHECK(!IsLayoutMultiColumnSet());
    return ScrollableOverflowIsSet()
               ? overflow_->scrollable_overflow->ScrollableOverflowRect()
               : NoOverflowRect();
  }

  PhysicalRect VisualOverflowRect() const final;
  // VisualOverflow has DCHECK for reading before it is computed. These
  // functions pretend there is no visual overflow when it is not computed.
  // TODO(crbug.com/1205708): Audit the usages and fix issues.
#if DCHECK_IS_ON()
  PhysicalRect VisualOverflowRectAllowingUnset() const;
#else
  ALWAYS_INLINE PhysicalRect VisualOverflowRectAllowingUnset() const {
    NOT_DESTROYED();
    return VisualOverflowRect();
  }
#endif

  PhysicalRect SelfVisualOverflowRect() const {
    NOT_DESTROYED();
    return VisualOverflowIsSet()
               ? overflow_->visual_overflow->SelfVisualOverflowRect()
               : PhysicalBorderBoxRect();
  }
  PhysicalRect ContentsVisualOverflowRect() const {
    NOT_DESTROYED();
    return VisualOverflowIsSet()
               ? overflow_->visual_overflow->ContentsVisualOverflowRect()
               : PhysicalRect();
  }

  // These methods don't mean the box *actually* has top/left overflow. They
  // mean that *if* the box overflows, it will overflow to the top/left rather
  // than the bottom/right. This happens when child content is laid out
  // right-to-left (e.g. direction:rtl) or or bottom-to-top (e.g. direction:rtl
  // writing-mode:vertical-rl).
  virtual bool HasTopOverflow() const;
  virtual bool HasLeftOverflow() const;

  // Sets the scrollable-overflow from the current set of layout-results.
  void SetScrollableOverflowFromLayoutResults();

  void AddSelfVisualOverflow(const PhysicalRect& r);
  void AddContentsVisualOverflow(const PhysicalRect& r);
  void UpdateHasSubpixelVisualEffectOutsets(const PhysicalBoxStrut&);

  PhysicalBoxStrut ComputeVisualEffectOverflowOutsets();

  void ClearVisualOverflow();

  bool CanUseFragmentsForVisualOverflow() const;
  void CopyVisualOverflowFromFragments();

  virtual void UpdateAfterLayout();

  DISABLE_CFI_PERF LayoutUnit ContentLeft() const {
    NOT_DESTROYED();
    return ClientLeft() + PaddingLeft();
  }
  DISABLE_CFI_PERF LayoutUnit ContentTop() const {
    NOT_DESTROYED();
    return ClientTop() + PaddingTop();
  }
  DISABLE_CFI_PERF LayoutUnit ContentWidth() const {
    NOT_DESTROYED();
    // We're dealing with LayoutUnit and saturated arithmetic here, so we need
    // to guard against negative results. The value returned from clientWidth()
    // may in itself be a victim of saturated arithmetic; e.g. if both border
    // sides were sufficiently wide (close to LayoutUnit::max()).  Here we
    // subtract two padding values from that result, which is another source of
    // saturated arithmetic.
    return (ClientWidth() - PaddingLeft() - PaddingRight())
        .ClampNegativeToZero();
  }
  DISABLE_CFI_PERF LayoutUnit ContentHeight() const {
    NOT_DESTROYED();
    // We're dealing with LayoutUnit and saturated arithmetic here, so we need
    // to guard against negative results. The value returned from clientHeight()
    // may in itself be a victim of saturated arithmetic; e.g. if both border
    // sides were sufficiently wide (close to LayoutUnit::max()).  Here we
    // subtract two padding values from that result, which is another source of
    // saturated arithmetic.
    return (ClientHeight() - PaddingTop() - PaddingBottom())
        .ClampNegativeToZero();
  }
  PhysicalSize ContentSize() const {
    NOT_DESTROYED();
    return PhysicalSize(ContentWidth(), ContentHeight());
  }
  LayoutUnit ContentLogicalWidth() const {
    NOT_DESTROYED();
    return StyleRef().IsHorizontalWritingMode() ? ContentWidth()
                                                : ContentHeight();
  }
  LayoutUnit ContentLogicalHeight() const {
    NOT_DESTROYED();
    return StyleRef().IsHorizontalWritingMode() ? ContentHeight()
                                                : ContentWidth();
  }

  // CSS intrinsic sizing getters.
  // https://drafts.csswg.org/css-sizing-4/#intrinsic-size-override
  LayoutUnit OverrideIntrinsicContentInlineSize() const;
  LayoutUnit OverrideIntrinsicContentBlockSize() const;

  // Returns element-native intrinsic size. Returns kIndefiniteSize if no such
  // size.
  LayoutUnit DefaultIntrinsicContentInlineSize() const;
  LayoutUnit DefaultIntrinsicContentBlockSize() const;

  // IE extensions. Used to calculate offsetWidth/Height. Overridden by inlines
  // (LayoutFlow) to return the remaining width on a given line (and the height
  // of a single line).
  LayoutUnit OffsetWidth() const final {
    NOT_DESTROYED();
    return Size().width;
  }
  LayoutUnit OffsetHeight() const final {
    NOT_DESTROYED();
    return Size().height;
  }

  bool UsesOverlayScrollbars() const;

  // Physical client rect (a.k.a. PhysicalPaddingBoxRect(), defined by
  // ClientLeft, ClientTop, ClientWidth and ClientHeight) represents the
  // interior of an object excluding borders and scrollbars.
  // Clamps the left scrollbar size so it is not wider than the content box.
  DISABLE_CFI_PERF LayoutUnit ClientLeft() const {
    NOT_DESTROYED();
    if (CanSkipComputeScrollbars())
      return BorderLeft();
    else
      return BorderLeft() + ComputeScrollbarsInternal(kClampToContentBox).left;
  }
  DISABLE_CFI_PERF LayoutUnit ClientTop() const {
    NOT_DESTROYED();
    if (CanSkipComputeScrollbars())
      return BorderTop();
    else
      return BorderTop() + ComputeScrollbarsInternal(kClampToContentBox).top;
  }

  // Size without borders and scrollbars.
  LayoutUnit ClientWidth() const;
  LayoutUnit ClientHeight() const;
  // Similar to ClientWidth() and ClientHeight(), but based on the specified
  // border-box size.
  LayoutUnit ClientWidthFrom(LayoutUnit width) const;
  LayoutUnit ClientHeightFrom(LayoutUnit height) const;
  DISABLE_CFI_PERF LayoutUnit ClientLogicalWidth() const {
    NOT_DESTROYED();
    return IsHorizontalWritingMode() ? ClientWidth() : ClientHeight();
  }
  DISABLE_CFI_PERF LayoutUnit ClientLogicalHeight() const {
    NOT_DESTROYED();
    return IsHorizontalWritingMode() ? ClientHeight() : ClientWidth();
  }

  LayoutUnit ClientWidthWithTableSpecialBehavior() const;
  LayoutUnit ClientHeightWithTableSpecialBehavior() const;

  // scrollWidth/scrollHeight will be the same as clientWidth/clientHeight
  // unless the object has overflow:hidden/scroll/auto specified and also has
  // overflow. These methods are virtual so that objects like textareas can
  // scroll shadow content (but pretend that they are the objects that are
  // scrolling).

  // Replaced ScrollLeft/Top by using Element::GetLayoutBoxForScrolling to
  // return the correct ScrollableArea.
  // TODO(cathiechen): We should do the same with ScrollWidth|Height .
  virtual LayoutUnit ScrollWidth() const;
  virtual LayoutUnit ScrollHeight() const;

  PhysicalBoxStrut MarginBoxOutsets() const;
  LayoutUnit MarginTop() const override {
    NOT_DESTROYED();
    return MarginBoxOutsets().top;
  }
  LayoutUnit MarginBottom() const override {
    NOT_DESTROYED();
    return MarginBoxOutsets().bottom;
  }
  LayoutUnit MarginLeft() const override {
    NOT_DESTROYED();
    return MarginBoxOutsets().left;
  }
  LayoutUnit MarginRight() const override {
    NOT_DESTROYED();
    return MarginBoxOutsets().right;
  }

  // Get the scroll marker group associated with this box, if any.
  LayoutBlock* GetScrollMarkerGroup();

  // Get the scroller that owns this scroll marker group.
  LayoutBlock* ScrollerFromScrollMarkerGroup() const;

  void QuadsInAncestorInternal(Vector<gfx::QuadF>&,
                               const LayoutBoxModelObject* ancestor,
                               MapCoordinatesFlags) const override;
  gfx::RectF LocalBoundingBoxRectForAccessibility() const override;

  void LayoutSubtreeRoot();

  void Paint(const PaintInfo&) const override;

  virtual bool IsInSelfHitTestingPhase(HitTestPhase phase) const {
    NOT_DESTROYED();
    return phase == HitTestPhase::kForeground;
  }

  bool HitTestAllPhases(HitTestResult&,
                        const HitTestLocation&,
                        const PhysicalOffset& accumulated_offset) final;
  bool NodeAtPoint(HitTestResult&,
                   const HitTestLocation&,
                   const PhysicalOffset& accumulated_offset,
                   HitTestPhase) override;
  bool HasHitTestableOverflow() const;
  // Fast check if |NodeAtPoint| may find a hit.
  bool MayIntersect(const HitTestResult& result,
                    const HitTestLocation& hit_test_location,
                    const PhysicalOffset& accumulated_offset) const;

  LayoutUnit OverrideContainingBlockContentLogicalWidth() const;
  bool HasOverrideContainingBlockContentLogicalWidth() const;
  void SetOverrideContainingBlockContentLogicalWidth(LayoutUnit);
  void ClearOverrideContainingBlockContentSize();

  enum PageBoundaryRule { kAssociateWithFormerPage, kAssociateWithLatterPage };

  bool HasInlineFragments() const final;
  wtf_size_t FirstInlineFragmentItemIndex() const final;
  void ClearFirstInlineFragmentItemIndex() final;
  void SetFirstInlineFragmentItemIndex(wtf_size_t) final;

  static void InvalidateItems(const LayoutResult&);

  void AddMeasureLayoutResult(const LayoutResult*);
  void SetCachedLayoutResult(const LayoutResult*, wtf_size_t index);

  // Store one layout result (with its physical fragment) at the specified
  // index.
  //
  // If there's already a result at the specified index, use
  // ReplaceLayoutResult() to do the job. Otherwise, use AppendLayoutResult().
  //
  // If it's going to be the last result, we'll also perform any necessary
  // finalization (see FinalizeLayoutResults()), and also delete all the old
  // entries following it (if there used to be more results in a previous
  // layout).
  //
  // In a few specific cases we'll even delete the entries following this
  // result, even if it's *not* going to be the last one. This is necessary when
  // we might read out the layout results again before we've got to the end (OOF
  // block fragmentation, etc.). In all other cases, we'll leave the old results
  // until we're done, as deleting entries will trigger unnecessary paint
  // invalidation. With any luck, we'll end up with the same number of results
  // as the last time, so that paint invalidation might not be necessary.
  void SetLayoutResult(const LayoutResult*, wtf_size_t index);

  // Append one layout result at the end.
  void AppendLayoutResult(const LayoutResult*);

  // Replace a specific layout result. Also perform finalization if it's the
  // last result (see FinalizeLayoutResults()), but this function does not
  // delete any (old) results following this one. Callers should generally use
  // SetLayoutResult() instead of this one, unless they have good reasons not
  // to.
  void ReplaceLayoutResult(const LayoutResult*, wtf_size_t index);

  void ShrinkLayoutResults(wtf_size_t results_to_keep);

  // Perform any finalization needed after all the layout results have been
  // added.
  void FinalizeLayoutResults();

  void RebuildFragmentTreeSpine();

  const LayoutResult* GetCachedLayoutResult(const BlockBreakToken*) const;
  const LayoutResult* GetCachedMeasureResult(
      const ConstraintSpace&,
      std::optional<FragmentGeometry>* fragment_geometry) const;

  // Call in situations where we know that there's at most one fragment. A
  // DCHECK will fail if there are multiple fragments.
  const LayoutResult* GetSingleCachedLayoutResult() const;

  // Retrieves the last (retrieved or set) measure LayoutResult, for
  // unit-testing purposes only.
  const LayoutResult* GetSingleCachedMeasureResultForTesting() const;

  // Returns the last layout result for this block flow with the given
  // constraint space and break token, or null if it is not up-to-date or
  // otherwise unavailable.
  //
  // This method (while determining if the layout result can be reused), *may*
  // calculate the |initial_fragment_geometry| of the node.
  //
  // |out_cache_status| indicates what type of layout pass is required.
  //
  // TODO(ikilpatrick): Move this function into BlockNode.
  const LayoutResult* CachedLayoutResult(
      const ConstraintSpace&,
      const BlockBreakToken*,
      const EarlyBreak*,
      const ColumnSpannerPath*,
      std::optional<FragmentGeometry>* initial_fragment_geometry,
      LayoutCacheStatus* out_cache_status);

  using LayoutResultList = HeapVector<Member<const LayoutResult>, 1>;
  class PhysicalFragmentList {
    STACK_ALLOCATED();

   public:
    explicit PhysicalFragmentList(const LayoutResultList& layout_results)
        : layout_results_(layout_results) {}

    wtf_size_t Size() const { return layout_results_.size(); }
    bool IsEmpty() const { return layout_results_.empty(); }

    bool MayHaveFragmentItems() const;
    bool HasFragmentItems() const {
      return MayHaveFragmentItems() && SlowHasFragmentItems();
    }
    bool SlowHasFragmentItems() const;

    wtf_size_t IndexOf(const PhysicalBoxFragment& fragment) const;
    bool Contains(const PhysicalBoxFragment& fragment) const;

    // Note: We can't use std::views.  It's banned in Chromium.
    class CORE_EXPORT Iterator {
     public:
      using iterator_category = std::forward_iterator_tag;
      using value_type = PhysicalBoxFragment;
      using difference_type = std::ptrdiff_t;
      using pointer = PhysicalBoxFragment*;
      using reference = PhysicalBoxFragment&;

      constexpr Iterator() = default;
      explicit Iterator(const LayoutResultList::const_iterator& iterator)
          : iterator_(iterator) {}

      const PhysicalBoxFragment& operator*() const;

      UNSAFE_BUFFER_USAGE Iterator& operator++() {
        // SAFETY: This is not safe. We should not use this operator directly.
        UNSAFE_BUFFERS(++iterator_);
        return *this;
      }
      UNSAFE_BUFFER_USAGE Iterator operator++(int) {
        Iterator copy = *this;
        // SAFETY: This is not safe. We should not use this operator directly.
        UNSAFE_BUFFERS(++*this);
        return copy;
      }

      bool operator==(const Iterator& other) const {
        return iterator_ == other.iterator_;
      }
      bool operator!=(const Iterator& other) const {
        return !operator==(other);
      }

     private:
      LayoutResultList::const_iterator iterator_;
    };

    Iterator begin() const { return Iterator(layout_results_.begin()); }
    Iterator end() const { return Iterator(layout_results_.end()); }

    const PhysicalBoxFragment& front() const;
    const PhysicalBoxFragment& back() const;

   private:
    const LayoutResultList& layout_results_;
  };

  PhysicalFragmentList PhysicalFragments() const {
    NOT_DESTROYED();
    return PhysicalFragmentList(layout_results_);
  }
  const LayoutResult* GetLayoutResult(wtf_size_t i) const;
  const LayoutResultList& GetLayoutResults() const {
    NOT_DESTROYED();
    return layout_results_;
  }
  const PhysicalBoxFragment* GetPhysicalFragment(wtf_size_t i) const;
  const FragmentData* FragmentDataFromPhysicalFragment(
      const PhysicalBoxFragment&) const;
  wtf_size_t PhysicalFragmentCount() const {
    NOT_DESTROYED();
    return layout_results_.size();
  }

  bool IsFragmentLessBox() const final {
    NOT_DESTROYED();
    return !PhysicalFragmentCount();
  }

  void SetSpannerPlaceholder(LayoutMultiColumnSpannerPlaceholder&);
  void ClearSpannerPlaceholder();
  LayoutMultiColumnSpannerPlaceholder* SpannerPlaceholder() const final {
    NOT_DESTROYED();
    return rare_data_ ? rare_data_->spanner_placeholder_.Get() : nullptr;
  }

  bool MapToVisualRectInAncestorSpaceInternal(
      const LayoutBoxModelObject* ancestor,
      TransformState&,
      VisualRectFlags = kDefaultVisualRectFlags) const override;

  LayoutUnit ContainingBlockLogicalHeightForRelPositioned() const;

  LayoutUnit ContainingBlockLogicalWidthForContent() const override;

  // Block flows subclass availableWidth/Height to handle multi column layout
  // (shrinking the width/height available to children when laying out.)
  LayoutUnit AvailableLogicalWidth() const {
    NOT_DESTROYED();
    return ContentLogicalWidth();
  }

  // Return both scrollbars and scrollbar gutters (defined by scrollbar-gutter).
  inline PhysicalBoxStrut ComputeScrollbars() const {
    NOT_DESTROYED();
    if (CanSkipComputeScrollbars())
      return PhysicalBoxStrut();
    else
      return ComputeScrollbarsInternal();
  }
  inline BoxStrut ComputeLogicalScrollbars() const {
    NOT_DESTROYED();
    if (CanSkipComputeScrollbars()) {
      return BoxStrut();
    } else {
      return ComputeScrollbarsInternal().ConvertToLogical(
          StyleRef().GetWritingDirection());
    }
  }

  bool IsUserScrollable() const;
  virtual void Autoscroll(const PhysicalOffset&);
  PhysicalOffset CalculateAutoscrollDirection(
      const gfx::PointF& point_in_root_frame) const;
  static LayoutBox* FindAutoscrollable(LayoutObject*,
                                       bool is_middle_click_autoscroll);
  static bool HasHorizontallyScrollableAncestor(LayoutObject*);

  DISABLE_CFI_PERF bool HasAutoVerticalScrollbar() const {
    NOT_DESTROYED();
    return HasNonVisibleOverflow() && StyleRef().HasAutoVerticalScroll();
  }
  DISABLE_CFI_PERF bool HasAutoHorizontalScrollbar() const {
    NOT_DESTROYED();
    return HasNonVisibleOverflow() && StyleRef().HasAutoHorizontalScroll();
  }
  DISABLE_CFI_PERF bool ScrollsOverflow() const {
    NOT_DESTROYED();
    return HasNonVisibleOverflow() && StyleRef().ScrollsOverflow();
  }
  // We place block-direction scrollbar on the left only if the writing-mode
  // is horizontal, so ShouldPlaceVerticalScrollbarOnLeft() is the same as
  // ShouldPlaceBlockDirectionScrollbarOnLogicalLeft(). The two forms can be
  // used in different contexts, e.g. the former for physical coordinate
  // contexts, and the later for logical coordinate contexts.
  bool ShouldPlaceVerticalScrollbarOnLeft() const {
    NOT_DESTROYED();
    return ShouldPlaceBlockDirectionScrollbarOnLogicalLeft();
  }
  virtual bool ShouldPlaceBlockDirectionScrollbarOnLogicalLeft() const {
    NOT_DESTROYED();
    return StyleRef().ShouldPlaceBlockDirectionScrollbarOnLogicalLeft();
  }

  bool HasScrollableOverflowX() const {
    NOT_DESTROYED();
    return ScrollsOverflowX() && ScrollWidth() != ClientWidth();
  }
  bool HasScrollableOverflowY() const {
    NOT_DESTROYED();
    return ScrollsOverflowY() && ScrollHeight() != ClientHeight();
  }
  bool ScrollsOverflowX() const {
    NOT_DESTROYED();
    return HasNonVisibleOverflow() && StyleRef().ScrollsOverflowX();
  }
  bool ScrollsOverflowY() const {
    NOT_DESTROYED();
    return HasNonVisibleOverflow() && StyleRef().ScrollsOverflowY();
  }

  // Return true if this box is monolithic, i.e. unbreakable in a fragmentation
  // context.
  virtual bool IsMonolithic() const;

  bool HasUnsplittableScrollingOverflow() const;

  PhysicalRect LocalCaretRect(int caret_offset) const override;

  // Returns the intersection of all overflow clips which apply.
  virtual PhysicalRect OverflowClipRect(
      const PhysicalOffset& location,
      OverlayScrollbarClipBehavior = kIgnoreOverlayScrollbarSize) const;
  PhysicalRect ClipRect(const PhysicalOffset& location) const;

  // Returns the combination of overflow clip, contain: paint clip and CSS clip
  // for this object.
  PhysicalRect ClippingRect(const PhysicalOffset& location) const;

  void ImageChanged(WrappedImagePtr, CanDeferInvalidation) override;
  ResourcePriority ComputeResourcePriority() const final;

  PositionWithAffinity PositionForPointInFragments(const PhysicalOffset&) const;

  virtual bool CreatesNewFormattingContext() const {
    NOT_DESTROYED();
    return true;
  }
  bool ShouldBeConsideredAsReplaced() const;

  // Return true if this block establishes a fragmentation context root (e.g. a
  // multicol container).
  virtual bool IsFragmentationContextRoot() const {
    NOT_DESTROYED();
    return false;
  }

  bool IsWritingModeRoot() const {
    NOT_DESTROYED();
    return !Parent() ||
           Parent()->StyleRef().GetWritingMode() != StyleRef().GetWritingMode();
  }

  bool IsCustomItem() const;

  bool IsFlexItem() const {
    NOT_DESTROYED();
    return !IsInline() && !IsOutOfFlowPositioned() && Parent() &&
           Parent()->IsFlexibleBox();
  }

  bool IsGridItem() const {
    NOT_DESTROYED();
    return Parent() && Parent()->IsLayoutGrid();
  }

  bool IsMasonryItem() const {
    NOT_DESTROYED();
    return Parent() && Parent()->IsLayoutMasonry();
  }

  bool IsMathItem() const {
    NOT_DESTROYED();
    return Parent() && Parent()->IsMathML();
  }

  LayoutUnit FirstLineHeight() const override;

  PhysicalOffset OffsetPoint(const Element* parent) const;
  LayoutUnit OffsetLeft(const Element*) const final;
  LayoutUnit OffsetTop(const Element*) const final;

  // Create a new WritingModeConverter to handle offsets and rectangles inside
  // this container. This ignores TextDirection.
  WritingModeConverter CreateWritingModeConverter() const;

  // Passing |location_container| causes flipped-block flipping w.r.t.
  // that container, or LocationContainer() otherwise.
  PhysicalOffset PhysicalLocation(
      const LayoutBox* location_container = nullptr) const {
    NOT_DESTROYED();
    return PhysicalLocationInternal(location_container ? location_container
                                                       : LocationContainer());
  }

  bool HasSelfVisualOverflow() const {
    NOT_DESTROYED();
    return VisualOverflowIsSet() &&
           !PhysicalBorderBoxRect().Contains(
               overflow_->visual_overflow->SelfVisualOverflowRect());
  }

  bool HasVisualOverflow() const {
    NOT_DESTROYED();
    return VisualOverflowIsSet();
  }
  bool HasScrollableOverflow() const {
    NOT_DESTROYED();
    return ScrollableOverflowIsSet();
  }

  // Returns true if reading flow should be used on this LayoutBox's content.
  // https://drafts.csswg.org/css-display-4/#reading-flow
  bool IsReadingFlowContainer() const;
  // Returns the nodes corresponding to this LayoutBox's layout children,
  // sorted in reading flow if IsReadingFlowContainer().
  const HeapVector<Member<Node>>& ReadingFlowNodes() const;

  // See README.md for an explanation of scroll origin.
  gfx::Vector2d OriginAdjustmentForScrollbars() const;
  gfx::Point ScrollOrigin() const;
  PhysicalOffset ScrolledContentOffset() const;

  // Scroll offset as snapped to physical pixels. This value should be used in
  // any values used after layout and inside "layout code" that cares about
  // where the content is displayed, rather than what the ideal offset is. For
  // most other cases ScrolledContentOffset is probably more appropriate. This
  // is the offset that's actually drawn to the screen.
  // TODO(crbug.com/962299): Pixel-snapping before PrePaint (when we know the
  // paint offset) is incorrect.
  gfx::Vector2d PixelSnappedScrolledContentOffset() const;

  // Maps from scrolling contents space to box space and apply overflow
  // clip if needed. Returns true if no clipping applied or the flattened quad
  // bounds actually intersects the clipping region. If edgeInclusive is true,
  // then this method may return true even if the resulting rect has zero area.
  //
  // When applying offsets and not clips, the TransformAccumulation is
  // respected. If there is a clip, the TransformState is flattened first.
  bool MapContentsRectToBoxSpace(
      TransformState&,
      TransformState::TransformAccumulation,
      const LayoutObject& contents,
      VisualRectFlags = kDefaultVisualRectFlags) const;

  // True if the contents scroll relative to this object. |this| must be a
  // containing block for |contents|.
  bool ContainedContentsScroll(const LayoutObject& contents) const;

  // Applies the box clip. This is like mapScrollingContentsRectToBoxSpace,
  // except it does not apply scroll.
  bool ApplyBoxClips(TransformState&,
                     TransformState::TransformAccumulation,
                     VisualRectFlags) const;

  // The optional |size| parameter is used if the size of the object isn't
  // correct yet.
  gfx::PointF PerspectiveOrigin(const PhysicalSize* size = nullptr) const;

  // Maps the visual rect state |transform_state| from this box into its
  // container, applying adjustments for the given container offset,
  // scrolling, container clipping, and transform (including container
  // perspective).
  bool MapVisualRectToContainer(const LayoutObject* container_object,
                                const PhysicalOffset& container_offset,
                                const LayoutObject* ancestor,
                                VisualRectFlags,
                                TransformState&) const;

  virtual LayoutBox* CreateAnonymousBoxWithSameTypeAs(
      const LayoutObject*) const {
    NOT_DESTROYED();
    NOTREACHED();
  }

  // Get the LayoutBox for the actual content. That's usually `this`, but if the
  // element creates multiple boxes (e.g. fieldsets and their anonymous content
  // child box), it may return something else. The box returned will be the one
  // that's created according to display type, scrollable overflow, and so on.
  virtual LayoutBox* ContentLayoutBox() {
    NOT_DESTROYED();
    return this;
  }

  ShapeOutsideInfo* GetShapeOutsideInfo() const;

  // CustomLayoutChild only exists if this LayoutBox is a IsCustomItem (aka. a
  // child of a LayoutCustom). This is created/destroyed when this LayoutBox is
  // inserted/removed from the layout tree.
  CustomLayoutChild* GetCustomLayoutChild() const;
  void AddCustomLayoutChildIfNeeded();
  void ClearCustomLayoutChild();

  bool HitTestClippedOutByBorder(
      const HitTestLocation&,
      const PhysicalOffset& border_box_location) const;

  bool HitTestOverflowControl(HitTestResult&,
                              const HitTestLocation&,
                              const PhysicalOffset&) const;

  // Returns true if the box intersects the viewport visible to the user.
  bool IntersectsVisibleViewport() const;

  void EnsureIsReadyForPaintInvalidation() override;
  void ClearPaintFlags() override;

  bool HasControlClip() const;

  class MutableForPainting : public LayoutObject::MutableForPainting {
   public:
    void SavePreviousSize() {
      GetLayoutBox().previous_size_ = GetLayoutBox().Size();
    }
    void ClearPreviousSize() { GetLayoutBox().previous_size_ = PhysicalSize(); }
    void SavePreviousOverflowData();
    void ClearPreviousOverflowData() {
      DCHECK(!GetLayoutBox().HasVisualOverflow());
      DCHECK(!GetLayoutBox().HasScrollableOverflow());
      GetLayoutBox().overflow_ = nullptr;
    }
    void SavePreviousContentBoxRect() {
      auto& rare_data = GetLayoutBox().EnsureRareData();
      rare_data.has_previous_content_box_rect_ = true;
      rare_data.previous_physical_content_box_rect_ =
          GetLayoutBox().PhysicalContentBoxRect();
    }
    void ClearPreviousContentBoxRect() {
      if (auto* rare_data = GetLayoutBox().rare_data_.Get())
        rare_data->has_previous_content_box_rect_ = false;
    }

    // Called from LayoutShiftTracker when we attach this LayoutBox to a node
    // for which we saved these values when the node was detached from its
    // original LayoutBox.
    void SetPreviousGeometryForLayoutShiftTracking(
        const PhysicalOffset& paint_offset,
        const PhysicalSize& size,
        const PhysicalRect& visual_overflow_rect);

    void UpdateBackgroundPaintLocation(bool needs_root_element_group);

   protected:
    friend class LayoutBox;
    MutableForPainting(const LayoutBox& box)
        : LayoutObject::MutableForPainting(box) {}
    LayoutBox& GetLayoutBox() {
      return static_cast<LayoutBox&>(layout_object_);
    }
  };

  MutableForPainting GetMutableForPainting() const {
    NOT_DESTROYED();
    return MutableForPainting(*this);
  }

  PhysicalSize PreviousSize() const {
    NOT_DESTROYED();
    return previous_size_;
  }
  PhysicalRect PreviousPhysicalContentBoxRect() const {
    NOT_DESTROYED();
    return rare_data_ && rare_data_->has_previous_content_box_rect_
               ? rare_data_->previous_physical_content_box_rect_
               : PhysicalRect(PhysicalOffset(), PreviousSize());
  }
  PhysicalRect PreviousVisualOverflowRect() const {
    NOT_DESTROYED();
    return overflow_ && overflow_->previous_overflow_data
               ? overflow_->previous_overflow_data
                     ->previous_visual_overflow_rect
               : PhysicalRect(PhysicalOffset(), PreviousSize());
  }
  PhysicalRect PreviousScrollableOverflowRect() const {
    NOT_DESTROYED();
    return overflow_ && overflow_->previous_overflow_data
               ? overflow_->previous_overflow_data
                     ->previous_scrollable_overflow_rect
               : PhysicalRect(PhysicalOffset(), PreviousSize());
  }
  PhysicalRect PreviousSelfVisualOverflowRect() const {
    NOT_DESTROYED();
    return overflow_ && overflow_->previous_overflow_data
               ? overflow_->previous_overflow_data
                     ->previous_self_visual_overflow_rect
               : PhysicalRect(PhysicalOffset(), PreviousSize());
  }

  // Returns the cached intrinsic logical widths when no children depend on the
  // block constraints.
  MinMaxSizesResult CachedIndefiniteIntrinsicLogicalWidths() const {
    NOT_DESTROYED();
    DCHECK(!IntrinsicLogicalWidthsDirty());
    return {intrinsic_logical_widths_,
            IntrinsicLogicalWidthsDependsOnBlockConstraints()};
  }

  // Returns the cached intrinsic logical widths if the initial block-size
  // matches.
  std::optional<MinMaxSizesResult> CachedIntrinsicLogicalWidths(
      LayoutUnit initial_block_size) const {
    NOT_DESTROYED();
    DCHECK(!IntrinsicLogicalWidthsDirty());
    if (initial_block_size == kIndefiniteSize) {
      if (IndefiniteIntrinsicLogicalWidthsDirty()) {
        return std::nullopt;
      }
      return MinMaxSizesResult(
          intrinsic_logical_widths_,
          IntrinsicLogicalWidthsDependsOnBlockConstraints());
    }
    if (min_max_sizes_cache_) {
      if (DefiniteIntrinsicLogicalWidthsDirty()) {
        return std::nullopt;
      }
      return min_max_sizes_cache_->Find(initial_block_size);
    }
    return std::nullopt;
  }

  // Sets the min/max sizes for this box.
  void SetIntrinsicLogicalWidths(LayoutUnit initial_block_size,
                                 const MinMaxSizesResult& result) {
    NOT_DESTROYED();
    // Write to the "indefinite" cache slot if:
    //  - If the initial block-size is indefinite.
    //  - If we don't have any children which depend on the initial block-size
    //    (it can change and we wouldn't give a different answer).
    if (initial_block_size == kIndefiniteSize ||
        !result.depends_on_block_constraints) {
      intrinsic_logical_widths_ = result.sizes;
      SetIntrinsicLogicalWidthsDependsOnBlockConstraints(
          result.depends_on_block_constraints);
      SetIndefiniteIntrinsicLogicalWidthsDirty(false);
    } else {
      if (!min_max_sizes_cache_) {
        min_max_sizes_cache_ = MakeGarbageCollected<MinMaxSizesCache>();
      } else if (DefiniteIntrinsicLogicalWidthsDirty()) {
        min_max_sizes_cache_->Clear();
      }
      min_max_sizes_cache_->Add(result.sizes, initial_block_size,
                                result.depends_on_block_constraints);
      SetDefiniteIntrinsicLogicalWidthsDirty(false);
    }
    ClearIntrinsicLogicalWidthsDirty();
  }

  // Make it public.
  using LayoutObject::BackgroundIsKnownToBeObscured;

  // Sets the coordinates of find-in-page scrollbar tickmarks, bypassing
  // DocumentMarkerController.  This is used by the PDF plugin.
  void OverrideTickmarks(Vector<gfx::Rect> tickmarks);

  // Issues a paint invalidation on the layout viewport's vertical scrollbar
  // (which is responsible for painting the tickmarks).
  void InvalidatePaintForTickmarks();

  bool MayHaveFragmentItems() const {
    NOT_DESTROYED();
    // When the tree is not clean, `ChildrenInline()` is not reliable.
    return (ChildrenInline() || NeedsLayout()) &&
           PhysicalFragments().MayHaveFragmentItems();
  }
  bool HasFragmentItems() const {
    NOT_DESTROYED();
    // See `MayHaveFragmentItems()`.
    return (ChildrenInline() || NeedsLayout()) &&
           PhysicalFragments().HasFragmentItems();
  }
#if EXPENSIVE_DCHECKS_ARE_ON()
  void CheckMayHaveFragmentItems() const;
#endif

  // Returns true if this box is fixed position and will not move with
  // scrolling. If the caller can pre-calculate |container_for_fixed_position|,
  // it should pass it to avoid recalculation.
  bool IsFixedToView(
      const LayoutObject* container_for_fixed_position = nullptr) const;

  // See StickyPositionScrollingConstraints::constraining_rect.
  PhysicalRect ComputeStickyConstrainingRect() const;

  AnchorPositionScrollData* GetAnchorPositionScrollData() const;
  bool NeedsAnchorPositionScrollAdjustment() const;
  PhysicalOffset AnchorPositionScrollTranslationOffset() const;

  bool AnchorPositionScrollAdjustmentAfectedByViewportScrolling() const;

  bool HasScrollbarGutters(ScrollbarOrientation orientation) const;

  // This should be called when the border-box size of this box is changed.
  void SizeChanged();

  // Finds the target anchor element for the given name in the containing block.
  // https://drafts.csswg.org/css-anchor-position-1/#target-anchor-element
  const LayoutObject* FindTargetAnchor(const ScopedCSSName&) const;

  // Returns this element's implicit anchor element if there is one and it is an
  // acceptable anchor element.
  // https://drafts.csswg.org/css-anchor-position-1/#ref-for-valdef-anchor-implicit
  const LayoutObject* AcceptableImplicitAnchor() const;

  const HeapVector<NonOverflowingScrollRange>* NonOverflowingScrollRanges()
      const;

  const BoxStrut& OutOfFlowInsetsForGetComputedStyle() const;

  Element* AccessibilityAnchor() const;
  const GCedHeapHashSet<Member<Element>>* DisplayLocksAffectedByAnchors() const;
  void NotifyContainingDisplayLocksForAnchorPositioning(
      const GCedHeapHashSet<Member<Element>>*
          past_display_locks_affected_by_anchors,
      const GCedHeapHashSet<Member<Element>>* display_locks_affected_by_anchors)
      const;
  bool NeedsAnchorPositionScrollAdjustmentInX() const;
  bool NeedsAnchorPositionScrollAdjustmentInY() const;

  using LayoutObject::GetBackgroundPaintLocation;

 protected:
  ~LayoutBox() override;

  virtual OverflowClipAxes ComputeOverflowClipAxes() const;

  void WillBeDestroyed() override;

  void InsertedIntoTree() override;
  void WillBeRemovedFromTree() override;

  void StyleWillChange(StyleDifference,
                       const ComputedStyle& new_style) override;
  void StyleDidChange(StyleDifference, const ComputedStyle* old_style) override;
  void UpdateFromStyle() override;

  void InLayoutNGInlineFormattingContextWillChange(bool) final;

  virtual ItemPosition SelfAlignmentNormalBehavior(
      const LayoutBox* child = nullptr) const {
    NOT_DESTROYED();
    DCHECK(!child);
    return ItemPosition::kStretch;
  }

  PhysicalRect BackgroundPaintedExtent() const;
  virtual bool ForegroundIsKnownToBeOpaqueInRect(
      const PhysicalRect& local_rect,
      unsigned max_depth_to_test) const;
  virtual bool ComputeBackgroundIsKnownToBeObscured() const;
  bool ComputeCanCompositeBackgroundAttachmentFixed() const override;

  virtual bool HitTestChildren(HitTestResult&,
                               const HitTestLocation&,
                               const PhysicalOffset& accumulated_offset,
                               HitTestPhase);

  void InvalidatePaint(const PaintInvalidatorContext&) const override;

  void ExcludeScrollbars(
      PhysicalRect&,
      OverlayScrollbarClipBehavior = kIgnoreOverlayScrollbarSize,
      ShouldIncludeScrollbarGutter = kIncludeScrollbarGutter) const;

  LayoutUnit ContainingBlockLogicalHeightForPositioned(
      const LayoutBoxModelObject* containing_block) const;

  virtual DeprecatedLayoutPoint LocationInternal() const {
    NOT_DESTROYED();
    return frame_location_;
  }
  // Allow LayoutMultiColumnSpannerPlaceholder to call LocationInternal() of
  // other instances.
  friend class LayoutMultiColumnSpannerPlaceholder;

  PhysicalOffset OffsetFromContainerInternal(
      const LayoutObject*,
      MapCoordinatesFlags mode) const override;

  // For atomic inlines, returns its resolved direction in text flow. Not to be
  // confused with the CSS property 'direction'.
  // Returns the CSS 'direction' property value when it is not atomic inline.
  TextDirection ResolvedDirection() const;

  // RecalcScrollableOverflow implementations for LayoutNG.
  RecalcScrollableOverflowResult RecalcScrollableOverflowNG();
  RecalcScrollableOverflowResult RecalcChildScrollableOverflowNG();

 private:
  inline bool ScrollableOverflowIsSet() const {
    NOT_DESTROYED();
    return overflow_ && overflow_->scrollable_overflow;
  }
#if DCHECK_IS_ON()
  void CheckIsVisualOverflowComputed() const;
#else
  ALWAYS_INLINE void CheckIsVisualOverflowComputed() const { NOT_DESTROYED(); }
#endif
  inline bool VisualOverflowIsSet() const {
    NOT_DESTROYED();
    CheckIsVisualOverflowComputed();
    return overflow_ && overflow_->visual_overflow;
  }

  // The outsets from this box's border-box that the element's content should be
  // clipped to, including overflow-clip-margin.
  PhysicalBoxStrut BorderOutsetsForClipping() const;

  void SetVisualOverflow(const PhysicalRect& self,
                         const PhysicalRect& contents);
  void CopyVisualOverflowFromFragmentsWithoutInvalidations();

  void UpdateShapeOutsideInfoAfterStyleChange(const ComputedStyle&,
                                              const ComputedStyle* old_style);
  void UpdateGridPositionAfterStyleChange(const ComputedStyle*);
  void UpdateScrollSnapMappingAfterStyleChange(const ComputedStyle& old_style);

  LayoutBoxRareData& EnsureRareData() {
    NOT_DESTROYED();
    if (!rare_data_)
      rare_data_ = MakeGarbageCollected<LayoutBoxRareData>();
    return *rare_data_.Get();
  }

  bool IsBox() const final {
    NOT_DESTROYED();
    return true;
  }

  void LocationChanged();

  void InflateVisualRectForFilter(TransformState&) const;
  void InflateVisualRectForFilterUnderContainer(
      TransformState&,
      const LayoutObject& container,
      const LayoutBoxModelObject* ancestor_to_stop_at) const;

  PhysicalRect DebugRect() const override;

  RasterEffectOutset VisualRectOutsetForRasterEffects() const override;

  inline bool CanSkipComputeScrollbars() const {
    NOT_DESTROYED();
    return (StyleRef().IsOverflowVisibleAlongBothAxes() ||
            !HasNonVisibleOverflow() ||
            (GetScrollableArea() &&
             !GetScrollableArea()->HasHorizontalScrollbar() &&
             !GetScrollableArea()->HasVerticalScrollbar())) &&
           StyleRef().IsScrollbarGutterAuto();
  }

  PhysicalBoxStrut ComputeScrollbarsInternal(
      ShouldClampToContentBox = kDoNotClampToContentBox,
      OverlayScrollbarClipBehavior = kIgnoreOverlayScrollbarSize,
      ShouldIncludeScrollbarGutter = kIncludeScrollbarGutter) const;

  PhysicalOffset PhysicalLocationInternal(
      const LayoutBox* container_box) const {
    NOT_DESTROYED();
    DCHECK_EQ(container_box, LocationContainer());
    DeprecatedLayoutPoint location = LocationInternal();
    if (!container_box || !container_box->HasFlippedBlocksWritingMode())
        [[likely]] {
      return PhysicalOffset(location);
    }

    return PhysicalOffset(
        container_box->Size().width - Size().width - location.X(),
        location.Y());
  }

  bool BackgroundClipBorderBoxIsEquivalentToPaddingBox() const;
  BackgroundPaintLocation ComputeBackgroundPaintLocation(
      bool needs_root_element_group) const;

  // Compute the border-box size from physical fragments.
  PhysicalSize ComputeSize() const;
  void InvalidateCachedGeometry();

  // Clear LayoutObject fields of physical fragments.
  void DisassociatePhysicalFragments();

 protected:
  // The CSS border box rect for this box.
  //
  // The rectangle is in LocationContainer's physical coordinates in flipped
  // block-flow direction of LocationContainer (see the COORDINATE SYSTEMS
  // section in LayoutBoxModelObject). The location is the distance from this
  // object's border edge to the LocationContainer's border edge. Thus it
  // includes any logical top/left along with this box's margins. It doesn't
  // include transforms, relative position offsets etc.
  DeprecatedLayoutPoint frame_location_;

  // TODO(crbug.com/1353190): Remove frame_size_.
  PhysicalSize frame_size_;

 private:
  // Previous value of frame_size_, updated after paint invalidation.
  PhysicalSize previous_size_;

 protected:
  MinMaxSizes intrinsic_logical_widths_;
  Member<MinMaxSizesCache> min_max_sizes_cache_;

  Member<MeasureCache> measure_cache_;
  LayoutResultList layout_results_;

  friend class LayoutBoxTest;

 private:
  // The index of the first fragment item associated with this object in
  // |FragmentItems::Items()|. Zero means there are no such item.
  // Valid only when IsInLayoutNGInlineFormattingContext().
  wtf_size_t first_fragment_item_index_ = 0u;

  Member<BoxOverflowModel> overflow_;
  Member<LayoutBoxRareData> rare_data_;

  FRIEND_TEST_ALL_PREFIXES(LayoutMultiColumnSetTest, ScrollAnchroingCrash);
};

template <>
struct DowncastTraits<LayoutBox> {
  static bool AllowFrom(const LayoutObject& object) { return object.IsBox(); }
};

inline LayoutBox* LayoutBox::PreviousSiblingBox() const {
  NOT_DESTROYED();
  return To<LayoutBox>(PreviousSibling());
}

inline LayoutBox* LayoutBox::NextSiblingBox() const {
  NOT_DESTROYED();
  return To<LayoutBox>(NextSibling());
}

inline LayoutBox* LayoutBox::ParentBox() const {
  NOT_DESTROYED();
  return To<LayoutBox>(Parent());
}

inline LayoutBox* LayoutBox::FirstChildBox() const {
  NOT_DESTROYED();
  return To<LayoutBox>(SlowFirstChild());
}

inline LayoutBox* LayoutBox::LastChildBox() const {
  NOT_DESTROYED();
  return To<LayoutBox>(SlowLastChild());
}

inline LayoutBox* LayoutBox::PreviousSiblingMultiColumnBox() const {
  NOT_DESTROYED();
  DCHECK(IsLayoutMultiColumnSpannerPlaceholder() || IsLayoutMultiColumnSet());
  LayoutBox* previous_box = PreviousSiblingBox();
  if (previous_box->IsLayoutFlowThread())
    return nullptr;
  return previous_box;
}

inline LayoutBox* LayoutBox::NextSiblingMultiColumnBox() const {
  NOT_DESTROYED();
  DCHECK(IsLayoutMultiColumnSpannerPlaceholder() || IsLayoutMultiColumnSet());
  return NextSiblingBox();
}

inline wtf_size_t LayoutBox::FirstInlineFragmentItemIndex() const {
  NOT_DESTROYED();
  if (!IsInLayoutNGInlineFormattingContext())
    return 0u;
  return first_fragment_item_index_;
}

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_LAYOUT_BOX_H_
