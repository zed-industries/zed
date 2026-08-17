/*
 * Copyright (C) 2009 Google Inc. All rights reserved.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_BINDINGS_V8_DOM_ACTIVITY_LOGGER_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_BINDINGS_V8_DOM_ACTIVITY_LOGGER_H_

#include <memory>

#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"
#include "v8/include/v8.h"

namespace blink {

class ExecutionContext;
class KURL;
class ScriptState;

class PLATFORM_EXPORT V8DOMActivityLogger {
  USING_FAST_MALLOC(V8DOMActivityLogger);

 public:
  virtual ~V8DOMActivityLogger() = default;

  virtual void LogGetter(ScriptState* script_state, const String& api_name) {}
  virtual void LogSetter(ScriptState* script_state,
                         const String& api_name,
                         const v8::Local<v8::Value>& new_value) {}
  virtual void LogMethod(ScriptState* script_state,
                         const String& api_name,
                         base::span<const v8::Local<v8::Value>> args) {}
  virtual void LogEvent(ExecutionContext* execution_context,
                        const String& event_name,
                        base::span<const String> args) {}

  void LogMethod(ScriptState* script_state,
                 const char* api_name,
                 v8::FunctionCallbackInfo<v8::Value>);

  // Associates a logger with the world identified by worldId (worlId may be 0
  // identifying the main world) and extension ID. Extension ID is used to
  // identify a logger for main world only (worldId == 0). If the world is not
  // a main world, an extension ID is ignored.
  //
  // A renderer process may host multiple extensions when the browser hits the
  // renderer process limit. In such case, we assign multiple extensions to
  // the same main world of a renderer process. In order to distinguish the
  // extensions and their activity loggers in the main world, we require an
  // extension ID. Otherwise, extension activities may be logged under
  // a wrong extension ID.
  static void SetActivityLogger(int world_id,
                                const String&,
                                std::unique_ptr<V8DOMActivityLogger>);
  static V8DOMActivityLogger* ActivityLogger(int world_id,
                                             const String& extension_id);
  static V8DOMActivityLogger* ActivityLogger(int world_id, const KURL&);

  // Returns activity logger for current V8 context or 0.
  static V8DOMActivityLogger* CurrentActivityLogger(v8::Isolate* isolate);
  // Returns activity logger for current V8 context if the context belongs to
  // an isolated world or 0.
  static V8DOMActivityLogger* CurrentActivityLoggerIfIsolatedWorld(
      v8::Isolate* isolate);
  static bool HasActivityLoggerInIsolatedWorlds();
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_BINDINGS_V8_DOM_ACTIVITY_LOGGER_H_
