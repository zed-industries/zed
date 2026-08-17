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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_SVG_SVG_LENGTH_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_SVG_SVG_LENGTH_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/css/css_numeric_literal_value.h"
#include "third_party/blink/renderer/core/css/css_primitive_value.h"
#include "third_party/blink/renderer/core/svg/properties/svg_listable_property.h"
#include "third_party/blink/renderer/core/svg/svg_length_functions.h"
#include "third_party/blink/renderer/core/svg/svg_parsing_error.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/wtf/casting.h"

namespace blink {

class Length;
class QualifiedName;
class SVGLengthContext;
class SVGLengthConversionData;
class SVGLengthTearOff;

class CORE_EXPORT SVGLength final : public SVGListablePropertyBase {
 public:
  typedef SVGLengthTearOff TearOffType;

  // Initial values for SVGLength properties. If adding a new initial value,
  // keep the list sorted within the same unit. The table containing the actual
  // values are in the .cc file.
  enum class Initial {
    kUnitlessZero,
    kPercentMinus10,
    kPercent0,
    kPercent50,
    kPercent100,
    kPercent120,
    kNumber3,
    kNumValues
  };
  static constexpr int kInitialValueBits = 3;

  explicit SVGLength(SVGLengthMode = SVGLengthMode::kOther);
  SVGLength(Initial, SVGLengthMode);
  SVGLength(const CSSPrimitiveValue&, SVGLengthMode);

  void SetInitial(unsigned);

  void Trace(Visitor*) const override;

  SVGLength* Clone() const;

  CSSPrimitiveValue::UnitType NumericLiteralType() const {
    DCHECK(value_->IsNumericLiteralValue());
    return To<CSSNumericLiteralValue>(*value_).GetType();
  }

  SVGLengthMode UnitMode() const {
    return static_cast<SVGLengthMode>(unit_mode_);
  }

  bool operator==(const SVGLength&) const;
  bool operator!=(const SVGLength& other) const { return !operator==(other); }

  Length ConvertToLength(const SVGLengthConversionData&) const;
  float Value(const SVGLengthConversionData&, float dimension) const;
  float Value(const SVGLengthContext&) const;
  float ValueInSpecifiedUnits() const {
    return To<CSSNumericLiteralValue>(*value_).GetFloatValue();
  }

  void SetValueAsNumber(float);
  void SetValueInSpecifiedUnits(float value);

  const CSSPrimitiveValue& AsCSSPrimitiveValue() const { return *value_; }

  String ValueAsString() const override;
  SVGParsingError SetValueAsString(const String&);

  void NewValueSpecifiedUnits(CSSPrimitiveValue::UnitType,
                              float value_in_specified_units);
  void ConvertToSpecifiedUnits(CSSPrimitiveValue::UnitType,
                               const SVGLengthContext&);

  // Helper functions
  bool IsRelative() const;
  bool IsFontRelative() const {
    // TODO(crbug.com/979895): This is the result of a refactoring, which might
    // have revealed an existing bug with calculated lengths. Investigate.
    return value_->IsNumericLiteralValue() &&
           To<CSSNumericLiteralValue>(*value_).IsFontRelativeLength();
  }
  bool IsCalculated() const { return value_->IsCalculated(); }
  bool IsPercentage() const { return value_->IsPercentage(); }
  bool HasContainerRelativeUnits() const {
    return value_->HasContainerRelativeUnits();
  }

  bool IsNegativeNumericLiteral() const;

  static bool NegativeValuesForbiddenForAnimatedLengthAttribute(
      const QualifiedName&);

  void Add(const SVGPropertyBase*, const SVGElement*) override;
  void CalculateAnimatedValue(
      const SMILAnimationEffectParameters&,
      float percentage,
      unsigned repeat_count,
      const SVGPropertyBase* from,
      const SVGPropertyBase* to,
      const SVGPropertyBase* to_at_end_of_duration_value,
      const SVGElement* context_element) override;
  float CalculateDistance(const SVGPropertyBase* to,
                          const SVGElement* context_element) const override;

  static AnimatedPropertyType ClassType() { return kAnimatedLength; }
  AnimatedPropertyType GetType() const override { return ClassType(); }

 private:
  Member<const CSSPrimitiveValue> value_;
  unsigned unit_mode_ : 2;
};

template <>
struct DowncastTraits<SVGLength> {
  static bool AllowFrom(const SVGPropertyBase& value) {
    return value.GetType() == SVGLength::ClassType();
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_SVG_SVG_LENGTH_H_
