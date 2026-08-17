// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_SCRIPT_VALUE_WRAPPER_SYNTHETIC_MODULE_SCRIPT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_SCRIPT_VALUE_WRAPPER_SYNTHETIC_MODULE_SCRIPT_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/script/module_script.h"

namespace WTF {
class TextPosition;
}  // namespace WTF

namespace blink {

class KURL;
class Modulator;
class ModuleScriptCreationParams;

// ValueWrapperSyntheticModuleScript is a module script
// (https://html.spec.whatwg.org/C/#module-script) that default-exports a single
// v8::Value, for example JSON Module Script:
// https://html.spec.whatwg.org/multipage/webappapis.html#json-module-script
class CORE_EXPORT ValueWrapperSyntheticModuleScript final
    : public ModuleScript {
 public:
  static ValueWrapperSyntheticModuleScript*
  CreateCSSWrapperSyntheticModuleScript(const ModuleScriptCreationParams&,
                                        Modulator* settings_object);

  static ValueWrapperSyntheticModuleScript*
  CreateJSONWrapperSyntheticModuleScript(const ModuleScriptCreationParams&,
                                         Modulator* settings_object);

  static ValueWrapperSyntheticModuleScript* CreateWithDefaultExport(
      v8::Local<v8::Value> value,
      Modulator* settings_object,
      const KURL& source_url,
      const KURL& base_url,
      const ScriptFetchOptions& fetch_options,
      const TextPosition& start_position = TextPosition::MinimumPosition());

  static ValueWrapperSyntheticModuleScript* CreateWithError(
      v8::Local<v8::Value> value,
      Modulator* settings_object,
      const KURL& source_url,
      const KURL& base_url,
      const ScriptFetchOptions& fetch_options,
      v8::Local<v8::Value> error,
      const TextPosition& start_position = TextPosition::MinimumPosition());

  ValueWrapperSyntheticModuleScript(Modulator* settings_object,
                                    v8::Local<v8::Module> record,
                                    const KURL& source_url,
                                    const KURL& base_url,
                                    const ScriptFetchOptions& fetch_options,
                                    v8::Local<v8::Value> value,
                                    const TextPosition& start_position);

  // <specdef
  // href="https://webidl.spec.whatwg.org/#synthetic-module-record">
  // An abstract operation that will be performed upon evaluation of the module,
  // taking the Synthetic Module Record as its sole argument. These will usually
  // set up the exported values, by using SetSyntheticModuleExport. They must
  // not modify [[ExportNames]]. They may return an abrupt completion.
  static v8::MaybeLocal<v8::Value> EvaluationSteps(
      v8::Local<v8::Context> context,
      v8::Local<v8::Module> module);

  void Trace(Visitor* visitor) const override;

 private:
  TraceWrapperV8Reference<v8::Value> export_value_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_SCRIPT_VALUE_WRAPPER_SYNTHETIC_MODULE_SCRIPT_H_
