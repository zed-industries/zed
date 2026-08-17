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

#ifndef SRC_TRACE_PROCESSOR_IMPORTERS_INSTRUMENTS_INSTRUMENTS_XML_TOKENIZER_H_
#define SRC_TRACE_PROCESSOR_IMPORTERS_INSTRUMENTS_INSTRUMENTS_XML_TOKENIZER_H_

#include "perfetto/base/status.h"
#include "perfetto/trace_processor/trace_blob_view.h"
#include "src/trace_processor/importers/common/chunked_trace_reader.h"
#include "src/trace_processor/types/trace_processor_context.h"

namespace perfetto::trace_processor::instruments_importer {

class InstrumentsXmlTokenizer : public ChunkedTraceReader {
 public:
  explicit InstrumentsXmlTokenizer(TraceProcessorContext*);
  ~InstrumentsXmlTokenizer() override;

  base::Status Parse(TraceBlobView) override;

  [[nodiscard]] base::Status NotifyEndOfFile() override;

 private:
  class Impl;

  class Impl* impl_;
};

}  // namespace perfetto::trace_processor::instruments_importer

#endif  // SRC_TRACE_PROCESSOR_IMPORTERS_INSTRUMENTS_INSTRUMENTS_XML_TOKENIZER_H_
