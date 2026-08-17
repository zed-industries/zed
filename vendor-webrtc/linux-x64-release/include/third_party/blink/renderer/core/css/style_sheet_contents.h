/*
 * (C) 1999-2003 Lars Knoll (knoll@kde.org)
 * Copyright (C) 2004, 2006, 2007, 2008, 2009, 2010, 2012 Apple Inc. All rights
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_STYLE_SHEET_CONTENTS_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_STYLE_SHEET_CONTENTS_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/css/parser/css_parser_context.h"
#include "third_party/blink/renderer/core/css/parser/css_tokenizer.h"
#include "third_party/blink/renderer/core/css/rule_set.h"
#include "third_party/blink/renderer/core/css/rule_set_diff.h"
#include "third_party/blink/renderer/core/loader/resource/css_style_sheet_resource.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_set.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/loader/fetch/render_blocking_behavior.h"
#include "third_party/blink/renderer/platform/weborigin/kurl.h"
#include "third_party/blink/renderer/platform/wtf/hash_map.h"
#include "third_party/blink/renderer/platform/wtf/text/atomic_string.h"
#include "third_party/blink/renderer/platform/wtf/text/atomic_string_hash.h"
#include "third_party/blink/renderer/platform/wtf/text/string_hash.h"
#include "third_party/blink/renderer/platform/wtf/text/text_position.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {

class CSSStyleSheet;
class Document;
class Node;
class StyleRuleBase;
class StyleRuleFontFace;
class RuleSetDiff;
class StyleRuleImport;
class StyleRuleNamespace;
enum class ParseSheetResult;

class CORE_EXPORT StyleSheetContents final
    : public GarbageCollected<StyleSheetContents> {
 public:
  static const Document* SingleOwnerDocument(const StyleSheetContents*);

  StyleSheetContents(const CSSParserContext* context,
                     const String& original_url = String(),
                     StyleRuleImport* owner_rule = nullptr);
  StyleSheetContents(const StyleSheetContents&);
  StyleSheetContents() = delete;
  ~StyleSheetContents();

  // TODO(xiaochengh): |parser_context_| should never be null. Make it return a
  // const reference here to avoid confusion.
  const CSSParserContext* ParserContext() const {
    return parser_context_.Get();
  }

  const AtomicString& DefaultNamespace() const { return default_namespace_; }
  const AtomicString& NamespaceURIFromPrefix(const AtomicString& prefix) const;

  void ParseAuthorStyleSheet(const CSSStyleSheetResource*);
  ParseSheetResult ParseString(const String&,
                               bool allow_import_rules = true,
                               CSSDeferPropertyParsing defer_property_parsing =
                                   CSSDeferPropertyParsing::kNo);

  bool IsCacheableForResource() const;
  bool IsCacheableForStyleElement() const;

  bool IsLoading() const;

  void CheckLoaded();

  // Called if this sheet has finished loading and then a dynamically added
  // @import rule starts loading a child stylesheet.
  void SetToPendingState();

  StyleSheetContents* RootStyleSheet() const;
  bool HasSingleOwnerNode() const;
  Node* SingleOwnerNode() const;
  Document* SingleOwnerDocument() const;
  bool HasSingleOwnerDocument() const { return has_single_owner_document_; }

  // Gets a client in the given TreeScope.
  CSSStyleSheet* ClientInTreeScope(const TreeScope& tree_scope) const;

  // Gets the first owner document in the list of registered clients, or nullptr
  // if there are none.
  Document* AnyOwnerDocument() const;

  const WTF::TextEncoding& Charset() const {
    return parser_context_->Charset();
  }

  bool LoadCompleted() const;
  bool HasFailedOrCanceledSubresources() const;

  void SetHasSyntacticallyValidCSSHeader(bool is_valid_css);
  bool HasSyntacticallyValidCSSHeader() const {
    return has_syntactically_valid_css_header_;
  }

  void SetHasFontFaceRule() { has_font_face_rule_ = true; }
  bool HasFontFaceRule() const { return has_font_face_rule_; }

  void ParserAddNamespace(const AtomicString& prefix, const AtomicString& uri);
  void ParserAppendRule(StyleRuleBase*);

  void ClearRules();

  // If the given rule exists, replace it with the new one. This is used when
  // CSSOM wants to modify the rule but cannot do so without reallocating
  // (see setCssSelectorText()).
  //
  // The position_hint variable is a pure hint as of where the old rule can
  // be found; if it is wrong or out-of-range (for instance because the rule
  // has been deleted, or some have been moved around), the function is still
  // safe to call, but will do a linear search for the rule. The return value
  // is an updated position hint suitable for the next ReplaceRuleIfExists()
  // call on the same (new) rule. The position_hint is not capable of describing
  // rules nested within other rules; the result will still be correct, but the
  // search will be slow for such rules.
  wtf_size_t ReplaceRuleIfExists(StyleRuleBase* old_rule,
                                 StyleRuleBase* new_rule,
                                 wtf_size_t position_hint);

  // Notify the style sheet that a rule has changed externally, for diff
  // purposes (see RuleSetDiff). In particular, if a rule changes selector
  // text or properties, we need to know about it here, since there's no
  // other way StyleSheetContents gets to know about such changes.
  // WrapperInsertRule() and other explicit changes to StyleSheetContents
  // already mark changes themselves.
  void NotifyRuleChanged(StyleRuleBase* rule) {
    if (rule_set_diff_) {
      rule_set_diff_->AddDiff(rule);
    }
  }
  void NotifyDiffUnrepresentable() {
    if (rule_set_diff_) {
      rule_set_diff_->MarkUnrepresentable();
    }
  }

  // Get/clear the diff between last time we did StartMutation()
  // (with an existing rule set) and now. See RuleSetDiff for more information.
  RuleSetDiff* GetRuleSetDiff() const { return rule_set_diff_.Get(); }
  void ClearRuleSetDiff() { rule_set_diff_.Clear(); }

  // Rules other than @import.
  const HeapVector<Member<StyleRuleBase>>& ChildRules() const {
    return child_rules_;
  }
  const HeapVector<Member<StyleRuleLayerStatement>>&
  PreImportLayerStatementRules() const {
    return pre_import_layer_statement_rules_;
  }
  const HeapVector<Member<StyleRuleImport>>& ImportRules() const {
    return import_rules_;
  }
  const HeapVector<Member<StyleRuleNamespace>>& NamespaceRules() const {
    return namespace_rules_;
  }

  void NotifyLoadedSheet(const CSSStyleSheetResource*);

  StyleSheetContents* ParentStyleSheet() const;
  StyleRuleImport* OwnerRule() const { return owner_rule_.Get(); }
  void ClearOwnerRule() { owner_rule_ = nullptr; }

  // The URL that started the redirect chain that led to this
  // style sheet. This property probably isn't useful for much except the
  // JavaScript binding (which needs to use this value for security).
  String OriginalURL() const { return original_url_; }
  // The response URL after redirects and service worker interception.
  const KURL& BaseURL() const { return parser_context_->BaseURL(); }

  // If true, allows reading and modifying of the CSS rules.
  // https://drafts.csswg.org/cssom/#concept-css-style-sheet-origin-clean-flag
  bool IsOriginClean() const { return parser_context_->IsOriginClean(); }

  unsigned RuleCount() const;
  StyleRuleBase* RuleAt(unsigned index) const;

  unsigned EstimatedSizeInBytes() const;

  bool WrapperInsertRule(StyleRuleBase*, unsigned index);
  bool WrapperDeleteRule(unsigned index);

  StyleSheetContents* Copy() const {
    return MakeGarbageCollected<StyleSheetContents>(*this);
  }

  void RegisterClient(CSSStyleSheet*);
  void UnregisterClient(CSSStyleSheet*);
  size_t ClientSize() const {
    return loading_clients_.size() + completed_clients_.size();
  }
  bool HasOneClient() { return ClientSize() == 1; }
  void ClientLoadCompleted(CSSStyleSheet*);
  void ClientLoadStarted(CSSStyleSheet*);

  bool IsMutable() const { return is_mutable_; }
  void StartMutation();

  bool IsUsedFromTextCache() const { return is_used_from_text_cache_; }
  void SetIsUsedFromTextCache() { is_used_from_text_cache_ = true; }

  bool IsReferencedFromResource() const {
    return referenced_from_resource_ != nullptr;
  }
  void SetReferencedFromResource(CSSStyleSheetResource*);
  void ClearReferencedFromResource();

  void SetHasMediaQueries();
  bool HasMediaQueries() const { return has_media_queries_; }

  bool DidLoadErrorOccur() const { return did_load_error_occur_; }

  RuleSet& GetRuleSet() {
    DCHECK(rule_set_);
    return *rule_set_.Get();
  }

  bool HasRuleSet() { return rule_set_.Get(); }
  RuleSet& EnsureRuleSet(const MediaQueryEvaluator&);
  void ClearRuleSet();
  // Create a RuleSet which is not associated (i.e. not owned)
  // by this StyleSheetContents. This is useful for matching rules
  // in  an "alternate reality", which is the case for InspectorGhostRules.
  RuleSet* CreateUnconnectedRuleSet(const MediaQueryEvaluator&) const;

  String SourceMapURL() const { return source_map_url_; }

  void SetRenderBlocking(RenderBlockingBehavior behavior) {
    render_blocking_behavior_ = behavior;
  }
  RenderBlockingBehavior GetRenderBlockingBehavior() const {
    return render_blocking_behavior_;
  }

  void Trace(Visitor*) const;

 private:
  StyleSheetContents& operator=(const StyleSheetContents&) = delete;
  void NotifyRemoveFontFaceRule(const StyleRuleFontFace*);

  Document* ClientSingleOwnerDocument() const;
  Document* ClientAnyOwnerDocument() const;

  Member<StyleRuleImport> owner_rule_;

  String original_url_;

  HeapVector<Member<StyleRuleLayerStatement>> pre_import_layer_statement_rules_;
  HeapVector<Member<StyleRuleImport>> import_rules_;
  HeapVector<Member<StyleRuleNamespace>> namespace_rules_;
  HeapVector<Member<StyleRuleBase>> child_rules_;
  using PrefixNamespaceURIMap = HashMap<AtomicString, AtomicString>;
  PrefixNamespaceURIMap namespaces_;
  AtomicString default_namespace_;
  WeakMember<CSSStyleSheetResource> referenced_from_resource_;

  bool has_syntactically_valid_css_header_ : 1;
  bool did_load_error_occur_ : 1;
  bool is_mutable_ : 1;
  bool has_font_face_rule_ : 1;
  bool has_media_queries_ : 1;
  bool has_single_owner_document_ : 1;
  bool is_used_from_text_cache_ : 1;

  Member<const CSSParserContext> parser_context_;

  HeapHashSet<WeakMember<CSSStyleSheet>> loading_clients_;
  HeapHashSet<WeakMember<CSSStyleSheet>> completed_clients_;

  Member<RuleSet> rule_set_;
  // If we have modified the style sheet since last creating
  // a rule set, this will be nonempty and contain the relevant
  // diffs (see RuleSetDiff). Constructed by StartMutation().
  Member<RuleSetDiff> rule_set_diff_;

  String source_map_url_;
  RenderBlockingBehavior render_blocking_behavior_ =
      RenderBlockingBehavior::kUnset;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_STYLE_SHEET_CONTENTS_H_
