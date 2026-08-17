// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_PARSER_CSS_LAZY_PARSING_STATE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_PARSER_CSS_LAZY_PARSING_STATE_H_

#include "third_party/blink/renderer/core/css/css_selector_list.h"
#include "third_party/blink/renderer/core/css/parser/css_parser_mode.h"
#include "third_party/blink/renderer/core/css/style_sheet_contents.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

// This class helps lazy parsing by retaining necessary state. It should not
// outlive the StyleSheetContents that initiated the parse, as it retains a raw
// reference to the UseCounter associated with the style sheet.
//
// Note: This class holds an extra reference to the underlying stylesheet
// text, and will extend its lifetime until this class is garbage collected.
// Currently, the only strong references to this class are from individual lazy
// properties, so after an entire lazy sheet is parsed, the extra memory should
// be released.
class CSSLazyParsingState final : public GarbageCollected<CSSLazyParsingState> {
 public:
  CSSLazyParsingState(const CSSParserContext*,
                      const String& sheet_text,
                      StyleSheetContents*);

  const CSSParserContext* Context();
  const String& SheetText() const { return sheet_text_; }

  void Trace(Visitor*) const;

 private:
  Member<const CSSParserContext> context_;
  // Also referenced on the css resource.
  String sheet_text_;

  // Weak to ensure lazy state will never cause the contents to live longer than
  // it should (we DCHECK this fact). Normally, the <link> tag will keep the
  // StyleSheetContents alive.
  //
  // When we mutate a stylesheet's rules, we do copy-on-write on its
  // StyleSheetContents, invalidating this pointer. However, we also Copy()
  // every single rule, which parses them eagerly, so we don't need to worry
  // about what happens to the CSSLazyParsingState in that case.
  // This happens in StyleSheetContents' copy constructor.
  WeakMember<StyleSheetContents> owning_contents_;

  // Cache the document as a proxy for caching the UseCounter. Grabbing the
  // UseCounter per every property parse is a bit more expensive.
  WeakMember<Document> document_;

  // Whether or not use counting is enabled for parsing. This will usually be
  // true, except for when stylesheets with @imports are removed from the page.
  // See StyleRuleImport::setCSSStyleSheet.
  const bool should_use_count_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_PARSER_CSS_LAZY_PARSING_STATE_H_
