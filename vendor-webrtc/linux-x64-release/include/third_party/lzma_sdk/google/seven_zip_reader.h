// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_LZMA_SDK_GOOGLE_SEVEN_ZIP_READER_H_
#define THIRD_PARTY_LZMA_SDK_GOOGLE_SEVEN_ZIP_READER_H_

#include <stddef.h>
#include <stdint.h>
#include <memory>

#include "base/containers/span.h"
#include "base/files/file.h"
#include "base/files/file_path.h"
#include "base/sequence_checker.h"

namespace seven_zip {

namespace internal {
class SevenZipReaderImpl;
}  // namespace internal

struct EntryInfo {
  // The relative path of this entry, within the archive.
  base::FilePath file_path;

  // The actual size of the entry.
  size_t file_size;

  // The last modified time from the 7z header, if present; or a null time
  // otherwise.
  base::Time last_modified_time;

  // Whether the entry is encrypted.
  bool is_encrypted;
};

enum class Result {
  kUnknownError,
  kSuccess,
  kFailedToAllocate,
  kBadCrc,
  kMemoryMappingFailed,
  kMalformedArchive,
  kIoError,
  kDiskFull,
  kNoFilename,
  kUnsupported,
  kEncryptedHeaders,
};

class Delegate {
 public:
  virtual ~Delegate() = default;

  // Handles errors that may occur when opening an archive.
  virtual void OnOpenError(Result result) = 0;

  // Handles a request for a temporary file to extract a "folder" (in the 7z
  // internal sense) into. This is only needed for some archives, so lazy
  // creation is useful. Guaranteed to be called at most once. Returns
  // a valid file to continue extraction. Returns an invalid file to to stop
  // extraction.
  virtual base::File OnTempFileRequest() = 0;

  // Handles a single entry in the 7z archive being ready for extraction.
  // Returns `true` to extract the entry, and `false` to stop extraction
  // entirely. When returning `true`, populates `output` with a span for
  // extraction. This span must have size equal to `entry.file_size`.
  virtual bool OnEntry(const EntryInfo &entry, base::span<uint8_t> &output) = 0;

  // Handles a single directory in the 7z archive being found. Returns `true` to
  // continue extraction, and `false` to stop extraction.
  virtual bool OnDirectory(const EntryInfo &entry) = 0;

  // Handles an entry being done extracting. If any errors occurred during
  // extraction, they are provided in `result`. Returns `true` to continue
  // extraction, and `false` to stop extraction.
  virtual bool EntryDone(Result result, const EntryInfo &entry) = 0;
};

// Encapsulates the state of the 7z unpacker. This class supports asynchronous
// unpacking. `delegate` can stop unpacking at any time and resume it with later
// calls to `Extract`.
class SevenZipReader {
 public:
  // Create a `SevenZipReader` with the given file and delegate. `delegate` must
  // outlive this object. Initialization of a `SevenZipReader` can fail, in
  // which case this function will return nullptr and notify the delegate.
  static std::unique_ptr<SevenZipReader> Create(base::File seven_zip_file,
                                                Delegate& delegate);

  ~SevenZipReader();

  SevenZipReader(const SevenZipReader&) = delete;
  SevenZipReader& operator=(const SevenZipReader&) = delete;

  SevenZipReader(SevenZipReader&&) = delete;
  SevenZipReader& operator=(SevenZipReader&&) = delete;

  // Begin extracting the archive. This will stop if the delegate requests it,
  // or once the archive is completely extracted.
  void Extract();

 private:
  SevenZipReader(std::unique_ptr<internal::SevenZipReaderImpl> impl,
                 Delegate& delegate);

  // Helper function for extracting a single entry. Returns `true` if extraction
  // should continue, and `false` if it should stop.
  bool ExtractEntry();

  std::unique_ptr<internal::SevenZipReaderImpl> impl_
      GUARDED_BY_CONTEXT(sequence_checker_);
  Delegate& delegate_ GUARDED_BY_CONTEXT(sequence_checker_);
  size_t entry_index_ GUARDED_BY_CONTEXT(sequence_checker_) = 0;

  SEQUENCE_CHECKER(sequence_checker_);
};

// Ensures that the one-time initialization of the LZMA SDK has been performed.
// This is usually called by `Extract` when needed, but is exposed here for
// fuzzer support.
void EnsureLzmaSdkInitialized();

} // namespace seven_zip

#endif // THIRD_PARTY_LZMA_SDK_GOOGLE_SEVEN_ZIP_READER_H_
