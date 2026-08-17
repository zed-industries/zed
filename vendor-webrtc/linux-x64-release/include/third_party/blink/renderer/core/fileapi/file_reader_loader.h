/*
 * Copyright (C) 2010 Google Inc.  All rights reserved.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_FILEAPI_FILE_READER_LOADER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_FILEAPI_FILE_READER_LOADER_H_

#include "base/dcheck_is_on.h"
#include "base/memory/weak_ptr.h"
#include "base/task/single_thread_task_runner.h"
#include "third_party/blink/public/mojom/blob/blob.mojom-blink.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/execution_context/execution_context.h"
#include "third_party/blink/renderer/core/fileapi/file_error.h"
#include "third_party/blink/renderer/core/fileapi/file_reader_client.h"
#include "third_party/blink/renderer/core/typed_arrays/array_buffer/array_buffer_contents.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "third_party/blink/renderer/platform/heap/persistent.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_receiver.h"
#include "third_party/blink/renderer/platform/wtf/forward.h"
#include "third_party/blink/renderer/platform/wtf/gc_plugin.h"

namespace blink {

class BlobDataHandle;
// Reads a Blob's content and forwards it to the FileReaderClient.
//
// Blobs are typically stored on disk, and should be read asynchronously
// whenever possible. Synchronous loading is implemented to support Web Platform
// features that we cannot (yet) remove, such as FileReaderSync and synchronous
// XMLHttpRequest.
//
// Each FileReaderLoader instance is only good for reading one Blob, and will
// leak resources if used multiple times.
class CORE_EXPORT FileReaderLoader : public GarbageCollected<FileReaderLoader>,
                                     public mojom::blink::BlobReaderClient {
 public:
  // If client is given, do the loading asynchronously. Otherwise, load
  // synchronously.
  FileReaderLoader(FileReaderClient*,
                   scoped_refptr<base::SingleThreadTaskRunner>);
  ~FileReaderLoader() override;

  void Start(scoped_refptr<BlobDataHandle>);
  void StartSync(scoped_refptr<BlobDataHandle>);
  void Cancel();

  // Returns the total bytes received. Bytes ignored by m_rawData won't be
  // counted.
  //
  // This value doesn't grow more than numeric_limits<unsigned> when
  // m_readType is not set to ReadByClient.
  uint64_t BytesLoaded() const { return bytes_loaded_; }

  // Before OnCalculatedSize() is called: Returns nullopt.
  // After OnCalculatedSize() is called: Returns the size of the resource.
  std::optional<uint64_t> TotalBytes() const { return total_bytes_; }

  FileErrorCode GetErrorCode() const { return error_code_; }

  int32_t GetNetError() const { return net_error_; }

  bool HasFinishedLoading() const { return finished_loading_; }

  void Trace(Visitor* visitor) const {
    visitor->Trace(client_);
    visitor->Trace(receiver_);
  }

 private:
  void StartInternal(scoped_refptr<BlobDataHandle>, bool is_sync);

  // These values are persisted to logs. Entries should not be renumbered and
  // numeric values should never be reused.
  enum class FailureType {
    kMojoPipeCreation = 0,
    kSyncDataNotAllLoaded = 1,
    kSyncOnCompleteNotReceived = 2,
    kBackendReadError = 3,
    kReadSizesIncorrect = 4,
    kDataPipeNotReadableWithBytesLeft = 5,
    kMojoPipeClosedEarly = 6,
    // Any MojoResult error we aren't expecting during data pipe reading falls
    // into this bucket. If there are a large number of errors reported here,
    // then there can be a new enumeration reported for mojo pipe errors.
    kMojoPipeUnexpectedReadError = 7,
    kClientFailure = 8,
    kMaxValue = kClientFailure,
  };

  void Cleanup();
  void Failed(FileErrorCode, FailureType type);

  bool IsSyncLoad() { return is_sync_; }

  void OnFinishLoading();

  // BlobReaderClient:
  void OnCalculatedSize(uint64_t total_size,
                        uint64_t expected_content_size) override;
  void OnComplete(int32_t status, uint64_t data_length) override;
  void OnDataPipeReadable(MojoResult);

  Member<FileReaderClient> client_;

  bool finished_loading_ = false;
  uint64_t bytes_loaded_ = 0;
  // total_bytes_ is set to the total size of the blob being loaded as soon as
  // it is known, and  the buffer for receiving data of total_bytes_ is
  // allocated and never grow even when extra data is appended.
  std::optional<uint64_t> total_bytes_;

  int32_t net_error_ = 0;  // net::OK
  FileErrorCode error_code_ = FileErrorCode::kOK;

  mojo::ScopedDataPipeConsumerHandle consumer_handle_;
  mojo::SimpleWatcher handle_watcher_;
  HeapMojoReceiver<mojom::blink::BlobReaderClient, FileReaderLoader> receiver_{
      this, nullptr};
  bool received_all_data_ = false;
  bool received_on_complete_ = false;
#if DCHECK_IS_ON()
  bool started_loading_ = false;
#endif  // DCHECK_IS_ON()

  bool is_sync_ = false;

  scoped_refptr<base::SingleThreadTaskRunner> task_runner_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_FILEAPI_FILE_READER_LOADER_H_
