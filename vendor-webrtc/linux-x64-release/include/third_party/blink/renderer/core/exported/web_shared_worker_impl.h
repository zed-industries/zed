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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_EXPORTED_WEB_SHARED_WORKER_IMPL_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_EXPORTED_WEB_SHARED_WORKER_IMPL_H_

#include <memory>

#include "base/memory/scoped_refptr.h"
#include "base/memory/weak_ptr.h"
#include "base/task/single_thread_task_runner.h"
#include "mojo/public/cpp/bindings/remote.h"
#include "services/metrics/public/cpp/ukm_source_id.h"
#include "services/network/public/mojom/content_security_policy.mojom-blink-forward.h"
#include "services/network/public/mojom/fetch_api.mojom-shared.h"
#include "third_party/blink/public/mojom/browser_interface_broker.mojom-blink.h"
#include "third_party/blink/public/mojom/frame/reporting_observer.mojom-blink-forward.h"
#include "third_party/blink/public/mojom/script/script_type.mojom-blink.h"
#include "third_party/blink/public/mojom/user_agent/user_agent_metadata.mojom-blink.h"
#include "third_party/blink/public/mojom/worker/shared_worker_host.mojom-blink.h"
#include "third_party/blink/public/mojom/worker/worker_content_settings_proxy.mojom-blink.h"
#include "third_party/blink/public/platform/web_fetch_client_settings_object.h"
#include "third_party/blink/public/web/web_shared_worker.h"
#include "third_party/blink/public/web/web_shared_worker_client.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/workers/shared_worker_reporting_proxy.h"
#include "third_party/blink/renderer/core/workers/worker_clients.h"
#include "third_party/blink/renderer/core/workers/worker_thread.h"

namespace blink {

class MessagePortChannel;
class SharedWorkerThread;
class WebSharedWorkerClient;
class WebString;
class WebURL;
struct WorkerMainScriptLoadParameters;

// This class is used by the worker process code to talk to the SharedWorker
// implementation. This is basically accessed on the main thread, but some
// methods must be called from a worker thread. Such methods are suffixed with
// *OnWorkerThread or have header comments.
//
// Owned by WebSharedWorkerClient. Destroyed in TerminateWorkerThread() or
// DidTerminateWorkerThread() via
// WebSharedWorkerClient::WorkerContextDestroyed().
class CORE_EXPORT WebSharedWorkerImpl final : public WebSharedWorker {
 public:
  ~WebSharedWorkerImpl() override;

  // WebSharedWorker methods:
  void Connect(int connection_request_id, MessagePortDescriptor port) override;
  void TerminateWorkerContext() override;

  // Callback methods for SharedWorkerReportingProxy.
  void CountFeature(WebFeature);
  void DidFailToFetchClassicScript();
  void DidFailToFetchModuleScript();
  void DidEvaluateTopLevelScript(bool success);
  void DidCloseWorkerGlobalScope();
  // This synchronously destroys |this|.
  void DidTerminateWorkerThread();

 private:
  friend class WebSharedWorker;

  WebSharedWorkerImpl(
      const blink::SharedWorkerToken& token,
      CrossVariantMojoRemote<mojom::SharedWorkerHostInterfaceBase> host,
      WebSharedWorkerClient*);

  void StartWorkerContext(
      const WebURL&,
      mojom::blink::ScriptType,
      network::mojom::CredentialsMode,
      const WebString& name,
      WebSecurityOrigin constructor_origin,
      WebSecurityOrigin origin_from_browser,
      bool is_constructor_secure_context,
      const WebString& user_agent,
      const blink::UserAgentMetadata& ua_metadata,
      const std::vector<WebContentSecurityPolicy>& content_security_policies,
      const WebFetchClientSettingsObject& outside_fetch_client_settings_object,
      const base::UnguessableToken& devtools_worker_token,
      CrossVariantMojoRemote<
          mojom::blink::WorkerContentSettingsProxyInterfaceBase>
          ontent_settings,
      CrossVariantMojoRemote<mojom::blink::BrowserInterfaceBrokerInterfaceBase>
          browser_interface_broker,
      bool pause_worker_context_on_start,
      std::unique_ptr<WorkerMainScriptLoadParameters>
          worker_main_script_load_params,
      std::unique_ptr<blink::WebPolicyContainer> policy_container,
      scoped_refptr<WebWorkerFetchContext> web_worker_fetch_context,
      ukm::SourceId ukm_source_id,
      bool require_cross_site_request_for_cookies,
      CrossVariantMojoReceiver<mojom::blink::ReportingObserverInterfaceBase>
          coep_reporting_observer,
      CrossVariantMojoReceiver<mojom::blink::ReportingObserverInterfaceBase>
          dip_reporting_observer);

  void DispatchPendingConnections();
  void ConnectToChannel(int connection_request_id,
                        blink::MessagePortChannel channel);
  void ConnectTaskOnWorkerThread(MessagePortChannel);

  SharedWorkerThread* GetWorkerThread() { return worker_thread_.get(); }

  // Shuts down the worker thread. This may synchronously destroy |this|.
  void TerminateWorkerThread();

  const Persistent<SharedWorkerReportingProxy> reporting_proxy_;
  const std::unique_ptr<SharedWorkerThread> worker_thread_;

  mojo::Remote<mojom::blink::SharedWorkerHost> host_;

  // |client_| owns |this|.
  WebSharedWorkerClient* client_;

  using PendingChannel =
      std::pair<int /* connection_request_id */, blink::MessagePortChannel>;
  Vector<PendingChannel> pending_channels_;

  scoped_refptr<base::SingleThreadTaskRunner> task_runner_for_connect_event_;

  bool running_ = false;
  bool asked_to_terminate_ = false;

  base::WeakPtrFactory<WebSharedWorkerImpl> weak_ptr_factory_{this};
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_EXPORTED_WEB_SHARED_WORKER_IMPL_H_
