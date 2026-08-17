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
 * THIS SOFTWARE IS PROVIDED BY APPLE COMPUTER, INC. ``AS IS'' AND ANY
 * EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR
 * PURPOSE ARE DISCLAIMED.  IN NO EVENT SHALL APPLE COMPUTER, INC. OR
 * CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
 * EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
 * PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
 * PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY
 * OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 *
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_MESSAGING_MESSAGE_PORT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_MESSAGING_MESSAGE_PORT_H_

#include <memory>
#include <vector>

#include "base/memory/scoped_refptr.h"
#include "base/task/single_thread_task_runner.h"
#include "base/unguessable_token.h"
#include "mojo/public/cpp/bindings/connector.h"
#include "mojo/public/cpp/bindings/message.h"
#include "third_party/blink/public/common/messaging/message_port_channel.h"
#include "third_party/blink/public/common/messaging/message_port_descriptor.h"
#include "third_party/blink/public/common/scheduler/task_attribution_id.h"
#include "third_party/blink/renderer/bindings/core/v8/active_script_wrappable.h"
#include "third_party/blink/renderer/bindings/core/v8/serialization/serialized_script_value.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/dom/events/event_listener.h"
#include "third_party/blink/renderer/core/dom/events/event_target.h"
#include "third_party/blink/renderer/core/execution_context/execution_context_lifecycle_observer.h"
#include "third_party/blink/renderer/platform/heap/prefinalizer.h"
#include "third_party/blink/renderer/platform/scheduler/public/task_attribution_info.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {

struct BlinkTransferableMessage;
class ExceptionState;
class ExecutionContext;
class PostMessageOptions;
class ScriptObject;
class ScriptState;
class ScriptValue;

class CORE_EXPORT MessagePort : public EventTarget,
                                public mojo::MessageReceiver,
                                public ActiveScriptWrappable<MessagePort>,
                                public ExecutionContextLifecycleObserver {
  DEFINE_WRAPPERTYPEINFO();
  USING_PRE_FINALIZER(MessagePort, Dispose);

 public:
  explicit MessagePort(ExecutionContext&);
  ~MessagePort() override = default;
  void Dispose();

  void postMessage(ScriptState*,
                   const ScriptValue& message,
                   HeapVector<ScriptObject> transfer,
                   ExceptionState&);
  void postMessage(ScriptState*,
                   const ScriptValue& message,
                   const PostMessageOptions*,
                   ExceptionState&);

  void start();
  void close();

  void OnConnectionError();
  void Entangle(MessagePortDescriptor, MessagePort*);
  void Entangle(MessagePortChannel);
  MessagePortChannel Disentangle();

  // Returns an empty array if there is an exception, or if the passed array is
  // empty.
  static Vector<MessagePortChannel> DisentanglePorts(ExecutionContext*,
                                                     const MessagePortArray&,
                                                     ExceptionState&);

  // Returns an empty array if the passed array is empty.
  static GCedMessagePortArray* EntanglePorts(ExecutionContext&,
                                             Vector<MessagePortChannel>);
  static GCedMessagePortArray* EntanglePorts(ExecutionContext&,
                                             std::vector<MessagePortChannel>);

  bool Started() const { return started_; }

  const AtomicString& InterfaceName() const override;
  ExecutionContext* GetExecutionContext() const override {
    return ExecutionContextLifecycleObserver::GetExecutionContext();
  }
  MessagePort* ToMessagePort() override { return this; }

  // ScriptWrappable implementation.
  bool HasPendingActivity() const final;

  // ExecutionContextLifecycleObserver implementation.
  void ContextDestroyed() override { close(); }

  DEFINE_ATTRIBUTE_EVENT_LISTENER(close, kClose)

  void setOnmessage(EventListener* listener) {
    SetAttributeEventListener(event_type_names::kMessage, listener);
    start();
  }
  EventListener* onmessage() {
    return GetAttributeEventListener(event_type_names::kMessage);
  }

  void setOnmessageerror(EventListener* listener) {
    SetAttributeEventListener(event_type_names::kMessageerror, listener);
    start();
  }
  EventListener* onmessageerror() {
    return GetAttributeEventListener(event_type_names::kMessageerror);
  }

  // A port starts out its life entangled, and remains entangled until it is
  // closed or is cloned.
  bool IsEntangled() const { return !closed_ && !IsNeutered(); }

  // A port gets neutered when it is transferred to a new owner via
  // postMessage().
  bool IsNeutered() const { return !connector_ || !connector_->is_valid(); }

  // For testing only: allows inspection of the entangled channel.
  ::MojoHandle EntangledHandleForTesting() const;

  void Trace(Visitor*) const override;

 private:
  class PostMessageTaskContainer
      : public GarbageCollected<PostMessageTaskContainer> {
   public:
    void AddPostMessageTask(scheduler::TaskAttributionInfo* task);

    scheduler::TaskAttributionInfo* GetAndDecrementPostMessageTask(
        std::optional<scheduler::TaskAttributionId> id);

    void Trace(Visitor* visitor) const { visitor->Trace(post_message_tasks_); }

   private:
    class PostMessageTask : public GarbageCollected<PostMessageTask> {
     public:
      PostMessageTask() = default;
      explicit PostMessageTask(scheduler::TaskAttributionInfo* task)
          : task_(task) {}
      scheduler::TaskAttributionInfo* GetTask() { return task_.Get(); }
      size_t DecrementAndReturnCounter() { return --counter_; }
      void IncrementCounter() { ++counter_; }
      void Trace(Visitor* visitor) const { visitor->Trace(task_); }

     private:
      Member<scheduler::TaskAttributionInfo> task_;
      size_t counter_ = 1;
    };

    // A container of pending PostMessage tasks.
    HeapHashMap<scheduler::TaskAttributionIdType, Member<PostMessageTask>>
        post_message_tasks_;
  };

  // mojo::MessageReceiver implementation.
  bool Accept(mojo::Message*) override;
  Event* CreateMessageEvent(BlinkTransferableMessage& message);
  void OnEntangledPortDisconnected();

  std::unique_ptr<mojo::Connector> connector_;

  bool started_ = false;
  bool closed_;

  scoped_refptr<base::SingleThreadTaskRunner> task_runner_;

  // The internal port owned by this class. The handle itself is moved into the
  // |connector_| while entangled.
  MessagePortDescriptor port_descriptor_;

  // The entangled port. Only set on initial entanglement, and gets unset as
  // soon as the ports are disentangled.
  WeakMember<MessagePort> initially_entangled_port_;

  Member<PostMessageTaskContainer> post_message_task_container_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_MESSAGING_MESSAGE_PORT_H_
