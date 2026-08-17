/*
 * Copyright (C) 2010 Google Inc. All rights reserved.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_INDEXEDDB_IDB_DATABASE_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_INDEXEDDB_IDB_DATABASE_H_

#include <memory>

#include "base/memory/scoped_refptr.h"
#include "third_party/blink/public/mojom/indexeddb/indexeddb.mojom-blink.h"
#include "third_party/blink/renderer/bindings/core/v8/active_script_wrappable.h"
#include "third_party/blink/renderer/bindings/modules/v8/v8_idb_object_store_parameters.h"
#include "third_party/blink/renderer/bindings/modules/v8/v8_idb_transaction_options.h"
#include "third_party/blink/renderer/core/dom/dom_string_list.h"
#include "third_party/blink/renderer/core/execution_context/execution_context_lifecycle_observer.h"
#include "third_party/blink/renderer/core/execution_context/execution_context_lifecycle_state_observer.h"
#include "third_party/blink/renderer/modules/event_modules.h"
#include "third_party/blink/renderer/modules/event_target_modules.h"
#include "third_party/blink/renderer/modules/indexeddb/idb_metadata.h"
#include "third_party/blink/renderer/modules/indexeddb/idb_object_store.h"
#include "third_party/blink/renderer/modules/indexeddb/idb_transaction.h"
#include "third_party/blink/renderer/modules/indexeddb/indexed_db.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_map.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_associated_receiver.h"
#include "third_party/blink/renderer/platform/scheduler/public/frame_or_worker_scheduler.h"

namespace blink {

class ExceptionState;
class ExecutionContext;
class ScriptState;
class V8UnionStringOrStringSequence;

class MODULES_EXPORT IDBDatabase final
    : public EventTarget,
      public ActiveScriptWrappable<IDBDatabase>,
      public ExecutionContextLifecycleStateObserver,
      public mojom::blink::IDBDatabaseCallbacks {
  DEFINE_WRAPPERTYPEINFO();

 public:
  IDBDatabase(
      ExecutionContext*,
      mojo::PendingAssociatedReceiver<mojom::blink::IDBDatabaseCallbacks>
          callbacks_receiver,
      mojo::PendingAssociatedRemote<mojom::blink::IDBDatabase> pending_database,
      int scheduling_priority);

  void Trace(Visitor*) const override;

  // Overwrites the database metadata, including object store and index
  // metadata. Used to pass metadata to the database when it is opened.
  void SetMetadata(const IDBDatabaseMetadata&);
  // Overwrites the database's own metadata, but does not change object store
  // and index metadata. Used to revert the database's metadata when a
  // versionchage transaction is aborted.
  void SetDatabaseMetadata(const IDBDatabaseMetadata&);
  void TransactionCreated(IDBTransaction*);

  // If `transaction` is an upgrade transaction, verifies that it is the same as
  // `version_change_transaction_` and clears that member. Called in both abort
  // and commit paths.
  void TransactionWillFinish(const IDBTransaction* transaction);

  // This will be called after the transaction's final event dispatch.
  void TransactionFinished(const IDBTransaction*);

  // Implement the IDL
  const String& name() const { return metadata_.name; }
  uint64_t version() const { return metadata_.version; }
  DOMStringList* objectStoreNames() const;

  IDBObjectStore* createObjectStore(const String& name,
                                    const IDBObjectStoreParameters* options,
                                    ExceptionState& exception_state) {
    return createObjectStore(name, IDBKeyPath(options->keyPath()),
                             options->autoIncrement(), exception_state);
  }
  IDBTransaction* transaction(ScriptState* script_state,
                              const V8UnionStringOrStringSequence* store_names,
                              const V8IDBTransactionMode& mode,
                              const IDBTransactionOptions* options,
                              ExceptionState& exception_state);
  void deleteObjectStore(const String& name, ExceptionState&);
  void close();

  DEFINE_ATTRIBUTE_EVENT_LISTENER(abort, kAbort)
  DEFINE_ATTRIBUTE_EVENT_LISTENER(close, kClose)
  DEFINE_ATTRIBUTE_EVENT_LISTENER(error, kError)
  DEFINE_ATTRIBUTE_EVENT_LISTENER(versionchange, kVersionchange)

  // mojom::blink::IDBDatabaseCallbacks:
  void ForcedClose() override;
  void VersionChange(int64_t old_version, int64_t new_version) override;
  void Abort(int64_t transaction_id,
             mojom::blink::IDBException code,
             const WTF::String& message) override;
  void Complete(int64_t transaction_id) override;

  // ScriptWrappable
  bool HasPendingActivity() const final;

  // ExecutionContextLifecycleStateObserver
  void ContextDestroyed() override;
  void ContextEnteredBackForwardCache() override;
  void ContextLifecycleStateChanged(
      mojom::blink::FrameLifecycleState state) override;

  // EventTarget
  const AtomicString& InterfaceName() const override;
  ExecutionContext* GetExecutionContext() const override;

  bool IsClosePending() const { return close_pending_; }
  const IDBDatabaseMetadata& Metadata() const { return metadata_; }

  int64_t FindObjectStoreId(const String& name) const;
  bool ContainsObjectStore(const String& name) const {
    return FindObjectStoreId(name) != IDBObjectStoreMetadata::kInvalidId;
  }
  void RenameObjectStore(int64_t store_id, const String& new_name);
  void RevertObjectStoreCreation(int64_t object_store_id);
  void RevertObjectStoreMetadata(
      scoped_refptr<IDBObjectStoreMetadata> old_metadata);

  static int64_t NextTransactionId();

  static const char kIndexDeletedErrorMessage[];
  static const char kIndexNameTakenErrorMessage[];
  static const char kIsKeyCursorErrorMessage[];
  static const char kNoKeyOrKeyRangeErrorMessage[];
  static const char kNoSuchIndexErrorMessage[];
  static const char kNoSuchObjectStoreErrorMessage[];
  static const char kNoValueErrorMessage[];
  static const char kNotValidKeyErrorMessage[];
  static const char kNotVersionChangeTransactionErrorMessage[];
  static const char kObjectStoreDeletedErrorMessage[];
  static const char kObjectStoreNameTakenErrorMessage[];
  static const char kRequestNotFinishedErrorMessage[];
  static const char kSourceDeletedErrorMessage[];
  static const char kTransactionFinishedErrorMessage[];
  static const char kTransactionInactiveErrorMessage[];
  static const char kTransactionReadOnlyErrorMessage[];
  static const char kDatabaseClosedErrorMessage[];

  static const int64_t kMinimumIndexId = 30;

  void RenameObjectStore(int64_t transaction_id,
                         int64_t object_store_id,
                         const String& new_name) {
    database_remote_->RenameObjectStore(transaction_id, object_store_id,
                                        new_name);
  }
  void CreateTransaction(mojo::PendingAssociatedReceiver<
                             mojom::blink::IDBTransaction> transaction_receiver,
                         int64_t transaction_id,
                         const Vector<int64_t>& object_store_ids,
                         mojom::blink::IDBTransactionMode mode,
                         mojom::blink::IDBTransactionDurability durability) {
    database_remote_->CreateTransaction(std::move(transaction_receiver),
                                        transaction_id, object_store_ids, mode,
                                        durability);
  }
  void VersionChangeIgnored() { database_remote_->VersionChangeIgnored(); }
  void Get(
      int64_t transaction_id,
      int64_t object_store_id,
      int64_t index_id,
      const IDBKeyRange*,
      bool key_only,
      base::OnceCallback<void(mojom::blink::IDBDatabaseGetResultPtr)> result);
  void GetAll(int64_t transaction_id,
              int64_t object_store_id,
              int64_t index_id,
              const IDBKeyRange*,
              mojom::blink::IDBGetAllResultType result_type,
              int64_t max_count,
              mojom::blink::IDBCursorDirection direction,
              IDBRequest*);
  void SetIndexKeys(int64_t transaction_id,
                    int64_t object_store_id,
                    std::unique_ptr<IDBKey> primary_key,
                    Vector<IDBIndexKeys>);
  void SetIndexesReady(int64_t transaction_id,
                       int64_t object_store_id,
                       const Vector<int64_t>& index_ids);
  void OpenCursor(int64_t object_store_id,
                  int64_t index_id,
                  const IDBKeyRange*,
                  mojom::blink::IDBCursorDirection direction,
                  bool key_only,
                  mojom::blink::IDBTaskType,
                  IDBRequest*);
  void Count(int64_t transaction_id,
             int64_t object_store_id,
             int64_t index_id,
             const IDBKeyRange*,
             mojom::blink::IDBDatabase::CountCallback callback);
  void Delete(int64_t transaction_id,
              int64_t object_store_id,
              const IDBKey* primary_key,
              mojom::blink::IDBDatabase::DeleteRangeCallback callback);
  void DeleteRange(int64_t transaction_id,
                   int64_t object_store_id,
                   const IDBKeyRange*,
                   mojom::blink::IDBDatabase::DeleteRangeCallback callback);
  void GetKeyGeneratorCurrentNumber(
      int64_t transaction_id,
      int64_t object_store_id,
      mojom::blink::IDBDatabase::GetKeyGeneratorCurrentNumberCallback callback);
  void Clear(int64_t transaction_id,
             int64_t object_store_id,
             mojom::blink::IDBDatabase::ClearCallback callback);
  void CreateIndex(int64_t transaction_id,
                   int64_t object_store_id,
                   int64_t index_id,
                   const String& name,
                   const IDBKeyPath&,
                   bool unique,
                   bool multi_entry);
  void DeleteIndex(int64_t transaction_id,
                   int64_t object_store_id,
                   int64_t index_id);
  void RenameIndex(int64_t transaction_id,
                   int64_t object_store_id,
                   int64_t index_id,
                   const String& new_name);
  void Abort(int64_t transaction_id);
  void DidBecomeInactive() { database_remote_->DidBecomeInactive(); }

  bool IsConnectionOpen() const;

  int scheduling_priority() const { return scheduling_priority_; }

  // Converts a lifecycle state to a priority integer. Lower values represent
  // higher priority.
  //
  // A note on the input type: the scheduling lifecycle state is used as an
  // imperfect proxy for the general priority of the frame. Its primary
  // advantage is that it is synchronously accessible during the flow of
  // creating a transaction. In contrast, the concept of priority in the
  // browser's `PerformanceManager` would require asynchronous lookup from IDB
  // backend code (which runs on a threadpool), which would add latency to
  // transaction processing. The scheduler's lifecycle state may be slightly out
  // of date if there are in-flight IPC from the browser, but:
  //
  // * prioritization is somewhat heuristic anyway
  // * nothing should break if the priority is occasionally misjudged.
  // * `scheduler_observer_` should eventually pick up and forward updates.
  static int GetSchedulingPriority(
      scheduler::SchedulingLifecycleState lifecycle_state);

 protected:
  // EventTarget
  DispatchEventResult DispatchEventInternal(Event&) override;

 private:
  IDBObjectStore* createObjectStore(const String& name,
                                    const IDBKeyPath&,
                                    bool auto_increment,
                                    ExceptionState&);
  void CloseConnection();

  void OnSchedulerLifecycleStateChanged(
      scheduler::SchedulingLifecycleState lifecycle_state);

  IDBDatabaseMetadata metadata_;
  HeapMojoAssociatedRemote<mojom::blink::IDBDatabase> database_remote_;
  Member<IDBTransaction> version_change_transaction_;
  HeapHashMap<int64_t, Member<IDBTransaction>> transactions_;

  bool close_pending_ = false;

  // See notes above `GetSchedulingPriority`.
  int scheduling_priority_;
  std::unique_ptr<FrameOrWorkerScheduler::LifecycleObserverHandle>
      scheduler_observer_;

  HeapMojoAssociatedReceiver<mojom::blink::IDBDatabaseCallbacks, IDBDatabase>
      callbacks_receiver_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_INDEXEDDB_IDB_DATABASE_H_
