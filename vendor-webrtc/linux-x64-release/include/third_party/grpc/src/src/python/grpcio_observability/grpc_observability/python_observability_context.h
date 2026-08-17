// Copyright 2023 gRPC authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#ifndef GRPC_PYTHON_OBSERVABILITY_H
#define GRPC_PYTHON_OBSERVABILITY_H

#include <grpc/slice.h>
#include <stddef.h>
#include <stdint.h>

#include <algorithm>
#include <atomic>
#include <string>
#include <vector>

#include "absl/strings/string_view.h"
#include "absl/strings/strip.h"
#include "absl/time/clock.h"
#include "absl/time/time.h"
#include "constants.h"
#include "sampler.h"
#include "src/core/lib/channel/channel_stack.h"

namespace grpc_observability {

namespace {
std::atomic<bool> g_python_census_stats_enabled(false);
std::atomic<bool> g_python_census_tracing_enabled(false);
}  // namespace

// Enables/Disables Python census stats/tracing. It's only safe to do at the
// start of a program, before any channels/servers are built.
void EnablePythonCensusStats(bool enable);
void EnablePythonCensusTracing(bool enable);
// Gets the current status of Python OpenCensus stats/tracing
bool PythonCensusStatsEnabled();
bool PythonCensusTracingEnabled();

static constexpr size_t kTraceIdSize = 16;
static constexpr size_t kSpanIdSize = 8;

constexpr uint8_t kVersionId = 0;
constexpr uint8_t kTraceIdField = 0;
constexpr uint8_t kSpanIdField = 1;
constexpr uint8_t kTraceOptionsField = 2;

constexpr int kVersionLen = 1;
constexpr int kTraceIdLen = 16;
constexpr int kSpanIdLen = 8;
constexpr int kTraceOptionsLen = 1;

constexpr int kVersionOfs = 0;
constexpr int kTraceIdOfs = 1;
constexpr int kSpanIdOfs = kTraceIdOfs + 1 + kTraceIdLen;
constexpr int kTraceOptionsOfs = kSpanIdOfs + 1 + kSpanIdLen;

static constexpr size_t kSizeTraceID = 16;
static constexpr size_t kSizeSpanID = 8;
static constexpr size_t kSizeTraceOptions = 1;

// The length of the grpc-trace-bin value:
//      1 (version)
//   +  1 (trace_id field)
//   + 16 (length of trace_id)
//   +  1 (span_id field)
//   +  8 (span_id length)
//   +  1 (trace_options field)
//   +  1 (trace_options length)
//   ----
//     29
constexpr int kGrpcTraceBinHeaderLen =
    kVersionLen + 1 + kTraceIdLen + 1 + kSpanIdLen + 1 + kTraceOptionsLen;

struct Tag {
  std::string key;
  std::string value;
};

struct Label {
  Label() {}
  Label(std::string k, std::string v) : key(k), value(v) {}
  std::string key;
  std::string value;
};

union MeasurementValue {
  double value_double;
  int64_t value_int;
};

struct Measurement {
  MetricsName name;
  MeasurementType type;
  MeasurementValue value;
  bool registered_method;
  bool include_exchange_labels;
};

struct Annotation {
  std::string time_stamp;
  std::string description;
};

struct SpanCensusData {
  std::string name;
  std::string start_time;
  std::string end_time;
  std::string trace_id;
  std::string span_id;
  std::string parent_span_id;
  std::string status;
  std::vector<Label> span_labels;
  std::vector<Annotation> span_annotations;
  int64_t child_span_count;
  bool should_sample;
};

// SpanContext is associated with span to help manage the current context of a
// span. It's created when creating a new Span and will be destroyed together
// with associated Span.
class SpanContext final {
 public:
  SpanContext() : is_valid_(false) {}

  SpanContext(const std::string& trace_id, const std::string& span_id,
              bool should_sample)
      : trace_id_(trace_id),
        span_id_(span_id),
        should_sample_(should_sample),
        is_valid_(true) {}

  // Returns the TraceId associated with this SpanContext.
  std::string TraceId() const { return trace_id_; }

  // Returns the SpanId associated with this SpanContext.
  std::string SpanId() const { return span_id_; }

  bool IsSampled() const { return should_sample_; }

  bool IsValid() const { return is_valid_; }

 private:
  std::string trace_id_;
  std::string span_id_;
  bool should_sample_;
  bool is_valid_;
};

// Span is associated with PythonCensusContext to help manage tracing related
// data. It's created by calling StartSpan and will be destroyed together with
// associated PythonCensusContext.
class Span final {
 public:
  explicit Span(const std::string& name, const std::string& parent_span_id,
                absl::Time start_time, const SpanContext& context)
      : name_(name),
        parent_span_id_(parent_span_id),
        start_time_(start_time),
        context_(context) {}

  void End() { end_time_ = absl::Now(); }

  void IncreaseChildSpanCount() { ++child_span_count_; }

  static Span StartSpan(absl::string_view name, const Span* parent);

  static Span StartSpan(absl::string_view name,
                        const SpanContext& parent_context);

  static Span StartSpan(absl::string_view name, absl::string_view trace_id);

  static Span BlankSpan() { return StartSpan("", ""); }

  const SpanContext& Context() const { return context_; }

  void SetStatus(absl::string_view status);

  void AddAttribute(absl::string_view key, absl::string_view value);

  void AddAnnotation(absl::string_view description);

  SpanCensusData ToCensusData() const;

 private:
  static bool ShouldSample(const std::string& trace_id) {
    return ProbabilitySampler::Get().ShouldSample(trace_id);
  }

  std::string name_;
  std::string parent_span_id_;
  absl::Time start_time_;
  absl::Time end_time_;
  std::string status_;
  std::vector<Label> span_labels_;
  std::vector<Annotation> span_annotations_;
  SpanContext context_;
  uint64_t child_span_count_ = 0;
};

// PythonCensusContext is associated with each clientCallTracer,
// clientCallAttemptTracer and ServerCallTracer to help manage the span,
// spanContext and labels for each tracer. Create a new PythonCensusContext will
// always result in creating a new span (and a new SpanContext for that span).
// It's created during callTracer initialization and will be destroyed after
// the destruction of each callTracer.
class PythonCensusContext {
 public:
  PythonCensusContext() : span_(Span::BlankSpan()), labels_({}) {}

  explicit PythonCensusContext(absl::string_view name)
      : span_(Span::StartSpan(name, nullptr)), labels_({}) {}

  PythonCensusContext(absl::string_view name, absl::string_view trace_id)
      : span_(Span::StartSpan(name, trace_id)), labels_({}) {}

  PythonCensusContext(absl::string_view name, const SpanContext& parent_context)
      : span_(Span::StartSpan(name, parent_context)), labels_({}) {}

  PythonCensusContext(absl::string_view name, const Span* parent,
                      const std::vector<Label>& labels)
      : span_(Span::StartSpan(name, parent)), labels_(labels) {}

  // For attempt Spans only
  PythonCensusContext(absl::string_view name, const Span* parent)
      : span_(Span::StartSpan(name, parent)), labels_({}) {}

  Span& GetSpan() { return span_; }
  std::vector<Label>& Labels() { return labels_; }  // Only used for metrics
  const SpanContext& GetSpanContext() const { return span_.Context(); }

  void AddSpanAttribute(absl::string_view key, absl::string_view attribute) {
    span_.AddAttribute(key, attribute);
  }

  void AddSpanAnnotation(absl::string_view description) {
    span_.AddAnnotation(description);
  }

  void IncreaseChildSpanCount() { span_.IncreaseChildSpanCount(); }

  void EndSpan() { GetSpan().End(); }

 private:
  grpc_observability::Span span_;
  std::vector<Label> labels_;
};

// Creates a new client context that is by default a new root context.
// If the current context is the default context then the newly created
// span automatically becomes a root span. This should only be called with a
// blank CensusContext as it overwrites it.
void GenerateClientContext(absl::string_view method, absl::string_view trace_id,
                           absl::string_view parent_span_id,
                           PythonCensusContext* context);

// Deserialize the incoming SpanContext and generate a new server context based
// on that. This new span will never be a root span. This should only be called
// with a blank CensusContext as it overwrites it.
void GenerateServerContext(absl::string_view header, absl::string_view method,
                           PythonCensusContext* context);

inline std::string GetMethod(const char* method) {
  if (std::string(method).empty()) {
    return "";
  }
  // Check for leading '/' and trim it if present.
  return std::string(absl::StripPrefix(method, "/"));
}

inline std::string GetTarget(const char* target) { return std::string(target); }

// Fills a pre-allocated buffer with the value for the grpc-trace-bin header.
// The buffer must be at least kGrpcTraceBinHeaderLen bytes long.
void ToGrpcTraceBinHeader(const PythonCensusContext& ctx, uint8_t* out);

// Parses the value of the binary grpc-trace-bin header, returning a
// SpanContext. If parsing fails, IsValid will be false.
//
// Example value, hex encoded:
//   00                               (version)
//   00                               (trace_id field)
//   12345678901234567890123456789012 (trace_id)
//   01                               (span_id field)
//   0000000000003039                 (span_id)
//   02                               (trace_options field)
//   01                               (options: enabled)
//
// See also:
// https://github.com/census-instrumentation/opencensus-specs/blob/master/encodings/BinaryEncoding.md
SpanContext FromGrpcTraceBinHeader(absl::string_view header);

// Serializes the outgoing trace context. tracing_buf must be
// opencensus::trace::propagation::kGrpcTraceBinHeaderLen bytes long.
size_t TraceContextSerialize(const PythonCensusContext& context,
                             char* tracing_buf, size_t tracing_buf_size);

// Serializes the outgoing stats context.  Field IDs are 1 byte followed by
// field data. A 1 byte version ID is always encoded first. Tags are directly
// serialized into the given grpc_slice.
size_t StatsContextSerialize(size_t max_tags_len, grpc_slice* tags);

// Deserialize incoming server stats. Returns the number of bytes deserialized.
size_t ServerStatsDeserialize(const char* buf, size_t buf_size,
                              uint64_t* server_elapsed_time);

// Serialize outgoing server stats. Returns the number of bytes serialized.
size_t ServerStatsSerialize(uint64_t server_elapsed_time, char* buf,
                            size_t buf_size);

// Returns the incoming data size from the grpc call final info.
uint64_t GetIncomingDataSize(const grpc_call_final_info* final_info);

// Returns the outgoing data size from the grpc call final info.
uint64_t GetOutgoingDataSize(const grpc_call_final_info* final_info);

}  // namespace grpc_observability

#endif  // GRPC_PYTHON_OBSERVABILITY_H
