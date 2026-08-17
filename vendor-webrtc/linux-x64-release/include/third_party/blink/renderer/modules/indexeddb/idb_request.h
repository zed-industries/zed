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
 * 3.  Neither the name of Apple Computer, Inc. ("Apple") nor the names of
 *     its contributors may be used to endorse or promote products derived
 *     from this software without specific prior written permission.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_INDEXEDDB_IDB_REQUEST_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_INDEXEDDB_IDB_REQUEST_H_

#include <memory>
#include <optional>
#include <utility>

#include "base/dcheck_is_on.h"
#include "base/memory/raw_ptr.h"
#include "base/memory/scoped_refptr.h"
#include "base/notreached.h"
#include "base/time/time.h"
#include "third_party/blink/public/mojom/indexeddb/indexeddb.mojom-blink-forward.h"
#include "third_party/blink/renderer/bindings/core/v8/active_script_wrappable.h"
#include "third_party/blink/renderer/bindings/core/v8/script_value.h"
#include "third_party/blink/renderer/core/dom/dom_string_list.h"
#include "third_party/blink/renderer/core/dom/events/event_listener.h"
#include "third_party/blink/renderer/core/dom/events/event_target.h"
#include "third_party/blink/renderer/core/execution_context/execution_context_lifecycle_observer.h"
#include "third_party/blink/renderer/core/probe/async_task_context.h"
#include "third_party/blink/renderer/modules/event_modules.h"
#include "third_party/blink/renderer/modules/indexeddb/idb_any.h"
#include "third_party/blink/renderer/modules/indexeddb/idb_transaction.h"
#include "third_party/blink/renderer/modules/indexeddb/indexed_db.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/blob/blob_data.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"

namespace blink {

class DOMException;
class ExceptionState;
class IDBCursor;
class IDBValue;
class ScriptState;
class V8IDBRequestReadyState;
class V8UnionIDBCursorOrIDBIndexOrIDBObjectStore;

class MODULES_EXPORT IDBRequest : public EventTarget,
                                  public ActiveScriptWrappable<IDBRequest>,
                                  public ExecutionContextLifecycleObserver {
  DEFINE_WRAPPERTYPEINFO();

 public:
  using Source = V8UnionIDBCursorOrIDBIndexOrIDBObjectStore;

  // A type that can be used to identify this request for tracing or UMA.
  enum class TypeForMetrics {
    kCursorAdvance,
    kCursorContinue,
    kCursorContinuePrimaryKey,
    kCursorDelete,

    kFactoryOpen,
    kFactoryDeleteDatabase,

    kIndexOpenCursor,
    kIndexCount,
    kIndexOpenKeyCursor,
    kIndexGet,
    kIndexGetAll,
    kIndexGetAllKeys,
    kIndexGetKey,

    kObjectStoreGet,
    kObjectStoreGetKey,
    kObjectStoreGetAll,
    kObjectStoreGetAllKeys,
    kObjectStoreDelete,
    kObjectStoreClear,
    kObjectStoreCreateIndex,
    kObjectStorePut,
    kObjectStoreAdd,
    kObjectStoreUpdate,
    kObjectStoreOpenCursor,
    kObjectStoreOpenKeyCursor,
    kObjectStoreCount,
    kObjectStoreGetAllRecords,
    kIndexGetAllRecords,
  };

  // Container for async tracing state.
  //
  // The documentation for TRACE_EVENT_NESTABLE_ASYNC_{BEGIN,END} suggests
  // identifying trace events by using pointers or a counter that is always
  // incremented on the same thread. This is not viable for IndexedDB, because
  // the same object can result in multiple trace events (requests associated
  // with cursors), and IndexedDB can be used from multiple threads in the same
  // renderer (workers). Furthermore, we want to record the beginning event of
  // an async trace right when we start serving an IDB API call, before the
  // IDBRequest object is created, so we can't rely on information in an
  // IDBRequest.
  //
  // This class solves the ID uniqueness problem by relying on an atomic counter
  // to generating unique IDs in a threadsafe manner. The atomic machinery is
  // used when tracing is enabled. The recording problem is solved by having
  // instances of this class store the information needed to record async trace
  // end events (via TRACE_EVENT_NESTABLE_ASYNC_END).
  //
  // From a mechanical perspective, creating an AsyncTraceState instance records
  // the beginning event of an async trace. The instance is then moved into an
  // IDBRequest, which records the async trace's end event at the right time.
  class MODULES_EXPORT AsyncTraceState {
   public:
    // Creates an empty instance, which does not produce any tracing events.
    //
    // This is used for internal requests that should not show up in an
    // application's trace. Examples of internal requests are the requests
    // issued by DevTools, and the requests used to populate indexes.
    AsyncTraceState() = default;

    // Disallow copy and assign.
    AsyncTraceState(const AsyncTraceState&) = delete;
    AsyncTraceState& operator=(const AsyncTraceState&) = delete;

    // Creates an instance that produces begin/end events of the given type.
    explicit AsyncTraceState(TypeForMetrics type);
    ~AsyncTraceState();

    // Used to transfer the trace end event state to an IDBRequest.
    AsyncTraceState(AsyncTraceState&& other) {
      DCHECK(IsEmpty());
      type_ = other.type_;
      other.type_.reset();
      start_time_ = other.start_time_;
      other.start_time_ = base::TimeTicks();
      id_ = other.id_;
      other.id_ = 0;
      is_fg_client_ = other.is_fg_client_;
    }
    AsyncTraceState& operator=(AsyncTraceState&& rhs) {
      DCHECK(IsEmpty());
      type_ = rhs.type_;
      rhs.type_.reset();
      start_time_ = rhs.start_time_;
      rhs.start_time_ = base::TimeTicks();
      id_ = rhs.id_;
      rhs.id_ = 0;
      is_fg_client_ = rhs.is_fg_client_;
      return *this;
    }

    // True if this instance does not store information for a tracing end event.
    //
    // An instance is cleared when RecordAndReset() is called on it, or when its
    // state is moved into a different instance. Empty instances are also
    // produced by the AsyncStateTrace() constructor.
    bool IsEmpty() const { return !type_; }

    // Records the trace end event whose information is stored in this instance.
    //
    // The method results in the completion of the async trace tracked by this
    // instance, so the instance is cleared.
    void RecordAndReset();

    // Records the trace end event and resets the instance, and also emits to
    // histograms that are relevant to this request type. `success` is true when
    // the dispatch result is not an error.
    void WillDispatchResult(bool success);

    void set_is_fg_client(bool is_fg_client) { is_fg_client_ = is_fg_client; }

   protected:  // For testing
    std::optional<TypeForMetrics> type() const { return type_; }
    const base::TimeTicks& start_time() const { return start_time_; }
    size_t id() const { return id_; }

   private:
    friend class IDBRequest;

    std::optional<TypeForMetrics> type_;
    base::TimeTicks start_time_;

    // This tracks whether the request is associated with a highest-priority
    // ExecutionContext (i.e. foreground tab), **as of when the request was
    // issued**.
    bool is_fg_client_ = false;

    // Uniquely generated ID that ties an async trace's begin and end events.
    size_t id_ = 0;
  };

  static IDBRequest* Create(ScriptState*,
                            IDBIndex* source,
                            IDBTransaction*,
                            AsyncTraceState);
  static IDBRequest* Create(ScriptState*,
                            IDBObjectStore* source,
                            IDBTransaction*,
                            AsyncTraceState);
  static IDBRequest* Create(ScriptState*,
                            IDBCursor*,
                            IDBTransaction* source,
                            AsyncTraceState);
  static IDBRequest* Create(ScriptState*,
                            const Source*,
                            IDBTransaction*,
                            AsyncTraceState);

  IDBRequest(ScriptState* script_state,
             const Source* source,
             IDBTransaction* transaction,
             AsyncTraceState metrics);
  ~IDBRequest() override;

  void Trace(Visitor*) const override;

  v8::Isolate* GetIsolate() const { return isolate_; }
  ScriptValue result(ScriptState*, ExceptionState&);
  DOMException* error(ExceptionState&) const;
  const Source* source(ScriptState* script_state) const;
  IDBTransaction* transaction() const { return transaction_.Get(); }

  bool isResultDirty() const { return result_dirty_; }
  IDBAny* ResultAsAny() const { return result_.Get(); }

  // Requests made during index population are implementation details and so
  // events should not be visible to script.
  void PreventPropagation() { prevent_propagation_ = true; }

  // Defined in the IDL
  enum ReadyState { PENDING = 1, DONE = 2, kEarlyDeath = 3 };

  V8IDBRequestReadyState readyState() const;

  DEFINE_ATTRIBUTE_EVENT_LISTENER(success, kSuccess)
  DEFINE_ATTRIBUTE_EVENT_LISTENER(error, kError)

  void SetCursorDetails(indexed_db::CursorType, mojom::IDBCursorDirection);
  void SetPendingCursor(IDBCursor*);

  // Step 5 of https://w3c.github.io/IndexedDB/#abort-a-transaction
  // requires this step to be queued rather than executed synchronously:
  //
  //     For each request of transaction’s request list
  //     [...] queue a task to run these steps
  //
  // Enforced by WPT: transaction-abort-request-error.html
  // In some situations, `Abort()` will have been initiated by the backend, in
  // which case this call is already executing in the task queue and
  // `queue_dispatch` should be false.
  void Abort(bool queue_dispatch);

  // Blink's delivery of results from IndexedDB's backing store to script is
  // more complicated than prescribed in the IndexedDB specification.
  //
  // IDBValue, which holds responses from the backing store, is either the
  // serialized V8 value, or a reference to a Blob that holds the serialized
  // value. IDBValueWrapping.h has the motivation and details. This introduces
  // the following complexities.
  //
  // 1) De-serialization is expensive, so it is done lazily in
  // IDBRequest::result(), which is called synchronously from script. On the
  // other hand, Blob data can only be fetched asynchronously. So, IDBValues
  // that reference serialized data stored in Blobs must be processed before
  // IDBRequest event handlers are invoked, because the event handler script may
  // call IDBRequest::result().
  //
  // 2) The IDBRequest events must be dispatched in the order in which the
  // requests were issued. If an IDBValue references a Blob, the Blob processing
  // must block event dispatch for all following IDBRequests in the same
  // transaction.
  //
  // HandleResponse() will create an IDBRequestQueueItem and append it to the
  // transaction's request list. The IDBRequestQueueItem will handle all blob
  // processing and then signal the Transaction that it's done. The blob
  // processing can complete synchronously, or there may be no blobs to process.
  // When the result is ready, the IDBRequestQueueItem will dispatch it via
  // `SendResult()`.
  void HandleResponse(std::unique_ptr<IDBKey>);
  void HandleResponse(std::unique_ptr<IDBValue>);
  void HandleResponseAdvanceCursor(std::unique_ptr<IDBKey>,
                                   std::unique_ptr<IDBKey> primary_key,
                                   std::unique_ptr<IDBValue>);
  void HandleResponse(int64_t);

  // Callbacks for various `IDBObjectStore` methods.
  void OnClear(bool success);
  void OnDelete(bool success);
  void OnCount(bool success, uint32_t count);
  void OnPut(mojom::blink::IDBTransactionPutResultPtr result);
  void OnGet(mojom::blink::IDBDatabaseGetResultPtr result);
  void OnGetAll(
      mojom::blink::IDBGetAllResultType result_type,
      mojo::PendingAssociatedReceiver<mojom::blink::IDBDatabaseGetAllResultSink>
          receiver);
  void OnOpenCursor(mojom::blink::IDBDatabaseOpenCursorResultPtr result);
  void OnAdvanceCursor(mojom::blink::IDBCursorResultPtr result);
  void OnGotKeyGeneratorCurrentNumber(int64_t number,
                                      mojom::blink::IDBErrorPtr error);

  // ScriptWrappable
  bool HasPendingActivity() const final;

  // ExecutionContextLifecycleObserver
  void ContextDestroyed() override;

  // EventTarget
  const AtomicString& InterfaceName() const override;
  ExecutionContext* GetExecutionContext() const final;

  // Called by a version change transaction that has finished to set this
  // request back from DONE (following "upgradeneeded") back to PENDING (for
  // the upcoming "success" or "error").
  void TransactionDidFinishAndDispatch();

  IDBCursor* GetResultCursor() const;

#if DCHECK_IS_ON()
  inline bool TransactionHasQueuedResults() const {
    return transaction_ && transaction_->HasQueuedResults();
  }
#endif  // DCHECK_IS_ON()

#if DCHECK_IS_ON()
  inline IDBRequestQueueItem* QueueItem() const { return queue_item_; }
#endif  // DCHECK_IS_ON()

  void AssignNewMetrics(AsyncTraceState metrics);

 protected:
  virtual bool CanStillSendResult() const;
  void SetResult(IDBAny*);
  // Sets `error_` and dispatches the exception to event listeners. When `force`
  // is true, this will ignore the status of `request_aborted_`, which might
  // otherwise block dispatch.
  void SendError(DOMException*, bool force = false);

  // EventTarget
  DispatchEventResult DispatchEventInternal(Event&) override;

  // Can be nullptr for requests that are not associated with a transaction,
  // i.e. delete requests and completed or unsuccessful open requests.
  Member<IDBTransaction> transaction_;

  ReadyState ready_state_ = PENDING;
  bool request_aborted_ = false;  // May be aborted by transaction then receive
                                  // async onsuccess; ignore vs. assert.
  // Maintain the isolate so that all externally allocated memory can be
  // registered against it.
  raw_ptr<v8::Isolate> isolate_;

  probe::AsyncTaskContext* async_task_context() { return &async_task_context_; }

  AsyncTraceState metrics_;

 private:
  friend class IDBRequestTest;

  // Calls SendResult().
  friend class IDBRequestQueueItem;

  // See docs above for HandleResponse() variants.
  void HandleResponse();

  void HandleError(mojom::blink::IDBErrorPtr error);

  // Sets the result and dispatches a success event to listeners.
  void SendResult(IDBAny*);

  // Speciality versions of `SendResult()`.
  void SendResultValue(std::unique_ptr<IDBValue> value);
  void SendResultCursor(mojo::PendingAssociatedRemote<mojom::blink::IDBCursor>,
                        std::unique_ptr<IDBKey>,
                        std::unique_ptr<IDBKey> primary_key,
                        std::unique_ptr<IDBValue>);
  // Uses `pending_cursor_`.
  void SendResultAdvanceCursor(std::unique_ptr<IDBKey>,
                               std::unique_ptr<IDBKey> primary_key,
                               std::unique_ptr<IDBValue>);
  void SendResultCursorInternal(IDBCursor*,
                                std::unique_ptr<IDBKey>,
                                std::unique_ptr<IDBKey> primary_key,
                                std::unique_ptr<IDBValue>);

  Member<const Source> source_;
  Member<IDBAny> result_;
  Member<DOMException> error_;

  bool has_pending_activity_ = true;

  // Only used if the result type will be a cursor.
  indexed_db::CursorType cursor_type_ = indexed_db::kCursorKeyAndValue;
  mojom::IDBCursorDirection cursor_direction_ = mojom::IDBCursorDirection::Next;
  // When a cursor is continued/advanced, |result_| is cleared and
  // |pendingCursor_| holds it.
  Member<IDBCursor> pending_cursor_;
  // New state is not applied to the cursor object until the event is
  // dispatched.
  std::unique_ptr<IDBKey> cursor_key_;
  std::unique_ptr<IDBKey> cursor_primary_key_;
  std::unique_ptr<IDBValue> cursor_value_;

  bool did_fire_upgrade_needed_event_ = false;
  bool prevent_propagation_ = false;
  bool result_dirty_ = true;

  // Non-null while this request is queued behind other requests that are still
  // getting post-processed.
  //
  // The IDBRequestQueueItem is owned by the result queue in IDBTransaction.
  raw_ptr<IDBRequestQueueItem> queue_item_ = nullptr;

  probe::AsyncTaskContext async_task_context_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_INDEXEDDB_IDB_REQUEST_H_
