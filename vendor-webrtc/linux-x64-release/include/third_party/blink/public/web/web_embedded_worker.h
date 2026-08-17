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

#ifndef THIRD_PARTY_BLINK_PUBLIC_WEB_WEB_EMBEDDED_WORKER_H_
#define THIRD_PARTY_BLINK_PUBLIC_WEB_WEB_EMBEDDED_WORKER_H_

#include <memory>
#include <vector>

#include "third_party/blink/public/mojom/browser_interface_broker.mojom-shared.h"
#include "third_party/blink/public/mojom/cache_storage/cache_storage.mojom-shared.h"
#include "third_party/blink/public/mojom/frame/reporting_observer.mojom-shared.h"
#include "third_party/blink/public/mojom/service_worker/service_worker_installed_scripts_manager.mojom-shared.h"
#include "third_party/blink/public/mojom/worker/worker_content_settings_proxy.mojom-shared.h"
#include "third_party/blink/public/platform/cross_variant_mojo_util.h"
#include "third_party/blink/public/platform/web_common.h"

namespace blink {

class InterfaceRegistry;
class WebServiceWorkerContextClient;
class WebURL;
struct WebEmbeddedWorkerStartData;

struct BLINK_EXPORT WebServiceWorkerInstalledScriptsManagerParams {
  WebServiceWorkerInstalledScriptsManagerParams() = delete;
  WebServiceWorkerInstalledScriptsManagerParams(
      std::vector<WebURL> installed_scripts_urls,
      CrossVariantMojoReceiver<
          mojom::ServiceWorkerInstalledScriptsManagerInterfaceBase>
          manager_receiver,
      CrossVariantMojoRemote<
          mojom::ServiceWorkerInstalledScriptsManagerHostInterfaceBase>
          manager_host_remote);
  ~WebServiceWorkerInstalledScriptsManagerParams() = default;

  std::vector<WebURL> installed_scripts_urls;
  CrossVariantMojoReceiver<
      mojom::ServiceWorkerInstalledScriptsManagerInterfaceBase>
      manager_receiver;
  CrossVariantMojoRemote<
      mojom::ServiceWorkerInstalledScriptsManagerHostInterfaceBase>
      manager_host_remote;
};

// An interface to start and terminate an embedded worker. Lives on
// a background thread from the ThreadPool.
class BLINK_EXPORT WebEmbeddedWorker {
 public:
  // Invoked on the main thread to instantiate a WebEmbeddedWorker.
  // WebServiceWorkerContextClient is owned by caller and must survive the
  // instance of WebEmbeddedWorker.
  static std::unique_ptr<WebEmbeddedWorker> Create(
      WebServiceWorkerContextClient*);

  virtual ~WebEmbeddedWorker() = default;

  // Starts and terminates WorkerThread and WorkerGlobalScope.
  virtual void StartWorkerContext(
      std::unique_ptr<WebEmbeddedWorkerStartData>,
      std::unique_ptr<WebServiceWorkerInstalledScriptsManagerParams>,
      CrossVariantMojoRemote<mojom::WorkerContentSettingsProxyInterfaceBase>
          content_settings,
      CrossVariantMojoRemote<mojom::CacheStorageInterfaceBase> cache_storage,
      CrossVariantMojoRemote<mojom::BrowserInterfaceBrokerInterfaceBase>
          browser_interface_broker,
      InterfaceRegistry* interface_registry,
      scoped_refptr<base::SingleThreadTaskRunner> initiator_thread_task_runner,
      CrossVariantMojoReceiver<mojom::ReportingObserverInterfaceBase>
          coep_reporting_observer,
      CrossVariantMojoReceiver<mojom::ReportingObserverInterfaceBase>
          dip_reporting_observer) = 0;
  virtual void TerminateWorkerContext() = 0;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_WEB_WEB_EMBEDDED_WORKER_H_
