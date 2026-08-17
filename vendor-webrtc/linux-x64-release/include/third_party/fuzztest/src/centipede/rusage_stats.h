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

// Utility classes to capture and log system resource usage of the current
// process.

#ifndef THIRD_PARTY_CENTIPEDE_RUSAGE_STATS_H_
#define THIRD_PARTY_CENTIPEDE_RUSAGE_STATS_H_

#include <sys/resource.h>

#include <array>
#include <cstddef>
#include <cstdint>
#include <iosfwd>
#include <memory>
#include <ostream>
#include <string>

#include "absl/log/check.h"
#include "absl/time/time.h"

namespace fuzztest::internal {

// Memory size in bytes.
using MemSize = int64_t;
// How many CPU hyperthreaded cores the process has been using on average.
// 1 corresponds to 1 hypercore. The max is the number of hyperthreaded cores
// on the system.
using CpuHyperCores = double;
// What percentage of the allotted system scheduling time the process has
// actually utilized for CPU processing, as opposed to idling (e.g. waiting for
// I/O etc.). The theoretical max is 1.0, which corresponds to 100% utilization,
// however the value can go slightly higher due to rounding errors in the system
// scheduler's accounting logic.
using CpuUtilization = long double;

//------------------------------------------------------------------------------
//                               RUsageScope
//
// Specifies the scope of resource usage measurements: a process or a thread.
//------------------------------------------------------------------------------
class RUsageScope {
 public:
  // Static ctors for supported use cases. If the same scope is used repeatedly,
  // callers should prefer caching it, as construction may involve syscalls.
  static RUsageScope ThisProcess();
  static RUsageScope Process(pid_t pid);

  // Copyable and movable.
  RUsageScope(const RUsageScope&) = default;
  RUsageScope& operator=(const RUsageScope&) = default;
  RUsageScope(RUsageScope&&) = default;
  RUsageScope& operator=(RUsageScope&&) = default;
  ~RUsageScope() = default;

  template <typename OStream>
  friend OStream& operator<<(OStream& os, const RUsageScope& s) {
      return os << s.description_;
  }

  // Opaque platform dependent information for rusage monitoring.
  class PlatformInfo;

  const PlatformInfo& info() const {
    // This can fail only when called after the object is moved.
    CHECK(info_ != nullptr);
    return *info_;
  }

 private:
  explicit RUsageScope(pid_t pid);

  std::string description_;
  // Use shared_ptr to make the class copyable (without actually copying the
  // potentially large PlatformInfo).
  std::shared_ptr<const PlatformInfo> info_;
};

//------------------------------------------------------------------------------
//                               ProcessTimer
//
// Measures the system, user, and wall times of the process. Can be a global
// variable because the implementation depends on nothing but syscalls.
// The parameterless RUsageTiming::Snapshot() uses the default global timer that
// starts with the process; clients also have an option to define and pass a
// custom timer to count from some other point in time.
//------------------------------------------------------------------------------

class ProcessTimer {
 public:
  ProcessTimer();
  void Get(double& user, double& sys, double& wall) const;

 private:
  absl::Time start_time_;
  struct rusage start_rusage_;
};

//------------------------------------------------------------------------------
//                                RUsageTiming
//
// An interface to measure, store, manipulate, and log the system timing stats
// of a process or a thread.
//------------------------------------------------------------------------------

struct RUsageTiming {
  //----------------------------------------------------------------------------
  //             Static factory ctors and friend operators

  static RUsageTiming Zero();
  static RUsageTiming Min();
  static RUsageTiming Max();

  // Returns the system timing stats for the specified rusage scope.
  // NOTE: Clients must cache the r-value returned by `RUsageScope` static ctors
  // to be able to call this (this is on purpose).
  static RUsageTiming Snapshot(const RUsageScope& scope);
  // Same as above, but using a custom timer. The caller is responsible for
  // setting up and passing the same timer object to all Snapshot() calls to get
  // consistent results.
  static RUsageTiming Snapshot(  //
      const RUsageScope& scope, const ProcessTimer& timer);

  // Comparisons. NOTE: `is_delta` is always ignored.
  friend bool operator==(const RUsageTiming& t1, const RUsageTiming& t2);
  friend bool operator!=(const RUsageTiming& t1, const RUsageTiming& t2);
  friend bool operator<(const RUsageTiming& t1, const RUsageTiming& t2);
  friend bool operator<=(const RUsageTiming& t1, const RUsageTiming& t2);
  friend bool operator>(const RUsageTiming& t1, const RUsageTiming& t2);
  friend bool operator>=(const RUsageTiming& t1, const RUsageTiming& t2);

  // Returns the low-water resource usage between the two args.
  static RUsageTiming LowWater(const RUsageTiming& t1, const RUsageTiming& t2);
  // Returns the high-water value between the two args.
  static RUsageTiming HighWater(const RUsageTiming& t1, const RUsageTiming& t2);

  // Returns the value with `is_delta` set to true. Useful for signed logging.
  friend RUsageTiming operator+(const RUsageTiming& t);
  // Returns the negated value with `is_delta` set to true.
  friend RUsageTiming operator-(const RUsageTiming& t);
  // Returns the signed delta between two stats, with `is_delta` set to true.
  friend RUsageTiming operator-(const RUsageTiming& t1, const RUsageTiming& t2);
  // Returns the sum of two stats, with `is_delta` set to true iff `t1` or `t2`
  // or both are deltas.
  friend RUsageTiming operator+(const RUsageTiming& t1, const RUsageTiming& t2);
  // Returns a RUsageTiming where every field is divided by `div`. `is_delta` is
  // carried over from `t`.
  friend RUsageTiming operator/(const RUsageTiming& t, int64_t div);

  // Streams `t.ShortStr()`.
  friend std::ostream& operator<<(std::ostream& os, const RUsageTiming& t);

  //----------------------------------------------------------------------------
  //                           Non-static methods

  // Returns the metrics as short string. If `is_delta` is true, positive values
  // will be prefixed with a '+'.
  std::string ShortStr() const;
  // Returns a formatted representation of the metrics. The format is fixed, so
  // if multiple objects get printed with newline separators, they will form a
  // table. If `is_delta` is true, positive values will be prefixed with a '+'.
  std::string FormattedStr() const;

  //----------------------------------------------------------------------------
  //                              Public data

  absl::Duration wall_time = absl::ZeroDuration();
  absl::Duration user_time = absl::ZeroDuration();
  absl::Duration sys_time = absl::ZeroDuration();
  CpuUtilization cpu_utilization = 0.;
  CpuHyperCores cpu_hyper_cores = 0.;
  // If true, positive values will be printed with a '+'.
  bool is_delta = false;
};

//------------------------------------------------------------------------------
//                               RUsageMemory
//
// An interface to measure, store, manipulate, and log the system memory usage
// of a process or a thread.
//------------------------------------------------------------------------------

struct RUsageMemory {
  //----------------------------------------------------------------------------
  //              Static factory ctors and friend operators

  static RUsageMemory Zero();
  static RUsageMemory Min();
  static RUsageMemory Max();

  // Returns the system memory stats for the specified rusage scope.
  // NOTE: Clients must cache the r-value returned by `RUsageScope` static ctors
  // to be able to call this (this is on purpose).
  static RUsageMemory Snapshot(const RUsageScope& scope);

  // Comparisons. NOTE: `is_delta` is always ignored.
  friend bool operator==(const RUsageMemory& m1, const RUsageMemory& m2);
  friend bool operator!=(const RUsageMemory& m1, const RUsageMemory& m2);
  friend bool operator<(const RUsageMemory& m1, const RUsageMemory& m2);
  friend bool operator<=(const RUsageMemory& m1, const RUsageMemory& m2);
  friend bool operator>(const RUsageMemory& m1, const RUsageMemory& m2);
  friend bool operator>=(const RUsageMemory& m1, const RUsageMemory& m2);

  // Returns the low-water value between the two args.
  static RUsageMemory LowWater(const RUsageMemory& m1, const RUsageMemory& m2);
  // Returns the high-water value usage between the two args.
  static RUsageMemory HighWater(const RUsageMemory& m1, const RUsageMemory& m2);

  // Returns the value with `is_delta` set to true. Useful for signed logging.
  friend RUsageMemory operator+(const RUsageMemory& m);
  // Returns the negated value with `is_delta` set to true.
  friend RUsageMemory operator-(const RUsageMemory& m);
  // Returns the signed delta of two stats, with `is_delta` set to true.
  friend RUsageMemory operator-(const RUsageMemory& m1, const RUsageMemory& m2);
  // Returns the sum of two stats, with `is_delta` set to true iff `m1` or `m2`
  // or both are deltas.
  friend RUsageMemory operator+(const RUsageMemory& m1, const RUsageMemory& m2);
  // Returns a value with every metric divided by `div`. `is_delta` is
  // carried over from `m`.
  friend RUsageMemory operator/(const RUsageMemory& m, int64_t div);

  // Streams `m.ShortStr()`.
  friend std::ostream& operator<<(std::ostream& os, const RUsageMemory& m);

  //----------------------------------------------------------------------------
  //                           Non-static methods

  // Returns the metrics as short string. If `is_delta` is true, positive values
  // will be prefixed with a '+'.
  std::string ShortStr() const;
  // Returns a formatted representation of the metrics. The format is fixed, so
  // if multiple objects get printed with newline separators, they will form a
  // table. If `is_delta` is true, positive values will be prefixed with a '+'.
  std::string FormattedStr() const;

  //----------------------------------------------------------------------------
  //                                   Data

  // Memory sizes are all in bytes. For the meaning of these, cf. `man proc` or
  // https://man7.org/linux/man-pages/man5/proc.5.html, sections
  // /proc/[pid]/{stat,statm,status}.
  MemSize mem_vsize = 0;
  MemSize mem_vpeak = 0;
  MemSize mem_rss = 0;
  MemSize mem_data = 0;
  MemSize mem_shared = 0;
  // If true, positive values will be printed with a '+'.
  bool is_delta = false;
};

//------------------------------------------------------------------------------
//                       Pretty-printing of the stats
//------------------------------------------------------------------------------

// Formats `duration` as the most compact human-readable string. Differences
// from absl::FormatDuration():
// - Durations up to 1s are rounded up to whole numbers of ns/us/ms.
// - Durations longer than 1s are rounded up to 2 decimals and are never
//   converted to hours/minutes/seconds.
// - Positive durations can be prefixed with a '+' (useful to indicate that the
//   value is a positive delta).
std::string FormatInOptimalUnits(absl::Duration duration, bool always_signed);
// Formats `bytes` as the most compact human-readable string in SI units.
// `always_signed` prints '+' before positive numbers (useful to indicate
// positive deltas).
std::string FormatInOptimalUnits(MemSize bytes, bool always_signed);
// Formats CPU utilization as a percentage.
// `always_signed` prints '+' before positive numbers (useful to indicate
// positive deltas).
std::string FormatInOptimalUnits(CpuUtilization util, bool always_signed);
// Formats CPU hypercores with decimal precision.
// `always_signed` prints '+' before positive numbers (useful to indicate
// positive deltas).
std::string FormatInOptimalUnits(CpuHyperCores cores, bool always_signed);

}  // namespace fuzztest::internal

#endif  // THIRD_PARTY_CENTIPEDE_RUSAGE_STATS_H_
