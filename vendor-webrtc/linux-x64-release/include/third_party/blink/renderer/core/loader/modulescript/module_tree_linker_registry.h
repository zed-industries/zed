// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LOADER_MODULESCRIPT_MODULE_TREE_LINKER_REGISTRY_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LOADER_MODULESCRIPT_MODULE_TREE_LINKER_REGISTRY_H_

#include "third_party/blink/public/mojom/fetch/fetch_api_request.mojom-blink-forward.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/loader/modulescript/module_script_creation_params.h"
#include "third_party/blink/renderer/platform/bindings/name_client.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_set.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/weborigin/kurl.h"

namespace blink {

class ModuleTreeLinker;

// ModuleTreeLinkerRegistry keeps active ModuleTreeLinkers alive.
class CORE_EXPORT ModuleTreeLinkerRegistry final
    : public GarbageCollected<ModuleTreeLinkerRegistry>,
      public NameClient {
 public:
  ModuleTreeLinkerRegistry() = default;
  ~ModuleTreeLinkerRegistry() final = default;

  // https://html.spec.whatwg.org/C/#fetch-a-module-script-tree
  // https://html.spec.whatwg.org/C/#fetch-a-module-worker-script-tree
  // https://html.spec.whatwg.org/C/#fetch-an-import()-module-script-graph
  void Fetch(const KURL& url,
             const ModuleType&,
             ResourceFetcher* fetch_client_settings_object_fetcher,
             mojom::blink::RequestContextType context_type,
             network::mojom::RequestDestination destination,
             const ScriptFetchOptions&,
             Modulator*,
             ModuleScriptCustomFetchType,
             ModuleTreeClient*,
             String referrer);

  // https://html.spec.whatwg.org/C/#fetch-an-inline-module-script-graph
  void FetchDescendantsForInlineScript(
      ModuleScript*,
      ResourceFetcher* fetch_client_settings_object_fetcher,
      mojom::blink::RequestContextType context_type,
      network::mojom::RequestDestination destination,
      Modulator*,
      ModuleScriptCustomFetchType,
      ModuleTreeClient*);

  void Trace(Visitor*) const;
  const char* NameInHeapSnapshot() const override {
    return "ModuleTreeLinkerRegistry";
  }

 private:
  friend class ModuleTreeLinker;
  void AddLinker(ModuleTreeLinker*);
  void ReleaseFinishedLinker(ModuleTreeLinker*);

  HeapHashSet<Member<ModuleTreeLinker>> active_tree_linkers_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LOADER_MODULESCRIPT_MODULE_TREE_LINKER_REGISTRY_H_
