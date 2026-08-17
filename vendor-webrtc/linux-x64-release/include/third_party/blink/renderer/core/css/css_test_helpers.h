// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_TEST_HELPERS_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_TEST_HELPERS_H_

#include <optional>

#include "base/memory/scoped_refptr.h"
#include "third_party/blink/renderer/core/css/css_selector.h"
#include "third_party/blink/renderer/core/css/css_selector_list.h"
#include "third_party/blink/renderer/core/css/rule_set.h"
#include "third_party/blink/renderer/core/testing/null_execution_context.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

class Document;
class CSSStyleSheet;
class CSSVariableData;
class CSSValue;
class CSSProperty;
class PropertyRegistration;

namespace css_test_helpers {

// Example usage:
//
// css_test_helpers::TestStyleSheet sheet;
// sheet.addCSSRule("body { color: red} #a { position: absolute }");
// RuleSet& ruleSet = sheet.ruleSet();
// ... examine RuleSet to find the rule and test properties on it.
class TestStyleSheet {
  DISALLOW_NEW();

 public:
  TestStyleSheet();
  ~TestStyleSheet();

  const Document& GetDocument() { return *document_; }

  void AddCSSRules(const String& rule_text, bool is_empty_sheet = false);
  RuleSet& GetRuleSet();
  CSSRuleList* CssRules();

 private:
  ScopedNullExecutionContext execution_context_;
  Persistent<Document> document_;
  Persistent<CSSStyleSheet> style_sheet_;
};

CSSStyleSheet* CreateStyleSheet(Document& document);
RuleSet* CreateRuleSet(Document& document, String text);

// Create a PropertyRegistration with the given name. An initial value must
// be provided when the syntax is not "*".
PropertyRegistration* CreatePropertyRegistration(
    const String& name,
    String syntax = "*",
    const CSSValue* initial_value = nullptr,
    bool is_inherited = false);

// Create a non-inherited PropertyRegistration with syntax <length>, and the
// given value in pixels as the initial value.
PropertyRegistration* CreateLengthRegistration(const String& name, int px);

void RegisterProperty(Document& document,
                      const String& name,
                      const String& syntax,
                      const std::optional<String>& initial_value,
                      bool is_inherited);
void RegisterProperty(Document& document,
                      const String& name,
                      const String& syntax,
                      const std::optional<String>& initial_value,
                      bool is_inherited,
                      ExceptionState&);
void DeclareProperty(Document& document,
                     const String& name,
                     const String& syntax,
                     const std::optional<String>& initial_value,
                     bool is_inherited);

CSSVariableData* CreateVariableData(String);
const CSSValue* CreateCustomIdent(const char*);
const CSSValue* ParseLonghand(Document& document,
                              const CSSProperty&,
                              const String& value);
const CSSPropertyValueSet* ParseDeclarationBlock(
    const String& block_text,
    CSSParserMode mode = kHTMLStandardMode);
StyleRuleBase* ParseRule(Document& document, String text);
StyleRuleBase* ParseNestedRule(Document& document,
                               String text,
                               CSSNestingType,
                               StyleRule* parent_rule_for_nesting);

// Parse a value according to syntax defined by:
// https://drafts.css-houdini.org/css-properties-values-api-1/#syntax-strings
const CSSValue* ParseValue(Document&, String syntax, String value);

CSSSelectorList* ParseSelectorList(const String&);
// Parse the selector as if nested with the given CSSNestingType, using
// the specified StyleRule to resolve either the parent selector "&"
// (for kNesting), or the :scope pseudo-class (for kScope).
CSSSelectorList* ParseSelectorList(const String&,
                                   CSSNestingType,
                                   const StyleRule* parent_rule_for_nesting);

}  // namespace css_test_helpers
}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_TEST_HELPERS_H_
