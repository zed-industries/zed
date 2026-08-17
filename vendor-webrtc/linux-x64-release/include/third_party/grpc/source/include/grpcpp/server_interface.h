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

#ifndef GRPCPP_SERVER_INTERFACE_H
#define GRPCPP_SERVER_INTERFACE_H

#include <grpc/grpc.h>
#include <grpc/impl/grpc_types.h>
#include <grpc/support/port_platform.h>
#include <grpc/support/time.h>
#include <grpcpp/impl/call.h>
#include <grpcpp/impl/call_hook.h>
#include <grpcpp/impl/codegen/interceptor_common.h>
#include <grpcpp/impl/completion_queue_tag.h>
#include <grpcpp/impl/rpc_service_method.h>
#include <grpcpp/server_context.h>
#include <grpcpp/support/byte_buffer.h>

#include "absl/log/absl_check.h"

namespace grpc {

class AsyncGenericService;
class Channel;
class CompletionQueue;
class GenericServerContext;
class ServerCompletionQueue;
class ServerCredentials;
class Service;

/// Models a gRPC server.
///
/// Servers are configured and started via \a grpc::ServerBuilder.
namespace internal {
class ServerAsyncStreamingInterface;
}  // namespace internal

class CallbackGenericService;

namespace experimental {
class ServerInterceptorFactoryInterface;
class ServerMetricRecorder;
}  // namespace experimental

class ServerInterface : public internal::CallHook {
 public:
  ~ServerInterface() override {}

  /// \a Shutdown does the following things:
  ///
  /// 1. Shutdown the server: deactivate all listening ports, mark it in
  ///    "shutdown mode" so that further call Request's or incoming RPC matches
  ///    are no longer allowed. Also return all Request'ed-but-not-yet-active
  ///    calls as failed (!ok). This refers to calls that have been requested
  ///    at the server by the server-side library or application code but that
  ///    have not yet been matched to incoming RPCs from the client. Note that
  ///    this would even include default calls added automatically by the gRPC
  ///    C++ API without the user's input (e.g., "Unimplemented RPC method")
  ///
  /// 2. Block until all rpc method handlers invoked automatically by the sync
  ///    API finish.
  ///
  /// 3. If all pending calls complete (and all their operations are
  ///    retrieved by Next) before \a deadline expires, this finishes
  ///    gracefully. Otherwise, forcefully cancel all pending calls associated
  ///    with the server after \a deadline expires. In the case of the sync API,
  ///    if the RPC function for a streaming call has already been started and
  ///    takes a week to complete, the RPC function won't be forcefully
  ///    terminated (since that would leave state corrupt and incomplete) and
  ///    the method handler will just keep running (which will prevent the
  ///    server from completing the "join" operation that it needs to do at
  ///    shutdown time).
  ///
  /// All completion queue associated with the server (for example, for async
  /// serving) must be shutdown *after* this method has returned:
  /// See \a ServerBuilder::AddCompletionQueue for details.
  /// They must also be drained (by repeated Next) after being shutdown.
  ///
  /// \param deadline How long to wait until pending rpcs are forcefully
  /// terminated.
  template <class T>
  void Shutdown(const T& deadline) {
    ShutdownInternal(TimePoint<T>(deadline).raw_time());
  }

  /// Shutdown the server without a deadline and forced cancellation.
  ///
  /// All completion queue associated with the server (for example, for async
  /// serving) must be shutdown *after* this method has returned:
  /// See \a ServerBuilder::AddCompletionQueue for details.
  void Shutdown() { ShutdownInternal(gpr_inf_future(GPR_CLOCK_MONOTONIC)); }

  /// Block waiting for all work to complete.
  ///
  /// \warning The server must be either shutting down or some other thread must
  /// call \a Shutdown for this function to ever return.
  virtual void Wait() = 0;

 protected:
  friend class grpc::Service;

  /// Register a service. This call does not take ownership of the service.
  /// The service must exist for the lifetime of the Server instance.
  virtual bool RegisterService(const std::string* host, Service* service) = 0;

  /// Register a generic service. This call does not take ownership of the
  /// service. The service must exist for the lifetime of the Server instance.
  virtual void RegisterAsyncGenericService(AsyncGenericService* service) = 0;

  /// Register a callback generic service. This call does not take ownership of
  /// the  service. The service must exist for the lifetime of the Server
  /// instance. May not be abstract since this is a post-1.0 API addition.

  virtual void RegisterCallbackGenericService(CallbackGenericService*
                                              /*service*/) {}

  /// Tries to bind \a server to the given \a addr.
  ///
  /// It can be invoked multiple times.
  ///
  /// \param addr The address to try to bind to the server (eg, localhost:1234,
  /// 192.168.1.1:31416, [::1]:27182, etc.).
  /// \params creds The credentials associated with the server.
  ///
  /// \return bound port number on success, 0 on failure.
  ///
  /// \warning It's an error to call this method on an already started server.
  virtual int AddListeningPort(const std::string& addr,
                               ServerCredentials* creds) = 0;

  /// Start the server.
  ///
  /// \param cqs Completion queues for handling asynchronous services. The
  /// caller is required to keep all completion queues live until the server is
  /// destroyed.
  /// \param num_cqs How many completion queues does \a cqs hold.
  virtual void Start(grpc::ServerCompletionQueue** cqs, size_t num_cqs) = 0;

  virtual void ShutdownInternal(gpr_timespec deadline) = 0;

  virtual int max_receive_message_size() const = 0;

  virtual grpc_server* server() = 0;

  void PerformOpsOnCall(internal::CallOpSetInterface* ops,
                        internal::Call* call) override = 0;

  class BaseAsyncRequest : public internal::CompletionQueueTag {
   public:
    BaseAsyncRequest(ServerInterface* server, grpc::ServerContext* context,
                     internal::ServerAsyncStreamingInterface* stream,
                     grpc::CompletionQueue* call_cq,
                     grpc::ServerCompletionQueue* notification_cq, void* tag,
                     bool delete_on_finalize);
    ~BaseAsyncRequest() override;

    bool FinalizeResult(void** tag, bool* status) override;

   private:
    void ContinueFinalizeResultAfterInterception();

   protected:
    ServerInterface* const server_;
    grpc::ServerContext* const context_;
    internal::ServerAsyncStreamingInterface* const stream_;
    grpc::CompletionQueue* const call_cq_;
    grpc::ServerCompletionQueue* const notification_cq_;
    void* const tag_;
    const bool delete_on_finalize_;
    grpc_call* call_;
    internal::Call call_wrapper_;
    internal::InterceptorBatchMethodsImpl interceptor_methods_;
    bool done_intercepting_;
    bool call_metric_recording_enabled_;
    experimental::ServerMetricRecorder* server_metric_recorder_;
  };

  /// RegisteredAsyncRequest is not part of the C++ API
  class RegisteredAsyncRequest : public BaseAsyncRequest {
   public:
    RegisteredAsyncRequest(ServerInterface* server,
                           grpc::ServerContext* context,
                           internal::ServerAsyncStreamingInterface* stream,
                           grpc::CompletionQueue* call_cq,
                           grpc::ServerCompletionQueue* notification_cq,
                           void* tag, const char* name,
                           internal::RpcMethod::RpcType type);

    bool FinalizeResult(void** tag, bool* status) override {
      // If we are done intercepting, then there is nothing more for us to do
      if (done_intercepting_) {
        return BaseAsyncRequest::FinalizeResult(tag, status);
      }
      call_wrapper_ = grpc::internal::Call(
          call_, server_, call_cq_, server_->max_receive_message_size(),
          context_->set_server_rpc_info(name_, type_,
                                        *server_->interceptor_creators()));
      return BaseAsyncRequest::FinalizeResult(tag, status);
    }

   protected:
    void IssueRequest(void* registered_method, grpc_byte_buffer** payload,
                      grpc::ServerCompletionQueue* notification_cq);
    const char* name_;
    const internal::RpcMethod::RpcType type_;
  };

  class NoPayloadAsyncRequest final : public RegisteredAsyncRequest {
   public:
    NoPayloadAsyncRequest(internal::RpcServiceMethod* registered_method,
                          ServerInterface* server, grpc::ServerContext* context,
                          internal::ServerAsyncStreamingInterface* stream,
                          grpc::CompletionQueue* call_cq,
                          grpc::ServerCompletionQueue* notification_cq,
                          void* tag)
        : RegisteredAsyncRequest(
              server, context, stream, call_cq, notification_cq, tag,
              registered_method->name(), registered_method->method_type()) {
      IssueRequest(registered_method->server_tag(), nullptr, notification_cq);
    }

    // uses RegisteredAsyncRequest::FinalizeResult
  };

  template <class Message>
  class PayloadAsyncRequest final : public RegisteredAsyncRequest {
   public:
    PayloadAsyncRequest(internal::RpcServiceMethod* registered_method,
                        ServerInterface* server, grpc::ServerContext* context,
                        internal::ServerAsyncStreamingInterface* stream,
                        grpc::CompletionQueue* call_cq,
                        grpc::ServerCompletionQueue* notification_cq, void* tag,
                        Message* request)
        : RegisteredAsyncRequest(
              server, context, stream, call_cq, notification_cq, tag,
              registered_method->name(), registered_method->method_type()),
          registered_method_(registered_method),
          request_(request) {
      IssueRequest(registered_method->server_tag(), payload_.bbuf_ptr(),
                   notification_cq);
    }

    ~PayloadAsyncRequest() override {
      payload_.Release();  // We do not own the payload_
    }

    bool FinalizeResult(void** tag, bool* status) override {
      // If we are done intercepting, then there is nothing more for us to do
      if (done_intercepting_) {
        return RegisteredAsyncRequest::FinalizeResult(tag, status);
      }
      if (*status) {
        if (!payload_.Valid() || !SerializationTraits<Message>::Deserialize(
                                      payload_.bbuf_ptr(), request_)
                                      .ok()) {
          // If deserialization fails, we cancel the call and instantiate
          // a new instance of ourselves to request another call.  We then
          // return false, which prevents the call from being returned to
          // the application.
          grpc_call_cancel_with_status(call_, GRPC_STATUS_INTERNAL,
                                       "Unable to parse request", nullptr);
          grpc_call_unref(call_);
          new PayloadAsyncRequest(registered_method_, server_, context_,
                                  stream_, call_cq_, notification_cq_, tag_,
                                  request_);
          delete this;
          return false;
        }
      }
      // Set interception point for recv message
      interceptor_methods_.AddInterceptionHookPoint(
          experimental::InterceptionHookPoints::POST_RECV_MESSAGE);
      interceptor_methods_.SetRecvMessage(request_, nullptr);
      return RegisteredAsyncRequest::FinalizeResult(tag, status);
    }

   private:
    internal::RpcServiceMethod* const registered_method_;
    Message* const request_;
    ByteBuffer payload_;
  };

  class GenericAsyncRequest : public BaseAsyncRequest {
   public:
    GenericAsyncRequest(ServerInterface* server, GenericServerContext* context,
                        internal::ServerAsyncStreamingInterface* stream,
                        grpc::CompletionQueue* call_cq,
                        grpc::ServerCompletionQueue* notification_cq, void* tag,
                        bool delete_on_finalize, bool issue_request = true);

    bool FinalizeResult(void** tag, bool* status) override;

   protected:
    void IssueRequest();

   private:
    grpc_call_details call_details_;
  };

  template <class Message>
  void RequestAsyncCall(internal::RpcServiceMethod* method,
                        grpc::ServerContext* context,
                        internal::ServerAsyncStreamingInterface* stream,
                        grpc::CompletionQueue* call_cq,
                        grpc::ServerCompletionQueue* notification_cq, void* tag,
                        Message* message) {
    ABSL_CHECK(method);
    new PayloadAsyncRequest<Message>(method, this, context, stream, call_cq,
                                     notification_cq, tag, message);
  }

  void RequestAsyncCall(internal::RpcServiceMethod* method,
                        grpc::ServerContext* context,
                        internal::ServerAsyncStreamingInterface* stream,
                        grpc::CompletionQueue* call_cq,
                        grpc::ServerCompletionQueue* notification_cq,
                        void* tag) {
    ABSL_CHECK(method);
    new NoPayloadAsyncRequest(method, this, context, stream, call_cq,
                              notification_cq, tag);
  }

  void RequestAsyncGenericCall(GenericServerContext* context,
                               internal::ServerAsyncStreamingInterface* stream,
                               grpc::CompletionQueue* call_cq,
                               grpc::ServerCompletionQueue* notification_cq,
                               void* tag) {
    new GenericAsyncRequest(this, context, stream, call_cq, notification_cq,
                            tag, true);
  }

 private:
  // EXPERIMENTAL
  // Getter method for the vector of interceptor factory objects.
  // Returns a nullptr (rather than being pure) since this is a post-1.0 method
  // and adding a new pure method to an interface would be a breaking change
  // (even though this is private and non-API)
  virtual std::vector<
      std::unique_ptr<experimental::ServerInterceptorFactoryInterface>>*
  interceptor_creators() {
    return nullptr;
  }

  // Whether per-call load reporting is enabled.
  virtual bool call_metric_recording_enabled() const = 0;

  // Interface to read or update server-wide metrics. Returns null when not set.
  virtual experimental::ServerMetricRecorder* server_metric_recorder()
      const = 0;

  // A method to get the callbackable completion queue associated with this
  // server. If the return value is nullptr, this server doesn't support
  // callback operations.
  // TODO(vjpai): Consider a better default like using a global CQ
  // Returns nullptr (rather than being pure) since this is a post-1.0 method
  // and adding a new pure method to an interface would be a breaking change
  // (even though this is private and non-API)
  virtual grpc::CompletionQueue* CallbackCQ() { return nullptr; }
};

}  // namespace grpc

#endif  // GRPCPP_SERVER_INTERFACE_H
