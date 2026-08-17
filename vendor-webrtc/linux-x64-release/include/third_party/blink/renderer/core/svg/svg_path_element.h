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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_SVG_SVG_PATH_ELEMENT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_SVG_SVG_PATH_ELEMENT_H_

#include "third_party/blink/renderer/core/svg/svg_geometry_element.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"

namespace blink {

class SVGAnimatedPath;
class SVGPathByteStream;
class StylePath;

class SVGPathElement final : public SVGGeometryElement {
  DEFINE_WRAPPERTYPEINFO();

 public:
  explicit SVGPathElement(Document&);

  Path AsPath() const override;
  PathBuilder AsMutablePath() const override;
  Path AttributePath() const;

  float getTotalLength(ExceptionState&) override;
  SVGPointTearOff* getPointAtLength(float distance, ExceptionState&) override;

  SVGAnimatedPath* GetPath() const { return path_.Get(); }

  float ComputePathLength() const override;
  const SVGPathByteStream& PathByteStream() const;

  gfx::RectF GetBBox() override;

  void Trace(Visitor*) const override;

 private:
  const StylePath* GetStylePath() const;

  void SvgAttributeChanged(const SvgAttributeChangedParams&) override;

  Node::InsertionNotificationRequest InsertedInto(ContainerNode&) override;
  void RemovedFrom(ContainerNode&) override;

  void InvalidateMPathDependencies();

  SVGAnimatedPropertyBase* PropertyFromAttribute(
      const QualifiedName& attribute_name) const override;
  void SynchronizeAllSVGAttributes() const override;
  void CollectExtraStyleForPresentationAttribute(
      HeapVector<CSSPropertyValue, 8>& style) override;

  Member<SVGAnimatedPath> path_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_SVG_SVG_PATH_ELEMENT_H_
