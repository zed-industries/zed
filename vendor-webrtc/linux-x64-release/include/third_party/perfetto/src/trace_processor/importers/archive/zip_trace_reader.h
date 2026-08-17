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

#ifndef SRC_TRACE_PROCESSOR_IMPORTERS_ARCHIVE_ZIP_TRACE_READER_H_
#define SRC_TRACE_PROCESSOR_IMPORTERS_ARCHIVE_ZIP_TRACE_READER_H_

#include <cstddef>
#include <map>

#include "perfetto/base/status.h"
#include "perfetto/trace_processor/trace_blob_view.h"
#include "src/trace_processor/importers/archive/archive_entry.h"
#include "src/trace_processor/importers/common/chunked_trace_reader.h"
#include "src/trace_processor/tables/metadata_tables_py.h"
#include "src/trace_processor/util/zip_reader.h"

namespace perfetto::trace_processor {

class ForwardingTraceParser;
class TraceProcessorContext;

// Forwards files contained in a ZIP to the appropriate ChunkedTraceReader. It
// is guaranteed that proto traces will be parsed first.
class ZipTraceReader : public ChunkedTraceReader {
 public:
  explicit ZipTraceReader(TraceProcessorContext* context);
  ~ZipTraceReader() override;

  // ChunkedTraceReader implementation
  base::Status Parse(TraceBlobView) override;
  base::Status NotifyEndOfFile() override;

 private:
  struct File {
    tables::TraceFileTable::Id id;
    TraceBlobView data;
  };
  TraceProcessorContext* const context_;
  util::ZipReader zip_reader_;
};

}  // namespace perfetto::trace_processor

#endif  // SRC_TRACE_PROCESSOR_IMPORTERS_ARCHIVE_ZIP_TRACE_READER_H_
