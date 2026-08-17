// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_REPEAT_VALUE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_REPEAT_VALUE_H_

#include "third_party/blink/renderer/core/css/css_primitive_value.h"
#include "third_party/blink/renderer/core/css/css_value_list.h"
#include "third_party/blink/renderer/core/css_value_keywords.h"
#include "third_party/blink/renderer/platform/wtf/casting.h"

namespace WTF {
class String;
}  // namespace WTF

namespace blink {
namespace cssvalue {

// CSSRepeatValue stores the list of values when the repeat syntax is used.
// Repeat syntax can either be auto or integer. For auto, the grammar is
// usually: <auto-repeat-value>  = repeat(auto, [ <value> ]+ ) For integer, the
// grammar is usually: <integer-repeat-value> = repeat( [ <positive-integer> ],
// [ <value> ]+ )
class CSSRepeatValue : public CSSValue {
 public:
  explicit CSSRepeatValue(const CSSPrimitiveValue* repetitions,
                          const CSSValueList& values)
      : CSSValue(kRepeatClass), repetitions_(repetitions), values_(&values) {}

  WTF::String CustomCSSText() const;
  bool Equals(const CSSRepeatValue&) const;

  const CSSPrimitiveValue* Repetitions() const;

  bool IsAutoRepeatValue() const;

  const CSSValueList& Values() const;

  void TraceAfterDispatch(blink::Visitor* visitor) const;

 private:
  Member<const CSSPrimitiveValue> repetitions_;
  Member<const CSSValueList> values_;
};

}  // namespace cssvalue

template <>
struct DowncastTraits<cssvalue::CSSRepeatValue> {
  static bool AllowFrom(const CSSValue& value) { return value.IsRepeatValue(); }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_REPEAT_VALUE_H_
