// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_SHARED_STORAGE_SHARED_STORAGE_WORKLET_MESSAGING_PROXY_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_SHARED_STORAGE_SHARED_STORAGE_WORKLET_MESSAGING_PROXY_H_

#include <memory>

#include "third_party/blink/public/mojom/shared_storage/shared_storage_worklet_service.mojom-blink-forward.h"
#include "third_party/blink/public/mojom/worker/worklet_global_scope_creation_params.mojom-blink-forward.h"
#include "third_party/blink/public/platform/browser_interface_broker_proxy.h"
#include "third_party/blink/renderer/core/workers/threaded_worklet_messaging_proxy.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/heap/cross_thread_handle.h"

namespace blink {

class WorkerThread;

// Acts as a proxy for the shared storage worklet global scope. Upon creation on
// the main thread, this will post a task to the worklet thread to initialize
// the `SharedStorageWorkletGlobalScope`. When the service is disconnected on
// the worklet thread (triggering
// `OnSharedStorageWorkletServiceDisconnectedOnWorkletThread`), a task will be
// posted to the main thread, and the main thread will in turn invoke
// `ThreadedMessagingProxyBase::ParentObjectDestroyed` to terminate the worklet
// thread gracefully. Eventually, this proxy will be destroyed on the main
// thread.
class MODULES_EXPORT SharedStorageWorkletMessagingProxy final
    : public ThreadedWorkletMessagingProxy {
 public:
  SharedStorageWorkletMessagingProxy(
      scoped_refptr<base::SingleThreadTaskRunner> main_thread_runner,
      mojo::PendingReceiver<mojom::blink::SharedStorageWorkletService> receiver,
      mojom::blink::WorkletGlobalScopeCreationParamsPtr
          global_scope_creation_params,
      base::OnceClosure worklet_terminated_callback);

  void WorkerThreadTerminated() override;

  void Trace(Visitor*) const override;

 private:
  friend class SharedStorageWorkletTest;

  static void InitializeSharedStorageWorkletServiceOnWorkletThread(
      scoped_refptr<base::SingleThreadTaskRunner> main_thread_runner,
      CrossThreadHandle<SharedStorageWorkletMessagingProxy>,
      WorkerThread* worker_thread,
      mojo::PendingReceiver<mojom::blink::SharedStorageWorkletService>
          receiver);

  static void OnSharedStorageWorkletServiceDisconnectedOnWorkletThread(
      scoped_refptr<base::SingleThreadTaskRunner> main_thread_runner,
      CrossThreadHandle<SharedStorageWorkletMessagingProxy>);

  std::unique_ptr<WorkerThread> CreateWorkerThread() override;

  base::OnceClosure worklet_terminated_callback_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_SHARED_STORAGE_SHARED_STORAGE_WORKLET_MESSAGING_PROXY_H_
