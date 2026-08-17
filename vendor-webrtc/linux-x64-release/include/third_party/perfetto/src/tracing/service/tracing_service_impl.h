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

#ifndef SRC_TRACING_SERVICE_TRACING_SERVICE_IMPL_H_
#define SRC_TRACING_SERVICE_TRACING_SERVICE_IMPL_H_

#include <algorithm>
#include <functional>
#include <map>
#include <memory>
#include <optional>
#include <set>
#include <utility>
#include <vector>

#include "perfetto/base/logging.h"
#include "perfetto/base/status.h"
#include "perfetto/base/time.h"
#include "perfetto/ext/base/circular_queue.h"
#include "perfetto/ext/base/clock_snapshots.h"
#include "perfetto/ext/base/periodic_task.h"
#include "perfetto/ext/base/uuid.h"
#include "perfetto/ext/base/weak_ptr.h"
#include "perfetto/ext/base/weak_runner.h"
#include "perfetto/ext/tracing/core/basic_types.h"
#include "perfetto/ext/tracing/core/client_identity.h"
#include "perfetto/ext/tracing/core/commit_data_request.h"
#include "perfetto/ext/tracing/core/observable_events.h"
#include "perfetto/ext/tracing/core/shared_memory_abi.h"
#include "perfetto/ext/tracing/core/trace_stats.h"
#include "perfetto/ext/tracing/core/tracing_service.h"
#include "perfetto/tracing/core/data_source_config.h"
#include "perfetto/tracing/core/data_source_descriptor.h"
#include "perfetto/tracing/core/forward_decls.h"
#include "perfetto/tracing/core/trace_config.h"
#include "src/android_stats/perfetto_atoms.h"
#include "src/tracing/core/id_allocator.h"
#include "src/tracing/service/clock.h"
#include "src/tracing/service/dependencies.h"
#include "src/tracing/service/random.h"

namespace protozero {
class MessageFilter;
}

namespace perfetto {

namespace protos {
namespace gen {
enum TraceStats_FinalFlushOutcome : int;
}
}  // namespace protos

class Consumer;
class Producer;
class SharedMemory;
class SharedMemoryArbiterImpl;
class TraceBuffer;
class TracePacket;

// The tracing service business logic.
class TracingServiceImpl : public TracingService {
 private:
  struct DataSourceInstance;
  struct TriggerInfo;

 public:
  static constexpr size_t kMaxShmSize = 32 * 1024 * 1024ul;
  static constexpr uint32_t kDataSourceStopTimeoutMs = 5000;
  static constexpr uint8_t kSyncMarker[] = {0x82, 0x47, 0x7a, 0x76, 0xb2, 0x8d,
                                            0x42, 0xba, 0x81, 0xdc, 0x33, 0x32,
                                            0x6d, 0x57, 0xa0, 0x79};
  static constexpr size_t kMaxTracePacketSliceSize =
      128 * 1024 - 512;  // This is ipc::kIPCBufferSize - 512, see assertion in
                         // tracing_integration_test.cc and b/195065199

  // This is a rough threshold to determine how many bytes to read from the
  // buffers on each iteration when writing into a file. Since filtering and
  // compression allocate memory, this effectively limits the amount of memory
  // allocated.
  static constexpr size_t kWriteIntoFileChunkSize = 1024 * 1024ul;

  // The implementation behind the service endpoint exposed to each producer.
  class ProducerEndpointImpl : public TracingService::ProducerEndpoint {
   public:
    ProducerEndpointImpl(ProducerID,
                         const ClientIdentity& client_identity,
                         TracingServiceImpl*,
                         base::TaskRunner*,
                         Producer*,
                         const std::string& producer_name,
                         const std::string& sdk_version,
                         bool in_process,
                         bool smb_scraping_enabled);
    ~ProducerEndpointImpl() override;

    // TracingService::ProducerEndpoint implementation.
    void Disconnect() override;
    void RegisterDataSource(const DataSourceDescriptor&) override;
    void UpdateDataSource(const DataSourceDescriptor&) override;
    void UnregisterDataSource(const std::string& name) override;
    void RegisterTraceWriter(uint32_t writer_id,
                             uint32_t target_buffer) override;
    void UnregisterTraceWriter(uint32_t writer_id) override;
    void CommitData(const CommitDataRequest&, CommitDataCallback) override;
    void SetupSharedMemory(std::unique_ptr<SharedMemory>,
                           size_t page_size_bytes,
                           bool provided_by_producer);
    std::unique_ptr<TraceWriter> CreateTraceWriter(
        BufferID,
        BufferExhaustedPolicy) override;
    SharedMemoryArbiter* MaybeSharedMemoryArbiter() override;
    bool IsShmemProvidedByProducer() const override;
    void NotifyFlushComplete(FlushRequestID) override;
    void NotifyDataSourceStarted(DataSourceInstanceID) override;
    void NotifyDataSourceStopped(DataSourceInstanceID) override;
    SharedMemory* shared_memory() const override;
    size_t shared_buffer_page_size_kb() const override;
    void ActivateTriggers(const std::vector<std::string>&) override;
    void Sync(std::function<void()> callback) override;

    void OnTracingSetup();
    void SetupDataSource(DataSourceInstanceID, const DataSourceConfig&);
    void StartDataSource(DataSourceInstanceID, const DataSourceConfig&);
    void StopDataSource(DataSourceInstanceID);
    void Flush(FlushRequestID,
               const std::vector<DataSourceInstanceID>&,
               FlushFlags);
    void OnFreeBuffers(const std::vector<BufferID>& target_buffers);
    void ClearIncrementalState(const std::vector<DataSourceInstanceID>&);

    bool is_allowed_target_buffer(BufferID buffer_id) const {
      return allowed_target_buffers_.count(buffer_id);
    }

    std::optional<BufferID> buffer_id_for_writer(WriterID writer_id) const {
      const auto it = writers_.find(writer_id);
      if (it != writers_.end())
        return it->second;
      return std::nullopt;
    }

    bool IsAndroidProcessFrozen();
    uid_t uid() const { return client_identity_.uid(); }
    pid_t pid() const { return client_identity_.pid(); }
    const ClientIdentity& client_identity() const { return client_identity_; }

   private:
    friend class TracingServiceImpl;
    ProducerEndpointImpl(const ProducerEndpointImpl&) = delete;
    ProducerEndpointImpl& operator=(const ProducerEndpointImpl&) = delete;

    ProducerID const id_;
    ClientIdentity const client_identity_;
    TracingServiceImpl* const service_;
    Producer* producer_;
    std::unique_ptr<SharedMemory> shared_memory_;
    size_t shared_buffer_page_size_kb_ = 0;
    SharedMemoryABI shmem_abi_;
    size_t shmem_size_hint_bytes_ = 0;
    size_t shmem_page_size_hint_bytes_ = 0;
    bool is_shmem_provided_by_producer_ = false;
    const std::string name_;
    std::string sdk_version_;
    bool in_process_;
    bool smb_scraping_enabled_;

    // Set of the global target_buffer IDs that the producer is configured to
    // write into in any active tracing session.
    std::set<BufferID> allowed_target_buffers_;

    // Maps registered TraceWriter IDs to their target buffers as registered by
    // the producer. Note that producers aren't required to register their
    // writers, so we may see commits of chunks with WriterIDs that aren't
    // contained in this map. However, if a producer does register a writer, the
    // service will prevent the writer from writing into any other buffer than
    // the one associated with it here. The BufferIDs stored in this map are
    // untrusted, so need to be verified against |allowed_target_buffers_|
    // before use.
    std::map<WriterID, BufferID> writers_;

    // This is used only in in-process configurations.
    // SharedMemoryArbiterImpl methods themselves are thread-safe.
    std::unique_ptr<SharedMemoryArbiterImpl> inproc_shmem_arbiter_;

    PERFETTO_THREAD_CHECKER(thread_checker_)
    base::WeakRunner weak_runner_;
  };

  // The implementation behind the service endpoint exposed to each consumer.
  class ConsumerEndpointImpl : public TracingService::ConsumerEndpoint {
   public:
    ConsumerEndpointImpl(TracingServiceImpl*,
                         base::TaskRunner*,
                         Consumer*,
                         uid_t uid);
    ~ConsumerEndpointImpl() override;

    void NotifyOnTracingDisabled(const std::string& error);

    // TracingService::ConsumerEndpoint implementation.
    void EnableTracing(const TraceConfig&, base::ScopedFile) override;
    void ChangeTraceConfig(const TraceConfig& cfg) override;
    void StartTracing() override;
    void DisableTracing() override;
    void ReadBuffers() override;
    void FreeBuffers() override;
    void Flush(uint32_t timeout_ms, FlushCallback, FlushFlags) override;
    void Detach(const std::string& key) override;
    void Attach(const std::string& key) override;
    void GetTraceStats() override;
    void ObserveEvents(uint32_t enabled_event_types) override;
    void QueryServiceState(QueryServiceStateArgs,
                           QueryServiceStateCallback) override;
    void QueryCapabilities(QueryCapabilitiesCallback) override;
    void SaveTraceForBugreport(SaveTraceForBugreportCallback) override;
    void CloneSession(CloneSessionArgs) override;

    // Will queue a task to notify the consumer about the state change.
    void OnDataSourceInstanceStateChange(const ProducerEndpointImpl&,
                                         const DataSourceInstance&);
    void OnAllDataSourcesStarted();

    base::WeakPtr<ConsumerEndpointImpl> GetWeakPtr() {
      return weak_ptr_factory_.GetWeakPtr();
    }

   private:
    friend class TracingServiceImpl;
    ConsumerEndpointImpl(const ConsumerEndpointImpl&) = delete;
    ConsumerEndpointImpl& operator=(const ConsumerEndpointImpl&) = delete;

    void NotifyCloneSnapshotTrigger(const TriggerInfo& trigger_name);

    // Returns a pointer to an ObservableEvents object that the caller can fill
    // and schedules a task to send the ObservableEvents to the consumer.
    ObservableEvents* AddObservableEvents();

    base::TaskRunner* const task_runner_;
    TracingServiceImpl* const service_;
    Consumer* const consumer_;
    uid_t const uid_;
    TracingSessionID tracing_session_id_ = 0;

    // Whether the consumer is interested in DataSourceInstance state change
    // events.
    uint32_t observable_events_mask_ = 0;

    // ObservableEvents that will be sent to the consumer. If set, a task to
    // flush the events to the consumer has been queued.
    std::unique_ptr<ObservableEvents> observable_events_;

    PERFETTO_THREAD_CHECKER(thread_checker_)
    base::WeakPtrFactory<ConsumerEndpointImpl> weak_ptr_factory_;  // Keep last.
  };

  class RelayEndpointImpl : public TracingService::RelayEndpoint {
   public:
    using SyncMode = RelayEndpoint::SyncMode;

    struct SyncedClockSnapshots {
      SyncedClockSnapshots(SyncMode _sync_mode,
                           base::ClockSnapshotVector _client_clocks,
                           base::ClockSnapshotVector _host_clocks)
          : sync_mode(_sync_mode),
            client_clocks(std::move(_client_clocks)),
            host_clocks(std::move(_host_clocks)) {}
      SyncMode sync_mode;
      base::ClockSnapshotVector client_clocks;
      base::ClockSnapshotVector host_clocks;
    };

    explicit RelayEndpointImpl(RelayClientID relay_client_id,
                               TracingServiceImpl* service);
    ~RelayEndpointImpl() override;

    void CacheSystemInfo(std::vector<uint8_t> serialized_system_info) override {
      serialized_system_info_ = serialized_system_info;
    }

    void SyncClocks(SyncMode sync_mode,
                    base::ClockSnapshotVector client_clocks,
                    base::ClockSnapshotVector host_clocks) override;
    void Disconnect() override;

    MachineID machine_id() const { return relay_client_id_.first; }

    base::CircularQueue<SyncedClockSnapshots>& synced_clocks() {
      return synced_clocks_;
    }

    std::vector<uint8_t>& serialized_system_info() {
      return serialized_system_info_;
    }

   private:
    RelayEndpointImpl(const RelayEndpointImpl&) = delete;
    RelayEndpointImpl& operator=(const RelayEndpointImpl&) = delete;

    RelayClientID relay_client_id_;
    TracingServiceImpl* const service_;
    std::vector<uint8_t> serialized_system_info_;
    base::CircularQueue<SyncedClockSnapshots> synced_clocks_;

    PERFETTO_THREAD_CHECKER(thread_checker_)
  };

  explicit TracingServiceImpl(std::unique_ptr<SharedMemory::Factory>,
                              base::TaskRunner*,
                              tracing_service::Dependencies,
                              InitOpts = {});
  ~TracingServiceImpl() override;

  // Called by ProducerEndpointImpl.
  void DisconnectProducer(ProducerID);
  void RegisterDataSource(ProducerID, const DataSourceDescriptor&);
  void UpdateDataSource(ProducerID, const DataSourceDescriptor&);
  void UnregisterDataSource(ProducerID, const std::string& name);
  void CopyProducerPageIntoLogBuffer(ProducerID,
                                     const ClientIdentity&,
                                     WriterID,
                                     ChunkID,
                                     BufferID,
                                     uint16_t num_fragments,
                                     uint8_t chunk_flags,
                                     bool chunk_complete,
                                     const uint8_t* src,
                                     size_t size);
  void ApplyChunkPatches(ProducerID,
                         const std::vector<CommitDataRequest::ChunkToPatch>&);
  void NotifyFlushDoneForProducer(ProducerID, FlushRequestID);
  void NotifyDataSourceStarted(ProducerID, DataSourceInstanceID);
  void NotifyDataSourceStopped(ProducerID, DataSourceInstanceID);
  void ActivateTriggers(ProducerID, const std::vector<std::string>& triggers);

  // Called by ConsumerEndpointImpl.
  bool DetachConsumer(ConsumerEndpointImpl*, const std::string& key);
  bool AttachConsumer(ConsumerEndpointImpl*, const std::string& key);
  void DisconnectConsumer(ConsumerEndpointImpl*);
  base::Status EnableTracing(ConsumerEndpointImpl*,
                             const TraceConfig&,
                             base::ScopedFile);
  void ChangeTraceConfig(ConsumerEndpointImpl*, const TraceConfig&);

  void StartTracing(TracingSessionID);
  void DisableTracing(TracingSessionID, bool disable_immediately = false);
  void Flush(TracingSessionID tsid,
             uint32_t timeout_ms,
             ConsumerEndpoint::FlushCallback,
             FlushFlags);
  void FlushAndDisableTracing(TracingSessionID);
  base::Status FlushAndCloneSession(ConsumerEndpointImpl*,
                                    ConsumerEndpoint::CloneSessionArgs);

  // Starts reading the internal tracing buffers from the tracing session `tsid`
  // and sends them to `*consumer` (which must be != nullptr).
  //
  // Only reads a limited amount of data in one call. If there's more data,
  // immediately schedules itself on a PostTask.
  //
  // Returns false in case of error.
  bool ReadBuffersIntoConsumer(TracingSessionID tsid,
                               ConsumerEndpointImpl* consumer);

  // Reads all the tracing buffers from the tracing session `tsid` and writes
  // them into the associated file.
  //
  // Reads all the data in the buffers (or until the file is full) before
  // returning.
  //
  // If the tracing session write_period_ms is 0, the file is full or there has
  // been an error, flushes the file and closes it. Otherwise, schedules itself
  // to be executed after write_period_ms.
  //
  // Returns false in case of error.
  bool ReadBuffersIntoFile(TracingSessionID);

  void FreeBuffers(TracingSessionID);

  // Service implementation.
  std::unique_ptr<TracingService::ProducerEndpoint> ConnectProducer(
      Producer*,
      const ClientIdentity& client_identity,
      const std::string& producer_name,
      size_t shared_memory_size_hint_bytes = 0,
      bool in_process = false,
      ProducerSMBScrapingMode smb_scraping_mode =
          ProducerSMBScrapingMode::kDefault,
      size_t shared_memory_page_size_hint_bytes = 0,
      std::unique_ptr<SharedMemory> shm = nullptr,
      const std::string& sdk_version = {}) override;

  std::unique_ptr<TracingService::ConsumerEndpoint> ConnectConsumer(
      Consumer*,
      uid_t) override;

  std::unique_ptr<TracingService::RelayEndpoint> ConnectRelayClient(
      RelayClientID) override;

  void DisconnectRelayClient(RelayClientID);

  // Set whether SMB scraping should be enabled by default or not. Producers can
  // override this setting for their own SMBs.
  void SetSMBScrapingEnabled(bool enabled) override {
    smb_scraping_enabled_ = enabled;
  }

  // Exposed mainly for testing.
  size_t num_producers() const { return producers_.size(); }
  ProducerEndpointImpl* GetProducer(ProducerID) const;

 private:
  struct TriggerHistory {
    int64_t timestamp_ns;
    uint64_t name_hash;

    bool operator<(const TriggerHistory& other) const {
      return timestamp_ns < other.timestamp_ns;
    }
  };

  struct RegisteredDataSource {
    ProducerID producer_id;
    DataSourceDescriptor descriptor;
  };

  // Represents an active data source for a tracing session.
  struct DataSourceInstance {
    DataSourceInstance(DataSourceInstanceID id,
                       const DataSourceConfig& cfg,
                       const std::string& ds_name,
                       bool notify_on_start,
                       bool notify_on_stop,
                       bool handles_incremental_state_invalidation,
                       bool no_flush_)
        : instance_id(id),
          config(cfg),
          data_source_name(ds_name),
          will_notify_on_start(notify_on_start),
          will_notify_on_stop(notify_on_stop),
          handles_incremental_state_clear(
              handles_incremental_state_invalidation),
          no_flush(no_flush_) {}
    DataSourceInstance(const DataSourceInstance&) = delete;
    DataSourceInstance& operator=(const DataSourceInstance&) = delete;

    DataSourceInstanceID instance_id;
    DataSourceConfig config;
    std::string data_source_name;
    bool will_notify_on_start;
    bool will_notify_on_stop;
    bool handles_incremental_state_clear;
    bool no_flush;

    enum DataSourceInstanceState {
      CONFIGURED,
      STARTING,
      STARTED,
      STOPPING,
      STOPPED
    };
    DataSourceInstanceState state = CONFIGURED;
  };

  struct PendingFlush {
    std::set<ProducerID> producers;
    ConsumerEndpoint::FlushCallback callback;
    explicit PendingFlush(decltype(callback) cb) : callback(std::move(cb)) {}
  };

  using PendingCloneID = uint64_t;

  struct TriggerInfo {
    uint64_t boot_time_ns = 0;
    std::string trigger_name;
    std::string producer_name;
    uid_t producer_uid = 0;
    uint64_t trigger_delay_ms = 0;
  };

  struct PendingClone {
    size_t pending_flush_cnt = 0;
    // This vector might not be populated all at once. Some buffers might be
    // nullptr while flushing is not done.
    std::vector<std::unique_ptr<TraceBuffer>> buffers;
    std::vector<int64_t> buffer_cloned_timestamps;
    bool flush_failed = false;
    base::WeakPtr<ConsumerEndpointImpl> weak_consumer;
    bool skip_trace_filter = false;
    std::optional<TriggerInfo> clone_trigger;
    int64_t clone_started_timestamp_ns = 0;
  };

  // Holds the state of a tracing session. A tracing session is uniquely bound
  // a specific Consumer. Each Consumer can own one or more sessions.
  struct TracingSession {
    enum State {
      DISABLED = 0,
      CONFIGURED,
      STARTED,
      DISABLING_WAITING_STOP_ACKS,
      CLONED_READ_ONLY,
    };

    TracingSession(TracingSessionID,
                   ConsumerEndpointImpl*,
                   const TraceConfig&,
                   base::TaskRunner*);
    TracingSession(TracingSession&&) = delete;
    TracingSession& operator=(TracingSession&&) = delete;

    size_t num_buffers() const { return buffers_index.size(); }

    uint32_t flush_timeout_ms() {
      uint32_t timeout_ms = config.flush_timeout_ms();
      return timeout_ms ? timeout_ms : kDefaultFlushTimeoutMs;
    }

    uint32_t data_source_stop_timeout_ms() {
      uint32_t timeout_ms = config.data_source_stop_timeout_ms();
      return timeout_ms ? timeout_ms : kDataSourceStopTimeoutMs;
    }

    PacketSequenceID GetPacketSequenceID(MachineID machine_id,
                                         ProducerID producer_id,
                                         WriterID writer_id) {
      auto key = std::make_tuple(machine_id, producer_id, writer_id);
      auto it = packet_sequence_ids.find(key);
      if (it != packet_sequence_ids.end())
        return it->second;
      // We shouldn't run out of sequence IDs (producer ID is 16 bit, writer IDs
      // are limited to 1024).
      static_assert(kMaxPacketSequenceID > kMaxProducerID * kMaxWriterID,
                    "PacketSequenceID value space doesn't cover service "
                    "sequence ID and all producer/writer ID combinations!");
      PERFETTO_DCHECK(last_packet_sequence_id < kMaxPacketSequenceID);
      PacketSequenceID sequence_id = ++last_packet_sequence_id;
      packet_sequence_ids[key] = sequence_id;
      return sequence_id;
    }

    DataSourceInstance* GetDataSourceInstance(
        ProducerID producer_id,
        DataSourceInstanceID instance_id) {
      for (auto& inst_kv : data_source_instances) {
        if (inst_kv.first != producer_id ||
            inst_kv.second.instance_id != instance_id) {
          continue;
        }
        return &inst_kv.second;
      }
      return nullptr;
    }

    bool AllDataSourceInstancesStarted() {
      return std::all_of(
          data_source_instances.begin(), data_source_instances.end(),
          [](decltype(data_source_instances)::const_reference x) {
            return x.second.state == DataSourceInstance::STARTED;
          });
    }

    bool AllDataSourceInstancesStopped() {
      return std::all_of(
          data_source_instances.begin(), data_source_instances.end(),
          [](decltype(data_source_instances)::const_reference x) {
            return x.second.state == DataSourceInstance::STOPPED;
          });
    }

    // Checks whether |clone_uid| is allowed to clone the current tracing
    // session.
    bool IsCloneAllowed(uid_t clone_uid) const;

    const TracingSessionID id;

    // The consumer that started the session.
    // Can be nullptr if the consumer detached from the session.
    ConsumerEndpointImpl* consumer_maybe_null;

    // Unix uid of the consumer. This is valid even after the consumer detaches
    // and does not change for the entire duration of the session. It is used to
    // prevent that a consumer re-attaches to a session from a different uid.
    uid_t const consumer_uid;

    // The list of triggers this session received while alive and the time they
    // were received at. This is used to insert 'fake' packets back to the
    // consumer so they can tell when some event happened. The order matches the
    // order they were received.
    std::vector<TriggerInfo> received_triggers;

    // The trace config provided by the Consumer when calling
    // EnableTracing(), plus any updates performed by ChangeTraceConfig.
    TraceConfig config;

    // List of data source instances that have been enabled on the various
    // producers for this tracing session.
    std::multimap<ProducerID, DataSourceInstance> data_source_instances;

    // For each Flush(N) request, keeps track of the set of producers for which
    // we are still awaiting a NotifyFlushComplete(N) ack.
    std::map<FlushRequestID, PendingFlush> pending_flushes;

    // For each Clone request, keeps track of the flushes acknowledgement that
    // we are still waiting for.
    std::map<PendingCloneID, PendingClone> pending_clones;

    PendingCloneID last_pending_clone_id_ = 0;

    // Maps a per-trace-session buffer index into the corresponding global
    // BufferID (shared namespace amongst all consumers). This vector has as
    // many entries as |config.buffers_size()|.
    std::vector<BufferID> buffers_index;

    std::map<std::tuple<MachineID, ProducerID, WriterID>, PacketSequenceID>
        packet_sequence_ids;
    PacketSequenceID last_packet_sequence_id = kServicePacketSequenceID;

    // Whether we should emit the trace stats next time we reach EOF while
    // performing ReadBuffers.
    bool should_emit_stats = false;

    // Whether we should emit the sync marker the next time ReadBuffers() is
    // called.
    bool should_emit_sync_marker = false;

    // Whether we put the initial packets (trace config, system info,
    // etc.) into the trace output yet.
    bool did_emit_initial_packets = false;

    // Whether we emitted clock offsets for relay clients yet.
    bool did_emit_remote_clock_sync_ = false;

    // Whether we should compress TracePackets after reading them.
    bool compress_deflate = false;

    // The number of received triggers we've emitted into the trace output.
    size_t num_triggers_emitted_into_trace = 0;

    // Packets that failed validation of the TrustedPacket.
    uint64_t invalid_packets = 0;

    // Flush() stats. See comments in trace_stats.proto for more.
    uint64_t flushes_requested = 0;
    uint64_t flushes_succeeded = 0;
    uint64_t flushes_failed = 0;

    // Outcome of the final Flush() done by FlushAndDisableTracing().
    protos::gen::TraceStats_FinalFlushOutcome final_flush_outcome{};

    // Set to true on the first call to MaybeNotifyAllDataSourcesStarted().
    bool did_notify_all_data_source_started = false;

    // Stores simple lifecycle events of a particular type (i.e. associated with
    // a single field id in the TracingServiceEvent proto).
    struct LifecycleEvent {
      LifecycleEvent(uint32_t f_id, uint32_t m_size = 1)
          : field_id(f_id), max_size(m_size), timestamps(m_size) {}

      // The field id of the event in the TracingServiceEvent proto.
      uint32_t field_id;

      // Stores the max size of |timestamps|. Set to 1 by default (in
      // the constructor) but can be overridden in TraceSession constructor
      // if a larger size is required.
      uint32_t max_size;

      // Stores the timestamps emitted for each event type (in nanoseconds).
      // Emitted into the trace and cleared when the consumer next calls
      // ReadBuffers.
      base::CircularQueue<int64_t> timestamps;
    };
    std::vector<LifecycleEvent> lifecycle_events;

    // Stores arbitrary lifecycle events that don't fit in lifecycle_events as
    // serialized TracePacket protos.
    struct ArbitraryLifecycleEvent {
      int64_t timestamp;
      std::vector<uint8_t> data;
    };

    std::optional<ArbitraryLifecycleEvent> slow_start_event;

    std::vector<ArbitraryLifecycleEvent> last_flush_events;

    // If this is a cloned tracing session, the timestamp at which each buffer
    // was cloned.
    std::vector<int64_t> buffer_cloned_timestamps;

    using ClockSnapshotData = base::ClockSnapshotVector;

    // Initial clock snapshot, captured at trace start time (when state goes to
    // TracingSession::STARTED). Emitted into the trace when the consumer first
    // calls ReadBuffers().
    ClockSnapshotData initial_clock_snapshot;

    // Stores clock snapshots to emit into the trace as a ring buffer. This
    // buffer is populated both periodically and when lifecycle events happen
    // but only when significant clock drift is detected. Emitted into the trace
    // and cleared when the consumer next calls ReadBuffers().
    base::CircularQueue<ClockSnapshotData> clock_snapshot_ring_buffer;

    State state = DISABLED;

    // If the consumer detached the session, this variable defines the key used
    // for identifying the session later when reattaching.
    std::string detach_key;

    // This is set when the Consumer calls sets |write_into_file| == true in the
    // TraceConfig. In this case this represents the file we should stream the
    // trace packets into, rather than returning it to the consumer via
    // OnTraceData().
    base::ScopedFile write_into_file;
    uint32_t write_period_ms = 0;
    uint64_t max_file_size_bytes = 0;
    uint64_t bytes_written_into_file = 0;

    // Periodic task for snapshotting service events (e.g. clocks, sync markers
    // etc)
    base::PeriodicTask snapshot_periodic_task;

    // Deferred task that stops the trace when |duration_ms| expires. This is
    // to handle the case of |prefer_suspend_clock_for_duration| which cannot
    // use PostDelayedTask.
    base::PeriodicTask timed_stop_task;

    // When non-NULL the packets should be post-processed using the filter.
    std::unique_ptr<protozero::MessageFilter> trace_filter;
    uint64_t filter_input_packets = 0;
    uint64_t filter_input_bytes = 0;
    uint64_t filter_output_bytes = 0;
    uint64_t filter_errors = 0;
    uint64_t filter_time_taken_ns = 0;
    std::vector<uint64_t> filter_bytes_discarded_per_buffer;

    // A randomly generated trace identifier. Note that this does NOT always
    // match the requested TraceConfig.trace_uuid_msb/lsb. Specifically, it does
    // until a gap-less snapshot is requested. Each snapshot re-generates the
    // uuid to avoid emitting two different traces with the same uuid.
    base::Uuid trace_uuid;

    // This is set when the clone operation was caused by a clone trigger.
    std::optional<TriggerInfo> clone_trigger;

    // NOTE: when adding new fields here consider whether that state should be
    // copied over in DoCloneSession() or not. Ask yourself: is this a
    // "runtime state" (e.g. active data sources) or a "trace (meta)data state"?
    // If the latter, it should be handled by DoCloneSession()).
  };

  TracingServiceImpl(const TracingServiceImpl&) = delete;
  TracingServiceImpl& operator=(const TracingServiceImpl&) = delete;

  bool IsInitiatorPrivileged(const TracingSession&);

  DataSourceInstance* SetupDataSource(const TraceConfig::DataSource&,
                                      const TraceConfig::ProducerConfig&,
                                      const RegisteredDataSource&,
                                      TracingSession*);

  // Returns the next available ProducerID that is not in |producers_|.
  ProducerID GetNextProducerID();

  // Returns a pointer to the |tracing_sessions_| entry or nullptr if the
  // session doesn't exists.
  TracingSession* GetTracingSession(TracingSessionID);

  // Returns a pointer to the |tracing_sessions_| entry with
  // |unique_session_name| in the config (or nullptr if the
  // session doesn't exists). CLONED_READ_ONLY sessions are ignored.
  TracingSession* GetTracingSessionByUniqueName(
      const std::string& unique_session_name);

  // Returns a pointer to the tracing session that has the highest
  // TraceConfig.bugreport_score, if any, or nullptr.
  TracingSession* FindTracingSessionWithMaxBugreportScore();

  // Returns a pointer to the |tracing_sessions_| entry, matching the given
  // uid and detach key, or nullptr if no such session exists.
  TracingSession* GetDetachedSession(uid_t, const std::string& key);

  // Update the memory guard rail by using the latest information from the
  // shared memory and trace buffers.
  void UpdateMemoryGuardrail();

  uint32_t DelayToNextWritePeriodMs(const TracingSession&);
  void StartDataSourceInstance(ProducerEndpointImpl*,
                               TracingSession*,
                               DataSourceInstance*);
  void StopDataSourceInstance(ProducerEndpointImpl*,
                              TracingSession*,
                              DataSourceInstance*,
                              bool disable_immediately);
  void PeriodicSnapshotTask(TracingSessionID);
  void MaybeSnapshotClocksIntoRingBuffer(TracingSession*);
  bool SnapshotClocks(TracingSession::ClockSnapshotData*);
  // Records a lifecycle event of type |field_id| with the current timestamp.
  void SnapshotLifecycleEvent(TracingSession*,
                              uint32_t field_id,
                              bool snapshot_clocks);
  // Deletes all the lifecycle events of type |field_id| and records just one,
  // that happened at time |boot_time_ns|.
  void SetSingleLifecycleEvent(TracingSession*,
                               uint32_t field_id,
                               int64_t boot_time_ns);
  void EmitClockSnapshot(TracingSession*,
                         TracingSession::ClockSnapshotData,
                         std::vector<TracePacket>*);
  void EmitSyncMarker(std::vector<TracePacket>*);
  void EmitStats(TracingSession*, std::vector<TracePacket>*);
  TraceStats GetTraceStats(TracingSession*);
  void EmitLifecycleEvents(TracingSession*, std::vector<TracePacket>*);
  void EmitUuid(TracingSession*, std::vector<TracePacket>*);
  void MaybeEmitTraceConfig(TracingSession*, std::vector<TracePacket>*);
  void EmitSystemInfo(std::vector<TracePacket>*);
  void MaybeEmitRemoteSystemInfo(std::vector<TracePacket>*);
  void MaybeEmitCloneTrigger(TracingSession*, std::vector<TracePacket>*);
  void MaybeEmitReceivedTriggers(TracingSession*, std::vector<TracePacket>*);
  void MaybeEmitRemoteClockSync(TracingSession*, std::vector<TracePacket>*);
  void MaybeNotifyAllDataSourcesStarted(TracingSession*);
  void OnFlushTimeout(TracingSessionID, FlushRequestID, FlushFlags);
  void OnDisableTracingTimeout(TracingSessionID);
  void OnAllDataSourceStartedTimeout(TracingSessionID);
  void DisableTracingNotifyConsumerAndFlushFile(TracingSession*);
  void PeriodicFlushTask(TracingSessionID, bool post_next_only);
  void CompleteFlush(TracingSessionID tsid,
                     ConsumerEndpoint::FlushCallback callback,
                     bool success);
  void ScrapeSharedMemoryBuffers(TracingSession*, ProducerEndpointImpl*);
  void PeriodicClearIncrementalStateTask(TracingSessionID, bool post_next_only);
  TraceBuffer* GetBufferByID(BufferID);
  void FlushDataSourceInstances(
      TracingSession*,
      uint32_t timeout_ms,
      const std::map<ProducerID, std::vector<DataSourceInstanceID>>&,
      ConsumerEndpoint::FlushCallback,
      FlushFlags);
  std::map<ProducerID, std::vector<DataSourceInstanceID>>
  GetFlushableDataSourceInstancesForBuffers(TracingSession*,
                                            const std::set<BufferID>&);
  bool DoCloneBuffers(const TracingSession&,
                      const std::set<BufferID>&,
                      PendingClone*);
  base::Status FinishCloneSession(ConsumerEndpointImpl*,
                                  TracingSessionID,
                                  std::vector<std::unique_ptr<TraceBuffer>>,
                                  std::vector<int64_t> buf_cloned_timestamps,
                                  bool skip_filter,
                                  bool final_flush_outcome,
                                  std::optional<TriggerInfo> clone_trigger,
                                  base::Uuid*,
                                  int64_t clone_started_timestamp_ns);
  void OnFlushDoneForClone(TracingSessionID src_tsid,
                           PendingCloneID clone_id,
                           const std::set<BufferID>& buf_ids,
                           bool final_flush_outcome);

  // Returns true if `*tracing_session` is waiting for a trigger that hasn't
  // happened.
  static bool IsWaitingForTrigger(TracingSession* tracing_session);

  // Reads the buffers from `*tracing_session` and returns them (along with some
  // metadata packets).
  //
  // The function stops when the cumulative size of the return packets exceeds
  // `threshold` (so it's not a strict upper bound) and sets `*has_more` to
  // true, or when there are no more packets (and sets `*has_more` to false).
  std::vector<TracePacket> ReadBuffers(TracingSession* tracing_session,
                                       size_t threshold,
                                       bool* has_more);

  // If `*tracing_session` has a filter, applies it to `*packets`. Doesn't
  // change the number of `*packets`, only their content.
  void MaybeFilterPackets(TracingSession* tracing_session,
                          std::vector<TracePacket>* packets);

  // If `*tracing_session` has compression enabled, compress `*packets`.
  void MaybeCompressPackets(TracingSession* tracing_session,
                            std::vector<TracePacket>* packets);

  // If `*tracing_session` is configured to write into a file, writes `packets`
  // into the file.
  //
  // Returns true if the file should be closed (because it's full or there has
  // been an error), false otherwise.
  bool WriteIntoFile(TracingSession* tracing_session,
                     std::vector<TracePacket> packets);
  void OnStartTriggersTimeout(TracingSessionID tsid);
  void MaybeLogUploadEvent(const TraceConfig&,
                           const base::Uuid&,
                           PerfettoStatsdAtom atom,
                           const std::string& trigger_name = "");
  void MaybeLogTriggerEvent(const TraceConfig&,
                            PerfettoTriggerAtom atom,
                            const std::string& trigger_name);
  size_t PurgeExpiredAndCountTriggerInWindow(int64_t now_ns,
                                             uint64_t trigger_name_hash);
  void StopOnDurationMsExpiry(TracingSessionID);

  std::unique_ptr<tracing_service::Clock> clock_;
  std::unique_ptr<tracing_service::Random> random_;
  const InitOpts init_opts_;
  std::unique_ptr<SharedMemory::Factory> shm_factory_;
  ProducerID last_producer_id_ = 0;
  DataSourceInstanceID last_data_source_instance_id_ = 0;
  TracingSessionID last_tracing_session_id_ = 0;
  FlushRequestID last_flush_request_id_ = 0;
  uid_t uid_ = 0;

  // Buffer IDs are global across all consumers (because a Producer can produce
  // data for more than one trace session, hence more than one consumer).
  IdAllocator<BufferID> buffer_ids_;

  std::multimap<std::string /*name*/, RegisteredDataSource> data_sources_;
  std::map<ProducerID, ProducerEndpointImpl*> producers_;
  std::map<RelayClientID, RelayEndpointImpl*> relay_clients_;
  std::map<TracingSessionID, TracingSession> tracing_sessions_;
  std::map<BufferID, std::unique_ptr<TraceBuffer>> buffers_;
  std::map<std::string, int64_t> session_to_last_trace_s_;

  // Contains timestamps of triggers.
  // The queue is sorted by timestamp and invocations older than 24 hours are
  // purged when a trigger happens.
  base::CircularQueue<TriggerHistory> trigger_history_;

  bool smb_scraping_enabled_ = false;
  bool lockdown_mode_ = false;

  uint8_t sync_marker_packet_[32];  // Lazily initialized.
  size_t sync_marker_packet_size_ = 0;

  // Stats.
  uint64_t chunks_discarded_ = 0;
  uint64_t patches_discarded_ = 0;

  PERFETTO_THREAD_CHECKER(thread_checker_)

  base::WeakRunner weak_runner_;
};

}  // namespace perfetto

#endif  // SRC_TRACING_SERVICE_TRACING_SERVICE_IMPL_H_
