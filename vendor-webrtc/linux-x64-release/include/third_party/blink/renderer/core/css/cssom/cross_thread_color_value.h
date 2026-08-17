// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSSOM_CROSS_THREAD_COLOR_VALUE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSSOM_CROSS_THREAD_COLOR_VALUE_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/css/css_primitive_value.h"
#include "third_party/blink/renderer/core/css/cssom/cross_thread_style_value.h"
#include "third_party/blink/renderer/platform/graphics/color.h"
#include "third_party/blink/renderer/platform/wtf/casting.h"

namespace blink {

class CSSStyleValue;

class CORE_EXPORT CrossThreadColorValue final : public CrossThreadStyleValue {
 public:
  explicit CrossThreadColorValue(Color value) : value_(value) {}
  CrossThreadColorValue(const CrossThreadColorValue&) = delete;
  CrossThreadColorValue& operator=(const CrossThreadColorValue&) = delete;
  ~CrossThreadColorValue() override = default;

  StyleValueType GetType() const override { return StyleValueType::kColorType; }
  CSSStyleValue* ToCSSStyleValue() override;
  std::unique_ptr<CrossThreadStyleValue> IsolatedCopy() const override;

  bool operator==(const CrossThreadStyleValue&) const override;

  Color GetValue() const { return value_; }

 private:
  friend class CrossThreadStyleValueTest;

  Color value_;
};

template <>
struct DowncastTraits<CrossThreadColorValue> {
  static bool AllowFrom(const CrossThreadStyleValue& color_value) {
    return color_value.GetType() ==
           CrossThreadStyleValue::StyleValueType::kColorType;
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSSOM_CROSS_THREAD_COLOR_VALUE_H_
