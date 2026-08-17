// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_SYNTAX_STRING_PARSER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_SYNTAX_STRING_PARSER_H_

#include <optional>

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/css/css_syntax_definition.h"
#include "third_party/blink/renderer/core/css/parser/css_tokenizer_input_stream.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

class CSSTokenizerInputStream;

// Produces a CSSSyntaxDefinition from a CSSTokenizerInputStream.
//
// https://drafts.css-houdini.org/css-properties-values-api-1/#parsing-syntax
class CORE_EXPORT CSSSyntaxStringParser {
  STACK_ALLOCATED();

 public:
  explicit CSSSyntaxStringParser(const String&);

  // https://drafts.css-houdini.org/css-properties-values-api-1/#consume-syntax-definition
  std::optional<CSSSyntaxDefinition> Parse();

 private:
  // https://drafts.css-houdini.org/css-properties-values-api-1/#consume-syntax-component
  //
  // Appends a CSSSyntaxComponent to the Vector on success.
  bool ConsumeSyntaxComponent(Vector<CSSSyntaxComponent>&);

  // https://drafts.css-houdini.org/css-properties-values-api-1/#consume-data-type-name
  //
  // Returns true if the input stream contained a supported data type name, i.e.
  // a string with a corresponding CSSSyntaxType.
  //
  // https://drafts.css-houdini.org/css-properties-values-api-1/#supported-names
  bool ConsumeDataTypeName(CSSSyntaxType&);

  // Consumes a name from the input stream, and stores the result in 'ident'.
  // Returns true if the value returned via 'ident' is not a css-wide keyword.
  bool ConsumeIdent(String& ident);

  // Consumes a '+' or '#' from the input stream (if present), and returns
  // the appropriate CSSSyntaxRepeat. CSSSyntaxRepeat::kNone is returned if
  // the next input code point is not '+' or '#'.
  CSSSyntaxRepeat ConsumeRepeatIfPresent();

  CSSTokenizerInputStream input_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_SYNTAX_STRING_PARSER_H_
