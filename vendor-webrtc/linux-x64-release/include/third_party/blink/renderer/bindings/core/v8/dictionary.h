/*
 * Copyright (C) 2010 Google Inc. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 *
 * 1.  Redistributions of source code must retain the above copyright
 *     notice, this list of conditions and the following disclaimer.
 * 2.  Redistributions in binary form must reproduce the above copyright
 *     notice, this list of conditions and the following disclaimer in the
 *     documentation and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY APPLE AND ITS CONTRIBUTORS "AS IS" AND ANY
 * EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED
 * WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
 * DISCLAIMED. IN NO EVENT SHALL APPLE OR ITS CONTRIBUTORS BE LIABLE FOR ANY
 * DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES
 * (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES;
 * LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND
 * ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF
 * THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_BINDINGS_CORE_V8_DICTIONARY_H_
#define THIRD_PARTY_BLINK_RENDERER_BINDINGS_CORE_V8_DICTIONARY_H_

#include <optional>

#include "base/notreached.h"
#include "third_party/blink/renderer/bindings/core/v8/native_value_traits_impl.h"
#include "third_party/blink/renderer/bindings/core/v8/script_value.h"
#include "third_party/blink/renderer/bindings/core/v8/v8_binding_for_core.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/bindings/exception_state.h"
#include "third_party/blink/renderer/platform/wtf/hash_map.h"
#include "third_party/blink/renderer/platform/wtf/text/string_view.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"
#include "v8/include/v8.h"

namespace blink {

// Dictionary class provides ways to retrieve property values as C++ objects
// from a V8 object. Instances of this class must not outlive V8's handle scope
// because they hold a V8 value without putting it on persistent handles.
class CORE_EXPORT Dictionary final {
  DISALLOW_NEW();

 public:
  Dictionary() : isolate_(nullptr) {}
  explicit Dictionary(v8::Isolate*,
                      v8::Local<v8::Value> dictionary_object,
                      ExceptionState&);
  // ScriptValue can refer a V8 object, and such a ScriptValue can be
  // converted into Dictionary without exceptions.
  explicit Dictionary(const ScriptValue& script_value)
      : isolate_(script_value.GetIsolate()) {
    CHECK(script_value.IsObject());
    dictionary_object_ = script_value.V8Value().As<v8::Object>();
    value_type_ = ValueType::kObject;
  }

  Dictionary(const Dictionary&) = default;
  Dictionary& operator=(const Dictionary&) = default;

  bool IsObject() const { return !dictionary_object_.IsEmpty(); }
  bool IsUndefinedOrNull() const { return !IsObject(); }

  v8::Local<v8::Value> V8Value() const {
    if (!isolate_)
      return v8::Local<v8::Value>();
    switch (value_type_) {
      case ValueType::kUndefined:
        return v8::Undefined(isolate_);
      case ValueType::kNull:
        return v8::Null(isolate_);
      case ValueType::kObject:
        return dictionary_object_;
      default:
        NOTREACHED();
    }
  }

  bool Get(const StringView& key, v8::Local<v8::Value>& value) const {
    return isolate_ && Get(V8String(isolate_, key), value);
  }
  bool Get(const StringView& key,
           v8::Local<v8::Value>& value,
           ExceptionState& exception_state) const {
    return isolate_ &&
           GetInternal(V8String(isolate_, key), value, exception_state);
  }
  bool Get(const StringView& key, Dictionary&) const;

  // Gets the value of the given property in this dictionary and returns it.
  // The type parameter |IDLType| is an IDL type (e.g., IDLByteString).
  //  - If accessing the property raises an error, the error is set to the
  //    ExceptionState and returns nothing.
  //  - If converting data fails, the error is set to the ExceptionState and
  //    returns nothing.
  //  - If |key| property is not present in this dictionary (including the case
  //    where the stored value is |undefined|), returns nothing.
  //  - Otherwise, returns the value.
  template <typename IDLType>
  std::optional<typename IDLType::ImplType> Get(
      const StringView& key,
      ExceptionState& exception_state) const {
    v8::Local<v8::Value> v8_value;
    DCHECK(!exception_state.HadException());
    if (!Get(key, v8_value, exception_state))
      return std::nullopt;
    DCHECK(!exception_state.HadException());
    DCHECK(!v8_value.IsEmpty());
    if (v8_value->IsUndefined())
      return std::nullopt;

    auto value = NativeValueTraits<IDLType>::NativeValue(isolate_, v8_value,
                                                         exception_state);
    if (exception_state.HadException())
      return std::nullopt;
    return value;
  }

  bool Get(v8::Local<v8::Value> key, v8::Local<v8::Value>& result) const;

  HashMap<String, String> GetOwnPropertiesAsStringHashMap(
      ExceptionState&) const;
  Vector<String> GetPropertyNames(ExceptionState&) const;

  bool HasProperty(const StringView& key, ExceptionState&) const;

  v8::Isolate* GetIsolate() const { return isolate_; }
  v8::Local<v8::Context> V8Context() const {
    DCHECK(isolate_);
    return isolate_->GetCurrentContext();
  }

 private:
  bool GetInternal(const v8::Local<v8::Value>& key,
                   v8::Local<v8::Value>& result,
                   ExceptionState&) const;

  v8::Isolate* isolate_;
  // Undefined, Null, or Object is allowed as type of dictionary.
  enum class ValueType {
    kUndefined,
    kNull,
    kObject
  } value_type_ = ValueType::kUndefined;
  v8::Local<v8::Object> dictionary_object_;  // an Object or empty
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_BINDINGS_CORE_V8_DICTIONARY_H_
