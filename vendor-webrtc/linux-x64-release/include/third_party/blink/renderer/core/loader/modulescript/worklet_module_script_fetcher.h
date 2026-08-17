// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LOADER_MODULESCRIPT_WORKLET_MODULE_SCRIPT_FETCHER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LOADER_MODULESCRIPT_WORKLET_MODULE_SCRIPT_FETCHER_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/loader/modulescript/module_script_fetcher.h"
#include "third_party/blink/renderer/core/workers/worklet_module_responses_map.h"

namespace blink {

class ResourceFetcher;
class WorkletGlobalScope;

// WorkletModuleScriptFetcher is an implementation of ModuleScriptFetcher
// interface for Worklets. This implements the custom "perform the fetch" hook
// defined in the Worklets spec:
// https://html.spec.whatwg.org/C/#fetching-scripts-perform-fetch
// https://drafts.css-houdini.org/worklets/#fetch-a-worklet-script
//
// WorkletModuleScriptFetcher either fetchs a cached result from
// WorkletModuleResponsesMap, or defers to ModuleScriptFetcher and
// stores the result in WorkletModuleResponsesMap on fetch completion.
class CORE_EXPORT WorkletModuleScriptFetcher final
    : public GarbageCollected<WorkletModuleScriptFetcher>,
      public ModuleScriptFetcher {
 public:
  WorkletModuleScriptFetcher(WorkletGlobalScope*,
                             base::PassKey<ModuleScriptLoader>);

  // Implements ModuleScriptFetcher.
  void Fetch(FetchParameters&,
             ModuleType,
             ResourceFetcher*,
             ModuleGraphLevel,
             ModuleScriptFetcher::Client*) override;

  void Trace(Visitor* visitor) const override;

 private:
  // Implements ResourceClient
  void NotifyFinished(Resource*) override;
  String DebugName() const override { return "WorkletModuleScriptFetcher"; }

  const Member<WorkletGlobalScope> global_scope_;
  KURL url_;
  ModuleType expected_module_type_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LOADER_MODULESCRIPT_WORKLET_MODULE_SCRIPT_FETCHER_H_
