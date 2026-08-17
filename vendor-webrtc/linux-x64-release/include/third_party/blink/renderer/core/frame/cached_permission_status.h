// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_CACHED_PERMISSION_STATUS_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_CACHED_PERMISSION_STATUS_H_

#include "base/memory/scoped_refptr.h"
#include "base/task/single_thread_task_runner.h"
#include "third_party/blink/public/mojom/permissions/permission.mojom-blink.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/graphics/dom_node_id.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_map.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_set.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_receiver_set.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_remote.h"
#include "third_party/blink/renderer/platform/wtf/forward.h"

namespace blink {

class LocalDOMWindow;

// This cache keeps track of permission statuses, restricted to the permission
// element. These permission statuses are not canonical and should not be used
// for actual permission checks.
// - It is initialized by an initial set of permission statuses passed down in
//   CommitNavigationParams.
// - Only registered permission elements can access this cache data.
// - When there are permission elements registered, the cache registers itself
//   for permission changes from the browser. And when there aren't any
//   permission elements, it unregisters itself from permission updates.
class CORE_EXPORT CachedPermissionStatus final
    : public GarbageCollected<CachedPermissionStatus>,
      public mojom::blink::PermissionObserver,
      public Supplement<LocalDOMWindow> {
 public:
  static const char kSupplementName[];

  // Returns the supplement, creating one as needed.
  static CachedPermissionStatus* From(LocalDOMWindow* window);

  using PermissionStatusMap =
      HashMap<mojom::blink::PermissionName, mojom::blink::PermissionStatus>;

  // Instances of this class receives notification from the cache, for example
  // receives any status changes notification. Notifications are only sent back
  // to the instances that have been registered first by calling RegisterClient.
  class Client : public GarbageCollectedMixin {
   public:
    virtual ~Client() = default;

    virtual void OnPermissionStatusChange(
        mojom::blink::PermissionName permission_name,
        mojom::blink::PermissionStatus status) {}

    virtual void OnPermissionStatusInitialized(
        PermissionStatusMap initilized_map) = 0;
  };

  explicit CachedPermissionStatus(LocalDOMWindow* local_dom_window);

  ~CachedPermissionStatus() override = default;

  void Trace(Visitor* visitor) const override;

  void SetPermissionStatusMap(PermissionStatusMap map) {
    permission_status_map_ = std::move(map);
  }

 private:
  friend class HTMLPermissionElement;
  friend class DocumentLoader;
  friend class CachedPermissionStatusTest;

  FRIEND_TEST_ALL_PREFIXES(CachedPermissionStatusTest, RegisterClient);
  FRIEND_TEST_ALL_PREFIXES(CachedPermissionStatusTest,
                           UnregisterClientRemoveObserver);
  FRIEND_TEST_ALL_PREFIXES(HTMLPemissionElementTest, SetTypeAfterInsertedInto);

  // Allow this object to keep track of the Client instances corresponding to
  // it.
  void RegisterClient(
      Client* client,
      const Vector<mojom::blink::PermissionDescriptorPtr>& permissions);
  void UnregisterClient(
      Client* client,
      const Vector<mojom::blink::PermissionDescriptorPtr>& permissions);

  void RegisterPermissionObserver(
      const mojom::blink::PermissionDescriptorPtr& descriptor,
      mojom::blink::PermissionStatus current_status);

  // mojom::blink::PermissionObserver override.
  void OnPermissionStatusChange(mojom::blink::PermissionStatus status) override;

  // Ensure there is a connection to the permission service and return it.
  mojom::blink::PermissionService* GetPermissionService();

  scoped_refptr<base::SingleThreadTaskRunner> GetTaskRunner();

  const PermissionStatusMap& GetPermissionStatusMap() const {
    return permission_status_map_;
  }

  using ClientMap = HeapHashMap<mojom::blink::PermissionName,
                                HeapHashSet<WeakMember<Client>>>;
  using PermissionToReceiversMap =
      HashMap<mojom::blink::PermissionName, mojo::ReceiverId>;
  using PermissionObserverReceiverSet =
      HeapMojoReceiverSet<mojom::blink::PermissionObserver,
                          CachedPermissionStatus,
                          HeapMojoWrapperMode::kWithContextObserver,
                          mojom::blink::PermissionName>;
  const ClientMap& GetClientsForTesting() const { return clients_; }

  const PermissionToReceiversMap& GetPermissionToReceiversMapForTesting()
      const {
    return permission_to_receivers_map_;
  }

  PermissionObserverReceiverSet& GetPermissionObserverReceiversForTesting() {
    return permission_observer_receivers_;
  }

  HeapMojoRemote<mojom::blink::PermissionService> permission_service_;

  // Holds all `PermissionObserver` receivers connected with remotes in browser
  // process.
  // This set uses `PermissionName` as context type. Once a receiver call is
  // triggered, we look into its name to determine which permission is changed.
  PermissionObserverReceiverSet permission_observer_receivers_;

  ClientMap clients_;

  // Track which `permission` are in the `permission_observer_receivers_` set so
  // they can be removed them when necessary.
  PermissionToReceiversMap permission_to_receivers_map_;

  PermissionStatusMap permission_status_map_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_CACHED_PERMISSION_STATUS_H_
