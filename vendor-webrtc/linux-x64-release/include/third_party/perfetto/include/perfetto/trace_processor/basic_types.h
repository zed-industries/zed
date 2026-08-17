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

#ifndef INCLUDE_PERFETTO_TRACE_PROCESSOR_BASIC_TYPES_H_
#define INCLUDE_PERFETTO_TRACE_PROCESSOR_BASIC_TYPES_H_

#include <cassert>
#include <cstdarg>
#include <cstddef>
#include <cstdint>

#include <optional>
#include <string>
#include <unordered_map>
#include <utility>
#include <vector>

#include "perfetto/base/export.h"
#include "perfetto/base/logging.h"

namespace perfetto::trace_processor {

// All metrics protos are in this directory. When loading metric extensions, the
// protos are mounted onto a virtual path inside this directory.
constexpr char kMetricProtoRoot[] = "protos/perfetto/metrics/";

// Enum which encodes how trace processor should parse the ingested data.
enum class ParsingMode {
  // This option causes trace processor to tokenize the raw trace bytes, sort
  // the events into timestamp order and parse the events into tables.
  //
  // This is the default mode.
  kDefault = 0,

  // This option causes trace processor to skip the sorting and parsing
  // steps of ingesting a trace, only retaining any information which could be
  // gathered during tokenization of the trace files.
  //
  // Note the exact information available with this option is left intentionally
  // undefined as it relies heavily on implementation details of trace
  // processor. It is mainly intended for use by the Perfetto UI which
  // integrates very closely with trace processor. General users should use
  // `kDefault` unless they know what they are doing.
  kTokenizeOnly = 1,

  // This option causes trace processor to skip the parsing step of ingesting
  // a trace.
  //
  // Note this option does not offer any visible benefits over `kTokenizeOnly`
  // but has the downside of being slower. It mainly exists for use by
  // developers debugging performance of trace processor.
  kTokenizeAndSort = 2,
};

// Enum which encodes how trace processor should try to sort the ingested data.
enum class SortingMode {
  // This option allows trace processor to use built-in heuristics about how to
  // sort the data. Generally, this option is correct for most embedders as
  // trace processor reads information from the trace to make the best decision.
  //
  // The exact heuristics are implementation details but will ensure that all
  // relevant tables are sorted by timestamp.
  //
  // This is the default mode.
  kDefaultHeuristics = 0,

  // This option forces trace processor to wait for all events to be passed to
  // it before doing a full sort of all the events. This causes any
  // heuristics trace processor would normally use to ingest partially sorted
  // data to be skipped.
  kForceFullSort = 1,
};

// Enum which encodes which event (if any) should be used to drop ftrace data
// from before this timestamp of that event.
enum class DropFtraceDataBefore {
  // Drops ftrace data before timestmap specified by the
  // TracingServiceEvent::tracing_started packet. If this packet is not in the
  // trace, no data is dropped. If preserve_ftrace_buffer (from the trace
  // config) is set, no data is dropped.
  // Note: this event was introduced in S+ so no data will be dropped on R-
  // traces.
  // This is the default approach.
  kTracingStarted = 0,

  // Retains all ftrace data regardless of timestamp and other events.
  kNoDrop = 1,

  // Drops ftrace data before timestmap specified by the
  // TracingServiceEvent::all_data_sources_started. If this packet is not in the
  // trace, no data is dropped.
  // This option can be used in cases where R- traces are being considered and
  // |kTracingStart| cannot be used because the event was not present.
  kAllDataSourcesStarted = 2
};

// Specifies whether the ftrace data should be "soft-dropped" until a given
// global timestamp, meaning we'll still populate the |ftrace_events| table
// and some other internal storage, but won't persist derived info such as
// slices. See also |DropFtraceDataBefore| above.
// Note: this might behave in surprising ways for traces using >1 tracefs
// instances, but those aren't seen in practice at the time of writing.
enum class SoftDropFtraceDataBefore {
  // Drop until the earliest timestamp covered by all per-cpu event bundles.
  // In other words, the maximum of all per-cpu "valid from" timestamps.
  // Important for correct parsing of traces where the ftrace data is written
  // into a central perfetto buffer in ring-buffer mode (as opposed to discard
  // mode).
  kAllPerCpuBuffersValid = 0,

  // Keep all events (though DropFtraceDataBefore still applies).
  kNoDrop = 1
};

// Enum which encodes which timestamp source (if any) should be used to drop
// track event data before this timestamp.
enum class DropTrackEventDataBefore {
  // Retain all track events. This is the default approach.
  kNoDrop = 0,

  // Drops track events before the timestamp specified by the
  // TrackEventRangeOfInterest trace packet. No data is dropped if this packet
  // is not present in the trace.
  kTrackEventRangeOfInterest = 1,
};

// Struct for configuring a TraceProcessor instance (see trace_processor.h).
struct PERFETTO_EXPORT_COMPONENT Config {
  // Indicates the parsing mode trace processor should use to extract
  // information from the raw trace bytes. See the enum documentation for more
  // details.
  ParsingMode parsing_mode = ParsingMode::kDefault;

  // Indicates the sortinng mode that trace processor should use on the
  // passed trace packets. See the enum documentation for more details.
  SortingMode sorting_mode = SortingMode::kDefaultHeuristics;

  // When set to false, this option makes the trace processor not include ftrace
  // events in the ftrace_event table; this makes converting events back to the
  // systrace text format impossible. On the other hand, it also saves ~50% of
  // memory usage of trace processor. For reference, Studio intends to use this
  // option.
  //
  // Note: "generic" ftrace events will be parsed into the ftrace_event table
  // even if this flag is false.
  //
  // Note: this option should really be named
  // `ingest_ftrace_in_ftrace_event_table` as the use of the `raw` table is
  // deprecated.
  bool ingest_ftrace_in_raw_table = true;

  // Indicates the event which should be used as a marker to drop ftrace data in
  // the trace before that event. See the enum documentation for more details.
  DropFtraceDataBefore drop_ftrace_data_before =
      DropFtraceDataBefore::kTracingStarted;

  // Specifies whether the ftrace data should be "soft-dropped" until a given
  // global timestamp.
  SoftDropFtraceDataBefore soft_drop_ftrace_data_before =
      SoftDropFtraceDataBefore::kAllPerCpuBuffersValid;

  // Indicates the source of timestamp before which track events should be
  // dropped. See the enum documentation for more details.
  DropTrackEventDataBefore drop_track_event_data_before =
      DropTrackEventDataBefore::kNoDrop;

  // Any built-in metric proto or sql files matching these paths are skipped
  // during trace processor metric initialization.
  std::vector<std::string> skip_builtin_metric_paths;

  // When set to true, the trace processor analyzes trace proto content, and
  // exports the field path -> total size mapping into an SQL table.
  //
  // The analysis feature is hidden behind the flag so that the users who don't
  // need this feature don't pay the performance costs.
  //
  // The flag has no impact on non-proto traces.
  bool analyze_trace_proto_content = false;

  // When set to true, trace processor will be augmented with a bunch of helpful
  // features for local development such as extra SQL fuctions.
  //
  // Note that the features behind this flag are subject to breakage without
  // backward compability guarantees at any time.
  bool enable_dev_features = false;

  // Sets developer-only flags to the provided values. Does not have any affect
  // unless |enable_dev_features| = true.
  std::unordered_map<std::string, std::string> dev_flags;

  // When set to true, trace processor will perform additional runtime checks
  // to catch additional classes of SQL errors.
  bool enable_extra_checks = false;
};

// Represents a dynamically typed value returned by SQL.
struct PERFETTO_EXPORT_COMPONENT SqlValue {
  // Represents the type of the value.
  enum Type {
    kNull = 0,
    kLong,
    kDouble,
    kString,
    kBytes,
    kLastType = kBytes,
  };

  SqlValue() = default;

  static SqlValue Long(int64_t v) {
    SqlValue value;
    value.long_value = v;
    value.type = Type::kLong;
    return value;
  }

  static SqlValue Double(double v) {
    SqlValue value;
    value.double_value = v;
    value.type = Type::kDouble;
    return value;
  }

  static SqlValue String(const char* v) {
    SqlValue value;
    value.string_value = v;
    value.type = Type::kString;
    return value;
  }

  static SqlValue Bytes(const void* v, size_t size) {
    SqlValue value;
    value.bytes_value = v;
    value.bytes_count = size;
    value.type = Type::kBytes;
    return value;
  }

  double AsDouble() const {
    PERFETTO_CHECK(type == kDouble);
    return double_value;
  }
  int64_t AsLong() const {
    PERFETTO_CHECK(type == kLong);
    return long_value;
  }
  const char* AsString() const {
    PERFETTO_CHECK(type == kString);
    return string_value;
  }
  const void* AsBytes() const {
    PERFETTO_CHECK(type == kBytes);
    return bytes_value;
  }

  bool is_null() const { return type == Type::kNull; }

  // Up to 1 of these fields can be accessed depending on |type|.
  union {
    // This string will be owned by the iterator that returned it and is valid
    // as long until the subsequent call to Next().
    const char* string_value;
    int64_t long_value;
    double double_value;
    const void* bytes_value;
  };
  // The size of bytes_value. Only valid when |type == kBytes|.
  size_t bytes_count = 0;
  Type type = kNull;
};

// Data used to register a new SQL package.
struct SqlPackage {
  // Must be unique among package, or can be used to override existing package
  // if |allow_override| is set.
  std::string name;

  // Pairs of strings mapping from the name of the module used by `INCLUDE
  // PERFETTO MODULE` statements to the contents of SQL files being executed.
  // Module names should only contain alphanumeric characters and '.', where
  // string before the first dot must be the package name.
  //
  // It is encouraged that include key should be the path to the SQL file being
  // run, with slashes replaced by dots and without the SQL extension. For
  // example, 'android/camera/jank.sql' would be included by
  // 'android.camera.jank'. This conforms to user expectations of how modules
  // behave in other languages (e.g. Java, Python etc).
  std::vector<std::pair<std::string, std::string>> modules;

  // If true, will allow overriding a package which already exists with `name.
  // Can only be set if enable_dev_features (in the TraceProcessorConfig object
  // when creating TraceProcessor) is true. Otherwise, this option will throw an
  // error.
  bool allow_override = false;
};

// Struct which defines how the trace should be summarized by
// `TraceProcessor::Summarize`.
struct TraceSummaryComputationSpec {
  // The set of metric ids which should be computed and returned in the
  // `TraceSummary` proto.
  std::vector<std::string> v2_metric_ids;

  // The id of the query (which must exist in the `query` field of one of the
  // TraceSummary specs) which will be used to populate the `metadata` field of
  // the TraceSummary proto. This query *must* output exactly two string columns
  // `key` and `value` which will be used to populate the `metadata` field of
  // the output proto.
  std::optional<std::string> metadata_query_id;
};

// A struct which defines the how the TraceSummary output proto should be
// formatted.
struct TraceSummaryOutputSpec {
  // The file format of the output returned from the trace summary functions.
  enum class Format : uint8_t {
    // Indicates that the ouput is `TraceSummary` encoded as a binary protobuf.
    kBinaryProto,
    // Indicates that the ouput is `TraceSummary` encoded as a text protobuf.
    kTextProto,
  };
  Format format;
};

// A struct wrapping the bytes of a `TraceSummarySpec` instance.
struct TraceSummarySpecBytes {
  // The pointer to the contents of `TraceSummarySpec`
  const uint8_t* ptr;

  // The number of bytes of the `TraceSummarySpec.
  size_t size;

  // The format of the data located at the pointer above.
  enum class Format : uint8_t {
    // Indicates that the spec is `TraceSummarySpec` encoded as a binary
    // protobuf.
    kBinaryProto,
    // Indicates that the spec is `TraceSummarySpec` encoded as a text
    // protobuf.
    kTextProto,
  };
  Format format;
};

// A struct wrapping the bytes of a `StructuredQuery` instance.
struct StructuredQueryBytes {
  // The pointer to the contents of `StructuredQuery`
  const uint8_t* ptr;

  // The number of bytes of the `StructuredQuery.
  size_t size;

  // The format of the data located at the pointer above.
  enum class Format : uint8_t {
    // Indicates that the spec is `StructuredQuery` encoded as a binary
    // protobuf.
    kBinaryProto,
    // Indicates that the spec is `StructuredQuery` encoded as a text
    // protobuf.
    kTextProto,
  };
  Format format;
};

// Experimental. Not considered part of Trace Processor API and shouldn't be
// used.
struct AnalyzedStructuredQuery {
  std::string sql;
  std::string textproto;

  // Modules referenced by sql
  std::vector<std::string> modules;
  // Preambles referenced by sql
  std::vector<std::string> preambles;
};

// Deprecated. Please use `RegisterSqlPackage` and `SqlPackage` instead.
struct SqlModule {
  std::string name;
  std::vector<std::pair<std::string, std::string>> files;
  bool allow_module_override = false;
};

}  // namespace perfetto::trace_processor

#endif  // INCLUDE_PERFETTO_TRACE_PROCESSOR_BASIC_TYPES_H_
