/*
 * Copyright (C) 2009 Google Inc. All rights reserved.
 * Copyright (C) 2010 Apple Inc. All rights reserved.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_WORKERS_SHARED_WORKER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_WORKERS_SHARED_WORKER_H_

#include "base/types/pass_key.h"
#include "third_party/blink/public/mojom/worker/shared_worker_connector.mojom-blink-forward.h"
#include "third_party/blink/renderer/bindings/core/v8/active_script_wrappable.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/workers/abstract_worker.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_remote.h"
#include "third_party/blink/renderer/platform/scheduler/public/frame_or_worker_scheduler.h"
#include "third_party/blink/renderer/platform/supplementable.h"

namespace blink {

class ExceptionState;
class StorageAccessHandle;
class V8UnionSharedWorkerOptionsOrString;

class CORE_EXPORT SharedWorker final
    : public AbstractWorker,
      public Supplementable<SharedWorker>,
      public ActiveScriptWrappable<SharedWorker> {
  DEFINE_WRAPPERTYPEINFO();

 public:
  // Creates and binds a shared worker for the given `context` using `url` and
  // `name_or_options` for the initialization. Any reportable errors are
  // recorded in `exception_state`.
  static SharedWorker* Create(
      ExecutionContext* context,
      const String& url,
      const V8UnionSharedWorkerOptionsOrString* name_or_options,
      ExceptionState& exception_state);

  // Like the above, but allows the use of a custom PublicURLManager and
  // SharedWorkerConnector. This is useful when trying to load blobs and create
  // workers in some non-default storage partition.
  static SharedWorker* Create(
      base::PassKey<StorageAccessHandle>,
      ExecutionContext* context,
      const String& url,
      const V8UnionSharedWorkerOptionsOrString* name_or_options,
      ExceptionState& exception_state,
      PublicURLManager* public_url_manager,
      const HeapMojoRemote<mojom::blink::SharedWorkerConnector>*
          connector_override);

  explicit SharedWorker(ExecutionContext*);
  ~SharedWorker() override;

  MessagePort* port() const { return port_.Get(); }

  const AtomicString& InterfaceName() const override;

  void SetIsBeingConnected(bool b) { is_being_connected_ = b; }
  bool IsBeingConnected() { return is_being_connected_; }

  bool HasPendingActivity() const final;

  void ContextLifecycleStateChanged(mojom::FrameLifecycleState state) override;
  void Trace(Visitor*) const override;

 private:
  static SharedWorker* CreateImpl(
      ExecutionContext* context,
      const String& url,
      const V8UnionSharedWorkerOptionsOrString* name_or_options,
      ExceptionState& exception_state,
      PublicURLManager* public_url_manager,
      const HeapMojoRemote<mojom::blink::SharedWorkerConnector>*
          connector_override);
  Member<MessagePort> port_;
  bool is_being_connected_;
  FrameOrWorkerScheduler::SchedulingAffectingFeatureHandle
      feature_handle_for_scheduler_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_WORKERS_SHARED_WORKER_H_
