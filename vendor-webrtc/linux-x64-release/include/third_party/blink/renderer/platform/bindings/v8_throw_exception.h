/*
 * Copyright (C) 2009 Google Inc. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1.  Redistributions of source code must retain the above copyright
 *     notice, this list of conditions and the following disclaimer.
 * 2.  Redistributions in binary form must reproduce the above copyright
 *     notice, this list of conditions and the following disclaimer in the
 *     documentation and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY APPLE INC. AND ITS CONTRIBUTORS ``AS IS'' AND
 * ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED
 * WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
 * DISCLAIMED. IN NO EVENT SHALL APPLE INC. OR ITS CONTRIBUTORS BE LIABLE FOR
 * ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
 * DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
 * SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER
 * CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY,
 * OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_BINDINGS_V8_THROW_EXCEPTION_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_BINDINGS_V8_THROW_EXCEPTION_H_

#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"
#include "v8/include/v8-forward.h"
#include "v8/include/v8-isolate.h"
#include "v8/include/v8-local-handle.h"

namespace blink {

// Provides utility functions to create and/or throw JS built-in errors.
class PLATFORM_EXPORT V8ThrowException {
  STATIC_ONLY(V8ThrowException);

 public:
  static void ThrowException(v8::Isolate* isolate,
                             v8::Local<v8::Value> exception) {
    if (!isolate->IsExecutionTerminating())
      isolate->ThrowException(exception);
  }

  static v8::Local<v8::Value> CreateError(v8::Isolate*, const String& message);
  static v8::Local<v8::Value> CreateRangeError(v8::Isolate*,
                                               const String& message);
  static v8::Local<v8::Value> CreateReferenceError(v8::Isolate*,
                                                   const String& message);
  static v8::Local<v8::Value> CreateSyntaxError(v8::Isolate*,
                                                const String& message);
  static v8::Local<v8::Value> CreateTypeError(v8::Isolate*,
                                              const String& message);
  static v8::Local<v8::Value> CreateWasmCompileError(v8::Isolate*,
                                                     const String& message);
  static v8::Local<v8::Value> CreateWasmLinkError(v8::Isolate*,
                                                  const String& message);
  static v8::Local<v8::Value> CreateWasmRuntimeError(v8::Isolate*,
                                                     const String& message);

  static void ThrowError(v8::Isolate*, const String& message);
  static void ThrowRangeError(v8::Isolate*, const String& message);
  static void ThrowReferenceError(v8::Isolate*, const String& message);
  static void ThrowSyntaxError(v8::Isolate*, const String& message);
  static void ThrowTypeError(v8::Isolate*, const String& message);
  static void ThrowWasmCompileError(v8::Isolate*, const String& message);
  static void ThrowWasmLinkError(v8::Isolate*, const String& message);
  static void ThrowWasmRuntimeError(v8::Isolate*, const String& message);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_BINDINGS_V8_THROW_EXCEPTION_H_
