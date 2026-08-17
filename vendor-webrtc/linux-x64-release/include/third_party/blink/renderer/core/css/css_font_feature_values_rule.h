// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_FONT_FEATURE_VALUES_RULE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_FONT_FEATURE_VALUES_RULE_H_

#include "third_party/blink/renderer/core/css/css_font_feature_values_map.h"
#include "third_party/blink/renderer/core/css/css_rule.h"
#include "third_party/blink/renderer/core/css/style_rule_font_feature_values.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "third_party/blink/renderer/platform/wtf/casting.h"

namespace blink {

class StyleRuleFontFeatureValues;

class CSSFontFeatureValuesRule final : public CSSRule {
  DEFINE_WRAPPERTYPEINFO();

 public:
  CSSFontFeatureValuesRule(StyleRuleFontFeatureValues*, CSSStyleSheet*);
  ~CSSFontFeatureValuesRule() override;

  void setFontFamily(const String& font_family);

  String fontFamily();

  CSSFontFeatureValuesMap* annotation();
  CSSFontFeatureValuesMap* ornaments();
  CSSFontFeatureValuesMap* stylistic();
  CSSFontFeatureValuesMap* swash();
  CSSFontFeatureValuesMap* characterVariant();
  CSSFontFeatureValuesMap* styleset();

  String cssText() const override;
  void Reattach(StyleRuleBase*) override;

  void Trace(Visitor*) const override;

 private:
  CSSRule::Type GetType() const override { return kFontFeatureValuesRule; }

  Member<StyleRuleFontFeatureValues> font_feature_values_rule_;
};

template <>
struct DowncastTraits<CSSFontFeatureValuesRule> {
  static bool AllowFrom(const CSSRule& rule) {
    return rule.GetType() == CSSRule::kFontFeatureValuesRule;
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_FONT_FEATURE_VALUES_RULE_H_
