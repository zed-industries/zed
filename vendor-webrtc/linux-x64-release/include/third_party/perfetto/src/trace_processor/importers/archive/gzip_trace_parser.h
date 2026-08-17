/*
 * Copyright (C) 2019 The Android Open Source Project
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

#ifndef SRC_TRACE_PROCESSOR_IMPORTERS_ARCHIVE_GZIP_TRACE_PARSER_H_
#define SRC_TRACE_PROCESSOR_IMPORTERS_ARCHIVE_GZIP_TRACE_PARSER_H_

#include <cstddef>
#include <cstdint>
#include <memory>

#include "perfetto/base/status.h"
#include "src/trace_processor/importers/common/chunked_trace_reader.h"
#include "src/trace_processor/util/gzip_utils.h"

namespace perfetto::trace_processor {

class TraceProcessorContext;

class GzipTraceParser : public ChunkedTraceReader {
 public:
  explicit GzipTraceParser(TraceProcessorContext*);
  explicit GzipTraceParser(std::unique_ptr<ChunkedTraceReader>);
  ~GzipTraceParser() override;

  // ChunkedTraceReader implementation
  base::Status Parse(TraceBlobView) override;
  base::Status NotifyEndOfFile() override;

  base::Status ParseUnowned(const uint8_t*, size_t);

 private:
  TraceProcessorContext* const context_;
  util::GzipDecompressor decompressor_;
  std::unique_ptr<ChunkedTraceReader> inner_;

  std::unique_ptr<uint8_t[]> buffer_;
  size_t bytes_written_ = 0;

  bool first_chunk_parsed_ = false;
  enum { kStreamBoundary, kMidStream } output_state_ = kStreamBoundary;
};

}  // namespace perfetto::trace_processor

#endif  // SRC_TRACE_PROCESSOR_IMPORTERS_ARCHIVE_GZIP_TRACE_PARSER_H_
