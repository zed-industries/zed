// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_ZOOM_ADJUSTED_PIXEL_VALUE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_ZOOM_ADJUSTED_PIXEL_VALUE_H_

#include "third_party/blink/renderer/core/css/css_numeric_literal_value.h"
#include "third_party/blink/renderer/core/css/css_primitive_value.h"
#include "third_party/blink/renderer/core/layout/adjust_for_absolute_zoom.h"

namespace blink {

class ComputedStyle;

inline CSSPrimitiveValue* ZoomAdjustedPixelValue(double value,
                                                 const ComputedStyle& style) {
  return CSSNumericLiteralValue::Create(
      AdjustForAbsoluteZoom::AdjustFloat(value, style),
      CSSPrimitiveValue::UnitType::kPixels);
}

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_ZOOM_ADJUSTED_PIXEL_VALUE_H_
