/*
 * Copyright (C) 2009 Google Inc. All rights reserved.
 * Copyright (C) 2012 Ericsson AB. All rights reserved.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_BINDINGS_V8_BINDING_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_BINDINGS_V8_BINDING_H_

#include "third_party/blink/renderer/platform/bindings/dom_wrapper_world.h"
#include "third_party/blink/renderer/platform/bindings/to_blink_string.h"
#include "third_party/blink/renderer/platform/bindings/v8_per_isolate_data.h"
#include "third_party/blink/renderer/platform/bindings/v8_value_cache.h"
#include "third_party/blink/renderer/platform/bindings/wrapper_type_info.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/wtf/forward.h"
#include "third_party/blink/renderer/platform/wtf/text/atomic_string.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"
#include "third_party/blink/renderer/platform/wtf/wtf_size_t.h"
#include "v8/include/v8-container.h"
#include "v8/include/v8-forward.h"
#include "v8/include/v8-function-callback.h"
#include "v8/include/v8-isolate.h"
#include "v8/include/v8-local-handle.h"
#include "v8/include/v8-maybe.h"
#include "v8/include/v8-persistent-handle.h"
#include "v8/include/v8-primitive.h"
#include "v8/include/v8-value.h"

namespace blink {

// This file contains bindings helper functions that do not have dependencies
// to core/ or bindings/core. For core-specific helper functions, see
// bindings/core/v8/v8_binding_for_core.h.

// Convert v8::String to a WTF::String. If the V8 string is not already
// an external string then it is transformed into an external string at this
// point to avoid repeated conversions.
inline String ToCoreString(v8::Isolate* isolate, v8::Local<v8::String> value) {
  return ToBlinkString<String>(isolate, value, kExternalize);
}

inline String ToCoreStringWithNullCheck(v8::Isolate* isolate,
                                        v8::Local<v8::String> value) {
  if (value.IsEmpty() || value->IsNull())
    return String();
  return ToCoreString(isolate, value);
}

inline String ToCoreStringWithUndefinedOrNullCheck(
    v8::Isolate* isolate,
    v8::Local<v8::String> value) {
  if (value.IsEmpty())
    return String();
  return ToCoreString(isolate, value);
}

inline AtomicString ToCoreAtomicString(v8::Isolate* isolate,
                                       v8::Local<v8::String> value) {
  return ToBlinkString<AtomicString>(isolate, value, kExternalize);
}

inline AtomicString ToCoreAtomicString(v8::Isolate* isolate,
                                       v8::Local<v8::Name> value) {
  DCHECK(!value.IsEmpty());
  // TODO(crbug.com/1476064): Support converting `value` when it is a symbol
  // instead of a string.
  if (!value->IsString()) {
    return AtomicString();
  }
  return ToBlinkString<AtomicString>(isolate, value.As<v8::String>(),
                                     kExternalize);
}

// This method will return a null String if the v8::Value does not contain a
// v8::String.  It will not call ToString() on the v8::Value. If you want
// ToString() to be called, please use the TONATIVE_FOR_V8STRINGRESOURCE_*()
// macros instead.
inline String ToCoreStringWithUndefinedOrNullCheck(v8::Isolate* isolate,
                                                   v8::Local<v8::Value> value) {
  if (value.IsEmpty() || !value->IsString())
    return String();
  return ToCoreString(isolate, value.As<v8::String>());
}

// Convert a string to a V8 string.

inline v8::Local<v8::String> V8String(v8::Isolate* isolate,
                                      const String& string) {
  if (string.empty()) {
    return v8::String::Empty(isolate);
  }
  return V8PerIsolateData::From(isolate)->GetStringCache()->V8ExternalString(
      isolate, string.Impl());
}

inline v8::Local<v8::String> V8String(v8::Isolate* isolate,
                                      const AtomicString& string) {
  return V8String(isolate, string.GetString());
}

inline v8::Local<v8::String> V8String(v8::Isolate* isolate,
                                      const StringView& string) {
  DCHECK(isolate);
  if (string.IsNull())
    return v8::String::Empty(isolate);
  if (StringImpl* impl = string.SharedImpl()) {
    return V8PerIsolateData::From(isolate)->GetStringCache()->V8ExternalString(
        isolate, impl);
  }
  if (string.Is8Bit()) {
    base::span<const LChar> chars = string.Span8();
    return v8::String::NewFromOneByte(isolate, chars.data(),
                                      v8::NewStringType::kNormal,
                                      static_cast<int>(chars.size()))
        .ToLocalChecked();
  }
  return v8::String::NewFromTwoByte(isolate, string.SpanUint16().data(),
                                    v8::NewStringType::kNormal,
                                    static_cast<int>(string.length()))
      .ToLocalChecked();
}

// As above, for string literals. The compiler doesn't optimize away the is8Bit
// and sharedImpl checks for string literals in the StringView version.
inline v8::Local<v8::String> V8String(v8::Isolate* isolate,
                                      const char* string) {
  DCHECK(isolate);
  if (!string || string[0] == '\0')
    return v8::String::Empty(isolate);
  return v8::String::NewFromOneByte(
             isolate, reinterpret_cast<const uint8_t*>(string),
             v8::NewStringType::kNormal, static_cast<int>(strlen(string)))
      .ToLocalChecked();
}

inline v8::Local<v8::String> V8String(v8::Isolate* isolate,
                                      const ParkableString& string) {
  if (string.IsNull())
    return v8::String::Empty(isolate);
  return V8PerIsolateData::From(isolate)->GetStringCache()->V8ExternalString(
      isolate, string);
}

inline v8::Local<v8::String> V8AtomicString(v8::Isolate* isolate,
                                            const StringView& string) {
  DCHECK(isolate);
  if (string.Is8Bit()) {
    base::span<const LChar> chars = string.Span8();
    return v8::String::NewFromOneByte(isolate, chars.data(),
                                      v8::NewStringType::kInternalized,
                                      static_cast<int>(chars.size()))
        .ToLocalChecked();
  }
  return v8::String::NewFromTwoByte(isolate, string.SpanUint16().data(),
                                    v8::NewStringType::kInternalized,
                                    static_cast<int>(string.length()))
      .ToLocalChecked();
}

// As above, for string literals. The compiler doesn't optimize away the is8Bit
// check for string literals in the StringView version.
inline v8::Local<v8::String> V8AtomicString(v8::Isolate* isolate,
                                            const char* string) {
  DCHECK(isolate);
  if (!string || string[0] == '\0')
    return v8::String::Empty(isolate);
  return v8::String::NewFromOneByte(
             isolate, reinterpret_cast<const uint8_t*>(string),
             v8::NewStringType::kInternalized, static_cast<int>(strlen(string)))
      .ToLocalChecked();
}

inline bool IsUndefinedOrNull(v8::Local<v8::Value> value) {
  return value.IsEmpty() || value->IsNullOrUndefined();
}
PLATFORM_EXPORT v8::Local<v8::Function> GetBoundFunction(
    v8::Local<v8::Function>);

// Freeze a V8 object. The type of the first parameter and the return value is
// intentionally v8::Value so that this function can wrap ToV8().
// If the argument isn't an object, this will crash.
PLATFORM_EXPORT v8::Local<v8::Value> FreezeV8Object(v8::Local<v8::Value>,
                                                    v8::Isolate*);

// Return values of indexed properties and named properties

enum class IndexedPropertySetterResult {
  kDidNotIntercept,  // Fallback to the default set operation.
  kIntercepted,      // Intercepted regardless of whether it succeeded or not.
};

enum class NamedPropertySetterResult {
  kDidNotIntercept,  // Fallback to the default set operation.
  kIntercepted,      // Intercepted regardless of whether it succeeded or not.
};

enum class NamedPropertyDeleterResult {
  kDidNotIntercept,  // Fallback to the default delete operation.
  kDeleted,          // Successfully deleted.
  kDidNotDelete,     // Intercepted but failed to delete.
};

constexpr v8::Intercepted BlinkInterceptorResultToV8Intercepted(
    IndexedPropertySetterResult value) {
  return value == IndexedPropertySetterResult::kDidNotIntercept
             ? v8::Intercepted::kNo
             : v8::Intercepted::kYes;
}

constexpr v8::Intercepted BlinkInterceptorResultToV8Intercepted(
    NamedPropertySetterResult value) {
  return value == NamedPropertySetterResult::kDidNotIntercept
             ? v8::Intercepted::kNo
             : v8::Intercepted::kYes;
}

constexpr v8::Intercepted BlinkInterceptorResultToV8Intercepted(
    NamedPropertyDeleterResult value) {
  return value == NamedPropertyDeleterResult::kDidNotIntercept
             ? v8::Intercepted::kNo
             : v8::Intercepted::kYes;
}

namespace bindings {

struct V8PropertyDescriptorBag {
 private:
  STACK_ALLOCATED();

 public:
  bool has_enumerable = false;
  bool has_configurable = false;
  bool has_value = false;
  bool has_writable = false;
  bool has_get = false;
  bool has_set = false;

  bool enumerable = false;
  bool configurable = false;
  bool writable = false;
  v8::Local<v8::Value> value;
  v8::Local<v8::Value> get;
  v8::Local<v8::Value> set;
};

// ToPropertyDescriptor
// https://tc39.es/ecma262/#sec-topropertydescriptor
PLATFORM_EXPORT void V8ObjectToPropertyDescriptor(
    v8::Isolate* isolate,
    v8::Local<v8::Value> descriptor_object,
    V8PropertyDescriptorBag& descriptor_bag);

}  // namespace bindings

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_BINDINGS_V8_BINDING_H_
