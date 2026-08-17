/*
 * Copyright (C) 2012 Google Inc. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 *
 * 1.  Redistributions of source code must retain the above copyright
 *     notice, this list of conditions and the following disclaimer.
 * 2.  Redistributions in binary form must reproduce the above copyright
 *     notice, this list of conditions and the following disclaimer in the
 *     documentation and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY APPLE AND ITS CONTRIBUTORS "AS IS" AND ANY
 * EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED
 * WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
 * DISCLAIMED. IN NO EVENT SHALL APPLE OR ITS CONTRIBUTORS BE LIABLE FOR ANY
 * DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES
 * (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES;
 * LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND
 * ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF
 * THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_INDEXEDDB_IDB_OPEN_DB_REQUEST_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_INDEXEDDB_IDB_OPEN_DB_REQUEST_H_

#include <memory>

#include "base/memory/raw_ptr.h"
#include "base/time/time.h"
#include "mojo/public/cpp/bindings/pending_associated_receiver.h"
#include "third_party/blink/public/mojom/indexeddb/indexeddb.mojom-blink.h"
#include "third_party/blink/renderer/modules/indexeddb/idb_factory_client.h"
#include "third_party/blink/renderer/modules/indexeddb/idb_request.h"
#include "third_party/blink/renderer/modules/indexeddb/idb_transaction.h"
#include "third_party/blink/renderer/modules/modules_export.h"

namespace blink {

class MODULES_EXPORT IDBOpenDBRequest final : public IDBRequest {
  DEFINE_WRAPPERTYPEINFO();

 public:
  IDBOpenDBRequest(
      ScriptState*,
      mojo::PendingAssociatedReceiver<mojom::blink::IDBDatabaseCallbacks>
          callbacks_receiver,
      IDBTransaction::TransactionMojoRemote transaction_remote,
      int64_t transaction_id,
      int64_t version,
      IDBRequest::AsyncTraceState metrics);
  ~IDBOpenDBRequest() override;

  void Trace(Visitor*) const override;

  // Returns a new IDBFactoryClient for this request.
  //
  // Each call must be paired with a FactoryClientDestroyed() call.
  std::unique_ptr<IDBFactoryClient> CreateFactoryClient();
  void FactoryClientDestroyed(IDBFactoryClient*);

  // These methods dispatch results directly, skipping the transaction's result
  // queue (see IDBRequest::HandleResponse()). This is safe because the open
  // request cannot be issued after a request that needs processing.
  void OnBlocked(int64_t existing_version);
  void OnUpgradeNeeded(int64_t old_version,
                       mojo::PendingAssociatedRemote<mojom::blink::IDBDatabase>,
                       scoped_refptr<base::SingleThreadTaskRunner>,
                       const IDBDatabaseMetadata&,
                       mojom::blink::IDBDataLoss,
                       String data_loss_message);
  void OnOpenDBSuccess(mojo::PendingAssociatedRemote<mojom::blink::IDBDatabase>,
                       scoped_refptr<base::SingleThreadTaskRunner>,
                       const IDBDatabaseMetadata&);
  void OnDeleteDBSuccess(int64_t old_version);
  void OnDBFactoryError(DOMException*);

  // ExecutionContextLifecycleObserver
  void ContextDestroyed() final;

  // EventTarget
  const AtomicString& InterfaceName() const override;

  void set_connection_priority(int priority) {
    connection_priority_ = priority;
    metrics_.set_is_fg_client(priority == 0);
  }

  DEFINE_ATTRIBUTE_EVENT_LISTENER(blocked, kBlocked)
  DEFINE_ATTRIBUTE_EVENT_LISTENER(upgradeneeded, kUpgradeneeded)

 protected:
  bool CanStillSendResult() const override;

  // EventTarget
  DispatchEventResult DispatchEventInternal(Event&) override;

 private:
  mojo::PendingAssociatedReceiver<mojom::blink::IDBDatabaseCallbacks>
      callbacks_receiver_;
  IDBTransaction::TransactionMojoRemote transaction_remote_;
  const int64_t transaction_id_;
  int64_t version_;

  base::Time start_time_;
  bool open_time_recorded_ = false;

  // The priority for this connection request which is passed to the backend.
  // This should be passed along to the database after a successful open
  // attempt.
  int connection_priority_ = 0;

  // Pointer back to the IDBFactoryClient that holds a persistent reference
  // to this object.
  raw_ptr<IDBFactoryClient> factory_client_ = nullptr;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_INDEXEDDB_IDB_OPEN_DB_REQUEST_H_
