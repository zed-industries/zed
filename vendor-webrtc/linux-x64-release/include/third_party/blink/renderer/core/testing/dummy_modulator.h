// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_TESTING_DUMMY_MODULATOR_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_TESTING_DUMMY_MODULATOR_H_

#include "base/task/single_thread_task_runner.h"
#include "third_party/blink/public/mojom/fetch/fetch_api_request.mojom-blink-forward.h"
#include "third_party/blink/renderer/bindings/core/v8/module_record.h"
#include "third_party/blink/renderer/bindings/core/v8/module_request.h"
#include "third_party/blink/renderer/core/script/import_map_error.h"
#include "third_party/blink/renderer/core/script/modulator.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"

namespace blink {

class ModuleRecordResolver;

// DummyModulator provides empty Modulator interface implementation w/
// NOTREACHED().
//
// DummyModulator is useful for unit-testing.
// Not all module implementation components require full-blown Modulator
// implementation. Unit tests can implement a subset of Modulator interface
// which is exercised from unit-under-test.
class DummyModulator : public Modulator {
 public:
  DummyModulator();
  DummyModulator(const DummyModulator&) = delete;
  DummyModulator& operator=(const DummyModulator&) = delete;
  ~DummyModulator() override;
  void Trace(Visitor*) const override;

  ModuleRecordResolver* GetModuleRecordResolver() override;
  base::SingleThreadTaskRunner* TaskRunner() override;
  ScriptState* GetScriptState() override;
  mojom::blink::V8CacheOptions GetV8CacheOptions() const override;
  bool IsScriptingDisabled() const override;

  void FetchTree(const KURL&,
                 ModuleType,
                 ResourceFetcher*,
                 mojom::blink::RequestContextType context_type,
                 network::mojom::RequestDestination destination,
                 const ScriptFetchOptions&,
                 ModuleScriptCustomFetchType,
                 ModuleTreeClient*,
                 String referrer) override;
  void FetchSingle(const ModuleScriptFetchRequest&,
                   ResourceFetcher*,
                   ModuleGraphLevel,
                   ModuleScriptCustomFetchType,
                   SingleModuleClient*) override;
  void FetchDescendantsForInlineScript(
      ModuleScript*,
      ResourceFetcher*,
      mojom::blink::RequestContextType context_type,
      network::mojom::RequestDestination destination,
      ModuleTreeClient*) override;
  ModuleScript* GetFetchedModuleScript(const KURL&, ModuleType) override;
  KURL ResolveModuleSpecifier(const String&, const KURL&, String*) override;
  String GetIntegrityMetadataString(const KURL&) const override;
  IntegrityMetadataSet GetIntegrityMetadata(const KURL&) const override;
  bool HasValidContext() override;
  void ResolveDynamically(const ModuleRequest& module_request,
                          const ReferrerScriptInfo&,
                          ScriptPromiseResolver<IDLAny>*) override;
  ModuleImportMeta HostGetImportMetaProperties(
      v8::Local<v8::Module>) const override;
  ModuleType ModuleTypeFromRequest(
      const ModuleRequest& module_request) const override;
  ModuleScriptFetcher* CreateModuleScriptFetcher(
      ModuleScriptCustomFetchType,
      base::PassKey<ModuleScriptLoader>) override;
  void ProduceCacheModuleTreeTopLevel(ModuleScript*) override;

  Member<ModuleRecordResolver> resolver_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_TESTING_DUMMY_MODULATOR_H_
