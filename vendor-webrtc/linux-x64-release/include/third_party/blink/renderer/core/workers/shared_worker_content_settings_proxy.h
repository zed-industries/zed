// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_WORKERS_SHARED_WORKER_CONTENT_SETTINGS_PROXY_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_WORKERS_SHARED_WORKER_CONTENT_SETTINGS_PROXY_H_

#include "mojo/public/cpp/bindings/pending_remote.h"
#include "mojo/public/cpp/bindings/remote.h"
#include "third_party/blink/public/mojom/worker/worker_content_settings_proxy.mojom-blink-forward.h"
#include "third_party/blink/public/platform/web_content_settings_client.h"
#include "third_party/blink/public/platform/web_security_origin.h"
#include "third_party/blink/renderer/platform/weborigin/security_origin.h"

namespace blink {

// SharedWorkerContentSettingsProxy provides content settings information.
// This is created on the main thread and then called on the worker thread.
// Information is requested via a Mojo connection to the browser process.
class SharedWorkerContentSettingsProxy : public WebContentSettingsClient {
 public:
  SharedWorkerContentSettingsProxy(
      mojo::PendingRemote<mojom::blink::WorkerContentSettingsProxy> host_info);
  ~SharedWorkerContentSettingsProxy() override;

  // WebContentSettingsClient overrides.
  bool AllowStorageAccessSync(StorageType storage_type) override;

 private:
  // To ensure the returned pointer is destructed on the same thread
  // that it was constructed on, this uses ThreadSpecific.
  mojo::Remote<mojom::blink::WorkerContentSettingsProxy>& GetService();

  // This is set on the main thread at the ctor,
  // and moved to thread local storage on the worker thread
  // when GetService() is called for the first time.
  mojo::PendingRemote<mojom::blink::WorkerContentSettingsProxy> host_info_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_WORKERS_SHARED_WORKER_CONTENT_SETTINGS_PROXY_H_
