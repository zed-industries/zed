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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_WORKERS_SHARED_WORKER_CLIENT_HOLDER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_WORKERS_SHARED_WORKER_CLIENT_HOLDER_H_

#include "base/memory/ptr_util.h"
#include "base/task/single_thread_task_runner.h"
#include "mojo/public/cpp/bindings/pending_remote.h"
#include "services/metrics/public/cpp/ukm_source_id.h"
#include "third_party/blink/public/mojom/blob/blob_url_store.mojom-blink-forward.h"
#include "third_party/blink/public/mojom/worker/shared_worker_client.mojom-blink.h"
#include "third_party/blink/public/mojom/worker/shared_worker_connector.mojom-blink.h"
#include "third_party/blink/public/mojom/worker/shared_worker_info.mojom-blink.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/execution_context/execution_context_lifecycle_observer.h"
#include "third_party/blink/renderer/core/frame/local_dom_window.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_remote.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_unique_receiver_set.h"
#include "third_party/blink/renderer/platform/wtf/forward.h"

namespace blink {

class MessagePortChannel;
class KURL;
class SharedWorker;

// This holds SharedWorkerClients connecting with SharedWorkerHosts in the
// browser process. Every call of SharedWorkerClientHolder::Connect() creates a
// new client instance regardless of existing connections, and keeps it until
// the connection gets lost.
//
// SharedWorkerClientHolder is a per-LocalDOMWindow object and owned by
// LocalDOMWindow via Supplement<LocalDOMWindow>.
class CORE_EXPORT SharedWorkerClientHolder final
    : public GarbageCollected<SharedWorkerClientHolder>,
      public Supplement<LocalDOMWindow> {
 public:
  static const char kSupplementName[];
  static SharedWorkerClientHolder* From(LocalDOMWindow&);

  explicit SharedWorkerClientHolder(LocalDOMWindow&);
  SharedWorkerClientHolder(const SharedWorkerClientHolder&) = delete;
  SharedWorkerClientHolder& operator=(const SharedWorkerClientHolder&) = delete;
  ~SharedWorkerClientHolder() = default;

  // Establishes a connection with SharedWorkerHost in the browser process.
  // `connector_override` is used to force creation of the shared worker on
  // a custom worker pool instead of the default pool `connector_`.
  void Connect(SharedWorker*,
               MessagePortChannel,
               const KURL&,
               mojo::PendingRemote<mojom::blink::BlobURLToken>,
               mojom::blink::WorkerOptionsPtr options,
               mojom::blink::SharedWorkerSameSiteCookies same_site_cookies,
               ukm::SourceId client_ukm_source_id,
               const HeapMojoRemote<mojom::blink::SharedWorkerConnector>*
                   connector_override,
               bool extended_lifetime);

  void Trace(Visitor* visitor) const override;

 private:
  HeapMojoRemote<mojom::blink::SharedWorkerConnector> connector_;
  HeapMojoUniqueReceiverSet<mojom::blink::SharedWorkerClient> client_receivers_;

  scoped_refptr<base::SingleThreadTaskRunner> task_runner_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_WORKERS_SHARED_WORKER_CLIENT_HOLDER_H_
