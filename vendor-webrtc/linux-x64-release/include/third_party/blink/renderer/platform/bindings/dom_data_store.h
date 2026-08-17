/*
 * Copyright (C) 2009 Google Inc. All rights reserved.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_BINDINGS_DOM_DATA_STORE_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_BINDINGS_DOM_DATA_STORE_H_

#include "base/containers/contains.h"
#include "third_party/blink/renderer/platform/bindings/dom_wrapper_world.h"
#include "third_party/blink/renderer/platform/bindings/script_state.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/bindings/trace_wrapper_v8_reference.h"
#include "third_party/blink/renderer/platform/bindings/wrapper_type_info.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_map.h"
#include "third_party/blink/renderer/platform/heap/prefinalizer.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/stack_util.h"
#include "third_party/blink/renderer/platform/wtf/std_lib_extras.h"
#include "v8/include/v8.h"

namespace blink {

// DOMDataStore is the entity responsible for managing wrapper references (C++
// to JS) in different worlds and settings (get/set wrapper, set return value).
// Such references are stored in two ways:
// - Inline in ScriptWrappable (or CustomWrappable); or
// - In an ephemeron map in the DOMDataStore instance that is tied to a context
//   in a V8 Isolate.
//
// The methods on DOMDataStore generally come in two forms:
// - static methods: Have fast paths for inline storage (e.g. main world on the
//   main rendering thread) and otherwise consider the current world, i.e., the
//   world that corresponds to the current context
//   (`v8::Isolate::GetCurrentContext()`).
// - instance methods: Have fast paths for inline storage and consider the
//   world the methods are invoked on. The calls are faster than the static
//   calls if the DOMDataStore is already around.
//
// Exceptions are methods that operate on all worlds instead of the current
// world. The methods mention this fact explicitly.
class DOMDataStore final : public GarbageCollected<DOMDataStore> {
 public:
  static DOMDataStore& Current(v8::Isolate* isolate) {
    return DOMWrapperWorld::Current(isolate).DomDataStore();
  }

  // Sets the `return_value` from `value` in the given `context`.
  static inline bool SetReturnValue(v8::ReturnValue<v8::Value> return_value,
                                    ScriptWrappable* value,
                                    v8::Local<v8::Context> context);

  // Sets the `return_value` from `value` in a world that can use inline
  // storage.
  static inline bool SetReturnValueFromInlineStorage(
      v8::ReturnValue<v8::Value> return_value,
      const ScriptWrappable* value);

  // Sets the `return_value` from `value` if already wrapped and returns false
  // otherwise. Will use the `v8_receiver` and `blink_receiver` to check whether
  // inline storage can be used.  Can be used from any world. Will only consider
  // the current world.
  static inline bool SetReturnValueFast(v8::ReturnValue<v8::Value> return_value,
                                        ScriptWrappable* value,
                                        v8::Local<v8::Object> v8_receiver,
                                        const ScriptWrappable* blink_receiver);

  // Returns the wrapper for `object` in the world corresponding to
  // `script_state`. Can be used from any world. Cross-checks the world in
  // `script_state` against the current world used by the Isolate (in the script
  // state).
  //
  // Prefer this method over the version taking the Isolate parameter for
  // performance reasons.
  static inline v8::MaybeLocal<v8::Object> GetWrapper(
      ScriptState* script_state,
      const ScriptWrappable* object);

  // Returns the wrapper for `object` in the world corresponding to `isolate`.
  // Can be used from any world. Will only consider the current world.
  //
  // Prefer the method taking the ScriptState if possible for performance
  // reasons.
  static inline v8::MaybeLocal<v8::Object> GetWrapper(
      v8::Isolate* isolate,
      const ScriptWrappable* object);

  // Associates the given `object` with the given `wrapper` if the object is not
  // yet associated with any wrapper.  Returns true if the given wrapper is
  // associated with the object, or false if the object is already associated
  // with a wrapper.  In the latter case, `wrapper` will be updated to the
  // existing wrapper. Can be used from any world. Will only consider the
  // current world.
  [[nodiscard]] static inline bool SetWrapper(
      v8::Isolate* isolate,
      ScriptWrappable* object,
      const WrapperTypeInfo* wrapper_type_info,
      v8::Local<v8::Object>& wrapper);

  // Same as `SetWrapper()` with the difference that it only considers inline
  // storage. Can be used from any world.
  template <bool entered_context = true>
  [[nodiscard]] static inline bool SetWrapperInInlineStorage(
      v8::Isolate* isolate,
      ScriptWrappable* object,
      const WrapperTypeInfo* wrapper_type_info,
      v8::Local<v8::Object>& wrapper);

  // Checks for the wrapper pair in all worlds and clears the first found pair.
  // This should only be needed for garbage collection. All other callsites
  // should know their worlds.
  template <typename HandleType>
  static inline bool ClearWrapperInAnyWorldIfEqualTo(ScriptWrappable* object,
                                                     const HandleType& handle);

  // Clears the inline storage wrapper if it is equal to `handle`. Can be used
  // from any world.
  template <typename HandleType>
  static inline bool ClearInlineStorageWrapperIfEqualTo(
      ScriptWrappable* object,
      const HandleType& handle);

  // Checks whether a wrapper for `object` exists. Can be used from any world.
  // Will only consider the current world.
  static inline bool ContainsWrapper(v8::Isolate* isolate,
                                     const ScriptWrappable* object);

  DOMDataStore(v8::Isolate* isolate, bool);
  DOMDataStore(const DOMDataStore&) = delete;
  DOMDataStore& operator=(const DOMDataStore&) = delete;

  // Destruction does not need any special logic: The wrapper map is reclaimed
  // without weak callbacks. Internally, the references of type
  // `TraceWrapperV8Reference` are reclaimed by a full GC. A non-tracing V8 GC
  // considers the references here still as roots but it may drop those roots
  // (see BlinkGCRootsHandler) which still works because the
  // WorldMap->DOMWrapperWorld->DomDataStore chain is valid.
  ~DOMDataStore() = default;

  // Clears all references explicitly. This is a performance optimization to
  // clear out TraceWrapperV8Reference eagerly.
  //
  // In practice, workers and worklets dispose their worlds before tear down.
  // Temporary isolated worlds just rely on destruction behavior.
  void Dispose();

  // Same as `GetWrapper()` but for a single world.
  template <bool entered_context = true>
  inline v8::MaybeLocal<v8::Object> Get(v8::Isolate* isolate,
                                        const ScriptWrappable* object);

  // Same as `SetWrapper()` but for a single world.
  template <bool entered_context = true>
  [[nodiscard]] inline bool Set(v8::Isolate* isolate,
                                ScriptWrappable* object,
                                const WrapperTypeInfo* wrapper_type_info,
                                v8::Local<v8::Object>& wrapper);

  // Same as `ContainsWrapper()` but for a single world.
  inline bool Contains(const ScriptWrappable* object) const;

  // Returns true if the pair {object, handle} exists in the current world.
  template <typename HandleType>
  inline bool EqualTo(const ScriptWrappable* object, const HandleType& handle);

  // Clears the connection between object and handle in the current world.
  template <typename HandleType>
  inline bool ClearIfEqualTo(ScriptWrappable* object,
                             const HandleType& handle) {
    if (can_use_inline_storage_) {
      return ClearInlineStorageWrapperIfEqualTo(object, handle);
    }
    return ClearInMapIfEqualTo(object, handle);
  }

  // Clears the connection between object and handle in the current world
  // assuming no inline storage is available for this world.
  template <typename HandleType>
  inline bool ClearInMapIfEqualTo(const ScriptWrappable* object,
                                  const HandleType& handle) {
    DCHECK(!can_use_inline_storage_);
    if (const auto& it = wrapper_map_.find(object);
        it != wrapper_map_.end() && it->value == handle) {
      it->value.Reset();
      wrapper_map_.erase(it);
      return true;
    }
    return false;
  }

  void Trace(Visitor*) const;

 private:
  // We can use the inline storage in a ScriptWrappable when we're in the main
  // world. This method does the fast check if we're in the main world. If this
  // method returns true, it is guaranteed that we're in the main world. On the
  // other hand, if this method returns false, nothing is guaranteed (we might
  // be in the main world).
  static bool CanUseInlineStorageForWrapper() {
    return !WTF::MayNotBeMainThread() &&
           !DOMWrapperWorld::NonMainWorldsExistInMainThread();
  }

  inline bool SetReturnValueFrom(v8::ReturnValue<v8::Value> return_value,
                                 const ScriptWrappable* value);

  using WrapperRefType = decltype(ScriptWrappable::wrapper_);

  // Convenience methods for accessing the inlined storage.
  static inline WrapperRefType& GetUncheckedInlineStorage(
      ScriptWrappable* wrappable) {
    return wrappable->wrapper_;
  }
  static inline const WrapperRefType& GetUncheckedInlineStorage(
      const ScriptWrappable* wrappable) {
    return GetUncheckedInlineStorage(const_cast<ScriptWrappable*>(wrappable));
  }
  static inline WrapperRefType& GetInlineStorage(v8::Isolate* isolate,
                                                 ScriptWrappable* wrappable) {
    // The following will crash if no context is entered. This is by design. The
    // validation can be skipped with `SetWrapperInInlineStorage<false>()`.
    DCHECK(Current(isolate).can_use_inline_storage_);
    return GetUncheckedInlineStorage(wrappable);
  }
  static inline const WrapperRefType& GetInlineStorage(
      v8::Isolate* isolate,
      const ScriptWrappable* wrappable) {
    return GetInlineStorage(isolate, const_cast<ScriptWrappable*>(wrappable));
  }

  // Specifies whether this data store is allowed to use inline storage of a
  // ScriptWrappable and can avoid using the ephemeron map below.
  const bool can_use_inline_storage_;
  // Ephemeron map: V8 wrapper will be kept alive as long as ScriptWrappable is.
  HeapHashMap<WeakMember<const ScriptWrappable>,
              TraceWrapperV8Reference<v8::Object>>
      wrapper_map_;
};

// static
bool DOMDataStore::SetReturnValue(v8::ReturnValue<v8::Value> return_value,
                                  ScriptWrappable* value,
                                  v8::Local<v8::Context> context) {
  if (CanUseInlineStorageForWrapper()) {
    return SetReturnValueFromInlineStorage(return_value, value);
  }
  auto& world = DOMWrapperWorld::World(return_value.GetIsolate(), context);
  return world.DomDataStore().SetReturnValueFrom(return_value, value);
}

// static
bool DOMDataStore::SetReturnValueFromInlineStorage(
    v8::ReturnValue<v8::Value> return_value,
    const ScriptWrappable* value) {
  const auto& ref = GetInlineStorage(return_value.GetIsolate(), value);
  if (!ref.IsEmpty()) {
    return_value.SetNonEmpty(ref);
    return true;
  }
  return false;
}

// static
bool DOMDataStore::SetReturnValueFast(v8::ReturnValue<v8::Value> return_value,
                                      ScriptWrappable* object,
                                      v8::Local<v8::Object> v8_receiver,
                                      const ScriptWrappable* blink_receiver) {
  // Fast checks for using inline storage:
  // 1. Fast check for inline storage.
  // 2. If the receiver receiver's inline wrapper is the V8 receiver. In this
  //    case we know we are in the world that can use inline storage.
  DCHECK(blink_receiver);
  DCHECK(!v8_receiver.IsEmpty());
  if (CanUseInlineStorageForWrapper() ||
      v8_receiver == GetUncheckedInlineStorage(blink_receiver)) {
    return SetReturnValueFromInlineStorage(return_value, object);
  }
  return Current(return_value.GetIsolate())
      .SetReturnValueFrom(return_value, object);
}

bool DOMDataStore::SetReturnValueFrom(v8::ReturnValue<v8::Value> return_value,
                                      const ScriptWrappable* value) {
  if (can_use_inline_storage_) {
    return SetReturnValueFromInlineStorage(return_value, value);
  }
  if (const auto it = wrapper_map_.find(value); it != wrapper_map_.end()) {
    return_value.SetNonEmpty(it->value);
    return true;
  }
  return false;
}

// static
v8::MaybeLocal<v8::Object> DOMDataStore::GetWrapper(
    v8::Isolate* isolate,
    const ScriptWrappable* object) {
  if (CanUseInlineStorageForWrapper()) {
    return GetInlineStorage(isolate, object).Get(isolate);
  }
  return Current(isolate).Get(isolate, object);
}

// static
v8::MaybeLocal<v8::Object> DOMDataStore::GetWrapper(
    ScriptState* script_state,
    const ScriptWrappable* object) {
  DOMDataStore& store = script_state->World().DomDataStore();
  auto* isolate = script_state->GetIsolate();
  return store.Get(isolate, object);
}

template <bool entered_context>
v8::MaybeLocal<v8::Object> DOMDataStore::Get(v8::Isolate* isolate,
                                             const ScriptWrappable* object) {
  DCHECK(!CanUseInlineStorageForWrapper() || can_use_inline_storage_);
  if (can_use_inline_storage_) {
    // The following will crash if no context is entered. This is by design. The
    // validation can be skipped with `entered_context`.
    DCHECK(!entered_context || Current(isolate).can_use_inline_storage_);
    return GetUncheckedInlineStorage(object).Get(isolate);
  }
  if (const auto it = wrapper_map_.find(object); it != wrapper_map_.end()) {
    return it->value.Get(isolate);
  }
  return {};
}

// static
bool DOMDataStore::SetWrapper(v8::Isolate* isolate,
                              ScriptWrappable* object,
                              const WrapperTypeInfo* wrapper_type_info,
                              v8::Local<v8::Object>& wrapper) {
  if (CanUseInlineStorageForWrapper()) {
    return SetWrapperInInlineStorage(isolate, object, wrapper_type_info,
                                     wrapper);
  }
  return Current(isolate).Set(isolate, object, wrapper_type_info, wrapper);
}

// static
template <bool entered_context>
bool DOMDataStore::SetWrapperInInlineStorage(
    v8::Isolate* isolate,
    ScriptWrappable* object,
    const WrapperTypeInfo* wrapper_type_info,
    v8::Local<v8::Object>& wrapper) {
  DCHECK(!wrapper.IsEmpty());
  auto& ref = entered_context ? GetInlineStorage(isolate, object)
                              : GetUncheckedInlineStorage(object);
  if (!ref.IsEmpty()) [[unlikely]] {
    wrapper = ref.Get(isolate);
    return false;
  }
  if (wrapper_type_info->SupportsDroppingWrapper()) {
    ref.Reset(isolate, wrapper,
              TraceWrapperV8Reference<v8::Object>::IsDroppable{});
  } else {
    ref.Reset(isolate, wrapper);
  }
  DCHECK(!ref.IsEmpty());
  return true;
}

template <bool entered_context>
bool DOMDataStore::Set(v8::Isolate* isolate,
                       ScriptWrappable* object,
                       const WrapperTypeInfo* wrapper_type_info,
                       v8::Local<v8::Object>& wrapper) {
  DCHECK(object);
  DCHECK(!wrapper.IsEmpty());
  if (can_use_inline_storage_) {
    return SetWrapperInInlineStorage<entered_context>(
        isolate, object, wrapper_type_info, wrapper);
  }
  auto result = wrapper_map_.insert(
      object, wrapper_type_info->SupportsDroppingWrapper()
                  ? TraceWrapperV8Reference<v8::Object>(
                        isolate, wrapper,
                        TraceWrapperV8Reference<v8::Object>::IsDroppable{})
                  : TraceWrapperV8Reference<v8::Object>(isolate, wrapper));
  // TODO(mlippautz): Check whether there's still recursive cases of
  // Wrap()/AssociateWithWrapper() that can run into the case of an existing
  // entry.
  if (!result.is_new_entry) [[unlikely]] {
    CHECK(!result.stored_value->value.IsEmpty());
    wrapper = result.stored_value->value.Get(isolate);
  }
  return result.is_new_entry;
}

//  static
bool DOMDataStore::ContainsWrapper(v8::Isolate* isolate,
                                   const ScriptWrappable* object) {
  if (CanUseInlineStorageForWrapper()) {
    return !GetInlineStorage(isolate, object).IsEmpty();
  }
  return Current(isolate).Contains(object);
}

// Same as `ContainsWrapper()` but for a single world.
bool DOMDataStore::Contains(const ScriptWrappable* object) const {
  if (can_use_inline_storage_) {
    return !GetUncheckedInlineStorage(object).IsEmpty();
  }
  return base::Contains(wrapper_map_, object);
}

// static
template <typename HandleType>
bool DOMDataStore::ClearWrapperInAnyWorldIfEqualTo(ScriptWrappable* object,
                                                   const HandleType& handle) {
  if (ClearInlineStorageWrapperIfEqualTo(object, handle)) [[likely]] {
    return true;
  }
  return DOMWrapperWorld::ClearWrapperInAnyNonInlineStorageWorldIfEqualTo(
      object, handle);
}

// static
template <typename HandleType>
bool DOMDataStore::ClearInlineStorageWrapperIfEqualTo(
    ScriptWrappable* object,
    const HandleType& handle) {
  auto& ref = GetUncheckedInlineStorage(object);
  if (ref == handle) {
    ref.Reset();
    return true;
  }
  return false;
}

template <typename HandleType>
bool DOMDataStore::EqualTo(const ScriptWrappable* object,
                           const HandleType& handle) {
  if (can_use_inline_storage_) {
    return GetUncheckedInlineStorage(object) == handle;
  }
  if (const auto& it = wrapper_map_.find(object);
      it != wrapper_map_.end() && it->value == handle) {
    return true;
  }
  return false;
}

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_BINDINGS_DOM_DATA_STORE_H_
