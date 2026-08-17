// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_LENGTH_UNITS_CHECKER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_LENGTH_UNITS_CHECKER_H_

#include <memory>
#include <utility>

#include "base/memory/ptr_util.h"
#include "third_party/blink/renderer/core/animation/css_interpolation_type.h"
#include "third_party/blink/renderer/core/css/css_primitive_value.h"
#include "third_party/blink/renderer/core/css/resolver/style_resolver_state.h"

namespace blink {

class LengthUnitsChecker : public CSSInterpolationType::CSSConversionChecker {
 public:
  struct UnitLength {
    explicit UnitLength(CSSPrimitiveValue::LengthUnitType unit,
                        const CSSToLengthConversionData& conversion_data)
        : unit(unit), length_pixels(UnitLengthPixels(unit, conversion_data)) {}

    const CSSPrimitiveValue::LengthUnitType unit;
    const double length_pixels;
  };

  static LengthUnitsChecker* MaybeCreate(
      const CSSPrimitiveValue::LengthTypeFlags& length_types,
      const StyleResolverState& state) {
    Vector<UnitLength> unit_lengths;
    for (wtf_size_t i = 0; i < length_types.size(); ++i) {
      if (i == CSSPrimitiveValue::kUnitTypePercentage || !length_types[i])
        continue;
      unit_lengths.push_back(
          UnitLength(static_cast<CSSPrimitiveValue::LengthUnitType>(i),
                     state.CssToLengthConversionData()));
    }
    if (unit_lengths.empty())
      return nullptr;
    return MakeGarbageCollected<LengthUnitsChecker>(std::move(unit_lengths));
  }

  explicit LengthUnitsChecker(Vector<UnitLength> unit_lengths)
      : unit_lengths_(std::move(unit_lengths)) {}

  bool IsValid(const StyleResolverState& state,
               const InterpolationValue& underlying) const final {
    for (const UnitLength& unit_length : unit_lengths_) {
      if (unit_length.length_pixels !=
          UnitLengthPixels(unit_length.unit,
                           state.CssToLengthConversionData())) {
        return false;
      }
    }
    return true;
  }

 private:
  static double UnitLengthPixels(
      CSSPrimitiveValue::LengthUnitType length_unit_type,
      const CSSToLengthConversionData& conversion_data) {
    return conversion_data.ZoomedComputedPixels(
        1, CSSPrimitiveValue::LengthUnitTypeToUnitType(length_unit_type));
  }

  Vector<UnitLength> unit_lengths_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_LENGTH_UNITS_CHECKER_H_
