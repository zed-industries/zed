// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_CONTENT_EXTRACTION_INNER_TEXT_BUILDER_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_CONTENT_EXTRACTION_INNER_TEXT_BUILDER_H_

#include <optional>

#include "base/memory/stack_allocated.h"
#include "third_party/blink/public/common/tokens/tokens.h"
#include "third_party/blink/public/mojom/content_extraction/inner_text.mojom-blink.h"
#include "third_party/blink/renderer/core/dom/text_visitor.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_vector.h"
#include "third_party/blink/renderer/platform/heap/member.h"

namespace blink {

class HTMLElement;
class HTMLIFrameElement;
class LocalFrame;

// Builds mojom::blink::InnerTextFrame for a frame, and all suitable
// iframes. See the mojom for details on the format.
//
// An InnerTextBuilder is created per Frame. Internally it tracks all iframes
// and will build results for them as well.
class MODULES_EXPORT InnerTextBuilder final : public TextVisitor {
  STACK_ALLOCATED();

 public:
  InnerTextBuilder(const InnerTextBuilder&) = delete;
  InnerTextBuilder& operator=(const InnerTextBuilder&) = delete;

  static mojom::blink::InnerTextFramePtr Build(
      LocalFrame& frame,
      const mojom::blink::InnerTextParams& params);

 private:
  // A ChildIFrame is created for every HTMLIFrame in a particular frame. These
  // are created (and added to `child_iframes_`) as the inner-text is generated.
  struct ChildIFrame : public GarbageCollected<ChildIFrame> {
    void Trace(Visitor* visitor) const;
    // The location of the child frame in the resulting inner-text of the
    // parent frame.
    unsigned offset;
    Member<const HTMLIFrameElement> iframe;
  };

  InnerTextBuilder(const mojom::blink::InnerTextParams& params,
                   HeapVector<Member<ChildIFrame>>& child_iframes);

  // Builds the results for a frame, and recurses through all child frames.
  void Build(HTMLElement& body, mojom::blink::InnerTextFrame& frame);

  // Adds text (or NodeLocation) to Segments `frame.segments`. `text_offset` is
  // the current offset into `text` and `next_child_offset` the offset into
  // `text` of the next child.
  void AddNextNonFrameSegments(const String& text,
                               unsigned next_child_offset,
                               unsigned& text_offset,
                               mojom::blink::InnerTextFrame& frame);

  // TextVisitor:
  void WillVisit(const Node& element, unsigned offset) override;

  const mojom::blink::InnerTextParams& params_;

  // Set if `params` contained a `InnerTextDomNodeId` and the node was found.
  std::optional<unsigned> matching_node_location_;

  // Child iframes encountered.
  HeapVector<Member<ChildIFrame>>& child_iframes_;
};

// An alternative implementation wrapping DocumentChunker passage extraction.
// This will be used only when one or more of the relevant optional parameters
// are specified on InnerTextParams.
class MODULES_EXPORT InnerTextPassagesBuilder final {
  STACK_ALLOCATED();

 public:
  InnerTextPassagesBuilder(const InnerTextPassagesBuilder&) = delete;
  InnerTextPassagesBuilder& operator=(const InnerTextPassagesBuilder&) = delete;

  static mojom::blink::InnerTextFramePtr Build(
      LocalFrame& frame,
      const mojom::blink::InnerTextParams& params);

 private:
  explicit InnerTextPassagesBuilder(
      const mojom::blink::InnerTextParams& params);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_CONTENT_EXTRACTION_INNER_TEXT_BUILDER_H_
