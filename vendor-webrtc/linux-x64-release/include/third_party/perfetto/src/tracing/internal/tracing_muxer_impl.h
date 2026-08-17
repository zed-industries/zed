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

#ifndef SRC_TRACING_INTERNAL_TRACING_MUXER_IMPL_H_
#define SRC_TRACING_INTERNAL_TRACING_MUXER_IMPL_H_

#include <stddef.h>
#include <stdint.h>

#include <array>
#include <atomic>
#include <bitset>
#include <functional>
#include <list>
#include <map>
#include <memory>
#include <set>
#include <utility>
#include <vector>

#include "perfetto/base/time.h"
#include "perfetto/ext/base/scoped_file.h"
#include "perfetto/ext/base/thread_checker.h"
#include "perfetto/ext/tracing/core/basic_types.h"
#include "perfetto/ext/tracing/core/consumer.h"
#include "perfetto/ext/tracing/core/producer.h"
#include "perfetto/ext/tracing/core/tracing_service.h"
#include "perfetto/tracing/backend_type.h"
#include "perfetto/tracing/core/data_source_descriptor.h"
#include "perfetto/tracing/core/forward_decls.h"
#include "perfetto/tracing/core/trace_config.h"
#include "perfetto/tracing/internal/basic_types.h"
#include "perfetto/tracing/internal/tracing_muxer.h"
#include "perfetto/tracing/tracing.h"

#include "protos/perfetto/common/interceptor_descriptor.gen.h"

namespace perfetto {

class ConsumerEndpoint;
class DataSourceBase;
class ProducerEndpoint;
class TraceWriterBase;
class TracingBackend;
class TracingSession;
struct TracingInitArgs;

namespace base {
class TaskRunner;
}

namespace shlib {
void ResetForTesting();
}

namespace test {
class TracingMuxerImplInternalsForTest;
}

namespace internal {

struct DataSourceStaticState;

// This class acts as a bridge between the public API and the TracingBackend(s).
// It exposes a simplified view of the world to the API methods handling all the
// bookkeeping to map data source instances and trace writers to the various
// backends. It deals with N data sources, M backends (1 backend == 1 tracing
// service == 1 producer connection) and T concurrent tracing sessions.
//
// Handing data source registration and start/stop flows [producer side]:
// ----------------------------------------------------------------------
// 1. The API client subclasses perfetto::DataSource and calls
//    DataSource::Register<MyDataSource>(). In turn this calls into the
//    TracingMuxer.
// 2. The tracing muxer iterates through all the backends (1 backend == 1
//    service == 1 producer connection) and registers the data source on each
//    backend.
// 3. When any (services behind a) backend starts tracing and requests to start
//    that specific data source, the TracingMuxerImpl constructs a new instance
//    of MyDataSource and calls the OnStart() method.
//
// Controlling trace and retrieving trace data [consumer side]:
// ------------------------------------------------------------
// 1. The API client calls Tracing::NewTrace(), returns a RAII TracingSession
//    object.
// 2. NewTrace() calls into internal::TracingMuxer(Impl). TracingMuxer
//    subclasses the TracingSession object (TracingSessionImpl) and returns it.
// 3. The tracing muxer identifies the backend (according to the args passed to
//    NewTrace), creates a new Consumer and connects to it.
// 4. When the API client calls Start()/Stop()/ReadTrace() methods, the
//    TracingMuxer forwards them to the consumer associated to the
//    TracingSession. Likewise for callbacks coming from the consumer-side of
//    the service.
class TracingMuxerImpl : public TracingMuxer {
 public:
  // This is different than TracingSessionID because it's global across all
  // backends. TracingSessionID is global only within the scope of one service.
  using TracingSessionGlobalID = uint64_t;

  struct RegisteredDataSource {
    DataSourceDescriptor descriptor;
    DataSourceFactory factory{};
    bool supports_multiple_instances = false;
    bool requires_callbacks_under_lock = false;
    bool no_flush = false;
    DataSourceStaticState* static_state = nullptr;
  };

  static void InitializeInstance(const TracingInitArgs&);
  static void ResetForTesting();
  static void Shutdown();

  // TracingMuxer implementation.
  bool RegisterDataSource(const DataSourceDescriptor&,
                          DataSourceFactory,
                          DataSourceParams,
                          bool no_flush,
                          DataSourceStaticState*) override;
  void UpdateDataSourceDescriptor(const DataSourceDescriptor&,
                                  const DataSourceStaticState*) override;
  std::unique_ptr<TraceWriterBase> CreateTraceWriter(
      DataSourceStaticState*,
      uint32_t data_source_instance_index,
      DataSourceState*,
      BufferExhaustedPolicy buffer_exhausted_policy) override;
  void DestroyStoppedTraceWritersForCurrentThread() override;
  void RegisterInterceptor(const InterceptorDescriptor&,
                           InterceptorFactory,
                           InterceptorBase::TLSFactory,
                           InterceptorBase::TracePacketCallback) override;

  void ActivateTriggers(const std::vector<std::string>&, uint32_t) override;

  std::unique_ptr<TracingSession> CreateTracingSession(
      BackendType,
      TracingConsumerBackend* (*system_backend_factory)());
  std::unique_ptr<StartupTracingSession> CreateStartupTracingSession(
      const TraceConfig& config,
      Tracing::SetupStartupTracingOpts);
  std::unique_ptr<StartupTracingSession> CreateStartupTracingSessionBlocking(
      const TraceConfig& config,
      Tracing::SetupStartupTracingOpts);

  // Producer-side bookkeeping methods.
  void UpdateDataSourcesOnAllBackends();
  void SetupDataSource(TracingBackendId,
                       uint32_t backend_connection_id,
                       DataSourceInstanceID,
                       const DataSourceConfig&);
  void StartDataSource(TracingBackendId, DataSourceInstanceID);
  void StopDataSource_AsyncBegin(TracingBackendId, DataSourceInstanceID);
  void ClearDataSourceIncrementalState(TracingBackendId, DataSourceInstanceID);
  void SyncProducersForTesting();

  // Consumer-side bookkeeping methods.
  void SetupTracingSession(TracingSessionGlobalID,
                           const std::shared_ptr<TraceConfig>&,
                           base::ScopedFile trace_fd = base::ScopedFile());
  void StartTracingSession(TracingSessionGlobalID);
  void CloneTracingSession(TracingSessionGlobalID,
                           TracingSession::CloneTraceArgs,
                           TracingSession::CloneTraceCallback);
  void ChangeTracingSessionConfig(TracingSessionGlobalID, const TraceConfig&);
  void StopTracingSession(TracingSessionGlobalID);
  void DestroyTracingSession(TracingSessionGlobalID);
  void FlushTracingSession(TracingSessionGlobalID,
                           uint32_t,
                           std::function<void(bool)>);
  void ReadTracingSessionData(
      TracingSessionGlobalID,
      std::function<void(TracingSession::ReadTraceCallbackArgs)>);
  void GetTraceStats(TracingSessionGlobalID,
                     TracingSession::GetTraceStatsCallback);
  void QueryServiceState(TracingSessionGlobalID,
                         TracingSession::QueryServiceStateCallback);

  // Sets the batching period to |batch_commits_duration_ms| on the backends
  // with type |backend_type|.
  void SetBatchCommitsDurationForTesting(uint32_t batch_commits_duration_ms,
                                         BackendType backend_type);

  // Enables direct SMB patching on the backends with type |backend_type| (see
  // SharedMemoryArbiter::EnableDirectSMBPatching). Returns true if the
  // operation succeeded for all backends with type |backend_type|, false
  // otherwise.
  bool EnableDirectSMBPatchingForTesting(BackendType backend_type);

  void SetMaxProducerReconnectionsForTesting(uint32_t count);

 private:
  friend class test::TracingMuxerImplInternalsForTest;
  friend void shlib::ResetForTesting();

  // For each TracingBackend we create and register one ProducerImpl instance.
  // This talks to the producer-side of the service, gets start/stop requests
  // from it and routes them to the registered data sources.
  // One ProducerImpl == one backend == one tracing service.
  // This class is needed to disambiguate callbacks coming from different
  // services. TracingMuxerImpl can't directly implement the Producer interface
  // because the Producer virtual methods don't allow to identify the service.
  class ProducerImpl : public Producer {
   public:
    ProducerImpl(TracingMuxerImpl*,
                 TracingBackendId,
                 uint32_t shmem_batch_commits_duration_ms,
                 bool shmem_direct_patching_enabled);
    ~ProducerImpl() override;

    void Initialize(std::unique_ptr<ProducerEndpoint> endpoint);
    void RegisterDataSource(const DataSourceDescriptor&,
                            DataSourceFactory,
                            DataSourceStaticState*);
    void DisposeConnection();

    // perfetto::Producer implementation.
    void OnConnect() override;
    void OnDisconnect() override;
    void OnTracingSetup() override;
    void OnStartupTracingSetup() override;
    void SetupDataSource(DataSourceInstanceID,
                         const DataSourceConfig&) override;
    void StartDataSource(DataSourceInstanceID,
                         const DataSourceConfig&) override;
    void StopDataSource(DataSourceInstanceID) override;
    void Flush(FlushRequestID,
               const DataSourceInstanceID*,
               size_t,
               FlushFlags) override;
    void ClearIncrementalState(const DataSourceInstanceID*, size_t) override;

    bool SweepDeadServices();
    void SendOnConnectTriggers();
    void NotifyFlushForDataSourceDone(DataSourceInstanceID, FlushRequestID);

    PERFETTO_THREAD_CHECKER(thread_checker_)
    TracingMuxerImpl* muxer_;
    TracingBackendId const backend_id_;
    bool connected_ = false;
    bool did_setup_tracing_ = false;
    bool did_setup_startup_tracing_ = false;
    std::atomic<uint32_t> connection_id_{0};
    uint16_t last_startup_target_buffer_reservation_ = 0;
    bool is_producer_provided_smb_ = false;
    bool producer_provided_smb_failed_ = false;

    const uint32_t shmem_batch_commits_duration_ms_ = 0;
    const bool shmem_direct_patching_enabled_ = false;

    // Set of data sources that have been actually registered on this producer.
    // This can be a subset of the global |data_sources_|, because data sources
    // can register before the producer is fully connected.
    std::bitset<kMaxDataSources> registered_data_sources_{};

    // A collection of disconnected service endpoints. Since trace writers on
    // arbitrary threads might continue writing data to disconnected services,
    // we keep the old services around and periodically try to clean up ones
    // that no longer have any writers (see SweepDeadServices).
    std::list<std::shared_ptr<ProducerEndpoint>> dead_services_;

    // Triggers that should be sent when the service connects (trigger_name,
    // expiration).
    std::list<std::pair<std::string, base::TimeMillis>> on_connect_triggers_;

    std::map<FlushRequestID, std::set<DataSourceInstanceID>> pending_flushes_;

    // The currently active service endpoint is maintained as an atomic shared
    // pointer so it won't get deleted from underneath threads that are creating
    // trace writers. At any given time one endpoint can be shared (and thus
    // kept alive) by the |service_| pointer, an entry in |dead_services_| and
    // as a pointer on the stack in CreateTraceWriter() (on an arbitrary
    // thread). The endpoint is never shared outside ProducerImpl itself.
    //
    // WARNING: Any *write* access to this variable or any *read* access from a
    // non-muxer thread must be done through std::atomic_{load,store} to avoid
    // data races.
    std::shared_ptr<ProducerEndpoint> service_;  // Keep last.
  };

  // For each TracingSession created by the API client (Tracing::NewTrace() we
  // create and register one ConsumerImpl instance.
  // This talks to the consumer-side of the service, gets end-of-trace and
  // on-trace-data callbacks and routes them to the API client callbacks.
  // This class is needed to disambiguate callbacks coming from different
  // tracing sessions.
  class ConsumerImpl : public Consumer {
   public:
    ConsumerImpl(TracingMuxerImpl*, BackendType, TracingSessionGlobalID);
    ~ConsumerImpl() override;

    void Initialize(std::unique_ptr<ConsumerEndpoint> endpoint);

    // perfetto::Consumer implementation.
    void OnConnect() override;
    void OnDisconnect() override;
    void OnTracingDisabled(const std::string& error) override;
    void OnTraceData(std::vector<TracePacket>, bool has_more) override;
    void OnDetach(bool success) override;
    void OnAttach(bool success, const TraceConfig&) override;
    void OnTraceStats(bool success, const TraceStats&) override;
    void OnObservableEvents(const ObservableEvents&) override;
    void OnSessionCloned(const OnSessionClonedArgs&) override;

    void NotifyStartComplete();
    void NotifyError(const TracingError&);
    void NotifyStopComplete();

    // Will eventually inform the |muxer_| when it is safe to remove |this|.
    void Disconnect();

    TracingMuxerImpl* muxer_;
    BackendType const backend_type_;
    TracingSessionGlobalID const session_id_;
    bool connected_ = false;

    // This is to handle the case where the Setup call from the API client
    // arrives before the consumer has connected. In this case we keep around
    // the config and check if we have it after connection.
    bool start_pending_ = false;

    // Similarly if the session is stopped before the consumer was connected, we
    // need to wait until the session has started before stopping it.
    bool stop_pending_ = false;

    // Similarly we need to buffer a call to get trace statistics if the
    // consumer wasn't connected yet.
    bool get_trace_stats_pending_ = false;

    // Similarly we need to buffer a session cloning args if the session is
    // cloning another sesison before the consumer was connected.
    std::optional<ConsumerEndpoint::CloneSessionArgs> session_to_clone_;

    // Whether this session was already stopped. This will happen in response to
    // Stop{,Blocking}, but also if the service stops the session for us
    // automatically (e.g., when there are no data sources).
    bool stopped_ = false;

    // shared_ptr because it's posted across threads. This is to avoid copying
    // it more than once.
    std::shared_ptr<TraceConfig> trace_config_;
    base::ScopedFile trace_fd_;

    // If the API client passes a callback to start, we should invoke this when
    // NotifyStartComplete() is invoked.
    std::function<void()> start_complete_callback_;

    // An internal callback used to implement StartBlocking().
    std::function<void()> blocking_start_complete_callback_;

    // If the API client passes a callback to get notification about the
    // errors, we should invoke this when NotifyError() is invoked.
    std::function<void(TracingError)> error_callback_;

    // If the API client passes a callback to stop, we should invoke this when
    // OnTracingDisabled() is invoked.
    std::function<void()> stop_complete_callback_;

    // An internal callback used to implement StopBlocking().
    std::function<void()> blocking_stop_complete_callback_;

    // Callback for a pending call to CloneTrace().
    TracingSession::CloneTraceCallback clone_trace_callback_;

    // Callback passed to ReadTrace().
    std::function<void(TracingSession::ReadTraceCallbackArgs)>
        read_trace_callback_;

    // Callback passed to GetTraceStats().
    TracingSession::GetTraceStatsCallback get_trace_stats_callback_;

    // Callback for a pending call to QueryServiceState().
    TracingSession::QueryServiceStateCallback query_service_state_callback_;

    // The states of all data sources in this tracing session. |true| means the
    // data source has started tracing.
    using DataSourceHandle = std::pair<std::string, std::string>;
    std::map<DataSourceHandle, bool> data_source_states_;

    std::unique_ptr<ConsumerEndpoint> service_;  // Keep before last.
    PERFETTO_THREAD_CHECKER(thread_checker_)     // Keep last.
  };

  // This object is returned to API clients when they call
  // Tracing::CreateTracingSession().
  class TracingSessionImpl : public TracingSession {
   public:
    TracingSessionImpl(TracingMuxerImpl*, TracingSessionGlobalID, BackendType);
    ~TracingSessionImpl() override;
    void Setup(const TraceConfig&, int fd) override;
    void Start() override;
    void StartBlocking() override;
    void CloneTrace(CloneTraceArgs args, CloneTraceCallback) override;
    void SetOnStartCallback(std::function<void()>) override;
    void SetOnErrorCallback(std::function<void(TracingError)>) override;
    void Stop() override;
    void StopBlocking() override;
    void Flush(std::function<void(bool)>, uint32_t timeout_ms) override;
    void ReadTrace(ReadTraceCallback) override;
    void SetOnStopCallback(std::function<void()>) override;
    void GetTraceStats(GetTraceStatsCallback) override;
    void QueryServiceState(QueryServiceStateCallback) override;
    void ChangeTraceConfig(const TraceConfig&) override;

   private:
    TracingMuxerImpl* const muxer_;
    TracingSessionGlobalID const session_id_;
    BackendType const backend_type_;
  };

  // This object is returned to API clients when they call
  // Tracing::SetupStartupTracing().
  class StartupTracingSessionImpl : public StartupTracingSession {
   public:
    StartupTracingSessionImpl(TracingMuxerImpl*,
                              TracingSessionGlobalID,
                              BackendType);
    ~StartupTracingSessionImpl() override;
    void Abort() override;
    void AbortBlocking() override;

   private:
    TracingMuxerImpl* const muxer_;
    TracingSessionGlobalID const session_id_;
    BackendType backend_type_;
  };

  struct RegisteredInterceptor {
    protos::gen::InterceptorDescriptor descriptor;
    InterceptorFactory factory{};
    InterceptorBase::TLSFactory tls_factory{};
    InterceptorBase::TracePacketCallback packet_callback{};
  };

  struct RegisteredStartupSession {
    TracingSessionID session_id = 0;
    int num_unbound_data_sources = 0;

    bool is_aborting = false;
    int num_aborting_data_sources = 0;

    std::function<void()> on_aborted;
    std::function<void()> on_adopted;
  };

  struct RegisteredProducerBackend {
    // Backends are supposed to have static lifetime.
    TracingProducerBackend* backend = nullptr;
    TracingBackendId id = 0;
    BackendType type{};

    TracingBackend::ConnectProducerArgs producer_conn_args;
    std::unique_ptr<ProducerImpl> producer;

    std::vector<RegisteredStartupSession> startup_sessions;
  };

  struct RegisteredConsumerBackend {
    // Backends are supposed to have static lifetime.
    TracingConsumerBackend* backend = nullptr;
    BackendType type{};
    // The calling code can request more than one concurrently active tracing
    // session for the same backend. We need to create one consumer per session.
    std::vector<std::unique_ptr<ConsumerImpl>> consumers;
  };

  void UpdateDataSourceOnAllBackends(RegisteredDataSource& rds,
                                     bool is_changed);
  explicit TracingMuxerImpl(const TracingInitArgs&);
  void Initialize(const TracingInitArgs& args);
  void AddBackends(const TracingInitArgs& args);
  void AddConsumerBackend(TracingConsumerBackend* backend, BackendType type);
  void AddProducerBackend(TracingProducerBackend* backend,
                          BackendType type,
                          const TracingInitArgs& args);
  ConsumerImpl* FindConsumer(TracingSessionGlobalID session_id);
  std::pair<ConsumerImpl*, RegisteredConsumerBackend*> FindConsumerAndBackend(
      TracingSessionGlobalID session_id);
  RegisteredProducerBackend* FindProducerBackendById(TracingBackendId id);
  RegisteredProducerBackend* FindProducerBackendByType(BackendType type);
  RegisteredConsumerBackend* FindConsumerBackendByType(BackendType type);
  void InitializeConsumer(TracingSessionGlobalID session_id);
  void OnConsumerDisconnected(ConsumerImpl* consumer);
  void OnProducerDisconnected(ProducerImpl* producer);
  // Test only method.
  void SweepDeadBackends();

  struct FindDataSourceRes {
    FindDataSourceRes() = default;
    FindDataSourceRes(DataSourceStaticState* a,
                      DataSourceState* b,
                      uint32_t c,
                      bool d)
        : static_state(a),
          internal_state(b),
          instance_idx(c),
          requires_callbacks_under_lock(d) {}
    explicit operator bool() const { return !!internal_state; }

    DataSourceStaticState* static_state = nullptr;
    DataSourceState* internal_state = nullptr;
    uint32_t instance_idx = 0;
    bool requires_callbacks_under_lock = false;
  };
  FindDataSourceRes FindDataSource(TracingBackendId, DataSourceInstanceID);

  FindDataSourceRes SetupDataSourceImpl(
      const RegisteredDataSource&,
      TracingBackendId,
      uint32_t backend_connection_id,
      DataSourceInstanceID,
      const DataSourceConfig&,
      TracingSessionGlobalID startup_session_id);
  void StartDataSourceImpl(const FindDataSourceRes&);
  void StopDataSource_AsyncBeginImpl(const FindDataSourceRes&);
  void StopDataSource_AsyncEnd(TracingBackendId,
                               uint32_t backend_connection_id,
                               DataSourceInstanceID,
                               const FindDataSourceRes&);
  bool FlushDataSource_AsyncBegin(TracingBackendId,
                                  DataSourceInstanceID,
                                  FlushRequestID,
                                  FlushFlags);
  void FlushDataSource_AsyncEnd(TracingBackendId,
                                uint32_t backend_connection_id,
                                DataSourceInstanceID,
                                const FindDataSourceRes&,
                                FlushRequestID);
  void AbortStartupTracingSession(TracingSessionGlobalID, BackendType);
  // When ResetForTesting() is executed, `cb` will be called on the calling
  // thread and on the muxer thread.
  void AppendResetForTestingCallback(std::function<void()> cb);

  // WARNING: If you add new state here, be sure to update ResetForTesting.
  std::unique_ptr<base::TaskRunner> task_runner_;
  std::vector<RegisteredDataSource> data_sources_;
  // These lists can only have one backend per BackendType. The elements are
  // sorted by BackendType priority (see BackendTypePriority). They always
  // contain a fake low-priority kUnspecifiedBackend at the end.
  std::list<RegisteredProducerBackend> producer_backends_;
  std::list<RegisteredConsumerBackend> consumer_backends_;
  std::vector<RegisteredInterceptor> interceptors_;
  TracingPolicy* policy_ = nullptr;

  // Learn more at TracingInitArgs::supports_multiple_data_source_instances
  bool supports_multiple_data_source_instances_ = true;

  std::atomic<TracingSessionGlobalID> next_tracing_session_id_{};
  std::atomic<uint32_t> next_data_source_index_{};
  uint32_t muxer_id_for_testing_{};

  // Maximum number of times we will try to reconnect producer backend.
  // Should only be modified for testing purposes.
  std::atomic<uint32_t> max_producer_reconnections_{100u};

  // Test only member.
  // After ResetForTesting() is called, holds tracing backends which needs to be
  // kept alive until all inbound references have gone away. See
  // SweepDeadBackends().
  std::list<RegisteredProducerBackend> dead_backends_;

  // Test only member.
  // Executes these cleanup functions on the calling thread and on the muxer
  // thread when ResetForTesting() is called.
  std::list<std::function<void()>> reset_callbacks_;

  PERFETTO_THREAD_CHECKER(thread_checker_)
};

}  // namespace internal
}  // namespace perfetto

#endif  // SRC_TRACING_INTERNAL_TRACING_MUXER_IMPL_H_
