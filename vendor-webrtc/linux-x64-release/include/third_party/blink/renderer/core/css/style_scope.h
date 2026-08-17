// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_STYLE_SCOPE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_STYLE_SCOPE_H_

#include <optional>

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/css/css_selector_list.h"
#include "third_party/blink/renderer/core/css/parser/css_nesting_type.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "third_party/blink/renderer/platform/heap/visitor.h"

namespace blink {

class CSSParserTokenStream;
class StyleRule;
class StyleSheetContents;

class CORE_EXPORT StyleScope final : public GarbageCollected<StyleScope> {
 public:
  // Construct a StyleScope with explicit roots specified by elements matching
  // the `from` selector list (within the StyleRule). The (optional) `to`
  // parameter selects the the limit elements, i.e. the extent of the scope.
  //
  // Note that the `from` selector list is represented here as a "dummy"
  // StyleRule instead of a CSSSelectorList, because scopes need to behave
  // as style rules to integrate with CSS Nesting.
  // https://drafts.csswg.org/css-nesting-1/#nesting-at-scope
  StyleScope(StyleRule* from, CSSSelectorList* to);
  // Construct a StyleScope with implicit roots at the parent nodes of the
  // stylesheet's owner nodes. Note that StyleScopes with implicit roots
  // can still have limits.
  explicit StyleScope(StyleSheetContents* contents, CSSSelectorList* to);
  StyleScope(const StyleScope&);
  // Note that the `nesting_type` and `parent_rule_for_nesting` provided here
  // are only used for parsing the <scope-start> selector. The <scope-end>
  // selector and style rules within the scope's body will use
  // CSSNestingType::kScope and `RuleForNesting()` instead.
  static StyleScope* Parse(CSSParserTokenStream& stream,
                           const CSSParserContext* context,
                           CSSNestingType nesting_type,
                           StyleRule* parent_rule_for_nesting,
                           StyleSheetContents* style_sheet);

  void Trace(blink::Visitor*) const;

  StyleScope* CopyWithParent(const StyleScope*) const;

  // From() and To() both return the first CSSSelector in a list, or nullptr
  // if there is no list.
  const CSSSelector* From() const;
  const CSSSelector* To() const;
  const StyleScope* Parent() const { return parent_.Get(); }

  // The rule to use for resolving the nesting selector (&) for this scope's
  // inner rules.
  StyleRule* RuleForNesting() const { return from_.Get(); }

  // Returns a copy of StyleScope, with any '&' selectors in the prelude updated
  // to `new_parent`. If no '&' selectors required an update, returns 'this'.
  //
  // See also CSSSelector::Renest.
  const StyleScope* Renest(StyleRule* new_parent) const;

  // https://drafts.csswg.org/css-cascade-6/#implicit-scope
  bool IsImplicit() const { return contents_.Get() != nullptr; }

 private:
  // If `contents_` is not nullptr, then this is a prelude-less @scope rule
  // which is implicitly scoped to the owner node's parent.
  Member<StyleSheetContents> contents_;
  Member<StyleRule> from_;      // May be nullptr.
  Member<CSSSelectorList> to_;  // May be nullptr.
  Member<const StyleScope> parent_;
  mutable std::optional<unsigned> specificity_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_STYLE_SCOPE_H_
