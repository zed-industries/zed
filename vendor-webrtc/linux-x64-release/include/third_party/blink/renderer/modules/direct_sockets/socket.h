// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_DIRECT_SOCKETS_SOCKET_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_DIRECT_SOCKETS_SOCKET_H_

#include "third_party/blink/public/mojom/direct_sockets/direct_sockets.mojom-blink.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise_property.h"
#include "third_party/blink/renderer/core/dom/dom_exception.h"
#include "third_party/blink/renderer/core/execution_context/execution_context_lifecycle_observer.h"
#include "third_party/blink/renderer/core/execution_context/execution_context_lifecycle_state_observer.h"
#include "third_party/blink/renderer/modules/direct_sockets/stream_wrapper.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/bindings/exception_state.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/bindings/trace_wrapper_v8_reference.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_remote.h"
#include "third_party/blink/renderer/platform/scheduler/public/frame_or_worker_scheduler.h"

namespace blink {

class ExceptionState;

// Base class for TCP and UDP sockets.
class MODULES_EXPORT Socket : public ExecutionContextLifecycleStateObserver {
 public:
  // IDL definitions
  virtual ScriptPromise<IDLUndefined> closed(ScriptState*) const;
  virtual ScriptPromise<IDLUndefined> close(ScriptState*, ExceptionState&) = 0;

 public:
  enum class State { kOpening, kOpen, kClosed, kAborted };

  explicit Socket(ScriptState*);
  ~Socket() override;

  Socket(const Socket&) = delete;
  Socket& operator=(const Socket&) = delete;

  static bool CheckContextAndPermissions(ScriptState*, ExceptionState&);
  static DOMException* CreateDOMExceptionFromNetErrorCode(int32_t net_error);

  void Trace(Visitor*) const override;

 protected:
  ScriptState* GetScriptState() const { return script_state_.Get(); }

  ScriptPromiseProperty<IDLUndefined, IDLAny>& GetClosedProperty() const {
    DCHECK(state_ == State::kOpening || state_ == State::kOpen);
    return *closed_;
  }

  blink::mojom::blink::DirectSocketsService* GetServiceRemote() const {
    return service_.get();
  }

  State GetState() const { return state_; }
  virtual void SetState(State state) { state_ = state; }

  // Resets |service_| and |feature_handle_for_scheduler_|.
  void ResetServiceAndFeatureHandle();

 private:
  // Invoked if |service_| goes down.
  virtual void OnServiceConnectionError() {}

  State state_ = State::kOpening;

  const Member<ScriptState> script_state_;

  HeapMojoRemote<blink::mojom::blink::DirectSocketsService> service_;

  FrameOrWorkerScheduler::SchedulingAffectingFeatureHandle
      feature_handle_for_scheduler_;

  Member<ScriptPromiseProperty<IDLUndefined, IDLAny>> closed_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_DIRECT_SOCKETS_SOCKET_H_
