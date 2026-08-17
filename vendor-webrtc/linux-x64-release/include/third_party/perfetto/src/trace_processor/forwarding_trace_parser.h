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

#ifndef SRC_TRACE_PROCESSOR_FORWARDING_TRACE_PARSER_H_
#define SRC_TRACE_PROCESSOR_FORWARDING_TRACE_PARSER_H_

#include <memory>

#include "perfetto/base/status.h"
#include "perfetto/trace_processor/trace_blob_view.h"
#include "src/trace_processor/importers/common/chunked_trace_reader.h"
#include "src/trace_processor/tables/metadata_tables_py.h"
#include "src/trace_processor/util/trace_type.h"

namespace perfetto::trace_processor {

class TraceProcessorContext;

class ForwardingTraceParser : public ChunkedTraceReader {
 public:
  explicit ForwardingTraceParser(TraceProcessorContext*,
                                 tables::TraceFileTable::Id);
  ~ForwardingTraceParser() override;

  // ChunkedTraceReader implementation
  base::Status Parse(TraceBlobView) override;
  [[nodiscard]] base::Status NotifyEndOfFile() override;

  TraceType trace_type() const { return trace_type_; }

 private:
  base::Status Init(const TraceBlobView&);
  void UpdateSorterForTraceType(TraceType trace_type);
  TraceProcessorContext* const context_;
  tables::TraceFileTable::Id file_id_;
  size_t trace_size_ = 0;
  std::unique_ptr<ChunkedTraceReader> reader_;
  TraceType trace_type_ = kUnknownTraceType;
};

}  // namespace perfetto::trace_processor

#endif  // SRC_TRACE_PROCESSOR_FORWARDING_TRACE_PARSER_H_
