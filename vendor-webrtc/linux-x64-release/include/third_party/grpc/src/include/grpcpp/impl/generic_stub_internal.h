//
//
// Copyright 2024 gRPC authors.
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

#ifndef GRPCPP_IMPL_GENERIC_STUB_INTERNAL_H
#define GRPCPP_IMPL_GENERIC_STUB_INTERNAL_H

#include <grpcpp/client_context.h>
#include <grpcpp/impl/rpc_method.h>
#include <grpcpp/support/byte_buffer.h>
#include <grpcpp/support/client_callback.h>
#include <grpcpp/support/status.h>
#include <grpcpp/support/stub_options.h>

#include <functional>

namespace grpc {

template <class RequestType, class ResponseType>
class TemplatedGenericStub;
template <class RequestType, class ResponseType>
class TemplatedGenericStubCallback;

namespace internal {

/// Generic stubs provide a type-unaware interface to call gRPC methods
/// by name. In practice, the Request and Response types should be basic
/// types like grpc::ByteBuffer or proto::MessageLite (the base protobuf).
template <class RequestType, class ResponseType>
class TemplatedGenericStubCallbackInternal {
 public:
  explicit TemplatedGenericStubCallbackInternal(
      std::shared_ptr<grpc::ChannelInterface> channel)
      : channel_(channel) {}

  /// Setup and start a unary call to a named method \a method using
  /// \a context and specifying the \a request and \a response buffers.
  void UnaryCall(ClientContext* context, const std::string& method,
                 StubOptions options, const RequestType* request,
                 ResponseType* response,
                 std::function<void(grpc::Status)> on_completion) {
    UnaryCallInternal(context, method, options, request, response,
                      std::move(on_completion));
  }

  /// Setup a unary call to a named method \a method using
  /// \a context and specifying the \a request and \a response buffers.
  /// Like any other reactor-based RPC, it will not be activated until
  /// StartCall is invoked on its reactor.
  void PrepareUnaryCall(ClientContext* context, const std::string& method,
                        StubOptions options, const RequestType* request,
                        ResponseType* response, ClientUnaryReactor* reactor) {
    PrepareUnaryCallInternal(context, method, options, request, response,
                             reactor);
  }

  /// Setup a call to a named method \a method using \a context and tied to
  /// \a reactor . Like any other bidi streaming RPC, it will not be activated
  /// until StartCall is invoked on its reactor.
  void PrepareBidiStreamingCall(
      ClientContext* context, const std::string& method, StubOptions options,
      ClientBidiReactor<RequestType, ResponseType>* reactor) {
    PrepareBidiStreamingCallInternal(context, method, options, reactor);
  }

 private:
  template <class Req, class Resp>
  friend class grpc::TemplatedGenericStub;
  template <class Req, class Resp>
  friend class grpc::TemplatedGenericStubCallback;
  std::shared_ptr<grpc::ChannelInterface> channel_;

  void UnaryCallInternal(ClientContext* context, const std::string& method,
                         StubOptions options, const RequestType* request,
                         ResponseType* response,
                         std::function<void(grpc::Status)> on_completion) {
    internal::CallbackUnaryCall(
        channel_.get(),
        grpc::internal::RpcMethod(method.c_str(), options.suffix_for_stats(),
                                  grpc::internal::RpcMethod::NORMAL_RPC),
        context, request, response, std::move(on_completion));
  }

  void PrepareUnaryCallInternal(ClientContext* context,
                                const std::string& method, StubOptions options,
                                const RequestType* request,
                                ResponseType* response,
                                ClientUnaryReactor* reactor) {
    internal::ClientCallbackUnaryFactory::Create<RequestType, ResponseType>(
        channel_.get(),
        grpc::internal::RpcMethod(method.c_str(), options.suffix_for_stats(),
                                  grpc::internal::RpcMethod::NORMAL_RPC),
        context, request, response, reactor);
  }

  void PrepareBidiStreamingCallInternal(
      ClientContext* context, const std::string& method, StubOptions options,
      ClientBidiReactor<RequestType, ResponseType>* reactor) {
    internal::ClientCallbackReaderWriterFactory<RequestType, ResponseType>::
        Create(channel_.get(),
               grpc::internal::RpcMethod(
                   method.c_str(), options.suffix_for_stats(),
                   grpc::internal::RpcMethod::BIDI_STREAMING),
               context, reactor);
  }
};

}  // namespace internal
}  // namespace grpc

#endif  // GRPCPP_IMPL_GENERIC_STUB_INTERNAL_H
