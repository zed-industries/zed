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
#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_SERVICE_WORKER_SERVICE_WORKER_THREAD_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_SERVICE_WORKER_SERVICE_WORKER_THREAD_H_

#include <memory>
#include "base/task/single_thread_task_runner.h"
#include "mojo/public/cpp/bindings/pending_remote.h"
#include "third_party/blink/public/common/tokens/tokens.h"
#include "third_party/blink/public/mojom/cache_storage/cache_storage.mojom-blink-forward.h"
#include "third_party/blink/renderer/core/workers/worker_thread.h"
#include "third_party/blink/renderer/modules/modules_export.h"

namespace blink {

class ServiceWorkerGlobalScopeProxy;
class ServiceWorkerInstalledScriptsManager;
struct GlobalScopeCreationParams;

// ServiceWorkerThread is an implementation of WorkerThread for service workers.
// This provides a backing thread and an installed scripts manager.
class MODULES_EXPORT ServiceWorkerThread final : public WorkerThread {
 public:
  // ServiceWorkerThread owns a given ServiceWorkerGlobalScopeProxy via
  // Persistent.
  ServiceWorkerThread(
      std::unique_ptr<ServiceWorkerGlobalScopeProxy>,
      std::unique_ptr<ServiceWorkerInstalledScriptsManager>,
      mojo::PendingRemote<mojom::blink::CacheStorage> cache_storage_remote,
      scoped_refptr<base::SingleThreadTaskRunner>
          parent_thread_default_task_runner,
      const ServiceWorkerToken& service_worker_token);
  ~ServiceWorkerThread() override;

  WorkerBackingThread& GetWorkerBackingThread() override {
    return *worker_backing_thread_;
  }
  void TerminateForTesting() override;

 private:
  WorkerOrWorkletGlobalScope* CreateWorkerGlobalScope(
      std::unique_ptr<GlobalScopeCreationParams>) override;

  ThreadType GetThreadType() const override {
    return ThreadType::kServiceWorkerThread;
  }

  std::unique_ptr<ServiceWorkerGlobalScopeProxy> global_scope_proxy_;
  std::unique_ptr<WorkerBackingThread> worker_backing_thread_;

  // Ownership of these members is moved out in CreateWorkerGlobalScope().
  std::unique_ptr<ServiceWorkerInstalledScriptsManager>
      installed_scripts_manager_;
  mojo::PendingRemote<mojom::blink::CacheStorage> cache_storage_remote_;

  const ServiceWorkerToken service_worker_token_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_SERVICE_WORKER_SERVICE_WORKER_THREAD_H_
