// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSSOM_CROSS_THREAD_UNPARSED_VALUE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSSOM_CROSS_THREAD_UNPARSED_VALUE_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/css/cssom/cross_thread_style_value.h"
#include "third_party/blink/renderer/platform/wtf/casting.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

class CSSStyleValue;

class CORE_EXPORT CrossThreadUnparsedValue final
    : public CrossThreadStyleValue {
 public:
  explicit CrossThreadUnparsedValue(const String& value) : value_(value) {}
  CrossThreadUnparsedValue(const CrossThreadUnparsedValue&) = delete;
  CrossThreadUnparsedValue& operator=(const CrossThreadUnparsedValue&) = delete;
  ~CrossThreadUnparsedValue() override = default;

  StyleValueType GetType() const override {
    return StyleValueType::kUnparsedType;
  }
  CSSStyleValue* ToCSSStyleValue() override;

  std::unique_ptr<CrossThreadStyleValue> IsolatedCopy() const override;

  bool operator==(const CrossThreadStyleValue&) const override;

 private:
  friend class CrossThreadStyleValueTest;

  String value_;
};

template <>
struct DowncastTraits<CrossThreadUnparsedValue> {
  static bool AllowFrom(const CrossThreadStyleValue& value) {
    return value.GetType() ==
           CrossThreadStyleValue::StyleValueType::kUnparsedType;
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSSOM_CROSS_THREAD_UNPARSED_VALUE_H_
