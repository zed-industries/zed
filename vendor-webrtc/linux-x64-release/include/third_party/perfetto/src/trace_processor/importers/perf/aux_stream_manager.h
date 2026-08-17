/*
 * Copyright (C) 2023 The Android Open Source Project
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

#ifndef SRC_TRACE_PROCESSOR_IMPORTERS_PERF_AUX_STREAM_MANAGER_H_
#define SRC_TRACE_PROCESSOR_IMPORTERS_PERF_AUX_STREAM_MANAGER_H_

#include <cstdint>
#include <functional>
#include <memory>
#include <optional>
#include <variant>

#include "perfetto/base/logging.h"
#include "perfetto/base/status.h"
#include "perfetto/ext/base/circular_queue.h"
#include "perfetto/ext/base/flat_hash_map.h"
#include "perfetto/ext/base/status_or.h"
#include "perfetto/trace_processor/trace_blob_view.h"
#include "src/trace_processor/importers/perf/aux_data_tokenizer.h"
#include "src/trace_processor/importers/perf/aux_record.h"
#include "src/trace_processor/importers/perf/auxtrace_record.h"
#include "src/trace_processor/importers/perf/itrace_start_record.h"
#include "src/trace_processor/importers/perf/perf_session.h"
#include "src/trace_processor/importers/perf/time_conv_record.h"
#include "src/trace_processor/storage/stats.h"

namespace perfetto {
namespace trace_processor {
class TraceProcessorContext;

namespace perf_importer {

struct Record;
class SampleId;
struct AuxtraceInfoRecord;

class AuxStreamManager;

// Takes care of reconstructing the original data stream out of AUX and AUXTRACE
// records. Does not parse tha actual data it just forwards it to the associated
// `AuxDataTokenizer` .
class AuxStream {
 public:
  enum class Type {
    kCpuBound,
    kThreadBound,
  };

  ~AuxStream();
  std::optional<uint64_t> ConvertTscToPerfTime(uint64_t cycles);

  Type type() const { return type_; }

  uint32_t cpu() const {
    PERFETTO_CHECK(type_ == Type::kCpuBound);
    return tid_or_cpu_;
  }

  uint32_t tid() const {
    PERFETTO_CHECK(type_ == Type::kThreadBound);
    return tid_or_cpu_;
  }

 private:
  class AuxtraceDataReader {
   public:
    AuxtraceDataReader(AuxtraceRecord auxtrace, TraceBlobView data);

    TraceBlobView ConsumeFront(uint64_t size);
    void DropUntil(uint64_t offset);

    uint64_t offset() const { return offset_; }
    uint64_t end() const { return offset_ + data_.size(); }
    uint64_t size() const { return data_.size(); }

   private:
    uint64_t offset_;
    TraceBlobView data_;
  };

  using OutstandingRecord = std::variant<ItraceStartRecord, AuxRecord>;

  friend AuxStreamManager;
  AuxStream(AuxStreamManager* manager, Type type, uint32_t tid_or_cpu);

  base::Status OnAuxRecord(AuxRecord aux);
  base::Status OnAuxtraceRecord(AuxtraceRecord auxtrace, TraceBlobView data);
  base::Status NotifyEndOfStream();
  base::Status OnItraceStartRecord(ItraceStartRecord start);

  base::Status MaybeParse();

  AuxStreamManager& manager_;
  Type type_;
  uint32_t tid_or_cpu_;
  AuxDataStream* data_stream_;
  base::CircularQueue<OutstandingRecord> outstanding_records_;
  uint64_t aux_end_ = 0;
  base::CircularQueue<AuxtraceDataReader> outstanding_auxtrace_data_;
  uint64_t auxtrace_end_ = 0;
  uint64_t tokenizer_offset_ = 0;
};

// Keeps track of all aux streams in a perf file.
class AuxStreamManager {
 public:
  explicit AuxStreamManager(TraceProcessorContext* context)
      : context_(context) {}
  base::Status OnAuxtraceInfoRecord(AuxtraceInfoRecord info);
  base::Status OnAuxRecord(AuxRecord aux);
  base::Status OnAuxtraceRecord(AuxtraceRecord auxtrace, TraceBlobView data);
  base::Status OnItraceStartRecord(ItraceStartRecord start);
  base::Status OnTimeConvRecord(TimeConvRecord time_conv) {
    time_conv_ = std::move(time_conv);
    return base::OkStatus();
  }

  base::Status FinalizeStreams();

  TraceProcessorContext* context() const { return context_; }

  std::optional<uint64_t> ConvertTscToPerfTime(uint64_t cycles) {
    if (!time_conv_) {
      context_->storage->IncrementStats(stats::perf_no_tsc_data);
      return std::nullopt;
    }
    return time_conv_->ConvertTscToPerfTime(cycles);
  }

 private:
  base::StatusOr<std::reference_wrapper<AuxStream>>
  GetOrCreateStreamForSampleId(const std::optional<SampleId>& sample_id);
  base::StatusOr<std::reference_wrapper<AuxStream>> GetOrCreateStreamForCpu(
      uint32_t cpu);

  TraceProcessorContext* const context_;
  std::unique_ptr<AuxDataTokenizer> tokenizer_;
  base::FlatHashMap<uint32_t, std::unique_ptr<AuxStream>>
      auxdata_streams_by_cpu_;
  std::optional<TimeConvRecord> time_conv_;
};

inline std::optional<uint64_t> AuxStream::ConvertTscToPerfTime(
    uint64_t cycles) {
  return manager_.ConvertTscToPerfTime(cycles);
}

}  // namespace perf_importer
}  // namespace trace_processor
}  // namespace perfetto

#endif  // SRC_TRACE_PROCESSOR_IMPORTERS_PERF_AUX_STREAM_MANAGER_H_
