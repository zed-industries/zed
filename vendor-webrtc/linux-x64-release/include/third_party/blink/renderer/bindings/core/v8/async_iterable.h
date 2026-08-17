// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_BINDINGS_CORE_V8_ASYNC_ITERABLE_H_
#define THIRD_PARTY_BLINK_RENDERER_BINDINGS_CORE_V8_ASYNC_ITERABLE_H_

#include "third_party/blink/renderer/bindings/core/v8/async_iterator_base.h"
#include "third_party/blink/renderer/bindings/core/v8/iterable.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise_resolver.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_deque.h"

namespace blink {

namespace bindings {

// Implements Web IDL 3.7.10.2. Asynchronous iterator prototype object
// https://webidl.spec.whatwg.org/#es-asynchronous-iterator-prototype-object
// especially about the part of the section independent from key type and
// value type.
class CORE_EXPORT AsyncIterationSourceBase
    : public AsyncIteratorBase::IterationSourceBase {
 public:
  ~AsyncIterationSourceBase() override = default;

  // https://webidl.spec.whatwg.org/#es-asynchronous-iterator-prototype-object
  ScriptPromise<IDLAny> Next(ScriptState*) final;
  ScriptPromise<IDLAny> Return(ScriptState*, ScriptValue) final;

  void Trace(Visitor* visitor) const override;

 protected:
  explicit AsyncIterationSourceBase(ScriptState* script_state, Kind kind);

  // This is called back as part of a get the next iteration result
  // algorithm [1], however this function is called back after a
  // ScriptPromiseResolverBase for the next entry has been created, so the
  // implementation of this callback doesn't need to create a promise resolver,
  // but needs to schedule a task to resolve/reject the promise.
  //
  // [1] https://webidl.spec.whatwg.org/#dfn-get-the-next-iteration-result
  virtual void GetNextIterationResult() = 0;

  // This is called back as part of an asynchronous iterator return
  // algorithm [1], however this function is called back after a
  // ScriptPromiseResolverBase for the return result has been created.
  //
  // [1] https://webidl.spec.whatwg.org/#asynchronous-iterator-return
  virtual void AsyncIteratorReturn(ScriptValue value) { NOTREACHED(); }

  bool HasPendingPromise() const {
    return pending_promise_resolver_ != nullptr;
  }

  // Returns the pending promise resolver by removing it from this instance.
  ScriptPromiseResolver<IDLAny>* TakePendingPromiseResolver() {
    DCHECK(pending_promise_resolver_);
    return pending_promise_resolver_.Release();
  }

  // Returns the special 'end of iteration' value.
  v8::Local<v8::Value> MakeEndOfIteration() const;

  // Returns the ScriptState where the async iterator is running.
  ScriptState* GetScriptState() const { return script_state_.Get(); }

 private:
  class CallableCommon;
  class RunNextStepsCallable;
  class RunFulfillStepsCallable;
  class RunRejectStepsCallable;
  class RunReturnStepsCallable;
  class RunReturnFulfillStepsCallable;

  ScriptPromise<IDLAny> RunNextSteps(ScriptState* script_state);
  ScriptValue RunFulfillSteps(ScriptState* script_state,
                              ScriptValue iter_result_object_or_undefined);
  ScriptValue RunRejectSteps(ScriptState* script_state, ScriptValue reason);

  ScriptPromise<IDLAny> RunReturnSteps(ScriptState* script_state,
                                       ScriptValue value);
  ScriptValue RunReturnFulfillSteps(ScriptState* script_state,
                                    ScriptValue value);

  Member<ScriptState> script_state_;
  Member<CallableCommon> on_settled_function_;
  Member<CallableCommon> on_fulfilled_function_;
  Member<CallableCommon> on_rejected_function_;

  // https://webidl.spec.whatwg.org/#dfn-default-asynchronous-iterator-object
  // its 'ongoing promise', which is a Promise or null,
  // its 'is finished', which is a boolean.
  MemberScriptPromise<IDLAny> ongoing_promise_;
  bool is_finished_ = false;

  // The pending promise resolver. This is basically corresponding to
  // 'nextPromise' in the spec, however, this must be resolved with either
  // a value returned by `MakeIterationResult` or the special
  // 'end of iteration' value returned by `MakeEndOfIteration`, otherwise
  // must be rejected. When used for 'returnStepsPromise', the fulfillment
  // value is ignored.
  // NOTE: `pending_promise_resolver_` should probably be typed according to
  // the iterable result type in the relevant IDL, but everything internal to
  // our implementation gives the resolver a v8::Value, which doesn't work
  // well with typing according to the IDL.
  Member<ScriptPromiseResolver<IDLAny>> pending_promise_resolver_;

  template <typename IDLKeyType,
            typename IDLValueType,
            typename KeyType,
            typename ValueType>
  friend class PairAsyncIterationSource;
  template <typename IDLValueType, typename ValueType>
  friend class ValueAsyncIterationSource;
};

template <typename IDLKeyType,
          typename IDLValueType,
          typename KeyType,
          typename ValueType>
class PairAsyncIterationSource : public AsyncIterationSourceBase {
 protected:
  explicit PairAsyncIterationSource(ScriptState* script_state, Kind kind)
      : AsyncIterationSourceBase(script_state, kind) {}

  // Returns an iteration result object to be used to resolve the pending
  // promise.
  v8::Local<v8::Value> MakeIterationResult(const KeyType& key,
                                           const ValueType& value) const {
    ScriptState* script_state = GetScriptState();
    switch (GetKind()) {
      case Kind::kKey: {
        v8::Local<v8::Value> v8_key =
            ToV8Traits<IDLKeyType>::ToV8(script_state, key);
        return ESCreateIterResultObject(script_state, false, v8_key);
      }
      case Kind::kValue: {
        v8::Local<v8::Value> v8_value =
            ToV8Traits<IDLValueType>::ToV8(script_state, value);
        return ESCreateIterResultObject(script_state, false, v8_value);
      }
      case Kind::kKeyValue: {
        v8::Local<v8::Value> v8_key =
            ToV8Traits<IDLKeyType>::ToV8(script_state, key);
        v8::Local<v8::Value> v8_value =
            ToV8Traits<IDLValueType>::ToV8(script_state, value);
        return ESCreateIterResultObject(script_state, false, v8_key, v8_value);
      }
    }
    NOTREACHED();
  }
};

template <typename IDLValueType, typename ValueType>
class ValueAsyncIterationSource : public AsyncIterationSourceBase {
 protected:
  explicit ValueAsyncIterationSource(ScriptState* script_state, Kind kind)
      : AsyncIterationSourceBase(script_state, kind) {}

  // Returns an iteration result object to be used to resolve the pending
  // promise.
  v8::Local<v8::Value> MakeIterationResult(const ValueType& value) const {
    ScriptState* script_state = GetScriptState();
    DCHECK_EQ(GetKind(), Kind::kValue);
    v8::Local<v8::Value> v8_value =
        ToV8Traits<IDLValueType>::ToV8(script_state, value);
    return ESCreateIterResultObject(script_state, false, v8_value);
  }
};

}  // namespace bindings

template <typename IDLInterface, typename... InitArgs>
class PairAsyncIterable {
 public:
  using AsyncIteratorType = AsyncIterator<IDLInterface>;
  // Check whether the v8_async_iterator_foo_bar.h is appropriately included.
  // Make sizeof require the complete definition of the class.
  static_assert(
      sizeof(AsyncIteratorType),  // Read the following for a compile error.
      "You need to include a generated header for AsyncIterator<IDLInterface> "
      "in order to inherit from PairAsyncIterable. "
      "For an IDL interface FooBar, #include "
      "\"third_party/blink/renderer/bindings/<component>/v8/"
      "v8_async_iterator_foo_bar.h\" is required.");

  using IDLKeyType = typename AsyncIteratorType::IDLKeyType;
  using IDLValueType = typename AsyncIteratorType::IDLValueType;
  using KeyType = typename AsyncIteratorType::KeyType;
  using ValueType = typename AsyncIteratorType::ValueType;
  using IterationSource = bindings::
      PairAsyncIterationSource<IDLKeyType, IDLValueType, KeyType, ValueType>;

  PairAsyncIterable() = default;
  ~PairAsyncIterable() = default;
  PairAsyncIterable(const PairAsyncIterable&) = delete;
  PairAsyncIterable& operator=(const PairAsyncIterable&) = delete;

  template <typename... ArgsAndExceptionState>
  AsyncIteratorType* keysForBinding(
      ScriptState* script_state,
      ArgsAndExceptionState&&... args_and_exception_state) {
    const auto kind = IterationSource::Kind::kKey;
    IterationSource* source = CreateIterationSource(
        script_state, kind,
        std::forward<ArgsAndExceptionState>(args_and_exception_state)...);
    if (!source) {
      return nullptr;
    }
    return MakeGarbageCollected<AsyncIteratorType>(source);
  }

  template <typename... ArgsAndExceptionState>
  AsyncIteratorType* valuesForBinding(
      ScriptState* script_state,
      ArgsAndExceptionState&&... args_and_exception_state) {
    const auto kind = IterationSource::Kind::kValue;
    IterationSource* source = CreateIterationSource(
        script_state, kind,
        std::forward<ArgsAndExceptionState>(args_and_exception_state)...);
    if (!source) {
      return nullptr;
    }
    return MakeGarbageCollected<AsyncIteratorType>(source);
  }

  template <typename... ArgsAndExceptionState>
  AsyncIteratorType* entriesForBinding(
      ScriptState* script_state,
      ArgsAndExceptionState&&... args_and_exception_state) {
    const auto kind = IterationSource::Kind::kKeyValue;
    IterationSource* source = CreateIterationSource(
        script_state, kind,
        std::forward<ArgsAndExceptionState>(args_and_exception_state)...);
    if (!source) {
      return nullptr;
    }
    return MakeGarbageCollected<AsyncIteratorType>(source);
  }

 private:
  virtual IterationSource* CreateIterationSource(
      ScriptState* script_state,
      IterationSource::Kind kind,
      InitArgs... args,
      ExceptionState& exception_state) = 0;
};

template <typename IDLInterface, typename... InitArgs>
class ValueAsyncIterable {
 public:
  using AsyncIteratorType = AsyncIterator<IDLInterface>;
  // Check whether the v8_async_iterator_foo_bar.h is appropriately included.
  // Make sizeof require the complete definition of the class.
  static_assert(
      sizeof(AsyncIteratorType),  // Read the following for a compile error.
      "You need to include a generated header for AsyncIterator<IDLInterface> "
      "in order to inherit from ValueAsyncIterable. "
      "For an IDL interface FooBar, #include "
      "\"third_party/blink/renderer/bindings/<component>/v8/"
      "v8_async_iterator_foo_bar.h\" is required.");

  using IDLValueType = typename AsyncIteratorType::IDLValueType;
  using ValueType = typename AsyncIteratorType::ValueType;
  using IterationSource =
      bindings::ValueAsyncIterationSource<IDLValueType, ValueType>;

  ValueAsyncIterable() = default;
  ~ValueAsyncIterable() = default;
  ValueAsyncIterable(const ValueAsyncIterable&) = delete;
  ValueAsyncIterable& operator=(const ValueAsyncIterable&) = delete;

  template <typename... ArgsAndExceptionState>
  AsyncIteratorType* valuesForBinding(
      ScriptState* script_state,
      ArgsAndExceptionState&&... args_and_exception_state) {
    const auto kind = IterationSource::Kind::kValue;
    IterationSource* source = CreateIterationSource(
        script_state, kind,
        std::forward<ArgsAndExceptionState>(args_and_exception_state)...);
    if (!source) {
      return nullptr;
    }
    return MakeGarbageCollected<AsyncIteratorType>(source);
  }

 private:
  virtual IterationSource* CreateIterationSource(
      ScriptState* script_state,
      IterationSource::Kind kind,
      InitArgs... args,
      ExceptionState& exception_state) = 0;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_BINDINGS_CORE_V8_ASYNC_ITERABLE_H_
