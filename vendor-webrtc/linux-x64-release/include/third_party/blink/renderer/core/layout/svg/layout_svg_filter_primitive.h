/*
 * Copyright (C) 2010 University of Szeged
 * Copyright (C) 2010 Zoltan Herczeg
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY UNIVERSITY OF SZEGED ``AS IS'' AND ANY
 * EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR
 * PURPOSE ARE DISCLAIMED.  IN NO EVENT SHALL UNIVERSITY OF SZEGED OR
 * CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
 * EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
 * PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
 * PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY
 * OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_SVG_LAYOUT_SVG_FILTER_PRIMITIVE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_SVG_LAYOUT_SVG_FILTER_PRIMITIVE_H_

#include "third_party/blink/renderer/core/layout/layout_object.h"

namespace blink {

class SVGFilterPrimitiveStandardAttributes;

class LayoutSVGFilterPrimitive final : public LayoutObject {
 public:
  explicit LayoutSVGFilterPrimitive(SVGFilterPrimitiveStandardAttributes*);

 private:
  bool IsChildAllowed(LayoutObject*, const ComputedStyle&) const override {
    NOT_DESTROYED();
    return false;
  }

  void WillBeDestroyed() override;
  void StyleDidChange(StyleDifference, const ComputedStyle*) override;
  SVGLayoutResult UpdateSVGLayout(const SVGLayoutInfo&) override;

  const char* GetName() const override {
    NOT_DESTROYED();
    return "LayoutSVGFilterPrimitive";
  }
  bool IsSVG() const final {
    NOT_DESTROYED();
    return true;
  }
  bool IsSVGFilterPrimitive() const final {
    NOT_DESTROYED();
    return true;
  }
  gfx::RectF ObjectBoundingBox() const override {
    NOT_DESTROYED();
    return gfx::RectF();
  }
  gfx::RectF StrokeBoundingBox() const override {
    NOT_DESTROYED();
    return gfx::RectF();
  }
  gfx::RectF VisualRectInLocalSVGCoordinates() const override {
    NOT_DESTROYED();
    return gfx::RectF();
  }
  gfx::RectF LocalBoundingBoxRectForAccessibility() const override {
    NOT_DESTROYED();
    return gfx::RectF();
  }
  gfx::RectF DecoratedBoundingBox() const override {
    NOT_DESTROYED();
    return gfx::RectF();
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_SVG_LAYOUT_SVG_FILTER_PRIMITIVE_H_
