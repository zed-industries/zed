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

#ifndef SRC_TRACE_PROCESSOR_TRACE_PROCESSOR_IMPL_H_
#define SRC_TRACE_PROCESSOR_TRACE_PROCESSOR_IMPL_H_

#include <sqlite3.h>

#include <atomic>
#include <cstddef>
#include <cstdint>
#include <memory>
#include <string>
#include <unordered_map>
#include <vector>

#include "perfetto/base/status.h"
#include "perfetto/trace_processor/basic_types.h"
#include "perfetto/trace_processor/trace_blob_view.h"
#include "perfetto/trace_processor/trace_processor.h"
#include "src/trace_processor/iterator_impl.h"
#include "src/trace_processor/metrics/metrics.h"
#include "src/trace_processor/perfetto_sql/engine/dataframe_shared_storage.h"
#include "src/trace_processor/perfetto_sql/engine/perfetto_sql_engine.h"
#include "src/trace_processor/perfetto_sql/intrinsics/functions/create_function.h"
#include "src/trace_processor/perfetto_sql/intrinsics/functions/create_view_function.h"
#include "src/trace_processor/trace_processor_storage_impl.h"
#include "src/trace_processor/util/descriptors.h"

namespace perfetto::trace_processor {

// Coordinates the loading of traces from an arbitrary source and allows
// execution of SQL queries on the events in these traces.
class TraceProcessorImpl : public TraceProcessor,
                           public TraceProcessorStorageImpl {
 public:
  explicit TraceProcessorImpl(const Config&);

  TraceProcessorImpl(const TraceProcessorImpl&) = delete;
  TraceProcessorImpl& operator=(const TraceProcessorImpl&) = delete;

  TraceProcessorImpl(TraceProcessorImpl&&) = delete;
  TraceProcessorImpl& operator=(TraceProcessorImpl&&) = delete;

  ~TraceProcessorImpl() override;

  // =================================================================
  // |        TraceProcessorStorage implementation starts here       |
  // =================================================================

  base::Status Parse(TraceBlobView) override;
  void Flush() override;
  base::Status NotifyEndOfFile() override;

  // =================================================================
  // |        PerfettoSQL related functionality starts here          |
  // =================================================================

  Iterator ExecuteQuery(const std::string& sql) override;

  base::Status RegisterSqlPackage(SqlPackage) override;

  base::Status RegisterSqlModule(SqlModule module) override;

  // =================================================================
  // |  Trace-based metrics (v2) related functionality starts here   |
  // =================================================================

  base::Status Summarize(const TraceSummaryComputationSpec& computation,
                         const std::vector<TraceSummarySpecBytes>& specs,
                         std::vector<uint8_t>* output,
                         const TraceSummaryOutputSpec& output_spec) override;

  // =================================================================
  // |        Metatracing related functionality starts here          |
  // =================================================================

  void EnableMetatrace(MetatraceConfig config) override;

  base::Status DisableAndReadMetatrace(
      std::vector<uint8_t>* trace_proto) override;

  // =================================================================
  // |              Advanced functionality starts here               |
  // =================================================================

  std::string GetCurrentTraceName() override;
  void SetCurrentTraceName(const std::string&) override;

  base::Status RegisterFileContent(const std::string& path,
                                   TraceBlobView content) override;

  void InterruptQuery() override;

  size_t RestoreInitialTables() override;

  // =================================================================
  // |  Trace-based metrics (v1) related functionality starts here   |
  // =================================================================

  base::Status RegisterMetric(const std::string& path,
                              const std::string& sql) override;

  base::Status ExtendMetricsProto(const uint8_t* data, size_t size) override;
  base::Status ExtendMetricsProto(
      const uint8_t* data,
      size_t size,
      const std::vector<std::string>& skip_prefixes) override;

  base::Status ComputeMetric(const std::vector<std::string>& metric_names,
                             std::vector<uint8_t>* metrics) override;
  base::Status ComputeMetricText(const std::vector<std::string>& metric_names,
                                 TraceProcessor::MetricResultFormat format,
                                 std::string* metrics_string) override;

  std::vector<uint8_t> GetMetricDescriptors() override;

  // ===================
  // |  Experimental   |
  // ===================

  base::Status AnalyzeStructuredQueries(
      const std::vector<StructuredQueryBytes>&,
      std::vector<AnalyzedStructuredQuery>*) override;

 private:
  // Needed for iterators to be able to access the context.
  friend class IteratorImpl;

  template <typename Table>
  void RegisterStaticTable(Table* table) {
    engine_->RegisterStaticTable(table, Table::Name(),
                                 Table::ComputeStaticSchema());
  }

  bool IsRootMetricField(const std::string& metric_name);

  void InitPerfettoSqlEngine();
  void IncludeBeforeEofPrelude();
  void IncludeAfterEofPrelude();

  const Config config_;

  DataframeSharedStorage dataframe_shared_storage_;
  std::unique_ptr<PerfettoSqlEngine> engine_;

  DescriptorPool metrics_descriptor_pool_;

  std::vector<metrics::SqlMetricFile> sql_metrics_;

  // Manually registers SQL packages are stored here, to be able to restore
  // them when running |RestoreInitialTables()|.
  std::vector<SqlPackage> manually_registered_sql_packages_;

  std::unordered_map<std::string, std::string> proto_field_to_sql_metric_path_;
  std::unordered_map<std::string, std::string> proto_fn_name_to_path_;

  // This is atomic because it is set by the CTRL-C signal handler and we need
  // to prevent single-flow compiler optimizations in ExecuteQuery().
  std::atomic<bool> query_interrupted_{false};

  // Track the number of objects registered with SQLite post prelude.
  uint64_t sqlite_objects_post_prelude_ = 0;

  std::string current_trace_name_;
  uint64_t bytes_parsed_ = 0;

  // NotifyEndOfFile should only be called once. Set to true whenever it is
  // called.
  bool notify_eof_called_ = false;
};

}  // namespace perfetto::trace_processor

#endif  // SRC_TRACE_PROCESSOR_TRACE_PROCESSOR_IMPL_H_
