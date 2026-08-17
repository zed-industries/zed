// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_GRID_INTEGER_REPEAT_VALUE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_GRID_INTEGER_REPEAT_VALUE_H_

#include <optional>

#include "base/check_op.h"
#include "third_party/blink/renderer/core/css/css_primitive_value.h"
#include "third_party/blink/renderer/core/css/css_value_list.h"
#include "third_party/blink/renderer/core/css_value_keywords.h"
#include "third_party/blink/renderer/platform/wtf/casting.h"

namespace WTF {
class String;
}  // namespace WTF

namespace blink {
namespace cssvalue {

// CSSGridIntegerRepeatValue stores the track sizes and line numbers when the
// integer-repeat syntax is used.
//
// Right now the integer-repeat syntax is as follows:
// <track-repeat> = repeat( [ <positive-integer> ],
//                          [ <line-names>? <track-size> ]+ <line-names>? )
// <fixed-repeat> = repeat( [ <positive-integer> ],
//                          [ <line-names>? <fixed-size> ]+ <line-names>? )
class CORE_EXPORT CSSGridIntegerRepeatValue : public CSSValueList {
 public:
  CSSGridIntegerRepeatValue(CSSPrimitiveValue* repetitions,
                            std::optional<wtf_size_t>(extra_clamp))
      : CSSValueList(kGridIntegerRepeatClass, kSpaceSeparator),
        repetitions_(repetitions),
        extra_clamp_(extra_clamp) {}

  WTF::String CustomCSSText() const;
  bool Equals(const CSSGridIntegerRepeatValue&) const;

  std::optional<wtf_size_t> GetRepetitionsIfKnown() const;
  wtf_size_t ComputeRepetitions(const CSSLengthResolver& resolver) const;

  void TraceAfterDispatch(blink::Visitor* visitor) const {
    visitor->Trace(repetitions_);
    CSSValueList::TraceAfterDispatch(visitor);
  }

 private:
  wtf_size_t ClampRepetitions(wtf_size_t repetitions) const;

  const Member<CSSPrimitiveValue> repetitions_;
  std::optional<wtf_size_t> extra_clamp_;
};

}  // namespace cssvalue

template <>
struct DowncastTraits<cssvalue::CSSGridIntegerRepeatValue> {
  static bool AllowFrom(const CSSValue& value) {
    return value.IsGridIntegerRepeatValue();
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_GRID_INTEGER_REPEAT_VALUE_H_
