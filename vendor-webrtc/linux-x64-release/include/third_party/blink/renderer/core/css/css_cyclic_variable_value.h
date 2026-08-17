// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_CYCLIC_VARIABLE_VALUE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_CYCLIC_VARIABLE_VALUE_H_

#include "base/types/pass_key.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/css/css_invalid_variable_value.h"
#include "third_party/blink/renderer/platform/wtf/casting.h"

namespace WTF {
class String;
}  // namespace WTF

namespace blink {

class CSSValuePool;

// CSSCyclicVariableValue is a special case of CSSInvalidVariableValue which
// indicates that a custom property is invalid because it's in a cycle.
//
// https://drafts.csswg.org/css-variables/#cycles
class CORE_EXPORT CSSCyclicVariableValue : public CSSInvalidVariableValue {
 public:
  static CSSCyclicVariableValue* Create();

  explicit CSSCyclicVariableValue(base::PassKey<CSSValuePool>)
      : CSSInvalidVariableValue(kCyclicVariableValueClass) {}

  WTF::String CustomCSSText() const;

  bool Equals(const CSSCyclicVariableValue&) const { return true; }

  void TraceAfterDispatch(blink::Visitor* visitor) const {
    CSSInvalidVariableValue::TraceAfterDispatch(visitor);
  }

 private:
  friend class CSSValuePool;
};

template <>
struct DowncastTraits<CSSCyclicVariableValue> {
  static bool AllowFrom(const CSSValue& value) {
    return value.IsCyclicVariableValue();
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_CYCLIC_VARIABLE_VALUE_H_
