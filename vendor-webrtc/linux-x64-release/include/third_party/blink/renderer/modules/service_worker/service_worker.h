/*
 * Copyright (C) 2013 Google Inc. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are
 * met:
 *
 *     * Redistributions of source code must retain the above copyright
 * notice, this list of conditions and the following disclaimer.
 *     * Redistributions in binary form must reproduce the above
 * copyright notice, this list of conditions and the following disclaimer
 * in the documentation and/or other materials provided with the
 * distribution.
 *     * Neither the name of Google Inc. nor the names of its
 * contributors may be used to endorse or promote products derived from
 * this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 * "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 * LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
 * A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
 * OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 * SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
 * LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
 * DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
 * THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_SERVICE_WORKER_SERVICE_WORKER_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_SERVICE_WORKER_SERVICE_WORKER_H_

#include <memory>
#include "base/memory/scoped_refptr.h"
#include "third_party/blink/public/mojom/service_worker/service_worker_object.mojom-blink.h"
#include "third_party/blink/public/platform/modules/service_worker/web_service_worker_object_info.h"
#include "third_party/blink/renderer/bindings/core/v8/active_script_wrappable.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise.h"
#include "third_party/blink/renderer/bindings/core/v8/serialization/serialized_script_value.h"
#include "third_party/blink/renderer/core/workers/abstract_worker.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_associated_receiver.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_associated_remote.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_wrapper_mode.h"

namespace blink {

class PostMessageOptions;
class ScriptObject;
class ScriptState;
class V8ServiceWorkerState;

class MODULES_EXPORT ServiceWorker final
    : public AbstractWorker,
      public ActiveScriptWrappable<ServiceWorker>,
      public mojom::blink::ServiceWorkerObject {
  DEFINE_WRAPPERTYPEINFO();

 public:
  static ServiceWorker* From(ExecutionContext*,
                             mojom::blink::ServiceWorkerObjectInfoPtr);
  // TODO(crbug.com/879019): Eventually we'll remove WebServiceWorkerObjectInfo
  // and use the above From() everywhere instead of this one.
  static ServiceWorker* From(ExecutionContext*, WebServiceWorkerObjectInfo);

  static ServiceWorker* Create(ExecutionContext* context,
                               WebServiceWorkerObjectInfo info) {
    ServiceWorker* worker =
        MakeGarbageCollected<ServiceWorker>(context, std::move(info));
    worker->UpdateStateIfNeeded();
    return worker;
  }

  ServiceWorker(ExecutionContext*, WebServiceWorkerObjectInfo);
  ~ServiceWorker() override;
  void Trace(Visitor*) const override;

  void postMessage(ScriptState*,
                   const ScriptValue& message,
                   HeapVector<ScriptObject> transfer,
                   ExceptionState&);
  void postMessage(ScriptState*,
                   const ScriptValue& message,
                   const PostMessageOptions*,
                   ExceptionState&);

  String scriptURL() const;
  V8ServiceWorkerState state() const;
  DEFINE_ATTRIBUTE_EVENT_LISTENER(statechange, kStatechange)

  ServiceWorker* ToServiceWorker() override { return this; }

  // ScriptWrappable overrides.
  bool HasPendingActivity() const final;

  // AbstractWorker overrides.
  const AtomicString& InterfaceName() const override;

  // Implements mojom::blink::ServiceWorkerObject.
  void StateChanged(mojom::blink::ServiceWorkerState new_state) override;

  ScriptPromise<IDLUndefined> InternalsTerminate(ScriptState*);

 private:
  // ExecutionContextLifecycleStateObserver overrides.
  void ContextLifecycleStateChanged(mojom::FrameLifecycleState state) override;
  void ContextDestroyed() override;

  void PostMessageInternal(BlinkTransferableMessage message);

  bool was_stopped_ = false;
  const KURL url_;
  mojom::blink::ServiceWorkerState state_;
  // Both |host_| and |receiver_| are associated with
  // content.mojom.ServiceWorkerContainer interface for a Document, and
  // content.mojom.ServiceWorker interface for a ServiceWorkerGlobalScope.
  //
  // |host_| keeps the Mojo connection to the
  // browser-side ServiceWorkerObjectHost, whose lifetime is bound
  // to |host_| via the Mojo connection.
  HeapMojoAssociatedRemote<mojom::blink::ServiceWorkerObjectHost> host_;
  // Receives messages from the content::ServiceWorkerObjectHost in the browser
  // process.
  HeapMojoAssociatedReceiver<mojom::blink::ServiceWorkerObject, ServiceWorker>
      receiver_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_SERVICE_WORKER_SERVICE_WORKER_H_
