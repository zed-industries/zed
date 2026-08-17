// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_BINDINGS_V8_SET_RETURN_VALUE_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_BINDINGS_V8_SET_RETURN_VALUE_H_

#include <optional>
#include <type_traits>

#include "third_party/blink/public/platform/web_string.h"
#include "third_party/blink/renderer/platform/bindings/dom_data_store.h"
#include "third_party/blink/renderer/platform/bindings/dom_wrapper_world.h"
#include "third_party/blink/renderer/platform/bindings/enumeration_base.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/bindings/v8_binding.h"
#include "third_party/blink/renderer/platform/bindings/v8_per_isolate_data.h"
#include "third_party/blink/renderer/platform/bindings/v8_value_cache.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "v8/include/v8-function-callback.h"
#include "v8/include/v8.h"

namespace blink::bindings {

// `V8SetReturnValue()` sets a return value in a V8 callback function.  The
// first two arguments are fixed as either `v8::FunctionCallbackInfo<T>` or
// `v8::PropertyCallbackInfo<T>` and the actual return value. The function may
// take more arguments as optimization hints depending on the return value type.

template <template <typename...> class Template, typename T>
struct IsSpecializationOf : std::false_type {};

template <template <typename...> class Template, typename... Args>
struct IsSpecializationOf<Template, Template<Args...>> : std::true_type {};

template <typename T>
concept FunctionCallbackInfoOrPropertyCallbackInfo =
    IsSpecializationOf<v8::FunctionCallbackInfo, T>::value ||
    IsSpecializationOf<v8::PropertyCallbackInfo, T>::value;

struct V8ReturnValue {
  STATIC_ONLY(V8ReturnValue);

  // Support compile-time overload resolution by making each value have its own
  // type.

  // Applies strict typing to IDL primitive types.
  template <typename T>
  struct PrimitiveType {};

  // Nullable or not
  enum NonNullable { kNonNullable };
  enum Nullable { kNullable };

  // Main world or not
  enum MainWorld { kMainWorld };

  // The return value can be a cross origin object.
  enum MaybeCrossOrigin { kMaybeCrossOrigin };

  // Returns the exposed object of the given type.
  enum InterfaceObject { kInterfaceObject };
  enum NamespaceObject { kNamespaceObject };

  // Selects the appropriate receiver from which e.g. the creation context can
  // be retrieved.
  static v8::Local<v8::Object> GetReceiver(
      const v8::FunctionCallbackInfo<v8::Value>& info) {
    return info.This();
  }
  static v8::Local<v8::Object> GetReceiver(
      const v8::PropertyCallbackInfo<v8::Value>& info) {
    return info.Holder();
  }
  // Helper function for ScriptWrappable
  template <FunctionCallbackInfoOrPropertyCallbackInfo CallbackInfo>
  static void SetWrapper(const CallbackInfo& info,
                         ScriptWrappable* wrappable,
                         v8::Local<v8::Context> creation_context) {
    v8::Local<v8::Value> wrapper =
        wrappable->Wrap(ScriptState::From(info.GetIsolate(), creation_context));
    info.GetReturnValue().SetNonEmpty(wrapper);
  }
};

// V8 handle types
template <FunctionCallbackInfoOrPropertyCallbackInfo CallbackInfo,
          typename S,
          typename... ExtraArgs>
void V8SetReturnValue(const CallbackInfo& info,
                      const v8::Local<S> value,
                      ExtraArgs... extra_args) {
  info.GetReturnValue().Set(value);
}

// Property descriptor
PLATFORM_EXPORT v8::Local<v8::Object> CreatePropertyDescriptorObject(
    v8::Isolate* isolate,
    const v8::PropertyDescriptor& desc);

template <FunctionCallbackInfoOrPropertyCallbackInfo CallbackInfo>
void V8SetReturnValue(const CallbackInfo& info,
                      const v8::PropertyDescriptor& value) {
  info.GetReturnValue().Set(
      CreatePropertyDescriptorObject(info.GetIsolate(), value));
}

// Indexed properties and named properties
PLATFORM_EXPORT inline void V8SetReturnValue(
    const v8::FunctionCallbackInfo<v8::Value>& info,
    IndexedPropertySetterResult value) {
  // If an operation implementing indexed property setter is invoked as a
  // regular operation, and the return type is not type void (V8SetReturnValue
  // won't be called in case of type void), then return the given value as is.
  info.GetReturnValue().Set(info[1]);
}

PLATFORM_EXPORT inline void V8SetReturnValue(
    const v8::PropertyCallbackInfo<void>& info,
    IndexedPropertySetterResult value) {
  // Setter callback is not expected to set the return value.
}

PLATFORM_EXPORT inline void V8SetReturnValue(
    const v8::FunctionCallbackInfo<v8::Value>& info,
    NamedPropertySetterResult value) {
  // If an operation implementing named property setter is invoked as a
  // regular operation, and the return type is not type void (V8SetReturnValue
  // won't be called in case of type void), then return the given value as is.
  info.GetReturnValue().Set(info[1]);
}

PLATFORM_EXPORT inline void V8SetReturnValue(
    const v8::PropertyCallbackInfo<void>& info,
    NamedPropertySetterResult value) {
  // Setter callback is not expected to set the return value.
}

PLATFORM_EXPORT inline void V8SetReturnValue(
    const v8::PropertyCallbackInfo<void>& info,
    NamedPropertyDeleterResult value) {}

PLATFORM_EXPORT inline void V8SetReturnValue(
    const v8::PropertyCallbackInfo<v8::Boolean>& info,
    NamedPropertyDeleterResult value) {
  switch (value) {
    case NamedPropertyDeleterResult::kDidNotIntercept:
      // Deleter callback doesn't have to set the return value if the
      // operation was not intercepted.
      return;

    case NamedPropertyDeleterResult::kDidNotDelete:
    case NamedPropertyDeleterResult::kDeleted:
      info.GetReturnValue().Set(value == NamedPropertyDeleterResult::kDeleted);
      return;
  }
  NOTREACHED();
}

// nullptr
template <FunctionCallbackInfoOrPropertyCallbackInfo CallbackInfo>
void V8SetReturnValue(const CallbackInfo& info, std::nullptr_t) {
  info.GetReturnValue().SetNull();
}

// Primitive types
template <FunctionCallbackInfoOrPropertyCallbackInfo CallbackInfo>
void V8SetReturnValue(const CallbackInfo& info, bool value) {
  info.GetReturnValue().Set(value);
}

template <FunctionCallbackInfoOrPropertyCallbackInfo CallbackInfo>
void V8SetReturnValue(const CallbackInfo& info, int16_t value) {
  info.GetReturnValue().Set(value);
}

template <FunctionCallbackInfoOrPropertyCallbackInfo CallbackInfo>
void V8SetReturnValue(const CallbackInfo& info, uint16_t value) {
  info.GetReturnValue().Set(value);
}

template <FunctionCallbackInfoOrPropertyCallbackInfo CallbackInfo>
void V8SetReturnValue(const CallbackInfo& info, int32_t value) {
  info.GetReturnValue().Set(value);
}

template <FunctionCallbackInfoOrPropertyCallbackInfo CallbackInfo>
void V8SetReturnValue(const CallbackInfo& info, uint32_t value) {
  info.GetReturnValue().Set(value);
}

template <FunctionCallbackInfoOrPropertyCallbackInfo CallbackInfo>
void V8SetReturnValue(const CallbackInfo& info, int64_t value) {
  info.GetReturnValue().Set(value);
}

template <FunctionCallbackInfoOrPropertyCallbackInfo CallbackInfo>
void V8SetReturnValue(const CallbackInfo& info, uint64_t value) {
  info.GetReturnValue().Set(value);
}

template <FunctionCallbackInfoOrPropertyCallbackInfo CallbackInfo>
void V8SetReturnValue(const CallbackInfo& info, double value) {
  info.GetReturnValue().Set(value);
}

// Primitive types with IDL type
//
// |IdlType| represents a C++ type corresponding to an IDL type, and |value| is
// passed from Blink implementation and its type occasionally does not match
// the IDL type because Blink is not always respectful to IDL types.  These
// functions fix such a type mismatch.
template <FunctionCallbackInfoOrPropertyCallbackInfo CallbackInfo,
          typename BlinkType,
          typename IdlType>
inline typename std::enable_if_t<std::is_arithmetic<BlinkType>::value ||
                                 std::is_enum<BlinkType>::value>
V8SetReturnValue(const CallbackInfo& info,
                 BlinkType value,
                 V8ReturnValue::PrimitiveType<IdlType>) {
  V8SetReturnValue(info, IdlType(value));
}

template <FunctionCallbackInfoOrPropertyCallbackInfo CallbackInfo,
          typename BlinkType>
inline void V8SetReturnValue(const CallbackInfo& info,
                             BlinkType* value,
                             V8ReturnValue::PrimitiveType<bool>) {
  V8SetReturnValue(info, bool(value));
}

// String types
template <FunctionCallbackInfoOrPropertyCallbackInfo CallbackInfo>
void V8SetReturnValue(const CallbackInfo& info,
                      const AtomicString& string,
                      v8::Isolate* isolate,
                      V8ReturnValue::NonNullable) {
  if (string.empty()) {
    info.GetReturnValue().SetEmptyString();
    return;
  }
  DCHECK(!string.IsNull());  // Null strings are empty.
  V8PerIsolateData::From(isolate)->GetStringCache()->SetReturnValueFromString(
      info.GetReturnValue(), string.Impl());
}

template <FunctionCallbackInfoOrPropertyCallbackInfo CallbackInfo>
void V8SetReturnValue(const CallbackInfo& info,
                      const String& string,
                      v8::Isolate* isolate,
                      V8ReturnValue::NonNullable) {
  if (string.empty()) {
    info.GetReturnValue().SetEmptyString();
    return;
  }
  DCHECK(!string.IsNull());  // Null strings are empty.
  V8PerIsolateData::From(isolate)->GetStringCache()->SetReturnValueFromString(
      info.GetReturnValue(), string.Impl());
}

template <FunctionCallbackInfoOrPropertyCallbackInfo CallbackInfo>
void V8SetReturnValue(const CallbackInfo& info,
                      const WebString& string,
                      v8::Isolate* isolate,
                      V8ReturnValue::NonNullable) {
  if (string.IsEmpty()) {
    info.GetReturnValue().SetEmptyString();
    return;
  }
  DCHECK(!string.IsNull());  // Null strings are empty.
  V8PerIsolateData::From(isolate)->GetStringCache()->SetReturnValueFromString(
      info.GetReturnValue(), static_cast<String>(string).Impl());
}

template <FunctionCallbackInfoOrPropertyCallbackInfo CallbackInfo>
void V8SetReturnValue(const CallbackInfo& info,
                      const AtomicString& string,
                      v8::Isolate* isolate,
                      V8ReturnValue::Nullable) {
  if (string.IsNull()) {
    info.GetReturnValue().SetNull();
    return;
  } else if (string.empty()) {
    info.GetReturnValue().SetEmptyString();
    return;
  }
  V8PerIsolateData::From(isolate)->GetStringCache()->SetReturnValueFromString(
      info.GetReturnValue(), string.Impl());
}

template <FunctionCallbackInfoOrPropertyCallbackInfo CallbackInfo>
void V8SetReturnValue(const CallbackInfo& info,
                      const String& string,
                      v8::Isolate* isolate,
                      V8ReturnValue::Nullable) {
  if (string.IsNull()) {
    info.GetReturnValue().SetNull();
    return;
  } else if (string.empty()) {
    info.GetReturnValue().SetEmptyString();
    return;
  }
  V8PerIsolateData::From(isolate)->GetStringCache()->SetReturnValueFromString(
      info.GetReturnValue(), string.Impl());
}

template <FunctionCallbackInfoOrPropertyCallbackInfo CallbackInfo>
void V8SetReturnValue(const CallbackInfo& info,
                      const WebString& string,
                      v8::Isolate* isolate,
                      V8ReturnValue::Nullable) {
  if (string.IsNull()) {
    info.GetReturnValue().SetNull();
    return;
  } else if (string.IsEmpty()) {
    info.GetReturnValue().SetEmptyString();
    return;
  }
  V8PerIsolateData::From(isolate)->GetStringCache()->SetReturnValueFromString(
      info.GetReturnValue(), static_cast<String>(string).Impl());
}

// ScriptWrappable
template <FunctionCallbackInfoOrPropertyCallbackInfo CallbackInfo>
void V8SetReturnValue(const CallbackInfo& info,
                      const ScriptWrappable* value,
                      V8ReturnValue::MainWorld) {
  DCHECK(DOMWrapperWorld::Current(info.GetIsolate()).IsMainWorld());
  if (!value) [[unlikely]] {
    info.GetReturnValue().SetNull();
    return;
  }
  ScriptWrappable* wrappable = const_cast<ScriptWrappable*>(value);
  if (DOMDataStore::SetReturnValueFromInlineStorage(info.GetReturnValue(),
                                                    wrappable)) {
    return;
  }
  V8ReturnValue::SetWrapper(
      info, wrappable,
      V8ReturnValue::GetReceiver(info)->GetCreationContextChecked(
          info.GetIsolate()));
}

template <FunctionCallbackInfoOrPropertyCallbackInfo CallbackInfo>
void V8SetReturnValue(const CallbackInfo& info,
                      const ScriptWrappable& value,
                      V8ReturnValue::MainWorld) {
  DCHECK(DOMWrapperWorld::Current(info.GetIsolate()).IsMainWorld());
  ScriptWrappable* wrappable = const_cast<ScriptWrappable*>(&value);
  if (DOMDataStore::SetReturnValueFromInlineStorage(info.GetReturnValue(),
                                                    wrappable)) {
    return;
  }
  V8ReturnValue::SetWrapper(
      info, wrappable,
      V8ReturnValue::GetReceiver(info)->GetCreationContextChecked(
          info.GetIsolate()));
}

template <FunctionCallbackInfoOrPropertyCallbackInfo CallbackInfo>
void V8SetReturnValue(const CallbackInfo& info,
                      const ScriptWrappable* value,
                      const ScriptWrappable* receiver) {
  if (!value) [[unlikely]] {
    return info.GetReturnValue().SetNull();
  }
  ScriptWrappable* wrappable = const_cast<ScriptWrappable*>(value);
  if (DOMDataStore::SetReturnValueFast(info.GetReturnValue(), wrappable,
                                       V8ReturnValue::GetReceiver(info),
                                       receiver)) {
    return;
  }
  V8ReturnValue::SetWrapper(
      info, wrappable,
      V8ReturnValue::GetReceiver(info)->GetCreationContextChecked(
          info.GetIsolate()));
}

template <FunctionCallbackInfoOrPropertyCallbackInfo CallbackInfo>
void V8SetReturnValue(const CallbackInfo& info,
                      const ScriptWrappable& value,
                      const ScriptWrappable* receiver) {
  ScriptWrappable* wrappable = const_cast<ScriptWrappable*>(&value);
  if (DOMDataStore::SetReturnValueFast(info.GetReturnValue(), wrappable,
                                       V8ReturnValue::GetReceiver(info),
                                       receiver)) {
    return;
  }
  V8ReturnValue::SetWrapper(
      info, wrappable,
      V8ReturnValue::GetReceiver(info)->GetCreationContextChecked(
          info.GetIsolate()));
}

template <FunctionCallbackInfoOrPropertyCallbackInfo CallbackInfo>
void V8SetReturnValue(const CallbackInfo& info,
                      const ScriptWrappable* value,
                      const ScriptWrappable* receiver,
                      V8ReturnValue::MaybeCrossOrigin) {
  if (!value) [[unlikely]] {
    return info.GetReturnValue().SetNull();
  }
  ScriptWrappable* wrappable = const_cast<ScriptWrappable*>(value);
  if (DOMDataStore::SetReturnValueFast(info.GetReturnValue(), wrappable,
                                       V8ReturnValue::GetReceiver(info),
                                       receiver)) {
    return;
  }
  // Check whether the creation context is available, and if not, use the
  // current context. When a cross-origin Window is associated with a
  // v8::Context::NewRemoteContext(), there is no creation context in the usual
  // sense. It's ok to use the current context in that case because:
  // 1) The Window objects must have their own creation context and must never
  //    need a creation context to be specified.
  // 2) Even though a v8::Context is not necessary in case
  //    of Window objects, v8::Isolate and DOMWrapperWorld are still necessary
  //    to create an appropriate wrapper object, and the ScriptState associated
  //    with the current context will still have the correct v8::Isolate and
  //    DOMWrapperWorld.
  v8::Local<v8::Context> context;
  if (!V8ReturnValue::GetReceiver(info)->GetCreationContext().ToLocal(
          &context)) {
    context = info.GetIsolate()->GetCurrentContext();
  }
  V8ReturnValue::SetWrapper(info, wrappable, context);
}

template <FunctionCallbackInfoOrPropertyCallbackInfo CallbackInfo>
void V8SetReturnValue(const CallbackInfo& info,
                      const ScriptWrappable& value,
                      const ScriptWrappable* receiver,
                      V8ReturnValue::MaybeCrossOrigin) {
  ScriptWrappable* wrappable = const_cast<ScriptWrappable*>(&value);
  if (DOMDataStore::SetReturnValueFast(info.GetReturnValue(), wrappable,
                                       V8ReturnValue::GetReceiver(info),
                                       receiver)) {
    return;
  }
  // Check whether the creation context is available, and if not, use the
  // current context. When a cross-origin Window is associated with a
  // v8::Context::NewRemoteContext(), there is no creation context in the usual
  // sense. It's ok to use the current context in that case because:
  // 1) The Window objects must have their own creation context and must never
  //    need a creation context to be specified.
  // 2) Even though a v8::Context is not necessary in case
  //    of Window objects, v8::Isolate and DOMWrapperWorld are still necessary
  //    to create an appropriate wrapper object, and the ScriptState associated
  //    with the current context will still have the correct v8::Isolate and
  //    DOMWrapperWorld.
  v8::Local<v8::Context> context;
  if (!V8ReturnValue::GetReceiver(info)->GetCreationContext().ToLocal(
          &context)) {
    context = info.GetIsolate()->GetCurrentContext();
  }
  V8ReturnValue::SetWrapper(info, wrappable, context);
}

// This `current_context` variant is for static operations where there is no
// receiver object.
template <FunctionCallbackInfoOrPropertyCallbackInfo CallbackInfo>
void V8SetReturnValue(const CallbackInfo& info,
                      const ScriptWrappable* value,
                      v8::Local<v8::Context> current_context) {
  if (!value) [[unlikely]] {
    return info.GetReturnValue().SetNull();
  }
  ScriptWrappable* wrappable = const_cast<ScriptWrappable*>(value);
  if (DOMDataStore::SetReturnValue(info.GetReturnValue(), wrappable,
                                   current_context)) {
    return;
  }
  V8ReturnValue::SetWrapper(info, wrappable, current_context);
}

// EnumerationBase
template <FunctionCallbackInfoOrPropertyCallbackInfo CallbackInfo,
          typename... ExtraArgs>
void V8SetReturnValue(const CallbackInfo& info,
                      const bindings::EnumerationBase& value,
                      v8::Isolate* isolate,
                      ExtraArgs... extra_args) {
  info.GetReturnValue().Set(V8AtomicString(isolate, value.AsCStr()));
}

// Nullable types
template <FunctionCallbackInfoOrPropertyCallbackInfo CallbackInfo,
          typename T,
          typename... ExtraArgs>
void V8SetReturnValue(const CallbackInfo& info,
                      std::optional<T> value,
                      ExtraArgs... extra_args) {
  if (value.has_value()) {
    V8SetReturnValue(info, value.value(),
                     std::forward<ExtraArgs>(extra_args)...);
  } else {
    info.GetReturnValue().SetNull();
  }
}

// Exposed objects
PLATFORM_EXPORT v8::Local<v8::Value> GetExposedInterfaceObject(
    v8::Isolate* isolate,
    v8::Local<v8::Object> creation_context,
    const WrapperTypeInfo* wrapper_type_info);

PLATFORM_EXPORT v8::Local<v8::Value> GetExposedNamespaceObject(
    v8::Isolate* isolate,
    v8::Local<v8::Object> creation_context,
    const WrapperTypeInfo* wrapper_type_info);

inline void V8SetReturnValue(const v8::PropertyCallbackInfo<v8::Value>& info,
                             const WrapperTypeInfo* wrapper_type_info,
                             V8ReturnValue::InterfaceObject) {
  info.GetReturnValue().Set(GetExposedInterfaceObject(
      info.GetIsolate(), info.Holder(), wrapper_type_info));
}

inline void V8SetReturnValue(const v8::PropertyCallbackInfo<v8::Value>& info,
                             const WrapperTypeInfo* wrapper_type_info,
                             V8ReturnValue::NamespaceObject) {
  info.GetReturnValue().Set(GetExposedNamespaceObject(
      info.GetIsolate(), info.Holder(), wrapper_type_info));
}

}  // namespace blink::bindings

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_BINDINGS_V8_SET_RETURN_VALUE_H_
