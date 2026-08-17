/*
 * Copyright (C) 1999 Lars Knoll (knoll@kde.org)
 *           (C) 1999 Antti Koivisto (koivisto@kde.org)
 *           (C) 2001 Dirk Mueller (mueller@kde.org)
 *           (C) 2006 Alexey Proskuryakov (ap@webkit.org)
 * Copyright (C) 2004, 2005, 2006, 2007, 2008, 2009, 2010, 2012 Apple Inc. All
 * rights reserved.
 * Copyright (C) 2008, 2009 Torch Mobile Inc. All rights reserved.
 * (http://www.torchmobile.com/)
 * Copyright (C) 2010 Nokia Corporation and/or its subsidiary(-ies)
 * Copyright (C) 2011 Google Inc. All rights reserved.
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
 *
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_STYLE_ENGINE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_STYLE_ENGINE_H_

#include <memory>
#include <utility>

#include "base/auto_reset.h"
#include "base/gtest_prod_util.h"
#include "third_party/blink/public/common/css/forced_colors.h"
#include "third_party/blink/public/mojom/css/preferred_color_scheme.mojom-blink-forward.h"
#include "third_party/blink/public/mojom/frame/color_scheme.mojom-blink-forward.h"
#include "third_party/blink/public/web/web_css_origin.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/css/active_style_sheets.h"
#include "third_party/blink/renderer/core/css/color_scheme_flags.h"
#include "third_party/blink/renderer/core/css/css_global_rule_set.h"
#include "third_party/blink/renderer/core/css/css_to_length_conversion_data.h"
#include "third_party/blink/renderer/core/css/invalidation/pending_invalidations.h"
#include "third_party/blink/renderer/core/css/invalidation/style_invalidator.h"
#include "third_party/blink/renderer/core/css/layout_tree_rebuild_root.h"
#include "third_party/blink/renderer/core/css/pending_sheet_type.h"
#include "third_party/blink/renderer/core/css/resolver/match_request.h"
#include "third_party/blink/renderer/core/css/rule_feature_set.h"
#include "third_party/blink/renderer/core/css/style_image_cache.h"
#include "third_party/blink/renderer/core/css/style_invalidation_root.h"
#include "third_party/blink/renderer/core/css/style_recalc_root.h"
#include "third_party/blink/renderer/core/css/try_value_flips.h"
#include "third_party/blink/renderer/core/css/vision_deficiency.h"
#include "third_party/blink/renderer/core/dom/document.h"
#include "third_party/blink/renderer/core/dom/element.h"
#include "third_party/blink/renderer/core/layout/geometry/axis.h"
#include "third_party/blink/renderer/core/style/position_try_fallbacks.h"
#include "third_party/blink/renderer/platform/bindings/name_client.h"
#include "third_party/blink/renderer/platform/fonts/font_selector_client.h"
#include "third_party/blink/renderer/platform/graphics/color.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_map.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_set.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/loader/fetch/render_blocking_behavior.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace WTF {
class TextPosition;
}  // namespace WTF

namespace blink {

class AnchorEvaluator;
class ComputedStyleBuilder;
class CounterStyle;
class CounterStyleMap;
class StyleContainmentScopeTree;
class CSSFontSelector;
class CSSPropertyValueSet;
class CSSStyleSheet;
class CSSValue;
class Document;
class DocumentStyleSheetCollection;
class ElementRuleCollector;
class Font;
class FontSelector;
class HTMLBodyElement;
class MediaQueryEvaluator;
class MediaQuerySet;
class Node;
class ReferenceFilterOperation;
class RuleFeatureSet;
class ShadowTreeStyleSheetCollection;
class DocumentStyleEnvironmentVariables;
class CascadeLayerMap;
class SpaceSplitString;
class StyleResolver;
class StyleResolverStats;
class StyleRuleFontFace;
class StyleRuleFontPaletteValues;
class StyleRuleKeyframes;
class StyleRuleUsageTracker;
class StyleScopeFrame;
class StyleSheet;
class StyleSheetContents;
class StyleInitialData;
class TextTrack;
class TreeScopeStyleSheetCollection;
class ViewportStyleResolver;
class SelectorFilter;
struct LogicalSize;

enum InvalidationScope { kInvalidateCurrentScope, kInvalidateAllScopes };

using StyleSheetKey = AtomicString;
using UnorderedTreeScopeSet = HeapHashSet<Member<TreeScope>>;

// The StyleEngine class manages style-related state for the document. There is
// a 1-1 relationship of Document to StyleEngine. The document calls the
// StyleEngine when the document is updated in a way that impacts styles.
class CORE_EXPORT StyleEngine final : public GarbageCollected<StyleEngine>,
                                      public FontSelectorClient,
                                      public NameClient {
 public:
  class DOMRemovalScope {
    STACK_ALLOCATED();

   public:
    explicit DOMRemovalScope(StyleEngine& engine)
        : in_removal_(&engine.in_dom_removal_, true) {}

   private:
    base::AutoReset<bool> in_removal_;
  };

  class DetachLayoutTreeScope {
    STACK_ALLOCATED();

   public:
    explicit DetachLayoutTreeScope(StyleEngine& engine)
        : engine_(engine), in_detach_scope_(&engine.in_detach_scope_, true) {}
    ~DetachLayoutTreeScope() {
      engine_.MarkForLayoutTreeChangesAfterDetach();
      engine_.InvalidateSVGResourcesAfterDetach();
    }

   private:
    StyleEngine& engine_;
    base::AutoReset<bool> in_detach_scope_;
  };

  class AttachScrollMarkersScope {
    STACK_ALLOCATED();

   public:
    explicit AttachScrollMarkersScope(StyleEngine& engine)
        : in_scroll_markers_attachment_scope_(
              &engine.in_scroll_markers_attachment_,
              true) {}

   private:
    base::AutoReset<bool> in_scroll_markers_attachment_scope_;
  };

  // There are a few instances where we are marking nodes style dirty from
  // within style recalc. That is generally not allowed, and if allowed we must
  // make sure we mark inside the subtree we are currently traversing, be sure
  // we will traverse the marked node as part of the current traversal. The
  // current instances of this situation is marked with this scope object to
  // skip DCHECKs. Do not introduce new functionality that requires introducing
  // more such scopes.
  class AllowMarkStyleDirtyFromRecalcScope {
    STACK_ALLOCATED();

   public:
    explicit AllowMarkStyleDirtyFromRecalcScope(StyleEngine& engine)
        : allow_marking_(&engine.allow_mark_style_dirty_from_recalc_, true) {}

   private:
    base::AutoReset<bool> allow_marking_;
  };

  // We postpone updating ::first-letter styles until layout tree rebuild to
  // know which text node contains the first letter. If we need to re-attach the
  // ::first-letter element as a result means we mark for re-attachment during
  // layout tree rebuild. That is not generally allowed, and we make sure we
  // explicitly allow it for that case.
  class AllowMarkForReattachFromRebuildLayoutTreeScope {
    STACK_ALLOCATED();

   public:
    explicit AllowMarkForReattachFromRebuildLayoutTreeScope(StyleEngine& engine)
        : allow_marking_(
              &engine.allow_mark_for_reattach_from_rebuild_layout_tree_,
              true) {}

   private:
    base::AutoReset<bool> allow_marking_;
  };

  // Set up the condition for allowing to skip style recalc before starting
  // RecalcStyle().
  class SkipStyleRecalcScope {
    STACK_ALLOCATED();

   public:
    explicit SkipStyleRecalcScope(StyleEngine& engine)
        : allow_skip_(&engine.allow_skip_style_recalc_,
                      engine.AllowSkipStyleRecalcForScope()) {}

   private:
    base::AutoReset<bool> allow_skip_;
  };

  explicit StyleEngine(Document&);
  ~StyleEngine() override;

  const HeapVector<Member<StyleSheet>>& StyleSheetsForStyleSheetList(
      TreeScope&);

  const HeapVector<std::pair<StyleSheetKey, Member<CSSStyleSheet>>>&
  InjectedAuthorStyleSheets() const {
    return injected_author_style_sheets_;
  }

  const HeapVector<Member<CSSStyleSheet>>& InspectorStyleSheets() const {
    return inspector_style_sheet_list_;
  }

  void AddTextTrack(TextTrack*);
  void RemoveTextTrack(TextTrack*);

  // An Element with no tag name, IDs, classes (etc), as described by the
  // WebVTT spec:
  // https://w3c.github.io/webvtt/#obtaining-css-boxes
  //
  // TODO(https://github.com/w3c/webvtt/issues/477): Make originating element
  // actually featureless.
  Element* EnsureVTTOriginatingElement();

  const ActiveStyleSheetVector ActiveStyleSheetsForInspector();

  // All ShadowRoots in the Document.
  const UnorderedTreeScopeSet& GetActiveTreeScopes() const {
    DCHECK_EQ(
        active_tree_scopes_.end(),
        std::find_if(active_tree_scopes_.begin(), active_tree_scopes_.end(),
                     [&, this](const Member<TreeScope>& tree_scope) {
                       return tree_scope.Get() == document_.Get();
                     }))
        << "Document must not appear in active_tree_scopes_";
    return active_tree_scopes_;
  }

  bool NeedsActiveStyleUpdate() const;
  void SetNeedsActiveStyleUpdate(TreeScope&);
  void AddStyleSheetCandidateNode(Node&);
  void RemoveStyleSheetCandidateNode(Node&, ContainerNode& insertion_point);
  void ModifiedStyleSheetCandidateNode(Node&);

  void AdoptedStyleSheetAdded(TreeScope& tree_scope, CSSStyleSheet* sheet);
  void AdoptedStyleSheetRemoved(TreeScope& tree_scope, CSSStyleSheet* sheet);

  void WatchedSelectorsChanged();
  void DocumentRulesSelectorsChanged();
  void InitialStyleChanged();
  void ColorSchemeChanged();
  void SetOwnerColorScheme(
      mojom::blink::ColorScheme color_scheme,
      mojom::blink::PreferredColorScheme preferred_color_scheme);
  mojom::blink::ColorScheme GetOwnerColorScheme() const {
    return owner_color_scheme_;
  }
  mojom::blink::PreferredColorScheme ResolveColorSchemeForEmbedding(
      const ComputedStyle* embedder_style) const;
  void ViewportStyleSettingChanged();

  void InjectSheet(const StyleSheetKey&,
                   StyleSheetContents*,
                   WebCssOrigin = WebCssOrigin::kAuthor);
  void RemoveInjectedSheet(const StyleSheetKey&,
                           WebCssOrigin = WebCssOrigin::kAuthor);
  CSSStyleSheet& CreateInspectorStyleSheet();
  RuleSet* WatchedSelectorsRuleSet() {
    DCHECK(global_rule_set_);
    return global_rule_set_->WatchedSelectorsRuleSet();
  }
  RuleSet* DocumentRulesSelectorsRuleSet() {
    DCHECK(global_rule_set_);
    return global_rule_set_->DocumentRulesSelectorsRuleSet();
  }

  // Helper class for making sure RuleSets that are ensured when collecting
  // sheets for a TreeScope are not shared between two equal sheets which
  // contain @layer rules since anonymous layers need to be unique.
  class RuleSetScope {
    STACK_ALLOCATED();

   public:
    RuleSetScope() = default;

    // Ensure a RuleSet for the passed in css_sheet
    RuleSet* RuleSetForSheet(StyleEngine& engine, CSSStyleSheet* css_sheet);

   private:
    // Keep track of ensured RuleSets with @layer rules to detect
    // StyleSheetContents sharing.
    HeapHashSet<Member<const RuleSet>> layer_rule_sets_;
  };

  RuleSet* RuleSetForSheet(CSSStyleSheet&);
  // See StyleSheetContents::CreateUnconnectedRuleSet.
  //
  // Note that this can return nullptr when the associated media query
  // does not match.
  RuleSet* CreateUnconnectedRuleSet(CSSStyleSheet&);

  // A functional @media query is evaluated as a part of some function
  // during value resolution. This is different from regular media queries,
  // which are evaluated when building the RuleSet.
  bool EvaluateFunctionalMediaQuery(const MediaQuerySet&);
  void MediaQueryAffectingValueChanged(MediaValueChange change);
  void UpdateActiveStyle();

  String PreferredStylesheetSetName() const {
    return preferred_stylesheet_set_name_;
  }
  void SetPreferredStylesheetSetNameIfNotSet(const String&);
  void SetHttpDefaultStyle(const String&);

  void AddPendingBlockingSheet(Node& style_sheet_candidate_node,
                               PendingSheetType type);
  void RemovePendingBlockingSheet(Node& style_sheet_candidate_node,
                                  PendingSheetType type);

  bool HasPendingScriptBlockingSheets() const {
    return pending_script_blocking_stylesheets_ > 0;
  }
  bool HaveScriptBlockingStylesheetsLoaded() const {
    return !HasPendingScriptBlockingSheets();
  }

  unsigned MaxDirectAdjacentSelectors() const {
    return GetRuleFeatureSet()
        .GetRuleInvalidationData()
        .MaxDirectAdjacentSelectors();
  }
  bool UsesFirstLineRules() const {
    return GetRuleFeatureSet().GetRuleInvalidationData().UsesFirstLineRules();
  }
  bool UsesWindowInactiveSelector() const {
    return GetRuleFeatureSet()
        .GetRuleInvalidationData()
        .UsesWindowInactiveSelector();
  }

  // Set when we recalc the style of any element that depends on layout.
  void SetStyleAffectedByLayout() { style_affected_by_layout_ = true; }
  bool StyleAffectedByLayout() { return style_affected_by_layout_; }

  bool StyleMaybeAffectedByLayout(const Element&);

  bool SkippedContainerRecalc() const { return skipped_container_recalc_ != 0; }
  void IncrementSkippedContainerRecalc() { ++skipped_container_recalc_; }
  void DecrementSkippedContainerRecalc() { --skipped_container_recalc_; }

  bool UsesLineHeightUnits() const { return uses_line_height_units_; }
  void SetUsesLineHeightUnits(bool uses_line_height_units) {
    uses_line_height_units_ = uses_line_height_units;
  }

  bool UsesGlyphRelativeUnits() const { return uses_glyph_relative_units_; }
  void SetUsesGlyphRelativeUnits(bool uses_glyph_relative_units) {
    uses_glyph_relative_units_ = uses_glyph_relative_units;
  }

  bool UsesRootFontRelativeUnits() const {
    return uses_root_font_relative_units_;
  }
  void SetUsesRootFontRelativeUnits(bool uses_root_font_relative_units) {
    uses_root_font_relative_units_ = uses_root_font_relative_units;
  }
  bool UpdateRootFontRelativeUnits(const ComputedStyle* old_root_style,
                                   const ComputedStyle* new_root_style);
  void SetUsesTreeCountingFunctions() { uses_tree_counting_functions_ = true; }

  void ResetCSSFeatureFlags(const RuleFeatureSet&);

  bool CountersChanged() const { return counters_changed_; }
  void MarkCountersDirty() { counters_changed_ = true; }
  void MarkCountersClean() { counters_changed_ = false; }
  // Traverse all elements and recalculate counters values.
  void UpdateCounters();

  void ShadowRootInsertedToDocument(ShadowRoot&);
  void ShadowRootRemovedFromDocument(ShadowRoot*);
  void ResetAuthorStyle(TreeScope&);

  StyleResolver& GetStyleResolver() const {
    DCHECK(resolver_);
    return *resolver_;
  }

  StyleContainmentScopeTree& EnsureStyleContainmentScopeTree();
  StyleContainmentScopeTree* GetStyleContainmentScopeTree() const {
    return style_containment_scope_tree_.Get();
  }

  void SetRuleUsageTracker(StyleRuleUsageTracker*);

  const Font* ComputeFont(Element& element,
                          const ComputedStyle& font_style,
                          const CSSPropertyValueSet& font_properties);

  PendingInvalidations& GetPendingNodeInvalidations() {
    return pending_invalidations_;
  }
  // Push all pending invalidations on the document.
  void InvalidateStyle();
  bool HasViewportDependentMediaQueries();
  bool HasViewportDependentPropertyRegistrations();

  class InApplyAnimationUpdateScope {
    STACK_ALLOCATED();

   public:
    explicit InApplyAnimationUpdateScope(StyleEngine& engine)
        : auto_reset_(&engine.in_apply_animation_update_, true) {}

   private:
    base::AutoReset<bool> auto_reset_;
  };

  bool InApplyAnimationUpdate() const { return in_apply_animation_update_; }

  class InEnsureComputedStyleScope {
    STACK_ALLOCATED();

   public:
    explicit InEnsureComputedStyleScope(StyleEngine& engine)
        : auto_reset_(&engine.in_ensure_computed_style_, true) {}

   private:
    base::AutoReset<bool> auto_reset_;
  };

  bool InEnsureComputedStyle() const { return in_ensure_computed_style_; }

  void UpdateStyleInvalidationRoot(ContainerNode* ancestor, Node* dirty_node);
  void UpdateStyleRecalcRoot(ContainerNode* ancestor, Node* dirty_node);
  void UpdateLayoutTreeRebuildRoot(ContainerNode* ancestor, Node* dirty_node);

  enum class AncestorAnalysis {
    // There is no dirtiness in the ancestor chain.
    kNone,
    // There is an interleaving root (i.e. a size query container) in the
    // ancestor chain.
    kInterleavingRoot,
    // There is a style recalc root or style invalidation root in the ancestor
    // chain.
    kStyleRoot,
  };

  // Analyze the inclusive flat-tree ancestors of the given node to understand
  // whether or not the next call to UpdateStyleAndLayoutTree would affect
  // that node.
  //
  // This is useful for avoiding unnecessary forced style updates when we
  // only care about the style of a specific node, e.g. getComputedStyle(e).
  AncestorAnalysis AnalyzeAncestors(const Node&);

  bool MarkReattachAllowed() const;
  bool MarkStyleDirtyAllowed() const;

  // Returns true if we can skip style recalc for a size container subtree and
  // resume it during layout.
  bool SkipStyleRecalcAllowed() const { return allow_skip_style_recalc_; }

  CSSFontSelector* GetFontSelector() { return font_selector_.Get(); }

  void RemoveFontFaceRules(const HeapVector<Member<const StyleRuleFontFace>>&);
  // updateGenericFontFamilySettings is used from WebSettingsImpl.
  void UpdateGenericFontFamilySettings();

  void DidDetach();

  CSSStyleSheet* CreateSheet(Element&,
                             const String& text,
                             WTF::TextPosition start_position,
                             PendingSheetType type,
                             RenderBlockingBehavior render_blocking_behavior);

  void CollectFeaturesTo(RuleFeatureSet& features);

  void EnsureUAStyleForFullscreen(const Element&);
  void EnsureUAStyleForElement(const Element&);
  void EnsureUAStyleForPseudoElement(PseudoId);
  void EnsureUAStyleForForcedColors();

  void PlatformColorsChanged();

  bool HasRulesForId(const AtomicString& id) const;
  void ClassChangedForElement(const SpaceSplitString& changed_classes,
                              Element&);
  void ClassChangedForElement(const SpaceSplitString& old_classes,
                              const SpaceSplitString& new_classes,
                              Element&);
  void AttributeChangedForElement(const QualifiedName& attribute_name,
                                  Element&);
  void IdChangedForElement(const AtomicString& old_id,
                           const AtomicString& new_id,
                           Element&);
  void PseudoStateChangedForElement(CSSSelector::PseudoType,
                                    Element&,
                                    bool invalidate_descendants_or_siblings,
                                    bool invalidate_ancestors_or_siblings);
  void PartChangedForElement(Element&);
  void ExportpartsChangedForElement(Element&);

  void ScheduleSiblingInvalidationsForElement(Element&,
                                              ContainerNode& scheduling_parent,
                                              unsigned min_direct_adjacent);
  void ScheduleInvalidationsForInsertedSibling(Element* before_element,
                                               Element& inserted_element);
  void ScheduleInvalidationsForRemovedSibling(Element* before_element,
                                              Element& removed_element,
                                              Element& after_element);
  void ScheduleNthPseudoInvalidations(ContainerNode&);
  void ApplyRuleSetInvalidationForTreeScope(
      TreeScope&,
      ContainerNode&,
      SelectorFilter&,
      StyleScopeFrame&,
      const HeapHashSet<Member<RuleSet>>&,
      unsigned changed_rule_flags,
      InvalidationScope = kInvalidateCurrentScope);
  void ApplyRuleSetInvalidationForSubtree(
      TreeScope&,
      Element&,
      SelectorFilter&,
      StyleScopeFrame& parent_style_scope_frame,
      const HeapHashSet<Member<RuleSet>>&,
      unsigned changed_rule_flags,
      InvalidationScope,
      bool invalidate_slotted,
      bool invalidate_part);
  void ScheduleCustomElementInvalidations(HashSet<AtomicString> tag_names);
  void ScheduleInvalidationsForHasPseudoAffectedByInsertionOrRemoval(
      ContainerNode* parent,
      Node* node_before_change,
      Element& changed_element,
      bool removal);
  void ScheduleInvalidationsForHasPseudoWhenAllChildrenRemoved(Element& parent);

  void NodeWillBeRemoved(Node&);
  void ChildrenRemoved(ContainerNode& parent);
  void FlatTreePositionChanged(Node& node) {
    style_recalc_root_.FlatTreePositionChanged(node);
  }
  void PseudoElementRemoved(Element& originating_element) {
    layout_tree_rebuild_root_.SubtreeModified(originating_element);
  }
  // Do necessary invalidations, which are not covered by a style recalc, for a
  // body element which changed between being the document.body element and not.
  void FirstBodyElementChanged(HTMLBodyElement*);

  // Invalidate caches that depends on the base url.
  void BaseURLChanged();

  unsigned StyleForElementCount() const { return style_for_element_count_; }
  void IncStyleForElementCount() { style_for_element_count_++; }

  StyleResolverStats* Stats() { return style_resolver_stats_.get(); }
  void SetStatsEnabled(bool);

  void ApplyRuleSetChanges(TreeScope&,
                           const ActiveStyleSheetVector& old_style_sheets,
                           const ActiveStyleSheetVector& new_style_sheets,
                           const HeapVector<Member<RuleSetDiff>>& diffs);
  void ApplyUserRuleSetChanges(const ActiveStyleSheetVector& old_style_sheets,
                               const ActiveStyleSheetVector& new_style_sheets);

  void VisionDeficiencyChanged();
  void ApplyVisionDeficiencyStyle(
      ComputedStyleBuilder& layout_view_style_builder);

  void CollectMatchingUserRules(ElementRuleCollector&);

  void PropertyRegistryChanged();

  void EnvironmentVariableChanged();
  void InvalidateEnvDependentStylesIfNeeded();

  bool HasComplexSafaAreaConstraints();
  void SetNeedsToUpdateComplexSafeAreaConstraints();

  void MarkAllElementsForStyleRecalc(const StyleChangeReasonForTracing& reason);
  void MarkViewportStyleDirty();
  bool IsViewportStyleDirty() const { return viewport_style_dirty_; }

  void MarkViewportUnitDirty(ViewportUnitFlag);
  void InvalidateViewportUnitStylesIfNeeded();

  void MarkFontsNeedUpdate();
  void InvalidateStyleAndLayoutForFontUpdates();

  void MarkCounterStylesNeedUpdate();
  void UpdateCounterStyles();

  // Set a flag to invalidate elements using position-try-fallbacks on next
  // lifecycle update when @position-try rules are added or removed.
  void MarkPositionTryStylesDirty(
      const HeapHashSet<Member<RuleSet>>& changed_rule_sets);

  // Mark elements affected by @position-try rules for style and layout update.
  void InvalidatePositionTryStyles();

  void MarkLastSuccessfulPositionFallbackDirtyForElement(Element& element) {
    anchored_element_dirty_set_.insert(&element);
  }
  void MarkForDefaultAnchorScrollShift(Element& element) {
    anchored_element_dirty_set_.insert(&element);
  }

  StyleRuleKeyframes* KeyframeStylesForAnimation(
      const AtomicString& animation_name);

  StyleRuleFontPaletteValues* FontPaletteValuesForNameAndFamily(
      AtomicString palette_name,
      AtomicString font_family);

  CounterStyleMap* GetUserCounterStyleMap() {
    return user_counter_style_map_.Get();
  }
  const CounterStyle& FindCounterStyleAcrossScopes(const AtomicString&,
                                                   const TreeScope*) const;

  // Resolve a tree-scoped reference to a @function rule.
  //
  // https://drafts.csswg.org/css-mixins-1/#function-rule
  // https://drafts.csswg.org/css-scoping-1/#css-tree-scoped-reference
  std::pair<StyleRuleFunction*, const TreeScope*> FindFunctionAcrossScopes(
      const AtomicString& name,
      const TreeScope*) const;

  const CascadeLayerMap* GetUserCascadeLayerMap() const {
    return user_cascade_layer_map_.Get();
  }

  DocumentStyleEnvironmentVariables& EnsureEnvironmentVariables();

  StyleInitialData* MaybeCreateAndGetInitialData();

  bool NeedsStyleInvalidation() const {
    return style_invalidation_root_.GetRootNode();
  }
  bool NeedsStyleRecalc() const { return style_recalc_root_.GetRootNode(); }
  bool NeedsLayoutTreeRebuild() const {
    return layout_tree_rebuild_root_.GetRootNode();
  }
  bool NeedsFullStyleUpdate() const;

  void UpdateViewport();
  void UpdateViewportStyle();
  void UpdateStyleAndLayoutTree();
  // To be called from layout when container queries change for the container.
  void UpdateStyleAndLayoutTreeForContainer(Element& container,
                                            const LogicalSize&,
                                            LogicalAxes contained_axes);
  // To be called from layout-tree building for subtree skipped for style
  // recalcs when we found out the container is eligible for size containment
  // after all.
  void UpdateStyleForNonEligibleContainer(Element& container);
  // Updates the style of `element`, and descendants if needed.
  // The provided `try_set` represents the declaration block from
  // a @position-try rule. The specified TryTacticList will cause
  // CSSFlipRevertValues to appear in the try-tactics layer (see
  // OutOfFlowData::try_tactics_set_).
  void UpdateStyleForOutOfFlow(Element& element,
                               const CSSPropertyValueSet* try_set,
                               const TryTacticList&,
                               AnchorEvaluator*);
  StyleRulePositionTry* GetPositionTryRule(const ScopedCSSName&);
  void RecalcStyle();

  void ClearEnsuredDescendantStyles(Element& element);
  void RebuildLayoutTree(Element* size_container = nullptr);
  bool InRebuildLayoutTree() const { return in_layout_tree_rebuild_; }
  bool InDOMRemoval() const { return in_dom_removal_; }
  bool InDetachLayoutTree() const { return in_detach_scope_; }
  bool InContainerQueryStyleRecalc() const {
    return in_container_query_style_recalc_;
  }
  bool InPositionTryStyleRecalc() const {
    return in_position_try_style_recalc_;
  }
  void SetInScrollMarkersAttachment(bool in_scroll_markers_attachment) {
    DCHECK(!in_scroll_markers_attachment_ || !in_scroll_markers_attachment);
    in_scroll_markers_attachment_ = in_scroll_markers_attachment;
  }
  bool InScrollMarkersAttachment() const {
    return in_scroll_markers_attachment_;
  }
  // Get the root element of an interleaving recalc, if any. This function will
  // return nullptr if the interleaving root is a PseudoElement, because such
  // elements can't be recalc roots.
  //
  // See StyleEngine::UpdateStyleAndLayoutTreeForContainer.
  // See StyleEngine::UpdateStyleForOutOfFlow.
  Element* GetInterleavingRecalcRoot() const {
    if (InContainerQueryStyleRecalc() || InPositionTryStyleRecalc()) {
      // During interleaved style recalc, the recalc root is either set
      // to the interleaving root (always an Element), or nullptr (if it's
      // a PseudoElement).
      return To<Element>(style_recalc_root_.GetRootNode());
    }
    return nullptr;
  }
  void DetachedFromParent(LayoutObject* parent) {
    // This method will be called for every LayoutObject while detaching a
    // subtree. Since the trees are detached bottom up, the last parent passed
    // in will be the parent of one of the roots being detached.
#if DCHECK_IS_ON()
    DCHECK(!in_dom_removal_ || in_detach_scope_)
        << "A DOMRemovalScope must use a DetachLayoutTreeScope to handle "
           "whitespace and list-item corrections in "
           "MarkForLayoutTreeChangesAfterLayout when LayoutObjects are "
           "detached";
#endif  // DCHECK_IS_ON()
    if (in_detach_scope_) {
      parent_for_detached_subtree_ = parent;
    }
  }

  void SetPageColorSchemes(const CSSValue* color_scheme);
  ColorSchemeFlags GetPageColorSchemes() const { return page_color_schemes_; }
  mojom::PreferredColorScheme GetPreferredColorScheme() const {
    return preferred_color_scheme_;
  }
  bool GetForceDarkModeEnabled() const { return force_dark_mode_enabled_; }
  ForcedColors GetForcedColors() const { return forced_colors_; }
  void UpdateColorSchemeBackground(bool color_scheme_changed = false);
  Color ForcedBackgroundColor() const { return forced_background_color_; }
  Color ColorAdjustBackgroundColor() const;

  ImageResourceContent* CacheImageContent(FetchParameters& params) {
    return style_image_cache_.CacheImageContent(GetDocument().Fetcher(),
                                                params);
  }

  void AddCachedFillOrClipPathURIValue(const AtomicString& string,
                                       const CSSValue& value);
  const CSSValue* GetCachedFillOrClipPathURIValue(const AtomicString& string);

  void Trace(Visitor*) const override;
  const char* NameInHeapSnapshot() const override { return "StyleEngine"; }

  RuleSet* DefaultViewTransitionStyle(const Element&) const;

  void UpdateViewTransitionOptIn();

  const ActiveStyleSheetVector& ActiveUserStyleSheets() const {
    return active_user_style_sheets_;
  }

  // See comment on viewport_size_.
  void UpdateViewportSize();
  const CSSToLengthConversionData::ViewportSize& GetViewportSize() const {
    DCHECK(viewport_size_ == CSSToLengthConversionData::ViewportSize(
                                 GetDocument().GetLayoutView()));
    return viewport_size_;
  }

  // Returns true if marked dirty for layout
  bool UpdateLastSuccessfulPositionFallbacksAndAnchorScrollShift();

  void RevisitStyleSheetForInspector(StyleSheetContents* contents,
                                     const RuleFeatureSet* features) const;

 private:
  void UpdateCounters(const Element& element,
                      CountersAttachmentContext& context);
  void UpdateLayoutCounters(const LayoutObject& layout_object,
                            CountersAttachmentContext& context);
  // FontSelectorClient implementation.
  void FontsNeedUpdate(FontSelector*, FontInvalidationReason) override;

  AncestorAnalysis AnalyzeInclusiveAncestor(const Node&);
  AncestorAnalysis AnalyzeExclusiveAncestor(const Node&);

  void LoadVisionDeficiencyFilter();

  bool NeedsActiveStyleSheetUpdate() const {
    return tree_scopes_removed_ || document_scope_dirty_ ||
           dirty_tree_scopes_.size() || user_style_dirty_;
  }

  TreeScopeStyleSheetCollection& EnsureStyleSheetCollectionFor(TreeScope&);
  TreeScopeStyleSheetCollection* StyleSheetCollectionFor(TreeScope&);
  bool ShouldUpdateDocumentStyleSheetCollection() const;
  bool ShouldUpdateShadowTreeStyleSheetCollection() const;

  void MarkDocumentDirty();
  void MarkTreeScopeDirty(TreeScope&);
  void MarkUserStyleDirty();

  Document& GetDocument() const { return *document_; }

  bool MediaQueryAffectingValueChanged(const ActiveStyleSheetVector&,
                                       MediaValueChange);
  void MediaQueryAffectingValueChanged(TreeScope&, MediaValueChange);
  void MediaQueryAffectingValueChanged(UnorderedTreeScopeSet&,
                                       MediaValueChange);
  void MediaQueryAffectingValueChanged(HeapHashSet<Member<TextTrack>>&,
                                       MediaValueChange);
  void EnsureUAStyleForTransitionPseudos();

  const RuleFeatureSet& GetRuleFeatureSet() const {
    DCHECK(global_rule_set_);
    return global_rule_set_->GetRuleFeatureSet();
  }

  void ClearResolvers();

  void CollectUserStyleFeaturesTo(RuleFeatureSet&) const;
  void CollectScopedStyleFeaturesTo(RuleFeatureSet&) const;

  CSSStyleSheet* ParseSheet(Element&,
                            const String& text,
                            WTF::TextPosition start_position,
                            RenderBlockingBehavior render_blocking_behavior);

  const DocumentStyleSheetCollection& GetDocumentStyleSheetCollection() const {
    DCHECK(document_style_sheet_collection_);
    return *document_style_sheet_collection_;
  }

  DocumentStyleSheetCollection& GetDocumentStyleSheetCollection() {
    DCHECK(document_style_sheet_collection_);
    return *document_style_sheet_collection_;
  }

  void UpdateActiveStyleSheetsInShadow(
      TreeScope*,
      UnorderedTreeScopeSet& tree_scopes_removed);

  bool ShouldSkipInvalidationFor(const Element&) const;
  bool IsSubtreeAndSiblingsStyleDirty(const Element&) const;
  void ApplyRuleSetInvalidationForElement(
      const TreeScope& tree_scope,
      Element& element,
      SelectorFilter& selector_filter,
      StyleScopeFrame& style_scope_frame,
      const HeapHashSet<Member<RuleSet>>& rule_sets,
      unsigned changed_rule_flags,
      bool is_shadow_host);
  void InvalidateSlottedElements(HTMLSlotElement&,
                                 const StyleChangeReasonForTracing&);
  void InvalidateForRuleSetChanges(
      TreeScope& tree_scope,
      const HeapHashSet<Member<RuleSet>>& changed_rule_sets,
      unsigned changed_rule_flags,
      InvalidationScope invalidation_scope);
  void InvalidateInitialData();

  void UpdateActiveUserStyleSheets();
  void UpdateActiveStyleSheets();
  void UpdateGlobalRuleSet() {
    DCHECK(!NeedsActiveStyleSheetUpdate());
    if (global_rule_set_) {
      global_rule_set_->Update(GetDocument());
    }
  }
  const MediaQueryEvaluator& EnsureMediaQueryEvaluator();
  void UpdateStyleSheetList(TreeScope&);

  // Returns true if any @font-face rules are added or removed.
  bool ClearFontFaceCacheAndAddUserFonts(
      const ActiveStyleSheetVector& user_sheets);

  void ClearKeyframeRules();
  void ClearPropertyRules();

  class AtRuleCascadeMap;

  void AddPropertyRulesFromSheets(AtRuleCascadeMap&,
                                  const ActiveStyleSheetVector&,
                                  bool is_user_style);
  void AddFontPaletteValuesRulesFromSheets(
      const ActiveStyleSheetVector& sheets);

  // Returns true if any @font-face rules are added.
  bool AddUserFontFaceRules(const RuleSet&);
  void AddUserKeyframeRules(const RuleSet&);
  void AddUserKeyframeStyle(StyleRuleKeyframes*);
  void AddFontPaletteValuesRules(const RuleSet& rule_set);
  void AddFontFeatureValuesRules(const RuleSet& rule_set);
  void AddPropertyRules(AtRuleCascadeMap&, const RuleSet&, bool is_user_style);
  bool UserKeyframeStyleShouldOverride(
      const StyleRuleKeyframes* new_rule,
      const StyleRuleKeyframes* existing_rule) const;
  void AddViewTransitionRules(const ActiveStyleSheetVector& sheets);

  CounterStyleMap& EnsureUserCounterStyleMap();

  void UpdateColorScheme();
  bool SupportsDarkColorScheme();
  void UpdateForcedBackgroundColor();

  void UpdateColorSchemeMetrics();

  void ViewportDefiningElementDidChange();
  void PropagateWritingModeAndDirectionToHTMLRoot();

  void RecalcStyle(StyleRecalcChange, const StyleRecalcContext&);
  void RecalcStyleForContainer(Element& container, StyleRecalcChange change);
  bool RecalcHighlightStylesForContainer(Element& container);
  void RecalcPositionTryStyleForPseudoElement(PseudoElement& pseudo_element,
                                              const StyleRecalcChange,
                                              const StyleRecalcContext&);

  void RecalcTransitionPseudoStyle();
  void RebuildTransitionPseudoLayoutTrees();

  // We may need to update whitespaces in the layout tree after a flat tree
  // removal which caused a layout subtree to be detached.
  void MarkForLayoutTreeChangesAfterDetach();

  // SVG resource may have had their layout object detached and need to notify
  // any clients about this.
  void InvalidateSVGResourcesAfterDetach();

  void RebuildLayoutTreeForTraversalRootAncestors(Element* parent,
                                                  Element* container_parent);

  // Separate path for layout tree rebuild for re-attaching children of a
  // fieldset size query container, or a size query container which must use
  // legacy layout fallback, during layout.
  void ReattachContainerSubtree(Element& container);

  // Invalidate ancestors or siblings affected by :has() state change
  inline void InvalidateElementAffectedByHas(
      Element&,
      bool for_element_affected_by_pseudo_in_has);
  class PseudoHasInvalidationTraversalContext;
  inline void InvalidateAncestorsOrSiblingsAffectedByHas(
      const PseudoHasInvalidationTraversalContext&);
  // Invalidate changed element affected by logical combinations in :has()
  inline void InvalidateChangedElementAffectedByLogicalCombinationsInHas(
      Element& changed_element,
      bool for_element_affected_by_pseudo_in_has);
  void ScheduleInvalidationsForHasPseudoAffectedByInsertion(
      Element* parent_or_shadow_host,
      Element* previous_sibling,
      Element& inserted_element,
      bool insert_shadow_root_child);
  void ScheduleInvalidationsForHasPseudoAffectedByRemoval(
      Element* parent_or_shadow_host,
      Element* previous_sibling,
      Element& removed_element,
      bool remove_shadow_root_child);

  // Initialization value for SkipStyleRecalcScope.
  bool AllowSkipStyleRecalcForScope() const;

  // See EvaluateFunctionalMediaQuery
  void InvalidateFunctionalMediaDependentStylesIfNeeded();

  Member<Document> document_;

  // Tree of style containment scopes. Is in charge of the document's quotes.
  Member<StyleContainmentScopeTree> style_containment_scope_tree_;

  // Tracks the number of currently loading top-level stylesheets. Sheets loaded
  // using the @import directive are not included in this count. We use this
  // count of pending sheets to detect when it is safe to execute scripts
  // (parser-inserted scripts may not run until all pending stylesheets have
  // loaded). See:
  // https://html.spec.whatwg.org/multipage/semantics.html#interactions-of-styling-and-scripting
  int pending_script_blocking_stylesheets_{0};

  // Tracks the number of currently loading top-level stylesheets which block
  // the HTML parser. Sheets loaded using the @import directive are not included
  // in this count. Once all of these sheets have loaded, the parser may
  // continue.
  int pending_parser_blocking_stylesheets_{0};

  // Stylesheets inserted by DevTools.
  HeapVector<Member<CSSStyleSheet>> inspector_style_sheet_list_;

  Member<DocumentStyleSheetCollection> document_style_sheet_collection_;

  Member<StyleRuleUsageTracker> tracker_;

  using StyleSheetCollectionMap =
      HeapHashMap<WeakMember<TreeScope>,
                  Member<ShadowTreeStyleSheetCollection>>;
  StyleSheetCollectionMap style_sheet_collection_map_;

  bool document_scope_dirty_{true};
  bool tree_scopes_removed_{false};
  bool user_style_dirty_{false};
  UnorderedTreeScopeSet dirty_tree_scopes_;
  UnorderedTreeScopeSet active_tree_scopes_;

  String preferred_stylesheet_set_name_;

  // Flag to track counter changes in the document, indicating
  // the need to call UpdateCounters.
  bool counters_changed_{false};

  bool uses_root_font_relative_units_{false};
  bool uses_glyph_relative_units_{false};
  bool uses_line_height_units_{false};
  // True if we ever resolved style that involved tree-counting functions such
  // as sibling-count() and sibling-index().
  bool uses_tree_counting_functions_{false};
  // True if we have performed style recalc for at least one element that
  // depends on container queries.
  bool style_affected_by_layout_{false};
  // The number of elements currently in a skipped style recalc state.
  //
  // Style recalc can be skipped for an element [1] if its style depends on
  // the size, which can be the case for container queries. This number is
  // used to understand whether or not we need to upgrade [2] a call to
  // UpdateStyleAndLayoutTree* to also include layout.
  //
  // [1] Element::SkipStyleRecalcForContainer.
  // [2] LayoutUpgrade
  int64_t skipped_container_recalc_{0};
  bool in_layout_tree_rebuild_{false};
  bool in_container_query_style_recalc_{false};
  bool in_position_try_style_recalc_{false};
  bool in_scroll_markers_attachment_{false};
  bool in_dom_removal_{false};
  bool in_detach_scope_{false};
  bool in_apply_animation_update_{false};
  bool in_ensure_computed_style_{false};
  bool viewport_style_dirty_{false};
  bool fonts_need_update_{false};
  bool counter_styles_need_update_{false};
  bool position_try_styles_dirty_{false};

  // Set to true if we allow marking style dirty from style recalc. Ideally, we
  // should get rid of this, but we keep track of where we allow it with
  // AllowMarkStyleDirtyFromRecalcScope.
  bool allow_mark_style_dirty_from_recalc_{false};

  // Set to true if we allow marking for reattachment from layout tree rebuild.
  // AllowMarkStyleDirtyFromRecalcScope.
  bool allow_mark_for_reattach_from_rebuild_layout_tree_{false};

  // Set to true if we are allowed to skip recalc for a size container subtree.
  bool allow_skip_style_recalc_{false};

  // See enum ViewportUnitFlag.
  unsigned viewport_unit_dirty_flags_{0};

  // True if some data backing env() has changed.
  bool is_env_dirty_{false};

  // Flags collected from all calls to EvaluateFunctionalMediaQuery.
  MediaQueryResultFlags functional_media_query_result_flags_;

  // True if the document has elements referencing env(safe-area-inset-bottom)
  bool has_complex_safe_area_constraints_{false};

  // True if |has_complex_safe_area_constraints_| should be recomputed.
  // This is set to true during style cascade when an element references
  // env(safe-area-inset-bottom), and remains true as long as there are
  // elements in the document with complex safe area constraints.
  bool needs_to_update_complex_safe_area_constraints_{false};

  VisionDeficiency vision_deficiency_{VisionDeficiency::kNoVisionDeficiency};
  Member<ReferenceFilterOperation> vision_deficiency_filter_;

  Member<StyleResolver> resolver_;
  Member<ViewportStyleResolver> viewport_resolver_;
  Member<MediaQueryEvaluator> media_query_evaluator_;
  Member<CSSGlobalRuleSet> global_rule_set_;

  PendingInvalidations pending_invalidations_;

  StyleInvalidationRoot style_invalidation_root_;
  StyleRecalcRoot style_recalc_root_;
  LayoutTreeRebuildRoot layout_tree_rebuild_root_;

  Member<CSSFontSelector> font_selector_;

  HeapHashMap<AtomicString, WeakMember<StyleSheetContents>>
      text_to_sheet_cache_;

  std::unique_ptr<StyleResolverStats> style_resolver_stats_;
  unsigned style_for_element_count_{0};

  HeapVector<std::pair<StyleSheetKey, Member<CSSStyleSheet>>>
      injected_user_style_sheets_;
  HeapVector<std::pair<StyleSheetKey, Member<CSSStyleSheet>>>
      injected_author_style_sheets_;

  ActiveStyleSheetVector active_user_style_sheets_;
  HeapVector<RuleSetGroup> user_rule_set_groups_;

  using KeyframesRuleMap =
      HeapHashMap<AtomicString, Member<StyleRuleKeyframes>>;
  KeyframesRuleMap keyframes_rule_map_;

  // Combined key consisting of the rule's name and the case-folded font-family
  // name to which this @font-palette-values rule selectively applies to.
  using FontPaletteValuesRuleMap =
      HeapHashMap<std::pair<AtomicString, String>,
                  Member<StyleRuleFontPaletteValues>>;
  FontPaletteValuesRuleMap font_palette_values_rule_map_;

  Member<CounterStyleMap> user_counter_style_map_;

  Member<CascadeLayerMap> user_cascade_layer_map_;

  Member<DocumentStyleEnvironmentVariables> environment_variables_;

  Member<StyleInitialData> initial_data_;

  // Page color schemes set by the viewport meta tag. E.g.
  // <meta name="color-scheme" content="light dark">.
  ColorSchemeFlags page_color_schemes_ =
      static_cast<ColorSchemeFlags>(ColorSchemeFlag::kNormal);

  // The preferred color-scheme is set from Settings for the main frame
  // document and from the owner_preferred_color_scheme_ for iframes, but may be
  // overridden by the ForceDarkMode setting where the preferred_color_scheme_
  // will be set to kLight to avoid dark styling to be applied before auto
  // darkening.
  //
  // This data member should be initialized in the constructor in order to avoid
  // including full mojom headers from this header.
  mojom::blink::PreferredColorScheme preferred_color_scheme_;

  // The preferred color-scheme to be used for this document if it is an iframe.
  // It is inferred from the used color-scheme of the FrameOwner element, the
  // FrameOwner element's page's supported color-schemes, and OS/Browser setting
  // preferred color-scheme.
  //
  // This data member should be initialized in the constructor in order to avoid
  // including full mojom headers from this header.
  mojom::blink::PreferredColorScheme owner_preferred_color_scheme_;

  // We pass the used value of color-scheme from the iframe element in the
  // embedding document. If the color-scheme of the owner element and the root
  // element in the embedded document differ, use a solid backdrop color instead
  // of the default transparency of an iframe.
  //
  // This data member should be initialized in the constructor in order to avoid
  // including full mojom headers from this header.
  mojom::blink::ColorScheme owner_color_scheme_;

  // The color of the canvas backdrop for the used color-scheme.
  Color color_scheme_background_;

  // Forced colors is set in WebThemeEngine.
  ForcedColors forced_colors_{ForcedColors::kNone};

  Color forced_background_color_;

  bool force_dark_mode_enabled_{false};

  friend class NodeTest;
  friend class StyleEngineTest;
  friend class WhitespaceAttacherTest;
  friend class StyleCascadeTest;
  friend class StyleImageCacheTest;
  FRIEND_TEST_ALL_PREFIXES(BlockChildIteratorTest, DeleteNodeWhileIteration);

  HeapHashSet<Member<TextTrack>> text_tracks_;
  Member<Element> vtt_originating_element_;

  // When removing subtrees from the flat tree DOM, whitespace siblings of the
  // root may need to be updated. The LayoutObject parent of the detached
  // subtree is stored here during in_dom_removal_ and is marked for whitespace
  // re-attachment after the removal.
  Member<LayoutObject> parent_for_detached_subtree_;

  // The @view-transition rule currently applying to the document.
  Member<StyleRuleViewTransition> view_transition_rule_;

  // Cache for sharing ImageResourceContent between CSSValues referencing the
  // same URL.
  StyleImageCache style_image_cache_;

  // A cache for CSSURIValue objects for SVG element presentation attributes for
  // fill and clip path. See SVGElement::CollectStyleForPresentationAttribute()
  // for more info.
  HeapHashMap<AtomicString, WeakMember<const CSSValue>>
      fill_or_clip_path_uri_value_cache_;

  // Cached because it can be expensive to compute anew for each element.
  // You must call UpdateViewportSize() once before resolving style.
  CSSToLengthConversionData::ViewportSize viewport_size_;

  // Stores various "flip sets" used to implement <try-tactic> from
  // CSS Anchor Positioning.
  TryValueFlips try_value_flips_;

  // Elements which had their computed position-try-fallbacks changed since last
  // time resize observers were considered. May need to have their last
  // successful option invalidated.
  HeapHashSet<Member<Element>> anchored_element_dirty_set_;

  // Names of @position-try rules which were added, removed, or modified since
  // last time resize observers were considered. Anchored elements with a last
  // successful option with position-try-fallbacks referring any of these names
  // will be invalidated.
  HashSet<AtomicString> dirty_position_try_names_;

  // Maps a functional media query to its evaluated result. When media values
  // change, this can be used to check if we need to invalidate any elements
  // as a response to that.
  //
  // Note that MediaQuerySets are cached during parsing [1], so identical @media
  // preludes are represented by the same pointer.
  //
  // [1] CSSParserImpl::ConsumeMediaRule
  HeapHashMap<Member<const MediaQuerySet>, bool>
      functional_media_query_results_;
};

void PossiblyScheduleNthPseudoInvalidations(Node& node);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_STYLE_ENGINE_H_
