// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_LAYOUT_FUNCTION_VALUE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_LAYOUT_FUNCTION_VALUE_H_

#include "third_party/blink/renderer/core/css/css_value.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "third_party/blink/renderer/platform/wtf/casting.h"

namespace WTF {
class String;
}  // namespace WTF

namespace blink {

class CSSCustomIdentValue;

namespace cssvalue {

class CSSLayoutFunctionValue : public CSSValue {
 public:
  CSSLayoutFunctionValue(CSSCustomIdentValue* name, bool is_inline);

  WTF::String CustomCSSText() const;
  AtomicString GetName() const;
  bool IsInline() const { return is_inline_; }

  bool Equals(const CSSLayoutFunctionValue&) const;
  void TraceAfterDispatch(blink::Visitor*) const;

 private:
  Member<CSSCustomIdentValue> name_;
  bool is_inline_;
};

}  // namespace cssvalue

template <>
struct DowncastTraits<cssvalue::CSSLayoutFunctionValue> {
  static bool AllowFrom(const CSSValue& value) {
    return value.IsLayoutFunctionValue();
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_LAYOUT_FUNCTION_VALUE_H_
