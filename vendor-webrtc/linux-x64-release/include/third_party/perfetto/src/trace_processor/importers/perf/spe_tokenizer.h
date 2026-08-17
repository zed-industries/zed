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

#ifndef SRC_TRACE_PROCESSOR_IMPORTERS_PERF_SPE_TOKENIZER_H_
#define SRC_TRACE_PROCESSOR_IMPORTERS_PERF_SPE_TOKENIZER_H_

#include <memory>

#include "perfetto/ext/base/status_or.h"
#include "src/trace_processor/importers/perf/aux_data_tokenizer.h"
#include "src/trace_processor/importers/perf/aux_stream_manager.h"
#include "src/trace_processor/importers/perf/auxtrace_info_record.h"

namespace perfetto ::trace_processor {
class TraceProcessorContext;
namespace perf_importer {

class SpeTokenizer : public AuxDataTokenizer {
 public:
  static base::StatusOr<std::unique_ptr<AuxDataTokenizer>> Create(
      TraceProcessorContext* context,
      AuxtraceInfoRecord) {
    return std::unique_ptr<AuxDataTokenizer>(new SpeTokenizer(context));
  }
  ~SpeTokenizer() override;
  base::StatusOr<AuxDataStream*> InitializeAuxDataStream(
      AuxStream* stream) override;

 private:
  explicit SpeTokenizer(TraceProcessorContext* context) : context_(context) {}
  TraceProcessorContext* const context_;
  std::vector<std::unique_ptr<AuxDataStream>> streams_;
};

}  // namespace perf_importer
}  // namespace perfetto::trace_processor

#endif  // SRC_TRACE_PROCESSOR_IMPORTERS_PERF_SPE_TOKENIZER_H_
