//
//
// Copyright 2016 gRPC authors.
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

#ifndef GRPCPP_IMPL_RPC_SERVICE_METHOD_H
#define GRPCPP_IMPL_RPC_SERVICE_METHOD_H

#include <grpcpp/impl/rpc_method.h>
#include <grpcpp/support/byte_buffer.h>
#include <grpcpp/support/config.h>
#include <grpcpp/support/status.h>

#include <climits>
#include <functional>
#include <map>
#include <memory>
#include <vector>

#include "absl/log/absl_check.h"
#include "absl/log/absl_log.h"

namespace grpc {
class ServerContextBase;
namespace internal {
/// Base class for running an RPC handler.
class MethodHandler {
 public:
  virtual ~MethodHandler() {}
  struct HandlerParameter {
    /// Constructor for HandlerParameter
    ///
    /// \param c : the gRPC Call structure for this server call
    /// \param context : the ServerContext structure for this server call
    /// \param req : the request payload, if appropriate for this RPC
    /// \param req_status : the request status after any interceptors have run
    /// \param handler_data: internal data for the handler.
    /// \param requester : used only by the callback API. It is a function
    ///        called by the RPC Controller to request another RPC (and also
    ///        to set up the state required to make that request possible)
    HandlerParameter(Call* c, grpc::ServerContextBase* context, void* req,
                     Status req_status, void* handler_data,
                     std::function<void()> requester)
        : call(c),
          server_context(context),
          request(req),
          status(req_status),
          internal_data(handler_data),
          call_requester(std::move(requester)) {}
    ~HandlerParameter() {}
    Call* const call;
    grpc::ServerContextBase* const server_context;
    void* const request;
    const Status status;
    void* const internal_data;
    const std::function<void()> call_requester;
  };
  virtual void RunHandler(const HandlerParameter& param) = 0;

  // Returns a pointer to the deserialized request. \a status reflects the
  // result of deserialization. This pointer and the status should be filled in
  // a HandlerParameter and passed to RunHandler. It is illegal to access the
  // pointer after calling RunHandler. Ownership of the deserialized request is
  // retained by the handler. Returns nullptr if deserialization failed.
  virtual void* Deserialize(grpc_call* /*call*/, grpc_byte_buffer* req,
                            Status* /*status*/, void** /*handler_data*/) {
    ABSL_CHECK_EQ(req, nullptr);
    return nullptr;
  }
};

/// Server side rpc method class
class RpcServiceMethod : public RpcMethod {
 public:
  /// Takes ownership of the handler
  RpcServiceMethod(const char* name, RpcMethod::RpcType type,
                   MethodHandler* handler)
      : RpcMethod(name, type),
        server_tag_(nullptr),
        api_type_(ApiType::SYNC),
        handler_(handler) {}

  enum class ApiType {
    SYNC,
    ASYNC,
    RAW,
    CALL_BACK,  // not CALLBACK because that is reserved in Windows
    RAW_CALL_BACK,
  };

  void set_server_tag(void* tag) { server_tag_ = tag; }
  void* server_tag() const { return server_tag_; }
  /// if MethodHandler is nullptr, then this is an async method
  MethodHandler* handler() const { return handler_.get(); }
  ApiType api_type() const { return api_type_; }
  void SetHandler(MethodHandler* handler) { handler_.reset(handler); }
  void SetServerApiType(RpcServiceMethod::ApiType type) {
    if ((api_type_ == ApiType::SYNC) &&
        (type == ApiType::ASYNC || type == ApiType::RAW)) {
      // this marks this method as async
      handler_.reset();
    } else if (api_type_ != ApiType::SYNC) {
      // this is not an error condition, as it allows users to declare a server
      // like WithRawMethod_foo<AsyncService>. However since it
      // overwrites behavior, it should be logged.
      ABSL_LOG(INFO)
          << "You are marking method " << name() << " as '"
          << TypeToString(api_type_)
          << "', even though it was previously marked '" << TypeToString(type)
          << "'. This behavior will overwrite the original behavior. If "
             "you expected this then ignore this message.";
    }
    api_type_ = type;
  }

 private:
  void* server_tag_;
  ApiType api_type_;
  std::unique_ptr<MethodHandler> handler_;

  const char* TypeToString(RpcServiceMethod::ApiType type) {
    switch (type) {
      case ApiType::SYNC:
        return "sync";
      case ApiType::ASYNC:
        return "async";
      case ApiType::RAW:
        return "raw";
      case ApiType::CALL_BACK:
        return "callback";
      case ApiType::RAW_CALL_BACK:
        return "raw_callback";
      default:
        GPR_UNREACHABLE_CODE(return "unknown");
    }
  }
};
}  // namespace internal

}  // namespace grpc

#endif  // GRPCPP_IMPL_RPC_SERVICE_METHOD_H
