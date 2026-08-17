// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LOADER_MODULESCRIPT_MODULE_TREE_LINKER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LOADER_MODULESCRIPT_MODULE_TREE_LINKER_H_

#include "base/dcheck_is_on.h"
#include "base/types/pass_key.h"
#include "third_party/blink/public/mojom/fetch/fetch_api_request.mojom-blink-forward.h"
#include "third_party/blink/public/platform/web_url_request.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/script/modulator.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_set.h"
#include "third_party/blink/renderer/platform/loader/fetch/fetch_client_settings_object_snapshot.h"
#include "third_party/blink/renderer/platform/weborigin/kurl.h"
#include "third_party/blink/renderer/platform/weborigin/kurl_hash.h"
#include "third_party/blink/renderer/platform/wtf/hash_set.h"

namespace blink {

class ModuleTreeLinkerRegistry;

// A ModuleTreeLinker is responsible for running and keeping intermediate states
// for top-level fetch * module script graph spec concepts and all the
// invocations of #internal-module-script-graph-fetching-procedure and
// #fetch-the-descendants-of-a-module-script.
//
// Modulator represents "a module map settings object" and
// ResourceFetcher represents "a fetch client settings object"
// by its |Context()->GetFetchClientSettingsObject()|.
class CORE_EXPORT ModuleTreeLinker final : public SingleModuleClient {
 public:
  ~ModuleTreeLinker() override = default;
  void Trace(Visitor*) const override;

  bool IsFetching() const {
    return State::kFetchingSelf <= state_ && state_ < State::kFinished;
  }
  bool HasFinished() const { return state_ == State::kFinished; }

  // Below methods are intended to be called via ModuleTreeLinkerRegistry.
  ModuleTreeLinker(ResourceFetcher* fetch_client_settings_object_fetcher,
                   mojom::blink::RequestContextType context_type,
                   network::mojom::RequestDestination destination,
                   Modulator*,
                   ModuleScriptCustomFetchType,
                   ModuleTreeLinkerRegistry*,
                   ModuleTreeClient*,
                   base::PassKey<ModuleTreeLinkerRegistry>);
  void FetchRoot(const KURL&,
                 ModuleType,
                 const ScriptFetchOptions&,
                 base::PassKey<ModuleTreeLinkerRegistry>,
                 String referrer);
  void FetchRootInline(ModuleScript*, base::PassKey<ModuleTreeLinkerRegistry>);

 private:
  enum class State {
    kInitial,
    // Running fetch of the module script corresponding to the target node.
    kFetchingSelf,
    // Running fetch of descendants of the target node.
    kFetchingDependencies,
    // Instantiating module_script_ and the node descendants.
    kInstantiating,
    kFinished,
  };

#if DCHECK_IS_ON()
  static const char* StateToString(State);
#endif
  void AdvanceState(State);

  void NotifyModuleLoadFinished(ModuleScript*) override;
  void FetchDescendants(const ModuleScript*);

  // Completion of [FD].
  void FinalizeFetchDescendantsForOneModuleScript();

  // [FDaI] Steps 4--8.
  void Instantiate();

  // [FFPE]
  ScriptValue FindFirstParseError(
      const ModuleScript*,
      HeapHashSet<Member<const ModuleScript>>*) const;

  const Member<ResourceFetcher> fetch_client_settings_object_fetcher_;

  const mojom::blink::RequestContextType context_type_;
  const network::mojom::RequestDestination destination_;
  const Member<Modulator> modulator_;
  const ModuleScriptCustomFetchType custom_fetch_type_;
  HashSet<std::pair<KURL, ModuleType>> visited_set_;
  const Member<ModuleTreeLinkerRegistry> registry_;
  const Member<ModuleTreeClient> client_;
  State state_ = State::kInitial;

  // Correspond to _result_ in the top-level fetch * module script graph
  // algorithms.
  Member<ModuleScript> result_;

  bool found_parse_error_ = false;

  size_t num_incomplete_fetches_ = 0;

#if DCHECK_IS_ON()
  KURL original_url_;
  KURL url_;
  ModuleType module_type_;
  bool root_is_inline_;

  friend CORE_EXPORT std::ostream& operator<<(std::ostream&, ModuleType);
  friend CORE_EXPORT std::ostream& operator<<(std::ostream&,
                                              const ModuleTreeLinker&);
#endif
};

#if DCHECK_IS_ON()
CORE_EXPORT std::ostream& operator<<(std::ostream&, const ModuleTreeLinker&);
#endif

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LOADER_MODULESCRIPT_MODULE_TREE_LINKER_H_
