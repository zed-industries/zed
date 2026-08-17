/*
 * Copyright (C) 2020 The Android Open Source Project
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

#ifndef SRC_PROFILING_COMMON_PROFILER_GUARDRAILS_H_
#define SRC_PROFILING_COMMON_PROFILER_GUARDRAILS_H_

#include <fcntl.h>
#include <unistd.h>

#include <cinttypes>
#include <optional>

#include "perfetto/ext/base/file_utils.h"
#include "perfetto/ext/base/scoped_file.h"
#include "perfetto/ext/tracing/core/basic_types.h"
#include "src/profiling/common/proc_utils.h"

namespace perfetto {
namespace profiling {

std::optional<uint64_t> GetCputimeSecForCurrentProcess();
// For testing.
std::optional<uint64_t> GetCputimeSecForCurrentProcess(
    base::ScopedFile stat_fd);

struct GuardrailConfig {
  uint64_t cpu_guardrail_sec = 0;
  std::optional<uint64_t> cpu_start_secs;
  uint32_t memory_guardrail_kb = 0;
};

class ProfilerCpuGuardrails {
 public:
  ProfilerCpuGuardrails();
  // Allows to supply custom stat fd for testing.
  explicit ProfilerCpuGuardrails(base::ScopedFile stat_fd);

  bool IsOverCpuThreshold(const GuardrailConfig& ds);

 private:
  std::optional<uint64_t> opt_cputime_sec_;
};

class ProfilerMemoryGuardrails {
 public:
  ProfilerMemoryGuardrails();
  // Allows to supply custom status fd for testing.
  explicit ProfilerMemoryGuardrails(base::ScopedFile status_fd);

  bool IsOverMemoryThreshold(const GuardrailConfig& ds);

 private:
  std::optional<uint32_t> anon_and_swap_;
};

}  // namespace profiling
}  // namespace perfetto

#endif  // SRC_PROFILING_COMMON_PROFILER_GUARDRAILS_H_
