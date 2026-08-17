/*
 * Copyright (C) 2004, 2005, 2006, 2008 Nikolas Zimmermann <zimmermann@kde.org>
 * Copyright (C) 2004, 2005, 2006 Rob Buis <buis@kde.org>
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_SVG_SVG_LINEAR_GRADIENT_ELEMENT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_SVG_SVG_LINEAR_GRADIENT_ELEMENT_H_

#include "third_party/blink/renderer/core/svg/svg_gradient_element.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"

namespace blink {

class SVGAnimatedLength;

struct LinearGradientAttributes;

class SVGLinearGradientElement final : public SVGGradientElement {
  DEFINE_WRAPPERTYPEINFO();

 public:
  explicit SVGLinearGradientElement(Document&);

  SVGAnimatedLength* x1() const { return x1_.Get(); }
  SVGAnimatedLength* y1() const { return y1_.Get(); }
  SVGAnimatedLength* x2() const { return x2_.Get(); }
  SVGAnimatedLength* y2() const { return y2_.Get(); }

  LinearGradientAttributes CollectGradientAttributes() const;

  void Trace(Visitor*) const override;

 private:
  void SvgAttributeChanged(const SvgAttributeChangedParams&) override;

  LayoutObject* CreateLayoutObject(const ComputedStyle&) override;

  bool SelfHasRelativeLengths() const override;

  SVGAnimatedPropertyBase* PropertyFromAttribute(
      const QualifiedName& attribute_name) const override;
  void SynchronizeAllSVGAttributes() const override;

  Member<SVGAnimatedLength> x1_;
  Member<SVGAnimatedLength> y1_;
  Member<SVGAnimatedLength> x2_;
  Member<SVGAnimatedLength> y2_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_SVG_SVG_LINEAR_GRADIENT_ELEMENT_H_
