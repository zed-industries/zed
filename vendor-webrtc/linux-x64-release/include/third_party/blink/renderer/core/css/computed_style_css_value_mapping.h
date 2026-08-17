// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_COMPUTED_STYLE_CSS_VALUE_MAPPING_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_COMPUTED_STYLE_CSS_VALUE_MAPPING_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/css/css_value.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_map.h"
#include "third_party/blink/renderer/platform/wtf/text/atomic_string.h"

namespace blink {

class ComputedStyle;
class PropertyRegistry;
enum class CSSValuePhase;

class CORE_EXPORT ComputedStyleCSSValueMapping {
  STATIC_ONLY(ComputedStyleCSSValueMapping);

 public:
  static HeapHashMap<AtomicString, Member<const CSSValue>> GetVariables(
      const ComputedStyle& style,
      const PropertyRegistry*,
      CSSValuePhase value_phase);

 private:
  static const CSSValue* Get(const AtomicString& custom_property_name,
                             const ComputedStyle&,
                             const PropertyRegistry*,
                             CSSValuePhase value_phase);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_COMPUTED_STYLE_CSS_VALUE_MAPPING_H_
