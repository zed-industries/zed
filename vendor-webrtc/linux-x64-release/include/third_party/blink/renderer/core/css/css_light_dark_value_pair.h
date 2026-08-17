// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_LIGHT_DARK_VALUE_PAIR_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_LIGHT_DARK_VALUE_PAIR_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/css/css_value_pair.h"

namespace blink {

class CORE_EXPORT CSSLightDarkValuePair : public CSSValuePair {
 public:
  CSSLightDarkValuePair(const CSSValue* first, const CSSValue* second)
      : CSSValuePair(kLightDarkValuePairClass, first, second) {}
  String CustomCSSText() const;
  void TraceAfterDispatch(blink::Visitor* visitor) const {
    CSSValuePair::TraceAfterDispatch(visitor);
  }
};

template <>
struct DowncastTraits<CSSLightDarkValuePair> {
  static bool AllowFrom(const CSSValue& value) {
    return value.IsLightDarkValuePair();
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_LIGHT_DARK_VALUE_PAIR_H_
