// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_WORKERS_GLOBAL_SCOPE_CREATION_PARAMS_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_WORKERS_GLOBAL_SCOPE_CREATION_PARAMS_H_

#include <memory>
#include <optional>

#include "base/task/single_thread_task_runner.h"
#include "base/unguessable_token.h"
#include "mojo/public/cpp/bindings/pending_remote.h"
#include "net/storage_access_api/status.h"
#include "services/metrics/public/cpp/ukm_source_id.h"
#include "services/network/public/cpp/permissions_policy/permissions_policy.h"
#include "services/network/public/mojom/referrer_policy.mojom-blink-forward.h"
#include "third_party/blink/public/common/tokens/tokens.h"
#include "third_party/blink/public/common/user_agent/user_agent_metadata.h"
#include "third_party/blink/public/mojom/blob/blob_url_store.mojom-blink-forward.h"
#include "third_party/blink/public/mojom/browser_interface_broker.mojom-blink-forward.h"
#include "third_party/blink/public/mojom/frame/reporting_observer.mojom-blink-forward.h"
#include "third_party/blink/public/mojom/loader/code_cache.mojom-blink-forward.h"
#include "third_party/blink/public/mojom/script/script_type.mojom-blink-forward.h"
#include "third_party/blink/public/mojom/v8_cache_options.mojom-blink.h"
#include "third_party/blink/public/platform/web_content_settings_client.h"
#include "third_party/blink/public/platform/web_worker_fetch_context.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/frame/csp/content_security_policy.h"
#include "third_party/blink/renderer/core/workers/worker_clients.h"
#include "third_party/blink/renderer/core/workers/worker_settings.h"
#include "third_party/blink/renderer/core/workers/worklet_module_responses_map.h"
#include "third_party/blink/renderer/platform/graphics/begin_frame_provider_params.h"
#include "third_party/blink/renderer/platform/loader/fetch/https_state.h"
#include "third_party/blink/renderer/platform/weborigin/kurl.h"
#include "third_party/blink/renderer/platform/wtf/forward.h"

namespace blink {

class InterfaceRegistry;
class WorkerClients;

// GlobalScopeCreationParams contains parameters for initializing
// WorkerGlobalScope or WorkletGlobalScope.
struct CORE_EXPORT GlobalScopeCreationParams final {
  USING_FAST_MALLOC(GlobalScopeCreationParams);

 public:
  GlobalScopeCreationParams(
      const KURL& script_url,
      mojom::blink::ScriptType script_type,
      const String& global_scope_name,
      const String& user_agent,
      const std::optional<UserAgentMetadata>& ua_metadata,
      scoped_refptr<WebWorkerFetchContext>,
      Vector<network::mojom::blink::ContentSecurityPolicyPtr>
          outside_content_security_policies,
      Vector<network::mojom::blink::ContentSecurityPolicyPtr>
          response_content_security_policies,
      network::mojom::ReferrerPolicy referrer_policy,
      const SecurityOrigin*,
      bool starter_secure_context,
      HttpsState starter_https_state,
      WorkerClients*,
      std::unique_ptr<WebContentSettingsClient>,
      const Vector<mojom::blink::OriginTrialFeature>* inherited_trial_features,
      const base::UnguessableToken& parent_devtools_token,
      std::unique_ptr<WorkerSettings>,
      mojom::blink::V8CacheOptions,
      WorkletModuleResponsesMap*,
      mojo::PendingRemote<mojom::blink::BrowserInterfaceBroker>
          browser_interface_broker = mojo::NullRemote(),
      mojo::PendingRemote<mojom::blink::CodeCacheHost> code_cahe_host =
          mojo::NullRemote(),
      mojo::PendingRemote<mojom::blink::BlobURLStore> blob_url_store =
          mojo::NullRemote(),
      BeginFrameProviderParams begin_frame_provider_params = {},
      const network::PermissionsPolicy* parent_permissions_policy = nullptr,
      base::UnguessableToken agent_cluster_id = {},
      ukm::SourceId ukm_source_id = ukm::kInvalidSourceId,
      const std::optional<ExecutionContextToken>& parent_context_token =
          std::nullopt,
      bool parent_cross_origin_isolated_capability = false,
      bool parent_is_isolated_context = false,
      InterfaceRegistry* interface_registry = nullptr,
      scoped_refptr<base::SingleThreadTaskRunner>
          agent_group_scheduler_compositor_task_runner = nullptr,
      const SecurityOrigin* top_level_frame_security_origin = nullptr,
      net::StorageAccessApiStatus parent_storage_access_api_status =
          net::StorageAccessApiStatus::kNone,
      bool require_cross_site_request_for_cookies = false,
      scoped_refptr<SecurityOrigin> origin_to_use = nullptr,
      mojo::PendingReceiver<mojom::blink::ReportingObserver>
          coep_reporting_observer = mojo::NullReceiver(),
      mojo::PendingReceiver<mojom::blink::ReportingObserver>
          dip_reporting_observer = mojo::NullReceiver());
  GlobalScopeCreationParams(const GlobalScopeCreationParams&) = delete;
  GlobalScopeCreationParams& operator=(const GlobalScopeCreationParams&) =
      delete;

  ~GlobalScopeCreationParams() = default;

  // The URL to be used as the worker global scope's URL.
  // According to the spec, this should be response URL of the top-level
  // worker script after the top-level worker script is loaded.
  // https://html.spec.whatwg.org/C/#run-a-worker
  //
  // However, this can't be set to response URL in case of module workers or
  // off-the-main-thread fetch, because at the time of GlobalScopeCreationParams
  // creation the response of worker script is not yet received. Therefore,
  // the worker global scope's URL should be set to the response URL outside
  // GlobalScopeCreationParams, but this mechanism is not yet implemented.
  // TODO(crbug/861564): implement this and set the response URL to module
  // workers.
  KURL script_url;

  mojom::blink::ScriptType script_type;

  String global_scope_name;
  String user_agent;
  UserAgentMetadata ua_metadata;

  scoped_refptr<WebWorkerFetchContext> web_worker_fetch_context;

  Vector<network::mojom::blink::ContentSecurityPolicyPtr>
      outside_content_security_policies;

  // This is used only for classic dedicated workers with off-the-main-thread
  // fetch disabled.
  //
  // TODO(https://crbug.com/835717): Remove this after dedicated workers support
  // off-the-main-thread script fetch by default.
  Vector<network::mojom::blink::ContentSecurityPolicyPtr>
      response_content_security_policies;

  network::mojom::ReferrerPolicy referrer_policy;

  // Origin trial features to be inherited by worker/worklet from the document
  // loading it.
  std::unique_ptr<Vector<mojom::blink::OriginTrialFeature>>
      inherited_trial_features;

  // The SecurityOrigin of the Document creating a Worker/Worklet.
  //
  // For Workers, the origin may have been configured with extra policy
  // privileges when it was created (e.g., enforce path-based file:// origins.)
  // To ensure that these are transferred to the origin of a new worker global
  // scope, supply the Document's SecurityOrigin as the 'starter origin'. See
  // SecurityOrigin::TransferPrivilegesFrom() for details on what privileges are
  // transferred.
  //
  // For Worklets, the origin is used for fetching module scripts. Worklet
  // scripts need to be fetched as sub-resources of the Document, and a module
  // script loader uses Document's SecurityOrigin for security checks.
  scoped_refptr<const SecurityOrigin> starter_origin;

  // The SecurityOrigin to be used by the worker, if it's pre-calculated
  // already (e.g. passed down from the browser to the renderer). Only set
  // for dedicated and shared workers. The origin is calculated in the browser
  // process and sent to the renderer. This guarantees both the renderer and
  // browser knows the exact origin used by the worker.
  scoped_refptr<SecurityOrigin> origin_to_use;

  // Indicates if the Document creating a Worker/Worklet is a secure context.
  //
  // Worklets are defined to have a unique, opaque origin, so are not secure:
  // https://drafts.css-houdini.org/worklets/#script-settings-for-worklets
  // Origin trials are only enabled in secure contexts so also consider the
  // context of the document. The value should be supplied as the result of
  // Document.IsSecureContext().
  bool starter_secure_context;

  HttpsState starter_https_state;

  // This object is created and initialized on the thread creating
  // a new worker context, but ownership of it and this
  // GlobalScopeCreationParams structure is passed along to the new worker
  // thread, where it is finalized.
  //
  // Hence, CrossThreadPersistent<> is required to allow finalization
  // to happen on a thread different than the thread creating the
  // persistent reference. If the worker thread creation context
  // supplies no extra 'clients', m_workerClients can be left as empty/null.
  CrossThreadPersistent<WorkerClients> worker_clients;

  std::unique_ptr<WebContentSettingsClient> content_settings_client;

  base::UnguessableToken parent_devtools_token;

  std::unique_ptr<WorkerSettings> worker_settings;

  mojom::blink::V8CacheOptions v8_cache_options;

  CrossThreadPersistent<WorkletModuleResponsesMap> module_responses_map;

  mojo::PendingRemote<mojom::blink::BrowserInterfaceBroker>
      browser_interface_broker;

  mojo::PendingRemote<mojom::blink::CodeCacheHost> code_cache_host_interface;

  mojo::PendingRemote<mojom::blink::BlobURLStore> blob_url_store;

  BeginFrameProviderParams begin_frame_provider_params;

  std::unique_ptr<network::PermissionsPolicy> worker_permissions_policy;

  // Set when the worker/worklet has the same AgentClusterID as the execution
  // context that created it (e.g. for a dedicated worker).
  // See https://tc39.github.io/ecma262/#sec-agent-clusters
  base::UnguessableToken agent_cluster_id;

  // Set to ukm::kInvalidSourceId when the global scope is not provided an ID.
  ukm::SourceId ukm_source_id;

  // The identity of the parent ExecutionContext that is the sole owner of this
  // worker or worklet, which caused it to be created, and to whose lifetime
  // this worker/worklet is bound. This is used for resource usage attribution.
  std::optional<ExecutionContextToken> parent_context_token;

  // https://html.spec.whatwg.org/C/#concept-settings-object-cross-origin-isolated-capability
  // Used by dedicated workers, and set to false when there is no parent.
  const bool parent_cross_origin_isolated_capability;

  // Governs whether Direct Sockets are available in a worker context, false
  // when no parent exists.
  //
  // TODO(crbug.com/1206150): We need a specification for this capability.
  const bool parent_is_isolated_context;

  InterfaceRegistry* const interface_registry;

  // The compositor task runner associated with the |AgentGroupScheduler| this
  // worker belongs to.
  scoped_refptr<base::SingleThreadTaskRunner>
      agent_group_scheduler_compositor_task_runner;

  // The security origin of the top level frame associated with the worker. This
  // can be used, for instance, to check if the top level frame has an opaque
  // origin.
  scoped_refptr<const SecurityOrigin> top_level_frame_security_origin;

  // Timestamp of the dedicated worker start.
  // i.e. when DedicatedWorkerStart() was called.
  std::optional<base::TimeTicks> dedicated_worker_start_time;

  // The parent ExecutionContext's Storage Access API status.
  const net::StorageAccessApiStatus parent_storage_access_api_status;

  // Late initialized on thread creation. This signals whether the world created
  // is the default world for an isolate.
  bool is_default_world_of_isolate = false;

  // If `require_cross_site_request_for_cookies` is specified, then all requests
  // made must have an empty site_for_cookies to ensure only SameSite=None
  // cookies can be attached to the request.
  // For context on usage see:
  // https://privacycg.github.io/saa-non-cookie-storage/shared-workers.html
  const bool require_cross_site_request_for_cookies;

  // Used by COEP and DocumentIsolationPolicy reporting to trigger
  // ReportingObserver events.
  mojo::PendingReceiver<mojom::blink::ReportingObserver>
      coep_reporting_observer;
  mojo::PendingReceiver<mojom::blink::ReportingObserver> dip_reporting_observer;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_WORKERS_GLOBAL_SCOPE_CREATION_PARAMS_H_
