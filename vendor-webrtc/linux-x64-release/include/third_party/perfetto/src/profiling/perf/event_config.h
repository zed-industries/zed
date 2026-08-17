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

#ifndef SRC_PROFILING_PERF_EVENT_CONFIG_H_
#define SRC_PROFILING_PERF_EVENT_CONFIG_H_

#include <functional>
#include <string>
#include <vector>

#include <linux/perf_event.h>
#include <stdint.h>
#include <sys/types.h>
#include <optional>

#include "perfetto/base/flat_set.h"
#include "perfetto/tracing/core/data_source_config.h"

#include "protos/perfetto/common/perf_events.gen.h"
#include "protos/perfetto/config/profiling/perf_event_config.gen.h"

namespace perfetto::profiling {

enum class RecordingMode { kPolling, kSampling };

// Callstack sampling parameter for unwinding only a fraction of seen processes
// (without enumerating them in the config).
struct ProcessSharding {
  uint32_t shard_count = 0;
  uint32_t chosen_shard = 0;
};

// Parsed allow/deny-list for filtering samples.
// An empty allow-list means that all targets are allowed unless explicitly
// denied.
struct TargetFilter {
  std::vector<std::string> cmdlines;
  std::vector<std::string> exclude_cmdlines;
  base::FlatSet<pid_t> pids;
  base::FlatSet<pid_t> exclude_pids;
  std::optional<ProcessSharding> process_sharding;
  uint32_t additional_cmdline_count = 0;
};

// Describes a perf event for two purposes:
// * encoding the event in the perf_event_open syscall
// * echoing the counter's config in the trace packet defaults, so that the
//   parser can tell which datastream belongs to which counter.
// Note: It's slightly odd to decode & pass around values we don't use outside
// of reencoding back into a defaults proto. One option would be to carry the
// Timebase proto, but this won't fit with the eventual support of multiple
// counters, as at the proto level it'll be a distinct message from Timebase.
struct PerfCounter {
  enum class Type { kBuiltinCounter, kTracepoint, kRawEvent };

  Type type = Type::kBuiltinCounter;

  // Optional config-supplied name for the counter, to identify it during
  // trace parsing, does not affect the syscall.
  std::string name;

  // valid if kBuiltinCounter
  protos::gen::PerfEvents::Counter counter =
      protos::gen::PerfEvents::PerfEvents::UNKNOWN_COUNTER;
  // valid if kTracepoint. Example: "sched:sched_switch".
  std::string tracepoint_name;
  // valid if kTracepoint
  std::string tracepoint_filter;

  // sycall-level description of the event (perf_event_attr):
  uint32_t attr_type = 0;
  uint64_t attr_config = 0;
  uint64_t attr_config1 = 0;  // optional extension
  uint64_t attr_config2 = 0;  // optional extension

  Type event_type() const { return type; }

  static PerfCounter BuiltinCounter(std::string name,
                                    protos::gen::PerfEvents::Counter counter,
                                    uint32_t type,
                                    uint64_t config);

  static PerfCounter Tracepoint(std::string name,
                                std::string tracepoint_name,
                                std::string tracepoint_filter,
                                uint64_t id);

  static PerfCounter RawEvent(std::string name,
                              uint32_t type,
                              uint64_t config,
                              uint64_t config1,
                              uint64_t config2);
};

// Describes a single profiling configuration. Bridges the gap between the data
// source config proto, and the raw |perf_event_attr| structs to pass to the
// perf_event_open syscall.
class EventConfig {
 public:
  using tracepoint_id_fn_t =
      std::function<uint32_t(const std::string&, const std::string&)>;

  static std::optional<EventConfig> Create(
      const protos::gen::PerfEventConfig& pb_config,
      const DataSourceConfig& raw_ds_config,
      std::optional<ProcessSharding> process_sharding,
      const tracepoint_id_fn_t& tracepoint_id_lookup);

  // clang-format off
  RecordingMode recording_mode() const { return recording_mode_; }
  uint32_t ring_buffer_pages() const { return ring_buffer_pages_; }
  uint32_t read_tick_period_ms() const { return read_tick_period_ms_; }
  uint64_t samples_per_tick_limit() const { return samples_per_tick_limit_; }
  uint32_t remote_descriptor_timeout_ms() const { return remote_descriptor_timeout_ms_; }
  uint32_t unwind_state_clear_period_ms() const { return unwind_state_clear_period_ms_; }
  uint64_t max_enqueued_footprint_bytes() const { return max_enqueued_footprint_bytes_; }
  protos::gen::PerfEventConfig::UnwindMode unwind_mode() const { return unwind_mode_; }
  const TargetFilter& filter() const { return target_filter_; }
  perf_event_attr* perf_attr() const { return const_cast<perf_event_attr*>(&perf_event_attr_); }
  const std::vector<perf_event_attr>& perf_attr_followers() const { return perf_event_followers_; }
  const PerfCounter& timebase_event() const { return timebase_event_; }
  const std::vector<PerfCounter>& follower_events() const { return follower_events_; }
  const std::vector<std::string>& target_installed_by() const { return target_installed_by_; }
  const DataSourceConfig& raw_ds_config() const { return raw_ds_config_; }
  // non-trivial accessors:
  bool sample_callstacks() const { return user_frames() || kernel_frames_; }
  bool user_frames() const { return IsUserFramesEnabled(unwind_mode_); }
  bool kernel_frames() const { return kernel_frames_; }
  // clang-format on

 private:
  static bool IsUserFramesEnabled(
      const protos::gen::PerfEventConfig::UnwindMode& unwind_mode);

  EventConfig(const DataSourceConfig& raw_ds_config,
              const perf_event_attr& pe_timebase,
              std::vector<perf_event_attr> pe_followers,
              PerfCounter timebase_event,
              std::vector<PerfCounter> follower_events,
              RecordingMode recording_mode,
              bool kernel_frames,
              protos::gen::PerfEventConfig::UnwindMode unwind_mode,
              TargetFilter target_filter,
              uint32_t ring_buffer_pages,
              uint32_t read_tick_period_ms,
              uint64_t samples_per_tick_limit,
              uint32_t remote_descriptor_timeout_ms,
              uint32_t unwind_state_clear_period_ms,
              uint64_t max_enqueued_footprint_bytes,
              std::vector<std::string> target_installed_by);

  static std::optional<EventConfig> CreatePolling(
      PerfCounter timebase_event,
      std::vector<PerfCounter> followers,
      const protos::gen::PerfEventConfig& pb_config,
      const DataSourceConfig& raw_ds_config);

  static std::optional<EventConfig> CreateSampling(
      PerfCounter timebase_event,
      std::vector<PerfCounter> followers,
      std::optional<ProcessSharding> process_sharding,
      const protos::gen::PerfEventConfig& pb_config,
      const DataSourceConfig& raw_ds_config);

  // Parameter struct for the timebase perf_event_open syscall.
  perf_event_attr perf_event_attr_ = {};

  // Additional events in the group, each configured with a separate syscall.
  std::vector<perf_event_attr> perf_event_followers_;

  // Timebase event, which is already described by |perf_event_attr_|. But this
  // additionally carries a tracepoint filter if that needs to be set via an
  // ioctl after creating the event.
  const PerfCounter timebase_event_;

  // Follower events, which are already described by |perf_event_followers_|.
  const std::vector<PerfCounter> follower_events_;

  // Whether we're using the read syscall to poll event counts, or mmapping a
  // ring buffer. In the earlier case, most of the subsequent fields are
  // unused.
  const RecordingMode recording_mode_;

  // If true, include kernel frames in sampled callstacks.
  const bool kernel_frames_;

  // Userspace unwinding mode.
  const protos::gen::PerfEventConfig::UnwindMode unwind_mode_;

  // Parsed allow/deny-list for filtering samples.
  const TargetFilter target_filter_;

  // Size (in 4k pages) of each per-cpu ring buffer shared with the kernel.
  // Must be a power of two.
  const uint32_t ring_buffer_pages_;

  // In polling mode - how often to read the counters.
  // In sampling mode - how often to read the ring buffers.
  const uint32_t read_tick_period_ms_;

  // Guardrail for the amount of samples a given read attempt will extract from
  // *each* per-cpu buffer.
  const uint64_t samples_per_tick_limit_;

  // Timeout for proc-fd lookup.
  const uint32_t remote_descriptor_timeout_ms_;

  // Optional period for clearing cached unwinder state. Skipped if zero.
  const uint32_t unwind_state_clear_period_ms_;

  // Optional threshold for load shedding in the reader<->unwinder queue.
  // Skipped if zero.
  const uint64_t max_enqueued_footprint_bytes_;

  // Only profile target if it was installed by one of the packages given.
  // Special values are:
  // * "@system": installed on the system partition
  // * "@product": installed on the product partition
  // * "@null": sideloaded
  const std::vector<std::string> target_installed_by_;

  // The raw data source config, as a pbzero-generated C++ class.
  const DataSourceConfig raw_ds_config_;
};

}  // namespace perfetto::profiling

#endif  // SRC_PROFILING_PERF_EVENT_CONFIG_H_
