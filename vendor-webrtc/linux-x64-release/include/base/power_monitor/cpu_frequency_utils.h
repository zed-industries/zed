// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_POWER_MONITOR_CPU_FREQUENCY_UTILS_H_
#define BASE_POWER_MONITOR_CPU_FREQUENCY_UTILS_H_

#include <optional>

#include "base/base_export.h"
#include "base/time/time.h"
#include "base/values.h"

namespace base {

struct BASE_EXPORT CpuFrequencyInfo {
  // These fields map to fields in the win32 PROCESSOR_POWER_INFORMATION
  // struct. They are currently only implemented on Windows since that's
  // the platform being investigated, but they could be replaced with
  // something more generic/cross-platform as needed.

  // The maximum frequency or the frequency limit of a CPU.
  unsigned long max_mhz;
  unsigned long mhz_limit;

  enum class CoreType {
    kPerformance,
    kBalanced,
    kEfficiency,
  };

  // A best effort guess at whether the associated CPU core is a performance
  // core, an efficiency core, or something in between (balanced).
  CoreType type;

  // The number of CPU cores that are in the C0 state (active).
  size_t num_active_cpus;
};

struct BASE_EXPORT CpuThroughputEstimationResult {
  // The estimated CPU frequency of the current core, in Hz.
  double estimated_frequency;
  // True if the current core is different after the estimation loop than
  // before.
  bool migrated;

  // The wall time and thread time of the CPU estimation task's assembly loop.
  base::TimeDelta wall_time;
  base::TimeDelta thread_time;
};

// Returns the estimated CPU frequency of the current core by executing a tight
// loop of predictable assembly instructions. The estimated frequency should be
// proportional to and about the same magnitude as the real CPU frequency,
// although it is possible for the code to be migrated/descheduled during the
// execution of this function. The measurement should be long enough to avoid
// Turbo Boost effect (~3ms) and be low enough to stay within the operating
// system scheduler quantum (~100ms).
// The return value is the estimated CPU frequency, in Hz.
BASE_EXPORT double EstimateCpuFrequency();
BASE_EXPORT std::optional<CpuThroughputEstimationResult>
EstimateCpuThroughput();

// Populates and returns a `CpuFrequencyInfo` struct with information from the
// current CPU core.
BASE_EXPORT CpuFrequencyInfo GetCpuFrequencyInfo();

#if BUILDFLAG(IS_WIN)
BASE_EXPORT void GenerateCpuInfoForTracingMetadata(base::Value::Dict* metadata);
#endif
}  // namespace base

#endif  // BASE_POWER_MONITOR_CPU_FREQUENCY_UTILS_H_
