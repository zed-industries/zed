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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_WORKERS_SHARED_WORKER_GLOBAL_SCOPE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_WORKERS_SHARED_WORKER_GLOBAL_SCOPE_H_

#include <memory>
#include "third_party/blink/public/common/messaging/message_port_channel.h"
#include "third_party/blink/public/common/tokens/tokens.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/workers/worker_global_scope.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"

namespace blink {

class SharedWorkerThread;
class WorkerClassicScriptLoader;

class CORE_EXPORT SharedWorkerGlobalScope final : public WorkerGlobalScope {
  DEFINE_WRAPPERTYPEINFO();

 public:
  SharedWorkerGlobalScope(
      std::unique_ptr<GlobalScopeCreationParams> creation_params,
      SharedWorkerThread* thread,
      base::TimeTicks time_origin,
      const SharedWorkerToken& token,
      bool require_cross_site_request_for_cookies);

  ~SharedWorkerGlobalScope() override;

  bool IsSharedWorkerGlobalScope() const override { return true; }

  // EventTarget
  const AtomicString& InterfaceName() const override;

  // WorkerGlobalScope
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

  // shared_worker_global_scope.idl
  const String name() const;
  DEFINE_ATTRIBUTE_EVENT_LISTENER(connect, kConnect)

  void Connect(MessagePortChannel channel);

  void Trace(Visitor*) const override;

  // Returns the token that uniquely identifies this worker.
  const SharedWorkerToken& GetSharedWorkerToken() const { return token_; }
  WorkerToken GetWorkerToken() const final { return token_; }
  bool CrossOriginIsolatedCapability() const final;
  bool IsIsolatedContext() const final;
  ExecutionContextToken GetExecutionContextToken() const final {
    return token_;
  }

  // If true, then all requests made must have an empty site_for_cookies to
  // ensure only SameSite=None cookies can be attached to the request.
  // For context on usage see:
  // https://privacycg.github.io/saa-non-cookie-storage/shared-workers.html
  bool DoesRequireCrossSiteRequestForCookies() const {
    return require_cross_site_request_for_cookies_;
  }

 private:
  void DidReceiveResponseForClassicScript(
      WorkerClassicScriptLoader* classic_script_loader);
  void DidFetchClassicScript(WorkerClassicScriptLoader* classic_script_loader,
                             const v8_inspector::V8StackTraceId& stack_id);

  void ExceptionThrown(ErrorEvent*) override;

  const SharedWorkerToken token_;

  const bool require_cross_site_request_for_cookies_;
};

template <>
struct DowncastTraits<SharedWorkerGlobalScope> {
  static bool AllowFrom(const ExecutionContext& context) {
    return context.IsSharedWorkerGlobalScope();
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_WORKERS_SHARED_WORKER_GLOBAL_SCOPE_H_
