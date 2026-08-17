// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_BINDINGS_CORE_V8_SCRIPT_EVALUATION_RESULT_H_
#define THIRD_PARTY_BLINK_RENDERER_BINDINGS_CORE_V8_SCRIPT_EVALUATION_RESULT_H_

#include "base/dcheck_is_on.h"
#include "third_party/blink/public/mojom/script/script_type.mojom-blink-forward.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "v8/include/v8.h"

namespace blink {
class ScriptState;

// ScriptEvaluationResult encapsulates the result of a classic or module script
// evaluation:
// - https://html.spec.whatwg.org/C/#run-a-classic-script
// - https://html.spec.whatwg.org/C/#run-a-module-script
//
// Note: Top-level await (TLA, https://github.com/whatwg/html/pull/4352) will
// affect the semantics where mentioned below.
class CORE_EXPORT ScriptEvaluationResult final {
  STACK_ALLOCATED();

 public:
  ScriptEvaluationResult() = delete;
  ScriptEvaluationResult(const ScriptEvaluationResult&) = delete;
  ScriptEvaluationResult& operator=(const ScriptEvaluationResult&) = delete;
  ScriptEvaluationResult(ScriptEvaluationResult&&) = default;
  ScriptEvaluationResult& operator=(ScriptEvaluationResult&&) = default;
  ~ScriptEvaluationResult() = default;

  enum class ResultType {
    // The script is not evaluated.
    // Spec: NormalCompletion with empty [[Value]]
    // |value_| is empty.
    kNotRun,

    // The script is successfully evaluated.
    // Spec: #run-a-classic-script/#run-a-module-script return
    //       NormalCompletion with non-empty [[Value]].
    // |value_| is its non-empty [[Value]].
    //
    // Modules after TLA:
    // |value_| is the promise returned by #run-a-module-script.
    // Note: The promise can be rejected.
    // The script is either:
    // - Successfully evaluated synchronously
    //   (|value_|'s [[PromiseState]] is fulfilled), or
    // - Throwing synchronously during evaluation
    //   (|value_|'s [[PromiseState]] is rejected), or
    // - Successfully evaluated until a top-level await and is waiting for the
    //   promise awaited
    //   (|value_|'s [[PromiseState]] is pending).
    kSuccess,

    // The script is evaluated and an exception is thrown.
    // Spec: #run-a-classic-script/#run-a-module-script return an abrupt
    //       completion.
    // |value_| is the non-empty exception thrown for
    // - module scripts or
    // - classic scripts with |RethrowErrorsOption::DoNotRethrow()|
    // or empty for
    // - classic scripts with |RethrowErrorsOption::Rethrow()|
    //
    // Note: The exception can be already caught and passed to
    // https://html.spec.whatwg.org/C/#report-the-error, instead of being
    // rethrown.
    //
    // Modules after TLA: #run-a-module-script returns a promise where
    // [[PromiseState]] is rejected and [[PromiseResult]] is |value_|,
    // only if #concept-script-error-to-rethrow is not null, i.e.
    // only for parse/instantiation errors.
    // Module scripts throwing synchronously during evaluation return kSuccess.
    // TODO(cbruni, hiroshige): Consider cleaning up this semantics.
    kException,

    // The script is evaluated and aborted prematurely by
    // #abort-a-running-script.
    // |value_| is empty.
    // This corresponds to that TryCatch::CanContinue() is false, and currently
    // only checked in classic scripts in WorkerOrWorkletGlobalScope, for
    // handling e.g.
    // worker.terminate().
    // TODO(crbug.com/1129793): Check TryCatch::CanContinue() also in module
    // scripts.
    kAborted
  };

  static ScriptEvaluationResult FromClassicNotRun();
  static ScriptEvaluationResult FromClassicSuccess(v8::Local<v8::Value> value);
  static ScriptEvaluationResult FromClassicExceptionRethrown();
  static ScriptEvaluationResult FromClassicException(
      v8::Local<v8::Value> exception);
  static ScriptEvaluationResult FromClassicAborted();

  static ScriptEvaluationResult FromModuleNotRun();
  static ScriptEvaluationResult FromModuleSuccess(v8::Local<v8::Value> value);
  static ScriptEvaluationResult FromModuleException(
      v8::Local<v8::Value> exception);
  static ScriptEvaluationResult FromModuleAborted();

  ResultType GetResultType() const { return result_type_; }

  // Can be called only when GetResultType() == kSuccess.
  // TODO(crbug.com/1132793): Fix some of the callers (in unit tests) that
  // expect bool, string, etc., because after TLA is enabled this will return a
  // promise for modules.
  v8::Local<v8::Value> GetSuccessValue() const;

  // Returns the value when GetResultType() == kSuccess, or empty otherwise.
  v8::Local<v8::Value> GetSuccessValueOrEmpty() const;

  // Returns the exception thrown.
  // Can be called only when GetResultType() == kException.
  v8::Local<v8::Value> GetExceptionForModule() const;

  // Returns the exception thrown for classic scripts for a worklet.
  // Can be called only when GetResultType() == kException.
  //
  // TODO(crbug.com/1419253): Shared storage worklet is using a ClassicScript
  // and relies on this method to get the exception. This is tentative.
  // Eventually, this method should be removed once shared storage migrates to
  // the blink-worklet's script loading infrastructure.
  v8::Local<v8::Value> GetExceptionForWorklet() const;

  // Returns the exception thrown for both module and classic scripts.
  // Can be called only when GetResultType() == kException.
  v8::Local<v8::Value> GetExceptionForClassicForTesting() const;

  // Returns the promise returned by #run-a-module-script.
  // Can be called only
  // - For module script with TLA is enabled, and
  // - If GetResultType() == kSuccess or kException.
  //   (For kNotRun/kAborted, we should do nothing)
  ScriptPromise<IDLAny> GetPromise(ScriptState* script_state) const;

 private:
  ScriptEvaluationResult(mojom::blink::ScriptType,
                         ResultType,
                         v8::Local<v8::Value>);

#if DCHECK_IS_ON()
  mojom::blink::ScriptType script_type_;
#endif

  ResultType result_type_;
  v8::Local<v8::Value> value_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_BINDINGS_CORE_V8_SCRIPT_EVALUATION_RESULT_H_
