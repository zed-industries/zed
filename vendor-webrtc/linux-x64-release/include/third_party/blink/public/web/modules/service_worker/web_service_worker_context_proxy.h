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

#ifndef THIRD_PARTY_BLINK_PUBLIC_WEB_MODULES_SERVICE_WORKER_WEB_SERVICE_WORKER_CONTEXT_PROXY_H_
#define THIRD_PARTY_BLINK_PUBLIC_WEB_MODULES_SERVICE_WORKER_WEB_SERVICE_WORKER_CONTEXT_PROXY_H_

#include "base/time/time.h"
#include "mojo/public/cpp/system/data_pipe.h"
#include "third_party/blink/public/mojom/service_worker/controller_service_worker.mojom-shared.h"
#include "third_party/blink/public/mojom/service_worker/service_worker.mojom-shared.h"
#include "third_party/blink/public/mojom/service_worker/service_worker_fetch_handler_type.mojom-shared.h"
#include "third_party/blink/public/platform/cross_variant_mojo_util.h"
#include "third_party/blink/public/platform/web_string.h"

#include <memory>

namespace blink {

class AssociatedInterfaceRegistry;
struct WebServiceWorkerError;
class WebURLResponse;

// A proxy interface to talk to the worker's GlobalScope implementation.
// All methods of this class must be called on the worker thread.
class WebServiceWorkerContextProxy {
 public:
  virtual ~WebServiceWorkerContextProxy() = default;

  virtual void BindServiceWorker(
      CrossVariantMojoReceiver<mojom::ServiceWorkerInterfaceBase> receiver) = 0;
  virtual void BindControllerServiceWorker(
      CrossVariantMojoReceiver<mojom::ControllerServiceWorkerInterfaceBase>
          receiver) = 0;

  virtual void OnNavigationPreloadResponse(
      int fetch_event_id,
      std::unique_ptr<WebURLResponse>,
      mojo::ScopedDataPipeConsumerHandle) = 0;
  virtual void OnNavigationPreloadError(
      int fetch_event_id,
      std::unique_ptr<WebServiceWorkerError>) = 0;
  virtual void OnNavigationPreloadComplete(int fetch_event_id,
                                           base::TimeTicks completion_time,
                                           int64_t encoded_data_length,
                                           int64_t encoded_body_length,
                                           int64_t decoded_body_length) = 0;

  virtual bool IsWindowInteractionAllowed() = 0;
  virtual void PauseEvaluation() = 0;
  virtual void ResumeEvaluation() = 0;
  virtual mojom::ServiceWorkerFetchHandlerType FetchHandlerType() = 0;
  virtual bool HasHidEventHandlers() = 0;
  virtual bool HasUsbEventHandlers() = 0;

  // Passes an associated endpoint handle to the remote end to be bound to a
  // Channel-associated interface named |name|.
  virtual void GetRemoteAssociatedInterface(
      const WebString& name,
      mojo::ScopedInterfaceEndpointHandle handle) = 0;

  template <typename Interface>
  void GetRemoteAssociatedInterface(
      mojo::PendingAssociatedReceiver<Interface> receiver) {
    GetRemoteAssociatedInterface(WebString::FromASCII(Interface::Name_),
                                 receiver.PassHandle());
  }

  virtual blink::AssociatedInterfaceRegistry&
  GetAssociatedInterfaceRegistry() = 0;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_WEB_MODULES_SERVICE_WORKER_WEB_SERVICE_WORKER_CONTEXT_PROXY_H_
