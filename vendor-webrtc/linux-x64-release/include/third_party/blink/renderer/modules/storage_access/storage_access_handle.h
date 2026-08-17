// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_STORAGE_ACCESS_STORAGE_ACCESS_HANDLE_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_STORAGE_ACCESS_STORAGE_ACCESS_HANDLE_H_

#include "third_party/blink/renderer/bindings/core/v8/script_promise.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise_resolver.h"
#include "third_party/blink/renderer/bindings/modules/v8/v8_storage_access_types.h"
#include "third_party/blink/renderer/core/frame/local_dom_window.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "third_party/blink/renderer/platform/supplementable.h"

namespace blink {

class Blob;
class BroadcastChannel;
class CacheStorage;
class ExceptionState;
class FileSystemDirectoryHandle;
class IDBFactory;
class LockManager;
class SharedWorker;
class StorageArea;
class StorageEstimate;
class V8UnionSharedWorkerOptionsOrString;

class MODULES_EXPORT StorageAccessHandle final
    : public ScriptWrappable,
      public Supplement<LocalDOMWindow> {
  DEFINE_WRAPPERTYPEINFO();

 public:
  static const char kSupplementName[];
  static const char kSessionStorageNotRequested[];
  static const char kLocalStorageNotRequested[];
  static const char kIndexedDBNotRequested[];
  static const char kLocksNotRequested[];
  static const char kCachesNotRequested[];
  static const char kGetDirectoryNotRequested[];
  static const char kEstimateNotRequested[];
  static const char kCreateObjectURLNotRequested[];
  static const char kRevokeObjectURLNotRequested[];
  static const char kBroadcastChannelNotRequested[];
  static const char kSharedWorkerNotRequested[];

  explicit StorageAccessHandle(LocalDOMWindow& window,
                               const StorageAccessTypes* storage_access_types);
  void Trace(Visitor* visitor) const override;

  StorageArea* sessionStorage(ExceptionState& exception_state) const;
  StorageArea* localStorage(ExceptionState& exception_state) const;
  IDBFactory* indexedDB(ExceptionState& exception_state) const;
  LockManager* locks(ExceptionState& exception_state) const;
  CacheStorage* caches(ExceptionState& exception_state) const;
  ScriptPromise<FileSystemDirectoryHandle> getDirectory(
      ScriptState* script_state,
      ExceptionState& exception_state) const;
  ScriptPromise<StorageEstimate> estimate(
      ScriptState* script_state,
      ExceptionState& exception_state) const;
  String createObjectURL(Blob* blob, ExceptionState& exception_state) const;
  void revokeObjectURL(const String& url,
                       ExceptionState& exception_state) const;
  class BroadcastChannel* BroadcastChannel(
      ExecutionContext* execution_context,
      const String& name,
      ExceptionState& exception_state) const;
  blink::SharedWorker* SharedWorker(
      ExecutionContext* context,
      const String& url,
      const V8UnionSharedWorkerOptionsOrString* name_or_options,
      ExceptionState& exception_state) const;

 private:
  void GetDirectoryImpl(
      ScriptPromiseResolver<FileSystemDirectoryHandle>* resolver) const;

  Member<const StorageAccessTypes> storage_access_types_;
};

namespace bindings {

// Ideally this would have lived in
// third_party/blink/renderer/bindings/core/v8/generated_code_helper.h
// but that cannot load code from the modules directory so we must define it
// here.
MODULES_EXPORT ExecutionContext* ExecutionContextFromV8Wrappable(
    const StorageAccessHandle* storage_access_handle);

}  // namespace bindings

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_STORAGE_ACCESS_STORAGE_ACCESS_HANDLE_H_
