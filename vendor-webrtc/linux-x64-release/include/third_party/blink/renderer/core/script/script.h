// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_SCRIPT_SCRIPT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_SCRIPT_SCRIPT_H_

#include <optional>

#include "third_party/blink/public/mojom/script/script_type.mojom-blink.h"
#include "third_party/blink/renderer/bindings/core/v8/v8_script_runner.h"
#include "third_party/blink/renderer/bindings/core/v8/v8_worker_type.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/loader/fetch/script_fetch_options.h"
#include "third_party/blink/renderer/platform/wtf/text/text_position.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

class LocalDOMWindow;

// https://html.spec.whatwg.org/C/#concept-script
class CORE_EXPORT Script : public GarbageCollected<Script> {
 public:
  virtual void Trace(Visitor* visitor) const {}

  virtual ~Script() {}

  virtual mojom::blink::ScriptType GetScriptType() const = 0;
  static mojom::blink::ScriptType V8WorkerTypeToScriptType(
      V8WorkerType::Enum script_type);

  // https://html.spec.whatwg.org/C/#run-a-classic-script
  // or
  // https://html.spec.whatwg.org/C/#run-a-module-script,
  // depending on the script type.
  // - Callers of `RunScript*AndReturnValue()` must enter a v8::HandleScope
  //   before calling.
  // - `ScriptState` == the script's modulator's `ScriptState` for modules.
  // - `ExecuteScriptPolicy::kExecuteScriptWhenScriptsDisabled` is allowed only
  //   for classic scripts with clear or historical reasons.
  //
  // On a ScriptState:
  void RunScriptOnScriptState(
      ScriptState*,
      ExecuteScriptPolicy =
          ExecuteScriptPolicy::kDoNotExecuteScriptWhenScriptsDisabled,
      V8ScriptRunner::RethrowErrorsOption =
          V8ScriptRunner::RethrowErrorsOption::DoNotRethrow());
  [[nodiscard]] virtual ScriptEvaluationResult
  RunScriptOnScriptStateAndReturnValue(
      ScriptState*,
      ExecuteScriptPolicy =
          ExecuteScriptPolicy::kDoNotExecuteScriptWhenScriptsDisabled,
      V8ScriptRunner::RethrowErrorsOption =
          V8ScriptRunner::RethrowErrorsOption::DoNotRethrow()) = 0;
  // On the main world of LocalDOMWindow:
  void RunScript(
      LocalDOMWindow*,
      ExecuteScriptPolicy =
          ExecuteScriptPolicy::kDoNotExecuteScriptWhenScriptsDisabled,
      V8ScriptRunner::RethrowErrorsOption =
          V8ScriptRunner::RethrowErrorsOption::DoNotRethrow());
  [[nodiscard]] ScriptEvaluationResult RunScriptAndReturnValue(
      LocalDOMWindow*,
      ExecuteScriptPolicy =
          ExecuteScriptPolicy::kDoNotExecuteScriptWhenScriptsDisabled,
      V8ScriptRunner::RethrowErrorsOption =
          V8ScriptRunner::RethrowErrorsOption::DoNotRethrow());

  const ScriptFetchOptions& FetchOptions() const { return fetch_options_; }
  const KURL& BaseUrl() const { return base_url_; }
  const KURL& SourceUrl() const { return source_url_; }
  const TextPosition& StartPosition() const { return start_position_; }

 protected:
  explicit Script(const ScriptFetchOptions& fetch_options,
                  const KURL& base_url,
                  const KURL& source_url,
                  const TextPosition& start_position)
      : fetch_options_(fetch_options),
        base_url_(base_url),
        source_url_(source_url),
        start_position_(start_position) {}

 private:
  // https://html.spec.whatwg.org/C/#concept-script-script-fetch-options
  const ScriptFetchOptions fetch_options_;

  // https://html.spec.whatwg.org/C/#concept-script-base-url
  const KURL base_url_;

  // The URL of the script, which is primarily intended for DevTools
  // javascript debugger, and can be observed as:
  // 1) The 'source-file' in CSP violations reports.
  // 2) The URL(s) in javascript stack traces.
  // 3) How relative source map are resolved.
  //
  // The fragment is stripped due to https://crbug.com/306239 (except for worker
  // top-level scripts), at the callers of Create(), or inside
  // CreateFromResource() and CreateUnspecifiedScript() in ClassicScript.
  //
  // It is important to keep the url fragment for worker top-level scripts so
  // that errors in worker scripts can include the fragment when reporting the
  // location of the failure. This is enforced by several tests in
  // external/wpt/workers/interfaces/WorkerGlobalScope/onerror/.
  //
  // Note that this can be different from the script's base URL
  // (`Script::BaseUrl()`, #concept-script-base-url).
  const KURL source_url_;

  const TextPosition start_position_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_SCRIPT_SCRIPT_H_
