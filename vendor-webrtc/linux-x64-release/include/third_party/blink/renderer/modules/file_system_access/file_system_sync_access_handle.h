// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_FILE_SYSTEM_ACCESS_FILE_SYSTEM_SYNC_ACCESS_HANDLE_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_FILE_SYSTEM_ACCESS_FILE_SYSTEM_SYNC_ACCESS_HANDLE_H_

#include "base/sequence_checker.h"
#include "third_party/blink/public/mojom/file_system_access/file_system_access_access_handle_host.mojom-blink.h"
#include "third_party/blink/renderer/bindings/modules/v8/v8_file_system_create_sync_access_handle_options.h"
#include "third_party/blink/renderer/bindings/modules/v8/v8_file_system_read_write_options.h"
#include "third_party/blink/renderer/core/execution_context/execution_context.h"
#include "third_party/blink/renderer/modules/file_system_access/allow_shared_buffer_source_util.h"
#include "third_party/blink/renderer/modules/file_system_access/file_system_access_file_delegate.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_remote.h"

namespace blink {

class FileSystemSyncAccessHandle final : public ScriptWrappable {
  DEFINE_WRAPPERTYPEINFO();

 public:
  explicit FileSystemSyncAccessHandle(
      ExecutionContext* context,
      FileSystemAccessFileDelegate* file_delegate,
      mojo::PendingRemote<mojom::blink::FileSystemAccessAccessHandleHost>
          access_handle_host,
      V8FileSystemSyncAccessHandleMode lock_mode);

  FileSystemSyncAccessHandle(const FileSystemSyncAccessHandle&) = delete;
  FileSystemSyncAccessHandle& operator=(const FileSystemSyncAccessHandle&) =
      delete;

  // GarbageCollected
  void Trace(Visitor* visitor) const override;

  void close();

  void flush(ExceptionState&);

  uint64_t getSize(ExceptionState&);

  void truncate(uint64_t size, ExceptionState&);

  uint64_t read(const AllowSharedBufferSource* buffer,
                FileSystemReadWriteOptions* options,
                ExceptionState&);

  uint64_t write(base::span<const uint8_t> buffer,
                 FileSystemReadWriteOptions* options,
                 ExceptionState&);

  String mode();

 private:
  FileSystemAccessFileDelegate* file_delegate() { return file_delegate_.Get(); }

  SEQUENCE_CHECKER(sequence_checker_);

  // Interface that provides file-like access to the backing storage.
  // The file delegate should only be accessed through the {file_delegate()}
  // getter.
  Member<FileSystemAccessFileDelegate> file_delegate_;

  // Mojo pipe that holds the renderer's lock on the file.
  HeapMojoRemote<mojom::blink::FileSystemAccessAccessHandleHost>
      access_handle_remote_;

  // File position cursor. See
  // https://fs.spec.whatwg.org/#filesystemsyncaccesshandle-file-position-cursor.
  uint64_t cursor_ = 0;

  bool is_closed_ = false;

  const V8FileSystemSyncAccessHandleMode lock_mode_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_FILE_SYSTEM_ACCESS_FILE_SYSTEM_SYNC_ACCESS_HANDLE_H_
