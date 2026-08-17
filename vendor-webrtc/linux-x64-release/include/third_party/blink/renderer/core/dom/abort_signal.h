// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_DOM_ABORT_SIGNAL_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_DOM_ABORT_SIGNAL_H_

#include "base/functional/callback_forward.h"
#include "third_party/blink/renderer/bindings/core/v8/script_value.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/dom/abort_signal_composition_type.h"
#include "third_party/blink/renderer/core/dom/events/event_target.h"
#include "third_party/blink/renderer/core/execution_context/execution_context_lifecycle_observer.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_linked_hash_set.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_vector.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "third_party/blink/renderer/platform/scheduler/public/post_cancellable_task.h"

namespace v8 {
class Isolate;
}  // namespace v8

namespace blink {

class AbortController;
class AbortSignalCompositionManager;
class AbortSignalRegistry;
class ExecutionContext;
class ScriptState;

// Implementation of https://dom.spec.whatwg.org/#interface-AbortSignal
class CORE_EXPORT AbortSignal : public EventTarget,
                                public ExecutionContextLifecycleObserver {
  DEFINE_WRAPPERTYPEINFO();

 public:
  enum class SignalType {
    // Associated with an AbortController.
    kController,
    // Created by AbortSignal.abort().
    kAborted,
    // Created by AbortSignal.timeout().
    kTimeout,
    // Created by AbortSignal.any() or used internally to combine signals.
    kComposite,
  };

  // The base class for "abort algorithm" defined at
  // https://dom.spec.whatwg.org/#abortsignal-abort-algorithms. This is
  // semantically equivalent to base::OnceClosure but is GarbageCollected.
  class Algorithm : public GarbageCollected<Algorithm> {
   public:
    virtual ~Algorithm() = default;

    // Called when the associated signal is aborted. This is called at most
    // once.
    virtual void Run() = 0;

    virtual void Trace(Visitor* visitor) const {}
  };

  // A garbage collected handle representing an abort algorithm. Abort
  // algorithms are no longer runnable after the handle is GCed. Algorithms can
  // be explcitly removed by passing the handle to `RemoveAlgorithm()`.
  class CORE_EXPORT AlgorithmHandle : public GarbageCollected<AlgorithmHandle> {
   public:
    AlgorithmHandle(Algorithm*, AbortSignal*);
    ~AlgorithmHandle();

    Algorithm* GetAlgorithm() { return algorithm_.Get(); }

    void Trace(Visitor* visitor) const;

   private:
    Member<Algorithm> algorithm_;
    // A reference to the signal the algorithm is associated with. This ensures
    // the associated signal stays alive while it has pending algorithms, which
    // is necessary for composite signals.
    Member<AbortSignal> signal_;
  };

  // Constructs a composite signal that is dependent on no other signals. This
  // is used to create non-abortable signal, e.g. fixed priority task signals
  // and default signals used in fetch.
  explicit AbortSignal(ExecutionContext*);

  // Constructs a new signal with the given `SignalType`.
  AbortSignal(ExecutionContext*, SignalType);

  // Constructs a composite signal. The signal will be aborted if any of
  // `source_signals` are aborted or become aborted.
  AbortSignal(ScriptState*,
              const HeapVector<Member<AbortSignal>>& source_signals);

  ~AbortSignal() override;

  // abort_signal.idl
  static AbortSignal* abort(ScriptState*);
  static AbortSignal* abort(ScriptState*, ScriptValue reason);
  static AbortSignal* any(ScriptState*,
                          HeapVector<Member<AbortSignal>> signals);
  static AbortSignal* timeout(ScriptState*, uint64_t milliseconds);
  ScriptValue reason(ScriptState*) const;
  bool aborted() const { return !abort_reason_.IsEmpty(); }
  void throwIfAborted(v8::Isolate*) const;
  DEFINE_ATTRIBUTE_EVENT_LISTENER(abort, kAbort)

  const AtomicString& InterfaceName() const override;
  ExecutionContext* GetExecutionContext() const override;

  // `ExecutionContextLifecycleObserver` overrides:
  void ContextDestroyed() override;

  // Internal API

  // The "add an algorithm" algorithm from the standard:
  // https://dom.spec.whatwg.org/#abortsignal-add for dependent features to call
  // to be notified when abort has been signalled.
  [[nodiscard]] AlgorithmHandle* AddAlgorithm(Algorithm* algorithm);

  // Same with above but with a base::OnceClosure. Use this only when you're
  // sure the objects attached to the callback don't form a reference cycle.
  [[nodiscard]] AlgorithmHandle* AddAlgorithm(base::OnceClosure algorithm);

  // The "remove an algorithm" algorithm from the standard:
  // https://dom.spec.whatwg.org/#abortsignal-remove.
  //
  // Removes the algorithm associated with the handle. Algorithms are no longer
  // runnable when their handles are GCed, but this can invoked directly if
  // needed, e.g. to not rely on GC timing.
  void RemoveAlgorithm(AlgorithmHandle*);

  class SignalAbortPassKey {
   private:
    SignalAbortPassKey() = default;

    friend class AbortController;
    friend class AbortSignal;
  };
  // The "To signal abort" algorithm from the standard:
  // https://dom.spec.whatwg.org/#abortsignal-add. Run all algorithms that were
  // added by AddAlgorithm(), in order of addition, then fire an "abort"
  // event. Does nothing if called more than once.
  void SignalAbort(ScriptState*, ScriptValue reason, SignalAbortPassKey);

  virtual bool IsTaskSignal() const { return false; }

  void Trace(Visitor*) const override;

  SignalType GetSignalType() const { return signal_type_; }

  bool IsCompositeSignal() const {
    return signal_type_ == AbortSignal::SignalType::kComposite;
  }

  // Returns true if this signal has not aborted and still might abort.
  bool CanAbort() const;

  // Returns the composition manager for this signal for the given type.
  // Subclasses are expected to override this to return the composition manager
  // associated with their type.
  virtual AbortSignalCompositionManager* GetCompositionManager(
      AbortSignalCompositionType);

  // Called by the composition manager when the signal is settled.
  virtual void OnSignalSettled(AbortSignalCompositionType);

  // Callback from `AbortController` during prefinalization, when the controller
  // can no longer emit events.
  virtual void DetachFromController();

 protected:
  // EventTarget callbacks.
  void AddedEventListener(const AtomicString& event_type,
                          RegisteredEventListener&) override;
  void RemovedEventListener(const AtomicString& event_type,
                            const RegisteredEventListener&) override;

  // Returns true iff the signal is settled for the given composition type.
  virtual bool IsSettledFor(AbortSignalCompositionType) const;

 private:
  void InitializeCompositeSignal(
      const HeapVector<Member<AbortSignal>>& source_signals);

  void AbortTimeoutFired(ScriptState*);

  enum class AddRemoveType { kAdded, kRemoved };
  void OnEventListenerAddedOrRemoved(const AtomicString& event_type,
                                     AddRemoveType);

  void SetAbortReason(ScriptState* script_state, ScriptValue reason);
  void RunAbortSteps();

  // https://dom.spec.whatwg.org/#abortsignal-abort-reason
  // There is one difference from the spec. The value is empty instead of
  // undefined when this signal is not aborted. This is because
  // ScriptValue::IsUndefined requires callers to enter a V8 context whereas
  // ScriptValue::IsEmpty does not.
  ScriptValue abort_reason_;
  HeapLinkedHashSet<WeakMember<AbortSignal::AlgorithmHandle>> abort_algorithms_;
  SignalType signal_type_;

  // This is set to a DependentSignalCompositionManager for composite signals or
  // a SourceSignalCompositionManager for non-composite signals. Null if
  // AbortSignalAny isn't enabled.
  Member<AbortSignalCompositionManager> composition_manager_;

  // Handle for the delayed task associated with `SignalType::kTimeout` signals.
  TaskHandle timout_task_handle_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_DOM_ABORT_SIGNAL_H_
