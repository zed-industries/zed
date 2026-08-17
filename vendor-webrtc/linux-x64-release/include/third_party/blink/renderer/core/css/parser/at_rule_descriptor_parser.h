// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_PARSER_AT_RULE_DESCRIPTOR_PARSER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_PARSER_AT_RULE_DESCRIPTOR_PARSER_H_

#include "third_party/blink/renderer/core/css/css_property_value.h"
#include "third_party/blink/renderer/core/css/parser/at_rule_descriptors.h"
#include "third_party/blink/renderer/core/css/style_rule.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {

class CSSParserContext;
class CSSParserTokenStream;
class CSSValue;

class AtRuleDescriptorParser {
  STATIC_ONLY(AtRuleDescriptorParser);

 public:
  static bool ParseDescriptorValue(StyleRule::RuleType,
                                   AtRuleDescriptorID,
                                   const AtomicString& variable_name,
                                   CSSParserTokenStream&,
                                   const CSSParserContext&,
                                   HeapVector<CSSPropertyValue, 64>&);
  static CSSValue* ParseFontFaceDescriptor(AtRuleDescriptorID,
                                           CSSParserTokenStream&,
                                           const CSSParserContext&);
  static CSSValue* ParseFontFaceDescriptor(AtRuleDescriptorID,
                                           StringView,
                                           const CSSParserContext&);
  static CSSValue* ParseFontFaceDeclaration(CSSParserTokenStream&,
                                            const CSSParserContext&);
  static CSSValue* ParseAtPropertyDescriptor(AtRuleDescriptorID,
                                             CSSParserTokenStream&,
                                             const CSSParserContext&);
  static CSSValue* ParseAtCounterStyleDescriptor(AtRuleDescriptorID,
                                                 CSSParserTokenStream&,
                                                 const CSSParserContext&);
  static CSSValue* ParseAtFontPaletteValuesDescriptor(AtRuleDescriptorID,
                                                      CSSParserTokenStream&,
                                                      const CSSParserContext&);
  static CSSValue* ParseAtViewTransitionDescriptor(AtRuleDescriptorID,
                                                   CSSParserTokenStream&,
                                                   const CSSParserContext&);
  static CSSValue* ParseAtFunctionDescriptor(AtRuleDescriptorID,
                                             CSSParserTokenStream&,
                                             const CSSParserContext&);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_PARSER_AT_RULE_DESCRIPTOR_PARSER_H_
