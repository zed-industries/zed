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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_INSPECTOR_INSPECTOR_STYLE_SHEET_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_INSPECTOR_INSPECTOR_STYLE_SHEET_H_

#include <memory>

#include "base/memory/scoped_refptr.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/css/css_font_palette_values_rule.h"
#include "third_party/blink/renderer/core/css/css_property_source_data.h"
#include "third_party/blink/renderer/core/css/css_scope_rule.h"
#include "third_party/blink/renderer/core/css/css_style_declaration.h"
#include "third_party/blink/renderer/core/inspector/inspector_css_parser_observer.h"
#include "third_party/blink/renderer/core/inspector/inspector_diff.h"
#include "third_party/blink/renderer/core/inspector/protocol/css.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/wtf/hash_map.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {

class CSSKeyframeRule;
class CSSMediaRule;
class CSSContainerRule;
class CSSPositionTryRule;
class CSSPropertyRule;
class CSSStyleDeclaration;
class CSSStyleRule;
class CSSStyleSheet;
class CSSSupportsRule;
class Document;
class Element;
class ExceptionState;
class InspectorNetworkAgent;
class InspectorResourceContainer;
class InspectorStyleSheetBase;

typedef HeapVector<Member<CSSRule>> CSSRuleVector;

class InspectorStyle final : public GarbageCollected<InspectorStyle> {
 public:
  InspectorStyle(CSSStyleDeclaration*,
                 CSSRuleSourceData*,
                 InspectorStyleSheetBase* parent_style_sheet);

  CSSStyleDeclaration* CssStyle() { return style_.Get(); }
  InspectorStyleSheetBase* InspectorStyleSheet() {
    return parent_style_sheet_.Get();
  }
  std::unique_ptr<protocol::CSS::CSSStyle> BuildObjectForStyle(
      Element* element = nullptr,
      PseudoId pseudo_id = kPseudoIdNone,
      const AtomicString& pseudo_argument = g_null_atom);
  bool StyleText(String* result);
  bool TextForRange(const SourceRange&, String* result);

  void Trace(Visitor*) const;

 private:
  void PopulateAllProperties(Vector<CSSPropertySourceData>& result);
  bool CheckRegisteredPropertySyntaxWithVarSubstitution(
      Element* element,
      const CSSPropertySourceData& property,
      PseudoId pseudo_id = kPseudoIdNone,
      const AtomicString& pseudo_argument = g_null_atom) const;
  std::unique_ptr<protocol::CSS::CSSStyle> StyleWithProperties(
      Element* element,
      PseudoId pseudo_id = kPseudoIdNone,
      const AtomicString& pseudo_argument = g_null_atom);
  String ShorthandValue(const String& shorthand_property);
  std::unique_ptr<protocol::Array<protocol::CSS::CSSProperty>>
  LonghandProperties(const CSSPropertySourceData& property_entry);

  Member<CSSStyleDeclaration> style_;
  Member<CSSRuleSourceData> source_data_;
  Member<InspectorStyleSheetBase> parent_style_sheet_;
};

class InspectorStyleSheetBase
    : public GarbageCollected<InspectorStyleSheetBase> {
 public:
  class CORE_EXPORT Listener {
   public:
    Listener() = default;
    virtual ~Listener() = default;
    virtual void StyleSheetChanged(InspectorStyleSheetBase*) = 0;
  };
  virtual ~InspectorStyleSheetBase() = default;
  virtual void Trace(Visitor* visitor) const {}

  String Id() { return id_; }

  virtual bool SetText(const String&, ExceptionState&) = 0;
  virtual bool GetText(String* result) = 0;
  virtual String SourceMapURL() { return String(); }
  virtual const Document* GetDocument() = 0;

  std::unique_ptr<protocol::CSS::CSSStyle> BuildObjectForStyle(
      CSSStyleDeclaration*,
      Element* element,
      PseudoId pseudo_id = kPseudoIdNone,
      const AtomicString& pseudo_argument = g_null_atom);
  std::unique_ptr<protocol::CSS::SourceRange> BuildSourceRangeObject(
      const SourceRange&);
  bool LineNumberAndColumnToOffset(unsigned line_number,
                                   unsigned column_number,
                                   unsigned* offset);
  virtual bool IsInlineStyle() = 0;

 protected:
  explicit InspectorStyleSheetBase(Listener*, String id);

  Listener* GetListener() { return listener_; }
  void OnStyleSheetTextChanged();
  const LineEndings* GetLineEndings();
  void ResetLineEndings();

  virtual InspectorStyle* GetInspectorStyle(CSSStyleDeclaration*) = 0;

 private:
  friend class InspectorStyle;

  String id_;
  Listener* listener_;
  std::unique_ptr<LineEndings> line_endings_;
};

class InspectorStyleSheet : public InspectorStyleSheetBase {
 public:
  InspectorStyleSheet(InspectorNetworkAgent*,
                      CSSStyleSheet* page_style_sheet,
                      const String& origin,
                      const String& document_url,
                      InspectorStyleSheetBase::Listener*,
                      InspectorResourceContainer*);
  ~InspectorStyleSheet() override;
  void Trace(Visitor*) const override;

  String FinalURL();
  bool SetText(const String&, ExceptionState&) override;
  void CSSOMStyleSheetTextReplaced(const String&);
  bool GetText(String* result) override;
  void MarkForSync() { marked_for_sync_ = true; }
  void SyncTextIfNeeded();

  CSSStyleRule* SetRuleSelector(const SourceRange&,
                                const String& selector,
                                SourceRange* new_range,
                                String* old_selector,
                                ExceptionState&);
  CSSKeyframeRule* SetKeyframeKey(const SourceRange&,
                                  const String& text,
                                  SourceRange* new_range,
                                  String* old_text,
                                  ExceptionState&);
  CSSPropertyRule* SetPropertyName(const SourceRange&,
                                   const String& text,
                                   SourceRange* new_range,
                                   String* old_text,
                                   ExceptionState&);
  CSSRule* SetStyleText(const SourceRange&,
                        const String& text,
                        SourceRange* new_range,
                        String* old_selector,
                        ExceptionState&);
  CSSMediaRule* SetMediaRuleText(const SourceRange&,
                                 const String& selector,
                                 SourceRange* new_range,
                                 String* old_selector,
                                 ExceptionState&);
  CSSContainerRule* SetContainerRuleText(const SourceRange&,
                                         const String& selector,
                                         SourceRange* new_range,
                                         String* old_selector,
                                         ExceptionState&);
  CSSScopeRule* SetScopeRuleText(const SourceRange&,
                                 const String& selector,
                                 SourceRange* new_range,
                                 String* old_selector,
                                 ExceptionState&);
  CSSSupportsRule* SetSupportsRuleText(const SourceRange&,
                                       const String& selector,
                                       SourceRange* new_range,
                                       String* old_selector,
                                       ExceptionState&);
  CSSStyleRule* AddRule(const String& rule_text,
                        const SourceRange& location,
                        SourceRange* added_range,
                        ExceptionState&);
  bool DeleteRule(const SourceRange&, ExceptionState&);
  std::unique_ptr<protocol::Array<String>> CollectClassNames();
  CSSStyleSheet* PageStyleSheet() { return page_style_sheet_.Get(); }
  String Origin() { return origin_; }
  bool CanBindOrigin();

  std::unique_ptr<protocol::CSS::CSSStyleSheetHeader>
  BuildObjectForStyleSheetInfo();
  std::unique_ptr<protocol::CSS::CSSRule> BuildObjectForRuleWithoutAncestorData(
      CSSStyleRule*,
      Element* element,
      PseudoId pseudo_id = kPseudoIdNone,
      const AtomicString& pseudo_argument = g_null_atom);
  std::unique_ptr<protocol::CSS::RuleUsage> BuildObjectForRuleUsage(CSSRule*,
                                                                    bool);
  std::unique_ptr<protocol::CSS::CSSPositionTryRule>
  BuildObjectForPositionTryRule(CSSPositionTryRule*, bool active);
  std::unique_ptr<protocol::CSS::CSSFontPaletteValuesRule>
  BuildObjectForFontPaletteValuesRule(CSSFontPaletteValuesRule*);
  std::unique_ptr<protocol::CSS::CSSPropertyRule> BuildObjectForPropertyRule(
      CSSPropertyRule*);
  std::unique_ptr<protocol::CSS::CSSKeyframeRule> BuildObjectForKeyframeRule(
      CSSKeyframeRule*,
      Element*);
  std::unique_ptr<protocol::CSS::SelectorList> BuildObjectForSelectorList(
      CSSStyleRule*);
  std::unique_ptr<protocol::CSS::SourceRange> RuleHeaderSourceRange(CSSRule*);
  std::unique_ptr<protocol::CSS::SourceRange> MediaQueryExpValueSourceRange(
      CSSRule*,
      wtf_size_t media_query_index,
      wtf_size_t media_query_exp_index);
  bool IsInlineStyle() override { return false; }
  const CSSRuleVector& FlatRules();
  CSSRuleSourceData* SourceDataForRule(CSSRule*);
  String SourceMapURL() override;
  const Document* GetDocument() override;

 protected:
  InspectorStyle* GetInspectorStyle(CSSStyleDeclaration*) override;

 private:
  CSSRuleSourceData* RuleSourceDataAfterSourceRange(const SourceRange&);
  CSSRuleSourceData* FindRuleByHeaderRange(const SourceRange&);
  CSSRuleSourceData* FindRuleByDeclarationsRange(const SourceRange&);
  CSSRule* RuleForSourceData(CSSRuleSourceData*);
  CSSStyleRule* InsertCSSOMRuleInStyleSheet(CSSRule* insert_before,
                                            const String& rule_text,
                                            ExceptionState&);
  CSSStyleRule* InsertCSSOMRuleInMediaRule(CSSMediaRule*,
                                           CSSRule* insert_before,
                                           const String& rule_text,
                                           ExceptionState&);
  CSSStyleRule* InsertCSSOMRuleBySourceRange(const SourceRange&,
                                             const String& rule_text,
                                             ExceptionState&);
  String SourceURL();
  void RemapSourceDataToCSSOMIfNecessary();
  void MapSourceDataToCSSOM();
  bool ResourceStyleSheetText(String* result, bool* loadingFailed);
  bool InlineStyleSheetText(String* result);
  bool InspectorStyleSheetText(String* result);
  String CollectStyleSheetRules();
  bool CSSOMStyleSheetText(String* result);
  std::unique_ptr<protocol::Array<protocol::CSS::Value>>
  SelectorsFromSource(CSSRuleSourceData*, const String&, CSSStyleRule*);
  Vector<const CSSSelector*> SelectorsFromRule(CSSStyleRule* rule);
  String Url();
  bool HasSourceURL();
  bool StartsAtZero();

  bool IsMutable() const;
  void Reset();
  void UpdateText();

  void ReplaceText(const SourceRange&,
                   const String& text,
                   SourceRange* new_range,
                   String* old_text);
  void InnerSetText(const String& new_text, bool mark_as_locally_modified);
  void ParseText(const String& text);
  String MergeCSSOMRulesWithText(const String& text);
  Element* OwnerStyleElement();

  Member<InspectorResourceContainer> resource_container_;
  Member<InspectorNetworkAgent> network_agent_;
  Member<CSSStyleSheet> page_style_sheet_;
  String origin_;
  String document_url_;
  Member<GCedCSSRuleSourceDataList> source_data_;
  String text_;
  CSSRuleVector cssom_flat_rules_;
  CSSRuleVector parsed_flat_rules_;
  InspectorIndexMap rule_to_source_data_;
  InspectorIndexMap source_data_to_rule_;
  String source_url_;
  std::optional<bool> request_failed_to_load_;
  // True means that CSSOM rules are to be synced with the original source text.
  bool marked_for_sync_;
};

class InspectorStyleSheetForInlineStyle final : public InspectorStyleSheetBase {
 public:
  InspectorStyleSheetForInlineStyle(Element*, Listener*);
  void DidModifyElementAttribute();
  bool SetText(const String&, ExceptionState&) override;
  bool GetText(String* result) override;
  CSSStyleDeclaration* InlineStyle();
  CSSRuleSourceData* RuleSourceData();

  void Trace(Visitor*) const override;

  const Document* GetDocument() override;

 protected:
  InspectorStyle* GetInspectorStyle(CSSStyleDeclaration*) override;

  // Also accessed by friend class InspectorStyle.
  bool IsInlineStyle() override { return true; }

 private:
  const String& ElementStyleText();

  Member<Element> element_;
  Member<InspectorStyle> inspector_style_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_INSPECTOR_INSPECTOR_STYLE_SHEET_H_
