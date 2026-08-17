// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_BINDINGS_CORE_V8_JS_EVENT_HANDLER_H_
#define THIRD_PARTY_BLINK_RENDERER_BINDINGS_CORE_V8_JS_EVENT_HANDLER_H_

#include "third_party/blink/renderer/bindings/core/v8/js_based_event_listener.h"
#include "third_party/blink/renderer/bindings/core/v8/v8_event_handler_non_null.h"
#include "third_party/blink/renderer/core/core_export.h"

namespace blink {

// |JSEventHandler| implements EventHandler in the HTML standard.
// https://html.spec.whatwg.org/C/#event-handler-attributes
class CORE_EXPORT JSEventHandler : public JSBasedEventListener {
 public:
  enum class HandlerType {
    kEventHandler,
    // For kOnErrorEventHandler
    // https://html.spec.whatwg.org/C/#onerroreventhandler
    kOnErrorEventHandler,
    // For OnBeforeUnloadEventHandler
    // https://html.spec.whatwg.org/C/#onbeforeunloadeventhandler
    kOnBeforeUnloadEventHandler,
  };

  // TODO(bindings): Consider to remove these two helper functions.  These are
  // only used by generated bindings code (OnxxxAttribute{Getter,Setter}), and
  // it should be implemented based on V8EventHandlerNonNull.
  static JSEventHandler* CreateOrNull(v8::Local<v8::Value>, HandlerType);
  static v8::Local<v8::Value> AsV8Value(v8::Isolate* isolate,
                                        EventTarget* event_target,
                                        EventListener* listener) {
    if (JSEventHandler* event_handler = DynamicTo<JSEventHandler>(listener)) {
      return event_handler->GetListenerObject(*event_target);
    }
    return v8::Null(isolate);
  }

  explicit JSEventHandler(V8EventHandlerNonNull* event_handler,
                          HandlerType type)
      : event_handler_(event_handler), type_(type) {}

  void Trace(Visitor* visitor) const override;

  // blink::EventListener overrides:
  bool IsEventHandler() const final { return true; }
  bool Matches(const EventListener& other) const override {
    return this == &other;
  }

  // blink::JSBasedEventListener overrides:
  // TODO(crbug.com/881688): remove empty check for this method. This method
  // should return v8::Object or v8::Null.
  v8::Local<v8::Value> GetListenerObject(EventTarget&) override {
    return event_handler_->CallbackObject();
  }
  v8::Local<v8::Value> GetEffectiveFunction(EventTarget&) override;

  // Helper functions for DowncastTraits.
  bool IsJSEventHandler() const override { return true; }

 protected:
  explicit JSEventHandler(HandlerType type) : type_(type) {}

  // blink::JSBasedEventListener override:
  v8::Isolate* GetIsolate() const override {
    return event_handler_->GetIsolate();
  }
  ScriptState* GetScriptState() const override {
    return event_handler_->CallbackRelevantScriptState();
  }
  ScriptState* GetScriptStateOrReportError(
      const char* operation) const override {
    return event_handler_->CallbackRelevantScriptStateOrReportError(
        "EventHandler", operation);
  }
  DOMWrapperWorld& GetWorld() const override {
    return event_handler_->GetWorld();
  }

  // Initializes |event_handler_| with |listener|. This method must be used only
  // when content attribute gets lazily compiled.
  void SetCompiledHandler(ScriptState* incumbent_script_state,
                          v8::Local<v8::Function> listener);

  bool HasCompiledHandler() const { return event_handler_ != nullptr; }

  // For checking special types of EventHandler.
  bool IsOnErrorEventHandler() const {
    return type_ == HandlerType::kOnErrorEventHandler;
  }
  bool IsOnBeforeUnloadEventHandler() const {
    return type_ == HandlerType::kOnBeforeUnloadEventHandler;
  }

 private:
  // blink::JSBasedEventListener override:
  // Performs "The event handler processing algorithm"
  // https://html.spec.whatwg.org/C/#the-event-handler-processing-algorithm
  void InvokeInternal(EventTarget&,
                      Event&,
                      v8::Local<v8::Value> js_event) override;

  Member<V8EventHandlerNonNull> event_handler_;
  const HandlerType type_;
};

template <>
struct DowncastTraits<JSEventHandler> {
  static bool AllowFrom(const EventListener& event_listener) {
    if (const JSBasedEventListener* js_based_event_listener =
            DynamicTo<JSBasedEventListener>(event_listener)) {
      return js_based_event_listener->IsJSEventHandler();
    }
    return false;
  }
  static bool AllowFrom(const JSBasedEventListener& event_listener) {
    return event_listener.IsJSEventHandler();
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_BINDINGS_CORE_V8_JS_EVENT_HANDLER_H_
