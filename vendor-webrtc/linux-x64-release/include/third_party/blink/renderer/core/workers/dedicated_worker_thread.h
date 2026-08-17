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
#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_WORKERS_DEDICATED_WORKER_THREAD_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_WORKERS_DEDICATED_WORKER_THREAD_H_

#include <memory>

#include "services/metrics/public/cpp/ukm_source_id.h"
#include "third_party/blink/public/mojom/frame/back_forward_cache_controller.mojom-blink-forward.h"
#include "third_party/blink/public/mojom/worker/dedicated_worker_host.mojom-blink-forward.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/workers/worker_thread.h"

namespace blink {

class DedicatedWorkerObjectProxy;
struct GlobalScopeCreationParams;

class CORE_EXPORT DedicatedWorkerThread : public WorkerThread {
 public:
  DedicatedWorkerThread(
      ExecutionContext* parent_execution_context,
      DedicatedWorkerObjectProxy&,
      mojo::PendingRemote<mojom::blink::DedicatedWorkerHost>
          dedicated_worker_host,
      mojo::PendingRemote<mojom::blink::BackForwardCacheControllerHost>
          back_forward_cache_controller_host);
  ~DedicatedWorkerThread() override;

  WorkerBackingThread& GetWorkerBackingThread() override {
    return *worker_backing_thread_;
  }
  DedicatedWorkerObjectProxy& WorkerObjectProxy() const {
    return worker_object_proxy_;
  }

 private:
  friend class DedicatedWorkerThreadForTest;

  WorkerOrWorkletGlobalScope* CreateWorkerGlobalScope(
      std::unique_ptr<GlobalScopeCreationParams>) override;

  ThreadType GetThreadType() const override {
    return ThreadType::kDedicatedWorkerThread;
  }

  std::unique_ptr<WorkerBackingThread> worker_backing_thread_;
  DedicatedWorkerObjectProxy& worker_object_proxy_;
  ukm::SourceId ukm_source_id_;

  // Passed to DedicatedWorkerGlobalScope on global scope creation.
  mojo::PendingRemote<mojom::blink::DedicatedWorkerHost>
      pending_dedicated_worker_host_;
  mojo::PendingRemote<mojom::blink::BackForwardCacheControllerHost>
      pending_back_forward_cache_controller_host_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_WORKERS_DEDICATED_WORKER_THREAD_H_
