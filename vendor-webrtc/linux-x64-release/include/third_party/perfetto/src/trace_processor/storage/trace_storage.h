/*
 * Copyright (C) 2017 The Android Open Source Project
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

#ifndef SRC_TRACE_PROCESSOR_STORAGE_TRACE_STORAGE_H_
#define SRC_TRACE_PROCESSOR_STORAGE_TRACE_STORAGE_H_

#include <algorithm>
#include <array>
#include <cstddef>
#include <cstdint>
#include <deque>
#include <functional>
#include <iterator>
#include <limits>
#include <map>
#include <memory>
#include <optional>
#include <string>
#include <string_view>
#include <utility>
#include <vector>

#include "perfetto/base/logging.h"
#include "perfetto/base/status.h"
#include "perfetto/base/time.h"
#include "perfetto/ext/base/string_view.h"
#include "perfetto/trace_processor/basic_types.h"
#include "perfetto/trace_processor/trace_blob_view.h"
#include "src/trace_processor/containers/null_term_string_view.h"
#include "src/trace_processor/containers/string_pool.h"
#include "src/trace_processor/db/column/types.h"
#include "src/trace_processor/db/typed_column_internal.h"
#include "src/trace_processor/storage/stats.h"
#include "src/trace_processor/tables/android_tables_py.h"
#include "src/trace_processor/tables/counter_tables_py.h"
#include "src/trace_processor/tables/etm_tables_py.h"
#include "src/trace_processor/tables/flow_tables_py.h"
#include "src/trace_processor/tables/jit_tables_py.h"
#include "src/trace_processor/tables/memory_tables_py.h"
#include "src/trace_processor/tables/metadata_tables_py.h"
#include "src/trace_processor/tables/perf_tables_py.h"
#include "src/trace_processor/tables/profiler_tables_py.h"
#include "src/trace_processor/tables/sched_tables_py.h"
#include "src/trace_processor/tables/slice_tables_py.h"
#include "src/trace_processor/tables/trace_proto_tables_py.h"
#include "src/trace_processor/tables/track_tables_py.h"
#include "src/trace_processor/tables/v8_tables_py.h"
#include "src/trace_processor/tables/winscope_tables_py.h"
#include "src/trace_processor/types/destructible.h"
#include "src/trace_processor/types/variadic.h"

namespace perfetto::trace_processor {
namespace etm {
class TargetMemory;
}

// UniquePid is an offset into |unique_processes_|. This is necessary because
// Unix pids are reused and thus not guaranteed to be unique over a long
// period of time.
using UniquePid = uint32_t;

// UniqueTid is an offset into |unique_threads_|. Necessary because tids can
// be reused.
using UniqueTid = uint32_t;

// StringId is an offset into |string_pool_|.
using StringId = StringPool::Id;
static const StringId kNullStringId = StringId::Null();

using ArgSetId = uint32_t;

using TrackId = tables::TrackTable::Id;

using CounterId = tables::CounterTable::Id;

using SliceId = tables::SliceTable::Id;

using SchedId = tables::SchedSliceTable::Id;

using MappingId = tables::StackProfileMappingTable::Id;

using FrameId = tables::StackProfileFrameTable::Id;

using SymbolId = tables::SymbolTable::Id;

using CallsiteId = tables::StackProfileCallsiteTable::Id;

using MetadataId = tables::MetadataTable::Id;

using FlamegraphId = tables::ExperimentalFlamegraphTable::Id;

using VulkanAllocId = tables::VulkanMemoryAllocationsTable::Id;

using ProcessMemorySnapshotId = tables::ProcessMemorySnapshotTable::Id;

using SnapshotNodeId = tables::MemorySnapshotNodeTable::Id;

static const TrackId kInvalidTrackId =
    TrackId(std::numeric_limits<uint32_t>::max());

enum class RefType {
  kRefNoRef = 0,
  kRefUtid = 1,
  kRefCpuId = 2,
  kRefIrq = 3,
  kRefSoftIrq = 4,
  kRefUpid = 5,
  kRefGpuId = 6,
  kRefTrack = 7,
  kRefMax
};

const std::vector<NullTermStringView>& GetRefTypeStringMap();

// Stores a data inside a trace file in a columnar form. This makes it efficient
// to read or search across a single field of the trace (e.g. all the thread
// names for a given CPU).
class TraceStorage {
 public:
  explicit TraceStorage(const Config& = Config());

  virtual ~TraceStorage();

  class VirtualTrackSlices {
   public:
    inline uint32_t AddVirtualTrackSlice(SliceId slice_id,
                                         int64_t thread_timestamp_ns,
                                         int64_t thread_duration_ns,
                                         int64_t thread_instruction_count,
                                         int64_t thread_instruction_delta) {
      slice_ids_.emplace_back(slice_id);
      thread_timestamp_ns_.emplace_back(thread_timestamp_ns);
      thread_duration_ns_.emplace_back(thread_duration_ns);
      thread_instruction_counts_.emplace_back(thread_instruction_count);
      thread_instruction_deltas_.emplace_back(thread_instruction_delta);
      return slice_count() - 1;
    }

    uint32_t slice_count() const {
      return static_cast<uint32_t>(slice_ids_.size());
    }

    const std::deque<SliceId>& slice_ids() const { return slice_ids_; }
    const std::deque<int64_t>& thread_timestamp_ns() const {
      return thread_timestamp_ns_;
    }
    const std::deque<int64_t>& thread_duration_ns() const {
      return thread_duration_ns_;
    }
    const std::deque<int64_t>& thread_instruction_counts() const {
      return thread_instruction_counts_;
    }
    const std::deque<int64_t>& thread_instruction_deltas() const {
      return thread_instruction_deltas_;
    }

    std::optional<uint32_t> FindRowForSliceId(SliceId slice_id) const {
      auto it =
          std::lower_bound(slice_ids().begin(), slice_ids().end(), slice_id);
      if (it != slice_ids().end() && *it == slice_id) {
        return static_cast<uint32_t>(std::distance(slice_ids().begin(), it));
      }
      return std::nullopt;
    }

    void UpdateThreadDeltasForSliceId(SliceId slice_id,
                                      int64_t end_thread_timestamp_ns,
                                      int64_t end_thread_instruction_count) {
      auto opt_row = FindRowForSliceId(slice_id);
      if (!opt_row)
        return;
      uint32_t row = *opt_row;
      int64_t begin_ns = thread_timestamp_ns_[row];
      thread_duration_ns_[row] = end_thread_timestamp_ns - begin_ns;
      int64_t begin_ticount = thread_instruction_counts_[row];
      thread_instruction_deltas_[row] =
          end_thread_instruction_count - begin_ticount;
    }

   private:
    std::deque<SliceId> slice_ids_;
    std::deque<int64_t> thread_timestamp_ns_;
    std::deque<int64_t> thread_duration_ns_;
    std::deque<int64_t> thread_instruction_counts_;
    std::deque<int64_t> thread_instruction_deltas_;
  };

  class SqlStats {
   public:
    static constexpr size_t kMaxLogEntries = 100;
    uint32_t RecordQueryBegin(const std::string& query, int64_t time_started);
    void RecordQueryFirstNext(uint32_t row, int64_t time_first_next);
    void RecordQueryEnd(uint32_t row, int64_t time_end);
    size_t size() const { return queries_.size(); }
    const std::deque<std::string>& queries() const { return queries_; }
    const std::deque<int64_t>& times_started() const { return times_started_; }
    const std::deque<int64_t>& times_first_next() const {
      return times_first_next_;
    }
    const std::deque<int64_t>& times_ended() const { return times_ended_; }

   private:
    uint32_t popped_queries_ = 0;

    std::deque<std::string> queries_;
    std::deque<int64_t> times_started_;
    std::deque<int64_t> times_first_next_;
    std::deque<int64_t> times_ended_;
  };

  struct Stats {
    using IndexMap = std::map<int, int64_t>;
    int64_t value = 0;
    IndexMap indexed_values;
  };
  using StatsMap = std::array<Stats, stats::kNumKeys>;

  // Return an unique identifier for the contents of each string.
  // The string is copied internally and can be destroyed after this called.
  // Virtual for testing.
  virtual StringId InternString(base::StringView str) {
    return string_pool_.InternString(str);
  }
  virtual StringId InternString(const char* str) {
    return InternString(base::StringView(str));
  }
  virtual StringId InternString(const std::string& str) {
    return InternString(base::StringView(str));
  }
  virtual StringId InternString(std::string_view str) {
    return InternString(base::StringView(str.data(), str.size()));
  }

  // Example usage: SetStats(stats::android_log_num_failed, 42);
  void SetStats(size_t key, int64_t value) {
    PERFETTO_DCHECK(key < stats::kNumKeys);
    PERFETTO_DCHECK(stats::kTypes[key] == stats::kSingle);
    stats_[key].value = value;
  }

  // Example usage: IncrementStats(stats::android_log_num_failed, -1);
  void IncrementStats(size_t key, int64_t increment = 1) {
    PERFETTO_DCHECK(key < stats::kNumKeys);
    PERFETTO_DCHECK(stats::kTypes[key] == stats::kSingle);
    stats_[key].value += increment;
  }

  // Example usage: IncrementIndexedStats(stats::cpu_failure, 1);
  void IncrementIndexedStats(size_t key, int index, int64_t increment = 1) {
    PERFETTO_DCHECK(key < stats::kNumKeys);
    PERFETTO_DCHECK(stats::kTypes[key] == stats::kIndexed);
    stats_[key].indexed_values[index] += increment;
  }

  // Example usage: SetIndexedStats(stats::cpu_failure, 1, 42);
  void SetIndexedStats(size_t key, int index, int64_t value) {
    PERFETTO_DCHECK(key < stats::kNumKeys);
    PERFETTO_DCHECK(stats::kTypes[key] == stats::kIndexed);
    stats_[key].indexed_values[index] = value;
  }

  // Example usage: opt_cpu_failure = GetIndexedStats(stats::cpu_failure, 1);
  std::optional<int64_t> GetIndexedStats(size_t key, int index) {
    PERFETTO_DCHECK(key < stats::kNumKeys);
    PERFETTO_DCHECK(stats::kTypes[key] == stats::kIndexed);
    auto kv = stats_[key].indexed_values.find(index);
    if (kv != stats_[key].indexed_values.end()) {
      return kv->second;
    }
    return std::nullopt;
  }

  int64_t GetStats(size_t key) {
    PERFETTO_DCHECK(key < stats::kNumKeys);
    PERFETTO_DCHECK(stats::kTypes[key] == stats::kSingle);
    return stats_[key].value;
  }

  class ScopedStatsTracer {
   public:
    ScopedStatsTracer(TraceStorage* storage, size_t key)
        : storage_(storage), key_(key), start_ns_(base::GetWallTimeNs()) {}

    ~ScopedStatsTracer() {
      if (!storage_)
        return;
      auto delta_ns = base::GetWallTimeNs() - start_ns_;
      storage_->IncrementStats(key_, delta_ns.count());
    }

    ScopedStatsTracer(ScopedStatsTracer&& other) noexcept { MoveImpl(&other); }

    ScopedStatsTracer& operator=(ScopedStatsTracer&& other) {
      MoveImpl(&other);
      return *this;
    }

   private:
    ScopedStatsTracer(const ScopedStatsTracer&) = delete;
    ScopedStatsTracer& operator=(const ScopedStatsTracer&) = delete;

    void MoveImpl(ScopedStatsTracer* other) {
      storage_ = other->storage_;
      key_ = other->key_;
      start_ns_ = other->start_ns_;
      other->storage_ = nullptr;
    }

    TraceStorage* storage_;
    size_t key_;
    base::TimeNanos start_ns_;
  };

  ScopedStatsTracer TraceExecutionTimeIntoStats(size_t key) {
    return ScopedStatsTracer(this, key);
  }

  // Reading methods.
  // Virtual for testing.
  virtual NullTermStringView GetString(StringId id) const {
    return string_pool_.Get(id);
  }

  // Requests the removal of unused capacity.
  // Matches the semantics of std::vector::shrink_to_fit.
  void ShrinkToFitTables() {
    // At the moment, we only bother calling ShrinkToFit on a set group
    // of tables. If we wanted to extend this to every table, we'd need to deal
    // with tracking all the tables in the storage: this is not worth doing
    // given most memory is used by these tables.
    thread_table_.ShrinkToFit();
    process_table_.ShrinkToFit();
    track_table_.ShrinkToFit();
    counter_table_.ShrinkToFit();
    slice_table_.ShrinkToFit();
    ftrace_event_table_.ShrinkToFit();
    sched_slice_table_.ShrinkToFit();
    thread_state_table_.ShrinkToFit();
    arg_table_.ShrinkToFit();
    heap_graph_object_table_.ShrinkToFit();
    heap_graph_reference_table_.ShrinkToFit();
  }

  const tables::ThreadTable& thread_table() const { return thread_table_; }
  tables::ThreadTable* mutable_thread_table() { return &thread_table_; }

  const tables::ProcessTable& process_table() const { return process_table_; }
  tables::ProcessTable* mutable_process_table() { return &process_table_; }

  const tables::FiledescriptorTable& filedescriptor_table() const {
    return filedescriptor_table_;
  }
  tables::FiledescriptorTable* mutable_filedescriptor_table() {
    return &filedescriptor_table_;
  }

  const tables::TrackTable& track_table() const { return track_table_; }
  tables::TrackTable* mutable_track_table() { return &track_table_; }

  const tables::GpuCounterGroupTable& gpu_counter_group_table() const {
    return gpu_counter_group_table_;
  }
  tables::GpuCounterGroupTable* mutable_gpu_counter_group_table() {
    return &gpu_counter_group_table_;
  }

  const tables::ThreadStateTable& thread_state_table() const {
    return thread_state_table_;
  }
  tables::ThreadStateTable* mutable_thread_state_table() {
    return &thread_state_table_;
  }

  const tables::SchedSliceTable& sched_slice_table() const {
    return sched_slice_table_;
  }
  tables::SchedSliceTable* mutable_sched_slice_table() {
    return &sched_slice_table_;
  }

  const tables::SliceTable& slice_table() const { return slice_table_; }
  tables::SliceTable* mutable_slice_table() { return &slice_table_; }

  const tables::SpuriousSchedWakeupTable& spurious_sched_wakeup_table() const {
    return spurious_sched_wakeup_table_;
  }
  tables::SpuriousSchedWakeupTable* mutable_spurious_sched_wakeup_table() {
    return &spurious_sched_wakeup_table_;
  }

  const tables::FlowTable& flow_table() const { return flow_table_; }
  tables::FlowTable* mutable_flow_table() { return &flow_table_; }

  const VirtualTrackSlices& virtual_track_slices() const {
    return virtual_track_slices_;
  }
  VirtualTrackSlices* mutable_virtual_track_slices() {
    return &virtual_track_slices_;
  }

  const tables::CounterTable& counter_table() const { return counter_table_; }
  tables::CounterTable* mutable_counter_table() { return &counter_table_; }

  const SqlStats& sql_stats() const { return sql_stats_; }
  SqlStats* mutable_sql_stats() { return &sql_stats_; }

  const tables::AndroidLogTable& android_log_table() const {
    return android_log_table_;
  }
  tables::AndroidLogTable* mutable_android_log_table() {
    return &android_log_table_;
  }

  const tables::AndroidDumpstateTable& android_dumpstate_table() const {
    return android_dumpstate_table_;
  }

  tables::AndroidDumpstateTable* mutable_android_dumpstate_table() {
    return &android_dumpstate_table_;
  }

  const tables::AndroidKeyEventsTable& android_key_events_table() const {
    return android_key_events_table_;
  }
  tables::AndroidKeyEventsTable* mutable_android_key_events_table() {
    return &android_key_events_table_;
  }

  const tables::AndroidMotionEventsTable& android_motion_events_table() const {
    return android_motion_events_table_;
  }
  tables::AndroidMotionEventsTable* mutable_android_motion_events_table() {
    return &android_motion_events_table_;
  }

  const tables::AndroidInputEventDispatchTable&
  android_input_event_dispatch_table() const {
    return android_input_event_dispatch_table_;
  }
  tables::AndroidInputEventDispatchTable*
  mutable_android_input_event_dispatch_table() {
    return &android_input_event_dispatch_table_;
  }

  const StatsMap& stats() const { return stats_; }

  const tables::MetadataTable& metadata_table() const {
    return metadata_table_;
  }
  tables::MetadataTable* mutable_metadata_table() { return &metadata_table_; }

  const tables::ClockSnapshotTable& clock_snapshot_table() const {
    return clock_snapshot_table_;
  }
  tables::ClockSnapshotTable* mutable_clock_snapshot_table() {
    return &clock_snapshot_table_;
  }

  const tables::ArgTable& arg_table() const { return arg_table_; }
  tables::ArgTable* mutable_arg_table() { return &arg_table_; }

  const tables::ChromeRawTable& chrome_raw_table() const {
    return chrome_raw_table_;
  }
  tables::ChromeRawTable* mutable_chrome_raw_table() {
    return &chrome_raw_table_;
  }

  const tables::FtraceEventTable& ftrace_event_table() const {
    return ftrace_event_table_;
  }
  tables::FtraceEventTable* mutable_ftrace_event_table() {
    return &ftrace_event_table_;
  }

  const tables::MachineTable& machine_table() const { return machine_table_; }
  tables::MachineTable* mutable_machine_table() { return &machine_table_; }

  const tables::CpuTable& cpu_table() const { return cpu_table_; }
  tables::CpuTable* mutable_cpu_table() { return &cpu_table_; }

  const tables::CpuFreqTable& cpu_freq_table() const { return cpu_freq_table_; }
  tables::CpuFreqTable* mutable_cpu_freq_table() { return &cpu_freq_table_; }

  const tables::StackProfileMappingTable& stack_profile_mapping_table() const {
    return stack_profile_mapping_table_;
  }
  tables::StackProfileMappingTable* mutable_stack_profile_mapping_table() {
    return &stack_profile_mapping_table_;
  }

  const tables::StackProfileFrameTable& stack_profile_frame_table() const {
    return stack_profile_frame_table_;
  }
  tables::StackProfileFrameTable* mutable_stack_profile_frame_table() {
    return &stack_profile_frame_table_;
  }

  const tables::StackProfileCallsiteTable& stack_profile_callsite_table()
      const {
    return stack_profile_callsite_table_;
  }
  tables::StackProfileCallsiteTable* mutable_stack_profile_callsite_table() {
    return &stack_profile_callsite_table_;
  }

  const tables::HeapProfileAllocationTable& heap_profile_allocation_table()
      const {
    return heap_profile_allocation_table_;
  }
  tables::HeapProfileAllocationTable* mutable_heap_profile_allocation_table() {
    return &heap_profile_allocation_table_;
  }

  const tables::PackageListTable& package_list_table() const {
    return package_list_table_;
  }
  tables::PackageListTable* mutable_package_list_table() {
    return &package_list_table_;
  }

  const tables::AndroidGameInterventionListTable&
  android_game_intervention_list_table() const {
    return android_game_intervention_list_table_;
  }
  tables::AndroidGameInterventionListTable*
  mutable_android_game_intervenion_list_table() {
    return &android_game_intervention_list_table_;
  }

  const tables::ProfilerSmapsTable& profiler_smaps_table() const {
    return profiler_smaps_table_;
  }
  tables::ProfilerSmapsTable* mutable_profiler_smaps_table() {
    return &profiler_smaps_table_;
  }

  const tables::TraceFileTable& trace_file_table() const {
    return trace_file_table_;
  }
  tables::TraceFileTable* mutable_trace_file_table() {
    return &trace_file_table_;
  }

  const tables::CpuProfileStackSampleTable& cpu_profile_stack_sample_table()
      const {
    return cpu_profile_stack_sample_table_;
  }
  tables::CpuProfileStackSampleTable* mutable_cpu_profile_stack_sample_table() {
    return &cpu_profile_stack_sample_table_;
  }

  const tables::PerfSessionTable& perf_session_table() const {
    return perf_session_table_;
  }
  tables::PerfSessionTable* mutable_perf_session_table() {
    return &perf_session_table_;
  }

  const tables::PerfSampleTable& perf_sample_table() const {
    return perf_sample_table_;
  }
  tables::PerfSampleTable* mutable_perf_sample_table() {
    return &perf_sample_table_;
  }

  const tables::InstrumentsSampleTable& instruments_sample_table() const {
    return instruments_sample_table_;
  }
  tables::InstrumentsSampleTable* mutable_instruments_sample_table() {
    return &instruments_sample_table_;
  }

  const tables::SymbolTable& symbol_table() const { return symbol_table_; }

  tables::SymbolTable* mutable_symbol_table() { return &symbol_table_; }

  const tables::HeapGraphObjectTable& heap_graph_object_table() const {
    return heap_graph_object_table_;
  }

  tables::HeapGraphObjectTable* mutable_heap_graph_object_table() {
    return &heap_graph_object_table_;
  }
  const tables::HeapGraphClassTable& heap_graph_class_table() const {
    return heap_graph_class_table_;
  }

  tables::HeapGraphClassTable* mutable_heap_graph_class_table() {
    return &heap_graph_class_table_;
  }

  const tables::HeapGraphReferenceTable& heap_graph_reference_table() const {
    return heap_graph_reference_table_;
  }

  tables::HeapGraphReferenceTable* mutable_heap_graph_reference_table() {
    return &heap_graph_reference_table_;
  }

  const tables::VulkanMemoryAllocationsTable& vulkan_memory_allocations_table()
      const {
    return vulkan_memory_allocations_table_;
  }

  tables::VulkanMemoryAllocationsTable*
  mutable_vulkan_memory_allocations_table() {
    return &vulkan_memory_allocations_table_;
  }

  const tables::MemorySnapshotTable& memory_snapshot_table() const {
    return memory_snapshot_table_;
  }
  tables::MemorySnapshotTable* mutable_memory_snapshot_table() {
    return &memory_snapshot_table_;
  }

  const tables::ProcessMemorySnapshotTable& process_memory_snapshot_table()
      const {
    return process_memory_snapshot_table_;
  }
  tables::ProcessMemorySnapshotTable* mutable_process_memory_snapshot_table() {
    return &process_memory_snapshot_table_;
  }

  const tables::MemorySnapshotNodeTable& memory_snapshot_node_table() const {
    return memory_snapshot_node_table_;
  }
  tables::MemorySnapshotNodeTable* mutable_memory_snapshot_node_table() {
    return &memory_snapshot_node_table_;
  }

  const tables::MemorySnapshotEdgeTable& memory_snapshot_edge_table() const {
    return memory_snapshot_edge_table_;
  }
  tables::MemorySnapshotEdgeTable* mutable_memory_snapshot_edge_table() {
    return &memory_snapshot_edge_table_;
  }

  const tables::AndroidNetworkPacketsTable& android_network_packets_table()
      const {
    return android_network_packets_table_;
  }
  tables::AndroidNetworkPacketsTable* mutable_android_network_packets_table() {
    return &android_network_packets_table_;
  }

  const tables::V8IsolateTable& v8_isolate_table() const {
    return v8_isolate_table_;
  }
  tables::V8IsolateTable* mutable_v8_isolate_table() {
    return &v8_isolate_table_;
  }
  const tables::V8JsScriptTable& v8_js_script_table() const {
    return v8_js_script_table_;
  }
  tables::V8JsScriptTable* mutable_v8_js_script_table() {
    return &v8_js_script_table_;
  }
  const tables::V8WasmScriptTable& v8_wasm_script_table() const {
    return v8_wasm_script_table_;
  }
  tables::V8WasmScriptTable* mutable_v8_wasm_script_table() {
    return &v8_wasm_script_table_;
  }
  const tables::V8JsFunctionTable& v8_js_function_table() const {
    return v8_js_function_table_;
  }
  tables::V8JsFunctionTable* mutable_v8_js_function_table() {
    return &v8_js_function_table_;
  }
  const tables::V8JsCodeTable& v8_js_code_table() const {
    return v8_js_code_table_;
  }
  tables::V8JsCodeTable* mutable_v8_js_code_table() {
    return &v8_js_code_table_;
  }
  const tables::V8InternalCodeTable& v8_internal_code_table() const {
    return v8_internal_code_table_;
  }
  tables::V8InternalCodeTable* mutable_v8_internal_code_table() {
    return &v8_internal_code_table_;
  }
  const tables::V8WasmCodeTable& v8_wasm_code_table() const {
    return v8_wasm_code_table_;
  }
  tables::V8WasmCodeTable* mutable_v8_wasm_code_table() {
    return &v8_wasm_code_table_;
  }
  const tables::V8RegexpCodeTable& v8_regexp_code_table() const {
    return v8_regexp_code_table_;
  }
  tables::V8RegexpCodeTable* mutable_v8_regexp_code_table() {
    return &v8_regexp_code_table_;
  }

  const tables::EtmV4ConfigurationTable& etm_v4_configuration_table() const {
    return etm_v4_configuration_table_;
  }
  tables::EtmV4ConfigurationTable* mutable_etm_v4_configuration_table() {
    return &etm_v4_configuration_table_;
  }
  const std::vector<std::unique_ptr<Destructible>>& etm_v4_configuration_data()
      const {
    return etm_v4_configuration_data_;
  }
  std::vector<std::unique_ptr<Destructible>>*
  mutable_etm_v4_configuration_data() {
    return &etm_v4_configuration_data_;
  }
  const tables::EtmV4SessionTable& etm_v4_session_table() const {
    return etm_v4_session_table_;
  }
  tables::EtmV4SessionTable* mutable_etm_v4_session_table() {
    return &etm_v4_session_table_;
  }
  const tables::EtmV4TraceTable& etm_v4_trace_table() const {
    return etm_v4_trace_table_;
  }
  tables::EtmV4TraceTable* mutable_etm_v4_trace_table() {
    return &etm_v4_trace_table_;
  }
  const std::vector<TraceBlobView>& etm_v4_trace_data() const {
    return etm_v4_trace_data_;
  }
  std::vector<TraceBlobView>* mutable_etm_v4_trace_data() {
    return &etm_v4_trace_data_;
  }
  const tables::FileTable& file_table() const { return file_table_; }
  tables::FileTable* mutable_file_table() { return &file_table_; }
  const tables::ElfFileTable& elf_file_table() const { return elf_file_table_; }
  tables::ElfFileTable* mutable_elf_file_table() { return &elf_file_table_; }

  const tables::JitCodeTable& jit_code_table() const { return jit_code_table_; }
  tables::JitCodeTable* mutable_jit_code_table() { return &jit_code_table_; }

  const tables::JitFrameTable& jit_frame_table() const {
    return jit_frame_table_;
  }
  tables::JitFrameTable* mutable_jit_frame_table() { return &jit_frame_table_; }

  tables::MmapRecordTable* mutable_mmap_record_table() {
    return &mmap_record_table_;
  }
  const tables::MmapRecordTable& mmap_record_table() const {
    return mmap_record_table_;
  }
  const tables::SpeRecordTable& spe_record_table() const {
    return spe_record_table_;
  }
  tables::SpeRecordTable* mutable_spe_record_table() {
    return &spe_record_table_;
  }

  const tables::InputMethodClientsTable& inputmethod_clients_table() const {
    return inputmethod_clients_table_;
  }
  tables::InputMethodClientsTable* mutable_inputmethod_clients_table() {
    return &inputmethod_clients_table_;
  }

  const tables::InputMethodManagerServiceTable&
  inputmethod_manager_service_table() const {
    return inputmethod_manager_service_table_;
  }
  tables::InputMethodManagerServiceTable*
  mutable_inputmethod_manager_service_table() {
    return &inputmethod_manager_service_table_;
  }

  const tables::InputMethodServiceTable& inputmethod_service_table() const {
    return inputmethod_service_table_;
  }
  tables::InputMethodServiceTable* mutable_inputmethod_service_table() {
    return &inputmethod_service_table_;
  }

  const tables::SurfaceFlingerLayersSnapshotTable&
  surfaceflinger_layers_snapshot_table() const {
    return surfaceflinger_layers_snapshot_table_;
  }
  tables::SurfaceFlingerLayersSnapshotTable*
  mutable_surfaceflinger_layers_snapshot_table() {
    return &surfaceflinger_layers_snapshot_table_;
  }

  const tables::SurfaceFlingerLayerTable& surfaceflinger_layer_table() const {
    return surfaceflinger_layer_table_;
  }
  tables::SurfaceFlingerLayerTable* mutable_surfaceflinger_layer_table() {
    return &surfaceflinger_layer_table_;
  }

  const tables::SurfaceFlingerTransactionsTable&
  surfaceflinger_transactions_table() const {
    return surfaceflinger_transactions_table_;
  }
  tables::SurfaceFlingerTransactionsTable*
  mutable_surfaceflinger_transactions_table() {
    return &surfaceflinger_transactions_table_;
  }

  const tables::ViewCaptureTable& viewcapture_table() const {
    return viewcapture_table_;
  }
  tables::ViewCaptureTable* mutable_viewcapture_table() {
    return &viewcapture_table_;
  }

  const tables::ViewCaptureViewTable& viewcapture_view_table() const {
    return viewcapture_view_table_;
  }
  tables::ViewCaptureViewTable* mutable_viewcapture_view_table() {
    return &viewcapture_view_table_;
  }

  const tables::ViewCaptureInternedDataTable& viewcapture_interned_data_table()
      const {
    return viewcapture_interned_data_table_;
  }
  tables::ViewCaptureInternedDataTable*
  mutable_viewcapture_interned_data_table() {
    return &viewcapture_interned_data_table_;
  }

  const tables::WindowManagerTable& windowmanager_table() const {
    return windowmanager_table_;
  }
  tables::WindowManagerTable* mutable_windowmanager_table() {
    return &windowmanager_table_;
  }

  const tables::WindowManagerShellTransitionsTable&
  window_manager_shell_transitions_table() const {
    return window_manager_shell_transitions_table_;
  }
  tables::WindowManagerShellTransitionsTable*
  mutable_window_manager_shell_transitions_table() {
    return &window_manager_shell_transitions_table_;
  }

  const tables::WindowManagerShellTransitionHandlersTable&
  window_manager_shell_transition_handlers_table() const {
    return window_manager_shell_transition_handlers_table_;
  }
  tables::WindowManagerShellTransitionHandlersTable*
  mutable_window_manager_shell_transition_handlers_table() {
    return &window_manager_shell_transition_handlers_table_;
  }

  const tables::WindowManagerShellTransitionProtosTable&
  window_manager_shell_transition_protos_table() const {
    return window_manager_shell_transition_protos_table_;
  }
  tables::WindowManagerShellTransitionProtosTable*
  mutable_window_manager_shell_transition_protos_table() {
    return &window_manager_shell_transition_protos_table_;
  }

  const tables::ProtoLogTable& protolog_table() const {
    return protolog_table_;
  }
  tables::ProtoLogTable* mutable_protolog_table() { return &protolog_table_; }

  const tables::ExperimentalProtoPathTable& experimental_proto_path_table()
      const {
    return experimental_proto_path_table_;
  }
  tables::ExperimentalProtoPathTable* mutable_experimental_proto_path_table() {
    return &experimental_proto_path_table_;
  }

  const tables::ExperimentalProtoContentTable&
  experimental_proto_content_table() const {
    return experimental_proto_content_table_;
  }
  tables::ExperimentalProtoContentTable*
  mutable_experimental_proto_content_table() {
    return &experimental_proto_content_table_;
  }

  const tables::ExpMissingChromeProcTable&
  experimental_missing_chrome_processes_table() const {
    return experimental_missing_chrome_processes_table_;
  }
  tables::ExpMissingChromeProcTable*
  mutable_experimental_missing_chrome_processes_table() {
    return &experimental_missing_chrome_processes_table_;
  }

  const StringPool& string_pool() const { return string_pool_; }
  StringPool* mutable_string_pool() { return &string_pool_; }

  // Number of interned strings in the pool. Includes the empty string w/ ID=0.
  size_t string_count() const { return string_pool_.size(); }

  base::Status ExtractArg(uint32_t arg_set_id,
                          const char* key,
                          std::optional<Variadic>* result) const {
    const auto& args = arg_table();
    Query q;
    q.constraints = {args.arg_set_id().eq(arg_set_id), args.key().eq(key)};
    auto it = args.FilterToIterator(q);
    if (!it) {
      *result = std::nullopt;
      return base::OkStatus();
    }
    *result = GetArgValue(it.row_number().row_number());
    if (++it) {
      return base::ErrStatus(
          "EXTRACT_ARG: received multiple args matching arg set id and key");
    }
    return base::OkStatus();
  }

  Variadic GetArgValue(uint32_t row) const {
    auto rr = arg_table_[row];

    Variadic v = Variadic::Null();
    v.type = *GetVariadicTypeForId(rr.value_type());
    switch (v.type) {
      case Variadic::Type::kBool:
        v.bool_value = static_cast<bool>(*rr.int_value());
        break;
      case Variadic::Type::kInt:
        v.int_value = *rr.int_value();
        break;
      case Variadic::Type::kUint:
        v.uint_value = static_cast<uint64_t>(*rr.int_value());
        break;
      case Variadic::Type::kString: {
        auto opt_value = rr.string_value();
        v.string_value = opt_value ? *opt_value : kNullStringId;
        break;
      }
      case Variadic::Type::kPointer:
        v.pointer_value = static_cast<uint64_t>(*rr.int_value());
        break;
      case Variadic::Type::kReal:
        v.real_value = *rr.real_value();
        break;
      case Variadic::Type::kJson: {
        auto opt_value = rr.string_value();
        v.json_value = opt_value ? *opt_value : kNullStringId;
        break;
      }
      case Variadic::Type::kNull:
        break;
    }
    return v;
  }

  StringId GetIdForVariadicType(Variadic::Type type) const {
    return variadic_type_ids_[type];
  }

  std::optional<Variadic::Type> GetVariadicTypeForId(StringId id) const {
    auto it =
        std::find(variadic_type_ids_.begin(), variadic_type_ids_.end(), id);
    if (it == variadic_type_ids_.end())
      return std::nullopt;

    int64_t idx = std::distance(variadic_type_ids_.begin(), it);
    return static_cast<Variadic::Type>(idx);
  }

 private:
  using StringHash = uint64_t;

  TraceStorage(const TraceStorage&) = delete;
  TraceStorage& operator=(const TraceStorage&) = delete;

  TraceStorage(TraceStorage&&) = delete;
  TraceStorage& operator=(TraceStorage&&) = delete;

  friend etm::TargetMemory;
  Destructible* etm_target_memory() { return etm_target_memory_.get(); }
  void set_etm_target_memory(std::unique_ptr<Destructible> target_memory) {
    etm_target_memory_ = std::move(target_memory);
  }

  // One entry for each unique string in the trace.
  StringPool string_pool_;

  // Stats about parsing the trace.
  StatsMap stats_{};

  // Extra data extracted from the trace. Includes:
  // * metadata from chrome and benchmarking infrastructure
  // * descriptions of android packages
  tables::MetadataTable metadata_table_{&string_pool_};

  // Contains data from all the clock snapshots in the trace.
  tables::ClockSnapshotTable clock_snapshot_table_{&string_pool_};

  // Metadata for tracks.
  tables::TrackTable track_table_{&string_pool_};
  tables::ThreadStateTable thread_state_table_{&string_pool_};

  // Track tables for counter events.
  tables::GpuCounterGroupTable gpu_counter_group_table_{&string_pool_};

  // Args for all other tables.
  tables::ArgTable arg_table_{&string_pool_};

  // Information about all the threads and processes in the trace.
  tables::ThreadTable thread_table_{&string_pool_};
  tables::ProcessTable process_table_{&string_pool_};
  tables::FiledescriptorTable filedescriptor_table_{&string_pool_};

  // Slices coming from userspace events (e.g. Chromium TRACE_EVENT macros).
  tables::SliceTable slice_table_{&string_pool_};

  // Flow events from userspace events (e.g. Chromium TRACE_EVENT macros).
  tables::FlowTable flow_table_{&string_pool_};

  // Slices from CPU scheduling data.
  tables::SchedSliceTable sched_slice_table_{&string_pool_};

  tables::SpuriousSchedWakeupTable spurious_sched_wakeup_table_{&string_pool_};

  // Additional attributes for virtual track slices (sub-type of
  // NestableSlices).
  VirtualTrackSlices virtual_track_slices_;

  // The values from the Counter events from the trace. This includes CPU
  // frequency events as well systrace trace_marker counter events.
  tables::CounterTable counter_table_{&string_pool_};

  SqlStats sql_stats_;

  tables::ChromeRawTable chrome_raw_table_{&string_pool_};
  tables::FtraceEventTable ftrace_event_table_{&string_pool_};

  tables::MachineTable machine_table_{&string_pool_};

  tables::CpuTable cpu_table_{&string_pool_};

  tables::CpuFreqTable cpu_freq_table_{&string_pool_};

  tables::AndroidLogTable android_log_table_{&string_pool_};

  tables::AndroidDumpstateTable android_dumpstate_table_{&string_pool_};

  tables::AndroidKeyEventsTable android_key_events_table_{&string_pool_};
  tables::AndroidMotionEventsTable android_motion_events_table_{&string_pool_};
  tables::AndroidInputEventDispatchTable android_input_event_dispatch_table_{
      &string_pool_};

  tables::StackProfileMappingTable stack_profile_mapping_table_{&string_pool_};
  tables::StackProfileFrameTable stack_profile_frame_table_{&string_pool_};
  tables::StackProfileCallsiteTable stack_profile_callsite_table_{
      &string_pool_};
  tables::HeapProfileAllocationTable heap_profile_allocation_table_{
      &string_pool_};
  tables::CpuProfileStackSampleTable cpu_profile_stack_sample_table_{
      &string_pool_};
  tables::PerfSessionTable perf_session_table_{&string_pool_};
  tables::PerfSampleTable perf_sample_table_{&string_pool_};
  tables::InstrumentsSampleTable instruments_sample_table_{&string_pool_};
  tables::PackageListTable package_list_table_{&string_pool_};
  tables::AndroidGameInterventionListTable
      android_game_intervention_list_table_{&string_pool_};
  tables::ProfilerSmapsTable profiler_smaps_table_{&string_pool_};

  tables::TraceFileTable trace_file_table_{&string_pool_};

  // Symbol tables (mappings from frames to symbol names)
  tables::SymbolTable symbol_table_{&string_pool_};
  tables::HeapGraphObjectTable heap_graph_object_table_{&string_pool_};
  tables::HeapGraphClassTable heap_graph_class_table_{&string_pool_};
  tables::HeapGraphReferenceTable heap_graph_reference_table_{&string_pool_};

  tables::VulkanMemoryAllocationsTable vulkan_memory_allocations_table_{
      &string_pool_};

  // Metadata for memory snapshot.
  tables::MemorySnapshotTable memory_snapshot_table_{&string_pool_};
  tables::ProcessMemorySnapshotTable process_memory_snapshot_table_{
      &string_pool_};
  tables::MemorySnapshotNodeTable memory_snapshot_node_table_{&string_pool_};
  tables::MemorySnapshotEdgeTable memory_snapshot_edge_table_{&string_pool_};

  // AndroidNetworkPackets tables
  tables::AndroidNetworkPacketsTable android_network_packets_table_{
      &string_pool_, &slice_table_};

  // V8 tables
  tables::V8IsolateTable v8_isolate_table_{&string_pool_};
  tables::V8JsScriptTable v8_js_script_table_{&string_pool_};
  tables::V8WasmScriptTable v8_wasm_script_table_{&string_pool_};
  tables::V8JsFunctionTable v8_js_function_table_{&string_pool_};
  tables::V8JsCodeTable v8_js_code_table_{&string_pool_};
  tables::V8InternalCodeTable v8_internal_code_table_{&string_pool_};
  tables::V8WasmCodeTable v8_wasm_code_table_{&string_pool_};
  tables::V8RegexpCodeTable v8_regexp_code_table_{&string_pool_};

  // Jit tables
  tables::JitCodeTable jit_code_table_{&string_pool_};
  tables::JitFrameTable jit_frame_table_{&string_pool_};

  // ETM tables
  tables::EtmV4ConfigurationTable etm_v4_configuration_table_{&string_pool_};
  // Indexed by tables::EtmV4ConfigurationTable::Id
  std::vector<std::unique_ptr<Destructible>> etm_v4_configuration_data_;
  tables::EtmV4SessionTable etm_v4_session_table_{&string_pool_};
  tables::EtmV4TraceTable etm_v4_trace_table_{&string_pool_};
  // Indexed by tables::EtmV4TraceTable::Id
  std::vector<TraceBlobView> etm_v4_trace_data_;
  std::unique_ptr<Destructible> etm_target_memory_;
  tables::FileTable file_table_{&string_pool_};
  tables::ElfFileTable elf_file_table_{&string_pool_};

  // Perf tables
  tables::MmapRecordTable mmap_record_table_{&string_pool_};
  tables::SpeRecordTable spe_record_table_{&string_pool_};

  // Winscope tables
  tables::InputMethodClientsTable inputmethod_clients_table_{&string_pool_};
  tables::InputMethodManagerServiceTable inputmethod_manager_service_table_{
      &string_pool_};
  tables::InputMethodServiceTable inputmethod_service_table_{&string_pool_};
  tables::SurfaceFlingerLayersSnapshotTable
      surfaceflinger_layers_snapshot_table_{&string_pool_};
  tables::SurfaceFlingerLayerTable surfaceflinger_layer_table_{&string_pool_};
  tables::SurfaceFlingerTransactionsTable surfaceflinger_transactions_table_{
      &string_pool_};
  tables::ViewCaptureTable viewcapture_table_{&string_pool_};
  tables::ViewCaptureViewTable viewcapture_view_table_{&string_pool_};
  tables::ViewCaptureInternedDataTable viewcapture_interned_data_table_{
      &string_pool_};
  tables::WindowManagerTable windowmanager_table_{&string_pool_};
  tables::WindowManagerShellTransitionsTable
      window_manager_shell_transitions_table_{&string_pool_};
  tables::WindowManagerShellTransitionHandlersTable
      window_manager_shell_transition_handlers_table_{&string_pool_};
  tables::WindowManagerShellTransitionProtosTable
      window_manager_shell_transition_protos_table_{&string_pool_};
  tables::ProtoLogTable protolog_table_{&string_pool_};

  tables::ExperimentalProtoPathTable experimental_proto_path_table_{
      &string_pool_};
  tables::ExperimentalProtoContentTable experimental_proto_content_table_{
      &string_pool_};

  tables::ExpMissingChromeProcTable
      experimental_missing_chrome_processes_table_{&string_pool_};

  // The below array allow us to map between enums and their string
  // representations.
  std::array<StringId, Variadic::kMaxType + 1> variadic_type_ids_;
};

}  // namespace perfetto::trace_processor

template <>
struct std::hash<::perfetto::trace_processor::BaseId> {
  using argument_type = ::perfetto::trace_processor::BaseId;
  using result_type = size_t;

  result_type operator()(const argument_type& r) const {
    return std::hash<uint32_t>{}(r.value);
  }
};

template <>
struct std::hash<::perfetto::trace_processor::TrackId>
    : std::hash<::perfetto::trace_processor::BaseId> {};
template <>
struct std::hash<::perfetto::trace_processor::MappingId>
    : std::hash<::perfetto::trace_processor::BaseId> {};
template <>
struct std::hash<::perfetto::trace_processor::CallsiteId>
    : std::hash<::perfetto::trace_processor::BaseId> {};
template <>
struct std::hash<::perfetto::trace_processor::FrameId>
    : std::hash<::perfetto::trace_processor::BaseId> {};
template <>
struct std::hash<::perfetto::trace_processor::tables::HeapGraphObjectTable::Id>
    : std::hash<::perfetto::trace_processor::BaseId> {};
template <>
struct std::hash<::perfetto::trace_processor::tables::V8IsolateTable::Id>
    : std::hash<::perfetto::trace_processor::BaseId> {};
template <>
struct std::hash<::perfetto::trace_processor::tables::JitCodeTable::Id>
    : std::hash<::perfetto::trace_processor::BaseId> {};

template <>
struct std::hash<
    ::perfetto::trace_processor::tables::StackProfileFrameTable::Row> {
  using argument_type =
      ::perfetto::trace_processor::tables::StackProfileFrameTable::Row;
  using result_type = size_t;

  result_type operator()(const argument_type& r) const {
    return std::hash<::perfetto::trace_processor::StringId>{}(r.name) ^
           std::hash<std::optional<::perfetto::trace_processor::MappingId>>{}(
               r.mapping) ^
           std::hash<int64_t>{}(r.rel_pc);
  }
};

template <>
struct std::hash<
    ::perfetto::trace_processor::tables::StackProfileCallsiteTable::Row> {
  using argument_type =
      ::perfetto::trace_processor::tables::StackProfileCallsiteTable::Row;
  using result_type = size_t;

  result_type operator()(const argument_type& r) const {
    return std::hash<int64_t>{}(r.depth) ^
           std::hash<std::optional<::perfetto::trace_processor::CallsiteId>>{}(
               r.parent_id) ^
           std::hash<::perfetto::trace_processor::FrameId>{}(r.frame_id);
  }
};

template <>
struct std::hash<
    ::perfetto::trace_processor::tables::StackProfileMappingTable::Row> {
  using argument_type =
      ::perfetto::trace_processor::tables::StackProfileMappingTable::Row;
  using result_type = size_t;

  result_type operator()(const argument_type& r) const {
    return std::hash<::perfetto::trace_processor::StringId>{}(r.build_id) ^
           std::hash<int64_t>{}(r.exact_offset) ^
           std::hash<int64_t>{}(r.start_offset) ^
           std::hash<int64_t>{}(r.start) ^ std::hash<int64_t>{}(r.end) ^
           std::hash<int64_t>{}(r.load_bias) ^
           std::hash<::perfetto::trace_processor::StringId>{}(r.name);
  }
};

#endif  // SRC_TRACE_PROCESSOR_STORAGE_TRACE_STORAGE_H_
