/*
 * Copyright (C) 2008, 2009 Apple Inc. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 * 2. Redistributions in binary form must reproduce the above copyright
 *
 * THIS SOFTWARE IS PROVIDED BY APPLE COMPUTER, INC. ``AS IS'' AND ANY
 * EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR
 * PURPOSE ARE DISCLAIMED.  IN NO EVENT SHALL APPLE COMPUTER, INC. OR
 * CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
 * EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
 * PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
 * PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY
 * OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 *
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_WORKERS_WORKER_GLOBAL_SCOPE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_WORKERS_WORKER_GLOBAL_SCOPE_H_

#include <memory>

#include "base/task/single_thread_task_runner.h"
#include "base/time/time.h"
#include "services/network/public/mojom/fetch_api.mojom-blink-forward.h"
#include "third_party/blink/public/common/loader/worker_main_script_load_parameters.h"
#include "third_party/blink/public/common/tokens/tokens.h"
#include "third_party/blink/public/mojom/loader/code_cache.mojom-blink-forward.h"
#include "third_party/blink/public/mojom/script/script_type.mojom-blink-forward.h"
#include "third_party/blink/public/platform/browser_interface_broker_proxy.h"
#include "third_party/blink/renderer/bindings/core/v8/active_script_wrappable.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/dom/frame_request_callback_collection.h"
#include "third_party/blink/renderer/core/execution_context/execution_context.h"
#include "third_party/blink/renderer/core/frame/csp/content_security_policy.h"
#include "third_party/blink/renderer/core/frame/policy_container.h"
#include "third_party/blink/renderer/core/frame/universal_global_scope.h"
#include "third_party/blink/renderer/core/frame/window_or_worker_global_scope.h"
#include "third_party/blink/renderer/core/script/script.h"
#include "third_party/blink/renderer/core/workers/custom_event_message.h"
#include "third_party/blink/renderer/core/workers/worker_classic_script_loader.h"
#include "third_party/blink/renderer/core/workers/worker_or_worklet_global_scope.h"
#include "third_party/blink/renderer/core/workers/worker_settings.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_map.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/loader/fetch/code_cache_host.h"
#include "third_party/blink/renderer/platform/loader/fetch/url_loader/cached_metadata_handler.h"
#include "third_party/blink/renderer/platform/mojo/browser_interface_broker_proxy_impl.h"
#include "third_party/blink/renderer/platform/weborigin/security_origin.h"
#include "third_party/blink/renderer/platform/wtf/casting.h"
#include "third_party/blink/renderer/platform/wtf/gc_plugin.h"
#include "v8/include/v8-inspector.h"

namespace blink {

struct BlinkTransferableMessage;
struct GlobalScopeCreationParams;
class ConsoleMessage;
class FetchClientSettingsObjectSnapshot;
class FontFaceSet;
class FontMatchingMetrics;
struct GlobalScopeCreationParams;
class InstalledScriptsManager;
class OffscreenFontSelector;
class WorkerResourceTimingNotifier;
class TrustedTypePolicyFactory;
class WorkerLocation;
class WorkerNavigator;
class WorkerThread;

class CORE_EXPORT WorkerGlobalScope
    : public WorkerOrWorkletGlobalScope,
      public WindowOrWorkerGlobalScope,
      public UniversalGlobalScope,
      public ActiveScriptWrappable<WorkerGlobalScope>,
      public Supplementable<WorkerGlobalScope> {
  DEFINE_WRAPPERTYPEINFO();

 public:
  ~WorkerGlobalScope() override;

  // Returns null if caching is not supported.
  // TODO(crbug/964467): Currently workers do fetch cached code but they don't
  // use it because we don't create a CachedMetadtaHandler. Only service workers
  // override this method and provide a valid handler. We need to implement it
  // for Dedicated / Shared workers too so we can benefit from code caches.
  virtual CachedMetadataHandler* CreateWorkerScriptCachedMetadataHandler(
      const KURL& script_url,
      std::unique_ptr<Vector<uint8_t>> meta_data) {
    return nullptr;
  }

  // WorkerOrWorkletGlobalScope
  bool IsClosing() const final { return closing_; }
  void Dispose() override;
  WorkerThread* GetThread() const final { return thread_; }
  const base::UnguessableToken& GetDevToolsToken() const override;
  CodeCacheHost* GetCodeCacheHost() override;
  std::optional<mojo::PendingRemote<network::mojom::blink::URLLoaderFactory>>
  FindRaceNetworkRequestURLLoaderFactory(
      const base::UnguessableToken& token) override {
    return std::nullopt;
  }

  void ExceptionUnhandled(int exception_id);

  // WorkerGlobalScope
  WorkerGlobalScope* self() { return this; }
  WorkerLocation* location() const;
  WorkerNavigator* navigator() const override;
  void close();

  String origin() const;

  DEFINE_ATTRIBUTE_EVENT_LISTENER(error, kError)
  DEFINE_ATTRIBUTE_EVENT_LISTENER(languagechange, kLanguagechange)
  DEFINE_ATTRIBUTE_EVENT_LISTENER(rejectionhandled, kRejectionhandled)
  DEFINE_ATTRIBUTE_EVENT_LISTENER(timezonechange, kTimezonechange)
  DEFINE_ATTRIBUTE_EVENT_LISTENER(unhandledrejection, kUnhandledrejection)

  // This doesn't take an ExceptionState argument, but actually can throw
  // exceptions directly to V8 (crbug/1114610).
  virtual void importScripts(const Vector<String>& urls);

  // ExecutionContext
  const KURL& Url() const final;
  KURL CompleteURL(const String&) const final;
  bool IsWorkerGlobalScope() const final { return true; }
  bool IsContextThread() const final;
  const KURL& BaseURL() const final;
  String UserAgent() const final { return user_agent_; }
  UserAgentMetadata GetUserAgentMetadata() const override {
    return ua_metadata_;
  }
  HttpsState GetHttpsState() const override { return https_state_; }
  scheduler::WorkerScheduler* GetScheduler() final;
  ukm::UkmRecorder* UkmRecorder() final;
  ScriptWrappable* ToScriptWrappable() final { return this; }

  void AddConsoleMessageImpl(ConsoleMessage*, bool discard_duplicates) final;
  const BrowserInterfaceBrokerProxyImpl& GetBrowserInterfaceBroker()
      const final;

  scoped_refptr<base::SingleThreadTaskRunner>
  GetAgentGroupSchedulerCompositorTaskRunner() final {
    return agent_group_scheduler_compositor_task_runner_;
  }

  OffscreenFontSelector* GetFontSelector() { return font_selector_.Get(); }

  CoreProbeSink* GetProbeSink() final;

  // EventTarget
  ExecutionContext* GetExecutionContext() const final;
  bool IsWindowOrWorkerGlobalScope() const final { return true; }

  // Initializes this global scope. This must be called after worker script
  // fetch, and before initiali script evaluation.
  //
  // This corresponds to following specs:
  // - For dedicated/shared workers, step 12.3-12.6 (a custom perform the fetch
  //   hook) in https://html.spec.whatwg.org/C/#run-a-worker
  // - For service workers, step 4.5-4.11 in
  //   https://w3c.github.io/ServiceWorker/#run-service-worker-algorithm
  virtual void Initialize(
      const KURL& response_url,
      network::mojom::ReferrerPolicy response_referrer_policy,
      Vector<network::mojom::blink::ContentSecurityPolicyPtr> response_csp,
      const Vector<String>* response_origin_trial_tokens) = 0;

  // These methods should be called in the scope of a pausable
  // task runner. ie. They should not be called when the context
  // is paused.
  void EvaluateClassicScript(const KURL& script_url,
                             String source_code,
                             std::unique_ptr<Vector<uint8_t>> cached_meta_data,
                             const v8_inspector::V8StackTraceId& stack_id);

  // Should be called (in all successful cases) when the worker top-level
  // script fetch is finished.
  // At this time, WorkerGlobalScope::Initialize() should be already called.
  // Spec: https://html.spec.whatwg.org/C/#run-a-worker Step 12 is completed,
  // and it's ready to proceed to Step 23.
  virtual void WorkerScriptFetchFinished(
      Script&,
      std::optional<v8_inspector::V8StackTraceId>);

  // Fetches and evaluates the top-level classic script.
  virtual void FetchAndRunClassicScript(
      const KURL& script_url,
      std::unique_ptr<WorkerMainScriptLoadParameters>
          worker_main_script_load_params_for_modules,
      std::unique_ptr<PolicyContainer> policy_container,
      const FetchClientSettingsObjectSnapshot& outside_settings_object,
      WorkerResourceTimingNotifier& outside_resource_timing_notifier,
      const v8_inspector::V8StackTraceId& stack_id) = 0;

  // Fetches and evaluates the top-level module script.
  virtual void FetchAndRunModuleScript(
      const KURL& module_url_record,
      std::unique_ptr<WorkerMainScriptLoadParameters>
          worker_main_script_load_params_for_modules,
      std::unique_ptr<PolicyContainer> policy_container,
      const FetchClientSettingsObjectSnapshot& outside_settings_object,
      WorkerResourceTimingNotifier& outside_resource_timing_notifier,
      network::mojom::CredentialsMode) = 0;

  void ReceiveMessage(BlinkTransferableMessage);
  Event* ReceiveCustomEventInternal(
      CrossThreadFunction<Event*(ScriptState*, CustomEventMessage)>,
      CrossThreadFunction<Event*(ScriptState*)>,
      CustomEventMessage);
  void ReceiveCustomEvent(
      CrossThreadFunction<Event*(ScriptState*, CustomEventMessage)>
          event_factory_callback,
      CrossThreadFunction<Event*(ScriptState*)> event_factory_error_callback,
      CustomEventMessage);
  base::TimeTicks TimeOrigin() const { return time_origin_; }
  WorkerSettings* GetWorkerSettings() const { return worker_settings_.get(); }

  void Trace(Visitor*) const override;

  // ActiveScriptWrappable.
  bool HasPendingActivity() const override;

  virtual InstalledScriptsManager* GetInstalledScriptsManager() {
    return nullptr;
  }

  FontFaceSet* fonts();

  TrustedTypePolicyFactory* GetTrustedTypes() const override;
  TrustedTypePolicyFactory* trustedTypes() const { return GetTrustedTypes(); }

  // TODO(https://crbug.com/835717): Remove this function after dedicated
  // workers support off-the-main-thread script fetch by default.
  virtual bool IsOffMainThreadScriptFetchDisabled() { return false; }

  // Takes the ownership of the parameters used to load the worker main module
  // script in renderer process.
  std::unique_ptr<WorkerMainScriptLoadParameters>
  TakeWorkerMainScriptLoadingParametersForModules();

  ukm::SourceId UkmSourceID() const override { return ukm_source_id_; }

  // Returns the token uniquely identifying this worker. The token type will
  // match the actual worker type.
  virtual WorkerToken GetWorkerToken() const = 0;

  // Tracks and reports metrics of attempted font match attempts (both
  // successful and not successful) by the worker.
  FontMatchingMetrics* GetFontMatchingMetrics();

  bool IsUrlValid() { return url_.IsValid(); }

  void SetMainResoureIdentifier(uint64_t identifier) {
    DCHECK(!main_resource_identifier_.has_value());
    main_resource_identifier_ = identifier;
  }

  std::optional<uint64_t> MainResourceIdentifier() const {
    return main_resource_identifier_;
  }

  const SecurityOrigin* top_level_frame_security_origin() const {
    return top_level_frame_security_origin_.get();
  }

 protected:
  WorkerGlobalScope(std::unique_ptr<GlobalScopeCreationParams>,
                    WorkerThread*,
                    base::TimeTicks time_origin,
                    bool is_service_worker_global_scope);

  // ExecutionContext
  void ExceptionThrown(ErrorEvent*) override;
  void RemoveURLFromMemoryCache(const KURL&) final;

  virtual bool FetchClassicImportedScript(
      const KURL& script_url,
      KURL* out_response_url,
      String* out_source_code,
      std::unique_ptr<Vector<uint8_t>>* out_cached_meta_data);

  // Notifies that the top-level worker script is ready to evaluate.
  // Worker top-level script is evaluated after it is fetched and
  // ReadyToRunWorkerScript() is called.
  void ReadyToRunWorkerScript();

  void InitializeURL(const KURL& url);

  mojom::blink::ScriptType GetScriptType() const { return script_type_; }

  // Sets the parameters for the worker main module script loaded by the browser
  // process.
  void SetWorkerMainScriptLoadingParametersForModules(
      std::unique_ptr<WorkerMainScriptLoadParameters>
          worker_main_script_load_params_for_modules);

 private:
  void SetWorkerSettings(std::unique_ptr<WorkerSettings>);

  // https://html.spec.whatwg.org/C/#run-a-worker Step 24.
  void RunWorkerScript();

  // Used for importScripts().
  void ImportScriptsInternal(const Vector<String>& urls);
  // ExecutionContext
  void AddInspectorIssue(AuditsIssue) final;
  EventTarget* ErrorEventTarget() final { return this; }

  // WorkerOrWorkletGlobalScope
  void WillBeginLoading() override;

  KURL url_;
  const mojom::blink::ScriptType script_type_;
  const String user_agent_;
  const UserAgentMetadata ua_metadata_;
  std::unique_ptr<WorkerSettings> worker_settings_;

  mutable Member<WorkerLocation> location_;
  mutable Member<WorkerNavigator> navigator_;
  mutable Member<TrustedTypePolicyFactory> trusted_types_;

  WorkerThread* thread_;

  // The compositor task runner associated with the |AgentGroupScheduler| this
  // worker belongs to.
  scoped_refptr<base::SingleThreadTaskRunner>
      agent_group_scheduler_compositor_task_runner_;

  bool closing_ = false;

  const base::TimeTicks time_origin_;

  HeapHashMap<int, Member<ErrorEvent>> pending_error_events_;
  int last_pending_error_event_id_ = 0;

  Member<OffscreenFontSelector> font_selector_;

  // Tracks and reports UKM metrics of the number of attempted font family match
  // attempts (both successful and not successful) by the worker.
  std::unique_ptr<FontMatchingMetrics> font_matching_metrics_;

  blink::BrowserInterfaceBrokerProxyImpl browser_interface_broker_proxy_;

  // State transition about worker top-level script evaluation.
  enum class ScriptEvalState {
    // Initial state: ReadyToRunWorkerScript() was not yet called.
    // Worker top-level script fetch might or might not be completed, and even
    // when the fetch completes in this state, script evaluation will be
    // deferred to when ReadyToRunWorkerScript() is called later.
    kPauseAfterFetch,
    // ReadyToRunWorkerScript() was already called.
    kReadyToEvaluate,
    // The worker top-level script was evaluated.
    kEvaluated,
  };
  ScriptEvalState script_eval_state_;

  Member<Script> worker_script_;
  std::optional<v8_inspector::V8StackTraceId> stack_id_;

  HttpsState https_state_;

  std::unique_ptr<ukm::UkmRecorder> ukm_recorder_;

  // `worker_main_script_load_params_for_modules_` is used to load a root module
  // script for dedicated workers and shared workers.
  std::unique_ptr<WorkerMainScriptLoadParameters>
      worker_main_script_load_params_for_modules_;

  // |main_resource_identifier_| is used to track main script that was started
  // in the browser process. This field not having a value does not imply
  // anything.
  std::optional<uint64_t> main_resource_identifier_;

  // This is the interface that handles generated code cache
  // requests both to fetch code cache when loading resources.
  std::unique_ptr<CodeCacheHost> code_cache_host_;

  const ukm::SourceId ukm_source_id_;

  // Pauses virtual time from the time the thread has initialized (including
  // DevTools agents being configured while waiting for debugger) till the main
  // script has completed loading. This is so that VT does not run while script
  // is being loaded.
  WebScopedVirtualTimePauser loading_virtual_time_pauser_;

  // The security origin of the top level frame associated with the worker. This
  // can be used, for instance, to check if the top level frame has an opaque
  // origin.
  scoped_refptr<const SecurityOrigin> top_level_frame_security_origin_;
};

template <>
struct DowncastTraits<WorkerGlobalScope> {
  static bool AllowFrom(const ExecutionContext& context) {
    return context.IsWorkerGlobalScope();
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_WORKERS_WORKER_GLOBAL_SCOPE_H_
