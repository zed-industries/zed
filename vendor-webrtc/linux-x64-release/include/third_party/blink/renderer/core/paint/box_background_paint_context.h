// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_BOX_BACKGROUND_PAINT_CONTEXT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_BOX_BACKGROUND_PAINT_CONTEXT_H_

#include "third_party/blink/renderer/core/layout/geometry/box_strut.h"
#include "third_party/blink/renderer/core/layout/geometry/physical_rect.h"
#include "third_party/blink/renderer/core/paint/paint_phase.h"
#include "third_party/blink/renderer/core/style/computed_style_constants.h"
#include "third_party/blink/renderer/core/style/fill_layer.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

class ComputedStyle;
class FillLayer;
class ImageResourceObserver;
class LayoutBox;
class LayoutBoxModelObject;
class LayoutTableCell;
class LayoutView;
class PhysicalBoxFragment;
struct PaintInfo;

struct SnappedAndUnsnappedOutsets {
  PhysicalBoxStrut snapped;
  PhysicalBoxStrut unsnapped;
};

// This class contains/describes the state needed to resolve a layer of a
// 'background-image' or 'mask-image'.
class BoxBackgroundPaintContext {
  STACK_ALLOCATED();

 public:
  // Constructor for LayoutView where the coordinate space is different.
  BoxBackgroundPaintContext(
      const LayoutView&,
      const PhysicalBoxFragment*,
      const PhysicalOffset& element_positioning_area_offset);

  // Generic constructor for all other elements.
  explicit BoxBackgroundPaintContext(const LayoutBoxModelObject&);

  // Constructor for TablesNG table parts.
  BoxBackgroundPaintContext(const LayoutTableCell& cell,
                            PhysicalOffset cell_offset,
                            const LayoutBox& table_part,
                            PhysicalSize table_part_size);

  explicit BoxBackgroundPaintContext(const PhysicalBoxFragment&);

  // Compute the initial position area based on the geometry for the object
  // this BackgroundPaintContext was created for.
  PhysicalRect NormalPositioningArea(const PhysicalRect& paint_rect) const;
  // As above, but also considers background-attachment: fixed.
  PhysicalRect ComputePositioningArea(const PaintInfo& paint_info,
                                      const FillLayer& fill_layer,
                                      const PhysicalRect& paint_rect) const;
  // The positioning area for a background layer with background-attachment:
  // fixed.
  PhysicalRect FixedAttachmentPositioningArea(const PaintInfo&) const;

  EFillBox EffectiveClip(const FillLayer& fill_layer) const {
    if (painting_view_) {
      // The root background should cover everything and therefore the
      // background-clip property has no effect.
      return EFillBox::kBorder;
    }
    return fill_layer.Clip();
  }

  PhysicalBoxStrut BorderOutsets() const;
  PhysicalBoxStrut PaddingOutsets() const;
  PhysicalBoxStrut VisualOverflowOutsets() const;

  PhysicalBoxStrut InnerBorderOutsets(
      const PhysicalRect& dest_rect,
      const PhysicalRect& positioning_area) const;
  SnappedAndUnsnappedOutsets ObscuredBorderOutsets(
      const PhysicalRect& dest_rect,
      const PhysicalRect& positioning_area) const;

  // The offset of the background image within the background positioning area.
  PhysicalOffset OffsetInBackground(const FillLayer&) const;

  bool DisallowBorderDerivedAdjustment() const;
  bool CanCompositeBackgroundAttachmentFixed() const;
  bool ShouldUseFixedAttachment(const FillLayer&) const;

  // Whether the background needs to be positioned relative to a container
  // element. Only used for tables.
  bool CellUsingContainerBackground() const {
    return cell_using_container_background_;
  }

  const ComputedStyle& Style() const;

  const ImageResourceObserver& ImageClient() const;
  const ComputedStyle& ImageStyle(const ComputedStyle& fragment_style) const;

  bool ShouldSkipBackgroundIfWhite() const;

  static bool HasBackgroundFixedToViewport(const LayoutBoxModelObject&);

 private:
  BoxBackgroundPaintContext(const LayoutBoxModelObject* box,
                            const LayoutBoxModelObject* positioning_box);

  // In most cases this is the same as positioning_box_. They are different
  // when we are painting:
  // 1. the view background (box_ is the LayoutView, and positioning_box_ is
  //    the LayoutView's RootBox()), or
  // 2. a table cell using its row/column's background (box_ is the table
  //    cell, and positioning_box_ is the row/column).
  // When they are different:
  // - ImageClient() uses box_ if painting view, otherwise positioning_box_;
  // - ImageStyle() uses positioning_box_;
  // - FillLayers come from box_ if painting view, otherwise positioning_box_.
  const LayoutBoxModelObject* box_;

  // The positioning box is the source of geometric information for positioning
  // and sizing the background. It also provides the information listed in the
  // comment for box_.
  const LayoutBoxModelObject* positioning_box_;

  const PhysicalBoxFragment* box_fragment_ = nullptr;

  // When painting table cells or the view, the positioning area
  // differs from the requested paint rect.
  PhysicalSize positioning_size_override_;

  // The background image offset from within the background positioning area
  // for non-fixed background attachment. Used for table cells and the view,
  // and also when an element is block-fragmented.
  PhysicalOffset element_positioning_area_offset_;

  bool has_background_fixed_to_viewport_ = false;
  bool painting_view_ = false;
  bool painting_table_cell_ = false;
  bool cell_using_container_background_ = false;
  bool box_has_multiple_fragments_ = false;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_BOX_BACKGROUND_PAINT_CONTEXT_H_
