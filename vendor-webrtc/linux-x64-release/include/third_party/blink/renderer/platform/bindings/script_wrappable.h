/*
 * Copyright (C) 2010 Google Inc. All rights reserved.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_BINDINGS_SCRIPT_WRAPPABLE_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_BINDINGS_SCRIPT_WRAPPABLE_H_

#include "build/build_config.h"
#include "third_party/blink/renderer/platform/bindings/name_client.h"
#include "third_party/blink/renderer/platform/bindings/trace_wrapper_v8_reference.h"
#include "third_party/blink/renderer/platform/bindings/wrapper_type_info.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/wtf/type_traits.h"
#include "v8/include/v8.h"

namespace blink {

class DOMDataStore;
class ScriptState;

// ScriptWrappable provides a way to map from/to C++ DOM implementation to/from
// JavaScript object (platform object).  ToV8() converts a ScriptWrappable to
// a v8::Object and ToScriptWrappable() converts a v8::Object back to
// a ScriptWrappable.  v8::Object as platform object is called "wrapper object".
// The wrapper object for the main world is stored in ScriptWrappable.  Wrapper
// objects for other worlds are stored in DOMDataStore.
class PLATFORM_EXPORT ScriptWrappable
    : public GarbageCollected<ScriptWrappable>,
      public NameClient {
 public:
  // This is a type dispatcher from ScriptWrappable* to a subtype, optimized for
  // use cases that perform downcasts multiple times.
  class TypeDispatcher final {
    STACK_ALLOCATED();

   public:
    // The input parameter `script_wrappable` must not be null.
    explicit TypeDispatcher(ScriptWrappable* script_wrappable)
        : script_wrappable_(script_wrappable),
          wrapper_type_info_(script_wrappable->GetWrapperTypeInfo()) {}
    ~TypeDispatcher() = default;

    TypeDispatcher(const TypeDispatcher&) = delete;
    TypeDispatcher& operator=(const TypeDispatcher&) = delete;

    // Downcasts the ScriptWrappable to the given template parameter type or
    // nullptr if the ScriptWrappable doesn't implement the given type. The
    // inheritance is checked with WrapperTypeInfo, i.e. the check is based on
    // the IDL definitions in *.idl files, not based on C++ class inheritance.
    template <typename T>
    T* DowncastTo() {
      if (wrapper_type_info_->IsSubclass(T::GetStaticWrapperTypeInfo()))
        return static_cast<T*>(script_wrappable_);
      return nullptr;
    }

    // Downcasts the ScriptWrappable to the given template parameter type iff
    // the ScriptWrappable implements the type as the most derived class (i.e.
    // the ScriptWrappable does _not_ implement a subtype of the given type).
    // Otherwise, returns nullptr. The inheritance is checked with
    // WrapperTypeInfo, i.e. the check is based on the IDL definitions in *.idl
    // files, not based on C++ class inheritance.
    template <typename T>
    T* ToMostDerived() {
      if (wrapper_type_info_ == T::GetStaticWrapperTypeInfo())
        return static_cast<T*>(script_wrappable_);
      return nullptr;
    }

   private:
    ScriptWrappable* script_wrappable_ = nullptr;
    const WrapperTypeInfo* wrapper_type_info_ = nullptr;
  };

  ScriptWrappable(const ScriptWrappable&) = delete;
  ScriptWrappable& operator=(const ScriptWrappable&) = delete;
  ~ScriptWrappable() override = default;

  const char* NameInHeapSnapshot() const override;

  virtual void Trace(Visitor*) const;

  // Returns the WrapperTypeInfo of the instance.
  //
  // This method must be overridden by DEFINE_WRAPPERTYPEINFO macro.
  virtual const WrapperTypeInfo* GetWrapperTypeInfo() const = 0;

  // Returns a wrapper object, creating it if needed.
  v8::Local<v8::Value> ToV8(ScriptState*);

  // This overload is used for the case when a `ToV8()` caller does not have
  // `script_state` but does have a receiver object (a creation context object)
  // which is needed to create a wrapper. If a wrapper object corresponding to
  // the receiver object exists, `ToV8()` can return it without a call to
  // `ScriptState::ForRelevantRealm`, which is slow.
  v8::Local<v8::Value> ToV8(v8::Isolate*,
                            v8::Local<v8::Object> creation_context_object);

  // Creates and returns a new wrapper object. This DCHECKs that a wrapper does
  // not exist yet. Use ToV8() if a wrapper might already exist.
  virtual v8::Local<v8::Value> Wrap(ScriptState*);

  // Associates the instance with the given |wrapper| if this instance is not
  // yet associated with any wrapper.  Returns the wrapper already associated
  // or |wrapper| if not yet associated.
  // The caller should always use the returned value rather than |wrapper|.
  [[nodiscard]] virtual v8::Local<v8::Object> AssociateWithWrapper(
      v8::Isolate*,
      const WrapperTypeInfo*,
      v8::Local<v8::Object> wrapper);

 protected:
  ScriptWrappable() = default;

 private:
  static_assert(
      std::is_trivially_destructible<
          TraceWrapperV8Reference<v8::Object>>::value,
      "TraceWrapperV8Reference<v8::Object> should be trivially destructible.");

  // Inline storage for the a single wrapper reference. Only
  // `DOMDataStore::UncheckedInlineStorageForWrappable()` should access this
  // field.
  TraceWrapperV8Reference<v8::Object> wrapper_;
  friend class DOMDataStore;
};

template <typename T>
  requires std::derived_from<T, ScriptWrappable>
T* ToScriptWrappable(v8::Isolate* isolate, v8::Local<v8::Object> wrapper) {
  const WrapperTypeInfo* wrapper_type_info = T::GetStaticWrapperTypeInfo();
  return static_cast<T*>(v8::Object::Unwrap<ScriptWrappable>(
      isolate, wrapper,
      v8::CppHeapPointerTagRange(wrapper_type_info->this_tag,
                                 wrapper_type_info->max_subclass_tag)));
}

// Defines |GetWrapperTypeInfo| virtual method which returns the WrapperTypeInfo
// of the instance. Also declares a static member of type WrapperTypeInfo, of
// which the definition is given by the IDL code generator.
//
// All the derived classes of ScriptWrappable, regardless of directly or
// indirectly, must write this macro in the class definition as long as the
// class has a corresponding .idl file.
#define DEFINE_WRAPPERTYPEINFO()                               \
 public:                                                       \
  const WrapperTypeInfo* GetWrapperTypeInfo() const override { \
    return &wrapper_type_info_;                                \
  }                                                            \
  static const WrapperTypeInfo* GetStaticWrapperTypeInfo() {   \
    return &wrapper_type_info_;                                \
  }                                                            \
                                                               \
 private:                                                      \
  static const WrapperTypeInfo& wrapper_type_info_

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_BINDINGS_SCRIPT_WRAPPABLE_H_
