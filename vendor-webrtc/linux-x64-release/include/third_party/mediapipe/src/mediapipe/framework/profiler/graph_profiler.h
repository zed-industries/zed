// Copyright 2019 The MediaPipe Authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#ifndef MEDIAPIPE_FRAMEWORK_PROFILER_GRAPH_PROFILER_H_
#define MEDIAPIPE_FRAMEWORK_PROFILER_GRAPH_PROFILER_H_

#include <atomic>
#include <cstddef>
#include <cstdint>
#include <memory>
#include <set>
#include <string>
#include <vector>

#include "absl/time/time.h"
#include "mediapipe/framework/calculator.pb.h"
#include "mediapipe/framework/calculator_context.h"
#include "mediapipe/framework/calculator_profile.pb.h"
#include "mediapipe/framework/deps/clock.h"
#include "mediapipe/framework/deps/monotonic_clock.h"
#include "mediapipe/framework/executor.h"
#include "mediapipe/framework/profiler/graph_tracer.h"
#include "mediapipe/framework/profiler/sharded_map.h"
#include "mediapipe/framework/validated_graph_config.h"

namespace mediapipe {

class GlProfilingHelper;

struct PacketId {
  // Stream name, excluding TAG if available.
  std::string stream_name;
  // Timestamp of the packet.
  int64_t timestamp_usec;

  bool operator==(const PacketId& other) const {
    return (stream_name == other.stream_name) &&
           (timestamp_usec == other.timestamp_usec);
  }
};

struct PacketInfo {
  // Number of remained consumer of this packet.
  // This is used to decide if this PacketInfo should be discarded.
  int64_t remaining_consumer_count;
  // Packet production time based on profiler's clock.
  int64_t production_time_usec;
  // The time when the Process(), that generated the corresponding source
  // packet, was started.
  int64_t source_process_start_usec;

  // For testing.
  bool operator==(const PacketInfo& other) const {
    return (remaining_consumer_count == other.remaining_consumer_count) &&
           (production_time_usec == other.production_time_usec) &&
           (source_process_start_usec == other.source_process_start_usec);
  }
};

// For testing
class GraphProfilerTestPeer;

// GraphProfiler::CaptureProfile option, see the method for details.
enum class PopulateGraphConfig { kNo, kFull };

// GraphProfiler keeps track of the following in microseconds based on the
// profiler clock, for each calculator
// - Open(), Process(), and Close() runtime.
// - Input stream latency: Time from when a packet was produced to when it was
// consumed by the calculator.
// - Process input latency: How long it took a packet to travel from start of
// the graph (source nodes) to reach the Calculator.
// - Process input latency: Process input latency + process runtime for a
// packet.
//
// The profiler can be configured in the graph definition:
//   profiler_config {
//     histogram_interval_size_usec : 2000000
//     num_histogram_intervals : 5
//     enable_profiler: true
//   }
//
// Because the graph definition affects the stream profiling and the profiler is
// singleton, the profiler can not be used with more than one graph. Thus the
// profiler disables itself and returns an empty stub if Initialize() is called
// more than once.
//
// The profiler uses the synchronized monotonic clock by default.
// The client can overwrite this by calling SetClock().
class GraphProfiler : public std::enable_shared_from_this<ProfilingContext> {
 public:
  GraphProfiler();
  ~GraphProfiler();

  // Not copyable or movable.
  GraphProfiler(const GraphProfiler&) = delete;
  GraphProfiler& operator=(const GraphProfiler&) = delete;

  // Initializes the profiler based on the input config.
  // This should be called before adding any calculator to the profiler.
  //
  // Because the graph definition affects the stream profiling and the profiler
  // is singleton, the profiler can not be used with more than one graph. Thus
  // the profiler disables itself and returns an empty stub if Initialize() is
  // called more than once.
  void Initialize(const ValidatedGraphConfig& validated_graph_config)
      ABSL_LOCKS_EXCLUDED(profiler_mutex_);

  // Sets the profiler clock.
  void SetClock(const std::shared_ptr<mediapipe::Clock>& clock)
      ABSL_LOCKS_EXCLUDED(profiler_mutex_);

  // Gets the profiler clock.
  const std::shared_ptr<mediapipe::Clock> GetClock() const
      ABSL_LOCKS_EXCLUDED(profiler_mutex_);

  // Pauses profiling. No-op if already paused.
  void Pause();
  // Resumes profiling. No-op if already profiling.
  void Resume();
  // Resets cumulative profiling data. This only resets the information about
  // Process() and does NOT affect information for Open() and Close() methods.
  void Reset() ABSL_LOCKS_EXCLUDED(profiler_mutex_);
  // Begins profiling for a single graph run.
  absl::Status Start(mediapipe::Executor* executor);
  // Ends profiling for a single graph run.
  absl::Status Stop();

  // Record a tracing event.
  void LogEvent(const TraceEvent& event);

  // Collects the runtime profile for Open(), Process(), and Close() of each
  // calculator in the graph. May be called at any time after the graph has been
  // initialized.
  absl::Status GetCalculatorProfiles(std::vector<CalculatorProfile>*) const
      ABSL_LOCKS_EXCLUDED(profiler_mutex_);

  // Records recent profiling and tracing data.  Includes events since the
  // previous call to CaptureProfile.
  //
  // If `populate_config` is `kFull`, `config` field of the resulting profile
  // will contain canonicalized config of the profiled graph, and
  // `graph_trace.calculator_name` will contain node names referring to that
  // config. Both fields are left empty if the option is set to `kNo`.
  absl::Status CaptureProfile(
      GraphProfile* result,
      PopulateGraphConfig populate_config = PopulateGraphConfig::kNo);

  // Writes recent profiling and tracing data to a file specified in the
  // ProfilerConfig.  Includes events since the previous call to WriteProfile.
  absl::Status WriteProfile();

  // Returns the trace event buffer.
  GraphTracer* tracer() { return packet_tracer_.get(); }

  // Creates and returns a GlProfilingHelper interface for a single GLContext.
  std::unique_ptr<GlProfilingHelper> CreateGlProfilingHelper();

  // Convenience temporary object to record scoped entry and exit.
  // Gets start_time_usec_ on construction and records process runtime on
  // destruction. The |calculator_context| and |profiler| must not be null.
  class Scope {
   public:
    // Constructs a scope.
    //
    // REQUIRES: `calculator_context` and `profiler` are not null, and must both
    // outlive this instance.
    inline explicit Scope(GraphTrace::EventType event_type,
                          CalculatorContext* calculator_context,
                          GraphProfiler* profiler)
        : calculator_method_(event_type),
          calculator_context_(*calculator_context),
          profiler_(profiler) {
      start_time_usec_ = profiler_->TimeNowUsec();
      if (profiler_->is_tracing_) {
        absl::Time time_now = absl::FromUnixMicros(start_time_usec_);
        profiler_->packet_tracer_->LogInputEvents(
            calculator_method_, &calculator_context_, time_now);
      }
    }

    inline ~Scope() {
      int64_t end_time_usec;
      if (profiler_->is_profiling_ || profiler_->is_tracing_) {
        end_time_usec = profiler_->TimeNowUsec();
      }
      if (profiler_->is_profiling_) {
        int64_t end_time_usec = profiler_->TimeNowUsec();
        switch (calculator_method_) {
          case GraphTrace::OPEN:
            profiler_->SetOpenRuntime(calculator_context_, start_time_usec_,
                                      end_time_usec);
            break;

          case GraphTrace::PROCESS:
            profiler_->AddProcessSample(calculator_context_, start_time_usec_,
                                        end_time_usec);
            break;

          case GraphTrace::CLOSE:
            profiler_->SetCloseRuntime(calculator_context_, start_time_usec_,
                                       end_time_usec);
            break;
          default:
            break;
        }
      }
      if (profiler_->is_tracing_) {
        absl::Time time_now = absl::FromUnixMicros(end_time_usec);
        profiler_->packet_tracer_->LogOutputEvents(
            calculator_method_, &calculator_context_, time_now);
      }
    }

   private:
    const GraphTrace::EventType calculator_method_;
    const CalculatorContext& calculator_context_;
    GraphProfiler* profiler_;
    int64_t start_time_usec_;
  };

  const ProfilerConfig& profiler_config() { return profiler_config_; }

  // Helper method to expose the config to other profilers.
  const ValidatedGraphConfig* GetValidatedGraphConfig() {
    return validated_graph_;
  }

  // Gets a numerical identifier for this GraphProfiler object.
  uint64_t GetGraphId() { return graph_id_; }

 private:
  // This can be used to add packet info for the input streams to the graph.
  // It treats the stream defined by |stream_name| as a stream produced by a
  // source calculator and thus uses |timestamp_usec| for the packet production
  // time and source production time.
  // It is the responsibility of the caller to make sure the |timestamp_usec|
  // is valid for profiling.
  void AddPacketInfo(const TraceEvent& packet_info)
      ABSL_LOCKS_EXCLUDED(profiler_mutex_);
  static void InitializeTimeHistogram(int64_t interval_size_usec,
                                      int64_t num_intervals,
                                      TimeHistogram* histogram);
  static void ResetTimeHistogram(TimeHistogram* histogram);
  // Add a sample to a time histogram.
  static void AddTimeSample(int64_t start_time_usec, int64_t end_time_usec,
                            TimeHistogram* histogram);

  // Add output streams to the stream consumer count map.
  // This is neeeded in case an output stream is not consumed by any calculator.
  void InitializeOutputStreams(const CalculatorGraphConfig::Node& node_config);
  // Initializes input stream profiles for a calculator by adding all the input
  // streams.
  // Although this adds back edges to the profile to keep the ordering, it does
  // not add them to |stream_consumer_counts_| to avoid using them for updating
  // |source_process_start_usec| and garbage collection while profiling.
  void InitializeInputStreams(const CalculatorGraphConfig::Node& node_config,
                              int64_t interval_size_usec, int64_t num_intervals,
                              CalculatorProfile* calculator_profile);
  // Returns the input stream back edges for a calculator.
  std::set<int> GetBackEdgeIds(const CalculatorGraphConfig::Node& node_config,
                               const tool::TagMap& input_tag_map);

  void AddPacketInfoInternal(const PacketId& packet_id,
                             int64_t production_time_usec,
                             int64_t source_process_start_usec);
  // Adds packet info for non-empty output packets.
  void AddPacketInfoForOutputPackets(
      const OutputStreamShardSet& output_stream_shard_set,
      int64_t production_time_usec, int64_t source_process_start_usec);

  // Updates the production time for outputs and the stream profile for inputs.
  int64_t AddStreamLatencies(const CalculatorContext& calculator_context,
                             int64_t start_time_usec, int64_t end_time_usec,
                             CalculatorProfile* calculator_profile);

  void SetOpenRuntime(const CalculatorContext& calculator_context,
                      int64_t start_time_usec, int64_t end_time_usec)
      ABSL_LOCKS_EXCLUDED(profiler_mutex_);
  void SetCloseRuntime(const CalculatorContext& calculator_context,
                       int64_t start_time_usec, int64_t end_time_usec)
      ABSL_LOCKS_EXCLUDED(profiler_mutex_);

  // Updates the input streams profiles for the calculator and returns the
  // minimum |source_process_start_usec| of all input packets, excluding empty
  // packets and back-edge packets. Returns -1 if there is no input packets.
  int64_t AddInputStreamTimeSamples(const CalculatorContext& calculator_context,
                                    int64_t start_time_usec,
                                    CalculatorProfile* calculator_profile);

  // Updates the Process() data for calculator.
  // Requires ReaderLock for is_profiling_.
  void AddProcessSample(const CalculatorContext& calculator_context,
                        int64_t start_time_usec, int64_t end_time_usec)
      ABSL_LOCKS_EXCLUDED(profiler_mutex_);

  // Helper method to get trace_log_path.  If the trace_log_path is empty and
  // tracing is enabled, this function returns a default platform dependent
  // trace_log_path.
  absl::StatusOr<std::string> GetTraceLogPath();

  // Helper method to get the clock time in microsecond.
  int64_t TimeNowUsec() { return ToUnixMicros(clock_->TimeNow()); }

 private:
  // The settings for this tracer.
  ProfilerConfig profiler_config_;

  // If true, the profiler has already been initialized and should not be
  // initialized again.
  std::atomic_bool is_initialized_;

  // If true, the profiler is profiling. Otherwise, it is paused.
  std::atomic_bool is_profiling_;

  // If true, the tracer records timing events.
  std::atomic_bool is_tracing_;

  // Stores all the calculator profiles with the calculator name as the key.
  using CalculatorProfileMap = ShardedMap<std::string, CalculatorProfile>;
  CalculatorProfileMap calculator_profiles_;
  // Stores the production time of a packet, based on profiler's clock.
  using PacketInfoMap =
      ShardedMap<std::string, std::list<std::pair<int64_t, PacketInfo>>>;
  PacketInfoMap packets_info_;

  // Global mutex for the profiler.
  mutable absl::Mutex profiler_mutex_;

  // Buffer of recent profile trace events.
  std::unique_ptr<GraphTracer> packet_tracer_;

  // The clock for time measurement, which must be a monotonic real time clock.
  std::shared_ptr<mediapipe::Clock> clock_;

  // Inidicates that profiling has started and not yet stopped.
  std::atomic_bool is_running_;

  // The end time of the previous output log.
  absl::Time previous_log_end_time_;

  // The index number of the previous output log.
  std::atomic<int> previous_log_index_;

  // The configuration for the graph being profiled.
  const ValidatedGraphConfig* validated_graph_;

  // A private resource for creating GraphProfiles.
  class GraphProfileBuilder;
  std::unique_ptr<GraphProfileBuilder> profile_builder_;

  // The globally incrementing identifier for all graphs in a process.
  static inline std::atomic_int next_instance_id_ = 0;

  // A unique identifier for this object. Only unique within a process.
  uint64_t graph_id_;

  // For testing.
  friend GraphProfilerTestPeer;
};

// The API class used to access the preferred profiler, such as
// GraphProfiler or GraphProfilerStub.  ProfilingContext is defined as
// a class rather than a typedef in order to support clients that refer
// to it only as a forward declaration, such as CalculatorState.
class ProfilingContext : public GraphProfiler {
  using GraphProfiler::GraphProfiler;
};

// For now, OSS always uses GlContextProfilerStub.
// TODO: Switch to GlContextProfiler when GlContext is moved to OSS.
#define MEDIAPIPE_DISABLE_GPU_PROFILER 1

// GlContextProfiler keeps track of all timestamp queries within a specific
// GlContext object. When created, the GlContextProfiler must be initialized
// before marking timestamps. Finally, when GlContext is no longer interested
// in marking timestamps or is about to be destroyed, Finish() must be called
// to complete all pending time queries and detach the timer from the GlContext.
// Note that the GlContextProfiler must be created and initialized within a
// valid GlContext object.
#if !MEDIAPIPE_DISABLE_GPU_PROFILER
class GlContextProfiler {
 public:
  explicit GlContextProfiler(
      std::shared_ptr<ProfilingContext> profiling_context)
      : profiling_context_(profiling_context) {}

  // Not copyable or movable.
  GlContextProfiler(const GlContextProfiler&) = delete;
  GlContextProfiler& operator=(const GlContextProfiler&) = delete;

  // Add a GlTimingInfo object to the collection of pending timestamp queries
  // associated with a specific graph node_id, packet input_timestamp and mark
  // if it is a start or stop event. When a stop event is marked, this function
  // blocks on the corresponding start event to complete.
  void MarkTimestamp(int node_id, Timestamp input_timestamp, bool is_finish);

  // Complete all pending timing queries and detach the timer from the
  // GlContext.
  void LogAllTimestamps();

 private:
  // Store GlTimeQuery and the corresponding TraceEvent object that should be
  // populated when the query completes together.
  struct GlTimingInfo {
    GlTimeQuery time_query;
    TraceEvent trace_event;
  };

  // Setup the timer for marking GPU timestamps. If successful in setup, return
  // true otherwise return false to indicate that timing measurment is not
  // supported.
  bool Initialize();

  absl::Time TimeNow();

  // Calibrate the GPU timer w.r.t. the CPU clock. If calibration is fails,
  // timing_measurement_supported_ is set to false.
  void CalibrateTimer(bool recalibrate);

  // Log a TraceEvent object to represent if the GPU calibration period has
  // started or just ended.
  void LogCalibrationEvent(bool started, absl::Time time);

  // Log TraceEvent objects for completed time queries. If the parameter wait is
  // set to true, wait for all time queries to complete before returning.
  void RetireReadyGlTimings(bool wait = false);

  // Get the TraceEvent object containing the timestamp recorded by the GPU if
  // the provided query was fulfilled. If it is still pending and wait is false,
  // return absl::nullopt.
  absl::optional<TraceEvent> GetTimeFromQuery(
      std::unique_ptr<GlTimingInfo>& query, bool wait);

  std::shared_ptr<ProfilingContext> profiling_context_;
  GlSimpleTimer gl_timer_;
  bool checked_timing_supported_ = false;
  bool timing_measurement_supported_ = false;
  std::deque<std::unique_ptr<GlTimingInfo>> pending_gl_times_;
  std::unique_ptr<GlTimingInfo> gl_start_query_;
};

// The API class used to access the preferred GlContext profiler, such as
// GlContextProfiler or GlContextProfilerStub. GlProfilingHelper is defined as
// a class rather than a typedef in order to support clients that refer
// to it only as a forward declaration.
class GlProfilingHelper : public GlContextProfiler {
  using GlContextProfiler::GlContextProfiler;
};
#else   // MEDIAPIPE_DISABLE_GPU_PROFILER
class GlContextProfilerStub {
 public:
  explicit GlContextProfilerStub(
      std::shared_ptr<ProfilingContext> profiling_context) {}
  // Not copyable or movable.
  GlContextProfilerStub(const GlContextProfilerStub&) = delete;
  GlContextProfilerStub& operator=(const GlContextProfilerStub&) = delete;
  bool Initialze() { return false; }
  void MarkTimestamp(int node_id, Timestamp input_timestamp, bool is_finish) {}
  void LogAllTimestamps() {}
};
class GlProfilingHelper : public GlContextProfilerStub {
  using GlContextProfilerStub::GlContextProfilerStub;
};
#endif  // !MEDIAPIPE_DISABLE_GPU_PROFILER
#undef MEDIAPIPE_DISABLE_GPU_PROFILER
}  // namespace mediapipe

#endif  // MEDIAPIPE_FRAMEWORK_PROFILER_GRAPH_PROFILER_H_
