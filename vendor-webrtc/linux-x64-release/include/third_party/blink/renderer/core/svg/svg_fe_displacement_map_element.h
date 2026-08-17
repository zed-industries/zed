/*
 * Copyright (C) 2006 Oliver Hunt <oliver@nerget.com>
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_SVG_SVG_FE_DISPLACEMENT_MAP_ELEMENT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_SVG_SVG_FE_DISPLACEMENT_MAP_ELEMENT_H_

#include "third_party/blink/renderer/core/svg/svg_animated_enumeration.h"
#include "third_party/blink/renderer/core/svg/svg_filter_primitive_standard_attributes.h"
#include "third_party/blink/renderer/platform/graphics/filters/fe_displacement_map.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"

namespace blink {

class SVGAnimatedNumber;

DECLARE_SVG_ENUM_MAP(ChannelSelectorType);

class SVGFEDisplacementMapElement final
    : public SVGFilterPrimitiveStandardAttributes {
  DEFINE_WRAPPERTYPEINFO();

 public:
  explicit SVGFEDisplacementMapElement(Document&);

  static ChannelSelectorType StringToChannel(const String&);

  SVGAnimatedNumber* scale() { return scale_.Get(); }
  SVGAnimatedString* in1() { return in1_.Get(); }
  SVGAnimatedString* in2() { return in2_.Get(); }
  SVGAnimatedEnumeration<ChannelSelectorType>* xChannelSelector() {
    return x_channel_selector_.Get();
  }
  SVGAnimatedEnumeration<ChannelSelectorType>* yChannelSelector() {
    return y_channel_selector_.Get();
  }

  void Trace(Visitor*) const override;

 private:
  bool SetFilterEffectAttribute(FilterEffect*,
                                const QualifiedName& attr_name) override;
  void SvgAttributeChanged(const SvgAttributeChangedParams&) override;
  FilterEffect* Build(SVGFilterBuilder*, Filter*) override;
  bool TaintsOrigin() const override { return false; }

  SVGAnimatedPropertyBase* PropertyFromAttribute(
      const QualifiedName& attribute_name) const override;
  void SynchronizeAllSVGAttributes() const override;

  Member<SVGAnimatedNumber> scale_;
  Member<SVGAnimatedString> in1_;
  Member<SVGAnimatedString> in2_;
  Member<SVGAnimatedEnumeration<ChannelSelectorType>> x_channel_selector_;
  Member<SVGAnimatedEnumeration<ChannelSelectorType>> y_channel_selector_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_SVG_SVG_FE_DISPLACEMENT_MAP_ELEMENT_H_
