// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_REVERT_LAYER_VALUE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_REVERT_LAYER_VALUE_H_

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

class CORE_EXPORT CSSRevertLayerValue : public CSSValue {
 public:
  static CSSRevertLayerValue* Create();

  explicit CSSRevertLayerValue(base::PassKey<CSSValuePool>)
      : CSSValue(kRevertLayerClass) {}

  WTF::String CustomCSSText() const;

  bool Equals(const CSSRevertLayerValue&) const { return true; }

  void TraceAfterDispatch(blink::Visitor* visitor) const {
    CSSValue::TraceAfterDispatch(visitor);
  }
};

}  // namespace cssvalue

template <>
struct DowncastTraits<cssvalue::CSSRevertLayerValue> {
  static bool AllowFrom(const CSSValue& value) {
    return value.IsRevertLayerValue();
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_REVERT_LAYER_VALUE_H_
