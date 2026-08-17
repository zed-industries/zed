// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_BINDINGS_CORE_V8_JS_EVENT_LISTENER_H_
#define THIRD_PARTY_BLINK_RENDERER_BINDINGS_CORE_V8_JS_EVENT_LISTENER_H_

#include "third_party/blink/renderer/bindings/core/v8/js_based_event_listener.h"
#include "third_party/blink/renderer/bindings/core/v8/v8_event_listener.h"
#include "third_party/blink/renderer/core/core_export.h"

namespace blink {

// |JSEventListener| implements EventListener in the DOM standard.
// https://dom.spec.whatwg.org/#callbackdef-eventlistener
class CORE_EXPORT JSEventListener final : public JSBasedEventListener {
 public:
  static JSEventListener* CreateOrNull(V8EventListener* listener) {
    return listener ? MakeGarbageCollected<JSEventListener>(listener) : nullptr;
  }

  explicit JSEventListener(V8EventListener* listener)
      : event_listener_(listener) {}

  void Trace(Visitor*) const override;

  // blink::EventListener overrides:
  //
  // Check the identity of |V8EventListener::callback_object_|. There can be
  // multiple CallbackInterfaceBase objects that have the same
  // |CallbackInterfaceBase::callback_object_| but have different
  // |CallbackInterfaceBase::incumbent_script_state_|s.
  bool Matches(const EventListener& other) const override {
    const auto* other_listener = DynamicTo<JSEventListener>(other);
    return other_listener && event_listener_->HasTheSameCallbackObject(
                                 *other_listener->event_listener_);
  }

  // blink::JSBasedEventListener overrides:
  // TODO(crbug.com/881688): remove empty check for this method. This method
  // should return v8::Object or v8::Null.
  v8::Local<v8::Value> GetListenerObject(EventTarget&) override {
    return event_listener_->CallbackObject();
  }
  v8::Local<v8::Value> GetEffectiveFunction(EventTarget&) override;

  // Helper functions for DowncastTraits.
  bool IsJSEventListener() const override { return true; }

 protected:
  // blink::JSBasedEventListener overrides:
  v8::Isolate* GetIsolate() const override {
    return event_listener_->GetIsolate();
  }
  ScriptState* GetScriptState() const override {
    return event_listener_->CallbackRelevantScriptState();
  }
  ScriptState* GetScriptStateOrReportError(
      const char* operation) const override {
    return event_listener_->CallbackRelevantScriptStateOrReportError(
        "EventListener", operation);
  }
  DOMWrapperWorld& GetWorld() const override {
    return event_listener_->GetWorld();
  }

 private:
  // blink::JSBasedEventListener override:
  void InvokeInternal(EventTarget&,
                      Event&,
                      v8::Local<v8::Value> js_event) override;

  const Member<V8EventListener> event_listener_;
};

template <>
struct DowncastTraits<JSEventListener> {
  static bool AllowFrom(const EventListener& event_listener) {
    auto* js_based = DynamicTo<JSBasedEventListener>(event_listener);
    return js_based && js_based->IsJSEventListener();
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_BINDINGS_CORE_V8_JS_EVENT_LISTENER_H_
