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

#ifndef SRC_TRACE_PROCESSOR_STORAGE_STATS_H_
#define SRC_TRACE_PROCESSOR_STORAGE_STATS_H_

#include <cstddef>

namespace perfetto::trace_processor::stats {

// Compile time list of parsing and processing stats.
// clang-format off
#define PERFETTO_TP_STATS(F)                                                   \
  F(android_br_parse_errors,              kSingle,  kError,    kTrace,    ""), \
  F(android_log_num_failed,               kSingle,  kError,    kTrace,    ""), \
  F(android_log_format_invalid,           kSingle,  kError,    kTrace,    ""), \
  F(android_log_num_skipped,              kSingle,  kInfo,     kTrace,    ""), \
  F(android_log_num_total,                kSingle,  kInfo,     kTrace,    ""), \
  F(deobfuscate_location_parse_error,     kSingle,  kError,    kTrace,    ""), \
  F(energy_breakdown_missing_values,      kSingle,  kError,    kAnalysis, ""), \
  F(energy_descriptor_invalid,            kSingle,  kError,    kAnalysis, ""), \
  F(entity_state_descriptor_invalid,      kSingle,  kError,    kAnalysis, ""), \
  F(entity_state_residency_invalid,       kSingle,  kError,    kAnalysis, ""), \
  F(entity_state_residency_lookup_failed, kSingle,  kError,    kAnalysis, ""), \
  F(energy_uid_breakdown_missing_values,  kSingle,  kError,    kAnalysis, ""), \
  F(frame_timeline_event_parser_errors,   kSingle,  kInfo,     kAnalysis, ""), \
  F(frame_timeline_unpaired_end_event,    kSingle,  kInfo,     kAnalysis, ""), \
  F(ftrace_bundle_tokenizer_errors,       kSingle,  kError,    kAnalysis, ""), \
  F(ftrace_cpu_bytes_begin,               kIndexed, kInfo,     kTrace,    ""), \
  F(ftrace_cpu_bytes_end,                 kIndexed, kInfo,     kTrace,    ""), \
  F(ftrace_cpu_bytes_delta,               kIndexed, kInfo,     kTrace,    ""), \
  F(ftrace_cpu_commit_overrun_begin,      kIndexed, kInfo,     kTrace,    ""), \
  F(ftrace_cpu_commit_overrun_end,        kIndexed, kInfo,     kTrace,    ""), \
  F(ftrace_cpu_commit_overrun_delta,      kIndexed, kError,    kTrace,    ""), \
  F(ftrace_cpu_dropped_events_begin,      kIndexed, kInfo,     kTrace,    ""), \
  F(ftrace_cpu_dropped_events_end,        kIndexed, kInfo,     kTrace,    ""), \
  F(ftrace_cpu_dropped_events_delta,      kIndexed, kError,    kTrace,    ""), \
  F(ftrace_cpu_entries_begin,             kIndexed, kInfo,     kTrace,    ""), \
  F(ftrace_cpu_entries_end,               kIndexed, kInfo,     kTrace,    ""), \
  F(ftrace_cpu_entries_delta,             kIndexed, kInfo,     kTrace,    ""), \
  F(ftrace_cpu_now_ts_begin,              kIndexed, kInfo,     kTrace,    ""), \
  F(ftrace_cpu_now_ts_end,                kIndexed, kInfo,     kTrace,    ""), \
  F(ftrace_cpu_oldest_event_ts_begin,     kIndexed, kInfo,     kTrace,    ""), \
  F(ftrace_cpu_oldest_event_ts_end,       kIndexed, kInfo,     kTrace,    ""), \
  F(ftrace_cpu_overrun_begin,             kIndexed, kInfo,     kTrace,    ""), \
  F(ftrace_cpu_overrun_end,               kIndexed, kInfo,     kTrace,    ""), \
  F(ftrace_cpu_overrun_delta,             kIndexed, kInfo,     kTrace,    ""), \
  F(ftrace_cpu_read_events_begin,         kIndexed, kInfo,     kTrace,    ""), \
  F(ftrace_cpu_read_events_end,           kIndexed, kInfo,     kTrace,    ""), \
  F(ftrace_cpu_read_events_delta,         kIndexed, kInfo,     kTrace,    ""), \
  F(ftrace_cpu_has_data_loss,             kIndexed, kDataLoss, kTrace,         \
       "Ftrace data for the given cpu has data losses and is therefore "       \
       "unreliable. The kernel buffer overwrote events between our reads "     \
       "in userspace. Try re-recording the trace with a bigger buffer "        \
       "(ftrace_config.buffer_size_kb), or with fewer enabled ftrace events."),\
  F(ftrace_kprobe_hits_begin,             kSingle,  kInfo,     kTrace,         \
       "The number of kretprobe hits at the beginning of the trace."),         \
  F(ftrace_kprobe_hits_end,               kSingle,  kInfo,     kTrace,         \
       "The number of kretprobe hits at the end of the trace."),               \
  F(ftrace_kprobe_hits_delta,             kSingle,  kInfo,     kTrace,         \
       "The number of kprobe hits encountered during the collection of the"    \
       "trace."),                                                              \
  F(ftrace_kprobe_misses_begin,           kSingle,  kInfo,     kTrace,         \
       "The number of kretprobe missed events at the beginning of the trace."),\
  F(ftrace_kprobe_misses_end,             kSingle,  kInfo,     kTrace,         \
       "The number of kretprobe missed events at the end of the trace."),      \
  F(ftrace_kprobe_misses_delta,           kSingle,  kDataLoss, kTrace,         \
       "The number of kretprobe missed events encountered during the "         \
       "collection of the trace. A value greater than zero is due to the "     \
       "maxactive parameter for the kretprobe being too small"),               \
  F(ftrace_setup_errors,                  kSingle,  kInfo,     kTrace,         \
       "One or more atrace/ftrace categories were not found or failed to "     \
       "enable. See ftrace_setup_errors in the metadata table for details."),  \
  F(ftrace_abi_errors_skipped_zero_data_length,                                \
                                          kSingle,  kInfo,     kAnalysis, ""), \
  F(ftrace_thermal_exynos_acpm_unknown_tz_id,                                  \
                                          kSingle,  kError,    kAnalysis, ""), \
  F(fuchsia_non_numeric_counters,         kSingle,  kError,    kAnalysis, ""), \
  F(fuchsia_timestamp_overflow,           kSingle,  kError,    kAnalysis, ""), \
  F(fuchsia_record_read_error,            kSingle,  kError,    kAnalysis, ""), \
  F(fuchsia_invalid_event,                kSingle,  kError,    kAnalysis, ""), \
  F(fuchsia_invalid_event_arg_type,       kSingle,  kError,    kAnalysis, ""), \
  F(fuchsia_invalid_event_arg_name,       kSingle,  kError,    kAnalysis, ""), \
  F(fuchsia_unknown_event_arg,            kSingle,  kError,    kAnalysis, ""), \
  F(fuchsia_invalid_string_ref,           kSingle,  kError,    kAnalysis, ""), \
  F(gpu_counters_invalid_spec,            kSingle,  kError,    kAnalysis, ""), \
  F(gpu_counters_missing_spec,            kSingle,  kError,    kAnalysis, ""), \
  F(gpu_render_stage_parser_errors,       kSingle,  kError,    kAnalysis, ""), \
  F(graphics_frame_event_parser_errors,   kSingle,  kInfo,     kAnalysis, ""), \
  F(guess_trace_type_duration_ns,         kSingle,  kInfo,     kAnalysis, ""), \
  F(interned_data_tokenizer_errors,       kSingle,  kInfo,     kAnalysis, ""), \
  F(invalid_clock_snapshots,              kSingle,  kError,    kAnalysis, ""), \
  F(invalid_cpu_times,                    kSingle,  kError,    kAnalysis, ""), \
  F(kernel_wakelock_reused_id,            kSingle,  kError,    kAnalysis,      \
       "Duplicated interning ID seen. Should never happen."),                  \
  F(kernel_wakelock_unknown_id,           kSingle,  kError,    kAnalysis,      \
       "Interning ID not found. Should never happen."),                        \
  F(kernel_wakelock_zero_value_reported,  kSingle,  kDataLoss, kTrace,         \
       "Zero value received from SuspendControlService. Indicates a transient "\
       "error in SuspendControlService."),                                     \
  F(kernel_wakelock_non_monotonic_value_reported,                              \
                                          kSingle,  kDataLoss, kTrace,         \
       "Decreased value received from SuspendControlService. Indicates a "     \
       "transient error in SuspendControlService."),                           \
  F(app_wakelock_parse_error,             kSingle,  kError,    kAnalysis,      \
       "Parsing packed repeated field. Should never happen."),                 \
  F(app_wakelock_unknown_id,              kSingle,  kError,    kAnalysis,      \
       "Interning ID not found. Should never happen."),                        \
  F(meminfo_unknown_keys,                 kSingle,  kError,    kAnalysis, ""), \
  F(mismatched_sched_switch_tids,         kSingle,  kError,    kAnalysis, ""), \
  F(mm_unknown_type,                      kSingle,  kError,    kAnalysis, ""), \
  F(parse_trace_duration_ns,              kSingle,  kInfo,     kAnalysis, ""), \
  F(power_rail_unknown_index,             kSingle,  kError,    kTrace,    ""), \
  F(proc_stat_unknown_counters,           kSingle,  kError,    kAnalysis, ""), \
  F(rss_stat_unknown_keys,                kSingle,  kError,    kAnalysis, ""), \
  F(rss_stat_negative_size,               kSingle,  kInfo,     kAnalysis, ""), \
  F(rss_stat_unknown_thread_for_mm_id,    kSingle,  kInfo,     kAnalysis, ""), \
  F(filter_input_bytes,                   kSingle,  kInfo,     kTrace,         \
       "Number of bytes pre-TraceFilter. The trace file would have been this " \
       "many bytes big if the TraceConfig didn't specify any TraceFilter. "    \
       "This affects the actual buffer usage, as filtering happens only "      \
       "when writing into the trace file (or over IPC)."),                     \
  F(filter_input_packets,                 kSingle,  kInfo,     kTrace,         \
       "Number of packets pre-TraceFilter. The trace file would have had so "  \
       "many packets if the TraceConfig didn't specify any TraceFilter."),     \
  F(filter_output_bytes,                  kSingle,  kInfo,     kTrace,         \
       "Number of bytes that made it through the TraceFilter, before the "     \
       "(optional) Zlib compression stage."),                                  \
  F(filter_time_taken_ns,                 kSingle,  kInfo,     kTrace,         \
       "Time cumulatively spent running the TraceFilter throughout the "       \
       "tracing session by MaybeFilterPackets()."),                            \
  F(filter_errors,                        kSingle,  kError,    kTrace,    ""), \
  F(flow_duplicate_id,                    kSingle,  kError,    kTrace,    ""), \
  F(flow_no_enclosing_slice,              kSingle,  kError,    kTrace,    ""), \
  F(flow_step_without_start,              kSingle,  kInfo,     kTrace,    ""), \
  F(flow_end_without_start,               kSingle,  kInfo,     kTrace,    ""), \
  F(flow_invalid_id,                      kSingle,  kError,    kTrace,    ""), \
  F(flow_without_direction,               kSingle,  kError,    kTrace,    ""), \
  F(stackprofile_empty_callstack,         kSingle,  kError,    kTrace,         \
      "Callstack had no frames. Ignored"),                                     \
  F(stackprofile_invalid_string_id,       kSingle,  kError,    kTrace,    ""), \
  F(stackprofile_invalid_mapping_id,      kSingle,  kError,    kTrace,    ""), \
  F(stackprofile_invalid_frame_id,        kSingle,  kError,    kTrace,    ""), \
  F(stackprofile_invalid_callstack_id,    kSingle,  kError,    kTrace,    ""), \
  F(stackprofile_parser_error,            kSingle,  kError,    kTrace,    ""), \
  F(systrace_parse_failure,               kSingle,  kError,    kAnalysis, ""), \
  F(task_state_invalid,                   kSingle,  kError,    kAnalysis, ""), \
  F(traced_buf_abi_violations,            kIndexed, kDataLoss, kTrace,    ""), \
  F(traced_buf_buffer_size,               kIndexed, kInfo,     kTrace,    ""), \
  F(traced_buf_bytes_overwritten,         kIndexed, kInfo,     kTrace,    ""), \
  F(traced_buf_bytes_read,                kIndexed, kInfo,     kTrace,    ""), \
  F(traced_buf_bytes_filtered_out,        kIndexed, kInfo,     kTrace,         \
       "Number of bytes discarded (input - output) by the TraceFilter for "    \
       "each buffer. It is a subset of, but does not add up perfectly to, "    \
       "(filter_input_bytes - filter_output_bytes) because of the synthetic "  \
       "metadata and stats packets generated by the tracing service itself."), \
  F(traced_buf_bytes_written,             kIndexed, kInfo,     kTrace,    ""), \
  F(traced_buf_clone_done_timestamp_ns,   kIndexed, kInfo,     kTrace,         \
    "The timestamp when the clone snapshot operation for this buffer "         \
    "finished"),                                                               \
  F(traced_buf_chunks_discarded,          kIndexed, kInfo,     kTrace,    ""), \
  F(traced_buf_chunks_overwritten,        kIndexed, kInfo,     kTrace,    ""), \
  F(traced_buf_chunks_read,               kIndexed, kInfo,     kTrace,    ""), \
  F(traced_buf_chunks_rewritten,          kIndexed, kInfo,     kTrace,    ""), \
  F(traced_buf_chunks_written,            kIndexed, kInfo,     kTrace,    ""), \
  F(traced_buf_chunks_committed_out_of_order,                                  \
                                          kIndexed, kInfo,     kTrace,    ""), \
  F(traced_buf_padding_bytes_cleared,     kIndexed, kInfo,     kTrace,    ""), \
  F(traced_buf_padding_bytes_written,     kIndexed, kInfo,     kTrace,    ""), \
  F(traced_buf_patches_failed,            kIndexed, kDataLoss, kTrace,         \
      "The tracing service potentially lost data from one of the data sources "\
      "writing into the given target_buffer. This entry can be ignored "       \
      "if you're using DISCARD buffers and traced_buf_chunks_discarded is "    \
      "nonzero, meaning that the buffer was filled."),                         \
  F(traced_buf_patches_succeeded,         kIndexed, kInfo,     kTrace,    ""), \
  F(traced_buf_readaheads_failed,         kIndexed, kInfo,     kTrace,    ""), \
  F(traced_buf_readaheads_succeeded,      kIndexed, kInfo,     kTrace,    ""), \
  F(traced_buf_trace_writer_packet_loss,  kIndexed, kDataLoss, kTrace,         \
      "The tracing service observed packet loss for this buffer during this "  \
      "tracing session. This also counts packet loss that happened before "    \
      "the RING_BUFFER start or after the DISCARD buffer end."),               \
  F(traced_buf_sequence_packet_loss,      kIndexed, kDataLoss, kAnalysis,      \
      "The number of groups of consecutive packets lost in each sequence for " \
      "this buffer"),                                                          \
  F(traced_buf_incremental_sequences_dropped, kIndexed, kDataLoss, kAnalysis,  \
      "For a given buffer, indicates the number of sequences where all the "   \
      "packets on that sequence were dropped due to lack of a valid "          \
      "incremental state (i.e. interned data). This is usually a strong sign " \
      "that either: "                                                          \
      "1) incremental state invalidation is disabled. "                        \
      "2) the incremental state invalidation interval is too low. "            \
      "In either case, see "                                                   \
      "https://perfetto.dev/docs/concepts/buffers"                             \
      "#incremental-state-in-trace-packets"),                                  \
  F(traced_buf_write_wrap_count,          kIndexed, kInfo,     kTrace,    ""), \
  F(traced_clone_started_timestamp_ns,    kSingle,  kInfo,     kTrace,         \
    "The timestamp when the clone snapshot operation for this trace started"), \
  F(traced_clone_trigger_timestamp_ns,    kSingle,  kInfo,     kTrace,         \
    "The timestamp when trigger for the clone snapshot operation for this "    \
    "trace was received"), \
  F(traced_chunks_discarded,              kSingle,  kInfo,     kTrace,    ""), \
  F(traced_data_sources_registered,       kSingle,  kInfo,     kTrace,    ""), \
  F(traced_data_sources_seen,             kSingle,  kInfo,     kTrace,    ""), \
  F(traced_final_flush_failed,            kSingle,  kDataLoss, kTrace,    ""), \
  F(traced_final_flush_succeeded,         kSingle,  kInfo,     kTrace,    ""), \
  F(traced_flushes_failed,                kSingle,  kDataLoss, kTrace,    ""), \
  F(traced_flushes_requested,             kSingle,  kInfo,     kTrace,    ""), \
  F(traced_flushes_succeeded,             kSingle,  kInfo,     kTrace,    ""), \
  F(traced_patches_discarded,             kSingle,  kInfo,     kTrace,    ""), \
  F(traced_producers_connected,           kSingle,  kInfo,     kTrace,    ""), \
  F(traced_producers_seen,                kSingle,  kInfo,     kTrace,    ""), \
  F(traced_total_buffers,                 kSingle,  kInfo,     kTrace,    ""), \
  F(traced_tracing_sessions,              kSingle,  kInfo,     kTrace,    ""), \
  F(track_event_parser_errors,            kSingle,  kInfo,     kAnalysis, ""), \
  F(track_event_dropped_packets_outside_of_range_of_interest,                  \
                                          kSingle,  kInfo,     kAnalysis,      \
      "The number of TrackEvent packets dropped by trace processor due to "    \
      "being outside of the range of interest. This happens if a trace has a " \
      "TrackEventRangeOfInterest packet, and track event dropping is "         \
      "enabled."),                                                             \
  F(track_event_tokenizer_errors,         kSingle,  kInfo,     kAnalysis, ""), \
  F(track_event_thread_invalid_end,       kSingle,  kError,    kTrace,         \
      "The end event for a thread track does not match a track event "         \
      "begin event. This can happen on mixed atrace/track_event traces "       \
      "and is usually caused by data loss or bugs when the events are "        \
      "emitted. The outcome of this is that slices can appear to be closed "   \
      "before they were closed in reality"),                                   \
  F(tokenizer_skipped_packets,            kSingle,  kInfo,     kAnalysis, ""), \
  F(vmstat_unknown_keys,                  kSingle,  kError,    kAnalysis, ""), \
  F(psi_unknown_resource,                 kSingle,  kError,    kAnalysis, ""), \
  F(vulkan_allocations_invalid_string_id,                                      \
                                          kSingle,  kError,    kTrace,    ""), \
  F(clock_sync_failure,                   kSingle,  kError,    kAnalysis, ""), \
  F(clock_sync_cache_miss,                kSingle,  kInfo,     kAnalysis, ""), \
  F(process_tracker_errors,               kSingle,  kError,    kAnalysis, ""), \
  F(json_tokenizer_failure,               kSingle,  kError,    kTrace,    ""), \
  F(json_parser_failure,                  kSingle,  kError,    kTrace,    ""), \
  F(json_display_time_unit,               kSingle,  kInfo,     kTrace,         \
      "The displayTimeUnit key was set in the JSON trace. In some prior "      \
      "versions of trace processor this key could effect how the trace "       \
      "processor parsed timestamps and durations. In this version the key is " \
      "ignored which more closely matches the bavahiour of catapult."),        \
  F(heap_graph_invalid_string_id,         kIndexed, kError,    kTrace,    ""), \
  F(heap_graph_non_finalized_graph,       kSingle,  kError,    kTrace,    ""), \
  F(heap_graph_malformed_packet,          kIndexed, kError,    kTrace,    ""), \
  F(heap_graph_missing_packet,            kIndexed, kError,    kTrace,    ""), \
  F(heapprofd_buffer_corrupted,           kIndexed, kError,    kTrace,         \
      "Shared memory buffer corrupted. This is a bug or memory corruption "    \
      "in the target. Indexed by target upid."),                               \
  F(heapprofd_hit_guardrail,              kIndexed, kError,    kTrace,         \
      "HeapprofdConfig specified a CPU or Memory Guardrail that was hit. "     \
      "Indexed by target upid."),                                              \
  F(heapprofd_buffer_overran,             kIndexed, kDataLoss, kTrace,         \
      "The shared memory buffer between the target and heapprofd overran. "    \
      "The profile was truncated early. Indexed by target upid."),             \
  F(heapprofd_client_error,               kIndexed, kError,    kTrace,         \
      "The heapprofd client ran into a problem and disconnected. "             \
      "See profile_packet.proto  for error codes."),                           \
  F(heapprofd_client_disconnected,        kIndexed, kInfo,     kTrace,    ""), \
  F(heapprofd_malformed_packet,           kIndexed, kError,    kTrace,    ""), \
  F(heapprofd_missing_packet,             kSingle,  kError,    kTrace,    ""), \
  F(heapprofd_rejected_concurrent,        kIndexed, kError,    kTrace,         \
      "The target was already profiled by another tracing session, so the "    \
      "profile was not taken. Indexed by target upid."),                       \
  F(heapprofd_non_finalized_profile,      kSingle,  kError,    kTrace,    ""), \
  F(heapprofd_sampling_interval_adjusted,                                      \
    kIndexed, kInfo,    kTrace,                                                \
      "By how many byes the interval for PID was increased "                   \
      "by adaptive sampling."),                                                \
  F(heapprofd_unwind_time_us,             kIndexed, kInfo,     kTrace,         \
      "Time spent unwinding callstacks."),                                     \
  F(heapprofd_unwind_samples,             kIndexed, kInfo,     kTrace,         \
      "Number of samples unwound."),                                           \
  F(heapprofd_client_spinlock_blocked,    kIndexed, kInfo,     kTrace,         \
       "Time (us) the heapprofd client was blocked on the spinlock."),         \
  F(heapprofd_last_profile_timestamp,     kIndexed, kInfo,     kTrace,         \
       "The timestamp (in trace time) for the last dump for a process"),       \
  F(symbolization_tmp_build_id_not_found,     kSingle,  kError,    kAnalysis,  \
       "Number of file mappings in /data/local/tmp without a build id. "       \
       "Symbolization doesn't work for executables in /data/local/tmp "        \
       "because of SELinux. Please use /data/local/tests"),                    \
  F(metatrace_overruns,                   kSingle,  kError,    kTrace,    ""), \
  F(packages_list_has_parse_errors,       kSingle,  kError,    kTrace,    ""), \
  F(packages_list_has_read_errors,        kSingle,  kError,    kTrace,    ""), \
  F(game_intervention_has_parse_errors,   kSingle,  kError,    kTrace,         \
       "One or more parsing errors occurred. This could result from "          \
       "unknown game more or intervention added to the file to be parsed."),   \
  F(game_intervention_has_read_errors,    kSingle,  kError,    kTrace,         \
       "The file to be parsed can't be opened. This can happend when "         \
       "the file name is not found or no permission to access the file"),      \
  F(compact_sched_has_parse_errors,       kSingle,  kError,    kTrace,    ""), \
  F(misplaced_end_event,                  kSingle,  kDataLoss, kAnalysis, ""), \
  F(truncated_sys_write_duration,         kSingle,  kInfo,     kAnalysis,      \
      "Count of sys_write slices that have a truncated duration to resolve "   \
      "nesting incompatibilities with atrace slices. Real durations "          \
      "can be recovered via the |raw| table."),                                \
  F(compact_sched_switch_skipped,         kSingle,  kInfo,     kAnalysis, ""), \
  F(compact_sched_waking_skipped,         kSingle,  kInfo,     kAnalysis, ""), \
  F(empty_chrome_metadata,                kSingle,  kError,    kTrace,    ""), \
  F(ninja_parse_errors,                   kSingle,  kError,    kTrace,    ""), \
  F(perf_cpu_lost_records,                kIndexed, kDataLoss, kTrace,         \
      "Count of perf samples lost due to kernel buffer overruns. The trace "   \
      "is missing information, but it's not known which processes are "        \
      "affected. Consider lowering the sampling frequency or raising "         \
      "the ring_buffer_pages config option."),                                 \
  F(perf_process_shard_count,             kIndexed, kInfo,     kTrace,    ""), \
  F(perf_chosen_process_shard,            kIndexed, kInfo,     kTrace,    ""), \
  F(perf_guardrail_stop_ts,               kIndexed, kDataLoss, kTrace,    ""), \
  F(perf_unknown_record_type,             kIndexed, kInfo,     kAnalysis, ""), \
  F(perf_record_skipped,                  kIndexed, kError,    kAnalysis, ""), \
  F(perf_samples_skipped,                 kSingle,  kError,    kAnalysis,      \
      "Count of skipped perf samples that otherwise matched the tracing "      \
      "config. This will cause a process to be completely absent from the "    \
      "trace, but does *not* imply data loss for processes that do have "      \
      "samples in this trace."),                                               \
  F(perf_counter_skipped_because_no_cpu,  kSingle,  kError,    kAnalysis, ""), \
  F(perf_features_skipped,                kIndexed, kInfo,     kAnalysis, ""), \
  F(perf_samples_cpu_mode_unknown,        kSingle,  kError,    kAnalysis, ""), \
  F(perf_samples_skipped_dataloss,        kSingle,  kDataLoss, kTrace,         \
      "Count of perf samples lost within the profiler (traced_perf), likely "  \
      "due to load shedding. This may impact any traced processes. The trace " \
      "protobuf needs to be inspected manually to confirm which processes "    \
      "are affected."),                                                        \
  F(perf_dummy_mapping_used,              kSingle,  kInfo,     kAnalysis, ""), \
  F(perf_aux_missing,                     kSingle,  kDataLoss, kTrace,         \
      "Number of bytes missing in AUX data streams due to missing "            \
      "PREF_RECORD_AUX messages."),                                            \
  F(perf_aux_ignored,                     kSingle,  kInfo,     kTrace,         \
       "AUX data was ignored because the proper parser is not implemented."), \
  F(perf_aux_lost,                        kSingle,  kDataLoss, kTrace,         \
      "Gaps in the AUX data stream pased to the tokenizer."), \
  F(perf_aux_truncated,                   kSingle,  kDataLoss, kTrace,         \
      "Data was truncated when being written to the AUX stream at the "        \
      "source."),\
  F(perf_aux_partial,                     kSingle,  kDataLoss, kTrace,         \
      "The PERF_RECORD_AUX contained partial data."), \
  F(perf_aux_collision,                   kSingle,  kDataLoss, kTrace,         \
      "The collection of a sample colliden with another. You should reduce "   \
      "the rate at which samples are collected."),                             \
  F(perf_auxtrace_missing,                kSingle,  kDataLoss, kTrace,         \
      "Number of bytes missing in AUX data streams due to missing "            \
      "PREF_RECORD_AUXTRACE messages."),                                       \
  F(perf_unknown_aux_data,                kIndexed, kDataLoss, kTrace,         \
      "AUX data type encountered for which there is no known parser."),        \
  F(perf_no_tsc_data,                     kSingle,  kInfo,     kTrace,         \
      "TSC data unavailable. Will be unable to translate HW clocks."),         \
  F(spe_no_timestamp,                     kSingle,  kInfo,     kTrace,         \
      "SPE record with no timestamp. Will try our best to assign a "           \
      "timestamp."),                                                           \
  F(spe_record_dropped,                   kSingle,  kDataLoss, kTrace,         \
      "SPE record dropped. E.g. Unable to assign it a timestamp."),            \
  F(etm_no_importer,                      kSingle,  kError,    kAnalysis,      \
      "Unable to parse ETM data because TraceProcessor was not compiled to  "  \
      " support it. Make sure you enable the `enable_perfetto_etm_importer` "  \
      " GN flag."),                                                            \
  F(memory_snapshot_parser_failure,       kSingle,  kError,    kAnalysis, ""), \
  F(thread_time_in_state_unknown_cpu_freq,                                     \
                                          kSingle,  kError,    kAnalysis, ""), \
  F(ftrace_packet_before_tracing_start,   kSingle,  kInfo,     kAnalysis,      \
      "An ftrace packet was seen before the tracing start timestamp from "     \
      "the tracing service. This happens if the ftrace buffers were not "      \
      "cleared properly. These packets are silently dropped by trace "         \
      "processor."),                                                           \
  F(sorter_push_event_out_of_order,       kSingle, kError,     kTrace,         \
      "Trace events are out of order event after sorting. This can happen "    \
      "due to many factors including clock sync drift, producers emitting "    \
      "events out of order or a bug in trace processor's logic of sorting."),  \
  F(unknown_extension_fields,             kSingle,  kError,    kTrace,         \
      "TraceEvent had unknown extension fields, which might result in "        \
      "missing some arguments. You may need a newer version of trace "         \
      "processor to parse them."),                                             \
  F(network_trace_intern_errors,          kSingle,  kInfo,     kAnalysis, ""), \
  F(network_trace_parse_errors,           kSingle,  kInfo,     kAnalysis, ""), \
  F(atom_timestamp_missing,               kSingle,  kError,    kTrace,         \
      "The corresponding timestamp_nanos entry for a StatsdAtom was "          \
      "missing. Defaulted to inaccurate packet timestamp."),                   \
  F(atom_unknown,                         kSingle,  kInfo,     kAnalysis,      \
      "Unknown statsd atom. Atom descriptor may need to be updated"),          \
  F(v8_intern_errors,                                                          \
                                          kSingle,  kDataLoss, kAnalysis,      \
      "Failed to resolve V8 interned data."),                                  \
  F(v8_isolate_has_no_code_range,                                              \
                                          kSingle,  kError,    kAnalysis,      \
      "V8 isolate had no code range. THis is currently no supported and means" \
      "we will be unable to parse JS code events for this isolate."),          \
  F(v8_no_defaults,                                                            \
                                          kSingle,  kDataLoss, kAnalysis,      \
      "Failed to resolve V8 default data."),                                   \
  F(v8_no_code_range,                                                          \
                                          kSingle,  kError,    kAnalysis,      \
      "V8 isolate had no code range."),                                        \
  F(v8_unknown_code_type,                 kSingle,  kError,    kAnalysis, ""), \
  F(v8_code_load_missing_code_range,      kSingle,  kError,    kAnalysis,      \
      "V8 load had no code range or an empty one. Event ignored."),            \
  F(winscope_inputmethod_clients_parse_errors,                                 \
                                          kSingle,  kInfo,     kAnalysis,      \
      "InputMethod clients packet has unknown fields, which results in "       \
      "some arguments missing. You may need a newer version of trace "         \
      "processor to parse them."),                                             \
  F(winscope_inputmethod_manager_service_parse_errors,                         \
                                          kSingle,  kInfo,     kAnalysis,      \
      "InputMethod manager service packet has unknown fields, which results "  \
      "in some arguments missing. You may need a newer version of trace "      \
      "processor to parse them."),                                             \
  F(winscope_inputmethod_service_parse_errors,                                 \
                                          kSingle,  kInfo,     kAnalysis,      \
      "InputMethod service packet has unknown fields, which results in "       \
      "some arguments missing. You may need a newer version of trace "         \
      "processor to parse them."),                                             \
  F(winscope_sf_layers_parse_errors,      kSingle,  kInfo,     kAnalysis,      \
      "SurfaceFlinger layers snapshot has unknown fields, which results in "   \
      "some arguments missing. You may need a newer version of trace "         \
      "processor to parse them."),                                             \
  F(winscope_sf_transactions_parse_errors,                                     \
                                          kSingle,  kInfo,     kAnalysis,      \
      "SurfaceFlinger transactions packet has unknown fields, which results "  \
      "in some arguments missing. You may need a newer version of trace "      \
      "processor to parse them."),                                             \
  F(winscope_shell_transitions_parse_errors,                                   \
                                          kSingle,  kInfo,     kAnalysis,      \
      "Shell transition packet has unknown fields, which results "             \
      "in some arguments missing. You may need a newer version of trace "      \
      "processor to parse them."),                                             \
  F(winscope_protolog_invalid_interpolation_parse_errors,                      \
                                          kSingle,  kInfo,     kAnalysis,      \
      "ProtoLog message string has invalid interplation parameter."),          \
  F(winscope_protolog_missing_interned_arg_parse_errors,                       \
                                          kSingle,  kInfo,     kAnalysis,      \
      "Failed to find interned ProtoLog argument."),                           \
  F(winscope_protolog_missing_interned_stacktrace_parse_errors,                \
                                          kSingle,  kInfo,     kAnalysis,      \
      "Failed to find interned ProtoLog stacktrace."),                         \
  F(winscope_protolog_message_decoding_failed,                                 \
                                          kSingle,  kInfo,     kAnalysis,      \
      "Failed to decode ProtoLog message."),                                   \
  F(winscope_protolog_view_config_collision,                                   \
                                          kSingle,  kInfo,     kAnalysis,      \
      "Got a viewer config collision!"),                                       \
  F(winscope_viewcapture_parse_errors,                                         \
                                          kSingle,  kInfo,     kAnalysis,      \
      "ViewCapture packet has unknown fields, which results in some "          \
      "arguments missing. You may need a newer version of trace processor "    \
      "to parse them."),                                                       \
  F(winscope_viewcapture_missing_interned_string_parse_errors,                 \
                                          kSingle,  kInfo,     kAnalysis,      \
      "Failed to find interned ViewCapture string."),                          \
  F(winscope_windowmanager_parse_errors, kSingle,  kInfo,     kAnalysis,       \
      "WindowManager state packet has unknown fields, which results "          \
      "in some arguments missing. You may need a newer version of trace "      \
      "processor to parse them."),                                             \
  F(jit_unknown_frame,                    kSingle,  kDataLoss, kTrace,         \
      "Indicates that we were unable to determine the function for a frame in "\
      "a jitted memory region"),                                               \
  F(ftrace_missing_event_id,              kSingle,  kInfo,    kAnalysis,       \
      "Indicates that the ftrace event was dropped because the event id was "  \
      "missing. This is an 'info' stat rather than an error stat because "     \
      "this can be legitimately missing due to proto filtering."),             \
  F(android_input_event_parse_errors,     kSingle,  kInfo,     kAnalysis,      \
      "Android input event packet has unknown fields, which results "          \
      "in some arguments missing. You may need a newer version of trace "      \
      "processor to parse them."),                                             \
  F(mali_unknown_mcu_state_id,            kSingle,  kError,   kAnalysis,       \
      "An invalid Mali GPU MCU state ID was detected."),                       \
  F(pixel_modem_negative_timestamp,       kSingle,  kError,   kAnalysis,       \
      "A negative timestamp was received from a Pixel modem event."),          \
  F(legacy_v8_cpu_profile_invalid_callsite, kSingle, kError,  kTrace,          \
      "Indicates a callsite in legacy v8 CPU profiling is invalid. This is "   \
      "a sign that the trace is malformed."),                                  \
  F(legacy_v8_cpu_profile_invalid_sample, kSingle,  kError,  kTrace,           \
      "Indicates a sample in legacy v8 CPU profile is invalid. This will "     \
      "cause CPU samples to be missing in the UI. This is a sign that the "    \
      "trace is malformed."),                                                  \
  F(config_write_into_file_no_flush,      kSingle,  kError,  kTrace,           \
      "The trace was collected with the `write_into_file` option set but "     \
      "*without* `flush_period_ms` being set. This will cause the trace to "   \
      "be fully loaded into memory and use significantly more memory than "    \
      "necessary."),                                                           \
  F(config_write_into_file_discard,        kIndexed,  kDataLoss,  kTrace,      \
      "The trace was collected with the `write_into_file` option set but "     \
      "uses a `DISCARD` buffer. This configuration is strongly discouraged "   \
      "and can cause mysterious data loss in the trace. Please use "           \
      "`RING_BUFFER` buffers instead.")
// clang-format on

enum Type {
  kSingle,  // Single-value property, one value per key.
  kIndexed  // Indexed property, multiple value per key (e.g. cpu_stats[1]).
};

enum Severity {
  kInfo,      // Diagnostic counters
  kDataLoss,  // Correct operation that still resulted in data loss
  kError      // If any kError counter is > 0 trace_processor_shell will
              // raise an error. This is also surfaced in the web UI.
};

enum Source {
  // The counter is collected when recording the trace on-device and is just
  // being reflected in the stats table.
  kTrace,

  // The counter is generated when importing / processing the trace in the trace
  // processor.
  kAnalysis
};

#if defined(__GNUC__) || defined(__clang__)
#if defined(__clang__)
#pragma clang diagnostic push
// Fix 'error: #pragma system_header ignored in main file' for clang in Google3.
#pragma clang diagnostic ignored "-Wpragma-system-header-outside-header"
#endif

// Ignore GCC warning about a missing argument for a variadic macro parameter.
#pragma GCC system_header

#if defined(__clang__)
#pragma clang diagnostic pop
#endif
#endif

// Declares an enum of literals (one for each stat). The enum values of each
// literal corresponds to the string index in the arrays below.
#define PERFETTO_TP_STATS_ENUM(name, ...) name
enum KeyIDs : size_t { PERFETTO_TP_STATS(PERFETTO_TP_STATS_ENUM), kNumKeys };

// The code below declares an array for each property (name, type, ...).

#define PERFETTO_TP_STATS_NAME(name, ...) #name
constexpr char const* kNames[] = {PERFETTO_TP_STATS(PERFETTO_TP_STATS_NAME)};

#define PERFETTO_TP_STATS_TYPE(_, type, ...) type
constexpr Type kTypes[] = {PERFETTO_TP_STATS(PERFETTO_TP_STATS_TYPE)};

#define PERFETTO_TP_STATS_SEVERITY(_, __, severity, ...) severity
constexpr Severity kSeverities[] = {
    PERFETTO_TP_STATS(PERFETTO_TP_STATS_SEVERITY)};

#define PERFETTO_TP_STATS_SOURCE(_, __, ___, source, ...) source
constexpr Source kSources[] = {PERFETTO_TP_STATS(PERFETTO_TP_STATS_SOURCE)};

#define PERFETTO_TP_STATS_DESCRIPTION(_, __, ___, ____, descr, ...) descr
constexpr char const* kDescriptions[] = {
    PERFETTO_TP_STATS(PERFETTO_TP_STATS_DESCRIPTION)};

}  // namespace perfetto::trace_processor::stats

#endif  // SRC_TRACE_PROCESSOR_STORAGE_STATS_H_
