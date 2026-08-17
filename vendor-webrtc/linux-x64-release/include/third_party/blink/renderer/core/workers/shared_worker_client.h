// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_WORKERS_SHARED_WORKER_CLIENT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_WORKERS_SHARED_WORKER_CLIENT_H_

#include "third_party/blink/public/mojom/worker/shared_worker_client.mojom-blink.h"

#include "third_party/blink/renderer/platform/heap/persistent.h"

namespace blink {

class SharedWorker;

// This is a client that connects with a SharedWorkerHost in the browser
// process. There can be multiple clients (including in different renderer
// processes) per shared worker. Clients are managed by the owning document's
// SharedWorkerClientHolder.
class SharedWorkerClient final : public mojom::blink::SharedWorkerClient {
 public:
  explicit SharedWorkerClient(SharedWorker*);
  ~SharedWorkerClient() override;

  // mojom::blink::SharedWorkerClient overrides.
  void OnCreated(mojom::SharedWorkerCreationContextType) override;
  void OnConnected(const Vector<mojom::WebFeature>& features_used) override;
  void OnScriptLoadFailed(const String& error_message) override;
  void OnFeatureUsed(mojom::WebFeature feature) override;

 private:
  Persistent<SharedWorker> worker_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_WORKERS_SHARED_WORKER_CLIENT_H_
