// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_PARSER_SIZES_MATH_FUNCTION_PARSER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_PARSER_SIZES_MATH_FUNCTION_PARSER_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/css/css_math_operator.h"
#include "third_party/blink/renderer/core/css/media_values.h"
#include "third_party/blink/renderer/core/css/parser/css_parser_token.h"
#include "third_party/blink/renderer/core/css/parser/css_parser_token_stream.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

struct SizesMathValue {
  DISALLOW_NEW();
  double value = 0;
  bool is_length = false;
  CSSMathOperator operation = CSSMathOperator::kInvalid;

  SizesMathValue() = default;

  SizesMathValue(double numeric_value, bool length)
      : value(numeric_value), is_length(length) {}

  explicit SizesMathValue(CSSMathOperator op) : operation(op) {}
};

class CORE_EXPORT SizesMathFunctionParser {
  STACK_ALLOCATED();

 public:
  SizesMathFunctionParser(CSSParserTokenStream&, MediaValues*);

  float Result() const;
  bool IsValid() const { return is_valid_; }

 private:
  bool CalcToReversePolishNotation(CSSParserTokenStream&);
  bool ConsumeCalc(CSSParserTokenStream&, Vector<CSSParserToken>& stack);
  bool ConsumeBlockContent(CSSParserTokenStream&,
                           Vector<CSSParserToken>& stack);
  bool Calculate();
  void AppendNumber(const CSSParserToken&);
  bool AppendLength(const CSSParserToken&);
  bool HandleComma(Vector<CSSParserToken>& stack, const CSSParserToken&);
  bool HandleRightParenthesis(Vector<CSSParserToken>& stack);
  bool HandleOperator(Vector<CSSParserToken>& stack, const CSSParserToken&);
  void AppendOperator(const CSSParserToken&);

  Vector<SizesMathValue> value_list_;
  MediaValues* media_values_;
  bool is_valid_;
  float result_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_PARSER_SIZES_MATH_FUNCTION_PARSER_H_
