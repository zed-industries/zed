// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_PENDING_SUBSTITUTION_VALUE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_PENDING_SUBSTITUTION_VALUE_H_

#include "third_party/blink/renderer/core/css/css_property_names.h"
#include "third_party/blink/renderer/core/css/css_unparsed_declaration_value.h"
#include "third_party/blink/renderer/core/css/css_value.h"
#include "third_party/blink/renderer/platform/wtf/casting.h"

namespace blink {
namespace cssvalue {

class CSSPendingSubstitutionValue : public CSSValue {
 public:
  CSSPendingSubstitutionValue(CSSPropertyID shorthand_property_id,
                              CSSUnparsedDeclarationValue* shorthand_value)
      : CSSValue(kPendingSubstitutionValueClass),
        shorthand_property_id_(shorthand_property_id),
        shorthand_value_(shorthand_value) {}

  CSSUnparsedDeclarationValue* ShorthandValue() const {
    return shorthand_value_.Get();
  }

  CSSPropertyID ShorthandPropertyId() const { return shorthand_property_id_; }

  bool Equals(const CSSPendingSubstitutionValue& other) const {
    return shorthand_value_ == other.shorthand_value_;
  }
  String CustomCSSText() const;

  void TraceAfterDispatch(blink::Visitor*) const;

 private:
  CSSPropertyID shorthand_property_id_;
  Member<CSSUnparsedDeclarationValue> shorthand_value_;
};

}  // namespace cssvalue

template <>
struct DowncastTraits<cssvalue::CSSPendingSubstitutionValue> {
  static bool AllowFrom(const CSSValue& value) {
    return value.IsPendingSubstitutionValue();
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_PENDING_SUBSTITUTION_VALUE_H_
