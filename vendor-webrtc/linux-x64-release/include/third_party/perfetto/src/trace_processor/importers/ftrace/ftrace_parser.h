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

#ifndef SRC_TRACE_PROCESSOR_IMPORTERS_FTRACE_FTRACE_PARSER_H_
#define SRC_TRACE_PROCESSOR_IMPORTERS_FTRACE_FTRACE_PARSER_H_

#include <array>
#include <cstddef>
#include <cstdint>
#include <string>
#include <unordered_map>
#include <unordered_set>
#include <utility>
#include <vector>

#include "perfetto/base/status.h"
#include "perfetto/ext/base/flat_hash_map.h"
#include "perfetto/ext/base/hash.h"
#include "perfetto/ext/base/string_view.h"
#include "perfetto/protozero/field.h"
#include "src/trace_processor/importers/common/event_tracker.h"
#include "src/trace_processor/importers/common/parser_types.h"
#include "src/trace_processor/importers/common/trace_parser.h"
#include "src/trace_processor/importers/ftrace/drm_tracker.h"
#include "src/trace_processor/importers/ftrace/ftrace_descriptors.h"
#include "src/trace_processor/importers/ftrace/gpu_work_period_tracker.h"
#include "src/trace_processor/importers/ftrace/iostat_tracker.h"
#include "src/trace_processor/importers/ftrace/mali_gpu_event_tracker.h"
#include "src/trace_processor/importers/ftrace/pixel_mm_kswapd_event_tracker.h"
#include "src/trace_processor/importers/ftrace/pkvm_hyp_cpu_tracker.h"
#include "src/trace_processor/importers/ftrace/rss_stat_tracker.h"
#include "src/trace_processor/importers/ftrace/thermal_tracker.h"
#include "src/trace_processor/importers/ftrace/virtio_gpu_tracker.h"
#include "src/trace_processor/storage/trace_storage.h"
#include "src/trace_processor/types/trace_processor_context.h"

namespace perfetto::trace_processor {

class FtraceParser {
 public:
  explicit FtraceParser(TraceProcessorContext* context);

  base::Status ParseFtraceStats(protozero::ConstBytes,
                                uint32_t packet_sequence_id);

  base::Status ParseFtraceEvent(uint32_t cpu,
                                int64_t ts,
                                const TracePacketData& data);
  base::Status ParseInlineSchedSwitch(uint32_t cpu,
                                      int64_t ts,
                                      const InlineSchedSwitch& data);
  base::Status ParseInlineSchedWaking(uint32_t cpu,
                                      int64_t ts,
                                      const InlineSchedWaking& data);

 private:
  void ParseGenericFtrace(int64_t timestamp,
                          uint32_t cpu,
                          uint32_t pid,
                          protozero::ConstBytes);
  void ParseTypedFtraceToRaw(uint32_t ftrace_id,
                             int64_t timestamp,
                             uint32_t cpu,
                             uint32_t pid,
                             protozero::ConstBytes,
                             PacketSequenceStateGeneration*);
  void ParseSchedSwitch(uint32_t cpu, int64_t timestamp, protozero::ConstBytes);
  void ParseKprobe(int64_t timestamp, uint32_t pid, protozero::ConstBytes);
  void ParseSchedWaking(int64_t timestamp, uint32_t pid, protozero::ConstBytes);
  void ParseSchedProcessFree(int64_t timestamp, protozero::ConstBytes);
  void ParseCpuFreq(int64_t timestamp, protozero::ConstBytes);
  void ParseCpuFreqThrottle(int64_t timestamp, protozero::ConstBytes);
  void ParseGpuFreq(int64_t timestamp, protozero::ConstBytes);
  void ParseKgslGpuFreq(int64_t timestamp, protozero::ConstBytes);
  void ParseCpuIdle(int64_t timestamp, protozero::ConstBytes);
  void ParsePrint(int64_t timestamp, uint32_t pid, protozero::ConstBytes);
  void ParseZero(int64_t timestamp, uint32_t pid, protozero::ConstBytes);
  void ParseMdssTracingMarkWrite(int64_t timestamp,
                                 uint32_t pid,
                                 protozero::ConstBytes);
  void ParseSdeTracingMarkWrite(int64_t timestamp,
                                uint32_t pid,
                                protozero::ConstBytes);
  void ParseDpuTracingMarkWrite(int64_t timestamp,
                                uint32_t pid,
                                protozero::ConstBytes);
  void ParseDpuDispDpuUnderrun(int64_t timestamp, protozero::ConstBytes);
  void ParseDpuDispVblankIrqEnable(int64_t timestamp, protozero::ConstBytes);
  void ParseG2dTracingMarkWrite(int64_t timestamp,
                                uint32_t pid,
                                protozero::ConstBytes);
  void ParseSamsungTracingMarkWrite(int64_t timestamp,
                                    uint32_t pid,
                                    protozero::ConstBytes);
  void ParseMaliTracingMarkWrite(int64_t timestamp,
                                 uint32_t pid,
                                 protozero::ConstBytes);
  void ParseLwisTracingMarkWrite(int64_t timestamp,
                                 uint32_t pid,
                                 protozero::ConstBytes);
  void ParseIonHeapGrowOrShrink(int64_t timestamp,
                                uint32_t pid,
                                protozero::ConstBytes,
                                bool grow);
  void ParseIonStat(int64_t timestamp, uint32_t pid, protozero::ConstBytes);
  void ParseBclIrq(int64_t timestamp, protozero::ConstBytes);
  void ParseDmaHeapStat(int64_t timestamp, uint32_t pid, protozero::ConstBytes);
  void ParseSignalGenerate(int64_t timestamp, protozero::ConstBytes);
  void ParseSignalDeliver(int64_t timestamp,
                          uint32_t pid,
                          protozero::ConstBytes);
  void ParseOOMScoreAdjUpdate(int64_t timestamp, protozero::ConstBytes);
  void ParseOOMKill(int64_t timestamp, protozero::ConstBytes);
  void ParseMmEventRecord(int64_t timestamp,
                          uint32_t pid,
                          protozero::ConstBytes);
  void ParseSysEnterEvent(int64_t timestamp,
                          uint32_t pid,
                          protozero::ConstBytes);
  void ParseSysExitEvent(int64_t timestamp,
                         uint32_t pid,
                         protozero::ConstBytes);
  void ParseI2cReadEvent(int64_t timestamp,
                         uint32_t pid,
                         protozero::ConstBytes);
  void ParseI2cWriteEvent(int64_t timestamp,
                          uint32_t pid,
                          protozero::ConstBytes);
  void ParseI2cResultEvent(int64_t timestamp,
                           uint32_t pid,
                           protozero::ConstBytes);
  void ParseTaskNewTask(int64_t timestamp,
                        uint32_t source_tid,
                        protozero::ConstBytes);
  void ParseTaskRename(protozero::ConstBytes);
  void ParseBinderTransaction(int64_t timestamp,
                              uint32_t pid,
                              protozero::ConstBytes);
  void ParseBinderTransactionReceived(int64_t timestamp,
                                      uint32_t pid,
                                      protozero::ConstBytes);
  void ParseBinderCommand(int64_t timestamp,
                          uint32_t pid,
                          protozero::ConstBytes);
  void ParseBinderReturn(int64_t timestamp,
                         uint32_t pid,
                         protozero::ConstBytes);
  void ParseBinderTransactionAllocBuf(int64_t timestamp,
                                      uint32_t pid,
                                      protozero::ConstBytes);
  void ParseBinderLocked(int64_t timestamp,
                         uint32_t pid,
                         protozero::ConstBytes);
  void ParseBinderLock(int64_t timestamp, uint32_t pid, protozero::ConstBytes);
  void ParseBinderUnlock(int64_t timestamp,
                         uint32_t pid,
                         protozero::ConstBytes);
  void ParseClockSetRate(int64_t timestamp, protozero::ConstBytes);
  void ParseClockEnable(int64_t timestamp, protozero::ConstBytes);
  void ParseClockDisable(int64_t timestamp, protozero::ConstBytes);
  void ParseScmCallStart(int64_t timestamp,
                         uint32_t pid,
                         protozero::ConstBytes);
  void ParseScmCallEnd(int64_t timestamp, uint32_t pid, protozero::ConstBytes);
  void ParseCmaAllocStart(int64_t timestamp, uint32_t pid);
  void ParseCmaAllocInfo(int64_t timestamp,
                         uint32_t pid,
                         protozero::ConstBytes);
  void ParseDirectReclaimBegin(int64_t timestamp,
                               uint32_t pid,
                               protozero::ConstBytes);
  void ParseDirectReclaimEnd(int64_t timestamp,
                             uint32_t pid,
                             protozero::ConstBytes);
  void ParseShrinkSlabStart(int64_t timestamp,
                            uint32_t pid,
                            protozero::ConstBytes,
                            PacketSequenceStateGeneration* seq_state);
  void ParseShrinkSlabEnd(int64_t timestamp,
                          uint32_t pid,
                          protozero::ConstBytes);
  void ParseWorkqueueExecuteStart(uint32_t cpu,
                                  int64_t timestamp,
                                  uint32_t pid,
                                  protozero::ConstBytes,
                                  PacketSequenceStateGeneration* seq_state);
  void ParseWorkqueueExecuteEnd(int64_t timestamp,
                                uint32_t pid,
                                protozero::ConstBytes);
  void ParseIrqHandlerEntry(uint32_t cpu,
                            int64_t timestamp,
                            protozero::ConstBytes);
  void ParseIrqHandlerExit(uint32_t cpu,
                           int64_t timestamp,
                           protozero::ConstBytes);
  void ParseSoftIrqEntry(uint32_t cpu,
                         int64_t timestamp,
                         protozero::ConstBytes);
  void ParseSoftIrqExit(uint32_t cpu, int64_t timestamp, protozero::ConstBytes);
  void ParseGpuMemTotal(int64_t timestamp, protozero::ConstBytes);
  void ParseSchedBlockedReason(protozero::ConstBytes,
                               PacketSequenceStateGeneration*);
  void ParseFastRpcDmaStat(int64_t timestamp,
                           uint32_t pid,
                           protozero::ConstBytes);
  void ParseCpuhpPause(int64_t, uint32_t, protozero::ConstBytes);
  void ParseNetifReceiveSkb(uint32_t cpu,
                            int64_t timestamp,
                            protozero::ConstBytes);
  void ParseNetDevXmit(uint32_t cpu, int64_t timestamp, protozero::ConstBytes);
  void ParseInetSockSetState(int64_t timestamp,
                             uint32_t pid,
                             protozero::ConstBytes);
  void ParseTcpRetransmitSkb(int64_t timestamp, protozero::ConstBytes);
  void ParseNapiGroReceiveEntry(uint32_t cpu,
                                int64_t timestamp,
                                protozero::ConstBytes);
  void ParseNapiGroReceiveExit(uint32_t cpu,
                               int64_t timestamp,
                               protozero::ConstBytes);
  void ParseCpuFrequencyLimits(int64_t timestamp, protozero::ConstBytes);
  void ParseKfreeSkb(int64_t timestamp, protozero::ConstBytes);
  void ParseUfshcdCommand(int64_t timestamp, protozero::ConstBytes);
  void ParseUfshcdClkGating(int64_t timestamp, protozero::ConstBytes);

  void ParseCrosEcSensorhubData(int64_t timestamp, protozero::ConstBytes);

  void ParseWakeSourceActivate(int64_t timestamp, protozero::ConstBytes);
  void ParseWakeSourceDeactivate(int64_t timestamp, protozero::ConstBytes);
  void ParseSuspendResume(int64_t timestamp,
                          uint32_t cpu,
                          uint32_t pid,
                          protozero::ConstBytes);
  void ParseSuspendResumeMinimal(int64_t timestamp, protozero::ConstBytes);
  void ParseSchedCpuUtilCfs(int64_t timestamp, protozero::ConstBytes);

  void ParseFuncgraphEntry(int64_t timestamp,
                           uint32_t cpu,
                           uint32_t pid,
                           protozero::ConstBytes blob,
                           PacketSequenceStateGeneration* seq_state);
  void ParseFuncgraphExit(int64_t timestamp,
                          uint32_t cpu,
                          uint32_t pid,
                          protozero::ConstBytes blob,
                          PacketSequenceStateGeneration* seq_state);

  void MaybeOnFirstFtraceEvent();
  StringId InternedKernelSymbolOrFallback(
      uint64_t key,
      PacketSequenceStateGeneration* seq_state);
  void ParseTrustySmc(uint32_t pid, int64_t timestamp, protozero::ConstBytes);
  void ParseTrustySmcDone(uint32_t pid,
                          int64_t timestamp,
                          protozero::ConstBytes);
  void ParseTrustyStdCall32(uint32_t pid,
                            int64_t ts,
                            protozero::ConstBytes data);
  void ParseTrustyStdCall32Done(uint32_t pid,
                                int64_t ts,
                                protozero::ConstBytes data);
  void ParseTrustyShareMemory(uint32_t pid, int64_t ts, protozero::ConstBytes);
  void ParseTrustyShareMemoryDone(uint32_t pid,
                                  int64_t ts,
                                  protozero::ConstBytes);
  void ParseTrustyReclaimMemory(uint32_t pid,
                                int64_t ts,
                                protozero::ConstBytes);
  void ParseTrustyReclaimMemoryDone(uint32_t pid,
                                    int64_t ts,
                                    protozero::ConstBytes);
  void ParseTrustyIrq(uint32_t pid, int64_t ts, protozero::ConstBytes);
  void ParseTrustyIpcHandleEvent(uint32_t pid,
                                 int64_t ts,
                                 protozero::ConstBytes);
  void ParseTrustyIpcConnect(uint32_t pid, int64_t ts, protozero::ConstBytes);
  void ParseTrustyIpcConnectEnd(uint32_t pid,
                                int64_t ts,
                                protozero::ConstBytes);
  void ParseTrustyIpcWrite(uint32_t pid, int64_t ts, protozero::ConstBytes);
  void ParseTrustyIpcWriteEnd(uint32_t pid, int64_t ts, protozero::ConstBytes);
  void ParseTrustyIpcRead(uint32_t pid, int64_t ts, protozero::ConstBytes);
  void ParseTrustyIpcReadEnd(uint32_t pid, int64_t ts, protozero::ConstBytes);
  void ParseTrustyIpcPoll(uint32_t pid, int64_t ts, protozero::ConstBytes);
  void ParseTrustyIpcRx(uint32_t pid, int64_t ts, protozero::ConstBytes);
  void ParseTrustyEnqueueNop(uint32_t pid, int64_t ts, protozero::ConstBytes);
  void ParseAndroidFsDatareadEnd(int64_t timestamp, protozero::ConstBytes);
  void ParseAndroidFsDatareadStart(int64_t ts,
                                   uint32_t pid,
                                   protozero::ConstBytes);
  StringId GetRpmStatusStringId(int32_t rpm_status_val);
  void ParseRpmStatus(int64_t ts, protozero::ConstBytes);
  void ParseDevicePmCallbackStart(int64_t ts,
                                  uint32_t cpu,
                                  uint32_t pid,
                                  protozero::ConstBytes);
  void ParseDevicePmCallbackEnd(int64_t ts, protozero::ConstBytes);
  void ParsePanelWriteGeneric(int64_t timestamp,
                              uint32_t pid,
                              protozero::ConstBytes);
  void ParseGoogleIccEvent(int64_t timestamp, protozero::ConstBytes);
  void ParseGoogleIrmEvent(int64_t timestamp, protozero::ConstBytes);
  void ParseDeviceFrequency(int64_t ts, protozero::ConstBytes blob);
  void ParseParamSetValueCpm(protozero::ConstBytes blob);
  void ParseBlockIoStart(int64_t ts, protozero::ConstBytes blob);
  void ParseBlockIoDone(int64_t ts, protozero::ConstBytes blob);
  void ParseCpuhpEnter(uint32_t fld_id,
                       int64_t ts,
                       uint32_t cpu,
                       protozero::ConstBytes blob);
  void ParseCpuhpExit(int64_t ts, protozero::ConstBytes blob);

  TraceProcessorContext* context_;
  RssStatTracker rss_stat_tracker_;
  DrmTracker drm_tracker_;
  IostatTracker iostat_tracker_;
  VirtioGpuTracker virtio_gpu_tracker_;
  MaliGpuEventTracker mali_gpu_event_tracker_;
  PkvmHypervisorCpuTracker pkvm_hyp_cpu_tracker_;
  GpuWorkPeriodTracker gpu_work_period_tracker_;
  ThermalTracker thermal_tracker_;
  PixelMmKswapdEventTracker pixel_mm_kswapd_event_tracker_;

  const StringId sched_wakeup_name_id_;
  const StringId sched_waking_name_id_;
  const StringId cpu_id_;
  const StringId suspend_resume_minimal_slice_name_id_;
  const StringId inode_arg_id_;
  const StringId signal_generate_id_;
  const StringId signal_deliver_id_;
  const StringId lmk_id_;
  const StringId comm_name_id_;
  const StringId signal_name_id_;
  const StringId oom_kill_id_;
  const StringId workqueue_id_;
  const StringId irq_id_;
  const StringId tcp_state_id_;
  const StringId tcp_event_id_;
  const StringId protocol_arg_id_;
  const StringId napi_gro_id_;
  const StringId ret_arg_id_;
  const StringId len_arg_id_;
  const StringId direct_reclaim_nr_reclaimed_id_;
  const StringId direct_reclaim_order_id_;
  const StringId direct_reclaim_may_writepage_id_;
  const StringId direct_reclaim_gfp_flags_id_;
  const StringId vec_arg_id_;
  const StringId io_wait_id_;
  const StringId function_id_;
  const StringId waker_utid_id_;
  const StringId cros_ec_arg_num_id_;
  const StringId cros_ec_arg_ec_id_;
  const StringId cros_ec_arg_sample_ts_id_;
  const StringId shrink_slab_id_;
  const StringId shrink_name_id_;
  const StringId shrink_total_scan_id_;
  const StringId shrink_freed_id_;
  const StringId shrink_priority_id_;
  const StringId trusty_category_id_;
  const StringId trusty_name_trusty_std_id_;
  const StringId trusty_name_tipc_rx_id_;
  const StringId cma_alloc_id_;
  const StringId cma_name_id_;
  const StringId cma_pfn_id_;
  const StringId cma_req_pages_id_;
  const StringId cma_nr_migrated_id_;
  const StringId cma_nr_reclaimed_id_;
  const StringId cma_nr_mapped_id_;
  const StringId cma_nr_isolate_fail_id_;
  const StringId cma_nr_migrate_fail_id_;
  const StringId cma_nr_test_fail_id_;
  const StringId syscall_ret_id_;
  const StringId syscall_args_id_;
  const StringId replica_slice_id_;
  const StringId file_path_id_;
  const StringId offset_id_start_;
  const StringId offset_id_end_;
  const StringId bytes_read_id_start_;
  const StringId bytes_read_id_end_;
  const StringId android_fs_data_read_id_;
  const StringId google_icc_event_id_;
  const StringId google_irm_event_id_;
  const StringId runtime_status_invalid_id_;
  const StringId runtime_status_active_id_;
  const StringId runtime_status_suspending_id_;
  const StringId runtime_status_resuming_id_;
  const StringId suspend_resume_main_event_id_;
  const StringId suspend_resume_device_pm_event_id_;
  const StringId suspend_resume_utid_arg_name_;
  const StringId suspend_resume_device_arg_name_;
  const StringId suspend_resume_driver_arg_name_;
  const StringId suspend_resume_callback_phase_arg_name_;
  const StringId suspend_resume_event_type_arg_name_;
  const StringId device_name_id_;
  const StringId block_io_id_;
  const StringId block_io_arg_sector_id_;
  const StringId cpuhp_action_cpu_id_;
  const StringId cpuhp_idx_id_;
  const StringId disp_vblank_irq_enable_id_;
  const StringId disp_vblank_irq_enable_output_id_arg_name_;

  std::vector<StringId> syscall_arg_name_ids_;

  struct FtraceMessageStrings {
    // The string id of name of the event field (e.g. sched_switch's id).
    StringId message_name_id = kNullStringId;
    std::array<StringId, kMaxFtraceEventFields> field_name_ids;
  };
  std::vector<FtraceMessageStrings> ftrace_message_strings_;

  struct MmEventCounterNames {
    MmEventCounterNames() = default;
    MmEventCounterNames(StringId _count, StringId _max_lat, StringId _avg_lat)
        : count(_count), max_lat(_max_lat), avg_lat(_avg_lat) {}

    StringId count = kNullStringId;
    StringId max_lat = kNullStringId;
    StringId avg_lat = kNullStringId;
  };

  // Keep kMmEventCounterSize equal to mm_event_type::MM_TYPE_NUM in the kernel.
  static constexpr size_t kMmEventCounterSize = 7;
  std::array<MmEventCounterNames, kMmEventCounterSize> mm_event_counter_names_;

  // Record number of received bytes from the network interface card.
  std::unordered_map<std::string, uint64_t> nic_received_bytes_;

  // Record number of transmitted bytes to the network interface card.
  std::unordered_map<std::string, uint64_t> nic_transmitted_bytes_;

  // Record number of kfree_skb with ip protocol.
  uint64_t num_of_kfree_skb_ip_prot = 0;

  // Keep sock to stream number mapping.
  std::unordered_map<uint64_t, uint32_t> skaddr_to_stream_;

  // Record number of tcp steams.
  uint32_t num_of_tcp_stream_ = 0;

  // A name collision is possible, always show if active wakelock exists
  // with a give name
  std::unordered_map<std::string, uint32_t> active_wakelock_to_count_;

  // Tiny state machine for tracking the suspend_enter bug.
  enum SuspendEnterBugState {
    // Nothing special.
    SUSPEND_STATE_INITIAL,
    // We are inside a suspend_enter slice.
    SUSPEND_STATE_ENTER,
    // We are inside a suspend_enter slice and a freeze_processes slice.
    SUSPEND_STATE_FREEZE
  };

  SuspendEnterBugState suspend_state_ = SUSPEND_STATE_INITIAL;

  // cookie for the current suspend_enter slice if any; needed to close it.
  int64_t suspend_enter_slice_cookie_ = 0;

  bool has_seen_first_ftrace_packet_ = false;

  // Ftrace events before this timestamp get dropped.
  int64_t drop_ftrace_data_before_ts_ = 0;

  // Ftrace events before this timestamp get parsed into the |ftrace_events|
  // table, but don't get propagated into other tables/trackers.
  // Must be no less than drop_ftrace_data_before_ts_.
  int64_t soft_drop_ftrace_data_before_ts_ = 0;

  // Does not skip any ftrace events.
  bool preserve_ftrace_buffer_ = false;

  // Sequence ids for which ftrace_errors have been seen. Used to avoid
  // putting them in the metadata multiple times (the ftrace data sources
  // re-emits begin stats on every flush).
  std::unordered_set<uint32_t> seen_errors_for_sequence_id_;

  // Tracks Linux devices with active runtime power management (RPM) status
  // slices.
  std::unordered_set<TrackId> active_rpm_tracks_;

  struct PairHash {
    std::size_t operator()(const std::pair<uint64_t, int64_t>& p) const {
      base::Hasher hasher;
      hasher.Update(p.first);
      hasher.Update(p.second);
      return static_cast<std::size_t>(hasher.digest());
    }
  };

  base::FlatHashMap<std::pair<uint64_t, int64_t>, uint32_t, PairHash>
      inode_offset_thread_map_;
};

}  // namespace perfetto::trace_processor

#endif  // SRC_TRACE_PROCESSOR_IMPORTERS_FTRACE_FTRACE_PARSER_H_
