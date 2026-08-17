/*
 * Copyright (C) 2006 Alexander Kellett <lypanov@kde.org>
 * Copyright (C) 2006, 2009 Apple Inc. All rights reserved.
 * Copyright (C) 2007 Rob Buis <buis@kde.org>
 * Copyright (C) 2009 Google, Inc.
 * Copyright (C) 2010 Patrick Gansterer <paroga@paroga.com>
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
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_SVG_LAYOUT_SVG_IMAGE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_SVG_LAYOUT_SVG_IMAGE_H_

#include "third_party/blink/renderer/core/layout/svg/layout_svg_model_object.h"
#include "ui/gfx/geometry/rect_f.h"

namespace blink {

class LayoutImageResource;
class SVGImageElement;

class LayoutSVGImage final : public LayoutSVGModelObject {
 public:
  explicit LayoutSVGImage(SVGImageElement*);
  ~LayoutSVGImage() override;
  void Trace(Visitor*) const override;

  void SetNeedsTransformUpdate() override {
    NOT_DESTROYED();
    needs_transform_update_ = true;
  }

  LayoutImageResource* ImageResource() {
    NOT_DESTROYED();
    return image_resource_.Get();
  }
  const LayoutImageResource* ImageResource() const {
    NOT_DESTROYED();
    return image_resource_.Get();
  }

  gfx::RectF ObjectBoundingBox() const override {
    NOT_DESTROYED();
    return object_bounding_box_;
  }
  bool IsObjectBoundingBoxValid() const {
    NOT_DESTROYED();
    return !object_bounding_box_.IsEmpty();
  }

  bool IsSVGImage() const final {
    NOT_DESTROYED();
    return true;
  }

  AffineTransform LocalSVGTransform() const override {
    NOT_DESTROYED();
    return local_transform_;
  }

  const char* GetName() const override {
    NOT_DESTROYED();
    return "LayoutSVGImage";
  }

 protected:
  void StyleDidChange(StyleDifference, const ComputedStyle* old_style) override;
  void WillBeDestroyed() override;

 private:
  gfx::RectF StrokeBoundingBox() const override {
    NOT_DESTROYED();
    return object_bounding_box_;
  }

  gfx::RectF DecoratedBoundingBox() const override {
    NOT_DESTROYED();
    return object_bounding_box_;
  }

  void ImageChanged(WrappedImagePtr, CanDeferInvalidation) override;

  SVGLayoutResult UpdateSVGLayout(const SVGLayoutInfo&) override;
  void Paint(const PaintInfo&) const override;

  bool UpdateBoundingBox();
  // Update LayoutObject state after layout has completed. Returns true if
  // boundaries needs to be propagated (because of a change to the transform).
  bool UpdateAfterSVGLayout(const SVGLayoutInfo&, bool bbox_changed);

  bool NodeAtPoint(HitTestResult&,
                   const HitTestLocation&,
                   const PhysicalOffset& accumulated_offset,
                   HitTestPhase) override;

  gfx::SizeF CalculateObjectSize() const;

  bool needs_transform_update_ : 1;
  bool transform_uses_reference_box_ : 1;
  AffineTransform local_transform_;
  gfx::RectF object_bounding_box_;
  Member<LayoutImageResource> image_resource_;
};

template <>
struct DowncastTraits<LayoutSVGImage> {
  static bool AllowFrom(const LayoutObject& object) {
    return object.IsSVGImage();
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_SVG_LAYOUT_SVG_IMAGE_H_
