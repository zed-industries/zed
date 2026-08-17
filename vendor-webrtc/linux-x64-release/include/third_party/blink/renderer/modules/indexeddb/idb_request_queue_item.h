// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_INDEXEDDB_IDB_REQUEST_QUEUE_ITEM_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_INDEXEDDB_IDB_REQUEST_QUEUE_ITEM_H_

#include <memory>

#include "base/dcheck_is_on.h"
#include "base/functional/callback.h"
#include "base/memory/weak_ptr.h"
#include "mojo/public/cpp/bindings/pending_associated_remote.h"
#include "third_party/blink/public/mojom/indexeddb/indexeddb.mojom-blink.h"
#include "third_party/blink/renderer/modules/indexeddb/idb_record_array.h"
#include "third_party/blink/renderer/platform/heap/persistent.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/deque.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {

class DOMException;
class IDBDatabaseGetAllResultSinkImpl;
class IDBKey;
class IDBRequest;
class IDBRequestLoader;
class IDBRequestTest;
class IDBValue;

// Queues up a transaction's IDBRequest results for orderly delivery.
//
// The IndexedDB specification requires that the events corresponding to IDB
// request results fire in the order in which the requests were issued. The
// browser-side backend processes requests in order, but the Blink side may need
// to perform post-processing on the results (e.g. large value unwrapping).
// When a result needs post-processing, this queue captures all results received
// during the post-processing steps. The events for these results may only fire
// after the post-processed result's event is fired.
//
// A queue item holds a Persistent (not garbage-collected) reference to the
// IDBRequest whose result it will deliver. This creates a reference cycle,
// because IDBRequest holds a pointer to its IDBTransaction, and IDBTransaction
// stores all pending IDBRequestQueueItem instances for its requests in a queue
// via non-garbage-collected pointers (std::unique_ptr). To avoid leaks, the
// request-processing code must ensure that IDBTransaction's queue gets drained
// at some point, even if the transaction's ExecutionContext goes away. The
// lifecycle tests in IDBTransactionTest aim to cover this requirement.
//
// Given that the cycle above exists, the closures passed to IDBRequestQueueItem
// can safely store Persistent pointers the IDBRequest or to the IDBTransaction.
class MODULES_EXPORT IDBRequestQueueItem {
  USING_FAST_MALLOC(IDBRequestQueueItem);

 public:
  IDBRequestQueueItem(IDBRequest*,
                      DOMException*,
                      base::OnceClosure on_result_ready);
  IDBRequestQueueItem(IDBRequest*, int64_t, base::OnceClosure on_result_ready);
  IDBRequestQueueItem(IDBRequest*, base::OnceClosure on_result_ready);
  IDBRequestQueueItem(IDBRequest*,
                      std::unique_ptr<IDBKey>,
                      base::OnceClosure on_result_ready);
  IDBRequestQueueItem(IDBRequest*,
                      std::unique_ptr<IDBValue>,
                      base::OnceClosure on_load_complete);
  IDBRequestQueueItem(IDBRequest*,
                      std::unique_ptr<IDBKey>,
                      std::unique_ptr<IDBKey> primary_key,
                      std::unique_ptr<IDBValue>,
                      base::OnceClosure on_result_ready);
  IDBRequestQueueItem(
      IDBRequest*,
      mojo::PendingAssociatedRemote<mojom::blink::IDBCursor> pending_cursor,
      std::unique_ptr<IDBKey>,
      std::unique_ptr<IDBKey> primary_key,
      std::unique_ptr<IDBValue>,
      base::OnceClosure on_result_ready);
  // Asynchronous fetching of multiple results.
  IDBRequestQueueItem(
      IDBRequest*,
      mojom::blink::IDBGetAllResultType get_all_result_type,
      mojo::PendingAssociatedReceiver<mojom::blink::IDBDatabaseGetAllResultSink>
          receiver,
      base::OnceClosure on_result_ready);

  ~IDBRequestQueueItem();

  // False if this result still requires post-processing.
  inline bool IsReady() { return ready_; }

  // The request whose queued result is tracked by this item.
  inline IDBRequest* Request() { return request_; }

  // Starts post-processing the IDBRequest's result.
  //
  // This method must be called after the IDBRequestQueueItem is enqueued into
  // the appropriate queue, because it is possible for the loading operation to
  // complete synchronously, in which case IDBTransaction::OnResultReady() will
  // be called with the (presumably) enqueued IDBRequest before this method
  // returns.
  void StartLoading();

  // Stops post-processing the IDBRequest's result.
  //
  // This method may be called without an associated StartLoading().
  void CancelLoading();

  // Calls the correct SendResult overload on the associated request.
  //
  // This should only be called by the request's IDBTransaction.
  void SendResult();

 private:
  friend class IDBDatabaseGetAllResultSinkImpl;
  friend class IDBRequestTest;

  // The IDBRequest callback that will be called for this result.
  enum ResponseType {
    kCanceled,
    kCursorKeyPrimaryKeyValue,
    kError,
    kNumber,
    kKey,
    kKeyArray,
    kKeyPrimaryKeyValue,
    kRecordArray,
    kValue,
    kValueArray,
    kVoid,
  };

  // Checks `values_` for wrapped entries and, if there are any, creates
  // `loader_` to unwrap them. Returns whether the loader was created.
  // Note that `values_` is std::move'd into the loader when it's created.
  bool MaybeCreateLoader();

  // Callback run from IDBRequestLoader with the unwrapped values.
  void OnLoadComplete(Vector<std::unique_ptr<IDBValue>>&& values,
                      DOMException* error);

  // Handle completion of post-processing, if any, of the result.
  void OnResultReady();

  // The IDBRequest that will receive a callback for this result.
  Persistent<IDBRequest> request_;

  // The error argument to the IDBRequest callback.
  //
  // Only used if the `response_type_` is `kError`.
  Persistent<DOMException> error_;

  // The key argument to the IDBRequest callback.
  //
  // Only used if `response_type_` is `kKey`, `kKeyPrimaryKeyValue` or
  // `kCursorKeyPrimaryKeyValue`.
  std::unique_ptr<IDBKey> key_;

  // The primary_key argument to the IDBRequest callback.
  //
  // Only used if `response_type_` is `kKeyPrimaryKeyValue` or
  // `kCursorKeyPrimaryKeyValue`.
  std::unique_ptr<IDBKey> primary_key_;

  // Contains 3 vectors: 1 for primary keys, 1 for values and 1 for index keys.
  // Several kinds of `response_type_` use 1 or more of these vectors:
  //
  // (1) `kValue`, `kKeyPrimaryKeyValue` and `kCursorKeyPrimaryKeyValue` store
  //     a single value in `records_.values`:
  //
  // (2) `kKeyArray` stores keys in `records_.primary_keys` for `getAllKeys()`.
  //
  // (3) `kValueArray` stores values in `records_.values` for `getAll()`.
  //
  // (4) `kRecordArray` stores primary keys, index keys and values in
  //     `records_` for `getAllRecords()`.  The vectors are parallel.
  //     `IDBObject::getAllRecords()` leaves `records_.index_keys` empty.
  IDBRecordArray records_;

  // The cursor argument to the IDBRequest callback.
  mojo::PendingAssociatedRemote<mojom::blink::IDBCursor> pending_cursor_;

  // Asynchronous result collection for get all.
  std::unique_ptr<IDBDatabaseGetAllResultSinkImpl> get_all_sink_;

  // Performs post-processing on this result.
  //
  // nullptr for results that do not require post-processing and for results
  // whose post-processing has completed.
  Persistent<IDBRequestLoader> loader_;

  // Called when result post-processing has completed.
  base::OnceClosure on_result_ready_;

  // The integer value argument to the IDBRequest callback.
  int64_t int64_value_;

  // Identifies the IDBRequest::SendResult() overload that will be called.
  ResponseType response_type_;

  // This always starts as false. Call `StartLoading()` to begin loading and
  // post-processing. If none is required, `ready_` will be set to true and
  // `on_result_ready_` will be fired immediately.
  bool ready_ = false;

  // True iff this result has started loading.
  bool started_loading_ = false;

#if DCHECK_IS_ON()
  // True if the appropriate `SendResult()` method was called in `IDBRequest`.
  //
  // If `CancelLoading()` is called, `response_type_` might be `kCanceled`. In
  // this case, `result_sent_` can be set to true even though no
  // `IDBRequest::SendResult()` call occurs.
  bool result_sent_ = false;
#endif  // DCHECK_IS_ON()

  // A WeakPtr to this class is passed in the callback to IDBRequestLoader for
  // correctness and future-proofing. There is no known case currently where
  // the callback is dispatched after we're destroyed.
  base::WeakPtrFactory<IDBRequestQueueItem> weak_factory_{this};
};

using IDBRequestQueue = Deque<std::unique_ptr<IDBRequestQueueItem>>;

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_INDEXEDDB_IDB_REQUEST_QUEUE_ITEM_H_
