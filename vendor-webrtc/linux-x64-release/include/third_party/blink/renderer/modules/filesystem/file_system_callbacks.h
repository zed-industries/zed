/*
 * Copyright (C) 2010 Google Inc. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are
 * met:
 *
 *     * Redistributions of source code must retain the above copyright
 * notice, this list of conditions and the following disclaimer.
 *     * Redistributions in binary form must reproduce the above
 * copyright notice, this list of conditions and the following disclaimer
 * in the documentation and/or other materials provided with the
 * distribution.
 *     * Neither the name of Google Inc. nor the names of its
 * contributors may be used to endorse or promote products derived from
 * this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 * "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 * LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
 * A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
 * OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 * SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
 * LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
 * DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
 * THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_FILESYSTEM_FILE_SYSTEM_CALLBACKS_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_FILESYSTEM_FILE_SYSTEM_CALLBACKS_H_

#include "third_party/blink/public/mojom/filesystem/file_system.mojom-blink-forward.h"
#include "third_party/blink/renderer/bindings/core/v8/v8_void_callback.h"
#include "third_party/blink/renderer/bindings/modules/v8/v8_entry_callback.h"
#include "third_party/blink/renderer/bindings/modules/v8/v8_error_callback.h"
#include "third_party/blink/renderer/bindings/modules/v8/v8_file_callback.h"
#include "third_party/blink/renderer/bindings/modules/v8/v8_file_system_callback.h"
#include "third_party/blink/renderer/bindings/modules/v8/v8_file_writer_callback.h"
#include "third_party/blink/renderer/bindings/modules/v8/v8_metadata_callback.h"
#include "third_party/blink/renderer/core/fileapi/file_error.h"
#include "third_party/blink/renderer/modules/filesystem/entry_heap_vector.h"
#include "third_party/blink/renderer/platform/blob/blob_data.h"
#include "third_party/blink/renderer/platform/file_metadata.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_vector.h"
#include "third_party/blink/renderer/platform/heap/persistent.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {

class DOMFileSystem;
class DOMFileSystemBase;
class DirectoryReaderBase;
class Entry;
class ExecutionContext;
class File;
class FileMetadata;
class FileWriterBase;
class Metadata;

class FileSystemCallbacksBase {
 public:
  virtual ~FileSystemCallbacksBase();

 protected:
  FileSystemCallbacksBase(DOMFileSystemBase*, ExecutionContext*);

  Persistent<DOMFileSystemBase> file_system_;
  Persistent<ExecutionContext> execution_context_;
  int async_operation_id_;
};

// This is a base class for the SnapshotFileCallback and CreateFileHelper.
// Both implement snapshot file operations.
class SnapshotFileCallbackBase {
 public:
  virtual ~SnapshotFileCallbackBase() = default;

  // Called when a snapshot file is created successfully.
  virtual void DidCreateSnapshotFile(const FileMetadata&) = 0;

  virtual void DidFail(base::File::Error error) = 0;
};

// Subclasses ----------------------------------------------------------------

class EntryCallbacks final : public FileSystemCallbacksBase {
 public:
  using SuccessCallback = base::OnceCallback<void(Entry*)>;
  using ErrorCallback = base::OnceCallback<void(base::File::Error)>;

  EntryCallbacks(SuccessCallback,
                 ErrorCallback,
                 ExecutionContext*,
                 DOMFileSystemBase*,
                 const String& expected_path,
                 bool is_directory);

  // Called when a requested operation is completed successfully.
  void DidSucceed();

  // Called when a request operation has failed.
  void DidFail(base::File::Error error);

 private:
  SuccessCallback success_callback_;
  ErrorCallback error_callback_;
  String expected_path_;
  bool is_directory_;
};

class EntriesCallbacks final : public FileSystemCallbacksBase {
 public:
  using SuccessCallback = base::RepeatingCallback<void(GCedEntryHeapVector*)>;
  using ErrorCallback = base::OnceCallback<void(base::File::Error)>;

  EntriesCallbacks(const SuccessCallback&,
                   ErrorCallback,
                   ExecutionContext*,
                   DirectoryReaderBase*,
                   const String& base_path);

  // Called when a directory entry is read.
  void DidReadDirectoryEntry(const String& name, bool is_directory);

  // Called after a chunk of directory entries have been read (i.e. indicates
  // it's good time to call back to the application). If hasMore is true there
  // can be more chunks.
  void DidReadDirectoryEntries(bool has_more);

  // Called when a request operation has failed.
  void DidFail(base::File::Error error);

 private:
  SuccessCallback success_callback_;
  ErrorCallback error_callback_;
  Persistent<DirectoryReaderBase> directory_reader_;
  String base_path_;
  Persistent<GCedEntryHeapVector> entries_;
};

class FileSystemCallbacks final : public FileSystemCallbacksBase {
 public:
  using SuccessCallback = base::OnceCallback<void(DOMFileSystem*)>;
  using ErrorCallback = base::OnceCallback<void(base::File::Error)>;

  FileSystemCallbacks(SuccessCallback,
                      ErrorCallback,
                      ExecutionContext*,
                      mojom::blink::FileSystemType);

  // Called when a requested file system is opened.
  void DidOpenFileSystem(const String& name, const KURL& root_url);

  // Called when a request operation has failed.
  void DidFail(base::File::Error error);

 private:
  SuccessCallback success_callback_;
  ErrorCallback error_callback_;
  mojom::blink::FileSystemType type_;
};

class ResolveURICallbacks final : public FileSystemCallbacksBase {
 public:
  using SuccessCallback = EntryCallbacks::SuccessCallback;
  using ErrorCallback = EntryCallbacks::ErrorCallback;

  ResolveURICallbacks(SuccessCallback, ErrorCallback, ExecutionContext*);

  // Called when a filesystem URL is resolved.
  void DidResolveURL(const String& name,
                     const KURL& root_url,
                     mojom::blink::FileSystemType,
                     const String& file_path,
                     bool is_directry);

  // Called when a request operation has failed.
  void DidFail(base::File::Error error);

 private:
  SuccessCallback success_callback_;
  ErrorCallback error_callback_;
};

class MetadataCallbacks final : public FileSystemCallbacksBase {
 public:
  using SuccessCallback = base::OnceCallback<void(Metadata* metadata)>;
  using ErrorCallback = base::OnceCallback<void(base::File::Error error)>;

  MetadataCallbacks(SuccessCallback,
                    ErrorCallback,
                    ExecutionContext*,
                    DOMFileSystemBase*);

  // Called when a file metadata is read successfully.
  void DidReadMetadata(const FileMetadata&);

  // Called when a request operation has failed.
  void DidFail(base::File::Error error);

 private:
  SuccessCallback success_callback_;
  ErrorCallback error_callback_;
};

class FileWriterCallbacks final : public FileSystemCallbacksBase {
 public:
  using SuccessCallback = base::OnceCallback<void(FileWriterBase* file_writer)>;
  using ErrorCallback = base::OnceCallback<void(base::File::Error error)>;

  FileWriterCallbacks(FileWriterBase*,
                      SuccessCallback,
                      ErrorCallback,
                      ExecutionContext*);

  // Called when an AsyncFileWrter has been created successfully.
  void DidCreateFileWriter(const KURL& path, int64_t length);

  // Called when a request operation has failed.
  void DidFail(base::File::Error error);

 private:
  Persistent<FileWriterBase> file_writer_;
  SuccessCallback success_callback_;
  ErrorCallback error_callback_;
};

class SnapshotFileCallback final : public SnapshotFileCallbackBase,
                                   public FileSystemCallbacksBase {
 public:
  using SuccessCallback = base::OnceCallback<void(File* file)>;
  using ErrorCallback = base::OnceCallback<void(base::File::Error error)>;

  SnapshotFileCallback(DOMFileSystemBase*,
                       const String& name,
                       const KURL&,
                       SuccessCallback,
                       ErrorCallback,
                       ExecutionContext*);

  // Called when a snapshot file is created successfully.
  void DidCreateSnapshotFile(const FileMetadata&) override;

  // Called when a request operation has failed.
  void DidFail(base::File::Error error) override;

 private:
  String name_;
  KURL url_;
  SuccessCallback success_callback_;
  ErrorCallback error_callback_;
};

class VoidCallbacks final : public FileSystemCallbacksBase {
 public:
  using SuccessCallback = base::OnceCallback<void()>;
  using ErrorCallback = base::OnceCallback<void(base::File::Error error)>;

  VoidCallbacks(SuccessCallback,
                ErrorCallback,
                ExecutionContext*,
                DOMFileSystemBase*);

  // Called when a requested operation is completed successfully.
  void DidSucceed();

  // Called when a request operation has failed.
  void DidFail(base::File::Error error);

 private:
  SuccessCallback success_callback_;
  ErrorCallback error_callback_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_FILESYSTEM_FILE_SYSTEM_CALLBACKS_H_
