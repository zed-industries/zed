// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_RAY_VALUE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_RAY_VALUE_H_

#include "third_party/blink/renderer/core/css/css_value.h"
#include "third_party/blink/renderer/platform/wtf/casting.h"

namespace WTF {
class String;
}  // namespace WTF

namespace blink {

class CSSIdentifierValue;
class CSSPrimitiveValue;

namespace cssvalue {

class CSSRayValue : public CSSValue {
 public:
  CSSRayValue(const CSSPrimitiveValue& angle,
              const CSSIdentifierValue& size,
              const CSSIdentifierValue* contain,
              const CSSValue* center_x,
              const CSSValue* center_y);

  const CSSPrimitiveValue& Angle() const { return *angle_; }
  const CSSIdentifierValue& Size() const { return *size_; }
  const CSSIdentifierValue* Contain() const { return contain_.Get(); }
  const CSSValue* CenterX() const { return center_x_.Get(); }
  const CSSValue* CenterY() const { return center_y_.Get(); }

  WTF::String CustomCSSText() const;

  bool Equals(const CSSRayValue&) const;

  void TraceAfterDispatch(blink::Visitor*) const;

 private:
  Member<const CSSPrimitiveValue> angle_;
  Member<const CSSIdentifierValue> size_;
  Member<const CSSIdentifierValue> contain_;
  Member<const CSSValue> center_x_;
  Member<const CSSValue> center_y_;
};

}  // namespace cssvalue

template <>
struct DowncastTraits<cssvalue::CSSRayValue> {
  static bool AllowFrom(const CSSValue& value) { return value.IsRayValue(); }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_RAY_VALUE_H_
