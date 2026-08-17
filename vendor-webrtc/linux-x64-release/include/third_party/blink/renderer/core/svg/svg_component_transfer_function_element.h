/*
 * Copyright (C) 2004, 2005, 2008 Nikolas Zimmermann <zimmermann@kde.org>
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_SVG_SVG_COMPONENT_TRANSFER_FUNCTION_ELEMENT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_SVG_SVG_COMPONENT_TRANSFER_FUNCTION_ELEMENT_H_

#include "third_party/blink/renderer/core/svg/svg_animated_enumeration.h"
#include "third_party/blink/renderer/core/svg/svg_element.h"
#include "third_party/blink/renderer/platform/graphics/filters/fe_component_transfer.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"

namespace blink {

class SVGAnimatedNumber;
class SVGAnimatedNumberList;

DECLARE_SVG_ENUM_MAP(ComponentTransferType);

class SVGComponentTransferFunctionElement : public SVGElement {
  DEFINE_WRAPPERTYPEINFO();

 public:
  ComponentTransferFunction TransferFunction() const;

  SVGAnimatedNumberList* tableValues() { return table_values_.Get(); }
  SVGAnimatedNumber* slope() { return slope_.Get(); }
  SVGAnimatedNumber* intercept() { return intercept_.Get(); }
  SVGAnimatedNumber* amplitude() { return amplitude_.Get(); }
  SVGAnimatedNumber* exponent() { return exponent_.Get(); }
  SVGAnimatedNumber* offset() { return offset_.Get(); }
  SVGAnimatedEnumeration<ComponentTransferType>* type() { return type_.Get(); }

  void Trace(Visitor*) const override;

 protected:
  SVGComponentTransferFunctionElement(const QualifiedName&, Document&);

  void SvgAttributeChanged(const SvgAttributeChangedParams&) final;

  bool LayoutObjectIsNeeded(const DisplayStyle& style) const final {
    return false;
  }

 private:
  SVGAnimatedPropertyBase* PropertyFromAttribute(
      const QualifiedName& attribute_name) const override;
  void SynchronizeAllSVGAttributes() const override;

  Member<SVGAnimatedNumberList> table_values_;
  Member<SVGAnimatedNumber> slope_;
  Member<SVGAnimatedNumber> intercept_;
  Member<SVGAnimatedNumber> amplitude_;
  Member<SVGAnimatedNumber> exponent_;
  Member<SVGAnimatedNumber> offset_;
  Member<SVGAnimatedEnumeration<ComponentTransferType>> type_;
};

template <>
struct DowncastTraits<SVGComponentTransferFunctionElement> {
  static bool AllowFrom(const SVGElement& svg_element) {
    return svg_element.HasTagName(svg_names::kFEFuncATag) ||
           svg_element.HasTagName(svg_names::kFEFuncBTag) ||
           svg_element.HasTagName(svg_names::kFEFuncGTag) ||
           svg_element.HasTagName(svg_names::kFEFuncRTag);
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_SVG_SVG_COMPONENT_TRANSFER_FUNCTION_ELEMENT_H_
