// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSSOM_CROSS_THREAD_STYLE_VALUE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSSOM_CROSS_THREAD_STYLE_VALUE_H_

#include <memory>

#include "third_party/blink/renderer/core/core_export.h"

namespace blink {

class CSSStyleValue;

// This class is designed for CSS Paint such that its instance can be safely
// passed cross threads.
class CORE_EXPORT CrossThreadStyleValue {
 public:
  enum class StyleValueType {
    kUnknownType,
    kUnparsedType,
    kKeywordType,
    kUnitType,
    kColorType,
  };

  CrossThreadStyleValue(const CrossThreadStyleValue&) = delete;
  CrossThreadStyleValue& operator=(const CrossThreadStyleValue&) = delete;
  virtual ~CrossThreadStyleValue() = default;

  virtual StyleValueType GetType() const = 0;
  virtual CSSStyleValue* ToCSSStyleValue() = 0;
  virtual std::unique_ptr<CrossThreadStyleValue> IsolatedCopy() const = 0;

  virtual bool operator==(const CrossThreadStyleValue&) const = 0;

 protected:
  CrossThreadStyleValue() = default;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSSOM_CROSS_THREAD_STYLE_VALUE_H_
