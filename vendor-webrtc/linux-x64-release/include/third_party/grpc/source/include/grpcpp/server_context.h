//
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
//

#ifndef GRPCPP_SERVER_CONTEXT_H
#define GRPCPP_SERVER_CONTEXT_H

#include <grpc/grpc.h>
#include <grpc/impl/call.h>
#include <grpc/impl/compression_types.h>
#include <grpc/support/port_platform.h>
#include <grpcpp/impl/call.h>
#include <grpcpp/impl/call_op_set.h>
#include <grpcpp/impl/codegen/create_auth_context.h>
#include <grpcpp/impl/codegen/metadata_map.h>
#include <grpcpp/impl/completion_queue_tag.h>
#include <grpcpp/impl/metadata_map.h>
#include <grpcpp/impl/rpc_service_method.h>
#include <grpcpp/security/auth_context.h>
#include <grpcpp/support/callback_common.h>
#include <grpcpp/support/config.h>
#include <grpcpp/support/message_allocator.h>
#include <grpcpp/support/server_callback.h>
#include <grpcpp/support/server_interceptor.h>
#include <grpcpp/support/status.h>
#include <grpcpp/support/string_ref.h>
#include <grpcpp/support/time.h>

#include <atomic>
#include <cassert>
#include <map>
#include <memory>
#include <type_traits>
#include <vector>

struct grpc_metadata;
struct grpc_call;
struct census_context;

namespace grpc {
template <class W, class R>
class ServerAsyncReader;
template <class W>
class ServerAsyncWriter;
template <class W>
class ServerAsyncResponseWriter;
template <class W, class R>
class ServerAsyncReaderWriter;
template <class R>
class ServerReader;
template <class W>
class ServerWriter;

namespace internal {
template <class ServiceType, class RequestType, class ResponseType>
class BidiStreamingHandler;
template <class RequestType, class ResponseType>
class CallbackUnaryHandler;
template <class RequestType, class ResponseType>
class CallbackClientStreamingHandler;
template <class RequestType, class ResponseType>
class CallbackServerStreamingHandler;
template <class RequestType, class ResponseType>
class CallbackBidiHandler;
template <class ServiceType, class RequestType, class ResponseType>
class ClientStreamingHandler;
template <class ResponseType>
void UnaryRunHandlerHelper(const MethodHandler::HandlerParameter&,
                           ResponseType*, Status&);
template <class ServiceType, class RequestType, class ResponseType,
          class BaseRequestType, class BaseResponseType>
class RpcMethodHandler;
template <class Base>
class FinishOnlyReactor;
template <class W, class R>
class ServerReaderWriterBody;
template <class ServiceType, class RequestType, class ResponseType>
class ServerStreamingHandler;
class ServerReactor;
template <class Streamer, bool WriteNeeded>
class TemplatedBidiStreamingHandler;
template <grpc::StatusCode code>
class ErrorMethodHandler;
}  // namespace internal

class ClientContext;
class CompletionQueue;
class GenericServerContext;
class Server;
class ServerInterface;
class ContextAllocator;
class GenericCallbackServerContext;

namespace internal {
class Call;
}  // namespace internal

namespace testing {
class InteropServerContextInspector;
class ServerContextTestSpouse;
class DefaultReactorTestPeer;
}  // namespace testing

namespace experimental {
class CallMetricRecorder;
class ServerMetricRecorder;
}  // namespace experimental

/// Base class of ServerContext.
class ServerContextBase {
 public:
  virtual ~ServerContextBase();

  /// Return the deadline for the server call.
  std::chrono::system_clock::time_point deadline() const {
    return grpc::Timespec2Timepoint(deadline_);
  }

  /// Return a \a gpr_timespec representation of the server call's deadline.
  gpr_timespec raw_deadline() const { return deadline_; }

  /// Add the (\a key, \a value) pair to the initial metadata
  /// associated with a server call. These are made available at the client side
  /// by the \a grpc::ClientContext::GetServerInitialMetadata() method.
  ///
  /// \warning This method should only be called before sending initial metadata
  /// to the client (which can happen explicitly, or implicitly when sending a
  /// a response message or status to the client).
  ///
  /// \param key The metadata key. If \a value is binary data, it must
  /// end in "-bin".
  /// \param value The metadata value. If its value is binary, the key name
  /// must end in "-bin".
  ///
  /// Metadata must conform to the following format:
  ///
  ///\verbatim
  /// Custom-Metadata -> Binary-Header / ASCII-Header
  /// Binary-Header -> {Header-Name "-bin" } {binary value}
  /// ASCII-Header -> Header-Name ASCII-Value
  /// Header-Name -> 1*( %x30-39 / %x61-7A / "_" / "-" / ".") ; 0-9 a-z _ - .
  /// ASCII-Value -> 1*( %x20-%x7E ) ; space and printable ASCII
  ///\endverbatim
  ///
  void AddInitialMetadata(const std::string& key, const std::string& value);

  /// Add the (\a key, \a value) pair to the initial metadata
  /// associated with a server call. These are made available at the client
  /// side by the \a grpc::ClientContext::GetServerTrailingMetadata() method.
  ///
  /// \warning This method should only be called before sending trailing
  /// metadata to the client (which happens when the call is finished and a
  /// status is sent to the client).
  ///
  /// \param key The metadata key. If \a value is binary data,
  /// it must end in "-bin".
  /// \param value The metadata value. If its value is binary, the key name
  /// must end in "-bin".
  ///
  /// Metadata must conform to the following format:
  ///
  ///\verbatim
  /// Custom-Metadata -> Binary-Header / ASCII-Header
  /// Binary-Header -> {Header-Name "-bin" } {binary value}
  /// ASCII-Header -> Header-Name ASCII-Value
  /// Header-Name -> 1*( %x30-39 / %x61-7A / "_" / "-" / ".") ; 0-9 a-z _ - .
  /// ASCII-Value -> 1*( %x20-%x7E ) ; space and printable ASCII
  ///\endverbatim
  ///
  void AddTrailingMetadata(const std::string& key, const std::string& value);

  /// Return whether this RPC failed before the server could provide its status
  /// back to the client. This could be because of explicit API cancellation
  /// from the client-side or server-side, because of deadline exceeded, network
  /// connection reset, HTTP/2 parameter configuration (e.g., max message size,
  /// max connection age), etc. It does NOT include failure due to a non-OK
  /// status return from the server application's request handler, including
  /// Status::CANCELLED.
  ///
  /// IsCancelled is always safe to call when using sync or callback API.
  /// When using async API, it is only safe to call IsCancelled after
  /// the AsyncNotifyWhenDone tag has been delivered. Thread-safe.
  bool IsCancelled() const;

  /// Cancel the Call from the server. This is a best-effort API and
  /// depending on when it is called, the RPC may still appear successful to
  /// the client. For example, if TryCancel() is called on a separate thread, it
  /// might race with the server handler which might return success to the
  /// client before TryCancel() was even started by the thread.
  ///
  /// It is the caller's responsibility to prevent such races and ensure that if
  /// TryCancel() is called, the serverhandler must return Status::CANCELLED.
  /// The only exception is that if the serverhandler is already returning an
  /// error status code, it is ok to not return Status::CANCELLED even if
  /// TryCancel() was called. Additionally, it is illegal to invoke TryCancel()
  /// before the call has actually begun, i.e., before metadata has been
  /// received from the client.
  ///
  /// For reasons such as the above, it is generally preferred to explicitly
  /// finish an RPC by returning Status::CANCELLED rather than using TryCancel.
  ///
  /// Note that TryCancel() does not change any of the tags that are pending
  /// on the completion queue. All pending tags will still be delivered
  /// (though their ok result may reflect the effect of cancellation).
  void TryCancel() const;

  /// Return a collection of initial metadata key-value pairs sent from the
  /// client. Note that keys may happen more than
  /// once (ie, a \a std::multimap is returned).
  ///
  /// It is safe to use this method after initial metadata has been received,
  /// Calls always begin with the client sending initial metadata, so this is
  /// safe to access as soon as the call has begun on the server side.
  ///
  /// \return A multimap of initial metadata key-value pairs from the server.
  const std::multimap<grpc::string_ref, grpc::string_ref>& client_metadata()
      const {
    return *client_metadata_.map();
  }

  /// Return the compression algorithm to be used by the server call.
  grpc_compression_level compression_level() const {
    return compression_level_;
  }

  /// Set \a level to be the compression level used for the server call.
  ///
  /// \param level The compression level used for the server call.
  void set_compression_level(grpc_compression_level level) {
    compression_level_set_ = true;
    compression_level_ = level;
  }

  /// Return a bool indicating whether the compression level for this call
  /// has been set (either implicitly or through a previous call to
  /// \a set_compression_level.
  bool compression_level_set() const { return compression_level_set_; }

  /// Return the compression algorithm the server call will request be used.
  /// Note that the gRPC runtime may decide to ignore this request, for example,
  /// due to resource constraints, or if the server is aware the client doesn't
  /// support the requested algorithm.
  grpc_compression_algorithm compression_algorithm() const {
    return compression_algorithm_;
  }
  /// Set \a algorithm to be the compression algorithm used for the server call.
  ///
  /// \param algorithm The compression algorithm used for the server call.
  void set_compression_algorithm(grpc_compression_algorithm algorithm);

  /// Set the serialized load reporting costs in \a cost_data for the call.
  void SetLoadReportingCosts(const std::vector<std::string>& cost_data);

  /// Return the authentication context for this server call.
  ///
  /// \see grpc::AuthContext.
  std::shared_ptr<const grpc::AuthContext> auth_context() const {
    if (auth_context_ == nullptr) {
      auth_context_ = grpc::CreateAuthContext(call_.call);
    }
    return auth_context_;
  }

  /// Return the peer uri in a string.
  /// WARNING: this value is never authenticated or subject to any security
  /// related code. It must not be used for any authentication related
  /// functionality. Instead, use auth_context.
  std::string peer() const;

  /// Get the census context associated with this server call.
  const struct census_context* census_context() const;

  /// Should be used for framework-level extensions only.
  /// Applications never need to call this method.
  grpc_call* c_call() { return call_.call; }

  /// Get the \a CallMetricRecorder object for the current RPC.
  /// Use it to record metrics during your RPC to send back to the
  /// client in order to make load balancing decisions. This will
  /// return nullptr if the feature hasn't been enabled using
  /// \a EnableCallMetricRecording.
  experimental::CallMetricRecorder* ExperimentalGetCallMetricRecorder() {
    return call_metric_recorder_;
  }

  /// EXPERIMENTAL API
  /// Returns the call's authority.
  grpc::string_ref ExperimentalGetAuthority() const;

 protected:
  /// Async only. Has to be called before the rpc starts.
  /// Returns the tag in completion queue when the rpc finishes.
  /// IsCancelled() can then be called to check whether the rpc was cancelled.
  /// Note: the tag will only be returned if call starts.
  /// If the call never starts, this tag will not be returned.
  void AsyncNotifyWhenDone(void* tag) {
    has_notify_when_done_tag_ = true;
    async_notify_when_done_tag_ = tag;
  }

  /// NOTE: This is an API for advanced users who need custom allocators.
  /// Get and maybe mutate the allocator state associated with the current RPC.
  /// Currently only applicable for callback unary RPC methods.
  RpcAllocatorState* GetRpcAllocatorState() { return message_allocator_state_; }

  /// Get a library-owned default unary reactor for use in minimal reaction
  /// cases. This supports typical unary RPC usage of providing a response and
  /// status. It supports immediate Finish (finish from within the method
  /// handler) or delayed Finish (finish called after the method handler
  /// invocation). It does not support reacting to cancellation or completion,
  /// or early sending of initial metadata. Since this is a library-owned
  /// reactor, it should not be delete'd or freed in any way. This is more
  /// efficient than creating a user-owned reactor both because of avoiding an
  /// allocation and because its minimal reactions are optimized using a core
  /// surface flag that allows their reactions to run inline without any
  /// thread-hop.
  ///
  /// This method should not be called more than once or called after return
  /// from the method handler.
  grpc::ServerUnaryReactor* DefaultReactor() {
    // Short-circuit the case where a default reactor was already set up by
    // the TestPeer.
    if (test_unary_ != nullptr) {
      return reinterpret_cast<Reactor*>(&default_reactor_);
    }
    new (&default_reactor_) Reactor;
#ifndef NDEBUG
    bool old = false;
    assert(default_reactor_used_.compare_exchange_strong(
        old, true, std::memory_order_relaxed));
#else
    default_reactor_used_.store(true, std::memory_order_relaxed);
#endif
    return reinterpret_cast<Reactor*>(&default_reactor_);
  }

  /// Constructors for use by derived classes
  ServerContextBase();
  ServerContextBase(gpr_timespec deadline, grpc_metadata_array* arr);

  void set_context_allocator(ContextAllocator* context_allocator) {
    context_allocator_ = context_allocator;
  }

  ContextAllocator* context_allocator() const { return context_allocator_; }

 private:
  friend class grpc::testing::InteropServerContextInspector;
  friend class grpc::testing::ServerContextTestSpouse;
  friend class grpc::testing::DefaultReactorTestPeer;
  friend class grpc::ServerInterface;
  friend class grpc::Server;
  template <class W, class R>
  friend class grpc::ServerAsyncReader;
  template <class W>
  friend class grpc::ServerAsyncWriter;
  template <class W>
  friend class grpc::ServerAsyncResponseWriter;
  template <class W, class R>
  friend class grpc::ServerAsyncReaderWriter;
  template <class R>
  friend class grpc::ServerReader;
  template <class W>
  friend class grpc::ServerWriter;
  template <class W, class R>
  friend class grpc::internal::ServerReaderWriterBody;
  template <class ResponseType>
  friend void grpc::internal::UnaryRunHandlerHelper(
      const internal::MethodHandler::HandlerParameter& param, ResponseType* rsp,
      Status& status);
  template <class ServiceType, class RequestType, class ResponseType,
            class BaseRequestType, class BaseResponseType>
  friend class grpc::internal::RpcMethodHandler;
  template <class ServiceType, class RequestType, class ResponseType>
  friend class grpc::internal::ClientStreamingHandler;
  template <class ServiceType, class RequestType, class ResponseType>
  friend class grpc::internal::ServerStreamingHandler;
  template <class Streamer, bool WriteNeeded>
  friend class grpc::internal::TemplatedBidiStreamingHandler;
  template <class RequestType, class ResponseType>
  friend class grpc::internal::CallbackUnaryHandler;
  template <class RequestType, class ResponseType>
  friend class grpc::internal::CallbackClientStreamingHandler;
  template <class RequestType, class ResponseType>
  friend class grpc::internal::CallbackServerStreamingHandler;
  template <class RequestType, class ResponseType>
  friend class grpc::internal::CallbackBidiHandler;
  template <grpc::StatusCode code>
  friend class grpc::internal::ErrorMethodHandler;
  template <class Base>
  friend class grpc::internal::FinishOnlyReactor;
  friend class grpc::ClientContext;
  friend class grpc::GenericServerContext;
  friend class grpc::GenericCallbackServerContext;

  /// Prevent copying.
  ServerContextBase(const ServerContextBase&);
  ServerContextBase& operator=(const ServerContextBase&);

  class CompletionOp;

  void BeginCompletionOp(
      grpc::internal::Call* call, std::function<void(bool)> callback,
      grpc::internal::ServerCallbackCall* callback_controller);
  /// Return the tag queued by BeginCompletionOp()
  grpc::internal::CompletionQueueTag* GetCompletionOpTag();

  void set_call(grpc_call* call, bool call_metric_recording_enabled,
                experimental::ServerMetricRecorder* server_metric_recorder) {
    call_.call = call;
    if (call_metric_recording_enabled) {
      CreateCallMetricRecorder(server_metric_recorder);
    }
  }

  void BindDeadlineAndMetadata(gpr_timespec deadline, grpc_metadata_array* arr);

  uint32_t initial_metadata_flags() const { return 0; }

  grpc::experimental::ServerRpcInfo* set_server_rpc_info(
      const char* method, grpc::internal::RpcMethod::RpcType type,
      const std::vector<std::unique_ptr<
          grpc::experimental::ServerInterceptorFactoryInterface>>& creators) {
    if (!creators.empty()) {
      rpc_info_ = new grpc::experimental::ServerRpcInfo(this, method, type);
      rpc_info_->RegisterInterceptors(creators);
    }
    return rpc_info_;
  }

  void set_message_allocator_state(RpcAllocatorState* allocator_state) {
    message_allocator_state_ = allocator_state;
  }

  void MaybeMarkCancelledOnRead() {
    if (grpc_call_failed_before_recv_message(call_.call)) {
      marked_cancelled_.store(true, std::memory_order_release);
    }
  }

  // This should be called only once and only when call metric recording is
  // enabled.
  void CreateCallMetricRecorder(
      experimental::ServerMetricRecorder* server_metric_recorder = nullptr);

  struct CallWrapper {
    ~CallWrapper();

    grpc_call* call = nullptr;
  };

  // NOTE: call_ must be the first data member of this object so that its
  //       destructor is the last to be called, since its destructor may unref
  //       the underlying core call which holds the arena that may be used to
  //       hold this object.
  CallWrapper call_;

  CompletionOp* completion_op_ = nullptr;
  bool has_notify_when_done_tag_ = false;
  void* async_notify_when_done_tag_ = nullptr;
  grpc::internal::CallbackWithSuccessTag completion_tag_;

  gpr_timespec deadline_;
  grpc::CompletionQueue* cq_ = nullptr;
  bool sent_initial_metadata_ = false;
  mutable std::shared_ptr<const grpc::AuthContext> auth_context_;
  mutable grpc::internal::MetadataMap client_metadata_;
  std::multimap<std::string, std::string> initial_metadata_;
  std::multimap<std::string, std::string> trailing_metadata_;

  bool compression_level_set_ = false;
  grpc_compression_level compression_level_;
  grpc_compression_algorithm compression_algorithm_;

  grpc::internal::CallOpSet<grpc::internal::CallOpSendInitialMetadata,
                            grpc::internal::CallOpSendMessage>
      pending_ops_;
  bool has_pending_ops_ = false;

  grpc::experimental::ServerRpcInfo* rpc_info_ = nullptr;
  RpcAllocatorState* message_allocator_state_ = nullptr;
  ContextAllocator* context_allocator_ = nullptr;
  experimental::CallMetricRecorder* call_metric_recorder_ = nullptr;

  class Reactor : public grpc::ServerUnaryReactor {
   public:
    void OnCancel() override {}
    void OnDone() override {}
    // Override InternalInlineable for this class since its reactions are
    // trivial and thus do not need to be run from the executor (triggering a
    // thread hop). This should only be used by internal reactors (thus the
    // name) and not by user application code.
    bool InternalInlineable() override { return true; }
  };

  void SetupTestDefaultReactor(std::function<void(grpc::Status)> func) {
    // NOLINTNEXTLINE(modernize-make-unique)
    test_unary_.reset(new TestServerCallbackUnary(this, std::move(func)));
  }
  bool test_status_set() const {
    return (test_unary_ != nullptr) && test_unary_->status_set();
  }
  grpc::Status test_status() const { return test_unary_->status(); }

  class TestServerCallbackUnary : public grpc::ServerCallbackUnary {
   public:
    TestServerCallbackUnary(ServerContextBase* ctx,
                            std::function<void(grpc::Status)> func)
        : reactor_(ctx->DefaultReactor()),
          func_(std::move(func)),
          call_(ctx->c_call()) {
      this->BindReactor(reactor_);
    }
    void Finish(grpc::Status s) override {
      status_ = s;
      func_(std::move(s));
      status_set_.store(true, std::memory_order_release);
    }
    void SendInitialMetadata() override {}

    bool status_set() const {
      return status_set_.load(std::memory_order_acquire);
    }
    grpc::Status status() const { return status_; }

   private:
    void CallOnDone() override {}

    grpc_call* call() override { return call_; }

    grpc::internal::ServerReactor* reactor() override { return reactor_; }

    grpc::ServerUnaryReactor* const reactor_;
    std::atomic_bool status_set_{false};
    grpc::Status status_;
    const std::function<void(grpc::Status s)> func_;
    grpc_call* call_;
  };

  alignas(Reactor) char default_reactor_[sizeof(Reactor)];
  std::atomic_bool default_reactor_used_{false};

  std::atomic_bool marked_cancelled_{false};

  std::unique_ptr<TestServerCallbackUnary> test_unary_;
};

/// A ServerContext or CallbackServerContext allows the code implementing a
/// service handler to:
///
/// - Add custom initial and trailing metadata key-value pairs that will
///   propagated to the client side.
/// - Control call settings such as compression and authentication.
/// - Access metadata coming from the client.
/// - Get performance metrics (ie, census).
///
/// Context settings are only relevant to the call handler they are supplied to,
/// that is to say, they aren't sticky across multiple calls. Some of these
/// settings, such as the compression options, can be made persistent at server
/// construction time by specifying the appropriate \a ChannelArguments
/// to a \a grpc::ServerBuilder, via \a ServerBuilder::AddChannelArgument.
///
/// \warning ServerContext instances should \em not be reused across rpcs.
class ServerContext : public ServerContextBase {
 public:
  ServerContext() {}  // for async calls

  using ServerContextBase::AddInitialMetadata;
  using ServerContextBase::AddTrailingMetadata;
  using ServerContextBase::auth_context;
  using ServerContextBase::c_call;
  using ServerContextBase::census_context;
  using ServerContextBase::client_metadata;
  using ServerContextBase::compression_algorithm;
  using ServerContextBase::compression_level;
  using ServerContextBase::compression_level_set;
  using ServerContextBase::deadline;
  using ServerContextBase::IsCancelled;
  using ServerContextBase::peer;
  using ServerContextBase::raw_deadline;
  using ServerContextBase::set_compression_algorithm;
  using ServerContextBase::set_compression_level;
  using ServerContextBase::SetLoadReportingCosts;
  using ServerContextBase::TryCancel;

  // Sync/CQ-based Async ServerContext only
  using ServerContextBase::AsyncNotifyWhenDone;

 private:
  // Constructor for internal use by server only
  friend class grpc::Server;
  ServerContext(gpr_timespec deadline, grpc_metadata_array* arr)
      : ServerContextBase(deadline, arr) {}

  // CallbackServerContext only
  using ServerContextBase::DefaultReactor;
  using ServerContextBase::GetRpcAllocatorState;

  /// Prevent copying.
  ServerContext(const ServerContext&) = delete;
  ServerContext& operator=(const ServerContext&) = delete;
};

class CallbackServerContext : public ServerContextBase {
 public:
  /// Public constructors are for direct use only by mocking tests. In practice,
  /// these objects will be owned by the library.
  CallbackServerContext() {}

  using ServerContextBase::AddInitialMetadata;
  using ServerContextBase::AddTrailingMetadata;
  using ServerContextBase::auth_context;
  using ServerContextBase::c_call;
  using ServerContextBase::census_context;
  using ServerContextBase::client_metadata;
  using ServerContextBase::compression_algorithm;
  using ServerContextBase::compression_level;
  using ServerContextBase::compression_level_set;
  using ServerContextBase::context_allocator;
  using ServerContextBase::deadline;
  using ServerContextBase::IsCancelled;
  using ServerContextBase::peer;
  using ServerContextBase::raw_deadline;
  using ServerContextBase::set_compression_algorithm;
  using ServerContextBase::set_compression_level;
  using ServerContextBase::set_context_allocator;
  using ServerContextBase::SetLoadReportingCosts;
  using ServerContextBase::TryCancel;

  // CallbackServerContext only
  using ServerContextBase::DefaultReactor;
  using ServerContextBase::GetRpcAllocatorState;

 private:
  // Sync/CQ-based Async ServerContext only
  using ServerContextBase::AsyncNotifyWhenDone;

  /// Prevent copying.
  CallbackServerContext(const CallbackServerContext&) = delete;
  CallbackServerContext& operator=(const CallbackServerContext&) = delete;
};

/// A CallbackServerContext allows users to use the contents of the
/// CallbackServerContext or GenericCallbackServerContext structure for the
/// callback API.
/// The library will invoke the allocator any time a new call is initiated.
/// and call the Release method after the server OnDone.
class ContextAllocator {
 public:
  virtual ~ContextAllocator() {}

  virtual CallbackServerContext* NewCallbackServerContext() { return nullptr; }

  virtual GenericCallbackServerContext* NewGenericCallbackServerContext() {
    return nullptr;
  }

  virtual void Release(CallbackServerContext*) {}

  virtual void Release(GenericCallbackServerContext*) {}
};

}  // namespace grpc

static_assert(
    std::is_base_of<grpc::ServerContextBase, grpc::ServerContext>::value,
    "improper base class");
static_assert(std::is_base_of<grpc::ServerContextBase,
                              grpc::CallbackServerContext>::value,
              "improper base class");
static_assert(sizeof(grpc::ServerContextBase) == sizeof(grpc::ServerContext),
              "wrong size");
static_assert(sizeof(grpc::ServerContextBase) ==
                  sizeof(grpc::CallbackServerContext),
              "wrong size");

#endif  // GRPCPP_SERVER_CONTEXT_H
