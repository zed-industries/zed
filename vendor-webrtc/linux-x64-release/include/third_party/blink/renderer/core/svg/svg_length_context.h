/*
 * Copyright (C) Research In Motion Limited 2011. All rights reserved.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_SVG_SVG_LENGTH_CONTEXT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_SVG_SVG_LENGTH_CONTEXT_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/css/css_primitive_value.h"
#include "third_party/blink/renderer/core/css/css_to_length_conversion_data.h"

namespace blink {

class CSSMathFunctionValue;
class ComputedStyle;
class Element;
class LayoutObject;
class SVGElement;

enum class SVGLengthMode;

class CORE_EXPORT SVGLengthContext {
  STACK_ALLOCATED();

 public:
  explicit SVGLengthContext(const SVGElement*);

  float ConvertValueToUserUnits(float,
                                SVGLengthMode,
                                CSSPrimitiveValue::UnitType from_unit) const;
  float ConvertValueFromUserUnits(float,
                                  SVGLengthMode,
                                  CSSPrimitiveValue::UnitType to_unit) const;
  float ResolveValue(const CSSMathFunctionValue&, SVGLengthMode) const;

  static const ComputedStyle* ComputedStyleForLengthResolving(
      const SVGElement&);

 private:
  double ConvertValueToUserUnitsUnclamped(
      float value,
      SVGLengthMode mode,
      CSSPrimitiveValue::UnitType from_unit) const;

  const SVGElement* context_;
};

class SVGLengthConversionData : public CSSToLengthConversionData {
  STACK_ALLOCATED();

 public:
  SVGLengthConversionData(const Element& context, const ComputedStyle& style);
  explicit SVGLengthConversionData(const LayoutObject& object);

 private:
  CSSToLengthConversionData::Flags ignored_flags_ = 0;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_SVG_SVG_LENGTH_CONTEXT_H_
