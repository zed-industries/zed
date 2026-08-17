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

#ifndef GRPCPP_IMPL_SERVICE_TYPE_H
#define GRPCPP_IMPL_SERVICE_TYPE_H

#include <grpcpp/impl/rpc_service_method.h>
#include <grpcpp/impl/serialization_traits.h>
#include <grpcpp/server_interface.h>
#include <grpcpp/support/config.h>
#include <grpcpp/support/status.h>

#include "absl/log/absl_check.h"

namespace grpc {

class CompletionQueue;
class ServerContext;
class ServerInterface;

namespace internal {
class Call;
class ServerAsyncStreamingInterface {
 public:
  virtual ~ServerAsyncStreamingInterface() {}

  /// Request notification of the sending of initial metadata to the client.
  /// Completion will be notified by \a tag on the associated completion
  /// queue. This call is optional, but if it is used, it cannot be used
  /// concurrently with or after the \a Finish method.
  ///
  /// \param[in] tag Tag identifying this request.
  virtual void SendInitialMetadata(void* tag) = 0;

 private:
  friend class grpc::ServerInterface;
  virtual void BindCall(Call* call) = 0;
};
}  // namespace internal

/// Descriptor of an RPC service and its various RPC methods
class Service {
 public:
  Service() : server_(nullptr) {}
  virtual ~Service() {}

  bool has_async_methods() const {
    for (const auto& method : methods_) {
      if (method && method->handler() == nullptr) {
        return true;
      }
    }
    return false;
  }

  bool has_synchronous_methods() const {
    for (const auto& method : methods_) {
      if (method &&
          method->api_type() == internal::RpcServiceMethod::ApiType::SYNC) {
        return true;
      }
    }
    return false;
  }

  bool has_callback_methods() const {
    for (const auto& method : methods_) {
      if (method && (method->api_type() ==
                         internal::RpcServiceMethod::ApiType::CALL_BACK ||
                     method->api_type() ==
                         internal::RpcServiceMethod::ApiType::RAW_CALL_BACK)) {
        return true;
      }
    }
    return false;
  }

  bool has_generic_methods() const {
    for (const auto& method : methods_) {
      if (method == nullptr) {
        return true;
      }
    }
    return false;
  }

 protected:
  template <class Message>
  void RequestAsyncUnary(int index, grpc::ServerContext* context,
                         Message* request,
                         internal::ServerAsyncStreamingInterface* stream,
                         grpc::CompletionQueue* call_cq,
                         grpc::ServerCompletionQueue* notification_cq,
                         void* tag) {
    // Typecast the index to size_t for indexing into a vector
    // while preserving the API that existed before a compiler
    // warning was first seen (grpc/grpc#11664)
    size_t idx = static_cast<size_t>(index);
    server_->RequestAsyncCall(methods_[idx].get(), context, stream, call_cq,
                              notification_cq, tag, request);
  }
  void RequestAsyncClientStreaming(
      int index, grpc::ServerContext* context,
      internal::ServerAsyncStreamingInterface* stream,
      grpc::CompletionQueue* call_cq,
      grpc::ServerCompletionQueue* notification_cq, void* tag) {
    size_t idx = static_cast<size_t>(index);
    server_->RequestAsyncCall(methods_[idx].get(), context, stream, call_cq,
                              notification_cq, tag);
  }
  template <class Message>
  void RequestAsyncServerStreaming(
      int index, grpc::ServerContext* context, Message* request,
      internal::ServerAsyncStreamingInterface* stream,
      grpc::CompletionQueue* call_cq,
      grpc::ServerCompletionQueue* notification_cq, void* tag) {
    size_t idx = static_cast<size_t>(index);
    server_->RequestAsyncCall(methods_[idx].get(), context, stream, call_cq,
                              notification_cq, tag, request);
  }
  void RequestAsyncBidiStreaming(
      int index, grpc::ServerContext* context,
      internal::ServerAsyncStreamingInterface* stream,
      grpc::CompletionQueue* call_cq,
      grpc::ServerCompletionQueue* notification_cq, void* tag) {
    size_t idx = static_cast<size_t>(index);
    server_->RequestAsyncCall(methods_[idx].get(), context, stream, call_cq,
                              notification_cq, tag);
  }

  void AddMethod(internal::RpcServiceMethod* method) {
    methods_.emplace_back(method);
  }

  void MarkMethodAsync(int index) {
    // This does not have to be a hard error, however no one has approached us
    // with a use case yet. Please file an issue if you believe you have one.
    size_t idx = static_cast<size_t>(index);
    ABSL_CHECK_NE(methods_[idx].get(), nullptr)
        << "Cannot mark the method as 'async' because it has already been "
           "marked as 'generic'.";
    methods_[idx]->SetServerApiType(internal::RpcServiceMethod::ApiType::ASYNC);
  }

  void MarkMethodRaw(int index) {
    // This does not have to be a hard error, however no one has approached us
    // with a use case yet. Please file an issue if you believe you have one.
    size_t idx = static_cast<size_t>(index);
    ABSL_CHECK_NE(methods_[idx].get(), nullptr)
        << "Cannot mark the method as 'raw' because it has already "
           "been marked as 'generic'.";
    methods_[idx]->SetServerApiType(internal::RpcServiceMethod::ApiType::RAW);
  }

  void MarkMethodGeneric(int index) {
    // This does not have to be a hard error, however no one has approached us
    // with a use case yet. Please file an issue if you believe you have one.
    size_t idx = static_cast<size_t>(index);
    ABSL_CHECK_NE(methods_[idx]->handler(), nullptr)
        << "Cannot mark the method as 'generic' because it has already been "
           "marked as 'async' or 'raw'.";
    methods_[idx].reset();
  }

  void MarkMethodStreamed(int index, internal::MethodHandler* streamed_method) {
    // This does not have to be a hard error, however no one has approached us
    // with a use case yet. Please file an issue if you believe you have one.
    size_t idx = static_cast<size_t>(index);
    ABSL_CHECK(methods_[idx] && methods_[idx]->handler())
        << "Cannot mark an async or generic method Streamed";
    methods_[idx]->SetHandler(streamed_method);

    // From the server's point of view, streamed unary is a special
    // case of BIDI_STREAMING that has 1 read and 1 write, in that order,
    // and split server-side streaming is BIDI_STREAMING with 1 read and
    // any number of writes, in that order.
    methods_[idx]->SetMethodType(internal::RpcMethod::BIDI_STREAMING);
  }

  void MarkMethodCallback(int index, internal::MethodHandler* handler) {
    // This does not have to be a hard error, however no one has approached us
    // with a use case yet. Please file an issue if you believe you have one.
    size_t idx = static_cast<size_t>(index);
    ABSL_CHECK_NE(methods_[idx].get(), nullptr)
        << "Cannot mark the method as 'callback' because it has already been "
           "marked as 'generic'.";
    methods_[idx]->SetHandler(handler);
    methods_[idx]->SetServerApiType(
        internal::RpcServiceMethod::ApiType::CALL_BACK);
  }

  void MarkMethodRawCallback(int index, internal::MethodHandler* handler) {
    // This does not have to be a hard error, however no one has approached us
    // with a use case yet. Please file an issue if you believe you have one.
    size_t idx = static_cast<size_t>(index);
    ABSL_CHECK_NE(methods_[idx].get(), nullptr)
        << "Cannot mark the method as 'raw callback' because it has already "
           "been marked as 'generic'.";
    methods_[idx]->SetHandler(handler);
    methods_[idx]->SetServerApiType(
        internal::RpcServiceMethod::ApiType::RAW_CALL_BACK);
  }

  internal::MethodHandler* GetHandler(int index) {
    size_t idx = static_cast<size_t>(index);
    return methods_[idx]->handler();
  }

 private:
  friend class Server;
  friend class ServerInterface;
  ServerInterface* server_;
  std::vector<std::unique_ptr<internal::RpcServiceMethod>> methods_;
};

}  // namespace grpc

#endif  // GRPCPP_IMPL_SERVICE_TYPE_H
