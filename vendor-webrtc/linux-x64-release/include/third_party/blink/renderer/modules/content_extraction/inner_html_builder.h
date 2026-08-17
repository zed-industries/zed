// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_CONTENT_EXTRACTION_INNER_HTML_BUILDER_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_CONTENT_EXTRACTION_INNER_HTML_BUILDER_H_

#include "base/memory/stack_allocated.h"
#include "third_party/blink/public/mojom/content_extraction/inner_html.mojom-blink.h"
#include "third_party/blink/renderer/core/editing/serializers/markup_accumulator.h"
#include "third_party/blink/renderer/modules/modules_export.h"

namespace blink {

class Document;
class HTMLElement;
class LocalFrame;

// Builds inner-html result for InnerHtmlAgent.
//
// Subclasses MarkupAccumulator to filter the set of elements includes.
class MODULES_EXPORT InnerHtmlBuilder final : public MarkupAccumulator {
  STACK_ALLOCATED();

 public:
  InnerHtmlBuilder(const InnerHtmlBuilder&) = delete;
  InnerHtmlBuilder& operator=(const InnerHtmlBuilder&) = delete;

  static String Build(LocalFrame& frame);

 private:
  explicit InnerHtmlBuilder(Document& d);

  // Builds the inner-html for `body`.
  String Build(HTMLElement& body);

  // MarkupAccumulator:
  EmitElementChoice WillProcessElement(const Element& e) override;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_CONTENT_EXTRACTION_INNER_HTML_BUILDER_H_
