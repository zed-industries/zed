// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_BINDINGS_V8_CROSS_ORIGIN_PROPERTY_SUPPORT_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_BINDINGS_V8_CROSS_ORIGIN_PROPERTY_SUPPORT_H_

#include "base/containers/span.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/wtf/text/string_view.h"
#include "v8/include/v8.h"

// This file provides utilities to support implementation of cross origin
// properties in generated bindings code.  Should be used only in generated
// bindings code.

namespace blink {
struct WrapperTypeInfo;

namespace bindings {

struct CrossOriginAttributeTableEntry final {
  const char* name;
  v8::FunctionCallback get_callback;
  v8::FunctionCallback set_callback;
  v8::NamedPropertyGetterCallback get_value;
  v8::NamedPropertySetterCallback set_value;
};

struct CrossOriginOperationTableEntry final {
  const char* name;
  v8::FunctionCallback callback;
  int func_length;
};

PLATFORM_EXPORT v8::MaybeLocal<v8::Function> GetCrossOriginFunction(
    v8::Isolate* isolate,
    const StringView& func_name,
    v8::FunctionCallback callback,
    int func_length,
    const WrapperTypeInfo* wrapper_type_info,
    v8::ExceptionContext exception_context,
    const char* interface_name);

PLATFORM_EXPORT v8::MaybeLocal<v8::Value> GetCrossOriginGetterSetter(
    v8::Isolate* isolate,
    const StringView& func_name,
    v8::FunctionCallback callback,
    int func_length,
    const WrapperTypeInfo* wrapper_type_info,
    v8::ExceptionContext exception_context,
    const char* interface_name);

// HTML 7.2.3.2 CrossOriginPropertyFallback ( P )
// https://html.spec.whatwg.org/C/#crossoriginpropertyfallback-(-p-)
PLATFORM_EXPORT bool IsSupportedInCrossOriginPropertyFallback(
    v8::Isolate* isolate,
    v8::Local<v8::Name> property_name);

// HTML 7.2.3.7 CrossOriginOwnPropertyKeys ( O )
// https://html.spec.whatwg.org/C/#crossoriginownpropertykeys-(-o-)
PLATFORM_EXPORT v8::Local<v8::Array> EnumerateCrossOriginProperties(
    v8::Isolate* isolate,
    base::span<const CrossOriginAttributeTableEntry> attributes,
    base::span<const CrossOriginOperationTableEntry> operations);

}  // namespace bindings

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_BINDINGS_V8_CROSS_ORIGIN_PROPERTY_SUPPORT_H_
