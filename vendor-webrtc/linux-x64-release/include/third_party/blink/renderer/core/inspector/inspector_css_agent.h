/*
 * Copyright (C) 2010, Google Inc. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1.  Redistributions of source code must retain the above copyright
 *     notice, this list of conditions and the following disclaimer.
 * 2.  Redistributions in binary form must reproduce the above copyright
 *     notice, this list of conditions and the following disclaimer in the
 *     documentation and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY APPLE INC. AND ITS CONTRIBUTORS ``AS IS'' AND
 * ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
 * ARE DISCLAIMED. IN NO EVENT SHALL APPLE INC. OR ITS CONTRIBUTORS BE LIABLE
 * FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
 * DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
 * SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER
 * CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT
 * LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY
 * OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH
 * DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_INSPECTOR_INSPECTOR_CSS_AGENT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_INSPECTOR_INSPECTOR_CSS_AGENT_H_

#include "base/memory/scoped_refptr.h"
#include "third_party/blink/renderer/core/animation/keyframe_effect.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/css/css_condition_rule.h"
#include "third_party/blink/renderer/core/css/css_import_rule.h"
#include "third_party/blink/renderer/core/css/css_keyframes_rule.h"
#include "third_party/blink/renderer/core/css/css_layer_block_rule.h"
#include "third_party/blink/renderer/core/css/css_rule_list.h"
#include "third_party/blink/renderer/core/css/css_selector.h"
#include "third_party/blink/renderer/core/css/css_starting_style_rule.h"
#include "third_party/blink/renderer/core/execution_context/execution_context.h"
#include "third_party/blink/renderer/core/frame/csp/content_security_policy.h"
#include "third_party/blink/renderer/core/inspector/inspector_base_agent.h"
#include "third_party/blink/renderer/core/inspector/inspector_dom_agent.h"
#include "third_party/blink/renderer/core/inspector/inspector_style_sheet.h"
#include "third_party/blink/renderer/core/inspector/protocol/css.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_map.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_set.h"
#include "third_party/blink/renderer/platform/heap/weak_cell.h"
#include "third_party/blink/renderer/platform/wtf/hash_counted_set.h"
#include "third_party/blink/renderer/platform/wtf/hash_map.h"
#include "third_party/blink/renderer/platform/wtf/hash_set.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {

namespace probe {
class RecalculateStyle;
}  // namespace probe

class CSSConditionRule;
class CSSContainerRule;
class CSSFunctionRule;
class CSSPropertyName;
class CSSRule;
class CSSStyleRule;
class CSSStyleSheet;
class CSSSupportsRule;
class CSSScopeRule;
class Color;
class Document;
class Element;
class FontCustomPlatformData;
class FontFace;
class InspectedFrames;
class InspectorGhostRules;
class InspectorNetworkAgent;
class InspectorResourceContainer;
class InspectorResourceContentLoader;
class MediaList;
class Node;
class LayoutObject;
class StyleRuleUsageTracker;

class CORE_EXPORT InspectorCSSAgent final
    : public InspectorBaseAgent<protocol::CSS::Metainfo>,
      public InspectorDOMAgent::DOMListener,
      public InspectorStyleSheetBase::Listener {
 public:
  enum MediaListSource {
    kMediaListSourceLinkedSheet,
    kMediaListSourceInlineSheet,
    kMediaListSourceMediaRule,
    kMediaListSourceImportRule
  };

  class InlineStyleOverrideScope {
    STACK_ALLOCATED();

   public:
    explicit InlineStyleOverrideScope(ExecutionContext* context) {
      DCHECK(context);
      content_security_policy_ = context->GetContentSecurityPolicy();
      DCHECK(content_security_policy_);
      content_security_policy_->SetOverrideAllowInlineStyle(true);
    }

    ~InlineStyleOverrideScope() {
      content_security_policy_->SetOverrideAllowInlineStyle(false);
    }

   private:
    ContentSecurityPolicy* content_security_policy_;
  };

  static CSSStyleRule* AsCSSStyleRule(CSSRule*);
  static CSSMediaRule* AsCSSMediaRule(CSSRule*);
  static CSSContainerRule* AsCSSContainerRule(CSSRule*);
  static CSSSupportsRule* AsCSSSupportsRule(CSSRule*);
  static CSSScopeRule* AsCSSScopeRule(CSSRule*);

  static void CollectAllDocumentStyleSheets(Document*,
                                            HeapVector<Member<CSSStyleSheet>>&);

  static void GetBackgroundColors(Element* element,
                                  Vector<Color>* background_colors,
                                  String* computed_font_size,
                                  String* computed_font_weight,
                                  float* text_opacity);

  // Collects all function references (i.e. <dashed-ident>s) within
  // the rule list, and the CSSFunctionRules that resulted from looking up
  // those function references.
  static void CollectReferencedFunctionRules(
      Document&,
      const HeapHashSet<Member<CSSStyleSheet>>& document_style_sheets,
      const RuleIndexList&,
      HeapHashMap<Member<const ScopedCSSName>, Member<CSSFunctionRule>>&
          result);

  InspectorCSSAgent(InspectorDOMAgent*,
                    InspectedFrames*,
                    InspectorNetworkAgent*,
                    InspectorResourceContentLoader*,
                    InspectorResourceContainer*);
  InspectorCSSAgent(const InspectorCSSAgent&) = delete;
  InspectorCSSAgent& operator=(const InspectorCSSAgent&) = delete;
  ~InspectorCSSAgent() override;
  void Trace(Visitor*) const override;

  void ForcePseudoState(Element*, CSSSelector::PseudoType, bool* result);
  void ForceStartingStyle(Element*, bool* result);
  void DidCommitLoadForLocalFrame(LocalFrame*) override;
  void Restore() override;
  void FlushPendingProtocolNotifications() override;
  void Reset();
  void MediaQueryResultChanged();

  void ActiveStyleSheetsUpdated(Document*);
  void DocumentDetached(Document*);
  void FontsUpdated(const FontFace*,
                    const String& src,
                    const FontCustomPlatformData*);
  void SetCoverageEnabled(bool);
  void WillChangeStyleElement(Element*);
  void DidMutateStyleSheet(CSSStyleSheet* css_style_sheet);
  void GetTextPosition(wtf_size_t offset,
                       const String* text,
                       TextPosition* result);
  void DidReplaceStyleSheetText(CSSStyleSheet* style_sheet, const String& text);
  void LocalFontsEnabled(bool* result);

  void enable(std::unique_ptr<EnableCallback>) override;
  protocol::Response disable() override;
  protocol::Response getAnimatedStylesForNode(
      int node_id,
      std::unique_ptr<protocol::Array<protocol::CSS::CSSAnimationStyle>>*
          animation_styles,
      std::unique_ptr<protocol::CSS::CSSStyle>* transitions_style,
      std::unique_ptr<
          protocol::Array<protocol::CSS::InheritedAnimatedStyleEntry>>*
          inherited) override;
  protocol::Response getMatchedStylesForNode(
      int node_id,
      std::unique_ptr<protocol::CSS::CSSStyle>* inline_style,
      std::unique_ptr<protocol::CSS::CSSStyle>* attributes_style,
      std::unique_ptr<protocol::Array<protocol::CSS::RuleMatch>>*
          matched_css_rules,
      std::unique_ptr<protocol::Array<protocol::CSS::PseudoElementMatches>>*,
      std::unique_ptr<protocol::Array<protocol::CSS::InheritedStyleEntry>>*,
      std::unique_ptr<
          protocol::Array<protocol::CSS::InheritedPseudoElementMatches>>*,
      std::unique_ptr<protocol::Array<protocol::CSS::CSSKeyframesRule>>*,
      std::unique_ptr<protocol::Array<protocol::CSS::CSSPositionTryRule>>*,
      std::optional<int>*,
      std::unique_ptr<protocol::Array<protocol::CSS::CSSPropertyRule>>*,
      std::unique_ptr<protocol::Array<protocol::CSS::CSSPropertyRegistration>>*,
      std::unique_ptr<protocol::CSS::CSSFontPaletteValuesRule>*,
      std::optional<int>* parent_layout_node_id,
      std::unique_ptr<protocol::Array<protocol::CSS::CSSFunctionRule>>*)
      override;
  protocol::Response getInlineStylesForNode(
      int node_id,
      std::unique_ptr<protocol::CSS::CSSStyle>* inline_style,
      std::unique_ptr<protocol::CSS::CSSStyle>* attributes_style) override;
  protocol::Response getComputedStyleForNode(
      int node_id,
      std::unique_ptr<
          protocol::Array<protocol::CSS::CSSComputedStyleProperty>>*) override;
  protocol::Response resolveValues(
      std::unique_ptr<protocol::Array<String>> values,
      int node_id,
      std::optional<String> property_name_optional,
      std::optional<protocol::DOM::PseudoType> pseudo_type,
      std::optional<String> pseudo_identifier,
      std::unique_ptr<protocol::Array<String>>* results) override;
  protocol::Response getLonghandProperties(
      const String& shorthand_name,
      const String& value,
      std::unique_ptr<protocol::Array<protocol::CSS::CSSProperty>>*
          longhand_properties) override;
  protocol::Response getPlatformFontsForNode(
      int node_id,
      std::unique_ptr<protocol::Array<protocol::CSS::PlatformFontUsage>>* fonts)
      override;
  protocol::Response collectClassNames(
      const String& style_sheet_id,
      std::unique_ptr<protocol::Array<String>>* class_names) override;
  protocol::Response getStyleSheetText(const String& style_sheet_id,
                                       String* text) override;
  protocol::Response setStyleSheetText(
      const String& style_sheet_id,
      const String& text,
      std::optional<String>* source_map_url) override;
  protocol::Response setRuleSelector(
      const String& style_sheet_id,
      std::unique_ptr<protocol::CSS::SourceRange>,
      const String& selector,
      std::unique_ptr<protocol::CSS::SelectorList>*) override;
  protocol::Response setPropertyRulePropertyName(
      const String& in_styleSheetId,
      std::unique_ptr<protocol::CSS::SourceRange> in_range,
      const String& in_propertyName,
      std::unique_ptr<protocol::CSS::Value>* out_propertyName) override;
  protocol::Response setKeyframeKey(
      const String& style_sheet_id,
      std::unique_ptr<protocol::CSS::SourceRange>,
      const String& key_text,
      std::unique_ptr<protocol::CSS::Value>* out_key_text) override;
  protocol::Response setStyleTexts(
      std::unique_ptr<protocol::Array<protocol::CSS::StyleDeclarationEdit>>
          edits,
      std::optional<int> node_for_property_syntax_validation,
      std::unique_ptr<protocol::Array<protocol::CSS::CSSStyle>>* styles)
      override;
  protocol::Response setMediaText(
      const String& style_sheet_id,
      std::unique_ptr<protocol::CSS::SourceRange>,
      const String& text,
      std::unique_ptr<protocol::CSS::CSSMedia>*) override;
  protocol::Response setContainerQueryText(
      const String& style_sheet_id,
      std::unique_ptr<protocol::CSS::SourceRange>,
      const String& text,
      std::unique_ptr<protocol::CSS::CSSContainerQuery>*) override;
  protocol::Response setScopeText(
      const String& style_sheet_id,
      std::unique_ptr<protocol::CSS::SourceRange>,
      const String& text,
      std::unique_ptr<protocol::CSS::CSSScope>*) override;
  protocol::Response setSupportsText(
      const String& style_sheet_id,
      std::unique_ptr<protocol::CSS::SourceRange>,
      const String& text,
      std::unique_ptr<protocol::CSS::CSSSupports>*) override;
  protocol::Response createStyleSheet(const String& frame_id,
                                      std::optional<bool> force,
                                      String* style_sheet_id) override;
  protocol::Response addRule(
      const String& style_sheet_id,
      const String& rule_text,
      std::unique_ptr<protocol::CSS::SourceRange>,
      std::optional<int> node_for_property_syntax_validation,
      std::unique_ptr<protocol::CSS::CSSRule>*) override;
  protocol::Response forcePseudoState(
      int node_id,
      std::unique_ptr<protocol::Array<String>> forced_pseudo_classes) override;
  protocol::Response forceStartingStyle(int node_id, bool forced) override;
  protocol::Response getMediaQueries(
      std::unique_ptr<protocol::Array<protocol::CSS::CSSMedia>>*) override;
  protocol::Response getLayersForNode(
      int node_id,
      std::unique_ptr<protocol::CSS::CSSLayerData>* root_layer) override;
  protocol::Response getLocationForSelector(
      const String& style_sheet_id,
      const String& selector_text,
      std::unique_ptr<protocol::Array<protocol::CSS::SourceRange>>* ranges)
      override;
  protocol::Response setEffectivePropertyValueForNode(
      int node_id,
      const String& property_name,
      const String& value) override;
  protocol::Response getBackgroundColors(
      int node_id,
      std::unique_ptr<protocol::Array<String>>* background_colors,
      std::optional<String>* computed_font_size,
      std::optional<String>* computed_font_weight) override;

  protocol::Response startRuleUsageTracking() override;
  protocol::Response takeCoverageDelta(
      std::unique_ptr<protocol::Array<protocol::CSS::RuleUsage>>* result,
      double* out_timestamp) override;
  protocol::Response stopRuleUsageTracking(
      std::unique_ptr<protocol::Array<protocol::CSS::RuleUsage>>* result)
      override;
  protocol::Response trackComputedStyleUpdatesForNode(
      std::optional<int> node_id) override;
  protocol::Response trackComputedStyleUpdates(
      std::unique_ptr<protocol::Array<protocol::CSS::CSSComputedStyleProperty>>
          properties_to_track) override;
  void takeComputedStyleUpdates(
      std::unique_ptr<TakeComputedStyleUpdatesCallback>) override;

  protocol::Response setLocalFontsEnabled(bool enabled) override;

  void CollectMediaQueriesFromRule(
      CSSRule*,
      protocol::Array<protocol::CSS::CSSMedia>*,
      protocol::Array<protocol::CSS::CSSRuleType>*);
  void CollectMediaQueriesFromStyleSheet(
      CSSStyleSheet*,
      protocol::Array<protocol::CSS::CSSMedia>*,
      protocol::Array<protocol::CSS::CSSRuleType>*);
  std::unique_ptr<protocol::CSS::CSSMedia> BuildMediaObject(const MediaList*,
                                                            MediaListSource,
                                                            const String&,
                                                            CSSStyleSheet*);

  CSSStyleDeclaration* FindEffectiveDeclaration(
      const CSSPropertyName&,
      const HeapVector<Member<CSSStyleDeclaration>>& styles);

  HeapVector<Member<CSSStyleDeclaration>> MatchingStyles(Element*);
  String StyleSheetId(CSSStyleSheet*);

  void DidUpdateComputedStyle(Element*,
                              const ComputedStyle*,
                              const ComputedStyle*);

  void Will(const probe::RecalculateStyle&);
  void Did(const probe::RecalculateStyle&);

 private:
  class StyleSheetAction;
  class SetStyleSheetTextAction;
  class ModifyRuleAction;
  class SetElementStyleAction;
  class AddRuleAction;

  void BuildRulesMap(InspectorStyleSheet* style_sheet,
                     HeapHashMap<Member<const StyleRule>, Member<CSSStyleRule>>*
                         rule_to_css_rule);
  static void CollectStyleSheets(CSSStyleSheet*,
                                 HeapVector<Member<CSSStyleSheet>>&);

  typedef HeapHashMap<String, Member<InspectorStyleSheet>>
      IdToInspectorStyleSheet;
  typedef HeapHashMap<String, Member<InspectorStyleSheetForInlineStyle>>
      IdToInspectorStyleSheetForInlineStyle;
  typedef HeapHashMap<Member<Node>, Member<InspectorStyleSheetForInlineStyle>>
      NodeToInspectorStyleSheet;  // bogus "stylesheets" with elements' inline
                                  // styles
  typedef HashMap<int, unsigned> NodeIdToForcedPseudoState;
  typedef HashMap<int, unsigned> NodeIdToNumberFocusedChildren;
  typedef HashSet<int> NodeIdToForcedStartingStyle;

  void ResourceContentLoaded(std::unique_ptr<EnableCallback>);
  void CompleteEnabled();
  void ResetNonPersistentData();
  InspectorStyleSheetForInlineStyle* AsInspectorStyleSheet(Element* element);

  void TriggerFontsUpdatedForDocument(Document*);

  void UpdateActiveStyleSheets(Document*);
  void SetActiveStyleSheets(Document*,
                            const HeapVector<Member<CSSStyleSheet>>&);
  protocol::Response SetStyleText(InspectorStyleSheetBase*,
                                  const SourceRange&,
                                  const String&,
                                  CSSStyleDeclaration*&);
  protocol::Response MultipleStyleTextsActions(
      std::unique_ptr<protocol::Array<protocol::CSS::StyleDeclarationEdit>>,
      HeapVector<Member<StyleSheetAction>>* actions);

  std::unique_ptr<protocol::Array<protocol::CSS::CSSPositionTryRule>>
  PositionTryRulesForElement(Element* element,
                             std::optional<size_t> active_position_try_index);

  std::pair<
      std::unique_ptr<protocol::Array<protocol::CSS::CSSPropertyRule>>,
      std::unique_ptr<protocol::Array<protocol::CSS::CSSPropertyRegistration>>>
  CustomPropertiesForNode(Element* element);
  std::unique_ptr<protocol::CSS::CSSFontPaletteValuesRule> FontPalettesForNode(
      Element& element);

  // If the |animating_element| is a pseudo element, then |element| is a
  // reference to its originating DOM element.
  std::unique_ptr<protocol::Array<protocol::CSS::CSSKeyframesRule>>
  AnimationsForNode(Element* element, Element* animating_element);
  CSSKeyframesRule* FindKeyframesRuleFromUAViewTransitionStylesheet(
      Element* element,
      StyleRuleKeyframes* keyframes_style_rule);
  CSSKeyframesRule* FindCSSOMWrapperForKeyframesRule(
      Element* element,
      StyleRuleKeyframes* keyframes_style_rule);

  void CollectPlatformFontsForLayoutObject(
      LayoutObject*,
      HashMap<std::pair<int, String>, std::pair<int, String>>*,
      unsigned descendants_depth);

  InspectorStyleSheet* BindStyleSheet(CSSStyleSheet*);
  String UnbindStyleSheet(InspectorStyleSheet*);
  InspectorStyleSheet* InspectorStyleSheetForRule(CSSStyleRule*);

  InspectorStyleSheet* CreateViaInspectorStyleSheet(Document*, bool);

  protocol::Response AssertEnabled();
  protocol::Response AssertInspectorStyleSheetForId(const String&,
                                                    InspectorStyleSheet*&);
  protocol::Response AssertStyleSheetForId(const String&,
                                           InspectorStyleSheetBase*&);
  String DetectOrigin(CSSStyleSheet* page_style_sheet,
                      Document* owner_document);

  std::unique_ptr<protocol::CSS::CSSRule> BuildObjectForRule(
      CSSStyleRule*,
      Element* element,
      PseudoId pseudo_id = kPseudoIdNone,
      const AtomicString& pseudo_argument = g_null_atom);
  std::unique_ptr<protocol::CSS::RuleUsage> BuildCoverageInfo(CSSStyleRule*,
                                                              bool);
  std::unique_ptr<protocol::Array<protocol::CSS::RuleMatch>>
  BuildArrayForMatchedRuleList(
      RuleIndexList*,
      Element*,
      const InspectorGhostRules&,
      PseudoId pseudo_id = kPseudoIdNone,
      const AtomicString& pseudo_argument = g_null_atom);
  std::unique_ptr<protocol::Array<protocol::CSS::CSSAnimationStyle>>
  BuildArrayForCSSAnimationStyleList(Element* element);
  std::unique_ptr<protocol::CSS::CSSStyle> BuildObjectForAttributesStyle(
      Element*);
  std::unique_ptr<protocol::Array<int>>
  BuildArrayForComputedStyleUpdatedNodes();
  std::unique_ptr<protocol::CSS::CSSStyle> BuildObjectForTransitionsStyle(
      Element*);

  // Container Queries implementation
  std::unique_ptr<protocol::CSS::CSSContainerQuery> BuildContainerQueryObject(
      CSSContainerRule*);
  void CollectContainerQueriesFromRule(
      CSSRule*,
      protocol::Array<protocol::CSS::CSSContainerQuery>*,
      protocol::Array<protocol::CSS::CSSRuleType>*);

  // Supports at-rule implementation
  std::unique_ptr<protocol::CSS::CSSSupports> BuildSupportsObject(
      CSSSupportsRule*);
  void CollectSupportsFromRule(CSSRule*,
                               protocol::Array<protocol::CSS::CSSSupports>*,
                               protocol::Array<protocol::CSS::CSSRuleType>*);

  std::unique_ptr<protocol::CSS::CSSLayerData> BuildLayerDataObject(
      const CascadeLayer* layer,
      unsigned& max_order);

  // Layers at-rule implementation
  std::unique_ptr<protocol::CSS::CSSLayer> BuildLayerObject(
      CSSLayerBlockRule* rule);
  std::unique_ptr<protocol::CSS::CSSLayer> BuildLayerObjectFromImport(
      CSSImportRule* rule);
  void CollectLayersFromRule(CSSRule*,
                             protocol::Array<protocol::CSS::CSSLayer>*,
                             protocol::Array<protocol::CSS::CSSRuleType>*);

  // Starting Style at-rule implementation
  std::unique_ptr<protocol::CSS::CSSStartingStyle> BuildStartingStyleObject(
      CSSStartingStyleRule* rule);
  void CollectStartingStylesFromRule(
      CSSRule*,
      protocol::Array<protocol::CSS::CSSStartingStyle>*,
      protocol::Array<protocol::CSS::CSSRuleType>*);

  void FillAncestorData(CSSRule* rule, protocol::CSS::CSSRule* result);

  // Scope at-rule implementation
  std::unique_ptr<protocol::CSS::CSSScope> BuildScopeObject(CSSScopeRule*);
  void CollectScopesFromRule(CSSRule*,
                             protocol::Array<protocol::CSS::CSSScope>*,
                             protocol::Array<protocol::CSS::CSSRuleType>*);

  // Function at-rule implementation
  std::unique_ptr<protocol::CSS::CSSFunctionRule> BuildObjectForFunctionRule(
      CSSFunctionRule*);
  std::unique_ptr<protocol::CSS::CSSFunctionConditionNode>
  BuildObjectForFunctionConditionNode(CSSConditionRule*);
  std::unique_ptr<protocol::Array<protocol::CSS::CSSFunctionNode>>
  BuildArrayForFunctionNodeChildren(CSSRuleList*);

  // InspectorDOMAgent::DOMListener implementation
  void DidAddDocument(Document*) override;
  void WillRemoveDOMNode(Node*) override;
  void DidModifyDOMAttr(Element*) override;

  // InspectorStyleSheet::Listener implementation
  void StyleSheetChanged(InspectorStyleSheetBase*) override;

  void ResetPseudoStates();
  void ResetStartingStyles();

  void IncrementFocusedCountForAncestors(Element*);
  void DecrementFocusedCountForAncestors(Element*);

  void NotifyComputedStyleUpdatedForNode(int node_id);
  static String ResolvePercentagesValues(Element*,
                                         CSSPropertyName,
                                         const CSSValue* parsed_value,
                                         const String& original_value);

  Member<InspectorDOMAgent> dom_agent_;
  Member<InspectedFrames> inspected_frames_;
  Member<InspectorNetworkAgent> network_agent_;
  Member<InspectorResourceContentLoader> resource_content_loader_;
  Member<InspectorResourceContainer> resource_container_;

  IdToInspectorStyleSheet id_to_inspector_style_sheet_;
  IdToInspectorStyleSheetForInlineStyle
      id_to_inspector_style_sheet_for_inline_style_;
  HeapHashMap<Member<CSSStyleSheet>, Member<InspectorStyleSheet>>
      css_style_sheet_to_inspector_style_sheet_;
  using DocumentStyleSheets =
      HeapHashMap<Member<Document>,
                  Member<GCedHeapHashSet<Member<CSSStyleSheet>>>>;
  DocumentStyleSheets document_to_css_style_sheets_;
  HeapHashSet<Member<Document>> invalidated_documents_;

  NodeToInspectorStyleSheet node_to_inspector_style_sheet_;
  NodeIdToForcedPseudoState node_id_to_forced_pseudo_state_;
  NodeIdToNumberFocusedChildren node_id_to_number_focused_children_;
  NodeIdToForcedStartingStyle node_id_to_forced_starting_style_;

  HeapHashMap<WeakMember<Document>, Member<CSSStyleSheet>>
      default_inspector_stylesheets_;

  Member<StyleRuleUsageTracker> tracker_;

  Member<CSSStyleSheet> inspector_user_agent_style_sheet_;
  // This is cached to track when the ViewTransition UA stylesheet changes
  // and a new binding is required to an InspectorStyleSheet.
  Member<CSSStyleSheet> user_agent_view_transition_style_sheet_;

  int resource_content_loader_client_id_;
  InspectorAgentState::Boolean enable_requested_;
  bool enable_completed_;
  InspectorAgentState::Boolean coverage_enabled_;
  InspectorAgentState::Boolean local_fonts_enabled_;

  // Maps style property names to the set of tracked values for that property.
  // Notifications are sent when the property changes to or from one of the
  // tracked values.
  HashMap<String, HashSet<String>> tracked_computed_styles_;
  std::unique_ptr<TakeComputedStyleUpdatesCallback>
      computed_style_updated_callback_;
  HashSet<int> computed_style_updated_node_ids_;

  // Keeps track of the node ids that has an active
  // computedStyleUpdatedForNode task
  HashSet<int> notify_computed_style_updated_node_ids_;
  WeakCellFactory<InspectorCSSAgent> weak_factory_{this};

  // Node to be tracked for `ComputedStyleUpdated` events.
  // This is set via `trackComputedStyleUpdatesForNode` call.
  std::optional<int> node_id_for_computed_style_updated_events_;

  friend class InspectorResourceContentLoaderCallback;
  friend class StyleSheetBinder;
  friend class InspectorCSSAgentTest;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_INSPECTOR_INSPECTOR_CSS_AGENT_H_
