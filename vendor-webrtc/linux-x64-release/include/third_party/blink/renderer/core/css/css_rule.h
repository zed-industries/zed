/*
 * (C) 1999-2003 Lars Knoll (knoll@kde.org)
 * (C) 2002-2003 Dirk Mueller (mueller@kde.org)
 * Copyright (C) 2002, 2006, 2007, 2012 Apple Inc. All rights reserved.
 * Copyright (C) 2011 Andreas Kling (kling@webkit.org)
 *
 * This library is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Library General Public
 * License as published by the Free Software Foundation; either
 * version 2 of the License, or (at your option) any later version.
 *
 * This library is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Library General Public License for more details.
 *
 * You should have received a copy of the GNU Library General Public License
 * along with this library; see the file COPYING.LIB.  If not, write to
 * the Free Software Foundation, Inc., 51 Franklin Street, Fifth Floor,
 * Boston, MA 02110-1301, USA.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_RULE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_RULE_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/css/media_query_set_owner.h"
#include "third_party/blink/renderer/core/frame/web_feature_forward.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_vector.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

class CSSParserContext;
class CSSRuleList;
class CSSStyleSheet;
class StyleRuleBase;
class MediaQuerySetOwner;
enum class SecureContextMode;
class ExecutionContext;
class ExceptionState;

class CORE_EXPORT CSSRule : public ScriptWrappable {
  DEFINE_WRAPPERTYPEINFO();

 public:
  ~CSSRule() override = default;

  enum Type {
    // Web-exposed values, see css_rule.idl:
    kStyleRule = 1,
    kCharsetRule = 2,
    kImportRule = 3,
    kMediaRule = 4,
    kFontFaceRule = 5,
    kPageRule = 6,
    kKeyframesRule = 7,
    kKeyframeRule = 8,
    kMarginRule = 9,
    kNamespaceRule = 10,
    kCounterStyleRule = 11,
    kSupportsRule = 12,
    kFontFeatureValuesRule = 14,
    // CSSOM constants are deprecated [1], and there will be no new
    // web-exposed values.
    //
    // [1] https://wiki.csswg.org/spec/cssom-constants

    // Values for internal use, not web-exposed:
    kFirstInternalRule = 16,
    // go/keep-sorted start
    kContainerRule = kFirstInternalRule,
    kFontFeatureRule,
    kFontPaletteValuesRule,
    kFunctionDeclarationsRule,
    kFunctionRule,
    kLayerBlockRule,
    kLayerStatementRule,
    kNestedDeclarationsRule,
    kPositionTryRule,
    kPropertyRule,
    kScopeRule,
    kStartingStyleRule,
    kViewTransitionRule,
    // go/keep-sorted end
  };

  virtual Type GetType() const = 0;

  // https://drafts.csswg.org/cssom/#dom-cssrule-type
  int type() const {
    Type type = GetType();
    return type >= Type::kFirstInternalRule ? 0 : static_cast<int>(type);
  }

  virtual String cssText() const = 0;
  virtual void Reattach(StyleRuleBase*) = 0;

  virtual CSSRuleList* cssRules() const { return nullptr; }
  virtual MediaQuerySetOwner* GetMediaQuerySetOwner() { return nullptr; }

  void SetParentStyleSheet(CSSStyleSheet*);

  void SetParentRule(CSSRule*);

  void Trace(Visitor*) const override;

  CSSStyleSheet* parentStyleSheet() const {
    if (parent_is_rule_) {
      return parent_ ? ParentAsCSSRule()->parentStyleSheet() : nullptr;
    }
    return ParentAsCSSStyleSheet();
  }

  CSSRule* parentRule() const {
    return parent_is_rule_ ? ParentAsCSSRule() : nullptr;
  }

  // The CSSOM spec states that "setting the cssText attribute must do nothing."
  void setCSSText(const String&) {}

 protected:
  explicit CSSRule(CSSStyleSheet* parent);

  bool HasCachedSelectorText() const { return has_cached_selector_text_; }
  void SetHasCachedSelectorText(bool has_cached_selector_text) const {
    has_cached_selector_text_ = has_cached_selector_text;
  }

  const CSSParserContext* ParserContext(SecureContextMode) const;

  void CountUse(WebFeature) const;

 private:
  bool VerifyParentIsCSSRule() const;
  bool VerifyParentIsCSSStyleSheet() const;

  CSSRule* ParentAsCSSRule() const {
    DCHECK(parent_is_rule_);
    DCHECK(VerifyParentIsCSSRule());
    return reinterpret_cast<CSSRule*>(parent_.Get());
  }
  CSSStyleSheet* ParentAsCSSStyleSheet() const {
    DCHECK(!parent_is_rule_);
    DCHECK(VerifyParentIsCSSStyleSheet());
    return reinterpret_cast<CSSStyleSheet*>(parent_.Get());
  }

  mutable unsigned char has_cached_selector_text_ : 1;
  unsigned char parent_is_rule_ : 1;

  // parent_ should reference either CSSRule or CSSStyleSheet (both are
  // descendants of ScriptWrappable). This field should only be accessed
  // via the getters above (ParentAsCSSRule and ParentAsCSSStyleSheet).
  Member<ScriptWrappable> parent_;

  friend StyleRuleBase* ParseRuleForInsert(
      const ExecutionContext* execution_context,
      const String& rule_string,
      unsigned index,
      size_t num_child_rules,
      CSSRule& parent_rule,
      ExceptionState& exception_state);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_RULE_H_
