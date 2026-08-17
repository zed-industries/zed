/*
 * Copyright (C) 2009 Google Inc. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are
 * met:
 *
 *     * Redistributions of source code must retain the above copyright
 * notice, this list of conditions and the following disclaimer.
 *     * Redistributions in binary form must reproduce the above
 * copyright notice, this list of conditions and the following disclaimer
 * in the documentation and/or other materials provided with the
 * distribution.
 *     * Neither the name of Google Inc. nor the names of its
 * contributors may be used to endorse or promote products derived from
 * this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 * "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 * LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
 * A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
 * OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 * SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
 * LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
 * DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
 * THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_WORKERS_DEDICATED_WORKER_GLOBAL_SCOPE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_WORKERS_DEDICATED_WORKER_GLOBAL_SCOPE_H_

#include <memory>

#include "base/types/pass_key.h"
#include "net/storage_access_api/status.h"
#include "third_party/blink/public/common/tokens/tokens.h"
#include "third_party/blink/public/mojom/frame/back_forward_cache_controller.mojom-blink.h"
#include "third_party/blink/public/mojom/worker/dedicated_worker_host.mojom-blink.h"
#include "third_party/blink/renderer/core/animation_frame/worker_animation_frame_provider.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/messaging/message_port.h"
#include "third_party/blink/renderer/core/workers/worker_global_scope.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_set.h"
#include "third_party/blink/renderer/platform/heap/visitor.h"
#include "third_party/blink/renderer/platform/wtf/casting.h"

namespace blink {

class DedicatedWorkerObjectProxy;
class DedicatedWorkerThread;
class PostMessageOptions;
class ScriptState;
class SourceLocation;
class WorkerClassicScriptLoader;
struct GlobalScopeCreationParams;

class CORE_EXPORT DedicatedWorkerGlobalScope final : public WorkerGlobalScope {
  DEFINE_WRAPPERTYPEINFO();

 public:
  // TODO(nhiroki): Merge Create() into the constructor after
  // off-the-main-thread worker script fetch is enabled by default.
  static DedicatedWorkerGlobalScope* Create(
      std::unique_ptr<GlobalScopeCreationParams>,
      DedicatedWorkerThread*,
      base::TimeTicks time_origin,
      mojo::PendingRemote<mojom::blink::DedicatedWorkerHost>
          dedicated_worker_host,
      mojo::PendingRemote<mojom::blink::BackForwardCacheControllerHost>
          back_forward_cache_controller_host);

  // Do not call this. Use Create() instead. This is public only for
  // MakeGarbageCollected.
  DedicatedWorkerGlobalScope(
      base::PassKey<DedicatedWorkerGlobalScope>,
      std::unique_ptr<GlobalScopeCreationParams>,
      DedicatedWorkerThread*,
      base::TimeTicks time_origin,
      std::unique_ptr<Vector<mojom::blink::OriginTrialFeature>>
          inherited_trial_features,
      const BeginFrameProviderParams& begin_frame_provider_params,
      bool parent_cross_origin_isolated_capability,
      bool direct_socket_isolated_capability,
      mojo::PendingRemote<mojom::blink::DedicatedWorkerHost>
          dedicated_worker_host,
      mojo::PendingRemote<mojom::blink::BackForwardCacheControllerHost>
          back_forward_cache_controller_host,
      base::TimeTicks dedicated_worker_start_time);

  ~DedicatedWorkerGlobalScope() override;

  // Implements ExecutionContext.
  void SetIsInBackForwardCache(bool) override;
  bool IsDedicatedWorkerGlobalScope() const override { return true; }
  net::StorageAccessApiStatus GetStorageAccessApiStatus() const override;

  // Implements EventTarget
  // (via WorkerOrWorkletGlobalScope -> EventTarget).
  const AtomicString& InterfaceName() const override;

  // RequestAnimationFrame
  int requestAnimationFrame(V8FrameRequestCallback* callback, ExceptionState&);
  void cancelAnimationFrame(int id);
  WorkerAnimationFrameProvider* GetAnimationFrameProvider() {
    return animation_frame_provider_.Get();
  }

  // Implements WorkerGlobalScope.
  void Dispose() override;
  void Initialize(
      const KURL& response_url,
      network::mojom::ReferrerPolicy response_referrer_policy,
      Vector<network::mojom::blink::ContentSecurityPolicyPtr> response_csp,
      const Vector<String>* response_origin_trial_tokens) override;
  void FetchAndRunClassicScript(
      const KURL& script_url,
      std::unique_ptr<WorkerMainScriptLoadParameters>
          worker_main_script_load_params,
      std::unique_ptr<PolicyContainer> policy_container,
      const FetchClientSettingsObjectSnapshot& outside_settings_object,
      WorkerResourceTimingNotifier& outside_resource_timing_notifier,
      const v8_inspector::V8StackTraceId& stack_id) override;
  void FetchAndRunModuleScript(
      const KURL& module_url_record,
      std::unique_ptr<WorkerMainScriptLoadParameters>
          worker_main_script_load_params,
      std::unique_ptr<PolicyContainer> policy_container,
      const FetchClientSettingsObjectSnapshot& outside_settings_object,
      WorkerResourceTimingNotifier& outside_resource_timing_notifier,
      network::mojom::CredentialsMode) override;
  bool IsOffMainThreadScriptFetchDisabled() override;
  void WorkerScriptFetchFinished(
      Script& worker_script,
      std::optional<v8_inspector::V8StackTraceId> stack_id) override;

  // Implements scheduler::WorkerScheduler::Delegate.
  void UpdateBackForwardCacheDisablingFeatures(
      BlockingDetails details) override;
  // Implements BackForwardCacheLoaderHelperImpl::Delegate.
  void EvictFromBackForwardCache(
      mojom::blink::RendererEvictionReason reason,
      std::unique_ptr<SourceLocation> source_location) override;
  void DidBufferLoadWhileInBackForwardCache(bool update_process_wide_count,
                                            size_t num_bytes) override;

  // Called by the bindings (dedicated_worker_global_scope.idl).
  const String name() const;
  void postMessage(ScriptState*,
                   const ScriptValue& message,
                   HeapVector<ScriptObject> transfer,
                   ExceptionState&);
  void postMessage(ScriptState*,
                   const ScriptValue& message,
                   const PostMessageOptions*,
                   ExceptionState&);
  DEFINE_ATTRIBUTE_EVENT_LISTENER(message, kMessage)
  DEFINE_ATTRIBUTE_EVENT_LISTENER(messageerror, kMessageerror)

  // Called by the Oilpan.
  void Trace(Visitor*) const override;

  // Returns the token that uniquely identifies this worker.
  const DedicatedWorkerToken& GetDedicatedWorkerToken() const { return token_; }
  WorkerToken GetWorkerToken() const final { return token_; }
  bool CrossOriginIsolatedCapability() const final {
    return cross_origin_isolated_capability_;
  }
  bool IsIsolatedContext() const final { return is_isolated_context_; }
  ExecutionContextToken GetExecutionContextToken() const final {
    return token_;
  }

  // Returns the ExecutionContextToken that uniquely identifies the parent
  // context that created this dedicated worker.
  std::optional<ExecutionContextToken> GetParentExecutionContextToken()
      const final {
    return parent_token_;
  }

 private:
  struct ParsedCreationParams {
    std::unique_ptr<GlobalScopeCreationParams> creation_params;
    ExecutionContextToken parent_context_token;
    net::StorageAccessApiStatus parent_storage_access_api_status;
  };

  static ParsedCreationParams ParseCreationParams(
      std::unique_ptr<GlobalScopeCreationParams> creation_params);

  // The public constructor extracts the |parent_context_token| from
  // |creation_params| and redirects here, otherwise the token is lost when we
  // move the |creation_params| to WorkerGlobalScope, and other worker types
  // don't care about that particular parameter. The helper function is required
  // because there's no guarantee about the order of evaluation of arguments.
  DedicatedWorkerGlobalScope(
      ParsedCreationParams parsed_creation_params,
      DedicatedWorkerThread* thread,
      base::TimeTicks time_origin,
      std::unique_ptr<Vector<mojom::blink::OriginTrialFeature>>
          inherited_trial_features,
      const BeginFrameProviderParams& begin_frame_provider_params,
      bool parent_cross_origin_isolated_capability,
      bool is_isolated_context,
      mojo::PendingRemote<mojom::blink::DedicatedWorkerHost>
          dedicated_worker_host,
      mojo::PendingRemote<mojom::blink::BackForwardCacheControllerHost>
          back_forward_cache_controller_host,
      base::TimeTicks dedicated_worker_start_time);

  void DidReceiveResponseForClassicScript(
      WorkerClassicScriptLoader* classic_script_loader);
  void DidFetchClassicScript(WorkerClassicScriptLoader* classic_script_loader,
                             const v8_inspector::V8StackTraceId& stack_id);

  DedicatedWorkerObjectProxy& WorkerObjectProxy() const;

  // A unique ID for this context.
  const DedicatedWorkerToken token_;
  // The ID of the parent context that owns this worker.
  const ExecutionContextToken parent_token_;
  bool cross_origin_isolated_capability_;
  bool is_isolated_context_;
  Member<WorkerAnimationFrameProvider> animation_frame_provider_;

  HeapMojoRemote<mojom::blink::DedicatedWorkerHost> dedicated_worker_host_{
      this};
  HeapMojoRemote<mojom::blink::BackForwardCacheControllerHost>
      back_forward_cache_controller_host_{this};

  // The total bytes buffered by all network requests in this worker while
  // frozen due to back-forward cache. This number gets reset when the worker
  // gets out of the back-forward cache.
  size_t total_bytes_buffered_while_in_back_forward_cache_ = 0;

  // The worker's Storage Access API status (inherited from the parent
  // ExecutionContext).
  net::StorageAccessApiStatus storage_access_api_status_;

  // The timestamp taken when FetchAndRunClassicScript() is called.
  base::TimeTicks fetch_classic_script_start_time_;

  // The timestamp taken when DedicatedWorker::Start() was called.
  base::TimeTicks dedicated_worker_start_time_;
};

template <>
struct DowncastTraits<DedicatedWorkerGlobalScope> {
  static bool AllowFrom(const ExecutionContext& context) {
    return context.IsDedicatedWorkerGlobalScope();
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_WORKERS_DEDICATED_WORKER_GLOBAL_SCOPE_H_
