/*
 * Copyright (C) 2008 Apple Inc. All Rights Reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY APPLE INC. ``AS IS'' AND ANY
 * EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR
 * PURPOSE ARE DISCLAIMED.  IN NO EVENT SHALL APPLE INC. OR
 * CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
 * EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
 * PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
 * PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY
 * OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_STORAGE_STORAGE_NAMESPACE_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_STORAGE_STORAGE_NAMESPACE_H_

#include "base/memory/raw_ptr.h"
#include "base/task/single_thread_task_runner.h"
#include "mojo/public/cpp/bindings/pending_receiver.h"
#include "third_party/blink/public/common/dom_storage/session_storage_namespace_id.h"
#include "third_party/blink/public/mojom/dom_storage/dom_storage.mojom-blink-forward.h"
#include "third_party/blink/public/mojom/dom_storage/session_storage_namespace.mojom-blink.h"
#include "third_party/blink/public/mojom/dom_storage/storage_area.mojom-blink-forward.h"
#include "third_party/blink/renderer/core/page/page.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_set.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_remote.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_wrapper_mode.h"
#include "third_party/blink/renderer/platform/storage/blink_storage_key.h"
#include "third_party/blink/renderer/platform/storage/blink_storage_key_hash.h"
#include "third_party/blink/renderer/platform/supplementable.h"
#include "third_party/blink/renderer/platform/wtf/hash_map.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

class CachedStorageArea;
class LocalDOMWindow;
class InspectorDOMStorageAgent;
class StorageController;

// Contains DOMStorage storage areas for BlinkStorageKeys & handles inspector
// agents. A namespace is either a SessionStorage namespace with a namespace_id,
// or a LocalStorage namespace with no (or an empty) namespace_id. The
// LocalStorage version of the StorageNamespace lives in the StorageController.
// InspectorDOMStorageAgents that are registered on this object are notified
// through `DidDispatchStorageEvent`.
class MODULES_EXPORT StorageNamespace final
    : public GarbageCollected<StorageNamespace>,
      public Supplement<Page> {
 public:
  // `kStandard` is access for a given context's storage, while
  // `kStorageAccessAPI` indicates a desire to load the first-party storage from
  // a third-party context. For more see:
  // third_party/blink/renderer/modules/storage_access/README.md
  enum class StorageContext { kStandard, kStorageAccessAPI };
  static const char kSupplementName[];

  static void ProvideSessionStorageNamespaceTo(
      Page&,
      const SessionStorageNamespaceId&);
  static StorageNamespace* From(Page* page) {
    return Supplement<Page>::From<StorageNamespace>(page);
  }

  // Creates a namespace for LocalStorage.
  StorageNamespace(StorageController*);
  // Creates a namespace for SessionStorage.
  StorageNamespace(Page& page, StorageController*, const String& namespace_id);

  // `storage_area` is ignored here if a cached namespace already exists.
  scoped_refptr<CachedStorageArea> GetCachedArea(
      LocalDOMWindow* local_dom_window,
      mojo::PendingRemote<mojom::blink::StorageArea> storage_area = {},
      StorageContext context = StorageContext::kStandard);

  scoped_refptr<CachedStorageArea> CreateCachedAreaForPrerender(
      LocalDOMWindow* local_dom_window,
      mojo::PendingRemote<mojom::blink::StorageArea> storage_area = {});

  void EvictSessionStorageCachedData();

  // Only valid to call this if `this` and `target` are session storage
  // namespaces.
  void CloneTo(const String& target);

  size_t TotalCacheSize() const;

  // Removes any CachedStorageAreas that aren't referenced by any source.
  void CleanUpUnusedAreas();

  bool IsSessionStorage() const { return !namespace_id_.empty(); }

  void AddInspectorStorageAgent(InspectorDOMStorageAgent* agent);
  void RemoveInspectorStorageAgent(InspectorDOMStorageAgent* agent);

  void Trace(Visitor* visitor) const override;

  // Iterates all of the inspector agents and calls
  // `DidDispatchDOMStorageEvent`.
  void DidDispatchStorageEvent(const BlinkStorageKey& storage_key,
                               const String& key,
                               const String& old_value,
                               const String& new_value);

  // Called by areas in `cached_areas_` to bind/rebind their StorageArea
  // interface.
  void BindStorageArea(
      const BlinkStorageKey& storage_key,
      const LocalFrameToken& local_frame_token,
      mojo::PendingReceiver<mojom::blink::StorageArea> receiver);

  // If this StorageNamespace was previously connected to the backend, this
  // forcibly disconnects it so that it reconnects lazily when next needed.
  // Also forces all owned CachedStorageAreas to be reconnected.
  void ResetStorageAreaAndNamespaceConnections();

 private:
  void EnsureConnected();

  HeapHashSet<WeakMember<InspectorDOMStorageAgent>> inspector_agents_;

  // Lives globally.
  raw_ptr<StorageController> controller_;
  String namespace_id_;
  scoped_refptr<base::SingleThreadTaskRunner> task_runner_;
  // `StorageNamespace` is a per-Page object and doesn't have any
  // `ExecutionContext`.
  HeapMojoRemote<mojom::blink::SessionStorageNamespace> namespace_{nullptr};
  // TODO(https://crbug.com/1212808) Migrate hash map and function.
  HashMap<std::unique_ptr<const BlinkStorageKey>,
          scoped_refptr<CachedStorageArea>,
          BlinkStorageKeyHashTraits>
      cached_areas_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_STORAGE_STORAGE_NAMESPACE_H_
