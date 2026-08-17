/*
 * Copyright (C) 2009, 2012 Google Inc. All rights reserved.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_BINDINGS_CORE_V8_WORKER_OR_WORKLET_SCRIPT_CONTROLLER_H_
#define THIRD_PARTY_BLINK_RENDERER_BINDINGS_CORE_V8_WORKER_OR_WORKLET_SCRIPT_CONTROLLER_H_

#include "third_party/blink/renderer/bindings/core/v8/rejected_promises.h"
#include "third_party/blink/renderer/bindings/core/v8/v8_binding_for_core.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "v8/include/v8.h"

namespace blink {

class KURL;
class WorkerOrWorkletGlobalScope;

class CORE_EXPORT WorkerOrWorkletScriptController final
    : public GarbageCollected<WorkerOrWorkletScriptController> {
 public:
  WorkerOrWorkletScriptController(WorkerOrWorkletGlobalScope*,
                                  v8::Isolate*,
                                  bool is_default_world_of_isolate);

  WorkerOrWorkletScriptController(const WorkerOrWorkletScriptController&) =
      delete;
  WorkerOrWorkletScriptController& operator=(
      const WorkerOrWorkletScriptController&) = delete;

  ~WorkerOrWorkletScriptController();
  void Dispose();

  bool IsExecutionForbidden() const;

  // Prevents future JavaScript execution.
  void ForbidExecution();

  // For WorkerGlobalScope and threaded WorkletGlobalScope, |url_for_debugger|
  // is and should be used only for setting name/origin that appears in
  // DevTools.
  // For main thread WorkletGlobalScope, WorkerOrWorkletGlobalScope::Name() is
  // used for setting DOMWrapperWorld's human readable name.
  // This should be called only once.
  void Initialize(const KURL& url_for_debugger);

  // Prepares for script evaluation. This must be called after Initialize()
  // before Evaluate().
  void PrepareForEvaluation();

  // Disables `eval()` on JavaScript. This must be called before Evaluate().
  void DisableEval(const String&);

  // Disables wasm code generation. This must be called before Evaluate().
  void SetWasmEvalErrorMessage(const String&);

  ScriptState* GetScriptState() { return script_state_.Get(); }

  // Used by V8 bindings:
  v8::Local<v8::Context> GetContext() {
    return script_state_ ? script_state_->GetContext()
                         : v8::Local<v8::Context>();
  }

  RejectedPromises* GetRejectedPromises() const {
    return rejected_promises_.get();
  }

  void Trace(Visitor*) const;

  bool IsContextInitialized() const {
    return script_state_ && !!script_state_->PerContextData();
  }
  bool IsReadyToEvaluate() const { return is_ready_to_evaluate_; }

 private:
  void DisableEvalInternal(const String& error_message);

  void SetWasmEvalErrorMessageInternal(const String& error_message);

  void DisposeContextIfNeeded();

  Member<WorkerOrWorkletGlobalScope> global_scope_;

  // The v8 isolate associated to the (worker or worklet) global scope. For
  // workers this should be the worker thread's isolate, while for worklets
  // usually the main thread's isolate is used.
  v8::Isolate* isolate_;

  Member<ScriptState> script_state_;
  Member<DOMWrapperWorld> world_;

  // Keeps the error message for `eval()` on JavaScript until Initialize().
  String disable_eval_pending_;
  String disable_wasm_eval_pending_;

  bool is_ready_to_evaluate_ = false;
  bool execution_forbidden_ = false;

  scoped_refptr<RejectedPromises> rejected_promises_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_BINDINGS_CORE_V8_WORKER_OR_WORKLET_SCRIPT_CONTROLLER_H_
