// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_PARSER_FONT_VARIANT_ALTERNATES_PARSER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_PARSER_FONT_VARIANT_ALTERNATES_PARSER_H_

#include "third_party/blink/renderer/core/css/css_alternate_value.h"
#include "third_party/blink/renderer/core/css/css_value_list.h"

namespace blink {

class CSSParserContext;
class CSSIdentifierValue;
class CSSParserTokenStream;

class FontVariantAlternatesParser {
  STACK_ALLOCATED();

 public:
  FontVariantAlternatesParser();

  enum class ParseResult { kConsumedValue, kDisallowedValue, kUnknownValue };

  ParseResult ConsumeAlternates(CSSParserTokenStream& stream,
                                const CSSParserContext& context);

  CSSValue* FinalizeValue();

 private:
  bool ConsumeAlternate(CSSParserTokenStream& stream,
                        const CSSParserContext& context);

  bool ConsumeHistoricalForms(CSSParserTokenStream& stream);

  CSSValueList* alternates_list_;
  cssvalue::CSSAlternateValue* stylistic_ = nullptr;
  CSSIdentifierValue* historical_forms_ = nullptr;
  cssvalue::CSSAlternateValue* styleset_ = nullptr;
  cssvalue::CSSAlternateValue* character_variant_ = nullptr;
  cssvalue::CSSAlternateValue* swash_ = nullptr;
  cssvalue::CSSAlternateValue* ornaments_ = nullptr;
  cssvalue::CSSAlternateValue* annotation_ = nullptr;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_PARSER_FONT_VARIANT_ALTERNATES_PARSER_H_
