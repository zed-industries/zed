// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_PARSER_CSS_SUPPORTS_PARSER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_PARSER_CSS_SUPPORTS_PARSER_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

class CSSParserImpl;
class CSSParserTokenStream;

class CORE_EXPORT CSSSupportsParser {
  STACK_ALLOCATED();

 public:
  enum class Result {
    // The supports condition evaluated to 'false'. The child rules of the
    // the @supports rule should have no effect.
    kUnsupported,
    // The supports condition evaluated to 'true'. The child rules of the
    // the @supports rule *should* have an effect.
    kSupported,
    // Used when the grammar of @supports was violated. If this is returned,
    // the entire @supports rule (including the child rules) should
    // be dropped.
    kParseFailure
  };

  static Result ConsumeSupportsCondition(CSSParserTokenStream&, CSSParserImpl&);

 private:
  friend class CSSSupportsParserTest;

  explicit CSSSupportsParser(CSSParserImpl& parser) : parser_(parser) {}

  // Parsing functions follow, as defined by:
  // https://drafts.csswg.org/css-conditional-3/#typedef-supports-condition

  // <supports-condition> = not <supports-in-parens>
  //                   | <supports-in-parens> [ and <supports-in-parens> ]*
  //                   | <supports-in-parens> [ or <supports-in-parens> ]*
  Result ConsumeSupportsCondition(CSSParserTokenStream&);

  // <supports-in-parens> = ( <supports-condition> )
  //                    | <supports-feature>
  //                    | <general-enclosed>
  Result ConsumeSupportsInParens(CSSParserTokenStream&);

  // <supports-feature> = <supports-selector-fn> | <supports-decl>
  bool ConsumeSupportsFeature(CSSParserTokenStream&);

  // <supports-selector-fn> = selector( <complex-selector> )
  bool ConsumeSupportsSelectorFn(CSSParserTokenStream&);

  // <supports-font-tech-fn> = font-tech( <font-tech> )
  bool ConsumeFontTechFn(CSSParserTokenStream& stream);

  // <supports-font-format-fn> = font-format( <font-format> )
  bool ConsumeFontFormatFn(CSSParserTokenStream& stream);

  // <supports-at-rule-fn> = at-rule( <at-rule> [ ; <descriptor> : <value> ]? )
  bool ConsumeAtRuleFn(CSSParserTokenStream& stream);

  // <supports-decl> = ( <declaration> )
  bool ConsumeSupportsDecl(CSSParserTokenStream&);

  // <general-enclosed> = [ <function-token> <any-value>? ) ]
  //                  | ( <any-value>? )
  bool ConsumeGeneralEnclosed(CSSParserTokenStream&);

  // This is an internal feature which is not web-exposed.
  bool ConsumeBlinkFeatureFn(CSSParserTokenStream&);

  CSSParserImpl& parser_;
};

inline CSSSupportsParser::Result operator!(CSSSupportsParser::Result result) {
  using Result = CSSSupportsParser::Result;
  if (result == Result::kUnsupported) {
    return Result::kSupported;
  }
  if (result == Result::kSupported) {
    return Result::kUnsupported;
  }
  return result;
}

inline CSSSupportsParser::Result operator&(CSSSupportsParser::Result a,
                                           CSSSupportsParser::Result b) {
  using Result = CSSSupportsParser::Result;
  if (a == Result::kParseFailure || b == Result::kParseFailure) {
    return Result::kParseFailure;
  }
  if (a != Result::kSupported || b != Result::kSupported) {
    return Result::kUnsupported;
  }
  return Result::kSupported;
}

inline CSSSupportsParser::Result operator|(CSSSupportsParser::Result a,
                                           CSSSupportsParser::Result b) {
  using Result = CSSSupportsParser::Result;
  if (a == Result::kParseFailure || b == Result::kParseFailure) {
    return Result::kParseFailure;
  }
  if (a == Result::kSupported || b == Result::kSupported) {
    return Result::kSupported;
  }
  return Result::kUnsupported;
}

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_PARSER_CSS_SUPPORTS_PARSER_H_
