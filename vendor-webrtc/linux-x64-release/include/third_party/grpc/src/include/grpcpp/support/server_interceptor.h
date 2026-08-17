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

#ifndef GRPCPP_SUPPORT_SERVER_INTERCEPTOR_H
#define GRPCPP_SUPPORT_SERVER_INTERCEPTOR_H

#include <grpcpp/impl/rpc_method.h>
#include <grpcpp/support/interceptor.h>
#include <grpcpp/support/string_ref.h>

#include <atomic>
#include <vector>

#include "absl/log/absl_check.h"

namespace grpc {
class ServerContextBase;
namespace internal {
class InterceptorBatchMethodsImpl;
}

namespace experimental {
class ServerRpcInfo;

// A factory interface for creation of server interceptors. A vector of
// factories can be provided to ServerBuilder which will be used to create a new
// vector of server interceptors per RPC. Server interceptor authors should
// create a subclass of ServerInterceptorFactorInterface which creates objects
// of their interceptors.
class ServerInterceptorFactoryInterface {
 public:
  virtual ~ServerInterceptorFactoryInterface() {}
  // Returns a pointer to an Interceptor object on successful creation, nullptr
  // otherwise. If nullptr is returned, this server interceptor factory is
  // ignored for the purposes of that RPC.
  virtual Interceptor* CreateServerInterceptor(ServerRpcInfo* info) = 0;
};

/// ServerRpcInfo represents the state of a particular RPC as it
/// appears to an interceptor. It is created and owned by the library and
/// passed to the CreateServerInterceptor method of the application's
/// ServerInterceptorFactoryInterface implementation
class ServerRpcInfo {
 public:
  /// Type categorizes RPCs by unary or streaming type
  enum class Type { UNARY, CLIENT_STREAMING, SERVER_STREAMING, BIDI_STREAMING };

  ~ServerRpcInfo() {}

  // Delete all copy and move constructors and assignments
  ServerRpcInfo(const ServerRpcInfo&) = delete;
  ServerRpcInfo& operator=(const ServerRpcInfo&) = delete;
  ServerRpcInfo(ServerRpcInfo&&) = delete;
  ServerRpcInfo& operator=(ServerRpcInfo&&) = delete;

  // Getter methods

  /// Return the fully-specified method name
  const char* method() const { return method_; }

  /// Return the type of the RPC (unary or a streaming flavor)
  Type type() const { return type_; }

  /// Return a pointer to the underlying ServerContext structure associated
  /// with the RPC to support features that apply to it
  ServerContextBase* server_context() { return ctx_; }

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

  ServerRpcInfo(ServerContextBase* ctx, const char* method,
                internal::RpcMethod::RpcType type)
      : ctx_(ctx), method_(method), type_(static_cast<Type>(type)) {}

  // Runs interceptor at pos \a pos.
  void RunInterceptor(
      experimental::InterceptorBatchMethods* interceptor_methods, size_t pos) {
    ABSL_CHECK_LT(pos, interceptors_.size());
    interceptors_[pos]->Intercept(interceptor_methods);
  }

  void RegisterInterceptors(
      const std::vector<
          std::unique_ptr<experimental::ServerInterceptorFactoryInterface>>&
          creators) {
    for (const auto& creator : creators) {
      auto* interceptor = creator->CreateServerInterceptor(this);
      if (interceptor != nullptr) {
        interceptors_.push_back(
            std::unique_ptr<experimental::Interceptor>(interceptor));
      }
    }
  }

  void Ref() { ref_.fetch_add(1, std::memory_order_relaxed); }
  void Unref() {
    if (GPR_UNLIKELY(ref_.fetch_sub(1, std::memory_order_acq_rel) == 1)) {
      delete this;
    }
  }

  ServerContextBase* ctx_ = nullptr;
  const char* method_ = nullptr;
  const Type type_;
  std::atomic<intptr_t> ref_{1};
  std::vector<std::unique_ptr<experimental::Interceptor>> interceptors_;

  friend class internal::InterceptorBatchMethodsImpl;
  friend class grpc::ServerContextBase;
};

}  // namespace experimental
}  // namespace grpc

#endif  // GRPCPP_SUPPORT_SERVER_INTERCEPTOR_H
