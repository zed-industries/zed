// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_ANCHOR_QUERY_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_ANCHOR_QUERY_H_

#include <variant>

#include "base/check_op.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/css/css_anchor_query_enums.h"
#include "third_party/blink/renderer/core/style/anchor_specifier_value.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

// The input to AnchorEvaluator::Evaluate.
//
// It represents either an anchor() function, or an anchor-size() function.
//
// https://drafts.csswg.org/css-anchor-position-1/#anchor-pos
// https://drafts.csswg.org/css-anchor-position-1/#anchor-size-fn
class CORE_EXPORT AnchorQuery {
  DISALLOW_NEW();

 public:
  AnchorQuery(CSSAnchorQueryType query_type,
              const AnchorSpecifierValue* anchor_specifier,
              float percentage,
              std::variant<CSSAnchorValue, CSSAnchorSizeValue> value)
      : query_type_(query_type),
        anchor_specifier_(anchor_specifier),
        percentage_(percentage),
        value_(value) {
    CHECK(anchor_specifier);
  }

  CSSAnchorQueryType Type() const { return query_type_; }
  const AnchorSpecifierValue& AnchorSpecifier() const {
    return *anchor_specifier_;
  }
  CSSAnchorValue AnchorSide() const {
    DCHECK_EQ(query_type_, CSSAnchorQueryType::kAnchor);
    return std::get<CSSAnchorValue>(value_);
  }
  float AnchorSidePercentage() const {
    DCHECK_EQ(query_type_, CSSAnchorQueryType::kAnchor);
    DCHECK_EQ(AnchorSide(), CSSAnchorValue::kPercentage);
    return percentage_;
  }
  float AnchorSidePercentageOrZero() const {
    DCHECK_EQ(query_type_, CSSAnchorQueryType::kAnchor);
    return AnchorSide() == CSSAnchorValue::kPercentage ? percentage_ : 0;
  }
  CSSAnchorSizeValue AnchorSize() const {
    DCHECK_EQ(query_type_, CSSAnchorQueryType::kAnchorSize);
    return std::get<CSSAnchorSizeValue>(value_);
  }

  bool operator==(const AnchorQuery& other) const;
  bool operator!=(const AnchorQuery& other) const { return !operator==(other); }
  void Trace(Visitor*) const;

 private:
  CSSAnchorQueryType query_type_;
  Member<const AnchorSpecifierValue> anchor_specifier_;
  float percentage_;
  std::variant<CSSAnchorValue, CSSAnchorSizeValue> value_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_ANCHOR_QUERY_H_
