// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_REVERT_VALUE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_REVERT_VALUE_H_

#include "base/memory/scoped_refptr.h"
#include "base/types/pass_key.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/css/css_value.h"
#include "third_party/blink/renderer/platform/wtf/casting.h"

namespace WTF {
class String;
}  // namespace WTF

namespace blink {

class CSSValuePool;

namespace cssvalue {

class CORE_EXPORT CSSRevertValue : public CSSValue {
 public:
  static CSSRevertValue* Create();

  explicit CSSRevertValue(base::PassKey<CSSValuePool>)
      : CSSValue(kRevertClass) {}

  WTF::String CustomCSSText() const;

  bool Equals(const CSSRevertValue&) const { return true; }

  void TraceAfterDispatch(blink::Visitor* visitor) const {
    CSSValue::TraceAfterDispatch(visitor);
  }
};

}  // namespace cssvalue

template <>
struct DowncastTraits<cssvalue::CSSRevertValue> {
  static bool AllowFrom(const CSSValue& value) { return value.IsRevertValue(); }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_REVERT_VALUE_H_
