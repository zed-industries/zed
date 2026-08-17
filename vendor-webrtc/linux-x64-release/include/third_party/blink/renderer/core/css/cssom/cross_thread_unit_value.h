// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSSOM_CROSS_THREAD_UNIT_VALUE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSSOM_CROSS_THREAD_UNIT_VALUE_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/css/css_primitive_value.h"
#include "third_party/blink/renderer/core/css/cssom/cross_thread_style_value.h"
#include "third_party/blink/renderer/platform/wtf/casting.h"

namespace blink {

class CSSStyleValue;

class CORE_EXPORT CrossThreadUnitValue final : public CrossThreadStyleValue {
 public:
  explicit CrossThreadUnitValue(double value, CSSPrimitiveValue::UnitType unit)
      : value_(value), unit_(unit) {}
  CrossThreadUnitValue(const CrossThreadUnitValue&) = delete;
  CrossThreadUnitValue& operator=(const CrossThreadUnitValue&) = delete;
  ~CrossThreadUnitValue() override = default;

  StyleValueType GetType() const override { return StyleValueType::kUnitType; }
  CSSStyleValue* ToCSSStyleValue() override;
  std::unique_ptr<CrossThreadStyleValue> IsolatedCopy() const override;

  bool operator==(const CrossThreadStyleValue&) const override;

  CSSPrimitiveValue::UnitType GetUnitType() const { return unit_; }

 private:
  friend class CrossThreadStyleValueTest;

  double value_;
  CSSPrimitiveValue::UnitType unit_;
};

template <>
struct DowncastTraits<CrossThreadUnitValue> {
  static bool AllowFrom(const CrossThreadStyleValue& unit_value) {
    return unit_value.GetType() ==
           CrossThreadStyleValue::StyleValueType::kUnitType;
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSSOM_CROSS_THREAD_UNIT_VALUE_H_
