// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_PERMISSIONS_PERMISSION_STATUS_LISTENER_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_PERMISSIONS_PERMISSION_STATUS_LISTENER_H_

#include "base/memory/weak_ptr.h"
#include "third_party/blink/public/mojom/permissions/permission.mojom-blink.h"
#include "third_party/blink/renderer/core/execution_context/execution_context_lifecycle_observer.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_set.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_receiver.h"

namespace blink {

class ExecutionContext;
class Permissions;
class V8PermissionState;

class PermissionStatusListener final
    : public GarbageCollected<PermissionStatusListener>,
      public ExecutionContextClient,
      public mojom::blink::PermissionObserver {
  using MojoPermissionDescriptor = mojom::blink::PermissionDescriptorPtr;
  using MojoPermissionStatus = mojom::blink::PermissionStatus;

 public:
  class Observer : public GarbageCollectedMixin {
   public:
    virtual ~Observer() = default;

    virtual void OnPermissionStatusChange(MojoPermissionStatus) = 0;

    void Trace(Visitor* visitor) const override {}
  };

  static PermissionStatusListener* Create(Permissions&,
                                          ExecutionContext*,
                                          MojoPermissionStatus,
                                          MojoPermissionDescriptor);

  PermissionStatusListener(Permissions&,
                           ExecutionContext*,
                           MojoPermissionStatus,
                           MojoPermissionDescriptor);
  ~PermissionStatusListener() override;

  PermissionStatusListener(const PermissionStatusListener&) = delete;
  PermissionStatusListener& operator=(const PermissionStatusListener&) = delete;

  void AddObserver(Observer* observer);
  void RemoveObserver(Observer* observer);
  void AddedEventListener(const AtomicString& event_type);
  void RemovedEventListener(const AtomicString& event_type);

  bool HasPendingActivity();
  void SetStatus(MojoPermissionStatus status) { status_ = status; }

  V8PermissionState state() const;
  String name() const;

  void Trace(Visitor*) const override;

 private:
  void StartListening();
  void StopListening();
  void NotifyEventListener(const AtomicString& event_type, bool is_added);

  // mojom::blink::PermissionObserver
  void OnPermissionStatusChange(MojoPermissionStatus) override;

  MojoPermissionStatus status_;
  MojoPermissionDescriptor descriptor_;
  HeapHashSet<WeakMember<Observer>> observers_;
  HeapMojoReceiver<mojom::blink::PermissionObserver, PermissionStatusListener>
      receiver_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_PERMISSIONS_PERMISSION_STATUS_LISTENER_H_
