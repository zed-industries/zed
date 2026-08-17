// Copyright 2025 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_SHAPE_FUNCTIONS_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_SHAPE_FUNCTIONS_H_

#include "third_party/blink/renderer/core/css/css_shape_value.h"
#include "third_party/blink/renderer/core/css_value_keywords.h"
#include "third_party/blink/renderer/core/style/style_shape.h"

namespace blink {
inline StyleShape::ControlPoint::Origin ToControlPointOrigin(
    CSSValueID origin) {
  switch (origin) {
    case CSSValueID::kOrigin:
      return StyleShape::ControlPoint::Origin::kReferenceBox;
    case CSSValueID::kStart:
      return StyleShape::ControlPoint::Origin::kSegmentStart;
    case CSSValueID::kEnd:
      return StyleShape::ControlPoint::Origin::kSegmentEnd;
    default:
      NOTREACHED();
  }
}

inline CSSValueID FromControlPointOrigin(
    StyleShape::ControlPoint::Origin origin) {
  switch (origin) {
    case StyleShape::ControlPoint::Origin::kReferenceBox:
      return CSSValueID::kOrigin;
    case StyleShape::ControlPoint::Origin::kSegmentStart:
      return CSSValueID::kStart;
    case StyleShape::ControlPoint::Origin::kSegmentEnd:
      return CSSValueID::kEnd;
  };
}
}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_SHAPE_FUNCTIONS_H_
