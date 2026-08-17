// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_INSPECTOR_INSPECTOR_CSS_PARSER_OBSERVER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_INSPECTOR_INSPECTOR_CSS_PARSER_OBSERVER_H_

#include <memory>
#include <optional>

#include "base/containers/span.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/css/css_property_source_data.h"
#include "third_party/blink/renderer/core/css/parser/css_at_rule_id.h"
#include "third_party/blink/renderer/core/css/parser/css_parser_observer.h"
#include "third_party/blink/renderer/core/css/style_rule.h"
#include "third_party/blink/renderer/platform/weborigin/kurl.h"
#include "third_party/blink/renderer/platform/wtf/text/text_position.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {

class Document;

typedef Vector<unsigned> LineEndings;

class CORE_EXPORT InspectorCSSParserObserver final : public CSSParserObserver {
  STACK_ALLOCATED();

 public:
  struct IssueReportingContext {
    KURL DocumentURL;
    TextPosition OffsetInSource;
  };
  InspectorCSSParserObserver(
      const String& parsed_text,
      Document* document,
      CSSRuleSourceDataList* result,
      std::optional<IssueReportingContext> issue_reporting_context = {})
      : parsed_text_(parsed_text),
        document_(document),
        result_(result),
        current_rule_data_(nullptr),
        issue_reporting_context_(issue_reporting_context),
        line_endings_(std::make_unique<LineEndings>()) {
    DCHECK(result_);
  }

 private:
  void StartRuleHeader(StyleRule::RuleType, unsigned) override;
  void EndRuleHeader(unsigned) override;
  void ObserveSelector(unsigned start_offset, unsigned end_offset) override;
  void StartRuleBody(unsigned) override;
  void EndRuleBody(unsigned) override;
  void ObserveProperty(unsigned start_offset,
                       unsigned end_offset,
                       bool is_important,
                       bool is_parsed) override;
  void ObserveComment(unsigned start_offset, unsigned end_offset) override;
  void ObserveErroneousAtRule(
      unsigned start_offset,
      CSSAtRuleID id,
      const Vector<CSSPropertyID, 2>& invalid_properties = {}) override;
  void ObserveNestedDeclarations(wtf_size_t insert_rule_index) override;

  TextPosition GetTextPosition(unsigned start_offset);
  void AddNewRuleToSourceTree(CSSRuleSourceData*);
  void RemoveLastRuleFromSourceTree();
  CSSRuleSourceData* PopRuleData();
  template <typename CharacterType>
  inline void SetRuleHeaderEnd(const base::span<const CharacterType>, unsigned);
  const LineEndings* GetLineEndings();
  void ReportPropertyRuleFailure(unsigned start_offset,
                                 CSSPropertyID invalid_property);

  const String& parsed_text_;
  Document* document_;
  CSSRuleSourceDataList* result_;
  CSSRuleSourceDataList current_rule_data_stack_;
  CSSRuleSourceData* current_rule_data_;
  std::optional<IssueReportingContext> issue_reporting_context_;
  std::unique_ptr<LineEndings> line_endings_;
  // A property that fails to parse (ObserveProperty with is_parsed=false)
  // temporarily becomes a replaceable property. A replaceable property can be
  // replaced by a (valid) style rule at the same offset. This is needed to
  // interpret the parsing behavior seen with nested style rules that start with
  // <ident-token>, where we first try to parse the token sequence as a
  // property, and then (if that fails) restart parsing as a style rule. This
  // means that we'll see both a ObserveProperty event and a StartRuleHeader
  // event at the same offset.
  //
  // When this situation happens, we remove the CSSPropertySourceData previously
  // produced by ObserveProperty once we've seen a valid style rule at the same
  // offset. Note that we do not consider a rule valid until we see the
  // StartRuleBody event, so the actual replacement takes place there.
  std::optional<unsigned> replaceable_property_offset_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_INSPECTOR_INSPECTOR_CSS_PARSER_OBSERVER_H_
