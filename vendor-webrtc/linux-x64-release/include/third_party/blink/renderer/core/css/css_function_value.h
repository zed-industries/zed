// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_FUNCTION_VALUE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_FUNCTION_VALUE_H_

#include "third_party/blink/renderer/core/css/css_value_list.h"
#include "third_party/blink/renderer/core/css_value_keywords.h"
#include "third_party/blink/renderer/platform/wtf/casting.h"

namespace WTF {
class String;
}  // namespace WTF

namespace blink {

class CSSFunctionValue : public CSSValueList {
 public:
  CSSFunctionValue(CSSValueID id)
      : CSSValueList(kFunctionClass, kCommaSeparator), value_id_(id) {}

  WTF::String CustomCSSText() const;

  bool Equals(const CSSFunctionValue& other) const {
    return value_id_ == other.value_id_ && CSSValueList::Equals(other);
  }
  CSSValueID FunctionType() const { return value_id_; }

  void TraceAfterDispatch(blink::Visitor* visitor) const {
    CSSValueList::TraceAfterDispatch(visitor);
  }

 private:
  const CSSValueID value_id_;
};

template <>
struct DowncastTraits<CSSFunctionValue> {
  static bool AllowFrom(const CSSValue& value) {
    return value.IsFunctionValue();
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_FUNCTION_VALUE_H_
