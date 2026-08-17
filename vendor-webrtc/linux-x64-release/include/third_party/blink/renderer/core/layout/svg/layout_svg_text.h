// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_SVG_LAYOUT_SVG_TEXT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_SVG_LAYOUT_SVG_TEXT_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/layout/svg/layout_svg_block.h"

namespace blink {

// The LayoutNG representation of SVG <text>.
class LayoutSVGText final : public LayoutSVGBlock {
 public:
  explicit LayoutSVGText(Element* element);

  void SubtreeStructureChanged(LayoutInvalidationReasonForTracing);
  // This is called whenever a text layout attribute on the <text> or a
  // descendant <tspan> is changed.
  void SetNeedsPositioningValuesUpdate();
  void SetNeedsTextMetricsUpdate();
  bool NeedsTextMetricsUpdate() const;

  bool IsObjectBoundingBoxValid() const;

  // These two functions return a LayoutSVGText or nullptr.
  static LayoutSVGText* LocateLayoutSVGTextAncestor(LayoutObject*);
  static const LayoutSVGText* LocateLayoutSVGTextAncestor(const LayoutObject*);

  static void NotifySubtreeStructureChanged(LayoutObject*,
                                            LayoutInvalidationReasonForTracing);

 private:
  // LayoutObject override:
  SVGLayoutResult UpdateSVGLayout(const SVGLayoutInfo&) override;
  // Update LayoutObject state after layout has completed. Returns true if
  // boundaries needs to be propagated (because of a change to the transform).
  bool UpdateAfterSVGLayout(const SVGLayoutInfo&, bool bounds_changed);

  const char* GetName() const override;
  bool IsSVGText() const final {
    NOT_DESTROYED();
    return true;
  }
  bool IsChildAllowed(LayoutObject* child, const ComputedStyle&) const override;
  void AddChild(LayoutObject* child, LayoutObject* before_child) override;
  void RemoveChild(LayoutObject* child) override;
  void InsertedIntoTree() override;
  void WillBeRemovedFromTree() override;
  gfx::RectF ObjectBoundingBox() const override;
  gfx::RectF StrokeBoundingBox() const override;
  gfx::RectF DecoratedBoundingBox() const override;
  gfx::RectF VisualRectInLocalSVGCoordinates() const override;
  void QuadsInAncestorInternal(Vector<gfx::QuadF>&,
                               const LayoutBoxModelObject* ancestor,
                               MapCoordinatesFlags) const override;
  gfx::RectF LocalBoundingBoxRectForAccessibility() const override;
  void StyleDidChange(StyleDifference, const ComputedStyle* old_style) override;
  void WillBeDestroyed() override;
  bool NodeAtPoint(HitTestResult& result,
                   const HitTestLocation& hit_test_location,
                   const PhysicalOffset& accumulated_offset,
                   HitTestPhase phase) override;
  PositionWithAffinity PositionForPoint(
      const PhysicalOffset& point_in_contents) const override;

  // LayoutBox override:
  bool CreatesNewFormattingContext() const override;
  void UpdateFromStyle() override;

  // LayoutBlock override:
  void Paint(const PaintInfo&) const override;

  void UpdateFont();
  void UpdateTransformAffectsVectorEffect();
  void InvalidateDescendantObjectBoundingBoxes();

  // bounding_box_* are mutable for on-demand computation in a const method.
  mutable gfx::RectF bounding_box_;
  mutable bool needs_update_bounding_box_ : 1;

  bool needs_text_metrics_update_ : 1;
};

template <>
struct DowncastTraits<LayoutSVGText> {
  static bool AllowFrom(const LayoutObject& object) {
    return object.IsSVGText();
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_SVG_LAYOUT_SVG_TEXT_H_
