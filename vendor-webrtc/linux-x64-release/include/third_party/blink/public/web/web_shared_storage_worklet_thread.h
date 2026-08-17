// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_WEB_WEB_SHARED_STORAGE_WORKLET_THREAD_H_
#define THIRD_PARTY_BLINK_PUBLIC_WEB_WEB_SHARED_STORAGE_WORKLET_THREAD_H_

#include "base/memory/scoped_refptr.h"
#include "base/task/single_thread_task_runner.h"
#include "third_party/blink/public/mojom/shared_storage/shared_storage_worklet_service.mojom-forward.h"
#include "third_party/blink/public/mojom/worker/worklet_global_scope_creation_params.mojom-forward.h"
#include "third_party/blink/public/platform/cross_variant_mojo_util.h"
#include "third_party/blink/public/platform/web_common.h"

namespace blink {

// An interface to start a self-owned shared storage worklet thread.
class BLINK_EXPORT WebSharedStorageWorkletThread {
 public:
  static void Start(
      scoped_refptr<base::SingleThreadTaskRunner> main_thread_runner,
      CrossVariantMojoReceiver<mojom::SharedStorageWorkletServiceInterfaceBase>
          receiver,
      mojom::WorkletGlobalScopeCreationParamsPtr global_scope_creation_params);

  virtual ~WebSharedStorageWorkletThread() = default;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_WEB_WEB_SHARED_STORAGE_WORKLET_THREAD_H_
