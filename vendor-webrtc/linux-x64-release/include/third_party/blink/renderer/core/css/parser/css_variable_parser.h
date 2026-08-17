// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_PARSER_CSS_VARIABLE_PARSER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_PARSER_CSS_VARIABLE_PARSER_H_

#include "base/memory/scoped_refptr.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/css/parser/css_parser_token.h"
#include "third_party/blink/renderer/core/execution_context/execution_context.h"
#include "third_party/blink/renderer/platform/wtf/hash_set.h"
#include "third_party/blink/renderer/platform/wtf/text/atomic_string.h"
#include "third_party/blink/renderer/platform/wtf/text/string_view.h"

namespace blink {

class CSSParserContext;
class CSSParserTokenStream;
class CSSUnparsedDeclarationValue;
class CSSVariableData;

class CORE_EXPORT CSSVariableParser {
 public:
  static const CSSValue* ParseDeclarationIncludingCSSWide(
      CSSParserTokenStream&,
      bool is_animation_tainted,
      const CSSParserContext&);
  static CSSUnparsedDeclarationValue* ParseDeclarationValue(
      StringView,
      bool is_animation_tainted,
      const CSSParserContext&);

  // Consume a declaration without trying to parse it as any specific
  // property. This is mostly useful for either custom property declarations,
  // or for standard properties referencing custom properties
  // (var(), or similarly env() etc.).
  //
  // Returns nullptr on failure, such as a stray top-level ! or },
  // or if “must_contain_variable_reference” (useful for standard
  // properties), “restricted_value” or “allow_important_annotation”
  // is violated. If so, the parser is left at an indeterminate place,
  // but with the same block level as it started. On success, returns
  // a CSSVariableData containing the original text for the property,
  // with leading and trailing whitespace and comments removed,
  // plus “!important” (if existing) stripped. The parser will be
  // at the end of the declaration, i.e., typically at a semicolon.
  //
  // A value for a standard property (restricted_value=true) has
  // the following restriction: it can not contain braces unless
  // it's the whole value [1]. This function makes use of that
  // restriction to early-out of the streaming tokenizer as
  // soon as possible. (This used to be important to avoid a O(n²),
  // but it is not anymore, as failure of this function is no longer
  // a common case in the happy parsing path.) If restricted_value=false
  // (as is the case with custom properties and descriptors), the function
  // will simply consume until AtEnd(), unless an error is encountered.
  //
  // [1] https://github.com/w3c/csswg-drafts/issues/9317
  static CSSVariableData* ConsumeUnparsedDeclaration(
      CSSParserTokenStream& stream,
      bool allow_important_annotation,
      bool is_animation_tainted,
      bool must_contain_variable_reference,
      bool restricted_value,
      bool comma_ends_declaration,
      bool& important,
      const CSSParserContext& context);

  // Custom properties registered with universal syntax [1] are parsed with
  // this function.
  //
  // https://drafts.css-houdini.org/css-properties-values-api-1/#universal-syntax-definition
  static CSSUnparsedDeclarationValue* ParseUniversalSyntaxValue(
      StringView,
      const CSSParserContext&,
      bool is_animation_tainted);

  static bool IsValidVariableName(const CSSParserToken&);
  static bool IsValidVariableName(StringView);

  // True if the stream starts with <dashed-ident> <whitespace>? <colon>.
  // This is primarily used to implement the "bad declaration" handling
  // in https://drafts.csswg.org/css-syntax/#consume-qualified-rule.
  //
  // Whitespace must be consumed before calling this, otherwise the function
  // returns false.
  static bool StartsCustomPropertyDeclaration(CSSParserTokenStream&);

  // NOTE: We have to strip both leading and trailing whitespace (and comments)
  // from values as per spec, but we assume the tokenizer has already done the
  // leading ones for us; see comment on CSSPropertyParser::ParseValue().
  static StringView StripTrailingWhitespaceAndComments(StringView);

  // Collect any instances of <dashed-function> anywhere in the stream.
  //
  // https://drafts.csswg.org/css-mixins-1/#typedef-dashed-function
  static void CollectDashedFunctions(CSSParserTokenStream&,
                                     HashSet<AtomicString>& result);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_PARSER_CSS_VARIABLE_PARSER_H_
