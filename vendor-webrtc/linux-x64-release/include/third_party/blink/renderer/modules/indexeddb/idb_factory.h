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

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_INDEXEDDB_IDB_FACTORY_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_INDEXEDDB_IDB_FACTORY_H_

#include <list>
#include <memory>

#include "base/memory/weak_ptr.h"
#include "base/task/single_thread_task_runner.h"
#include "mojo/public/cpp/bindings/pending_associated_remote.h"
#include "mojo/public/cpp/bindings/pending_remote.h"
#include "third_party/blink/public/mojom/indexeddb/indexeddb.mojom-blink-forward.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise_resolver.h"
#include "third_party/blink/renderer/modules/indexeddb/idb_open_db_request.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/heap/weak_cell.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_remote.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

class ExceptionState;
class ScriptState;
class IDBDatabaseInfo;
class IDBFactoryClient;

// This implements the IDBFactory Web IDL interface, i.e. the `window.indexedDB`
// object.
class MODULES_EXPORT IDBFactory final
    : public ScriptWrappable,
      public ExecutionContextLifecycleObserver {
  DEFINE_WRAPPERTYPEINFO();

 public:
  explicit IDBFactory(ExecutionContext* context);
  ~IDBFactory() override;

  void SetRemote(mojo::PendingRemote<mojom::blink::IDBFactory>);

  // Implement the IDBFactory IDL
  IDBOpenDBRequest* open(ScriptState*, const String& name, ExceptionState&);
  IDBOpenDBRequest* open(ScriptState*,
                         const String& name,
                         uint64_t version,
                         ExceptionState&);
  IDBOpenDBRequest* deleteDatabase(ScriptState*,
                                   const String& name,
                                   ExceptionState&);
  int16_t cmp(ScriptState*,
              const ScriptValue& first,
              const ScriptValue& second,
              ExceptionState&);

  // These are not exposed to the web applications and only used by DevTools.
  IDBOpenDBRequest* CloseConnectionsAndDeleteDatabase(ScriptState*,
                                                      const String& name,
                                                      ExceptionState&);

  ScriptPromise<IDLSequence<IDBDatabaseInfo>> GetDatabaseInfo(ScriptState*,
                                                              ExceptionState&);

  // This method is exposed specifically for DevTools.
  void GetDatabaseInfoForDevTools(
      mojom::blink::IDBFactory::GetDatabaseInfoCallback callback);

  // ExecutionContextLifecycleObserver
  void ContextDestroyed() override;

  void Trace(Visitor*) const override;

 private:
  ExecutionContext* GetValidContext(ScriptState* script_state);

  // Initializes and returns the mojo pipe to the back end.
  HeapMojoRemote<mojom::blink::IDBFactory>& GetRemote();

  scoped_refptr<base::SingleThreadTaskRunner> GetTaskRunner();

  IDBOpenDBRequest* OpenInternal(ScriptState*,
                                 const String& name,
                                 int64_t version,
                                 ExceptionState&);
  void OpenInternalImpl(
      IDBOpenDBRequest* request,
      mojo::PendingAssociatedRemote<mojom::blink::IDBDatabaseCallbacks>
          callbacks_remote,
      mojo::PendingAssociatedReceiver<mojom::blink::IDBTransaction>
          transaction_receiver,
      const String& name,
      int64_t version,
      int64_t transaction_id);

  IDBOpenDBRequest* DeleteDatabaseInternal(ScriptState*,
                                           const String& name,
                                           ExceptionState&,
                                           bool);
  void DeleteDatabaseInternalImpl(
      IDBOpenDBRequest* request,
      const String& name,
      bool force_close);

  void GetDatabaseInfoImpl(
      ScriptPromiseResolver<IDLSequence<IDBDatabaseInfo>>*);
  void DidGetDatabaseInfo(
      ScriptPromiseResolver<IDLSequence<IDBDatabaseInfo>>*,
      Vector<mojom::blink::IDBNameAndVersionPtr> names_and_versions,
      mojom::blink::IDBErrorPtr error);

  void GetDatabaseInfoForDevToolsHelper(
      mojom::blink::IDBFactory::GetDatabaseInfoCallback callback);

  void AllowIndexedDB(base::OnceCallback<void()> callback);
  void DidAllowIndexedDB(bool allow_access);

  mojo::PendingAssociatedRemote<mojom::blink::IDBFactoryClient>
  CreatePendingRemote(std::unique_ptr<IDBFactoryClient> client);

  // Whether the context has permission to use IDB.
  std::optional<bool> allowed_;
  // Holds requests that were paused while `allowed_` is being fetched. These
  // will all be invoked in order when `allowed_` is decided.
  Vector<base::OnceClosure> callbacks_waiting_on_permission_decision_;

  HeapMojoRemote<mojom::blink::IDBFactory> remote_;

  WeakCellFactory<IDBFactory> weak_factory_{this};
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_INDEXEDDB_IDB_FACTORY_H_
