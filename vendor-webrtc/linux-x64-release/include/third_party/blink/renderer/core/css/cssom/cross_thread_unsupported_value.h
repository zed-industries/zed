// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSSOM_CROSS_THREAD_UNSUPPORTED_VALUE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSSOM_CROSS_THREAD_UNSUPPORTED_VALUE_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/css/cssom/cross_thread_style_value.h"
#include "third_party/blink/renderer/platform/wtf/casting.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

class CSSStyleValue;

class CORE_EXPORT CrossThreadUnsupportedValue final
    : public CrossThreadStyleValue {
 public:
  explicit CrossThreadUnsupportedValue(const String& value) : value_(value) {}
  CrossThreadUnsupportedValue(const CrossThreadUnsupportedValue&) = delete;
  CrossThreadUnsupportedValue& operator=(const CrossThreadUnsupportedValue&) =
      delete;
  ~CrossThreadUnsupportedValue() override = default;

  StyleValueType GetType() const override {
    return StyleValueType::kUnknownType;
  }
  CSSStyleValue* ToCSSStyleValue() override;
  std::unique_ptr<CrossThreadStyleValue> IsolatedCopy() const override;

  bool operator==(const CrossThreadStyleValue&) const override;

 private:
  friend class CrossThreadStyleValueTest;

  String value_;
};

template <>
struct DowncastTraits<CrossThreadUnsupportedValue> {
  static bool AllowFrom(const CrossThreadStyleValue& unsupported_value) {
    return unsupported_value.GetType() ==
           CrossThreadStyleValue::StyleValueType::kUnknownType;
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSSOM_CROSS_THREAD_UNSUPPORTED_VALUE_H_
