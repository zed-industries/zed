// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_CONTENT_EXTRACTION_INNER_TEXT_AGENT_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_CONTENT_EXTRACTION_INNER_TEXT_AGENT_H_

#include "base/types/pass_key.h"
#include "mojo/public/cpp/bindings/pending_receiver.h"
#include "third_party/blink/public/mojom/content_extraction/inner_text.mojom-blink.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_receiver_set.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_wrapper_mode.h"
#include "third_party/blink/renderer/platform/supplementable.h"

namespace blink {

class Document;
class LocalFrame;

// InnerTextAgent is responsible for handling requests for inner-text. It calls
// to InnerTextBuilder to handle building of the text.
class InnerTextAgent final : public GarbageCollected<InnerTextAgent>,
                             public mojom::blink::InnerTextAgent,
                             public Supplement<Document> {
 public:
  static const char kSupplementName[];
  static InnerTextAgent* From(Document&);
  static void BindReceiver(
      LocalFrame* frame,
      mojo::PendingReceiver<mojom::blink::InnerTextAgent> receiver);

  explicit InnerTextAgent(base::PassKey<InnerTextAgent>, LocalFrame&);
  InnerTextAgent(const InnerTextAgent&) = delete;
  InnerTextAgent& operator=(const InnerTextAgent&) = delete;
  ~InnerTextAgent() override;

  void Trace(Visitor* visitor) const override;

  // mojom::blink::InnerTextAgent overrides.
  void GetInnerText(mojom::blink::InnerTextParamsPtr params,
                    GetInnerTextCallback callback) override;

 private:
  void Bind(mojo::PendingReceiver<mojom::blink::InnerTextAgent> receiver);

  HeapMojoReceiverSet<mojom::blink::InnerTextAgent, InnerTextAgent>
      receiver_set_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_CONTENT_EXTRACTION_INNER_TEXT_AGENT_H_
