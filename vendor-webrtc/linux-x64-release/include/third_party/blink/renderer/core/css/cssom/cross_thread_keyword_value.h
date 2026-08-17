// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSSOM_CROSS_THREAD_KEYWORD_VALUE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSSOM_CROSS_THREAD_KEYWORD_VALUE_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/css/cssom/cross_thread_style_value.h"
#include "third_party/blink/renderer/platform/wtf/casting.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

class CSSStyleValue;

class CORE_EXPORT CrossThreadKeywordValue final : public CrossThreadStyleValue {
 public:
  explicit CrossThreadKeywordValue(const String& keyword)
      : keyword_value_(keyword) {}
  CrossThreadKeywordValue(const CrossThreadKeywordValue&) = delete;
  CrossThreadKeywordValue& operator=(const CrossThreadKeywordValue&) = delete;
  ~CrossThreadKeywordValue() override = default;

  StyleValueType GetType() const override {
    return StyleValueType::kKeywordType;
  }
  CSSStyleValue* ToCSSStyleValue() override;

  std::unique_ptr<CrossThreadStyleValue> IsolatedCopy() const override;

  bool operator==(const CrossThreadStyleValue&) const override;

 private:
  friend class CrossThreadStyleValueTest;

  String keyword_value_;
};

template <>
struct DowncastTraits<CrossThreadKeywordValue> {
  static bool AllowFrom(const CrossThreadStyleValue& keyword_value) {
    return keyword_value.GetType() ==
           CrossThreadStyleValue::StyleValueType::kKeywordType;
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSSOM_CROSS_THREAD_KEYWORD_VALUE_H_
