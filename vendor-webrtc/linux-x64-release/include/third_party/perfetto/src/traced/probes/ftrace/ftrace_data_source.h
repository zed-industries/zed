/*
 * Copyright (C) 2018 The Android Open Source Project
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

#ifndef SRC_TRACED_PROBES_FTRACE_FTRACE_DATA_SOURCE_H_
#define SRC_TRACED_PROBES_FTRACE_FTRACE_DATA_SOURCE_H_

#include <functional>
#include <map>
#include <memory>

#include "perfetto/base/flat_set.h"
#include "perfetto/ext/base/weak_ptr.h"
#include "perfetto/ext/tracing/core/basic_types.h"
#include "perfetto/ext/tracing/core/trace_writer.h"
#include "src/traced/probes/ftrace/ftrace_config_utils.h"
#include "src/traced/probes/ftrace/ftrace_metadata.h"
#include "src/traced/probes/ftrace/ftrace_stats.h"
#include "src/traced/probes/probes_data_source.h"

namespace perfetto {

class FtraceController;
class ProcessStatsDataSource;
class InodeFileDataSource;
struct FtraceDataSourceConfig;

namespace protos {
namespace pbzero {
enum FtraceParseStatus : int32_t;
}  // namespace pbzero
}  // namespace protos

// This class handles the state for one particular tracing session involving
// ftrace. There can be several concurrent tracing sessions involving ftrace
// and this class is essentially the building block used to multiplex them.
// This class is instantiated by ProbesProducer. ProbesProducer also owns the
// FtraceController.
class FtraceDataSource : public ProbesDataSource {
 public:
  static const ProbesDataSource::Descriptor descriptor;

  FtraceDataSource(base::WeakPtr<FtraceController>,
                   TracingSessionID,
                   FtraceConfig,
                   std::unique_ptr<TraceWriter>);
  ~FtraceDataSource() override;

  // Hands out internal pointers to callbacks.
  FtraceDataSource(const FtraceDataSource&) = delete;
  FtraceDataSource& operator=(const FtraceDataSource&) = delete;
  FtraceDataSource(FtraceDataSource&&) = delete;
  FtraceDataSource& operator=(FtraceDataSource&&) = delete;

  // Called by FtraceController soon after ProbesProducer creates the data
  // source, to inject ftrace dependencies.
  void Initialize(FtraceConfigId, const FtraceDataSourceConfig* parsing_config);

  // ProbesDataSource implementation.
  void Start() override;

  // Flushes the ftrace buffers into the userspace trace buffers and writes
  // also ftrace stats.
  void Flush(FlushRequestID, std::function<void()> callback) override;
  void OnFtraceFlushComplete(FlushRequestID);

  FtraceConfigId config_id() const { return config_id_; }
  const FtraceConfig& config() const { return config_; }
  const FtraceDataSourceConfig* parsing_config() const {
    return parsing_config_;
  }

  FtraceMetadata* mutable_metadata() { return &metadata_; }
  FtraceSetupErrors* mutable_setup_errors() {
    return &stats_before_.setup_errors;
  }
  base::FlatSet<protos::pbzero::FtraceParseStatus>* mutable_parse_errors() {
    return &parse_errors_;
  }
  TraceWriter* trace_writer() { return writer_.get(); }

  uint64_t* mutable_bundle_end_timestamp(size_t cpu) {
    if (cpu >= bundle_end_ts_by_cpu_.size())
      bundle_end_ts_by_cpu_.resize(cpu + 1);
    return &bundle_end_ts_by_cpu_[cpu];
  }

 private:
  void WriteStats();

  const FtraceConfig config_;
  FtraceMetadata metadata_;
  // Stats as saved during data source setup, will be emitted with phase
  // START_OF_TRACE on every flush:
  FtraceStats stats_before_{};
  // Accumulates errors encountered while parsing the binary ftrace data (e.g.
  // data disagreeing with our understanding of the ring buffer ABI):
  base::FlatSet<protos::pbzero::FtraceParseStatus> parse_errors_;
  std::map<FlushRequestID, std::function<void()>> pending_flushes_;
  // Remembers, for each per-cpu buffer, the last written event's timestamp.
  std::vector<uint64_t> bundle_end_ts_by_cpu_;

  // -- Fields initialized by the Initialize() call:
  FtraceConfigId config_id_ = 0;
  std::unique_ptr<TraceWriter> writer_;
  base::WeakPtr<FtraceController> controller_weak_;
  // Muxer-held state for parsing ftrace according to this data source's
  // configuration. Not the raw FtraceConfig proto (held by |config_|).
  const FtraceDataSourceConfig* parsing_config_;
  // -- End of fields set by Initialize().
};

}  // namespace perfetto

#endif  // SRC_TRACED_PROBES_FTRACE_FTRACE_DATA_SOURCE_H_
