// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_DOCUMENT_METADATA_DOCUMENT_METADATA_SERVER_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_DOCUMENT_METADATA_DOCUMENT_METADATA_SERVER_H_

#include "base/types/pass_key.h"
#include "mojo/public/cpp/bindings/pending_receiver.h"
#include "third_party/blink/public/mojom/document_metadata/document_metadata.mojom-blink.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_receiver.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_wrapper_mode.h"
#include "third_party/blink/renderer/platform/supplementable.h"

namespace blink {

class Document;
class LocalFrame;

// Mojo interface to return extracted metadata for AppIndexing.
class DocumentMetadataServer final
    : public GarbageCollected<DocumentMetadataServer>,
      public mojom::blink::DocumentMetadata,
      public Supplement<Document> {
 public:
  static const char kSupplementName[];
  static DocumentMetadataServer* From(Document&);
  static void BindReceiver(
      LocalFrame*,
      mojo::PendingReceiver<mojom::blink::DocumentMetadata>);

  explicit DocumentMetadataServer(base::PassKey<DocumentMetadataServer>,
                                  LocalFrame&);

  // Not copyable or movable
  DocumentMetadataServer(const DocumentMetadataServer&) = delete;
  DocumentMetadataServer& operator=(const DocumentMetadataServer&) = delete;

  void Trace(Visitor* visitor) const override;

  // mojom::blink::DocumentMetadata overrides.
  void GetEntities(GetEntitiesCallback) override;

 private:
  void Bind(mojo::PendingReceiver<mojom::blink::DocumentMetadata> receiver);

  HeapMojoReceiver<mojom::blink::DocumentMetadata, DocumentMetadataServer>
      receiver_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_DOCUMENT_METADATA_DOCUMENT_METADATA_SERVER_H_
