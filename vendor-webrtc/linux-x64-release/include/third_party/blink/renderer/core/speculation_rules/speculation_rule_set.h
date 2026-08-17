// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_SPECULATION_RULES_SPECULATION_RULE_SET_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_SPECULATION_RULES_SPECULATION_RULE_SET_H_

#include "base/containers/span.h"
#include "base/types/pass_key.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/speculation_rules/speculation_rule.h"
#include "third_party/blink/renderer/platform/graphics/dom_node_id.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_vector.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"

namespace blink {

class Document;
class ExecutionContext;
class KURL;
class ScriptElementBase;
class SpeculationRule;
class SpeculationRulesResource;
class StyleRule;

using SpeculationRuleSetId = String;

// Summary of an error got in parse.
enum class SpeculationRuleSetErrorType {
  kNoError,
  // Source is not a valid JSON object and entire parse failed.
  kSourceIsNotJsonObject,
  // An invalid or unsupported rule was ignored.
  kInvalidRulesSkipped,
  kMaxValue = kInvalidRulesSkipped,
};

enum class BrowserInjectedSpeculationRuleOptOut { kRespect, kIgnore };

// A set of rules generated from a single <script type=speculationrules>, which
// provides rules to identify URLs and corresponding conditions for speculation,
// grouped by the action that is suggested.
//
// https://wicg.github.io/nav-speculation/speculation-rules.html#speculation-rule-set
class CORE_EXPORT SpeculationRuleSet final
    : public GarbageCollected<SpeculationRuleSet> {
 public:
  // Stores the original source text and base URL (if the base URL used isn't
  // the document's base URL) used for parsing a rule set.
  class CORE_EXPORT Source : public GarbageCollected<Source> {
   public:
    // Don't call this directly; use the factory methods below instead!
    Source(base::PassKey<Source>,
           const String& source_text,
           Document*,
           std::optional<DOMNodeId> node_id,
           std::optional<KURL> base_url,
           std::optional<uint64_t> request_id,
           bool ignore_opt_out);

    static Source* FromInlineScript(const String& source_text,
                                    Document&,
                                    DOMNodeId node_id);
    static Source* FromRequest(const String& source_text,
                               const KURL& base_url,
                               uint64_t request_id);
    static Source* FromBrowserInjected(
        const String& source_text,
        const KURL& base_url,
        BrowserInjectedSpeculationRuleOptOut opt_out);

    const String& GetSourceText() const;

    // Has a value iff IsFromInlineScript() is true.
    const std::optional<DOMNodeId>& GetNodeId() const;

    // Have values iff IsFromRequest() is true.
    const std::optional<KURL> GetSourceURL() const;
    const std::optional<uint64_t>& GetRequestId() const;

    KURL GetBaseURL() const;

    bool IsFromInlineScript() const;
    bool IsFromRequest() const;
    bool IsFromBrowserInjected() const;
    bool IsFromBrowserInjectedAndRespectsOptOut() const;

    void Trace(Visitor*) const;

   private:
    // Set for all types
    String source_text_;

    // Set by FromInlineScript()
    Member<Document> document_;
    std::optional<DOMNodeId> node_id_;

    // Set by FromRequest() and FromBrowserInjected()
    std::optional<KURL> base_url_;

    // Set by FromRequest()
    std::optional<uint64_t> request_id_;

    // Set by FromBrowserInjected();
    bool ignore_opt_out_ = false;
  };

  SpeculationRuleSet(base::PassKey<SpeculationRuleSet>, Source* source);

  // Returns parsed rule sets (never nullptr). If an error occurred in
  // parsing entire JSON object, returns an empty rule set. If an error
  // occurred in parsing a rule set for a key or a rule, skips that one.
  static SpeculationRuleSet* Parse(Source* source, ExecutionContext* context);

  SpeculationRuleSetId InspectorId() const { return inspector_id_; }

  const HeapVector<Member<SpeculationRule>>& prefetch_rules() const {
    return prefetch_rules_;
  }
  const HeapVector<Member<SpeculationRule>>& prefetch_with_subresources_rules()
      const {
    return prefetch_with_subresources_rules_;
  }
  const HeapVector<Member<SpeculationRule>>& prerender_rules() const {
    return prerender_rules_;
  }

  bool has_document_rule() const { return has_document_rule_; }
  bool requires_unfiltered_input() const { return requires_unfiltered_input_; }

  Source* source() const { return source_.Get(); }

  const HeapVector<Member<StyleRule>>& selectors() { return selectors_; }

  // Returns an summary and detail of an error got in `Parse`.
  // `error_message` is empty iff `error_type` is `kNoError`.
  // An error indicates that one or more rules were skipped.
  SpeculationRuleSetErrorType error_type() const { return error_type_; }
  const String& error_message() const { return error_message_; }
  // Returns a list of detailed warnings from the `Parse` method. Warnings
  // indicate that there are issues with one or more rules but these rules were
  // still accepted in contrast with rules with an error that would be skipped.
  const Vector<String>& warning_messages() const { return warning_messages_; }
  // Shorthand to check `error_type` is not `kNoError`.
  bool HasError() const;
  // Shorthand to check if there are any warning messages.
  bool HasWarnings() const;
  bool ShouldReportUMAForError() const;

  void AddConsoleMessageForValidation(ScriptElementBase& script_element);
  void AddConsoleMessageForValidation(Document& element_document,
                                      SpeculationRulesResource& resource);

  static mojom::blink::SpeculationTargetHint SpeculationTargetHintFromString(
      const StringView& target_hint_str);

  void Trace(Visitor*) const;

 private:
  void SetError(SpeculationRuleSetErrorType error_type, String error_message);
  void AddWarnings(base::span<const String> warning_messages);

  SpeculationRuleSetId inspector_id_;
  HeapVector<Member<SpeculationRule>> prefetch_rules_;
  HeapVector<Member<SpeculationRule>> prefetch_with_subresources_rules_;
  HeapVector<Member<SpeculationRule>> prerender_rules_;
  // The original source is reused to reparse speculation rule sets when the
  // document base URL changes.
  Member<Source> source_;
  HeapVector<Member<StyleRule>> selectors_;
  bool has_document_rule_ = false;

  // If true, this ruleset contains a rule which may not work correctly if input
  // is filtered.
  // TODO(crbug.com/1425870): Remove this once such rules work without this
  // hack.
  bool requires_unfiltered_input_ = false;

  SpeculationRuleSetErrorType error_type_ =
      SpeculationRuleSetErrorType::kNoError;
  String error_message_;
  Vector<String> warning_messages_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_SPECULATION_RULES_SPECULATION_RULE_SET_H_
