// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_PERMISSIONS_PERMISSION_STATUS_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_PERMISSIONS_PERMISSION_STATUS_H_

#include "third_party/blink/renderer/bindings/core/v8/active_script_wrappable.h"
#include "third_party/blink/renderer/core/dom/events/event_target.h"
#include "third_party/blink/renderer/core/execution_context/execution_context_lifecycle_state_observer.h"
#include "third_party/blink/renderer/modules/permissions/permission_status_listener.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_receiver.h"
#include "third_party/blink/renderer/platform/wtf/text/atomic_string.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

class ExecutionContext;
class ScriptPromiseResolverBase;
class V8PermissionState;

// Expose the status of a given permission type for the current
// ExecutionContext.
class PermissionStatus final : public EventTarget,
                               public ActiveScriptWrappable<PermissionStatus>,
                               public ExecutionContextLifecycleStateObserver,
                               public PermissionStatusListener::Observer {
  DEFINE_WRAPPERTYPEINFO();

  using MojoPermissionDescriptor = mojom::blink::PermissionDescriptorPtr;
  using MojoPermissionStatus = mojom::blink::PermissionStatus;

 public:
  static PermissionStatus* Take(PermissionStatusListener*,
                                ScriptPromiseResolverBase*);

  PermissionStatus(PermissionStatusListener*, ExecutionContext*);
  ~PermissionStatus() override;

  // EventTarget implementation.
  const AtomicString& InterfaceName() const override;
  ExecutionContext* GetExecutionContext() const override;
  // Called when an event listener has been successfully added.
  void AddedEventListener(const AtomicString& event_type,
                          RegisteredEventListener&) override;

  // Called when an event listener has been successfully removed.
  void RemovedEventListener(const AtomicString& event_type,
                            const RegisteredEventListener&) override;

  // ScriptWrappable implementation.
  bool HasPendingActivity() const final;

  // ExecutionContextLifecycleStateObserver implementation.
  void ContextLifecycleStateChanged(mojom::FrameLifecycleState) override;
  void ContextDestroyed() override {}

  // PermissionStatusListener::Observer
  void OnPermissionStatusChange(MojoPermissionStatus) override;

  V8PermissionState state() const;

  String name() const;

  DEFINE_ATTRIBUTE_EVENT_LISTENER(change, kChange)

  void Trace(Visitor*) const override;

 private:
  void StartListening();
  void StopListening();

  WeakMember<PermissionStatusListener> listener_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_PERMISSIONS_PERMISSION_STATUS_H_
