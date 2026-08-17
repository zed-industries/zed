/*
 * Copyright (c) 2010 Google Inc. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are
 * met:
 *
 *     * Redistributions of source code must retain the above copyright
 * notice, this list of conditions and the following disclaimer.
 *     * Redistributions in binary form must reproduce the above
 * copyright notice, this list of conditions and the following disclaimer
 * in the documentation and/or other materials provided with the
 * distribution.
 *     * Neither the name of Google Inc. nor the names of its
 * contributors may be used to endorse or promote products derived from
 * this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 * "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 * LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
 * A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
 * OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 * SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
 * LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
 * DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
 * THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_PROPERTY_SOURCE_DATA_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_PROPERTY_SOURCE_DATA_H_

#include "third_party/blink/renderer/core/css/style_rule.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/wtf/forward.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {

class SourceRange {
  DISALLOW_NEW();

 public:
  SourceRange() = default;
  SourceRange(unsigned start, unsigned end) : start(start), end(end) {}
  unsigned length() const { return end - start; }
  bool operator==(const SourceRange& o) const {
    return start == o.start && end == o.end;
  }
  bool operator!=(const SourceRange& o) const { return !operator==(o); }

  unsigned start = 0;
  unsigned end = 0;
};

}  // namespace blink

WTF_ALLOW_MOVE_AND_INIT_WITH_MEM_FUNCTIONS(blink::SourceRange)

namespace blink {

class CSSPropertySourceData {
  DISALLOW_NEW();

 public:
  CSSPropertySourceData(const String& name,
                        const String& value,
                        bool important,
                        bool disabled,
                        bool parsed_ok,
                        const SourceRange& range);
  CSSPropertySourceData(const CSSPropertySourceData& other);

  String name;
  String value;
  bool important;
  bool disabled;
  bool parsed_ok;
  SourceRange range;
};

}  // namespace blink

WTF_ALLOW_MOVE_AND_INIT_WITH_MEM_FUNCTIONS(blink::CSSPropertySourceData)

namespace blink {

class CSSRuleSourceData final : public GarbageCollected<CSSRuleSourceData> {
 public:
  explicit CSSRuleSourceData(StyleRule::RuleType type) : type(type) {}
  void Trace(Visitor* visitor) const { visitor->Trace(child_rules); }

  bool HasProperties() const {
    return type == StyleRule::kStyle || type == StyleRule::kFontFace ||
           type == StyleRule::kPage || type == StyleRule::kPageMargin ||
           type == StyleRule::kProperty || type == StyleRule::kKeyframe ||
           type == StyleRule::kFontPaletteValues ||
           type == StyleRule::kPositionTry;
  }

  bool HasMedia() const {
    return type == StyleRule::kMedia || type == StyleRule::kImport;
  }

  bool HasContainer() const { return type == StyleRule::kContainer; }

  bool HasSupports() const { return type == StyleRule::kSupports; }

  bool HasScope() const { return type == StyleRule::kScope; }

  StyleRule::RuleType type;

  // Range of the selector list in the enclosing source.
  SourceRange rule_header_range;

  // Range of the rule body, i.e. the range between {}.
  SourceRange rule_body_range;

  // The range for the leading declarations. This may be different from
  // `rule_body_range` for nested style rules, e.g.:
  //
  //   .a {
  //     color: green; /* leading */
  //     left: 10px;   /* leading */
  //     & { }
  //     opacity: 2px; /* not leading */
  //   }
  //
  // Non-leading declarations are wrapped in child rules of type
  // CSSNestedDeclarations, and therefore not considered part of the
  // declarations for *this* rule.
  SourceRange rule_declarations_range;

  // Only for CSSStyleRules.
  Vector<SourceRange> selector_ranges;

  // Only for CSSStyleRules, CSSFontFaceRules, and CSSPageRules.
  Vector<CSSPropertySourceData> property_data;

  // Only for CSSMediaRules.
  HeapVector<Member<CSSRuleSourceData>> child_rules;

  // Only for CSSMediaRules and CSSImportRules.
  // Source ranges for media query -> expression -> value.
  Vector<Vector<SourceRange>> media_query_exp_value_ranges;
};

using CSSRuleSourceDataList = HeapVector<Member<CSSRuleSourceData>>;
using GCedCSSRuleSourceDataList = GCedHeapVector<Member<CSSRuleSourceData>>;

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_PROPERTY_SOURCE_DATA_H_
