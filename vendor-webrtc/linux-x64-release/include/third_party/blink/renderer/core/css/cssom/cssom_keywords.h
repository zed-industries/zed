// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSSOM_CSSOM_KEYWORDS_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSSOM_CSSOM_KEYWORDS_H_

#include "third_party/blink/renderer/core/css/css_property_names.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

class CSSKeywordValue;

// CSSOMKeywords provides utility methods for determining whether a given
// CSSKeywordValue is valid for a given CSS Property.
//
// The implementation for this class is generated using input from
// css_properties.json5 and build/scripts/make_cssom_types.py.
class CSSOMKeywords {
  STATIC_ONLY(CSSOMKeywords);

 public:
  static bool ValidKeywordForProperty(CSSPropertyID, const CSSKeywordValue&);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSSOM_CSSOM_KEYWORDS_H_
