// Copyright 2021 The gRPC Authors
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
#ifndef GRPC_EVENT_ENGINE_EVENT_ENGINE_H
#define GRPC_EVENT_ENGINE_EVENT_ENGINE_H

#include <grpc/event_engine/endpoint_config.h>
#include <grpc/event_engine/extensible.h>
#include <grpc/event_engine/internal/write_event.h>
#include <grpc/event_engine/memory_allocator.h>
#include <grpc/event_engine/port.h>
#include <grpc/event_engine/slice_buffer.h>
#include <grpc/support/port_platform.h>

#include <bitset>
#include <initializer_list>
#include <vector>

#include "absl/functional/any_invocable.h"
#include "absl/status/status.h"
#include "absl/status/statusor.h"

// TODO(vigneshbabu): Define the Endpoint::Write metrics collection system
// TODO(hork): remove all references to the factory methods.
namespace grpc_event_engine {
namespace experimental {

////////////////////////////////////////////////////////////////////////////////
/// The EventEngine Interface
///
/// Overview
/// --------
///
/// The EventEngine encapsulates all platform-specific behaviors related to low
/// level network I/O, timers, asynchronous execution, and DNS resolution.
///
/// This interface allows developers to provide their own event management and
/// network stacks. Motivating uses cases for supporting custom EventEngines
/// include the ability to hook into external event loops, and using different
/// EventEngine instances for each channel to better insulate network I/O and
/// callback processing from other channels.
///
/// A default cross-platform EventEngine instance is provided by gRPC.
///
/// Lifespan and Ownership
/// ----------------------
///
/// gRPC takes shared ownership of EventEngines via std::shared_ptrs to ensure
/// that the engines remain available until they are no longer needed. Depending
/// on the use case, engines may live until gRPC is shut down.
///
/// EXAMPLE USAGE (Not yet implemented)
///
/// Custom EventEngines can be specified per channel, and allow configuration
/// for both clients and servers. To set a custom EventEngine for a client
/// channel, you can do something like the following:
///
///    ChannelArguments args;
///    std::shared_ptr<EventEngine> engine = std::make_shared<MyEngine>(...);
///    args.SetEventEngine(engine);
///    MyAppClient client(grpc::CreateCustomChannel(
///        "localhost:50051", grpc::InsecureChannelCredentials(), args));
///
/// A gRPC server can use a custom EventEngine by calling the
/// ServerBuilder::SetEventEngine method:
///
///    ServerBuilder builder;
///    std::shared_ptr<EventEngine> engine = std::make_shared<MyEngine>(...);
///    builder.SetEventEngine(engine);
///    std::unique_ptr<Server> server(builder.BuildAndStart());
///    server->Wait();
///
///
/// Blocking EventEngine Callbacks
/// ------------------------------
///
/// Doing blocking work in EventEngine callbacks is generally not advisable.
/// While gRPC's default EventEngine implementations have some capacity to scale
/// their thread pools to avoid starvation, this is not an instantaneous
/// process. Further, user-provided EventEngines may not be optimized to handle
/// excessive blocking work at all.
///
/// *Best Practice* : Occasional blocking work may be fine, but we do not
/// recommend running a mostly blocking workload in EventEngine threads.
///
///
/// Thread-safety guarantees
/// ------------------------
///
/// All EventEngine methods are guaranteed to be thread-safe, no external
/// synchronization is required to call any EventEngine method. Please note that
/// this does not apply to application callbacks, which may be run concurrently;
/// application state synchronization must be managed by the application.
///
////////////////////////////////////////////////////////////////////////////////
class EventEngine : public std::enable_shared_from_this<EventEngine>,
                    public Extensible {
 public:
  /// A duration between two events.
  ///
  /// Throughout the EventEngine API durations are used to express how long
  /// until an action should be performed.
  using Duration = std::chrono::duration<int64_t, std::nano>;
  /// A custom closure type for EventEngine task execution.
  ///
  /// Throughout the EventEngine API, \a Closure ownership is retained by the
  /// caller - the EventEngine will never delete a Closure, and upon
  /// cancellation, the EventEngine will simply forget the Closure exists. The
  /// caller is responsible for all necessary cleanup.

  class Closure {
   public:
    Closure() = default;
    // Closure's are an interface, and thus non-copyable.
    Closure(const Closure&) = delete;
    Closure& operator=(const Closure&) = delete;
    // Polymorphic type => virtual destructor
    virtual ~Closure() = default;
    // Run the contained code.
    virtual void Run() = 0;
  };
  /// Represents a scheduled task.
  ///
  /// \a TaskHandles are returned by \a Run* methods, and can be given to the
  /// \a Cancel method.
  struct TaskHandle {
    intptr_t keys[2];
    static const GRPC_DLL TaskHandle kInvalid;
  };
  /// A handle to a cancellable connection attempt.
  ///
  /// Returned by \a Connect, and can be passed to \a CancelConnect.
  struct ConnectionHandle {
    intptr_t keys[2];
    static const GRPC_DLL ConnectionHandle kInvalid;
  };
  /// Thin wrapper around a platform-specific sockaddr type. A sockaddr struct
  /// exists on all platforms that gRPC supports.
  ///
  /// Platforms are expected to provide definitions for:
  /// * sockaddr
  /// * sockaddr_in
  /// * sockaddr_in6
  class ResolvedAddress {
   public:
    static constexpr socklen_t MAX_SIZE_BYTES = 128;

    ResolvedAddress(const sockaddr* address, socklen_t size);
    ResolvedAddress() = default;
    ResolvedAddress(const ResolvedAddress&) = default;
    const struct sockaddr* address() const;
    socklen_t size() const;

   private:
    char address_[MAX_SIZE_BYTES] = {};
    socklen_t size_ = 0;
  };

  /// One end of a connection between a gRPC client and server. Endpoints are
  /// created when connections are established, and Endpoint operations are
  /// gRPC's primary means of communication.
  ///
  /// Endpoints must use the provided MemoryAllocator for all data buffer memory
  /// allocations. gRPC allows applications to set memory constraints per
  /// Channel or Server, and the implementation depends on all dynamic memory
  /// allocation being handled by the quota system.
  class Endpoint : public Extensible {
   public:
    /// Shuts down all connections and invokes all pending read or write
    /// callbacks with an error status.
    virtual ~Endpoint() = default;
    /// A struct representing optional arguments that may be provided to an
    /// EventEngine Endpoint Read API  call.
    ///
    /// Passed as argument to an Endpoint \a Read
    class ReadArgs final {
     public:
      ReadArgs() = default;
      ReadArgs(const ReadArgs&) = delete;
      ReadArgs& operator=(const ReadArgs&) = delete;
      ReadArgs(ReadArgs&&) = default;
      ReadArgs& operator=(ReadArgs&&) = default;

      // A suggestion to the endpoint implementation to read at-least the
      // specified number of bytes over the network connection before marking
      // the endpoint read operation as complete. gRPC may use this argument
      // to minimize the number of endpoint read API calls over the lifetime
      // of a connection.
      void set_read_hint_bytes(int64_t read_hint_bytes) {
        read_hint_bytes_ = read_hint_bytes;
      }
      int64_t read_hint_bytes() const { return read_hint_bytes_; }

     private:
      int64_t read_hint_bytes_ = 1;
    };
    /// Reads data from the Endpoint.
    ///
    /// When data is available on the connection, that data is moved into the
    /// \a buffer. If the read succeeds immediately, it returns true and the \a
    /// on_read callback is not executed. Otherwise it returns false and the \a
    /// on_read callback executes asynchronously when the read completes. The
    /// caller must ensure that the callback has access to the buffer when it
    /// executes. Ownership of the buffer is not transferred. Either an error is
    /// passed to the callback (like socket closed), or valid data is available
    /// in the buffer, but never both at the same time. Implementations that
    /// receive valid data must not throw that data away - that is, if valid
    /// data is received on the underlying endpoint, a callback will be made
    /// with that data available and an ok status.
    ///
    /// There can be at most one outstanding read per Endpoint at any given
    /// time. An outstanding read is one in which the \a on_read callback has
    /// not yet been executed for some previous call to \a Read.  If an attempt
    /// is made to call \a Read while a previous read is still outstanding, the
    /// \a EventEngine must abort.
    ///
    /// For failed read operations, implementations should pass the appropriate
    /// statuses to \a on_read. For example, callbacks might expect to receive
    /// CANCELLED on endpoint shutdown.
    virtual bool Read(absl::AnyInvocable<void(absl::Status)> on_read,
                      SliceBuffer* buffer, ReadArgs args) = 0;
    //// The set of write events that can be reported by an Endpoint.
    using WriteEvent = ::grpc_event_engine::experimental::internal::WriteEvent;
    /// An output WriteMetric consists of a key and a value.
    /// The space of keys can be queried from the endpoint via the
    /// \a AllWriteMetrics, \a GetMetricName and \a GetMetricKey APIs.
    /// The value is an int64_t that is implementation-defined. Check with the
    /// endpoint implementation documentation for the semantics of each metric.
    struct WriteMetric {
      size_t key;
      int64_t value;
    };
    using WriteEventCallback = absl::AnyInvocable<void(
        WriteEvent, absl::Time, std::vector<WriteMetric>) const>;
    // A bitmask of the events that the caller is interested in.
    // Each bit corresponds to an entry in WriteEvent.
    using WriteEventSet = std::bitset<static_cast<int>(WriteEvent::kCount)>;
    // A sink to receive write events.
    // The requested metrics are the keys of the metrics that the caller is
    // interested in. The on_event callback will be called on each event
    // requested.
    class WriteEventSink final {
     public:
      WriteEventSink(absl::Span<const size_t> requested_metrics,
                     std::initializer_list<WriteEvent> requested_events,
                     WriteEventCallback on_event)
          : requested_metrics_(requested_metrics),
            on_event_(std::move(on_event)) {
        for (auto event : requested_events) {
          requested_events_mask_.set(static_cast<int>(event));
        }
      }

      absl::Span<const size_t> requested_metrics() const {
        return requested_metrics_;
      }

      bool requested_event(WriteEvent event) const {
        return requested_events_mask_.test(static_cast<int>(event));
      }

      WriteEventSet requested_events_mask() const {
        return requested_events_mask_;
      }

      WriteEventCallback TakeEventCallback() { return std::move(on_event_); }

     private:
      absl::Span<const size_t> requested_metrics_;
      WriteEventSet requested_events_mask_;
      // The callback to be called on each event.
      WriteEventCallback on_event_;
    };
    /// A struct representing optional arguments that may be provided to an
    /// EventEngine Endpoint Write API call.
    ///
    /// Passed as argument to an Endpoint \a Write
    class WriteArgs final {
     public:
      WriteArgs() = default;
      WriteArgs(const WriteArgs&) = delete;
      WriteArgs& operator=(const WriteArgs&) = delete;
      WriteArgs(WriteArgs&&) = default;
      WriteArgs& operator=(WriteArgs&&) = default;

      // A sink to receive write events.
      std::optional<WriteEventSink> TakeMetricsSink() {
        auto sink = std::move(metrics_sink_);
        metrics_sink_.reset();
        return sink;
      }

      bool has_metrics_sink() const { return metrics_sink_.has_value(); }

      void set_metrics_sink(WriteEventSink sink) {
        metrics_sink_ = std::move(sink);
      }

      // Represents private information that may be passed by gRPC for
      // select endpoints expected to be used only within google.
      // TODO(ctiller): Remove this method once all callers are migrated to
      // metrics sink.
      void* GetDeprecatedAndDiscouragedGoogleSpecificPointer() {
        return google_specific_;
      }

      void SetDeprecatedAndDiscouragedGoogleSpecificPointer(void* pointer) {
        google_specific_ = pointer;
      }

      // A suggestion to the endpoint implementation to group data to be written
      // into frames of the specified max_frame_size. gRPC may use this
      // argument to dynamically control the max sizes of frames sent to a
      // receiver in response to high receiver memory pressure.
      int64_t max_frame_size() const { return max_frame_size_; }

      void set_max_frame_size(int64_t max_frame_size) {
        max_frame_size_ = max_frame_size;
      }

     private:
      std::optional<WriteEventSink> metrics_sink_;
      void* google_specific_ = nullptr;
      int64_t max_frame_size_ = 1024 * 1024;
    };
    /// Writes data out on the connection.
    ///
    /// If the write succeeds immediately, it returns true and the
    /// \a on_writable callback is not executed. Otherwise it returns false and
    /// the \a on_writable callback is called asynchronously when the connection
    /// is ready for more data. The Slices within the \a data buffer may be
    /// mutated at will by the Endpoint until \a on_writable is called. The \a
    /// data SliceBuffer will remain valid after calling \a Write, but its state
    /// is otherwise undefined.  All bytes in \a data must have been written
    /// before calling \a on_writable unless an error has occurred.
    ///
    /// There can be at most one outstanding write per Endpoint at any given
    /// time. An outstanding write is one in which the \a on_writable callback
    /// has not yet been executed for some previous call to \a Write.  If an
    /// attempt is made to call \a Write while a previous write is still
    /// outstanding, the \a EventEngine must abort.
    ///
    /// For failed write operations, implementations should pass the appropriate
    /// statuses to \a on_writable. For example, callbacks might expect to
    /// receive CANCELLED on endpoint shutdown.
    virtual bool Write(absl::AnyInvocable<void(absl::Status)> on_writable,
                       SliceBuffer* data, WriteArgs args) = 0;
    /// Returns an address in the format described in DNSResolver. The returned
    /// values are expected to remain valid for the life of the Endpoint.
    virtual const ResolvedAddress& GetPeerAddress() const = 0;
    virtual const ResolvedAddress& GetLocalAddress() const = 0;
    /// Returns the list of write metrics that the endpoint supports.
    /// The keys are used to identify the metrics in the GetMetricName and
    /// GetMetricKey APIs. The current value of the metric can be queried by
    /// adding a WriteEventSink to the WriteArgs of a Write call.
    virtual std::vector<size_t> AllWriteMetrics() = 0;
    /// Returns the name of the write metric with the given key.
    /// If the key is not found, returns std::nullopt.
    virtual std::optional<absl::string_view> GetMetricName(size_t key) = 0;
    /// Returns the key of the write metric with the given name.
    /// If the name is not found, returns std::nullopt.
    virtual std::optional<size_t> GetMetricKey(absl::string_view name) = 0;
  };

  /// Called when a new connection is established.
  ///
  /// If the connection attempt was not successful, implementations should pass
  /// the appropriate statuses to this callback. For example, callbacks might
  /// expect to receive DEADLINE_EXCEEDED statuses when appropriate, or
  /// CANCELLED statuses on EventEngine shutdown.
  using OnConnectCallback =
      absl::AnyInvocable<void(absl::StatusOr<std::unique_ptr<Endpoint>>)>;

  /// Listens for incoming connection requests from gRPC clients and initiates
  /// request processing once connections are established.
  class Listener : public Extensible {
   public:
    /// Called when the listener has accepted a new client connection.
    using AcceptCallback = absl::AnyInvocable<void(
        std::unique_ptr<Endpoint>, MemoryAllocator memory_allocator)>;
    virtual ~Listener() = default;
    /// Bind an address/port to this Listener.
    ///
    /// It is expected that multiple addresses/ports can be bound to this
    /// Listener before Listener::Start has been called. Returns either the
    /// bound port or an appropriate error status.
    virtual absl::StatusOr<int> Bind(const ResolvedAddress& addr) = 0;
    virtual absl::Status Start() = 0;
  };

  /// Factory method to create a network listener / server.
  ///
  /// Once a \a Listener is created and started, the \a on_accept callback will
  /// be called once asynchronously for each established connection. This method
  /// may return a non-OK status immediately if an error was encountered in any
  /// synchronous steps required to create the Listener. In this case,
  /// \a on_shutdown will never be called.
  ///
  /// If this method returns a Listener, then \a on_shutdown will be invoked
  /// exactly once when the Listener is shut down, and only after all
  /// \a on_accept callbacks have finished executing. The status passed to it
  /// will indicate if there was a problem during shutdown.
  ///
  /// The provided \a MemoryAllocatorFactory is used to create \a
  /// MemoryAllocators for Endpoint construction.
  virtual absl::StatusOr<std::unique_ptr<Listener>> CreateListener(
      Listener::AcceptCallback on_accept,
      absl::AnyInvocable<void(absl::Status)> on_shutdown,
      const EndpointConfig& config,
      std::unique_ptr<MemoryAllocatorFactory> memory_allocator_factory) = 0;
  /// Creates a client network connection to a remote network listener.
  ///
  /// Even in the event of an error, it is expected that the \a on_connect
  /// callback will be asynchronously executed exactly once by the EventEngine.
  /// A connection attempt can be cancelled using the \a CancelConnect method.
  ///
  /// Implementation Note: it is important that the \a memory_allocator be used
  /// for all read/write buffer allocations in the EventEngine implementation.
  /// This allows gRPC's \a ResourceQuota system to monitor and control memory
  /// usage with graceful degradation mechanisms. Please see the \a
  /// MemoryAllocator API for more information.
  virtual ConnectionHandle Connect(OnConnectCallback on_connect,
                                   const ResolvedAddress& addr,
                                   const EndpointConfig& args,
                                   MemoryAllocator memory_allocator,
                                   Duration timeout) = 0;

  /// Request cancellation of a connection attempt.
  ///
  /// If the associated connection has already been completed, it will not be
  /// cancelled, and this method will return false.
  ///
  /// If the associated connection has not been completed, it will be cancelled,
  /// and this method will return true. The \a OnConnectCallback will not be
  /// called, and \a on_connect will be destroyed before this method returns.
  virtual bool CancelConnect(ConnectionHandle handle) = 0;
  /// Provides asynchronous resolution.
  ///
  /// This object has a destruction-is-cancellation semantic.
  /// Implementations should make sure that all pending requests are cancelled
  /// when the object is destroyed and all pending callbacks will be called
  /// shortly. If cancellation races with request completion, implementations
  /// may choose to either cancel or satisfy the request.
  class DNSResolver : public Extensible {
   public:
    /// Optional configuration for DNSResolvers.
    struct ResolverOptions {
      /// If empty, default DNS servers will be used.
      /// Must be in the "IP:port" format as described in naming.md.
      std::string dns_server;
    };
    /// DNS SRV record type.
    struct SRVRecord {
      std::string host;
      int port = 0;
      int priority = 0;
      int weight = 0;
    };
    /// Called with the collection of sockaddrs that were resolved from a given
    /// target address.
    using LookupHostnameCallback =
        absl::AnyInvocable<void(absl::StatusOr<std::vector<ResolvedAddress>>)>;
    /// Called with a collection of SRV records.
    using LookupSRVCallback =
        absl::AnyInvocable<void(absl::StatusOr<std::vector<SRVRecord>>)>;
    /// Called with the result of a TXT record lookup
    using LookupTXTCallback =
        absl::AnyInvocable<void(absl::StatusOr<std::vector<std::string>>)>;

    virtual ~DNSResolver() = default;

    /// Asynchronously resolve an address.
    ///
    /// \a default_port may be a non-numeric named service port, and will only
    /// be used if \a address does not already contain a port component.
    ///
    /// When the lookup is complete or cancelled, the \a on_resolve callback
    /// will be invoked with a status indicating the success or failure of the
    /// lookup. Implementations should pass the appropriate statuses to the
    /// callback. For example, callbacks might expect to receive CANCELLED or
    /// NOT_FOUND.
    virtual void LookupHostname(LookupHostnameCallback on_resolve,
                                absl::string_view name,
                                absl::string_view default_port) = 0;
    /// Asynchronously perform an SRV record lookup.
    ///
    /// \a on_resolve has the same meaning and expectations as \a
    /// LookupHostname's \a on_resolve callback.
    virtual void LookupSRV(LookupSRVCallback on_resolve,
                           absl::string_view name) = 0;
    /// Asynchronously perform a TXT record lookup.
    ///
    /// \a on_resolve has the same meaning and expectations as \a
    /// LookupHostname's \a on_resolve callback.
    virtual void LookupTXT(LookupTXTCallback on_resolve,
                           absl::string_view name) = 0;
  };

  /// At time of destruction, the EventEngine must have no active
  /// responsibilities. EventEngine users (applications) are responsible for
  /// cancelling all tasks and DNS lookups, shutting down listeners and
  /// endpoints, prior to EventEngine destruction. If there are any outstanding
  /// tasks, any running listeners, etc. at time of EventEngine destruction,
  /// that is an invalid use of the API, and it will result in undefined
  /// behavior.
  virtual ~EventEngine() = default;

  // TODO(nnoble): consider whether we can remove this method before we
  // de-experimentalize this API.
  virtual bool IsWorkerThread() = 0;

  /// Creates and returns an instance of a DNSResolver, optionally configured by
  /// the \a options struct. This method may return a non-OK status if an error
  /// occurred when creating the DNSResolver. If the caller requests a custom
  /// DNS server, and the EventEngine implementation does not support it, this
  /// must return an error.
  virtual absl::StatusOr<std::unique_ptr<DNSResolver>> GetDNSResolver(
      const DNSResolver::ResolverOptions& options) = 0;

  /// Asynchronously executes a task as soon as possible.
  ///
  /// \a Closures passed to \a Run cannot be cancelled. The \a closure will not
  /// be deleted after it has been run, ownership remains with the caller.
  ///
  /// Implementations must not execute the closure in the calling thread before
  /// \a Run returns. For example, if the caller must release a lock before the
  /// closure can proceed, running the closure immediately would cause a
  /// deadlock.
  virtual void Run(Closure* closure) = 0;
  /// Asynchronously executes a task as soon as possible.
  ///
  /// \a Closures passed to \a Run cannot be cancelled. Unlike the overloaded \a
  /// Closure alternative, the absl::AnyInvocable version's \a closure will be
  /// deleted by the EventEngine after the closure has been run.
  ///
  /// This version of \a Run may be less performant than the \a Closure version
  /// in some scenarios. This overload is useful in situations where performance
  /// is not a critical concern.
  ///
  /// Implementations must not execute the closure in the calling thread before
  /// \a Run returns.
  virtual void Run(absl::AnyInvocable<void()> closure) = 0;
  /// Synonymous with scheduling an alarm to run after duration \a when.
  ///
  /// The \a closure will execute when time \a when arrives unless it has been
  /// cancelled via the \a Cancel method. If cancelled, the closure will not be
  /// run, nor will it be deleted. Ownership remains with the caller.
  ///
  /// Implementations must not execute the closure in the calling thread before
  /// \a RunAfter returns.
  ///
  /// Implementations may return a \a kInvalid handle if the callback can be
  /// immediately executed, and is therefore not cancellable.
  virtual TaskHandle RunAfter(Duration when, Closure* closure) = 0;
  /// Synonymous with scheduling an alarm to run after duration \a when.
  ///
  /// The \a closure will execute when time \a when arrives unless it has been
  /// cancelled via the \a Cancel method. If cancelled, the closure will not be
  /// run. Unlike the overloaded \a Closure alternative, the absl::AnyInvocable
  /// version's \a closure will be deleted by the EventEngine after the closure
  /// has been run, or upon cancellation.
  ///
  /// This version of \a RunAfter may be less performant than the \a Closure
  /// version in some scenarios. This overload is useful in situations where
  /// performance is not a critical concern.
  ///
  /// Implementations must not execute the closure in the calling thread before
  /// \a RunAfter returns.
  virtual TaskHandle RunAfter(Duration when,
                              absl::AnyInvocable<void()> closure) = 0;
  /// Request cancellation of a task.
  ///
  /// If the associated closure cannot be cancelled for any reason, this
  /// function will return false.
  ///
  /// If the associated closure can be cancelled, the associated callback will
  /// never be run, and this method will return true. If the callback type was
  /// an absl::AnyInvocable, it will be destroyed before the method returns.
  virtual bool Cancel(TaskHandle handle) = 0;
};

/// [DEPRECATED] Replace gRPC's default EventEngine factory.
///
/// Applications may call \a SetEventEngineFactory at any time to replace the
/// default factory used within gRPC. EventEngines will be created when
/// necessary, when they are otherwise not provided by the application.
///
/// To be certain that none of the gRPC-provided built-in EventEngines are
/// created, applications must set a custom EventEngine factory method *before*
/// grpc is initialized.
// TODO(hork): delete once all known users have migrated away
void SetEventEngineFactory(
    absl::AnyInvocable<std::shared_ptr<EventEngine>()> factory);

/// [DEPRECATED] Reset gRPC's EventEngine factory to the built-in default.
///
/// Applications that have called \a SetEventEngineFactory can remove their
/// custom factory using this method. The built-in EventEngine factories will be
/// used going forward. This has no affect on any EventEngines that were created
/// using the previous factories.
//
// TODO(hork): delete once all known users have migrated away
void EventEngineFactoryReset();

/// Create a new EventEngine instance.
std::shared_ptr<EventEngine> CreateEventEngine();

/// Set the default EventEngine instance, which will be used throughout gRPC
///
/// gRPC will hold a ref to this engine until either
/// \a ShutdownDefaultEventEngine() is called or \a SetDefaultEventEngine() is
/// called again with a different value. Passing a value of nullptr will cause
/// gRPC to drop the ref it was holding without setting it to a new one.
///
/// Earlier calls to \a GetDefaultEventEngine will still hold a ref to the
/// previous default engine instance, if any.
void SetDefaultEventEngine(std::shared_ptr<EventEngine> engine);

/// Returns the default EventEngine instance.
///
/// Note that if SetDefaultEventEngine() has not been called, then the default
/// EventEngine may be created and destroyed as needed, meaning that multiple
/// calls to GetDefaultEventEngine() over a process's lifetime may return
/// different instances. Callers are expected to call GetDefaultEventEngine()
/// once and hold the returned reference for as long as they need the
/// EventEngine instance.
std::shared_ptr<EventEngine> GetDefaultEventEngine();

/// Resets gRPC to use one of the default internal EventEngines for all *new*
/// \a GetDefaultEventEngine requests and blocks until all refs on the active
/// default engine have been released (destroying that engine).
///
/// If you called \a SetDefaultEventEngine, you must call either
/// \a ShutdownDefaultEventEngine or \a SetDefaultEventEngine(nullptr) at the
/// end of your program. If you don't, the engine will never be destroyed.
///
/// If you want to reset the default engine to one of gRPC's internal versions
/// without waiting for all references to be released on the current default
/// engine, call \a SetDefaultEventEngine(nullptr) instead.
void ShutdownDefaultEventEngine();

bool operator==(const EventEngine::TaskHandle& lhs,
                const EventEngine::TaskHandle& rhs);
bool operator!=(const EventEngine::TaskHandle& lhs,
                const EventEngine::TaskHandle& rhs);
std::ostream& operator<<(std::ostream& out,
                         const EventEngine::TaskHandle& handle);
bool operator==(const EventEngine::ConnectionHandle& lhs,
                const EventEngine::ConnectionHandle& rhs);
bool operator!=(const EventEngine::ConnectionHandle& lhs,
                const EventEngine::ConnectionHandle& rhs);
std::ostream& operator<<(std::ostream& out,
                         const EventEngine::ConnectionHandle& handle);

namespace detail {
std::string FormatHandleString(uint64_t key1, uint64_t key2);
}

template <typename Sink>
void AbslStringify(Sink& out, const EventEngine::ConnectionHandle& handle) {
  out.Append(detail::FormatHandleString(handle.keys[0], handle.keys[1]));
}

template <typename Sink>
void AbslStringify(Sink& out, const EventEngine::TaskHandle& handle) {
  out.Append(detail::FormatHandleString(handle.keys[0], handle.keys[1]));
}

}  // namespace experimental
}  // namespace grpc_event_engine

#endif  // GRPC_EVENT_ENGINE_EVENT_ENGINE_H
