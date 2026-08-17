// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_SERVICE_WORKER_RESPOND_WITH_OBSERVER_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_SERVICE_WORKER_RESPOND_WITH_OBSERVER_H_

#include "base/time/time.h"
#include "third_party/blink/public/mojom/service_worker/service_worker_error_type.mojom-blink-forward.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise.h"
#include "third_party/blink/renderer/core/dom/events/event_target.h"
#include "third_party/blink/renderer/core/execution_context/execution_context_lifecycle_observer.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/modules/service_worker/wait_until_observer.h"
#include "third_party/blink/renderer/platform/bindings/exception_state.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"

namespace blink {

class ExecutionContext;
class ScriptState;
class ScriptValue;

// This is a base class to implement respondWith. The respondWith has the three
// types of results: fulfilled, rejected and not called. Derived classes for
// each event should implement the procedure of the three behaviors by
// overriding onResponseFulfilled, onResponseRejected and onNoResponse.
class MODULES_EXPORT RespondWithObserver
    : public GarbageCollected<RespondWithObserver>,
      public ExecutionContextClient {
 public:
  virtual ~RespondWithObserver() = default;

  template <typename IDLType, typename Derived>
  void RespondWith(ScriptState* script_state,
                   const ScriptPromise<IDLType>& script_promise,
                   ThenCallable<IDLType, Derived>* on_fulfill,
                   ExceptionState& exception_state) {
    if (!StartRespondWith(exception_state)) {
      return;
    }
    has_started_ = true;
    auto next_promise =
        script_promise.Then(script_state, on_fulfill,
                            MakeGarbageCollected<RespondWithReject>(this));
    // 3. `Add r to the extend lifetime promises.`
    // 4. `Increment the pending promises count by one.`
    // This is accomplised by WaitUntil().
    bool will_wait =
        observer_->WaitUntil(script_state, next_promise, exception_state);
    // If the WaitUntilObserver won't observe the response promise, the event
    // can end before the response result is reported back to the
    // ServiceWorkerContextClient, which it doesn't expect (e.g., for fetch
    // events, RespondToFetchEvent*() must be called before
    // DidHandleFetchEvent()). So WaitUntilObserver must observe the promise and
    // call our callbacks before it determines the event is done.
    DCHECK(will_wait);
  }

  void WillDispatchEvent();
  void DidDispatchEvent(ScriptState*, DispatchEventResult dispatch_result);
  // Called when the respondWith() promise was rejected.
  virtual void OnResponseRejected(mojom::ServiceWorkerResponseError) = 0;

  // Called when the event handler finished without calling respondWith().
  virtual void OnNoResponse(ScriptState*) = 0;

  void Trace(Visitor*) const override;

 protected:
  RespondWithObserver(ExecutionContext*, int event_id, WaitUntilObserver*);

  class RespondWithReject final
      : public ThenCallable<IDLAny, RespondWithReject, IDLPromise<IDLAny>> {
   public:
    explicit RespondWithReject(RespondWithObserver* observer)
        : observer_(observer) {}
    void Trace(Visitor* visitor) const final;
    ScriptPromise<IDLAny> React(ScriptState*, ScriptValue);

   private:
    Member<RespondWithObserver> observer_;
  };

  bool WaitUntil(ScriptState*,
                 const ScriptPromise<IDLUndefined>&,
                 ExceptionState&);

  const int event_id_;
  base::TimeTicks event_dispatch_time_;

 private:
  bool has_started_ = false;

  bool StartRespondWith(ExceptionState&);

  // RespondWith should ensure the ExtendableEvent is alive until the promise
  // passed to RespondWith is resolved. The lifecycle of the ExtendableEvent
  // is controlled by WaitUntilObserver, so not only
  // WaitUntilObserver::ThenFunction but RespondWith needs to have a strong
  // reference to the WaitUntilObserver.
  Member<WaitUntilObserver> observer_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_SERVICE_WORKER_RESPOND_WITH_OBSERVER_H_
