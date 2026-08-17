// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_FLIP_REVERT_VALUE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_FLIP_REVERT_VALUE_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/css/css_property_names.h"
#include "third_party/blink/renderer/core/css/css_value.h"
#include "third_party/blink/renderer/core/css/try_tactic_transform.h"
#include "third_party/blink/renderer/platform/wtf/casting.h"

namespace blink {

namespace cssvalue {

// This value works similarly to CSSRevertLayerValue, with two important
// differences:
//
//  - It reverts to the value of specified property, effectively allowing
//    "cross-property" reverts.
//  - The value is transformed using the specified TryTacticTransform.
//
// These two things can be used together to implement "try tactics" [1] from
// CSS Anchor Positioning.
//
// Note that this class intentionally does not inherit from CSSFunctionValue,
// even though it's serialized as a function. This is because CSSFlipRevertValue
// is only created internally - there's no way to *parse* such a value,
// and therefore we have no CSSValueID for CSSFlipRevertValue.
//
// [1]
// https://drafts.csswg.org/css-anchor-position-1/#typedef-position-try-fallbacks-try-tactic
class CORE_EXPORT CSSFlipRevertValue : public CSSValue {
 public:
  explicit CSSFlipRevertValue(CSSPropertyID property_id,
                              TryTacticTransform transform)
      : CSSValue(kFlipRevertClass),
        property_id_(property_id),
        transform_(transform) {
    CHECK_NE(property_id, CSSPropertyID::kInvalid);
    CHECK_NE(property_id, CSSPropertyID::kVariable);
  }

  CSSPropertyID PropertyID() const { return property_id_; }

  TryTacticTransform Transform() const { return transform_; }

  String CustomCSSText() const;

  bool Equals(const CSSFlipRevertValue& o) const {
    return property_id_ == o.property_id_ && transform_ == o.transform_;
  }

  void TraceAfterDispatch(blink::Visitor* visitor) const {
    CSSValue::TraceAfterDispatch(visitor);
  }

 private:
  // We revert to the value of this property, which is typically not the same
  // as the property holding this value.
  CSSPropertyID property_id_;
  // Used to transform the reverted value, see TryValueFlips::FlipValue.
  TryTacticTransform transform_;
};

}  // namespace cssvalue

template <>
struct DowncastTraits<cssvalue::CSSFlipRevertValue> {
  static bool AllowFrom(const CSSValue& value) {
    return value.IsFlipRevertValue();
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_FLIP_REVERT_VALUE_H_
