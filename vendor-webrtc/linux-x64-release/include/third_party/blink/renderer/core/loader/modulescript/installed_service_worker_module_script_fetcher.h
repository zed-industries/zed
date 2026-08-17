// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LOADER_MODULESCRIPT_INSTALLED_SERVICE_WORKER_MODULE_SCRIPT_FETCHER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LOADER_MODULESCRIPT_INSTALLED_SERVICE_WORKER_MODULE_SCRIPT_FETCHER_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/loader/modulescript/module_script_fetcher.h"

namespace blink {

class WorkerGlobalScope;

// InstalledServiceWorkerModuleScriptFetcher is an implementation of
// ModuleScriptFetcher for retrieving an installed ServiceWorker script
// from ServiceWorker's script storage.
class CORE_EXPORT InstalledServiceWorkerModuleScriptFetcher final
    : public GarbageCollected<InstalledServiceWorkerModuleScriptFetcher>,
      public ModuleScriptFetcher {
 public:
  InstalledServiceWorkerModuleScriptFetcher(WorkerGlobalScope*,
                                            base::PassKey<ModuleScriptLoader>);

  // Implements ModuleScriptFetcher.
  void Fetch(FetchParameters&,
             ModuleType,
             ResourceFetcher*,
             ModuleGraphLevel,
             ModuleScriptFetcher::Client*) override;

  void Trace(Visitor*) const override;

 private:
  String DebugName() const override {
    return "InstalledServiceWorkerModuleScriptFetcher";
  }

  const Member<WorkerGlobalScope> global_scope_;
  ModuleType expected_module_type_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LOADER_MODULESCRIPT_INSTALLED_SERVICE_WORKER_MODULE_SCRIPT_FETCHER_H_
