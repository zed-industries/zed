// Copyright 2022 The Centipede Authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

// TODO(ussuri): Upgrade to optionally measure the metrics of a given thread,
//  not the entire process (available via /proc/self/tasks/<tid>/<file>).

//------------------------------------------------------------------------------
//                                RUsageProfiler
//
// A profiler for the current process's timing and system memory usage. Unlike
// external sampling profilers that require code instrumentation and slow
// profiling runs, RUsageProfiler's stat collection and reporting are
// permanently compiled into the client's code, consume no additional CPU cycles
// or RAM when idle and very few additional cycles or RAM when active, and can
// be activated at any time, e.g. by simply passing a flag.
//
// Another difference is that RUsageProfiler doesn't just track timing, but
// other system resource usage as well, such as several types of the process's
// memory.
//
// While traditional profilers report performance of functions, the focus of
// RUsageProfiler is performance of higher-level logical units of processing
// that can both be smaller than a single function or span multiple functions,
// classes, and modules.
//
// To achieve that, profiling metrics collection is explicit, intrusive and
// driven entirely by client code: RUsageProfiler profiles only the bits of
// logic it is told to profile, rather than every function call
// indiscriminately. The downside is that client code requires explicit
// profiling statements. The upside is that the client gets a very different
// level of control, as well as a differently structured and differently focused
// resource usage stats, unavailable with external sampling profilers: for
// example, RUsageProfiler makes it very easy to measure the overall resource
// usage dynamics of a complicated code path regardless of what function calls
// it makes or which external libraries it uses, or to print a comparison
// diagram of resource usage by different stages of a multi-stage processor
// module.
//
// Resource usage collection is available via one of or a combination of:
// 1) Explicit snapshots at specific execution checkpoints.
// 2) An asynchronous timelapse sequence of automatic snapshots taken at
//    regular intervals between two checkpoints.
//
//                                BASIC USAGE
//
// At construction, RUsageProfiler ctor records and logs an initial snapshot of
// the metrics requested by the client; at destruction, the dtor logs the
// current resource usage and the delta from the time of construction.
//
//                               ADVANCED USAGE
//
// Additional intermediate snapshots can be recorded at a client's request.
// These snapshots can also be immediately logged, with both the absolute and
// delta metrics printed.
//
// A final chronological report of the resource usage can also be generated and
// logged. The report contains each tracked metric's snapshot history over this
// RUsageProfiler object's lifetime. Each snapshot is annotated with the source
// location and time. The metric values are printed in the numeric and
// pseudo-graphical form (as a progress-like bar representing the value relative
// to its overall observed range).
//
//                             TIMELAPSE PROFILING
//
// RUsageProfiler also supports a limited timelapse mode. In contrast to
// traditional sampling profilers, it simply takes snapshots of resource usage
// at regular intervals, but doesn't collect per-function call usage stats.
//
// This lends it particularly useful for detection of problematic resource usage
// patterns in a blackbox or near-blackbox external API calls or modules,
// such as third-party libs, or measuring the effect of differently tuned
// parameter configurations on the performance profile of complicated,
// multi-function, multi-class, or multi-module pieces of logic.
//
//                              MULTI-THREADING
//
// RUsageProfiler is thread-safe.
//
// Keep in mind that this is a process-scoped profiler, not thread-scoped: it
// records and reports the current _process's_ timing and resource usage, not
// the current _thread's_. This means that in a multi-threaded context, the
// numbers recorded by each snapshot will reflect the timing and memory used up
// by _all_ the threads of the process. For example, if a thread pool executes
// the same profiled function in N threads, the stats reported by the
// function-level profiler may be erratic and not very indicative of the
// function's true performance.
//
//                          EXAMPLE USAGE - DIRECT
//
//  void foo() {
//    // Logs the initial snapshot:
//    RUsageProfiler profiler{kAllMetrics, ABSL_LOC, __func__};
//    ...
//    profiler.TakeSnapshot(ABSL_LOC);  // Takes another snapshot
//    ...
//    profiler.TakeSnapshot(ABSL_LOC).Log();  // Records and logs a snapshot
//    ...
//    VLOG(1) << profiler.TakeSnapshot(ABSL_LOC);  // A different way to log
//    ...
//  }  // Dtor logs a final snapshot
//
//                           EXAMPLE USAGE - MACROS
//
//  void foo() {
//    RPROF_THIS_FUNCTION(VLOG_IS_ON(2));  // Profile the function @ --v>=2
//    ...
//    RPROF_SNAPSHOT_AND_LOG();  // Record and log a function-level snapshot
//    for (...) {
//        RPROF_THIS_SCOPE(VLOG_IS_ON(3));  // Profile loop iterations @ --v>=3
//        ...
//    }
//    RPROF_SNAPSHOT();  // Record (not log) another function-level snapshot
//  }  // Dtor logs a final snapshot and a chronological report
//
//                              EXAMPLE SNAPSHOTS
//
// clang-format off
//  I1105 16:52:20.831313  932765 foo.cc:79] PROFILER [P.1 DoSomethingFn()] SNAPSHOT [S.0 INITIAL]:  // NOLINT
//    [P.1:S.0] TIMING   | Wall:          11us | User:           2us | Sys:            4us | CpuUtil:      8.11% | CpuCores:       0.6 |  // NOLINT
//    [P.1:S.0] MEMORY   | RSS:        119.79M | VSize:        2.04G | VPeak:        2.04G | Data:       152.45M | ShMem:       12.83M |  // NOLINT
//  I1105 16:52:38.130159  932926 foo.cc:119] PROFILER [P.2 Heartbeats] SNAPSHOT [S.1 Timelapse]:  // NOLINT
//    [P.6:S.4] TIMING   | Wall:         3.00s | User:          23ms | Sys:           71ms | CpuUtil:      0.00% | CpuCores:       0.0 |  // NOLINT
//    [P.6:S.4] MEMORY   | RSS:        146.48M | VSize:        2.05G | VPeak:        2.05G | Data:       298.46M | ShMem:       12.85M |  // NOLINT
//  I1105 16:52:23.880263  932765 foo.cc:82] PROFILER [P.1 DoSomethingFn()] SNAPSHOT [S.1 Scope 1 done]:  // NOLINT
//    [P.1:S.1] TIMING   | Wall:         3.05s | User:          16ms | Sys:           33ms | CpuUtil:      0.00% | CpuCores:       0.0 |  // NOLINT
//    [P.1:S.1] TIMING Δ | Wall:        +3.05s | User:         +16ms | Sys:          +33ms | CpuUtil:     -8.11% | CpuCores:      -0.5 |  // NOLINT
//    [P.1:S.1] MEMORY   | RSS:        167.75M | VSize:        2.04G | VPeak:        2.04G | Data:       200.58M | ShMem:       12.83M |  // NOLINT
//  I1105 16:52:26.913993  932765 foo.cc:89] PROFILER [P.1 DoSomethingFn()] SNAPSHOT [S.3 Loop iteration 0 done]:  // NOLINT
//    [P.1:S.3] TIMING   | Wall:         6.08s | User:          37ms | Sys:           46ms | CpuUtil:      0.00% | CpuCores:       0.0 |  // NOLINT
//    [P.1:S.3] MEMORY   | RSS:        148.27M | VSize:        2.04G | VPeak:        2.04G | Data:       200.70M | ShMem:       12.83M |  // NOLINT
//  I1105 16:52:43.133988  932926 foo.cc:119] PROFILER [P.2 Heartbeats] SNAPSHOT [S.2 Timelapse]:  // NOLINT
//    [P.6:S.9] TIMING   | Wall:         8.01s | User:          42ms | Sys:          103ms | CpuUtil:     18.95% | CpuCores:       0.0 |  // NOLINT
//    [P.6:S.9] TIMING Δ | Wall:        +1.00s | User:          +1ms | Sys:          +10ms | CpuUtil:     -0.49% | CpuCores:      -0.0 |  // NOLINT
//    [P.6:S.9] MEMORY   | RSS:        158.96M | VSize:        2.05G | VPeak:        2.05G | Data:       298.71M | ShMem:       12.85M |  // NOLINT
//  I1105 16:52:28.962669  932765 foo.cc:79] PROFILER [P.1 DoSomethingFn()] SNAPSHOT [S.6 FINAL]:  // NOLINT
//    [P.1:S.6] TIMING   | Wall:         8.13s | User:          54ms | Sys:           75ms | CpuUtil:     22.56% | CpuCores:       0.0 |  // NOLINT
//    [P.1:S.6] TIMING Δ | Wall:         +17ms | User:          +9ms | Sys:           +6ms | CpuUtil:    +22.56% | CpuCores:      +0.0 |  // NOLINT
//    [P.1:S.6] MEMORY   | RSS:        145.92M | VSize:        2.04G | VPeak:        2.04G | Data:       211.08M | ShMem:       12.83M |  // NOLINT
//    ...
// clang-format off
//
//                       EXAMPLE FINAL REPORT (TRUNCATED)
//
// clang-format off
//
//  I1105 16:52:28.963056  932765 foo.cc:79] PROFILER [P.1 WasteTimeAndGobbleBytes()] FINAL REPORT:  // NOLINT
//
//  === TIMING [P.1 DoSomethingFn()] ===
//
//  WALL TIME:
//    foo.cc:79 @ 16:52:20.83 [P.1:S.0 INITIAL         ]       11us [--------------------------------------------------]  // NOLINT
//    foo.cc:82 @ 16:52:23.88 [P.1:S.1 Scope 1         ]      3.05s [##################--------------------------------]  // NOLINT
//    foo.cc:86 @ 16:52:25.90 [P.1:S.2 Scope 2         ]      5.07s [###############################-------------------]  // NOLINT
//    foo.cc:89 @ 16:52:26.91 [P.1:S.3 Loop iteration 0]      6.08s [#####################################-------------]  // NOLINT
//    foo.cc:89 @ 16:52:27.92 [P.1:S.4 Loop iteration 1]      7.10s [###########################################-------]  // NOLINT
//    foo.cc:79 @ 16:52:28.96 [P.1:S.6 FINAL           ]      8.13s [##################################################]  // NOLINT
//  ... Same stats for other timing metrics.
//
//  === Δ TIMING [P.1 DoSomethingFn()] ===
//
//  Δ WALL TIME:
//    foo.cc:79 @ 16:52:20.83 [P.1:S.0 INITIAL         ]       +0ns [--------------------------------------------------]  // NOLINT
//    foo.cc:82 @ 16:52:23.88 [P.1:S.1 Scope 1         ]     +3.05s [##################################################]  // NOLINT
//    foo.cc:86 @ 16:52:25.90 [P.1:S.2 Scope 2         ]     +2.03s [#################################-----------------]  // NOLINT
//    foo.cc:89 @ 16:52:26.91 [P.1:S.3 Loop iteration 0]     +1.01s [################----------------------------------]  // NOLINT
//    foo.cc:89 @ 16:52:27.92 [P.1:S.4 Loop iteration 1]     +1.01s [################----------------------------------]  // NOLINT
//    foo.cc:79 @ 16:52:28.96 [P.1:S.6 FINAL           ]      +17ms [--------------------------------------------------]  // NOLINT
//  ... Same stats for other timing metrics.
//
//  === MEMORY USAGE [P.1 DoSomethingFn()] ===
//
//  RESIDENT SET SIZE:
//    foo.cc:79 @ 16:52:20.83 [P.1:S.0 INITIAL         ]    119.79M [--------------------------------------------------]  // NOLINT
//    foo.cc:82 @ 16:52:23.88 [P.1:S.1 Scope 1         ]    167.75M [##################################################]  // NOLINT
//    foo.cc:86 @ 16:52:25.90 [P.1:S.2 Scope 2         ]    139.66M [####################------------------------------]  // NOLINT
//    foo.cc:89 @ 16:52:26.91 [P.1:S.3 Loop iteration 0]    148.27M [#############################---------------------]  // NOLINT
//    foo.cc:89 @ 16:52:27.92 [P.1:S.4 Loop iteration 1]    156.07M [#####################################-------------]  // NOLINT
//    foo.cc:79 @ 16:52:28.96 [P.1:S.6 FINAL           ]    145.92M [###########################-----------------------]  // NOLINT
//  ... Same stats for other memory types.
//
//  === Δ MEMORY USAGE [P.1 DoSomethingFn()] ===
//
//  Δ RESIDENT SET SIZE:
//    foo.cc:79 @ 16:52:20.83 [P.1:S.0 INITIAL         ]        +0B [------------------|--------------------------------]  // NOLINT
//    foo.cc:82 @ 16:52:23.88 [P.1:S.1 Scope 1         ]    +47.95M [------------------|################################]  // NOLINT
//    foo.cc:86 @ 16:52:25.90 [P.1:S.2 Scope 2         ]    -28.09M [##################|--------------------------------]  // NOLINT
//    foo.cc:89 @ 16:52:26.91 [P.1:S.3 Loop iteration 0]     +8.62M [------------------|######--------------------------]  // NOLINT
//    foo.cc:89 @ 16:52:27.92 [P.1:S.4 Loop iteration 1]     +7.80M [------------------|#####---------------------------]  // NOLINT
//    foo.cc:79 @ 16:52:28.96 [P.1:S.6 FINAL           ]    -19.88M [-----#############|--------------------------------]  // NOLINT
//  ... Same stats for other memory types.
//
// clang-format on
//------------------------------------------------------------------------------

#ifndef THIRD_PARTY_CENTIPEDE_RUSAGE_PROFILER_H_
#define THIRD_PARTY_CENTIPEDE_RUSAGE_PROFILER_H_

#include <atomic>
#include <cstdint>
#include <deque>
#include <iosfwd>
#include <memory>
#include <ostream>
#include <string>
#include <string_view>

#include "absl/base/nullability.h"
#include "absl/base/thread_annotations.h"
#include "absl/strings/str_cat.h"  // IWYU pragma: keep
#include "absl/synchronization/mutex.h"
#include "absl/time/time.h"
#include "./centipede/periodic_action.h"
#include "./centipede/rusage_stats.h"

namespace fuzztest::internal {

// A simple source location wrapper. Typically, construct as
// `SourceLocation{__FILE__, __LINE__}` and pass around by-value.
// TODO(ussuri): Switch to absl::SourceLocation or std::source_location.
struct SourceLocation {
  explicit SourceLocation() = default;
  SourceLocation(absl::Nonnull<const char*> file, int line)
      : file{file}, line{line} {}

  const char* const file = "<unknown>";
  const int line = 0;
};

class RUsageProfiler {
 public:
  //----------------------------------------------------------------------------
  //                                 Types

  // A profiling snapshot.
  struct Snapshot {
    // Returns this snapshot's source location.
    std::string WhereStr() const;
    // Same as above, but shortens the file path to the basename.
    std::string ShortWhereStr() const;

    // Returns this snapshot's recording date/time in local timezone.
    std::string WhenStr() const;
    // Same as above, but omits the date.
    std::string ShortWhenStr() const;

    // Returns this snapshot's formatted metrics. The formatting is consistent
    // across snapshots, so if printed in a loop, these will form a table.
    std::string FormattedMetricsStr() const;
    // Same as above, but the metrics are printed in one line without the
    // table-like formatting.
    std::string ShortMetricsStr() const;

    // Logs this snapshot to LOG(INFO). The source location that annotates the
    // log message is set to `location` instead of the actual call's location.
    // Returns *this so clients can do either of
    //   Snapshot s = profiler.TakeSnapshot();
    //   Snapshot s = profiler.TakeSnapshot().Log();
    const Snapshot& Log() const;

    // Writes a short version of this snapshot to an ostream.
    friend std::ostream& operator<<(std::ostream&, const Snapshot&);

    // Metadata.
    const int64_t id = -1;
    const std::string title;
    const SourceLocation location{};
    const absl::Time time;

    // The parent profiler's data.
    const int profiler_id = -1;
    const std::string profiler_desc;

    // Recorded metrics.
    const RUsageTiming timing = RUsageTiming::Zero();
    const RUsageTiming delta_timing = RUsageTiming::Zero();
    const RUsageMemory memory = RUsageMemory::Zero();
    const RUsageMemory delta_memory = RUsageMemory::Zero();
  };

  // An abstract interface used to stream in a profiling report in
  // GenerateReport(). Also used inside PrintReport() to overcome the LOG()'s
  // limitation on the size of a single printed message.
  class ReportSink {
   public:
    virtual ~ReportSink() = default;
    virtual ReportSink& operator<<(std::string_view fragment) = 0;
  };

  //----------------------------------------------------------------------------
  //                                 APIs

  // Which metric categories to track and report.
  enum Metrics : unsigned {
    kMetricsOff = 0,
    kSnapTiming = 1,   // Timing at the time of snapshot
    kDeltaTiming = 2,  // Delta timing from the previous snapshot
    kTiming = kSnapTiming | kDeltaTiming,
    kSnapMemory = 4,   // Memory at the time of snapshot
    kDeltaMemory = 8,  // Delta memory from the previous snapshot
    kMemory = kSnapMemory | kDeltaMemory,
    kAllMetrics = kTiming | kMemory,
  };
  using MetricsMask = decltype(kMetricsOff | kMetricsOff);

  // Automatic logging enabled via RAII.
  enum RaiiActions : unsigned {
    kRaiiOff = 0,
    kCtorSnapshot = 1,
    kDtorSnapshot = 2,
    kDtorReport = 4,
    kFinalReport = kDtorReport,
    kRaiiSnapshots = kCtorSnapshot | kDtorSnapshot,
    kAllRaii = kRaiiSnapshots | kFinalReport
  };
  using RaiiActionsMask = decltype(kRaiiOff | kRaiiOff);

  // Initializes this profiler and possibly takes an initial snapshot if
  // raii_actions & kCtorSnapshot != 0. SourceLocation `location` parameter is
  // used to annotate this profiler's log messages with the source location of
  // the caller, as if the caller printed them. That makes it easy to attribute
  // the logged resource usage to the actual user rather than RUsageProfiler.
  RUsageProfiler(                     //
      RUsageScope scope,              // Which process/thread to monitor
      MetricsMask metrics,            // Which metrics to track
      RaiiActionsMask raii_actions,   // Which RAII logs to enable
      SourceLocation location,        // Pass SourceLocation{__FILE__, __LINE__}
      std::string description = "");  // Annotate logs in addition to ID

  // This version turns on all RAII logging and immediately initiates timelapse
  // snapshots at the specified interval, unless the interval is
  // absl::ZeroDuration or absl::InfiniteDuration.
  //
  // Dtor will stop taking snapshots and print a chronological report.
  // Snapshotting can also be manually stopped at any time using
  // StopTimelapse().
  //
  // As with manually started timelapse snapshotting (via StartTimelapse()),
  // the client can still request explicit snapshots at any time, interleaved
  // with timelapse ones.
  RUsageProfiler(                         //
      RUsageScope scope,                  // Which process/thread to monitor
      MetricsMask metrics,                // Which metrics to track
      absl::Duration timelapse_interval,  // Take timelapse snapshots this often
      bool also_log_timelapses,           // Log timelapse snapshots as taken
      SourceLocation location,            // SourceLocation{__FILE__, __LINE__}
      std::string description = "");      // Annotate logs in addition to ID

  // Logs the final report as returned by GenerateReport().
  ~RUsageProfiler();

  // Records and returns a snapshot of the current metrics. The snapshot's
  // source location is set to `location`, so its Log() will print a log message
  // as if it were emitted by the `location` source line. As such, the rule of
  // thumb should be to pass `SourceLocation{__FILE__, __LINE__}`. The returned
  // reference remains valid until RUsageProfiler is destroyed.
  const Snapshot& TakeSnapshot(SourceLocation loc, std::string title = "");

  // Starts taking and optionally also logging periodic snapshots at a given
  // interval in a separate thread.
  //
  // Convenient for measuring sample-based resource usage of a black-box
  // external API (e.g. third-party) or a complex bit of logic spanning multiple
  // functions/classes/modules in order to either detect problematic usage
  // patterns or the effect of different parameter configurations on the overall
  // performance. GenerateReport() or PrintReport() are particularly well-suited
  // for viewing the results of timelapse measurements in graphical form.
  //
  // The client is free to continue taking explicit snapshots at any time,
  // interleaved with timelapse ones.
  void StartTimelapse(          //
      SourceLocation loc,       //
      absl::Duration interval,  //
      bool also_log = false,    //
      std::string title = "");

  // Stops taking timelapse snapshots previously initiated by StartTimelapse().
  void StopTimelapse();

  // Returns a vector of manual and timelapse snapshots recorded so far.
  const std::deque<Snapshot>& GetSnapshots() const { return snapshots_; }

  // Prints to `sink` a report consisting of chronological charts for each of
  // the tracked metrics recorded since this profiler's construction up to this
  // point.
  void GenerateReport(absl::Nonnull<ReportSink*> report_sink) const;

  // Logs the report returned by GenerateReport(). The log message's source
  // location is set to `location`: as a rule of thumb, pass
  // `SourceLocation{__FILE__, __LINE__}` -- the explanation before
  // TakeSnapshot() does apply here.
  void PrintReport(SourceLocation loc, const std::string& title = "");

 private:
  friend class RUsageProfilerTest_ValidateManualSnapshots_Test;

  //----------------------------------------------------------------------------
  //                                  Data

  // Global instance counter.
  static std::atomic<int> next_id_;

  // Scope (the current process or the current thread).
  const RUsageScope scope_;
  // Metrics and report flavors to keep track of and print.
  const Metrics metrics_;
  // Enabled RAII actions.
  const RaiiActions raii_actions_;
  // The source location where this profiler got created, as recorded by ctor.
  const SourceLocation ctor_loc_;
  // The descriptive name of this profiler provided by the client. Used to
  // annotate verbose log messages.
  const std::string description_;
  // The sequential ID of this profiler. Used to annotate all log
  const int id_;

  // Mutex for the mutable data further below.
  mutable absl::Mutex mutex_;

  // Chronological snapshots. Using std::deque gives a better-than-vector
  // average insertion speed, preserves iterators across insertions, and strikes
  // a balance between vector's and list's additional storage.
  std::deque<Snapshot> snapshots_ ABSL_GUARDED_BY(mutex_);
  // A temporarily lived periodic action that records and optionally logs
  // timelapse snapshots. (Re)created by each new call to StartTimelapse() and
  // terminated by StopTimelapse() or the dtor, whichever comes first.
  std::unique_ptr<PeriodicAction> timelapse_recorder_ ABSL_GUARDED_BY(mutex_);

  // An auto-starting timer passed to RUsageTiming::Snapshot() in order to track
  // this RUsageProfiler object's lifetime stats rather than the process's
  // lifetime stats, which is the default.
  ProcessTimer timer_;
};

}  // namespace fuzztest::internal

//------------------------------------------------------------------------------
//               Convenience macros for easy use of RUsageProfiler
//------------------------------------------------------------------------------

// TODO(ussuri): The macros all use RUsageScope::ThisProcess(). Parameterize.

#define RPROF_NAME(prefix, line) RPROF_NAME_CONCAT(prefix, line)
#define RPROF_NAME_CONCAT(prefix, line) prefix##line
#define FUNCTION_LEVEL_RPROF_NAME RPROF_NAME(rprof_, 0)
#define SCOPE_LEVEL_RPROF_NAME RPROF_NAME(rprof_, __LINE__)

// Profile the timing and resource usage of the current function, with an option
// to take additional intermediate snapshots via RPROF_SNAPSHOT* later in the
// function.
//
// The intended canonical place to call this macro is right after the function's
// open brace or precondition checks: with just that, the entire function's
// system timing and resource usage will be logged upon return. Only one such
// macro call is allowed per function.
// clang-format off
#define RPROF_THIS_FUNCTION(enable)                                        \
  fuzztest::internal::RUsageProfiler FUNCTION_LEVEL_RPROF_NAME = {            \
      /*scope=*/fuzztest::internal::RUsageScope::ThisProcess(),               \
      /*metrics=*/(enable) ? fuzztest::internal::RUsageProfiler::kAllMetrics  \
                           : fuzztest::internal::RUsageProfiler::kMetricsOff, \
      /*raii_actions=*/fuzztest::internal::RUsageProfiler::kRaiiSnapshots,    \
      /*location=*/{__FILE__, __LINE__},                                   \
      /*description=*/absl::StrCat(__func__, "()")                         \
  }
// clang-format on

// Same as RPROF_THIS_FUNCTION, but with a full report printed at return from
// the function.
// clang-format off
#define RPROF_THIS_FUNCTION_WITH_REPORT(enable)                            \
  fuzztest::internal::RUsageProfiler FUNCTION_LEVEL_RPROF_NAME = {            \
      /*scope=*/fuzztest::internal::RUsageScope::ThisProcess(),               \
      /*metrics=*/(enable) ? fuzztest::internal::RUsageProfiler::kAllMetrics  \
                           : fuzztest::internal::RUsageProfiler::kMetricsOff, \
      /*raii_actions=*/fuzztest::internal::RUsageProfiler::kAllRaii,          \
      /*location=*/{__FILE__, __LINE__},                                   \
      /*description=*/absl::StrCat(__func__, "()")                         \
  }
// clang-format on

// Same as RPROF_THIS_FUNCTION, but immediately initiates timelapse snapshots
// at the specified `interval` and prints a final report for them. Additional
// snapshots can still be taken with RPROF_SNAPSHOT*.
// clang-format off
#define RPROF_THIS_FUNCTION_WITH_TIMELAPSE(                                \
    enable, timelapse_interval, also_log_timelapses)                       \
  fuzztest::internal::RUsageProfiler FUNCTION_LEVEL_RPROF_NAME = {            \
      /*scope=*/fuzztest::internal::RUsageScope::ThisProcess(),               \
      /*metrics=*/(enable) ? fuzztest::internal::RUsageProfiler::kAllMetrics  \
                           : fuzztest::internal::RUsageProfiler::kMetricsOff, \
      /*timelapse_interval=*/timelapse_interval,                           \
      /*also_log_timelapses=*/also_log_timelapses,                         \
      /*location=*/{__FILE__, __LINE__},                                   \
      /*description=*/absl::StrCat(__func__, "()")                         \
  }
// clang-format on

// Sets an existing RUsageProfiler as this function's profiler such that it can
// be used with `RPROF_SNAPSHOT` and other similar macros below, which normally
// work with the other `RPROF_THIS_FUNCTION.*` macros.
// clang-format off
#define RPROF_THIS_FUNCTION_BY_EXISTING_RPROF(profiler)                 \
  ::fuzztest::internal::RUsageProfiler& FUNCTION_LEVEL_RPROF_NAME = profiler;
// clang-format on

// Records and returns an intermediate snapshot using the profiler defined by an
// earlier RPROF_THIS_FUNCTION in the same function. An optional snapshot
// title can be passed as a macro argument.
// NOTE: Here and below, the '##' in front of __VA_ARGS__ eats up the preceding
// comma in case __VA_ARGS__ is empty, thus avoiding a malformed expression.
// clang-format off
#define RPROF_SNAPSHOT(...) \
  FUNCTION_LEVEL_RPROF_NAME.TakeSnapshot( \
      {__FILE__, __LINE__}, ##__VA_ARGS__)
// clang-format on

// Records AND logs an intermediate snapshot using the profiler defined by an
// earlier RPROF_THIS_FUNCTION() in the same function. An optional snapshot
// title can be passed as a macro argument.
// clang-format off
#define RPROF_SNAPSHOT_AND_LOG(...) \
  FUNCTION_LEVEL_RPROF_NAME.TakeSnapshot( \
      {__FILE__, __LINE__}, ##__VA_ARGS__).Log()
// clang-format on

// Starts taking periodic snapshots using the function-level snapshot created by
// an earlier RPROF_THIS_FUNCTION*(). `interval` is an absl::Duration.
// `also_log` will also log the snapshots. An optional snapshot title can be
// passed as the last macro argument.
// clang-format off
#define RPROF_START_TIMELAPSE(interval, also_log, ...) \
  FUNCTION_LEVEL_RPROF_NAME.StartTimelapse( \
      {__FILE__, __LINE__}, interval, also_log, ##__VA_ARGS__)
// clang-format on

#define RPROF_STOP_TIMELAPSE() FUNCTION_LEVEL_RPROF_NAME.StopTimelapse()

// Prints a final report to the log using the profiler defined by an earlier
// RPROF_THIS_FUNCTION in the same function. An optional report title can be
// passed as a macro argument.
// clang-format off
#define RPROF_DUMP_REPORT_TO_LOG(...) \
  FUNCTION_LEVEL_RPROF_NAME.PrintReport({__FILE__, __LINE__}, ##__VA_ARGS__)
// clang-format on

// Profiles a given scope: a snapshot and a delta of the system timing and
// resource usage are logged at the call site and at the scope exit.
//
// Unlike, RPROF_THIS_FUNCTION, RPROF_THIS_SCOPE can be called any number of
// times per scope, provided the calls are on different lines. That includes
// nested scopes.
//
// Also unlike RPROF_THIS_FUNCTION, RPROF_THIS_SCOPE lacks a complimentary
// macro for intermediate snapshotting using the same profiler: this macro is
// intended as a simple, fast way to profile a scope; for anything more
// involved, use RPROF_THIS_FUNCTION() or RUsageProfiler directly.
// clang-format off
#define RPROF_THIS_SCOPE(enable, description)                              \
  fuzztest::internal::RUsageProfiler SCOPE_LEVEL_RPROF_NAME = {               \
      /*scope=*/fuzztest::internal::RUsageScope::ThisProcess(),               \
      /*metrics=*/(enable) ? fuzztest::internal::RUsageProfiler::kAllMetrics  \
                           : fuzztest::internal::RUsageProfiler::kMetricsOff, \
      /*raii_actions=*/fuzztest::internal::RUsageProfiler::kRaiiSnapshots,    \
      /*location=*/{__FILE__, __LINE__},                                   \
      /*description=*/description                                          \
  }
// clang-format on

// clang-format off
#define RPROF_THIS_SCOPE_WITH_TIMELAPSE(                                   \
    enable, timelapse_interval, also_log_timelapses, description)          \
  fuzztest::internal::RUsageProfiler SCOPE_LEVEL_RPROF_NAME = {               \
      /*scope=*/fuzztest::internal::RUsageScope::ThisProcess(),               \
      /*metrics=*/(enable) ? fuzztest::internal::RUsageProfiler::kAllMetrics  \
                           : fuzztest::internal::RUsageProfiler::kMetricsOff, \
      /*timelapse_interval=*/timelapse_interval,                           \
      /*also_log_timelapses=*/also_log_timelapses,                         \
      /*location=*/{__FILE__, __LINE__},                                   \
      /*description=*/description                                          \
  }
// clang-format on

#endif  // THIRD_PARTY_CENTIPEDE_RUSAGE_PROFILER_H_
