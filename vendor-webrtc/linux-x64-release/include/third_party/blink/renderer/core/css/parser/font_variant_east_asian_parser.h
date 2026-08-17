// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_PARSER_FONT_VARIANT_EAST_ASIAN_PARSER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_PARSER_FONT_VARIANT_EAST_ASIAN_PARSER_H_

#include "third_party/blink/renderer/core/css/css_value_list.h"
#include "third_party/blink/renderer/core/css/properties/css_parsing_utils.h"

namespace blink {

class FontVariantEastAsianParser {
  STACK_ALLOCATED();

 public:
  FontVariantEastAsianParser()
      : east_asian_form_value_(nullptr),
        east_asian_width_value_(nullptr),
        ruby_value_(nullptr) {}

  enum class ParseResult { kConsumedValue, kDisallowedValue, kUnknownValue };

  ParseResult ConsumeEastAsian(CSSParserTokenStream& stream) {
    CSSValueID value_id = stream.Peek().Id();
    switch (value_id) {
      case CSSValueID::kJis78:
      case CSSValueID::kJis83:
      case CSSValueID::kJis90:
      case CSSValueID::kJis04:
      case CSSValueID::kSimplified:
      case CSSValueID::kTraditional:
        if (east_asian_form_value_) {
          return ParseResult::kDisallowedValue;
        }
        east_asian_form_value_ = css_parsing_utils::ConsumeIdent(stream);
        return ParseResult::kConsumedValue;
      case CSSValueID::kFullWidth:
      case CSSValueID::kProportionalWidth:
        if (east_asian_width_value_) {
          return ParseResult::kDisallowedValue;
        }
        east_asian_width_value_ = css_parsing_utils::ConsumeIdent(stream);
        return ParseResult::kConsumedValue;
      case CSSValueID::kRuby:
        if (ruby_value_) {
          return ParseResult::kDisallowedValue;
        }
        ruby_value_ = css_parsing_utils::ConsumeIdent(stream);
        return ParseResult::kConsumedValue;
      default:
        return ParseResult::kUnknownValue;
    }
  }

  CSSValue* FinalizeValue() {
    CSSValueList* result = CSSValueList::CreateSpaceSeparated();
    if (east_asian_form_value_) {
      result->Append(*east_asian_form_value_);
      east_asian_form_value_ = nullptr;
    }
    if (east_asian_width_value_) {
      result->Append(*east_asian_width_value_);
      east_asian_width_value_ = nullptr;
    }
    if (ruby_value_) {
      result->Append(*ruby_value_);
      ruby_value_ = nullptr;
    }

    if (!result->length()) {
      return CSSIdentifierValue::Create(CSSValueID::kNormal);
    }
    return result;
  }

 private:
  CSSIdentifierValue* east_asian_form_value_;
  CSSIdentifierValue* east_asian_width_value_;
  CSSIdentifierValue* ruby_value_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_PARSER_FONT_VARIANT_EAST_ASIAN_PARSER_H_
