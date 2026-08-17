// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LOADER_MODULESCRIPT_DOCUMENT_MODULE_SCRIPT_FETCHER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LOADER_MODULESCRIPT_DOCUMENT_MODULE_SCRIPT_FETCHER_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/execution_context/execution_context_lifecycle_observer.h"
#include "third_party/blink/renderer/core/loader/modulescript/module_script_fetcher.h"
#include "third_party/blink/renderer/platform/loader/fetch/resource_fetcher.h"

namespace blink {

// DocumentModuleScriptFetcher is an implmenetation of ModuleScriptFetcher
// interface used for <script type='module'> on Document.
// TODO(nhiroki): This class is also used for non-custom module script fetch on
// workers. We should rename this to something like ModuleScriptFetcherImpl that
// doesn't relate to Document.
class CORE_EXPORT DocumentModuleScriptFetcher final
    : public GarbageCollected<DocumentModuleScriptFetcher>,
      public ModuleScriptFetcher,
      public ExecutionContextClient {
 public:
  explicit DocumentModuleScriptFetcher(ExecutionContext* execution_context,
                                       base::PassKey<ModuleScriptLoader>);

  // Implements ModuleScriptFetcher.
  void Fetch(FetchParameters&,
             ModuleType,
             ResourceFetcher*,
             ModuleGraphLevel,
             Client*) override;

  // Implements ResourceClient
  void NotifyFinished(Resource*) override;
  String DebugName() const override { return "DocumentModuleScriptFetcher"; }

  void Trace(Visitor*) const override;

 private:
  Member<Client> client_;
  ModuleType expected_module_type_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LOADER_MODULESCRIPT_DOCUMENT_MODULE_SCRIPT_FETCHER_H_
