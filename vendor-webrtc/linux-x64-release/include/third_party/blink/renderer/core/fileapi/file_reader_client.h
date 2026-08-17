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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_FILEAPI_FILE_READER_CLIENT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_FILEAPI_FILE_READER_CLIENT_H_

#include "base/task/single_thread_task_runner.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/fileapi/file_error.h"
#include "third_party/blink/renderer/core/fileapi/file_reader_data.h"
#include "third_party/blink/renderer/core/typed_arrays/array_buffer/array_buffer_contents.h"
#include "third_party/blink/renderer/platform/blob/blob_data.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"

namespace blink {

enum class FileErrorCode;

// FileReaderClient is used by the FileReaderLoader to read a given blob's raw
// data. For a more convenient way to read blobs that matches the different
// FileReadType's, see FileReaderAccumulator below.
// For more information on how to read Blobs in your specific situation, see:
// https://chromium.googlesource.com/chromium/src/+/HEAD/storage/browser/blob/README.md#how-to-use-blobs-blink-accessing-reading
class CORE_EXPORT FileReaderClient : public GarbageCollectedMixin {
 public:
  virtual ~FileReaderClient() = default;

  // Clients must implement this method to be informed about when the loading
  // actually started.
  // If an error occurred while processing the data, a FileErrorCode can be
  // returned. In such case, the blob's processing will end a DidFail will be
  // called with the returned error.
  // Clients must not make re-entrant calls to the FileReaderLoader in this
  // method.
  virtual FileErrorCode DidStartLoading(uint64_t total_size) = 0;
  // Clients must implement this method to receive the blob's data.
  // If an error occurred while processing the data, a FileErrorCode can be
  // returned. In such case, the blob's processing will end a DidFail will be
  // called with the returned error.
  // Clients must not make re-entrant calls to the FileReaderLoader in this
  // method.
  virtual FileErrorCode DidReceiveData(base::span<const uint8_t> data) = 0;
  // Clients must implement this method to be informed about when the loading
  // ended.
  virtual void DidFinishLoading() = 0;
  // Clients must implement this method to be informed about any failures that
  // occurred.
  virtual void DidFail(FileErrorCode) = 0;

  void Trace(Visitor*) const override {}
};

// FileReaderAccumulator helps aggregating the data received from
// FileReaderLoader. This class implements a regular FileReaderClient and
// reads the file' chunks until completion. On completion, it provides a
// convenient way to convert the received data to the FileReadType types via
// the FileReaderData class helper.
class CORE_EXPORT FileReaderAccumulator : public FileReaderClient {
 public:
  // Clients might implement this method if they want to know when the loading
  // process started. The client can also choose the loader behaviour by
  // returning whether to continue or stop the loader.
  virtual FileErrorCode DidStartLoading() { return FileErrorCode::kOK; }
  // Clients might implement this method if they want to know when the loader
  // received some data. The client can also choose the loader behaviour by
  // returning whether to continue or stop the loader.
  virtual FileErrorCode DidReceiveData() { return FileErrorCode::kOK; }
  // Clients must implement this method to receive the aggregated file data.
  virtual void DidFinishLoading(FileReaderData) = 0;
  // Clients might implement this method to known whether the loading failed.
  // If this method is implemented, it is recommended to call the parent
  // implementation as well for early cleanup.
  void DidFail(FileErrorCode) override;
  void Trace(Visitor* visitor) const override {
    FileReaderClient::Trace(visitor);
  }

 private:
  FileErrorCode DidStartLoading(uint64_t total_bytes) final;
  FileErrorCode DidReceiveData(base::span<const uint8_t> data) final;
  void DidFinishLoading() final;

  uint64_t bytes_loaded_ = 0;
  ArrayBufferContents raw_data_;
};

// The class acts similar to the FileReaderAccumulator, except that it
// works synchronously. This provides a few handy methods to help wait for the
// aggregated result.
class CORE_EXPORT SyncedFileReaderAccumulator
    : public GarbageCollected<SyncedFileReaderAccumulator>,
      public FileReaderAccumulator {
 public:
  static std::pair<FileErrorCode, FileReaderData> Load(
      scoped_refptr<BlobDataHandle> handle,
      scoped_refptr<base::SingleThreadTaskRunner>);

  void Trace(Visitor* visitor) const override {
    FileReaderAccumulator::Trace(visitor);
  }

 private:
  void DidFail(FileErrorCode error_code) final;
  void DidFinishLoading(FileReaderData obj) final;

  FileReaderData stored_;
  FileErrorCode error_code_ = FileErrorCode::kOK;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_FILEAPI_FILE_READER_CLIENT_H_
