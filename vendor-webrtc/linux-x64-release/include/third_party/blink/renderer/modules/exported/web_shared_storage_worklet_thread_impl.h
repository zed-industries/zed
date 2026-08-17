// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_EXPORTED_WEB_SHARED_STORAGE_WORKLET_THREAD_IMPL_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_EXPORTED_WEB_SHARED_STORAGE_WORKLET_THREAD_IMPL_H_

#include "third_party/blink/public/mojom/shared_storage/shared_storage_worklet_service.mojom-blink-forward.h"
#include "third_party/blink/public/mojom/worker/worklet_global_scope_creation_params.mojom-blink-forward.h"
#include "third_party/blink/public/web/web_shared_storage_worklet_thread.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/modules/shared_storage/shared_storage_worklet_messaging_proxy.h"
#include "third_party/blink/renderer/platform/heap/self_keep_alive.h"

namespace blink {

// A thread container for running shared storage worklet operations. This object
// lives on the main thread, and posts the task to the worklet thread to
// initialize the worklet environment belonging to one Document. The object owns
// itself, cleaning up when the worklet has shut down.
class MODULES_EXPORT WebSharedStorageWorkletThreadImpl final
    : public GarbageCollected<WebSharedStorageWorkletThreadImpl>,
      public WebSharedStorageWorkletThread {
 public:
  WebSharedStorageWorkletThreadImpl(
      scoped_refptr<base::SingleThreadTaskRunner> main_thread_runner,
      mojo::PendingReceiver<mojom::blink::SharedStorageWorkletService> receiver,
      mojom::blink::WorkletGlobalScopeCreationParamsPtr
          global_scope_creation_params);

  ~WebSharedStorageWorkletThreadImpl() override;

  void Trace(Visitor*) const;

 private:
  void DeleteSelf();

  scoped_refptr<base::SingleThreadTaskRunner> main_thread_runner_;

  Member<SharedStorageWorkletMessagingProxy> messaging_proxy_;

  SelfKeepAlive<WebSharedStorageWorkletThreadImpl> keep_alive_{this};
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_EXPORTED_WEB_SHARED_STORAGE_WORKLET_THREAD_IMPL_H_
