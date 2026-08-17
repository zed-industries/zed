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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_SVG_SVG_RADIAL_GRADIENT_ELEMENT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_SVG_SVG_RADIAL_GRADIENT_ELEMENT_H_

#include "third_party/blink/renderer/core/svg/svg_gradient_element.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"

namespace blink {

class SVGAnimatedLength;

struct RadialGradientAttributes;

class SVGRadialGradientElement final : public SVGGradientElement {
  DEFINE_WRAPPERTYPEINFO();

 public:
  explicit SVGRadialGradientElement(Document&);

  SVGAnimatedLength* cx() const { return cx_.Get(); }
  SVGAnimatedLength* cy() const { return cy_.Get(); }
  SVGAnimatedLength* r() const { return r_.Get(); }
  SVGAnimatedLength* fx() const { return fx_.Get(); }
  SVGAnimatedLength* fy() const { return fy_.Get(); }
  SVGAnimatedLength* fr() const { return fr_.Get(); }

  RadialGradientAttributes CollectGradientAttributes() const;

  void Trace(Visitor*) const override;

 private:
  void SvgAttributeChanged(const SvgAttributeChangedParams&) override;

  LayoutObject* CreateLayoutObject(const ComputedStyle&) override;

  bool SelfHasRelativeLengths() const override;

  SVGAnimatedPropertyBase* PropertyFromAttribute(
      const QualifiedName& attribute_name) const override;
  void SynchronizeAllSVGAttributes() const override;

  Member<SVGAnimatedLength> cx_;
  Member<SVGAnimatedLength> cy_;
  Member<SVGAnimatedLength> r_;
  Member<SVGAnimatedLength> fx_;
  Member<SVGAnimatedLength> fy_;
  Member<SVGAnimatedLength> fr_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_SVG_SVG_RADIAL_GRADIENT_ELEMENT_H_
