// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_STORAGE_CACHED_STORAGE_AREA_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_STORAGE_CACHED_STORAGE_AREA_H_

#include "base/memory/weak_ptr.h"
#include "base/trace_event/memory_dump_provider.h"
#include "mojo/public/cpp/bindings/pending_remote.h"
#include "mojo/public/cpp/bindings/receiver.h"
#include "mojo/public/cpp/bindings/remote.h"
#include "third_party/blink/public/mojom/dom_storage/storage_area.mojom-blink.h"
#include "third_party/blink/public/platform/scheduler/web_scoped_virtual_time_pauser.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/modules/storage/storage_area_map.h"
#include "third_party/blink/renderer/modules/storage/storage_namespace.h"
#include "third_party/blink/renderer/platform/heap/persistent.h"
#include "third_party/blink/renderer/platform/storage/blink_storage_key.h"
#include "third_party/blink/renderer/platform/wtf/deque.h"
#include "third_party/blink/renderer/platform/wtf/hash_map.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

class LocalDOMWindow;

// An in-process implementation of LocalStorage using a LevelDB Mojo service.
// Maintains a complete cache of the BlinkStorageKey's Map of key/value pairs
// for fast access. The cache is primed on first access and changes are written
// to the backend through the level db interface pointer. Mutations originating
// in other processes are applied to the cache via mojom::LevelDBObserver
// callbacks.
// There is one CachedStorageArea for potentially many LocalStorageArea
// objects.
class MODULES_EXPORT CachedStorageArea
    : public mojom::blink::StorageAreaObserver,
      public RefCounted<CachedStorageArea>,
      public base::trace_event::MemoryDumpProvider {
 public:
  // Instances of this class are used to identify the "source" of any changes
  // made to this storage area, as well as to dispatch any incoming change
  // events. Change events are not sent back to the source that caused the
  // change. The source passed to the various methods that modify storage
  // should have been registered first by calling RegisterSource.
  class Source : public GarbageCollectedMixin {
   public:
    virtual ~Source() = default;
    virtual KURL GetPageUrl() const = 0;
    // Return 'true' to continue receiving events, and 'false' to stop.
    virtual bool EnqueueStorageEvent(const String& key,
                                     const String& old_value,
                                     const String& new_value,
                                     const String& url) = 0;
    virtual blink::WebScopedVirtualTimePauser CreateWebScopedVirtualTimePauser(
        const char* name,
        WebScopedVirtualTimePauser::VirtualTaskDuration duration) = 0;
    virtual LocalDOMWindow* GetDOMWindow() = 0;
  };

  enum class AreaType {
    kSessionStorage,
    kLocalStorage,
  };

  CachedStorageArea(
      AreaType type,
      const BlinkStorageKey& storage_key,
      LocalDOMWindow* local_dom_window,
      StorageNamespace* storage_namespace,
      bool is_session_storage_for_prerendering,
      mojo::PendingRemote<mojom::blink::StorageArea> storage_area = {});

  CachedStorageArea(const CachedStorageArea&) = delete;
  CachedStorageArea& operator=(const CachedStorageArea&) = delete;

  // These correspond to blink::Storage.
  unsigned GetLength();
  String GetKey(unsigned index);
  String GetItem(const String& key);
  bool SetItem(const String& key, const String& value, Source* source);
  void RemoveItem(const String& key, Source* source);
  void Clear(Source* source);

  // Allow this object to keep track of the Source instances corresponding to
  // it, which is needed for mutation event notifications.
  // Returns the (unique) id allocated for this source for testing purposes.
  String RegisterSource(Source* source);

  size_t quota_used() const { return map_ ? map_->quota_used() : 0; }
  size_t memory_used() const { return map_ ? map_->memory_used() : 0; }

  // Only public to allow tests to parametrize on this type.
  enum class FormatOption {
    kLocalStorageDetectFormat,
    kSessionStorageForceUTF16,
    kSessionStorageForceUTF8
  };

  mojo::Remote<mojom::blink::StorageArea>& RemoteArea() { return remote_area_; }

  // Invoked by the owning StorageNamespace the renderer is told to reset its
  // DOM Storage connections (e.g. to recover from a backend crash). Normally
  // a new StorageArea pipe is bound through the owning StorageNamespace, but
  // |new_area| may be provided by test code instead.
  void ResetConnection(
      mojo::PendingRemote<mojom::blink::StorageArea> new_area = {});

  bool is_session_storage_for_prerendering() const {
    return is_session_storage_for_prerendering_;
  }

  void EvictCachedData();

  void SetRemoteAreaForTesting(
      mojo::PendingRemote<mojom::blink::StorageArea> area) {
    remote_area_.Bind(std::move(area));
  }

 private:
  friend class RefCounted<CachedStorageArea>;
  ~CachedStorageArea() override;

  friend class CachedStorageAreaTest;
  friend class CachedStorageAreaStringFormatTest;
  friend class MockStorageArea;

  // Simple structure used to return information about each locally-initiated
  // mutation on the StorageArea until the mutation is acknowledged by a
  // corresponding StorageAreaObserver event. See |pending_mutations_by_source_|
  // and |pending_mutations_by_key_| below.
  struct PendingMutation {
    String key;
    String new_value;
    String old_value;
  };

  LocalDOMWindow* GetBestCurrentDOMWindow();

  void BindStorageArea(
      mojo::PendingRemote<mojom::blink::StorageArea> new_area = {},
      LocalDOMWindow* local_dom_window = nullptr);

  // mojom::blink::StorageAreaObserver:
  void KeyChanged(const Vector<uint8_t>& key,
                  const Vector<uint8_t>& new_value,
                  const std::optional<Vector<uint8_t>>& old_value,
                  const String& source) override;
  void KeyChangeFailed(const Vector<uint8_t>& key,
                       const String& source) override;
  void KeyDeleted(const Vector<uint8_t>& key,
                  const std::optional<Vector<uint8_t>>& old_value,
                  const String& source) override;
  void AllDeleted(bool was_nonempty, const String& source) override;
  void ShouldSendOldValueOnMutations(bool value) override;

  // base::trace_event::MemoryDumpProvider:
  bool OnMemoryDump(const base::trace_event::MemoryDumpArgs& args,
                    base::trace_event::ProcessMemoryDump* pmd) override;

  // Enqueues state regarding a locally-initiated storage mutation which will
  // eventually be acknowledged by a StorageAreaObserver event targeting
  // |receiver_|.
  void EnqueuePendingMutation(const String& key,
                              const String& new_value,
                              const String& old_value,
                              const String& source);

  // Dequeues and returns the oldest PendingMutation from the queue for |source|
  // if one exists. If there is no pending mutation queued for |source| this
  // returns null.
  std::unique_ptr<PendingMutation> PopPendingMutation(const String& source);

  void MaybeApplyNonLocalMutationForKey(const String& key,
                                        const String& new_value);

  // Synchronously fetches the areas data if it hasn't been fetched already.
  void EnsureLoaded();

  bool IsSessionStorage() const;
  FormatOption GetKeyFormat() const;
  FormatOption GetValueFormat() const;

  void EnqueueStorageEvent(const String& key,
                           const String& old_value,
                           const String& new_value,
                           const String& url,
                           const String& storage_area_id);

  void EnqueueCheckpointMicrotask(Source* source);
  void NotifyCheckpoint();

  static String Uint8VectorToString(const Vector<uint8_t>& input,
                                    FormatOption format_option);
  static Vector<uint8_t> StringToUint8Vector(const String& input,
                                             FormatOption format_option);

  const AreaType type_;
  const BlinkStorageKey storage_key_;
  const WeakPersistent<StorageNamespace> storage_namespace_;
  // Session storage state for prerendering is initialized by cloning the
  // primary session storage state. It is used locally by the prerendering
  // context, and does not get propagated back to the primary state (i.e., via
  // remote_area_). For more details:
  // https://docs.google.com/document/d/1I5Hr8I20-C1GBr4tAXdm0U8a1RDUKHt4n7WcH4fxiSE/edit?usp=sharing
  const bool is_session_storage_for_prerendering_;

  std::unique_ptr<StorageAreaMap> map_;

  // Queues of local mutations which are pending browser acknowledgement via
  // StorageAreaObserver events. This map is keyed by local source ID and owns
  // the PendingMutation objects.
  //
  // Only used for Local Storage. Session Storage operations are confined to
  // local side-effects and are not acknowledged with StorageAreaObsever events.
  using OwnedPendingMutationQueue = Deque<std::unique_ptr<PendingMutation>>;
  HashMap<String, OwnedPendingMutationQueue> pending_mutations_by_source_;

  // Queues of local mutations indexed per key. Every queued value references a
  // PendingMutation owned by |pending_mutations_by_source_| above.
  //
  // These per-key queues exists for two main reasons:
  //
  // * As long as a key's queue is non-empty, non-local mutations observed for
  //   the key are ignored.
  // * Any time a non-local |AllDeleted| (i.e. script |clear()|) is observed,
  //   the most recent mutation for each key is re-applied locally to the
  //   cleared area, as this improves locally script-observable consistency.
  //
  // Only used for Local Storage. Session Storage operations are confined to
  // local side-effects and are not acknowledged with StorageAreaObsever events.
  HashMap<String, Deque<PendingMutation*>> pending_mutations_by_key_;

  // See ShouldSendOldValueOnMutations().
  bool should_send_old_value_on_mutations_ = true;

  bool checkpoint_queued_ = false;

  // Connection to the backing implementation of this StorageArea. This is
  // always bound.
  mojo::Remote<mojom::blink::StorageArea> remote_area_;

  // Receives StorageAreaObserver events from the StorageArea implementation and
  // dispatches them to this CachedStorageArea.
  mojo::Receiver<mojom::blink::StorageAreaObserver> receiver_{this};

  Persistent<GCedHeapHashMap<WeakMember<Source>, String>> areas_;

  base::WeakPtrFactory<CachedStorageArea> weak_factory_{this};
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_STORAGE_CACHED_STORAGE_AREA_H_
