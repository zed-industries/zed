/*
 * Copyright (C) 2004, 2005, 2006, 2007 Nikolas Zimmermann <zimmermann@kde.org>
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_SVG_SVG_FILTER_PRIMITIVE_STANDARD_ATTRIBUTES_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_SVG_SVG_FILTER_PRIMITIVE_STANDARD_ATTRIBUTES_H_

#include "third_party/blink/renderer/core/svg/svg_element.h"
#include "third_party/blink/renderer/core/svg/svg_unit_types.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"

namespace gfx {
class RectF;
}

namespace blink {

class Filter;
class FilterEffect;
class SVGAnimatedLength;
class SVGAnimatedString;
class SVGFilterBuilder;

class SVGFilterPrimitiveStandardAttributes : public SVGElement {
  // No DEFINE_WRAPPERTYPEINFO() here because a) this class is never
  // instantiated, and b) we don't generate corresponding V8T.h or V8T.cpp.
  // The subclasses must write DEFINE_WRAPPERTYPEINFO().
 public:
  void SetStandardAttributes(FilterEffect*,
                             SVGUnitTypes::SVGUnitType,
                             const gfx::RectF& reference_box) const;

  virtual FilterEffect* Build(SVGFilterBuilder*, Filter*) = 0;
  // Returns true, if the new value is different from the old one.
  virtual bool SetFilterEffectAttribute(FilterEffect*, const QualifiedName&);

  virtual bool TaintsOrigin() const { return true; }

  // JS API
  SVGAnimatedLength* x() const { return x_.Get(); }
  SVGAnimatedLength* y() const { return y_.Get(); }
  SVGAnimatedLength* width() const { return width_.Get(); }
  SVGAnimatedLength* height() const { return height_.Get(); }
  SVGAnimatedString* result() const { return result_.Get(); }

  void Trace(Visitor*) const override;

  void PrimitiveAttributeChanged(const QualifiedName&);
  void Invalidate();

 protected:
  SVGFilterPrimitiveStandardAttributes(const QualifiedName&, Document&);

  void SvgAttributeChanged(const SvgAttributeChangedParams&) override;
  void ChildrenChanged(const ChildrenChange&) override;

  SVGAnimatedPropertyBase* PropertyFromAttribute(
      const QualifiedName& attribute_name) const override;
  void SynchronizeAllSVGAttributes() const override;

 private:
  bool IsFilterEffect() const final { return true; }

  LayoutObject* CreateLayoutObject(const ComputedStyle&) override;
  bool LayoutObjectIsNeeded(const DisplayStyle&) const final;

  Member<SVGAnimatedLength> x_;
  Member<SVGAnimatedLength> y_;
  Member<SVGAnimatedLength> width_;
  Member<SVGAnimatedLength> height_;
  Member<SVGAnimatedString> result_;
};

void InvalidateFilterPrimitiveParent(SVGElement&);

template <>
struct DowncastTraits<SVGFilterPrimitiveStandardAttributes> {
  static bool AllowFrom(const Node& node) {
    auto* element = DynamicTo<SVGElement>(node);
    return element && element->IsFilterEffect();
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_SVG_SVG_FILTER_PRIMITIVE_STANDARD_ATTRIBUTES_H_
