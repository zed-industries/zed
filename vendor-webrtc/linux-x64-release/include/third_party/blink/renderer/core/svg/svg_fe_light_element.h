/*
 * Copyright (C) 2004, 2005 Nikolas Zimmermann <zimmermann@kde.org>
 * Copyright (C) 2004, 2005, 2006 Rob Buis <buis@kde.org>
 * Copyright (C) 2005 Oliver Hunt <oliver@nerget.com>
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_SVG_SVG_FE_LIGHT_ELEMENT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_SVG_SVG_FE_LIGHT_ELEMENT_H_

#include <optional>

#include "third_party/blink/renderer/core/svg/svg_element.h"
#include "third_party/blink/renderer/core/svg_names.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"

namespace gfx {
class Point3F;
}

namespace blink {

class Filter;
class FELighting;
class LightSource;
class SVGAnimatedNumber;

class SVGFELightElement : public SVGElement {
 public:
  static SVGFELightElement* FindLightElement(const SVGElement&);

  virtual scoped_refptr<LightSource> GetLightSource(Filter*) const = 0;
  std::optional<bool> SetLightSourceAttribute(FELighting*,
                                              const QualifiedName&) const;

  SVGAnimatedNumber* azimuth() { return azimuth_.Get(); }
  const SVGAnimatedNumber* azimuth() const { return azimuth_.Get(); }
  SVGAnimatedNumber* elevation() { return elevation_.Get(); }
  const SVGAnimatedNumber* elevation() const { return elevation_.Get(); }
  SVGAnimatedNumber* x() { return x_.Get(); }
  const SVGAnimatedNumber* x() const { return x_.Get(); }
  SVGAnimatedNumber* y() { return y_.Get(); }
  const SVGAnimatedNumber* y() const { return y_.Get(); }
  SVGAnimatedNumber* z() { return z_.Get(); }
  const SVGAnimatedNumber* z() const { return z_.Get(); }
  SVGAnimatedNumber* pointsAtX() { return points_at_x_.Get(); }
  const SVGAnimatedNumber* pointsAtX() const { return points_at_x_.Get(); }
  SVGAnimatedNumber* pointsAtY() { return points_at_y_.Get(); }
  const SVGAnimatedNumber* pointsAtY() const { return points_at_y_.Get(); }
  SVGAnimatedNumber* pointsAtZ() { return points_at_z_.Get(); }
  const SVGAnimatedNumber* pointsAtZ() const { return points_at_z_.Get(); }
  SVGAnimatedNumber* specularExponent() { return specular_exponent_.Get(); }
  const SVGAnimatedNumber* specularExponent() const {
    return specular_exponent_.Get();
  }
  SVGAnimatedNumber* limitingConeAngle() { return limiting_cone_angle_.Get(); }
  const SVGAnimatedNumber* limitingConeAngle() const {
    return limiting_cone_angle_.Get();
  }

  void Trace(Visitor*) const override;

 protected:
  SVGFELightElement(const QualifiedName&, Document&);

  gfx::Point3F GetPosition() const;
  gfx::Point3F PointsAt() const;

 private:
  void SvgAttributeChanged(const SvgAttributeChangedParams&) final;
  void ChildrenChanged(const ChildrenChange&) final;

  bool LayoutObjectIsNeeded(const DisplayStyle&) const override {
    return false;
  }

  SVGAnimatedPropertyBase* PropertyFromAttribute(
      const QualifiedName& attribute_name) const override;
  void SynchronizeAllSVGAttributes() const override;

  Member<SVGAnimatedNumber> azimuth_;
  Member<SVGAnimatedNumber> elevation_;
  Member<SVGAnimatedNumber> x_;
  Member<SVGAnimatedNumber> y_;
  Member<SVGAnimatedNumber> z_;
  Member<SVGAnimatedNumber> points_at_x_;
  Member<SVGAnimatedNumber> points_at_y_;
  Member<SVGAnimatedNumber> points_at_z_;
  Member<SVGAnimatedNumber> specular_exponent_;
  Member<SVGAnimatedNumber> limiting_cone_angle_;
};

template <>
struct DowncastTraits<SVGFELightElement> {
  static bool AllowFrom(const Node& node) {
    auto* svg_element = DynamicTo<SVGElement>(node);
    return svg_element && AllowFrom(*svg_element);
  }
  static bool AllowFrom(const SVGElement& svg_element) {
    return svg_element.HasTagName(svg_names::kFEDistantLightTag) ||
           svg_element.HasTagName(svg_names::kFEPointLightTag) ||
           svg_element.HasTagName(svg_names::kFESpotLightTag);
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_SVG_SVG_FE_LIGHT_ELEMENT_H_
