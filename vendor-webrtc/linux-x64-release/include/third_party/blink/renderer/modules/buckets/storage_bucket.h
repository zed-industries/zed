// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_BUCKETS_STORAGE_BUCKET_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_BUCKETS_STORAGE_BUCKET_H_

#include "mojo/public/cpp/bindings/pending_remote.h"
#include "mojo/public/cpp/bindings/remote.h"
#include "third_party/blink/public/mojom/buckets/bucket_manager_host.mojom-blink.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise_resolver.h"
#include "third_party/blink/renderer/core/dom/dom_high_res_time_stamp.h"
#include "third_party/blink/renderer/core/execution_context/execution_context.h"
#include "third_party/blink/renderer/core/execution_context/execution_context_lifecycle_observer.h"
#include "third_party/blink/renderer/core/execution_context/navigator_base.h"
#include "third_party/blink/renderer/modules/file_system_access/file_system_directory_handle.h"
#include "third_party/blink/renderer/modules/file_system_access/storage_manager_file_system_access.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_remote.h"

namespace blink {

class CacheStorage;
class FileSystemDirectoryHandle;
class IDBFactory;
class LockManager;
class ScriptState;
class StorageEstimate;
class V8StorageBucketDurability;

class StorageBucket final : public ScriptWrappable,
                            public ExecutionContextClient {
  DEFINE_WRAPPERTYPEINFO();

 public:
  StorageBucket(NavigatorBase* navigator,
                const String& name,
                mojo::PendingRemote<mojom::blink::BucketHost> remote);

  ~StorageBucket() override = default;

  const String& name();
  ScriptPromise<IDLBoolean> persist(ScriptState*);
  ScriptPromise<IDLBoolean> persisted(ScriptState*);
  ScriptPromise<StorageEstimate> estimate(ScriptState*);
  ScriptPromise<V8StorageBucketDurability> durability(ScriptState*);
  ScriptPromise<IDLUndefined> setExpires(ScriptState*,
                                         const DOMHighResTimeStamp&);
  ScriptPromise<IDLNullable<IDLDOMHighResTimeStamp>> expires(ScriptState*);
  IDBFactory* indexedDB();
  LockManager* locks();
  CacheStorage* caches(ExceptionState&);
  ScriptPromise<FileSystemDirectoryHandle> getDirectory(ScriptState*,
                                                        ExceptionState&);

  // An empty `directory_path_components` array will retrieve the root.
  void GetDirectoryForDevTools(
      ExecutionContext* context,
      Vector<String> directory_path_components,
      base::OnceCallback<void(mojom::blink::FileSystemAccessErrorPtr,
                              FileSystemDirectoryHandle*)> callback);

  // GarbageCollected
  void Trace(Visitor*) const override;

 private:
  void DidRequestPersist(ScriptPromiseResolver<IDLBoolean>* resolver,
                         bool persisted,
                         bool success);
  void DidGetPersisted(ScriptPromiseResolver<IDLBoolean>* resolver,
                       bool persisted,
                       bool success);
  void DidGetEstimate(ScriptPromiseResolver<StorageEstimate>*,
                      int64_t current_usage,
                      int64_t current_quota,
                      bool success);
  void DidGetDurability(
      ScriptPromiseResolver<V8StorageBucketDurability>* resolver,
      mojom::blink::BucketDurability durability,
      bool success);
  void DidSetExpires(ScriptPromiseResolver<IDLUndefined>*, bool success);
  void DidGetExpires(
      ScriptPromiseResolver<IDLNullable<IDLDOMHighResTimeStamp>>* resolver,
      const std::optional<base::Time> expires,
      bool success);
  void GetSandboxedFileSystem(
      ScriptPromiseResolver<FileSystemDirectoryHandle>* resolver);
  void GetSandboxedFileSystemForDevtools(
      ExecutionContext* context,
      const Vector<String>& directory_path_components,
      base::OnceCallback<void(mojom::blink::FileSystemAccessErrorPtr,
                              FileSystemDirectoryHandle*)> callback,
      mojom::blink::FileSystemAccessErrorPtr result);

  String name_;

  // BucketHost in the browser process.
  HeapMojoRemote<mojom::blink::BucketHost> remote_;

  Member<IDBFactory> idb_factory_;
  Member<LockManager> lock_manager_;
  Member<CacheStorage> caches_;
  Member<NavigatorBase> navigator_base_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_BUCKETS_STORAGE_BUCKET_H_
