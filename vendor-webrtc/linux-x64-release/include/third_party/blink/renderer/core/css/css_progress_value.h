// Copyright 2025 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_PROGRESS_VALUE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_PROGRESS_VALUE_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/css/css_value.h"
#include "third_party/blink/renderer/platform/wtf/casting.h"

namespace blink {
namespace cssvalue {

class CSSProgressValue : public CSSValue {
 public:
  CORE_EXPORT CSSProgressValue(const CSSValue& progress,
                               const CSSValue* easing_function);

  const CSSValue& Progress() const { return *progress_; }
  const CSSValue* EasingFunction() const { return easing_function_; }

  WTF::String CustomCSSText() const;

  bool Equals(const CSSProgressValue&) const;

  void TraceAfterDispatch(blink::Visitor* visitor) const;

 private:
  Member<const CSSValue> progress_;
  Member<const CSSValue> easing_function_;
};

}  // namespace cssvalue

template <>
struct DowncastTraits<cssvalue::CSSProgressValue> {
  static bool AllowFrom(const CSSValue& value) {
    return value.IsProgressValue();
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_PROGRESS_VALUE_H_
