// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_AXIS_VALUE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_AXIS_VALUE_H_

#include "third_party/blink/renderer/core/css/css_value_list.h"
#include "third_party/blink/renderer/core/css_value_keywords.h"
#include "third_party/blink/renderer/platform/wtf/casting.h"

namespace WTF {
class String;
}  // namespace WTF

namespace blink {

class CSSLengthResolver;
class CSSPrimitiveValue;

namespace cssvalue {

class CSSAxisValue : public CSSValueList {
 public:
  struct Axis : std::tuple<double, double, double> {};

  explicit CSSAxisValue(CSSValueID axis_name);
  CSSAxisValue(const CSSPrimitiveValue* x,
               const CSSPrimitiveValue* y,
               const CSSPrimitiveValue* z);

  WTF::String CustomCSSText() const;

  Axis ComputeAxis(const CSSLengthResolver&) const;
  CSSValueID AxisName() const { return axis_name_; }

  void TraceAfterDispatch(blink::Visitor* visitor) const {
    CSSValueList::TraceAfterDispatch(visitor);
  }

 private:
  CSSValueID axis_name_;
};

}  // namespace cssvalue

template <>
struct DowncastTraits<cssvalue::CSSAxisValue> {
  static bool AllowFrom(const CSSValue& value) { return value.IsAxisValue(); }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_AXIS_VALUE_H_
