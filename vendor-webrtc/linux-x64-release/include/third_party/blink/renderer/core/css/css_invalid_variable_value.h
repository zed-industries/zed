// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_INVALID_VARIABLE_VALUE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_INVALID_VARIABLE_VALUE_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/css/css_value.h"
#include "third_party/blink/renderer/platform/wtf/casting.h"

namespace WTF {
class String;
}  // namespace WTF

namespace blink {

// A value which represents custom properties that are invalid at computed-
// value time.
//
// https://drafts.csswg.org/css-variables/#invalid-at-computed-value-time
class CORE_EXPORT CSSInvalidVariableValue : public CSSValue {
 public:
  static CSSInvalidVariableValue* Create();

  // Only construct through MakeGarbageCollected for the initial value. Use
  // Create() to get the pooled value.
  CSSInvalidVariableValue() : CSSValue(kInvalidVariableValueClass) {}

  WTF::String CustomCSSText() const;

  bool Equals(const CSSInvalidVariableValue&) const { return true; }

  void TraceAfterDispatch(blink::Visitor* visitor) const {
    CSSValue::TraceAfterDispatch(visitor);
  }

 protected:
  explicit CSSInvalidVariableValue(ClassType class_type)
      : CSSValue(class_type) {}

 private:
  friend class CSSValuePool;
};

template <>
struct DowncastTraits<CSSInvalidVariableValue> {
  static bool AllowFrom(const CSSValue& value) {
    return value.IsInvalidVariableValue();
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_INVALID_VARIABLE_VALUE_H_
