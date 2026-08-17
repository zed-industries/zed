// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_UNPARSED_DECLARATION_VALUE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_UNPARSED_DECLARATION_VALUE_H_

#include "base/memory/scoped_refptr.h"
#include "third_party/blink/renderer/core/css/css_value.h"
#include "third_party/blink/renderer/core/css/css_variable_data.h"
#include "third_party/blink/renderer/core/css/parser/css_parser_context.h"
#include "third_party/blink/renderer/platform/wtf/casting.h"

namespace blink {

// This represents a CSS declaration value that we haven't fully parsed into
// a CSSValue, but left basically as untyped text (potentially for further
// parsing later). This can happen in one out of two cases:
//
//  - A CSS longhand property contains at least one variable reference, e.g.:
//    color: var(--x)
//  - A custom property with or without variable references, e.g.:
//    --foo: abc;
//
// The former will eventually be parsed in StyleCascade's apply step,
// when we know the correct value of all variables. The latter may never
// be further substituted at all.
//
// CSS shorthand properties containing at least one variable reference are
// represented by either CSSPendingSubstitutionValue (Blink) or CSSUnparsedValue
// (Typed CSSOM), which wraps this.
//
// https://drafts.csswg.org/css-syntax-3/#typedef-declaration-value
// https://drafts.csswg.org/css-variables/#defining-variables
class CORE_EXPORT CSSUnparsedDeclarationValue final : public CSSValue {
 public:
  explicit CSSUnparsedDeclarationValue(CSSVariableData* data)
      : CSSValue(kUnparsedDeclarationClass), data_(data) {}

  CSSUnparsedDeclarationValue(CSSVariableData* data,
                              const CSSParserContext* context)
      : CSSValue(kUnparsedDeclarationClass),
        parser_context_(context),
        data_(data) {}

  CSSVariableData* VariableDataValue() const { return data_.Get(); }
  const CSSParserContext* ParserContext() const {
    // TODO(crbug.com/985028): CSSUnparsedDeclarationValue should always have
    // a CSSParserContext.
    return parser_context_.Get();
  }

  bool Equals(const CSSUnparsedDeclarationValue& other) const {
    return base::ValuesEquivalent(data_, other.data_);
  }
  String CustomCSSText() const;
  unsigned CustomHash() const;

  void TraceAfterDispatch(blink::Visitor*) const;

 private:
  // The parser context is used to resolve relative URLs, as described in:
  // https://drafts.css-houdini.org/css-properties-values-api-1/#relative-urls
  const Member<const CSSParserContext> parser_context_;
  Member<CSSVariableData> data_;
};

template <>
struct DowncastTraits<CSSUnparsedDeclarationValue> {
  static bool AllowFrom(const CSSValue& value) {
    return value.IsUnparsedDeclaration();
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_UNPARSED_DECLARATION_VALUE_H_
