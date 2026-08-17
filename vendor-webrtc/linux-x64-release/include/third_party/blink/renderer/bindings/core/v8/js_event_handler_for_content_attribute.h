// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_BINDINGS_CORE_V8_JS_EVENT_HANDLER_FOR_CONTENT_ATTRIBUTE_H_
#define THIRD_PARTY_BLINK_RENDERER_BINDINGS_CORE_V8_JS_EVENT_HANDLER_FOR_CONTENT_ATTRIBUTE_H_

#include "third_party/blink/renderer/bindings/core/v8/js_event_handler.h"
#include "third_party/blink/renderer/platform/bindings/dom_wrapper_world.h"
#include "third_party/blink/renderer/platform/wtf/text/text_position.h"

namespace blink {

class QualifiedName;

// |JSEventHandlerForContentAttribute| supports lazy compilation for content
// attribute. This performs in the same way as |JSEventHandler| after it gets
// compiled.
class JSEventHandlerForContentAttribute final : public JSEventHandler {
 public:
  static JSEventHandlerForContentAttribute* Create(
      ExecutionContext*,
      const QualifiedName&,
      const AtomicString& value,
      JSEventHandler::HandlerType = JSEventHandler::HandlerType::kEventHandler);
  JSEventHandlerForContentAttribute(ExecutionContext*,
                                    const QualifiedName&,
                                    const AtomicString& value,
                                    JSEventHandler::HandlerType);

  // blink::EventListener overrides:
  bool IsEventHandlerForContentAttribute() const override { return true; }

  // blink::JSBasedEventListener overrides:
  v8::Local<v8::Value> GetListenerObject(EventTarget&) override;
  std::unique_ptr<SourceLocation> GetSourceLocation(EventTarget&) override;

  const String& ScriptBody() const override { return script_body_; }

 protected:
  // blink::JSBasedEventListener override:
  v8::Isolate* GetIsolate() const override { return isolate_; }
  ScriptState* GetScriptState() const override {
    DCHECK(HasCompiledHandler());
    return JSEventHandler::GetScriptState();
  }

  // An assumption here is that the content attributes are used only in the main
  // world or the isolated world for the content scripts, they are never used in
  // other isolated worlds nor worker/worklets.
  // In case of the content scripts, Blink runs script in the main world instead
  // of the isolated world for the content script by design.
  DOMWrapperWorld& GetWorld() const override {
    return DOMWrapperWorld::MainWorld(isolate_);
  }

 private:
  // Implements Step 3. of "get the current value of the event handler".
  // The compiled v8::Function is returned and |JSEventHandler::event_handler_|
  // gets initialized with it if lazy compilation succeeds.
  // Otherwise, v8::Null is returned.
  // https://html.spec.whatwg.org/C/#getting-the-current-value-of-the-event-handler
  v8::Local<v8::Value> GetCompiledHandler(EventTarget&);

  // Lazy compilation for content attribute should be tried only once, but we
  // cannot see whether it had never tried to compile or it has already failed
  // when |HasCompiledHandler()| returns false. |did_compile_| is used for
  // checking that.
  bool did_compile_;
  const AtomicString function_name_;
  String script_body_;
  String source_url_;
  TextPosition position_;
  v8::Isolate* isolate_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_BINDINGS_CORE_V8_JS_EVENT_HANDLER_FOR_CONTENT_ATTRIBUTE_H_
