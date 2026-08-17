// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_HTML_PARSER_HTML_DOCUMENT_PARSER_FASTPATH_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_HTML_PARSER_HTML_DOCUMENT_PARSER_FASTPATH_H_

#include "base/containers/enum_set.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/dom/parser_content_policy.h"

namespace WTF {
class String;
}  // namespace WTF

namespace blink {

class ContainerNode;
class Document;
class DocumentFragment;
class Element;

// Captures additional parsing behaviors that may require special cases.
enum class HTMLFragmentParsingBehavior {
  // Strips initial whitespace if the context element is body. This is used
  // for DOMParser, which expects initial whitespace to be stripped.
  kStripInitialWhitespaceForBody = 1 << 0,

  // Whether shadow roots are needed.
  kIncludeShadowRoots = 1 << 1,

  kMinValue = kStripInitialWhitespaceForBody,
  kMaxValue = kIncludeShadowRoots,
};

using HTMLFragmentParsingBehaviorSet =
    base::EnumSet<HTMLFragmentParsingBehavior,
                  HTMLFragmentParsingBehavior::kMinValue,
                  HTMLFragmentParsingBehavior::kMaxValue>;

// If this fails because of an unsupported tag and
// `failed_because_unsupported_tag` is non-null, then it is set to true.
CORE_EXPORT bool TryParsingHTMLFragment(
    const WTF::String& source,
    Document& document,
    ContainerNode& root_node,
    Element& context_element,
    ParserContentPolicy policy,
    HTMLFragmentParsingBehaviorSet behavior,
    bool* failed_because_unsupported_tag = nullptr);

// Logs histograms to help track why parsing failed because of an unsupported
// tag when `fragment` was generated.
void LogTagsForUnsupportedTagTypeFailure(DocumentFragment& fragment);

// Captures the potential outcomes for fast path html parser.
// These values are persisted to logs. Entries should not be renumbered and
// numeric values should never be reused.
// This is exposed for testing.
enum class HtmlFastPathResult {
  kSucceeded = 0,
  kFailedTracingEnabled = 1,
  kFailedParserContentPolicy = 2,
  kFailedInForm = 3,
  kFailedUnsupportedContextTag = 4,
  kFailedOptionWithChild = 5,
  kFailedDidntReachEndOfInput = 6,
  kFailedContainsNull = 7,
  kFailedParsingTagName = 8,
  kFailedParsingQuotedAttributeValue = 9,
  kFailedParsingUnquotedAttributeValue = 10,
  kFailedParsingQuotedEscapedAttributeValue = 11,
  kFailedParsingUnquotedEscapedAttributeValue = 12,
  kFailedParsingCharacterReference = 13,
  kFailedEndOfInputReached = 14,
  kFailedParsingAttributes = 15,
  kFailedParsingSpecificElements = 16,
  kFailedParsingElement = 17,
  kFailedUnsupportedTag = 18,
  kFailedEndOfInputReachedForContainer = 19,
  kFailedUnexpectedTagNameCloseState = 20,
  kFailedEndTagNameMismatch = 21,
  kFailedShadowRoots = 22,
  // This value is no longer used.
  // kFailedDirAttributeDirty = 23,
  kFailedOnAttribute = 24,
  kFailedMaxDepth = 25,
  kFailedBigText = 25,
  // This value is no longer used.
  kFailedCssPseudoDirEnabledAndDirAttributeDirtyDeprecated = 26,
  kMaxValue = kFailedCssPseudoDirEnabledAndDirAttributeDirtyDeprecated,
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_HTML_PARSER_HTML_DOCUMENT_PARSER_FASTPATH_H_
