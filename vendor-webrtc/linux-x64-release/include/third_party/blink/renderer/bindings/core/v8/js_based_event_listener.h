// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_BINDINGS_CORE_V8_JS_BASED_EVENT_LISTENER_H_
#define THIRD_PARTY_BLINK_RENDERER_BINDINGS_CORE_V8_JS_BASED_EVENT_LISTENER_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/dom/events/event_listener.h"
#include "third_party/blink/renderer/platform/bindings/v8_private_property.h"
#include "third_party/blink/renderer/platform/wtf/casting.h"
#include "v8/include/v8.h"

namespace blink {

class DOMWrapperWorld;
class Event;
class EventTarget;
class ScriptState;
class SourceLocation;

// |JSBasedEventListener| is the base class for JS-based event listeners,
// i.e. EventListener and EventHandler in the standards.
// This provides the essential APIs of JS-based event listeners and also
// implements the common features.
class CORE_EXPORT JSBasedEventListener : public EventListener {
 public:
  ~JSBasedEventListener() override;

  // blink::EventListener overrides:
  bool BelongsToTheCurrentWorld(ExecutionContext*) const final;
  // Implements step 2. of "inner invoke".
  // See: https://dom.spec.whatwg.org/#concept-event-listener-inner-invoke
  void Invoke(ExecutionContext*, Event*) final;

  // |GetListenerObject()| and |GetEffectiveFunction()| may cause JS in the
  // content attribute to get compiled, potentially unsuccessfully.
  //
  // Implements "get the current value of the event handler".
  // https://html.spec.whatwg.org/C/#getting-the-current-value-of-the-event-handler
  // Returns v8::Null with firing error event instead of throwing an exception
  // on failing to compile the uncompiled script body in eventHandler's value.
  // Also, this can return empty because of crbug.com/881688 .
  virtual v8::Local<v8::Value> GetListenerObject(EventTarget&) = 0;

  // Returns v8::Function that handles invoked event or v8::Undefined without
  // throwing any exception.
  virtual v8::Local<v8::Value> GetEffectiveFunction(EventTarget&) = 0;

  // Only DevTools is allowed to use this method.
  DOMWrapperWorld& GetWorldForInspector() const { return GetWorld(); }

  virtual std::unique_ptr<SourceLocation> GetSourceLocation(EventTarget&);

  // Helper functions for DowncastTraits.
  bool IsJSBasedEventListener() const override { return true; }
  virtual bool IsJSEventListener() const { return false; }
  virtual bool IsJSEventHandler() const { return false; }

 protected:
  JSBasedEventListener();

  virtual v8::Isolate* GetIsolate() const = 0;
  // Returns the ScriptState of the relevant realm of the callback object.
  // Must be used only when it's sure that the callback object is the same
  // origin-domain.
  virtual ScriptState* GetScriptState() const = 0;
  // Returns the ScriptState of the relevant realm of the callback object iff
  // the callback is the same origin-domain. Otherwise, reports the error and
  // returns nullptr.
  virtual ScriptState* GetScriptStateOrReportError(
      const char* operation) const = 0;
  virtual DOMWrapperWorld& GetWorld() const = 0;

 private:
  // Performs "call a user object's operation", required in "inner-invoke".
  // "The event handler processing algorithm" corresponds to this in the case of
  // EventHandler.
  // This may throw an exception on invoking the listener.
  // See step 2-10:
  // https://dom.spec.whatwg.org/#concept-event-listener-inner-invoke
  virtual void InvokeInternal(EventTarget&,
                              Event&,
                              v8::Local<v8::Value> js_event) = 0;
};

template <>
struct DowncastTraits<JSBasedEventListener> {
  static bool AllowFrom(const EventListener& event_listener) {
    return event_listener.IsJSBasedEventListener();
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_BINDINGS_CORE_V8_JS_BASED_EVENT_LISTENER_H_
