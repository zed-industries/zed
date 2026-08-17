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

#ifndef SRC_TRACE_PROCESSOR_IMPORTERS_PERF_AUX_DATA_TOKENIZER_H_
#define SRC_TRACE_PROCESSOR_IMPORTERS_PERF_AUX_DATA_TOKENIZER_H_

#include <cstdint>
#include <functional>
#include <memory>

#include "perfetto/base/status.h"
#include "perfetto/ext/base/status_or.h"
#include "perfetto/trace_processor/trace_blob_view.h"
#include "src/trace_processor/importers/common/clock_tracker.h"
#include "src/trace_processor/importers/perf/perf_session.h"

namespace perfetto {
namespace trace_processor {

class TraceProcessorContext;
namespace perf_importer {

struct AuxRecord;
class AuxStream;
struct ItraceStartRecord;

class AuxDataStream {
 public:
  virtual ~AuxDataStream();
  virtual void OnDataLoss(uint64_t) = 0;
  virtual base::Status Parse(AuxRecord record, TraceBlobView data) = 0;
  virtual base::Status NotifyEndOfStream() = 0;
  virtual base::Status OnItraceStartRecord(ItraceStartRecord start) = 0;
};

// Base class for aux data tokenizers.
// A factory is created upon encountering an AUXTRACE_INFO record. the payload
// for such messages usually contains trace specific information to setup trace
// specific parsing. Subclasses are responsible for parsing the payload and
// storing any data needed to create `AuxDataStream` instances as new data
// streams are encountered in the trace.
class AuxDataTokenizer {
 public:
  virtual ~AuxDataTokenizer();
  virtual base::StatusOr<AuxDataStream*> InitializeAuxDataStream(
      AuxStream* stream) = 0;
};

// Dummy tokenizer that just discard data.
// Used to skip streams that we do not know how to parse.
class DummyAuxDataStream : public AuxDataStream {
 public:
  explicit DummyAuxDataStream(TraceProcessorContext* context);
  void OnDataLoss(uint64_t size) override;
  base::Status Parse(AuxRecord, TraceBlobView data) override;
  base::Status NotifyEndOfStream() override;
  base::Status OnItraceStartRecord(ItraceStartRecord start) override;

 private:
  TraceProcessorContext* const context_;
};

// Dummy tokenizer that just discard data.
// Used to skip streams that we do not know how to parse.
class DummyAuxDataTokenizer : public AuxDataTokenizer {
 public:
  explicit DummyAuxDataTokenizer(TraceProcessorContext* context);
  ~DummyAuxDataTokenizer() override;
  virtual base::StatusOr<AuxDataStream*> InitializeAuxDataStream(
      AuxStream* stream) override;

 private:
  DummyAuxDataStream stream_;
};

}  // namespace perf_importer
}  // namespace trace_processor
}  // namespace perfetto

#endif  // SRC_TRACE_PROCESSOR_IMPORTERS_PERF_AUX_DATA_TOKENIZER_H_
