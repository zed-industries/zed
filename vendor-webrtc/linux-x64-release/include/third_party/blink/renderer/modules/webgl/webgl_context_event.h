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

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_WEBGL_WEBGL_CONTEXT_EVENT_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_WEBGL_WEBGL_CONTEXT_EVENT_H_

#include "third_party/blink/renderer/bindings/modules/v8/v8_webgl_context_event_init.h"
#include "third_party/blink/renderer/modules/event_modules.h"

namespace blink {

class WebGLContextEvent final : public Event {
  DEFINE_WRAPPERTYPEINFO();

 public:
  static WebGLContextEvent* Create() {
    return MakeGarbageCollected<WebGLContextEvent>();
  }
  static WebGLContextEvent* Create(const AtomicString& type,
                                   const String& status_message) {
    return MakeGarbageCollected<WebGLContextEvent>(type, status_message);
  }
  static WebGLContextEvent* Create(const AtomicString& type,
                                   const WebGLContextEventInit* initializer) {
    return MakeGarbageCollected<WebGLContextEvent>(type, initializer);
  }

  WebGLContextEvent();
  WebGLContextEvent(const AtomicString& type, const String& status_message);
  WebGLContextEvent(const AtomicString&, const WebGLContextEventInit*);
  ~WebGLContextEvent() override;

  const String& statusMessage() const { return status_message_; }

  const AtomicString& InterfaceName() const override;

  void Trace(Visitor*) const override;

 private:
  String status_message_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_WEBGL_WEBGL_CONTEXT_EVENT_H_
