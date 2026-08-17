// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LOADER_MODULESCRIPT_MODULE_SCRIPT_FETCHER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LOADER_MODULESCRIPT_MODULE_SCRIPT_FETCHER_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/loader/modulescript/module_script_creation_params.h"
#include "third_party/blink/renderer/core/loader/resource/script_resource.h"
#include "third_party/blink/renderer/core/script/modulator.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_vector.h"
#include "third_party/blink/renderer/platform/loader/fetch/fetch_parameters.h"
#include "third_party/blink/renderer/platform/loader/fetch/resource_fetcher.h"

namespace blink {

class ConsoleMessage;
class ModuleScriptLoader;

// ModuleScriptFetcher is an abstract class to fetch module scripts. Derived
// classes are expected to fetch a module script for the given FetchParameters
// and return its client a fetched resource as ModuleScriptCreationParams.
class CORE_EXPORT ModuleScriptFetcher : public ResourceClient {
 public:
  // ModuleScriptFetcher should only be called from ModuleScriptLoader.
  explicit ModuleScriptFetcher(base::PassKey<ModuleScriptLoader>);

  class CORE_EXPORT Client : public GarbageCollectedMixin {
   public:
    virtual void NotifyFetchFinishedError(
        const HeapVector<Member<ConsoleMessage>>& error_messages) = 0;
    virtual void NotifyFetchFinishedSuccess(
        const ModuleScriptCreationParams&) = 0;

    // These helpers are used only from WorkletModuleResponsesMap.
    // TODO(nhiroki): Move these helpers to WorkletModuleResponsesMap.
    void OnFetched(const ModuleScriptCreationParams&);
    void OnFailed();
  };

  // Fetch() must be called right after ModuleScriptFetcher is constructed.
  // Fetch() must not be called more than once.
  //
  // Takes a non-const reference to FetchParameters because
  // ScriptResource::Fetch() requires it.
  virtual void Fetch(FetchParameters&,
                     ModuleType,
                     ResourceFetcher*,
                     ModuleGraphLevel,
                     Client*) = 0;

  void Trace(Visitor*) const override;

 protected:
  // The returned value is empty in case of failure, and contains
  // the module type otherwise.
  static std::optional<ResolvedModuleType> WasModuleLoadSuccessful(
      ScriptResource* resource,
      ModuleType expected_module_type,
      HeapVector<Member<ConsoleMessage>>* error_messages);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LOADER_MODULESCRIPT_MODULE_SCRIPT_FETCHER_H_
