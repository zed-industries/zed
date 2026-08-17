// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_SCRIPT_MODULATOR_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_SCRIPT_MODULATOR_H_

#include "base/task/single_thread_task_runner.h"
#include "base/types/pass_key.h"
#include "services/network/public/mojom/referrer_policy.mojom-blink-forward.h"
#include "third_party/blink/public/mojom/fetch/fetch_api_request.mojom-blink-forward.h"
#include "third_party/blink/public/platform/web_url_request.h"
#include "third_party/blink/renderer/bindings/core/v8/module_record.h"
#include "third_party/blink/renderer/bindings/core/v8/module_request.h"
#include "third_party/blink/renderer/bindings/core/v8/sanitize_script_errors.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise_resolver.h"
#include "third_party/blink/renderer/bindings/core/v8/v8_code_cache.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/script/import_map_error.h"
#include "third_party/blink/renderer/core/script/module_import_meta.h"
#include "third_party/blink/renderer/platform/bindings/name_client.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/bindings/v8_per_context_data.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/loader/fetch/integrity_metadata.h"
#include "third_party/blink/renderer/platform/weborigin/kurl.h"
#include "third_party/blink/renderer/platform/weborigin/referrer.h"
#include "third_party/blink/renderer/platform/wtf/text/text_position.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

class ModuleScript;
class ModuleScriptFetchRequest;
class ModuleScriptFetcher;
class ModuleScriptLoader;
class ImportMap;
class ReferrerScriptInfo;
class ResourceFetcher;
class ModuleRecordResolver;
class ScriptFetchOptions;
class ScriptState;
enum class ModuleType;

// A SingleModuleClient is notified when single module script node (node as in a
// module tree graph) load is complete and its corresponding entry is created in
// module map.
class CORE_EXPORT SingleModuleClient
    : public GarbageCollected<SingleModuleClient>,
      public NameClient {
 public:
  ~SingleModuleClient() override = default;
  virtual void Trace(Visitor* visitor) const {}
  const char* NameInHeapSnapshot() const override {
    return "SingleModuleClient";
  }

  virtual void NotifyModuleLoadFinished(ModuleScript*) = 0;
};

// A ModuleTreeClient is notified when a module script and its whole descendent
// tree load is complete.
class CORE_EXPORT ModuleTreeClient : public GarbageCollected<ModuleTreeClient>,
                                     public NameClient {
 public:
  ~ModuleTreeClient() override = default;
  virtual void Trace(Visitor* visitor) const {}
  const char* NameInHeapSnapshot() const override { return "ModuleTreeClient"; }

  virtual void NotifyModuleTreeLoadFinished(ModuleScript*) = 0;
};

// spec: "top-level module fetch flag"
// https://html.spec.whatwg.org/C/#fetching-scripts-is-top-level
enum class ModuleGraphLevel { kTopLevelModuleFetch, kDependentModuleFetch };

// spec: "custom peform the fetch hook"
// https://html.spec.whatwg.org/C/#fetching-scripts-perform-fetch
enum class ModuleScriptCustomFetchType {
  // Fetch module scripts without invoking custom fetch steps.
  kNone,

  // Perform custom fetch steps for worker's constructor defined in the HTML
  // spec:
  // https://html.spec.whatwg.org/C/#worker-processing-model
  kWorkerConstructor,

  // Perform custom fetch steps for Worklet's addModule() function defined in
  // the Worklet spec:
  // https://drafts.css-houdini.org/worklets/#fetch-a-worklet-script
  kWorkletAddModule,

  // Fetch a Service Worker's installed module script from the Service Worker's
  // script storage.
  kInstalledServiceWorker
};

// A Modulator is an interface for "environment settings object" concept for
// module scripts.
// https://html.spec.whatwg.org/C/#environment-settings-object
//
// A Modulator also serves as an entry point for various module spec algorithms.
class CORE_EXPORT Modulator : public GarbageCollected<Modulator>,
                              public V8PerContextData::Data,
                              public NameClient {
 public:
  static Modulator* From(ScriptState*);
  ~Modulator() override;

  static void SetModulator(ScriptState*, Modulator*);
  static void ClearModulator(ScriptState*);

  void Trace(Visitor* visitor) const override;
  const char* NameInHeapSnapshot() const override { return "Modulator"; }

  virtual ModuleRecordResolver* GetModuleRecordResolver() = 0;
  virtual base::SingleThreadTaskRunner* TaskRunner() = 0;

  virtual ScriptState* GetScriptState() = 0;

  virtual mojom::blink::V8CacheOptions GetV8CacheOptions() const = 0;

  // https://html.spec.whatwg.org/C/#concept-bc-noscript
  // "scripting is disabled for settings's responsible browsing context"
  virtual bool IsScriptingDisabled() const = 0;

  // https://html.spec.whatwg.org/C/#fetch-a-module-script-tree
  // https://html.spec.whatwg.org/C/#fetch-a-module-worker-script-tree
  // Note that |this| is the "module map settings object" and
  // ResourceFetcher represents "fetch client settings object"
  // used in the "fetch a module worker script graph" algorithm.
  virtual void FetchTree(
      const KURL&,
      ModuleType,
      ResourceFetcher* fetch_client_settings_object_fetcher,
      mojom::blink::RequestContextType context_type,
      network::mojom::RequestDestination destination,
      const ScriptFetchOptions&,
      ModuleScriptCustomFetchType,
      ModuleTreeClient*,
      String referrer = Referrer::ClientReferrerString()) = 0;

  // Asynchronously retrieve a module script from the module map, or fetch it
  // and put it in the map if it's not there already.
  // https://html.spec.whatwg.org/C/#fetch-a-single-module-script
  // Note that |this| is the "module map settings object" and
  // |fetch_client_settings_object_fetcher| represents
  // "fetch client settings object", which can be different from the
  // ResourceFetcher associated with |this|.
  virtual void FetchSingle(
      const ModuleScriptFetchRequest&,
      ResourceFetcher* fetch_client_settings_object_fetcher,
      ModuleGraphLevel,
      ModuleScriptCustomFetchType,
      SingleModuleClient*) = 0;

  virtual void FetchDescendantsForInlineScript(
      ModuleScript*,
      ResourceFetcher* fetch_client_settings_object_fetcher,
      mojom::blink::RequestContextType context_type,
      network::mojom::RequestDestination destination,
      ModuleTreeClient*) = 0;

  // Synchronously retrieves a single module script from existing module map
  // entry.
  // Note: returns nullptr if the module map entry doesn't exist, or
  // is still "fetching".
  // ModuleType indicates the resource type of the module script, e.g.
  // JavaScript, JSON, or CSS. This is used as part of the module map cache key
  // alongside the URL, so both are needed to retrieve the correct module. See
  // https://github.com/whatwg/html/pull/5883
  virtual ModuleScript* GetFetchedModuleScript(const KURL&, ModuleType) = 0;

  // https://html.spec.whatwg.org/C#resolve-a-module-specifier
  virtual KURL ResolveModuleSpecifier(const String& module_request,
                                      const KURL& base_url,
                                      String* failure_reason) = 0;

  // https://tc39.github.io/proposal-dynamic-import/#sec-hostimportmoduledynamically
  virtual void ResolveDynamically(const ModuleRequest& module_request,
                                  const ReferrerScriptInfo&,
                                  ScriptPromiseResolver<IDLAny>*) = 0;

  // Methods below relate to import maps.
  // https://html.spec.whatwg.org/C#import-maps

  // https://html.spec.whatwg.org/C#merge-existing-and-new-import-maps
  virtual void MergeExistingAndNewImportMaps(ImportMap*) {
    // 1. Assert: global implements Window
    NOTREACHED();
  }

  const ImportMap* GetImportMapForTest() const { return import_map_.Get(); }

  enum class AcquiringImportMapsState {
    // The flag is true.
    kAcquiring,

    // The flag is false, due to multiple import maps.
    kMultipleImportMaps,

    // The flag is false, because module script loading is already started.
    kAfterModuleScriptLoad
  };
  AcquiringImportMapsState GetAcquiringImportMapsState() const {
    return acquiring_import_maps_;
  }
  void SetAcquiringImportMapsState(AcquiringImportMapsState value) {
    acquiring_import_maps_ = value;
  }

  // https://html.spec.whatwg.org/C#hostgetimportmetaproperties
  virtual ModuleImportMeta HostGetImportMetaProperties(
      v8::Local<v8::Module>) const = 0;

  // https://html.spec.whatwg.org/C#resolving-a-module-integrity-metadata
  virtual String GetIntegrityMetadataString(const KURL&) const = 0;
  virtual IntegrityMetadataSet GetIntegrityMetadata(const KURL&) const = 0;

  virtual bool HasValidContext() = 0;

  virtual ModuleType ModuleTypeFromRequest(
      const ModuleRequest& module_request) const = 0;

  virtual ModuleScriptFetcher* CreateModuleScriptFetcher(
      ModuleScriptCustomFetchType,
      base::PassKey<ModuleScriptLoader> pass_key) = 0;

  // Produce V8 code cache for the given ModuleScript and its submodules.
  virtual void ProduceCacheModuleTreeTopLevel(ModuleScript*) = 0;

  // https://html.spec.whatwg.org/C#add-module-to-resolved-module-set
  virtual void AddModuleToResolvedModuleSet(std::optional<AtomicString>,
                                            AtomicString) {
    // 2. If global does not implement Window, then return.
  }

 protected:
  Member<ImportMap> import_map_;

 private:
  // TODO(crbug.com/365578430): Remove this state and its setters/getters once
  // the MultipleImportMaps flag is removed.
  AcquiringImportMapsState acquiring_import_maps_ =
      AcquiringImportMapsState::kAcquiring;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_SCRIPT_MODULATOR_H_
