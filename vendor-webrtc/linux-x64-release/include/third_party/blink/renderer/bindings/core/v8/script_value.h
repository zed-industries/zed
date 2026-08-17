/*
 * Copyright (C) 2008, 2009 Google Inc. All rights reserved.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_BINDINGS_CORE_V8_SCRIPT_VALUE_H_
#define THIRD_PARTY_BLINK_RENDERER_BINDINGS_CORE_V8_SCRIPT_VALUE_H_

#include "base/memory/scoped_refptr.h"
#include "third_party/blink/renderer/bindings/core/v8/native_value_traits.h"
#include "third_party/blink/renderer/bindings/core/v8/world_safe_v8_reference.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/bindings/trace_wrapper_v8_reference.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"
#include "v8/include/v8.h"

namespace blink {
namespace bindings {
class DictionaryBase;
class UnionBase;
}  // namespace bindings
class ScriptState;

// ScriptValue is used when an idl specifies the type as 'any'. ScriptValue
// stores the v8 value using WorldSafeV8Reference.
class CORE_EXPORT ScriptValue final {
  DISALLOW_NEW();

 public:
  // ScriptValue::From() is restricted to certain types that are unambiguous in
  // how they are exposed to V8. Objects that need to know what the expected IDL
  // type is in order to be correctly converted must explicitly use ToV8Traits<>
  // to get a v8::Value, then pass it directly to the constructor.
  template <typename T>
    requires std::derived_from<T, bindings::DictionaryBase> ||
             std::derived_from<T, ScriptWrappable> ||
             std::derived_from<T, bindings::UnionBase>
  static ScriptValue From(ScriptState* script_state, T* value) {
    return ScriptValue(script_state->GetIsolate(), value->ToV8(script_state));
  }

  ScriptValue() = default;

  ScriptValue(v8::Isolate* isolate, v8::Local<v8::Value> value)
      : value_(isolate, value) {}

  ~ScriptValue() {
    // Reset() below eagerly cleans up Oilpan-internal book-keeping data
    // structures. Since most uses of ScriptValue are from stack or parameters
    // this significantly helps in keeping memory compact at the expense of a
    // few more finalizers in the on-heap use case. Keeping the internals
    // compact is important in AudioWorklet use cases that don't allocate and
    // thus never trigger GC.
    //
    // Note: If you see a CHECK() fail in non-production code (e.g. C++ unit
    // tests) then this means that the test runs manual GCs and/or invokes the
    // `RunLoop` to trigger GCs without stack while having a ScriptValue on the
    // stack which is not supported. To solve this pass the `v8::StackState`
    // explicitly on GCs. Alternatively, you can keep ScriptValue alive via
    // wrapper objects through Persistent instead of referring to it from the
    // stack.
    //
    // TODO(v8:v8:13372): Remove once v8::TracedReference is implemented as
    // direct pointer.
    value_.Reset();
  }

  ScriptValue(const ScriptValue& value) = default;

  // Prefer getting the Isolate from ScriptState or similar.
  v8::Isolate* GetIsolate() const {
    DCHECK(!IsEmpty());
    return value_.GetIsolate();
  }

  ScriptValue& operator=(const ScriptValue& value) = default;

  bool operator==(const ScriptValue& value) const {
    if (IsEmpty())
      return value.IsEmpty();
    if (value.IsEmpty())
      return false;
    return value_ == value.value_;
  }

  bool operator!=(const ScriptValue& value) const { return !operator==(value); }

  // This creates a new local Handle; Don't use this in performance-sensitive
  // places.
  bool IsFunction() const {
    DCHECK(!IsEmpty());
    v8::Local<v8::Value> value = V8Value();
    return !value.IsEmpty() && value->IsFunction();
  }

  // This creates a new local Handle; Don't use this in performance-sensitive
  // places.
  bool IsNull() const {
    DCHECK(!IsEmpty());
    v8::Local<v8::Value> value = V8Value();
    return !value.IsEmpty() && value->IsNull();
  }

  // This creates a new local Handle; Don't use this in performance-sensitive
  // places.
  bool IsUndefined() const {
    DCHECK(!IsEmpty());
    v8::Local<v8::Value> value = V8Value();
    return !value.IsEmpty() && value->IsUndefined();
  }

  // This creates a new local Handle; Don't use this in performance-sensitive
  // places.
  bool IsObject() const {
    DCHECK(!IsEmpty());
    v8::Local<v8::Value> value = V8Value();
    return !value.IsEmpty() && value->IsObject();
  }

  bool IsEmpty() const { return value_.IsEmpty(); }

  void Clear() {
    value_.Reset();
  }

  v8::Local<v8::Value> V8Value() const;
  // Returns v8Value() if a given ScriptState is the same as the
  // ScriptState which is associated with this ScriptValue. Otherwise
  // this "clones" the v8 value and returns it.
  v8::Local<v8::Value> V8ValueFor(ScriptState*) const;

  // Coereces the underlying v8::Value to a string.
  bool ToString(String&) const;

  static ScriptValue CreateNull(v8::Isolate*);

  void Trace(Visitor* visitor) const { visitor->Trace(value_); }

 private:
  WorldSafeV8Reference<v8::Value> value_;
};

// ScriptObject is used when an idl specifies the type as 'object'
class CORE_EXPORT ScriptObject final {
  DISALLOW_NEW();

 public:
  // ScriptObject::From() is restricted to certain types that are unambiguous in
  // how they are exposed to V8 and that are known to be objects.
  template <typename T>
    requires std::derived_from<T, bindings::DictionaryBase> ||
             std::derived_from<T, ScriptWrappable>
  static ScriptObject From(ScriptState* script_state, T* value) {
    return ScriptObject(script_state->GetIsolate(), value->ToV8(script_state));
  }

  ScriptObject() = default;
  ScriptObject(v8::Isolate* isolate, v8::Local<v8::Value> value)
      : object_(isolate, value) {
    CHECK(!value.IsEmpty());
    CHECK(value->IsObject() || value->IsNull());
  }

  static ScriptObject CreateNull(v8::Isolate* isolate) {
    return ScriptObject(isolate, v8::Null(isolate));
  }

  v8::Local<v8::Object> V8Object() const {
    CHECK(object_.IsObject());
    return object_.V8Value().As<v8::Object>();
  }

  v8::Local<v8::Object> V8ObjectFor(ScriptState* script_state) const {
    CHECK(object_.IsObject());
    return object_.V8ValueFor(script_state).As<v8::Object>();
  }

  bool IsNull() const { return object_.IsNull(); }

  void Clear() { object_.Clear(); }

  // NOLINTNEXTLINE(google-explicit-constructor)
  operator const ScriptValue&() const { return object_; }
  // NOLINTNEXTLINE(google-explicit-constructor)
  operator ScriptValue&() { return object_; }

  bool operator==(const ScriptObject& object) const {
    return object_ == object.object_;
  }

  bool operator!=(const ScriptObject& object) const {
    return !operator==(object);
  }

  void Trace(Visitor* visitor) const { visitor->Trace(object_); }

 private:
  ScriptValue object_;
};

}  // namespace blink

namespace WTF {

// VectorTraits for ScriptValue depend entirely on
// WorldSafeV8Reference<v8::Value>.
template <>
struct VectorTraits<blink::ScriptValue>
    : VectorTraits<blink::WorldSafeV8Reference<v8::Value>> {};

template <>
struct VectorTraits<blink::ScriptObject>
    : VectorTraits<blink::WorldSafeV8Reference<v8::Value>> {};

}  // namespace WTF

#endif  // THIRD_PARTY_BLINK_RENDERER_BINDINGS_CORE_V8_SCRIPT_VALUE_H_
