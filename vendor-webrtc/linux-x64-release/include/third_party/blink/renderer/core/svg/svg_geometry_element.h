/*
 * Copyright (C) 2013 Samsung Electronics. All rights reserved.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_SVG_SVG_GEOMETRY_ELEMENT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_SVG_SVG_GEOMETRY_ELEMENT_H_

#include "third_party/blink/renderer/core/svg/svg_graphics_element.h"

namespace blink {

class AffineTransform;
class DOMPointInit;
class Path;
class PathBuilder;
class SVGAnimatedNumber;
class SVGPointTearOff;

class SVGGeometryElement : public SVGGraphicsElement {
  DEFINE_WRAPPERTYPEINFO();

 public:
  virtual Path AsPath() const = 0;
  virtual PathBuilder AsMutablePath() const = 0;

  bool isPointInFill(const DOMPointInit*) const;
  bool isPointInStroke(const DOMPointInit*) const;

  Path ToClipPath(const AffineTransform* clip_transform = nullptr) const;

  SVGAnimatedNumber* pathLength() const { return path_length_.Get(); }

  virtual float getTotalLength(ExceptionState&);
  virtual SVGPointTearOff* getPointAtLength(float distance, ExceptionState&);

  float AuthorPathLength() const;
  float PathLengthScaleFactor() const;
  static float PathLengthScaleFactor(float computed_path_length,
                                     float author_path_length);

  void Trace(Visitor*) const override;

 protected:
  SVGGeometryElement(const QualifiedName&,
                     Document&,
                     ConstructionType = kCreateSVGElement);

  void SvgAttributeChanged(const SvgAttributeChangedParams&) override;

  void GeometryAttributeChanged();
  void GeometryPresentationAttributeChanged(const SVGAnimatedPropertyBase&);

  SVGAnimatedPropertyBase* PropertyFromAttribute(
      const QualifiedName& attribute_name) const override;
  void SynchronizeAllSVGAttributes() const override;

 private:
  bool IsSVGGeometryElement() const final { return true; }
  virtual float ComputePathLength() const;
  LayoutObject* CreateLayoutObject(const ComputedStyle&) override;

  Member<SVGAnimatedNumber> path_length_;
};

template <>
struct DowncastTraits<SVGGeometryElement> {
  static bool AllowFrom(const Node& node) {
    auto* svg_element = DynamicTo<SVGElement>(node);
    return svg_element && AllowFrom(*svg_element);
  }
  static bool AllowFrom(const SVGElement& svg_element) {
    return svg_element.IsSVGGeometryElement();
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_SVG_SVG_GEOMETRY_ELEMENT_H_
