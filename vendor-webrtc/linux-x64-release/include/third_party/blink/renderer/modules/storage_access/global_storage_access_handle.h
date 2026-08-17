// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_STORAGE_ACCESS_GLOBAL_STORAGE_ACCESS_HANDLE_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_STORAGE_ACCESS_GLOBAL_STORAGE_ACCESS_HANDLE_H_

#include "base/types/pass_key.h"
#include "mojo/public/cpp/bindings/pending_remote.h"
#include "third_party/blink/public/mojom/broadcastchannel/broadcast_channel.mojom-blink.h"
#include "third_party/blink/public/mojom/storage_access/storage_access_handle.mojom-blink.h"
#include "third_party/blink/public/mojom/worker/shared_worker_connector.mojom-blink.h"
#include "third_party/blink/renderer/core/fileapi/public_url_manager.h"
#include "third_party/blink/renderer/core/frame/local_dom_window.h"
#include "third_party/blink/renderer/modules/cache_storage/cache_storage.h"
#include "third_party/blink/renderer/modules/indexeddb/idb_factory.h"
#include "third_party/blink/renderer/modules/locks/lock_manager.h"
#include "third_party/blink/renderer/modules/storage/storage_area.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/supplementable.h"

namespace blink {

// This singleton-per-LocalDOMWindow owns the objects/remotes needed for any
// StorageAccessHandle on that same LocalDOMWindow. We store them here instead
// of on the StorageAccessHandle to avoid duplicate constructions and to prevent
// disconnections when the handle is garbage collected.
class GlobalStorageAccessHandle final
    : public GarbageCollected<GlobalStorageAccessHandle>,
      public Supplement<LocalDOMWindow> {
 public:
  static const char kSupplementName[];

  static GlobalStorageAccessHandle& From(LocalDOMWindow& window);

  explicit GlobalStorageAccessHandle(base::PassKey<GlobalStorageAccessHandle>,
                                     LocalDOMWindow& window)
      : Supplement<LocalDOMWindow>(window),
        remote_(window.GetExecutionContext()),
        broadcast_channel_provider_(window.GetExecutionContext()),
        shared_worker_connector_(window.GetExecutionContext()) {}

  HeapMojoRemote<mojom::blink::StorageAccessHandle>& GetRemote();
  StorageArea* GetSessionStorageArea();
  StorageArea* GetLocalStorageArea();
  IDBFactory* GetIDBFactory();
  LockManager* GetLockManager();
  CacheStorage* GetCacheStorage();
  PublicURLManager* GetPublicURLManager();
  HeapMojoAssociatedRemote<mojom::blink::BroadcastChannelProvider>&
  GetBroadcastChannelProvider();
  HeapMojoRemote<mojom::blink::SharedWorkerConnector>&
  GetSharedWorkerConnector();

  void Trace(Visitor* visitor) const override;

 private:
  HeapMojoRemote<mojom::blink::StorageAccessHandle> remote_;
  Member<StorageArea> session_storage_area_;
  Member<StorageArea> local_storage_area_;
  Member<IDBFactory> idb_factory_;
  Member<LockManager> lock_manager_;
  Member<CacheStorage> cache_storage_;
  Member<PublicURLManager> public_url_manager_;
  HeapMojoAssociatedRemote<mojom::blink::BroadcastChannelProvider>
      broadcast_channel_provider_;
  HeapMojoRemote<mojom::blink::SharedWorkerConnector> shared_worker_connector_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_STORAGE_ACCESS_GLOBAL_STORAGE_ACCESS_HANDLE_H_
