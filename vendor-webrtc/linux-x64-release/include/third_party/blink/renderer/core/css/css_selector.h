/*
 * Copyright (C) 1999-2003 Lars Knoll (knoll@kde.org)
 *               1999 Waldo Bastian (bastian@kde.org)
 * Copyright (C) 2004, 2006, 2007, 2008, 2009, 2010, 2013 Apple Inc. All rights
 * reserved.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_SELECTOR_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_SELECTOR_H_

#include <memory>
#include <utility>

#include "base/check_op.h"
#include "base/compiler_specific.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/css/parser/css_nesting_type.h"
#include "third_party/blink/renderer/core/css/parser/css_parser_mode.h"
#include "third_party/blink/renderer/core/dom/qualified_name.h"
#include "third_party/blink/renderer/core/style/computed_style_constants.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "third_party/blink/renderer/platform/heap/visitor.h"
#include "third_party/blink/renderer/platform/wtf/bit_field.h"
#include "third_party/blink/renderer/platform/wtf/gc_plugin.h"

namespace WTF {
class StringBuilder;
}  // namespace WTF

namespace blink {

class CSSParserContext;
class CSSSelectorList;
class Document;
class StyleRule;

// This class represents a simple selector for a StyleRule.

// CSS selector representation is somewhat complicated and subtle. A
// representative list of selectors is in CSSSelectorTest; run it in a debug
// build to see useful debugging output.
//
// ** NextSimpleSelector() and Relation():
//
// Selectors are represented as an array of simple selectors (defined more
// or less according to
// http://www.w3.org/TR/css3-selectors/#simple-selectors-dfn).
// The NextInSimpleSelector() member function returns the next simple selector
// in the list. The Relation() member function returns the relationship of the
// current simple selector to the one in NextSimpleSelector(). For example, the
// CSS selector .a.b #c is represented as:
//
// SelectorText(): .a.b #c
// --> (relation == kDescendant)
//   SelectorText(): .a.b
//   --> (relation == kSubSelector)
//     SelectorText(): .b
//
// The ordering of the simple selectors varies depending on the situation.
// * Relations using combinators
//   (http://www.w3.org/TR/css3-selectors/#combinators), such as descendant,
//   sibling, etc., are parsed right-to-left (in the example above, this is why
//   #c is earlier in the simple selector chain than .a.b).
// * SubSelector relations are parsed left-to-right, such as the .a.b example
//   above.
// * ShadowPseudo relations are parsed right-to-left. Example:
//   summary::-webkit-details-marker is parsed as: selectorText():
//   summary::-webkit-details-marker --> (relation == ShadowPseudo)
//   selectorText(): summary
//
// ** match():
//
// The match of the current simple selector tells us the type of selector, such
// as class, id, tagname, or pseudo-class. Inline comments in the Match enum
// give examples of when each type would occur.
//
// ** value(), attribute():
//
// value() tells you the value of the simple selector. For example, for class
// selectors, value() will tell you the class string, and for id selectors it
// will tell you the id(). See below for the special case of attribute
// selectors.
//
// ** Attribute selectors.
//
// Attribute selectors return the attribute name in the attribute() method. The
// value() method returns the value matched against in case of selectors like
// [attr="value"].
//
class CORE_EXPORT CSSSelector {
  // CSSSelector typically lives on Oilpan; either in StyleRule's
  // AdditionalBytes, as part of CSSSelectorList, or (during parsing)
  // in a HeapVector. However, it is never really allocated as a separate
  // Oilpan object, so it does not inherit from GarbageCollected.
  DISALLOW_NEW();

 public:
  /* how the attribute value has to match.... Default is Exact */
  enum MatchType {
    kUnknown,
    kInvalidList,       // Used as a marker in CSSSelectorList.
    kTag,               // Example: div
    kUniversalTag,      // Example: * (possibly with namespace)
    kId,                // Example: #id
    kClass,             // Example: .class
    kPseudoClass,       // Example: :nth-child(2)
    kPseudoElement,     // Example: ::first-line
    kPagePseudoClass,   // Example: @page :right
    kAttributeExact,    // Example: E[foo="bar"]
    kAttributeSet,      // Example: E[foo]
    kAttributeHyphen,   // Example: E[foo|="bar"]
    kAttributeList,     // Example: E[foo~="bar"]
    kAttributeContain,  // css3: E[foo*="bar"]
    kAttributeBegin,    // css3: E[foo^="bar"]
    kAttributeEnd,      // css3: E[foo$="bar"]
    kFirstAttributeSelectorMatch = kAttributeExact,
  };
  enum class AttributeMatchType;

  CSSSelector();

  // NOTE: Will not deep-copy the selector list, if any.
  CSSSelector(const CSSSelector&);

  CSSSelector(CSSSelector&&);
  explicit CSSSelector(const QualifiedName&, bool tag_is_implicit = false);
  explicit CSSSelector(MatchType match_type,
                       const QualifiedName& attribute,
                       AttributeMatchType case_sensitivity);
  explicit CSSSelector(MatchType match_type,
                       const QualifiedName& attribute,
                       AttributeMatchType case_sensitivity,
                       const AtomicString& value);
  explicit CSSSelector(const StyleRule* parent_rule, bool is_implicit);
  explicit CSSSelector(const AtomicString& pseudo_name, bool is_implicit);

  ~CSSSelector();

  String SelectorText() const;
  // Like `SelectorText`, but replaces any "pseudo-references" with an expansion
  // which makes the result useful as a key in the :has() cache.
  // A "pseudo-reference" is either a '&' selector (which is replaced with
  // :is(<parent rule selector list>)), or a :scope selector (which is replaced
  // with :-internal-scope-<scope_id>).
  //
  // Note that this means that the returned text is not necessarily a valid
  // selector.
  String SelectorTextExpandingPseudoReferences(uintptr_t scope_id) const;
  String SimpleSelectorTextForDebug() const;

  CSSSelector& operator=(const CSSSelector&) = delete;
  CSSSelector& operator=(CSSSelector&&);
  bool operator==(const CSSSelector&) const = delete;
  bool operator!=(const CSSSelector&) const = delete;

  static constexpr unsigned kIdSpecificity = 0x010000;
  static constexpr unsigned kClassLikeSpecificity = 0x000100;
  static constexpr unsigned kTagSpecificity = 0x000001;

  static constexpr unsigned kMaxValueMask = 0xffffff;
  static constexpr unsigned kIdMask = 0xff0000;
  static constexpr unsigned kClassMask = 0x00ff00;
  static constexpr unsigned kElementMask = 0x0000ff;

  // http://www.w3.org/TR/css3-selectors/#specificity
  // We use 256 as the base of the specificity number system.
  unsigned Specificity() const;
  // Returns specificity components in decreasing order of significance.
  std::array<uint8_t, 3> SpecificityTuple() const;

  enum RelationType {
    // No combinator. Used between simple selectors within the same compound.
    kSubSelector,
    // "Space" combinator
    kDescendant,
    // > combinator
    kChild,
    // + combinator
    kDirectAdjacent,
    // ~ combinator
    kIndirectAdjacent,
    // The relation types below are implicit combinators inserted at parse time
    // before pseudo elements which match another flat tree element than the
    // rest of the compound.
    //
    // Implicit combinator inserted before pseudo elements matching an element
    // inside a UA shadow tree. This combinator allows the selector matching to
    // cross a shadow root.
    //
    // Examples:
    // input::placeholder, video::cue(i), video::--webkit-media-controls-panel
    kUAShadow,
    // Implicit combinator inserted before ::slotted() selectors.
    kShadowSlot,
    // Implicit combinator inserted before ::part() selectors which allows
    // matching a ::part in shadow-including descendant tree for #host in
    // "#host::part(button)".
    kShadowPart,

    // leftmost "Space" combinator of relative selector
    kRelativeDescendant,
    // leftmost > combinator of relative selector
    kRelativeChild,
    // leftmost + combinator of relative selector
    kRelativeDirectAdjacent,
    // leftmost ~ combinator of relative selector
    kRelativeIndirectAdjacent,
  };

  enum PseudoType {
    kPseudoActive,
    kPseudoActiveViewTransition,
    kPseudoActiveViewTransitionType,
    kPseudoAfter,
    kPseudoAny,
    kPseudoAnyLink,
    kPseudoAutofill,
    kPseudoAutofillPreviewed,
    kPseudoAutofillSelected,
    kPseudoBackdrop,
    kPseudoBefore,
    kPseudoCheckMark,
    kPseudoChecked,
    kPseudoCornerPresent,
    kPseudoCurrent,
    kPseudoDecrement,
    kPseudoDefault,
    kPseudoDetailsContent,
    kPseudoDialogInTopLayer,
    kPseudoDisabled,
    kPseudoDoubleButton,
    kPseudoDrag,
    kPseudoEmpty,
    kPseudoEnabled,
    kPseudoEnd,
    kPseudoFileSelectorButton,
    kPseudoFirstChild,
    kPseudoFirstLetter,
    kPseudoFirstLine,
    kPseudoFirstOfType,
    kPseudoFirstPage,
    kPseudoFocus,
    kPseudoFocusVisible,
    kPseudoFocusWithin,
    kPseudoFullPageMedia,
    kPseudoHasInterest,
    kPseudoHasPartialInterest,
    kPseudoHasSlotted,
    kPseudoHorizontal,
    kPseudoHover,
    kPseudoIncrement,
    kPseudoIndeterminate,
    kPseudoInvalid,
    kPseudoIs,
    kPseudoLang,
    kPseudoLastChild,
    kPseudoLastOfType,
    kPseudoLeftPage,
    kPseudoLink,
    kPseudoMarker,
    kPseudoModal,
    kPseudoNoButton,
    kPseudoNot,
    kPseudoNthChild,  // Includes :nth-child(An+B of <selector>)
    kPseudoNthLastChild,
    kPseudoNthLastOfType,
    kPseudoNthOfType,
    kPseudoOnlyChild,
    kPseudoOnlyOfType,
    kPseudoOptional,
    kPseudoParent,  // Written as & (in nested rules).
    kPseudoPart,
    kPseudoPermissionElementInvalidStyle,
    kPseudoPermissionElementOccluded,
    kPseudoPermissionGranted,
    kPseudoPlaceholder,
    kPseudoPlaceholderShown,
    kPseudoReadOnly,
    kPseudoReadWrite,
    kPseudoRequired,
    kPseudoResizer,
    kPseudoRightPage,
    kPseudoRoot,
    kPseudoScope,
    kPseudoScrollbar,
    kPseudoScrollbarButton,
    kPseudoScrollbarCorner,
    kPseudoScrollbarThumb,
    kPseudoScrollbarTrack,
    kPseudoScrollbarTrackPiece,
    kPseudoSearchText,
    kPseudoPickerIcon,
    kPseudoPicker,
    kPseudoSelection,
    kPseudoSelectorFragmentAnchor,
    kPseudoSingleButton,
    kPseudoStart,
    kPseudoState,
    kPseudoTarget,
    kPseudoTargetOfInterest,
    kPseudoTargetOfPartialInterest,
    kPseudoUnknown,
    // Something that was unparsable, but contained either a nesting
    // selector (&), or a :scope pseudo-class, and must therefore be kept
    // for serialization purposes.
    kPseudoUnparsed,
    kPseudoUserInvalid,
    kPseudoUserValid,
    kPseudoValid,
    kPseudoVertical,
    kPseudoVisited,
    kPseudoWebKitAutofill,
    kPseudoWebkitAnyLink,
    kPseudoWhere,
    kPseudoWindowInactive,
    // TODO(foolip): When the unprefixed Fullscreen API is enabled, merge
    // kPseudoFullScreen and kPseudoFullscreen into one. (kPseudoFullscreen is
    // controlled by the FullscreenUnprefixed REF, but is otherwise an alias.)
    kPseudoFullScreen,
    kPseudoFullScreenAncestor,
    kPseudoFullscreen,
    kPseudoInRange,
    kPseudoOutOfRange,
    kPseudoPaused,
    kPseudoPictureInPicture,
    kPseudoPlaying,
    kPseudoXrOverlay,
    // Pseudo elements in UA ShadowRoots. Available in any stylesheets.
    kPseudoWebKitCustomElement,
    // Pseudo elements in UA ShadowRoots. Available only in UA stylesheets.
    kPseudoBlinkInternalElement,
    // Pseudo element for fragment styling
    kPseudoColumn,
    kPseudoCue,
    kPseudoDefined,
    kPseudoDir,
    kPseudoFutureCue,
    kPseudoGrammarError,
    kPseudoHas,
    kPseudoHasDatalist,
    kPseudoHighlight,
    kPseudoHost,
    kPseudoHostContext,
    kPseudoHostHasNonAutoAppearance,
    kPseudoIsHtml,
    kPseudoListBox,
    kPseudoMultiSelectFocus,
    kPseudoOpen,
    kPseudoPastCue,
    kPseudoPopoverInTopLayer,
    kPseudoPopoverOpen,
    kPseudoRelativeAnchor,
    kPseudoSlotted,
    kPseudoSpatialNavigationFocus,
    kPseudoSpellingError,
    kPseudoTargetText,
    kPseudoVideoPersistent,
    kPseudoVideoPersistentAncestor,

    // Active ::scroll-marker styling.
    // https://drafts.csswg.org/css-overflow-5/#active-scroll-marker
    kPseudoTargetCurrent,

    // The following selectors are used to target pseudo elements created for
    // ViewTransition.
    // See https://drafts.csswg.org/css-view-transitions-1/#pseudo
    // for details.
    kPseudoViewTransition,
    kPseudoViewTransitionGroup,
    kPseudoViewTransitionImagePair,
    kPseudoViewTransitionNew,
    kPseudoViewTransitionOld,
    // Scroll markers pseudos for Carousel
    kPseudoScrollMarker,
    kPseudoScrollMarkerGroup,
    // Scroll button pseudo for Carousel
    kPseudoScrollButton,
  };

  enum class AttributeMatchType : int {
    kCaseSensitive,
    kCaseInsensitive,
    kCaseSensitiveAlways,
  };

  PseudoType GetPseudoType() const {
    return static_cast<PseudoType>(bits_.get<PseudoTypeField>());
  }

  void UpdatePseudoType(const AtomicString&,
                        const CSSParserContext&,
                        bool has_arguments,
                        CSSParserMode);
  void SetUnparsedPlaceholder(CSSNestingType, const AtomicString&);
  // If this simple selector contains a parent selector (&), returns kNesting.
  // Otherwise, if this simple selector contains a :scope pseudo-class,
  // returns kScope. Otherwise, returns kNone.
  //
  // Note that this means that a selector which contains both '&' and :scope
  // (which can happen for kPseudoUnparsed) will return kNesting. This is OK,
  // since any selector which is nest-containing is also treated as
  // scope-containing during parsing.
  CSSNestingType GetNestingType() const;
  // Sets this CSSSelector to a :where() class with the specified argument.
  void SetWhere(CSSSelectorList*);
  void UpdatePseudoPage(const AtomicString&, const Document*);
  static PseudoType NameToPseudoType(const AtomicString&,
                                     bool has_arguments,
                                     const Document* document);
  static PseudoId GetPseudoId(PseudoType);

  // Re-nesting
  // ==========
  //
  // During parsing, the parent pseudo-class ('&') gains a reference
  // to its parent StyleRule (if any), see `parent_rule_for_nesting`
  // passed to CSSSelectorParser. This is problematic for certain CSSOM
  // mutations, e.g. selectorText="..." on an outer style rule, since
  // any '&' selectors held within that outer style rule now need
  // to be point to the modified selector/rule.
  //
  // Since the selector list of a StyleRule is effectively an inline part
  // of the StyleRule itself (see AdditionalBytesForSelectors), and because
  // RuleSet invalidation requires both the old and new rule to exist
  // in a usable state at the same time (see RuleSetDiff), we handle such
  // mutations by effectively making a copy of the nested rule and its selector
  // list, except that any '&' selectors are updated to the point to the new
  // outer rule.
  //
  // If this simple selector contains any parent pseudo-classes ('&'),
  // returns a copy with any parent rule references updated to `new_parent`.
  // Returns std::nullopt when no references needed an update.
  std::optional<CSSSelector> Renest(StyleRule* new_parent) const;

  // Selectors are kept in an array by CSSSelectorList. The next component of
  // the selector is the next item in the array.
  // SAFETY: Performance-sensitive. Trusts that SetLastInComplexSelector()
  // has been called on the last element in the array to prevent an OOB
  // access from occurring.
  const CSSSelector* NextSimpleSelector() const {
    return IsLastInComplexSelector() ? nullptr : UNSAFE_BUFFERS(this + 1);
  }
  CSSSelector* NextSimpleSelector() {
    return IsLastInComplexSelector() ? nullptr : UNSAFE_BUFFERS(this + 1);
  }

  static const AtomicString& UniversalSelectorAtom() { return g_null_atom; }
  const QualifiedName& TagQName() const;
  const StyleRule* ParentRule() const;  // Only valid for kPseudoParent.
  const AtomicString& Value() const;
  const AtomicString& SerializingValue() const;

  // WARNING: Use of QualifiedName by attribute() is a lie.
  // attribute() will return a QualifiedName with prefix and namespaceURI
  // set to g_star_atom to mean "matches any namespace". Be very careful
  // how you use the returned QualifiedName.
  // http://www.w3.org/TR/css3-selectors/#attrnmsp
  const QualifiedName& Attribute() const;
  AttributeMatchType AttributeMatch() const;
  bool LegacyCaseInsensitiveMatch() const;
  // Returns the argument of a parameterized selector. For example, :lang(en-US)
  // would have an argument of en-US.
  // Note that :nth-* selectors don't store an argument and just store the
  // numbers.
  const AtomicString& Argument() const {
    return HasRareData() ? data_.rare_data_->argument_ : g_null_atom;
  }
  const CSSSelectorList* SelectorList() const {
    return HasRareData() ? data_.rare_data_->selector_list_.Get() : nullptr;
  }
  // Similar to SelectorList(), but also works for kPseudoParent
  // (i.e., nested selectors); on &, will give the parent's selector list.
  // Will return nullptr if no such list exists (e.g. if we are not a
  // pseudo selector at all), or if we are a & rule that's in a non-nesting
  // context (which is valid, but won't match anything).
  const CSSSelector* SelectorListOrParent() const;
  const Vector<AtomicString>& IdentList() const {
    CHECK(HasRareData() && data_.rare_data_->ident_list_);
    return *data_.rare_data_->ident_list_;
  }
  bool ContainsPseudoInsideHasPseudoClass() const {
    return HasRareData() && data_.rare_data_->bits_.has_.contains_pseudo_;
  }
  bool ContainsComplexLogicalCombinationsInsideHasPseudoClass() const {
    return HasRareData() &&
           data_.rare_data_->bits_.has_.contains_complex_logical_combinations_;
  }
  bool HasArgumentMatchInShadowTree() const {
    return HasRareData() &&
           data_.rare_data_->bits_.has_.argument_match_in_shadow_tree_;
  }

#if DCHECK_IS_ON()
  void Show() const;
  void Show(int indent) const;
#endif  // DCHECK_IS_ON()

  bool IsASCIILower(const AtomicString& value);
  void SetValue(const AtomicString&, bool match_lower_case);
  void SetArgument(const AtomicString&);
  void SetSelectorList(CSSSelectorList*);
  void SetIdentList(std::unique_ptr<Vector<AtomicString>>);
  void SetContainsPseudoInsideHasPseudoClass();
  void SetContainsComplexLogicalCombinationsInsideHasPseudoClass();
  void SetHasArgumentMatchInShadowTree();

  void SetNth(int a, int b, CSSSelectorList* sub_selector);
  bool MatchNth(unsigned count) const;

  static bool IsAdjacentRelation(RelationType relation) {
    return relation == kDirectAdjacent || relation == kIndirectAdjacent;
  }
  bool IsAttributeSelector() const {
    return Match() >= kFirstAttributeSelectorMatch;
  }
  bool IsHostPseudoClass() const {
    return GetPseudoType() == kPseudoHost ||
           GetPseudoType() == kPseudoHostContext;
  }
  // Test for combinations including :host() or :host-context()
  // (See Example 3 under https://drafts.csswg.org/selectors-4/#data-model).
  bool IsOrContainsHostPseudoClass() const;
  bool IsUserActionPseudoClass() const;
  bool IsIdClassOrAttributeSelector() const;

  RelationType Relation() const {
    return static_cast<RelationType>(bits_.get<RelationField>());
  }
  void SetRelation(RelationType relation) {
    bits_.set<RelationField>(relation);
    DCHECK_EQ(Relation(),
              relation);  // using a bitfield.
  }

  MatchType Match() const {
    return static_cast<MatchType>(bits_.get<MatchField>());
  }
  void SetMatch(MatchType match) {
    bits_.set<MatchField>(match);
    DCHECK_EQ(Match(), match);  // using a bitfield.
  }

  bool IsLastInSelectorList() const {
    return bits_.get<IsLastInSelectorListField>();
  }
  void SetLastInSelectorList(bool is_last) {
    bits_.set<IsLastInSelectorListField>(is_last);
  }

  bool IsLastInComplexSelector() const {
    return bits_.get<IsLastInComplexSelectorField>();
  }
  void SetLastInComplexSelector(bool is_last) {
    bits_.set<IsLastInComplexSelectorField>(is_last);
  }

  // https://drafts.csswg.org/selectors/#compound
  bool IsCompound() const;

  enum LinkMatchMask {
    kMatchLink = 1,
    kMatchVisited = 2,
    kMatchAll = kMatchLink | kMatchVisited
  };

  // True if :link or :visited pseudo-classes are found anywhere in
  // the selector.
  bool HasLinkOrVisited() const;

  bool HasRareData() const { return bits_.get<HasRareDataField>(); }

  bool IsForPage() const { return bits_.get<IsForPageField>(); }
  void SetForPage() { bits_.set<IsForPageField>(true); }

  bool IsCoveredByBucketing() const {
    return bits_.get<IsCoveredByBucketingField>();
  }
  void SetCoveredByBucketing(bool value) {
    bits_.set<IsCoveredByBucketingField>(value);
  }

  bool IsScopeContaining() const { return bits_.get<IsScopeContainingField>(); }
  void SetScopeContaining(bool value) {
    bits_.set<IsScopeContainingField>(value);
  }

  bool MatchesPseudoElement() const;
  bool IsAllowedInParentPseudo() const;
  bool IsTreeAbidingPseudoElement() const;
  bool IsElementBackedPseudoElement() const;
  static bool IsElementBackedPseudoElement(CSSSelector::PseudoType pseudo);
  bool IsAllowedAfterPart() const;

  // Returns true if the immediately preceding simple selector is ::slotted.
  bool FollowsSlotted() const;

  // Returns true if any preceding selectors have combinators that cross tree
  // scopes.
  bool CrossesTreeScopes() const;

  // True if the selector was added implicitly. This can happen for e.g.
  // nested rules that would otherwise lack the scoping selector (:scope).
  bool IsImplicit() const { return bits_.get<IsImplicitlyAddedField>(); }

  // Returns true for simple selectors whose evaluation depends on DOM tree
  // position like :first-of-type and :nth-child().
  bool IsChildIndexedSelector() const;

  void Trace(Visitor* visitor) const;

  static String FormatPseudoTypeForDebugging(PseudoType);

 private:
  // Trace() branches on the match/pseudo_type flags,
  // RuleData bucketing sets is_covered_by_bucketing,
  // and these could happen concurrently. This trips up TSan,
  // even though the race is benign, so use an atomic read
  // instead of C++ bitfields.
  using BitField = WTF::ConcurrentlyReadBitField<uint32_t>;
  using RelationField =
      BitField::DefineFirstValue<uint32_t, 4>;  // RelationType
  using MatchField = RelationField::DefineNextValue<uint32_t, 4>;  // MatchType
  using PseudoTypeField =
      MatchField::DefineNextValue<uint32_t, 8>;  // PseudoType
  using IsLastInSelectorListField = PseudoTypeField::DefineNextValue<bool, 1>;
  using IsLastInComplexSelectorField =
      IsLastInSelectorListField::DefineNextValue<bool, 1>;
  using HasRareDataField =
      IsLastInComplexSelectorField::DefineNextValue<bool, 1>;
  using IsForPageField = HasRareDataField::DefineNextValue<bool, 1>;
  using IsImplicitlyAddedField = IsForPageField::DefineNextValue<bool, 1>;

  // If set, we don't need to check this simple selector when matching;
  // it will always match, since we can only see the selector if we
  // checked a given bucket. For instance, if we have a rule like
  // #foo.bar, it will be put in the rule set bucket for #foo
  // (ID selectors are prioritized over nearly everything), and we can
  // mark #foo as covered by bucketing (but still need to check .bar).
  // Of course, this doesn't cover ancestors or siblings; if we have
  // something like .c .c.c, only the two rightmost selectors will get
  // this bit set. Also, we often get into things like namespaces which
  // makes this more conservative than we'd like (bucketing on e.g.
  // tag names do not generally care about it).
  //
  // Furthermore, as a convention, matching such a rule would never set
  // flags in MatchResult.
  //
  // This always starts out false, and is set when we bucket a given
  // RuleData (by calling MarkAsCoveredByBucketing()).
  using IsCoveredByBucketingField =
      IsImplicitlyAddedField::DefineNextValue<bool, 1>;
  // Used for attribute selector (with value). Real type is AttributeMatchType.
  using AttributeMatchField =
      IsCoveredByBucketingField::DefineNextValue<unsigned, 2>;
  using LegacyCaseInsensitiveMatchField =
      AttributeMatchField::DefineNextValue<bool, 1>;
  // Set for any simple CSSSelector which contains either a :scope pseudo-class,
  // or a '&' pseudo-class. The reason '&' is included in the definition
  // of "scope-containing", is that '&' behaves like ':scope' whenever
  // there is no parent rule to refer to.
  //
  // Selectors with this flag set trigger "scope activation" during
  // selector matching, see SelectorChecker::MatchForScopeActivation.
  using IsScopeContainingField = AttributeMatchField::DefineNextValue<bool, 1>;
  // 3 free bits here.
  BitField bits_;
  // 32 padding bits here (on 64-bit platforms).

  void SetPseudoType(PseudoType pseudo_type) {
    bits_.set<PseudoTypeField>(pseudo_type);
    DCHECK_EQ(GetPseudoType(), pseudo_type);  // using a bitfield.
  }

  unsigned SpecificityForOneSelector() const;
  unsigned SpecificityForPage() const;

  template <bool expand_pseudo_references>
  bool SerializeSimpleSelector(WTF::StringBuilder& builder,
                               uintptr_t scope_id) const;

  template <bool expand_pseudo_references>
  const CSSSelector* SerializeCompound(WTF::StringBuilder&,
                                       uintptr_t scope_id) const;

  template <bool expand_pseudo_references>
  static void SerializeSelectorList(const CSSSelectorList* selector_list,
                                    WTF::StringBuilder& builder,
                                    uintptr_t scope_id);

  template <bool expand_pseudo_references>
  String SelectorTextInternal(uintptr_t scope_id) const;

  struct RareData : public GarbageCollected<RareData> {
    explicit RareData(const AtomicString& value);
    // Does not deep copy `selector_list_`.
    RareData(const RareData&);
    ~RareData();

    bool MatchNth(unsigned count);
    int NthAValue() const { return bits_.nth_.a_; }
    int NthBValue() const { return bits_.nth_.b_; }
    // See CSSSelector::Renest.
    RareData* Renest(StyleRule* new_parent);

    AtomicString matching_value_;
    AtomicString serializing_value_;
    union {
      struct {
        int a_;  // Used for :nth-*
        int b_;  // Used for :nth-*
      } nth_;

      struct {
        // Used for :has() with pseudos in its argument. e.g. :has(:hover)
        bool contains_pseudo_;

        // Used for :has() with logical combinations (:is(), :where(), :not())
        // containing complex selector in its argument. e.g. :has(:is(.a .b))
        bool contains_complex_logical_combinations_;

        // Used for :has() next to :host() (e.g. ':host:has(.a)') so that the
        // :has() argument is tested on the elements in shadow tree of the host
        // element.
        bool argument_match_in_shadow_tree_;
      } has_;

      // See GetNestingType.
      CSSNestingType unparsed_nesting_type_;
    } bits_;
    QualifiedName attribute_;  // Used for attribute selector
    AtomicString argument_;    // Used for :contains, :lang, :dir, etc.
    Member<CSSSelectorList>
        selector_list_;  // Used :is, :not, :-webkit-any, etc.
    std::unique_ptr<Vector<AtomicString>>
        ident_list_;  // Used for ::part(), :active-view-transition-type().

    void Trace(Visitor* visitor) const;
  };
  void CreateRareData();

  // The type tag for DataUnion is actually inferred from multiple state
  // variables in the containing CSSSelector using the following rules.
  //
  //  if (Match() == kTag || Match() == kUniversalTag) {
  //     /* data_.tag_q_name_or_attribute_ is valid (is tag_q_name) */
  //  } else if (Match() == kAttributeSet) {
  //     /* data_.tag_q_name_or_attribute_ is valid (is attribute) */
  //  } else if (Match() == kPseudoClass && GetPseudoType() == kPseudoParent) {
  //     /* data_.parent_rule_ is valid */
  //  } else if (HasRareData()) {
  //     /* data_.rare_data_ is valid */
  //  } else {
  //     /* data_.value_ is valid */
  //  }
  //
  // Note that it is important to placement-new and explicitly destruct the
  // fields when shifting between types tags for a DataUnion! Otherwise there
  // will be undefined behavior! This luckily only happens when transitioning
  // from a normal |value_| to a |rare_data_|.
  GC_PLUGIN_IGNORE("crbug.com/1146383")
  union DataUnion {
    enum ConstructUninitializedTag { kConstructUninitialized };
    explicit DataUnion(ConstructUninitializedTag) {}

    enum ConstructEmptyValueTag { kConstructEmptyValue };
    explicit DataUnion(ConstructEmptyValueTag) : value_() {}

    // A string `value` is used by many different selectors to store the string
    // part of the selector. For example the name of a pseudo class (without
    // the colon), the class name of a class selector (without the dot),
    // the attribute of an attribute selector (without the brackets), etc.
    explicit DataUnion(const AtomicString& value) : value_(value) {}

    explicit DataUnion(const QualifiedName& tag_q_name_or_attribute)
        : tag_q_name_or_attribute_(tag_q_name_or_attribute) {}

    explicit DataUnion(const StyleRule* parent_rule)
        : parent_rule_(parent_rule) {}

    explicit DataUnion(Member<RareData> rare_data) : rare_data_(rare_data) {}

    ~DataUnion() {}

    AtomicString value_;

    // For kTag or kUniversalTag, used for tag_q_name. For kAttributeSet, used
    // for the attribute selector if and only if it's a value-less match (for
    // other kAttribute*, no room, and we have to store attribute + value in
    // RareData).
    QualifiedName tag_q_name_or_attribute_;

    Member<RareData> rare_data_;
    Member<const StyleRule> parent_rule_;  // For & (parent in nest).
  } data_;
};

inline const QualifiedName& CSSSelector::Attribute() const {
  DCHECK(IsAttributeSelector());
  if (HasRareData()) {
    return data_.rare_data_->attribute_;
  } else {
    DCHECK_EQ(Match(), kAttributeSet);
    return data_.tag_q_name_or_attribute_;
  }
}

inline CSSSelector::AttributeMatchType CSSSelector::AttributeMatch() const {
  DCHECK(IsAttributeSelector());
  return static_cast<AttributeMatchType>(bits_.get<AttributeMatchField>());
}

inline bool CSSSelector::LegacyCaseInsensitiveMatch() const {
  DCHECK(IsAttributeSelector());
  return bits_.get<LegacyCaseInsensitiveMatchField>();
}

inline bool CSSSelector::IsASCIILower(const AtomicString& value) {
  for (wtf_size_t i = 0; i < value.length(); ++i) {
    if (IsASCIIUpper(value[i])) {
      return false;
    }
  }
  return true;
}

inline void CSSSelector::SetValue(const AtomicString& value,
                                  bool match_lower_case = false) {
  DCHECK_NE(Match(), static_cast<unsigned>(kTag));
  DCHECK_NE(Match(), static_cast<unsigned>(kUniversalTag));
  DCHECK(!(Match() == kPseudoClass && GetPseudoType() == kPseudoParent));
  if (match_lower_case && !HasRareData() && !IsASCIILower(value)) {
    CreateRareData();
  }

  if (!HasRareData()) {
    data_.value_ = value;
    return;
  }
  data_.rare_data_->matching_value_ =
      match_lower_case ? value.LowerASCII() : value;
  data_.rare_data_->serializing_value_ = value;
}

inline CSSSelector::CSSSelector()
    : bits_(RelationField::encode(kSubSelector) | MatchField::encode(kUnknown) |
            PseudoTypeField::encode(kPseudoUnknown) |
            IsLastInSelectorListField::encode(false) |
            IsLastInComplexSelectorField::encode(false) |
            HasRareDataField::encode(false) | IsForPageField::encode(false) |
            IsImplicitlyAddedField::encode(false) |
            IsCoveredByBucketingField::encode(false) |
            AttributeMatchField::encode(0) |
            LegacyCaseInsensitiveMatchField::encode(false) |
            IsScopeContainingField::encode(false)),
      data_(DataUnion::kConstructEmptyValue) {}

inline CSSSelector::CSSSelector(const QualifiedName& tag_q_name,
                                bool tag_is_implicit)
    : bits_(RelationField::encode(kSubSelector) |
            MatchField::encode(
                (tag_q_name == AnyQName() ||
                 tag_q_name.LocalName() == CSSSelector::UniversalSelectorAtom())
                    ? kUniversalTag
                    : kTag) |
            PseudoTypeField::encode(kPseudoUnknown) |
            IsLastInSelectorListField::encode(false) |
            IsLastInComplexSelectorField::encode(false) |
            HasRareDataField::encode(false) | IsForPageField::encode(false) |
            IsImplicitlyAddedField::encode(tag_is_implicit) |
            IsCoveredByBucketingField::encode(false) |
            AttributeMatchField::encode(0) |
            LegacyCaseInsensitiveMatchField::encode(false) |
            IsScopeContainingField::encode(false)),
      data_(tag_q_name) {}

inline CSSSelector::CSSSelector(const StyleRule* parent_rule, bool is_implicit)
    : bits_(RelationField::encode(kSubSelector) |
            MatchField::encode(kPseudoClass) |
            PseudoTypeField::encode(kPseudoParent) |
            IsLastInSelectorListField::encode(false) |
            IsLastInComplexSelectorField::encode(false) |
            HasRareDataField::encode(false) | IsForPageField::encode(false) |
            IsImplicitlyAddedField::encode(is_implicit) |
            IsCoveredByBucketingField::encode(false) |
            AttributeMatchField::encode(0) |
            LegacyCaseInsensitiveMatchField::encode(false) |
            IsScopeContainingField::encode(false)),
      data_(parent_rule) {}

inline CSSSelector::CSSSelector(const AtomicString& pseudo_name,
                                bool is_implicit)
    : bits_(RelationField::encode(kSubSelector) |
            MatchField::encode(kPseudoClass) |
            PseudoTypeField::encode(NameToPseudoType(pseudo_name,
                                                     /* has_arguments */ false,
                                                     /* document */ nullptr)) |
            IsLastInSelectorListField::encode(false) |
            IsLastInComplexSelectorField::encode(false) |
            HasRareDataField::encode(false) | IsForPageField::encode(false) |
            IsImplicitlyAddedField::encode(is_implicit) |
            IsCoveredByBucketingField::encode(false) |
            AttributeMatchField::encode(0) |
            LegacyCaseInsensitiveMatchField::encode(false) |
            IsScopeContainingField::encode(false)),
      data_(pseudo_name) {}

inline CSSSelector::CSSSelector(const CSSSelector& o)
    : bits_(o.bits_), data_(DataUnion::kConstructUninitialized) {
  if (o.Match() == kTag || o.Match() == kUniversalTag ||
      o.Match() == kAttributeSet) {
    new (&data_.tag_q_name_or_attribute_)
        QualifiedName(o.data_.tag_q_name_or_attribute_);
  } else if (o.Match() == kPseudoClass && o.GetPseudoType() == kPseudoParent) {
    data_.parent_rule_ = o.data_.parent_rule_;
  } else if (o.HasRareData()) {
    data_.rare_data_ = o.data_.rare_data_;  // Oilpan-managed.
  } else {
    new (&data_.value_) AtomicString(o.data_.value_);
  }
}

inline CSSSelector::CSSSelector(CSSSelector&& o)
    : data_(DataUnion::kConstructUninitialized) {
  // Seemingly Clang started generating terrible code for the obvious move
  // constructor (i.e., using similar code as in the copy constructor above)
  // after moving to Oilpan, copying the bits one by one. We already allow
  // memcpy + memset by traits, so we can do it by ourselves, too.
  UNSAFE_TODO(memcpy(this, &o, sizeof(*this)));
  UNSAFE_TODO(memset(&o, 0, sizeof(o)));
}

inline CSSSelector::~CSSSelector() {
  if (Match() == kTag || Match() == kUniversalTag || Match() == kAttributeSet) {
    data_.tag_q_name_or_attribute_.~QualifiedName();
  } else if (Match() == kPseudoClass && GetPseudoType() == kPseudoParent)
    ;  // Nothing to do.
  else if (HasRareData())
    ;  // Nothing to do.
  else {
    data_.value_.~AtomicString();
  }
}

inline CSSSelector& CSSSelector::operator=(CSSSelector&& other) {
  this->~CSSSelector();
  new (this) CSSSelector(std::move(other));
  return *this;
}

inline const QualifiedName& CSSSelector::TagQName() const {
  DCHECK(Match() == static_cast<unsigned>(kTag) ||
         Match() == static_cast<unsigned>(kUniversalTag));
  return data_.tag_q_name_or_attribute_;
}

inline const StyleRule* CSSSelector::ParentRule() const {
  DCHECK_EQ(Match(), static_cast<unsigned>(kPseudoClass));
  DCHECK_EQ(GetPseudoType(), kPseudoParent);
  return data_.parent_rule_.Get();
}

inline const AtomicString& CSSSelector::Value() const {
  DCHECK_NE(Match(), static_cast<unsigned>(kTag));
  DCHECK_NE(Match(), static_cast<unsigned>(kUniversalTag));
  if (HasRareData()) {
    return data_.rare_data_->matching_value_;
  }
  return data_.value_;
}

inline const AtomicString& CSSSelector::SerializingValue() const {
  DCHECK_NE(Match(), static_cast<unsigned>(kTag));
  DCHECK_NE(Match(), static_cast<unsigned>(kUniversalTag));
  if (HasRareData()) {
    return data_.rare_data_->serializing_value_;
  }
  return data_.value_;
}

inline bool CSSSelector::IsUserActionPseudoClass() const {
  return GetPseudoType() == kPseudoHover || GetPseudoType() == kPseudoActive ||
         GetPseudoType() == kPseudoFocus || GetPseudoType() == kPseudoDrag ||
         GetPseudoType() == kPseudoFocusWithin ||
         GetPseudoType() == kPseudoFocusVisible;
}

inline bool CSSSelector::IsIdClassOrAttributeSelector() const {
  return IsAttributeSelector() || Match() == CSSSelector::kId ||
         Match() == CSSSelector::kClass;
}

inline void swap(CSSSelector& a, CSSSelector& b) {
  char tmp[sizeof(CSSSelector)];
  UNSAFE_TODO({
    memcpy(tmp, &a, sizeof(CSSSelector));
    memcpy(&a, &b, sizeof(CSSSelector));
    memcpy(&b, tmp, sizeof(CSSSelector));
  });
}

// Converts descendant to relative descendant, child to relative child
// and so on. Subselector is converted to relative descendant.
// All others that don't have a corresponding relative combinator will
// call NOTREACHED().
CSSSelector::RelationType ConvertRelationToRelative(
    CSSSelector::RelationType relation);

// Returns the maximum specificity within a list of selectors. This is typically
// used to calculate the specificity of selectors that have an inner selector
// list, e.g. :is(), :where() etc.
unsigned MaximumSpecificity(const CSSSelector* first_selector);

}  // namespace blink

namespace WTF {
template <>
struct VectorTraits<blink::CSSSelector> : VectorTraitsBase<blink::CSSSelector> {
  static const bool kCanInitializeWithMemset = true;
  static const bool kCanClearUnusedSlotsWithMemset = true;
  static const bool kCanMoveWithMemcpy = true;
};
}  // namespace WTF

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_SELECTOR_H_
