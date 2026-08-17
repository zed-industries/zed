/*
 * Copyright (C) 1999 Lars Knoll (knoll@kde.org)
 *           (C) 2004-2005 Allan Sandfeld Jensen (kde@carewolf.com)
 * Copyright (C) 2006, 2007 Nicholas Shanks (webkit@nickshanks.com)
 * Copyright (C) 2005, 2006, 2007, 2008, 2009, 2010, 2011, 2012, 2013 Apple Inc.
 * All rights reserved.
 * Copyright (C) 2007 Alexey Proskuryakov <ap@webkit.org>
 * Copyright (C) 2007, 2008 Eric Seidel <eric@webkit.org>
 * Copyright (C) 2008, 2009 Torch Mobile Inc. All rights reserved.
 * (http://www.torchmobile.com/)
 * Copyright (c) 2011, Code Aurora Forum. All rights reserved.
 * Copyright (C) Research In Motion Limited 2011. All rights reserved.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_SELECTOR_CHECKER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_SELECTOR_CHECKER_H_

#include <limits>

#include "base/dcheck_is_on.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/css/css_selector.h"
#include "third_party/blink/renderer/core/css/resolver/element_resolve_context.h"
#include "third_party/blink/renderer/core/css/resolver/match_flags.h"
#include "third_party/blink/renderer/core/css/style_request.h"
#include "third_party/blink/renderer/core/css/style_scope.h"
#include "third_party/blink/renderer/core/css/style_scope_frame.h"
#include "third_party/blink/renderer/core/dom/element.h"
#include "third_party/blink/renderer/core/scroll/scroll_types.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_map.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_vector.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "third_party/blink/renderer/platform/heap/visitor.h"

namespace blink {

class CSSSelector;
class ContainerNode;
class CustomScrollbar;
class Element;
class PartNames;

class CORE_EXPORT SelectorChecker {
  STACK_ALLOCATED();

 public:
  enum Mode {
    // Used when matching selectors inside style recalc. This mode will set
    // restyle flags across the tree during matching which impact how style
    // sharing and invalidation work later.
    kResolvingStyle,

    // Used when collecting which rules match into a StyleRuleList, the engine
    // internal represention.
    //
    // TODO(esprehn): This doesn't change the behavior of the SelectorChecker
    // we should merge it with a generic CollectingRules mode.
    kCollectingStyleRules,

    // Used when collecting which rules match into a CSSRuleList, the CSSOM api
    // represention.
    //
    // TODO(esprehn): This doesn't change the behavior of the SelectorChecker
    // we should merge it with a generic CollectingRules mode.
    kCollectingCSSRules,

    // Used when matching rules for querySelector and <content select>. This
    // disables the special handling for positional selectors during parsing
    // and also enables static profile only selectors like >>>.
    kQueryingRules,
  };

  explicit inline SelectorChecker(const Mode& mode)
      : scrollbar_(nullptr),
        part_names_(nullptr),
        pseudo_argument_(g_null_atom),
        scrollbar_part_(kNoPart),
        mode_(mode),
        is_ua_rule_(false) {}
  inline SelectorChecker(PartNames* part_names,
                         const StyleRequest& style_request,
                         const Mode& mode,
                         const bool& is_ua_rule)
      : scrollbar_(style_request.scrollbar),
        part_names_(part_names),
        pseudo_argument_(style_request.pseudo_argument),
        pseudo_ident_list_(style_request.pseudo_ident_list),
        scrollbar_part_(style_request.scrollbar_part),
        mode_(mode),
        is_ua_rule_(is_ua_rule) {}

  SelectorChecker(const SelectorChecker&) = delete;
  SelectorChecker& operator=(const SelectorChecker&) = delete;

  // When matching certain selectors (e.g :hover), we sometimes want to mark the
  // relevant element(s) as being affected by that selector to aid some
  // invalidation process later. We also typically need a distinction between
  // elements that are affected themselves and elements that are *related* to
  // affected elements (e.g. has an affected ancestor or sibling). The impact of
  // a given SelectorCheckingContext tells us which invalidation flags to set.
  enum class Impact {
    // Invalidation
    // flags on to the element itself should be set.
    kSubject = 0b01,
    // Ancestors or previous siblings
    // should have their invalidation flags set.
    kNonSubject = 0b10,
    // Invalidation flags should be set as if both subject and non-subjects are
    // impacted. This can be used defensively in situations where we don't know
    // the full impact of a given selector at the time that selector is
    // evaluated, e.g. '@scope (:hover) { ... }'.
    kBoth = 0b11
  };

  // Wraps the current element and a CSSSelector and stores some other state of
  // the selector matching process.
  struct SelectorCheckingContext {
    STACK_ALLOCATED();

   public:
    // Initial selector constructor
    explicit SelectorCheckingContext(Element* element) : element(element) {}
    explicit SelectorCheckingContext(
        const ElementResolveContext& element_context)
        : element(&element_context.GetUltimateOriginatingElementOrSelf()),
          pseudo_element(element_context.GetPseudoElement()),
          pseudo_element_ancestors(
              element_context.GetPseudoElementAncestors()) {}

    // If matching for real element, returns that element;
    // Otherwise, matching is for pseudo element, returns
    // pseudo element ancestor at `index`, or the last pseudo element ancestor,
    // if `index` is greater than the size of ancestors array (to collect
    // pseudo elements styles).
    // Note: `index` == kNotFound means matching is yet being done for real
    // element.
    Element& GetElementForMatching(wtf_size_t index) const;

    // Group fields by type to avoid perf test regression.
    // https://crrev.com/c/3362008
    const CSSSelector* selector = nullptr;

    // The "tree context" [1] whenever the selector is "matched against
    // a tree" [2].
    //
    // This can most easily be understood as "where a given selector
    // comes from":
    //
    //  - For style rules at the document level, this is a Document.
    //  - For style rules inside a shadow tree, this is a ShadowRoot.
    //  - For querySelector, this is the root [3] of the node querySelector
    //    was called on, meaning it can either be a Document, ShadowRoot,
    //    or DocumentFragment.
    //
    // Some selector matching in Blink takes place with tree_scope==nullptr,
    // for example UA rules. This should be functionally identical to providing
    // the Document associated with `element` as the tree scope.
    //
    // [1] https://drafts.csswg.org/selectors-4/#match-a-selector-against-a-tree
    // [2] https://drafts.csswg.org/css-scoping-1/#tree-context
    // [3] https://dom.spec.whatwg.org/#concept-tree-root
    const TreeScope* tree_scope = nullptr;
    // The scoping root [1], whenever the selector is scoped [2].
    //
    // This is often the same node as `tree_scope`, but differs when selectors
    // are scoped to some specific Element within that tree. For example,
    // for Element.querySelector, `scope` is set to the element querySelector
    // is invoked on.
    //
    // If `scope` is an element, the :scope pseudo-class matches that element,
    // and only that element. Otherwise, it matches what :root matches.
    // This behavior is different from what specs expect, see discussion
    // in Issue 7261 [3].
    //
    // See also documentation on `style_scope` below, as it is relevant for
    // how `scope` is used throughout the evaluation of the selector.
    //
    // [1] https://drafts.csswg.org/selectors-4/#scoping-root
    // [2] https://drafts.csswg.org/selectors-4/#scoped-selector
    // [3] https://github.com/w3c/csswg-drafts/issues/7261
    const ContainerNode* scope = nullptr;
    // The StyleScope (i.e. @scope) associated with the selector.
    //
    // This points to the innermost StyleScope, i.e. the StyleScope holding
    // on to the style rule directly.
    //
    // A non-nullptr `style_scope` causes "scope activation" [1] at some point
    // during selector matching, which is basically lazy evaluation of what
    // the scoping roots are. For each found scoping root, we'll match the
    // selector (perhaps partially) with `scope` set to that root.
    //
    // Both `scope` and `style_scope` may be specified on the
    // SelectorCheckingContext passed to SelectorChecker::Match,
    // but if `style_scope` is non-nullptr, then `scope` is effectively ignored:
    // `scope` will be assigned a value by the "scope activation" process
    // referenced above.
    //
    // [1] SelectorChecker::MatchForScopeActivation
    const StyleScope* style_scope = nullptr;
    // A cache used to carry out the work described for `style_scope`.
    // Must be non-nullptr when `style_scope` is non-nullptr.
    StyleScopeFrame* style_scope_frame = nullptr;

    Element* element = nullptr;
    Element* previous_element = nullptr;
    Element* vtt_originating_element = nullptr;
    ContainerNode* relative_anchor_element = nullptr;

    AtomicString* pseudo_argument = nullptr;
    // The pseudo element type of pseudo element we are matching styles for.
    PseudoId pseudo_id = kPseudoIdNone;
    // The last pseudo element selector we saw. This is not necessarily the
    // pseudo_id above since we may have nested pseudo elements. Also, this may
    // be the pseudo element selector we are looking at while matching styles
    // for the originating element.
    PseudoId previously_matched_pseudo_element = kPseudoIdNone;
    Impact impact = Impact::kSubject;

    bool is_sub_selector = false;
    bool in_rightmost_compound = true;
    bool has_scrollbar_pseudo = false;
    bool in_nested_complex_selector = false;
    // If true, elements that are links will match :visited. Otherwise,
    // they will match :link.
    bool match_visited = false;
    // The `match_visited` flag can become false during selector matching
    // for various reasons (see DisallowMatchVisited and its call sites).
    // The `had_match_visited` flag tracks whether was initially true or not.
    // This is needed by @scope (CalculateActivations), which needs to evaluate
    // visited-dependent selectors according to the original `match_visited`
    // setting.
    bool had_match_visited = false;
    bool pseudo_has_in_rightmost_compound = true;
    bool is_inside_has_pseudo_class = false;
    // Affects whether or not :current matches after a ::search-text.
    bool search_text_request_is_current = false;
    // Real PseudoElement that we match for. `element` will be its originating
    // element, but `element` is only needed for matching. The rules will be
    // saved on `pseudo_element`.
    // Is null if matching is for real element.
    Element* pseudo_element = nullptr;
    // Pseudo element ancestors array for PseudoElement rule matching (including
    // the nested ones), read more in ElementResolveContext.
    // Includes the pseudo_element as last entry and all originating pseudo
    // elements up the tree (doesn't include real originating element, which
    // would be `element`). Is empty if matching is for real element.
    base::span<Element* const> pseudo_element_ancestors;
  };

  struct MatchResult {
    STACK_ALLOCATED();

   public:
    void SetFlag(MatchFlag flag) { flags |= static_cast<MatchFlags>(flag); }
    bool HasFlag(MatchFlag flag) const {
      return flags & static_cast<MatchFlags>(flag);
    }

    PseudoId dynamic_pseudo{kPseudoIdNone};

    // Index of the current pseudo element ancestor, used for PseudoElement
    // matching. See comments above for pseudo_element_ancestors.
    // kNotFound means matching is yet being done for real element.
    // When matching for e.g. div::column::scroll-marker, ::column is index 0
    // and ::scroll-marker is index 1.
    wtf_size_t pseudo_ancestor_index{kNotFound};
    // Note: can become equal to ancestors array size, meaning we match
    // pseudo style for pseudo_element. If increased further, selector
    // matching is failed in SelectorChecker code.
    void DescendToNextPseudoElement() {
      pseudo_ancestor_index =
          pseudo_ancestor_index == kNotFound ? 0u : pseudo_ancestor_index + 1u;
    }

    // Comes from an AtomicString, but not stored as one to avoid
    // the cost of checking the refcount on cleaning up from every
    // Match() call. Owned by the CSS selector it came from.
    StringImpl* custom_highlight_name{nullptr};

    // From the :has() argument selector checking, we need to get the element
    // that matches the leftmost compound selector to mark all possible :has()
    // anchor elements (the relative anchor element of the :has() argument).
    //
    // <main id=main>
    //   <div id=d1>
    //     <div id=d2 class="a">
    //       <div id=d3 class="a">
    //         <div id=d4>
    //           <div id=d5 class="b">
    //           </div>
    //         </div>
    //       </div>
    //     </div>
    //   </div>
    // </div>
    // <script>
    //  main.querySelectorAll('div:has(.a .b)'); // Should return #d1, #d2
    // </script>
    //
    // In case of the above example, the selector 'div:has(.a .b)' is checked
    // on the descendants of '#main' element in this order:
    // - 'div#d1', 'div#d2', 'div#d3', 'div#d4', 'div#d5'
    // When checking the selector on 'div#d1', we can get all possible :has()
    // anchor element while checking the :has() argument selector ('.a .b')
    // on the descendants of 'div#d1'.
    // Among the descendants of 'div#d1', 'div#d5' matches the argument selector
    // '.a .b'. More precisely, the 'div#d5' matches the argument selector
    // ':-internal-relative-anchor .a .b' only when the ':-internal-relative-
    // anchor' matches any ancestors of the element matches the leftmost
    // compound of the argument selector ('.a').
    // So, in case of checking the 'div:has(.a .b)' on 'div#d1', 'div#d1' and
    // 'div#d2' can be a :has() argument anchor element because 'div#d3' and
    // 'div#d4' are the element that matches the leftmost compound '.a' of the
    // :has() argument '.a .b'.
    // To avoid repetitive argument checking, the :has() anchor elements are
    // stored in the CheckPseudoHasResultCache. To cache the anchor elements
    // correctly, MatchResult returns the elements that match the leftmost
    // compound of the :has() argument selector.
    //
    // This field is only for checking :has() pseudo class. To avoid the
    // MatchResult instance allocation overhead on checking the other selectors,
    // MatchResult has a pointer field to hold the reference of the vector
    // instance instead of having the vector instance field.
    HeapVector<Member<Element>>* has_argument_leftmost_compound_matches{
        nullptr};
    unsigned proximity{std::numeric_limits<unsigned>::max()};
    MatchFlags flags{0};
  };

  // Used for situations where we have "inner" selector matching, such as
  // :is(...). Ensures that we propagate the necessary sub-result data
  // to the outer MatchResult.
  class SubResult : public MatchResult {
    STACK_ALLOCATED();

   public:
    explicit SubResult(MatchResult& parent) : parent_(parent) {
      pseudo_ancestor_index = parent_.pseudo_ancestor_index;
    }
    ~SubResult() {
      parent_.flags |= flags;
      PropagatePseudoAncestorIndex();
    }
    void PropagatePseudoAncestorIndex() {
      // Propagate only useful change in index, which is either:
      // a) kNotFound -> 0;
      // b) increasing index value.
      // Propagating is needed to later check that we've reached the end of
      // the ancestors array, meaning that the selector matches the full
      // ancestors chain.
      if (pseudo_ancestor_index != kNotFound) {
        parent_.pseudo_ancestor_index =
            parent_.pseudo_ancestor_index == kNotFound
                ? pseudo_ancestor_index
                : std::max(pseudo_ancestor_index,
                           parent_.pseudo_ancestor_index);
      }
    }

   private:
    MatchResult& parent_;
  };

  bool Match(const SelectorCheckingContext& context, MatchResult& result) const;

  bool Match(const SelectorCheckingContext& context) const {
    MatchResult ignore_result;
    return Match(context, ignore_result);
  }

  static bool MatchesFocusPseudoClass(const Element&,
                                      PseudoId matching_for_pseudo_element);
  static bool MatchesFocusVisiblePseudoClass(const Element&);
  static bool MatchesSelectorFragmentAnchorPseudoClass(const Element&);

 private:
  // Does the work of checking whether the simple selector and element pointed
  // to by the context are a match. Delegates most of the work to the Check*
  // methods below.
  ALWAYS_INLINE bool CheckOne(const SelectorCheckingContext&,
                              MatchResult&) const;

  enum MatchStatus {
    kSelectorMatches,
    kSelectorFailsLocally,
    kSelectorFailsAllSiblings,
    kSelectorFailsCompletely
  };

  // MatchSelector is the core of the recursive selector matching process. It
  // calls through to the Match* methods below and Match above.
  //
  // At each level of the recursion the context (which selector and element we
  // are considering) is represented by a SelectorCheckingContext. A context may
  // also contain a scope, this can limit the matching to occur within a
  // specific shadow tree (and its descendants). As the recursion proceeds, new
  // `SelectorCheckingContext` objects are created by copying a previous one and
  // changing the selector and/or the element being matched
  //
  // MatchSelector uses CheckOne to determine what element matches the current
  // selector. If CheckOne succeeds we recurse with a new context pointing to
  // the next selector (in a selector list, we proceed leftwards through the
  // compound selectors). If CheckOne fails we may try again with a different
  // element or we may fail the match entirely. In both cases, the next element
  // to try (e.g. same element, parent, sibling) depends on the combinators in
  // the selectors.
  MatchStatus MatchSelector(const SelectorCheckingContext&, MatchResult&) const;
  MatchStatus MatchForSubSelector(const SelectorCheckingContext&,
                                  MatchResult&) const;
  MatchStatus MatchForScopeActivation(const SelectorCheckingContext&,
                                      MatchResult&) const;
  MatchStatus MatchForRelation(const SelectorCheckingContext&,
                               MatchResult&) const;
  MatchStatus MatchForPseudoContent(const SelectorCheckingContext&,
                                    const Element&,
                                    MatchResult&) const;
  MatchStatus MatchForPseudoShadow(const SelectorCheckingContext&,
                                   const ContainerNode*,
                                   MatchResult&) const;
  bool CheckPseudoClass(const SelectorCheckingContext&, MatchResult&) const;
  bool CheckPseudoAutofill(CSSSelector::PseudoType, Element&) const;
  bool CheckPseudoElement(const SelectorCheckingContext&, MatchResult&) const;
  bool CheckScrollbarPseudoClass(const SelectorCheckingContext&,
                                 MatchResult&) const;
  bool CheckPseudoHost(const SelectorCheckingContext&, MatchResult&) const;
  bool CheckPseudoScope(const SelectorCheckingContext&, MatchResult&) const;
  bool CheckPseudoNot(const SelectorCheckingContext&, MatchResult&) const;
  bool CheckPseudoHas(const SelectorCheckingContext&, MatchResult&) const;
  bool MatchesAnyInList(const SelectorCheckingContext& context,
                        const CSSSelector* selector_list,
                        MatchResult& result) const;

  const StyleScopeActivations& EnsureActivations(const SelectorCheckingContext&,
                                                 const StyleScope&) const;
  const StyleScopeActivations* CalculateActivations(
      const TreeScope*,
      Element&,
      const StyleScope&,
      const StyleScopeActivations& outer_activations,
      StyleScopeFrame*,
      bool match_visited) const;
  bool MatchesWithScope(Element&,
                        const CSSSelector& selector_list,
                        const TreeScope*,
                        const ContainerNode* scope,
                        bool match_visited,
                        MatchFlags&) const;
  // https://drafts.csswg.org/css-cascade-6/#scoping-limit
  bool ElementIsScopingLimit(const TreeScope*,
                             const StyleScope&,
                             const StyleScopeActivation&,
                             Element& element,
                             bool match_visited,
                             MatchFlags&) const;

  CustomScrollbar* scrollbar_;
  PartNames* part_names_;
  const String pseudo_argument_;
  const Vector<AtomicString> pseudo_ident_list_;
  ScrollbarPart scrollbar_part_;
  Mode mode_;
  bool is_ua_rule_;
#if DCHECK_IS_ON()
  mutable bool inside_match_ = false;
#endif

  friend class NthIndexCache;
};

// An accelerated selector checker that matches only selectors with a
// certain set of restrictions, informally called “easy” selectors.
// (Not to be confused with simple selectors, which is a standards-defined
// term.) Easy selectors support only a very small subset of the full
// CSS selector machinery, but does so much faster than SelectorChecker
// (typically a bit over twice as fast), and that subset tends to be enough
// for ~80% of actual selectors checks on a typical web page. (It is also
// ree from the complexities of Shadow DOM and does not check whether
// the query exceeds the scope, so it cannot be used for querySelector().)
//
// The set of supported selectors is formally given as “anything IsEasy()
// returns true for”, but roughly encompasses the following:
//
//  - Tag matches (e.g. div).
//  - ID matches (e.g. #id).
//  - Class matches (e.g. .c).
//  - Case-sensitive attribute is-set and exact matches ([foo] and [foo="bar"]).
//  - Subselector and descendant combinators.
//  - Anything that does not need further checking
//    (CSSSelector::IsCoveredByBucketing()).
//
// Given this, it does not need to set up any context, do recursion,
// backtracking, have large switch/cases for pseudos, or the similar.
//
// You must include selector_checker-inl.h to use this class;
// its functions are declared ALWAYS_INLINE because the call overhead
// is so large compared to what the functions are actually doing.
class CORE_EXPORT EasySelectorChecker {
 public:
  // Returns true iff the given selector is easy and can be given to Match().
  // Should be precomputed for the given selector.
  //
  // If IsEasy() is true, this selector can never return any match flags,
  // or match (dynamic) pseudos.
  ALWAYS_INLINE static bool IsEasy(const CSSSelector* selector);

  // Returns whether the given selector matches the given element.
  // The following preconditions apply:
  //
  //  - The selector must be easy (see IsEasy()).
  //  - Tag matching must be case-sensitive in the current context,
  //    i.e., that the element is _not_ a non-HTML element in an
  //    HTML document.
  //
  // Unlike SelectorChecker, does not check style_scope; the caller
  // will need to do that if desired.
  ALWAYS_INLINE static bool Match(const CSSSelector* selector,
                                  const Element* element);

 private:
  ALWAYS_INLINE static bool MatchOne(const CSSSelector* selector,
                                     const Element* element);
  ALWAYS_INLINE static bool AttributeIsSet(const Element& element,
                                           const QualifiedName& attr);
  ALWAYS_INLINE static bool AttributeMatches(const Element& element,
                                             const QualifiedName& attr,
                                             const AtomicString& value,
                                             bool insensitive_match);
  ALWAYS_INLINE static bool AttributeItemHasName(
      const Attribute& attribute_item,
      const Element& element,
      const QualifiedName& name);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_SELECTOR_CHECKER_H_
