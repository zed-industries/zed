// Copyright 2025 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_SUPERELLIPSE_VALUE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_SUPERELLIPSE_VALUE_H_

#include "third_party/blink/renderer/core/css/css_primitive_value.h"
#include "third_party/blink/renderer/core/css/css_value.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/wtf/casting.h"

namespace blink {

namespace cssvalue {

class CSSSuperellipseValue : public CSSValue {
 public:
  explicit CSSSuperellipseValue(const CSSPrimitiveValue& param)
      : CSSValue(kSuperellipseClass), param_(param) {}

  String CustomCSSText() const;

  bool Equals(const CSSSuperellipseValue& other) const {
    return *param_ == *other.param_;
  }

  unsigned CustomHash() const { return param_->Hash(); }

  const CSSPrimitiveValue& Param() const { return *param_; }

  void TraceAfterDispatch(blink::Visitor*) const;

 private:
  Member<const CSSPrimitiveValue> param_;
};

}  // namespace cssvalue

template <>
struct DowncastTraits<cssvalue::CSSSuperellipseValue> {
  static bool AllowFrom(const CSSValue& value) {
    return value.IsSuperellipseValue();
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_SUPERELLIPSE_VALUE_H_
