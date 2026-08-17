/*
 * Copyright (C) 2024 The Android Open Source Project
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

#ifndef SRC_TRACE_PROCESSOR_IMPORTERS_ARCHIVE_TAR_TRACE_READER_H_
#define SRC_TRACE_PROCESSOR_IMPORTERS_ARCHIVE_TAR_TRACE_READER_H_

#include <cstddef>
#include <cstdint>
#include <map>

#include "perfetto/base/status.h"
#include "perfetto/ext/base/status_or.h"
#include "perfetto/trace_processor/trace_blob_view.h"
#include "src/trace_processor/importers/archive/archive_entry.h"
#include "src/trace_processor/importers/common/chunked_trace_reader.h"
#include "src/trace_processor/tables/metadata_tables_py.h"
#include "src/trace_processor/util/trace_blob_view_reader.h"

namespace perfetto::trace_processor {

class TraceProcessorContext;

class TarTraceReader : public ChunkedTraceReader {
 public:
  explicit TarTraceReader(TraceProcessorContext*);
  ~TarTraceReader() override;

  // ChunkedTraceReader implementation
  base::Status Parse(TraceBlobView) override;
  base::Status NotifyEndOfFile() override;

 private:
  struct Metadata {
    std::string name;
    uint64_t size;
    char type_flag;
  };
  enum class ParseResult {
    kOk,
    kNeedsMoreData,
  };

  struct File {
    tables::TraceFileTable::Id id;
    std::vector<TraceBlobView> data;
  };

  enum class State { kMetadata, kContent, kZeroMetadata, kDone };

  base::StatusOr<ParseResult> ParseMetadata();
  base::StatusOr<ParseResult> ParseContent();
  base::StatusOr<ParseResult> ParseLongName();
  base::StatusOr<ParseResult> ParsePadding();

  void AddFile(const Metadata& metadata,
               TraceBlobView header,
               std::vector<TraceBlobView> data);

  TraceProcessorContext* const context_;
  State state_{State::kMetadata};
  util::TraceBlobViewReader buffer_;
  std::optional<Metadata> metadata_;
  std::optional<std::string> long_name_;
  std::map<ArchiveEntry, File> ordered_files_;
};

}  // namespace perfetto::trace_processor

#endif  // SRC_TRACE_PROCESSOR_IMPORTERS_ARCHIVE_TAR_TRACE_READER_H_
