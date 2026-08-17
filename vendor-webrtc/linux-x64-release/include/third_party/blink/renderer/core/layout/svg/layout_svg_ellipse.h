/*
 * Copyright (C) 2012 Google, Inc.
 * All rights reserved.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_SVG_LAYOUT_SVG_ELLIPSE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_SVG_LAYOUT_SVG_ELLIPSE_H_

#include "third_party/blink/renderer/core/layout/svg/layout_svg_shape.h"
#include "ui/gfx/geometry/point_f.h"

namespace blink {

class LayoutSVGEllipse final : public LayoutSVGShape {
 public:
  explicit LayoutSVGEllipse(SVGGeometryElement*);
  ~LayoutSVGEllipse() override;

  const char* GetName() const override {
    NOT_DESTROYED();
    return "LayoutSVGEllipse";
  }

 private:
  void StyleDidChange(StyleDifference, const ComputedStyle* old_style) override;

  gfx::RectF UpdateShapeFromElement() override;
  bool ShapeDependentStrokeContains(const HitTestLocation&) override;
  bool ShapeDependentFillContains(const HitTestLocation&,
                                  const WindRule) const override;
  bool CalculateGeometryDependsOnViewport() const;
  void CalculateRadiiAndCenter();
  bool CanUseStrokeHitTestFastPath() const;
  bool HasContinuousStroke() const;

 private:
  gfx::PointF center_;
  float radius_x_ = 0;
  float radius_y_ = 0;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_SVG_LAYOUT_SVG_ELLIPSE_H_
