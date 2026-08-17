// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_INDEXEDDB_MOCK_IDB_TRANSACTION_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_INDEXEDDB_MOCK_IDB_TRANSACTION_H_

#include <gmock/gmock.h>
#include <memory>

#include "mojo/public/cpp/bindings/associated_receiver.h"
#include "mojo/public/cpp/bindings/pending_associated_receiver.h"
#include "third_party/blink/public/mojom/indexeddb/indexeddb.mojom-blink.h"

namespace blink {

// Mock for the **remote** (i.e. the browser side of the Mojo connection).
class MockIDBTransaction
    : public testing::StrictMock<mojom::blink::IDBTransaction> {
 public:
  MOCK_METHOD(void,
              CreateObjectStore,
              (int64_t object_store_id,
               const String& name,
               const IDBKeyPath&,
               bool auto_increment),
              (override));
  MOCK_METHOD(void, DeleteObjectStore, (int64_t object_store_id), (override));
  MOCK_METHOD(void,
              Put,
              (int64_t object_store_id,
               std::unique_ptr<IDBValue> value,
               std::unique_ptr<IDBKey> key,
               mojom::blink::IDBPutMode,
               Vector<IDBIndexKeys>,
               PutCallback callback),
              (override));
  MOCK_METHOD(void, Commit, (int64_t num_errors_handled), (override));

  void Bind(mojo::PendingAssociatedReceiver<mojom::blink::IDBTransaction>);

 private:
  mojo::AssociatedReceiver<mojom::blink::IDBTransaction> receiver_{this};
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_INDEXEDDB_MOCK_IDB_TRANSACTION_H_
