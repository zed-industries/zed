// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LOADER_MODULESCRIPT_WORKER_MODULE_SCRIPT_FETCHER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LOADER_MODULESCRIPT_WORKER_MODULE_SCRIPT_FETCHER_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/loader/modulescript/module_script_fetcher.h"
#include "third_party/blink/renderer/platform/loader/fetch/url_loader/worker_main_script_loader.h"
#include "third_party/blink/renderer/platform/loader/fetch/url_loader/worker_main_script_loader_client.h"
#include "third_party/blink/renderer/platform/wtf/text/string_builder.h"

namespace blink {

class WorkerGlobalScope;

// WorkerModuleScriptFetcher is an implementation of ModuleScriptFetcher
// interface for WebWorkers. This implements the custom "perform the fetch" hook
// defined in the HTML spec:
// https://html.spec.whatwg.org/C/#fetching-scripts-perform-fetch
// https://html.spec.whatwg.org/C/#worker-processing-model
class CORE_EXPORT WorkerModuleScriptFetcher final
    : public GarbageCollected<WorkerModuleScriptFetcher>,
      public ModuleScriptFetcher,
      public WorkerMainScriptLoaderClient {
 public:
  WorkerModuleScriptFetcher(WorkerGlobalScope*,
                            base::PassKey<ModuleScriptLoader>);

  // Implements ModuleScriptFetcher.
  void Fetch(FetchParameters&,
             ModuleType,
             ResourceFetcher*,
             ModuleGraphLevel,
             ModuleScriptFetcher::Client*) override;

  // Implements WorkerMainScriptLoaderClient, and these will be called for
  // dedicated workers and shared workers.
  void DidReceiveDataWorkerMainScript(base::span<const char> span) override;
  void OnStartLoadingBodyWorkerMainScript(
      const ResourceResponse& resource_response) override;
  void OnFinishedLoadingWorkerMainScript() override;
  void OnFailedLoadingWorkerMainScript() override;

  void Trace(Visitor*) const override;

 private:
  // Implements ResourceClient
  void NotifyFinished(Resource*) override;
  String DebugName() const override { return "WorkerModuleScriptFetcher"; }

  void NotifyClient(const KURL& request_url,
                    ResolvedModuleType module_type,
                    const ParkableString& source_text,
                    const ResourceResponse& response,
                    CachedMetadataHandler* cache_handler);

  const Member<WorkerGlobalScope> global_scope_;

  // These are used for dedicated workers and shared workers.
  Member<WorkerMainScriptLoader> worker_main_script_loader_;
  std::unique_ptr<TextResourceDecoder> decoder_;
  StringBuilder source_text_;

  Member<ResourceFetcher> fetch_client_settings_object_fetcher_;
  Member<Client> client_;
  ModuleGraphLevel level_;
  ModuleType expected_module_type_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LOADER_MODULESCRIPT_WORKER_MODULE_SCRIPT_FETCHER_H_
