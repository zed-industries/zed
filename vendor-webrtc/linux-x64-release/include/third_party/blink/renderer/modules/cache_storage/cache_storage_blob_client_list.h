// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_CACHE_STORAGE_CACHE_STORAGE_BLOB_CLIENT_LIST_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_CACHE_STORAGE_CACHE_STORAGE_BLOB_CLIENT_LIST_H_

#include "mojo/public/cpp/bindings/receiver.h"
#include "third_party/blink/public/mojom/blob/blob.mojom-blink.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_vector.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/loader/fetch/data_pipe_bytes_consumer.h"

namespace blink {

class ExecutionContext;

// This class holds a list of BlobReaderClient implementations alive until
// they complete or the entire list is garbage collected.
class MODULES_EXPORT CacheStorageBlobClientList
    : public GarbageCollected<CacheStorageBlobClientList> {
 public:
  CacheStorageBlobClientList() = default;

  CacheStorageBlobClientList(const CacheStorageBlobClientList&) = delete;
  CacheStorageBlobClientList& operator=(const CacheStorageBlobClientList&) =
      delete;

  void AddClient(
      ExecutionContext* context,
      mojo::PendingReceiver<mojom::blink::BlobReaderClient>
          client_pending_receiver,
      DataPipeBytesConsumer::CompletionNotifier* completion_notifier);

  void Trace(Visitor* visitor) const;

 private:
  class Client;

  void RevokeClient(Client* client);

  HeapVector<Member<Client>> clients_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_CACHE_STORAGE_CACHE_STORAGE_BLOB_CLIENT_LIST_H_
