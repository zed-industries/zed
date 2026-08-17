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

#ifndef THIRD_PARTY_BLINK_PUBLIC_PLATFORM_MODULES_SERVICE_WORKER_WEB_SERVICE_WORKER_PROVIDER_H_
#define THIRD_PARTY_BLINK_PUBLIC_PLATFORM_MODULES_SERVICE_WORKER_WEB_SERVICE_WORKER_PROVIDER_H_

#include <memory>
#include <vector>

#include "third_party/blink/public/mojom/service_worker/service_worker_registration.mojom-shared.h"
#include "third_party/blink/public/platform/modules/service_worker/web_service_worker_registration_object_info.h"
#include "third_party/blink/public/platform/web_callbacks.h"

namespace blink {

class WebURL;
class WebServiceWorkerProviderClient;
struct WebFetchClientSettingsObject;
struct WebServiceWorkerError;

// WebServiceWorkerProvider attaches to a Document
// and "provides" it with functionality needed to implement
// ServiceWorkerContainer.idl functions.
//
// It is implemented by content::WebServiceWorkerProviderImpl.
//
// WebServiceWorkerProvider is created in ServiceWorkerContainer::From(),
// which is used to instantiate navigator.serviceWorker. It is owned by
// ServiceWorkerContainer, which is a garbage collected Supplement for Document.
//
// Each ServiceWorkerContainer instance has a WebServiceWorkerProvider.
// ServiceWorkerContainer is called the "client" of the
// WebServiceWorkerProvider, and it implements WebServiceWorkerProviderClient.
class WebServiceWorkerProvider {
 public:
  // Sets the "client" for this provider. The client will be notified of
  // controller changes, message events, and feature usages apropos of the
  // document this WebServiceWorkerProvider is for.
  virtual void SetClient(WebServiceWorkerProviderClient*) {}

  using WebServiceWorkerRegistrationCallbacks =
      WebCallbacks<WebServiceWorkerRegistrationObjectInfo,
                   const WebServiceWorkerError&>;
  using WebServiceWorkerGetRegistrationCallbacks =
      WebCallbacks<WebServiceWorkerRegistrationObjectInfo,
                   const WebServiceWorkerError&>;

  using WebServiceWorkerGetRegistrationsCallbacks =
      WebCallbacks<std::vector<WebServiceWorkerRegistrationObjectInfo>,
                   const WebServiceWorkerError&>;
  using GetRegistrationForReadyCallback =
      base::OnceCallback<void(WebServiceWorkerRegistrationObjectInfo)>;

  // For ServiceWorkerContainer#register(). Requests the embedder to register a
  // service worker.
  // TODO(yuryu): Use the blink::mojom::RegistrationOptions type after Onion
  // Soup.
  virtual void RegisterServiceWorker(
      const WebURL& pattern,
      const WebURL& script_url,
      blink::mojom::ScriptType script_type,
      blink::mojom::ServiceWorkerUpdateViaCache update_via_cache,
      const WebFetchClientSettingsObject& fetch_client_settings_object,
      std::unique_ptr<WebServiceWorkerRegistrationCallbacks>) {}
  // For ServiceWorkerContainer#getRegistration(). Requests the embedder to
  // return a registration.
  virtual void GetRegistration(
      const WebURL& document_url,
      std::unique_ptr<WebServiceWorkerGetRegistrationCallbacks>) {}
  // For ServiceWorkerContainer#getRegistrations(). Requests the embedder to
  // return matching registrations.
  virtual void GetRegistrations(
      std::unique_ptr<WebServiceWorkerGetRegistrationsCallbacks>) {}
  // For ServiceWorkerContainer#ready. Requests the embedder to return the
  // ready registration.
  virtual void GetRegistrationForReady(GetRegistrationForReadyCallback) {}
  // Helper function for checking URLs. The |scope| and |script_url| cannot
  // include escape sequences for "/" or "\" as per spec, as they would break
  // would the path restriction.
  virtual bool ValidateScopeAndScriptURL(const WebURL& scope,
                                         const WebURL& script_url,
                                         WebString* error_message) {
    return false;
  }

  virtual ~WebServiceWorkerProvider() = default;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_PLATFORM_MODULES_SERVICE_WORKER_WEB_SERVICE_WORKER_PROVIDER_H_
