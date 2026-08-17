// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_FILE_SYSTEM_ACCESS_FILE_SYSTEM_FILE_HANDLE_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_FILE_SYSTEM_ACCESS_FILE_SYSTEM_FILE_HANDLE_H_

#include "mojo/public/cpp/bindings/pending_remote.h"
#include "third_party/blink/public/mojom/file_system_access/file_system_access_file_handle.mojom-blink.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise_resolver.h"
#include "third_party/blink/renderer/modules/file_system_access/file_system_handle.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_remote.h"
#include "third_party/blink/renderer/platform/wtf/casting.h"

namespace blink {
class File;
class FileSystemCreateWritableOptions;
class FileSystemCreateSyncAccessHandleOptions;
class FileSystemSyncAccessHandle;
class FileSystemWritableFileStream;

class FileSystemFileHandle final : public FileSystemHandle {
  DEFINE_WRAPPERTYPEINFO();

 public:
  FileSystemFileHandle(
      ExecutionContext* context,
      const String& name,
      mojo::PendingRemote<mojom::blink::FileSystemAccessFileHandle>);

  bool isFile() const override { return true; }

  ScriptPromise<FileSystemWritableFileStream> createWritable(
      ScriptState*,
      const FileSystemCreateWritableOptions* options,
      ExceptionState&);
  ScriptPromise<File> getFile(ScriptState*, ExceptionState&);

  // TODO(fivedots): Define if this method should be generally exposed or only
  // on files backed by the Origin Private File System.
  ScriptPromise<FileSystemSyncAccessHandle> createSyncAccessHandle(
      ScriptState*,
      ExceptionState&);
  ScriptPromise<FileSystemSyncAccessHandle> createSyncAccessHandle(
      ScriptState*,
      const FileSystemCreateSyncAccessHandleOptions* options,
      ExceptionState&);

  mojo::PendingRemote<mojom::blink::FileSystemAccessTransferToken> Transfer()
      override;

  mojom::blink::FileSystemAccessFileHandle* MojoHandle() {
    return mojo_ptr_.get();
  }

  void Trace(Visitor*) const override;

 private:
  void QueryPermissionImpl(
      bool writable,
      base::OnceCallback<void(mojom::blink::PermissionStatus)>) override;
  void RequestPermissionImpl(
      bool writable,
      base::OnceCallback<void(mojom::blink::FileSystemAccessErrorPtr,
                              mojom::blink::PermissionStatus)>) override;
  void MoveImpl(
      mojo::PendingRemote<mojom::blink::FileSystemAccessTransferToken> dest,
      const String& new_entry_name,
      base::OnceCallback<void(mojom::blink::FileSystemAccessErrorPtr)>)
      override;
  void RemoveImpl(
      const FileSystemRemoveOptions* options,
      base::OnceCallback<void(mojom::blink::FileSystemAccessErrorPtr)>)
      override;
  void IsSameEntryImpl(
      mojo::PendingRemote<mojom::blink::FileSystemAccessTransferToken> other,
      base::OnceCallback<void(mojom::blink::FileSystemAccessErrorPtr, bool)>)
      override;
  void GetUniqueIdImpl(
      base::OnceCallback<void(mojom::blink::FileSystemAccessErrorPtr,
                              const WTF::String&)>) override;
  void GetCloudIdentifiersImpl(
      base::OnceCallback<void(
          mojom::blink::FileSystemAccessErrorPtr,
          Vector<mojom::blink::FileSystemAccessCloudIdentifierPtr>)>) override;

  void CreateSyncAccessHandleImpl(
      const FileSystemCreateSyncAccessHandleOptions* options,
      ScriptPromiseResolver<FileSystemSyncAccessHandle>* resolver);

  // Callback for StorageManagerFileSystemAccess::CheckGetDirectoryIsAllowed.
  void OnGotFileSystemStorageAccessStatus(
      ScriptPromiseResolver<FileSystemSyncAccessHandle>* resolver,
      base::OnceClosure on_allowed_callback,
      mojom::blink::FileSystemAccessErrorPtr result);

  HeapMojoRemote<mojom::blink::FileSystemAccessFileHandle> mojo_ptr_;
  std::optional<std::tuple</*status=*/mojom::blink::FileSystemAccessStatus,
                           /*file_error=*/::base::File::Error,
                           /*message=*/WTF::String>>
      storage_access_status_;
};

template <>
struct DowncastTraits<FileSystemFileHandle> {
  static bool AllowFrom(const FileSystemHandle& handle) {
    return handle.isFile();
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_FILE_SYSTEM_ACCESS_FILE_SYSTEM_FILE_HANDLE_H_
