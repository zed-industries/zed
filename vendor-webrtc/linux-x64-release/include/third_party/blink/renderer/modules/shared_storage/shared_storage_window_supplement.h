// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_SHARED_STORAGE_SHARED_STORAGE_WINDOW_SUPPLEMENT_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_SHARED_STORAGE_SHARED_STORAGE_WINDOW_SUPPLEMENT_H_

#include "third_party/blink/public/mojom/shared_storage/shared_storage.mojom-blink.h"
#include "third_party/blink/renderer/core/frame/local_dom_window.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_associated_remote.h"
#include "third_party/blink/renderer/platform/supplementable.h"

namespace blink {

class MODULES_EXPORT SharedStorageWindowSupplement
    : public GarbageCollected<SharedStorageWindowSupplement>,
      public Supplement<LocalDOMWindow> {
 public:
  static SharedStorageWindowSupplement* From(LocalDOMWindow& window);

  static const char kSupplementName[];

  void Trace(Visitor*) const override;

  explicit SharedStorageWindowSupplement(LocalDOMWindow&);

  // Initializes (if needed) and returns `shared_storage_document_service_`.
  // Prerequisite: a frame must be connected to the associated `LocalDOMWindow`.
  mojom::blink::SharedStorageDocumentService* GetSharedStorageDocumentService();

 private:
  HeapMojoAssociatedRemote<mojom::blink::SharedStorageDocumentService>
      shared_storage_document_service_{nullptr};
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_SHARED_STORAGE_SHARED_STORAGE_WINDOW_SUPPLEMENT_H_
