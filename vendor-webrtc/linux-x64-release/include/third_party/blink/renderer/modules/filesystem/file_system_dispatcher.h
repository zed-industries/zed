// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_FILESYSTEM_FILE_SYSTEM_DISPATCHER_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_FILESYSTEM_FILE_SYSTEM_DISPATCHER_H_

#include <memory>

#include "mojo/public/cpp/bindings/pending_remote.h"
#include "third_party/blink/public/mojom/filesystem/file_system.mojom-blink.h"
#include "third_party/blink/renderer/modules/filesystem/file_system_callbacks.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_map.h"
#include "third_party/blink/renderer/platform/heap/disallow_new_wrapper.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_remote.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_unique_receiver_set.h"
#include "third_party/blink/renderer/platform/supplementable.h"

namespace WTF {
class String;
}

namespace blink {

class Blob;
class ExecutionContext;
class KURL;
class SecurityOrigin;

// Sends messages via mojo to the blink::mojom::FileSystemManager service
// running in the browser process. It is owned by ExecutionContext, and
// instances are created lazily by calling FileSystemDispatcher::From().
class FileSystemDispatcher : public GarbageCollected<FileSystemDispatcher>,
                             public Supplement<ExecutionContext> {
 public:
  using StatusCallback = base::OnceCallback<void(base::File::Error error)>;
  using WriteCallback =
      base::RepeatingCallback<void(int64_t bytes, bool complete)>;

  static const char kSupplementName[];

  static FileSystemDispatcher& From(ExecutionContext* context);

  explicit FileSystemDispatcher(ExecutionContext& context);
  virtual ~FileSystemDispatcher();

  mojom::blink::FileSystemManager& GetFileSystemManager();

  void OpenFileSystem(const SecurityOrigin* origin,
                      mojom::blink::FileSystemType type,
                      std::unique_ptr<FileSystemCallbacks> callbacks);
  void OpenFileSystemSync(const SecurityOrigin* origin,
                          mojom::blink::FileSystemType type,
                          std::unique_ptr<FileSystemCallbacks> callbacks);

  void ResolveURL(const KURL& filesystem_url,
                  std::unique_ptr<ResolveURICallbacks> callbacks);
  void ResolveURLSync(const KURL& filesystem_url,
                      std::unique_ptr<ResolveURICallbacks> callbacks);

  void Move(const KURL& src_path,
            const KURL& dest_path,
            std::unique_ptr<EntryCallbacks> callbacks);
  void MoveSync(const KURL& src_path,
                const KURL& dest_path,
                std::unique_ptr<EntryCallbacks> callbacks);

  void Copy(const KURL& src_path,
            const KURL& dest_path,
            std::unique_ptr<EntryCallbacks> callbacks);
  void CopySync(const KURL& src_path,
                const KURL& dest_path,
                std::unique_ptr<EntryCallbacks> callbacks);

  void Remove(const KURL& path,
              bool recursive,
              std::unique_ptr<VoidCallbacks> callbacks);
  void RemoveSync(const KURL& path,
                  bool recursive,
                  std::unique_ptr<VoidCallbacks> callbacks);

  void ReadMetadata(const KURL& path,
                    std::unique_ptr<MetadataCallbacks> callbacks);
  void ReadMetadataSync(const KURL& path,
                        std::unique_ptr<MetadataCallbacks> callbacks);

  void CreateFile(const KURL& path,
                  bool exclusive,
                  std::unique_ptr<EntryCallbacks> callbacks);
  void CreateFileSync(const KURL& path,
                      bool exclusive,
                      std::unique_ptr<EntryCallbacks> callbacks);

  void CreateDirectory(const KURL& path,
                       bool exclusive,
                       bool recursive,
                       std::unique_ptr<EntryCallbacks> callbacks);
  void CreateDirectorySync(const KURL& path,
                           bool exclusive,
                           bool recursive,
                           std::unique_ptr<EntryCallbacks> callbacks);

  void Exists(const KURL& path,
              bool for_directory,
              std::unique_ptr<EntryCallbacks> callbacks);
  void ExistsSync(const KURL& path,
                  bool for_directory,
                  std::unique_ptr<EntryCallbacks> callbacks);

  void ReadDirectory(const KURL& path,
                     std::unique_ptr<EntriesCallbacks> callbacks);
  void ReadDirectorySync(const KURL& path,
                         std::unique_ptr<EntriesCallbacks> callbacks);

  void InitializeFileWriter(const KURL& path,
                            std::unique_ptr<FileWriterCallbacks>);
  void InitializeFileWriterSync(const KURL& path,
                                std::unique_ptr<FileWriterCallbacks>);

  void Truncate(const KURL& path,
                int64_t offset,
                int* request_id_out,
                StatusCallback callback);
  void TruncateSync(const KURL& path, int64_t offset, StatusCallback callback);

  void Write(const KURL& path,
             const Blob& blob,
             int64_t offset,
             int* request_id_out,
             const WriteCallback& success_callback,
             StatusCallback error_callback);
  void WriteSync(const KURL& path,
                 const Blob& blob,
                 int64_t offset,
                 const WriteCallback& success_callback,
                 StatusCallback error_callback);

  void Cancel(int request_id_to_cancel, StatusCallback callback);

  void CreateSnapshotFile(const KURL& file_path,
                          std::unique_ptr<SnapshotFileCallbackBase> callbacks);
  void CreateSnapshotFileSync(
      const KURL& file_path,
      std::unique_ptr<SnapshotFileCallbackBase> callbacks);

  void Trace(Visitor*) const override;

 private:
  class WriteListener;
  class ReadDirectoryListener;

  void DidOpenFileSystem(std::unique_ptr<FileSystemCallbacks> callbacks,
                         const String& name,
                         const KURL& root,
                         base::File::Error error_code);
  void DidResolveURL(std::unique_ptr<ResolveURICallbacks> callbacks,
                     mojom::blink::FileSystemInfoPtr info,
                     const base::FilePath& file_path,
                     bool is_directory,
                     base::File::Error error_code);
  void DidRemove(std::unique_ptr<VoidCallbacks> callbacks,
                 base::File::Error error_code);
  void DidFinish(std::unique_ptr<EntryCallbacks> callbacks,
                 base::File::Error error_code);
  void DidReadMetadata(std::unique_ptr<MetadataCallbacks> callbacks,
                       const base::File::Info& file_info,
                       base::File::Error error);
  void DidReadDirectory(
      std::unique_ptr<EntriesCallbacks> callbacks,
      Vector<filesystem::mojom::blink::DirectoryEntryPtr> entries,
      base::File::Error error_code);
  void InitializeFileWriterCallback(
      const KURL& path,
      std::unique_ptr<FileWriterCallbacks> callbacks,
      const base::File::Info& file_info,
      base::File::Error error);
  void DidTruncate(int operation_id,
                   StatusCallback callback,
                   base::File::Error error_code);
  void DidWrite(const WriteCallback& callback,
                int operation_id,
                int64_t bytes,
                bool complete);
  void WriteErrorCallback(StatusCallback callback,
                          int operation_id,
                          base::File::Error error);
  void DidCancel(StatusCallback callback,
                 int cancelled_operation_id,
                 base::File::Error error_code);
  void DidCreateSnapshotFile(
      std::unique_ptr<SnapshotFileCallbackBase> callbacks,
      const base::File::Info& file_info,
      const base::FilePath& platform_path,
      base::File::Error error_code,
      mojo::PendingRemote<mojom::blink::ReceivedSnapshotListener> listener);

  void RemoveOperationRemote(int operation_id);

  HeapMojoRemote<mojom::blink::FileSystemManager> file_system_manager_;
  using OperationsMap =
      HeapHashMap<int,
                  Member<DisallowNewWrapper<HeapMojoRemote<
                      mojom::blink::FileSystemCancellableOperation>>>>;
  OperationsMap cancellable_operations_;
  int next_operation_id_;
  HeapMojoUniqueReceiverSet<mojom::blink::FileSystemOperationListener>
      op_listeners_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_FILESYSTEM_FILE_SYSTEM_DISPATCHER_H_
