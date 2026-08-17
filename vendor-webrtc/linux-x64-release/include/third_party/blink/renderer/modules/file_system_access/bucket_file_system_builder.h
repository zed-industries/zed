// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_FILE_SYSTEM_ACCESS_BUCKET_FILE_SYSTEM_BUILDER_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_FILE_SYSTEM_ACCESS_BUCKET_FILE_SYSTEM_BUILDER_H_

#include <deque>

#include "third_party/blink/public/mojom/file_system_access/file_system_access_error.mojom-blink.h"
#include "third_party/blink/renderer/core/execution_context/execution_context.h"
#include "third_party/blink/renderer/core/inspector/protocol/file_system.h"
#include "third_party/blink/renderer/modules/file_system_access/file_system_directory_handle.h"
#include "third_party/blink/renderer/modules/file_system_access/file_system_file_handle.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_receiver.h"

namespace blink {

// This callback is used to pass the constructed
// `protocol::FileSystem::Directory` from a `FileSystemDirectoryHandle`.
using DirectoryCallback =
    base::OnceCallback<void(mojom::blink::FileSystemAccessErrorPtr,
                            std::unique_ptr<protocol::FileSystem::Directory>)>;

// This class receives a directory or file handle and builds a corresponding
// `protocol::FileSystem::Directory` or `protocol::FileSystem::File` object.
// This class is instantiated by the BuildDirectory method.
class MODULES_EXPORT BucketFileSystemBuilder final
    : public GarbageCollected<BucketFileSystemBuilder>,
      public ExecutionContextClient,
      public mojom::blink::FileSystemAccessDirectoryEntriesListener {
 public:
  // This method takes a directory handle and returns the complete nested files
  // and directory tree. It recursively calls itself when one of its children is
  // a directory. It's the entry point of this class.
  static void BuildDirectoryTree(ExecutionContext* execution_context,
                                 const String storage_key,
                                 const String name,
                                 DirectoryCallback callback,
                                 FileSystemDirectoryHandle* handle);

  // This should never be called directly. Instead, call `BuildDirectoryTree`.
  BucketFileSystemBuilder(ExecutionContext* execution_context,
                          const String storage_key,
                          const String name,
                          DirectoryCallback completion_callback);

  // mojom::blink::FileSystemAccessDirectoryEntriesListener
  void DidReadDirectory(mojom::blink::FileSystemAccessErrorPtr result,
                        Vector<mojom::blink::FileSystemAccessEntryPtr> entries,
                        bool has_more_entries) override;

  mojo::PendingRemote<mojom::blink::FileSystemAccessDirectoryEntriesListener>
  GetListener();

  // GarbageCollected
  void Trace(Visitor* visitor) const override;

 private:
  // This method is called for each file seen to track the `nested_files`
  // encountered.
  void DidBuildFile(base::OnceClosure barrier_callback,
                    mojom::blink::FileSystemAccessErrorPtr result,
                    std::unique_ptr<protocol::FileSystem::File> file);

  // This method is called when all the entries have been iterated on and the
  // `nested_files_` and `nested_directories_` are fully created. It builds the
  // `protocol::FileSystem::Directory` and runs `completion_callback_`.
  void DidBuildDirectory();

  void OnMojoDisconnect();

  const String storage_key_;
  const String directory_name_;
  HeapDeque<Member<FileSystemHandle>> file_system_handle_queue_;
  DirectoryCallback completion_callback_;
  std::unique_ptr<protocol::Array<String>> nested_directories_;
  std::unique_ptr<protocol::Array<protocol::FileSystem::File>> nested_files_;
  HeapMojoReceiver<mojom::blink::FileSystemAccessDirectoryEntriesListener,
                   BucketFileSystemBuilder>
      receiver_;
  SelfKeepAlive<BucketFileSystemBuilder> self_keep_alive_{this};
};
}  // namespace blink
#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_FILE_SYSTEM_ACCESS_BUCKET_FILE_SYSTEM_BUILDER_H_
