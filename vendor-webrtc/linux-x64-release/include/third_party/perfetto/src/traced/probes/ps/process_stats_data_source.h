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

#ifndef SRC_TRACED_PROBES_PS_PROCESS_STATS_DATA_SOURCE_H_
#define SRC_TRACED_PROBES_PS_PROCESS_STATS_DATA_SOURCE_H_

#include <functional>
#include <limits>
#include <memory>
#include <unordered_map>
#include <utility>
#include <vector>

#include "perfetto/base/flat_set.h"
#include "perfetto/ext/base/scoped_file.h"
#include "perfetto/ext/base/weak_ptr.h"
#include "perfetto/ext/tracing/core/basic_types.h"
#include "perfetto/ext/tracing/core/trace_writer.h"
#include "perfetto/tracing/core/forward_decls.h"
#include "src/traced/probes/probes_data_source.h"

namespace perfetto {

namespace base {
class TaskRunner;
}

namespace protos {
namespace pbzero {
class ProcessTree;
class ProcessStats;
class ProcessStats_Process;
}  // namespace pbzero
}  // namespace protos

class ProcessStatsDataSource : public ProbesDataSource {
 public:
  static const ProbesDataSource::Descriptor descriptor;

  ProcessStatsDataSource(base::TaskRunner*,
                         TracingSessionID,
                         std::unique_ptr<TraceWriter> writer,
                         const DataSourceConfig&);
  ~ProcessStatsDataSource() override;

  base::WeakPtr<ProcessStatsDataSource> GetWeakPtr() const;
  void WriteAllProcesses();
  void OnPids(const base::FlatSet<int32_t>& pids);
  void OnRenamePids(const base::FlatSet<int32_t>& pids);
  void OnFds(const base::FlatSet<std::pair<pid_t, uint64_t>>& fds);

  // ProbesDataSource implementation.
  void Start() override;
  void Flush(FlushRequestID, std::function<void()> callback) override;
  void ClearIncrementalState() override;

  bool on_demand_dumps_enabled() const { return enable_on_demand_dumps_; }

  // Virtual for testing.
  virtual const char* GetProcMountpoint();
  virtual base::ScopedDir OpenProcDir();
  virtual std::string ReadProcPidFile(int32_t pid, const std::string& file);

 private:
  struct CachedProcessStats {
    uint32_t vm_size_kb = std::numeric_limits<uint32_t>::max();
    uint32_t vm_rss_kb = std::numeric_limits<uint32_t>::max();
    uint32_t rss_anon_kb = std::numeric_limits<uint32_t>::max();
    uint32_t rss_file_kb = std::numeric_limits<uint32_t>::max();
    uint32_t rss_shmem_kb = std::numeric_limits<uint32_t>::max();
    uint32_t vm_swap_kb = std::numeric_limits<uint32_t>::max();
    uint32_t vm_locked_kb = std::numeric_limits<uint32_t>::max();
    uint32_t vm_hvm_kb = std::numeric_limits<uint32_t>::max();
    int32_t oom_score_adj = std::numeric_limits<int32_t>::max();
    uint32_t smr_rss_kb = std::numeric_limits<uint32_t>::max();
    uint32_t smr_pss_kb = std::numeric_limits<uint32_t>::max();
    uint32_t smr_pss_anon_kb = std::numeric_limits<uint32_t>::max();
    uint32_t smr_pss_file_kb = std::numeric_limits<uint32_t>::max();
    uint32_t smr_pss_shmem_kb = std::numeric_limits<uint32_t>::max();
    uint32_t smr_swap_pss_kb = std::numeric_limits<uint32_t>::max();
    uint64_t runtime_user_mode_ns = std::numeric_limits<uint64_t>::max();
    uint64_t runtime_kernel_mode_ns = std::numeric_limits<uint64_t>::max();
    // file descriptors
    base::FlatSet<uint64_t> seen_fds;
  };

  // Common functions.
  ProcessStatsDataSource(const ProcessStatsDataSource&) = delete;
  ProcessStatsDataSource& operator=(const ProcessStatsDataSource&) = delete;

  void StartNewPacketIfNeeded();
  void FinalizeCurPacket();
  protos::pbzero::ProcessTree* GetOrCreatePsTree();
  protos::pbzero::ProcessStats* GetOrCreateStats();
  protos::pbzero::ProcessStats_Process* GetOrCreateStatsProcess(int32_t pid);

  // Functions for snapshotting process/thread long-term info and relationships.
  bool WriteProcess(int32_t pid,
                    const std::string& proc_status,
                    const std::string& proc_stat);
  void WriteThread(int32_t tid, int32_t tgid);
  void WriteDetailedThread(int32_t tid,
                           int32_t tgid,
                           const std::string& proc_status);
  void WriteProcessOrThread(int32_t pid);

  // Functions for periodically sampling process stats/counters.
  static void Tick(base::WeakPtr<ProcessStatsDataSource>);
  void WriteAllProcessStats();
  bool WriteProcessRuntimes(int32_t pid, const std::string& proc_stat);
  bool WriteMemCounters(int32_t pid, const std::string& proc_status);
  void WriteFds(int32_t pid);
  void WriteSingleFd(int32_t pid, uint64_t fd);

  // Scans /proc/pid/status and writes the ProcessTree packet for input pids.
  void WriteProcessTree(const base::FlatSet<int32_t>&);

  // Read and "latch" the current procfs scan-start timestamp, which
  // we reset only in FinalizeCurPacket.
  uint64_t CacheProcFsScanStartTimestamp();

  // Common fields used for both process/tree relationships and stats/counters.
  base::TaskRunner* const task_runner_;
  std::unique_ptr<TraceWriter> writer_;
  TraceWriter::TracePacketHandle cur_packet_;

  // Cached before-scan timestamp; zero means cached time is absent.
  // By the time we create the trace packet into which we dump procfs
  // scan results, we've already read at least one bit of data from
  // procfs, and by that point, it's too late to snap a timestamp from
  // before we started looking at procfs at all, which is what trace
  // analysis wants.  To solve this problem, we record the scan-start
  // timestamp here when we first open something in procfs and use
  // that time when we create the packet.
  // We reset this field after each FinalizeCurPacket().
  uint64_t cur_procfs_scan_start_timestamp_ = 0;

  // Fields for keeping track of the state of process/tree relationships.
  protos::pbzero::ProcessTree* cur_ps_tree_ = nullptr;
  bool record_thread_names_ = false;
  bool enable_on_demand_dumps_ = true;
  bool dump_all_procs_on_start_ = false;
  bool resolve_process_fds_ = false;
  bool scan_smaps_rollup_ = false;
  bool record_process_age_ = false;
  bool record_process_runtime_ = false;

  // This set contains PIDs as per the Linux kernel notion of a PID (which is
  // really a TID). In practice this set will contain all TIDs for all processes
  // seen, not just the main thread id (aka thread group ID).
  struct SeenPid {
    int32_t pid;
    int32_t tgid;

    SeenPid(int32_t _pid, int32_t _tgid = 0) : pid(_pid), tgid(_tgid) {}
    // TODO(rsavitski): add comparator support to FlatSet
    bool operator==(const SeenPid& other) const { return pid == other.pid; }
    bool operator<(const SeenPid& other) const { return pid < other.pid; }
  };
  base::FlatSet<SeenPid> seen_pids_;

  // Fields for keeping track of the periodic stats/counters.
  uint32_t poll_period_ms_ = 0;
  uint64_t cache_ticks_ = 0;
  protos::pbzero::ProcessStats* cur_ps_stats_ = nullptr;
  protos::pbzero::ProcessStats_Process* cur_ps_stats_process_ = nullptr;
  std::vector<bool> skip_mem_for_pids_;

  // Cached process stats per process. Cleared every |cache_ttl_ticks_| *
  // |poll_period_ms_| ms.
  uint32_t process_stats_cache_ttl_ticks_ = 0;
  std::unordered_map<int32_t, CachedProcessStats> process_stats_cache_;

  // If true, the next trace packet will have the |incremental_state_cleared|
  // flag set. Set initially and when handling a ClearIncrementalState call.
  bool did_clear_incremental_state_ = true;

  base::WeakPtrFactory<ProcessStatsDataSource> weak_factory_;  // Keep last.
};

}  // namespace perfetto

#endif  // SRC_TRACED_PROBES_PS_PROCESS_STATS_DATA_SOURCE_H_
