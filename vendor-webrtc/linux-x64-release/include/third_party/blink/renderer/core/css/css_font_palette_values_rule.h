// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_FONT_PALETTE_VALUES_RULE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_FONT_PALETTE_VALUES_RULE_H_

#include "third_party/blink/renderer/core/css/css_rule.h"
#include "third_party/blink/renderer/core/css/parser/at_rule_descriptors.h"
#include "third_party/blink/renderer/core/css/style_rule_css_style_declaration.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "third_party/blink/renderer/platform/wtf/casting.h"

namespace blink {

class StyleRuleFontPaletteValues;

class CSSFontPaletteValuesRule final : public CSSRule {
  DEFINE_WRAPPERTYPEINFO();

 public:
  CSSFontPaletteValuesRule(StyleRuleFontPaletteValues*, CSSStyleSheet*);
  ~CSSFontPaletteValuesRule() override;

  String cssText() const override;
  void Reattach(StyleRuleBase*) override;

  String name() const;
  String fontFamily() const;
  String basePalette() const;
  String overrideColors() const;
  StyleRuleFontPaletteValues* FontPaletteValues() const;
  CSSStyleDeclaration* Style();

  void Trace(Visitor*) const override;

 private:
  CSSRule::Type GetType() const override { return kFontPaletteValuesRule; }

  Member<StyleRuleFontPaletteValues> font_palette_values_rule_;
  Member<StyleRuleCSSStyleDeclaration> font_palette_values_cssom_wrapper_;
};

template <>
struct DowncastTraits<CSSFontPaletteValuesRule> {
  static bool AllowFrom(const CSSRule& rule) {
    return rule.GetType() == CSSRule::kFontPaletteValuesRule;
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_FONT_PALETTE_VALUES_RULE_H_
