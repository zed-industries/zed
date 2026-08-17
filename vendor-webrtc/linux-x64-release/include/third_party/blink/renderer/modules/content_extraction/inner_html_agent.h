// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_CONTENT_EXTRACTION_INNER_HTML_AGENT_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_CONTENT_EXTRACTION_INNER_HTML_AGENT_H_

#include "base/types/pass_key.h"
#include "mojo/public/cpp/bindings/pending_receiver.h"
#include "third_party/blink/public/mojom/content_extraction/inner_html.mojom-blink.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_receiver_set.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_wrapper_mode.h"
#include "third_party/blink/renderer/platform/supplementable.h"

namespace blink {

class Document;
class LocalFrame;

// InnerHtmlAgent is responsible for handling requests for inner-html. It calls
// to InnerHtmlBuilder to handle building of the text.
class InnerHtmlAgent final : public GarbageCollected<InnerHtmlAgent>,
                             public mojom::blink::InnerHtmlAgent,
                             public Supplement<Document> {
 public:
  static const char kSupplementName[];
  static InnerHtmlAgent* From(Document&);
  static void BindReceiver(
      LocalFrame* frame,
      mojo::PendingReceiver<mojom::blink::InnerHtmlAgent> receiver);

  explicit InnerHtmlAgent(base::PassKey<InnerHtmlAgent>, LocalFrame&);
  InnerHtmlAgent(const InnerHtmlAgent&) = delete;
  InnerHtmlAgent& operator=(const InnerHtmlAgent&) = delete;
  ~InnerHtmlAgent() override;

  void Trace(Visitor* visitor) const override;

  // mojom::blink::InnerHtmlAgent overrides.
  void GetInnerHtml(GetInnerHtmlCallback callback) override;

 private:
  void Bind(mojo::PendingReceiver<mojom::blink::InnerHtmlAgent> receiver);

  HeapMojoReceiverSet<mojom::blink::InnerHtmlAgent, InnerHtmlAgent>
      receiver_set_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_CONTENT_EXTRACTION_INNER_HTML_AGENT_H_
