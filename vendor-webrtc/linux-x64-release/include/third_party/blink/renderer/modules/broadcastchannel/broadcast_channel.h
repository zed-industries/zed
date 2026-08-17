// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_BROADCASTCHANNEL_BROADCAST_CHANNEL_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_BROADCASTCHANNEL_BROADCAST_CHANNEL_H_

#include "third_party/blink/public/mojom/broadcastchannel/broadcast_channel.mojom-blink.h"
#include "third_party/blink/renderer/bindings/core/v8/active_script_wrappable.h"
#include "third_party/blink/renderer/core/dom/events/event_target.h"
#include "third_party/blink/renderer/core/execution_context/execution_context_lifecycle_observer.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/heap/prefinalizer.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_associated_receiver.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_associated_remote.h"
#include "third_party/blink/renderer/platform/scheduler/public/frame_or_worker_scheduler.h"

namespace blink {

class BroadcastChannelTester;
class ScriptValue;
class StorageAccessHandle;

class MODULES_EXPORT BroadcastChannel final
    : public EventTarget,
      public ActiveScriptWrappable<BroadcastChannel>,
      public ExecutionContextLifecycleObserver,
      public mojom::blink::BroadcastChannelClient {
  DEFINE_WRAPPERTYPEINFO();
  USING_PRE_FINALIZER(BroadcastChannel, Dispose);

 public:
  static BroadcastChannel* Create(ExecutionContext*,
                                  const String& name,
                                  ExceptionState&);

  BroadcastChannel(ExecutionContext*, const String& name);
  BroadcastChannel(base::PassKey<StorageAccessHandle>,
                   ExecutionContext* execution_context,
                   const String& name,
                   mojom::blink::BroadcastChannelProvider* provider);
  BroadcastChannel(
      base::PassKey<BroadcastChannelTester>,
      ExecutionContext*,
      const String& name,
      mojo::PendingAssociatedReceiver<mojom::blink::BroadcastChannelClient>
          receiver,
      mojo::PendingAssociatedRemote<mojom::blink::BroadcastChannelClient>
          remote);

  BroadcastChannel(const BroadcastChannel&) = delete;
  BroadcastChannel& operator=(const BroadcastChannel&) = delete;

  ~BroadcastChannel() override;
  void Dispose();

  // IDL
  String name() const { return name_; }
  void postMessage(const ScriptValue&, ExceptionState&);
  void close();
  DEFINE_ATTRIBUTE_EVENT_LISTENER(message, kMessage)
  DEFINE_ATTRIBUTE_EVENT_LISTENER(messageerror, kMessageerror)

  // EventTarget:
  const AtomicString& InterfaceName() const override;
  ExecutionContext* GetExecutionContext() const override {
    return ExecutionContextLifecycleObserver::GetExecutionContext();
  }

  // ScriptWrappable:
  bool HasPendingActivity() const override;

  // ExecutionContextLifecycleObserver:
  void ContextDestroyed() override;

  void Trace(Visitor*) const override;

  bool IsRemoteClientConnectedForTesting() const;

 private:
  BroadcastChannel(
      ExecutionContext*,
      const String& name,
      mojo::PendingAssociatedReceiver<mojom::blink::BroadcastChannelClient>
          receiver,
      mojo::PendingAssociatedRemote<mojom::blink::BroadcastChannelClient>
          remote);

  void SetupDisconnectHandlers();

  void PostMessageInternal(
      scoped_refptr<SerializedScriptValue> value,
      scoped_refptr<SecurityOrigin> sender_origin,
      const base::UnguessableToken sender_agent_cluster_id);

  // mojom::blink::BroadcastChannelClient:
  void OnMessage(BlinkCloneableMessage) override;

  // Called when the mojo binding disconnects.
  void OnError();

  // Close the mojo receivers and remotes.
  void CloseInternal();

  String name_;

  // Tracks whether this BroadcastChannel object has had close.
  bool explicitly_closed_ = false;

  // BroadcastChannelClient receiver for messages sent from the browser to
  // this channel and BroadcastChannelClient remote for messages sent from
  // this channel to the browser.
  HeapMojoAssociatedReceiver<mojom::blink::BroadcastChannelClient,
                             BroadcastChannel>
      receiver_;
  HeapMojoAssociatedRemote<mojom::blink::BroadcastChannelClient> remote_client_;

  // Notifies the scheduler that a broadcast channel is active.
  FrameOrWorkerScheduler::SchedulingAffectingFeatureHandle
      feature_handle_for_scheduler_;

  // When a BroadcastChannel is instantiated from a frame execution context,
  // `associated_remote_` holds the AssociatedRemote used to send
  // ConnectToChannel messages (with ordering preserved) to the
  // RenderFrameHostImpl associated with this frame. When a BroadcastChannel is
  // instantiated from a worker execution context, this member is not used.
  HeapMojoAssociatedRemote<mojom::blink::BroadcastChannelProvider>
      associated_remote_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_BROADCASTCHANNEL_BROADCAST_CHANNEL_H_
