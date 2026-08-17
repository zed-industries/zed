// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_DYNAMIC_RANGE_LIMIT_MIX_VALUE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_DYNAMIC_RANGE_LIMIT_MIX_VALUE_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/css/css_primitive_value.h"
#include "third_party/blink/renderer/core/css/css_value.h"
#include "third_party/blink/renderer/platform/wtf/casting.h"

namespace blink {
namespace cssvalue {

class CORE_EXPORT CSSDynamicRangeLimitMixValue : public CSSValue {
 public:
  CSSDynamicRangeLimitMixValue(
      HeapVector<Member<const CSSValue>>&& limits,
      HeapVector<Member<const CSSPrimitiveValue>>&& percentages)
      : CSSValue(kDynamicRangeLimitMixClass),
        limits_(limits),
        percentages_(percentages) {
    CHECK(limits_.size() == percentages_.size());
  }

  String CustomCSSText() const;

  void TraceAfterDispatch(blink::Visitor* visitor) const;

  bool Equals(const CSSDynamicRangeLimitMixValue& other) const;
  const HeapVector<Member<const CSSValue>>& Limits() const { return limits_; }
  const HeapVector<Member<const CSSPrimitiveValue>>& Percentages() const {
    return percentages_;
  }

 private:
  const HeapVector<Member<const CSSValue>> limits_;
  const HeapVector<Member<const CSSPrimitiveValue>> percentages_;
};

}  // namespace cssvalue

template <>
struct DowncastTraits<cssvalue::CSSDynamicRangeLimitMixValue> {
  static bool AllowFrom(const CSSValue& value) {
    return value.IsDynamicRangeLimitMixValue();
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_DYNAMIC_RANGE_LIMIT_MIX_VALUE_H_
