/*
 * Copyright (C) 2004, 2005, 2008 Nikolas Zimmermann <zimmermann@kde.org>
 * Copyright (C) 2004, 2005, 2006, 2007 Rob Buis <buis@kde.org>
 * Copyright (C) 2014 Google, Inc.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_SVG_SVG_GRAPHICS_ELEMENT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_SVG_SVG_GRAPHICS_ELEMENT_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/svg/svg_tests.h"
#include "third_party/blink/renderer/core/svg/svg_transformable_element.h"

namespace blink {

class SVGMatrixTearOff;
class SVGRectTearOff;

class CORE_EXPORT SVGGraphicsElement : public SVGTransformableElement,
                                       public SVGTests {
  DEFINE_WRAPPERTYPEINFO();

 public:
  ~SVGGraphicsElement() override;

  SVGMatrixTearOff* getCTM();
  SVGMatrixTearOff* getScreenCTM();

  SVGElement* nearestViewportElement() const;
  SVGElement* farthestViewportElement() const;

  virtual gfx::RectF GetBBox();
  SVGRectTearOff* getBBoxFromJavascript();

  bool IsValid() const final { return SVGTests::IsValid(); }

  AffineTransform ComputeCTM(
      CTMScope mode,
      const SVGGraphicsElement* ancestor = nullptr) const;

  void Trace(Visitor*) const override;

 protected:
  SVGGraphicsElement(const QualifiedName&,
                     Document&,
                     ConstructionType = kCreateSVGElement);

  FocusableState SupportsFocus(UpdateBehavior update_behavior) const override {
    if (HasFocusEventListeners()) {
      return FocusableState::kFocusable;
    }
    return Element::SupportsFocus(update_behavior);
  }

  void SvgAttributeChanged(const SvgAttributeChangedParams&) override;

  SVGAnimatedPropertyBase* PropertyFromAttribute(
      const QualifiedName& attribute_name) const override;
  void SynchronizeAllSVGAttributes() const override;

 private:
  bool IsSVGGraphicsElement() const final { return true; }
};

template <>
struct DowncastTraits<SVGGraphicsElement> {
  static bool AllowFrom(const Node& node) {
    auto* svg_element = DynamicTo<SVGElement>(node);
    return svg_element && AllowFrom(*svg_element);
  }
  static bool AllowFrom(const SVGElement& svg_element) {
    return svg_element.IsSVGGraphicsElement();
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_SVG_SVG_GRAPHICS_ELEMENT_H_
