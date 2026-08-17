// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_CONTENT_DISTRIBUTION_VALUE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_CONTENT_DISTRIBUTION_VALUE_H_

#include "base/memory/scoped_refptr.h"
#include "third_party/blink/renderer/core/css/css_identifier_value.h"
#include "third_party/blink/renderer/core/css/css_value.h"
#include "third_party/blink/renderer/core/css/css_value_pair.h"
#include "third_party/blink/renderer/platform/wtf/casting.h"

namespace blink {
namespace cssvalue {

class CSSContentDistributionValue : public CSSValue {
 public:
  CSSContentDistributionValue(CSSValueID distribution,
                              CSSValueID position,
                              CSSValueID overflow);

  CSSValueID Distribution() const { return distribution_; }

  CSSValueID Position() const { return position_; }

  CSSValueID Overflow() const { return overflow_; }

  String CustomCSSText() const;

  bool Equals(const CSSContentDistributionValue&) const;
  unsigned CustomHash() const;

  void TraceAfterDispatch(blink::Visitor* visitor) const {
    CSSValue::TraceAfterDispatch(visitor);
  }

 private:
  CSSValueID distribution_;
  CSSValueID position_;
  CSSValueID overflow_;
};

}  // namespace cssvalue

template <>
struct DowncastTraits<cssvalue::CSSContentDistributionValue> {
  static bool AllowFrom(const CSSValue& value) {
    return value.IsContentDistributionValue();
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_CONTENT_DISTRIBUTION_VALUE_H_
