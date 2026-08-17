/*
 * Copyright (C) 2004, 2005, 2006, 2008 Nikolas Zimmermann <zimmermann@kde.org>
 * Copyright (C) 2004, 2005, 2006, 2007 Rob Buis <buis@kde.org>
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_SVG_SVG_POLY_ELEMENT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_SVG_SVG_POLY_ELEMENT_H_

#include "third_party/blink/renderer/core/svg/svg_geometry_element.h"
#include "third_party/blink/renderer/core/svg_names.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"

namespace blink {

class PathBuilder;
class SVGAnimatedPointList;
class SVGPointListTearOff;

class SVGPolyElement : public SVGGeometryElement {
 public:
  SVGAnimatedPointList* Points() const { return points_.Get(); }

  SVGPointListTearOff* pointsFromJavascript();
  SVGPointListTearOff* animatedPoints();

  void Trace(Visitor*) const override;

 protected:
  SVGPolyElement(const QualifiedName&, Document&);

  PathBuilder AsPathFromPoints() const;

 private:
  void SvgAttributeChanged(const SvgAttributeChangedParams&) final;

  SVGAnimatedPropertyBase* PropertyFromAttribute(
      const QualifiedName& attribute_name) const override;
  void SynchronizeAllSVGAttributes() const override;

  Member<SVGAnimatedPointList> points_;
};

template <>
struct DowncastTraits<SVGPolyElement> {
  static bool AllowFrom(const Node& node) {
    auto* svg_element = DynamicTo<SVGElement>(node);
    return svg_element && AllowFrom(*svg_element);
  }
  static bool AllowFrom(const SVGElement& svg_element) {
    return svg_element.HasTagName(svg_names::kPolygonTag) ||
           svg_element.HasTagName(svg_names::kPolylineTag);
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_SVG_SVG_POLY_ELEMENT_H_
