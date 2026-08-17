/*
 * Copyright (C) 2013 Google Inc. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are
 * met:
 *
 * 1. Redistributions of source code must retain the above copyright
 * notice, this list of conditions and the following disclaimer.
 *
 * 2. Redistributions in binary form must reproduce the above
 * copyright notice, this list of conditions and the following disclaimer
 * in the documentation and/or other materials provided with the
 * distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY GOOGLE INC. AND ITS CONTRIBUTORS
 * "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 * LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
 * A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL GOOGLE INC.
 * OR ITS CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 * SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
 * LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
 * DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
 * THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_INDEXEDDB_IDB_FACTORY_CLIENT_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_INDEXEDDB_IDB_FACTORY_CLIENT_H_

#include "base/task/single_thread_task_runner.h"
#include "mojo/public/cpp/bindings/pending_associated_remote.h"
#include "third_party/blink/public/mojom/indexeddb/indexeddb.mojom-blink.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/heap/persistent.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

class IDBOpenDBRequest;
struct IDBDatabaseMetadata;

class MODULES_EXPORT IDBFactoryClient final
    : public mojom::blink::IDBFactoryClient {
  USING_FAST_MALLOC(IDBFactoryClient);

 public:
  explicit IDBFactoryClient(IDBOpenDBRequest* request);
  ~IDBFactoryClient() override;

  void DetachRequest();

  void Error(mojom::blink::IDBException code, const String& message) override;
  void OpenSuccess(
      mojo::PendingAssociatedRemote<mojom::blink::IDBDatabase> pending_database,
      const IDBDatabaseMetadata& metadata) override;
  void DeleteSuccess(int64_t) override;
  void Blocked(int64_t old_version) override;
  void UpgradeNeeded(
      mojo::PendingAssociatedRemote<mojom::blink::IDBDatabase> pending_database,
      int64_t old_version,
      mojom::blink::IDBDataLoss data_loss,
      const String& data_loss_message,
      const IDBDatabaseMetadata&) override;

 private:
  void Detach();
  void DetachFromRequest();

  Persistent<IDBOpenDBRequest> request_;
  scoped_refptr<base::SingleThreadTaskRunner> task_runner_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_INDEXEDDB_IDB_FACTORY_CLIENT_H_
