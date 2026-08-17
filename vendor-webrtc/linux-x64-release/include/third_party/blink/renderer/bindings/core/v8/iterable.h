// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_BINDINGS_CORE_V8_ITERABLE_H_
#define THIRD_PARTY_BLINK_RENDERER_BINDINGS_CORE_V8_ITERABLE_H_

#include "third_party/blink/renderer/bindings/core/v8/to_v8_traits.h"
#include "third_party/blink/renderer/bindings/core/v8/v8_for_each_iterator_callback.h"
#include "third_party/blink/renderer/platform/bindings/exception_state.h"
#include "third_party/blink/renderer/platform/bindings/sync_iterator_base.h"

namespace blink {

namespace bindings {

class EnumerationBase;

namespace {

// Helper class to construct a type T without an argument. Note that IDL
// enumeration types are not default-constructible on purpose.
template <typename T, typename unused = void>
class IDLTypeDefaultConstructible {
  STACK_ALLOCATED();

 public:
  T content;
};

template <typename T>
class IDLTypeDefaultConstructible<
    T,
    std::enable_if_t<std::is_base_of_v<EnumerationBase, T>>> {
  STACK_ALLOCATED();

 public:
  T content{static_cast<typename T::Enum>(0)};
};

}  // namespace

// https://tc39.es/ecma262/#sec-createiterresultobject
CORE_EXPORT v8::Local<v8::Object> ESCreateIterResultObject(
    ScriptState* script_state,
    bool done,
    v8::Local<v8::Value> value);
CORE_EXPORT v8::Local<v8::Object> ESCreateIterResultObject(
    ScriptState* script_state,
    bool done,
    v8::Local<v8::Value> item1,
    v8::Local<v8::Value> item2);

template <typename IDLKeyType,
          typename IDLValueType,
          typename KeyType,
          typename ValueType>
class PairSyncIterationSource : public SyncIteratorBase::IterationSourceBase {
 public:
  v8::Local<v8::Object> Next(ScriptState* script_state,
                             SyncIteratorBase::Kind kind,
                             ExceptionState& exception_state) override {
    IDLTypeDefaultConstructible<KeyType> key;
    IDLTypeDefaultConstructible<ValueType> value;
    if (!FetchNextItem(script_state, key.content, value.content,
                       exception_state)) {
      if (exception_state.HadException())
        return {};
      return ESCreateIterResultObject(
          script_state, true, v8::Undefined(script_state->GetIsolate()));
    }

    switch (kind) {
      case SyncIteratorBase::Kind::kKey: {
        v8::Local<v8::Value> v8_key =
            ToV8Traits<IDLKeyType>::ToV8(script_state, key.content);
        return ESCreateIterResultObject(script_state, false, v8_key);
      }
      case SyncIteratorBase::Kind::kValue: {
        v8::Local<v8::Value> v8_value =
            ToV8Traits<IDLValueType>::ToV8(script_state, value.content);
        return ESCreateIterResultObject(script_state, false, v8_value);
      }
      case SyncIteratorBase::Kind::kKeyValue: {
        v8::Local<v8::Value> v8_key =
            ToV8Traits<IDLKeyType>::ToV8(script_state, key.content);
        v8::Local<v8::Value> v8_value =
            ToV8Traits<IDLValueType>::ToV8(script_state, value.content);
        return ESCreateIterResultObject(script_state, false, v8_key, v8_value);
      }
    }

    NOTREACHED();
  }

  void ForEach(ScriptState* script_state,
               const ScriptValue& this_value,
               V8ForEachIteratorCallback* callback,
               const ScriptValue& this_arg,
               ExceptionState& exception_state) {
    TryRethrowScope rethrow_scope(script_state->GetIsolate(), exception_state);

    v8::Local<v8::Value> v8_callback_this_value = this_arg.V8Value();
    IDLTypeDefaultConstructible<KeyType> key;
    IDLTypeDefaultConstructible<ValueType> value;
    v8::Local<v8::Value> v8_key;
    v8::Local<v8::Value> v8_value;

    while (true) {
      if (!FetchNextItem(script_state, key.content, value.content,
                         exception_state))
        return;

      v8_key = ToV8Traits<IDLKeyType>::ToV8(script_state, key.content);
      v8_value = ToV8Traits<IDLValueType>::ToV8(script_state, value.content);

      if (callback
              ->Invoke(v8_callback_this_value,
                       ScriptValue(script_state->GetIsolate(), v8_value),
                       ScriptValue(script_state->GetIsolate(), v8_key),
                       this_value)
              .IsNothing()) {
        return;
      }
    }
  }

 private:
  virtual bool FetchNextItem(ScriptState* script_state,
                             KeyType& key,
                             ValueType& value,
                             ExceptionState& exception_state) = 0;
};

template <typename IDLValueType, typename ValueType>
class ValueSyncIterationSource : public SyncIteratorBase::IterationSourceBase {
 public:
  v8::Local<v8::Object> Next(ScriptState* script_state,
                             SyncIteratorBase::Kind kind,
                             ExceptionState& exception_state) override {
    IDLTypeDefaultConstructible<ValueType> value;
    if (!FetchNextItem(script_state, value.content, exception_state)) {
      if (exception_state.HadException())
        return {};
      return ESCreateIterResultObject(
          script_state, true, v8::Undefined(script_state->GetIsolate()));
    }

    v8::Local<v8::Value> v8_value =
        ToV8Traits<IDLValueType>::ToV8(script_state, value.content);

    switch (kind) {
      case SyncIteratorBase::Kind::kKey:
      case SyncIteratorBase::Kind::kValue:
        return ESCreateIterResultObject(script_state, false, v8_value);
      case SyncIteratorBase::Kind::kKeyValue:
        return ESCreateIterResultObject(script_state, false, v8_value,
                                        v8_value);
    }

    NOTREACHED();
  }

  void ForEach(ScriptState* script_state,
               const ScriptValue& this_value,
               V8ForEachIteratorCallback* callback,
               const ScriptValue& this_arg,
               ExceptionState& exception_state) {
    TryRethrowScope rethrow_scope(script_state->GetIsolate(), exception_state);

    v8::Local<v8::Value> v8_callback_this_value = this_arg.V8Value();
    IDLTypeDefaultConstructible<ValueType> value;
    v8::Local<v8::Value> v8_value;

    while (true) {
      if (!FetchNextItem(script_state, value.content, exception_state))
        return;

      v8_value = ToV8Traits<IDLValueType>::ToV8(script_state, value.content);
      ScriptValue script_value(script_state->GetIsolate(), v8_value);

      if (callback
              ->Invoke(v8_callback_this_value, script_value, script_value,
                       this_value)
              .IsNothing()) {
        return;
      }
    }
  }

 private:
  virtual bool FetchNextItem(ScriptState* script_state,
                             ValueType& value,
                             ExceptionState& exception_state) = 0;
};

}  // namespace bindings

template <typename IDLInterface>
class PairSyncIterable {
 public:
  using SyncIteratorType = SyncIterator<IDLInterface>;
  // Check whether the v8_sync_iterator_foo_bar.h is appropriately included.
  // Make sizeof require the complete definition of the class.
  static_assert(
      sizeof(SyncIteratorType),  // Read the following for a compile error.
      "You need to include a generated header for SyncIterator<IDLInterface> "
      "in order to inherit from PairSyncIterable. "
      "For an IDL interface FooBar, #include "
      "\"third_party/blink/renderer/bindings/<component>/v8/"
      "v8_sync_iterator_foo_bar.h\" is required.");

  using IDLKeyType = typename SyncIteratorType::IDLKeyType;
  using IDLValueType = typename SyncIteratorType::IDLValueType;
  using KeyType = typename SyncIteratorType::KeyType;
  using ValueType = typename SyncIteratorType::ValueType;
  using IterationSource = bindings::
      PairSyncIterationSource<IDLKeyType, IDLValueType, KeyType, ValueType>;

  PairSyncIterable() = default;
  ~PairSyncIterable() = default;
  PairSyncIterable(const PairSyncIterable&) = delete;
  PairSyncIterable& operator=(const PairSyncIterable&) = delete;

  SyncIteratorType* keysForBinding(ScriptState* script_state,
                                   ExceptionState& exception_state) {
    IterationSource* source =
        CreateIterationSource(script_state, exception_state);
    if (!source)
      return nullptr;
    return MakeGarbageCollected<SyncIteratorType>(source,
                                                  SyncIteratorType::Kind::kKey);
  }

  SyncIteratorType* valuesForBinding(ScriptState* script_state,
                                     ExceptionState& exception_state) {
    IterationSource* source =
        CreateIterationSource(script_state, exception_state);
    if (!source)
      return nullptr;
    return MakeGarbageCollected<SyncIteratorType>(
        source, SyncIteratorType::Kind::kValue);
  }

  SyncIteratorType* entriesForBinding(ScriptState* script_state,
                                      ExceptionState& exception_state) {
    IterationSource* source =
        CreateIterationSource(script_state, exception_state);
    if (!source)
      return nullptr;
    return MakeGarbageCollected<SyncIteratorType>(
        source, SyncIteratorType::Kind::kKeyValue);
  }

  void forEachForBinding(ScriptState* script_state,
                         const ScriptValue& this_value,
                         V8ForEachIteratorCallback* callback,
                         const ScriptValue& this_arg,
                         ExceptionState& exception_state) {
    IterationSource* source =
        CreateIterationSource(script_state, exception_state);
    if (!source)
      return;
    source->ForEach(script_state, this_value, callback, this_arg,
                    exception_state);
  }

 private:
  virtual IterationSource* CreateIterationSource(
      ScriptState* script_state,
      ExceptionState& exception_state) = 0;
};

template <typename IDLInterface>
class ValueSyncIterable {
 public:
  using SyncIteratorType = SyncIterator<IDLInterface>;
  // Check whether the v8_sync_iterator_foo_bar.h is appropriately included.
  // Make sizeof require the complete definition of the class.
  static_assert(
      sizeof(SyncIteratorType),  // Read the following for a compile error.
      "You need to include a generated header for SyncIterator<IDLInterface> "
      "in order to inherit from ValueSyncIterable. "
      "For an IDL interface FooBar, #include "
      "\"third_party/blink/renderer/bindings/<component>/v8/"
      "v8_sync_iterator_foo_bar.h\" is required.");

  using IDLValueType = typename SyncIteratorType::IDLValueType;
  using ValueType = typename SyncIteratorType::ValueType;
  using IterationSource =
      bindings::ValueSyncIterationSource<IDLValueType, ValueType>;

  ValueSyncIterable() = default;
  ~ValueSyncIterable() = default;
  ValueSyncIterable(const ValueSyncIterable&) = delete;
  ValueSyncIterable& operator=(const ValueSyncIterable&) = delete;

  SyncIteratorType* keysForBinding(ScriptState* script_state,
                                   ExceptionState& exception_state) {
    IterationSource* source =
        CreateIterationSource(script_state, exception_state);
    if (!source)
      return nullptr;
    return MakeGarbageCollected<SyncIteratorType>(source,
                                                  SyncIteratorType::Kind::kKey);
  }

  SyncIteratorType* valuesForBinding(ScriptState* script_state,
                                     ExceptionState& exception_state) {
    IterationSource* source =
        CreateIterationSource(script_state, exception_state);
    if (!source)
      return nullptr;
    return MakeGarbageCollected<SyncIteratorType>(
        source, SyncIteratorType::Kind::kValue);
  }

  SyncIteratorType* entriesForBinding(ScriptState* script_state,
                                      ExceptionState& exception_state) {
    IterationSource* source =
        CreateIterationSource(script_state, exception_state);
    if (!source)
      return nullptr;
    return MakeGarbageCollected<SyncIteratorType>(
        source, SyncIteratorType::Kind::kKeyValue);
  }

  void forEachForBinding(ScriptState* script_state,
                         const ScriptValue& this_value,
                         V8ForEachIteratorCallback* callback,
                         const ScriptValue& this_arg,
                         ExceptionState& exception_state) {
    IterationSource* source =
        CreateIterationSource(script_state, exception_state);
    if (!source)
      return;
    source->ForEach(script_state, this_value, callback, this_arg,
                    exception_state);
  }

 private:
  virtual IterationSource* CreateIterationSource(
      ScriptState* script_state,
      ExceptionState& exception_state) = 0;
};

// Unpacks `sync_iteration_result`, stores 'value' and 'done' properties in
// `out_value` and 'out_done` respectively, and returns true on success.
[[nodiscard]] CORE_EXPORT bool V8UnpackIterationResult(
    ScriptState* script_state,
    v8::Local<v8::Object> sync_iteration_result,
    v8::Local<v8::Value>* out_value,
    bool* out_done);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_BINDINGS_CORE_V8_ITERABLE_H_
