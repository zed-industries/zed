// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LOADER_MODULESCRIPT_MODULE_SCRIPT_LOADER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LOADER_MODULESCRIPT_MODULE_SCRIPT_LOADER_H_

#include "base/dcheck_is_on.h"
#include "third_party/blink/public/platform/web_url_request.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/loader/modulescript/module_script_creation_params.h"
#include "third_party/blink/renderer/core/loader/modulescript/module_script_fetch_request.h"
#include "third_party/blink/renderer/core/loader/modulescript/module_script_fetcher.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/weborigin/kurl.h"

namespace blink {

class Modulator;
class ModuleScript;
class ModuleScriptLoaderClient;
class ModuleScriptLoaderRegistry;
class ResourceFetcher;
enum class ModuleGraphLevel;

// ModuleScriptLoader is responsible for loading a new single ModuleScript.
//
// ModuleScriptLoader constructs FetchParameters and asks ModuleScriptFetcher
// to fetch a script with the parameters. Then, it returns its client a compiled
// ModuleScript.
//
// ModuleScriptLoader(s) should only be used via Modulator and its ModuleMap.
class CORE_EXPORT ModuleScriptLoader final
    : public GarbageCollected<ModuleScriptLoader>,
      public ModuleScriptFetcher::Client {
  enum class State {
    kInitial,
    // FetchParameters is being processed, and ModuleScriptLoader hasn't
    // notifyFinished().
    kFetching,
    // Finished successfully or w/ error.
    kFinished,
  };

 public:
  ModuleScriptLoader(Modulator*,
                     const ScriptFetchOptions&,
                     ModuleScriptLoaderRegistry*,
                     ModuleScriptLoaderClient*);
  ModuleScriptLoader(const ModuleScriptLoader&) = delete;
  ModuleScriptLoader& operator=(const ModuleScriptLoader&) = delete;
  ~ModuleScriptLoader();

  static void Fetch(const ModuleScriptFetchRequest&,
                    ResourceFetcher* fetch_client_settings_object_fetcher,
                    ModuleGraphLevel,
                    Modulator* module_map_settings_object,
                    ModuleScriptCustomFetchType,
                    ModuleScriptLoaderRegistry*,
                    ModuleScriptLoaderClient*);

  // Implements ModuleScriptFetcher::Client.
  void NotifyFetchFinishedSuccess(const ModuleScriptCreationParams&) override;
  void NotifyFetchFinishedError(
      const HeapVector<Member<ConsoleMessage>>& error_messages) override;

  bool IsInitialState() const { return state_ == State::kInitial; }
  bool HasFinished() const { return state_ == State::kFinished; }

  void Trace(Visitor*) const override;

  friend class WorkletModuleResponsesMapTest;

 private:
  void FetchInternal(const ModuleScriptFetchRequest&,
                     ResourceFetcher* fetch_client_settings_object_fetcher,
                     ModuleGraphLevel,
                     ModuleScriptCustomFetchType);

  void AdvanceState(State new_state);

  using PassKey = base::PassKey<ModuleScriptLoader>;
  // PassKey should be private and cannot be accessed from outside, but allow
  // accessing only from friend classes for testing.
  static base::PassKey<ModuleScriptLoader> CreatePassKeyForTests() {
    return PassKey();
  }

#if DCHECK_IS_ON()
  static const char* StateToString(State);
#endif

  Member<Modulator> modulator_;
  State state_ = State::kInitial;
  const ScriptFetchOptions options_;
  Member<ModuleScript> module_script_;
  Member<ModuleScriptLoaderRegistry> registry_;
  Member<ModuleScriptLoaderClient> client_;
  Member<ModuleScriptFetcher> module_fetcher_;
#if DCHECK_IS_ON()
  KURL url_;
#endif
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LOADER_MODULESCRIPT_MODULE_SCRIPT_LOADER_H_
