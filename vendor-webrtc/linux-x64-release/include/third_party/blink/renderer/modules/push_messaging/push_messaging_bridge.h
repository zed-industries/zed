// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_PUSH_MESSAGING_PUSH_MESSAGING_BRIDGE_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_PUSH_MESSAGING_PUSH_MESSAGING_BRIDGE_H_

#include "third_party/blink/public/mojom/permissions/permission.mojom-blink.h"
#include "third_party/blink/public/mojom/permissions/permission_status.mojom-blink.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise_resolver.h"
#include "third_party/blink/renderer/modules/service_worker/service_worker_registration.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_remote.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_wrapper_mode.h"
#include "third_party/blink/renderer/platform/supplementable.h"

namespace blink {

class PushSubscriptionOptionsInit;
class ScriptState;
class V8PermissionState;

// The bridge is responsible for establishing and maintaining the Mojo
// connection to the permission service. It's keyed on an active Service Worker
// Registration.
//
// TODO(peter): Use the PushMessaging Mojo service directly from here.
class PushMessagingBridge final : public GarbageCollected<PushMessagingBridge>,
                                  public Supplement<ServiceWorkerRegistration> {
 public:
  static const char kSupplementName[];

  static PushMessagingBridge* From(ServiceWorkerRegistration* registration);

  explicit PushMessagingBridge(ServiceWorkerRegistration& registration);

  PushMessagingBridge(const PushMessagingBridge&) = delete;
  PushMessagingBridge& operator=(const PushMessagingBridge&) = delete;

  ~PushMessagingBridge();

  // Asynchronously determines the permission state for the current origin.
  ScriptPromise<V8PermissionState> GetPermissionState(
      ScriptState* script_state,
      const PushSubscriptionOptionsInit* options);

  void Trace(Visitor*) const override;

 private:
  // Method to be invoked when the permission status has been retrieved from the
  // permission service. Will settle the given |resolver|.
  void DidGetPermissionState(ScriptPromiseResolver<V8PermissionState>* resolver,
                             mojom::blink::PermissionStatus status);

  HeapMojoRemote<mojom::blink::PermissionService> permission_service_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_PUSH_MESSAGING_PUSH_MESSAGING_BRIDGE_H_
