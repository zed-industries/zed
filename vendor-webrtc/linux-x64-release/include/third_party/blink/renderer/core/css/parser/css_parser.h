// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_PARSER_CSS_PARSER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_PARSER_CSS_PARSER_H_

#include <memory>
#include "third_party/blink/public/mojom/frame/color_scheme.mojom-blink-forward.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/css/css_property_names.h"
#include "third_party/blink/renderer/core/css/css_property_value_set.h"
#include "third_party/blink/renderer/core/css/parser/css_nesting_type.h"
#include "third_party/blink/renderer/core/css/parser/css_parser_context.h"
#include "third_party/blink/renderer/core/css/parser/css_tokenizer.h"
#include "third_party/blink/renderer/core/css/style_rule_keyframe.h"

namespace ui {
class ColorProvider;
}  // namespace ui

namespace blink {

class Color;
class CSSParserObserver;
class CSSSelector;
class CSSSelectorList;
class Element;
class ExecutionContext;
class ImmutableCSSPropertyValueSet;
class StyleRule;
class StyleRuleBase;
class StyleRuleKeyframe;
class StyleSheetContents;
class CSSValue;
class CSSPrimitiveValue;
enum class ParseSheetResult;
enum class SecureContextMode;

// This class serves as the public API for the css/parser subsystem
class CORE_EXPORT CSSParser {
  STATIC_ONLY(CSSParser);

 public:
  // As well as regular rules, allows @import and @namespace but not @charset
  static StyleRuleBase* ParseRule(const CSSParserContext* context,
                                  StyleSheetContents* style_sheet,
                                  CSSNestingType,
                                  StyleRule* parent_rule_for_nesting,
                                  const String& rule);

  static ParseSheetResult ParseSheet(
      const CSSParserContext*,
      StyleSheetContents*,
      const String&,
      CSSDeferPropertyParsing defer_property_parsing =
          CSSDeferPropertyParsing::kNo,
      bool allow_import_rules = true);
  // See CSSSelectorParser for lifetime of the returned value.
  static base::span<CSSSelector> ParseSelector(
      const CSSParserContext*,
      CSSNestingType,
      StyleRule* parent_rule_for_nesting,
      StyleSheetContents*,
      const String&,
      HeapVector<CSSSelector>& arena);
  static CSSSelectorList* ParsePageSelector(const CSSParserContext&,
                                            StyleSheetContents*,
                                            const String&);
  static StyleRuleBase* ParseMarginRule(const CSSParserContext*,
                                        StyleSheetContents*,
                                        const String&);
  static bool ParseDeclarationList(const CSSParserContext*,
                                   MutableCSSPropertyValueSet*,
                                   const String&);

  static StyleRuleBase* ParseNestedDeclarationsRule(
      const CSSParserContext*,
      CSSNestingType,
      StyleRule* parent_rule_for_nesting,
      StringView);

  static MutableCSSPropertyValueSet::SetResult ParseValue(
      MutableCSSPropertyValueSet*,
      CSSPropertyID unresolved_property,
      StringView value,
      bool important,
      const ExecutionContext* execution_context = nullptr);
  static MutableCSSPropertyValueSet::SetResult ParseValue(
      MutableCSSPropertyValueSet*,
      CSSPropertyID unresolved_property,
      StringView value,
      bool important,
      SecureContextMode,
      StyleSheetContents*,
      const ExecutionContext* execution_context = nullptr);

  // Appends to a vector instead of to a property value set (so no deduplication
  // etc.). Also note that this takes in the resolved property; there's no
  // reason for internal code to ever use an alias here. Returns the number of
  // properties that were added (0 for parse error).
  static unsigned ParseForPresentationStyle(
      HeapVector<CSSPropertyValue, 8>& result,
      CSSPropertyID resolved_property,
      StringView value,
      CSSParserMode parser_mode,
      StyleSheetContents* context_sheet,  // Used for URL references.
      const ExecutionContext* execution_context);

  static MutableCSSPropertyValueSet::SetResult ParseValueForCustomProperty(
      MutableCSSPropertyValueSet*,
      const AtomicString& property_name,
      StringView value,
      bool important,
      SecureContextMode,
      StyleSheetContents*,
      bool is_animation_tainted);

  // This is for non-shorthands only
  static const CSSValue* ParseSingleValue(CSSPropertyID,
                                          const String&,
                                          const CSSParserContext*);

  static const CSSValue* ParseFontFaceDescriptor(CSSPropertyID,
                                                 const String&,
                                                 const CSSParserContext*);

  static ImmutableCSSPropertyValueSet* ParseInlineStyleDeclaration(
      const String&,
      Element*);
  static ImmutableCSSPropertyValueSet* ParseInlineStyleDeclaration(
      const String&,
      CSSParserMode,
      SecureContextMode,
      const Document*);

  static std::unique_ptr<Vector<KeyframeOffset>> ParseKeyframeKeyList(
      const CSSParserContext*,
      const String&);
  static StyleRuleKeyframe* ParseKeyframeRule(const CSSParserContext*,
                                              const String&);
  static String ParseCustomPropertyName(const String&);

  static bool ParseSupportsCondition(const String&, const ExecutionContext*);

  // The color will only be changed when string contains a valid CSS color, so
  // callers can set it to a default color and ignore the boolean result.
  static bool ParseColor(Color&, const String&, bool strict = true);
  static bool ParseSystemColor(Color&,
                               const String&,
                               mojom::blink::ColorScheme color_scheme,
                               const ui::ColorProvider* color_provider,
                               bool is_in_web_app_scope);

  static void ParseSheetForInspector(const CSSParserContext*,
                                     StyleSheetContents*,
                                     const String&,
                                     CSSParserObserver&);
  static void ParseDeclarationListForInspector(const CSSParserContext*,
                                               const String&,
                                               CSSParserObserver&);

  static CSSPrimitiveValue* ParseLengthPercentage(
      const String&,
      const CSSParserContext*,
      CSSPrimitiveValue::ValueRange);

  // https://html.spec.whatwg.org/multipage/canvas.html#dom-context-2d-font
  // https://drafts.csswg.org/css-font-loading/#find-the-matching-font-faces
  static MutableCSSPropertyValueSet* ParseFont(const String&,
                                               const ExecutionContext*);

 private:
  static MutableCSSPropertyValueSet::SetResult ParseValue(
      MutableCSSPropertyValueSet*,
      CSSPropertyID unresolved_property,
      StringView,
      bool important,
      const CSSParserContext*);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_PARSER_CSS_PARSER_H_
