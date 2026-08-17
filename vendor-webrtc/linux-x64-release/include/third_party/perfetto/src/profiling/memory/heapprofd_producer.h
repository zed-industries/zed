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

#ifndef SRC_PROFILING_MEMORY_HEAPPROFD_PRODUCER_H_
#define SRC_PROFILING_MEMORY_HEAPPROFD_PRODUCER_H_

#include <array>
#include <cinttypes>
#include <functional>
#include <map>
#include <optional>
#include <vector>

#include "perfetto/base/task_runner.h"
#include "perfetto/ext/base/unix_socket.h"
#include "perfetto/ext/base/unix_task_runner.h"
#include "perfetto/ext/tracing/core/basic_types.h"
#include "perfetto/ext/tracing/core/producer.h"
#include "perfetto/ext/tracing/core/trace_writer.h"
#include "perfetto/ext/tracing/core/tracing_service.h"
#include "perfetto/tracing/core/data_source_config.h"
#include "perfetto/tracing/core/forward_decls.h"
#include "src/profiling/common/interning_output.h"
#include "src/profiling/common/proc_utils.h"
#include "src/profiling/common/profiler_guardrails.h"
#include "src/profiling/memory/bookkeeping.h"
#include "src/profiling/memory/bookkeeping_dump.h"
#include "src/profiling/memory/log_histogram.h"
#include "src/profiling/memory/shared_ring_buffer.h"
#include "src/profiling/memory/system_property.h"
#include "src/profiling/memory/unwinding.h"
#include "src/profiling/memory/unwound_messages.h"

#include "protos/perfetto/config/profiling/heapprofd_config.gen.h"

namespace perfetto {
namespace profiling {

using HeapprofdConfig = protos::gen::HeapprofdConfig;

struct Process {
  pid_t pid;
  std::string cmdline;
};

// TODO(rsavitski): central daemon can do less work if it knows that the global
// operating mode is fork-based, as it then will not be interacting with the
// clients. This can be implemented as an additional mode here.
enum class HeapprofdMode { kCentral, kChild };

bool HeapprofdConfigToClientConfiguration(
    const HeapprofdConfig& heapprofd_config,
    ClientConfiguration* cli_config);

// Heap profiling producer. Can be instantiated in two modes, central and
// child (also referred to as fork mode).
//
// The central mode producer is instantiated by the system heapprofd daemon. Its
// primary responsibility is activating profiling (via system properties and
// signals) in targets identified by profiling configs. On debug platform
// builds, the central producer can also handle the out-of-process unwinding &
// writing of the profiles for all client processes.
//
// An alternative model is where the central heapprofd triggers the profiling in
// the target process, but the latter fork-execs a private heapprofd binary to
// handle unwinding only for that process. The forked heapprofd instantiates
// this producer in the "child" mode. In this scenario, the profiled process
// never talks to the system daemon.
//
// TODO(fmayer||rsavitski): cover interesting invariants/structure of the
// implementation (e.g. number of data sources in child mode), including
// threading structure.
class HeapprofdProducer : public Producer, public UnwindingWorker::Delegate {
 public:
  friend class SocketDelegate;

  // TODO(fmayer): Split into two delegates for the listening socket in kCentral
  // and for the per-client sockets to make this easier to understand?
  // Alternatively, find a better name for this.
  class SocketDelegate : public base::UnixSocket::EventListener {
   public:
    explicit SocketDelegate(HeapprofdProducer* producer)
        : producer_(producer) {}

    void OnDisconnect(base::UnixSocket* self) override;
    void OnNewIncomingConnection(
        base::UnixSocket* self,
        std::unique_ptr<base::UnixSocket> new_connection) override;
    void OnDataAvailable(base::UnixSocket* self) override;

   private:
    HeapprofdProducer* producer_;
  };

  HeapprofdProducer(HeapprofdMode mode,
                    base::TaskRunner* task_runner,
                    bool exit_when_done);
  ~HeapprofdProducer() override;

  // Producer Impl:
  void OnConnect() override;
  void OnDisconnect() override;
  void SetupDataSource(DataSourceInstanceID, const DataSourceConfig&) override;
  void StartDataSource(DataSourceInstanceID, const DataSourceConfig&) override;
  void StopDataSource(DataSourceInstanceID) override;
  void OnTracingSetup() override;
  void Flush(FlushRequestID,
             const DataSourceInstanceID* data_source_ids,
             size_t num_data_sources,
             FlushFlags) override;
  void ClearIncrementalState(const DataSourceInstanceID* /*data_source_ids*/,
                             size_t /*num_data_sources*/) override {}

  // TODO(fmayer): Refactor once/if we have generic reconnect logic.
  void ConnectWithRetries(const char* socket_name);
  void DumpAll();

  // UnwindingWorker::Delegate impl:
  void PostAllocRecord(UnwindingWorker*, std::unique_ptr<AllocRecord>) override;
  void PostFreeRecord(UnwindingWorker*, std::vector<FreeRecord>) override;
  void PostHeapNameRecord(UnwindingWorker*, HeapNameRecord) override;
  void PostSocketDisconnected(UnwindingWorker*,
                              DataSourceInstanceID,
                              pid_t,
                              SharedRingBuffer::Stats) override;
  void PostDrainDone(UnwindingWorker*, DataSourceInstanceID) override;

  void HandleAllocRecord(AllocRecord*);
  void HandleFreeRecord(FreeRecord);
  void HandleHeapNameRecord(HeapNameRecord);
  void HandleSocketDisconnected(DataSourceInstanceID,
                                pid_t,
                                SharedRingBuffer::Stats);

  // Valid only if mode_ == kChild.
  void SetTargetProcess(pid_t target_pid, std::string target_cmdline);
  void SetDataSourceCallback(std::function<void()> fn);

  // Exposed for testing.
  void SetProducerEndpoint(
      std::unique_ptr<TracingService::ProducerEndpoint> endpoint);

  base::UnixSocket::EventListener& socket_delegate() {
    return socket_delegate_;
  }

  // Adopts the (connected) sockets inherited from the target process, invoking
  // the on-connection callback.
  // Specific to mode_ == kChild
  void AdoptSocket(base::ScopedFile fd);

  void TerminateWhenDone();

 private:
  // State of the connection to tracing service (traced).
  enum State {
    kNotStarted = 0,
    kNotConnected,
    kConnecting,
    kConnected,
  };

  struct ProcessState {
    struct HeapInfo {
      HeapInfo(GlobalCallstackTrie* cs, bool dam) : heap_tracker(cs, dam) {}

      HeapTracker heap_tracker;
      std::string heap_name;
      uint64_t sampling_interval = 0u;
      uint64_t orig_sampling_interval = 0u;
    };
    ProcessState(GlobalCallstackTrie* c, bool d)
        : callsites(c), dump_at_max_mode(d) {}
    bool disconnected = false;
    SharedRingBuffer::ErrorState error_state =
        SharedRingBuffer::ErrorState::kNoError;
    bool buffer_corrupted = false;

    uint64_t heap_samples = 0;
    uint64_t map_reparses = 0;
    uint64_t unwinding_errors = 0;

    uint64_t total_unwinding_time_us = 0;
    uint64_t client_spinlock_blocked_us = 0;
    GlobalCallstackTrie* callsites;
    bool dump_at_max_mode;
    LogHistogram unwinding_time_us;
    std::map<uint32_t, HeapInfo> heap_infos;

    HeapInfo& GetHeapInfo(uint32_t heap_id) {
      auto it = heap_infos.find(heap_id);
      if (it == heap_infos.end()) {
        it = heap_infos.emplace_hint(
            it, std::piecewise_construct, std::forward_as_tuple(heap_id),
            std::forward_as_tuple(callsites, dump_at_max_mode));
      }
      return it->second;
    }

    HeapTracker& GetHeapTracker(uint32_t heap_id) {
      return GetHeapInfo(heap_id).heap_tracker;
    }
  };

  struct DataSource {
    explicit DataSource(std::unique_ptr<TraceWriter> tw)
        : trace_writer(std::move(tw)) {
      // Make MSAN happy.
      memset(&client_configuration, 0, sizeof(client_configuration));
    }

    DataSourceInstanceID id;
    std::unique_ptr<TraceWriter> trace_writer;
    DataSourceConfig ds_config;
    HeapprofdConfig config;
    ClientConfiguration client_configuration;
    std::vector<SystemProperties::Handle> properties;
    std::set<pid_t> signaled_pids;
    std::set<pid_t> rejected_pids;
    std::map<pid_t, ProcessState> process_states;
    std::vector<std::string> normalized_cmdlines;
    InterningOutputTracker intern_state;
    bool shutting_down = false;
    bool started = false;
    bool hit_guardrail = false;
    bool was_stopped = false;
    uint32_t stop_timeout_ms;
    uint32_t dump_interval_ms = 0;
    size_t pending_free_drains = 0;
    GuardrailConfig guardrail_config;
  };

  struct PendingProcess {
    std::unique_ptr<base::UnixSocket> sock;
    DataSourceInstanceID data_source_instance_id;
    SharedRingBuffer shmem;
  };

  void HandleClientConnection(std::unique_ptr<base::UnixSocket> new_connection,
                              Process process);

  void ConnectService();
  void Restart();
  void ResetConnectionBackoff();
  void IncreaseConnectionBackoff();

  void CheckDataSourceMemoryTask();
  void CheckDataSourceCpuTask();

  void FinishDataSourceFlush(FlushRequestID flush_id);
  void DumpProcessesInDataSource(DataSource* ds);
  void DumpProcessState(DataSource* ds, pid_t pid, ProcessState* process);
  static void SetStats(protos::pbzero::ProfilePacket::ProcessStats* stats,
                       const ProcessState& process_state);

  void DoDrainAndContinuousDump(DataSourceInstanceID id);
  void DoContinuousDump(DataSource* ds);
  void DrainDone(DataSourceInstanceID);

  UnwindingWorker& UnwinderForPID(pid_t);
  bool IsPidProfiled(pid_t);
  DataSource* GetDataSourceForProcess(const Process& proc);
  void RecordOtherSourcesAsRejected(DataSource* active_ds, const Process& proc);

  void SetStartupProperties(DataSource* data_source);
  void SignalRunningProcesses(DataSource* data_source);

  // Specific to mode_ == kChild
  void TerminateProcess(int exit_status);

  void ShutdownDataSource(DataSource* ds);
  bool MaybeFinishDataSource(DataSource* ds);

  void WriteRejectedConcurrentSession(BufferID buffer_id, pid_t pid);

  // Class state:

  // Task runner is owned by the main thread.
  base::TaskRunner* const task_runner_;
  const HeapprofdMode mode_;
  // TODO(fmayer): Refactor to make this boolean unnecessary.
  // Whether to terminate this producer after the first data-source has
  // finished.
  bool exit_when_done_;

  // State of connection to the tracing service.
  State state_ = kNotStarted;
  uint32_t connection_backoff_ms_ = 0;
  const char* producer_sock_name_ = nullptr;

  // Client processes that have connected, but with which we have not yet
  // finished the handshake.
  std::map<pid_t, PendingProcess> pending_processes_;

  // Must outlive data_sources_ - owns at least the shared memory referenced by
  // TraceWriters.
  std::unique_ptr<TracingService::ProducerEndpoint> endpoint_;

  // Must outlive data_sources_ - HeapTracker references the trie.
  GlobalCallstackTrie callsites_;

  // Must outlive data_sources_ - DataSource can hold
  // SystemProperties::Handle-s.
  // Specific to mode_ == kCentral
  SystemProperties properties_;

  std::map<FlushRequestID, size_t> flushes_in_progress_;
  std::map<DataSourceInstanceID, DataSource> data_sources_;

  // Specific to mode_ == kChild
  Process target_process_{base::kInvalidPid, ""};
  std::optional<std::function<void()>> data_source_callback_;

  SocketDelegate socket_delegate_;

  base::WeakPtrFactory<HeapprofdProducer> weak_factory_;

  // UnwindingWorker's destructor might attempt to post producer tasks, so this
  // needs to outlive weak_factory_.
  std::vector<UnwindingWorker> unwinding_workers_;
};

}  // namespace profiling
}  // namespace perfetto

#endif  // SRC_PROFILING_MEMORY_HEAPPROFD_PRODUCER_H_
