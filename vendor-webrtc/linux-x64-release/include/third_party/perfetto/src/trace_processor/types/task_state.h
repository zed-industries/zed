/*
 * Copyright (C) 2019 The Android Open Source Project
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

#ifndef SRC_TRACE_PROCESSOR_TYPES_TASK_STATE_H_
#define SRC_TRACE_PROCESSOR_TYPES_TASK_STATE_H_

#include <stdint.h>
#include <array>
#include <optional>

#include "src/trace_processor/types/version_number.h"

namespace perfetto {
namespace trace_processor {
namespace ftrace_utils {

// Linux kernel scheduling events (sched_switch) contain a bitmask of the
// switched-out task's state (prev_state). Perfetto doesn't record the event
// format string during tracing, the trace contains only the raw bitmask as an
// integer. Certain kernel versions made backwards incompatible changes to the
// bitmask's raw representation, so this class guesses how to decode the flags
// based on the kernel's major+minor version as recorded in the trace. Note:
// this means we can be wrong if patch backports change the flags, or the
// kernel diverged from upstream. But this has worked well enough in practice
// so far.
//
// There are three specific kernel version intervals we handle:
// * [4.14, ...)
// * [4.8, 4.14)
// * (..., 4.8), where we assume the 4.4 bitmask
//
// (Therefore kernels before 4.2 most likely have incorrect preemption flag
// parsing.)
//
// For 4.14, we assume that the kernel has a backport of the bugfix
// https://github.com/torvalds/linux/commit/3f5fe9fe ("sched/debug: Fix task
// state recording/printout"). In other words, traces collected on unpatched
// 4.14 kernels will have incorrect flags decoded.
class TaskState {
 public:
  using TaskStateStr = std::array<char, 4>;

  // We transcode the raw bitmasks into a set of these flags to make them
  // kernel version agnostic.
  //
  // Warning: do NOT depend on the numeric values of these constants, and
  // especially do NOT attempt to use these constants when operating on raw
  // prev_state masks unless you're changing task_state.cc itself.
  enum ParsedFlag : uint16_t {
    kRunnable = 0x0000,  // no flag (besides kPreempted) means "running"
    kInterruptibleSleep = 0x0001,
    kUninterruptibleSleep = 0x0002,
    kStopped = 0x0004,
    kTraced = 0x0008,
    kExitDead = 0x0010,
    kExitZombie = 0x0020,

    // Starting from here, different kernels have different values:
    kParked = 0x0040,

    // No longer reported on 4.14+:
    kTaskDead = 0x0080,
    kWakeKill = 0x0100,
    kWaking = 0x0200,
    kNoLoad = 0x0400,

    // Special states that don't map onto the scheduler's constants:
    kIdle = 0x4000,
    kPreempted = 0x8000,  // exclusive as only running tasks can be preempted

    // Sentinel value that is an invalid combination of flags:
    kInvalid = 0xffff
  };

  static TaskState FromRawPrevState(
      uint16_t raw_state,
      std::optional<VersionNumber> kernel_version);
  static TaskState FromSystrace(const char* state_str);
  static TaskState FromParsedFlags(uint16_t parsed_state);

  // TODO(rsavitski): consider moving the factory methods to an optional return
  // type instead.
  bool is_valid() const { return parsed_ != kInvalid; }

  // Returns the textual representation of this state as a null-terminated
  // array. |separator| specifies if a separator should be printed between the
  // atoms (default: \0 meaning no separator).
  TaskStateStr ToString(char separator = '\0') const;

  // Converts the TaskState back to the raw format, to be used only when
  // parsing systrace.
  // NB: this makes a hard assumption on the 4.4 flag layout, since systrace
  // files don't specify a kernel version, so when trace_processor later calls
  // FromRawPrevState to construct sched.end_state column values, it'll default
  // to the 4.4 layout.
  // TODO(rsavitski): can we get rid of this entirely and avoid the
  // str -> TaskState -> uint16_t -> str conversion chain?
  uint16_t ToRawStateOnlyForSystraceConversions() const;

  uint16_t ParsedForTesting() const { return parsed_; }

 private:
  TaskState() = default;
  explicit TaskState(uint16_t raw_state,
                     std::optional<VersionNumber> kernel_version);
  explicit TaskState(const char* state_str);

  bool is_runnable() const { return !(parsed_ & ~kPreempted); }

  uint16_t parsed_ = 0;
};

}  // namespace ftrace_utils
}  // namespace trace_processor
}  // namespace perfetto

#endif  // SRC_TRACE_PROCESSOR_TYPES_TASK_STATE_H_
