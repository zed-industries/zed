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

#ifndef THIRD_PARTY_BLINK_PUBLIC_WEB_MODULES_SERVICE_WORKER_WEB_SERVICE_WORKER_CONTEXT_CLIENT_H_
#define THIRD_PARTY_BLINK_PUBLIC_WEB_MODULES_SERVICE_WORKER_WEB_SERVICE_WORKER_CONTEXT_CLIENT_H_

#include "base/memory/scoped_refptr.h"
#include "services/network/public/mojom/url_loader.mojom-shared.h"
#include "third_party/blink/public/mojom/devtools/console_message.mojom-shared.h"
#include "third_party/blink/public/mojom/devtools/devtools_agent.mojom-shared.h"
#include "third_party/blink/public/mojom/service_worker/service_worker_event_status.mojom-shared.h"
#include "third_party/blink/public/mojom/use_counter/metrics/web_feature.mojom-shared.h"
#include "third_party/blink/public/platform/cross_variant_mojo_util.h"
#include "third_party/blink/public/platform/modules/service_worker/web_service_worker_fetch_context.h"
#include "third_party/blink/public/platform/web_url.h"
#include "v8/include/v8-forward.h"

namespace base {
class SequencedTaskRunner;
}

namespace blink {

class WebServiceWorkerContextProxy;
class WebString;
class WebURLResponse;
struct WebServiceWorkerError;

// WebServiceWorkerContextClient is a "client" of a service worker execution
// context. This interface is implemented by the embedder and allows the
// embedder to communicate with the service worker execution context.  It is
// created on the initiator thread (the main thread or the IO thread) and then
// passed on to the worker thread by a newly created ServiceWorkerGlobalScope.
//
// Unless otherwise noted, all methods of this class are called on the worker
// thread.
class WebServiceWorkerContextClient {
 public:
  using RequestTerminationCallback = base::OnceCallback<void(bool)>;

  virtual ~WebServiceWorkerContextClient() = default;

  // ServiceWorker has prepared everything for script loading and is now ready
  // for DevTools inspection. Called on the initiator thread.
  virtual void WorkerReadyForInspectionOnInitiatorThread(
      CrossVariantMojoRemote<mojom::DevToolsAgentInterfaceBase>
          devtools_agent_remote,
      CrossVariantMojoReceiver<mojom::DevToolsAgentHostInterfaceBase>
          devtools_agent_host_receiver) {}

  // The worker started but it could not execute because fetching the classic
  // script failed on the worker thread.
  virtual void FailedToFetchClassicScript() {}

  // The worker started but it could not execute because fetching module script
  // failed on the worker thread.
  virtual void FailedToFetchModuleScript() {}

  // The worker script was successfully loaded on the worker thread.
  // When off-the-main-thread script fetch is on, this is called for both
  // new-script and installed-script cases. If off-the-main-thread script fetch
  // is off, this is called for only the installed-script case.
  //
  // This is called after WorkerContextStarted(). Script evaluation does not
  // start until WillEvaluateScript().
  virtual void WorkerScriptLoadedOnWorkerThread() {}

  // Called when a WorkerGlobalScope was created for the worker thread. This
  // also gives a proxy to the embedder to talk to the newly created
  // WorkerGlobalScope. The proxy is owned by WorkerGlobalScope and should not
  // be destroyed by the caller. No proxy methods should be called after
  // willDestroyWorkerContext() is called.
  //
  // |worker_task_runner| is a task runner that runs tasks on the worker thread
  // and safely discards tasks when the thread stops. See
  // blink::WorkerThread::GetTaskRunner().
  //
  // For new workers (on-main-thread script fetch), this is called after
  // WorkerScriptLoadedOnWorkerThread().
  //
  // For installed workers, this is called before
  // WorkerScriptLoadedOnInitiatorThread().
  //
  // Script evaluation does not start until WillEvaluateScript().
  virtual void WorkerContextStarted(
      WebServiceWorkerContextProxy*,
      scoped_refptr<base::SequencedTaskRunner> worker_task_runner) {}

  // Called immediately before V8 script evaluation starts for the main script.
  // This means all setup is finally complete: the script has been loaded, the
  // worker thread has started, the script has been passed to the worker thread,
  // and CSP and ReferrerPolicy information has been set on the worker thread.
  //
  // |v8_context| is the V8 context of the worker and is used to support
  // service workers in Chrome extensions.
  virtual void WillEvaluateScript(v8::Local<v8::Context> v8_context) {}

  // Called when initial script evaluation finished for the main script.
  // |success| is true if the evaluation completed with no uncaught exception.
  virtual void DidEvaluateScript(bool success) {}

  // Called when the worker context is going to be initialized. This is the
  // initial method call after creating the worker scheduler.
  virtual void WillInitializeWorkerContext() {}

  // WorkerGlobalScope is about to be destroyed. The client should clear
  // the WebServiceWorkerGlobalScopeProxy when this is called.
  virtual void WillDestroyWorkerContext(v8::Local<v8::Context> context) {}

  // WorkerGlobalScope was destroyed and the worker is ready to be terminated.
  virtual void WorkerContextDestroyed() {}

  // Called when some API to be recorded in UseCounter is called on the worker
  // global scope.
  virtual void CountFeature(mojom::WebFeature feature) {}

  // Called when the WorkerGlobalScope had an error or an exception.
  virtual void ReportException(const WebString& error_message,
                               int line_number,
                               int column_number,
                               const WebString& source_url) {}

  // Called when a console message was written.
  virtual void ReportConsoleMessage(blink::mojom::ConsoleMessageSource source,
                                    blink::mojom::ConsoleMessageLevel level,
                                    const WebString& message,
                                    int line_number,
                                    const WebString& source_url) {}

  // Called when the navigation preload (FetchEvent#preloadResponse) is needed.
  virtual void SetupNavigationPreload(
      int fetch_event_id,
      const WebURL& url,
      CrossVariantMojoReceiver<network::mojom::URLLoaderClientInterfaceBase>
          preload_url_loader_client_receiver) {}

  // Called when we need to request to terminate this worker due to idle
  // timeout.
  virtual void RequestTermination(RequestTerminationCallback) {}

  virtual bool ShouldNotifyServiceWorkerOnWebSocketActivity(
      v8::Local<v8::Context> context) {
    return false;
  }

  // Off-main-thread start up:
  // Creates a WebWorkerFetchContext for subresource fetches on a service
  // worker. This is called on the initiator thread.
  virtual scoped_refptr<blink::WebServiceWorkerFetchContext>
  CreateWorkerFetchContextOnInitiatorThread() = 0;

  // Called to resolve the FetchEvent.preloadResponse promise.
  virtual void OnNavigationPreloadResponse(
      int fetch_event_id,
      std::unique_ptr<WebURLResponse> response,
      mojo::ScopedDataPipeConsumerHandle data_pipe) = 0;

  // Called when the navigation preload request completed. Either
  // OnNavigationPreloadComplete() or OnNavigationPreloadError() must be
  // called to release the preload related resources.
  virtual void OnNavigationPreloadComplete(int fetch_event_id,
                                           base::TimeTicks completion_time,
                                           int64_t encoded_data_length,
                                           int64_t encoded_body_length,
                                           int64_t decoded_body_length) = 0;

  // Called when an error occurred while receiving the response of the
  // navigation preload request.
  virtual void OnNavigationPreloadError(
      int fetch_event_id,
      std::unique_ptr<WebServiceWorkerError> error) = 0;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_WEB_MODULES_SERVICE_WORKER_WEB_SERVICE_WORKER_CONTEXT_CLIENT_H_
