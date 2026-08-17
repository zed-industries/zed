// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_SCRIPT_MODULE_SCRIPT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_SCRIPT_MODULE_SCRIPT_H_

#include "third_party/blink/renderer/bindings/core/v8/module_record.h"
#include "third_party/blink/renderer/bindings/core/v8/script_value.h"
#include "third_party/blink/renderer/bindings/core/v8/world_safe_v8_reference.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/script/modulator.h"
#include "third_party/blink/renderer/core/script/script.h"
#include "third_party/blink/renderer/platform/bindings/name_client.h"
#include "third_party/blink/renderer/platform/bindings/parkable_string.h"
#include "third_party/blink/renderer/platform/bindings/trace_wrapper_v8_reference.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/loader/fetch/url_loader/cached_metadata_handler.h"
#include "third_party/blink/renderer/platform/weborigin/kurl.h"
#include "third_party/blink/renderer/platform/weborigin/kurl_hash.h"
#include "third_party/blink/renderer/platform/wtf/hash_map.h"
#include "third_party/blink/renderer/platform/wtf/text/text_position.h"
#include "v8/include/v8.h"

namespace blink {

// ModuleScript is a model object for the "module script" spec concept.
// https://html.spec.whatwg.org/C/#module-script
class CORE_EXPORT ModuleScript : public Script {
 public:
  v8::Local<v8::Module> V8Module() const;
  bool HasEmptyRecord() const;

  // Note: ParseError-related methods should only be used from ModuleTreeLinker
  //       or unit tests. You probably want to check |*ErrorToRethrow*()|
  //       instead.

  void SetParseErrorAndClearRecord(ScriptValue error);
  bool HasParseError() const { return !parse_error_.IsEmpty(); }

  // CreateParseError() retrieves |parse_error_| as a ScriptValue.
  ScriptValue CreateParseError() const;

  void SetErrorToRethrow(ScriptValue error);
  bool HasErrorToRethrow() const { return !error_to_rethrow_.IsEmpty(); }
  ScriptValue CreateErrorToRethrow() const;

  // Resolves a module specifier with the module script's base URL.
  KURL ResolveModuleSpecifier(const String& module_request,
                              String* failure_reason = nullptr) const;

  void Trace(Visitor*) const override;

  virtual void ProduceCache() {}

  [[nodiscard]] ScriptEvaluationResult RunScriptOnScriptStateAndReturnValue(
      ScriptState*,
      ExecuteScriptPolicy =
          ExecuteScriptPolicy::kDoNotExecuteScriptWhenScriptsDisabled,
      V8ScriptRunner::RethrowErrorsOption =
          V8ScriptRunner::RethrowErrorsOption::DoNotRethrow()) override;

  Modulator* SettingsObject() const { return settings_object_.Get(); }

 protected:
  ModuleScript(Modulator*,
               v8::Local<v8::Module>,
               const KURL& source_url,
               const KURL& base_url,
               const ScriptFetchOptions&,
               const TextPosition& start_position);

 private:
  mojom::blink::ScriptType GetScriptType() const override {
    return mojom::blink::ScriptType::kModule;
  }

  friend class ModuleTreeLinkerTestModulator;

  // https://html.spec.whatwg.org/C/#settings-object
  Member<Modulator> settings_object_;

  // https://html.spec.whatwg.org/C/#concept-script-record
  TraceWrapperV8Reference<v8::Module> record_;

  // https://html.spec.whatwg.org/C/#concept-script-parse-error
  //
  // |record_|, |parse_error_| and |error_to_rethrow_| are wrapper traced and
  // kept alive via one or more of following reference graphs:
  // * non-inline module script case
  //   DOMWindow -> Modulator/ModulatorImpl -> ModuleMap -> ModuleMap::Entry
  //   -> ModuleScript
  // * inline module script case, before the PendingScript is created.
  //   DOMWindow -> Modulator/ModulatorImpl -> ModuleTreeLinkerRegistry
  //   -> ModuleTreeLinker -> ModuleScript
  // * inline module script case, after the PendingScript is created.
  //   HTMLScriptElement -> ScriptLoader -> ModulePendingScript
  //   -> ModulePendingScriptTreeClient -> ModuleScript.
  // * inline module script case, queued in HTMLParserScriptRunner,
  //   when HTMLScriptElement is removed before execution.
  //   Document -> HTMLDocumentParser -> HTMLParserScriptRunner
  //   -> ModulePendingScript -> ModulePendingScriptTreeClient
  //   -> ModuleScript.
  // * inline module script case, queued in ScriptRunner.
  //   Document -> ScriptRunner -> ScriptLoader -> ModulePendingScript
  //   -> ModulePendingScriptTreeClient -> ModuleScript.
  // All the classes/references on the graphs above should be
  // Member<>/etc.,
  //
  // A parse error and an error to rethrow belong to a script, not to a
  // |parse_error_| and |error_to_rethrow_| should belong to a script (i.e.
  // blink::Script) according to the spec, but are put here in ModuleScript,
  // because:
  // - Error handling for classic and module scripts are somehow separated and
  //   there are no urgent motivation for merging the error handling and placing
  //   the errors in Script, and
  // - Classic scripts are handled according to the spec before
  //   https://github.com/whatwg/html/pull/2991. This shouldn't cause any
  //   observable functional changes, and updating the classic script handling
  //   will require moderate code changes (e.g. to move compilation timing).
  WorldSafeV8Reference<v8::Value> parse_error_;

  // https://html.spec.whatwg.org/C/#concept-script-error-to-rethrow
  WorldSafeV8Reference<v8::Value> error_to_rethrow_;

  mutable HashMap<String, KURL> specifier_to_url_cache_;
};

CORE_EXPORT std::ostream& operator<<(std::ostream&, const ModuleScript&);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_SCRIPT_MODULE_SCRIPT_H_
