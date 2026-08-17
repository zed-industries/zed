/*
 * Copyright (C) 2022 The Android Open Source Project
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

#ifndef SRC_TRACE_PROCESSOR_UTIL_ZIP_READER_H_
#define SRC_TRACE_PROCESSOR_UTIL_ZIP_READER_H_

#include <cstddef>
#include <cstdint>
#include <functional>
#include <optional>
#include <string>
#include <utility>
#include <vector>

#include "perfetto/base/status.h"
#include "perfetto/ext/base/status_or.h"
#include "perfetto/ext/base/string_view.h"
#include "perfetto/trace_processor/trace_blob_view.h"
#include "src/trace_processor/util/gzip_utils.h"
#include "src/trace_processor/util/trace_blob_view_reader.h"

// ZipReader allows to read Zip files in a streaming fashion.
// Key features:
// - Read-only access, there is no ZipWriter.
// - Files can be processed as they are seen in the zip archive, without needing
//   to see the whole .zip file first.
// - It does not read the final zip central directory. Only the metadata in the
//   inline file headers is exposed.
// - Only the compressed payload is kept around in memory.
// - Supports line-based streaming for compressed text files (e.g. logs). This
//   enables line-based processing of compressed logs without having to
//   decompress fully the individual text file in memory.
// - Does NOT support zip64, encryption and other advanced zip file features.
// - It is not suitable for security-sensitive contexts. E.g. it doesn't deal
//   with zip path traversal attacks (the same file showing up twice with two
//   different payloads).
//
// Possible future features:
// - The user could setup a filter (a glob, or a callback) to select the
//   interesting files (e.g. *.txt) and skip the appending of the other entries.
//   This would avoid completely the cost of keeping in memory the compressed
//   payload of unwanted files (e.g. dumpstate.bin in BRs).

namespace perfetto::trace_processor::util {

class ZipReader;

constexpr size_t kZipFileHdrSize = 30;

// Holds the metadata and compressed payload of a zip file and allows
// decompression. The lifecycle of a ZipFile is completely independent of the
// ZipReader that created it. ZipFile(s) can be std::move(d) around and even
// outlive the ZipReader.
class ZipFile {
 public:
  // Note: the lifetime of the lines passed in the vector argument is valid only
  // for the duration of the callback. Don't retain the StringView(s) passed.
  using LinesCallback =
      std::function<void(const std::vector<base::StringView>&)>;

  ZipFile();
  ~ZipFile();
  ZipFile(ZipFile&&) noexcept;
  ZipFile& operator=(ZipFile&&) noexcept;
  ZipFile(const ZipFile&) = delete;
  ZipFile& operator=(const ZipFile&) = delete;

  // Bulk decompression. It keeps around the compressed data internally, so
  // this can be called several times.
  base::Status Decompress(std::vector<uint8_t>*) const;

  // Streaming line-based decompression for text files.
  // It decompresses the file in chunks and passes batches of lines to the
  // caller, without decompressing the whole file into memory.
  // The typical use case is processing large log files from a bugreport.
  // Like the above, this is idempotent and keeps around the compressed data.
  base::Status DecompressLines(LinesCallback) const;

  // File name, including the relative path (e.g., "FS/data/misc/foobar")
  const std::string& name() const { return hdr_.fname; }

  // Seconds since the Epoch. This is effectively time_t on 64 bit platforms.
  int64_t GetDatetime() const;

  // Returns the modified time in the format %Y-%m-%d %H:%M:%S.
  std::string GetDatetimeStr() const;

  size_t uncompressed_size() const { return hdr_.uncompressed_size; }
  size_t compressed_size() const { return hdr_.compressed_size; }

 private:
  friend class ZipReader;

  base::Status DoDecompressionChecks() const;

  // Rationale for having this as a nested sub-struct:
  // 1. Makes the move operator easier to maintain.
  // 2. Allows the ZipReader to handle a copy of this struct for the file
  //    being parsed. ZipReade will move the hdr into a full ZipFile once it
  //    has established the file is complete and valid.
  struct Header {
    uint32_t signature = 0;
    uint16_t version = 0;
    uint16_t flags = 0;
    uint16_t compression = 0;
    uint32_t checksum = 0;
    uint16_t mtime = 0;
    uint16_t mdate = 0;
    uint32_t compressed_size = 0;
    uint32_t uncompressed_size = 0;
    uint16_t fname_len = 0;
    uint16_t extra_field_len = 0;
    std::string fname;
  };

  Header hdr_{};
  TraceBlobView compressed_data_;
  // If adding new fields here, remember to update the move operators.
};

class ZipReader {
 public:
  ZipReader();
  ~ZipReader();

  ZipReader(const ZipReader&) = delete;
  ZipReader& operator=(const ZipReader&) = delete;
  ZipReader(ZipReader&&) = delete;
  ZipReader& operator=(ZipReader&&) = delete;

  // Parses data incrementally from a zip file in chunks. The chunks can be
  // arbitrarily cut. You can pass the whole file in one go, byte by byte or
  // anything in between.
  // files() is updated incrementally as soon as a new whole compressed file
  // has been processed. You don't need to get to the end of the zip file to
  // see all files. The final "central directory" at the end of the file is
  // actually ignored.
  base::Status Parse(TraceBlobView);

  // Returns a list of all the files discovered so far.
  const std::vector<ZipFile>& files() const { return files_; }

  // Moves ownership of the ZipFiles to the caller. The caller can use this
  // to reduce the memory working set and retain only the files they care about.
  std::vector<ZipFile> TakeFiles() { return std::move(files_); }

  // Find a file by its path inside the zip archive.
  ZipFile* Find(const std::string& path);

 private:
  // Keeps track of the incremental parsing state of the current zip stream.
  // When a compressed file is completely parsed, a ZipFile instance is
  // constructed and appended to `files_`.
  struct FileParseState {
    enum {
      kHeader,
      kFilename,
      kSkipBytes,
      kCompressedData,
    } parse_state = kHeader;
    size_t ignore_bytes_after_fname = 0;
    // Used to track the number of bytes fed into the decompressor when we don't
    // know the compressed size upfront.
    size_t decompressor_bytes_fed = 0;
    GzipDecompressor decompressor{GzipDecompressor::InputMode::kRawDeflate};
    std::optional<TraceBlobView> compressed;
    ZipFile::Header hdr{};
  };

  base::Status TryParseHeader();
  base::Status TryParseFilename();
  base::Status TrySkipBytes();
  base::Status TryParseCompressedData();
  base::StatusOr<std::optional<TraceBlobView>> TryParseUnsizedCompressedData();

  FileParseState cur_;
  std::vector<ZipFile> files_;
  util::TraceBlobViewReader reader_;
};

}  // namespace perfetto::trace_processor::util

#endif  // SRC_TRACE_PROCESSOR_UTIL_ZIP_READER_H_
