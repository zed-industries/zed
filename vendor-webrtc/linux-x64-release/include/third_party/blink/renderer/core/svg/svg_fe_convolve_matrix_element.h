/*
 * Copyright (C) 2009 Dirk Schulze <krit@webkit.org>
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_SVG_SVG_FE_CONVOLVE_MATRIX_ELEMENT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_SVG_SVG_FE_CONVOLVE_MATRIX_ELEMENT_H_

#include "third_party/blink/renderer/core/svg/svg_animated_enumeration.h"
#include "third_party/blink/renderer/core/svg/svg_filter_primitive_standard_attributes.h"
#include "third_party/blink/renderer/platform/graphics/filters/fe_convolve_matrix.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"

namespace blink {

class SVGAnimatedBoolean;
class SVGAnimatedNumber;
class SVGAnimatedNumberList;
class SVGAnimatedNumberOptionalNumber;
class SVGAnimatedInteger;
class SVGAnimatedIntegerOptionalInteger;

DECLARE_SVG_ENUM_MAP(FEConvolveMatrix::EdgeModeType);

class SVGFEConvolveMatrixElement final
    : public SVGFilterPrimitiveStandardAttributes {
  DEFINE_WRAPPERTYPEINFO();

 public:
  explicit SVGFEConvolveMatrixElement(Document&);

  SVGAnimatedBoolean* preserveAlpha() { return preserve_alpha_.Get(); }
  SVGAnimatedNumber* divisor() { return divisor_.Get(); }
  SVGAnimatedNumber* bias() { return bias_.Get(); }
  SVGAnimatedNumber* kernelUnitLengthX();
  SVGAnimatedNumber* kernelUnitLengthY();
  SVGAnimatedNumberList* kernelMatrix() { return kernel_matrix_.Get(); }
  SVGAnimatedString* in1() { return in1_.Get(); }
  SVGAnimatedEnumeration<FEConvolveMatrix::EdgeModeType>* edgeMode() {
    return edge_mode_.Get();
  }
  SVGAnimatedInteger* orderX() const;
  SVGAnimatedInteger* orderY() const;
  SVGAnimatedInteger* targetX() { return target_x_.Get(); }
  SVGAnimatedInteger* targetY() { return target_y_.Get(); }

  void Trace(Visitor*) const override;

 private:
  gfx::Size MatrixOrder() const;
  gfx::Point TargetPoint() const;
  float ComputeDivisor() const;

  bool SetFilterEffectAttribute(FilterEffect*, const QualifiedName&) override;
  void SvgAttributeChanged(const SvgAttributeChangedParams&) override;
  FilterEffect* Build(SVGFilterBuilder*, Filter*) override;
  bool TaintsOrigin() const override { return false; }

  SVGAnimatedPropertyBase* PropertyFromAttribute(
      const QualifiedName& attribute_name) const override;
  void SynchronizeAllSVGAttributes() const override;

  Member<SVGAnimatedNumber> bias_;
  Member<SVGAnimatedNumber> divisor_;
  Member<SVGAnimatedString> in1_;
  Member<SVGAnimatedEnumeration<FEConvolveMatrix::EdgeModeType>> edge_mode_;
  Member<SVGAnimatedNumberList> kernel_matrix_;
  Member<SVGAnimatedNumberOptionalNumber> kernel_unit_length_;
  Member<SVGAnimatedIntegerOptionalInteger> order_;
  Member<SVGAnimatedBoolean> preserve_alpha_;
  Member<SVGAnimatedInteger> target_x_;
  Member<SVGAnimatedInteger> target_y_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_SVG_SVG_FE_CONVOLVE_MATRIX_ELEMENT_H_
