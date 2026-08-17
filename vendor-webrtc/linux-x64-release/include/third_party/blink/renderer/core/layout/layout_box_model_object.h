/*
 * Copyright (C) 1999 Lars Knoll (knoll@kde.org)
 *           (C) 1999 Antti Koivisto (koivisto@kde.org)
 * Copyright (C) 2003, 2006, 2007, 2009 Apple Inc. All rights reserved.
 * Copyright (C) 2010 Google Inc. All rights reserved.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_LAYOUT_BOX_MODEL_OBJECT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_LAYOUT_BOX_MODEL_OBJECT_H_

#include "base/notreached.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/layout/background_bleed_avoidance.h"
#include "third_party/blink/renderer/core/layout/content_change_type.h"
#include "third_party/blink/renderer/core/layout/layout_object.h"
#include "third_party/blink/renderer/platform/text/writing_mode_utils.h"

namespace blink {

class PaintLayer;
class PaintLayerScrollableArea;
struct LogicalRect;

enum PaintLayerType {
  kNoPaintLayer,
  kNormalPaintLayer,
  // A forced or overflow clip layer is required for bookkeeping purposes,
  // but does not force a layer to be self painting.
  kOverflowClipPaintLayer,
  kForcedPaintLayer
};

// This class is the base class for all CSS objects.
//
// All CSS objects follow the box model object. See THE BOX MODEL section in
// LayoutBox for more information.
//
// This class actually doesn't have the box model but it exposes some common
// functions or concepts that sub-classes can extend upon. For example, there
// are accessors for margins, borders, paddings and borderBoundingBox().
//
// The reason for this partial implementation is that the 2 classes inheriting
// from it (LayoutBox and LayoutInline) have different requirements but need to
// have a PaintLayer. For a full implementation of the box model, see LayoutBox.
//
// An important member of this class is PaintLayer, which is stored in a rare-
// data pattern (see: Layer()). PaintLayers are instantiated for several reasons
// based on the return value of layerTypeRequired().
// Interestingly, most SVG objects inherit from LayoutSVGModelObject and thus
// can't have a PaintLayer. This is an unfortunate artifact of our
// design as it limits code sharing and prevents hardware accelerating SVG
// (the current design require a PaintLayer for compositing).
//
//
// ***** COORDINATE SYSTEMS *****
//
// In order to fully understand LayoutBoxModelObject and the inherited classes,
// we need to introduce the concept of coordinate systems.
// There are 4 coordinate systems:
// - physical coordinates: it is the coordinate system used for painting and
//   correspond to physical direction as seen on the physical display (screen,
//   printed page). In CSS, 'top', 'right', 'bottom', 'left' are all in physical
//   coordinates. The code matches this convention too.
//
// - logical coordinates: this is the coordinate system used for layout. It is
//   determined by 'writing-mode' and 'direction'. Any property using 'before',
//   'after', 'start' or 'end' is in logical coordinates. Those are also named
//   respectively 'logical top', 'logical bottom', 'logical left' and
//   'logical right'.
//
// Example with writing-mode: vertical-rl; direction: ltr;
//
//                    'top' / 'start' side
//
//                     block-flow direction
//           <------------------------------------ |
//           ------------------------------------- |
//           |        c   |          s           | |
// 'left'    |        o   |          o           | |   inline     'right'
//    /      |        n   |          m           | |  direction      /
// 'after'   |        t   |          e           | |              'before'
//  side     |        e   |                      | |                side
//           |        n   |                      | |
//           |        t   |                      | |
//           ------------------------------------- v
//
//                 'bottom' / 'end' side
//
// See https://drafts.csswg.org/css-writing-modes-3/#text-flow for some
// extra details.
//
// - logical coordinates without flipping inline direction: those are "logical
//   block coordinates", without considering text direction. Examples are
//   "LogicalLeft" and "LogicalRight".
//
// For more information, see the following doc about coordinate spaces:
// https://chromium.googlesource.com/chromium/src.git/+/main/third_party/blink/renderer/core/layout/README.md#coordinate-spaces
class CORE_EXPORT LayoutBoxModelObject : public LayoutObject {
 public:
  LayoutBoxModelObject(ContainerNode*);
  ~LayoutBoxModelObject() override;

  // This is the only way layers should ever be destroyed.
  void DestroyLayer();

  // Computes the sticky constraints for this object.
  StickyPositionScrollingConstraints* ComputeStickyPositionConstraints() const;

  PhysicalOffset StickyPositionOffset() const;
  virtual LayoutBlock* StickyContainer() const;

  StickyPositionScrollingConstraints* StickyConstraints() const {
    NOT_DESTROYED();
    return FirstFragment().StickyConstraints();
  }
  void SetStickyConstraints(StickyPositionScrollingConstraints* constraints) {
    NOT_DESTROYED();
    GetMutableForPainting().FirstFragment().SetStickyConstraints(constraints);
    SetNeedsPaintPropertyUpdate();
  }

  // IE extensions. Used to calculate offsetWidth/Height. Overridden by inlines
  // (LayoutInline) to return the remaining width on a given line (and the
  // height of a single line).
  virtual LayoutUnit OffsetLeft(const Element*) const;
  virtual LayoutUnit OffsetTop(const Element*) const;
  virtual LayoutUnit OffsetWidth() const = 0;
  virtual LayoutUnit OffsetHeight() const = 0;

  bool HasSelfPaintingLayer() const;
  PaintLayer* Layer() const {
    NOT_DESTROYED();
    return FirstFragment().Layer();
  }
  // The type of PaintLayer to instantiate. Any value returned from this
  // function other than NoPaintLayer will lead to a PaintLayer being created.
  virtual PaintLayerType LayerTypeRequired() const = 0;
  PaintLayerScrollableArea* GetScrollableArea() const;

  virtual void UpdateFromStyle();

  virtual PhysicalRect VisualOverflowRect() const = 0;

  // Returns the visual overflow rect, expanded to the area affected by any
  // filters that paint outside of the box, in physical coordinates.
  PhysicalRect VisualOverflowRectIncludingFilters() const;

  // Returns a physical rect that is a result of apply this object's filters to
  // it. If there are no filters, it returns its argument.
  PhysicalRect ApplyFiltersToRect(const PhysicalRect&) const;

  // These return the CSS computed padding values.
  LayoutUnit ComputedCSSPaddingTop() const {
    NOT_DESTROYED();
    return ComputedCSSPadding(StyleRef().PaddingTop());
  }
  LayoutUnit ComputedCSSPaddingBottom() const {
    NOT_DESTROYED();
    return ComputedCSSPadding(StyleRef().PaddingBottom());
  }
  LayoutUnit ComputedCSSPaddingLeft() const {
    NOT_DESTROYED();
    return ComputedCSSPadding(StyleRef().PaddingLeft());
  }
  LayoutUnit ComputedCSSPaddingRight() const {
    NOT_DESTROYED();
    return ComputedCSSPadding(StyleRef().PaddingRight());
  }

  // These functions are used during layout.
  // - Table override them to exclude padding with collapsing borders.
  virtual LayoutUnit PaddingTop() const {
    NOT_DESTROYED();
    return ComputedCSSPaddingTop();
  }
  virtual LayoutUnit PaddingBottom() const {
    NOT_DESTROYED();
    return ComputedCSSPaddingBottom();
  }
  virtual LayoutUnit PaddingLeft() const {
    NOT_DESTROYED();
    return ComputedCSSPaddingLeft();
  }
  virtual LayoutUnit PaddingRight() const {
    NOT_DESTROYED();
    return ComputedCSSPaddingRight();
  }

  // Returns a WritingDirectionMode-aware logical padding value.
  LayoutUnit PaddingBlockStart() const {
    NOT_DESTROYED();
    return PhysicalPaddingToLogical().BlockStart();
  }
  LayoutUnit PaddingBlockEnd() const {
    NOT_DESTROYED();
    return PhysicalPaddingToLogical().BlockEnd();
  }
  LayoutUnit PaddingInlineEnd() const {
    NOT_DESTROYED();
    return PhysicalPaddingToLogical().InlineEnd();
  }

  virtual LayoutUnit BorderTop() const {
    NOT_DESTROYED();
    return LayoutUnit(StyleRef().BorderTopWidth());
  }
  virtual LayoutUnit BorderBottom() const {
    NOT_DESTROYED();
    return LayoutUnit(StyleRef().BorderBottomWidth());
  }
  virtual LayoutUnit BorderLeft() const {
    NOT_DESTROYED();
    return LayoutUnit(StyleRef().BorderLeftWidth());
  }
  virtual LayoutUnit BorderRight() const {
    NOT_DESTROYED();
    return LayoutUnit(StyleRef().BorderRightWidth());
  }

  // Returns a WritingDirectionMode-aware logical border value.
  LayoutUnit BorderBlockStart() const {
    NOT_DESTROYED();
    return PhysicalBorderToLogical().BlockStart();
  }
  LayoutUnit BorderBlockEnd() const {
    NOT_DESTROYED();
    return PhysicalBorderToLogical().BlockEnd();
  }
  LayoutUnit BorderInlineStart() const {
    NOT_DESTROYED();
    return PhysicalBorderToLogical().InlineStart();
  }
  LayoutUnit BorderInlineEnd() const {
    NOT_DESTROYED();
    return PhysicalBorderToLogical().InlineEnd();
  }

  LayoutUnit BorderWidth() const {
    NOT_DESTROYED();
    return BorderLeft() + BorderRight();
  }
  LayoutUnit BorderHeight() const {
    NOT_DESTROYED();
    return BorderTop() + BorderBottom();
  }

  PhysicalBoxStrut BorderOutsets() const {
    NOT_DESTROYED();
    return {BorderTop(), BorderRight(), BorderBottom(), BorderLeft()};
  }

  PhysicalBoxStrut PaddingOutsets() const {
    NOT_DESTROYED();
    return {PaddingTop(), PaddingRight(), PaddingBottom(), PaddingLeft()};
  }

  // Returns a WritingDirectionMode-aware logical border+padding value.
  DISABLE_CFI_PERF LayoutUnit BorderAndPaddingBlockStart() const {
    NOT_DESTROYED();
    return BorderBlockStart() + PaddingBlockStart();
  }
  DISABLE_CFI_PERF LayoutUnit BorderAndPaddingBlockEnd() const {
    NOT_DESTROYED();
    return BorderBlockEnd() + PaddingBlockEnd();
  }

  DISABLE_CFI_PERF LayoutUnit BorderAndPaddingHeight() const {
    NOT_DESTROYED();
    return BorderTop() + BorderBottom() + PaddingTop() + PaddingBottom();
  }
  DISABLE_CFI_PERF LayoutUnit BorderAndPaddingWidth() const {
    NOT_DESTROYED();
    return BorderLeft() + BorderRight() + PaddingLeft() + PaddingRight();
  }
  DISABLE_CFI_PERF LayoutUnit BorderAndPaddingBlockSize() const {
    NOT_DESTROYED();
    if (!StyleRef().HasBorder() && !StyleRef().MayHavePadding()) {
      return LayoutUnit();
    }
    return IsHorizontalWritingMode() ? BorderAndPaddingHeight()
                                     : BorderAndPaddingWidth();
  }
  DISABLE_CFI_PERF LayoutUnit BorderAndPaddingInlineSize() const {
    NOT_DESTROYED();
    if (!StyleRef().HasBorder() && !StyleRef().MayHavePadding()) {
      return LayoutUnit();
    }
    return IsHorizontalWritingMode() ? BorderAndPaddingWidth()
                                     : BorderAndPaddingHeight();
  }
  DISABLE_CFI_PERF LayoutUnit BorderAndPaddingInlineStart() const {
    NOT_DESTROYED();
    return BorderInlineStart() + PhysicalPaddingToLogical().InlineStart();
  }
  DISABLE_CFI_PERF LayoutUnit BorderAndPaddingInlineEnd() const {
    NOT_DESTROYED();
    return BorderInlineEnd() + PaddingInlineEnd();
  }

  LayoutUnit PaddingLogicalHeight() const {
    NOT_DESTROYED();
    const auto logical_padding = PhysicalPaddingToLogical();
    return logical_padding.BlockStart() + logical_padding.BlockEnd();
  }

  virtual LayoutUnit MarginTop() const = 0;
  virtual LayoutUnit MarginBottom() const = 0;
  virtual LayoutUnit MarginLeft() const = 0;
  virtual LayoutUnit MarginRight() const = 0;

  // Returns a WritingDirectionMode-aware logical margin value.
  LayoutUnit MarginBlockStart(
      const ComputedStyle* other_style = nullptr) const {
    NOT_DESTROYED();
    return PhysicalMarginToLogical(other_style).BlockStart();
  }
  LayoutUnit MarginBlockEnd(const ComputedStyle* other_style = nullptr) const {
    NOT_DESTROYED();
    return PhysicalMarginToLogical(other_style).BlockEnd();
  }
  LayoutUnit MarginInlineStart(
      const ComputedStyle* other_style = nullptr) const {
    NOT_DESTROYED();
    return PhysicalMarginToLogical(other_style).InlineStart();
  }
  LayoutUnit MarginInlineEnd(const ComputedStyle* other_style = nullptr) const {
    NOT_DESTROYED();
    return PhysicalMarginToLogical(other_style).InlineEnd();
  }

  DISABLE_CFI_PERF LayoutUnit MarginHeight() const {
    NOT_DESTROYED();
    return MarginTop() + MarginBottom();
  }
  DISABLE_CFI_PERF LayoutUnit MarginWidth() const {
    NOT_DESTROYED();
    return MarginLeft() + MarginRight();
  }
  DISABLE_CFI_PERF LayoutUnit MarginLogicalHeight() const {
    NOT_DESTROYED();
    const auto logical_margin = PhysicalMarginToLogical(nullptr);
    return logical_margin.BlockStart() + logical_margin.BlockEnd();
  }
  DISABLE_CFI_PERF LayoutUnit MarginLogicalWidth() const {
    NOT_DESTROYED();
    const auto logical_margin = PhysicalMarginToLogical(nullptr);
    return logical_margin.InlineStart() + logical_margin.InlineEnd();
  }

  PhysicalBoxStrut MarginOutsets() const {
    NOT_DESTROYED();
    return {MarginTop(), MarginRight(), MarginBottom(), MarginLeft()};
  }

  virtual LayoutUnit ContainingBlockLogicalWidthForContent() const;

  virtual void ChildBecameNonInline(LayoutObject* /*child*/) {
    NOT_DESTROYED();
  }

  // Overridden by subclasses to determine line-height of the first-line.
  virtual LayoutUnit FirstLineHeight() const = 0;

  // Returns true if the background is painted opaque in the given rect.
  // The query rect is given in local coordinate system.
  virtual bool BackgroundIsKnownToBeOpaqueInRect(const PhysicalRect&) const {
    NOT_DESTROYED();
    return false;
  }

  // This object's background is transferred to its LayoutView if:
  // 1. it's the document element, or
  // 2. it's the first <body> if the document element is <html> and doesn't have
  //    a background. http://www.w3.org/TR/css3-background/#body-background
  // If it's the case, the used background should be the initial value (i.e.
  // transparent). The first condition is actually an implementation detail
  // because we paint the view background in ViewPainter instead of the painter
  // of the layout object of the document element.
  bool BackgroundTransfersToView(
      const ComputedStyle* document_element_style = nullptr) const;

  void RecalcVisualOverflow() override;

  void AddOutlineRectsForNormalChildren(OutlineRectCollector&,
                                        const PhysicalOffset& additional_offset,
                                        OutlineType) const;

  void UpdateCanCompositeBackgroundAttachmentFixed(
      bool enable_composited_background_attachment_fixed);

 protected:
  void WillBeDestroyed() override;

  PhysicalOffset OffsetFromContainerInternal(
      const LayoutObject*,
      MapCoordinatesFlags) const override;

  PhysicalOffset AdjustedPositionRelativeTo(const PhysicalOffset&,
                                            const Element*) const;

  LogicalRect LocalCaretRectForEmptyElement(
      LayoutUnit width,
      LayoutUnit text_indent_offset) const;

  void AddOutlineRectsForDescendant(const LayoutObject& descendant,
                                    OutlineRectCollector&,
                                    const PhysicalOffset& additional_offset,
                                    OutlineType) const;

  bool ShouldBeHandledAsInline(const ComputedStyle& style) const;
  void StyleWillChange(StyleDifference,
                       const ComputedStyle& new_style) override;
  void StyleDidChange(StyleDifference, const ComputedStyle* old_style) override;
  virtual bool ComputeCanCompositeBackgroundAttachmentFixed() const {
    NOT_DESTROYED();
    return false;
  }

 public:
  // These functions are only used internally to manipulate the layout tree
  // structure via remove/insert/appendChildNode.
  // Since they are typically called only to move objects around within
  // anonymous blocks (which only have layers in the case of column spans), the
  // default for fullRemoveInsert is false rather than true.
  void MoveChildTo(LayoutBoxModelObject* to_box_model_object,
                   LayoutObject* child,
                   LayoutObject* before_child,
                   bool full_remove_insert = false);
  void MoveChildTo(LayoutBoxModelObject* to_box_model_object,
                   LayoutObject* child,
                   bool full_remove_insert = false) {
    NOT_DESTROYED();
    MoveChildTo(to_box_model_object, child, nullptr, full_remove_insert);
  }
  void MoveAllChildrenTo(LayoutBoxModelObject* to_box_model_object,
                         bool full_remove_insert = false) {
    NOT_DESTROYED();
    MoveAllChildrenTo(to_box_model_object, nullptr, full_remove_insert);
  }
  void MoveAllChildrenTo(LayoutBoxModelObject* to_box_model_object,
                         LayoutObject* before_child,
                         bool full_remove_insert = false) {
    NOT_DESTROYED();
    MoveChildrenTo(to_box_model_object, SlowFirstChild(), nullptr, before_child,
                   full_remove_insert);
  }
  // Move all of the kids from |startChild| up to but excluding |endChild|. 0
  // can be passed as the |endChild| to denote that all the kids from
  // |startChild| onwards should be moved.
  void MoveChildrenTo(LayoutBoxModelObject* to_box_model_object,
                      LayoutObject* start_child,
                      LayoutObject* end_child,
                      bool full_remove_insert = false) {
    NOT_DESTROYED();
    MoveChildrenTo(to_box_model_object, start_child, end_child, nullptr,
                   full_remove_insert);
  }
  virtual void MoveChildrenTo(LayoutBoxModelObject* to_box_model_object,
                              LayoutObject* start_child,
                              LayoutObject* end_child,
                              LayoutObject* before_child,
                              bool full_remove_insert = false);

  LayoutObject* SplitAnonymousBoxesAroundChild(LayoutObject* before_child);
  virtual LayoutBox* CreateAnonymousBoxToSplit(
      const LayoutBox* box_to_split) const;

 private:
  void CreateLayerAfterStyleChange();

  LayoutUnit ComputedCSSPadding(const Length&) const;
  bool IsBoxModelObject() const final {
    NOT_DESTROYED();
    return true;
  }

  PhysicalToLogicalGetter<LayoutUnit, LayoutBoxModelObject>
  PhysicalPaddingToLogical() const {
    NOT_DESTROYED();
    return PhysicalToLogicalGetter<LayoutUnit, LayoutBoxModelObject>(
        StyleRef().GetWritingDirection(), *this,
        &LayoutBoxModelObject::PaddingTop, &LayoutBoxModelObject::PaddingRight,
        &LayoutBoxModelObject::PaddingBottom,
        &LayoutBoxModelObject::PaddingLeft);
  }

  PhysicalToLogicalGetter<LayoutUnit, LayoutBoxModelObject>
  PhysicalMarginToLogical(const ComputedStyle* other_style) const {
    NOT_DESTROYED();
    const auto& style = other_style ? *other_style : StyleRef();
    return PhysicalToLogicalGetter<LayoutUnit, LayoutBoxModelObject>(
        style.GetWritingDirection(), *this, &LayoutBoxModelObject::MarginTop,
        &LayoutBoxModelObject::MarginRight, &LayoutBoxModelObject::MarginBottom,
        &LayoutBoxModelObject::MarginLeft);
  }

  PhysicalToLogicalGetter<LayoutUnit, LayoutBoxModelObject>
  PhysicalBorderToLogical() const {
    NOT_DESTROYED();
    return PhysicalToLogicalGetter<LayoutUnit, LayoutBoxModelObject>(
        StyleRef().GetWritingDirection(), *this,
        &LayoutBoxModelObject::BorderTop, &LayoutBoxModelObject::BorderRight,
        &LayoutBoxModelObject::BorderBottom, &LayoutBoxModelObject::BorderLeft);
  }
};

template <>
struct DowncastTraits<LayoutBoxModelObject> {
  static bool AllowFrom(const LayoutObject& object) {
    return object.IsBoxModelObject();
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_LAYOUT_BOX_MODEL_OBJECT_H_
