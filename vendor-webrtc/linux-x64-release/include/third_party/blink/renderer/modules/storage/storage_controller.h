// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_STORAGE_STORAGE_CONTROLLER_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_STORAGE_STORAGE_CONTROLLER_H_

#include "base/functional/callback.h"
#include "base/sequence_checker.h"
#include "mojo/public/cpp/bindings/pending_receiver.h"
#include "mojo/public/cpp/bindings/receiver.h"
#include "mojo/public/cpp/bindings/remote.h"
#include "third_party/blink/public/common/dom_storage/session_storage_namespace_id.h"
#include "third_party/blink/public/mojom/dom_storage/dom_storage.mojom-blink.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/modules/storage/storage_area.h"
#include "third_party/blink/renderer/platform/heap/persistent.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

class CachedStorageArea;
class InspectorDOMStorageAgent;
class LocalDOMWindow;
class LocalFrame;
class StorageNamespace;

// Singleton that manages the creation & accounting for DOMStorage objects. It
// does this by holding weak references to all session storage namespaces, and
// owning the local storage namespace internally. The total cache size is
// exposed with `TotalCacheSize()`, and `ClearAreasIfNeeded()` will - if our
// total cache size is larger than `total_cache_limit` - clear away any cache
// areas in live namespaces that no longer have references from Blink objects.
//
// SessionStorage StorageNamespace objects are created with
// `CreateSessionStorageNamespace` and live as a supplement on the Page.
//
// The LocalStorage StorageNamespace object is owned internally, and
// StorageController delegates the following methods to that namespace:
// GetLocalStorageArea, AddLocalStorageInspectorStorageAgent,
// RemoveLocalStorageInspectorStorageAgent
class MODULES_EXPORT StorageController : public mojom::blink::DomStorageClient {
  USING_FAST_MALLOC(StorageController);

 public:
  // Returns the one global StorageController instance.
  static StorageController* GetInstance();

  static bool CanAccessStorageArea(LocalFrame* frame,
                                   StorageArea::StorageType type);

  // Visible for testing.
  struct DomStorageConnection {
    mojo::Remote<mojom::blink::DomStorage> dom_storage_remote;
    mojo::PendingReceiver<mojom::blink::DomStorageClient> client_receiver;
  };
  StorageController(DomStorageConnection connection, size_t total_cache_limit);

  // Creates a MakeGarbageCollected<StorageNamespace> for Session storage, and
  // holds a weak reference for accounting & clearing. If there is already a
  // StorageNamespace created for the given id, it is returned.
  StorageNamespace* CreateSessionStorageNamespace(Page& page,
                                                  const String& namespace_id);

  // Returns the total size of all cached areas in namespaces this controller
  // knows of.
  size_t TotalCacheSize() const;

  // Cleans up unused areas if the total cache size is over the cache limit.
  void ClearAreasIfNeeded();

  // Methods that delegate to the internal StorageNamespace used for
  // LocalStorage:
  scoped_refptr<CachedStorageArea> GetLocalStorageArea(
      LocalDOMWindow* local_dom_window,
      mojo::PendingRemote<mojom::blink::StorageArea> local_storage_area = {},
      StorageNamespace::StorageContext context =
          StorageNamespace::StorageContext::kStandard);
  void AddLocalStorageInspectorStorageAgent(InspectorDOMStorageAgent* agent);
  void RemoveLocalStorageInspectorStorageAgent(InspectorDOMStorageAgent* agent);

  mojom::blink::DomStorage* dom_storage() const {
    return dom_storage_remote_.get();
  }

 private:
  void EnsureLocalStorageNamespaceCreated();

  // mojom::blink::DomStorageClient:
  void ResetStorageAreaAndNamespaceConnections() override;

  Persistent<GCedHeapHashMap<String, WeakMember<StorageNamespace>>> namespaces_;
  Persistent<StorageNamespace> local_storage_namespace_;
  size_t total_cache_limit_;

  mojo::Remote<mojom::blink::DomStorage> dom_storage_remote_;
  mojo::Receiver<mojom::blink::DomStorageClient> dom_storage_client_receiver_{
      this};

  SEQUENCE_CHECKER(sequence_checker_);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_STORAGE_STORAGE_CONTROLLER_H_
