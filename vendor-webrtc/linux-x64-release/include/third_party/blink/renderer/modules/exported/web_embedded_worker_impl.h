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

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_EXPORTED_WEB_EMBEDDED_WORKER_IMPL_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_EXPORTED_WEB_EMBEDDED_WORKER_IMPL_H_

#include <memory>

#include "base/memory/raw_ptr.h"
#include "base/task/single_thread_task_runner.h"
#include "mojo/public/cpp/bindings/pending_remote.h"
#include "third_party/blink/public/mojom/cache_storage/cache_storage.mojom-blink-forward.h"
#include "third_party/blink/public/mojom/frame/reporting_observer.mojom-blink-forward.h"
#include "third_party/blink/public/web/web_embedded_worker.h"
#include "third_party/blink/public/web/web_embedded_worker_start_data.h"
#include "third_party/blink/renderer/core/workers/global_scope_creation_params.h"
#include "third_party/blink/renderer/core/workers/worker_clients.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/modules/service_worker/service_worker_content_settings_proxy.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"

namespace blink {

class ServiceWorkerInstalledScriptsManager;
class ServiceWorkerThread;
struct CrossThreadFetchClientSettingsObjectData;

// The implementation of WebEmbeddedWorker. This is responsible for starting
// and terminating a service worker thread. Lives on a ThreadPool background
// thread.
//
// Because it lives on a ThreadPool thread, this class does not make
// GarbageCollected objects (https://crbug.com/988335).
class MODULES_EXPORT WebEmbeddedWorkerImpl final : public WebEmbeddedWorker {
 public:
  explicit WebEmbeddedWorkerImpl(WebServiceWorkerContextClient*);

  WebEmbeddedWorkerImpl(const WebEmbeddedWorkerImpl&) = delete;
  WebEmbeddedWorkerImpl& operator=(const WebEmbeddedWorkerImpl&) = delete;

  ~WebEmbeddedWorkerImpl() override;

  // WebEmbeddedWorker overrides.
  void StartWorkerContext(
      std::unique_ptr<WebEmbeddedWorkerStartData>,
      std::unique_ptr<WebServiceWorkerInstalledScriptsManagerParams>,
      CrossVariantMojoRemote<
          mojom::blink::WorkerContentSettingsProxyInterfaceBase>
          content_settings,
      CrossVariantMojoRemote<mojom::blink::CacheStorageInterfaceBase>
          cache_storage,
      CrossVariantMojoRemote<mojom::blink::BrowserInterfaceBrokerInterfaceBase>
          browser_interface_broker,
      InterfaceRegistry* interface_registry,
      scoped_refptr<base::SingleThreadTaskRunner> initiator_thread_task_runner,
      CrossVariantMojoReceiver<mojom::blink::ReportingObserverInterfaceBase>
          coep_reporting_observer,
      CrossVariantMojoReceiver<mojom::blink::ReportingObserverInterfaceBase>
          dip_reporting_observer) override;
  void TerminateWorkerContext() override;

  void WaitForShutdownForTesting();

 private:
  void StartWorkerThread(
      std::unique_ptr<WebEmbeddedWorkerStartData> worker_start_data,
      std::unique_ptr<ServiceWorkerInstalledScriptsManager>,
      std::unique_ptr<ServiceWorkerContentSettingsProxy>,
      mojo::PendingRemote<mojom::blink::CacheStorage>,
      mojo::PendingRemote<mojom::blink::BrowserInterfaceBroker>,
      InterfaceRegistry* interface_registry,
      scoped_refptr<base::SingleThreadTaskRunner> initiator_thread_task_runner,
      mojo::PendingReceiver<mojom::blink::ReportingObserver>
          coep_reporting_observer,
      mojo::PendingReceiver<mojom::blink::ReportingObserver>
          dip_reporting_observer);

  // Creates a cross-thread copyable outside settings object for top-level
  // worker script fetch.
  std::unique_ptr<CrossThreadFetchClientSettingsObjectData>
  CreateFetchClientSettingsObjectData(
      const KURL& script_url,
      const SecurityOrigin*,
      const HttpsState&,
      const WebFetchClientSettingsObject& passed_settings_object);

  // Client must remain valid through the entire life time of the worker.
  const raw_ptr<WebServiceWorkerContextClient> worker_context_client_;

  std::unique_ptr<ServiceWorkerThread> worker_thread_;

  bool asked_to_terminate_ = false;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_EXPORTED_WEB_EMBEDDED_WORKER_IMPL_H_
