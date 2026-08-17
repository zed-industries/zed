/*
 * Copyright (C) 2009 Google Inc. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY APPLE INC. AND ITS CONTRIBUTORS ``AS IS''
 * AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO,
 * THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR
 * PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL APPLE INC. OR ITS CONTRIBUTORS
 * BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR
 * CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF
 * SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS
 * INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN
 * CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE)
 * ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF
 * THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_BINDINGS_CORE_V8_V8_SCRIPT_RUNNER_H_
#define THIRD_PARTY_BLINK_RENDERER_BINDINGS_CORE_V8_V8_SCRIPT_RUNNER_H_

#include <optional>

#include "third_party/blink/renderer/bindings/core/v8/sanitize_script_errors.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/forward.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"
#include "v8/include/v8.h"

namespace WTF {
class TextPosition;
}

namespace blink {

class ClassicScript;
class ExecutionContext;
class ModuleScript;
class ModuleScriptCreationParams;
class ReferrerScriptInfo;
class ScriptEvaluationResult;
class ScriptState;

enum class ExecuteScriptPolicy {
  kExecuteScriptWhenScriptsDisabled,
  kDoNotExecuteScriptWhenScriptsDisabled
};

class CORE_EXPORT V8ScriptRunner final {
  STATIC_ONLY(V8ScriptRunner);

 public:
  // Rethrow errors flag in
  // https://html.spec.whatwg.org/C/#run-a-classic-script
  // implemented by CompileAndRunScript() and
  // https://html.spec.whatwg.org/C/#run-a-module-script
  // implemented by EvaluateModule().
  class RethrowErrorsOption final {
    STACK_ALLOCATED();

   public:
    RethrowErrorsOption(RethrowErrorsOption&&) = default;
    RethrowErrorsOption& operator=(RethrowErrorsOption&&) = default;

    RethrowErrorsOption(const RethrowErrorsOption&) = delete;
    RethrowErrorsOption& operator=(const RethrowErrorsOption&) = delete;

    // Rethrow errors flag is false.
    static RethrowErrorsOption DoNotRethrow() {
      return RethrowErrorsOption(std::nullopt);
    }

    // Rethrow errors flag is true.
    // When an exception is to be rethrown,
    // For classic scripts:
    //    The exception is rethrown to V8, and ScriptEvaluationResult doesn't
    //    retain the exception.
    //    When script's muted errors is true, a NetworkError with
    //    `message` is thrown. This is used only for importScripts(), and
    //    `message` is used to throw NetworkErrors with the same message text,
    //    no matter whether the NetworkError is thrown inside or outside
    //    V8ScriptRunner.
    // For module scripts:
    //    The exception is caught and
    //    ScriptEvaluationResult::GetExceptionForModule() returns the exception
    //    to be rethrown.
    static RethrowErrorsOption Rethrow(const String& message) {
      return RethrowErrorsOption(message);
    }

    bool ShouldRethrow() const { return static_cast<bool>(message_); }
    String Message() const { return *message_; }

   private:
    explicit RethrowErrorsOption(std::optional<String> message)
        : message_(std::move(message)) {}

    // `nullopt` <=> rethrow errors is false.
    std::optional<String> message_;
  };

  // For the following methods, the caller sites have to hold
  // a HandleScope and a ContextScope.
  static v8::MaybeLocal<v8::Script> CompileScript(
      ScriptState*,
      const ClassicScript&,
      v8::ScriptOrigin,
      v8::ScriptCompiler::CompileOptions,
      v8::ScriptCompiler::NoCacheReason,
      bool can_use_crowdsourced_compile_hints = false);
  static v8::MaybeLocal<v8::Module> CompileModule(
      v8::Isolate*,
      const ModuleScriptCreationParams&,
      const WTF::TextPosition&,
      v8::ScriptCompiler::CompileOptions,
      v8::ScriptCompiler::NoCacheReason,
      const ReferrerScriptInfo&);
  static ScriptEvaluationResult CompileAndRunScript(ScriptState*,
                                                    ClassicScript*,
                                                    ExecuteScriptPolicy,
                                                    RethrowErrorsOption);
  static v8::MaybeLocal<v8::Value> CallAsConstructor(
      v8::Isolate*,
      v8::Local<v8::Object>,
      ExecutionContext*,
      int argc = 0,
      v8::Local<v8::Value> argv[] = nullptr);
  static v8::MaybeLocal<v8::Value> CallFunction(v8::Local<v8::Function>,
                                                ExecutionContext*,
                                                v8::Local<v8::Value> receiver,
                                                int argc,
                                                v8::Local<v8::Value> argv[],
                                                v8::Isolate*);

  // https://html.spec.whatwg.org/C/#run-a-module-script
  // Callers must enter a v8::HandleScope before calling.
  // See the class comments of RethrowErrorsOption and ScriptEvaluationResult
  // for exception handling and return value semantics.
  static ScriptEvaluationResult EvaluateModule(ModuleScript*,
                                               RethrowErrorsOption);

  // Only to be used from ModuleRecord::ReportException().
  static void ReportExceptionForModule(v8::Isolate*,
                                       v8::Local<v8::Value> exception,
                                       const String& file_name,
                                       const WTF::TextPosition&);

  // Reports an exception to the message handler, as if it were an uncaught
  // exception. Can only be called on the main thread.
  //
  // TODO(adamk): This should live on V8ThrowException, but it depends on
  // V8Initializer and so can't trivially move to platform/bindings.
  static void ReportException(v8::Isolate*, v8::Local<v8::Value> exception);

 private:
  static v8::MaybeLocal<v8::Value> RunCompiledScript(
      v8::Isolate*,
      v8::Local<v8::Script>,
      v8::Local<v8::Data> host_defined_options,
      ExecutionContext*);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_BINDINGS_CORE_V8_V8_SCRIPT_RUNNER_H_
