/*
 * Copyright (C) 2013 Google Inc. All rights reserved.
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

#ifndef THIRD_PARTY_BLINK_PUBLIC_WEB_WEB_EMBEDDED_WORKER_START_DATA_H_
#define THIRD_PARTY_BLINK_PUBLIC_WEB_WEB_EMBEDDED_WORKER_START_DATA_H_

#include "base/unguessable_token.h"
#include "services/metrics/public/cpp/ukm_source_id.h"
#include "third_party/blink/public/common/loader/worker_main_script_load_parameters.h"
#include "third_party/blink/public/common/tokens/tokens.h"
#include "third_party/blink/public/common/user_agent/user_agent_metadata.h"
#include "third_party/blink/public/mojom/service_worker/service_worker_registration.mojom-shared.h"
#include "third_party/blink/public/platform/web_fetch_client_settings_object.h"
#include "third_party/blink/public/platform/web_policy_container.h"
#include "third_party/blink/public/platform/web_string.h"
#include "third_party/blink/public/platform/web_url.h"

namespace blink {

struct WebEmbeddedWorkerStartData {
  enum WaitForDebuggerMode { kDontWaitForDebugger, kWaitForDebugger };

  WebURL script_url;
  WebString user_agent;
  UserAgentMetadata ua_metadata;
  mojom::ScriptType script_type;
  // Whether to pause the initialization and wait for debugger to attach
  // before proceeding. This technique allows debugging worker startup.
  WaitForDebuggerMode wait_for_debugger_mode;
  // Unique worker token used by DevTools to attribute different instrumentation
  // to the same worker.
  base::UnguessableToken devtools_worker_token;
  ukm::SourceId ukm_source_id = ukm::kInvalidSourceId;

  WebFetchClientSettingsObject outside_fetch_client_settings_object;

  // Unique token that identifies this worker across the browser and renderer
  // processes. This is not persistent across worker restarts.
  blink::ServiceWorkerToken service_worker_token;

  // Non-null only when the service worker is new and the script needs to be
  // loaded from the network.
  std::unique_ptr<WorkerMainScriptLoadParameters> main_script_load_params;

  std::unique_ptr<WebPolicyContainer> policy_container;

  explicit WebEmbeddedWorkerStartData(
      WebFetchClientSettingsObject outside_fetch_client_settings_object)
      : wait_for_debugger_mode(kDontWaitForDebugger),
        outside_fetch_client_settings_object(
            std::move(outside_fetch_client_settings_object)) {}
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_WEB_WEB_EMBEDDED_WORKER_START_DATA_H_
