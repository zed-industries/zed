// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_WORKERS_WORKER_OR_WORKLET_GLOBAL_SCOPE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_WORKERS_WORKER_OR_WORKLET_GLOBAL_SCOPE_H_

#include <bitset>

#include "base/task/single_thread_task_runner.h"
#include "base/unguessable_token.h"
#include "services/network/public/mojom/fetch_api.mojom-blink-forward.h"
#include "services/network/public/mojom/web_sandbox_flags.mojom-blink-forward.h"
#include "third_party/blink/public/mojom/loader/code_cache.mojom-forward.h"
#include "third_party/blink/public/mojom/v8_cache_options.mojom-blink.h"
#include "third_party/blink/public/platform/cross_variant_mojo_util.h"
#include "third_party/blink/public/platform/web_content_settings_client.h"
#include "third_party/blink/public/platform/web_url_request.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/dom/events/event_target.h"
#include "third_party/blink/renderer/core/execution_context/execution_context.h"
#include "third_party/blink/renderer/core/frame/csp/content_security_policy.h"
#include "third_party/blink/renderer/core/frame/deprecation/deprecation.h"
#include "third_party/blink/renderer/core/frame/web_feature_forward.h"
#include "third_party/blink/renderer/core/loader/back_forward_cache_loader_helper_impl.h"
#include "third_party/blink/renderer/core/script/modulator.h"
#include "third_party/blink/renderer/core/workers/worker_clients.h"
#include "third_party/blink/renderer/core/workers/worker_navigator.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_set.h"
#include "third_party/blink/renderer/platform/loader/fetch/resource_load_scheduler.h"
#include "third_party/blink/renderer/platform/loader/fetch/resource_loader_options.h"
#include "third_party/blink/renderer/platform/scheduler/public/worker_scheduler.h"
#include "third_party/blink/renderer/platform/wtf/casting.h"

namespace blink {

class FetchClientSettingsObject;
class FetchClientSettingsObjectSnapshot;
class Modulator;
class ModuleTreeClient;
class ResourceFetcher;
class WorkerResourceTimingNotifier;
class SubresourceFilter;
class WebContentSettingsClient;
class WebWorkerFetchContext;
class WorkerOrWorkletScriptController;
class WorkerReportingProxy;
class WorkerThread;

class CORE_EXPORT WorkerOrWorkletGlobalScope
    : public EventTarget,
      public ExecutionContext,
      public scheduler::WorkerScheduler::Delegate,
      public BackForwardCacheLoaderHelperImpl::Delegate {
 public:
  WorkerOrWorkletGlobalScope(
      v8::Isolate*,
      scoped_refptr<SecurityOrigin> origin,
      bool is_creator_secure_context,
      Agent* agent,
      const String& name,
      const base::UnguessableToken& parent_devtools_token,
      mojom::blink::V8CacheOptions,
      WorkerClients*,
      std::unique_ptr<WebContentSettingsClient>,
      scoped_refptr<WebWorkerFetchContext>,
      WorkerReportingProxy&,
      bool is_worker_loaded_from_data_url,
      bool is_default_world_of_isolate);
  ~WorkerOrWorkletGlobalScope() override;

  // EventTarget
  const AtomicString& InterfaceName() const override;

  // ScriptWrappable
  v8::Local<v8::Value> Wrap(ScriptState*) final;
  v8::Local<v8::Object> AssociateWithWrapper(
      v8::Isolate*,
      const WrapperTypeInfo*,
      v8::Local<v8::Object> wrapper) final;

  // ExecutionContext
  bool IsWorkerOrWorkletGlobalScope() const final { return true; }
  bool IsJSExecutionForbidden() const final;
  void DisableEval(const String& error_message) final;
  void SetWasmEvalErrorMessage(const String& error_message) final;
  bool CanExecuteScripts(ReasonForCallingCanExecuteScripts) final;
  bool HasInsecureContextInAncestors() const override;

  // scheduler::WorkerScheduler::Delegate
  void UpdateBackForwardCacheDisablingFeatures(
      BlockingDetails details) override {}

  // BackForwardCacheLoaderHelperImpl::Delegate
  void EvictFromBackForwardCache(
      mojom::blink::RendererEvictionReason reason,
      std::unique_ptr<SourceLocation> source_location) override {}
  void DidBufferLoadWhileInBackForwardCache(bool update_process_wide_count,
                                            size_t num_bytes) override {}

  // Returns true when the WorkerOrWorkletGlobalScope is closing (e.g. via
  // WorkerGlobalScope#close() method). If this returns true, the worker is
  // going to be shutdown after the current task execution. Globals that
  // don't support close operation should always return false.
  virtual bool IsClosing() const = 0;

  // Should be called before destroying the global scope object. Allows
  // sub-classes to perform any cleanup needed.
  virtual void Dispose();

  void SetModulator(Modulator*);

  // UseCounter
  void CountUse(WebFeature feature) final;
  void CountDeprecation(WebFeature feature) final;
  void CountWebDXFeature(WebDXFeature feature) final;

  // May return nullptr if this global scope is not threaded (i.e.,
  // WorkletGlobalScope for the main thread) or after Dispose() is called.
  virtual WorkerThread* GetThread() const = 0;

  // Returns nullptr if this global scope is a WorkletGlobalScope
  virtual WorkerNavigator* navigator() const { return nullptr; }

  // Returns the resource fetcher for subresources (a.k.a. inside settings
  // resource fetcher). See core/workers/README.md for details.
  ResourceFetcher* Fetcher() override;

  // ResourceFetcher for off-the-main-thread worker top-level script fetching,
  // corresponding to "outside" fetch client's settings object.
  // CreateOutsideSettingsFetcher() is called for each invocation of top-level
  // script fetch, which can occur multiple times in worklets.
  // TODO(hiroshige, nhiroki): Currently this outside ResourceFetcher and its
  // WorkerFetchContext is mostly the copy of the insideSettings
  // ResourceFetcher, and have dependencies to WorkerOrWorkletGlobalScope. Plumb
  // more data to the outside ResourceFetcher to fix the behavior and reduce the
  // dependencies.
  ResourceFetcher* CreateOutsideSettingsFetcher(
      const FetchClientSettingsObject&,
      WorkerResourceTimingNotifier&);

  const String Name() const { return name_; }
  const base::UnguessableToken& GetParentDevToolsToken() {
    return parent_devtools_token_;
  }
  virtual const base::UnguessableToken& GetDevToolsToken() const = 0;

  WorkerClients* Clients() const { return worker_clients_.Get(); }

  // May return nullptr.
  WebContentSettingsClient* ContentSettingsClient() const {
    return content_settings_client_.get();
  }

  WorkerOrWorkletScriptController* ScriptController() {
    return script_controller_.Get();
  }
  mojom::blink::V8CacheOptions GetV8CacheOptions() const override {
    return v8_cache_options_;
  }

  WorkerReportingProxy& ReportingProxy() { return reporting_proxy_; }

  void Trace(Visitor*) const override;

  scoped_refptr<base::SingleThreadTaskRunner> GetTaskRunner(TaskType) override;

  void SetSandboxFlags(network::mojom::blink::WebSandboxFlags mask);

  void SetDefersLoadingForResourceFetchers(LoaderFreezeMode);

  virtual int GetOutstandingThrottledLimit() const;

  // TODO(crbug/964467): Currently all workers fetch cached code but only
  // services workers use them. Dedicated / Shared workers don't use the cached
  // code since we don't create a CachedMetadataHandler. We need to fix this by
  // creating a cached metadta handler for all workers.
  virtual CodeCacheHost* GetCodeCacheHost() { return nullptr; }

  // Called after the thread initialization is complete but before the script
  // has started loading.
  virtual void WillBeginLoading() {}

  Deprecation& GetDeprecation() { return deprecation_; }

  // Returns the current list of user preferred languages.
  String GetAcceptLanguages() const;

  // Called when a console message is recorded via the console API. This
  // will invoke `WorkerReportingProxy::ReportConsoleMessage()`.
  virtual void OnConsoleApiMessage(mojom::ConsoleMessageLevel level,
                                   const String& message,
                                   SourceLocation* location);

  // Called when BestEffortServiceWorker(crbug.com/1420517) is enabled.
  virtual std::optional<
      mojo::PendingRemote<network::mojom::blink::URLLoaderFactory>>
  FindRaceNetworkRequestURLLoaderFactory(
      const base::UnguessableToken& token) = 0;

 protected:
  // Sets outside's CSP used for off-main-thread top-level worker script
  // fetch.
  void SetOutsideContentSecurityPolicies(
      Vector<network::mojom::blink::ContentSecurityPolicyPtr>);

  // Initializes inside's CSP used for subresource fetch etc.
  void InitContentSecurityPolicyFromVector(
      Vector<network::mojom::blink::ContentSecurityPolicyPtr> policies);
  virtual void BindContentSecurityPolicyToExecutionContext();

  void FetchModuleScript(const KURL& module_url_record,
                         const FetchClientSettingsObjectSnapshot&,
                         WorkerResourceTimingNotifier&,
                         mojom::blink::RequestContextType context_type,
                         network::mojom::RequestDestination destination,
                         network::mojom::CredentialsMode,
                         ModuleScriptCustomFetchType,
                         ModuleTreeClient*);

  const Vector<network::mojom::blink::ContentSecurityPolicyPtr>&
  OutsideContentSecurityPolicies() const {
    return outside_content_security_policies_;
  }

  WebWorkerFetchContext* web_worker_fetch_context() const {
    return web_worker_fetch_context_.get();
  }

  virtual ResourceLoadScheduler::ThrottleOptionOverride
  GetThrottleOptionOverride() const;

  // This method must be call after the fetcher is created if conditions
  // change such that a different ThrottleOptionOverride should be applied.
  void UpdateFetcherThrottleOptionOverride();

  bool IsCreatorSecureContext() const { return is_creator_secure_context_; }

 private:
  void InitializeWebFetchContextIfNeeded();
  ResourceFetcher* CreateFetcherInternal(const FetchClientSettingsObject&,
                                         ContentSecurityPolicy&,
                                         WorkerResourceTimingNotifier&);

  bool web_fetch_context_initialized_ = false;

  // Whether the creator execution context is secure.
  const bool is_creator_secure_context_ = false;

  const String name_;
  const base::UnguessableToken parent_devtools_token_;

  CrossThreadPersistent<WorkerClients> worker_clients_;
  std::unique_ptr<WebContentSettingsClient> content_settings_client_;

  Member<ResourceFetcher> inside_settings_resource_fetcher_;

  // Keeps track of all ResourceFetchers (including
  // |inside_settings_resource_fetcher_|) for disposing and pausing/unpausing.
  HeapHashSet<WeakMember<ResourceFetcher>> resource_fetchers_;

  // A WorkerOrWorkletGlobalScope has one WebWorkerFetchContext and one
  // corresponding SubresourceFilter, which are shared by all
  // WorkerFetchContexts of |this| global scope, i.e. those behind
  // ResourceFetchers created by EnsureFetcher() and
  // CreateOutsideSettingsFetcher().
  // As all references to |web_worker_fetch_context_| are on the context
  // thread, |web_worker_fetch_context_| is destructed on the context
  // thread.
  //
  // TODO(crbug/903579): Consider putting WebWorkerFetchContext-originated
  // things at a single place. Currently they are placed here and subclasses of
  // WebWorkerFetchContext.
  const scoped_refptr<WebWorkerFetchContext> web_worker_fetch_context_;
  Member<SubresourceFilter> subresource_filter_;

  Member<WorkerOrWorkletScriptController> script_controller_;
  const mojom::blink::V8CacheOptions v8_cache_options_;

  // TODO(hiroshige): Pass outsideSettings-CSP via
  // outsideSettings-FetchClientSettingsObject.
  Vector<network::mojom::blink::ContentSecurityPolicyPtr>
      outside_content_security_policies_;

  WorkerReportingProxy& reporting_proxy_;

  // This is the set of features that this worker has used.
  std::bitset<static_cast<size_t>(WebFeature::kMaxValue) + 1> used_features_;
  // This is the set of WebDXFeatures that this worker has used.
  std::bitset<static_cast<size_t>(WebDXFeature::kMaxValue) + 1>
      used_webdx_features_;

  // This tracks deprecation features that have been used.
  Deprecation deprecation_;
};

template <>
struct DowncastTraits<WorkerOrWorkletGlobalScope> {
  static bool AllowFrom(const ExecutionContext& context) {
    return context.IsWorkerGlobalScope() || context.IsWorkletGlobalScope();
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_WORKERS_WORKER_OR_WORKLET_GLOBAL_SCOPE_H_
