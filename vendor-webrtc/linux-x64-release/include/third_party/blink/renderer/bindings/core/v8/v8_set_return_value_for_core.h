// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_BINDINGS_CORE_V8_V8_SET_RETURN_VALUE_FOR_CORE_H_
#define THIRD_PARTY_BLINK_RENDERER_BINDINGS_CORE_V8_V8_SET_RETURN_VALUE_FOR_CORE_H_

#include "third_party/blink/renderer/bindings/core/v8/js_event_handler.h"
#include "third_party/blink/renderer/bindings/core/v8/native_value_traits_impl.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise.h"
#include "third_party/blink/renderer/bindings/core/v8/script_value.h"
#include "third_party/blink/renderer/platform/bindings/v8_set_return_value.h"

namespace blink {

namespace bindings {

// ScriptValue
template <typename CallbackInfo>
void V8SetReturnValue(const CallbackInfo& info, const ScriptValue& value) {
  V8SetReturnValue(info, value.V8Value());
}

// ScriptPromise
template <typename CallbackInfo, typename T>
void V8SetReturnValue(const CallbackInfo& info, const ScriptPromise<T>& value) {
  V8SetReturnValue(info, value.V8Promise());
}

// EventListener
template <typename CallbackInfo>
void V8SetReturnValue(const CallbackInfo& info,
                      const EventListener* value,
                      v8::Isolate* isolate,
                      EventTarget* event_target) {
  EventListener* event_listener = const_cast<EventListener*>(value);
  info.GetReturnValue().Set(
      JSEventHandler::AsV8Value(isolate, event_target, event_listener));
}

}  // namespace bindings

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_BINDINGS_CORE_V8_V8_SET_RETURN_VALUE_FOR_CORE_H_
