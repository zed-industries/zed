// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_INITIAL_COLOR_VALUE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_INITIAL_COLOR_VALUE_H_

#include "base/types/pass_key.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/css/css_value.h"
#include "third_party/blink/renderer/platform/wtf/casting.h"

namespace WTF {
class String;
}  // namespace WTF

namespace blink {

class CSSValuePool;

// TODO(crbug.com/1046753): Remove this class when canvastext is supported.
class CORE_EXPORT CSSInitialColorValue : public CSSValue {
 public:
  static CSSInitialColorValue* Create();

  explicit CSSInitialColorValue(base::PassKey<CSSValuePool>)
      : CSSValue(kInitialColorValueClass) {}

  WTF::String CustomCSSText() const;

  bool Equals(const CSSInitialColorValue&) const { return true; }

  void TraceAfterDispatch(blink::Visitor* visitor) const {
    CSSValue::TraceAfterDispatch(visitor);
  }

 private:
  friend class CSSValuePool;
};

template <>
struct DowncastTraits<CSSInitialColorValue> {
  static bool AllowFrom(const CSSValue& value) {
    return value.IsInitialColorValue();
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_INITIAL_COLOR_VALUE_H_
