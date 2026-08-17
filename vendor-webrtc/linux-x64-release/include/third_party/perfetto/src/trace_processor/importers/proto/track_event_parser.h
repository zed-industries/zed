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

#ifndef SRC_TRACE_PROCESSOR_IMPORTERS_PROTO_TRACK_EVENT_PARSER_H_
#define SRC_TRACE_PROCESSOR_IMPORTERS_PROTO_TRACK_EVENT_PARSER_H_

#include <array>
#include <cstdint>
#include <optional>
#include <vector>

#include "perfetto/protozero/field.h"
#include "src/trace_processor/importers/common/parser_types.h"
#include "src/trace_processor/importers/common/slice_tracker.h"
#include "src/trace_processor/importers/common/trace_parser.h"
#include "src/trace_processor/importers/proto/active_chrome_processes_tracker.h"
#include "src/trace_processor/importers/proto/chrome_string_lookup.h"
#include "src/trace_processor/storage/trace_storage.h"
#include "src/trace_processor/util/proto_to_args_parser.h"

namespace Json {
class Value;
}

namespace perfetto::trace_processor {

// Field numbers to be added to args table automatically via reflection
//
// TODO(ddrone): replace with a predicate on field id to import new fields
// automatically
static constexpr uint16_t kReflectFields[] = {
    24, 25, 26, 27, 28, 29, 32, 33, 34, 35, 38, 39, 40, 41, 43, 49, 50};

class PacketSequenceStateGeneration;
class TraceProcessorContext;
class TrackEventTracker;

class TrackEventParser {
 public:
  TrackEventParser(TraceProcessorContext*, TrackEventTracker*);

  void ParseTrackDescriptor(int64_t packet_timestamp,
                            protozero::ConstBytes,
                            uint32_t packet_sequence_id);
  UniquePid ParseProcessDescriptor(int64_t packet_timestamp,
                                   protozero::ConstBytes);
  UniqueTid ParseThreadDescriptor(protozero::ConstBytes);

  void ParseTrackEvent(int64_t ts,
                       const TrackEventData* event_data,
                       protozero::ConstBytes,
                       uint32_t packet_sequence_id);

  void NotifyEndOfFile();

 private:
  class EventImporter;

  void ParseChromeProcessDescriptor(UniquePid, protozero::ConstBytes);
  void ParseChromeThreadDescriptor(UniqueTid, protozero::ConstBytes);
  void ParseCounterDescriptor(TrackId, protozero::ConstBytes);
  void AddActiveProcess(int64_t packet_timestamp, int32_t pid);

  // Reflection-based proto TrackEvent field parser.
  util::ProtoToArgsParser args_parser_;

  TraceProcessorContext* context_;
  TrackEventTracker* track_event_tracker_;

  const StringId counter_name_thread_time_id_;
  const StringId counter_name_thread_instruction_count_id_;
  const StringId task_file_name_args_key_id_;
  const StringId task_function_name_args_key_id_;
  const StringId task_line_number_args_key_id_;
  const StringId log_message_body_key_id_;
  const StringId log_message_source_location_function_name_key_id_;
  const StringId log_message_source_location_file_name_key_id_;
  const StringId log_message_source_location_line_number_key_id_;
  const StringId log_message_priority_id_;
  const StringId source_location_function_name_key_id_;
  const StringId source_location_file_name_key_id_;
  const StringId source_location_line_number_key_id_;
  const StringId raw_legacy_event_id_;
  const StringId legacy_event_passthrough_utid_id_;
  const StringId legacy_event_category_key_id_;
  const StringId legacy_event_name_key_id_;
  const StringId legacy_event_phase_key_id_;
  const StringId legacy_event_duration_ns_key_id_;
  const StringId legacy_event_thread_timestamp_ns_key_id_;
  const StringId legacy_event_thread_duration_ns_key_id_;
  const StringId legacy_event_thread_instruction_count_key_id_;
  const StringId legacy_event_thread_instruction_delta_key_id_;
  const StringId legacy_event_use_async_tts_key_id_;
  const StringId legacy_event_unscoped_id_key_id_;
  const StringId legacy_event_global_id_key_id_;
  const StringId legacy_event_local_id_key_id_;
  const StringId legacy_event_id_scope_key_id_;
  const StringId legacy_event_bind_id_key_id_;
  const StringId legacy_event_bind_to_enclosing_key_id_;
  const StringId legacy_event_flow_direction_key_id_;
  const StringId histogram_name_key_id_;
  const StringId flow_direction_value_in_id_;
  const StringId flow_direction_value_out_id_;
  const StringId flow_direction_value_inout_id_;
  const StringId chrome_legacy_ipc_class_args_key_id_;
  const StringId chrome_legacy_ipc_line_args_key_id_;
  const StringId chrome_host_app_package_name_id_;
  const StringId chrome_crash_trace_id_name_id_;
  const StringId chrome_process_label_flat_key_id_;
  const StringId chrome_process_type_id_;
  const StringId event_category_key_id_;
  const StringId event_name_key_id_;

  ChromeStringLookup chrome_string_lookup_;
  std::vector<uint32_t> reflect_fields_;
  ActiveChromeProcessesTracker active_chrome_processes_tracker_;
};

}  // namespace perfetto::trace_processor

#endif  // SRC_TRACE_PROCESSOR_IMPORTERS_PROTO_TRACK_EVENT_PARSER_H_
