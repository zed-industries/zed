// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_PARSER_FONT_VARIANT_LIGATURES_PARSER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_PARSER_FONT_VARIANT_LIGATURES_PARSER_H_

#include "third_party/blink/renderer/core/css/css_identifier_value.h"
#include "third_party/blink/renderer/core/css/css_value_list.h"
#include "third_party/blink/renderer/core/css/properties/css_parsing_utils.h"

namespace blink {

class FontVariantLigaturesParser {
  STACK_ALLOCATED();

 public:
  FontVariantLigaturesParser()
      : saw_common_ligatures_value_(false),
        saw_discretionary_ligatures_value_(false),
        saw_historical_ligatures_value_(false),
        saw_contextual_ligatures_value_(false),
        result_(CSSValueList::CreateSpaceSeparated()) {}

  enum class ParseResult { kConsumedValue, kDisallowedValue, kUnknownValue };

  ParseResult ConsumeLigature(CSSParserTokenStream& stream) {
    CSSValueID value_id = stream.Peek().Id();
    switch (value_id) {
      case CSSValueID::kNoCommonLigatures:
      case CSSValueID::kCommonLigatures:
        if (saw_common_ligatures_value_) {
          return ParseResult::kDisallowedValue;
        }
        saw_common_ligatures_value_ = true;
        break;
      case CSSValueID::kNoDiscretionaryLigatures:
      case CSSValueID::kDiscretionaryLigatures:
        if (saw_discretionary_ligatures_value_) {
          return ParseResult::kDisallowedValue;
        }
        saw_discretionary_ligatures_value_ = true;
        break;
      case CSSValueID::kNoHistoricalLigatures:
      case CSSValueID::kHistoricalLigatures:
        if (saw_historical_ligatures_value_) {
          return ParseResult::kDisallowedValue;
        }
        saw_historical_ligatures_value_ = true;
        break;
      case CSSValueID::kNoContextual:
      case CSSValueID::kContextual:
        if (saw_contextual_ligatures_value_) {
          return ParseResult::kDisallowedValue;
        }
        saw_contextual_ligatures_value_ = true;
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
  bool saw_common_ligatures_value_;
  bool saw_discretionary_ligatures_value_;
  bool saw_historical_ligatures_value_;
  bool saw_contextual_ligatures_value_;
  CSSValueList* result_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_PARSER_FONT_VARIANT_LIGATURES_PARSER_H_
