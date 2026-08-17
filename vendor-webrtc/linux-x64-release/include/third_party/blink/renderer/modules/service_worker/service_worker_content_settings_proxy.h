// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_SERVICE_WORKER_SERVICE_WORKER_CONTENT_SETTINGS_PROXY_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_SERVICE_WORKER_SERVICE_WORKER_CONTENT_SETTINGS_PROXY_H_

#include "mojo/public/cpp/bindings/pending_remote.h"
#include "mojo/public/cpp/bindings/remote.h"
#include "third_party/blink/public/mojom/worker/worker_content_settings_proxy.mojom-blink.h"
#include "third_party/blink/public/platform/web_content_settings_client.h"
#include "third_party/blink/public/platform/web_security_origin.h"
#include "third_party/blink/renderer/platform/weborigin/security_origin.h"

namespace blink {

// Provides the content settings information from browser process.  This proxy
// is created by WebEmbeddedWorkerImpl::StartWorkerContext() on a background
// ThreadPool thread, and destroyed on that thread. But all methods are called
// on the service worker thread.
class ServiceWorkerContentSettingsProxy final
    : public blink::WebContentSettingsClient {
 public:
  explicit ServiceWorkerContentSettingsProxy(
      mojo::PendingRemote<mojom::blink::WorkerContentSettingsProxy> host_info);

  ServiceWorkerContentSettingsProxy(const ServiceWorkerContentSettingsProxy&) =
      delete;
  ServiceWorkerContentSettingsProxy& operator=(
      const ServiceWorkerContentSettingsProxy&) = delete;

  ~ServiceWorkerContentSettingsProxy() override;

  void SetSecurityOrigin(scoped_refptr<const blink::SecurityOrigin>);

  // WebContentSettingsClient overrides.
  // Asks the browser process about the settings.
  // Blocks until the response arrives.
  bool AllowStorageAccessSync(StorageType storage_type) override;

 private:
  // To ensure the returned pointer is destructed on the same thread
  // that it was constructed on, this uses ThreadSpecific.
  mojo::Remote<mojom::blink::WorkerContentSettingsProxy>& GetService();

  // This is set on the ThreadPool thread at the ctor, and moved to thread
  // local storage on the service worker thread when GetService() is called for
  // the first time.
  mojo::PendingRemote<mojom::blink::WorkerContentSettingsProxy> host_info_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_SERVICE_WORKER_SERVICE_WORKER_CONTENT_SETTINGS_PROXY_H_
