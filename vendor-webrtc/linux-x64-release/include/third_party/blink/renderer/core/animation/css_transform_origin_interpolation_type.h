// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_CSS_TRANSFORM_ORIGIN_INTERPOLATION_TYPE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_CSS_TRANSFORM_ORIGIN_INTERPOLATION_TYPE_H_

#include "base/check_op.h"
#include "third_party/blink/renderer/core/animation/css_length_list_interpolation_type.h"
#include "third_party/blink/renderer/core/animation/css_position_axis_list_interpolation_type.h"
#include "third_party/blink/renderer/core/animation/interpolable_length.h"
#include "third_party/blink/renderer/core/animation/list_interpolation_functions.h"
#include "third_party/blink/renderer/core/css/css_numeric_literal_value.h"
#include "third_party/blink/renderer/core/css/css_value_list.h"

namespace blink {

class CSSTransformOriginInterpolationType
    : public CSSLengthListInterpolationType {
 public:
  explicit CSSTransformOriginInterpolationType(PropertyHandle property)
      : CSSLengthListInterpolationType(property) {}

 private:
  InterpolationValue MaybeConvertValue(const CSSValue& value,
                                       const StyleResolverState&,
                                       ConversionCheckers&) const final {
    const CSSValueList& list = To<CSSValueList>(value);
    DCHECK_GE(list.length(), 2u);
    return ListInterpolationFunctions::CreateList(
        3, [&list](wtf_size_t index) {
          if (index == list.length()) {
            return InterpolationValue(InterpolableLength::MaybeConvertCSSValue(
                *CSSNumericLiteralValue::Create(
                    0, CSSPrimitiveValue::UnitType::kPixels)));
          }
          const CSSValue& item = list.Item(index);
          if (index < 2)
            return CSSPositionAxisListInterpolationType::
                ConvertPositionAxisCSSValue(item);
          return InterpolationValue(
              InterpolableLength::MaybeConvertCSSValue(item));
        });
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_CSS_TRANSFORM_ORIGIN_INTERPOLATION_TYPE_H_
