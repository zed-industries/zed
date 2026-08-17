/*
 * Copyright (c) 2011 Google Inc. All rights reserved.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_INSPECTOR_MAIN_THREAD_DEBUGGER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_INSPECTOR_MAIN_THREAD_DEBUGGER_H_

#include <memory>
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/dom/document_lifecycle.h"
#include "third_party/blink/renderer/core/inspector/thread_debugger_common_impl.h"
#include "v8/include/v8-inspector.h"
#include "v8/include/v8.h"

namespace WTF {
class String;
}  // namespace WTF

namespace blink {

class ErrorEvent;
class LocalFrame;
class ScriptState;
class SecurityOrigin;
class SourceLocation;

class CORE_EXPORT MainThreadDebugger final : public ThreadDebuggerCommonImpl {
 public:
  class ClientMessageLoop {
    USING_FAST_MALLOC(ClientMessageLoop);

   public:
    enum MessageLoopKind { kNormalPause, kInstrumentationPause };

    virtual ~ClientMessageLoop() = default;
    virtual void Run(LocalFrame*, MessageLoopKind) = 0;
    virtual void QuitNow() = 0;
    virtual void RunIfWaitingForDebugger(LocalFrame*) = 0;
  };

  explicit MainThreadDebugger(v8::Isolate*);
  MainThreadDebugger(const MainThreadDebugger&) = delete;
  MainThreadDebugger& operator=(const MainThreadDebugger&) = delete;
  ~MainThreadDebugger() override;

  static MainThreadDebugger* Instance(v8::Isolate*);

  bool IsWorker() override { return false; }
  bool IsPaused() const { return paused_; }
  void SetClientMessageLoop(std::unique_ptr<ClientMessageLoop>);

  // TODO(dgozman): by making this method virtual, we can move many methods to
  // ThreadDebugger and avoid some duplication. Should be careful about
  // performance.
  int ContextGroupId(LocalFrame*);
  void DidClearContextsForFrame(LocalFrame*);
  void ContextCreated(ScriptState*, LocalFrame*, const SecurityOrigin*);
  void ContextWillBeDestroyed(ScriptState*);
  void ExceptionThrown(ExecutionContext*, ErrorEvent*);

 private:
  FRIEND_TEST_ALL_PREFIXES(MainThreadDebuggerMultipleMainFramesTest, Allow);

  void ReportConsoleMessage(ExecutionContext*,
                            mojom::ConsoleMessageSource,
                            mojom::ConsoleMessageLevel,
                            const WTF::String& message,
                            SourceLocation*) override;
  int ContextGroupId(ExecutionContext*) override;

  // V8InspectorClient implementation.
  void runMessageLoopOnPause(int context_group_id) override;
  void runMessageLoopOnInstrumentationPause(int context_group_id) override;
  void quitMessageLoopOnPause() override;
  void muteMetrics(int context_group_id) override;
  void unmuteMetrics(int context_group_id) override;
  v8::Local<v8::Context> ensureDefaultContextInGroup(
      int context_group_id) override;
  void beginEnsureAllContextsInGroup(int context_group_id) override;
  void endEnsureAllContextsInGroup(int context_group_id) override;
  bool canExecuteScripts(int context_group_id) override;
  void runIfWaitingForDebugger(int context_group_id) override;
  void consoleAPIMessage(int context_group_id,
                         v8::Isolate::MessageErrorLevel,
                         const v8_inspector::StringView& message,
                         const v8_inspector::StringView& url,
                         unsigned line_number,
                         unsigned column_number,
                         v8_inspector::V8StackTrace*) override;
  void consoleClear(int context_group_id) override;
  void installAdditionalCommandLineAPI(v8::Local<v8::Context>,
                                       v8::Local<v8::Object>) override;
  v8::MaybeLocal<v8::Value> memoryInfo(v8::Isolate*,
                                       v8::Local<v8::Context>) override;

  static void QuerySelectorCallback(const v8::FunctionCallbackInfo<v8::Value>&);
  static void QuerySelectorAllCallback(
      const v8::FunctionCallbackInfo<v8::Value>&);
  static void XpathSelectorCallback(const v8::FunctionCallbackInfo<v8::Value>&);

  std::unique_ptr<ClientMessageLoop> client_message_loop_;
  bool paused_;
  std::unique_ptr<DocumentLifecycle::PostponeTransitionScope>
      postponed_transition_scope_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_INSPECTOR_MAIN_THREAD_DEBUGGER_H_
