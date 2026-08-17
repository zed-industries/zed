/*
 * Copyright (c) 2009, Google Inc. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are
 * met:
 *
 *     * Redistributions of source code must retain the above copyright
 * notice, this list of conditions and the following disclaimer.
 *     * Redistributions in binary form must reproduce the above
 * copyright notice, this list of conditions and the following disclaimer
 * in the documentation and/or other materials provided with the
 * distribution.
 *     * Neither the name of Google Inc. nor the names of its
 * contributors may be used to endorse or promote products derived from
 * this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 * "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 * LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
 * A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
 * OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 * SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
 * LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
 * DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
 * THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_SVG_LAYOUT_SVG_MODEL_OBJECT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_SVG_LAYOUT_SVG_MODEL_OBJECT_H_

#include "third_party/blink/renderer/core/layout/layout_object.h"
#include "third_party/blink/renderer/core/svg/svg_element.h"

namespace blink {

// Most layoutObjects in the SVG layout tree will inherit from this class
// but not all. (e.g. LayoutSVGForeignObject, LayoutSVGBlock) thus methods
// required by SVG layoutObjects need to be declared on LayoutObject, but shared
// logic can go in this class or in SVGLayoutSupport.

class LayoutSVGModelObject : public LayoutObject {
 public:
  explicit LayoutSVGModelObject(SVGElement*);

  bool IsChildAllowed(LayoutObject*, const ComputedStyle&) const override;

  gfx::RectF VisualRectInLocalSVGCoordinates() const override {
    NOT_DESTROYED();
    return DecoratedBoundingBox();
  }

  void QuadsInAncestorInternal(Vector<gfx::QuadF>&,
                               const LayoutBoxModelObject* ancestor,
                               MapCoordinatesFlags) const override;
  gfx::RectF LocalBoundingBoxRectForAccessibility() const final;

  void MapLocalToAncestor(const LayoutBoxModelObject* ancestor,
                          TransformState&,
                          MapCoordinatesFlags) const final;
  void MapAncestorToLocal(const LayoutBoxModelObject* ancestor,
                          TransformState&,
                          MapCoordinatesFlags) const final;
  void StyleDidChange(StyleDifference, const ComputedStyle* old_style) override;

  SVGElement* GetElement() const {
    NOT_DESTROYED();
    return To<SVGElement>(LayoutObject::GetNode());
  }

  bool IsSVG() const final {
    NOT_DESTROYED();
    return true;
  }

 protected:
  void ImageChanged(WrappedImagePtr, CanDeferInvalidation) override;
  void WillBeDestroyed() override;

  void InsertedIntoTree() override;
  void WillBeRemovedFromTree() override;

  bool CheckForImplicitTransformChange(const SVGLayoutInfo&,
                                       bool bbox_changed) const;

 private:
  // LayoutSVGModelObject subclasses should use GetElement() instead.
  void GetNode() const = delete;

  void AddOutlineRects(OutlineRectCollector&,
                       OutlineInfo*,
                       const PhysicalOffset& additional_offset,
                       OutlineType) const final;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_SVG_LAYOUT_SVG_MODEL_OBJECT_H_
