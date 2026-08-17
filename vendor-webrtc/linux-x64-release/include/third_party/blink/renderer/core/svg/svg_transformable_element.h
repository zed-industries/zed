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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_SVG_SVG_TRANSFORMABLE_ELEMENT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_SVG_SVG_TRANSFORMABLE_ELEMENT_H_

#include "third_party/blink/renderer/core/svg/svg_element.h"

namespace blink {

class AffineTransform;
class SVGAnimatedTransformList;

// Intermediate class for SVGElements that have a 'transform' IDL
// attribute. Not exposed in IDL.
class SVGTransformableElement : public SVGElement {
 public:
  ~SVGTransformableElement() override;

  AffineTransform LocalCoordinateSpaceTransform(CTMScope) const override;
  AffineTransform* AnimateMotionTransform() override;

  SVGAnimatedTransformList* transform() { return transform_.Get(); }
  const SVGAnimatedTransformList* transform() const { return transform_.Get(); }

  void Trace(Visitor*) const override;

 protected:
  SVGTransformableElement(const QualifiedName&,
                          Document&,
                          ConstructionType = kCreateSVGElement);

  void CollectExtraStyleForPresentationAttribute(
      HeapVector<CSSPropertyValue, 8>& style) override;
  void SvgAttributeChanged(const SvgAttributeChangedParams&) override;

  SVGAnimatedPropertyBase* PropertyFromAttribute(
      const QualifiedName& attribute_name) const override;
  void SynchronizeAllSVGAttributes() const override;

  Member<SVGAnimatedTransformList> transform_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_SVG_SVG_TRANSFORMABLE_ELEMENT_H_
