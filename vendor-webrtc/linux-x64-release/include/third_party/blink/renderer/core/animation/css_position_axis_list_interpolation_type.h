// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_CSS_POSITION_AXIS_LIST_INTERPOLATION_TYPE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_CSS_POSITION_AXIS_LIST_INTERPOLATION_TYPE_H_

#include "third_party/blink/renderer/core/animation/css_length_list_interpolation_type.h"

namespace blink {

class CSSPositionAxisListInterpolationType
    : public CSSLengthListInterpolationType {
 public:
  explicit CSSPositionAxisListInterpolationType(PropertyHandle property)
      : CSSLengthListInterpolationType(property) {}

  static InterpolationValue ConvertPositionAxisCSSValue(const CSSValue&);

 private:
  InterpolationValue MaybeConvertValue(const CSSValue&,
                                       const StyleResolverState&,
                                       ConversionCheckers&) const final;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_CSS_POSITION_AXIS_LIST_INTERPOLATION_TYPE_H_
