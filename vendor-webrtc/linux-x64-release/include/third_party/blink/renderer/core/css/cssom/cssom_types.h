// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSSOM_CSSOM_TYPES_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSSOM_CSSOM_TYPES_H_

#include "third_party/blink/renderer/core/css/css_property_names.h"
#include "third_party/blink/renderer/core/css/cssom/css_style_value.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

// This class provides utility functions for determining whether a property
// can accept a CSSStyleValue type or instance. Its implementation is generated
// using input from css_properties.json5 and the script
// build/scripts/make_cssom_types.py.
class CSSOMTypes {
  STATIC_ONLY(CSSOMTypes);

 public:
  static bool IsCSSStyleValueLength(const CSSStyleValue&);
  static bool IsCSSStyleValueNumber(const CSSStyleValue&);
  static bool IsCSSStyleValueTime(const CSSStyleValue&);
  static bool IsCSSStyleValueAngle(const CSSStyleValue&);
  static bool IsCSSStyleValuePercentage(const CSSStyleValue&);
  static bool IsCSSStyleValueResolution(const CSSStyleValue&);
  static bool IsCSSStyleValueFlex(const CSSStyleValue&);
  static bool IsCSSStyleValueImage(const CSSStyleValue&);
  static bool IsCSSStyleValueTransform(const CSSStyleValue&);
  static bool IsCSSStyleValuePosition(const CSSStyleValue&);

  static bool IsPropertySupported(CSSPropertyID);
  static bool PropertyCanTake(CSSPropertyID,
                              const AtomicString& custom_property_name,
                              const CSSStyleValue&);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSSOM_CSSOM_TYPES_H_
