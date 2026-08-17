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

#ifndef INCLUDE_PERFETTO_TRACE_PROCESSOR_TRACE_PROCESSOR_H_
#define INCLUDE_PERFETTO_TRACE_PROCESSOR_TRACE_PROCESSOR_H_

#include <cstddef>
#include <cstdint>
#include <memory>
#include <string>
#include <vector>

#include "perfetto/base/export.h"
#include "perfetto/base/status.h"
#include "perfetto/trace_processor/basic_types.h"
#include "perfetto/trace_processor/iterator.h"
#include "perfetto/trace_processor/metatrace_config.h"
#include "perfetto/trace_processor/trace_blob_view.h"
#include "perfetto/trace_processor/trace_processor_storage.h"

namespace perfetto::trace_processor {

// Extends TraceProcessorStorage to support execution of SQL queries on loaded
// traces. See TraceProcessorStorage for parsing of trace files.
class PERFETTO_EXPORT_COMPONENT TraceProcessor : public TraceProcessorStorage {
 public:
  // For legacy API clients. Iterator used to be a nested class here. Many API
  // clients depends on it at this point.
  using Iterator = ::perfetto::trace_processor::Iterator;

  // Creates a new instance of TraceProcessor.
  static std::unique_ptr<TraceProcessor> CreateInstance(const Config&);

  ~TraceProcessor() override;

  // =================================================================
  // |        PerfettoSQL related functionality starts here          |
  // =================================================================

  // Executes the SQL on the loaded portion of the trace.
  //
  // More than one SQL statement can be passed to this function; all but the
  // last will be fully executed by this function before retuning. The last
  // statement will be executed and will yield rows as the caller calls Next()
  // over the returned Iterator.
  //
  // See documentation of the Iterator class for an example on how to use
  // the returned iterator.
  virtual Iterator ExecuteQuery(const std::string& sql) = 0;

  // Registers SQL files with the associated path under the package named
  // |sql_package.name|.
  //
  // For example, if you registered a package called "camera" with a file path
  // "camera/cpu/metrics.sql" you can include it (run the file) using "INCLUDE
  // PERFETTO MODULE camera.cpu.metrics". The first word of the string has to be
  // a package name and there can be only one package registered with a given
  // name.
  virtual base::Status RegisterSqlPackage(SqlPackage) = 0;

  // =================================================================
  // |        Trace summary related functionality starts here        |
  // =================================================================

  // Creates a summary of the trace as defined by the `computation` and `specs`
  // parameters.
  //
  // `computation` is a `TraceSummaryComputationSpec` struct which decides how
  // the trace should be summarized. It does not contain any business logic
  // itself, instead just referencing the contents of `specs`.
  //
  // Each entry in `specs` should point to an instance of the `TraceSummarySpec`
  // proto with `spec_format` defining the file format of each specs. This
  // function accepts a vector to make it easy to compute metrics in the common
  // case of having many different `TraceSummarySpec` files, each with a subset
  // of the summary to be computed (e.g. metrics shareded across multiple
  // files).
  //
  // The result of computing the summary will be returned in `output` (with a
  // schema specified by the `TraceSummary` proto) with `output_spec` defining
  // the format that the data should be returned in.
  //
  // Conceptual note: this function is designed with a split in `computation`
  // vs `specs` is to allow for `specs` to be stored as self-contained set of
  // protos on the filesystem or in a git repo which are then referenced by the
  // embedder of trace processor to actually decide which parts of the spec
  // matter for a particular trace. This allows decoupling what should be
  // computed from how that computation should happen.
  //
  // Implementation note: after this function returns, any or all of the
  // referenced PerfettoSQL modules in any computed metrics will remain
  // included. This behaviour is *not* considered part of the API and should not
  // be relied on. It is likely this will change in the future.
  virtual base::Status Summarize(
      const TraceSummaryComputationSpec& computation,
      const std::vector<TraceSummarySpecBytes>& specs,
      std::vector<uint8_t>* output,
      const TraceSummaryOutputSpec& output_spec) = 0;

  // =================================================================
  // |        Metatracing related functionality starts here          |
  // =================================================================

  // Enables "meta-tracing" of trace processor.
  // Metatracing involves tracing trace processor itself to root-cause
  // performace issues in trace processor. See |DisableAndReadMetatrace| for
  // more information on the format of the metatrace.
  using MetatraceConfig = metatrace::MetatraceConfig;
  using MetatraceCategories = metatrace::MetatraceCategories;
  virtual void EnableMetatrace(MetatraceConfig config = {}) = 0;

  // Disables "meta-tracing" of trace processor and writes the trace as a
  // sequence of |TracePackets| into |trace_proto| returning the status of this
  // read.
  virtual base::Status DisableAndReadMetatrace(
      std::vector<uint8_t>* trace_proto) = 0;

  // =================================================================
  // |              Advanced functionality starts here               |
  // =================================================================

  // Sets/returns the name of the currently loaded trace or an empty string if
  // no trace is fully loaded yet. This has no effect on the Trace Processor
  // functionality and is used for UI purposes only.
  // The returned name is NOT a path and will contain extra text w.r.t. the
  // argument originally passed to SetCurrentTraceName(), e.g., "file (42 MB)".
  virtual std::string GetCurrentTraceName() = 0;
  virtual void SetCurrentTraceName(const std::string&) = 0;

  // Registers the contents of a file.
  // This method can be used to pass out of band data to the trace processor
  // which can be used by importers to do some advanced processing. For example
  // if you pass binaries these are used to decode ETM traces.
  // Registering the same file twice will return an error.
  virtual base::Status RegisterFileContent(const std::string& path,
                                           TraceBlobView content) = 0;

  // Interrupts the current query. Typically used by Ctrl-C handler.
  virtual void InterruptQuery() = 0;

  // Restores Trace Processor to its pristine state. It preserves the built-in
  // tables/views/functions created by the ingestion process. Returns the number
  // of objects created in runtime that has been deleted.
  // NOTE: No Iterators can active when called.
  virtual size_t RestoreInitialTables() = 0;

  // Deprecated. Use |RegisterSqlPackage()| instead, which is identical in
  // functionality to |RegisterSqlModule()| and the only difference is in
  // the argument, which is directly translatable to |SqlPackage|.
  virtual base::Status RegisterSqlModule(SqlModule) = 0;

  // =================================================================
  // |  Trace-based metrics (v1) related functionality starts here   |
  // =================================================================
  //
  // WARNING: The metrics v1 system is "soft" deprecated: no new metrics are
  // allowed but we still fully support any existing metrics written using this
  // system.
  //
  // If possible, prefer using the metrics v2 methods above for any new
  // usecases.

  // Registers a metric at the given path which will run the specified SQL.
  virtual base::Status RegisterMetric(const std::string& path,
                                      const std::string& sql) = 0;

  // Reads the FileDescriptorSet proto message given by |data| and |size| and
  // adds any extensions to the metrics proto to allow them to be available as
  // proto builder functions when computing metrics.
  virtual base::Status ExtendMetricsProto(const uint8_t* data, size_t size) = 0;

  // Behaves exactly as ExtendMetricsProto, except any FileDescriptor with
  // filename matching a prefix in |skip_prefixes| is skipped.
  virtual base::Status ExtendMetricsProto(
      const uint8_t* data,
      size_t size,
      const std::vector<std::string>& skip_prefixes) = 0;

  // Computes the given metrics on the loded portion of the trace. If
  // successful, the output argument |metrics_proto| will be filled with the
  // proto-encoded bytes for the message TraceMetrics in
  // perfetto/metrics/metrics.proto.
  virtual base::Status ComputeMetric(
      const std::vector<std::string>& metric_names,
      std::vector<uint8_t>* metrics_proto) = 0;

  enum MetricResultFormat {
    kProtoText = 0,
    kJson = 1,
  };

  // Computes metrics as the ComputeMetric function above, but instead of
  // producing proto encoded bytes, the output argument |metrics_string| is
  // filled with the metric formatted in the requested |format|.
  virtual base::Status ComputeMetricText(
      const std::vector<std::string>& metric_names,
      MetricResultFormat format,
      std::string* metrics_string) = 0;

  // Gets all the currently loaded proto descriptors used in metric computation.
  // This includes all compiled-in binary descriptors, and all proto descriptors
  // loaded by trace processor shell at runtime. The message is encoded as
  // DescriptorSet, defined in perfetto/trace_processor/trace_processor.proto.
  virtual std::vector<uint8_t> GetMetricDescriptors() = 0;

  // =================================================================
  // |                        Experimental                           |
  // =================================================================

  virtual base::Status AnalyzeStructuredQueries(
      const std::vector<StructuredQueryBytes>& queries,
      std::vector<AnalyzedStructuredQuery>* output) = 0;
};

}  // namespace perfetto::trace_processor

#endif  // INCLUDE_PERFETTO_TRACE_PROCESSOR_TRACE_PROCESSOR_H_
