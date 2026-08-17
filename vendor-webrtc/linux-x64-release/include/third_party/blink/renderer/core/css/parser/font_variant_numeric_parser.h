// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_PARSER_FONT_VARIANT_NUMERIC_PARSER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_PARSER_FONT_VARIANT_NUMERIC_PARSER_H_

#include "third_party/blink/renderer/core/css/css_value_list.h"
#include "third_party/blink/renderer/core/css/properties/css_parsing_utils.h"

namespace blink {

class FontVariantNumericParser {
  STACK_ALLOCATED();

 public:
  FontVariantNumericParser() : result_(CSSValueList::CreateSpaceSeparated()) {}

  enum class ParseResult { kConsumedValue, kDisallowedValue, kUnknownValue };

  ParseResult ConsumeNumeric(CSSParserTokenStream& stream) {
    CSSValueID value_id = stream.Peek().Id();
    switch (value_id) {
      case CSSValueID::kLiningNums:
      case CSSValueID::kOldstyleNums:
        if (saw_numeric_figure_value_) {
          return ParseResult::kDisallowedValue;
        }
        saw_numeric_figure_value_ = true;
        break;
      case CSSValueID::kProportionalNums:
      case CSSValueID::kTabularNums:
        if (saw_numeric_spacing_value_) {
          return ParseResult::kDisallowedValue;
        }
        saw_numeric_spacing_value_ = true;
        break;
      case CSSValueID::kDiagonalFractions:
      case CSSValueID::kStackedFractions:
        if (saw_numeric_fraction_value_) {
          return ParseResult::kDisallowedValue;
        }
        saw_numeric_fraction_value_ = true;
        break;
      case CSSValueID::kOrdinal:
        if (saw_ordinal_value_) {
          return ParseResult::kDisallowedValue;
        }
        saw_ordinal_value_ = true;
        break;
      case CSSValueID::kSlashedZero:
        if (saw_slashed_zero_value_) {
          return ParseResult::kDisallowedValue;
        }
        saw_slashed_zero_value_ = true;
        break;
      default:
        return ParseResult::kUnknownValue;
    }

    result_->Append(*css_parsing_utils::ConsumeIdent(stream));
    return ParseResult::kConsumedValue;
  }

  CSSValue* FinalizeValue() {
    if (!result_->length()) {
      return CSSIdentifierValue::Create(CSSValueID::kNormal);
    }
    CSSValue* result = result_;
    result_ = nullptr;
    return result;
  }

 private:
  bool saw_numeric_figure_value_{false};
  bool saw_numeric_spacing_value_{false};
  bool saw_numeric_fraction_value_{false};
  bool saw_ordinal_value_{false};
  bool saw_slashed_zero_value_{false};
  CSSValueList* result_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_PARSER_FONT_VARIANT_NUMERIC_PARSER_H_
