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

#ifndef SRC_TRACE_PROCESSOR_IMPORTERS_PERF_TEXT_PERF_TEXT_TRACE_TOKENIZER_H_
#define SRC_TRACE_PROCESSOR_IMPORTERS_PERF_TEXT_PERF_TEXT_TRACE_TOKENIZER_H_

#include <string>
#include "perfetto/base/status.h"
#include "perfetto/ext/base/flat_hash_map.h"
#include "src/trace_processor/importers/common/chunked_trace_reader.h"
#include "src/trace_processor/importers/common/virtual_memory_mapping.h"
#include "src/trace_processor/types/trace_processor_context.h"
#include "src/trace_processor/util/trace_blob_view_reader.h"

namespace perfetto::trace_processor::perf_text_importer {

class PerfTextTraceTokenizer : public ChunkedTraceReader {
 public:
  explicit PerfTextTraceTokenizer(TraceProcessorContext*);
  ~PerfTextTraceTokenizer() override;

  base::Status Parse(TraceBlobView) override;
  base::Status NotifyEndOfFile() override;

 private:
  TraceProcessorContext* const context_;
  util::TraceBlobViewReader reader_;
  base::FlatHashMap<std::string, DummyMemoryMapping*> mappings_;
};

}  // namespace perfetto::trace_processor::perf_text_importer

#endif  // SRC_TRACE_PROCESSOR_IMPORTERS_PERF_TEXT_PERF_TEXT_TRACE_TOKENIZER_H_
