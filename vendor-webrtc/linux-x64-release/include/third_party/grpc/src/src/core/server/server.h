//
// Copyright 2015 gRPC authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//

#ifndef GRPC_SRC_CORE_SERVER_SERVER_H
#define GRPC_SRC_CORE_SERVER_SERVER_H

#include <grpc/compression.h>
#include <grpc/grpc.h>
#include <grpc/passive_listener.h>
#include <grpc/slice.h>
#include <grpc/support/port_platform.h>
#include <grpc/support/time.h>
#include <stddef.h>
#include <stdint.h>

#include <algorithm>
#include <atomic>
#include <functional>
#include <list>
#include <memory>
#include <optional>
#include <string>
#include <utility>
#include <vector>

#include "absl/base/thread_annotations.h"
#include "absl/container/flat_hash_map.h"
#include "absl/container/flat_hash_set.h"
#include "absl/hash/hash.h"
#include "absl/random/random.h"
#include "absl/status/statusor.h"
#include "absl/strings/string_view.h"
#include "src/core/call/metadata_batch.h"
#include "src/core/channelz/channelz.h"
#include "src/core/lib/channel/channel_args.h"
#include "src/core/lib/channel/channel_fwd.h"
#include "src/core/lib/channel/channel_stack.h"
#include "src/core/lib/debug/trace.h"
#include "src/core/lib/iomgr/call_combiner.h"
#include "src/core/lib/iomgr/closure.h"
#include "src/core/lib/iomgr/endpoint.h"
#include "src/core/lib/iomgr/error.h"
#include "src/core/lib/iomgr/iomgr_fwd.h"
#include "src/core/lib/iomgr/resolved_address.h"
#include "src/core/lib/promise/arena_promise.h"
#include "src/core/lib/resource_quota/connection_quota.h"
#include "src/core/lib/slice/slice.h"
#include "src/core/lib/surface/channel.h"
#include "src/core/lib/surface/completion_queue.h"
#include "src/core/lib/transport/transport.h"
#include "src/core/server/server_interface.h"
#include "src/core/telemetry/call_tracer.h"
#include "src/core/util/cpp_impl_of.h"
#include "src/core/util/dual_ref_counted.h"
#include "src/core/util/orphanable.h"
#include "src/core/util/random_early_detection.h"
#include "src/core/util/ref_counted_ptr.h"
#include "src/core/util/sync.h"
#include "src/core/util/time.h"

#define GRPC_ARG_SERVER_MAX_PENDING_REQUESTS "grpc.server.max_pending_requests"
#define GRPC_ARG_SERVER_MAX_PENDING_REQUESTS_HARD_LIMIT \
  "grpc.server.max_pending_requests_hard_limit"

namespace grpc_core {

class ServerConfigFetcher
    : public CppImplOf<ServerConfigFetcher, grpc_server_config_fetcher> {
 public:
  class ConnectionManager
      : public grpc_core::DualRefCounted<ConnectionManager> {
   public:
    virtual absl::StatusOr<grpc_core::ChannelArgs>
    UpdateChannelArgsForConnection(const grpc_core::ChannelArgs& args,
                                   grpc_endpoint* tcp) = 0;
  };

  class WatcherInterface {
   public:
    virtual ~WatcherInterface() = default;
    // UpdateConnectionManager() is invoked by the config fetcher when a new
    // config is available. Implementations should update the connection manager
    // and start serving if not already serving.
    virtual void UpdateConnectionManager(
        grpc_core::RefCountedPtr<ConnectionManager> manager) = 0;
    // Implementations should stop serving when this is called. Serving should
    // only resume when UpdateConfig() is invoked.
    virtual void StopServing() = 0;
  };

  virtual ~ServerConfigFetcher() = default;

  virtual void StartWatch(std::string listening_address,
                          std::unique_ptr<WatcherInterface> watcher) = 0;
  virtual void CancelWatch(WatcherInterface* watcher) = 0;
  virtual grpc_pollset_set* interested_parties() = 0;
};

namespace experimental {
class PassiveListenerImpl;
}  // namespace experimental

namespace testing {
class ServerTestPeer;
class ListenerStateTestPeer;
}  // namespace testing

class Server : public ServerInterface,
               public InternallyRefCounted<Server>,
               public CppImplOf<Server, grpc_server> {
 public:
  // Filter vtable.
  static const grpc_channel_filter kServerTopFilter;

  // Opaque type used for registered methods.
  struct RegisteredMethod;

  // An object to represent the most relevant characteristics of a
  // newly-allocated call object when using an AllocatingRequestMatcherBatch.
  struct BatchCallAllocation {
    void* tag;
    grpc_call** call;
    grpc_metadata_array* initial_metadata;
    grpc_call_details* details;
    grpc_completion_queue* cq;
  };

  // An object to represent the most relevant characteristics of a
  // newly-allocated call object when using an
  // AllocatingRequestMatcherRegistered.
  struct RegisteredCallAllocation {
    void* tag;
    grpc_call** call;
    grpc_metadata_array* initial_metadata;
    gpr_timespec* deadline;
    grpc_byte_buffer** optional_payload;
    grpc_completion_queue* cq;
  };

  class ListenerState;

  /// Interface for listeners.
  class ListenerInterface : public InternallyRefCounted<ListenerInterface> {
   public:
    // State for a connection that is being managed by this listener.
    // The LogicalConnection interface helps the server keep track of
    // connections during handshake. If the server uses a config fetcher, the
    // connection continues to be tracked by the server to drain connections on
    // a config update. If not, the server stops the tracking after handshake is
    // done. As such, implementations of `LogicalConnection` should cancel the
    // handshake on `Orphan` if still in progress, but not close down the
    // transport.
    // Implementations are responsible for informing ListenerState about the
    // following stages of a connection -
    // 1) Invoke AddLogicalConnection() on accepting a new connection. Do not
    // invoke if the connection is going to be closed immediately.
    // 2) Invoke OnHandshakeDone() (irrespective of error) once handshake is
    // done. No need to invoke if `RemoveLogicalConnection()` has already been
    // invoked.
    // 3) Invoke RemoveLogicalConnection() when the connection is closed. Do not
    // invoke if the connection was never added.
    // TODO(yashykt): In the case where there is no config fetcher, we remove
    // the connection from our map and instead use `ChannelData` to keep track
    // of the connections. This is much cheaper (8 bytes per connection) as
    // compared to implementations of LogicalConnection which can be more than
    // 24 bytes based on the chttp2 implementation. This complexity causes
    // weirdness for our interfaces. Figure out a way to combine these two
    // tracking systems, without increasing memory utilization.
    class LogicalConnection : public InternallyRefCounted<LogicalConnection> {
     public:
      explicit LogicalConnection(const char* trace = nullptr)
          : InternallyRefCounted(trace) {}
      ~LogicalConnection() override = default;

      // The following two methods are called in the context of a server config
      // event.
      virtual void SendGoAway() = 0;
      virtual void DisconnectImmediately() = 0;
    };

    explicit ListenerInterface(const char* trace = nullptr)
        : InternallyRefCounted(trace) {}
    ~ListenerInterface() override = default;

    /// Starts listening.
    virtual void Start() = 0;

    /// Returns the channelz node for the listen socket, or null if not
    /// supported.
    virtual channelz::ListenSocketNode* channelz_listen_socket_node() const = 0;

    virtual void SetServerListenerState(
        RefCountedPtr<ListenerState> listener_state) = 0;

    virtual const grpc_resolved_address* resolved_address() const = 0;

    /// Sets a closure to be invoked by the listener when its destruction
    /// is complete.
    virtual void SetOnDestroyDone(grpc_closure* on_destroy_done) = 0;
  };

  // Implements the connection management and config fetching mechanism for
  // listeners.
  // Note that an alternative implementation would have been to combine the
  // ListenerInterface and ListenerState into a single parent class, but
  // they are being separated to make code simpler to understand.
  class ListenerState : public RefCounted<ListenerState> {
   public:
    explicit ListenerState(RefCountedPtr<Server> server,
                           OrphanablePtr<ListenerInterface> l);

    void Start();

    void Stop();

    ListenerInterface* listener() { return listener_.get(); }

    Server* server() const { return server_.get(); }

    // Adds a LogicalConnection to the listener and updates the channel args if
    // needed, and returns ChannelArgs if successful.
    std::optional<ChannelArgs> AddLogicalConnection(
        OrphanablePtr<ListenerInterface::LogicalConnection> connection,
        const ChannelArgs& args, grpc_endpoint* endpoint)
        ABSL_LOCKS_EXCLUDED(mu_);

    void OnHandshakeDone(ListenerInterface::LogicalConnection* connection);

    // Removes the logical connection from being tracked. This could happen for
    // reasons such as the connection being closed, or the connection has been
    // established (including handshake) and doesn't have a server config
    // fetcher.
    void RemoveLogicalConnection(
        ListenerInterface::LogicalConnection* connection);

    const MemoryQuotaRefPtr& memory_quota() const { return memory_quota_; }

    const ConnectionQuotaRefPtr& connection_quota() const {
      return connection_quota_;
    }

    grpc_event_engine::experimental::EventEngine* event_engine() const {
      return event_engine_;
    }

   private:
    friend class grpc_core::testing::ListenerStateTestPeer;

    class ConfigFetcherWatcher : public ServerConfigFetcher::WatcherInterface {
     public:
      explicit ConfigFetcherWatcher(ListenerState* listener_state)
          : listener_state_(listener_state) {}

      void UpdateConnectionManager(
          RefCountedPtr<ServerConfigFetcher::ConnectionManager>
              connection_manager) override;

      void StopServing() override;

     private:
      // This doesn't need to be ref-counted since we start and stop config
      // fetching as part of starting and stopping the listener.
      ListenerState* const listener_state_;
    };

    struct ConnectionsToBeDrained {
      absl::flat_hash_set<OrphanablePtr<ListenerInterface::LogicalConnection>>
          connections;
      grpc_core::Timestamp timestamp;
    };

    void DrainConnectionsLocked() ABSL_EXCLUSIVE_LOCKS_REQUIRED(mu_);

    void OnDrainGraceTimer();

    void MaybeStartNewGraceTimerLocked() ABSL_EXCLUSIVE_LOCKS_REQUIRED(mu_);

    void RemoveConnectionsToBeDrainedOnEmptyLocked(
        std::deque<ConnectionsToBeDrained>::iterator it)
        ABSL_EXCLUSIVE_LOCKS_REQUIRED(mu_);

    RefCountedPtr<Server> const server_;
    MemoryQuotaRefPtr const memory_quota_;
    ConnectionQuotaRefPtr connection_quota_;
    grpc_event_engine::experimental::EventEngine* const event_engine_;
    OrphanablePtr<ListenerInterface> listener_;
    grpc_closure destroy_done_;
    ConfigFetcherWatcher* config_fetcher_watcher_ = nullptr;
    Mutex mu_;  // We could share this mutex with Listener implementations. It's
                // a tradeoff between increased memory requirement and more
                // granular critical regions.
    RefCountedPtr<ServerConfigFetcher::ConnectionManager> connection_manager_
        ABSL_GUARDED_BY(mu_);
    bool is_serving_ ABSL_GUARDED_BY(mu_) = false;
    bool started_ ABSL_GUARDED_BY(mu_) = false;
    absl::flat_hash_set<OrphanablePtr<ListenerInterface::LogicalConnection>>
        connections_ ABSL_GUARDED_BY(mu_);
    std::deque<ConnectionsToBeDrained> connections_to_be_drained_list_
        ABSL_GUARDED_BY(mu_);
    grpc_event_engine::experimental::EventEngine::TaskHandle
        drain_grace_timer_handle_ ABSL_GUARDED_BY(mu_) =
            grpc_event_engine::experimental::EventEngine::TaskHandle::kInvalid;
  };

  explicit Server(const ChannelArgs& args);
  ~Server() override;

  void Orphan() ABSL_LOCKS_EXCLUDED(mu_global_) override;

  const ChannelArgs& channel_args() const override { return channel_args_; }
  channelz::ServerNode* channelz_node() const override {
    return channelz_node_.get();
  }

  // Do not call this before Start(). Returns the pollsets. The
  // vector itself is immutable, but the pollsets inside are mutable. The
  // result is valid for the lifetime of the server.
  const std::vector<grpc_pollset*>& pollsets() const { return pollsets_; }

  ServerConfigFetcher* config_fetcher() const { return config_fetcher_.get(); }

  ServerCallTracerFactory* server_call_tracer_factory() const override {
    return server_call_tracer_factory_;
  }

  void set_config_fetcher(std::unique_ptr<ServerConfigFetcher> config_fetcher) {
    config_fetcher_ = std::move(config_fetcher);
  }

  bool HasOpenConnections() ABSL_LOCKS_EXCLUDED(mu_global_);

  // Adds a listener to the server.  When the server starts, it will call
  // the listener's Start() method, and when it shuts down, it will orphan
  // the listener.
  void AddListener(OrphanablePtr<ListenerInterface> listener);

  // Starts listening for connections.
  void Start() ABSL_LOCKS_EXCLUDED(mu_global_);

  // Sets up a transport.  Creates a channel stack and binds the transport to
  // the server.  Called from the listener when a new connection is accepted.
  // Takes ownership of a ref on resource_user from the caller.
  grpc_error_handle SetupTransport(Transport* transport,
                                   grpc_pollset* accepting_pollset,
                                   const ChannelArgs& args)
      ABSL_LOCKS_EXCLUDED(mu_global_);

  void RegisterCompletionQueue(grpc_completion_queue* cq);

  // Functions to specify that a specific registered method or the unregistered
  // collection should use a specific allocator for request matching.
  void SetRegisteredMethodAllocator(
      grpc_completion_queue* cq, void* method_tag,
      std::function<RegisteredCallAllocation()> allocator);
  void SetBatchMethodAllocator(grpc_completion_queue* cq,
                               std::function<BatchCallAllocation()> allocator);

  RegisteredMethod* RegisterMethod(
      const char* method, const char* host,
      grpc_server_register_method_payload_handling payload_handling,
      uint32_t flags);

  grpc_call_error RequestCall(grpc_call** call, grpc_call_details* details,
                              grpc_metadata_array* request_metadata,
                              grpc_completion_queue* cq_bound_to_call,
                              grpc_completion_queue* cq_for_notification,
                              void* tag);

  grpc_call_error RequestRegisteredCall(
      RegisteredMethod* rm, grpc_call** call, gpr_timespec* deadline,
      grpc_metadata_array* request_metadata,
      grpc_byte_buffer** optional_payload,
      grpc_completion_queue* cq_bound_to_call,
      grpc_completion_queue* cq_for_notification, void* tag_new);

  void ShutdownAndNotify(grpc_completion_queue* cq, void* tag)
      ABSL_LOCKS_EXCLUDED(mu_global_, mu_call_);

  void StopListening();

  void CancelAllCalls() ABSL_LOCKS_EXCLUDED(mu_global_);

  void SendGoaways() ABSL_LOCKS_EXCLUDED(mu_global_, mu_call_);

  grpc_compression_options compression_options() const override {
    return compression_options_;
  }

 private:
  // note: the grpc_core::Server redundant namespace qualification is
  // required for older gcc versions.
  // TODO(yashykt): eliminate this friend statement as part of your upcoming
  // server listener refactoring.
  friend absl::Status(::grpc_server_add_passive_listener)(
      grpc_core::Server* server, grpc_server_credentials* credentials,
      std::shared_ptr<grpc_core::experimental::PassiveListenerImpl>
          passive_listener);

  friend class grpc_core::testing::ServerTestPeer;

  struct RequestedCall;

  class RequestMatcherInterface;
  class RealRequestMatcher;
  class AllocatingRequestMatcherBase;
  class AllocatingRequestMatcherBatch;
  class AllocatingRequestMatcherRegistered;

  class ChannelData final {
   public:
    ChannelData() = default;
    ~ChannelData();

    void InitTransport(RefCountedPtr<Server> server,
                       RefCountedPtr<Channel> channel, size_t cq_idx,
                       Transport* transport, intptr_t channelz_socket_uuid);

    RefCountedPtr<Server> server() const { return server_; }
    Channel* channel() const { return channel_.get(); }
    size_t cq_idx() const { return cq_idx_; }

    // Filter vtable functions.
    static grpc_error_handle InitChannelElement(
        grpc_channel_element* elem, grpc_channel_element_args* args);
    static void DestroyChannelElement(grpc_channel_element* elem);

   private:
    class ConnectivityWatcher;

    static void AcceptStream(void* arg, Transport* /*transport*/,
                             const void* transport_server_data);

    void Destroy() ABSL_EXCLUSIVE_LOCKS_REQUIRED(server_->mu_global_);

    static void FinishDestroy(void* arg, grpc_error_handle error);

    RefCountedPtr<Server> server_;
    RefCountedPtr<Channel> channel_;
    // The index into Server::cqs_ of the CQ used as a starting point for
    // where to publish new incoming calls.
    size_t cq_idx_;
    std::optional<std::list<ChannelData*>::iterator> list_position_;
    grpc_closure finish_destroy_channel_closure_;
    intptr_t channelz_socket_uuid_;
  };

  class CallData {
   public:
    enum class CallState {
      NOT_STARTED,  // Waiting for metadata.
      PENDING,      // Initial metadata read, not flow controlled in yet.
      ACTIVATED,    // Flow controlled in, on completion queue.
      ZOMBIED,      // Cancelled before being queued.
    };

    CallData(grpc_call_element* elem, const grpc_call_element_args& args,
             RefCountedPtr<Server> server);
    ~CallData();

    // Starts the recv_initial_metadata batch on the call.
    // Invoked from ChannelData::AcceptStream().
    void Start(grpc_call_element* elem);

    void SetState(CallState state);

    // Attempts to move from PENDING to ACTIVATED state.  Returns true
    // on success.
    bool MaybeActivate();

    // Publishes an incoming call to the application after it has been
    // matched.
    void Publish(size_t cq_idx, RequestedCall* rc);

    void KillZombie();

    void FailCallCreation();

    // Filter vtable functions.
    static grpc_error_handle InitCallElement(
        grpc_call_element* elem, const grpc_call_element_args* args);
    static void DestroyCallElement(grpc_call_element* elem,
                                   const grpc_call_final_info* /*final_info*/,
                                   grpc_closure* /*ignored*/);
    static void StartTransportStreamOpBatch(
        grpc_call_element* elem, grpc_transport_stream_op_batch* batch);

   private:
    // Helper functions for handling calls at the top of the call stack.
    static void RecvInitialMetadataBatchComplete(void* arg,
                                                 grpc_error_handle error);
    void StartNewRpc(grpc_call_element* elem);
    static void PublishNewRpc(void* arg, grpc_error_handle error);

    // Functions used inside the call stack.
    void StartTransportStreamOpBatchImpl(grpc_call_element* elem,
                                         grpc_transport_stream_op_batch* batch);
    static void RecvInitialMetadataReady(void* arg, grpc_error_handle error);
    static void RecvTrailingMetadataReady(void* arg, grpc_error_handle error);

    RefCountedPtr<Server> server_;

    grpc_call* call_;

    std::atomic<CallState> state_{CallState::NOT_STARTED};

    std::optional<Slice> path_;
    std::optional<Slice> host_;
    Timestamp deadline_ = Timestamp::InfFuture();

    grpc_completion_queue* cq_new_ = nullptr;

    RequestMatcherInterface* matcher_ = nullptr;
    grpc_byte_buffer* payload_ = nullptr;

    grpc_closure kill_zombie_closure_;

    grpc_metadata_array initial_metadata_ =
        grpc_metadata_array();  // Zero-initialize the C struct.
    grpc_closure recv_initial_metadata_batch_complete_;

    grpc_metadata_batch* recv_initial_metadata_ = nullptr;
    grpc_closure recv_initial_metadata_ready_;
    grpc_closure* original_recv_initial_metadata_ready_;
    grpc_error_handle recv_initial_metadata_error_;

    bool seen_recv_trailing_metadata_ready_ = false;
    grpc_closure recv_trailing_metadata_ready_;
    grpc_closure* original_recv_trailing_metadata_ready_;
    grpc_error_handle recv_trailing_metadata_error_;

    grpc_closure publish_;

    CallCombiner* call_combiner_;
  };

  struct ShutdownTag {
    ShutdownTag(void* tag_arg, grpc_completion_queue* cq_arg)
        : tag(tag_arg), cq(cq_arg) {}
    void* const tag;
    grpc_completion_queue* const cq;
    grpc_cq_completion completion;
  };

  struct StringViewStringViewPairHash
      : absl::flat_hash_set<
            std::pair<absl::string_view, absl::string_view>>::hasher {
    using is_transparent = void;
  };

  struct StringViewStringViewPairEq
      : std::equal_to<std::pair<absl::string_view, absl::string_view>> {
    using is_transparent = void;
  };

  class TransportConnectivityWatcher;

  RegisteredMethod* GetRegisteredMethod(const absl::string_view& host,
                                        const absl::string_view& path);
  void SetRegisteredMethodOnMetadata(ClientMetadata& metadata);

  static void ListenerDestroyDone(void* arg, grpc_error_handle error);

  static void DoneShutdownEvent(void* server,
                                grpc_cq_completion* /*completion*/) {
    static_cast<Server*>(server)->Unref();
  }

  static void DoneRequestEvent(void* req, grpc_cq_completion* completion);

  void FailCall(size_t cq_idx, RequestedCall* rc, grpc_error_handle error);
  grpc_call_error QueueRequestedCall(size_t cq_idx, RequestedCall* rc);

  void MaybeFinishShutdown() ABSL_EXCLUSIVE_LOCKS_REQUIRED(mu_global_)
      ABSL_LOCKS_EXCLUDED(mu_call_);

  void KillPendingWorkLocked(grpc_error_handle error)
      ABSL_EXCLUSIVE_LOCKS_REQUIRED(mu_call_);

  static grpc_call_error ValidateServerRequest(
      grpc_completion_queue* cq_for_notification, void* tag,
      grpc_byte_buffer** optional_payload, RegisteredMethod* rm);
  grpc_call_error ValidateServerRequestAndCq(
      size_t* cq_idx, grpc_completion_queue* cq_for_notification, void* tag,
      grpc_byte_buffer** optional_payload, RegisteredMethod* rm);

  std::vector<RefCountedPtr<Channel>> GetChannelsLocked() const;

  // Take a shutdown ref for a request (increment by 2) and return if shutdown
  // has not been called.
  bool ShutdownRefOnRequest() {
    int old_value = shutdown_refs_.fetch_add(2, std::memory_order_acq_rel);
    return (old_value & 1) != 0;
  }

  // Decrement the shutdown ref counter by either 1 (for shutdown call) or 2
  // (for in-flight request) and possibly call MaybeFinishShutdown if
  // appropriate.
  void ShutdownUnrefOnRequest() ABSL_LOCKS_EXCLUDED(mu_global_) {
    if (shutdown_refs_.fetch_sub(2, std::memory_order_acq_rel) == 2) {
      MutexLock lock(&mu_global_);
      MaybeFinishShutdown();
    }
  }
  void ShutdownUnrefOnShutdownCall() ABSL_EXCLUSIVE_LOCKS_REQUIRED(mu_global_) {
    if (shutdown_refs_.fetch_sub(1, std::memory_order_acq_rel) == 1) {
      // There is no request in-flight.
      MaybeFinishShutdown();
    }
  }

  bool ShutdownCalled() const {
    return (shutdown_refs_.load(std::memory_order_acquire) & 1) == 0;
  }

  // Returns whether there are no more shutdown refs, which means that shutdown
  // has been called and all accepted requests have been published if using an
  // AllocatingRequestMatcher.
  bool ShutdownReady() const {
    return shutdown_refs_.load(std::memory_order_acquire) == 0;
  }

  // Returns a promise that resolves to
  // tuple<
  //     optional<MessageHandle>,
  //     RequestMatcherInterface::MatchResult,
  //     ClientMetadataHandle>
  auto MatchRequestAndMaybeReadFirstMessage(CallHandler call_handler,
                                            ClientMetadataHandle md);
  auto MatchAndPublishCall(CallHandler call_handler);
  absl::StatusOr<RefCountedPtr<UnstartedCallDestination>> MakeCallDestination(
      const ChannelArgs& args);

  ChannelArgs const channel_args_;
  RefCountedPtr<channelz::ServerNode> channelz_node_;
  std::unique_ptr<ServerConfigFetcher> config_fetcher_;
  ServerCallTracerFactory* const server_call_tracer_factory_;

  std::vector<grpc_completion_queue*> cqs_;
  std::vector<grpc_pollset*> pollsets_;
  bool started_ = false;
  const grpc_compression_options compression_options_;

  // The two following mutexes control access to server-state.
  // mu_global_ controls access to non-call-related state (e.g., channel state).
  // mu_call_ controls access to call-related state (e.g., the call lists).
  //
  // If they are ever required to be nested, you must lock mu_global_
  // before mu_call_. This is currently used in shutdown processing
  // (ShutdownAndNotify() and MaybeFinishShutdown()).
  Mutex mu_global_;  // mutex for server and channel state
  Mutex mu_call_;    // mutex for call-specific state

  // startup synchronization: flag, signals whether we are doing the listener
  // start routine or not.
  bool starting_ ABSL_GUARDED_BY(mu_global_) = false;
  CondVar starting_cv_;

  // Map of registered methods.
  absl::flat_hash_map<std::pair<std::string, std::string> /*host, method*/,
                      std::unique_ptr<RegisteredMethod>,
                      StringViewStringViewPairHash, StringViewStringViewPairEq>
      registered_methods_;

  // Request matcher for unregistered methods.
  std::unique_ptr<RequestMatcherInterface> unregistered_request_matcher_;

  // The shutdown refs counter tracks whether or not shutdown has been called
  // and whether there are any AllocatingRequestMatcher requests that have been
  // accepted but not yet started (+2 on each one). If shutdown has been called,
  // the lowest bit will be 0 (defaults to 1) and the counter will be even. The
  // server should not notify on shutdown until the counter is 0 (shutdown is
  // called and there are no requests that are accepted but not started).
  std::atomic<int> shutdown_refs_{1};
  bool shutdown_published_ ABSL_GUARDED_BY(mu_global_) = false;
  std::vector<ShutdownTag> shutdown_tags_ ABSL_GUARDED_BY(mu_global_);

  RandomEarlyDetection pending_backlog_protector_ ABSL_GUARDED_BY(mu_call_){
      static_cast<uint64_t>(
          std::max(0, channel_args_.GetInt(GRPC_ARG_SERVER_MAX_PENDING_REQUESTS)
                          .value_or(1000))),
      static_cast<uint64_t>(std::max(
          0,
          channel_args_.GetInt(GRPC_ARG_SERVER_MAX_PENDING_REQUESTS_HARD_LIMIT)
              .value_or(3000)))};
  const Duration max_time_in_pending_queue_;

  std::list<ChannelData*> channels_;
  absl::flat_hash_set<OrphanablePtr<ServerTransport>> connections_
      ABSL_GUARDED_BY(mu_global_);
  RefCountedPtr<ServerConfigFetcher::ConnectionManager> connection_manager_
      ABSL_GUARDED_BY(mu_global_);
  size_t connections_open_ ABSL_GUARDED_BY(mu_global_) = 0;

  std::list<RefCountedPtr<ListenerState>> listener_states_;
  size_t listeners_destroyed_ = 0;

  // The last time we printed a shutdown progress message.
  gpr_timespec last_shutdown_message_time_;
};

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_SERVER_SERVER_H
