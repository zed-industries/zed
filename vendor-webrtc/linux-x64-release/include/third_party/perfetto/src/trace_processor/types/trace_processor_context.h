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

#ifndef SRC_TRACE_PROCESSOR_TYPES_TRACE_PROCESSOR_CONTEXT_H_
#define SRC_TRACE_PROCESSOR_TYPES_TRACE_PROCESSOR_CONTEXT_H_

#include <cstdint>
#include <memory>
#include <optional>
#include <vector>

#include "perfetto/trace_processor/basic_types.h"
#include "src/trace_processor/tables/metadata_tables_py.h"
#include "src/trace_processor/types/destructible.h"

namespace perfetto::trace_processor {

class AndroidDumpstateEventParser;
class AndroidLogEventParser;
class ArgsTracker;
class ArgsTranslationTable;
class ArtMethodParser;
class TrackCompressor;
class ChunkedTraceReader;
class ClockConverter;
class ClockTracker;
class CpuTracker;
class DeobfuscationMappingTable;
class DescriptorPool;
class EtwModule;
class EventTracker;
class FlowTracker;
class ForwardingTraceParser;
class FtraceModule;
class FuchsiaRecordParser;
class GeckoTraceParser;
class GlobalArgsTracker;
class HeapGraphTracker;
class InstrumentsRowParser;
class JsonTraceParser;
class LegacyV8CpuProfileTracker;
class MachineTracker;
class MappingTracker;
class MetadataTracker;
class MultiMachineTraceManager;
class PacketAnalyzer;
class PerfRecordParser;
class PerfSampleTracker;
class PerfTextTraceParser;
class ProcessTracker;
class ProcessTrackTranslationTable;
class ProtoImporterModule;
class ProtoTraceParser;
class SchedEventTracker;
class SliceTracker;
class SliceTranslationTable;
class SpeRecordParser;
class StackProfileTracker;
class TraceFileTracker;
class TraceReaderRegistry;
class TraceSorter;
class TraceStorage;
class TrackEventModule;
class TrackTracker;

using MachineId = tables::MachineTable::Id;

class TraceProcessorContext {
 public:
  struct InitArgs {
    Config config;
    std::shared_ptr<TraceStorage> storage;
    uint32_t raw_machine_id = 0;
  };

  explicit TraceProcessorContext(const InitArgs&);

  // The default constructor is used in testing.
  TraceProcessorContext();
  ~TraceProcessorContext();

  TraceProcessorContext(TraceProcessorContext&&);
  TraceProcessorContext& operator=(TraceProcessorContext&&);

  Config config;

  // |storage| is shared among multiple contexts in multi-machine tracing.
  std::shared_ptr<TraceStorage> storage;

  std::unique_ptr<TraceReaderRegistry> reader_registry;

  // We might create multiple `ChunkedTraceReader` instances (e.g. one for each
  // file in a ZIP ). The instances are kept around here as some tokenizers
  // might keep state that is later needed after sorting.
  std::vector<std::unique_ptr<ChunkedTraceReader>> chunk_readers;

  // The sorter is used to sort trace data by timestamp and is shared among
  // multiple machines.
  std::shared_ptr<TraceSorter> sorter;

  // Keep the global tracker before the args tracker as we access the global
  // tracker in the destructor of the args tracker. Also keep it before other
  // trackers, as they may own ArgsTrackers themselves.
  std::shared_ptr<GlobalArgsTracker> global_args_tracker;
  std::unique_ptr<ArgsTracker> args_tracker;
  std::unique_ptr<ArgsTranslationTable> args_translation_table;

  std::unique_ptr<TrackTracker> track_tracker;
  std::unique_ptr<TrackCompressor> track_compressor;
  std::unique_ptr<SliceTracker> slice_tracker;
  std::unique_ptr<SliceTranslationTable> slice_translation_table;
  std::unique_ptr<FlowTracker> flow_tracker;
  std::unique_ptr<ProcessTracker> process_tracker;
  std::unique_ptr<ProcessTrackTranslationTable> process_track_translation_table;
  std::unique_ptr<EventTracker> event_tracker;
  std::unique_ptr<SchedEventTracker> sched_event_tracker;
  std::unique_ptr<ClockTracker> clock_tracker;
  std::unique_ptr<ClockConverter> clock_converter;
  std::unique_ptr<MappingTracker> mapping_tracker;
  std::unique_ptr<MachineTracker> machine_tracker;
  std::unique_ptr<PerfSampleTracker> perf_sample_tracker;
  std::unique_ptr<StackProfileTracker> stack_profile_tracker;
  std::unique_ptr<MetadataTracker> metadata_tracker;
  std::unique_ptr<CpuTracker> cpu_tracker;
  std::unique_ptr<TraceFileTracker> trace_file_tracker;
  std::unique_ptr<LegacyV8CpuProfileTracker> legacy_v8_cpu_profile_tracker;

  // These fields are stored as pointers to Destructible objects rather than
  // their actual type (a subclass of Destructible), as the concrete subclass
  // type is only available in storage_full target. To access these fields use
  // the GetOrCreate() method on their subclass type, e.g.
  // SyscallTracker::GetOrCreate(context)
  // clang-format off
  std::unique_ptr<Destructible> android_probes_tracker;                 // AndroidProbesTracker
  std::unique_ptr<Destructible> android_battery_stats_history_tracker;  // AndroidBatteryStatsHistoryStringTracker
  std::unique_ptr<Destructible> binder_tracker;                         // BinderTracker
  std::unique_ptr<Destructible> heap_graph_tracker;                     // HeapGraphTracker
  std::unique_ptr<Destructible> syscall_tracker;                        // SyscallTracker
  std::unique_ptr<Destructible> system_info_tracker;                    // SystemInfoTracker
  std::unique_ptr<Destructible> v4l2_tracker;                           // V4l2Tracker
  std::unique_ptr<Destructible> virtio_video_tracker;                   // VirtioVideoTracker
  std::unique_ptr<Destructible> systrace_parser;                        // SystraceParser
  std::unique_ptr<Destructible> thread_state_tracker;                   // ThreadStateTracker
  std::unique_ptr<Destructible> i2c_tracker;                            // I2CTracker
  std::unique_ptr<Destructible> perf_data_tracker;                      // PerfDataTracker
  std::unique_ptr<Destructible> content_analyzer;                       // ProtoContentAnalyzer
  std::unique_ptr<Destructible> shell_transitions_tracker;              // ShellTransitionsTracker
  std::unique_ptr<Destructible> protolog_messages_tracker;              // ProtoLogMessagesTracker
  std::unique_ptr<Destructible> ftrace_sched_tracker;                   // FtraceSchedEventTracker
  std::unique_ptr<Destructible> v8_tracker;                             // V8Tracker
  std::unique_ptr<Destructible> jit_tracker;                            // JitTracker
  std::unique_ptr<Destructible> protolog_message_decoder;               // ProtoLogMessageDecoder
  std::unique_ptr<Destructible> instruments_row_data_tracker;           // RowDataTracker
  std::unique_ptr<Destructible> perf_tracker;                           // PerfTracker
  std::unique_ptr<Destructible> etm_tracker;                            // EtmTracker
  std::unique_ptr<Destructible> elf_tracker;                            // ElfTracker
  std::unique_ptr<Destructible> file_tracker;                           // FileTracker
  // clang-format on

  std::unique_ptr<ProtoTraceParser> proto_trace_parser;

  // These fields are trace parsers which will be called by |forwarding_parser|
  // once the format of the trace is discovered. They are placed here as they
  // are only available in the lib target.
  std::unique_ptr<JsonTraceParser> json_trace_parser;
  std::unique_ptr<FuchsiaRecordParser> fuchsia_record_parser;
  std::unique_ptr<PerfRecordParser> perf_record_parser;
  std::unique_ptr<SpeRecordParser> spe_record_parser;
  std::unique_ptr<InstrumentsRowParser> instruments_row_parser;
  std::unique_ptr<AndroidDumpstateEventParser> android_dumpstate_event_parser;
  std::unique_ptr<AndroidLogEventParser> android_log_event_parser;
  std::unique_ptr<GeckoTraceParser> gecko_trace_parser;
  std::unique_ptr<ArtMethodParser> art_method_parser;
  std::unique_ptr<PerfTextTraceParser> perf_text_parser;

  // This field contains the list of proto descriptors that can be used by
  // reflection-based parsers.
  std::unique_ptr<DescriptorPool> descriptor_pool_;

  // The module at the index N is registered to handle field id N in
  // TracePacket.
  std::vector<std::vector<ProtoImporterModule*>> modules_by_field;
  std::vector<std::unique_ptr<ProtoImporterModule>> modules;
  // Pointers to modules from the modules vector that need to be called for
  // all fields.
  std::vector<ProtoImporterModule*> modules_for_all_fields;
  FtraceModule* ftrace_module = nullptr;
  EtwModule* etw_module = nullptr;
  TrackEventModule* track_module = nullptr;

  // Marks whether the uuid was read from the trace.
  // If the uuid was NOT read, the uuid will be made from the hash of the first
  // 4KB of the trace.
  bool uuid_found_in_trace = false;

  std::optional<MachineId> machine_id() const;

  // Manages the contexts for reading trace data emitted from remote machines.
  std::unique_ptr<MultiMachineTraceManager> multi_machine_trace_manager;
};

}  // namespace perfetto::trace_processor

#endif  // SRC_TRACE_PROCESSOR_TYPES_TRACE_PROCESSOR_CONTEXT_H_
