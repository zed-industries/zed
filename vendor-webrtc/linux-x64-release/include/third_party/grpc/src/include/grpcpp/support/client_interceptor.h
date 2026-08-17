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

#ifndef GRPCPP_SUPPORT_CLIENT_INTERCEPTOR_H
#define GRPCPP_SUPPORT_CLIENT_INTERCEPTOR_H

#include <grpcpp/impl/rpc_method.h>
#include <grpcpp/support/interceptor.h>
#include <grpcpp/support/string_ref.h>

#include <memory>
#include <vector>

#include "absl/log/absl_check.h"

namespace grpc {

class Channel;
class ClientContext;

namespace internal {
class InterceptorBatchMethodsImpl;
}

namespace experimental {
class ClientRpcInfo;

// A factory interface for creation of client interceptors. A vector of
// factories can be provided at channel creation which will be used to create a
// new vector of client interceptors per RPC. Client interceptor authors should
// create a subclass of ClientInterceptorFactorInterface which creates objects
// of their interceptors.
class ClientInterceptorFactoryInterface {
 public:
  virtual ~ClientInterceptorFactoryInterface() {}
  // Returns a pointer to an Interceptor object on successful creation, nullptr
  // otherwise. If nullptr is returned, this server interceptor factory is
  // ignored for the purposes of that RPC.
  virtual Interceptor* CreateClientInterceptor(ClientRpcInfo* info) = 0;
};
}  // namespace experimental

namespace internal {
extern experimental::ClientInterceptorFactoryInterface*
    g_global_client_interceptor_factory;

extern experimental::ClientInterceptorFactoryInterface*
    g_global_client_stats_interceptor_factory;
}  // namespace internal

/// ClientRpcInfo represents the state of a particular RPC as it
/// appears to an interceptor. It is created and owned by the library and
/// passed to the CreateClientInterceptor method of the application's
/// ClientInterceptorFactoryInterface implementation
namespace experimental {
class ClientRpcInfo {
 public:
  // TODO(yashykt): Stop default-constructing ClientRpcInfo and remove UNKNOWN
  //                from the list of possible Types.
  /// Type categorizes RPCs by unary or streaming type
  enum class Type {
    UNARY,
    CLIENT_STREAMING,
    SERVER_STREAMING,
    BIDI_STREAMING,
    UNKNOWN  // UNKNOWN is not API and will be removed later
  };

  ~ClientRpcInfo() {}

  // Delete copy constructor but allow default move constructor
  ClientRpcInfo(const ClientRpcInfo&) = delete;
  ClientRpcInfo(ClientRpcInfo&&) = default;

  // Getter methods

  /// Return the fully-specified method name
  const char* method() const { return method_; }

  /// Return an identifying suffix for the client stub, or nullptr if one wasn't
  /// specified.
  const char* suffix_for_stats() const { return suffix_for_stats_; }

  /// Return a pointer to the channel on which the RPC is being sent
  ChannelInterface* channel() { return channel_; }

  /// Return a pointer to the underlying ClientContext structure associated
  /// with the RPC to support features that apply to it
  grpc::ClientContext* client_context() { return ctx_; }

  /// Return the type of the RPC (unary or a streaming flavor)
  Type type() const { return type_; }

 private:
  static_assert(Type::UNARY ==
                    static_cast<Type>(internal::RpcMethod::NORMAL_RPC),
                "violated expectation about Type enum");
  static_assert(Type::CLIENT_STREAMING ==
                    static_cast<Type>(internal::RpcMethod::CLIENT_STREAMING),
                "violated expectation about Type enum");
  static_assert(Type::SERVER_STREAMING ==
                    static_cast<Type>(internal::RpcMethod::SERVER_STREAMING),
                "violated expectation about Type enum");
  static_assert(Type::BIDI_STREAMING ==
                    static_cast<Type>(internal::RpcMethod::BIDI_STREAMING),
                "violated expectation about Type enum");

  // Default constructor should only be used by ClientContext
  ClientRpcInfo() = default;

  // Constructor will only be called from ClientContext
  ClientRpcInfo(grpc::ClientContext* ctx, internal::RpcMethod::RpcType type,
                const char* method, const char* suffix_for_stats,
                grpc::ChannelInterface* channel)
      : ctx_(ctx),
        type_(static_cast<Type>(type)),
        method_(method),
        suffix_for_stats_(suffix_for_stats),
        channel_(channel) {}

  // Move assignment should only be used by ClientContext
  // TODO(yashykt): Delete move assignment
  ClientRpcInfo& operator=(ClientRpcInfo&&) = default;

  // Runs interceptor at pos \a pos.
  void RunInterceptor(
      experimental::InterceptorBatchMethods* interceptor_methods, size_t pos) {
    ABSL_CHECK_LT(pos, interceptors_.size());
    interceptors_[pos]->Intercept(interceptor_methods);
  }

  void RegisterInterceptors(
      const std::vector<std::unique_ptr<
          experimental::ClientInterceptorFactoryInterface>>& creators,
      size_t interceptor_pos) {
    // TODO(yashykt): This calculation seems broken for the case where an
    // interceptor factor returns nullptr.
    size_t num_interceptors =
        creators.size() +
        (internal::g_global_client_stats_interceptor_factory != nullptr) +
        (internal::g_global_client_interceptor_factory != nullptr);
    if (interceptor_pos > num_interceptors) {
      // No interceptors to register
      return;
    }
    if (internal::g_global_client_stats_interceptor_factory != nullptr) {
      interceptors_.push_back(std::unique_ptr<experimental::Interceptor>(
          internal::g_global_client_stats_interceptor_factory
              ->CreateClientInterceptor(this)));
      --interceptor_pos;
    }
    // NOTE: The following is not a range-based for loop because it will only
    //       iterate over a portion of the creators vector.
    for (auto it = creators.begin() + interceptor_pos; it != creators.end();
         ++it) {
      auto* interceptor = (*it)->CreateClientInterceptor(this);
      if (interceptor != nullptr) {
        interceptors_.push_back(
            std::unique_ptr<experimental::Interceptor>(interceptor));
      }
    }
    if (internal::g_global_client_interceptor_factory != nullptr) {
      interceptors_.push_back(std::unique_ptr<experimental::Interceptor>(
          internal::g_global_client_interceptor_factory
              ->CreateClientInterceptor(this)));
    }
  }

  grpc::ClientContext* ctx_ = nullptr;
  // TODO(yashykt): make type_ const once move-assignment is deleted
  Type type_{Type::UNKNOWN};
  const char* method_ = nullptr;
  const char* suffix_for_stats_ = nullptr;
  grpc::ChannelInterface* channel_ = nullptr;
  std::vector<std::unique_ptr<experimental::Interceptor>> interceptors_;
  bool hijacked_ = false;
  size_t hijacked_interceptor_ = 0;

  friend class internal::InterceptorBatchMethodsImpl;
  friend class grpc::ClientContext;
};

// PLEASE DO NOT USE THIS. ALWAYS PREFER PER CHANNEL INTERCEPTORS OVER A GLOBAL
// INTERCEPTOR. IF USAGE IS ABSOLUTELY NECESSARY, PLEASE READ THE SAFETY NOTES.
// Registers a global client interceptor factory object, which is used for all
// RPCs made in this process. The application is responsible for maintaining the
// life of the object while gRPC operations are in progress. The global
// interceptor factory should only be registered once at the start of the
// process before any gRPC operations have begun.
void RegisterGlobalClientInterceptorFactory(
    ClientInterceptorFactoryInterface* factory);

// For testing purposes only
void TestOnlyResetGlobalClientInterceptorFactory();

}  // namespace experimental
}  // namespace grpc

#endif  // GRPCPP_SUPPORT_CLIENT_INTERCEPTOR_H
