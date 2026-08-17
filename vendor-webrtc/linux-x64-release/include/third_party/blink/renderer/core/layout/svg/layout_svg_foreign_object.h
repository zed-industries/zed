// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_SVG_LAYOUT_SVG_FOREIGN_OBJECT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_SVG_LAYOUT_SVG_FOREIGN_OBJECT_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/layout/svg/layout_svg_block.h"

namespace blink {

// LayoutSVGForeignObject is the LayoutObject associated with <foreignobject>.
// http://www.w3.org/TR/SVG/extend.html#ForeignObjectElement
//
// Foreign object is a way of inserting arbitrary non-SVG content into SVG.
// A good example of this is HTML in SVG. Because of this, CSS content has to
// be aware of SVG: e.g. when determining containing blocks we stop at the
// enclosing foreign object (see LayoutObject::ComputeIsFixedContainer).
//
// Note that SVG is also allowed in HTML with the HTML5 parsing rules so SVG
// content also has to be aware of CSS objects.
// See http://www.w3.org/TR/html5/syntax.html#elements-0 with the rules for
// 'foreign elements'. TODO(jchaffraix): Find a better place for this paragraph.
//
// The coordinate space for the descendants of the foreignObject does not
// include the effective zoom (it is baked into any lengths as usual). The
// transform that defines the userspace of the element is:
//
//   [CSS transform] * [inverse effective zoom] (* ['x' and 'y' translation])
//
// Because of this, the frame rect and visual rect includes effective zoom. The
// object bounding box (ObjectBoundingBox method) is however not zoomed to be
// compatible with the expectations of the getBBox() DOM interface.
class LayoutSVGForeignObject final : public LayoutSVGBlock {
 public:
  explicit LayoutSVGForeignObject(Element* element);

  bool IsObjectBoundingBoxValid() const;

  // A method to call when recursively hit testing from an SVG parent.
  // Since LayoutSVGRoot has a PaintLayer always, this will cause a
  // trampoline through PaintLayer::HitTest and back to a call to NodeAtPoint
  // on this object. This is why there are two methods.
  bool NodeAtPointFromSVG(HitTestResult& result,
                          const HitTestLocation& hit_test_location,
                          const PhysicalOffset& accumulated_offset,
                          HitTestPhase phase);

 private:
  // LayoutObject override:
  SVGLayoutResult UpdateSVGLayout(const SVGLayoutInfo&) override;
  // Update LayoutObject state after layout has completed. Returns true if
  // boundaries needs to be propagated (because of a change to the transform).
  bool UpdateAfterSVGLayout(const SVGLayoutInfo&, bool bounds_changed);

  const char* GetName() const override;
  bool IsSVGForeignObject() const final {
    NOT_DESTROYED();
    return true;
  }
  bool IsChildAllowed(LayoutObject* child,
                      const ComputedStyle& style) const override;
  gfx::RectF ObjectBoundingBox() const override;
  gfx::RectF StrokeBoundingBox() const override;
  gfx::RectF DecoratedBoundingBox() const override;
  gfx::RectF VisualRectInLocalSVGCoordinates() const override;
  AffineTransform LocalToSVGParentTransform() const override;

  // LayoutBox override:
  DeprecatedLayoutPoint LocationInternal() const override;
  PaintLayerType LayerTypeRequired() const override;
  bool CreatesNewFormattingContext() const override;

  // LayoutBlock override:
  void StyleDidChange(StyleDifference, const ComputedStyle* old_style) override;

  // The resolved viewport in the regular SVG coordinate space (after any
  // 'transform' has been applied but without zoom-adjustment).
  gfx::RectF viewport_;

  // Override of LayoutBox::frame_location_.
  // A physical fragment for <foreignObject> doesn't have the owner
  // PhysicalFragmentLink.
  DeprecatedLayoutPoint overridden_location_;
};

template <>
struct DowncastTraits<LayoutSVGForeignObject> {
  static bool AllowFrom(const LayoutObject& object) {
    return object.IsSVGForeignObject();
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_SVG_LAYOUT_SVG_FOREIGN_OBJECT_H_
