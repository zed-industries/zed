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

#ifndef THIRD_PARTY_BLINK_PUBLIC_WEB_WEB_SHARED_WORKER_H_
#define THIRD_PARTY_BLINK_PUBLIC_WEB_WEB_SHARED_WORKER_H_

#include <memory>
#include <vector>

#include "base/unguessable_token.h"
#include "services/metrics/public/cpp/ukm_source_id.h"
#include "services/network/public/mojom/content_security_policy.mojom-shared.h"
#include "services/network/public/mojom/fetch_api.mojom-shared.h"
#include "third_party/blink/public/common/tokens/tokens.h"
#include "third_party/blink/public/common/user_agent/user_agent_metadata.h"
#include "third_party/blink/public/mojom/browser_interface_broker.mojom-shared.h"
#include "third_party/blink/public/mojom/frame/policy_container.mojom-forward.h"
#include "third_party/blink/public/mojom/frame/reporting_observer.mojom-shared.h"
#include "third_party/blink/public/mojom/script/script_type.mojom-shared.h"
#include "third_party/blink/public/mojom/worker/shared_worker_host.mojom-shared.h"
#include "third_party/blink/public/mojom/worker/worker_content_settings_proxy.mojom-shared.h"
#include "third_party/blink/public/platform/cross_variant_mojo_util.h"
#include "third_party/blink/public/platform/task_type.h"
#include "third_party/blink/public/platform/web_common.h"
#include "third_party/blink/public/platform/web_content_security_policy_struct.h"
#include "third_party/blink/public/platform/web_policy_container.h"
#include "third_party/blink/public/platform/web_security_origin.h"

namespace blink {

class MessagePortDescriptor;
class WebString;
class WebSharedWorkerClient;
class WebURL;
class WebWorkerFetchContext;
struct WebFetchClientSettingsObject;
struct WorkerMainScriptLoadParameters;

// This is the interface to a SharedWorker thread.
class BLINK_EXPORT WebSharedWorker {
 public:
  virtual ~WebSharedWorker() {}

  // Instantiates a WebSharedWorker that interacts with the shared worker and
  // starts a worker context.
  // WebSharedWorkerClient given here should own this instance.
  static std::unique_ptr<WebSharedWorker> CreateAndStart(
      const blink::SharedWorkerToken& token,
      const WebURL& script_url,
      mojom::ScriptType script_type,
      network::mojom::CredentialsMode,
      const WebString& name,
      WebSecurityOrigin constructor_origin,
      WebSecurityOrigin origin_from_browser,
      bool is_constructor_secure_context,
      const WebString& user_agent,
      const UserAgentMetadata& ua_metadata,
      const std::vector<WebContentSecurityPolicy>& content_security_policies,
      const WebFetchClientSettingsObject& outside_fetch_client_settings_object,
      const base::UnguessableToken& devtools_worker_token,
      CrossVariantMojoRemote<mojom::WorkerContentSettingsProxyInterfaceBase>
          content_settings,
      CrossVariantMojoRemote<mojom::BrowserInterfaceBrokerInterfaceBase>
          browser_interface_broker,
      bool pause_worker_context_on_start,
      std::unique_ptr<blink::WorkerMainScriptLoadParameters>
          worker_main_script_load_params,
      std::unique_ptr<blink::WebPolicyContainer> policy_container,
      scoped_refptr<WebWorkerFetchContext> web_worker_fetch_context,
      CrossVariantMojoRemote<mojom::SharedWorkerHostInterfaceBase>,
      WebSharedWorkerClient*,
      ukm::SourceId ukm_source_id,
      bool require_cross_site_request_for_cookies,
      CrossVariantMojoReceiver<mojom::ReportingObserverInterfaceBase>
          coep_reporting_observer,
      CrossVariantMojoReceiver<mojom::ReportingObserverInterfaceBase>
          dip_reporting_observer);

  // Sends a connect event to the SharedWorker context.
  virtual void Connect(int connection_request_id,
                       MessagePortDescriptor port) = 0;

  // Invoked to shutdown the worker when there are no more associated documents.
  // This eventually deletes this instance.
  virtual void TerminateWorkerContext() = 0;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_WEB_WEB_SHARED_WORKER_H_
