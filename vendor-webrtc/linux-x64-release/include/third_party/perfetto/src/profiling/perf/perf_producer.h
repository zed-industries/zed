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

#ifndef SRC_PROFILING_PERF_PERF_PRODUCER_H_
#define SRC_PROFILING_PERF_PERF_PRODUCER_H_

#include <unistd.h>
#include <map>
#include <memory>
#include <optional>

#include "perfetto/base/task_runner.h"
#include "perfetto/ext/base/scoped_file.h"
#include "perfetto/ext/base/unix_socket.h"
#include "perfetto/ext/base/weak_ptr.h"
#include "perfetto/ext/tracing/core/basic_types.h"
#include "perfetto/ext/tracing/core/producer.h"
#include "perfetto/ext/tracing/core/trace_writer.h"
#include "perfetto/ext/tracing/core/tracing_service.h"
#include "src/profiling/common/callstack_trie.h"
#include "src/profiling/common/interning_output.h"
#include "src/profiling/perf/common_types.h"
#include "src/profiling/perf/event_config.h"
#include "src/profiling/perf/event_reader.h"
#include "src/profiling/perf/proc_descriptors.h"
#include "src/profiling/perf/unwinding.h"
#include "src/tracing/service/metatrace_writer.h"
// TODO(rsavitski): move to e.g. src/tracefs/.
#include "src/traced/probes/ftrace/ftrace_procfs.h"

namespace perfetto {
namespace profiling {

// TODO(rsavitski): describe the high-level architecture and threading. Rough
// summary in the mean time: three stages: (1) kernel buffer reader that parses
// the samples -> (2) callstack unwinder -> (3) interning and serialization of
// samples. This class handles stages (1) and (3) on the main thread. Unwinding
// is done by |Unwinder| on a dedicated thread.
class PerfProducer : public Producer,
                     public ProcDescriptorDelegate,
                     public Unwinder::Delegate {
 public:
  PerfProducer(ProcDescriptorGetter* proc_fd_getter,
               base::TaskRunner* task_runner);
  ~PerfProducer() override = default;

  PerfProducer(const PerfProducer&) = delete;
  PerfProducer& operator=(const PerfProducer&) = delete;
  PerfProducer(PerfProducer&&) = delete;
  PerfProducer& operator=(PerfProducer&&) = delete;

  void ConnectWithRetries(const char* socket_name);

  // Producer impl:
  void OnConnect() override;
  void OnDisconnect() override;
  void OnTracingSetup() override {}
  void SetupDataSource(DataSourceInstanceID, const DataSourceConfig&) override;
  void StartDataSource(DataSourceInstanceID instance_id,
                       const DataSourceConfig& config) override;
  void StopDataSource(DataSourceInstanceID instance_id) override;
  void Flush(FlushRequestID flush_id,
             const DataSourceInstanceID* data_source_ids,
             size_t num_data_sources,
             FlushFlags) override;
  void ClearIncrementalState(const DataSourceInstanceID* data_source_ids,
                             size_t num_data_sources) override;

  // ProcDescriptorDelegate impl:
  void OnProcDescriptors(pid_t pid,
                         uid_t uid,
                         base::ScopedFile maps_fd,
                         base::ScopedFile mem_fd) override;

  // Unwinder::Delegate impl (callbacks from unwinder):
  void PostEmitSample(DataSourceInstanceID ds_id,
                      CompletedSample sample) override;
  void PostEmitUnwinderSkippedSample(DataSourceInstanceID ds_id,
                                     ParsedSample sample) override;
  void PostFinishDataSourceStop(DataSourceInstanceID ds_id) override;

  // Calls `cb` when all data sources have been registered.
  void SetAllDataSourcesRegisteredCb(std::function<void()> cb) {
    all_data_sources_registered_cb_ = std::move(cb);
  }

  // public for testing:
  static bool ShouldRejectDueToFilter(
      pid_t pid,
      const TargetFilter& filter,
      bool skip_cmdline,
      base::FlatSet<std::string>* additional_cmdlines,
      std::function<bool(std::string*)> read_proc_pid_cmdline);

 private:
  // State of the producer's connection to tracing service (traced).
  enum State {
    kNotStarted = 0,
    kNotConnected,
    kConnecting,
    kConnected,
  };

  // Represents the data source scoped view of a process:
  // * whether the process is in scope of the tracing session (if the latter
  //   specifies a callstack sampling target filter).
  // * for userspace processes, the state of the (possibly asynchronous) lookup
  //   of /proc/<pid>/{maps,mem} file descriptors, which are necessary for
  //   callstack unwinding of samples.
  // For kernel threads (or when sampling only kernelspace callstacks) the
  // proc-fds are not necessary, so those processes transition directly to
  // either kAccepted or kRejected.
  // TODO(rsavitski): double-check and clarify pid reuse semantics. For
  // userspace sampling, at most one incarnation of the pid is handled since we
  // do not obtain new proc descriptors. But counter-only and kernel-only cases
  // aren't as stateful and will keep emitting samples.
  enum class ProcessTrackingStatus {
    kInitial = 0,
    kFdsResolving,  // waiting on proc-fd lookup
    kAccepted,      // process relevant and ready for unwinding (for userspace -
                    // procfds received)
    kFdsTimedOut,   // proc-fd lookup timed out
    kRejected       // process not considered relevant for the data source
  };

  struct DataSourceState {
    enum class Status { kActive, kShuttingDown };

    DataSourceState(EventConfig _event_config,
                    uint64_t _tracing_session_id,
                    std::unique_ptr<TraceWriter> _trace_writer,
                    std::vector<EventReader> _per_cpu_readers)
        : event_config(std::move(_event_config)),
          tracing_session_id(_tracing_session_id),
          trace_writer(std::move(_trace_writer)),
          per_cpu_readers(std::move(_per_cpu_readers)) {}

    Status status = Status::kActive;
    const EventConfig event_config;
    uint64_t tracing_session_id;
    std::unique_ptr<TraceWriter> trace_writer;
    // Indexed by cpu, vector never resized.
    std::vector<EventReader> per_cpu_readers;
    // Tracks the incremental state for interned entries.
    InterningOutputTracker interning_output;
    // Producer thread's view of sampled processes. This is the primary tracking
    // structure, but a subset of updates are replicated to a similar structure
    // in the |Unwinder|, which needs to track whether the necessary unwinding
    // inputs for a given process' samples are ready.
    std::map<pid_t, ProcessTrackingStatus> process_states;
    // Additional state for EventConfig.TargetFilter: command lines we have
    // decided to unwind, up to a total of additional_cmdline_count values.
    base::FlatSet<std::string> additional_cmdlines;
  };

  // For |EmitSkippedSample|.
  enum class SampleSkipReason {
    kReadFdTimeout = 0,  // discarded since fd lookup previously timed out
    kUnwindEnqueue,      // discarded due to unwinder queue being full
    kUnwindStage,        // discarded at unwind stage (any reason)
    kRejected,           // doesn't match target scope from the config
  };

  void ConnectService();
  void Restart();
  void ResetConnectionBackoff();
  void IncreaseConnectionBackoff();

  // Periodic read task.
  void TickDataSourceRead(DataSourceInstanceID ds_id);
  // Snapshots the current values of the counters using the read syscall.
  void ReadCounters(DataSourceState& ds);
  // Reads a batch of samples from all kernel ring buffers for this data source.
  bool ReadRingBuffers(DataSourceInstanceID ds_id, DataSourceState& ds);

  // Returns *false* if the reader has caught up with the writer position, true
  // otherwise. Return value is only useful if the underlying perf_event has
  // been paused (to identify when the buffer is empty). |max_samples| is a cap
  // on the amount of samples that will be parsed, which might be more than the
  // number of underlying records (as there might be non-sample records).
  bool ReadAndParsePerCpuBuffer(EventReader* reader,
                                uint64_t max_samples,
                                DataSourceInstanceID ds_id,
                                DataSourceState& ds);

  void InitiateDescriptorLookup(DataSourceInstanceID ds_id,
                                pid_t pid,
                                uint32_t timeout_ms);
  // Do not call directly, use |InitiateDescriptorLookup|.
  void StartDescriptorLookup(DataSourceInstanceID ds_id,
                             pid_t pid,
                             uint32_t timeout_ms);
  void EvaluateDescriptorLookupTimeout(DataSourceInstanceID ds_id, pid_t pid);

  void EmitCounterOnlySample(DataSourceState& ds,
                             const CommonSampleData& sample,
                             bool has_process_context);
  void EmitSample(DataSourceInstanceID ds_id, CompletedSample sample);
  void EmitRingBufferLoss(DataSourceInstanceID ds_id,
                          size_t cpu,
                          uint64_t records_lost);

  void PostEmitSkippedSample(DataSourceInstanceID ds_id,
                             ParsedSample sample,
                             SampleSkipReason reason);
  // Emit a packet indicating that a sample was relevant, but skipped as it was
  // considered to be not unwindable (e.g. the process no longer exists).
  void EmitSkippedSample(DataSourceInstanceID ds_id,
                         ParsedSample sample,
                         SampleSkipReason reason);

  // Starts the shutdown of the given data source instance, starting with
  // pausing the reader frontend. Once the reader reaches the point where all
  // kernel buffers have been fully consumed, it will notify the |Unwinder| to
  // proceed with the shutdown sequence. The unwinder in turn will call back to
  // this producer once there are no more outstanding samples for the data
  // source at the unwinding stage.
  void InitiateReaderStop(DataSourceState* ds);
  // Destroys the state belonging to this instance, and acks the stop to the
  // tracing service.
  void FinishDataSourceStop(DataSourceInstanceID ds_id);
  // Immediately destroys the data source state, and instructs the unwinder to
  // do the same. This is used for abrupt stops.
  void PurgeDataSource(DataSourceInstanceID ds_id);

  // Immediately stops the data source if this daemon's overall memory footprint
  // is above the given threshold. This periodic task is started only for data
  // sources that specify a limit.
  void CheckMemoryFootprintPeriodic(DataSourceInstanceID ds_id,
                                    uint32_t max_daemon_memory_kb);

  // Chooses a random parameter for a callstack sampling option. Done at this
  // level as the choice is shared by all data sources within a tracing session.
  std::optional<ProcessSharding> GetOrChooseCallstackProcessShard(
      uint64_t tracing_session_id,
      uint32_t shard_count);

  void StartMetatraceSource(DataSourceInstanceID ds_id, BufferID target_buffer);

  // Task runner owned by the main thread.
  base::TaskRunner* const task_runner_;
  State state_ = kNotStarted;
  const char* producer_socket_name_ = nullptr;
  uint32_t connection_backoff_ms_ = 0;

  // Valid and stable for the lifetime of this class.
  ProcDescriptorGetter* const proc_fd_getter_;

  // Owns shared memory, must outlive trace writing.
  std::unique_ptr<TracingService::ProducerEndpoint> endpoint_;

  // If multiple metatrace sources are enabled concurrently,
  // only the first one becomes active.
  std::map<DataSourceInstanceID, MetatraceWriter> metatrace_writers_;

  // Interns callstacks across all data sources.
  // TODO(rsavitski): for long profiling sessions, consider purging trie when it
  // grows too large (at the moment purged only when no sources are active).
  // TODO(rsavitski): interning sequences are monotonic for the lifetime of the
  // daemon. Consider resetting them at safe points - possible when no sources
  // are active, and tricky otherwise. In the latter case, it'll require
  // emitting incremental sequence invalidation packets on all relevant
  // sequences.
  GlobalCallstackTrie callstack_trie_;

  // State associated with perf-sampling data sources.
  std::map<DataSourceInstanceID, DataSourceState> data_sources_;

  // Unwinding stage, running on a dedicated thread.
  UnwinderHandle unwinding_worker_;

  // Used for tracepoint name -> id lookups. Initialized lazily, and in general
  // best effort - can be null if tracefs isn't accessible.
  std::unique_ptr<FtraceProcfs> tracefs_;

  std::function<void()> all_data_sources_registered_cb_;

  base::WeakPtrFactory<PerfProducer> weak_factory_;  // keep last
};

}  // namespace profiling
}  // namespace perfetto

#endif  // SRC_PROFILING_PERF_PERF_PRODUCER_H_
