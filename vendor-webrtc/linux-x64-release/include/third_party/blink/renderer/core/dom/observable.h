// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_DOM_OBSERVABLE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_DOM_OBSERVABLE_H_

#include <optional>

#include "base/types/pass_key.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/execution_context/execution_context_lifecycle_observer.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"

namespace blink {

class ExecutionContext;
class ObservableInternalObserver;
class ScriptState;
class Subscriber;
class SubscribeOptions;
class V8CatchCallback;
class V8Mapper;
class V8Predicate;
class V8Reducer;
class V8SubscribeCallback;
class V8UnionObservableInspectorOrObserverCallback;
class V8UnionObserverOrObserverCallback;
class V8Visitor;
class V8VoidFunction;

// Implementation of the DOM `Observable` API. See
// https://github.com/WICG/observable and
// https://docs.google.com/document/d/1NEobxgiQO-fTSocxJBqcOOOVZRmXcTFg9Iqrhebb7bg/edit.
class CORE_EXPORT Observable final : public ScriptWrappable,
                                     public ExecutionContextClient {
  DEFINE_WRAPPERTYPEINFO();

 public:
  // This delegate is an internal (non-web-exposed) version of
  // `V8SubscribeCallback` for `Observer`s that are created natively by the web
  // platform. `OnSubscribe()` owns the passed-in `Subscriber` so that the
  // delegate, like an actual JS `V8SubscribeCallback`, can forward events to
  // the underlying observable subscriber.
  class SubscribeDelegate : public GarbageCollected<SubscribeDelegate> {
   public:
    virtual ~SubscribeDelegate() = default;
    virtual void OnSubscribe(Subscriber*, ScriptState*) = 0;
    virtual void Trace(Visitor* visitor) const {}
  };

  // Called by v8 bindings.
  static Observable* Create(ScriptState*, V8SubscribeCallback*);

  Observable(ExecutionContext*, V8SubscribeCallback*);
  Observable(ExecutionContext*, SubscribeDelegate*);

  // API methods:
  void subscribe(ScriptState*,
                 V8UnionObserverOrObserverCallback*,
                 SubscribeOptions*);

  static Observable* from(ScriptState* script_state,
                          ScriptValue value,
                          ExceptionState& exception_state);

  // Observable-returning operators. See
  // https://wicg.github.io/observable/#observable-returning-operators.
  Observable* takeUntil(ScriptState*, Observable*);
  Observable* map(ScriptState*, V8Mapper*);
  Observable* filter(ScriptState*, V8Predicate*);
  Observable* take(ScriptState*, uint64_t);
  Observable* drop(ScriptState*, uint64_t);
  // `flatMap()` and `switchMap()` do not actually throw exceptions to script,
  // but we need access to the `exception_state` to determine if future calls to
  // `from()` succeeded or failed. In the failure case, we clear the exception
  // from the stack and report it to the relevant `Subscriber`.
  Observable* flatMap(ScriptState*, V8Mapper*, ExceptionState& exception_state);
  Observable* switchMap(ScriptState*,
                        V8Mapper*,
                        ExceptionState& exception_state);
  Observable* inspect(ScriptState*,
                      V8UnionObservableInspectorOrObserverCallback*);
  // See documentation above `flatMap()` and `switchMap()` for why this method
  // accepts an `exception_state`, even though it does not throw an exception
  // itself.
  //
  // This is the implementation for the `Observable#catch()` method, although
  // internally in C++ it has to be named something other than `catch()` due to
  // `catch` being a language keyword.
  Observable* catchImpl(ScriptState*, V8CatchCallback*, ExceptionState&);
  Observable* finally(ScriptState*, V8VoidFunction*);

  // Promise-returning operators. See
  // https://wicg.github.io/observable/#promise-returning-operators.
  ScriptPromise<IDLSequence<IDLAny>> toArray(ScriptState*, SubscribeOptions*);
  ScriptPromise<IDLUndefined> forEach(ScriptState*,
                                      V8Visitor*,
                                      SubscribeOptions*);
  ScriptPromise<IDLAny> first(ScriptState*, SubscribeOptions*);
  ScriptPromise<IDLAny> last(ScriptState*, SubscribeOptions*);
  ScriptPromise<IDLBoolean> some(ScriptState*, V8Predicate*, SubscribeOptions*);
  ScriptPromise<IDLBoolean> every(ScriptState*,
                                  V8Predicate*,
                                  SubscribeOptions*);
  ScriptPromise<IDLAny> find(ScriptState*, V8Predicate*, SubscribeOptions*);
  ScriptPromise<IDLAny> reduce(ScriptState*, V8Reducer*);
  ScriptPromise<IDLAny> reduce(ScriptState*,
                               V8Reducer*,
                               v8::Local<v8::Value>,
                               SubscribeOptions*);

  void Trace(Visitor*) const override;

  // The `subscribe()` API is used when web content subscribes to an Observable
  // with a `V8UnionObserverOrObserverCallback`, whereas this API is used when
  // native code subscribes to an `Observable` with a native internal observer.
  // For consistency with the web-exposed `subscribe()` method, the
  // `ScriptState` does not have to be associated with a valid context.
  void SubscribeWithNativeObserver(ScriptState*,
                                   ObservableInternalObserver*,
                                   SubscribeOptions*);

 private:
  // Used by both overloads of `reduce()`.
  ScriptPromise<IDLAny> ReduceInternal(ScriptState*,
                                       V8Reducer*,
                                       std::optional<ScriptValue>,
                                       SubscribeOptions*);

  // The `ScriptState` argument does not need to be associated with a valid
  // context (this method early-returns in that case).
  void SubscribeInternal(ScriptState*,
                         V8UnionObserverOrObserverCallback*,
                         ObservableInternalObserver*,
                         SubscribeOptions*);

  // Exactly one of `subscribe_callback_` and `subscribe_delegate_` must be
  // non-null. `subscribe_callback_` is non-null when `this` is created from
  // script, and the subscribe callback is a JS-provided callback function,
  // whereas `subscribe_delegate_` is used for `Observable`s created internally
  // in C++, where the subscription steps are native steps.
  //
  // `subscribe_callback_` gets called when the `subscribe` method is invoked.
  // When run, is run, errors are caught and "reported":
  // https://html.spec.whatwg.org/C#report-the-exception.
  const Member<V8SubscribeCallback> subscribe_callback_;
  const Member<SubscribeDelegate> subscribe_delegate_;

  // The most recent `Subscriber` associated with `this`. It is set in
  // `SubscribeInternal`, and used to register all subsequent subscriptions
  // until it becomes inactive or garbage collected. Once inactive or garbage
  // collected, `this` no longer has an "active" subscription, and this member
  // will be set anew in subsequent invocations of `SubscribeInternal()`.
  WeakMember<Subscriber> weak_subscriber_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_DOM_OBSERVABLE_H_
