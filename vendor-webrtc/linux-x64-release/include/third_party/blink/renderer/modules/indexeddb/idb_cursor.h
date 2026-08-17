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

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_INDEXEDDB_IDB_CURSOR_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_INDEXEDDB_IDB_CURSOR_H_

#include <stdint.h>

#include <memory>

#include "base/dcheck_is_on.h"
#include "base/gtest_prod_util.h"
#include "base/memory/scoped_refptr.h"
#include "mojo/public/cpp/bindings/associated_remote.h"
#include "third_party/blink/public/mojom/indexeddb/indexeddb.mojom-blink.h"
#include "third_party/blink/renderer/bindings/core/v8/script_value.h"
#include "third_party/blink/renderer/bindings/modules/v8/v8_idb_cursor_direction.h"
#include "third_party/blink/renderer/modules/indexeddb/idb_factory_client.h"
#include "third_party/blink/renderer/modules/indexeddb/idb_key.h"
#include "third_party/blink/renderer/modules/indexeddb/idb_request.h"
#include "third_party/blink/renderer/modules/indexeddb/indexed_db.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_associated_remote.h"

namespace blink {

class ExceptionState;
class IDBAny;
class IDBTransaction;
class IDBValue;
class ScriptState;
class V8UnionIDBIndexOrIDBObjectStore;

class MODULES_EXPORT IDBCursor : public ScriptWrappable {
  DEFINE_WRAPPERTYPEINFO();

 public:
  using Source = V8UnionIDBIndexOrIDBObjectStore;

  static mojom::blink::IDBCursorDirection V8EnumToDirection(
      V8IDBCursorDirection::Enum mode);

  // Reset cursor prefetch caches for all cursors except `except_cursor`.
  // In most callers, `except_cursor` is passed as nullptr, causing all cursors
  // to have their prefetch cache to be reset.
  static void ResetCursorPrefetchCaches(int64_t transaction_id,
                                        IDBCursor* except_cursor);

  IDBCursor(
      mojo::PendingAssociatedRemote<mojom::blink::IDBCursor> pending_cursor,
      mojom::blink::IDBCursorDirection direction,
      IDBRequest* request,
      const Source* source,
      IDBTransaction* transaction);
  ~IDBCursor() override;

  void Trace(Visitor*) const override;
  void ContextWillBeDestroyed();

  // Implement the IDL
  V8IDBCursorDirection direction() const;
  ScriptValue key(ScriptState*);
  ScriptValue primaryKey(ScriptState*);
  ScriptValue value(ScriptState*);
  IDBRequest* request() { return request_.Get(); }
  const Source* source() const;

  IDBRequest* update(ScriptState*, const ScriptValue&, ExceptionState&);
  void advance(unsigned, ExceptionState&);
  void Continue(ScriptState*, const ScriptValue& key, ExceptionState&);
  void continuePrimaryKey(ScriptState*,
                          const ScriptValue& key,
                          const ScriptValue& primary_key,
                          ExceptionState&);
  IDBRequest* Delete(ScriptState*, ExceptionState&);

  bool isKeyDirty() const { return key_dirty_; }
  bool isPrimaryKeyDirty() const { return primary_key_dirty_; }
  bool isValueDirty() const { return value_dirty_; }

  void Continue(std::unique_ptr<IDBKey>,
                std::unique_ptr<IDBKey> primary_key,
                IDBRequest::AsyncTraceState,
                ExceptionState&);
  void PostSuccessHandlerCallback();
  bool IsDeleted() const;
  void Close();
  void SetValueReady(std::unique_ptr<IDBKey>,
                     std::unique_ptr<IDBKey> primary_key,
                     std::unique_ptr<IDBValue>);
  const IDBKey* IdbPrimaryKey() const;
  virtual bool IsKeyCursor() const { return true; }
  virtual bool IsCursorWithValue() const { return false; }

 private:
  // Used to implement IDBCursor.advance().
  void AdvanceImpl(uint32_t count, IDBRequest* request);

  // Used to implement IDBCursor.continue() and IDBCursor.continuePrimaryKey().
  //
  // The key and primary key are null when they are not supplied by the
  // application. When both arguments are null, the cursor advances by one
  // entry.
  //
  // The keys pointed to by IDBKey* are only guaranteed to be alive for
  // the duration of the call.
  void CursorContinue(const IDBKey* key,
                      const IDBKey* primary_key,
                      IDBRequest* request);

  void PrefetchCallback(IDBRequest* request,
                        mojom::blink::IDBCursorResultPtr result);

  // Called after a cursor request's success handler is executed.
  //
  // This is only used by the cursor prefetching logic, and does not result in
  // an IPC.
  void SetPrefetchData(Vector<std::unique_ptr<IDBKey>> keys,
                       Vector<std::unique_ptr<IDBKey>> primary_keys,
                       Vector<std::unique_ptr<IDBValue>> values);

  void CachedAdvance(uint32_t count, IDBRequest* request);
  void CachedContinue(IDBRequest* request);
  void AdvanceCallback(IDBRequest* request,
                       mojom::blink::IDBCursorResultPtr result);
  void ResetPrefetchCache();
  int64_t GetTransactionId() const;

  IDBObjectStore* EffectiveObjectStore() const;

  // Runs some common checks to make sure the state of `this` allows operations
  // to proceed. Returns true if so, otherwise returns false after throwing an
  // exception on `exception_state`. If `read_only_error_message` is non-null,
  // it will be enforced that `this` is a writable cursor.
  bool CheckForCommonExceptions(ExceptionState& exception_state,
                                const char* read_only_error_message);

  FRIEND_TEST_ALL_PREFIXES(IDBCursorTest, AdvancePrefetchTest);
  FRIEND_TEST_ALL_PREFIXES(IDBCursorTest, PrefetchReset);
  FRIEND_TEST_ALL_PREFIXES(IDBCursorTest, PrefetchTest);

  static constexpr int kPrefetchContinueThreshold = 2;
  static constexpr int kMinPrefetchAmount = 5;
  static constexpr int kMaxPrefetchAmount = 100;

  // Prefetch cache. Keys and values are stored in reverse order so that a
  // cache'd continue can pop a value off of the back and prevent new memory
  // allocations.
  Vector<std::unique_ptr<IDBKey>> prefetch_keys_;
  Vector<std::unique_ptr<IDBKey>> prefetch_primary_keys_;
  Vector<std::unique_ptr<IDBValue>> prefetch_values_;

  // Number of continue calls that would qualify for a pre-fetch.
  int continue_count_ = 0;

  // Number of items used from the last prefetch.
  int used_prefetches_ = 0;

  // Number of onsuccess handlers we are waiting for.
  int pending_onsuccess_callbacks_ = 0;

  // Number of items to request in next prefetch.
  int prefetch_amount_ = kMinPrefetchAmount;

  HeapMojoAssociatedRemote<mojom::blink::IDBCursor> remote_;
  Member<IDBRequest> request_;
  const mojom::IDBCursorDirection direction_;
  Member<const Source> source_;
  Member<IDBTransaction> transaction_;
  bool got_value_ = false;
  bool key_dirty_ = true;
  bool primary_key_dirty_ = true;
  bool value_dirty_ = true;
  std::unique_ptr<IDBKey> key_;

  // Owns the cursor's primary key, unless it needs to be injected in the
  // cursor's value. IDBValue's class comment describes when primary key
  // injection occurs.
  //
  // The cursor's primary key should generally be accessed via IdbPrimaryKey(),
  // as it handles both cases correctly.
  std::unique_ptr<IDBKey> primary_key_unless_injected_;
  Member<IDBAny> value_;

#if DCHECK_IS_ON()
  bool value_has_injected_primary_key_ = false;
#endif  // DCHECK_IS_ON()
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_INDEXEDDB_IDB_CURSOR_H_
