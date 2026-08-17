// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_RULE_SET_DIFF_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_RULE_SET_DIFF_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_set.h"
#include "v8/include/cppgc/garbage-collected.h"

namespace blink {

class RuleSet;
class StyleRule;
class StyleRuleBase;

// When mutating a stylesheet (inserting rules, deleting rules, modifying
// selectors, modifying contents of rules), RuleSetDiff stores a list of
// affected rules. This is so that when we invalidate style based on the
// selectors in the old and new rulesets, we can consider only the selectors
// that were actually changed, instead of every rule in the sheet. This reduces
// recalculation scope significantly in several common situations, such as
// inserting a single rule into a large stylesheet. The RuleSetDiff is
// essentially a mapping from (old ruleset, new ruleset) -> (changed rules),
// which can then be used to create a “diff ruleset” that contains fewer
// selectors to check during invalidation.
//
// For simplicity, we keep a list of StyleRules, even though we only actually
// care about the selectors; CSSSelector is not usually kept alive on its own,
// and comparing StyleRule is cheaper than trying to deduplicate selectors.
// We can have false positives (e.g., if someone changed a rule but then changed
// it back again) but never false negatives. If a stylesheet modifies something
// that is not a StyleRule (such as a @keyframe, or an @import statement),
// we give up and mark the entire diff as “unrepresentable”; this means that
// we will need to test all selectors in both the old and new rule sets.
//
// We do not diff entirely unrelated stylesheets (e.g. if someone changes
// an entire stylesheet with innerText); RuleSetDiff only gets populated
// where people use explicit CSSOM mutation (insertRule etc.).
class CORE_EXPORT RuleSetDiff : public GarbageCollected<RuleSetDiff> {
 public:
  // Construct a diff for mutating a stylesheet whose existing rule set is
  // <old_ruleset>. We don't really know the new ruleset until later,
  // so it is given in NewRuleSetCreated().
  explicit RuleSetDiff(RuleSet* old_ruleset) : old_ruleset_(old_ruleset) {}

  // Mark that the given rule was part of a relevant change. If it's a
  // @keyframe or @import or similar (anything that is not StyleRule),
  // this is the same as calling MarkUnrepresentable(), since
  // such changes can have very wide-ranging effects throughout the
  // generated rule set.
  //
  // Note that the rule can have child rules (CSS nesting); RuleSetDiff
  // takes this into account when running CreateDiffRuleset().
  // In particular, when considering whether to include a style rule A,
  // and AddDiff() has been called on B, and B is a parent (directly
  // or indirectly) of A, A will be included.
  void AddDiff(StyleRuleBase* rule);

  void MarkUnrepresentable() {
    DCHECK(!HasNewRuleSet());
    unrepresentable_ = true;
    changed_rules_.clear();
  }

  // These are signals that a new ruleset was just created (or cleared)
  // for the stylesheet that used to be represented by “old_ruleset”,
  // completing the pair. Usually, NewRuleSetCreated() means that we are
  // about to replace “old_ruleset” with “new_ruleset” and a diff ruleset
  // is soon to be created. Once this happens, you cannot add new diffs
  // (since they would not be represented in new_ruleset, which is fixed
  // after reaction).
  void NewRuleSetCreated(RuleSet* new_ruleset) {
    DCHECK(!HasNewRuleSet());
    new_ruleset_ = new_ruleset;
  }
  void NewRuleSetCleared() { new_ruleset_ = nullptr; }
  bool HasNewRuleSet() const { return new_ruleset_ != nullptr; }

  bool Matches(const RuleSet* old_ruleset, const RuleSet* new_ruleset) const {
    DCHECK(HasNewRuleSet());
    return old_ruleset == old_ruleset_ && new_ruleset == new_ruleset_;
  }

  void Trace(Visitor* visitor) const {
    visitor->Trace(old_ruleset_);
    visitor->Trace(new_ruleset_);
    visitor->Trace(changed_rules_);
  }

  // Creates a RuleSet that contains only those rules in “old_ruleset_”
  // and “new_ruleset_” that are covered by a change given to AddDiff().
  // Will return nullptr on failure; in particular, if an unrepresentable
  // change has been entered at any point. If this happens, the caller
  // will need to check all selectors in both old_ruleset_ and new_ruleset_
  // itself.
  RuleSet* CreateDiffRuleset() const;

 private:
  Member<RuleSet> old_ruleset_;
  Member<RuleSet> new_ruleset_;
  HeapHashSet<Member<StyleRule>> changed_rules_;
  bool unrepresentable_ = false;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_RULE_SET_DIFF_H_
